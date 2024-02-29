use std::sync::Arc;

use anyhow::Result;
use arbiter_core::middleware::ArbiterMiddleware;
use arbiter_engine::messager::{Message, Messager};

use super::*;
use crate::behaviors::price_changer::PriceUpdate;

#[derive(Debug, Deserialize, Serialize)]
pub struct Terminator {
    pub steps: usize,
    pub current_step: usize,
}

#[async_trait::async_trait]
impl Behavior<Message> for Terminator {
    async fn startup(
        &mut self,
        _client: Arc<ArbiterMiddleware>,
        messager: Messager,
    ) -> Result<Option<EventStream<Message>>> {
        Ok(Some(messager.clone().stream().unwrap()))
    }

    async fn process(&mut self, event: Message) -> Result<ControlFlow> {
        if self.current_step >= self.steps {
            return Ok(ControlFlow::Halt);
        }

        let _query: PriceUpdate = match serde_json::from_str::<PriceUpdate>(&event.data) {
            Ok(_) => {
                self.current_step += 1;
                return Ok(ControlFlow::Continue);
            }
            Err(_) => {
                return Ok(ControlFlow::Continue);
            }
        };
    }
}
