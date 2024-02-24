use arbiter_engine::machine::*;
use arbiter_macros::Behaviors;
use serde::{Deserialize, Serialize};

pub mod deployer;
pub mod pool_admin;
pub mod token_admin;
pub mod price_changer;

use deployer::Deployer;
use pool_admin::PoolAdmin;
use token_admin::TokenAdmin;
use price_changer::PriceChanger;

#[derive(Behaviors, Debug, Serialize, Deserialize)]
pub enum Behaviors {
    Deployer(Deployer),
    TokenAdmin(TokenAdmin),
    PoolAdmin(PoolAdmin),
    PriceChanger(PriceChanger),
}
