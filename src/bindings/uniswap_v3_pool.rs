pub use uniswap_v3_pool::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod uniswap_v3_pool {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("burn"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("burn"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tickLower"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(24usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int24"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tickUpper"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(24usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int24"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint128"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount0"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount1"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("collect"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("collect"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tickLower"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(24usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int24"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tickUpper"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(24usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int24"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount0Requested"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint128"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount1Requested"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint128"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount0"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint128"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount1"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint128"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("collectProtocol"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("collectProtocol"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount0Requested"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint128"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount1Requested"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint128"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount0"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint128"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount1"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint128"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("factory"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("factory"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("fee"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("fee"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint24"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("feeGrowthGlobal0X128"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "feeGrowthGlobal0X128",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("feeGrowthGlobal1X128"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "feeGrowthGlobal1X128",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("flash"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("flash"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount0"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount1"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "increaseObservationCardinalityNext",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "increaseObservationCardinalityNext",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "observationCardinalityNext",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("initialize"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("initialize"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sqrtPriceX96"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        160usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint160"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("liquidity"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("liquidity"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint128"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("maxLiquidityPerTick"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "maxLiquidityPerTick",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint128"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("mint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("mint"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tickLower"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(24usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int24"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tickUpper"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(24usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int24"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint128"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount0"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount1"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("observations"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("observations"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("blockTimestamp"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tickCumulative"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(56usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int56"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "secondsPerLiquidityCumulativeX128",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        160usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint160"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("initialized"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("observe"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("observe"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("secondsAgos"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tickCumulatives"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Int(56usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int56[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "secondsPerLiquidityCumulativeX128s",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(160usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint160[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("positions"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("positions"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("liquidity"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint128"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "feeGrowthInside0LastX128",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "feeGrowthInside1LastX128",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokensOwed0"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint128"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokensOwed1"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint128"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("protocolFees"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("protocolFees"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token0"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint128"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token1"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint128"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setFeeProtocol"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setFeeProtocol"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("feeProtocol0"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("feeProtocol1"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("slot0"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("slot0"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sqrtPriceX96"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        160usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint160"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tick"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(24usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int24"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("observationIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "observationCardinality",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "observationCardinalityNext",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("feeProtocol"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("unlocked"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("snapshotCumulativesInside"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "snapshotCumulativesInside",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tickLower"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(24usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int24"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tickUpper"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(24usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int24"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "tickCumulativeInside",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(56usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int56"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "secondsPerLiquidityInsideX128",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        160usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint160"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("secondsInside"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("swap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("swap"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("zeroForOne"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountSpecified"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sqrtPriceLimitX96"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        160usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint160"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount0"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount1"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("tickBitmap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("tickBitmap"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int16"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("tickSpacing"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("tickSpacing"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(24usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int24"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ticks"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ticks"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(24usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int24"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("liquidityGross"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint128"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("liquidityNet"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int128"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "feeGrowthOutside0X128",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "feeGrowthOutside1X128",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "tickCumulativeOutside",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(56usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int56"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "secondsPerLiquidityOutsideX128",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        160usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint160"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("secondsOutside"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("initialized"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("token0"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("token0"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("token1"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("token1"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Burn"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Burn"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tickLower"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(24usize),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tickUpper"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(24usize),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount0"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount1"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Collect"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Collect"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tickLower"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(24usize),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tickUpper"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(24usize),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount0"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount1"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CollectProtocol"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("CollectProtocol"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount0"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount1"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Flash"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Flash"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount0"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount1"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("paid0"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("paid1"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "IncreaseObservationCardinalityNext",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "IncreaseObservationCardinalityNext",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "observationCardinalityNextOld",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "observationCardinalityNextNew",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Initialize"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Initialize"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sqrtPriceX96"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        160usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tick"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(24usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Mint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Mint"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tickLower"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(24usize),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tickUpper"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(24usize),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount0"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount1"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SetFeeProtocol"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("SetFeeProtocol"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("feeProtocol0Old"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("feeProtocol1Old"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("feeProtocol0New"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("feeProtocol1New"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Swap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Swap"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount0"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount1"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sqrtPriceX96"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        160usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("liquidity"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tick"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(24usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static UNISWAPV3POOL_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x01``@R4\x80\x15b\0\0\x12W`\0\x80\xFD[P0``\x1B`\x80R`@\x80Qc\x08\x905s`\xE4\x1B\x81R\x90Q`\0\x913\x91c\x89\x03W0\x91`\x04\x80\x82\x01\x92`\xA0\x92\x90\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15b\0\0VW`\0\x80\xFD[PZ\xFA\x15\x80\x15b\0\0kW=`\0\x80>=`\0\xFD[PPPP`@Q=`\xA0\x81\x10\x15b\0\0\x82W`\0\x80\xFD[P\x80Q` \x80\x83\x01Q`@\x84\x01Q``\x80\x86\x01Q`\x80\x90\x96\x01Q`\xE8\x96\x87\x1B`\x01`\x01`\xE8\x1B\x03\x19\x16a\x01\0R\x91\x81\x1B`\x01`\x01``\x1B\x03\x19\x90\x81\x16`\xE0R\x92\x81\x1B\x83\x16`\xC0R\x93\x90\x93\x1B\x16`\xA0R`\x02\x82\x81\x0B\x90\x0B\x90\x92\x1Ba\x01 R\x91Pb\0\0\xF7\x90\x82\x90b\0\x01\x0F\x81\x1Bb\0*\xDB\x17\x90\x1CV[`\x80\x1B`\x01`\x01`\x80\x1B\x03\x19\x16a\x01@RPb\0\x01}V[`\0\x80\x82`\x02\x81\x90\x0Bb\r\x89\xE7\x19\x81b\0\x01%W\xFE[\x05\x02\x90P`\0\x83`\x02\x81\x90\x0Bb\r\x89\xE8\x81b\0\x01=W\xFE[\x05\x02\x90P`\0\x84`\x02\x0B\x83\x83\x03`\x02\x0B\x81b\0\x01UW\xFE[\x05`\x01\x01\x90P\x80b\xFF\xFF\xFF\x16`\x01`\x01`\x80\x1B\x03\x80\x16\x81b\0\x01sW\xFE[\x04\x95\x94PPPPPV[`\x80Q``\x1C`\xA0Q``\x1C`\xC0Q``\x1C`\xE0Q``\x1Ca\x01\0Q`\xE8\x1Ca\x01 Q`\xE8\x1Ca\x01@Q`\x80\x1CaUNb\0\x02J`\09\x80a\x1F\xABR\x80aJMR\x80aJ\x84RP\x80a\x0B\xD8R\x80a(\x97R\x80aJ\xB8R\x80aJ\xEARP\x80a\x0C\xC7R\x80a\x19\x88R\x80a\x19\xBFR\x80a(\xDFRP\x80a\x11\x85R\x80a\x1ABR\x80a\x1E\xB1R\x80a#\xE6R\x80a(\xBBR\x80a=|RP\x80a\x08\xAAR\x80a\x12\xB3R\x80a\x1A\x11R\x80a\x1EKR\x80a#`R\x80a<3RP\x80a 8R\x80a\"\x1FR\x80a(sRP\x80a+RRPaUN`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\x8EW`\x005`\xE0\x1C\x80cp\xCFuJ\x11a\0\xDEW\x80c\xC4Z\x01U\x11a\0\x97W\x80c\xDD\xCA?C\x11a\0qW\x80c\xDD\xCA?C\x14a\x07\xD8W\x80c\xF3\x05\x83\x99\x14a\x07\xF8W\x80c\xF3\r\xBA\x93\x14a\x08\0W\x80c\xF67s\x1D\x14a\x08\x82Wa\x01\x8EV[\x80c\xC4Z\x01U\x14a\x07\xA9W\x80c\xD0\xC9:|\x14a\x07\xB1W\x80c\xD2\x12 \xA7\x14a\x07\xD0Wa\x01\x8EV[\x80cp\xCFuJ\x14a\x05\xA0W\x80c\x82\x06\xA4\xD1\x14a\x05\xA8W\x80c\x85\xB6g)\x14a\x05\xD0W\x80c\x88;\xDB\xFD\x14a\x06\rW\x80c\xA3A#\xA7\x14a\x07\x14W\x80c\xA3\x88\x07\xF2\x14a\x07NWa\x01\x8EV[\x80c8P\xC7\xBD\x11a\x01KW\x80cI\x0El\xBC\x11a\x01%W\x80cI\x0El\xBC\x14a\x04LW\x80cO\x1E\xB3\xD8\x14a\x04\xD6W\x80cQN\xA4\xBF\x14a\x05'W\x80cS9\xC2\x96\x14a\x05\x80Wa\x01\x8EV[\x80c8P\xC7\xBD\x14a\x039W\x80c<\x8A}\x8D\x14a\x03\x92W\x80cF\x14\x13\x19\x14a\x042Wa\x01\x8EV[\x80c\r\xFE\x16\x81\x14a\x01\x93W\x80c\x12\x8A\xCB\x08\x14a\x01\xB7W\x80c\x1Ahe\x02\x14a\x02dW\x80c\x1A\xD8\xB0;\x14a\x02\x88W\x80c%,\t\xD7\x14a\x02\xBFW\x80c2\x14\x8Fg\x14a\x03\x16W[`\0\x80\xFD[a\x01\x9Ba\x08\xA8V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[a\x02K`\x04\x806\x03`\xA0\x81\x10\x15a\x01\xCDW`\0\x80\xFD[`\x01`\x01`\xA0\x1B\x03\x825\x81\x16\x92` \x81\x015\x15\x15\x92`@\x82\x015\x92``\x83\x015\x16\x91\x90\x81\x01\x90`\xA0\x81\x01`\x80\x82\x015`\x01` \x1B\x81\x11\x15a\x02\rW`\0\x80\xFD[\x82\x01\x83` \x82\x01\x11\x15a\x02\x1FW`\0\x80\xFD[\x805\x90` \x01\x91\x84`\x01\x83\x02\x84\x01\x11`\x01` \x1B\x83\x11\x17\x15a\x02@W`\0\x80\xFD[P\x90\x92P\x90Pa\x08\xCCV[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x80Q\x91\x82\x90\x03\x01\x90\xF3[a\x02la\x14kV[`@\x80Q`\x01`\x01`\x80\x1B\x03\x90\x92\x16\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[a\x02\x90a\x14zV[`@Q\x80\x83`\x01`\x01`\x80\x1B\x03\x16\x81R` \x01\x82`\x01`\x01`\x80\x1B\x03\x16\x81R` \x01\x92PPP`@Q\x80\x91\x03\x90\xF3[a\x02\xDC`\x04\x806\x03` \x81\x10\x15a\x02\xD5W`\0\x80\xFD[P5a\x14\x94V[`@\x80Qc\xFF\xFF\xFF\xFF\x90\x95\x16\x85R`\x06\x93\x90\x93\x0B` \x85\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x83\x83\x01R\x15\x15``\x83\x01RQ\x90\x81\x90\x03`\x80\x01\x90\xF3[a\x037`\x04\x806\x03` \x81\x10\x15a\x03,W`\0\x80\xFD[P5a\xFF\xFF\x16a\x14\xD9V[\0[a\x03Aa\x15\xD3V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x98\x16\x88R`\x02\x96\x90\x96\x0B` \x88\x01Ra\xFF\xFF\x94\x85\x16\x87\x87\x01R\x92\x84\x16``\x87\x01R\x92\x16`\x80\x85\x01R`\xFF\x90\x91\x16`\xA0\x84\x01R\x15\x15`\xC0\x83\x01RQ\x90\x81\x90\x03`\xE0\x01\x90\xF3[a\x02K`\x04\x806\x03`\xA0\x81\x10\x15a\x03\xA8W`\0\x80\xFD[`\x01`\x01`\xA0\x1B\x03\x825\x16\x91` \x81\x015`\x02\x90\x81\x0B\x92`@\x83\x015\x90\x91\x0B\x91`\x01`\x01`\x80\x1B\x03``\x82\x015\x16\x91\x81\x01\x90`\xA0\x81\x01`\x80\x82\x015`\x01` \x1B\x81\x11\x15a\x03\xF4W`\0\x80\xFD[\x82\x01\x83` \x82\x01\x11\x15a\x04\x06W`\0\x80\xFD[\x805\x90` \x01\x91\x84`\x01\x83\x02\x84\x01\x11`\x01` \x1B\x83\x11\x17\x15a\x04'W`\0\x80\xFD[P\x90\x92P\x90Pa\x16#V[a\x04:a\x18\xDFV[`@\x80Q\x91\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[a\x037`\x04\x806\x03`\x80\x81\x10\x15a\x04bW`\0\x80\xFD[`\x01`\x01`\xA0\x1B\x03\x825\x16\x91` \x81\x015\x91`@\x82\x015\x91\x90\x81\x01\x90`\x80\x81\x01``\x82\x015`\x01` \x1B\x81\x11\x15a\x04\x98W`\0\x80\xFD[\x82\x01\x83` \x82\x01\x11\x15a\x04\xAAW`\0\x80\xFD[\x805\x90` \x01\x91\x84`\x01\x83\x02\x84\x01\x11`\x01` \x1B\x83\x11\x17\x15a\x04\xCBW`\0\x80\xFD[P\x90\x92P\x90Pa\x18\xE5V[a\x02\x90`\x04\x806\x03`\xA0\x81\x10\x15a\x04\xECW`\0\x80\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x16\x90` \x81\x015`\x02\x90\x81\x0B\x91`@\x81\x015\x90\x91\x0B\x90`\x01`\x01`\x80\x1B\x03``\x82\x015\x81\x16\x91`\x80\x015\x16a\x1D@V[a\x05D`\x04\x806\x03` \x81\x10\x15a\x05=W`\0\x80\xFD[P5a\x1FZV[`@\x80Q`\x01`\x01`\x80\x1B\x03\x96\x87\x16\x81R` \x81\x01\x95\x90\x95R\x84\x81\x01\x93\x90\x93R\x90\x84\x16``\x84\x01R\x90\x92\x16`\x80\x82\x01R\x90Q\x90\x81\x90\x03`\xA0\x01\x90\xF3[a\x04:`\x04\x806\x03` \x81\x10\x15a\x05\x96W`\0\x80\xFD[P5`\x01\x0Ba\x1F\x97V[a\x02la\x1F\xA9V[a\x037`\x04\x806\x03`@\x81\x10\x15a\x05\xBEW`\0\x80\xFD[P`\xFF\x815\x81\x16\x91` \x015\x16a\x1F\xCDV[a\x02\x90`\x04\x806\x03``\x81\x10\x15a\x05\xE6W`\0\x80\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x16\x90`\x01`\x01`\x80\x1B\x03` \x82\x015\x81\x16\x91`@\x015\x16a!\xB1V[a\x06{`\x04\x806\x03` \x81\x10\x15a\x06#W`\0\x80\xFD[\x81\x01\x90` \x81\x01\x815`\x01` \x1B\x81\x11\x15a\x06=W`\0\x80\xFD[\x82\x01\x83` \x82\x01\x11\x15a\x06OW`\0\x80\xFD[\x805\x90` \x01\x91\x84` \x83\x02\x84\x01\x11`\x01` \x1B\x83\x11\x17\x15a\x06pW`\0\x80\xFD[P\x90\x92P\x90Pa$~V[`@Q\x80\x80` \x01\x80` \x01\x83\x81\x03\x83R\x85\x81\x81Q\x81R` \x01\x91P\x80Q\x90` \x01\x90` \x02\x80\x83\x83`\0[\x83\x81\x10\x15a\x06\xBFW\x81\x81\x01Q\x83\x82\x01R` \x01a\x06\xA7V[PPPP\x90P\x01\x83\x81\x03\x82R\x84\x81\x81Q\x81R` \x01\x91P\x80Q\x90` \x01\x90` \x02\x80\x83\x83`\0[\x83\x81\x10\x15a\x06\xFEW\x81\x81\x01Q\x83\x82\x01R` \x01a\x06\xE6V[PPPP\x90P\x01\x94PPPPP`@Q\x80\x91\x03\x90\xF3[a\x02K`\x04\x806\x03``\x81\x10\x15a\x07*W`\0\x80\xFD[P\x805`\x02\x90\x81\x0B\x91` \x81\x015\x90\x91\x0B\x90`@\x015`\x01`\x01`\x80\x1B\x03\x16a%\x0BV[a\x07x`\x04\x806\x03`@\x81\x10\x15a\x07dW`\0\x80\xFD[P\x805`\x02\x90\x81\x0B\x91` \x015\x90\x0Ba&\x82V[`@\x80Q`\x06\x94\x90\x94\x0B\x84R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x84\x01Rc\xFF\xFF\xFF\xFF\x16\x82\x82\x01RQ\x90\x81\x90\x03``\x01\x90\xF3[a\x01\x9Ba(qV[a\x07\xB9a(\x95V[`@\x80Q`\x02\x92\x90\x92\x0B\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[a\x01\x9Ba(\xB9V[a\x07\xE0a(\xDDV[`@\x80Qb\xFF\xFF\xFF\x90\x92\x16\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[a\x04:a)\x01V[a\x08 `\x04\x806\x03` \x81\x10\x15a\x08\x16W`\0\x80\xFD[P5`\x02\x0Ba)\x07V[`@\x80Q`\x01`\x01`\x80\x1B\x03\x90\x99\x16\x89R`\x0F\x97\x90\x97\x0B` \x89\x01R\x87\x87\x01\x95\x90\x95R``\x87\x01\x93\x90\x93R`\x06\x91\x90\x91\x0B`\x80\x86\x01R`\x01`\x01`\xA0\x1B\x03\x16`\xA0\x85\x01Rc\xFF\xFF\xFF\xFF\x16`\xC0\x84\x01R\x15\x15`\xE0\x83\x01RQ\x90\x81\x90\x03a\x01\0\x01\x90\xF3[a\x037`\x04\x806\x03` \x81\x10\x15a\x08\x98W`\0\x80\xFD[P5`\x01`\x01`\xA0\x1B\x03\x16a)qV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0\x80a\x08\xD7a+GV[\x85a\t\x0EW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x02`$\x82\x01RaAS`\xF0\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`@\x80Q`\xE0\x81\x01\x82R`\0T`\x01`\x01`\xA0\x1B\x03\x81\x16\x82R`\x01`\xA0\x1B\x81\x04`\x02\x90\x81\x0B\x81\x0B\x90\x0B` \x83\x01Ra\xFF\xFF`\x01`\xB8\x1B\x82\x04\x81\x16\x93\x83\x01\x93\x90\x93R`\x01`\xC8\x1B\x81\x04\x83\x16``\x83\x01R`\x01`\xD8\x1B\x81\x04\x90\x92\x16`\x80\x82\x01R`\xFF`\x01`\xE8\x1B\x83\x04\x81\x16`\xA0\x83\x01R`\x01`\xF0\x1B\x90\x92\x04\x90\x91\x16\x15\x15`\xC0\x82\x01\x81\x90Ra\t\xC7W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x03`$\x82\x01RbLOK`\xE8\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x87a\n\x12W\x80`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x86`\x01`\x01`\xA0\x1B\x03\x16\x11\x80\x15a\n\rWPs\xFF\xFD\x89c\xEF\xD1\xFCjPd\x88I]\x95\x1DRc\x98\x8D&`\x01`\x01`\xA0\x1B\x03\x87\x16\x10[a\nDV[\x80`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x86`\x01`\x01`\xA0\x1B\x03\x16\x10\x80\x15a\nDWPd\x01\0\x02v\xA3`\x01`\x01`\xA0\x1B\x03\x87\x16\x11[a\n{W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x03`$\x82\x01Rb\x14\xD4\x13`\xEA\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\0\x80T`\xFF`\xF0\x1B\x19\x16\x81U`@\x80Q`\xC0\x81\x01\x90\x91R\x80\x8Aa\n\xAAW`\x04\x84`\xA0\x01Q`\xFF\x16\x90\x1Ca\n\xBDV[`\x10\x84`\xA0\x01Q`\xFF\x16\x81a\n\xBBW\xFE[\x06[`\xFF\x16\x81R`\x04T`\x01`\x01`\x80\x1B\x03\x16` \x82\x01R`@\x01a\n\xDEa+~V[c\xFF\xFF\xFF\xFF\x16\x81R` \x01`\0`\x06\x0B\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\0\x15\x15\x81RP\x90P`\0\x80\x89\x13\x90P`\0`@Q\x80`\xE0\x01`@R\x80\x8B\x81R` \x01`\0\x81R` \x01\x85`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x85` \x01Q`\x02\x0B\x81R` \x01\x8Ca\x0BZW`\x02Ta\x0B^V[`\x01T[\x81R` \x01`\0`\x01`\x01`\x80\x1B\x03\x16\x81R` \x01\x84` \x01Q`\x01`\x01`\x80\x1B\x03\x16\x81RP\x90P[\x80Q\x15\x80\x15\x90a\x0B\xADWP\x88`\x01`\x01`\xA0\x1B\x03\x16\x81`@\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x15a\x0FwWa\x0B\xBAaT\xB5V[`@\x82\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81R``\x82\x01Qa\x0B\xFD\x90`\x06\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x8Fa+\x82V[\x15\x15`@\x83\x01R`\x02\x90\x81\x0B\x81\x0B` \x83\x01\x81\x90Rb\r\x89\xE7\x19\x91\x0B\x12\x15a\x0C.Wb\r\x89\xE7\x19` \x82\x01Ra\x0CMV[` \x81\x01Qb\r\x89\xE8`\x02\x91\x90\x91\x0B\x13\x15a\x0CMWb\r\x89\xE8` \x82\x01R[a\x0CZ\x81` \x01Qa,\xC4V[`\x01`\x01`\xA0\x1B\x03\x16``\x82\x01R`@\x82\x01Qa\x0C\xEB\x90\x8Da\x0C\x94W\x8B`\x01`\x01`\xA0\x1B\x03\x16\x83``\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x11a\x0C\xAEV[\x8B`\x01`\x01`\xA0\x1B\x03\x16\x83``\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x10[a\x0C\xBCW\x82``\x01Qa\x0C\xBEV[\x8B[`\xC0\x85\x01Q\x85Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a/\xF5V[`\xC0\x85\x01R`\xA0\x84\x01R`\x80\x83\x01R`\x01`\x01`\xA0\x1B\x03\x16`@\x83\x01R\x82\x15a\rMWa\r!\x81`\xC0\x01Q\x82`\x80\x01Q\x01a1\xE7V[\x82Q\x03\x82R`\xA0\x81\x01Qa\rC\x90a\r8\x90a1\xE7V[` \x84\x01Q\x90a1\xFDV[` \x83\x01Ra\r\x88V[a\rZ\x81`\xA0\x01Qa1\xE7V[\x82Q\x01\x82R`\xC0\x81\x01Q`\x80\x82\x01Qa\r\x82\x91a\rw\x91\x01a1\xE7V[` \x84\x01Q\x90a2\x19V[` \x83\x01R[\x83Q`\xFF\x16\x15a\r\xCEW`\0\x84`\0\x01Q`\xFF\x16\x82`\xC0\x01Q\x81a\r\xA8W\xFE[`\xC0\x84\x01\x80Q\x92\x90\x91\x04\x91\x82\x90\x03\x90R`\xA0\x84\x01\x80Q\x90\x91\x01`\x01`\x01`\x80\x1B\x03\x16\x90RP[`\xC0\x82\x01Q`\x01`\x01`\x80\x1B\x03\x16\x15a\x0E\rWa\x0E\x01\x81`\xC0\x01Q`\x01`\x80\x1B\x84`\xC0\x01Q`\x01`\x01`\x80\x1B\x03\x16a2/V[`\x80\x83\x01\x80Q\x90\x91\x01\x90R[\x80``\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x82`@\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a\x0F6W\x80`@\x01Q\x15a\x0F\rW\x83`\xA0\x01Qa\x0E\x97Wa\x0Eu\x84`@\x01Q`\0\x87` \x01Q\x88`@\x01Q\x88` \x01Q\x8A``\x01Q`\x08a2\xDF\x90\x96\x95\x94\x93\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\x01`\x01`\xA0\x1B\x03\x16`\x80\x86\x01R`\x06\x90\x81\x0B\x90\x0B``\x85\x01R`\x01`\xA0\x85\x01R[`\0a\x0E\xE3\x82` \x01Q\x8Ea\x0E\xAEW`\x01Ta\x0E\xB4V[\x84`\x80\x01Q[\x8Fa\x0E\xC3W\x85`\x80\x01Qa\x0E\xC7V[`\x02T[`\x80\x89\x01Q``\x8A\x01Q`@\x8B\x01Q`\x05\x95\x94\x93\x92\x91\x90a4qV[\x90P\x8C\x15a\x0E\xEFW`\0\x03[a\x0E\xFD\x83`\xC0\x01Q\x82a5+V[`\x01`\x01`\x80\x1B\x03\x16`\xC0\x84\x01RP[\x8Ba\x0F\x1CW\x80` \x01Qa\x0F%V[`\x01\x81` \x01Q\x03[`\x02\x90\x81\x0B\x90\x0B``\x83\x01Ra\x0FqV[\x80`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x82`@\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0FqWa\x0Fd\x82`@\x01Qa5\xE1V[`\x02\x90\x81\x0B\x90\x0B``\x83\x01R[Pa\x0B\x87V[\x83` \x01Q`\x02\x0B\x81``\x01Q`\x02\x0B\x14a\x10EW`\0\x80a\x0F\xC5\x86`@\x01Q\x86`@\x01Q\x88` \x01Q\x88` \x01Q\x8A``\x01Q\x8B`\x80\x01Q`\x08a8\xFC\x90\x96\x95\x94\x93\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[`@\x85\x01Q``\x86\x01Q`\0\x80Ta\xFF\xFF`\xC8\x1B\x19\x16`\x01`\xC8\x1Ba\xFF\xFF\x95\x86\x16\x02\x17a\xFF\xFF`\xB8\x1B\x19\x16`\x01`\xB8\x1B\x95\x90\x94\x16\x94\x90\x94\x02\x92\x90\x92\x17b\xFF\xFF\xFF`\xA0\x1B\x19\x16`\x01`\xA0\x1Bb\xFF\xFF\xFF`\x02\x94\x90\x94\x0B\x93\x90\x93\x16\x92\x90\x92\x02\x91\x90\x91\x17`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x17\x90UPa\x10j\x90PV[`@\x81\x01Q`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90U[\x80`\xC0\x01Q`\x01`\x01`\x80\x1B\x03\x16\x83` \x01Q`\x01`\x01`\x80\x1B\x03\x16\x14a\x10\xB0W`\xC0\x81\x01Q`\x04\x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90U[\x8A\x15a\x11\0W`\x80\x81\x01Q`\x01U`\xA0\x81\x01Q`\x01`\x01`\x80\x1B\x03\x16\x15a\x10\xFBW`\xA0\x81\x01Q`\x03\x80T`\x01`\x01`\x80\x1B\x03\x19\x81\x16`\x01`\x01`\x80\x1B\x03\x91\x82\x16\x90\x93\x01\x16\x91\x90\x91\x17\x90U[a\x11FV[`\x80\x81\x01Q`\x02U`\xA0\x81\x01Q`\x01`\x01`\x80\x1B\x03\x16\x15a\x11FW`\xA0\x81\x01Q`\x03\x80T`\x01`\x01`\x80\x1B\x03\x80\x82\x16`\x01`\x80\x1B\x92\x83\x90\x04\x82\x16\x90\x94\x01\x16\x02\x91\x90\x91\x17\x90U[\x81\x15\x15\x8B\x15\x15\x14a\x11_W` \x81\x01Q\x81Q\x8B\x03a\x11lV[\x80`\0\x01Q\x8A\x03\x81` \x01Q[\x90\x96P\x94P\x8A\x15a\x12\xA5W`\0\x85\x12\x15a\x11\xAEWa\x11\xAE\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x8D\x87`\0\x03a:\x97V[`\0a\x11\xB8a;\xE5V[\x90P3`\x01`\x01`\xA0\x1B\x03\x16c\xFAF\x1E3\x88\x88\x8C\x8C`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x85\x81R` \x01\x84\x81R` \x01\x80` \x01\x82\x81\x03\x82R\x84\x84\x82\x81\x81R` \x01\x92P\x80\x82\x847`\0\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPP\x95PPPPPP`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x12<W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x12PW=`\0\x80>=`\0\xFD[PPPPa\x12\\a;\xE5V[a\x12f\x82\x89a=\x1EV[\x11\x15a\x12\x9FW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x03`$\x82\x01RbIIA`\xE8\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[Pa\x13\xCFV[`\0\x86\x12\x15a\x12\xDCWa\x12\xDC\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x8D\x88`\0\x03a:\x97V[`\0a\x12\xE6a=.V[\x90P3`\x01`\x01`\xA0\x1B\x03\x16c\xFAF\x1E3\x88\x88\x8C\x8C`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x85\x81R` \x01\x84\x81R` \x01\x80` \x01\x82\x81\x03\x82R\x84\x84\x82\x81\x81R` \x01\x92P\x80\x82\x847`\0\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPP\x95PPPPPP`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x13jW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x13~W=`\0\x80>=`\0\xFD[PPPPa\x13\x8Aa=.V[a\x13\x94\x82\x88a=\x1EV[\x11\x15a\x13\xCDW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x03`$\x82\x01RbIIA`\xE8\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[P[`@\x80\x82\x01Q`\xC0\x83\x01Q``\x80\x85\x01Q\x84Q\x8B\x81R` \x81\x01\x8B\x90R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x81\x87\x01R`\x01`\x01`\x80\x1B\x03\x90\x93\x16\x91\x83\x01\x91\x90\x91R`\x02\x0B`\x80\x82\x01R\x91Q\x90\x8E\x16\x913\x91\x7F\xC4 y\xF9JcP\xD7\xE6#_)\x17I$\xF9(\xCC*\xC8\x18\xEBd\xFE\xD8\0N\x11_\xBC\xCAg\x91\x81\x90\x03`\xA0\x01\x90\xA3PP`\0\x80T`\xFF`\xF0\x1B\x19\x16`\x01`\xF0\x1B\x17\x90UP\x91\x98\x90\x97P\x95PPPPPPV[`\x04T`\x01`\x01`\x80\x1B\x03\x16\x81V[`\x03T`\x01`\x01`\x80\x1B\x03\x80\x82\x16\x91`\x01`\x80\x1B\x90\x04\x16\x82V[`\x08\x81a\xFF\xFF\x81\x10a\x14\xA5W`\0\x80\xFD[\x01Tc\xFF\xFF\xFF\xFF\x81\x16\x91P`\x01` \x1B\x81\x04`\x06\x0B\x90`\x01`X\x1B\x81\x04`\x01`\x01`\xA0\x1B\x03\x16\x90`\x01`\xF8\x1B\x90\x04`\xFF\x16\x84V[`\0T`\x01`\xF0\x1B\x90\x04`\xFF\x16a\x15\x1DW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x03`$\x82\x01RbLOK`\xE8\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\0\x80T`\xFF`\xF0\x1B\x19\x16\x90Ua\x152a+GV[`\0\x80T`\x01`\xD8\x1B\x90\x04a\xFF\xFF\x16\x90a\x15N`\x08\x83\x85a=\xC6V[`\0\x80Ta\xFF\xFF\x80\x84\x16`\x01`\xD8\x1B\x81\x02a\xFF\xFF`\xD8\x1B\x19\x90\x93\x16\x92\x90\x92\x17\x90\x92U\x91\x92P\x83\x16\x14a\x15\xBBW`@\x80Qa\xFF\xFF\x80\x85\x16\x82R\x83\x16` \x82\x01R\x81Q\x7F\xACI\xE5\x18\xF9\n5\x8Fe.D\0\x16O\x05\xA5\xD8\xF7\xE3^wG'\x9B\xC3\xA9=\xBFXN\x12Z\x92\x91\x81\x90\x03\x90\x91\x01\x90\xA1[PP`\0\x80T`\xFF`\xF0\x1B\x19\x16`\x01`\xF0\x1B\x17\x90UPV[`\0T`\x01`\x01`\xA0\x1B\x03\x81\x16\x90`\x01`\xA0\x1B\x81\x04`\x02\x0B\x90a\xFF\xFF`\x01`\xB8\x1B\x82\x04\x81\x16\x91`\x01`\xC8\x1B\x81\x04\x82\x16\x91`\x01`\xD8\x1B\x82\x04\x16\x90`\xFF`\x01`\xE8\x1B\x82\x04\x81\x16\x91`\x01`\xF0\x1B\x90\x04\x16\x87V[`\0\x80T\x81\x90`\x01`\xF0\x1B\x90\x04`\xFF\x16a\x16jW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x03`$\x82\x01RbLOK`\xE8\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\0\x80T`\xFF`\xF0\x1B\x19\x16\x90U`\x01`\x01`\x80\x1B\x03\x85\x16a\x16\x8AW`\0\x80\xFD[`\0\x80a\x16\xD8`@Q\x80`\x80\x01`@R\x80\x8C`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8B`\x02\x0B\x81R` \x01\x8A`\x02\x0B\x81R` \x01a\x16\xCE\x8A`\x01`\x01`\x80\x1B\x03\x16a>iV[`\x0F\x0B\x90Ra>zV[\x92P\x92PP\x81\x93P\x80\x92P`\0\x80`\0\x86\x11\x15a\x16\xFAWa\x16\xF7a;\xE5V[\x91P[\x84\x15a\x17\x0BWa\x17\x08a=.V[\x90P[3`\x01`\x01`\xA0\x1B\x03\x16c\xD3Hy\x97\x87\x87\x8B\x8B`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x85\x81R` \x01\x84\x81R` \x01\x80` \x01\x82\x81\x03\x82R\x84\x84\x82\x81\x81R` \x01\x92P\x80\x82\x847`\0\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPP\x95PPPPPP`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x17\x8DW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x17\xA1W=`\0\x80>=`\0\xFD[PPPP`\0\x86\x11\x15a\x17\xF8Wa\x17\xB6a;\xE5V[a\x17\xC0\x83\x88a=\x1EV[\x11\x15a\x17\xF8W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x02`$\x82\x01Ra\x04\xD3`\xF4\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x84\x15a\x18HWa\x18\x06a=.V[a\x18\x10\x82\x87a=\x1EV[\x11\x15a\x18HW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x02`$\x82\x01RaM1`\xF0\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x89`\x02\x0B\x8B`\x02\x0B\x8D`\x01`\x01`\xA0\x1B\x03\x16\x7FzS\x08\x0B\xA4\x14\x15\x8B\xE7\xECi\xB9\x87\xB5\xFB}\x07\xDE\xE1\x01\xFE\x85H\x8F\x08S\xAE\x16#\x9D\x0B\xDE3\x8D\x8B\x8B`@Q\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x84`\x01`\x01`\x80\x1B\x03\x16\x81R` \x01\x83\x81R` \x01\x82\x81R` \x01\x94PPPPP`@Q\x80\x91\x03\x90\xA4PP`\0\x80T`\xFF`\xF0\x1B\x19\x16`\x01`\xF0\x1B\x17\x90UP\x91\x98\x90\x97P\x95PPPPPPV[`\x02T\x81V[`\0T`\x01`\xF0\x1B\x90\x04`\xFF\x16a\x19)W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x03`$\x82\x01RbLOK`\xE8\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\0\x80T`\xFF`\xF0\x1B\x19\x16\x90Ua\x19>a+GV[`\x04T`\x01`\x01`\x80\x1B\x03\x16\x80a\x19\x80W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x01`$\x82\x01R`\x13`\xFA\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\0a\x19\xB5\x86\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0b\xFF\xFF\xFF\x16b\x0FB@a@\xBAV[\x90P`\0a\x19\xEC\x86\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0b\xFF\xFF\xFF\x16b\x0FB@a@\xBAV[\x90P`\0a\x19\xF8a;\xE5V[\x90P`\0a\x1A\x04a=.V[\x90P\x88\x15a\x1A7Wa\x1A7\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x8B\x8Ba:\x97V[\x87\x15a\x1AhWa\x1Ah\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x8B\x8Aa:\x97V[3`\x01`\x01`\xA0\x1B\x03\x16c\xE9\xCB\xAF\xB0\x85\x85\x8A\x8A`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x85\x81R` \x01\x84\x81R` \x01\x80` \x01\x82\x81\x03\x82R\x84\x84\x82\x81\x81R` \x01\x92P\x80\x82\x847`\0\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPP\x95PPPPPP`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1A\xEAW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1A\xFEW=`\0\x80>=`\0\xFD[PPPP`\0a\x1B\x0Ca;\xE5V[\x90P`\0a\x1B\x18a=.V[\x90P\x81a\x1B%\x85\x88a=\x1EV[\x11\x15a\x1B]W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x02`$\x82\x01Ra\x04c`\xF4\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x80a\x1Bh\x84\x87a=\x1EV[\x11\x15a\x1B\xA0W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x02`$\x82\x01RaF1`\xF0\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x83\x82\x03\x83\x82\x03\x81\x15a\x1C/W`\0\x80T`\x01`\xE8\x1B\x90\x04`\x0F\x16\x90\x81\x15a\x1B\xD3W\x81`\xFF\x16\x84\x81a\x1B\xCDW\xFE[\x04a\x1B\xD6V[`\0[\x90P`\x01`\x01`\x80\x1B\x03\x81\x16\x15a\x1C\tW`\x03\x80T`\x01`\x01`\x80\x1B\x03\x80\x82\x16\x84\x01\x16`\x01`\x01`\x80\x1B\x03\x19\x90\x91\x16\x17\x90U[a\x1C#\x81\x85\x03`\x01`\x80\x1B\x8D`\x01`\x01`\x80\x1B\x03\x16a2/V[`\x01\x80T\x90\x91\x01\x90UPP[\x80\x15a\x1C\xBAW`\0\x80T`\x01`\xE8\x1B\x90\x04`\x04\x1C`\x0F\x16\x90\x81\x15a\x1C_W\x81`\xFF\x16\x83\x81a\x1CYW\xFE[\x04a\x1CbV[`\0[\x90P`\x01`\x01`\x80\x1B\x03\x81\x16\x15a\x1C\x94W`\x03\x80T`\x01`\x01`\x80\x1B\x03`\x01`\x80\x1B\x80\x83\x04\x82\x16\x85\x01\x82\x16\x02\x91\x16\x17\x90U[a\x1C\xAE\x81\x84\x03`\x01`\x80\x1B\x8D`\x01`\x01`\x80\x1B\x03\x16a2/V[`\x02\x80T\x90\x91\x01\x90UPP[\x8D`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x7F\xBD\xBD\xB7\x1Dx`7k\xA5+%\xA5\x02\x8B\xEE\xA25\x816J@R/k\xCF\xB8k\xB1\xF2\xDC\xA63\x8F\x8F\x86\x86`@Q\x80\x85\x81R` \x01\x84\x81R` \x01\x83\x81R` \x01\x82\x81R` \x01\x94PPPPP`@Q\x80\x91\x03\x90\xA3PP`\0\x80T`\xFF`\xF0\x1B\x19\x16`\x01`\xF0\x1B\x17\x90UPPPPPPPPPPPPV[`\0\x80T\x81\x90`\x01`\xF0\x1B\x90\x04`\xFF\x16a\x1D\x87W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x03`$\x82\x01RbLOK`\xE8\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\0\x80T`\xFF`\xF0\x1B\x19\x16\x81Ua\x1D\xA1`\x073\x89\x89a@\xF4V[`\x03\x81\x01T\x90\x91P`\x01`\x01`\x80\x1B\x03\x90\x81\x16\x90\x86\x16\x11a\x1D\xC2W\x84a\x1D\xD1V[`\x03\x81\x01T`\x01`\x01`\x80\x1B\x03\x16[`\x03\x82\x01T\x90\x93P`\x01`\x01`\x80\x1B\x03`\x01`\x80\x1B\x90\x91\x04\x81\x16\x90\x85\x16\x11a\x1D\xF9W\x83a\x1E\x0FV[`\x03\x81\x01T`\x01`\x80\x1B\x90\x04`\x01`\x01`\x80\x1B\x03\x16[\x91P`\x01`\x01`\x80\x1B\x03\x83\x16\x15a\x1EtW`\x03\x81\x01\x80T`\x01`\x01`\x80\x1B\x03\x19\x81\x16`\x01`\x01`\x80\x1B\x03\x91\x82\x16\x86\x90\x03\x82\x16\x17\x90\x91Ua\x1Et\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x8A\x90\x86\x16a:\x97V[`\x01`\x01`\x80\x1B\x03\x82\x16\x15a\x1E\xDAW`\x03\x81\x01\x80T`\x01`\x01`\x80\x1B\x03`\x01`\x80\x1B\x80\x83\x04\x82\x16\x86\x90\x03\x82\x16\x02\x91\x81\x16\x91\x90\x91\x17\x90\x91Ua\x1E\xDA\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x8A\x90\x85\x16a:\x97V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x8A\x16\x81R`\x01`\x01`\x80\x1B\x03\x80\x86\x16` \x83\x01R\x84\x16\x81\x83\x01R\x90Q`\x02\x88\x81\x0B\x92\x90\x8A\x90\x0B\x913\x91\x7Fp\x93S8\xE6\x97uEj\x85\xDD\xEF\"l9_\xB6h\xB6?\xA0\x11__ a\x0B8\x8El\xA9\xC0\x91\x90\x81\x90\x03``\x01\x90\xA4P`\0\x80T`\xFF`\xF0\x1B\x19\x16`\x01`\xF0\x1B\x17\x90U\x90\x96\x90\x95P\x93PPPPV[`\x07` R`\0\x90\x81R`@\x90 \x80T`\x01\x82\x01T`\x02\x83\x01T`\x03\x90\x93\x01T`\x01`\x01`\x80\x1B\x03\x92\x83\x16\x93\x91\x92\x81\x81\x16\x91`\x01`\x80\x1B\x90\x04\x16\x85V[`\x06` R`\0\x90\x81R`@\x90 T\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0T`\x01`\xF0\x1B\x90\x04`\xFF\x16a \x11W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x03`$\x82\x01RbLOK`\xE8\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\0\x80T`\xFF`\xF0\x1B\x19\x16\x90U`@\x80Qc\x8D\xA5\xCB[`\xE0\x1B\x81R\x90Q`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91c\x8D\xA5\xCB[\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a ~W`\0\x80\xFD[PZ\xFA\x15\x80\x15a \x92W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a \xA8W`\0\x80\xFD[PQ`\x01`\x01`\xA0\x1B\x03\x163\x14a \xBEW`\0\x80\xFD[`\xFF\x82\x16\x15\x80a \xE1WP`\x04\x82`\xFF\x16\x10\x15\x80\x15a \xE1WP`\n\x82`\xFF\x16\x11\x15[\x80\x15a!\x0BWP`\xFF\x81\x16\x15\x80a!\x0BWP`\x04\x81`\xFF\x16\x10\x15\x80\x15a!\x0BWP`\n\x81`\xFF\x16\x11\x15[a!\x14W`\0\x80\xFD[`\0\x80Ta\x0F\xF0`\x04\x84\x90\x1B\x16\x84\x01`\xFF\x90\x81\x16`\x01`\xE8\x1B\x90\x81\x02`\xFF`\xE8\x1B\x19\x84\x16\x17\x90\x93U\x91\x90\x04\x16\x7F\x97=\x8D\x92\xBB)\x9FJ\xF6\xCEI\xB5*\x8A\xDB\x85\xAEF\xB9\xF2\x14\xC4\xC4\xFC\x06\xACw@\x127\xB13`\x10\x82`@\x80Q`\xFF\x93\x90\x92\x06\x83\x16\x82R`\x0F`\x04\x86\x90\x1C\x16` \x83\x01R\x86\x83\x16\x82\x82\x01R\x91\x85\x16``\x82\x01R\x90Q\x90\x81\x90\x03`\x80\x01\x90\xA1PP`\0\x80T`\xFF`\xF0\x1B\x19\x16`\x01`\xF0\x1B\x17\x90UPV[`\0\x80T\x81\x90`\x01`\xF0\x1B\x90\x04`\xFF\x16a!\xF8W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x03`$\x82\x01RbLOK`\xE8\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\0\x80T`\xFF`\xF0\x1B\x19\x16\x90U`@\x80Qc\x8D\xA5\xCB[`\xE0\x1B\x81R\x90Q`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91c\x8D\xA5\xCB[\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\"eW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\"yW=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\"\x8FW`\0\x80\xFD[PQ`\x01`\x01`\xA0\x1B\x03\x163\x14a\"\xA5W`\0\x80\xFD[`\x03T`\x01`\x01`\x80\x1B\x03\x90\x81\x16\x90\x85\x16\x11a\"\xC1W\x83a\"\xCEV[`\x03T`\x01`\x01`\x80\x1B\x03\x16[`\x03T\x90\x92P`\x01`\x01`\x80\x1B\x03`\x01`\x80\x1B\x90\x91\x04\x81\x16\x90\x84\x16\x11a\"\xF4W\x82a#\x08V[`\x03T`\x01`\x80\x1B\x90\x04`\x01`\x01`\x80\x1B\x03\x16[\x90P`\x01`\x01`\x80\x1B\x03\x82\x16\x15a#\x89W`\x03T`\x01`\x01`\x80\x1B\x03\x83\x81\x16\x91\x16\x14\x15a#7W`\0\x19\x90\x91\x01\x90[`\x03\x80T`\x01`\x01`\x80\x1B\x03\x19\x81\x16`\x01`\x01`\x80\x1B\x03\x91\x82\x16\x85\x90\x03\x82\x16\x17\x90\x91Ua#\x89\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x87\x90\x85\x16a:\x97V[`\x01`\x01`\x80\x1B\x03\x81\x16\x15a$\x0FW`\x03T`\x01`\x01`\x80\x1B\x03\x82\x81\x16`\x01`\x80\x1B\x90\x92\x04\x16\x14\x15a#\xBAW`\0\x19\x01[`\x03\x80T`\x01`\x01`\x80\x1B\x03`\x01`\x80\x1B\x80\x83\x04\x82\x16\x85\x90\x03\x82\x16\x02\x91\x81\x16\x91\x90\x91\x17\x90\x91Ua$\x0F\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x87\x90\x84\x16a:\x97V[`@\x80Q`\x01`\x01`\x80\x1B\x03\x80\x85\x16\x82R\x83\x16` \x82\x01R\x81Q`\x01`\x01`\xA0\x1B\x03\x88\x16\x923\x92\x7FYkW9\x06!\x8D4\x11\x85\x0B&\xA6\xB47\xD6\xC4R/\xDBC\xD2\xD28bc\xF8mP\xB8\xB1Q\x92\x90\x81\x90\x03\x90\x91\x01\x90\xA3`\0\x80T`\xFF`\xF0\x1B\x19\x16`\x01`\xF0\x1B\x17\x90U\x90\x94\x90\x93P\x91PPV[``\x80a$\x89a+GV[a%\0a$\x94a+~V[\x85\x85\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x82\x90RPT`\x04T`\x08\x96\x95\x94P`\x01`\xA0\x1B\x82\x04`\x02\x0B\x93Pa\xFF\xFF`\x01`\xB8\x1B\x83\x04\x81\x16\x93P`\x01`\x01`\x80\x1B\x03\x90\x91\x16\x91`\x01`\xC8\x1B\x90\x04\x16aAXV[\x91P\x91P\x92P\x92\x90PV[`\0\x80T\x81\x90`\x01`\xF0\x1B\x90\x04`\xFF\x16a%RW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x03`$\x82\x01RbLOK`\xE8\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\0\x80T`\xFF`\xF0\x1B\x19\x16\x81U`@\x80Q`\x80\x81\x01\x82R3\x81R`\x02\x88\x81\x0B` \x83\x01R\x87\x90\x0B\x91\x81\x01\x91\x90\x91R\x81\x90\x81\x90a%\xAB\x90``\x81\x01a%\x9E`\x01`\x01`\x80\x1B\x03\x8A\x16a>iV[`\0\x03`\x0F\x0B\x90Ra>zV[\x92P\x92P\x92P\x81`\0\x03\x94P\x80`\0\x03\x93P`\0\x85\x11\x80a%\xCCWP`\0\x84\x11[\x15a&\x0BW`\x03\x83\x01\x80T`\x01`\x01`\x80\x1B\x03\x80\x82\x16\x80\x89\x01\x82\x16`\x01`\x80\x1B\x93\x84\x90\x04\x83\x16\x89\x01\x90\x92\x16\x90\x92\x02\x90\x91\x17`\x01`\x01`\x80\x1B\x03\x19\x16\x17\x90U[`@\x80Q`\x01`\x01`\x80\x1B\x03\x88\x16\x81R` \x81\x01\x87\x90R\x80\x82\x01\x86\x90R\x90Q`\x02\x89\x81\x0B\x92\x90\x8B\x90\x0B\x913\x91\x7F\x0C9l\xD9\x89\xA3\x9FDY\xB5\xFA\x1A\xEDj\x9A\x8D\xCD\xBCE\x90\x8A\xCF\xD6~\x02\x8C\xD5h\xDA\x98\x98,\x91\x90\x81\x90\x03``\x01\x90\xA4PP`\0\x80T`\xFF`\xF0\x1B\x19\x16`\x01`\xF0\x1B\x17\x90UP\x90\x94\x90\x93P\x91PPV[`\0\x80`\0a&\x8Fa+GV[a&\x99\x85\x85aB\xB2V[`\x02\x85\x81\x0B\x81\x0B`\0\x90\x81R`\x05` R`@\x80\x82 \x87\x84\x0B\x90\x93\x0B\x82R\x81 `\x03\x83\x01T`\x06\x81\x90\x0B\x93`\x01`8\x1B\x82\x04`\x01`\x01`\xA0\x1B\x03\x16\x92\x84\x92`\x01`\xD8\x1B\x81\x04c\xFF\xFF\xFF\xFF\x16\x92\x84\x92\x90\x91`\x01`\xF8\x1B\x90\x04`\xFF\x16\x80a&\xFDW`\0\x80\xFD[`\x03\x82\x01T`\x06\x81\x90\x0B\x98P`\x01`8\x1B\x81\x04`\x01`\x01`\xA0\x1B\x03\x16\x96P`\x01`\xD8\x1B\x81\x04c\xFF\xFF\xFF\xFF\x16\x94P`\x01`\xF8\x1B\x90\x04`\xFF\x16\x80a'>W`\0\x80\xFD[PP`@\x80Q`\xE0\x81\x01\x82R`\0T`\x01`\x01`\xA0\x1B\x03\x81\x16\x82R`\x01`\xA0\x1B\x81\x04`\x02\x90\x81\x0B\x81\x0B\x81\x0B` \x84\x01\x81\x90Ra\xFF\xFF`\x01`\xB8\x1B\x84\x04\x81\x16\x95\x85\x01\x95\x90\x95R`\x01`\xC8\x1B\x83\x04\x85\x16``\x85\x01R`\x01`\xD8\x1B\x83\x04\x90\x94\x16`\x80\x84\x01R`\xFF`\x01`\xE8\x1B\x83\x04\x81\x16`\xA0\x85\x01R`\x01`\xF0\x1B\x90\x92\x04\x90\x91\x16\x15\x15`\xC0\x83\x01R\x90\x93P\x8E\x81\x0B\x91\x90\x0B\x12\x15\x90Pa'\xE7WP\x93\x90\x94\x03\x96P\x90\x03\x93P\x90\x03\x90Pa(jV[\x8A`\x02\x0B\x81` \x01Q`\x02\x0B\x12\x15a([W`\0a(\x03a+~V[` \x83\x01Q`@\x84\x01Q`\x04T``\x86\x01Q\x93\x94P`\0\x93\x84\x93a(9\x93`\x08\x93\x88\x93\x87\x93\x92\x91`\x01`\x01`\x80\x1B\x03\x16\x90a2\xDFV[\x9A\x90\x03\x98\x90\x98\x03\x9BPP\x94\x90\x96\x03\x92\x90\x92\x03\x96P\x90\x91\x03\x03\x92Pa(j\x91PPV[P\x94\x90\x93\x03\x96P\x03\x93P\x90\x03\x90P[\x92P\x92P\x92V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\x01T\x81V[`\x05` R`\0\x90\x81R`@\x90 \x80T`\x01\x82\x01T`\x02\x83\x01T`\x03\x90\x93\x01T`\x01`\x01`\x80\x1B\x03\x83\x16\x93`\x01`\x80\x1B\x90\x93\x04`\x0F\x0B\x92\x90`\x06\x81\x90\x0B\x90`\x01`8\x1B\x81\x04`\x01`\x01`\xA0\x1B\x03\x16\x90`\x01`\xD8\x1B\x81\x04c\xFF\xFF\xFF\xFF\x16\x90`\x01`\xF8\x1B\x90\x04`\xFF\x16\x88V[`\0T`\x01`\x01`\xA0\x1B\x03\x16\x15a)\xB4W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x02`$\x82\x01RaAI`\xF0\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\0a)\xBF\x82a5\xE1V[\x90P`\0\x80a)\xD7a)\xCFa+~V[`\x08\x90aC{V[`@\x80Q`\xE0\x81\x01\x82R`\x01`\x01`\xA0\x1B\x03\x88\x16\x80\x82R`\x02\x88\x81\x0B` \x80\x85\x01\x82\x90R`\0\x85\x87\x01\x81\x90Ra\xFF\xFF\x89\x81\x16``\x88\x01\x81\x90R\x90\x89\x16`\x80\x88\x01\x81\x90R`\xA0\x88\x01\x83\x90R`\x01`\xC0\x90\x98\x01\x97\x90\x97R\x81T`\x01`\xF0\x1B`\x01`\x01`\xA0\x1B\x03\x19\x90\x91\x16\x87\x17b\xFF\xFF\xFF`\xA0\x1B\x19\x16`\x01`\xA0\x1Bb\xFF\xFF\xFF\x97\x87\x90\x0B\x97\x90\x97\x16\x96\x90\x96\x02\x95\x90\x95\x17c\xFF\xFF\xFF\xFF`\xB8\x1B\x19\x16`\x01`\xC8\x1B\x90\x91\x02\x17a\xFF\xFF`\xD8\x1B\x19\x16`\x01`\xD8\x1B\x90\x96\x02\x95\x90\x95\x17a\xFF\xFF`\xE8\x1B\x19\x16\x92\x90\x92\x17\x90\x93U\x83Q\x91\x82R\x81\x01\x91\x90\x91R\x81Q\x93\x95P\x91\x93P\x7F\x98c`6\xCBf\xA9\xC1\x9A7C^\xFC\x1E\x90\x14!\x90!N\x8A\xBE\xB8!\xBD\xBA?)\x90\xDDL\x95\x92\x91\x82\x90\x03\x01\x90\xA1PPPPV[`\0\x80\x82`\x02\x81\x90\x0Bb\r\x89\xE7\x19\x81a*\xF0W\xFE[\x05\x02\x90P`\0\x83`\x02\x81\x90\x0Bb\r\x89\xE8\x81a+\x07W\xFE[\x05\x02\x90P`\0\x84`\x02\x0B\x83\x83\x03`\x02\x0B\x81a+\x1EW\xFE[\x05`\x01\x01\x90P\x80b\xFF\xFF\xFF\x16`\x01`\x01`\x80\x1B\x03\x80\x16\x81a+;W\xFE[\x04\x93PPPP[\x91\x90PV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a+|W`\0\x80\xFD[V[B\x90V[`\0\x80`\0\x84`\x02\x0B\x86`\x02\x0B\x81a+\x96W\xFE[\x05\x90P`\0\x86`\x02\x0B\x12\x80\x15a+\xBDWP\x84`\x02\x0B\x86`\x02\x0B\x81a+\xB6W\xFE[\x07`\x02\x0B\x15\x15[\x15a+\xC7W`\0\x19\x01[\x83\x15a,<W`\0\x80a+\xD9\x83aC\xC7V[`\x01\x82\x81\x0B\x81\x0B`\0\x90\x81R` \x8D\x90R`@\x90 T`\xFF\x83\x16\x91\x90\x91\x1B\x80\x01`\0\x19\x01\x90\x81\x16\x80\x15\x15\x97P\x92\x94P\x90\x92P\x90\x85a,\x1EW\x88\x83`\xFF\x16\x86\x03\x02a,1V[\x88a,(\x82aC\xD9V[\x84\x03`\xFF\x16\x86\x03\x02[\x96PPPPPa,\xBAV[`\0\x80a,K\x83`\x01\x01aC\xC7V[\x91P\x91P`\0`\x01\x82`\xFF\x16`\x01\x90\x1B\x03\x19\x90P`\0\x81\x8B`\0\x86`\x01\x0B`\x01\x0B\x81R` \x01\x90\x81R` \x01`\0 T\x16\x90P\x80`\0\x14\x15\x95P\x85a,\x9DW\x88\x83`\xFF\x03`\xFF\x16\x86`\x01\x01\x01\x02a,\xB3V[\x88\x83a,\xA8\x83aDxV[\x03`\xFF\x16\x86`\x01\x01\x01\x02[\x96PPPPP[P\x94P\x94\x92PPPV[`\0\x80`\0\x83`\x02\x0B\x12a,\xDBW\x82`\x02\x0Ba,\xE3V[\x82`\x02\x0B`\0\x03[\x90Pb\r\x89\xE8\x81\x11\x15a-!W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x01`$\x82\x01R`\x15`\xFA\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\0`\x01\x82\x16a-5W`\x01`\x80\x1Ba-GV[o\xFF\xFC\xB93\xBDo\xAD7\xAA-\x16-\x1AY@\x01[p\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P`\x02\x82\x16\x15a-{Wo\xFF\xF9rr7=A2Y\xA4i\x90X\x0E!:\x02`\x80\x1C[`\x04\x82\x16\x15a-\x9AWo\xFF\xF2\xE5\x0F_ei2\xEF\x125|\xF3\xC7\xFD\xCC\x02`\x80\x1C[`\x08\x82\x16\x15a-\xB9Wo\xFF\xE5\xCA\xCA~\x10\xE4\xE6\x1C6$\xEA\xA0\x94\x1C\xD0\x02`\x80\x1C[`\x10\x82\x16\x15a-\xD8Wo\xFF\xCB\x98C\xD6\x0FaY\xC9\xDBX\x83\\\x92fD\x02`\x80\x1C[` \x82\x16\x15a-\xF7Wo\xFF\x97;A\xFA\x98\xC0\x81G.h\x96\xDF\xB2T\xC0\x02`\x80\x1C[`@\x82\x16\x15a.\x16Wo\xFF.\xA1df\xC9j8C\xECx\xB3&\xB5(a\x02`\x80\x1C[`\x80\x82\x16\x15a.5Wo\xFE]\xEE\x04j\x99\xA2\xA8\x11\xC4a\xF1\x96\x9C0S\x02`\x80\x1C[a\x01\0\x82\x16\x15a.UWo\xFC\xBE\x86\xC7\x90\n\x88\xAE\xDC\xFF\xC8;G\x9A\xA3\xA4\x02`\x80\x1C[a\x02\0\x82\x16\x15a.uWo\xF9\x87\xA7%:\xC4\x13\x17o+\x07L\xF7\x81^T\x02`\x80\x1C[a\x04\0\x82\x16\x15a.\x95Wo\xF39+\x08\"\xB7\0\x05\x94\x0Cz9\x8EKp\xF3\x02`\x80\x1C[a\x08\0\x82\x16\x15a.\xB5Wo\xE7\x15\x94u\xA2\xC2\x9BtC\xB2\x9C\x7F\xA6\xE8\x89\xD9\x02`\x80\x1C[a\x10\0\x82\x16\x15a.\xD5Wo\xD0\x97\xF3\xBD\xFD \"\xB8\x84Z\xD8\xF7\x92\xAAX%\x02`\x80\x1C[a \0\x82\x16\x15a.\xF5Wo\xA9\xF7FF-\x87\x0F\xDF\x8Ae\xDC\x1F\x90\xE0a\xE5\x02`\x80\x1C[a@\0\x82\x16\x15a/\x15Wop\xD8i\xA1V\xD2\xA1\xB8\x90\xBB=\xF6+\xAF2\xF7\x02`\x80\x1C[a\x80\0\x82\x16\x15a/5Wo1\xBE\x13_\x97\xD0\x8F\xD9\x81#\x15\x05T/\xCF\xA6\x02`\x80\x1C[b\x01\0\0\x82\x16\x15a/VWo\t\xAAP\x8B[z\x84\xE1\xC6w\xDET\xF3\xE9\x9B\xC9\x02`\x80\x1C[b\x02\0\0\x82\x16\x15a/vWn]j\xF8\xDE\xDB\x81\x19f\x99\xC3)\"^\xE6\x04\x02`\x80\x1C[b\x04\0\0\x82\x16\x15a/\x95Wm\"\x16\xE5\x84\xF5\xFA\x1E\xA9&\x04\x1B\xED\xFE\x98\x02`\x80\x1C[b\x08\0\0\x82\x16\x15a/\xB2Wk\x04\x8A\x17\x03\x91\xF7\xDCBDN\x8F\xA2\x02`\x80\x1C[`\0\x84`\x02\x0B\x13\x15a/\xCDW\x80`\0\x19\x81a/\xC9W\xFE[\x04\x90P[`\x01` \x1B\x81\x06\x15a/\xE0W`\x01a/\xE3V[`\0[`\xFF\x16` \x82\x90\x1C\x01\x92PPP\x91\x90PV[`\0\x80\x80\x80`\x01`\x01`\xA0\x1B\x03\x80\x89\x16\x90\x8A\x16\x10\x15\x81\x87\x12\x80\x15\x90a0zW`\0a0.\x89\x89b\x0FB@\x03b\xFF\xFF\xFF\x16b\x0FB@a2/V[\x90P\x82a0GWa0B\x8C\x8C\x8C`\x01aEbV[a0TV[a0T\x8B\x8D\x8C`\x01aE\xDDV[\x95P\x85\x81\x10a0eW\x8A\x96Pa0tV[a0q\x8C\x8B\x83\x86aF\x88V[\x96P[Pa0\xC4V[\x81a0\x91Wa0\x8C\x8B\x8B\x8B`\0aE\xDDV[a0\x9EV[a0\x9E\x8A\x8C\x8B`\0aEbV[\x93P\x83\x88`\0\x03\x10a0\xB2W\x89\x95Pa0\xC4V[a0\xC1\x8B\x8A\x8A`\0\x03\x85aF\xD4V[\x95P[`\x01`\x01`\xA0\x1B\x03\x8A\x81\x16\x90\x87\x16\x14\x82\x15a1'W\x80\x80\x15a0\xE3WP\x81[a0\xF9Wa0\xF4\x87\x8D\x8C`\x01aE\xDDV[a0\xFBV[\x85[\x95P\x80\x80\x15a1\x08WP\x81\x15[a1\x1EWa1\x19\x87\x8D\x8C`\0aEbV[a1 V[\x84[\x94Pa1qV[\x80\x80\x15a11WP\x81[a1GWa1B\x8C\x88\x8C`\x01aEbV[a1IV[\x85[\x95P\x80\x80\x15a1VWP\x81\x15[a1lWa1g\x8C\x88\x8C`\0aE\xDDV[a1nV[\x84[\x94P[\x81\x15\x80\x15a1\x81WP\x88`\0\x03\x85\x11[\x15a1\x8DW\x88`\0\x03\x94P[\x81\x80\x15a1\xACWP\x8A`\x01`\x01`\xA0\x1B\x03\x16\x87`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x15a1\xBBW\x85\x89\x03\x93Pa1\xD8V[a1\xD5\x86\x89b\xFF\xFF\xFF\x16\x8Ab\x0FB@\x03b\xFF\xFF\xFF\x16a@\xBAV[\x93P[PPP\x95P\x95P\x95P\x95\x91PPV[`\0`\x01`\xFF\x1B\x82\x10a1\xF9W`\0\x80\xFD[P\x90V[\x80\x82\x03\x82\x81\x13\x15`\0\x83\x12\x15\x14a2\x13W`\0\x80\xFD[\x92\x91PPV[\x81\x81\x01\x82\x81\x12\x15`\0\x83\x12\x15\x14a2\x13W`\0\x80\xFD[`\0\x80\x80`\0\x19\x85\x87\t\x86\x86\x02\x92P\x82\x81\x10\x90\x83\x90\x03\x03\x90P\x80a2eW`\0\x84\x11a2ZW`\0\x80\xFD[P\x82\x90\x04\x90Pa2\xD8V[\x80\x84\x11a2qW`\0\x80\xFD[`\0\x84\x86\x88\t`\0\x86\x81\x03\x87\x16\x96\x87\x90\x04\x96`\x02`\x03\x89\x02\x81\x18\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x90\x91\x03\x02\x91\x81\x90\x03\x81\x90\x04`\x01\x01\x86\x84\x11\x90\x95\x03\x94\x90\x94\x02\x91\x90\x94\x03\x92\x90\x92\x04\x91\x90\x91\x17\x91\x90\x91\x02\x91PP[\x93\x92PPPV[`\0\x80c\xFF\xFF\xFF\xFF\x87\x16a3\x85W`\0\x89\x86a\xFF\xFF\x16a\xFF\xFF\x81\x10a3\0W\xFE[`@\x80Q`\x80\x81\x01\x82R\x91\x90\x92\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x80\x84R`\x01` \x1B\x83\x04`\x06\x90\x81\x0B\x81\x0B\x90\x0B` \x85\x01R`\x01`X\x1B\x83\x04`\x01`\x01`\xA0\x1B\x03\x16\x94\x84\x01\x94\x90\x94R`\x01`\xF8\x1B\x90\x91\x04`\xFF\x16\x15\x15``\x83\x01R\x90\x92P\x8A\x16\x14a3qWa3n\x81\x8A\x89\x88aG V[\x90P[\x80` \x01Q\x81`@\x01Q\x92P\x92PPa4eV[\x86\x88\x03`\0\x80a3\x9A\x8C\x8C\x85\x8C\x8C\x8C\x8CaG\xC3V[\x91P\x91P\x81`\0\x01Qc\xFF\xFF\xFF\xFF\x16\x83c\xFF\xFF\xFF\xFF\x16\x14\x15a3\xCCW\x81` \x01Q\x82`@\x01Q\x94P\x94PPPPa4eV[\x80Qc\xFF\xFF\xFF\xFF\x84\x81\x16\x91\x16\x14\x15a3\xF4W\x80` \x01Q\x81`@\x01Q\x94P\x94PPPPa4eV[\x81Q\x81Q` \x80\x85\x01Q\x90\x84\x01Q\x91\x83\x90\x03\x92\x86\x03\x91c\xFF\xFF\xFF\xFF\x80\x84\x16\x92\x90\x85\x16\x91\x03`\x06\x0B\x81a4\"W\xFE[\x05\x02\x84` \x01Q\x01\x82c\xFF\xFF\xFF\xFF\x16\x82c\xFF\xFF\xFF\xFF\x16\x86`@\x01Q\x86`@\x01Q\x03`\x01`\x01`\xA0\x1B\x03\x16\x02\x81a4TW\xFE[\x04\x85`@\x01Q\x01\x96P\x96PPPPPP[\x97P\x97\x95PPPPPPV[`\x02\x95\x86\x0B\x86\x0B`\0\x90\x81R` \x97\x90\x97R`@\x90\x96 `\x01\x81\x01\x80T\x90\x95\x03\x90\x94U\x93\x83\x01\x80T\x90\x92\x03\x90\x91U`\x03\x82\x01\x80Tc\xFF\xFF\xFF\xFF`\x01`\xD8\x1B`\x01`\x01`\xA0\x1B\x03`\x01`8\x1B\x80\x85\x04\x82\x16\x90\x96\x03\x16\x90\x94\x02`\x01`8\x1B`\x01`\xD8\x1B\x03\x19\x90\x92\x16\x91\x90\x91\x17`\x06\x81\x81\x0B\x90\x96\x03\x90\x95\x0Bf\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16f\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x95\x16\x94\x90\x94\x17\x82\x81\x04\x85\x16\x90\x95\x03\x90\x93\x16\x02c\xFF\xFF\xFF\xFF`\xD8\x1B\x19\x90\x93\x16\x92\x90\x92\x17\x90UT`\x01`\x80\x1B\x90\x04`\x0F\x0B\x90V[`\0\x80\x82`\x0F\x0B\x12\x15a5\x90W\x82`\x01`\x01`\x80\x1B\x03\x16\x82`\0\x03\x84\x03\x91P\x81`\x01`\x01`\x80\x1B\x03\x16\x10a5\x8BW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x02`$\x82\x01RaLS`\xF0\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[a2\x13V[\x82`\x01`\x01`\x80\x1B\x03\x16\x82\x84\x01\x91P\x81`\x01`\x01`\x80\x1B\x03\x16\x10\x15a2\x13W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x02`$\x82\x01RaLA`\xF0\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\0d\x01\0\x02v\xA3`\x01`\x01`\xA0\x1B\x03\x83\x16\x10\x80\x15\x90a6\x1DWPs\xFF\xFD\x89c\xEF\xD1\xFCjPd\x88I]\x95\x1DRc\x98\x8D&`\x01`\x01`\xA0\x1B\x03\x83\x16\x10[a6RW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x01`$\x82\x01R`)`\xF9\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[d\x01\0\0\0\0`\x01`\xC0\x1B\x03` \x83\x90\x1B\x16`\x01`\x01`\x80\x1B\x03\x81\x11`\x07\x1B\x81\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11`\x06\x1B\x90\x81\x1Cc\xFF\xFF\xFF\xFF\x81\x11`\x05\x1B\x90\x81\x1Ca\xFF\xFF\x81\x11`\x04\x1B\x90\x81\x1C`\xFF\x81\x11`\x03\x90\x81\x1B\x91\x82\x1C`\x0F\x81\x11`\x02\x1B\x90\x81\x1C\x91\x82\x11`\x01\x90\x81\x1B\x92\x83\x1C\x97\x90\x88\x11\x96\x17\x90\x94\x17\x90\x92\x17\x17\x90\x91\x17\x17\x17`\x80\x81\x10a6\xE6W`\x7F\x81\x03\x83\x90\x1C\x91Pa6\xF0V[\x80`\x7F\x03\x83\x90\x1B\x91P[\x90\x80\x02`\x7F\x81\x81\x1C`\xFF\x83\x81\x1C\x91\x90\x91\x1C\x80\x02\x80\x83\x1C\x81\x83\x1C\x1C\x80\x02\x80\x84\x1C\x81\x84\x1C\x1C\x80\x02\x80\x85\x1C\x81\x85\x1C\x1C\x80\x02\x80\x86\x1C\x81\x86\x1C\x1C\x80\x02\x80\x87\x1C\x81\x87\x1C\x1C\x80\x02\x80\x88\x1C\x81\x88\x1C\x1C\x80\x02\x80\x89\x1C\x81\x89\x1C\x1C\x80\x02\x80\x8A\x1C\x81\x8A\x1C\x1C\x80\x02\x80\x8B\x1C\x81\x8B\x1C\x1C\x80\x02\x80\x8C\x1C\x81\x8C\x1C\x1C\x80\x02\x80\x8D\x1C\x81\x8D\x1C\x1C\x80\x02\x80\x8E\x1C\x9C\x81\x90\x1C\x9C\x90\x9C\x1C\x80\x02\x9C\x8D\x90\x1C\x9E\x9D`\x7F\x19\x8F\x01`@\x1B`\xC0\x91\x90\x91\x1Cg\x80\0\0\0\0\0\0\0\x16\x17`\xC1\x9B\x90\x9B\x1Cg@\0\0\0\0\0\0\0\x16\x9A\x90\x9A\x17`\xC2\x99\x90\x99\x1Cg \0\0\0\0\0\0\0\x16\x98\x90\x98\x17`\xC3\x97\x90\x97\x1Cg\x10\0\0\0\0\0\0\0\x16\x96\x90\x96\x17`\xC4\x95\x90\x95\x1Cg\x08\0\0\0\0\0\0\0\x16\x94\x90\x94\x17`\xC5\x93\x90\x93\x1Cg\x04\0\0\0\0\0\0\0\x16\x92\x90\x92\x17`\xC6\x91\x90\x91\x1Cg\x02\0\0\0\0\0\0\0\x16\x17`\xC7\x91\x90\x91\x1C`\x01`8\x1B\x16\x17`\xC8\x91\x90\x91\x1Cf\x80\0\0\0\0\0\0\x16\x17`\xC9\x91\x90\x91\x1Cf@\0\0\0\0\0\0\x16\x17`\xCA\x91\x90\x91\x1Cf \0\0\0\0\0\0\x16\x17`\xCB\x91\x90\x91\x1Cf\x10\0\0\0\0\0\0\x16\x17`\xCC\x91\x90\x91\x1Cf\x08\0\0\0\0\0\0\x16\x17`\xCD\x91\x90\x91\x1Cf\x04\0\0\0\0\0\0\x16\x17i6'\xA3\x01\xD7\x10UwL\x85\x81\x02o\x02\x8Fd\x81\xAB\x7F\x04ZZ\xF0\x12\xA1\x9D\0:\xA9\x19\x81\x01`\x80\x90\x81\x1D\x90o\xDB-\xF0\x9E\x81\x95\x9A\x81E^&\x07\x99\xA0c/\x83\x01\x90\x1D`\x02\x81\x81\x0B\x90\x83\x90\x0B\x14a8\xEDW\x88`\x01`\x01`\xA0\x1B\x03\x16a8\xD1\x82a,\xC4V[`\x01`\x01`\xA0\x1B\x03\x16\x11\x15a8\xE6W\x81a8\xE8V[\x80[a8\xEFV[\x81[\x99\x98PPPPPPPPPV[`\0\x80`\0\x89\x89a\xFF\xFF\x16a\xFF\xFF\x81\x10a9\x12W\xFE[`@\x80Q`\x80\x81\x01\x82R\x91\x90\x92\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x80\x84R`\x01` \x1B\x83\x04`\x06\x90\x81\x0B\x81\x0B\x90\x0B` \x85\x01R`\x01`X\x1B\x83\x04`\x01`\x01`\xA0\x1B\x03\x16\x94\x84\x01\x94\x90\x94R`\x01`\xF8\x1B\x90\x91\x04`\xFF\x16\x15\x15``\x83\x01R\x90\x92P\x89\x16\x14\x15a9\x81W\x88\x85\x92P\x92PPa4eV[\x84a\xFF\xFF\x16\x84a\xFF\xFF\x16\x11\x80\x15a9\xA2WP`\x01\x85\x03a\xFF\xFF\x16\x89a\xFF\xFF\x16\x14[\x15a9\xAFW\x83\x91Pa9\xB3V[\x84\x91P[\x81a\xFF\xFF\x16\x89`\x01\x01a\xFF\xFF\x16\x81a9\xC7W\xFE[\x06\x92Pa9\xD6\x81\x89\x89\x89aG V[\x8A\x84a\xFF\xFF\x16a\xFF\xFF\x81\x10a9\xE7W\xFE[\x82Q\x91\x01\x80T` \x84\x01Q`@\x85\x01Q``\x90\x95\x01Q\x15\x15`\x01`\xF8\x1B\x02`\x01`\x01`\xF8\x1B\x03`\x01`\x01`\xA0\x1B\x03\x90\x96\x16`\x01`X\x1B\x02\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x06\x93\x90\x93\x0Bf\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x01` \x1B\x02j\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\x19c\xFF\xFF\xFF\xFF\x90\x97\x16c\xFF\xFF\xFF\xFF\x19\x90\x95\x16\x94\x90\x94\x17\x95\x90\x95\x16\x92\x90\x92\x17\x16\x92\x90\x92\x17\x92\x90\x92\x16\x17\x90UP\x97P\x97\x95PPPPPPV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`$\x83\x01R`D\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`d\x90\x92\x01\x83R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\xA9\x05\x9C\xBB`\xE0\x1B\x17\x81R\x92Q\x82Q`\0\x94\x85\x94\x93\x89\x16\x93\x92\x91\x82\x91\x90\x80\x83\x83[` \x83\x10a;\x13W\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01a:\xF4V[`\x01\x83` \x03a\x01\0\n\x03\x80\x19\x82Q\x16\x81\x84Q\x16\x80\x82\x17\x85RPPPPPP\x90P\x01\x91PP`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a;uW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a;zV[``\x91P[P\x91P\x91P\x81\x80\x15a;\xA8WP\x80Q\x15\x80a;\xA8WP\x80\x80` \x01\x90Q` \x81\x10\x15a;\xA5W`\0\x80\xFD[PQ[a;\xDEW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x02`$\x82\x01Ra*#`\xF1\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[PPPPPV[`@\x80Q0`$\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`D\x90\x91\x01\x82R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16cp\xA0\x821`\xE0\x1B\x17\x81R\x91Q\x81Q`\0\x93\x84\x93\x84\x93`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x93\x91\x92\x90\x91\x82\x91\x90\x80\x83\x83[` \x83\x10a<~W\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01a<_V[`\x01\x83` \x03a\x01\0\n\x03\x80\x19\x82Q\x16\x81\x84Q\x16\x80\x82\x17\x85RPPPPPP\x90P\x01\x91PP`\0`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80`\0\x81\x14a<\xDEW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a<\xE3V[``\x91P[P\x91P\x91P\x81\x80\x15a<\xF7WP` \x81Q\x10\x15[a=\0W`\0\x80\xFD[\x80\x80` \x01\x90Q` \x81\x10\x15a=\x15W`\0\x80\xFD[PQ\x92PPP\x90V[\x80\x82\x01\x82\x81\x10\x15a2\x13W`\0\x80\xFD[`@\x80Q0`$\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`D\x90\x91\x01\x82R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16cp\xA0\x821`\xE0\x1B\x17\x81R\x91Q\x81Q`\0\x93\x84\x93\x84\x93`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x93\x91\x92\x90\x91\x82\x91\x90\x80\x83\x83` \x83\x10a<~W\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01a<_V[`\0\x80\x83a\xFF\xFF\x16\x11a>\x04W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x01`$\x82\x01R`I`\xF8\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x82a\xFF\xFF\x16\x82a\xFF\xFF\x16\x11a>\x1AWP\x81a2\xD8V[\x82[\x82a\xFF\xFF\x16\x81a\xFF\xFF\x16\x10\x15a>`W`\x01\x85\x82a\xFF\xFF\x16a\xFF\xFF\x81\x10a>?W\xFE[\x01\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90U`\x01\x01a>\x1CV[P\x90\x93\x92PPPV[\x80`\x0F\x81\x90\x0B\x81\x14a+BW`\0\x80\xFD[`\0\x80`\0a>\x87a+GV[a>\x99\x84` \x01Q\x85`@\x01QaB\xB2V[`@\x80Q`\xE0\x81\x01\x82R`\0T`\x01`\x01`\xA0\x1B\x03\x81\x16\x82R`\x01`\xA0\x1B\x81\x04`\x02\x90\x81\x0B\x81\x0B\x90\x0B` \x80\x84\x01\x82\x90Ra\xFF\xFF`\x01`\xB8\x1B\x84\x04\x81\x16\x85\x87\x01R`\x01`\xC8\x1B\x84\x04\x81\x16``\x80\x87\x01\x91\x90\x91R`\x01`\xD8\x1B\x85\x04\x90\x91\x16`\x80\x86\x01R`\xFF`\x01`\xE8\x1B\x85\x04\x81\x16`\xA0\x87\x01R`\x01`\xF0\x1B\x90\x94\x04\x90\x93\x16\x15\x15`\xC0\x85\x01R\x88Q\x90\x89\x01Q\x94\x89\x01Q\x92\x89\x01Q\x93\x94a?=\x94\x91\x93\x90\x92\x90\x91\x90aI\xBDV[\x93P\x84``\x01Q`\x0F\x0B`\0\x14a@\xB2W\x84` \x01Q`\x02\x0B\x81` \x01Q`\x02\x0B\x12\x15a?\x92Wa?\x8Ba?t\x86` \x01Qa,\xC4V[a?\x81\x87`@\x01Qa,\xC4V[\x87``\x01QaKrV[\x92Pa@\xB2V[\x84`@\x01Q`\x02\x0B\x81` \x01Q`\x02\x0B\x12\x15a@\x88W`\x04T`@\x82\x01Q`\x01`\x01`\x80\x1B\x03\x90\x91\x16\x90a?\xE4\x90a?\xC8a+~V[` \x85\x01Q``\x86\x01Q`\x80\x87\x01Q`\x08\x94\x93\x92\x91\x87\x91a8\xFCV[`\0\x80Ta\xFF\xFF`\xC8\x1B\x19\x16`\x01`\xC8\x1Ba\xFF\xFF\x93\x84\x16\x02\x17a\xFF\xFF`\xB8\x1B\x19\x16`\x01`\xB8\x1B\x93\x90\x92\x16\x92\x90\x92\x02\x17\x90U\x81Q`@\x87\x01Qa@4\x91\x90a@*\x90a,\xC4V[\x88``\x01QaKrV[\x93Pa@Ra@F\x87` \x01Qa,\xC4V[\x83Q``\x89\x01QaK\xB6V[\x92Pa@b\x81\x87``\x01Qa5+V[`\x04\x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UPa@\xB2V[a@\xAFa@\x98\x86` \x01Qa,\xC4V[a@\xA5\x87`@\x01Qa,\xC4V[\x87``\x01QaK\xB6V[\x91P[P\x91\x93\x90\x92PV[`\0a@\xC7\x84\x84\x84a2/V[\x90P`\0\x82\x80a@\xD3W\xFE[\x84\x86\t\x11\x15a2\xD8W`\0\x19\x81\x10a@\xEAW`\0\x80\xFD[`\x01\x01\x93\x92PPPV[`@\x80Q``\x94\x90\x94\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16` \x80\x86\x01\x91\x90\x91R`\x02\x93\x84\x0B`\xE8\x90\x81\x1B`4\x87\x01R\x92\x90\x93\x0B\x90\x91\x1B`7\x84\x01R\x80Q\x80\x84\x03`\x1A\x01\x81R`:\x90\x93\x01\x81R\x82Q\x92\x82\x01\x92\x90\x92 `\0\x90\x81R\x92\x90R\x90 \x90V[``\x80`\0\x83a\xFF\xFF\x16\x11aA\x98W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x01`$\x82\x01R`I`\xF8\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x86Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x80\x15aA\xB0W`\0\x80\xFD[P`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15aA\xDAW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x91P\x86Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x80\x15aA\xF5W`\0\x80\xFD[P`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15aB\x1FW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x87Q\x81\x10\x15aB\xA5WaBP\x8A\x8A\x8A\x84\x81Q\x81\x10aB?W\xFE[` \x02` \x01\x01Q\x8A\x8A\x8A\x8Aa2\xDFV[\x84\x83\x81Q\x81\x10aB\\W\xFE[` \x02` \x01\x01\x84\x84\x81Q\x81\x10aBoW\xFE[` \x02` \x01\x01\x82`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81RP\x82`\x06\x0B`\x06\x0B\x81RPPP\x80\x80`\x01\x01\x91PPaB%V[P\x97P\x97\x95PPPPPPV[\x80`\x02\x0B\x82`\x02\x0B\x12aB\xF2W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x03`$\x82\x01RbTLU`\xE8\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[b\r\x89\xE7\x19`\x02\x83\x90\x0B\x12\x15aC5W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x03`$\x82\x01RbTLM`\xE8\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[b\r\x89\xE8`\x02\x82\x90\x0B\x13\x15aCwW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x03`$\x82\x01RbTUM`\xE8\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[PPV[`@\x80Q`\x80\x81\x01\x82Rc\xFF\xFF\xFF\xFF\x92\x83\x16\x80\x82R`\0` \x83\x01\x81\x90R\x92\x82\x01\x92\x90\x92R`\x01``\x90\x91\x01\x81\x90R\x83Tc\xFF\xFF\xFF\xFF\x19\x16\x90\x91\x17\x90\x91\x16`\x01`\xF8\x1B\x17\x90\x91U\x90\x81\x90V[`\x02\x0B`\x08\x81\x90\x1D\x91a\x01\0\x90\x91\x07\x90V[`\0\x80\x82\x11aC\xE7W`\0\x80\xFD[`\x01`\x80\x1B\x82\x10aC\xFAW`\x80\x91\x82\x1C\x91\x01[h\x01\0\0\0\0\0\0\0\0\x82\x10aD\x12W`@\x91\x82\x1C\x91\x01[`\x01` \x1B\x82\x10aD%W` \x91\x82\x1C\x91\x01[b\x01\0\0\x82\x10aD7W`\x10\x91\x82\x1C\x91\x01[a\x01\0\x82\x10aDHW`\x08\x91\x82\x1C\x91\x01[`\x10\x82\x10aDXW`\x04\x91\x82\x1C\x91\x01[`\x04\x82\x10aDhW`\x02\x91\x82\x1C\x91\x01[`\x02\x82\x10a+BW`\x01\x01\x91\x90PV[`\0\x80\x82\x11aD\x86W`\0\x80\xFD[P`\xFF`\x01`\x01`\x80\x1B\x03\x82\x16\x15aD\xA1W`\x7F\x19\x01aD\xA9V[`\x80\x82\x90\x1C\x91P[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x15aD\xC2W`?\x19\x01aD\xCAV[`@\x82\x90\x1C\x91P[c\xFF\xFF\xFF\xFF\x82\x16\x15aD\xDFW`\x1F\x19\x01aD\xE7V[` \x82\x90\x1C\x91P[a\xFF\xFF\x82\x16\x15aD\xFAW`\x0F\x19\x01aE\x02V[`\x10\x82\x90\x1C\x91P[`\xFF\x82\x16\x15aE\x14W`\x07\x19\x01aE\x1CV[`\x08\x82\x90\x1C\x91P[`\x0F\x82\x16\x15aE.W`\x03\x19\x01aE6V[`\x04\x82\x90\x1C\x91P[`\x03\x82\x16\x15aEHW`\x01\x19\x01aEPV[`\x02\x82\x90\x1C\x91P[`\x01\x82\x16\x15a+BW`\0\x19\x01\x91\x90PV[`\0\x83`\x01`\x01`\xA0\x1B\x03\x16\x85`\x01`\x01`\xA0\x1B\x03\x16\x11\x15aE\x82W\x92\x93\x92[\x81aE\xAFWaE\xAA\x83`\x01`\x01`\x80\x1B\x03\x16\x86\x86\x03`\x01`\x01`\xA0\x1B\x03\x16`\x01``\x1Ba2/V[aE\xD2V[aE\xD2\x83`\x01`\x01`\x80\x1B\x03\x16\x86\x86\x03`\x01`\x01`\xA0\x1B\x03\x16`\x01``\x1Ba@\xBAV[\x90P[\x94\x93PPPPV[`\0\x83`\x01`\x01`\xA0\x1B\x03\x16\x85`\x01`\x01`\xA0\x1B\x03\x16\x11\x15aE\xFDW\x92\x93\x92[`\x01``\x1B`\x01`\xE0\x1B\x03``\x84\x90\x1B\x16`\x01`\x01`\xA0\x1B\x03\x86\x86\x03\x81\x16\x90\x87\x16aF'W`\0\x80\xFD[\x83aFWW\x86`\x01`\x01`\xA0\x1B\x03\x16aFJ\x83\x83\x89`\x01`\x01`\xA0\x1B\x03\x16a2/V[\x81aFQW\xFE[\x04aF}V[aF}aFn\x83\x83\x89`\x01`\x01`\xA0\x1B\x03\x16a@\xBAV[\x88`\x01`\x01`\xA0\x1B\x03\x16aK\xE5V[\x97\x96PPPPPPPV[`\0\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x11aF\x9FW`\0\x80\xFD[`\0\x84`\x01`\x01`\x80\x1B\x03\x16\x11aF\xB5W`\0\x80\xFD[\x81aF\xC7WaE\xAA\x85\x85\x85`\x01aK\xF0V[aE\xD2\x85\x85\x85`\x01aL\xD1V[`\0\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x11aF\xEBW`\0\x80\xFD[`\0\x84`\x01`\x01`\x80\x1B\x03\x16\x11aG\x01W`\0\x80\xFD[\x81aG\x13WaE\xAA\x85\x85\x85`\0aL\xD1V[aE\xD2\x85\x85\x85`\0aK\xF0V[aG(aT\xF1V[`\0\x85`\0\x01Q\x85\x03\x90P`@Q\x80`\x80\x01`@R\x80\x86c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x82c\xFF\xFF\xFF\xFF\x16\x86`\x02\x0B\x02\x88` \x01Q\x01`\x06\x0B\x81R` \x01`\0\x85`\x01`\x01`\x80\x1B\x03\x16\x11aG|W`\x01aG~V[\x84[`\x01`\x01`\x80\x1B\x03\x16c\xFF\xFF\xFF\xFF`\x80\x1B`\x80\x85\x90\x1B\x16\x81aG\x9CW\xFE[\x04\x88`@\x01Q\x01`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\x01\x15\x15\x81RP\x91PP\x94\x93PPPPV[aG\xCBaT\xF1V[aG\xD3aT\xF1V[\x88\x85a\xFF\xFF\x16a\xFF\xFF\x81\x10aG\xE4W\xFE[`@\x80Q`\x80\x81\x01\x82R\x91\x90\x92\x01Tc\xFF\xFF\xFF\xFF\x81\x16\x80\x83R`\x01` \x1B\x82\x04`\x06\x90\x81\x0B\x81\x0B\x90\x0B` \x84\x01R`\x01`X\x1B\x82\x04`\x01`\x01`\xA0\x1B\x03\x16\x93\x83\x01\x93\x90\x93R`\x01`\xF8\x1B\x90\x04`\xFF\x16\x15\x15``\x82\x01R\x92PaHH\x90\x89\x90\x89aM\xB4V[\x15aH\x80W\x86c\xFF\xFF\xFF\xFF\x16\x82`\0\x01Qc\xFF\xFF\xFF\xFF\x16\x14\x15aHjWa4eV[\x81aHw\x83\x89\x89\x88aG V[\x91P\x91Pa4eV[\x88\x83a\xFF\xFF\x16\x86`\x01\x01a\xFF\xFF\x16\x81aH\x95W\xFE[\x06a\xFF\xFF\x16a\xFF\xFF\x81\x10aH\xA5W\xFE[`@\x80Q`\x80\x81\x01\x82R\x92\x90\x91\x01Tc\xFF\xFF\xFF\xFF\x81\x16\x83R`\x01` \x1B\x81\x04`\x06\x90\x81\x0B\x81\x0B\x90\x0B` \x84\x01R`\x01`\x01`\xA0\x1B\x03`\x01`X\x1B\x82\x04\x16\x91\x83\x01\x91\x90\x91R`\xFF`\x01`\xF8\x1B\x90\x91\x04\x16\x15\x15``\x82\x01\x81\x90R\x90\x92PaIZW`@\x80Q`\x80\x81\x01\x82R\x8ATc\xFF\xFF\xFF\xFF\x81\x16\x82R`\x01` \x1B\x81\x04`\x06\x90\x81\x0B\x81\x0B\x90\x0B` \x83\x01R`\x01`X\x1B\x81\x04`\x01`\x01`\xA0\x1B\x03\x16\x92\x82\x01\x92\x90\x92R`\x01`\xF8\x1B\x90\x91\x04`\xFF\x16\x15\x15``\x82\x01R\x91P[aIi\x88\x83`\0\x01Q\x89aM\xB4V[aI\xA0W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x03`$\x82\x01Rb\x13\xD3\x11`\xEA\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[aI\xAD\x89\x89\x89\x88\x87aNuV[\x91P\x91P\x97P\x97\x95PPPPPPV[`\0aI\xCC`\x07\x87\x87\x87a@\xF4V[`\x01T`\x02T\x91\x92P\x90`\0\x80`\x0F\x87\x90\x0B\x15aK\x12W`\0aI\xEDa+~V[`\0\x80T`\x04T\x92\x93P\x90\x91\x82\x91aJ7\x91`\x08\x91\x86\x91\x85\x91`\x01`\xA0\x1B\x81\x04`\x02\x0B\x91a\xFF\xFF`\x01`\xB8\x1B\x83\x04\x81\x16\x92`\x01`\x01`\x80\x1B\x03\x90\x92\x16\x91`\x01`\xC8\x1B\x90\x04\x16a2\xDFV[\x90\x92P\x90PaJq`\x05\x8D\x8B\x8D\x8B\x8B\x87\x89\x8B`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0aP\x13V[\x94PaJ\xA8`\x05\x8C\x8B\x8D\x8B\x8B\x87\x89\x8B`\x01\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0aP\x13V[\x93P\x84\x15aJ\xDCWaJ\xDC`\x06\x8D\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0aQ\xCCV[\x83\x15aK\x0EWaK\x0E`\x06\x8C\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0aQ\xCCV[PPP[`\0\x80aK$`\x05\x8C\x8C\x8B\x8A\x8AaR2V[\x90\x92P\x90PaK5\x87\x8A\x84\x84aR\xDEV[`\0\x89`\x0F\x0B\x12\x15aKcW\x83\x15aKRWaKR`\x05\x8CaTsV[\x82\x15aKcWaKc`\x05\x8BaTsV[PPPPPP\x95\x94PPPPPV[`\0\x80\x82`\x0F\x0B\x12aK\x98WaK\x93aK\x8E\x85\x85\x85`\x01aE\xDDV[a1\xE7V[aE\xD5V[aK\xABaK\x8E\x85\x85\x85`\0\x03`\0aE\xDDV[`\0\x03\x94\x93PPPPV[`\0\x80\x82`\x0F\x0B\x12aK\xD2WaK\x93aK\x8E\x85\x85\x85`\x01aEbV[aK\xABaK\x8E\x85\x85\x85`\0\x03`\0aEbV[\x80\x82\x04\x91\x06\x15\x15\x01\x90V[`\0\x81\x15aLcW`\0`\x01`\x01`\xA0\x1B\x03\x84\x11\x15aL&WaL!\x84`\x01``\x1B\x87`\x01`\x01`\x80\x1B\x03\x16a2/V[aL>V[`\x01`\x01`\x80\x1B\x03\x85\x16``\x85\x90\x1B\x81aL<W\xFE[\x04[\x90PaL[aLV`\x01`\x01`\xA0\x1B\x03\x88\x16\x83a=\x1EV[aT\x9FV[\x91PPaE\xD5V[`\0`\x01`\x01`\xA0\x1B\x03\x84\x11\x15aL\x91WaL\x8C\x84`\x01``\x1B\x87`\x01`\x01`\x80\x1B\x03\x16a@\xBAV[aL\xA8V[aL\xA8``\x85\x90\x1B`\x01`\x01`\x80\x1B\x03\x87\x16aK\xE5V[\x90P\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x11aL\xBFW`\0\x80\xFD[`\x01`\x01`\xA0\x1B\x03\x86\x16\x03\x90PaE\xD5V[`\0\x82aL\xDFWP\x83aE\xD5V[`\x01``\x1B`\x01`\xE0\x1B\x03``\x85\x90\x1B\x16\x82\x15aMmW`\x01`\x01`\xA0\x1B\x03\x86\x16\x84\x81\x02\x90\x85\x82\x81aM\rW\xFE[\x04\x14\x15aM>W\x81\x81\x01\x82\x81\x10aM<WaM2\x83\x89`\x01`\x01`\xA0\x1B\x03\x16\x83a@\xBAV[\x93PPPPaE\xD5V[P[aMd\x82aM_\x87\x8A`\x01`\x01`\xA0\x1B\x03\x16\x86\x81aMXW\xFE[\x04\x90a=\x1EV[aK\xE5V[\x92PPPaE\xD5V[`\x01`\x01`\xA0\x1B\x03\x86\x16\x84\x81\x02\x90\x85\x82\x81aM\x84W\xFE[\x04\x14\x80\x15aM\x91WP\x80\x82\x11[aM\x9AW`\0\x80\xFD[\x80\x82\x03aM2aLV\x84`\x01`\x01`\xA0\x1B\x03\x8B\x16\x84a@\xBAV[`\0\x83c\xFF\xFF\xFF\xFF\x16\x83c\xFF\xFF\xFF\xFF\x16\x11\x15\x80\x15aM\xDEWP\x83c\xFF\xFF\xFF\xFF\x16\x82c\xFF\xFF\xFF\xFF\x16\x11\x15[\x15aM\xFAW\x81c\xFF\xFF\xFF\xFF\x16\x83c\xFF\xFF\xFF\xFF\x16\x11\x15\x90Pa2\xD8V[`\0\x84c\xFF\xFF\xFF\xFF\x16\x84c\xFF\xFF\xFF\xFF\x16\x11aN!W\x83c\xFF\xFF\xFF\xFF\x16`\x01` \x1B\x01aN)V[\x83c\xFF\xFF\xFF\xFF\x16[d\xFF\xFF\xFF\xFF\xFF\x16\x90P`\0\x85c\xFF\xFF\xFF\xFF\x16\x84c\xFF\xFF\xFF\xFF\x16\x11aNYW\x83c\xFF\xFF\xFF\xFF\x16`\x01` \x1B\x01aNaV[\x83c\xFF\xFF\xFF\xFF\x16[d\xFF\xFF\xFF\xFF\xFF\x16\x90\x91\x11\x15\x95\x94PPPPPV[aN}aT\xF1V[aN\x85aT\xF1V[`\0\x83a\xFF\xFF\x16\x85`\x01\x01a\xFF\xFF\x16\x81aN\x9BW\xFE[\x06a\xFF\xFF\x16\x90P`\0`\x01\x85a\xFF\xFF\x16\x83\x01\x03\x90P`\0[P`\x02\x81\x83\x01\x04\x89a\xFF\xFF\x87\x16\x82\x81aN\xC8W\xFE[\x06a\xFF\xFF\x81\x10aN\xD4W\xFE[`@\x80Q`\x80\x81\x01\x82R\x92\x90\x91\x01Tc\xFF\xFF\xFF\xFF\x81\x16\x83R`\x01` \x1B\x81\x04`\x06\x90\x81\x0B\x81\x0B\x90\x0B` \x84\x01R`\x01`\x01`\xA0\x1B\x03`\x01`X\x1B\x82\x04\x16\x91\x83\x01\x91\x90\x91R`\xFF`\x01`\xF8\x1B\x90\x91\x04\x16\x15\x15``\x82\x01\x81\x90R\x90\x95PaO>W\x80`\x01\x01\x92PaN\xB3V[\x89\x86a\xFF\xFF\x16\x82`\x01\x01\x81aOOW\xFE[\x06a\xFF\xFF\x81\x10aO[W\xFE[`@\x80Q`\x80\x81\x01\x82R\x92\x90\x91\x01Tc\xFF\xFF\xFF\xFF\x81\x16\x83R`\x01` \x1B\x81\x04`\x06\x90\x81\x0B\x81\x0B\x90\x0B` \x84\x01R`\x01`\x01`\xA0\x1B\x03`\x01`X\x1B\x82\x04\x16\x91\x83\x01\x91\x90\x91R`\xFF`\x01`\xF8\x1B\x90\x91\x04\x16\x15\x15``\x82\x01R\x85Q\x90\x94P`\0\x90aO\xC5\x90\x8B\x90\x8BaM\xB4V[\x90P\x80\x80\x15aO\xDEWPaO\xDE\x8A\x8A\x87`\0\x01QaM\xB4V[\x15aO\xE9WPaP\x06V[\x80aO\xF9W`\x01\x82\x03\x92PaP\0V[\x81`\x01\x01\x93P[PaN\xB3V[PPP\x95P\x95\x93PPPPV[`\x02\x8A\x81\x0B\x90\x0B`\0\x90\x81R` \x8C\x90R`@\x81 \x80T`\x01`\x01`\x80\x1B\x03\x16\x82aP>\x82\x8Da5+V[\x90P\x84`\x01`\x01`\x80\x1B\x03\x16\x81`\x01`\x01`\x80\x1B\x03\x16\x11\x15aP\x8CW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x02`$\x82\x01RaLO`\xF0\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\x01`\x01`\x80\x1B\x03\x82\x81\x16\x15\x90\x82\x16\x15\x81\x14\x15\x94P\x15aQ1W\x8C`\x02\x0B\x8E`\x02\x0B\x13aQ\x19W`\x01\x83\x01\x8B\x90U`\x02\x83\x01\x8A\x90U`\x03\x83\x01\x80T`\x01`8\x1B`\x01`\xD8\x1B\x03\x19\x16`\x01`8\x1B`\x01`\x01`\xA0\x1B\x03\x8C\x16\x02\x17f\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16f\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x06\x8B\x90\x0B\x16\x17c\xFF\xFF\xFF\xFF`\xD8\x1B\x19\x16`\x01`\xD8\x1Bc\xFF\xFF\xFF\xFF\x8A\x16\x02\x17\x90U[`\x03\x83\x01\x80T`\x01`\x01`\xF8\x1B\x03\x16`\x01`\xF8\x1B\x17\x90U[\x82T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x82\x16\x17\x83U\x85aQzW\x82TaQu\x90aQp\x90`\x01`\x80\x1B\x90\x04`\x0F\x90\x81\x0B\x81\x0B\x90\x8F\x90\x0Ba2\x19V[a>iV[aQ\x9BV[\x82TaQ\x9B\x90aQp\x90`\x01`\x80\x1B\x90\x04`\x0F\x90\x81\x0B\x81\x0B\x90\x8F\x90\x0Ba1\xFDV[\x83T`\x0F\x91\x90\x91\x0B`\x01`\x01`\x80\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02\x91\x16\x17\x90\x92UP\x90\x9C\x9BPPPPPPPPPPPPV[\x80`\x02\x0B\x82`\x02\x0B\x81aQ\xDBW\xFE[\x07`\x02\x0B\x15aQ\xE9W`\0\x80\xFD[`\0\x80aR\x04\x83`\x02\x0B\x85`\x02\x0B\x81aQ\xFEW\xFE[\x05aC\xC7V[`\x01\x91\x82\x0B\x82\x0B`\0\x90\x81R` \x97\x90\x97R`@\x90\x96 \x80T`\xFF\x90\x97\x16\x91\x90\x91\x1B\x90\x95\x18\x90\x94UPPPPV[`\x02\x85\x81\x0B\x80\x82\x0B`\0\x90\x81R` \x89\x90R`@\x80\x82 \x88\x85\x0B\x85\x0B\x83R\x90\x82 \x91\x93\x84\x93\x91\x92\x91\x84\x91\x82\x91\x90\x8A\x90\x0B\x12aRxWPP`\x01\x82\x01T`\x02\x83\x01TaR\x8BV[\x83`\x01\x01T\x88\x03\x91P\x83`\x02\x01T\x87\x03\x90P[`\0\x80\x8B`\x02\x0B\x8B`\x02\x0B\x12\x15aR\xADWPP`\x01\x83\x01T`\x02\x84\x01TaR\xC0V[\x84`\x01\x01T\x8A\x03\x91P\x84`\x02\x01T\x89\x03\x90P[\x92\x90\x98\x03\x97\x90\x97\x03\x9B\x96\x90\x95\x03\x94\x90\x94\x03\x98P\x93\x96PPPPPPPV[`@\x80Q`\xA0\x81\x01\x82R\x85T`\x01`\x01`\x80\x1B\x03\x90\x81\x16\x82R`\x01\x87\x01T` \x83\x01R`\x02\x87\x01T\x92\x82\x01\x92\x90\x92R`\x03\x86\x01T\x80\x83\x16``\x83\x01R`\x01`\x80\x1B\x90\x04\x90\x91\x16`\x80\x82\x01R`\0`\x0F\x85\x90\x0BaS}W\x81Q`\x01`\x01`\x80\x1B\x03\x16aSuW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x02`$\x82\x01Ra\x04\xE5`\xF4\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[P\x80QaS\x8CV[\x81QaS\x89\x90\x86a5+V[\x90P[`\0aS\xB0\x83` \x01Q\x86\x03\x84`\0\x01Q`\x01`\x01`\x80\x1B\x03\x16`\x01`\x80\x1Ba2/V[\x90P`\0aS\xD6\x84`@\x01Q\x86\x03\x85`\0\x01Q`\x01`\x01`\x80\x1B\x03\x16`\x01`\x80\x1Ba2/V[\x90P\x86`\x0F\x0B`\0\x14aS\xFDW\x87T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x84\x16\x17\x88U[`\x01\x88\x01\x86\x90U`\x02\x88\x01\x85\x90U`\x01`\x01`\x80\x1B\x03\x82\x16\x15\x15\x80aT+WP`\0\x81`\x01`\x01`\x80\x1B\x03\x16\x11[\x15aTiW`\x03\x88\x01\x80T`\x01`\x01`\x80\x1B\x03\x19\x81\x16`\x01`\x01`\x80\x1B\x03\x91\x82\x16\x85\x01\x82\x16\x17\x80\x82\x16`\x01`\x80\x1B\x91\x82\x90\x04\x83\x16\x85\x01\x90\x92\x16\x02\x17\x90U[PPPPPPPPV[`\x02\x90\x81\x0B\x81\x0B`\0\x90\x81R` \x92\x90\x92R`@\x82 \x82\x81U`\x01\x81\x01\x83\x90U\x90\x81\x01\x82\x90U`\x03\x01UV[\x80`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a+BW`\0\x80\xFD[`@\x80Q`\xE0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x91\x90\x91R\x90V[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91R\x90V\xFE\xA2dipfsX\"\x12 \xEA\xA0\xBDv\xAAV\x9Er\xA2\x92\x91\xA7J\x80HU\x02\xC0\xA8W\xBB\xFF\t\x90\x94\xD0\x1AdS\x9A\x07\xE8dsolcC\0\x07\x06\x003";
    /// The bytecode of the contract.
    pub static UNISWAPV3POOL_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\x8EW`\x005`\xE0\x1C\x80cp\xCFuJ\x11a\0\xDEW\x80c\xC4Z\x01U\x11a\0\x97W\x80c\xDD\xCA?C\x11a\0qW\x80c\xDD\xCA?C\x14a\x07\xD8W\x80c\xF3\x05\x83\x99\x14a\x07\xF8W\x80c\xF3\r\xBA\x93\x14a\x08\0W\x80c\xF67s\x1D\x14a\x08\x82Wa\x01\x8EV[\x80c\xC4Z\x01U\x14a\x07\xA9W\x80c\xD0\xC9:|\x14a\x07\xB1W\x80c\xD2\x12 \xA7\x14a\x07\xD0Wa\x01\x8EV[\x80cp\xCFuJ\x14a\x05\xA0W\x80c\x82\x06\xA4\xD1\x14a\x05\xA8W\x80c\x85\xB6g)\x14a\x05\xD0W\x80c\x88;\xDB\xFD\x14a\x06\rW\x80c\xA3A#\xA7\x14a\x07\x14W\x80c\xA3\x88\x07\xF2\x14a\x07NWa\x01\x8EV[\x80c8P\xC7\xBD\x11a\x01KW\x80cI\x0El\xBC\x11a\x01%W\x80cI\x0El\xBC\x14a\x04LW\x80cO\x1E\xB3\xD8\x14a\x04\xD6W\x80cQN\xA4\xBF\x14a\x05'W\x80cS9\xC2\x96\x14a\x05\x80Wa\x01\x8EV[\x80c8P\xC7\xBD\x14a\x039W\x80c<\x8A}\x8D\x14a\x03\x92W\x80cF\x14\x13\x19\x14a\x042Wa\x01\x8EV[\x80c\r\xFE\x16\x81\x14a\x01\x93W\x80c\x12\x8A\xCB\x08\x14a\x01\xB7W\x80c\x1Ahe\x02\x14a\x02dW\x80c\x1A\xD8\xB0;\x14a\x02\x88W\x80c%,\t\xD7\x14a\x02\xBFW\x80c2\x14\x8Fg\x14a\x03\x16W[`\0\x80\xFD[a\x01\x9Ba\x08\xA8V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[a\x02K`\x04\x806\x03`\xA0\x81\x10\x15a\x01\xCDW`\0\x80\xFD[`\x01`\x01`\xA0\x1B\x03\x825\x81\x16\x92` \x81\x015\x15\x15\x92`@\x82\x015\x92``\x83\x015\x16\x91\x90\x81\x01\x90`\xA0\x81\x01`\x80\x82\x015`\x01` \x1B\x81\x11\x15a\x02\rW`\0\x80\xFD[\x82\x01\x83` \x82\x01\x11\x15a\x02\x1FW`\0\x80\xFD[\x805\x90` \x01\x91\x84`\x01\x83\x02\x84\x01\x11`\x01` \x1B\x83\x11\x17\x15a\x02@W`\0\x80\xFD[P\x90\x92P\x90Pa\x08\xCCV[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x80Q\x91\x82\x90\x03\x01\x90\xF3[a\x02la\x14kV[`@\x80Q`\x01`\x01`\x80\x1B\x03\x90\x92\x16\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[a\x02\x90a\x14zV[`@Q\x80\x83`\x01`\x01`\x80\x1B\x03\x16\x81R` \x01\x82`\x01`\x01`\x80\x1B\x03\x16\x81R` \x01\x92PPP`@Q\x80\x91\x03\x90\xF3[a\x02\xDC`\x04\x806\x03` \x81\x10\x15a\x02\xD5W`\0\x80\xFD[P5a\x14\x94V[`@\x80Qc\xFF\xFF\xFF\xFF\x90\x95\x16\x85R`\x06\x93\x90\x93\x0B` \x85\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x83\x83\x01R\x15\x15``\x83\x01RQ\x90\x81\x90\x03`\x80\x01\x90\xF3[a\x037`\x04\x806\x03` \x81\x10\x15a\x03,W`\0\x80\xFD[P5a\xFF\xFF\x16a\x14\xD9V[\0[a\x03Aa\x15\xD3V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x98\x16\x88R`\x02\x96\x90\x96\x0B` \x88\x01Ra\xFF\xFF\x94\x85\x16\x87\x87\x01R\x92\x84\x16``\x87\x01R\x92\x16`\x80\x85\x01R`\xFF\x90\x91\x16`\xA0\x84\x01R\x15\x15`\xC0\x83\x01RQ\x90\x81\x90\x03`\xE0\x01\x90\xF3[a\x02K`\x04\x806\x03`\xA0\x81\x10\x15a\x03\xA8W`\0\x80\xFD[`\x01`\x01`\xA0\x1B\x03\x825\x16\x91` \x81\x015`\x02\x90\x81\x0B\x92`@\x83\x015\x90\x91\x0B\x91`\x01`\x01`\x80\x1B\x03``\x82\x015\x16\x91\x81\x01\x90`\xA0\x81\x01`\x80\x82\x015`\x01` \x1B\x81\x11\x15a\x03\xF4W`\0\x80\xFD[\x82\x01\x83` \x82\x01\x11\x15a\x04\x06W`\0\x80\xFD[\x805\x90` \x01\x91\x84`\x01\x83\x02\x84\x01\x11`\x01` \x1B\x83\x11\x17\x15a\x04'W`\0\x80\xFD[P\x90\x92P\x90Pa\x16#V[a\x04:a\x18\xDFV[`@\x80Q\x91\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[a\x037`\x04\x806\x03`\x80\x81\x10\x15a\x04bW`\0\x80\xFD[`\x01`\x01`\xA0\x1B\x03\x825\x16\x91` \x81\x015\x91`@\x82\x015\x91\x90\x81\x01\x90`\x80\x81\x01``\x82\x015`\x01` \x1B\x81\x11\x15a\x04\x98W`\0\x80\xFD[\x82\x01\x83` \x82\x01\x11\x15a\x04\xAAW`\0\x80\xFD[\x805\x90` \x01\x91\x84`\x01\x83\x02\x84\x01\x11`\x01` \x1B\x83\x11\x17\x15a\x04\xCBW`\0\x80\xFD[P\x90\x92P\x90Pa\x18\xE5V[a\x02\x90`\x04\x806\x03`\xA0\x81\x10\x15a\x04\xECW`\0\x80\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x16\x90` \x81\x015`\x02\x90\x81\x0B\x91`@\x81\x015\x90\x91\x0B\x90`\x01`\x01`\x80\x1B\x03``\x82\x015\x81\x16\x91`\x80\x015\x16a\x1D@V[a\x05D`\x04\x806\x03` \x81\x10\x15a\x05=W`\0\x80\xFD[P5a\x1FZV[`@\x80Q`\x01`\x01`\x80\x1B\x03\x96\x87\x16\x81R` \x81\x01\x95\x90\x95R\x84\x81\x01\x93\x90\x93R\x90\x84\x16``\x84\x01R\x90\x92\x16`\x80\x82\x01R\x90Q\x90\x81\x90\x03`\xA0\x01\x90\xF3[a\x04:`\x04\x806\x03` \x81\x10\x15a\x05\x96W`\0\x80\xFD[P5`\x01\x0Ba\x1F\x97V[a\x02la\x1F\xA9V[a\x037`\x04\x806\x03`@\x81\x10\x15a\x05\xBEW`\0\x80\xFD[P`\xFF\x815\x81\x16\x91` \x015\x16a\x1F\xCDV[a\x02\x90`\x04\x806\x03``\x81\x10\x15a\x05\xE6W`\0\x80\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x16\x90`\x01`\x01`\x80\x1B\x03` \x82\x015\x81\x16\x91`@\x015\x16a!\xB1V[a\x06{`\x04\x806\x03` \x81\x10\x15a\x06#W`\0\x80\xFD[\x81\x01\x90` \x81\x01\x815`\x01` \x1B\x81\x11\x15a\x06=W`\0\x80\xFD[\x82\x01\x83` \x82\x01\x11\x15a\x06OW`\0\x80\xFD[\x805\x90` \x01\x91\x84` \x83\x02\x84\x01\x11`\x01` \x1B\x83\x11\x17\x15a\x06pW`\0\x80\xFD[P\x90\x92P\x90Pa$~V[`@Q\x80\x80` \x01\x80` \x01\x83\x81\x03\x83R\x85\x81\x81Q\x81R` \x01\x91P\x80Q\x90` \x01\x90` \x02\x80\x83\x83`\0[\x83\x81\x10\x15a\x06\xBFW\x81\x81\x01Q\x83\x82\x01R` \x01a\x06\xA7V[PPPP\x90P\x01\x83\x81\x03\x82R\x84\x81\x81Q\x81R` \x01\x91P\x80Q\x90` \x01\x90` \x02\x80\x83\x83`\0[\x83\x81\x10\x15a\x06\xFEW\x81\x81\x01Q\x83\x82\x01R` \x01a\x06\xE6V[PPPP\x90P\x01\x94PPPPP`@Q\x80\x91\x03\x90\xF3[a\x02K`\x04\x806\x03``\x81\x10\x15a\x07*W`\0\x80\xFD[P\x805`\x02\x90\x81\x0B\x91` \x81\x015\x90\x91\x0B\x90`@\x015`\x01`\x01`\x80\x1B\x03\x16a%\x0BV[a\x07x`\x04\x806\x03`@\x81\x10\x15a\x07dW`\0\x80\xFD[P\x805`\x02\x90\x81\x0B\x91` \x015\x90\x0Ba&\x82V[`@\x80Q`\x06\x94\x90\x94\x0B\x84R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x84\x01Rc\xFF\xFF\xFF\xFF\x16\x82\x82\x01RQ\x90\x81\x90\x03``\x01\x90\xF3[a\x01\x9Ba(qV[a\x07\xB9a(\x95V[`@\x80Q`\x02\x92\x90\x92\x0B\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[a\x01\x9Ba(\xB9V[a\x07\xE0a(\xDDV[`@\x80Qb\xFF\xFF\xFF\x90\x92\x16\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[a\x04:a)\x01V[a\x08 `\x04\x806\x03` \x81\x10\x15a\x08\x16W`\0\x80\xFD[P5`\x02\x0Ba)\x07V[`@\x80Q`\x01`\x01`\x80\x1B\x03\x90\x99\x16\x89R`\x0F\x97\x90\x97\x0B` \x89\x01R\x87\x87\x01\x95\x90\x95R``\x87\x01\x93\x90\x93R`\x06\x91\x90\x91\x0B`\x80\x86\x01R`\x01`\x01`\xA0\x1B\x03\x16`\xA0\x85\x01Rc\xFF\xFF\xFF\xFF\x16`\xC0\x84\x01R\x15\x15`\xE0\x83\x01RQ\x90\x81\x90\x03a\x01\0\x01\x90\xF3[a\x037`\x04\x806\x03` \x81\x10\x15a\x08\x98W`\0\x80\xFD[P5`\x01`\x01`\xA0\x1B\x03\x16a)qV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0\x80a\x08\xD7a+GV[\x85a\t\x0EW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x02`$\x82\x01RaAS`\xF0\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`@\x80Q`\xE0\x81\x01\x82R`\0T`\x01`\x01`\xA0\x1B\x03\x81\x16\x82R`\x01`\xA0\x1B\x81\x04`\x02\x90\x81\x0B\x81\x0B\x90\x0B` \x83\x01Ra\xFF\xFF`\x01`\xB8\x1B\x82\x04\x81\x16\x93\x83\x01\x93\x90\x93R`\x01`\xC8\x1B\x81\x04\x83\x16``\x83\x01R`\x01`\xD8\x1B\x81\x04\x90\x92\x16`\x80\x82\x01R`\xFF`\x01`\xE8\x1B\x83\x04\x81\x16`\xA0\x83\x01R`\x01`\xF0\x1B\x90\x92\x04\x90\x91\x16\x15\x15`\xC0\x82\x01\x81\x90Ra\t\xC7W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x03`$\x82\x01RbLOK`\xE8\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x87a\n\x12W\x80`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x86`\x01`\x01`\xA0\x1B\x03\x16\x11\x80\x15a\n\rWPs\xFF\xFD\x89c\xEF\xD1\xFCjPd\x88I]\x95\x1DRc\x98\x8D&`\x01`\x01`\xA0\x1B\x03\x87\x16\x10[a\nDV[\x80`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x86`\x01`\x01`\xA0\x1B\x03\x16\x10\x80\x15a\nDWPd\x01\0\x02v\xA3`\x01`\x01`\xA0\x1B\x03\x87\x16\x11[a\n{W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x03`$\x82\x01Rb\x14\xD4\x13`\xEA\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\0\x80T`\xFF`\xF0\x1B\x19\x16\x81U`@\x80Q`\xC0\x81\x01\x90\x91R\x80\x8Aa\n\xAAW`\x04\x84`\xA0\x01Q`\xFF\x16\x90\x1Ca\n\xBDV[`\x10\x84`\xA0\x01Q`\xFF\x16\x81a\n\xBBW\xFE[\x06[`\xFF\x16\x81R`\x04T`\x01`\x01`\x80\x1B\x03\x16` \x82\x01R`@\x01a\n\xDEa+~V[c\xFF\xFF\xFF\xFF\x16\x81R` \x01`\0`\x06\x0B\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\0\x15\x15\x81RP\x90P`\0\x80\x89\x13\x90P`\0`@Q\x80`\xE0\x01`@R\x80\x8B\x81R` \x01`\0\x81R` \x01\x85`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x85` \x01Q`\x02\x0B\x81R` \x01\x8Ca\x0BZW`\x02Ta\x0B^V[`\x01T[\x81R` \x01`\0`\x01`\x01`\x80\x1B\x03\x16\x81R` \x01\x84` \x01Q`\x01`\x01`\x80\x1B\x03\x16\x81RP\x90P[\x80Q\x15\x80\x15\x90a\x0B\xADWP\x88`\x01`\x01`\xA0\x1B\x03\x16\x81`@\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x15a\x0FwWa\x0B\xBAaT\xB5V[`@\x82\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81R``\x82\x01Qa\x0B\xFD\x90`\x06\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x8Fa+\x82V[\x15\x15`@\x83\x01R`\x02\x90\x81\x0B\x81\x0B` \x83\x01\x81\x90Rb\r\x89\xE7\x19\x91\x0B\x12\x15a\x0C.Wb\r\x89\xE7\x19` \x82\x01Ra\x0CMV[` \x81\x01Qb\r\x89\xE8`\x02\x91\x90\x91\x0B\x13\x15a\x0CMWb\r\x89\xE8` \x82\x01R[a\x0CZ\x81` \x01Qa,\xC4V[`\x01`\x01`\xA0\x1B\x03\x16``\x82\x01R`@\x82\x01Qa\x0C\xEB\x90\x8Da\x0C\x94W\x8B`\x01`\x01`\xA0\x1B\x03\x16\x83``\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x11a\x0C\xAEV[\x8B`\x01`\x01`\xA0\x1B\x03\x16\x83``\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x10[a\x0C\xBCW\x82``\x01Qa\x0C\xBEV[\x8B[`\xC0\x85\x01Q\x85Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a/\xF5V[`\xC0\x85\x01R`\xA0\x84\x01R`\x80\x83\x01R`\x01`\x01`\xA0\x1B\x03\x16`@\x83\x01R\x82\x15a\rMWa\r!\x81`\xC0\x01Q\x82`\x80\x01Q\x01a1\xE7V[\x82Q\x03\x82R`\xA0\x81\x01Qa\rC\x90a\r8\x90a1\xE7V[` \x84\x01Q\x90a1\xFDV[` \x83\x01Ra\r\x88V[a\rZ\x81`\xA0\x01Qa1\xE7V[\x82Q\x01\x82R`\xC0\x81\x01Q`\x80\x82\x01Qa\r\x82\x91a\rw\x91\x01a1\xE7V[` \x84\x01Q\x90a2\x19V[` \x83\x01R[\x83Q`\xFF\x16\x15a\r\xCEW`\0\x84`\0\x01Q`\xFF\x16\x82`\xC0\x01Q\x81a\r\xA8W\xFE[`\xC0\x84\x01\x80Q\x92\x90\x91\x04\x91\x82\x90\x03\x90R`\xA0\x84\x01\x80Q\x90\x91\x01`\x01`\x01`\x80\x1B\x03\x16\x90RP[`\xC0\x82\x01Q`\x01`\x01`\x80\x1B\x03\x16\x15a\x0E\rWa\x0E\x01\x81`\xC0\x01Q`\x01`\x80\x1B\x84`\xC0\x01Q`\x01`\x01`\x80\x1B\x03\x16a2/V[`\x80\x83\x01\x80Q\x90\x91\x01\x90R[\x80``\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x82`@\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a\x0F6W\x80`@\x01Q\x15a\x0F\rW\x83`\xA0\x01Qa\x0E\x97Wa\x0Eu\x84`@\x01Q`\0\x87` \x01Q\x88`@\x01Q\x88` \x01Q\x8A``\x01Q`\x08a2\xDF\x90\x96\x95\x94\x93\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\x01`\x01`\xA0\x1B\x03\x16`\x80\x86\x01R`\x06\x90\x81\x0B\x90\x0B``\x85\x01R`\x01`\xA0\x85\x01R[`\0a\x0E\xE3\x82` \x01Q\x8Ea\x0E\xAEW`\x01Ta\x0E\xB4V[\x84`\x80\x01Q[\x8Fa\x0E\xC3W\x85`\x80\x01Qa\x0E\xC7V[`\x02T[`\x80\x89\x01Q``\x8A\x01Q`@\x8B\x01Q`\x05\x95\x94\x93\x92\x91\x90a4qV[\x90P\x8C\x15a\x0E\xEFW`\0\x03[a\x0E\xFD\x83`\xC0\x01Q\x82a5+V[`\x01`\x01`\x80\x1B\x03\x16`\xC0\x84\x01RP[\x8Ba\x0F\x1CW\x80` \x01Qa\x0F%V[`\x01\x81` \x01Q\x03[`\x02\x90\x81\x0B\x90\x0B``\x83\x01Ra\x0FqV[\x80`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x82`@\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0FqWa\x0Fd\x82`@\x01Qa5\xE1V[`\x02\x90\x81\x0B\x90\x0B``\x83\x01R[Pa\x0B\x87V[\x83` \x01Q`\x02\x0B\x81``\x01Q`\x02\x0B\x14a\x10EW`\0\x80a\x0F\xC5\x86`@\x01Q\x86`@\x01Q\x88` \x01Q\x88` \x01Q\x8A``\x01Q\x8B`\x80\x01Q`\x08a8\xFC\x90\x96\x95\x94\x93\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[`@\x85\x01Q``\x86\x01Q`\0\x80Ta\xFF\xFF`\xC8\x1B\x19\x16`\x01`\xC8\x1Ba\xFF\xFF\x95\x86\x16\x02\x17a\xFF\xFF`\xB8\x1B\x19\x16`\x01`\xB8\x1B\x95\x90\x94\x16\x94\x90\x94\x02\x92\x90\x92\x17b\xFF\xFF\xFF`\xA0\x1B\x19\x16`\x01`\xA0\x1Bb\xFF\xFF\xFF`\x02\x94\x90\x94\x0B\x93\x90\x93\x16\x92\x90\x92\x02\x91\x90\x91\x17`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x17\x90UPa\x10j\x90PV[`@\x81\x01Q`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90U[\x80`\xC0\x01Q`\x01`\x01`\x80\x1B\x03\x16\x83` \x01Q`\x01`\x01`\x80\x1B\x03\x16\x14a\x10\xB0W`\xC0\x81\x01Q`\x04\x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90U[\x8A\x15a\x11\0W`\x80\x81\x01Q`\x01U`\xA0\x81\x01Q`\x01`\x01`\x80\x1B\x03\x16\x15a\x10\xFBW`\xA0\x81\x01Q`\x03\x80T`\x01`\x01`\x80\x1B\x03\x19\x81\x16`\x01`\x01`\x80\x1B\x03\x91\x82\x16\x90\x93\x01\x16\x91\x90\x91\x17\x90U[a\x11FV[`\x80\x81\x01Q`\x02U`\xA0\x81\x01Q`\x01`\x01`\x80\x1B\x03\x16\x15a\x11FW`\xA0\x81\x01Q`\x03\x80T`\x01`\x01`\x80\x1B\x03\x80\x82\x16`\x01`\x80\x1B\x92\x83\x90\x04\x82\x16\x90\x94\x01\x16\x02\x91\x90\x91\x17\x90U[\x81\x15\x15\x8B\x15\x15\x14a\x11_W` \x81\x01Q\x81Q\x8B\x03a\x11lV[\x80`\0\x01Q\x8A\x03\x81` \x01Q[\x90\x96P\x94P\x8A\x15a\x12\xA5W`\0\x85\x12\x15a\x11\xAEWa\x11\xAE\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x8D\x87`\0\x03a:\x97V[`\0a\x11\xB8a;\xE5V[\x90P3`\x01`\x01`\xA0\x1B\x03\x16c\xFAF\x1E3\x88\x88\x8C\x8C`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x85\x81R` \x01\x84\x81R` \x01\x80` \x01\x82\x81\x03\x82R\x84\x84\x82\x81\x81R` \x01\x92P\x80\x82\x847`\0\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPP\x95PPPPPP`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x12<W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x12PW=`\0\x80>=`\0\xFD[PPPPa\x12\\a;\xE5V[a\x12f\x82\x89a=\x1EV[\x11\x15a\x12\x9FW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x03`$\x82\x01RbIIA`\xE8\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[Pa\x13\xCFV[`\0\x86\x12\x15a\x12\xDCWa\x12\xDC\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x8D\x88`\0\x03a:\x97V[`\0a\x12\xE6a=.V[\x90P3`\x01`\x01`\xA0\x1B\x03\x16c\xFAF\x1E3\x88\x88\x8C\x8C`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x85\x81R` \x01\x84\x81R` \x01\x80` \x01\x82\x81\x03\x82R\x84\x84\x82\x81\x81R` \x01\x92P\x80\x82\x847`\0\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPP\x95PPPPPP`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x13jW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x13~W=`\0\x80>=`\0\xFD[PPPPa\x13\x8Aa=.V[a\x13\x94\x82\x88a=\x1EV[\x11\x15a\x13\xCDW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x03`$\x82\x01RbIIA`\xE8\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[P[`@\x80\x82\x01Q`\xC0\x83\x01Q``\x80\x85\x01Q\x84Q\x8B\x81R` \x81\x01\x8B\x90R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x81\x87\x01R`\x01`\x01`\x80\x1B\x03\x90\x93\x16\x91\x83\x01\x91\x90\x91R`\x02\x0B`\x80\x82\x01R\x91Q\x90\x8E\x16\x913\x91\x7F\xC4 y\xF9JcP\xD7\xE6#_)\x17I$\xF9(\xCC*\xC8\x18\xEBd\xFE\xD8\0N\x11_\xBC\xCAg\x91\x81\x90\x03`\xA0\x01\x90\xA3PP`\0\x80T`\xFF`\xF0\x1B\x19\x16`\x01`\xF0\x1B\x17\x90UP\x91\x98\x90\x97P\x95PPPPPPV[`\x04T`\x01`\x01`\x80\x1B\x03\x16\x81V[`\x03T`\x01`\x01`\x80\x1B\x03\x80\x82\x16\x91`\x01`\x80\x1B\x90\x04\x16\x82V[`\x08\x81a\xFF\xFF\x81\x10a\x14\xA5W`\0\x80\xFD[\x01Tc\xFF\xFF\xFF\xFF\x81\x16\x91P`\x01` \x1B\x81\x04`\x06\x0B\x90`\x01`X\x1B\x81\x04`\x01`\x01`\xA0\x1B\x03\x16\x90`\x01`\xF8\x1B\x90\x04`\xFF\x16\x84V[`\0T`\x01`\xF0\x1B\x90\x04`\xFF\x16a\x15\x1DW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x03`$\x82\x01RbLOK`\xE8\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\0\x80T`\xFF`\xF0\x1B\x19\x16\x90Ua\x152a+GV[`\0\x80T`\x01`\xD8\x1B\x90\x04a\xFF\xFF\x16\x90a\x15N`\x08\x83\x85a=\xC6V[`\0\x80Ta\xFF\xFF\x80\x84\x16`\x01`\xD8\x1B\x81\x02a\xFF\xFF`\xD8\x1B\x19\x90\x93\x16\x92\x90\x92\x17\x90\x92U\x91\x92P\x83\x16\x14a\x15\xBBW`@\x80Qa\xFF\xFF\x80\x85\x16\x82R\x83\x16` \x82\x01R\x81Q\x7F\xACI\xE5\x18\xF9\n5\x8Fe.D\0\x16O\x05\xA5\xD8\xF7\xE3^wG'\x9B\xC3\xA9=\xBFXN\x12Z\x92\x91\x81\x90\x03\x90\x91\x01\x90\xA1[PP`\0\x80T`\xFF`\xF0\x1B\x19\x16`\x01`\xF0\x1B\x17\x90UPV[`\0T`\x01`\x01`\xA0\x1B\x03\x81\x16\x90`\x01`\xA0\x1B\x81\x04`\x02\x0B\x90a\xFF\xFF`\x01`\xB8\x1B\x82\x04\x81\x16\x91`\x01`\xC8\x1B\x81\x04\x82\x16\x91`\x01`\xD8\x1B\x82\x04\x16\x90`\xFF`\x01`\xE8\x1B\x82\x04\x81\x16\x91`\x01`\xF0\x1B\x90\x04\x16\x87V[`\0\x80T\x81\x90`\x01`\xF0\x1B\x90\x04`\xFF\x16a\x16jW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x03`$\x82\x01RbLOK`\xE8\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\0\x80T`\xFF`\xF0\x1B\x19\x16\x90U`\x01`\x01`\x80\x1B\x03\x85\x16a\x16\x8AW`\0\x80\xFD[`\0\x80a\x16\xD8`@Q\x80`\x80\x01`@R\x80\x8C`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8B`\x02\x0B\x81R` \x01\x8A`\x02\x0B\x81R` \x01a\x16\xCE\x8A`\x01`\x01`\x80\x1B\x03\x16a>iV[`\x0F\x0B\x90Ra>zV[\x92P\x92PP\x81\x93P\x80\x92P`\0\x80`\0\x86\x11\x15a\x16\xFAWa\x16\xF7a;\xE5V[\x91P[\x84\x15a\x17\x0BWa\x17\x08a=.V[\x90P[3`\x01`\x01`\xA0\x1B\x03\x16c\xD3Hy\x97\x87\x87\x8B\x8B`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x85\x81R` \x01\x84\x81R` \x01\x80` \x01\x82\x81\x03\x82R\x84\x84\x82\x81\x81R` \x01\x92P\x80\x82\x847`\0\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPP\x95PPPPPP`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x17\x8DW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x17\xA1W=`\0\x80>=`\0\xFD[PPPP`\0\x86\x11\x15a\x17\xF8Wa\x17\xB6a;\xE5V[a\x17\xC0\x83\x88a=\x1EV[\x11\x15a\x17\xF8W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x02`$\x82\x01Ra\x04\xD3`\xF4\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x84\x15a\x18HWa\x18\x06a=.V[a\x18\x10\x82\x87a=\x1EV[\x11\x15a\x18HW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x02`$\x82\x01RaM1`\xF0\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x89`\x02\x0B\x8B`\x02\x0B\x8D`\x01`\x01`\xA0\x1B\x03\x16\x7FzS\x08\x0B\xA4\x14\x15\x8B\xE7\xECi\xB9\x87\xB5\xFB}\x07\xDE\xE1\x01\xFE\x85H\x8F\x08S\xAE\x16#\x9D\x0B\xDE3\x8D\x8B\x8B`@Q\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x84`\x01`\x01`\x80\x1B\x03\x16\x81R` \x01\x83\x81R` \x01\x82\x81R` \x01\x94PPPPP`@Q\x80\x91\x03\x90\xA4PP`\0\x80T`\xFF`\xF0\x1B\x19\x16`\x01`\xF0\x1B\x17\x90UP\x91\x98\x90\x97P\x95PPPPPPV[`\x02T\x81V[`\0T`\x01`\xF0\x1B\x90\x04`\xFF\x16a\x19)W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x03`$\x82\x01RbLOK`\xE8\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\0\x80T`\xFF`\xF0\x1B\x19\x16\x90Ua\x19>a+GV[`\x04T`\x01`\x01`\x80\x1B\x03\x16\x80a\x19\x80W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x01`$\x82\x01R`\x13`\xFA\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\0a\x19\xB5\x86\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0b\xFF\xFF\xFF\x16b\x0FB@a@\xBAV[\x90P`\0a\x19\xEC\x86\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0b\xFF\xFF\xFF\x16b\x0FB@a@\xBAV[\x90P`\0a\x19\xF8a;\xE5V[\x90P`\0a\x1A\x04a=.V[\x90P\x88\x15a\x1A7Wa\x1A7\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x8B\x8Ba:\x97V[\x87\x15a\x1AhWa\x1Ah\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x8B\x8Aa:\x97V[3`\x01`\x01`\xA0\x1B\x03\x16c\xE9\xCB\xAF\xB0\x85\x85\x8A\x8A`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x85\x81R` \x01\x84\x81R` \x01\x80` \x01\x82\x81\x03\x82R\x84\x84\x82\x81\x81R` \x01\x92P\x80\x82\x847`\0\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPP\x95PPPPPP`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1A\xEAW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1A\xFEW=`\0\x80>=`\0\xFD[PPPP`\0a\x1B\x0Ca;\xE5V[\x90P`\0a\x1B\x18a=.V[\x90P\x81a\x1B%\x85\x88a=\x1EV[\x11\x15a\x1B]W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x02`$\x82\x01Ra\x04c`\xF4\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x80a\x1Bh\x84\x87a=\x1EV[\x11\x15a\x1B\xA0W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x02`$\x82\x01RaF1`\xF0\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x83\x82\x03\x83\x82\x03\x81\x15a\x1C/W`\0\x80T`\x01`\xE8\x1B\x90\x04`\x0F\x16\x90\x81\x15a\x1B\xD3W\x81`\xFF\x16\x84\x81a\x1B\xCDW\xFE[\x04a\x1B\xD6V[`\0[\x90P`\x01`\x01`\x80\x1B\x03\x81\x16\x15a\x1C\tW`\x03\x80T`\x01`\x01`\x80\x1B\x03\x80\x82\x16\x84\x01\x16`\x01`\x01`\x80\x1B\x03\x19\x90\x91\x16\x17\x90U[a\x1C#\x81\x85\x03`\x01`\x80\x1B\x8D`\x01`\x01`\x80\x1B\x03\x16a2/V[`\x01\x80T\x90\x91\x01\x90UPP[\x80\x15a\x1C\xBAW`\0\x80T`\x01`\xE8\x1B\x90\x04`\x04\x1C`\x0F\x16\x90\x81\x15a\x1C_W\x81`\xFF\x16\x83\x81a\x1CYW\xFE[\x04a\x1CbV[`\0[\x90P`\x01`\x01`\x80\x1B\x03\x81\x16\x15a\x1C\x94W`\x03\x80T`\x01`\x01`\x80\x1B\x03`\x01`\x80\x1B\x80\x83\x04\x82\x16\x85\x01\x82\x16\x02\x91\x16\x17\x90U[a\x1C\xAE\x81\x84\x03`\x01`\x80\x1B\x8D`\x01`\x01`\x80\x1B\x03\x16a2/V[`\x02\x80T\x90\x91\x01\x90UPP[\x8D`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x7F\xBD\xBD\xB7\x1Dx`7k\xA5+%\xA5\x02\x8B\xEE\xA25\x816J@R/k\xCF\xB8k\xB1\xF2\xDC\xA63\x8F\x8F\x86\x86`@Q\x80\x85\x81R` \x01\x84\x81R` \x01\x83\x81R` \x01\x82\x81R` \x01\x94PPPPP`@Q\x80\x91\x03\x90\xA3PP`\0\x80T`\xFF`\xF0\x1B\x19\x16`\x01`\xF0\x1B\x17\x90UPPPPPPPPPPPPV[`\0\x80T\x81\x90`\x01`\xF0\x1B\x90\x04`\xFF\x16a\x1D\x87W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x03`$\x82\x01RbLOK`\xE8\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\0\x80T`\xFF`\xF0\x1B\x19\x16\x81Ua\x1D\xA1`\x073\x89\x89a@\xF4V[`\x03\x81\x01T\x90\x91P`\x01`\x01`\x80\x1B\x03\x90\x81\x16\x90\x86\x16\x11a\x1D\xC2W\x84a\x1D\xD1V[`\x03\x81\x01T`\x01`\x01`\x80\x1B\x03\x16[`\x03\x82\x01T\x90\x93P`\x01`\x01`\x80\x1B\x03`\x01`\x80\x1B\x90\x91\x04\x81\x16\x90\x85\x16\x11a\x1D\xF9W\x83a\x1E\x0FV[`\x03\x81\x01T`\x01`\x80\x1B\x90\x04`\x01`\x01`\x80\x1B\x03\x16[\x91P`\x01`\x01`\x80\x1B\x03\x83\x16\x15a\x1EtW`\x03\x81\x01\x80T`\x01`\x01`\x80\x1B\x03\x19\x81\x16`\x01`\x01`\x80\x1B\x03\x91\x82\x16\x86\x90\x03\x82\x16\x17\x90\x91Ua\x1Et\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x8A\x90\x86\x16a:\x97V[`\x01`\x01`\x80\x1B\x03\x82\x16\x15a\x1E\xDAW`\x03\x81\x01\x80T`\x01`\x01`\x80\x1B\x03`\x01`\x80\x1B\x80\x83\x04\x82\x16\x86\x90\x03\x82\x16\x02\x91\x81\x16\x91\x90\x91\x17\x90\x91Ua\x1E\xDA\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x8A\x90\x85\x16a:\x97V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x8A\x16\x81R`\x01`\x01`\x80\x1B\x03\x80\x86\x16` \x83\x01R\x84\x16\x81\x83\x01R\x90Q`\x02\x88\x81\x0B\x92\x90\x8A\x90\x0B\x913\x91\x7Fp\x93S8\xE6\x97uEj\x85\xDD\xEF\"l9_\xB6h\xB6?\xA0\x11__ a\x0B8\x8El\xA9\xC0\x91\x90\x81\x90\x03``\x01\x90\xA4P`\0\x80T`\xFF`\xF0\x1B\x19\x16`\x01`\xF0\x1B\x17\x90U\x90\x96\x90\x95P\x93PPPPV[`\x07` R`\0\x90\x81R`@\x90 \x80T`\x01\x82\x01T`\x02\x83\x01T`\x03\x90\x93\x01T`\x01`\x01`\x80\x1B\x03\x92\x83\x16\x93\x91\x92\x81\x81\x16\x91`\x01`\x80\x1B\x90\x04\x16\x85V[`\x06` R`\0\x90\x81R`@\x90 T\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0T`\x01`\xF0\x1B\x90\x04`\xFF\x16a \x11W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x03`$\x82\x01RbLOK`\xE8\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\0\x80T`\xFF`\xF0\x1B\x19\x16\x90U`@\x80Qc\x8D\xA5\xCB[`\xE0\x1B\x81R\x90Q`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91c\x8D\xA5\xCB[\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a ~W`\0\x80\xFD[PZ\xFA\x15\x80\x15a \x92W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a \xA8W`\0\x80\xFD[PQ`\x01`\x01`\xA0\x1B\x03\x163\x14a \xBEW`\0\x80\xFD[`\xFF\x82\x16\x15\x80a \xE1WP`\x04\x82`\xFF\x16\x10\x15\x80\x15a \xE1WP`\n\x82`\xFF\x16\x11\x15[\x80\x15a!\x0BWP`\xFF\x81\x16\x15\x80a!\x0BWP`\x04\x81`\xFF\x16\x10\x15\x80\x15a!\x0BWP`\n\x81`\xFF\x16\x11\x15[a!\x14W`\0\x80\xFD[`\0\x80Ta\x0F\xF0`\x04\x84\x90\x1B\x16\x84\x01`\xFF\x90\x81\x16`\x01`\xE8\x1B\x90\x81\x02`\xFF`\xE8\x1B\x19\x84\x16\x17\x90\x93U\x91\x90\x04\x16\x7F\x97=\x8D\x92\xBB)\x9FJ\xF6\xCEI\xB5*\x8A\xDB\x85\xAEF\xB9\xF2\x14\xC4\xC4\xFC\x06\xACw@\x127\xB13`\x10\x82`@\x80Q`\xFF\x93\x90\x92\x06\x83\x16\x82R`\x0F`\x04\x86\x90\x1C\x16` \x83\x01R\x86\x83\x16\x82\x82\x01R\x91\x85\x16``\x82\x01R\x90Q\x90\x81\x90\x03`\x80\x01\x90\xA1PP`\0\x80T`\xFF`\xF0\x1B\x19\x16`\x01`\xF0\x1B\x17\x90UPV[`\0\x80T\x81\x90`\x01`\xF0\x1B\x90\x04`\xFF\x16a!\xF8W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x03`$\x82\x01RbLOK`\xE8\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\0\x80T`\xFF`\xF0\x1B\x19\x16\x90U`@\x80Qc\x8D\xA5\xCB[`\xE0\x1B\x81R\x90Q`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91c\x8D\xA5\xCB[\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\"eW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\"yW=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\"\x8FW`\0\x80\xFD[PQ`\x01`\x01`\xA0\x1B\x03\x163\x14a\"\xA5W`\0\x80\xFD[`\x03T`\x01`\x01`\x80\x1B\x03\x90\x81\x16\x90\x85\x16\x11a\"\xC1W\x83a\"\xCEV[`\x03T`\x01`\x01`\x80\x1B\x03\x16[`\x03T\x90\x92P`\x01`\x01`\x80\x1B\x03`\x01`\x80\x1B\x90\x91\x04\x81\x16\x90\x84\x16\x11a\"\xF4W\x82a#\x08V[`\x03T`\x01`\x80\x1B\x90\x04`\x01`\x01`\x80\x1B\x03\x16[\x90P`\x01`\x01`\x80\x1B\x03\x82\x16\x15a#\x89W`\x03T`\x01`\x01`\x80\x1B\x03\x83\x81\x16\x91\x16\x14\x15a#7W`\0\x19\x90\x91\x01\x90[`\x03\x80T`\x01`\x01`\x80\x1B\x03\x19\x81\x16`\x01`\x01`\x80\x1B\x03\x91\x82\x16\x85\x90\x03\x82\x16\x17\x90\x91Ua#\x89\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x87\x90\x85\x16a:\x97V[`\x01`\x01`\x80\x1B\x03\x81\x16\x15a$\x0FW`\x03T`\x01`\x01`\x80\x1B\x03\x82\x81\x16`\x01`\x80\x1B\x90\x92\x04\x16\x14\x15a#\xBAW`\0\x19\x01[`\x03\x80T`\x01`\x01`\x80\x1B\x03`\x01`\x80\x1B\x80\x83\x04\x82\x16\x85\x90\x03\x82\x16\x02\x91\x81\x16\x91\x90\x91\x17\x90\x91Ua$\x0F\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x87\x90\x84\x16a:\x97V[`@\x80Q`\x01`\x01`\x80\x1B\x03\x80\x85\x16\x82R\x83\x16` \x82\x01R\x81Q`\x01`\x01`\xA0\x1B\x03\x88\x16\x923\x92\x7FYkW9\x06!\x8D4\x11\x85\x0B&\xA6\xB47\xD6\xC4R/\xDBC\xD2\xD28bc\xF8mP\xB8\xB1Q\x92\x90\x81\x90\x03\x90\x91\x01\x90\xA3`\0\x80T`\xFF`\xF0\x1B\x19\x16`\x01`\xF0\x1B\x17\x90U\x90\x94\x90\x93P\x91PPV[``\x80a$\x89a+GV[a%\0a$\x94a+~V[\x85\x85\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x82\x90RPT`\x04T`\x08\x96\x95\x94P`\x01`\xA0\x1B\x82\x04`\x02\x0B\x93Pa\xFF\xFF`\x01`\xB8\x1B\x83\x04\x81\x16\x93P`\x01`\x01`\x80\x1B\x03\x90\x91\x16\x91`\x01`\xC8\x1B\x90\x04\x16aAXV[\x91P\x91P\x92P\x92\x90PV[`\0\x80T\x81\x90`\x01`\xF0\x1B\x90\x04`\xFF\x16a%RW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x03`$\x82\x01RbLOK`\xE8\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\0\x80T`\xFF`\xF0\x1B\x19\x16\x81U`@\x80Q`\x80\x81\x01\x82R3\x81R`\x02\x88\x81\x0B` \x83\x01R\x87\x90\x0B\x91\x81\x01\x91\x90\x91R\x81\x90\x81\x90a%\xAB\x90``\x81\x01a%\x9E`\x01`\x01`\x80\x1B\x03\x8A\x16a>iV[`\0\x03`\x0F\x0B\x90Ra>zV[\x92P\x92P\x92P\x81`\0\x03\x94P\x80`\0\x03\x93P`\0\x85\x11\x80a%\xCCWP`\0\x84\x11[\x15a&\x0BW`\x03\x83\x01\x80T`\x01`\x01`\x80\x1B\x03\x80\x82\x16\x80\x89\x01\x82\x16`\x01`\x80\x1B\x93\x84\x90\x04\x83\x16\x89\x01\x90\x92\x16\x90\x92\x02\x90\x91\x17`\x01`\x01`\x80\x1B\x03\x19\x16\x17\x90U[`@\x80Q`\x01`\x01`\x80\x1B\x03\x88\x16\x81R` \x81\x01\x87\x90R\x80\x82\x01\x86\x90R\x90Q`\x02\x89\x81\x0B\x92\x90\x8B\x90\x0B\x913\x91\x7F\x0C9l\xD9\x89\xA3\x9FDY\xB5\xFA\x1A\xEDj\x9A\x8D\xCD\xBCE\x90\x8A\xCF\xD6~\x02\x8C\xD5h\xDA\x98\x98,\x91\x90\x81\x90\x03``\x01\x90\xA4PP`\0\x80T`\xFF`\xF0\x1B\x19\x16`\x01`\xF0\x1B\x17\x90UP\x90\x94\x90\x93P\x91PPV[`\0\x80`\0a&\x8Fa+GV[a&\x99\x85\x85aB\xB2V[`\x02\x85\x81\x0B\x81\x0B`\0\x90\x81R`\x05` R`@\x80\x82 \x87\x84\x0B\x90\x93\x0B\x82R\x81 `\x03\x83\x01T`\x06\x81\x90\x0B\x93`\x01`8\x1B\x82\x04`\x01`\x01`\xA0\x1B\x03\x16\x92\x84\x92`\x01`\xD8\x1B\x81\x04c\xFF\xFF\xFF\xFF\x16\x92\x84\x92\x90\x91`\x01`\xF8\x1B\x90\x04`\xFF\x16\x80a&\xFDW`\0\x80\xFD[`\x03\x82\x01T`\x06\x81\x90\x0B\x98P`\x01`8\x1B\x81\x04`\x01`\x01`\xA0\x1B\x03\x16\x96P`\x01`\xD8\x1B\x81\x04c\xFF\xFF\xFF\xFF\x16\x94P`\x01`\xF8\x1B\x90\x04`\xFF\x16\x80a'>W`\0\x80\xFD[PP`@\x80Q`\xE0\x81\x01\x82R`\0T`\x01`\x01`\xA0\x1B\x03\x81\x16\x82R`\x01`\xA0\x1B\x81\x04`\x02\x90\x81\x0B\x81\x0B\x81\x0B` \x84\x01\x81\x90Ra\xFF\xFF`\x01`\xB8\x1B\x84\x04\x81\x16\x95\x85\x01\x95\x90\x95R`\x01`\xC8\x1B\x83\x04\x85\x16``\x85\x01R`\x01`\xD8\x1B\x83\x04\x90\x94\x16`\x80\x84\x01R`\xFF`\x01`\xE8\x1B\x83\x04\x81\x16`\xA0\x85\x01R`\x01`\xF0\x1B\x90\x92\x04\x90\x91\x16\x15\x15`\xC0\x83\x01R\x90\x93P\x8E\x81\x0B\x91\x90\x0B\x12\x15\x90Pa'\xE7WP\x93\x90\x94\x03\x96P\x90\x03\x93P\x90\x03\x90Pa(jV[\x8A`\x02\x0B\x81` \x01Q`\x02\x0B\x12\x15a([W`\0a(\x03a+~V[` \x83\x01Q`@\x84\x01Q`\x04T``\x86\x01Q\x93\x94P`\0\x93\x84\x93a(9\x93`\x08\x93\x88\x93\x87\x93\x92\x91`\x01`\x01`\x80\x1B\x03\x16\x90a2\xDFV[\x9A\x90\x03\x98\x90\x98\x03\x9BPP\x94\x90\x96\x03\x92\x90\x92\x03\x96P\x90\x91\x03\x03\x92Pa(j\x91PPV[P\x94\x90\x93\x03\x96P\x03\x93P\x90\x03\x90P[\x92P\x92P\x92V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\x01T\x81V[`\x05` R`\0\x90\x81R`@\x90 \x80T`\x01\x82\x01T`\x02\x83\x01T`\x03\x90\x93\x01T`\x01`\x01`\x80\x1B\x03\x83\x16\x93`\x01`\x80\x1B\x90\x93\x04`\x0F\x0B\x92\x90`\x06\x81\x90\x0B\x90`\x01`8\x1B\x81\x04`\x01`\x01`\xA0\x1B\x03\x16\x90`\x01`\xD8\x1B\x81\x04c\xFF\xFF\xFF\xFF\x16\x90`\x01`\xF8\x1B\x90\x04`\xFF\x16\x88V[`\0T`\x01`\x01`\xA0\x1B\x03\x16\x15a)\xB4W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x02`$\x82\x01RaAI`\xF0\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\0a)\xBF\x82a5\xE1V[\x90P`\0\x80a)\xD7a)\xCFa+~V[`\x08\x90aC{V[`@\x80Q`\xE0\x81\x01\x82R`\x01`\x01`\xA0\x1B\x03\x88\x16\x80\x82R`\x02\x88\x81\x0B` \x80\x85\x01\x82\x90R`\0\x85\x87\x01\x81\x90Ra\xFF\xFF\x89\x81\x16``\x88\x01\x81\x90R\x90\x89\x16`\x80\x88\x01\x81\x90R`\xA0\x88\x01\x83\x90R`\x01`\xC0\x90\x98\x01\x97\x90\x97R\x81T`\x01`\xF0\x1B`\x01`\x01`\xA0\x1B\x03\x19\x90\x91\x16\x87\x17b\xFF\xFF\xFF`\xA0\x1B\x19\x16`\x01`\xA0\x1Bb\xFF\xFF\xFF\x97\x87\x90\x0B\x97\x90\x97\x16\x96\x90\x96\x02\x95\x90\x95\x17c\xFF\xFF\xFF\xFF`\xB8\x1B\x19\x16`\x01`\xC8\x1B\x90\x91\x02\x17a\xFF\xFF`\xD8\x1B\x19\x16`\x01`\xD8\x1B\x90\x96\x02\x95\x90\x95\x17a\xFF\xFF`\xE8\x1B\x19\x16\x92\x90\x92\x17\x90\x93U\x83Q\x91\x82R\x81\x01\x91\x90\x91R\x81Q\x93\x95P\x91\x93P\x7F\x98c`6\xCBf\xA9\xC1\x9A7C^\xFC\x1E\x90\x14!\x90!N\x8A\xBE\xB8!\xBD\xBA?)\x90\xDDL\x95\x92\x91\x82\x90\x03\x01\x90\xA1PPPPV[`\0\x80\x82`\x02\x81\x90\x0Bb\r\x89\xE7\x19\x81a*\xF0W\xFE[\x05\x02\x90P`\0\x83`\x02\x81\x90\x0Bb\r\x89\xE8\x81a+\x07W\xFE[\x05\x02\x90P`\0\x84`\x02\x0B\x83\x83\x03`\x02\x0B\x81a+\x1EW\xFE[\x05`\x01\x01\x90P\x80b\xFF\xFF\xFF\x16`\x01`\x01`\x80\x1B\x03\x80\x16\x81a+;W\xFE[\x04\x93PPPP[\x91\x90PV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a+|W`\0\x80\xFD[V[B\x90V[`\0\x80`\0\x84`\x02\x0B\x86`\x02\x0B\x81a+\x96W\xFE[\x05\x90P`\0\x86`\x02\x0B\x12\x80\x15a+\xBDWP\x84`\x02\x0B\x86`\x02\x0B\x81a+\xB6W\xFE[\x07`\x02\x0B\x15\x15[\x15a+\xC7W`\0\x19\x01[\x83\x15a,<W`\0\x80a+\xD9\x83aC\xC7V[`\x01\x82\x81\x0B\x81\x0B`\0\x90\x81R` \x8D\x90R`@\x90 T`\xFF\x83\x16\x91\x90\x91\x1B\x80\x01`\0\x19\x01\x90\x81\x16\x80\x15\x15\x97P\x92\x94P\x90\x92P\x90\x85a,\x1EW\x88\x83`\xFF\x16\x86\x03\x02a,1V[\x88a,(\x82aC\xD9V[\x84\x03`\xFF\x16\x86\x03\x02[\x96PPPPPa,\xBAV[`\0\x80a,K\x83`\x01\x01aC\xC7V[\x91P\x91P`\0`\x01\x82`\xFF\x16`\x01\x90\x1B\x03\x19\x90P`\0\x81\x8B`\0\x86`\x01\x0B`\x01\x0B\x81R` \x01\x90\x81R` \x01`\0 T\x16\x90P\x80`\0\x14\x15\x95P\x85a,\x9DW\x88\x83`\xFF\x03`\xFF\x16\x86`\x01\x01\x01\x02a,\xB3V[\x88\x83a,\xA8\x83aDxV[\x03`\xFF\x16\x86`\x01\x01\x01\x02[\x96PPPPP[P\x94P\x94\x92PPPV[`\0\x80`\0\x83`\x02\x0B\x12a,\xDBW\x82`\x02\x0Ba,\xE3V[\x82`\x02\x0B`\0\x03[\x90Pb\r\x89\xE8\x81\x11\x15a-!W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x01`$\x82\x01R`\x15`\xFA\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\0`\x01\x82\x16a-5W`\x01`\x80\x1Ba-GV[o\xFF\xFC\xB93\xBDo\xAD7\xAA-\x16-\x1AY@\x01[p\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P`\x02\x82\x16\x15a-{Wo\xFF\xF9rr7=A2Y\xA4i\x90X\x0E!:\x02`\x80\x1C[`\x04\x82\x16\x15a-\x9AWo\xFF\xF2\xE5\x0F_ei2\xEF\x125|\xF3\xC7\xFD\xCC\x02`\x80\x1C[`\x08\x82\x16\x15a-\xB9Wo\xFF\xE5\xCA\xCA~\x10\xE4\xE6\x1C6$\xEA\xA0\x94\x1C\xD0\x02`\x80\x1C[`\x10\x82\x16\x15a-\xD8Wo\xFF\xCB\x98C\xD6\x0FaY\xC9\xDBX\x83\\\x92fD\x02`\x80\x1C[` \x82\x16\x15a-\xF7Wo\xFF\x97;A\xFA\x98\xC0\x81G.h\x96\xDF\xB2T\xC0\x02`\x80\x1C[`@\x82\x16\x15a.\x16Wo\xFF.\xA1df\xC9j8C\xECx\xB3&\xB5(a\x02`\x80\x1C[`\x80\x82\x16\x15a.5Wo\xFE]\xEE\x04j\x99\xA2\xA8\x11\xC4a\xF1\x96\x9C0S\x02`\x80\x1C[a\x01\0\x82\x16\x15a.UWo\xFC\xBE\x86\xC7\x90\n\x88\xAE\xDC\xFF\xC8;G\x9A\xA3\xA4\x02`\x80\x1C[a\x02\0\x82\x16\x15a.uWo\xF9\x87\xA7%:\xC4\x13\x17o+\x07L\xF7\x81^T\x02`\x80\x1C[a\x04\0\x82\x16\x15a.\x95Wo\xF39+\x08\"\xB7\0\x05\x94\x0Cz9\x8EKp\xF3\x02`\x80\x1C[a\x08\0\x82\x16\x15a.\xB5Wo\xE7\x15\x94u\xA2\xC2\x9BtC\xB2\x9C\x7F\xA6\xE8\x89\xD9\x02`\x80\x1C[a\x10\0\x82\x16\x15a.\xD5Wo\xD0\x97\xF3\xBD\xFD \"\xB8\x84Z\xD8\xF7\x92\xAAX%\x02`\x80\x1C[a \0\x82\x16\x15a.\xF5Wo\xA9\xF7FF-\x87\x0F\xDF\x8Ae\xDC\x1F\x90\xE0a\xE5\x02`\x80\x1C[a@\0\x82\x16\x15a/\x15Wop\xD8i\xA1V\xD2\xA1\xB8\x90\xBB=\xF6+\xAF2\xF7\x02`\x80\x1C[a\x80\0\x82\x16\x15a/5Wo1\xBE\x13_\x97\xD0\x8F\xD9\x81#\x15\x05T/\xCF\xA6\x02`\x80\x1C[b\x01\0\0\x82\x16\x15a/VWo\t\xAAP\x8B[z\x84\xE1\xC6w\xDET\xF3\xE9\x9B\xC9\x02`\x80\x1C[b\x02\0\0\x82\x16\x15a/vWn]j\xF8\xDE\xDB\x81\x19f\x99\xC3)\"^\xE6\x04\x02`\x80\x1C[b\x04\0\0\x82\x16\x15a/\x95Wm\"\x16\xE5\x84\xF5\xFA\x1E\xA9&\x04\x1B\xED\xFE\x98\x02`\x80\x1C[b\x08\0\0\x82\x16\x15a/\xB2Wk\x04\x8A\x17\x03\x91\xF7\xDCBDN\x8F\xA2\x02`\x80\x1C[`\0\x84`\x02\x0B\x13\x15a/\xCDW\x80`\0\x19\x81a/\xC9W\xFE[\x04\x90P[`\x01` \x1B\x81\x06\x15a/\xE0W`\x01a/\xE3V[`\0[`\xFF\x16` \x82\x90\x1C\x01\x92PPP\x91\x90PV[`\0\x80\x80\x80`\x01`\x01`\xA0\x1B\x03\x80\x89\x16\x90\x8A\x16\x10\x15\x81\x87\x12\x80\x15\x90a0zW`\0a0.\x89\x89b\x0FB@\x03b\xFF\xFF\xFF\x16b\x0FB@a2/V[\x90P\x82a0GWa0B\x8C\x8C\x8C`\x01aEbV[a0TV[a0T\x8B\x8D\x8C`\x01aE\xDDV[\x95P\x85\x81\x10a0eW\x8A\x96Pa0tV[a0q\x8C\x8B\x83\x86aF\x88V[\x96P[Pa0\xC4V[\x81a0\x91Wa0\x8C\x8B\x8B\x8B`\0aE\xDDV[a0\x9EV[a0\x9E\x8A\x8C\x8B`\0aEbV[\x93P\x83\x88`\0\x03\x10a0\xB2W\x89\x95Pa0\xC4V[a0\xC1\x8B\x8A\x8A`\0\x03\x85aF\xD4V[\x95P[`\x01`\x01`\xA0\x1B\x03\x8A\x81\x16\x90\x87\x16\x14\x82\x15a1'W\x80\x80\x15a0\xE3WP\x81[a0\xF9Wa0\xF4\x87\x8D\x8C`\x01aE\xDDV[a0\xFBV[\x85[\x95P\x80\x80\x15a1\x08WP\x81\x15[a1\x1EWa1\x19\x87\x8D\x8C`\0aEbV[a1 V[\x84[\x94Pa1qV[\x80\x80\x15a11WP\x81[a1GWa1B\x8C\x88\x8C`\x01aEbV[a1IV[\x85[\x95P\x80\x80\x15a1VWP\x81\x15[a1lWa1g\x8C\x88\x8C`\0aE\xDDV[a1nV[\x84[\x94P[\x81\x15\x80\x15a1\x81WP\x88`\0\x03\x85\x11[\x15a1\x8DW\x88`\0\x03\x94P[\x81\x80\x15a1\xACWP\x8A`\x01`\x01`\xA0\x1B\x03\x16\x87`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x15a1\xBBW\x85\x89\x03\x93Pa1\xD8V[a1\xD5\x86\x89b\xFF\xFF\xFF\x16\x8Ab\x0FB@\x03b\xFF\xFF\xFF\x16a@\xBAV[\x93P[PPP\x95P\x95P\x95P\x95\x91PPV[`\0`\x01`\xFF\x1B\x82\x10a1\xF9W`\0\x80\xFD[P\x90V[\x80\x82\x03\x82\x81\x13\x15`\0\x83\x12\x15\x14a2\x13W`\0\x80\xFD[\x92\x91PPV[\x81\x81\x01\x82\x81\x12\x15`\0\x83\x12\x15\x14a2\x13W`\0\x80\xFD[`\0\x80\x80`\0\x19\x85\x87\t\x86\x86\x02\x92P\x82\x81\x10\x90\x83\x90\x03\x03\x90P\x80a2eW`\0\x84\x11a2ZW`\0\x80\xFD[P\x82\x90\x04\x90Pa2\xD8V[\x80\x84\x11a2qW`\0\x80\xFD[`\0\x84\x86\x88\t`\0\x86\x81\x03\x87\x16\x96\x87\x90\x04\x96`\x02`\x03\x89\x02\x81\x18\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x90\x91\x03\x02\x91\x81\x90\x03\x81\x90\x04`\x01\x01\x86\x84\x11\x90\x95\x03\x94\x90\x94\x02\x91\x90\x94\x03\x92\x90\x92\x04\x91\x90\x91\x17\x91\x90\x91\x02\x91PP[\x93\x92PPPV[`\0\x80c\xFF\xFF\xFF\xFF\x87\x16a3\x85W`\0\x89\x86a\xFF\xFF\x16a\xFF\xFF\x81\x10a3\0W\xFE[`@\x80Q`\x80\x81\x01\x82R\x91\x90\x92\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x80\x84R`\x01` \x1B\x83\x04`\x06\x90\x81\x0B\x81\x0B\x90\x0B` \x85\x01R`\x01`X\x1B\x83\x04`\x01`\x01`\xA0\x1B\x03\x16\x94\x84\x01\x94\x90\x94R`\x01`\xF8\x1B\x90\x91\x04`\xFF\x16\x15\x15``\x83\x01R\x90\x92P\x8A\x16\x14a3qWa3n\x81\x8A\x89\x88aG V[\x90P[\x80` \x01Q\x81`@\x01Q\x92P\x92PPa4eV[\x86\x88\x03`\0\x80a3\x9A\x8C\x8C\x85\x8C\x8C\x8C\x8CaG\xC3V[\x91P\x91P\x81`\0\x01Qc\xFF\xFF\xFF\xFF\x16\x83c\xFF\xFF\xFF\xFF\x16\x14\x15a3\xCCW\x81` \x01Q\x82`@\x01Q\x94P\x94PPPPa4eV[\x80Qc\xFF\xFF\xFF\xFF\x84\x81\x16\x91\x16\x14\x15a3\xF4W\x80` \x01Q\x81`@\x01Q\x94P\x94PPPPa4eV[\x81Q\x81Q` \x80\x85\x01Q\x90\x84\x01Q\x91\x83\x90\x03\x92\x86\x03\x91c\xFF\xFF\xFF\xFF\x80\x84\x16\x92\x90\x85\x16\x91\x03`\x06\x0B\x81a4\"W\xFE[\x05\x02\x84` \x01Q\x01\x82c\xFF\xFF\xFF\xFF\x16\x82c\xFF\xFF\xFF\xFF\x16\x86`@\x01Q\x86`@\x01Q\x03`\x01`\x01`\xA0\x1B\x03\x16\x02\x81a4TW\xFE[\x04\x85`@\x01Q\x01\x96P\x96PPPPPP[\x97P\x97\x95PPPPPPV[`\x02\x95\x86\x0B\x86\x0B`\0\x90\x81R` \x97\x90\x97R`@\x90\x96 `\x01\x81\x01\x80T\x90\x95\x03\x90\x94U\x93\x83\x01\x80T\x90\x92\x03\x90\x91U`\x03\x82\x01\x80Tc\xFF\xFF\xFF\xFF`\x01`\xD8\x1B`\x01`\x01`\xA0\x1B\x03`\x01`8\x1B\x80\x85\x04\x82\x16\x90\x96\x03\x16\x90\x94\x02`\x01`8\x1B`\x01`\xD8\x1B\x03\x19\x90\x92\x16\x91\x90\x91\x17`\x06\x81\x81\x0B\x90\x96\x03\x90\x95\x0Bf\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16f\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x95\x16\x94\x90\x94\x17\x82\x81\x04\x85\x16\x90\x95\x03\x90\x93\x16\x02c\xFF\xFF\xFF\xFF`\xD8\x1B\x19\x90\x93\x16\x92\x90\x92\x17\x90UT`\x01`\x80\x1B\x90\x04`\x0F\x0B\x90V[`\0\x80\x82`\x0F\x0B\x12\x15a5\x90W\x82`\x01`\x01`\x80\x1B\x03\x16\x82`\0\x03\x84\x03\x91P\x81`\x01`\x01`\x80\x1B\x03\x16\x10a5\x8BW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x02`$\x82\x01RaLS`\xF0\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[a2\x13V[\x82`\x01`\x01`\x80\x1B\x03\x16\x82\x84\x01\x91P\x81`\x01`\x01`\x80\x1B\x03\x16\x10\x15a2\x13W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x02`$\x82\x01RaLA`\xF0\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\0d\x01\0\x02v\xA3`\x01`\x01`\xA0\x1B\x03\x83\x16\x10\x80\x15\x90a6\x1DWPs\xFF\xFD\x89c\xEF\xD1\xFCjPd\x88I]\x95\x1DRc\x98\x8D&`\x01`\x01`\xA0\x1B\x03\x83\x16\x10[a6RW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x01`$\x82\x01R`)`\xF9\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[d\x01\0\0\0\0`\x01`\xC0\x1B\x03` \x83\x90\x1B\x16`\x01`\x01`\x80\x1B\x03\x81\x11`\x07\x1B\x81\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11`\x06\x1B\x90\x81\x1Cc\xFF\xFF\xFF\xFF\x81\x11`\x05\x1B\x90\x81\x1Ca\xFF\xFF\x81\x11`\x04\x1B\x90\x81\x1C`\xFF\x81\x11`\x03\x90\x81\x1B\x91\x82\x1C`\x0F\x81\x11`\x02\x1B\x90\x81\x1C\x91\x82\x11`\x01\x90\x81\x1B\x92\x83\x1C\x97\x90\x88\x11\x96\x17\x90\x94\x17\x90\x92\x17\x17\x90\x91\x17\x17\x17`\x80\x81\x10a6\xE6W`\x7F\x81\x03\x83\x90\x1C\x91Pa6\xF0V[\x80`\x7F\x03\x83\x90\x1B\x91P[\x90\x80\x02`\x7F\x81\x81\x1C`\xFF\x83\x81\x1C\x91\x90\x91\x1C\x80\x02\x80\x83\x1C\x81\x83\x1C\x1C\x80\x02\x80\x84\x1C\x81\x84\x1C\x1C\x80\x02\x80\x85\x1C\x81\x85\x1C\x1C\x80\x02\x80\x86\x1C\x81\x86\x1C\x1C\x80\x02\x80\x87\x1C\x81\x87\x1C\x1C\x80\x02\x80\x88\x1C\x81\x88\x1C\x1C\x80\x02\x80\x89\x1C\x81\x89\x1C\x1C\x80\x02\x80\x8A\x1C\x81\x8A\x1C\x1C\x80\x02\x80\x8B\x1C\x81\x8B\x1C\x1C\x80\x02\x80\x8C\x1C\x81\x8C\x1C\x1C\x80\x02\x80\x8D\x1C\x81\x8D\x1C\x1C\x80\x02\x80\x8E\x1C\x9C\x81\x90\x1C\x9C\x90\x9C\x1C\x80\x02\x9C\x8D\x90\x1C\x9E\x9D`\x7F\x19\x8F\x01`@\x1B`\xC0\x91\x90\x91\x1Cg\x80\0\0\0\0\0\0\0\x16\x17`\xC1\x9B\x90\x9B\x1Cg@\0\0\0\0\0\0\0\x16\x9A\x90\x9A\x17`\xC2\x99\x90\x99\x1Cg \0\0\0\0\0\0\0\x16\x98\x90\x98\x17`\xC3\x97\x90\x97\x1Cg\x10\0\0\0\0\0\0\0\x16\x96\x90\x96\x17`\xC4\x95\x90\x95\x1Cg\x08\0\0\0\0\0\0\0\x16\x94\x90\x94\x17`\xC5\x93\x90\x93\x1Cg\x04\0\0\0\0\0\0\0\x16\x92\x90\x92\x17`\xC6\x91\x90\x91\x1Cg\x02\0\0\0\0\0\0\0\x16\x17`\xC7\x91\x90\x91\x1C`\x01`8\x1B\x16\x17`\xC8\x91\x90\x91\x1Cf\x80\0\0\0\0\0\0\x16\x17`\xC9\x91\x90\x91\x1Cf@\0\0\0\0\0\0\x16\x17`\xCA\x91\x90\x91\x1Cf \0\0\0\0\0\0\x16\x17`\xCB\x91\x90\x91\x1Cf\x10\0\0\0\0\0\0\x16\x17`\xCC\x91\x90\x91\x1Cf\x08\0\0\0\0\0\0\x16\x17`\xCD\x91\x90\x91\x1Cf\x04\0\0\0\0\0\0\x16\x17i6'\xA3\x01\xD7\x10UwL\x85\x81\x02o\x02\x8Fd\x81\xAB\x7F\x04ZZ\xF0\x12\xA1\x9D\0:\xA9\x19\x81\x01`\x80\x90\x81\x1D\x90o\xDB-\xF0\x9E\x81\x95\x9A\x81E^&\x07\x99\xA0c/\x83\x01\x90\x1D`\x02\x81\x81\x0B\x90\x83\x90\x0B\x14a8\xEDW\x88`\x01`\x01`\xA0\x1B\x03\x16a8\xD1\x82a,\xC4V[`\x01`\x01`\xA0\x1B\x03\x16\x11\x15a8\xE6W\x81a8\xE8V[\x80[a8\xEFV[\x81[\x99\x98PPPPPPPPPV[`\0\x80`\0\x89\x89a\xFF\xFF\x16a\xFF\xFF\x81\x10a9\x12W\xFE[`@\x80Q`\x80\x81\x01\x82R\x91\x90\x92\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x80\x84R`\x01` \x1B\x83\x04`\x06\x90\x81\x0B\x81\x0B\x90\x0B` \x85\x01R`\x01`X\x1B\x83\x04`\x01`\x01`\xA0\x1B\x03\x16\x94\x84\x01\x94\x90\x94R`\x01`\xF8\x1B\x90\x91\x04`\xFF\x16\x15\x15``\x83\x01R\x90\x92P\x89\x16\x14\x15a9\x81W\x88\x85\x92P\x92PPa4eV[\x84a\xFF\xFF\x16\x84a\xFF\xFF\x16\x11\x80\x15a9\xA2WP`\x01\x85\x03a\xFF\xFF\x16\x89a\xFF\xFF\x16\x14[\x15a9\xAFW\x83\x91Pa9\xB3V[\x84\x91P[\x81a\xFF\xFF\x16\x89`\x01\x01a\xFF\xFF\x16\x81a9\xC7W\xFE[\x06\x92Pa9\xD6\x81\x89\x89\x89aG V[\x8A\x84a\xFF\xFF\x16a\xFF\xFF\x81\x10a9\xE7W\xFE[\x82Q\x91\x01\x80T` \x84\x01Q`@\x85\x01Q``\x90\x95\x01Q\x15\x15`\x01`\xF8\x1B\x02`\x01`\x01`\xF8\x1B\x03`\x01`\x01`\xA0\x1B\x03\x90\x96\x16`\x01`X\x1B\x02\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x06\x93\x90\x93\x0Bf\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x01` \x1B\x02j\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\x19c\xFF\xFF\xFF\xFF\x90\x97\x16c\xFF\xFF\xFF\xFF\x19\x90\x95\x16\x94\x90\x94\x17\x95\x90\x95\x16\x92\x90\x92\x17\x16\x92\x90\x92\x17\x92\x90\x92\x16\x17\x90UP\x97P\x97\x95PPPPPPV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`$\x83\x01R`D\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`d\x90\x92\x01\x83R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\xA9\x05\x9C\xBB`\xE0\x1B\x17\x81R\x92Q\x82Q`\0\x94\x85\x94\x93\x89\x16\x93\x92\x91\x82\x91\x90\x80\x83\x83[` \x83\x10a;\x13W\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01a:\xF4V[`\x01\x83` \x03a\x01\0\n\x03\x80\x19\x82Q\x16\x81\x84Q\x16\x80\x82\x17\x85RPPPPPP\x90P\x01\x91PP`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a;uW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a;zV[``\x91P[P\x91P\x91P\x81\x80\x15a;\xA8WP\x80Q\x15\x80a;\xA8WP\x80\x80` \x01\x90Q` \x81\x10\x15a;\xA5W`\0\x80\xFD[PQ[a;\xDEW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x02`$\x82\x01Ra*#`\xF1\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[PPPPPV[`@\x80Q0`$\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`D\x90\x91\x01\x82R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16cp\xA0\x821`\xE0\x1B\x17\x81R\x91Q\x81Q`\0\x93\x84\x93\x84\x93`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x93\x91\x92\x90\x91\x82\x91\x90\x80\x83\x83[` \x83\x10a<~W\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01a<_V[`\x01\x83` \x03a\x01\0\n\x03\x80\x19\x82Q\x16\x81\x84Q\x16\x80\x82\x17\x85RPPPPPP\x90P\x01\x91PP`\0`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80`\0\x81\x14a<\xDEW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a<\xE3V[``\x91P[P\x91P\x91P\x81\x80\x15a<\xF7WP` \x81Q\x10\x15[a=\0W`\0\x80\xFD[\x80\x80` \x01\x90Q` \x81\x10\x15a=\x15W`\0\x80\xFD[PQ\x92PPP\x90V[\x80\x82\x01\x82\x81\x10\x15a2\x13W`\0\x80\xFD[`@\x80Q0`$\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`D\x90\x91\x01\x82R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16cp\xA0\x821`\xE0\x1B\x17\x81R\x91Q\x81Q`\0\x93\x84\x93\x84\x93`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x93\x91\x92\x90\x91\x82\x91\x90\x80\x83\x83` \x83\x10a<~W\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01a<_V[`\0\x80\x83a\xFF\xFF\x16\x11a>\x04W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x01`$\x82\x01R`I`\xF8\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x82a\xFF\xFF\x16\x82a\xFF\xFF\x16\x11a>\x1AWP\x81a2\xD8V[\x82[\x82a\xFF\xFF\x16\x81a\xFF\xFF\x16\x10\x15a>`W`\x01\x85\x82a\xFF\xFF\x16a\xFF\xFF\x81\x10a>?W\xFE[\x01\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90U`\x01\x01a>\x1CV[P\x90\x93\x92PPPV[\x80`\x0F\x81\x90\x0B\x81\x14a+BW`\0\x80\xFD[`\0\x80`\0a>\x87a+GV[a>\x99\x84` \x01Q\x85`@\x01QaB\xB2V[`@\x80Q`\xE0\x81\x01\x82R`\0T`\x01`\x01`\xA0\x1B\x03\x81\x16\x82R`\x01`\xA0\x1B\x81\x04`\x02\x90\x81\x0B\x81\x0B\x90\x0B` \x80\x84\x01\x82\x90Ra\xFF\xFF`\x01`\xB8\x1B\x84\x04\x81\x16\x85\x87\x01R`\x01`\xC8\x1B\x84\x04\x81\x16``\x80\x87\x01\x91\x90\x91R`\x01`\xD8\x1B\x85\x04\x90\x91\x16`\x80\x86\x01R`\xFF`\x01`\xE8\x1B\x85\x04\x81\x16`\xA0\x87\x01R`\x01`\xF0\x1B\x90\x94\x04\x90\x93\x16\x15\x15`\xC0\x85\x01R\x88Q\x90\x89\x01Q\x94\x89\x01Q\x92\x89\x01Q\x93\x94a?=\x94\x91\x93\x90\x92\x90\x91\x90aI\xBDV[\x93P\x84``\x01Q`\x0F\x0B`\0\x14a@\xB2W\x84` \x01Q`\x02\x0B\x81` \x01Q`\x02\x0B\x12\x15a?\x92Wa?\x8Ba?t\x86` \x01Qa,\xC4V[a?\x81\x87`@\x01Qa,\xC4V[\x87``\x01QaKrV[\x92Pa@\xB2V[\x84`@\x01Q`\x02\x0B\x81` \x01Q`\x02\x0B\x12\x15a@\x88W`\x04T`@\x82\x01Q`\x01`\x01`\x80\x1B\x03\x90\x91\x16\x90a?\xE4\x90a?\xC8a+~V[` \x85\x01Q``\x86\x01Q`\x80\x87\x01Q`\x08\x94\x93\x92\x91\x87\x91a8\xFCV[`\0\x80Ta\xFF\xFF`\xC8\x1B\x19\x16`\x01`\xC8\x1Ba\xFF\xFF\x93\x84\x16\x02\x17a\xFF\xFF`\xB8\x1B\x19\x16`\x01`\xB8\x1B\x93\x90\x92\x16\x92\x90\x92\x02\x17\x90U\x81Q`@\x87\x01Qa@4\x91\x90a@*\x90a,\xC4V[\x88``\x01QaKrV[\x93Pa@Ra@F\x87` \x01Qa,\xC4V[\x83Q``\x89\x01QaK\xB6V[\x92Pa@b\x81\x87``\x01Qa5+V[`\x04\x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UPa@\xB2V[a@\xAFa@\x98\x86` \x01Qa,\xC4V[a@\xA5\x87`@\x01Qa,\xC4V[\x87``\x01QaK\xB6V[\x91P[P\x91\x93\x90\x92PV[`\0a@\xC7\x84\x84\x84a2/V[\x90P`\0\x82\x80a@\xD3W\xFE[\x84\x86\t\x11\x15a2\xD8W`\0\x19\x81\x10a@\xEAW`\0\x80\xFD[`\x01\x01\x93\x92PPPV[`@\x80Q``\x94\x90\x94\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16` \x80\x86\x01\x91\x90\x91R`\x02\x93\x84\x0B`\xE8\x90\x81\x1B`4\x87\x01R\x92\x90\x93\x0B\x90\x91\x1B`7\x84\x01R\x80Q\x80\x84\x03`\x1A\x01\x81R`:\x90\x93\x01\x81R\x82Q\x92\x82\x01\x92\x90\x92 `\0\x90\x81R\x92\x90R\x90 \x90V[``\x80`\0\x83a\xFF\xFF\x16\x11aA\x98W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x01`$\x82\x01R`I`\xF8\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x86Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x80\x15aA\xB0W`\0\x80\xFD[P`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15aA\xDAW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x91P\x86Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x80\x15aA\xF5W`\0\x80\xFD[P`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15aB\x1FW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x87Q\x81\x10\x15aB\xA5WaBP\x8A\x8A\x8A\x84\x81Q\x81\x10aB?W\xFE[` \x02` \x01\x01Q\x8A\x8A\x8A\x8Aa2\xDFV[\x84\x83\x81Q\x81\x10aB\\W\xFE[` \x02` \x01\x01\x84\x84\x81Q\x81\x10aBoW\xFE[` \x02` \x01\x01\x82`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81RP\x82`\x06\x0B`\x06\x0B\x81RPPP\x80\x80`\x01\x01\x91PPaB%V[P\x97P\x97\x95PPPPPPV[\x80`\x02\x0B\x82`\x02\x0B\x12aB\xF2W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x03`$\x82\x01RbTLU`\xE8\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[b\r\x89\xE7\x19`\x02\x83\x90\x0B\x12\x15aC5W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x03`$\x82\x01RbTLM`\xE8\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[b\r\x89\xE8`\x02\x82\x90\x0B\x13\x15aCwW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x03`$\x82\x01RbTUM`\xE8\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[PPV[`@\x80Q`\x80\x81\x01\x82Rc\xFF\xFF\xFF\xFF\x92\x83\x16\x80\x82R`\0` \x83\x01\x81\x90R\x92\x82\x01\x92\x90\x92R`\x01``\x90\x91\x01\x81\x90R\x83Tc\xFF\xFF\xFF\xFF\x19\x16\x90\x91\x17\x90\x91\x16`\x01`\xF8\x1B\x17\x90\x91U\x90\x81\x90V[`\x02\x0B`\x08\x81\x90\x1D\x91a\x01\0\x90\x91\x07\x90V[`\0\x80\x82\x11aC\xE7W`\0\x80\xFD[`\x01`\x80\x1B\x82\x10aC\xFAW`\x80\x91\x82\x1C\x91\x01[h\x01\0\0\0\0\0\0\0\0\x82\x10aD\x12W`@\x91\x82\x1C\x91\x01[`\x01` \x1B\x82\x10aD%W` \x91\x82\x1C\x91\x01[b\x01\0\0\x82\x10aD7W`\x10\x91\x82\x1C\x91\x01[a\x01\0\x82\x10aDHW`\x08\x91\x82\x1C\x91\x01[`\x10\x82\x10aDXW`\x04\x91\x82\x1C\x91\x01[`\x04\x82\x10aDhW`\x02\x91\x82\x1C\x91\x01[`\x02\x82\x10a+BW`\x01\x01\x91\x90PV[`\0\x80\x82\x11aD\x86W`\0\x80\xFD[P`\xFF`\x01`\x01`\x80\x1B\x03\x82\x16\x15aD\xA1W`\x7F\x19\x01aD\xA9V[`\x80\x82\x90\x1C\x91P[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x15aD\xC2W`?\x19\x01aD\xCAV[`@\x82\x90\x1C\x91P[c\xFF\xFF\xFF\xFF\x82\x16\x15aD\xDFW`\x1F\x19\x01aD\xE7V[` \x82\x90\x1C\x91P[a\xFF\xFF\x82\x16\x15aD\xFAW`\x0F\x19\x01aE\x02V[`\x10\x82\x90\x1C\x91P[`\xFF\x82\x16\x15aE\x14W`\x07\x19\x01aE\x1CV[`\x08\x82\x90\x1C\x91P[`\x0F\x82\x16\x15aE.W`\x03\x19\x01aE6V[`\x04\x82\x90\x1C\x91P[`\x03\x82\x16\x15aEHW`\x01\x19\x01aEPV[`\x02\x82\x90\x1C\x91P[`\x01\x82\x16\x15a+BW`\0\x19\x01\x91\x90PV[`\0\x83`\x01`\x01`\xA0\x1B\x03\x16\x85`\x01`\x01`\xA0\x1B\x03\x16\x11\x15aE\x82W\x92\x93\x92[\x81aE\xAFWaE\xAA\x83`\x01`\x01`\x80\x1B\x03\x16\x86\x86\x03`\x01`\x01`\xA0\x1B\x03\x16`\x01``\x1Ba2/V[aE\xD2V[aE\xD2\x83`\x01`\x01`\x80\x1B\x03\x16\x86\x86\x03`\x01`\x01`\xA0\x1B\x03\x16`\x01``\x1Ba@\xBAV[\x90P[\x94\x93PPPPV[`\0\x83`\x01`\x01`\xA0\x1B\x03\x16\x85`\x01`\x01`\xA0\x1B\x03\x16\x11\x15aE\xFDW\x92\x93\x92[`\x01``\x1B`\x01`\xE0\x1B\x03``\x84\x90\x1B\x16`\x01`\x01`\xA0\x1B\x03\x86\x86\x03\x81\x16\x90\x87\x16aF'W`\0\x80\xFD[\x83aFWW\x86`\x01`\x01`\xA0\x1B\x03\x16aFJ\x83\x83\x89`\x01`\x01`\xA0\x1B\x03\x16a2/V[\x81aFQW\xFE[\x04aF}V[aF}aFn\x83\x83\x89`\x01`\x01`\xA0\x1B\x03\x16a@\xBAV[\x88`\x01`\x01`\xA0\x1B\x03\x16aK\xE5V[\x97\x96PPPPPPPV[`\0\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x11aF\x9FW`\0\x80\xFD[`\0\x84`\x01`\x01`\x80\x1B\x03\x16\x11aF\xB5W`\0\x80\xFD[\x81aF\xC7WaE\xAA\x85\x85\x85`\x01aK\xF0V[aE\xD2\x85\x85\x85`\x01aL\xD1V[`\0\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x11aF\xEBW`\0\x80\xFD[`\0\x84`\x01`\x01`\x80\x1B\x03\x16\x11aG\x01W`\0\x80\xFD[\x81aG\x13WaE\xAA\x85\x85\x85`\0aL\xD1V[aE\xD2\x85\x85\x85`\0aK\xF0V[aG(aT\xF1V[`\0\x85`\0\x01Q\x85\x03\x90P`@Q\x80`\x80\x01`@R\x80\x86c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x82c\xFF\xFF\xFF\xFF\x16\x86`\x02\x0B\x02\x88` \x01Q\x01`\x06\x0B\x81R` \x01`\0\x85`\x01`\x01`\x80\x1B\x03\x16\x11aG|W`\x01aG~V[\x84[`\x01`\x01`\x80\x1B\x03\x16c\xFF\xFF\xFF\xFF`\x80\x1B`\x80\x85\x90\x1B\x16\x81aG\x9CW\xFE[\x04\x88`@\x01Q\x01`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\x01\x15\x15\x81RP\x91PP\x94\x93PPPPV[aG\xCBaT\xF1V[aG\xD3aT\xF1V[\x88\x85a\xFF\xFF\x16a\xFF\xFF\x81\x10aG\xE4W\xFE[`@\x80Q`\x80\x81\x01\x82R\x91\x90\x92\x01Tc\xFF\xFF\xFF\xFF\x81\x16\x80\x83R`\x01` \x1B\x82\x04`\x06\x90\x81\x0B\x81\x0B\x90\x0B` \x84\x01R`\x01`X\x1B\x82\x04`\x01`\x01`\xA0\x1B\x03\x16\x93\x83\x01\x93\x90\x93R`\x01`\xF8\x1B\x90\x04`\xFF\x16\x15\x15``\x82\x01R\x92PaHH\x90\x89\x90\x89aM\xB4V[\x15aH\x80W\x86c\xFF\xFF\xFF\xFF\x16\x82`\0\x01Qc\xFF\xFF\xFF\xFF\x16\x14\x15aHjWa4eV[\x81aHw\x83\x89\x89\x88aG V[\x91P\x91Pa4eV[\x88\x83a\xFF\xFF\x16\x86`\x01\x01a\xFF\xFF\x16\x81aH\x95W\xFE[\x06a\xFF\xFF\x16a\xFF\xFF\x81\x10aH\xA5W\xFE[`@\x80Q`\x80\x81\x01\x82R\x92\x90\x91\x01Tc\xFF\xFF\xFF\xFF\x81\x16\x83R`\x01` \x1B\x81\x04`\x06\x90\x81\x0B\x81\x0B\x90\x0B` \x84\x01R`\x01`\x01`\xA0\x1B\x03`\x01`X\x1B\x82\x04\x16\x91\x83\x01\x91\x90\x91R`\xFF`\x01`\xF8\x1B\x90\x91\x04\x16\x15\x15``\x82\x01\x81\x90R\x90\x92PaIZW`@\x80Q`\x80\x81\x01\x82R\x8ATc\xFF\xFF\xFF\xFF\x81\x16\x82R`\x01` \x1B\x81\x04`\x06\x90\x81\x0B\x81\x0B\x90\x0B` \x83\x01R`\x01`X\x1B\x81\x04`\x01`\x01`\xA0\x1B\x03\x16\x92\x82\x01\x92\x90\x92R`\x01`\xF8\x1B\x90\x91\x04`\xFF\x16\x15\x15``\x82\x01R\x91P[aIi\x88\x83`\0\x01Q\x89aM\xB4V[aI\xA0W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x03`$\x82\x01Rb\x13\xD3\x11`\xEA\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[aI\xAD\x89\x89\x89\x88\x87aNuV[\x91P\x91P\x97P\x97\x95PPPPPPV[`\0aI\xCC`\x07\x87\x87\x87a@\xF4V[`\x01T`\x02T\x91\x92P\x90`\0\x80`\x0F\x87\x90\x0B\x15aK\x12W`\0aI\xEDa+~V[`\0\x80T`\x04T\x92\x93P\x90\x91\x82\x91aJ7\x91`\x08\x91\x86\x91\x85\x91`\x01`\xA0\x1B\x81\x04`\x02\x0B\x91a\xFF\xFF`\x01`\xB8\x1B\x83\x04\x81\x16\x92`\x01`\x01`\x80\x1B\x03\x90\x92\x16\x91`\x01`\xC8\x1B\x90\x04\x16a2\xDFV[\x90\x92P\x90PaJq`\x05\x8D\x8B\x8D\x8B\x8B\x87\x89\x8B`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0aP\x13V[\x94PaJ\xA8`\x05\x8C\x8B\x8D\x8B\x8B\x87\x89\x8B`\x01\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0aP\x13V[\x93P\x84\x15aJ\xDCWaJ\xDC`\x06\x8D\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0aQ\xCCV[\x83\x15aK\x0EWaK\x0E`\x06\x8C\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0aQ\xCCV[PPP[`\0\x80aK$`\x05\x8C\x8C\x8B\x8A\x8AaR2V[\x90\x92P\x90PaK5\x87\x8A\x84\x84aR\xDEV[`\0\x89`\x0F\x0B\x12\x15aKcW\x83\x15aKRWaKR`\x05\x8CaTsV[\x82\x15aKcWaKc`\x05\x8BaTsV[PPPPPP\x95\x94PPPPPV[`\0\x80\x82`\x0F\x0B\x12aK\x98WaK\x93aK\x8E\x85\x85\x85`\x01aE\xDDV[a1\xE7V[aE\xD5V[aK\xABaK\x8E\x85\x85\x85`\0\x03`\0aE\xDDV[`\0\x03\x94\x93PPPPV[`\0\x80\x82`\x0F\x0B\x12aK\xD2WaK\x93aK\x8E\x85\x85\x85`\x01aEbV[aK\xABaK\x8E\x85\x85\x85`\0\x03`\0aEbV[\x80\x82\x04\x91\x06\x15\x15\x01\x90V[`\0\x81\x15aLcW`\0`\x01`\x01`\xA0\x1B\x03\x84\x11\x15aL&WaL!\x84`\x01``\x1B\x87`\x01`\x01`\x80\x1B\x03\x16a2/V[aL>V[`\x01`\x01`\x80\x1B\x03\x85\x16``\x85\x90\x1B\x81aL<W\xFE[\x04[\x90PaL[aLV`\x01`\x01`\xA0\x1B\x03\x88\x16\x83a=\x1EV[aT\x9FV[\x91PPaE\xD5V[`\0`\x01`\x01`\xA0\x1B\x03\x84\x11\x15aL\x91WaL\x8C\x84`\x01``\x1B\x87`\x01`\x01`\x80\x1B\x03\x16a@\xBAV[aL\xA8V[aL\xA8``\x85\x90\x1B`\x01`\x01`\x80\x1B\x03\x87\x16aK\xE5V[\x90P\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x11aL\xBFW`\0\x80\xFD[`\x01`\x01`\xA0\x1B\x03\x86\x16\x03\x90PaE\xD5V[`\0\x82aL\xDFWP\x83aE\xD5V[`\x01``\x1B`\x01`\xE0\x1B\x03``\x85\x90\x1B\x16\x82\x15aMmW`\x01`\x01`\xA0\x1B\x03\x86\x16\x84\x81\x02\x90\x85\x82\x81aM\rW\xFE[\x04\x14\x15aM>W\x81\x81\x01\x82\x81\x10aM<WaM2\x83\x89`\x01`\x01`\xA0\x1B\x03\x16\x83a@\xBAV[\x93PPPPaE\xD5V[P[aMd\x82aM_\x87\x8A`\x01`\x01`\xA0\x1B\x03\x16\x86\x81aMXW\xFE[\x04\x90a=\x1EV[aK\xE5V[\x92PPPaE\xD5V[`\x01`\x01`\xA0\x1B\x03\x86\x16\x84\x81\x02\x90\x85\x82\x81aM\x84W\xFE[\x04\x14\x80\x15aM\x91WP\x80\x82\x11[aM\x9AW`\0\x80\xFD[\x80\x82\x03aM2aLV\x84`\x01`\x01`\xA0\x1B\x03\x8B\x16\x84a@\xBAV[`\0\x83c\xFF\xFF\xFF\xFF\x16\x83c\xFF\xFF\xFF\xFF\x16\x11\x15\x80\x15aM\xDEWP\x83c\xFF\xFF\xFF\xFF\x16\x82c\xFF\xFF\xFF\xFF\x16\x11\x15[\x15aM\xFAW\x81c\xFF\xFF\xFF\xFF\x16\x83c\xFF\xFF\xFF\xFF\x16\x11\x15\x90Pa2\xD8V[`\0\x84c\xFF\xFF\xFF\xFF\x16\x84c\xFF\xFF\xFF\xFF\x16\x11aN!W\x83c\xFF\xFF\xFF\xFF\x16`\x01` \x1B\x01aN)V[\x83c\xFF\xFF\xFF\xFF\x16[d\xFF\xFF\xFF\xFF\xFF\x16\x90P`\0\x85c\xFF\xFF\xFF\xFF\x16\x84c\xFF\xFF\xFF\xFF\x16\x11aNYW\x83c\xFF\xFF\xFF\xFF\x16`\x01` \x1B\x01aNaV[\x83c\xFF\xFF\xFF\xFF\x16[d\xFF\xFF\xFF\xFF\xFF\x16\x90\x91\x11\x15\x95\x94PPPPPV[aN}aT\xF1V[aN\x85aT\xF1V[`\0\x83a\xFF\xFF\x16\x85`\x01\x01a\xFF\xFF\x16\x81aN\x9BW\xFE[\x06a\xFF\xFF\x16\x90P`\0`\x01\x85a\xFF\xFF\x16\x83\x01\x03\x90P`\0[P`\x02\x81\x83\x01\x04\x89a\xFF\xFF\x87\x16\x82\x81aN\xC8W\xFE[\x06a\xFF\xFF\x81\x10aN\xD4W\xFE[`@\x80Q`\x80\x81\x01\x82R\x92\x90\x91\x01Tc\xFF\xFF\xFF\xFF\x81\x16\x83R`\x01` \x1B\x81\x04`\x06\x90\x81\x0B\x81\x0B\x90\x0B` \x84\x01R`\x01`\x01`\xA0\x1B\x03`\x01`X\x1B\x82\x04\x16\x91\x83\x01\x91\x90\x91R`\xFF`\x01`\xF8\x1B\x90\x91\x04\x16\x15\x15``\x82\x01\x81\x90R\x90\x95PaO>W\x80`\x01\x01\x92PaN\xB3V[\x89\x86a\xFF\xFF\x16\x82`\x01\x01\x81aOOW\xFE[\x06a\xFF\xFF\x81\x10aO[W\xFE[`@\x80Q`\x80\x81\x01\x82R\x92\x90\x91\x01Tc\xFF\xFF\xFF\xFF\x81\x16\x83R`\x01` \x1B\x81\x04`\x06\x90\x81\x0B\x81\x0B\x90\x0B` \x84\x01R`\x01`\x01`\xA0\x1B\x03`\x01`X\x1B\x82\x04\x16\x91\x83\x01\x91\x90\x91R`\xFF`\x01`\xF8\x1B\x90\x91\x04\x16\x15\x15``\x82\x01R\x85Q\x90\x94P`\0\x90aO\xC5\x90\x8B\x90\x8BaM\xB4V[\x90P\x80\x80\x15aO\xDEWPaO\xDE\x8A\x8A\x87`\0\x01QaM\xB4V[\x15aO\xE9WPaP\x06V[\x80aO\xF9W`\x01\x82\x03\x92PaP\0V[\x81`\x01\x01\x93P[PaN\xB3V[PPP\x95P\x95\x93PPPPV[`\x02\x8A\x81\x0B\x90\x0B`\0\x90\x81R` \x8C\x90R`@\x81 \x80T`\x01`\x01`\x80\x1B\x03\x16\x82aP>\x82\x8Da5+V[\x90P\x84`\x01`\x01`\x80\x1B\x03\x16\x81`\x01`\x01`\x80\x1B\x03\x16\x11\x15aP\x8CW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x02`$\x82\x01RaLO`\xF0\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\x01`\x01`\x80\x1B\x03\x82\x81\x16\x15\x90\x82\x16\x15\x81\x14\x15\x94P\x15aQ1W\x8C`\x02\x0B\x8E`\x02\x0B\x13aQ\x19W`\x01\x83\x01\x8B\x90U`\x02\x83\x01\x8A\x90U`\x03\x83\x01\x80T`\x01`8\x1B`\x01`\xD8\x1B\x03\x19\x16`\x01`8\x1B`\x01`\x01`\xA0\x1B\x03\x8C\x16\x02\x17f\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16f\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x06\x8B\x90\x0B\x16\x17c\xFF\xFF\xFF\xFF`\xD8\x1B\x19\x16`\x01`\xD8\x1Bc\xFF\xFF\xFF\xFF\x8A\x16\x02\x17\x90U[`\x03\x83\x01\x80T`\x01`\x01`\xF8\x1B\x03\x16`\x01`\xF8\x1B\x17\x90U[\x82T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x82\x16\x17\x83U\x85aQzW\x82TaQu\x90aQp\x90`\x01`\x80\x1B\x90\x04`\x0F\x90\x81\x0B\x81\x0B\x90\x8F\x90\x0Ba2\x19V[a>iV[aQ\x9BV[\x82TaQ\x9B\x90aQp\x90`\x01`\x80\x1B\x90\x04`\x0F\x90\x81\x0B\x81\x0B\x90\x8F\x90\x0Ba1\xFDV[\x83T`\x0F\x91\x90\x91\x0B`\x01`\x01`\x80\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02\x91\x16\x17\x90\x92UP\x90\x9C\x9BPPPPPPPPPPPPV[\x80`\x02\x0B\x82`\x02\x0B\x81aQ\xDBW\xFE[\x07`\x02\x0B\x15aQ\xE9W`\0\x80\xFD[`\0\x80aR\x04\x83`\x02\x0B\x85`\x02\x0B\x81aQ\xFEW\xFE[\x05aC\xC7V[`\x01\x91\x82\x0B\x82\x0B`\0\x90\x81R` \x97\x90\x97R`@\x90\x96 \x80T`\xFF\x90\x97\x16\x91\x90\x91\x1B\x90\x95\x18\x90\x94UPPPPV[`\x02\x85\x81\x0B\x80\x82\x0B`\0\x90\x81R` \x89\x90R`@\x80\x82 \x88\x85\x0B\x85\x0B\x83R\x90\x82 \x91\x93\x84\x93\x91\x92\x91\x84\x91\x82\x91\x90\x8A\x90\x0B\x12aRxWPP`\x01\x82\x01T`\x02\x83\x01TaR\x8BV[\x83`\x01\x01T\x88\x03\x91P\x83`\x02\x01T\x87\x03\x90P[`\0\x80\x8B`\x02\x0B\x8B`\x02\x0B\x12\x15aR\xADWPP`\x01\x83\x01T`\x02\x84\x01TaR\xC0V[\x84`\x01\x01T\x8A\x03\x91P\x84`\x02\x01T\x89\x03\x90P[\x92\x90\x98\x03\x97\x90\x97\x03\x9B\x96\x90\x95\x03\x94\x90\x94\x03\x98P\x93\x96PPPPPPPV[`@\x80Q`\xA0\x81\x01\x82R\x85T`\x01`\x01`\x80\x1B\x03\x90\x81\x16\x82R`\x01\x87\x01T` \x83\x01R`\x02\x87\x01T\x92\x82\x01\x92\x90\x92R`\x03\x86\x01T\x80\x83\x16``\x83\x01R`\x01`\x80\x1B\x90\x04\x90\x91\x16`\x80\x82\x01R`\0`\x0F\x85\x90\x0BaS}W\x81Q`\x01`\x01`\x80\x1B\x03\x16aSuW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x02`$\x82\x01Ra\x04\xE5`\xF4\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[P\x80QaS\x8CV[\x81QaS\x89\x90\x86a5+V[\x90P[`\0aS\xB0\x83` \x01Q\x86\x03\x84`\0\x01Q`\x01`\x01`\x80\x1B\x03\x16`\x01`\x80\x1Ba2/V[\x90P`\0aS\xD6\x84`@\x01Q\x86\x03\x85`\0\x01Q`\x01`\x01`\x80\x1B\x03\x16`\x01`\x80\x1Ba2/V[\x90P\x86`\x0F\x0B`\0\x14aS\xFDW\x87T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x84\x16\x17\x88U[`\x01\x88\x01\x86\x90U`\x02\x88\x01\x85\x90U`\x01`\x01`\x80\x1B\x03\x82\x16\x15\x15\x80aT+WP`\0\x81`\x01`\x01`\x80\x1B\x03\x16\x11[\x15aTiW`\x03\x88\x01\x80T`\x01`\x01`\x80\x1B\x03\x19\x81\x16`\x01`\x01`\x80\x1B\x03\x91\x82\x16\x85\x01\x82\x16\x17\x80\x82\x16`\x01`\x80\x1B\x91\x82\x90\x04\x83\x16\x85\x01\x90\x92\x16\x02\x17\x90U[PPPPPPPPV[`\x02\x90\x81\x0B\x81\x0B`\0\x90\x81R` \x92\x90\x92R`@\x82 \x82\x81U`\x01\x81\x01\x83\x90U\x90\x81\x01\x82\x90U`\x03\x01UV[\x80`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a+BW`\0\x80\xFD[`@\x80Q`\xE0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x91\x90\x91R\x90V[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91R\x90V\xFE\xA2dipfsX\"\x12 \xEA\xA0\xBDv\xAAV\x9Er\xA2\x92\x91\xA7J\x80HU\x02\xC0\xA8W\xBB\xFF\t\x90\x94\xD0\x1AdS\x9A\x07\xE8dsolcC\0\x07\x06\x003";
    /// The deployed bytecode of the contract.
    pub static UNISWAPV3POOL_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct UniswapV3Pool<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for UniswapV3Pool<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for UniswapV3Pool<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for UniswapV3Pool<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for UniswapV3Pool<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(UniswapV3Pool))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> UniswapV3Pool<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    UNISWAPV3POOL_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                UNISWAPV3POOL_ABI.clone(),
                UNISWAPV3POOL_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `burn` (0xa34123a7) function
        pub fn burn(
            &self,
            tick_lower: i32,
            tick_upper: i32,
            amount: u128,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([163, 65, 35, 167], (tick_lower, tick_upper, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `collect` (0x4f1eb3d8) function
        pub fn collect(
            &self,
            recipient: ::ethers::core::types::Address,
            tick_lower: i32,
            tick_upper: i32,
            amount_0_requested: u128,
            amount_1_requested: u128,
        ) -> ::ethers::contract::builders::ContractCall<M, (u128, u128)> {
            self.0
                .method_hash(
                    [79, 30, 179, 216],
                    (
                        recipient,
                        tick_lower,
                        tick_upper,
                        amount_0_requested,
                        amount_1_requested,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `collectProtocol` (0x85b66729) function
        pub fn collect_protocol(
            &self,
            recipient: ::ethers::core::types::Address,
            amount_0_requested: u128,
            amount_1_requested: u128,
        ) -> ::ethers::contract::builders::ContractCall<M, (u128, u128)> {
            self.0
                .method_hash(
                    [133, 182, 103, 41],
                    (recipient, amount_0_requested, amount_1_requested),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `factory` (0xc45a0155) function
        pub fn factory(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([196, 90, 1, 85], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `fee` (0xddca3f43) function
        pub fn fee(&self) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([221, 202, 63, 67], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `feeGrowthGlobal0X128` (0xf3058399) function
        pub fn fee_growth_global_0x128(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([243, 5, 131, 153], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `feeGrowthGlobal1X128` (0x46141319) function
        pub fn fee_growth_global_1x128(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([70, 20, 19, 25], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `flash` (0x490e6cbc) function
        pub fn flash(
            &self,
            recipient: ::ethers::core::types::Address,
            amount_0: ::ethers::core::types::U256,
            amount_1: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([73, 14, 108, 188], (recipient, amount_0, amount_1, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `increaseObservationCardinalityNext` (0x32148f67) function
        pub fn increase_observation_cardinality_next(
            &self,
            observation_cardinality_next: u16,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([50, 20, 143, 103], observation_cardinality_next)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0xf637731d) function
        pub fn initialize(
            &self,
            sqrt_price_x96: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([246, 55, 115, 29], sqrt_price_x96)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `liquidity` (0x1a686502) function
        pub fn liquidity(&self) -> ::ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([26, 104, 101, 2], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `maxLiquidityPerTick` (0x70cf754a) function
        pub fn max_liquidity_per_tick(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([112, 207, 117, 74], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mint` (0x3c8a7d8d) function
        pub fn mint(
            &self,
            recipient: ::ethers::core::types::Address,
            tick_lower: i32,
            tick_upper: i32,
            amount: u128,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash(
                    [60, 138, 125, 141],
                    (recipient, tick_lower, tick_upper, amount, data),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `observations` (0x252c09d7) function
        pub fn observations(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (u32, i64, ::ethers::core::types::U256, bool),
        > {
            self.0
                .method_hash([37, 44, 9, 215], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `observe` (0x883bdbfd) function
        pub fn observe(
            &self,
            seconds_agos: ::std::vec::Vec<u32>,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::std::vec::Vec<i64>, ::std::vec::Vec<::ethers::core::types::U256>),
        > {
            self.0
                .method_hash([136, 59, 219, 253], seconds_agos)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `positions` (0x514ea4bf) function
        pub fn positions(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (u128, ::ethers::core::types::U256, ::ethers::core::types::U256, u128, u128),
        > {
            self.0
                .method_hash([81, 78, 164, 191], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `protocolFees` (0x1ad8b03b) function
        pub fn protocol_fees(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, (u128, u128)> {
            self.0
                .method_hash([26, 216, 176, 59], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setFeeProtocol` (0x8206a4d1) function
        pub fn set_fee_protocol(
            &self,
            fee_protocol_0: u8,
            fee_protocol_1: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([130, 6, 164, 209], (fee_protocol_0, fee_protocol_1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `slot0` (0x3850c7bd) function
        pub fn slot_0(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, i32, u16, u16, u16, u8, bool),
        > {
            self.0
                .method_hash([56, 80, 199, 189], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `snapshotCumulativesInside` (0xa38807f2) function
        pub fn snapshot_cumulatives_inside(
            &self,
            tick_lower: i32,
            tick_upper: i32,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (i64, ::ethers::core::types::U256, u32),
        > {
            self.0
                .method_hash([163, 136, 7, 242], (tick_lower, tick_upper))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swap` (0x128acb08) function
        pub fn swap(
            &self,
            recipient: ::ethers::core::types::Address,
            zero_for_one: bool,
            amount_specified: ::ethers::core::types::I256,
            sqrt_price_limit_x96: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::I256, ::ethers::core::types::I256),
        > {
            self.0
                .method_hash(
                    [18, 138, 203, 8],
                    (
                        recipient,
                        zero_for_one,
                        amount_specified,
                        sqrt_price_limit_x96,
                        data,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `tickBitmap` (0x5339c296) function
        pub fn tick_bitmap(
            &self,
            p0: i16,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([83, 57, 194, 150], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `tickSpacing` (0xd0c93a7c) function
        pub fn tick_spacing(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, i32> {
            self.0
                .method_hash([208, 201, 58, 124], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ticks` (0xf30dba93) function
        pub fn ticks(
            &self,
            p0: i32,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                u128,
                i128,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                i64,
                ::ethers::core::types::U256,
                u32,
                bool,
            ),
        > {
            self.0
                .method_hash([243, 13, 186, 147], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `token0` (0x0dfe1681) function
        pub fn token_0(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([13, 254, 22, 129], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `token1` (0xd21220a7) function
        pub fn token_1(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([210, 18, 32, 167], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Burn` event
        pub fn burn_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, BurnFilter> {
            self.0.event()
        }
        ///Gets the contract's `Collect` event
        pub fn collect_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, CollectFilter> {
            self.0.event()
        }
        ///Gets the contract's `CollectProtocol` event
        pub fn collect_protocol_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            CollectProtocolFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Flash` event
        pub fn flash_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, FlashFilter> {
            self.0.event()
        }
        ///Gets the contract's `IncreaseObservationCardinalityNext` event
        pub fn increase_observation_cardinality_next_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            IncreaseObservationCardinalityNextFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Initialize` event
        pub fn initialize_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            InitializeFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Mint` event
        pub fn mint_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, MintFilter> {
            self.0.event()
        }
        ///Gets the contract's `SetFeeProtocol` event
        pub fn set_fee_protocol_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SetFeeProtocolFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Swap` event
        pub fn swap_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SwapFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            UniswapV3PoolEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for UniswapV3Pool<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Burn", abi = "Burn(address,int24,int24,uint128,uint256,uint256)")]
    pub struct BurnFilter {
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub tick_lower: i32,
        #[ethevent(indexed)]
        pub tick_upper: i32,
        pub amount: u128,
        pub amount_0: ::ethers::core::types::U256,
        pub amount_1: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "Collect",
        abi = "Collect(address,address,int24,int24,uint128,uint128)"
    )]
    pub struct CollectFilter {
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        pub recipient: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub tick_lower: i32,
        #[ethevent(indexed)]
        pub tick_upper: i32,
        pub amount_0: u128,
        pub amount_1: u128,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "CollectProtocol",
        abi = "CollectProtocol(address,address,uint128,uint128)"
    )]
    pub struct CollectProtocolFilter {
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub recipient: ::ethers::core::types::Address,
        pub amount_0: u128,
        pub amount_1: u128,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "Flash",
        abi = "Flash(address,address,uint256,uint256,uint256,uint256)"
    )]
    pub struct FlashFilter {
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub recipient: ::ethers::core::types::Address,
        pub amount_0: ::ethers::core::types::U256,
        pub amount_1: ::ethers::core::types::U256,
        pub paid_0: ::ethers::core::types::U256,
        pub paid_1: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "IncreaseObservationCardinalityNext",
        abi = "IncreaseObservationCardinalityNext(uint16,uint16)"
    )]
    pub struct IncreaseObservationCardinalityNextFilter {
        pub observation_cardinality_next_old: u16,
        pub observation_cardinality_next_new: u16,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Initialize", abi = "Initialize(uint160,int24)")]
    pub struct InitializeFilter {
        pub sqrt_price_x96: ::ethers::core::types::U256,
        pub tick: i32,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "Mint",
        abi = "Mint(address,address,int24,int24,uint128,uint256,uint256)"
    )]
    pub struct MintFilter {
        pub sender: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub tick_lower: i32,
        #[ethevent(indexed)]
        pub tick_upper: i32,
        pub amount: u128,
        pub amount_0: ::ethers::core::types::U256,
        pub amount_1: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "SetFeeProtocol", abi = "SetFeeProtocol(uint8,uint8,uint8,uint8)")]
    pub struct SetFeeProtocolFilter {
        pub fee_protocol_0_old: u8,
        pub fee_protocol_1_old: u8,
        pub fee_protocol_0_new: u8,
        pub fee_protocol_1_new: u8,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "Swap",
        abi = "Swap(address,address,int256,int256,uint160,uint128,int24)"
    )]
    pub struct SwapFilter {
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub recipient: ::ethers::core::types::Address,
        pub amount_0: ::ethers::core::types::I256,
        pub amount_1: ::ethers::core::types::I256,
        pub sqrt_price_x96: ::ethers::core::types::U256,
        pub liquidity: u128,
        pub tick: i32,
    }
    ///Container type for all of the contract's events
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub enum UniswapV3PoolEvents {
        BurnFilter(BurnFilter),
        CollectFilter(CollectFilter),
        CollectProtocolFilter(CollectProtocolFilter),
        FlashFilter(FlashFilter),
        IncreaseObservationCardinalityNextFilter(
            IncreaseObservationCardinalityNextFilter,
        ),
        InitializeFilter(InitializeFilter),
        MintFilter(MintFilter),
        SetFeeProtocolFilter(SetFeeProtocolFilter),
        SwapFilter(SwapFilter),
    }
    impl ::ethers::contract::EthLogDecode for UniswapV3PoolEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = BurnFilter::decode_log(log) {
                return Ok(UniswapV3PoolEvents::BurnFilter(decoded));
            }
            if let Ok(decoded) = CollectFilter::decode_log(log) {
                return Ok(UniswapV3PoolEvents::CollectFilter(decoded));
            }
            if let Ok(decoded) = CollectProtocolFilter::decode_log(log) {
                return Ok(UniswapV3PoolEvents::CollectProtocolFilter(decoded));
            }
            if let Ok(decoded) = FlashFilter::decode_log(log) {
                return Ok(UniswapV3PoolEvents::FlashFilter(decoded));
            }
            if let Ok(decoded) = IncreaseObservationCardinalityNextFilter::decode_log(
                log,
            ) {
                return Ok(
                    UniswapV3PoolEvents::IncreaseObservationCardinalityNextFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = InitializeFilter::decode_log(log) {
                return Ok(UniswapV3PoolEvents::InitializeFilter(decoded));
            }
            if let Ok(decoded) = MintFilter::decode_log(log) {
                return Ok(UniswapV3PoolEvents::MintFilter(decoded));
            }
            if let Ok(decoded) = SetFeeProtocolFilter::decode_log(log) {
                return Ok(UniswapV3PoolEvents::SetFeeProtocolFilter(decoded));
            }
            if let Ok(decoded) = SwapFilter::decode_log(log) {
                return Ok(UniswapV3PoolEvents::SwapFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for UniswapV3PoolEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BurnFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::CollectFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::CollectProtocolFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FlashFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::IncreaseObservationCardinalityNextFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InitializeFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::MintFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetFeeProtocolFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SwapFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<BurnFilter> for UniswapV3PoolEvents {
        fn from(value: BurnFilter) -> Self {
            Self::BurnFilter(value)
        }
    }
    impl ::core::convert::From<CollectFilter> for UniswapV3PoolEvents {
        fn from(value: CollectFilter) -> Self {
            Self::CollectFilter(value)
        }
    }
    impl ::core::convert::From<CollectProtocolFilter> for UniswapV3PoolEvents {
        fn from(value: CollectProtocolFilter) -> Self {
            Self::CollectProtocolFilter(value)
        }
    }
    impl ::core::convert::From<FlashFilter> for UniswapV3PoolEvents {
        fn from(value: FlashFilter) -> Self {
            Self::FlashFilter(value)
        }
    }
    impl ::core::convert::From<IncreaseObservationCardinalityNextFilter>
    for UniswapV3PoolEvents {
        fn from(value: IncreaseObservationCardinalityNextFilter) -> Self {
            Self::IncreaseObservationCardinalityNextFilter(value)
        }
    }
    impl ::core::convert::From<InitializeFilter> for UniswapV3PoolEvents {
        fn from(value: InitializeFilter) -> Self {
            Self::InitializeFilter(value)
        }
    }
    impl ::core::convert::From<MintFilter> for UniswapV3PoolEvents {
        fn from(value: MintFilter) -> Self {
            Self::MintFilter(value)
        }
    }
    impl ::core::convert::From<SetFeeProtocolFilter> for UniswapV3PoolEvents {
        fn from(value: SetFeeProtocolFilter) -> Self {
            Self::SetFeeProtocolFilter(value)
        }
    }
    impl ::core::convert::From<SwapFilter> for UniswapV3PoolEvents {
        fn from(value: SwapFilter) -> Self {
            Self::SwapFilter(value)
        }
    }
    ///Container type for all input parameters for the `burn` function with signature `burn(int24,int24,uint128)` and selector `0xa34123a7`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "burn", abi = "burn(int24,int24,uint128)")]
    pub struct BurnCall {
        pub tick_lower: i32,
        pub tick_upper: i32,
        pub amount: u128,
    }
    ///Container type for all input parameters for the `collect` function with signature `collect(address,int24,int24,uint128,uint128)` and selector `0x4f1eb3d8`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "collect", abi = "collect(address,int24,int24,uint128,uint128)")]
    pub struct CollectCall {
        pub recipient: ::ethers::core::types::Address,
        pub tick_lower: i32,
        pub tick_upper: i32,
        pub amount_0_requested: u128,
        pub amount_1_requested: u128,
    }
    ///Container type for all input parameters for the `collectProtocol` function with signature `collectProtocol(address,uint128,uint128)` and selector `0x85b66729`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "collectProtocol",
        abi = "collectProtocol(address,uint128,uint128)"
    )]
    pub struct CollectProtocolCall {
        pub recipient: ::ethers::core::types::Address,
        pub amount_0_requested: u128,
        pub amount_1_requested: u128,
    }
    ///Container type for all input parameters for the `factory` function with signature `factory()` and selector `0xc45a0155`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "factory", abi = "factory()")]
    pub struct FactoryCall;
    ///Container type for all input parameters for the `fee` function with signature `fee()` and selector `0xddca3f43`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "fee", abi = "fee()")]
    pub struct FeeCall;
    ///Container type for all input parameters for the `feeGrowthGlobal0X128` function with signature `feeGrowthGlobal0X128()` and selector `0xf3058399`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "feeGrowthGlobal0X128", abi = "feeGrowthGlobal0X128()")]
    pub struct FeeGrowthGlobal0X128Call;
    ///Container type for all input parameters for the `feeGrowthGlobal1X128` function with signature `feeGrowthGlobal1X128()` and selector `0x46141319`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "feeGrowthGlobal1X128", abi = "feeGrowthGlobal1X128()")]
    pub struct FeeGrowthGlobal1X128Call;
    ///Container type for all input parameters for the `flash` function with signature `flash(address,uint256,uint256,bytes)` and selector `0x490e6cbc`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "flash", abi = "flash(address,uint256,uint256,bytes)")]
    pub struct FlashCall {
        pub recipient: ::ethers::core::types::Address,
        pub amount_0: ::ethers::core::types::U256,
        pub amount_1: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `increaseObservationCardinalityNext` function with signature `increaseObservationCardinalityNext(uint16)` and selector `0x32148f67`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "increaseObservationCardinalityNext",
        abi = "increaseObservationCardinalityNext(uint16)"
    )]
    pub struct IncreaseObservationCardinalityNextCall {
        pub observation_cardinality_next: u16,
    }
    ///Container type for all input parameters for the `initialize` function with signature `initialize(uint160)` and selector `0xf637731d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "initialize", abi = "initialize(uint160)")]
    pub struct InitializeCall {
        pub sqrt_price_x96: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `liquidity` function with signature `liquidity()` and selector `0x1a686502`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "liquidity", abi = "liquidity()")]
    pub struct LiquidityCall;
    ///Container type for all input parameters for the `maxLiquidityPerTick` function with signature `maxLiquidityPerTick()` and selector `0x70cf754a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "maxLiquidityPerTick", abi = "maxLiquidityPerTick()")]
    pub struct MaxLiquidityPerTickCall;
    ///Container type for all input parameters for the `mint` function with signature `mint(address,int24,int24,uint128,bytes)` and selector `0x3c8a7d8d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "mint", abi = "mint(address,int24,int24,uint128,bytes)")]
    pub struct MintCall {
        pub recipient: ::ethers::core::types::Address,
        pub tick_lower: i32,
        pub tick_upper: i32,
        pub amount: u128,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `observations` function with signature `observations(uint256)` and selector `0x252c09d7`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "observations", abi = "observations(uint256)")]
    pub struct ObservationsCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `observe` function with signature `observe(uint32[])` and selector `0x883bdbfd`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "observe", abi = "observe(uint32[])")]
    pub struct ObserveCall {
        pub seconds_agos: ::std::vec::Vec<u32>,
    }
    ///Container type for all input parameters for the `positions` function with signature `positions(bytes32)` and selector `0x514ea4bf`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "positions", abi = "positions(bytes32)")]
    pub struct PositionsCall(pub [u8; 32]);
    ///Container type for all input parameters for the `protocolFees` function with signature `protocolFees()` and selector `0x1ad8b03b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "protocolFees", abi = "protocolFees()")]
    pub struct ProtocolFeesCall;
    ///Container type for all input parameters for the `setFeeProtocol` function with signature `setFeeProtocol(uint8,uint8)` and selector `0x8206a4d1`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setFeeProtocol", abi = "setFeeProtocol(uint8,uint8)")]
    pub struct SetFeeProtocolCall {
        pub fee_protocol_0: u8,
        pub fee_protocol_1: u8,
    }
    ///Container type for all input parameters for the `slot0` function with signature `slot0()` and selector `0x3850c7bd`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "slot0", abi = "slot0()")]
    pub struct Slot0Call;
    ///Container type for all input parameters for the `snapshotCumulativesInside` function with signature `snapshotCumulativesInside(int24,int24)` and selector `0xa38807f2`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "snapshotCumulativesInside",
        abi = "snapshotCumulativesInside(int24,int24)"
    )]
    pub struct SnapshotCumulativesInsideCall {
        pub tick_lower: i32,
        pub tick_upper: i32,
    }
    ///Container type for all input parameters for the `swap` function with signature `swap(address,bool,int256,uint160,bytes)` and selector `0x128acb08`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "swap", abi = "swap(address,bool,int256,uint160,bytes)")]
    pub struct SwapCall {
        pub recipient: ::ethers::core::types::Address,
        pub zero_for_one: bool,
        pub amount_specified: ::ethers::core::types::I256,
        pub sqrt_price_limit_x96: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `tickBitmap` function with signature `tickBitmap(int16)` and selector `0x5339c296`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "tickBitmap", abi = "tickBitmap(int16)")]
    pub struct TickBitmapCall(pub i16);
    ///Container type for all input parameters for the `tickSpacing` function with signature `tickSpacing()` and selector `0xd0c93a7c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "tickSpacing", abi = "tickSpacing()")]
    pub struct TickSpacingCall;
    ///Container type for all input parameters for the `ticks` function with signature `ticks(int24)` and selector `0xf30dba93`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "ticks", abi = "ticks(int24)")]
    pub struct TicksCall(pub i32);
    ///Container type for all input parameters for the `token0` function with signature `token0()` and selector `0x0dfe1681`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "token0", abi = "token0()")]
    pub struct Token0Call;
    ///Container type for all input parameters for the `token1` function with signature `token1()` and selector `0xd21220a7`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "token1", abi = "token1()")]
    pub struct Token1Call;
    ///Container type for all of the contract's call
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub enum UniswapV3PoolCalls {
        Burn(BurnCall),
        Collect(CollectCall),
        CollectProtocol(CollectProtocolCall),
        Factory(FactoryCall),
        Fee(FeeCall),
        FeeGrowthGlobal0X128(FeeGrowthGlobal0X128Call),
        FeeGrowthGlobal1X128(FeeGrowthGlobal1X128Call),
        Flash(FlashCall),
        IncreaseObservationCardinalityNext(IncreaseObservationCardinalityNextCall),
        Initialize(InitializeCall),
        Liquidity(LiquidityCall),
        MaxLiquidityPerTick(MaxLiquidityPerTickCall),
        Mint(MintCall),
        Observations(ObservationsCall),
        Observe(ObserveCall),
        Positions(PositionsCall),
        ProtocolFees(ProtocolFeesCall),
        SetFeeProtocol(SetFeeProtocolCall),
        Slot0(Slot0Call),
        SnapshotCumulativesInside(SnapshotCumulativesInsideCall),
        Swap(SwapCall),
        TickBitmap(TickBitmapCall),
        TickSpacing(TickSpacingCall),
        Ticks(TicksCall),
        Token0(Token0Call),
        Token1(Token1Call),
    }
    impl ::ethers::core::abi::AbiDecode for UniswapV3PoolCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <BurnCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Burn(decoded));
            }
            if let Ok(decoded) = <CollectCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Collect(decoded));
            }
            if let Ok(decoded) = <CollectProtocolCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CollectProtocol(decoded));
            }
            if let Ok(decoded) = <FactoryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Factory(decoded));
            }
            if let Ok(decoded) = <FeeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Fee(decoded));
            }
            if let Ok(decoded) = <FeeGrowthGlobal0X128Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FeeGrowthGlobal0X128(decoded));
            }
            if let Ok(decoded) = <FeeGrowthGlobal1X128Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FeeGrowthGlobal1X128(decoded));
            }
            if let Ok(decoded) = <FlashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Flash(decoded));
            }
            if let Ok(decoded) = <IncreaseObservationCardinalityNextCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IncreaseObservationCardinalityNext(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) = <LiquidityCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Liquidity(decoded));
            }
            if let Ok(decoded) = <MaxLiquidityPerTickCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MaxLiquidityPerTick(decoded));
            }
            if let Ok(decoded) = <MintCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Mint(decoded));
            }
            if let Ok(decoded) = <ObservationsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Observations(decoded));
            }
            if let Ok(decoded) = <ObserveCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Observe(decoded));
            }
            if let Ok(decoded) = <PositionsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Positions(decoded));
            }
            if let Ok(decoded) = <ProtocolFeesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ProtocolFees(decoded));
            }
            if let Ok(decoded) = <SetFeeProtocolCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetFeeProtocol(decoded));
            }
            if let Ok(decoded) = <Slot0Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Slot0(decoded));
            }
            if let Ok(decoded) = <SnapshotCumulativesInsideCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SnapshotCumulativesInside(decoded));
            }
            if let Ok(decoded) = <SwapCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Swap(decoded));
            }
            if let Ok(decoded) = <TickBitmapCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TickBitmap(decoded));
            }
            if let Ok(decoded) = <TickSpacingCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TickSpacing(decoded));
            }
            if let Ok(decoded) = <TicksCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Ticks(decoded));
            }
            if let Ok(decoded) = <Token0Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Token0(decoded));
            }
            if let Ok(decoded) = <Token1Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Token1(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for UniswapV3PoolCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Burn(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Collect(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CollectProtocol(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Factory(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Fee(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::FeeGrowthGlobal0X128(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FeeGrowthGlobal1X128(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Flash(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IncreaseObservationCardinalityNext(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Liquidity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MaxLiquidityPerTick(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Mint(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Observations(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Observe(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Positions(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProtocolFees(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetFeeProtocol(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Slot0(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SnapshotCumulativesInside(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Swap(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TickBitmap(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TickSpacing(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Ticks(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Token0(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Token1(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for UniswapV3PoolCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Burn(element) => ::core::fmt::Display::fmt(element, f),
                Self::Collect(element) => ::core::fmt::Display::fmt(element, f),
                Self::CollectProtocol(element) => ::core::fmt::Display::fmt(element, f),
                Self::Factory(element) => ::core::fmt::Display::fmt(element, f),
                Self::Fee(element) => ::core::fmt::Display::fmt(element, f),
                Self::FeeGrowthGlobal0X128(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FeeGrowthGlobal1X128(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Flash(element) => ::core::fmt::Display::fmt(element, f),
                Self::IncreaseObservationCardinalityNext(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::Liquidity(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxLiquidityPerTick(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Mint(element) => ::core::fmt::Display::fmt(element, f),
                Self::Observations(element) => ::core::fmt::Display::fmt(element, f),
                Self::Observe(element) => ::core::fmt::Display::fmt(element, f),
                Self::Positions(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProtocolFees(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetFeeProtocol(element) => ::core::fmt::Display::fmt(element, f),
                Self::Slot0(element) => ::core::fmt::Display::fmt(element, f),
                Self::SnapshotCumulativesInside(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Swap(element) => ::core::fmt::Display::fmt(element, f),
                Self::TickBitmap(element) => ::core::fmt::Display::fmt(element, f),
                Self::TickSpacing(element) => ::core::fmt::Display::fmt(element, f),
                Self::Ticks(element) => ::core::fmt::Display::fmt(element, f),
                Self::Token0(element) => ::core::fmt::Display::fmt(element, f),
                Self::Token1(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<BurnCall> for UniswapV3PoolCalls {
        fn from(value: BurnCall) -> Self {
            Self::Burn(value)
        }
    }
    impl ::core::convert::From<CollectCall> for UniswapV3PoolCalls {
        fn from(value: CollectCall) -> Self {
            Self::Collect(value)
        }
    }
    impl ::core::convert::From<CollectProtocolCall> for UniswapV3PoolCalls {
        fn from(value: CollectProtocolCall) -> Self {
            Self::CollectProtocol(value)
        }
    }
    impl ::core::convert::From<FactoryCall> for UniswapV3PoolCalls {
        fn from(value: FactoryCall) -> Self {
            Self::Factory(value)
        }
    }
    impl ::core::convert::From<FeeCall> for UniswapV3PoolCalls {
        fn from(value: FeeCall) -> Self {
            Self::Fee(value)
        }
    }
    impl ::core::convert::From<FeeGrowthGlobal0X128Call> for UniswapV3PoolCalls {
        fn from(value: FeeGrowthGlobal0X128Call) -> Self {
            Self::FeeGrowthGlobal0X128(value)
        }
    }
    impl ::core::convert::From<FeeGrowthGlobal1X128Call> for UniswapV3PoolCalls {
        fn from(value: FeeGrowthGlobal1X128Call) -> Self {
            Self::FeeGrowthGlobal1X128(value)
        }
    }
    impl ::core::convert::From<FlashCall> for UniswapV3PoolCalls {
        fn from(value: FlashCall) -> Self {
            Self::Flash(value)
        }
    }
    impl ::core::convert::From<IncreaseObservationCardinalityNextCall>
    for UniswapV3PoolCalls {
        fn from(value: IncreaseObservationCardinalityNextCall) -> Self {
            Self::IncreaseObservationCardinalityNext(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for UniswapV3PoolCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<LiquidityCall> for UniswapV3PoolCalls {
        fn from(value: LiquidityCall) -> Self {
            Self::Liquidity(value)
        }
    }
    impl ::core::convert::From<MaxLiquidityPerTickCall> for UniswapV3PoolCalls {
        fn from(value: MaxLiquidityPerTickCall) -> Self {
            Self::MaxLiquidityPerTick(value)
        }
    }
    impl ::core::convert::From<MintCall> for UniswapV3PoolCalls {
        fn from(value: MintCall) -> Self {
            Self::Mint(value)
        }
    }
    impl ::core::convert::From<ObservationsCall> for UniswapV3PoolCalls {
        fn from(value: ObservationsCall) -> Self {
            Self::Observations(value)
        }
    }
    impl ::core::convert::From<ObserveCall> for UniswapV3PoolCalls {
        fn from(value: ObserveCall) -> Self {
            Self::Observe(value)
        }
    }
    impl ::core::convert::From<PositionsCall> for UniswapV3PoolCalls {
        fn from(value: PositionsCall) -> Self {
            Self::Positions(value)
        }
    }
    impl ::core::convert::From<ProtocolFeesCall> for UniswapV3PoolCalls {
        fn from(value: ProtocolFeesCall) -> Self {
            Self::ProtocolFees(value)
        }
    }
    impl ::core::convert::From<SetFeeProtocolCall> for UniswapV3PoolCalls {
        fn from(value: SetFeeProtocolCall) -> Self {
            Self::SetFeeProtocol(value)
        }
    }
    impl ::core::convert::From<Slot0Call> for UniswapV3PoolCalls {
        fn from(value: Slot0Call) -> Self {
            Self::Slot0(value)
        }
    }
    impl ::core::convert::From<SnapshotCumulativesInsideCall> for UniswapV3PoolCalls {
        fn from(value: SnapshotCumulativesInsideCall) -> Self {
            Self::SnapshotCumulativesInside(value)
        }
    }
    impl ::core::convert::From<SwapCall> for UniswapV3PoolCalls {
        fn from(value: SwapCall) -> Self {
            Self::Swap(value)
        }
    }
    impl ::core::convert::From<TickBitmapCall> for UniswapV3PoolCalls {
        fn from(value: TickBitmapCall) -> Self {
            Self::TickBitmap(value)
        }
    }
    impl ::core::convert::From<TickSpacingCall> for UniswapV3PoolCalls {
        fn from(value: TickSpacingCall) -> Self {
            Self::TickSpacing(value)
        }
    }
    impl ::core::convert::From<TicksCall> for UniswapV3PoolCalls {
        fn from(value: TicksCall) -> Self {
            Self::Ticks(value)
        }
    }
    impl ::core::convert::From<Token0Call> for UniswapV3PoolCalls {
        fn from(value: Token0Call) -> Self {
            Self::Token0(value)
        }
    }
    impl ::core::convert::From<Token1Call> for UniswapV3PoolCalls {
        fn from(value: Token1Call) -> Self {
            Self::Token1(value)
        }
    }
    ///Container type for all return fields from the `burn` function with signature `burn(int24,int24,uint128)` and selector `0xa34123a7`
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
        Hash
    )]
    pub struct BurnReturn {
        pub amount_0: ::ethers::core::types::U256,
        pub amount_1: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `collect` function with signature `collect(address,int24,int24,uint128,uint128)` and selector `0x4f1eb3d8`
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
        Hash
    )]
    pub struct CollectReturn {
        pub amount_0: u128,
        pub amount_1: u128,
    }
    ///Container type for all return fields from the `collectProtocol` function with signature `collectProtocol(address,uint128,uint128)` and selector `0x85b66729`
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
        Hash
    )]
    pub struct CollectProtocolReturn {
        pub amount_0: u128,
        pub amount_1: u128,
    }
    ///Container type for all return fields from the `factory` function with signature `factory()` and selector `0xc45a0155`
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
        Hash
    )]
    pub struct FactoryReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `fee` function with signature `fee()` and selector `0xddca3f43`
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
        Hash
    )]
    pub struct FeeReturn(pub u32);
    ///Container type for all return fields from the `feeGrowthGlobal0X128` function with signature `feeGrowthGlobal0X128()` and selector `0xf3058399`
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
        Hash
    )]
    pub struct FeeGrowthGlobal0X128Return(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `feeGrowthGlobal1X128` function with signature `feeGrowthGlobal1X128()` and selector `0x46141319`
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
        Hash
    )]
    pub struct FeeGrowthGlobal1X128Return(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `liquidity` function with signature `liquidity()` and selector `0x1a686502`
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
        Hash
    )]
    pub struct LiquidityReturn(pub u128);
    ///Container type for all return fields from the `maxLiquidityPerTick` function with signature `maxLiquidityPerTick()` and selector `0x70cf754a`
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
        Hash
    )]
    pub struct MaxLiquidityPerTickReturn(pub u128);
    ///Container type for all return fields from the `mint` function with signature `mint(address,int24,int24,uint128,bytes)` and selector `0x3c8a7d8d`
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
        Hash
    )]
    pub struct MintReturn {
        pub amount_0: ::ethers::core::types::U256,
        pub amount_1: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `observations` function with signature `observations(uint256)` and selector `0x252c09d7`
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
        Hash
    )]
    pub struct ObservationsReturn {
        pub block_timestamp: u32,
        pub tick_cumulative: i64,
        pub seconds_per_liquidity_cumulative_x128: ::ethers::core::types::U256,
        pub initialized: bool,
    }
    ///Container type for all return fields from the `observe` function with signature `observe(uint32[])` and selector `0x883bdbfd`
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
        Hash
    )]
    pub struct ObserveReturn {
        pub tick_cumulatives: ::std::vec::Vec<i64>,
        pub seconds_per_liquidity_cumulative_x12_8s: ::std::vec::Vec<
            ::ethers::core::types::U256,
        >,
    }
    ///Container type for all return fields from the `positions` function with signature `positions(bytes32)` and selector `0x514ea4bf`
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
        Hash
    )]
    pub struct PositionsReturn {
        pub liquidity: u128,
        pub fee_growth_inside_0_last_x128: ::ethers::core::types::U256,
        pub fee_growth_inside_1_last_x128: ::ethers::core::types::U256,
        pub tokens_owed_0: u128,
        pub tokens_owed_1: u128,
    }
    ///Container type for all return fields from the `protocolFees` function with signature `protocolFees()` and selector `0x1ad8b03b`
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
        Hash
    )]
    pub struct ProtocolFeesReturn {
        pub token_0: u128,
        pub token_1: u128,
    }
    ///Container type for all return fields from the `slot0` function with signature `slot0()` and selector `0x3850c7bd`
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
        Hash
    )]
    pub struct Slot0Return {
        pub sqrt_price_x96: ::ethers::core::types::U256,
        pub tick: i32,
        pub observation_index: u16,
        pub observation_cardinality: u16,
        pub observation_cardinality_next: u16,
        pub fee_protocol: u8,
        pub unlocked: bool,
    }
    ///Container type for all return fields from the `snapshotCumulativesInside` function with signature `snapshotCumulativesInside(int24,int24)` and selector `0xa38807f2`
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
        Hash
    )]
    pub struct SnapshotCumulativesInsideReturn {
        pub tick_cumulative_inside: i64,
        pub seconds_per_liquidity_inside_x128: ::ethers::core::types::U256,
        pub seconds_inside: u32,
    }
    ///Container type for all return fields from the `swap` function with signature `swap(address,bool,int256,uint160,bytes)` and selector `0x128acb08`
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
        Hash
    )]
    pub struct SwapReturn {
        pub amount_0: ::ethers::core::types::I256,
        pub amount_1: ::ethers::core::types::I256,
    }
    ///Container type for all return fields from the `tickBitmap` function with signature `tickBitmap(int16)` and selector `0x5339c296`
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
        Hash
    )]
    pub struct TickBitmapReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `tickSpacing` function with signature `tickSpacing()` and selector `0xd0c93a7c`
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
        Hash
    )]
    pub struct TickSpacingReturn(pub i32);
    ///Container type for all return fields from the `ticks` function with signature `ticks(int24)` and selector `0xf30dba93`
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
        Hash
    )]
    pub struct TicksReturn {
        pub liquidity_gross: u128,
        pub liquidity_net: i128,
        pub fee_growth_outside_0x128: ::ethers::core::types::U256,
        pub fee_growth_outside_1x128: ::ethers::core::types::U256,
        pub tick_cumulative_outside: i64,
        pub seconds_per_liquidity_outside_x128: ::ethers::core::types::U256,
        pub seconds_outside: u32,
        pub initialized: bool,
    }
    ///Container type for all return fields from the `token0` function with signature `token0()` and selector `0x0dfe1681`
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
        Hash
    )]
    pub struct Token0Return(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `token1` function with signature `token1()` and selector `0xd21220a7`
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
        Hash
    )]
    pub struct Token1Return(pub ::ethers::core::types::Address);
}
