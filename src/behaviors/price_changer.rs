use std::sync::Arc;

use anyhow::Result;
use arbiter_core::middleware::ArbiterMiddleware;
use arbiter_engine::messager::{Message, Messager};
use ethers::types::H160;
use super::*;
use crate::bindings::liquid_exchange::LiquidExchange;

use RustQuant::stochastics::*;
use RustQuant::models::*;
use RustQuant::stochastics::process::Trajectories;

use std::fmt;

#[derive(Serialize, Deserialize)]
pub struct PriceChanger {
    #[serde(skip)]
    pub params: OrnsteinUhlenbeckParams,

    #[serde(skip)]
    #[serde(default = "trajectory_default")]
    pub current_chunk: Trajectories,

    #[serde(skip)]
    pub client: Option<Arc<ArbiterMiddleware>>,

    cursor: usize,
    value: f64,
}

fn trajectory_default() -> Trajectories {
    Trajectories {
        times: Vec::new(),
        paths: Vec::new(),
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct OrnsteinUhlenbeckParams {
    mu: f64, 
    sigma: f64, 
    theta: f64,
}

impl fmt::Debug for PriceChanger {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PriceUpdate {
    liquid_exchange: H160,
}

impl PriceChanger {
    /// Public constructor function to create a [`PriceChanger`] behaviour.
    pub fn new(initial_value: f64, params: OrnsteinUhlenbeckParams) -> Self {
        let ou = OrnsteinUhlenbeck::new(params.mu, params.sigma, params.theta);

        // Chunk our price trajectory over 100 price points.
        let current_chunk = ou.euler_maruyama(initial_value, 0.0, 100.0, 100_usize, 1_usize, false);

        Self {
            params,
            current_chunk,
            cursor: 0,
            client: None,
            value: initial_value,
        }
    }
}

#[async_trait::async_trait]
impl Behavior<Message> for PriceChanger {
    async fn startup(
        &mut self,
        _client: Arc<ArbiterMiddleware>,
        _messager: Messager,
    ) -> Result<Option<EventStream<Message>>> {
        Ok(None)
    }

    async fn process(&mut self, event: Message) -> Result<ControlFlow> {
        let ou = OrnsteinUhlenbeck::new(self.params.mu, self.params.sigma, self.params.theta);

        let query: PriceUpdate = match serde_json::from_str(&event.data) {
            Ok(query) => query,
            Err(_) => {
                eprintln!("Failed to deserialize the event data into a PriceUpdate");
                return Ok(ControlFlow::Continue);
            }
        };

        if self.cursor >= 99 {
            self.cursor = 0;
            self.value = self.current_chunk.paths.clone()[0][self.cursor];
            self.current_chunk = ou.euler_maruyama(self.value, 0.0, 100.0, 100_usize, 1_usize, false);
        }

        let liquid_exchange = LiquidExchange::new(query.liquid_exchange, self.client.clone().unwrap());

        let price = self.current_chunk.paths.clone()[0][self.cursor];

        liquid_exchange
            .set_price(ethers::utils::parse_ether(price)?)
            .send()
            .await?
            .await?;

        self.cursor += 1;

        Ok(ControlFlow::Continue)
    }
}
