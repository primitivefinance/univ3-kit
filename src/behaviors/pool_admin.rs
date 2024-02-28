use std::sync::Arc;

use anyhow::{anyhow, Result};
use arbiter_core::middleware::ArbiterMiddleware;
use arbiter_engine::messager::{Message, Messager};
use ethers::types::H160;

use super::*;
use crate::bindings::uniswap_v3_factory::UniswapV3Factory;

#[derive(Debug, Serialize, Deserialize)]
pub struct PoolAdmin {
    #[serde(skip)]
    pub messager: Option<Messager>,

    #[serde(skip)]
    pub client: Option<Arc<ArbiterMiddleware>>,
}

/// Used as an action to ask what tokens are available.
#[derive(Debug, Deserialize, Serialize)]
pub enum PoolAdminQuery {
    /// Deploy request.
    PoolCreation(PoolCreation),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PoolCreation {
    factory: H160,
    token_0: H160,
    token_1: H160,
    fee: u32,
}

#[async_trait::async_trait]
impl Behavior<Message> for PoolAdmin {
    async fn startup(
        &mut self,
        client: Arc<ArbiterMiddleware>,
        messager: Messager,
    ) -> Result<Option<EventStream<Message>>> {
        self.client = Some(client.clone());
        self.messager = Some(messager.clone());

        Ok(Some(messager.clone().stream().unwrap()))
    }

    async fn process(&mut self, event: Message) -> Result<ControlFlow> {
        let query: PoolAdminQuery = match serde_json::from_str(&event.data) {
            Ok(query) => query,
            Err(_) => {
                eprintln!("Failed to deserialize the event data into a PoolAdminQuery");
                return Ok(ControlFlow::Continue);
            }
        };

        match query {
            PoolAdminQuery::PoolCreation(pool_creation) => {
                let _ = UniswapV3Factory::new(pool_creation.factory, self.client.clone().unwrap())
                    .create_pool(
                        pool_creation.token_0,
                        pool_creation.token_1,
                        pool_creation.fee,
                    )
                    .call()
                    .await
                    .map_err(|e| anyhow!("Failed to create pool: {}", e));

                Ok(ControlFlow::Continue)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use anyhow::Result;
    use arbiter_bindings::bindings::arbiter_token::ArbiterToken;
    use arbiter_core::middleware::ArbiterMiddleware;
    use arbiter_engine::{
        agent::Agent,
        machine::{Behavior, EventStream},
        messager::{Messager, To},
        world::World,
    };
    use ethers::middleware::Middleware;
    use serde::{Deserialize, Serialize};

    use crate::behaviors::{
        deployer::deploy_factory,
        pool_admin::{PoolAdmin, PoolCreation},
    };

    #[derive(Debug, Deserialize, Serialize)]
    pub struct MockDeployer {}

    /// Behaviour initialized after mock agents have run to assert assumptions about their execution.
    #[derive(Debug, Deserialize, Serialize)]
    pub struct Asserter {}

    #[async_trait::async_trait]
    impl Behavior<()> for MockDeployer {
        async fn startup(
            &mut self,
            client: Arc<ArbiterMiddleware>,
            messager: Messager,
        ) -> Result<Option<EventStream<()>>> {
            let factory = deploy_factory(&client.clone()).await?;

            let token_0 = ArbiterToken::deploy(
                client.clone(),
                ("Token 0".to_string(), "TKN0".to_string(), 18_u8),
            )?
            .send()
            .await?;

            let token_1 =
                ArbiterToken::deploy(client, ("Token 1".to_string(), "TKN1".to_string(), 18_u8))?
                    .send()
                    .await?;

            let pool_creation_request = PoolCreation {
                factory: factory.address(),
                token_0: token_0.address(),
                token_1: token_1.address(),
                fee: 100,
            };

            messager
                .send(To::All, serde_json::to_string(&pool_creation_request)?)
                .await?;

            Ok(None)
        }
    }

    #[async_trait::async_trait]
    impl Behavior<()> for Asserter {
        async fn startup(
            &mut self,
            client: Arc<ArbiterMiddleware>,
            _messager: Messager,
        ) -> Result<Option<EventStream<()>>> {
            let accounts = client.get_accounts().await?;

            // it is enough to assert that the accounts have been created.
            assert!(accounts.len() == 5, "actual length: {}", accounts.len());

            Ok(None)
        }
    }

    #[tokio::test]
    async fn pool_admin_behaviour_test() {
        let mut world = World::new("univ3");

        // messager and client are initialized later.
        let pool_admin_behaviour = PoolAdmin {
            messager: None,
            client: None,
        };

        let mock_deployer = MockDeployer {};
        let asserter = Asserter {};

        let pool_admin = Agent::builder("pool_admin").with_behavior(pool_admin_behaviour);
        let deployer = Agent::builder("mock_deployer").with_behavior(mock_deployer);
        let asserter = Agent::builder("asserter").with_behavior(asserter);

        world.add_agent(pool_admin);
        world.add_agent(deployer);

        let _ = world.run().await.expect("world failed to run");

        let env = world.environment;
        let mut world = World::new("univ3");

        world.environment = env;

        world.add_agent(asserter);

        world.run().await.expect("world failed to run");
    }
}
