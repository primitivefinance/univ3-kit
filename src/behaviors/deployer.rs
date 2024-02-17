use std::sync::Arc;

use anyhow::Result;
use arbiter_core::middleware::ArbiterMiddleware;
use arbiter_engine::{
    machine::{Behavior, EventStream},
    messager::Messager,
};

use super::*;
use crate::bindings::{token::ArbiterToken, uniswap_v3_factory::UniswapV3Factory};

#[derive(Debug, Deserialize, Serialize)]
pub struct Deployer;

#[async_trait::async_trait]
impl Behavior<()> for Deployer {
    async fn startup(
        &mut self,
        client: Arc<ArbiterMiddleware>,
        _messager: Messager,
    ) -> Result<Option<EventStream<()>>> {
        let token_0 = ArbiterToken::deploy(
            client.clone(),
            (String::from("Token 0"), String::from("0"), 18),
        )?
        .send()
        .await?;
        
        let token_1 = ArbiterToken::deploy(
            client.clone(),
            (String::from("Token 1"), String::from("1"), 18),
        )?
        .send()
        .await?;

        let factory = UniswapV3Factory::deploy(client, ())?.send().await?;

        let pool = factory
            .create_pool(token_0.address(), token_1.address(), 100)
            .call()
            .await?;

        Ok(None)
    }
}
