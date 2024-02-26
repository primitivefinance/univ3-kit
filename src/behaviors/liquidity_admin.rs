use std::{collections::HashMap, sync::Arc};

use anyhow::Result;
use arbiter_core::middleware::ArbiterMiddleware;
use arbiter_engine::messager::{Message, Messager};
use ethers::types::H160;

use super::*;
use crate::bindings::{
    nonfungible_position_manager::NonfungiblePositionManager, shared_types::*,
    uniswap_v3_pool::UniswapV3Pool,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct LiquidityAdmin {
    pub pool_addresses: Option<HashMap<String, H160>>,
    #[serde(skip)]
    pub position_manager: Option<NonfungiblePositionManager<ArbiterMiddleware>>,
    #[serde(skip)]
    pub messager: Option<Messager>,
    #[serde(skip)]
    pub client: Option<Arc<ArbiterMiddleware>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum LiquidityAdminQuery {
    MintPosition(MintPosition),
    IncreasePosition(IncreasePosition),
    DecreasePosition(DecreasePosition),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MintPosition {
    pool: H160,
    params: MintParams,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct IncreasePosition {
    pool: H160,
    params: IncreaseLiquidityParams,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DecreasePosition {
    pool: H160,
    params: DecreaseLiquidityParams,
}

#[async_trait::async_trait]
impl Behavior<Message> for LiquidityAdmin {
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
        let query: LiquidityAdminQuery = match serde_json::from_str(&event.data) {
            Ok(query) => query,
            Err(_) => {
                eprintln!("Failed to deserialize the event data into a PositionCreation query");
                return Ok(ControlFlow::Continue);
            }
        };

        match query {
            LiquidityAdminQuery::MintPosition(mint_position) => {
                if let Some(position_manager) = self.position_manager.as_ref() {
                    match position_manager.mint(mint_position.params).send().await {
                        Ok(_) => println!("Minting position successful"),
                        Err(e) => eprintln!("Failed to mint position: {:?}", e),
                    }
                } else {
                    eprintln!("Position manager not initialized");
                }

                Ok(ControlFlow::Continue)
            }
            LiquidityAdminQuery::IncreasePosition(increase_position) => {
                if let Some(position_manager) = self.position_manager.as_ref() {
                    match position_manager
                        .increase_liquidity(increase_position.params)
                        .send()
                        .await
                    {
                        Ok(_) => println!("Increasing position successful"),
                        Err(e) => eprintln!("Failed to increase position: {:?}", e),
                    }
                } else {
                    eprintln!("Position manager not initialized");
                }
                Ok(ControlFlow::Continue)
            }
            LiquidityAdminQuery::DecreasePosition(decrease_position) => {
                if let Some(position_manager) = self.position_manager.as_ref() {
                    match position_manager
                        .decrease_liquidity(decrease_position.params)
                        .send()
                        .await
                    {
                        Ok(_) => println!("Decreasing position successful"),
                        Err(e) => eprintln!("Failed to decrease position: {:?}", e),
                    }
                } else {
                    eprintln!("Position manager not initialized");
                }
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
        liquidity_admin::{LiquidityAdmin, LiquidityAdminQuery},
        pool_admin::{PoolAdmin, PoolCreation},
        token_admin::MintRequest,
    };

    #[derive(Debug, Deserialize, Serialize)]
    pub struct MockDeployer {}

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

            // WIP need to test liquidity admin
            assert!(accounts.len() == 5, "actual length: {}", accounts.len());

            Ok(None)
        }
    }

    #[tokio::test]
    async fn liquidity_admin_behaviour_test() {
        let mut world = World::new("univ3");

        let liquidity_admin_behavior = LiquidityAdmin {
            pool_addresses: None,
            position_manager: None,
            messager: None,
            client: None,
        };

        // messager and client are initialized later.
        let pool_admin_behaviour = PoolAdmin {
            messager: None,
            client: None,
        };

        let mock_deployer = MockDeployer {};
        let asserter = Asserter {};

        let liquidity_admin =
            Agent::builder("liquidity_admin").with_behavior(liquidity_admin_behavior);
        let pool_admin = Agent::builder("pool_admin").with_behavior(pool_admin_behaviour);
        let deployer = Agent::builder("mock_deployer").with_behavior(mock_deployer);
        let asserter = Agent::builder("asserter").with_behavior(asserter);

        world.add_agent(liquidity_admin);
        world.add_agent(pool_admin);
        world.add_agent(deployer);

        world.run().await.expect("world failed to run");

        let env = world.environment;
        let mut world = World::new("univ3");

        world.environment = env;

        world.add_agent(asserter);

        world.run().await.expect("world failed to run");
    }
}
