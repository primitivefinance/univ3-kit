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
    pub factory: H160,
    pub token_0: H160,
    pub token_1: H160,
    pub fee: u32,
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

        Ok(None)
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
    use arbiter_engine::{agent::Agent, world::World};
    use tracing_subscriber::FmtSubscriber;

    use crate::bindings::uniswap_v3_factory::UniswapV3Factory;
    use arbiter_bindings::bindings::arbiter_token::ArbiterToken;
    use arbiter_engine::messager::To;

    use crate::behaviors::pool_admin::{PoolAdmin, PoolCreation};

    #[tokio::test]
    async fn pool_admin_behaviour_test() {
        let subscriber = FmtSubscriber::new();

        tracing::subscriber::set_global_default(subscriber)
            .expect("setting default subscriber failed");

        let mut world = World::new("univ3");

        let factory = UniswapV3Factory::deploy(world.client.clone(), ())
            .unwrap()
            .send()
            .await
            .unwrap();

        let token_0 = ArbiterToken::deploy(
            client.clone(),
            (String::from("0"), String::from("0"), 18_u8),
        ).unwrap()
        .send()
        .await.unwrap();

        let token_1 = ArbiterToken::deploy(
            client.clone(),
            (String::from("1"), String::from("1"), 18_u8),
        ).unwrap()
        .send()
        .await.unwrap();

        // messager and client are initialized later.
        let pool_admin_behaviour = PoolAdmin {
            messager: None,
            client: None,
        };

        let pool_creation_request = PoolCreation {
            factory: factory.address(),
            token_0: token_0.address(),
            token_1: token_1.address(),
            fee: 100,
        };

        let agent = Agent::builder("pool_admin");

        world.add_agent(agent.with_behavior(pool_admin_behaviour));

        let messager = world.messager.clone();

        messager
            .send(To::All, serde_json::to_string(&pool_creation_request).unwrap())
            .await.unwrap();

        world.run().await.expect("world failed to run");

        // world
        //     .environment
        //     .db
        //     .accounts
        //     .get(&factory.address())
        //     .expect("factory should be deployed");
    }
}
