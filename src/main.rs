use arbiter_engine::{agent::Agent, world::World};

pub mod behaviors;
pub mod bindings;

use crate::behaviors::Deployer;

#[tokio::main]
async fn main() {
    let mut world = World::new("univ3");

    let deployer = Agent::builder("deployer").with_behavior(Deployer);

    world.add_agent(deployer);
    world.run().await;
}
