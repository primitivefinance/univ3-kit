use std::{fmt, sync::Arc};

use anyhow::Result;
use arbiter_core::middleware::ArbiterMiddleware;
use arbiter_engine::messager::{Message, Messager};
use ethers::types::H160;
use RustQuant::{
    models::*,
    stochastics::{process::Trajectories, *},
};

use super::*;
use crate::bindings::liquid_exchange::LiquidExchange;

#[derive(Serialize, Deserialize)]
pub struct PriceChanger {
    mu: f64,
    sigma: f64,
    theta: f64,

    seed: u64,

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
    pub fn new(initial_value: f64, mu: f64, sigma: f64, theta: f64, seed: u64) -> Self {
        let ou = OrnsteinUhlenbeck::new(mu, sigma, theta);

        // Chunk our price trajectory over 100 price points.
        let current_chunk =
            ou.seedable_euler_maruyama(initial_value, 0.0, 100.0, 100_usize, 1_usize, false, seed);

        Self {
            mu,
            sigma,
            theta,
            seed,
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
        client: Arc<ArbiterMiddleware>,
        _messager: Messager,
    ) -> Result<Option<EventStream<Message>>> {
        self.client = Some(client);

        Ok(None)
    }

    async fn process(&mut self, event: Message) -> Result<ControlFlow> {
        let ou = OrnsteinUhlenbeck::new(self.mu, self.sigma, self.theta);

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
            self.current_chunk = ou.seedable_euler_maruyama(
                self.value, 0.0, 100.0, 100_usize, 1_usize, false, self.seed,
            );
        }

        let liquid_exchange =
            LiquidExchange::new(query.liquid_exchange, self.client.clone().unwrap());

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
