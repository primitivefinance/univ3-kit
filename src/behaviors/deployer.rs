use std::sync::Arc;

use anyhow::Result;
use arbiter_core::middleware::ArbiterMiddleware;
use arbiter_engine::{
    machine::{Behavior, EventStream},
    messager::{Messager, To},
};
use ethers::types::H160;

use super::*;
use crate::bindings::{token::ArbiterToken, uniswap_v3_factory::UniswapV3Factory};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DeploymentData {
    token_0: H160,
    token_1: H160,
    factory: H160,
    pool: H160,
}

impl DeploymentData {
    pub fn new(token_0: H160, token_1: H160, factory: H160, pool: H160) -> Self {
        Self {
            token_0,
            token_1,
            factory,
            pool,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Deployer;

#[async_trait::async_trait]
impl Behavior<()> for Deployer {
    async fn startup(
        &mut self,
        client: Arc<ArbiterMiddleware>,
        messager: Messager,
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

        // Construct a data object that all other behaviours will recieve containing requisites for simulation.
        let deployment_data = DeploymentData::new(
            token_0.address(),
            token_1.address(),
            factory.address(),
            pool,
        );

        let _ = messager
            .send(To::All, serde_json::to_string(&deployment_data)?)
            .await;

        Ok(None)
    }
}
