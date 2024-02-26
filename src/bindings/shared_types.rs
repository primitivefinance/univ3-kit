///`CollectParams(uint256,address,uint128,uint128)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct CollectParams {
    pub token_id: ::ethers::core::types::U256,
    pub recipient: ::ethers::core::types::Address,
    pub amount_0_max: u128,
    pub amount_1_max: u128,
}
///`DecreaseLiquidityParams(uint256,uint128,uint256,uint256,uint256)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct DecreaseLiquidityParams {
    pub token_id: ::ethers::core::types::U256,
    pub liquidity: u128,
    pub amount_0_min: ::ethers::core::types::U256,
    pub amount_1_min: ::ethers::core::types::U256,
    pub deadline: ::ethers::core::types::U256,
}
///`IncreaseLiquidityParams(uint256,uint256,uint256,uint256,uint256,uint256)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct IncreaseLiquidityParams {
    pub token_id: ::ethers::core::types::U256,
    pub amount_0_desired: ::ethers::core::types::U256,
    pub amount_1_desired: ::ethers::core::types::U256,
    pub amount_0_min: ::ethers::core::types::U256,
    pub amount_1_min: ::ethers::core::types::U256,
    pub deadline: ::ethers::core::types::U256,
}
///`MintParams(address,address,uint24,int24,int24,uint256,uint256,uint256,uint256,address,uint256)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct MintParams {
    pub token_0: ::ethers::core::types::Address,
    pub token_1: ::ethers::core::types::Address,
    pub fee: u32,
    pub tick_lower: i32,
    pub tick_upper: i32,
    pub amount_0_desired: ::ethers::core::types::U256,
    pub amount_1_desired: ::ethers::core::types::U256,
    pub amount_0_min: ::ethers::core::types::U256,
    pub amount_1_min: ::ethers::core::types::U256,
    pub recipient: ::ethers::core::types::Address,
    pub deadline: ::ethers::core::types::U256,
}
///`QuoteExactInputSingleParams(address,address,uint256,uint24,uint160)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct QuoteExactInputSingleParams {
    pub token_in: ::ethers::core::types::Address,
    pub token_out: ::ethers::core::types::Address,
    pub amount_in: ::ethers::core::types::U256,
    pub fee: u32,
    pub sqrt_price_limit_x96: ::ethers::core::types::U256,
}
///`QuoteExactOutputSingleParams(address,address,uint256,uint24,uint160)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct QuoteExactOutputSingleParams {
    pub token_in: ::ethers::core::types::Address,
    pub token_out: ::ethers::core::types::Address,
    pub amount: ::ethers::core::types::U256,
    pub fee: u32,
    pub sqrt_price_limit_x96: ::ethers::core::types::U256,
}
///`ExactInputParams(bytes,address,uint256,uint256,uint256)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct ExactInputParams {
    pub path: ::ethers::core::types::Bytes,
    pub recipient: ::ethers::core::types::Address,
    pub deadline: ::ethers::core::types::U256,
    pub amount_in: ::ethers::core::types::U256,
    pub amount_out_minimum: ::ethers::core::types::U256,
}
///`ExactInputSingleParams(address,address,uint24,address,uint256,uint256,uint256,uint160)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct ExactInputSingleParams {
    pub token_in: ::ethers::core::types::Address,
    pub token_out: ::ethers::core::types::Address,
    pub fee: u32,
    pub recipient: ::ethers::core::types::Address,
    pub deadline: ::ethers::core::types::U256,
    pub amount_in: ::ethers::core::types::U256,
    pub amount_out_minimum: ::ethers::core::types::U256,
    pub sqrt_price_limit_x96: ::ethers::core::types::U256,
}
///`ExactOutputParams(bytes,address,uint256,uint256,uint256)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct ExactOutputParams {
    pub path: ::ethers::core::types::Bytes,
    pub recipient: ::ethers::core::types::Address,
    pub deadline: ::ethers::core::types::U256,
    pub amount_out: ::ethers::core::types::U256,
    pub amount_in_maximum: ::ethers::core::types::U256,
}
///`ExactOutputSingleParams(address,address,uint24,address,uint256,uint256,uint256,uint160)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct ExactOutputSingleParams {
    pub token_in: ::ethers::core::types::Address,
    pub token_out: ::ethers::core::types::Address,
    pub fee: u32,
    pub recipient: ::ethers::core::types::Address,
    pub deadline: ::ethers::core::types::U256,
    pub amount_out: ::ethers::core::types::U256,
    pub amount_in_maximum: ::ethers::core::types::U256,
    pub sqrt_price_limit_x96: ::ethers::core::types::U256,
}
///`PopulatedTick(int24,int128,uint128)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct PopulatedTick {
    pub tick: i32,
    pub liquidity_net: i128,
    pub liquidity_gross: u128,
}
///`MigrateParams(address,uint256,uint8,address,address,uint24,int24,int24,uint256,uint256,address,uint256,bool)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct MigrateParams {
    pub pair: ::ethers::core::types::Address,
    pub liquidity_to_migrate: ::ethers::core::types::U256,
    pub percentage_to_migrate: u8,
    pub token_0: ::ethers::core::types::Address,
    pub token_1: ::ethers::core::types::Address,
    pub fee: u32,
    pub tick_lower: i32,
    pub tick_upper: i32,
    pub amount_0_min: ::ethers::core::types::U256,
    pub amount_1_min: ::ethers::core::types::U256,
    pub recipient: ::ethers::core::types::Address,
    pub deadline: ::ethers::core::types::U256,
    pub refund_as_eth: bool,
}
