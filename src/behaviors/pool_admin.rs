use std::{collections::HashMap, sync::Arc};

use anyhow::{anyhow, Result};
use arbiter_core::middleware::ArbiterMiddleware;
use arbiter_engine::messager::{Message, Messager, To};
use ethers::types::H160;

use super::*;
use crate::bindings::uniswap_v3_factory::UniswapV3Factory;

#[derive(Debug, Serialize, Deserialize)]
pub struct PoolAdmin {
    #[serde(skip)]
    pub messager: Option<Messager>,

    #[serde(skip)]
    pub client: Option<Arc<ArbiterMiddleware>>,
}

/// Used as an action to ask what tokens are available.
#[derive(Debug, Deserialize, Serialize)]
pub enum PoolAdminQuery {
    /// Deploy request.
    PoolCreation(PoolCreation),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PoolCreation {
    factory: H160,
    token_0: H160,
    token_1: H160,
    fee: u32,
}

#[async_trait::async_trait]
impl Behavior<Message> for PoolAdmin {
    async fn startup(
        &mut self,
        client: Arc<ArbiterMiddleware>,
        messager: Messager,
    ) -> Result<Option<EventStream<Message>>> {
        self.client = Some(client.clone());
        self.messager = Some(messager.clone());

        Ok(None)
    }

    async fn process(&mut self, event: Message) -> Result<ControlFlow> {
        let query: PoolAdminQuery = match serde_json::from_str(&event.data) {
            Ok(query) => query,
            Err(_) => {
                eprintln!("Failed to deserialize the event data into a PoolAdminQuery");
                return Ok(ControlFlow::Continue);
            }
        };

        match query {
            PoolAdminQuery::PoolCreation(pool_creation) => {
                UniswapV3Factory::new(pool_creation.factory, self.client.clone().unwrap())
                    .create_pool(
                        pool_creation.token_0,
                        pool_creation.token_1,
                        pool_creation.fee,
                    )
                    .call()
                    .await
                    .map_err(|e| anyhow!("Failed to create pool: {}", e));

                Ok(ControlFlow::Continue)
            }
        }
    }
}
