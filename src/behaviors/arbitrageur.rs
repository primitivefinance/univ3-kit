use std::sync::Arc;

use anyhow::{anyhow, Result};
use arbiter_core::middleware::ArbiterMiddleware;
use arbiter_engine::messager::{Message, Messager};
use ethers::types::H160;

use super::*;
use crate::bindings::uniswap_v3_factory::UniswapV3Factory;

#[derive(Serialize, Deserialize)]
struct Arbitrageur {
    pub liquid_exchange: Option<H160>,
    pub client: Option<Arc<ArbiterMiddleware>>,
}

#[async_trait::async_trait]
impl Behavior<Message> for Arbitrageur {
    async fn startup(
        &mut self,
        _client: Arc<ArbiterMiddleware>,
        messager: Messager,
    ) -> Result<Option<EventStream<Message>>> {
        while let Some(message) = messager.clone().stream().unwrap().next().await {
            match serde_json::from_str::<DeploymentData>(&message.data) {
                Ok(data) => {
                    self.liquid_exchange = Some(data.liquid_exchange);
                    break;
                }
                Err(_) => continue,
            };
        }

        

        self.client = Some(client.clone());

        Ok(Some(messager.clone().stream().unwrap()))
    }

    async fn process(&mut self, event: Message) -> Result<ControlFlow> {
        let _query: PriceUpdate = match serde_json::from_str(&event.data) {
            Ok(query) => query,
            Err(_) => {
                eprintln!("Failed to deserialize the event data into a PriceUpdate");
                return Ok(ControlFlow::Continue);
            }
        };

        let liquid_exchange =
            LiquidExchange::new(self.liquid_exchange.unwrap(), self.client.clone().unwrap());

        let liquid_exchange_price = liquid_exchange.price().send()
            .await?
            .await?;

        
    }
}