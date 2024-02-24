use std::{sync::Arc};
use std::collections::HashMap;

use anyhow::{anyhow, Result};
use arbiter_core::middleware::ArbiterMiddleware;
use arbiter_engine::messager::{Message, Messager};
use ethers::types::H160;

use super::*;
use crate::bindings::uniswap_v3_pool::UniswapV3Pool;
use crate::bindings::v3_migrator::V3MigratorCalls::NonfungiblePositionManager;

#[derive(Debug, Serialize, Deserialize)]
pub struct LiquidityAdmin {
    pub pool_addresses: Option<HashMap<String, H160>>,
    pub pools: Option<HashMap<H160, UniswapV3Pool<ArbiterMiddleware>>>,
    #[serde(skip)]
    pub messager: Option<Messager>,
    #[serde(skip)]
    pub client: Option<Arc<ArbiterMiddleware>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum LiquidityAdminQuery {
    PositionCreation(PositionCreation),
    PositionIncrease(PositionIncrease),
    PositionDecrease(PositionDecrease)
}

/*
struct MintParams {
        address token0;
        address token1;
        uint24 fee;
        int24 tickLower;
        int24 tickUpper;
        uint256 amount0Desired;
        uint256 amount1Desired;
        uint256 amount0Min;
        uint256 amount1Min;
        address recipient;
        uint256 deadline;
    }
*/

#[derive(Debug, Deserialize, Serialize)]
pub struct PositionCreation {
    pool: H160,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PositionIncrease {
    pool: H160
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PositionDecrease {
    pool: H160
}


#[async_trait::async_trait]
impl Behavior<()> for LiquidityAdmin {
    async fn startup(
        &mut self,
        client: Arc<ArbiterMiddleware>,
        messager: Messager,
    ) -> Result<Option<EventStream<Message>>> {
        self.client = Some(client.clone());
        self.messager = Some(messager.clone());
        Ok(None)
    }

    async fn process(
        &mut self, event: Message
    ) -> Result<ControlFlow> {
        let query: PositionCreation = match serde_json::from_str(&event.data) {
            Ok(query) => query,
            Err(_) => {
                eprintln!("Failed to deserialize the event data into a PositionCreation query");
                return Ok(ControlFlow::Continue);
            }
        };

        match query {
            LiquidityAdminQuery::PositionCreation(position_creation) => {
                let _ = NonfungiblePositionManager::new(position_creation.pool, self.client.clone().unwrap())
                    .mint(
                        position_creation.pool
                    )
                    .call()
                    .await
                    .map_err(|e| anyhow!("Failed to create pool: {}", e));

                Ok(ControlFlow::Continue)
            }
        }
    }
}