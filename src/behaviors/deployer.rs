use std::sync::Arc;

use anyhow::Result;
use arbiter_core::{middleware::ArbiterMiddleware};
use arbiter_engine::{
    machine::{Behavior, EventStream},
    messager::Messager,
};

use super::*;

use crate::bindings::uniswap_v3_factory::UniswapV3Factory;

#[derive(Debug, Deserialize, Serialize)]
pub struct Deployer;

#[async_trait::async_trait]
impl Behavior<()> for Deployer {
    async fn startup(
        &mut self,
        client: Arc<ArbiterMiddleware>,
        _messager: Messager,
    ) -> Result<Option<EventStream<()>>> {
        UniswapV3Factory::deploy(client, ()).unwrap().send().await.unwrap();

        Ok(None)
    }
}