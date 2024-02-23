use std::sync::Arc;

use anyhow::{anyhow, Result};
use arbiter_bindings::bindings::{arbiter_token::ArbiterToken, liquid_exchange::LiquidExchange};
use arbiter_core::middleware::ArbiterMiddleware;
use arbiter_engine::{
    machine::{Behavior, EventStream},
    messager::{Messager, To},
};
use ethers::types::H160;
use tracing::debug;

use super::*;
use crate::bindings::uniswap_v3_factory::UniswapV3Factory;

#[derive(Debug, Deserialize, Serialize)]
pub struct DeploymentData {
    token_0: H160,
    token_1: H160,
    factory: H160,
    liquid_exchange: H160,
    pool: H160,
}

impl DeploymentData {
    pub fn new(
        token_0: H160,
        token_1: H160,
        factory: H160,
        liquid_exchange: H160,
        pool: H160,
    ) -> Self {
        Self {
            token_0,
            token_1,
            factory,
            liquid_exchange,
            pool,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Deployer {}

#[async_trait::async_trait]
impl Behavior<()> for Deployer {
    async fn startup(
        &mut self,
        client: Arc<ArbiterMiddleware>,
        messager: Messager,
    ) -> Result<Option<EventStream<()>>> {
        let token_0 = deploy_token(client.clone(), "TOKEN 0", "TKN0").await?;
        let token_1 = deploy_token(client.clone(), "TOKEN 1", "TKN1").await?;

        let factory = deploy_factory(&client).await?;
        let liquid_exchange = deploy_liquid_exchange(&client).await?;

        let pool = create_pool(&factory, token_0.address(), token_1.address()).await?;

        let deployment_data = DeploymentData {
            token_0: token_0.address(),
            token_1: token_1.address(),
            factory: factory.address(),
            liquid_exchange: liquid_exchange.address(),
            pool,
        };

        messager
            .send(To::All, serde_json::to_string(&deployment_data)?)
            .await?;

        Ok(None)
    }
}

async fn deploy_token(
    client: Arc<ArbiterMiddleware>,
    name: &str,
    symbol: &str,
) -> Result<ArbiterToken<ArbiterMiddleware>> {
    let token = ArbiterToken::deploy(
        client.clone(),
        (name.to_string(), symbol.to_string(), 18_u8),
    )?
    .send()
    .await?;

    Ok(token)
}

async fn deploy_factory(
    client: &Arc<ArbiterMiddleware>,
) -> Result<UniswapV3Factory<ArbiterMiddleware>> {
    UniswapV3Factory::deploy(client.clone(), ())
        .map_err(|e| anyhow!("Failed to deploy factory: {}", e))?
        .send()
        .await
        .map_err(|e| anyhow!("Failed to send factory deployment: {}", e))
}

async fn deploy_liquid_exchange(
    client: &Arc<ArbiterMiddleware>,
) -> Result<LiquidExchange<ArbiterMiddleware>> {
    LiquidExchange::deploy(client.clone(), ())
        .map_err(|e| anyhow!("Failed to deploy liquid exchange: {}", e))?
        .send()
        .await
        .map_err(|e| anyhow!("Failed to send liquid exchange: {}", e))
}

async fn create_pool<M>(factory: &UniswapV3Factory<M>, token_0: H160, token_1: H160) -> Result<H160>
where
    M: ethers::providers::Middleware,
{
    factory
        .create_pool(token_0, token_1, 100)
        .call()
        .await
        .map_err(|e| anyhow!("Failed to create pool: {}", e))
}
