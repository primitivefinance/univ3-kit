use std::sync::Arc;
use anyhow::anyhow;
use anyhow::Result;
use arbiter_core::middleware::ArbiterMiddleware;
use arbiter_engine::{
    machine::{Behavior, EventStream},
    messager::{Messager, To},
};

use ethers::types::H160;

use super::*;
use crate::bindings::{token::ArbiterToken, uniswap_v3_factory::UniswapV3Factory};

#[derive(Debug, Deserialize, Serialize)]
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

#[derive(Debug, Deserialize, Serialize)]
pub struct Deployer;

#[async_trait::async_trait]
impl Behavior<()> for Deployer {
    async fn startup(
        &mut self,
        client: Arc<ArbiterMiddleware>,
        messager: Messager,
    ) -> Result<Option<EventStream<()>>> {
        let token_0 = self.deploy_token(&client, "Token 0", "0").await?;
        let token_1 = self.deploy_token(&client, "Token 1", "1").await?;
        let factory = self.deploy_factory(&client).await?;
        let pool = self.create_pool(&factory, token_0.address(), token_1.address()).await?;

        let deployment_data = DeploymentData {
            token_0: token_0.address(),
            token_1: token_1.address(),
            factory: factory.address(),
            pool: pool,
        };

        messager.send(To::All, serde_json::to_string(&deployment_data)?).await;

        Ok(None)
    }
}

impl Deployer {
    async fn deploy_token(
        &self,
        client: &Arc<ArbiterMiddleware>,
        name: &str,
        symbol: &str,
    ) -> Result<ArbiterToken<ArbiterMiddleware>> {
        ArbiterToken::deploy(
            client.clone(),
            (String::from(name), String::from(symbol), 18),
        )
        .map_err(|e| anyhow!("Failed to deploy token {}: {}", name, e))?
        .send()
        .await
        .map_err(|e| anyhow!("Failed to send token {}: {}", name, e))
    }

    async fn deploy_factory(
        &self,
        client: &Arc<ArbiterMiddleware>,
    ) -> Result<UniswapV3Factory<ArbiterMiddleware>> {
        UniswapV3Factory::deploy(client.clone(), ())
            .map_err(|e| anyhow!("Failed to deploy factory: {}", e))?
            .send()
            .await
            .map_err(|e| anyhow!("Failed to send factory deployment: {}", e))
    }

     async fn create_pool<M>(
        &self,
        factory: &UniswapV3Factory<M>,
        token_0: H160,
        token_1: H160,
    ) -> Result<H160> where M: ethers::providers::Middleware  {
        factory
            .create_pool(token_0, token_1, 100)
            .call()
            .await
            .map_err(|e| anyhow!("Failed to create pool: {}", e))
    }
}
