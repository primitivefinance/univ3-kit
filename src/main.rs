pub mod behaviors;
pub mod bindings;

use crate::behaviors::Behaviors;

/// To run this example, you can do the following from the project root directory:
/// ```sh
/// cargo run simulate configs/example.toml
/// ```
/// If you would like to see more detailed logs, you can run the following:
/// ```sh
/// cargo run simulate configs/example.toml -vvv
/// ```
/// to get `debug` level logs (the default with no verbosity is `ERROR`).
///
/// By running
/// ```sh
/// cargo run
/// ```
/// you will get the `--help` message for the project.
#[arbiter_macros::main(
    name = "Uniswap V3 Simulation",
    about = "Simulating Uniswap V3 with Arbiter",
    behaviors = Behaviors
)]
async fn main() {}
