#![allow(dead_code)]
use std::sync::Arc;

use anyhow::{anyhow, Result};
use arbiter_bindings::bindings::{arbiter_token::ArbiterToken, liquid_exchange::LiquidExchange};
use arbiter_core::middleware::ArbiterMiddleware;
use arbiter_engine::{
    machine::{Behavior, EventStream},
    messager::{Messager, To},
};
use ethers::types::{H160, U256};
use tracing::info;

use self::token_admin::TokenData;
use super::*;
use crate::bindings::uniswap_v3_factory::UniswapV3Factory;

#[derive(Debug, Deserialize, Serialize)]
pub struct DeploymentData {
    pub factory: H160,
    pub liquid_exchange: H160,
}

impl DeploymentData {
    pub fn new(factory: H160, liquid_exchange: H160) -> Self {
        Self {
            factory,
            liquid_exchange,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LiquidExchangeParameters {
    pub asset_token_parameters: TokenData,
    pub quote_token_parameters: TokenData,
    pub initial_price: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Deployer {
    pub liquid_exchange_parameters: LiquidExchangeParameters,
}

#[async_trait::async_trait]
impl Behavior<()> for Deployer {
    async fn startup(
        &mut self,
        client: Arc<ArbiterMiddleware>,
        messager: Messager,
    ) -> Result<Option<EventStream<()>>> {
        let factory = deploy_factory(&client).await?;

        let le_param = self.liquid_exchange_parameters.clone();
        let liquid_exchange = deploy_liquid_exchange(
            &client,
            &le_param.asset_token_parameters,
            &le_param.quote_token_parameters,
            le_param.initial_price,
        )
        .await?;
        info!("Factory deployed at {:?}", factory.address());
        info!(
            "Liquid exchange deployed at {:?}",
            liquid_exchange.address()
        );
        info!(
            "Liquid exchange initial price : {:?}",
            liquid_exchange.price().call().await?
        );

        let deployment_data = DeploymentData {
            factory: factory.address(),
            liquid_exchange: liquid_exchange.address(),
        };

        messager.send(To::All, &deployment_data).await?;

        Ok(None)
    }
}

pub async fn deploy_factory(
    client: &Arc<ArbiterMiddleware>,
) -> Result<UniswapV3Factory<ArbiterMiddleware>> {
    UniswapV3Factory::deploy(client.clone(), ())
        .map_err(|e| anyhow!("Failed to deploy factory: {}", e))?
        .send()
        .await
        .map_err(|e| anyhow!("Failed to send factory deployment: {}", e))
}

pub async fn deploy_liquid_exchange(
    client: &Arc<ArbiterMiddleware>,
    arbx_param: &TokenData,
    arby_param: &TokenData,
    initial_price: f64,
) -> Result<LiquidExchange<ArbiterMiddleware>> {
    let initial_liquid_exchange_price = U256::from((initial_price * 10f64.powf(18.0)) as u64);

    // Deploy an instance of the `ArbiterToken` contract using our predefined constants.
    let arbx = ArbiterToken::deploy(
        client.clone(),
        (
            arbx_param.name.clone(),
            arbx_param.symbol.clone(),
            arbx_param.decimals,
        ),
    )?
    .send()
    .await?;
    info!("Arbiter Token X contract deployed at {:?}", arbx.address());

    // Deploy the second instance of the `ArbiterToken` contract using our predefined constants.
    let arby = ArbiterToken::deploy(
        client.clone(),
        (
            arby_param.name.clone(),
            arby_param.symbol.clone(),
            arby_param.decimals,
        ),
    )?
    .send()
    .await?;
    info!("Arbiter Token Y contract deployed at {:?}", arby.address());

    LiquidExchange::deploy(
        client.clone(),
        (
            arbx.address(),
            arby.address(),
            initial_liquid_exchange_price,
        ),
    )
    .map_err(|e| anyhow!("Failed to deploy liquid exchange: {}", e))?
    .send()
    .await
    .map_err(|e| anyhow!("Failed to send liquid exchange: {}", e))
}
