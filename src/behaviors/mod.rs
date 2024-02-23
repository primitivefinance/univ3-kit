use arbiter_engine::machine::*;
use arbiter_macros::Behaviors;
use serde::{Deserialize, Serialize};

pub mod deployer;
use deployer::Deployer;

#[derive(Behaviors, Debug, Serialize, Deserialize)]
pub enum Behaviors {
    Deployer(Deployer),
}
