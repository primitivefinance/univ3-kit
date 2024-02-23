use arbiter_engine::machine::*;
use arbiter_macros::Behaviors;
use serde::{Deserialize, Serialize};

pub mod deployer;
pub mod pool_admin;
pub mod token_admin;

use deployer::Deployer;
use pool_admin::PoolAdmin;
use token_admin::TokenAdmin;

#[derive(Behaviors, Debug, Serialize, Deserialize)]
pub enum Behaviors {
    Deployer(Deployer),
    TokenAdmin(TokenAdmin),
    PoolAdmin(PoolAdmin),
}
