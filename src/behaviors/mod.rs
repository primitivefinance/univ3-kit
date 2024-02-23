use arbiter_engine::machine::*;
use arbiter_macros::Behaviors;
use serde::{Deserialize, Serialize};

pub mod deployer;
pub mod token_admin;
pub mod pool_admin;

use deployer::Deployer;
use token_admin::TokenAdmin;
use pool_admin::PoolAdmin;

#[derive(Behaviors, Debug, Serialize, Deserialize)]
pub enum Behaviors {
    Deployer(Deployer),
    TokenAdmin(TokenAdmin),
    PoolAdmin(PoolAdmin),
}
