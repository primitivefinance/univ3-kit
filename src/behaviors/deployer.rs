#![allow(dead_code)]
use std::sync::Arc;

use anyhow::{anyhow, Result};
use arbiter_bindings::bindings::liquid_exchange::LiquidExchange;
use arbiter_bindings::bindings::weth::WETH;
use arbiter_core::middleware::ArbiterMiddleware;
use arbiter_engine::{
    machine::{Behavior, EventStream},
    messager::{Messager, To},
};
use ethers::abi::Bytes;
use ethers::prelude::Client;
use ethers::types::{Address, H160};
use crate::bindings::{
    nonfungible_position_manager::NonfungiblePositionManager,
    nonfungible_token_position_descriptor::NonfungibleTokenPositionDescriptor
};

use super::*;
use crate::bindings::uniswap_v3_factory::UniswapV3Factory;

#[derive(Debug, Deserialize, Serialize)]
pub struct DeploymentData {
    factory: H160,
    liquid_exchange: H160,
    weth: H160,
    position_manager: H160,
}

impl DeploymentData {
    pub fn new(factory: H160, liquid_exchange: H160, weth: H160, position_manager: H160) -> Self {
        Self {
            factory,
            liquid_exchange,
            weth,
            position_manager
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
        let weth = deploy_weth(&client).await?;
        let token_descriptor = deploy_token_descriptor(&client).await?;
        let position_manager =
            deploy_position_manager(&client, factory.address(), weth.address(), ).await?;

        let deployment_data = DeploymentData {
            factory: factory.address(),
            liquid_exchange: liquid_exchange.address(),
            weth: weth.address(),
            position_manager: position_manager.address()
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

async fn deploy_weth(
    client: &Arc<ArbiterMiddleware>,
) -> Result<WETH<ArbiterMiddleware>> {
    WETH::deploy(client.clone(), ())
        .map_err(|e| anyhow!("Failed to deploy liquid exchange: {}", e))?
        .send()
        .await
        .map_err(|e| anyhow!("Failed to deploy liquid exchange: {}", e))
}

async fn deploy_token_descriptor(
    client: &Arc<ArbiterMiddleware>,
    weth_address: Address,
    native_currency_label_bytes: Bytes
) -> Result<NonfungibleTokenPositionDescriptor<ArbiterMiddleware>> {

}

async fn deploy_position_manager(
    client: &Arc<ArbiterMiddleware>,
    factory_address: Address,
    weth_address: Address,
    token_descriptor_address: Address
) -> Result<NonfungiblePositionManager<ArbiterMiddleware>> {
    NonfungiblePositionManager::deploy(client.clone(), (factory_address, weth_address, token_descriptor_address))
        .map_err(|e| anyhow!("Failed to deploy liquid exchange: {}", e))?
        .send()
        .await
        .map_err(|e| anyhow!("Failed to deploy liquid exchange: {}", e))
}
