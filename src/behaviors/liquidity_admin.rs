use std::{sync::Arc};
use std::collections::HashMap;

use anyhow::Result;
use arbiter_core::middleware::ArbiterMiddleware;
use arbiter_engine::messager::{Message, Messager};
use ethers::types::H160;

use super::*;
use crate::bindings::uniswap_v3_pool::UniswapV3Pool;

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
    /// Mint position request.
    PositionCreation(PositionCreation),
    PositionIncrease(PositionIncrease),
    PositionDecrease(PositionDecrease)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PositionCreation {
    pool: H160,
    token_0: H160,
    token_1: H160,
    fee: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PositionIncrease {

}

#[derive(Debug, Deserialize, Serialize)]
pub struct PositionDecrease {

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

    }
}