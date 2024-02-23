#[allow(dead_code)]
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
        println!("Prior to token deployment.");
        let token_0 =
            ArbiterToken::deploy(client.clone(), ("Token 0".to_owned(), "TKN".to_owned(), 18))?
                .send()
                .await?;
        println!("Deployed token 0");
        let token_1 = ArbiterToken::deploy(
            client.clone(),
            ("Token 1".to_owned(), "TKN0".to_owned(), 18),
        )?
        .send()
        .await?;
        println!("Deployed tokens");
        let factory = deploy_factory(&client).await?;
        let liquid_exchange = deploy_liquid_exchange(&client).await?;

        let pool = create_pool(&factory, token_0.address(), token_1.address()).await?;
        println!("Got here.");

        let deployment_data = DeploymentData {
            token_0: token_0.address(),
            token_1: token_1.address(),
            factory: factory.address(),
            liquid_exchange: liquid_exchange.address(),
            pool,
        };

        println!("Deployment data: {:?}", deployment_data);
        messager
            .send(To::All, serde_json::to_string(&deployment_data)?)
            .await?;
        debug!("Sent deployment data: {:?}", deployment_data);

        Ok(None)
    }
}

async fn deploy_token(
    client: Arc<ArbiterMiddleware>,
    name: &str,
    symbol: &str,
) -> Result<ArbiterToken<ArbiterMiddleware>> {
    println!("In here.");
    let thing = ArbiterToken::deploy(
        client.clone(),
        (String::from(name), String::from(symbol), 18),
    )?
    .send()
    .await;
    println!("Thing: {:?}", thing);

    Ok(thing?)
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
