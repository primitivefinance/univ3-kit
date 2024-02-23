#![allow(dead_code)]
use std::sync::Arc;

use anyhow::{anyhow, Result};
use arbiter_bindings::bindings::liquid_exchange::LiquidExchange;
use arbiter_core::middleware::ArbiterMiddleware;
use arbiter_engine::{
    machine::{Behavior, EventStream},
    messager::{Messager, To},
};
use ethers::types::H160;

use super::*;
use crate::bindings::uniswap_v3_factory::UniswapV3Factory;

#[derive(Debug, Deserialize, Serialize)]
pub struct DeploymentData {
    factory: H160,
    liquid_exchange: H160,
}

impl DeploymentData {
    pub fn new(factory: H160, liquid_exchange: H160) -> Self {
        Self {
            factory,
            liquid_exchange,
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
        let factory = deploy_factory(&client).await?;
        let liquid_exchange = deploy_liquid_exchange(&client).await?;

        let deployment_data = DeploymentData {
            factory: factory.address(),
            liquid_exchange: liquid_exchange.address(),
        };

        messager
            .send(To::All, serde_json::to_string(&deployment_data)?)
            .await?;

        Ok(None)
    }
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
