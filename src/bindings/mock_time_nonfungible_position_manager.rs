pub use mock_time_nonfungible_position_manager::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types
)]
pub mod mock_time_nonfungible_position_manager {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_factory"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_WETH9"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_tokenDescriptor"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("DOMAIN_SEPARATOR"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("DOMAIN_SEPARATOR"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PERMIT_TYPEHASH"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("PERMIT_TYPEHASH"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("WETH9"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("WETH9"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("approve"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("approve"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("to"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("tokenId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("balanceOf"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("balanceOf"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("owner"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("baseURI"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("baseURI"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::String,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("string"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("burn"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("burn"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("tokenId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("collect"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("collect"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("params"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct INonfungiblePositionManager.CollectParams",
                                ),
                            ),
                        },],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amount0"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amount1"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("createAndInitializePoolIfNecessary"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned(
                            "createAndInitializePoolIfNecessary",
                        ),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("token0"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("token1"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("fee"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint24"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("sqrtPriceX96"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(160usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint160"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("pool"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("decreaseLiquidity"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("decreaseLiquidity"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("params"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct INonfungiblePositionManager.DecreaseLiquidityParams",
                                ),
                            ),
                        },],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amount0"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amount1"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("factory"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("factory"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getApproved"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getApproved"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("tokenId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("increaseLiquidity"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("increaseLiquidity"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("params"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct INonfungiblePositionManager.IncreaseLiquidityParams",
                                ),
                            ),
                        },],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("liquidity"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(128usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint128"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amount0"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amount1"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isApprovedForAll"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("isApprovedForAll"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("owner"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("mint"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("mint"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("params"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(24usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(24usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct INonfungiblePositionManager.MintParams",
                                ),
                            ),
                        },],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("tokenId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("liquidity"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(128usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint128"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amount0"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amount1"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("multicall"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("multicall"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("data"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes[]"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("results"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes[]"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("name"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("name"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::String,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("string"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ownerOf"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("ownerOf"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("tokenId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("permit"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("permit"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("spender"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("tokenId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("deadline"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("v"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint8"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("r"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("s"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("positions"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("positions"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("tokenId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("nonce"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint96"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("token0"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("token1"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("fee"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint24"),
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
                                name: ::std::borrow::ToOwned::to_owned("liquidity"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(128usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint128"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("feeGrowthInside0LastX128",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("feeGrowthInside1LastX128",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("tokensOwed0"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(128usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint128"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("tokensOwed1"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(128usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint128"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("refundETH"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("refundETH"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("safeTransferFrom"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("safeTransferFrom"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability:
                                ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("safeTransferFrom"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability:
                                ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("selfPermit"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("selfPermit"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("token"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("value"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("deadline"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("v"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint8"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("r"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("s"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("selfPermitAllowed"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("selfPermitAllowed"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("token"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("nonce"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("expiry"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("v"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint8"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("r"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("s"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("selfPermitAllowedIfNecessary"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("selfPermitAllowedIfNecessary",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("token"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("nonce"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("expiry"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("v"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint8"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("r"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("s"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("selfPermitIfNecessary"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("selfPermitIfNecessary",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("token"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("value"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("deadline"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("v"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint8"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("r"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("s"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setApprovalForAll"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setApprovalForAll"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("approved"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setTime"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setTime"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_time"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("supportsInterface"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("supportsInterface"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("interfaceId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(4usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes4"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("sweepToken"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("sweepToken"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("token"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amountMinimum"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("recipient"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("symbol"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("symbol"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::String,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("string"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("tokenByIndex"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("tokenByIndex"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("index"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("tokenOfOwnerByIndex"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("tokenOfOwnerByIndex",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("owner"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("index"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("tokenURI"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("tokenURI"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("tokenId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::String,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("string"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("totalSupply"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("totalSupply"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transferFrom"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("transferFrom"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("from"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("to"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("tokenId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("uniswapV3MintCallback"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("uniswapV3MintCallback",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amount0Owed"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amount1Owed"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
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
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("unwrapWETH9"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("unwrapWETH9"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amountMinimum"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("recipient"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                    },],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Approval"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Approval"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("owner"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("approved"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("tokenId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: true,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ApprovalForAll"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("ApprovalForAll"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("owner"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("approved"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Collect"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Collect"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("tokenId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("recipient"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("amount0"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("amount1"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DecreaseLiquidity"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("DecreaseLiquidity"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("tokenId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("liquidity"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(128usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("amount0"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("amount1"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("IncreaseLiquidity"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("IncreaseLiquidity"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("tokenId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("liquidity"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(128usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("amount0"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("amount1"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Transfer"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Transfer"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("from"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("to"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("tokenId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: true,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: true,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static MOCKTIMENONFUNGIBLEPOSITIONMANAGER_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x01 `@R`\r\x80T`\x01`\x01`\xB0\x1B\x03\x19\x16`\x01\x17`\x01`\x01`\xB0\x1B\x03\x16`\x01`\xB0\x1B\x17\x90U4\x80\x15b\0\x004W`\0\x80\xFD[P`@Qb\0]\x968\x03\x80b\0]\x96\x839\x81\x01`@\x81\x90Rb\0\0W\x91b\0\x02\xE1V[\x82\x82\x82\x82\x82`@Q\x80`@\x01`@R\x80`\x1B\x81R` \x01\x7FUniswap V3 Positions NFT-V1\0\0\0\0\0\x81RP`@Q\x80`@\x01`@R\x80`\n\x81R` \x01iUNI-V3-POS`\xB0\x1B\x81RP`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01`1`\xF8\x1B\x81RP\x82\x82b\0\0\xE9c\x01\xFF\xC9\xA7`\xE0\x1Bb\0\x01\x93` \x1B` \x1CV[\x81Qb\0\0\xFE\x90`\x06\x90` \x85\x01\x90b\0\x02\x18V[P\x80Qb\0\x01\x14\x90`\x07\x90` \x84\x01\x90b\0\x02\x18V[Pb\0\x01'c\x80\xACX\xCD`\xE0\x1Bb\0\x01\x93V[b\0\x019c[^\x13\x9F`\xE0\x1Bb\0\x01\x93V[b\0\x01Kcx\x0E\x9Dc`\xE0\x1Bb\0\x01\x93V[PP\x82Q` \x93\x84\x01 `\x80R\x80Q\x92\x01\x91\x90\x91 `\xA0RP`\x01`\x01``\x1B\x03\x19``\x92\x83\x1B\x81\x16`\xC0R\x90\x82\x1B\x81\x16`\xE0R\x91\x90\x1B\x16a\x01\0RPb\0\x03*\x93PPPPV[`\x01`\x01`\xE0\x1B\x03\x19\x80\x82\x16\x14\x15b\0\x01\xF3W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FERC165: invalid interface id\0\0\0\0`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\x01`\x01`\xE0\x1B\x03\x19\x16`\0\x90\x81R` \x81\x90R`@\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UV[\x82\x80T`\x01\x81`\x01\x16\x15a\x01\0\x02\x03\x16`\x02\x90\x04\x90`\0R` `\0 \x90`\x1F\x01` \x90\x04\x81\x01\x92\x82b\0\x02PW`\0\x85Ub\0\x02\x9BV[\x82`\x1F\x10b\0\x02kW\x80Q`\xFF\x19\x16\x83\x80\x01\x17\x85Ub\0\x02\x9BV[\x82\x80\x01`\x01\x01\x85U\x82\x15b\0\x02\x9BW\x91\x82\x01[\x82\x81\x11\x15b\0\x02\x9BW\x82Q\x82U\x91` \x01\x91\x90`\x01\x01\x90b\0\x02~V[Pb\0\x02\xA9\x92\x91Pb\0\x02\xADV[P\x90V[[\x80\x82\x11\x15b\0\x02\xA9W`\0\x81U`\x01\x01b\0\x02\xAEV[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x02\xDCW`\0\x80\xFD[\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15b\0\x02\xF6W\x82\x83\xFD[b\0\x03\x01\x84b\0\x02\xC4V[\x92Pb\0\x03\x11` \x85\x01b\0\x02\xC4V[\x91Pb\0\x03!`@\x85\x01b\0\x02\xC4V[\x90P\x92P\x92P\x92V[`\x80Q`\xA0Q`\xC0Q``\x1C`\xE0Q``\x1Ca\x01\0Q``\x1CaY\xE7b\0\x03\xAF`\09\x80a(\x0FRP\x80a\x02DR\x80a\x169R\x80a\x17$R\x80a\x17\xACR\x80a:\xB2R\x80a:\xF8R\x80a;lRP\x80a\n0R\x80a\rPR\x80a\x0E\x17R\x80a'\xC2R\x80a(\xAFR\x80a+\xA9R\x80a3\xAERP\x80a\x144RP\x80a\x14\x13RPaY\xE7`\0\xF3\xFE`\x80`@R`\x046\x10a\x024W`\x005`\xE0\x1C\x80cOl\xCC\xE7\x11a\x01.W\x80c\xAC\x96P\xD8\x11a\0\xABW\x80c\xD3Hy\x97\x11a\0oW\x80c\xD3Hy\x97\x14a\x06yW\x80c\xDF*\xB5\xBB\x14a\x06\x99W\x80c\xE9\x85\xE9\xC5\x14a\x06\xACW\x80c\xF3\x99\\g\x14a\x06\xCCW\x80c\xFCoxe\x14a\x06\xDFWa\x02\xA4V[\x80c\xAC\x96P\xD8\x14a\x05\xF1W\x80c\xB8\x8DO\xDE\x14a\x06\x11W\x80c\xC2\xE3\x14\n\x14a\x061W\x80c\xC4Z\x01U\x14a\x06DW\x80c\xC8{V\xDD\x14a\x06YWa\x02\xA4V[\x80c\x881dV\x11a\0\xF2W\x80c\x881dV\x14a\x05NW\x80c\x95\xD8\x9BA\x14a\x05qW\x80c\x99\xFB\xAB\x88\x14a\x05\x86W\x80c\xA2,\xB4e\x14a\x05\xBEW\x80c\xA4\xA7\x8F\x0C\x14a\x05\xDEWa\x02\xA4V[\x80cOl\xCC\xE7\x14a\x04\xC6W\x80ccR!\x1E\x14a\x04\xE6W\x80cl\x03`\xEB\x14a\x05\x06W\x80cp\xA0\x821\x14a\x05\x1BW\x80cz\xC2\xFF{\x14a\x05;Wa\x02\xA4V[\x80c#\xB8r\xDD\x11a\x01\xBCW\x80cB\x84.\x0E\x11a\x01\x80W\x80cB\x84.\x0E\x14a\x04XW\x80cB\x96lh\x14a\x04xW\x80cFY\xA4\x94\x14a\x04\x8BW\x80cI@K|\x14a\x04\x9EW\x80cJ\xA4\xA4\xFC\x14a\x04\xB1Wa\x02\xA4V[\x80c#\xB8r\xDD\x14a\x03\xCEW\x80c/t\\Y\x14a\x03\xEEW\x80c0\xAD\xF8\x1F\x14a\x04\x0EW\x80c6D\xE5\x15\x14a\x04#W\x80c;\xEB&\xC4\x14a\x048Wa\x02\xA4V[\x80c\x0CI\xCC\xBE\x11a\x02\x03W\x80c\x0CI\xCC\xBE\x14a\x03NW\x80c\x12!\x0E\x8A\x14a\x03oW\x80c\x13\xEA\xD5b\x14a\x03wW\x80c\x18\x16\r\xDD\x14a\x03\x8AW\x80c!\x9F]\x17\x14a\x03\xACWa\x02\xA4V[\x80c\x01\xFF\xC9\xA7\x14a\x02\xA9W\x80c\x06\xFD\xDE\x03\x14a\x02\xDFW\x80c\x08\x18\x12\xFC\x14a\x03\x01W\x80c\t^\xA7\xB3\x14a\x03.Wa\x02\xA4V[6a\x02\xA4W3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x02\xA2W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01RhNot WETH9`\xB8\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\0[`\0\x80\xFD[4\x80\x15a\x02\xB5W`\0\x80\xFD[Pa\x02\xC9a\x02\xC46`\x04aN\xC8V[a\x06\xF2V[`@Qa\x02\xD6\x91\x90aT\tV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\xEBW`\0\x80\xFD[Pa\x02\xF4a\x07\x15V[`@Qa\x02\xD6\x91\x90aT\\V[4\x80\x15a\x03\rW`\0\x80\xFD[Pa\x03!a\x03\x1C6`\x04aQ\xC2V[a\x07\xABV[`@Qa\x02\xD6\x91\x90aR\xECV[4\x80\x15a\x03:W`\0\x80\xFD[Pa\x02\xA2a\x03I6`\x04aM\x92V[a\x07\xFEV[a\x03aa\x03\\6`\x04aO\x8DV[a\x08\xD4V[`@Qa\x02\xD6\x92\x91\x90aU\xE3V[a\x02\xA2a\r\x1CV[a\x03!a\x03\x856`\x04aL%V[a\r.V[4\x80\x15a\x03\x96W`\0\x80\xFD[Pa\x03\x9Fa\x10\"V[`@Qa\x02\xD6\x91\x90aT\x14V[a\x03\xBFa\x03\xBA6`\x04aO\x9EV[a\x103V[`@Qa\x02\xD6\x93\x92\x91\x90aU\x9EV[4\x80\x15a\x03\xDAW`\0\x80\xFD[Pa\x02\xA2a\x03\xE96`\x04aL~V[a\x13HV[4\x80\x15a\x03\xFAW`\0\x80\xFD[Pa\x03\x9Fa\x04\t6`\x04aM\x92V[a\x13\x9FV[4\x80\x15a\x04\x1AW`\0\x80\xFD[Pa\x03\x9Fa\x13\xCAV[4\x80\x15a\x04/W`\0\x80\xFD[Pa\x03\x9Fa\x13\xEEV[4\x80\x15a\x04DW`\0\x80\xFD[Pa\x02\xA2a\x04S6`\x04aQ\xC2V[a\x14\xACV[4\x80\x15a\x04dW`\0\x80\xFD[Pa\x02\xA2a\x04s6`\x04aL~V[a\x14\xB1V[a\x02\xA2a\x04\x866`\x04aQ\xC2V[a\x14\xCCV[a\x02\xA2a\x04\x996`\x04aM\xFEV[a\x15\x9BV[a\x02\xA2a\x04\xAC6`\x04aQ\xDAV[a\x165V[4\x80\x15a\x04\xBDW`\0\x80\xFD[Pa\x03!a\x17\xAAV[4\x80\x15a\x04\xD2W`\0\x80\xFD[Pa\x03\x9Fa\x04\xE16`\x04aQ\xC2V[a\x17\xCEV[4\x80\x15a\x04\xF2W`\0\x80\xFD[Pa\x03!a\x05\x016`\x04aQ\xC2V[a\x17\xE4V[4\x80\x15a\x05\x12W`\0\x80\xFD[Pa\x02\xF4a\x18\x0CV[4\x80\x15a\x05'W`\0\x80\xFD[Pa\x03\x9Fa\x0566`\x04aK\xD1V[a\x18\x11V[a\x02\xA2a\x05I6`\x04aM\xFEV[a\x18yV[a\x05aa\x05\\6`\x04aPZV[a\x1CkV[`@Qa\x02\xD6\x94\x93\x92\x91\x90aU\xBFV[4\x80\x15a\x05}W`\0\x80\xFD[Pa\x02\xF4a!\x83V[4\x80\x15a\x05\x92W`\0\x80\xFD[Pa\x05\xA6a\x05\xA16`\x04aQ\xC2V[a!\xE4V[`@Qa\x02\xD6\x9C\x9B\x9A\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90aU\xF1V[4\x80\x15a\x05\xCAW`\0\x80\xFD[Pa\x02\xA2a\x05\xD96`\x04aMeV[a#\xF5V[a\x02\xA2a\x05\xEC6`\x04aM\xFEV[a$\xFAV[a\x06\x04a\x05\xFF6`\x04aNYV[a%\x93V[`@Qa\x02\xD6\x91\x90aS\xA9V[4\x80\x15a\x06\x1DW`\0\x80\xFD[Pa\x02\xA2a\x06,6`\x04aL\xBEV[a&\xD3V[a\x02\xA2a\x06?6`\x04aM\xFEV[a'1V[4\x80\x15a\x06PW`\0\x80\xFD[Pa\x03!a'\xC0V[4\x80\x15a\x06eW`\0\x80\xFD[Pa\x02\xF4a\x06t6`\x04aQ\xC2V[a'\xE4V[4\x80\x15a\x06\x85W`\0\x80\xFD[Pa\x02\xA2a\x06\x946`\x04aR!V[a(\x9AV[a\x02\xA2a\x06\xA76`\x04aM\xBDV[a)\x18V[4\x80\x15a\x06\xB8W`\0\x80\xFD[Pa\x02\xC9a\x06\xC76`\x04aK\xEDV[a)\xF0V[a\x02\xA2a\x06\xDA6`\x04aM\xFEV[a*\x1EV[a\x03aa\x06\xED6`\x04aOvV[a*\x90V[`\x01`\x01`\xE0\x1B\x03\x19\x81\x16`\0\x90\x81R` \x81\x90R`@\x90 T`\xFF\x16[\x91\x90PV[`\x06\x80T`@\x80Q` `\x1F`\x02`\0\x19a\x01\0`\x01\x88\x16\x15\x02\x01\x90\x95\x16\x94\x90\x94\x04\x93\x84\x01\x81\x90\x04\x81\x02\x82\x01\x81\x01\x90\x92R\x82\x81R``\x93\x90\x92\x90\x91\x83\x01\x82\x82\x80\x15a\x07\xA1W\x80`\x1F\x10a\x07vWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x07\xA1V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x07\x84W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[`\0a\x07\xB6\x82a/[V[a\x07\xDBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xD2\x90aT\x95V[`@Q\x80\x91\x03\x90\xFD[P`\0\x90\x81R`\x0C` R`@\x90 T`\x01``\x1B\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\0a\x08\t\x82a\x17\xE4V[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a\x08\\W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`!\x81R` \x01\x80aY``!\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[\x80`\x01`\x01`\xA0\x1B\x03\x16a\x08na/hV[`\x01`\x01`\xA0\x1B\x03\x16\x14\x80a\x08\x8AWPa\x08\x8A\x81a\x06\xC7a/hV[a\x08\xC5W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`8\x81R` \x01\x80aX\x8A`8\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[a\x08\xCF\x83\x83a/lV[PPPV[`\0\x80\x825a\x08\xE33\x82a/\xE2V[a\x08\xFFW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xD2\x90aToV[\x83`\x80\x015\x80a\t\ra0~V[\x11\x15a\tVW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr\x15\x1C\x98[\x9C\xD8X\xDD\x1A[\xDB\x88\x1D\x1B\xDB\xC8\x1B\xDB\x19`j\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\0a\th`@\x87\x01` \x88\x01aPlV[`\x01`\x01`\x80\x1B\x03\x16\x11a\t{W`\0\x80\xFD[\x845`\0\x90\x81R`\x0C` \x90\x81R`@\x91\x82\x90 `\x01\x81\x01T\x90\x92`\x01`\x80\x1B\x90\x91\x04`\x01`\x01`\x80\x1B\x03\x16\x91a\t\xB6\x91\x89\x01\x90\x89\x01aPlV[`\x01`\x01`\x80\x1B\x03\x16\x81`\x01`\x01`\x80\x1B\x03\x16\x10\x15a\t\xD4W`\0\x80\xFD[`\x01\x82\x81\x01T`\x01`\x01`P\x1B\x03\x16`\0\x90\x81R`\x0B` \x90\x81R`@\x80\x83 \x81Q``\x81\x01\x83R\x81T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R\x91\x90\x95\x01T\x90\x81\x16\x92\x85\x01\x92\x90\x92R`\x01`\xA0\x1B\x90\x91\x04b\xFF\xFF\xFF\x16\x90\x83\x01Ra\nU\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83a0\x84V[`\x01\x85\x01T\x90\x91P`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\xA3A#\xA7\x90`\x01`P\x1B\x81\x04`\x02\x90\x81\x0B\x91`\x01`h\x1B\x90\x04\x90\x0Ba\n\x94`@\x8E\x01` \x8F\x01aPlV[`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\n\xB2\x93\x92\x91\x90aT6V[`@\x80Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\n\xCBW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\n\xDFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\x03\x91\x90aQ\xFEV[\x90\x98P\x96P`@\x89\x015\x88\x10\x80\x15\x90a\x0B WP\x88``\x015\x87\x10\x15[a\x0B<W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xD2\x90aT\xE1V[`\x01\x84\x01T`\0\x90a\x0Be\x900\x90`\x01`P\x1B\x81\x04`\x02\x90\x81\x0B\x91`\x01`h\x1B\x90\x04\x90\x0Ba1cV[\x90P`\0\x80\x83`\x01`\x01`\xA0\x1B\x03\x16cQN\xA4\xBF\x84`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0B\x96\x91\x90aT\x14V[`\xA0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0B\xAEW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x0B\xC2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xE6\x91\x90aP\xB6V[PP\x92P\x92PPa\x0C\x0B\x87`\x02\x01T\x83\x03\x87`\x01`\x01`\x80\x1B\x03\x16`\x01`\x80\x1Ba1\xB8V[`\x04\x88\x01\x80T`\x01`\x01`\x80\x1B\x03\x19\x81\x16\x92\x8E\x01`\x01`\x01`\x80\x1B\x03\x91\x82\x16\x01\x81\x16\x92\x90\x92\x17\x90U`\x03\x88\x01Ta\x0CL\x91\x90\x83\x03\x90\x88\x16`\x01`\x80\x1Ba1\xB8V[`\x04\x88\x01\x80T`\x01`\x01`\x80\x1B\x03\x80\x82\x16\x93\x8E\x01`\x01`\x80\x1B\x92\x83\x90\x04\x82\x16\x01\x16\x02\x91\x90\x91\x17\x90U`\x02\x87\x01\x82\x90U`\x03\x87\x01\x81\x90Ua\x0C\x92`@\x8D\x01` \x8E\x01aPlV[\x86\x03\x87`\x01\x01`\x10a\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP\x8B`\0\x015\x7F&\xF6\xA0H\xEE\x918\xF2\xC0\xCE&o2,\xB9\x92(\xE8\xD6\x19\xAE+\xFF0\xC6\x7F\x8D\xCF\x9D#w\xB4\x8D` \x01` \x81\x01\x90a\x0C\xF5\x91\x90aPlV[\x8D\x8D`@Qa\r\x06\x93\x92\x91\x90aU\x9EV[`@Q\x80\x91\x03\x90\xA2PPPPPPPPP\x91P\x91V[G\x15a\r,Wa\r,3Ga2gV[V[`\0\x83`\x01`\x01`\xA0\x1B\x03\x16\x85`\x01`\x01`\xA0\x1B\x03\x16\x10a\rNW`\0\x80\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x16\x98\xEE\x82\x86\x86\x86`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x84`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x82b\xFF\xFF\xFF\x16\x81R` \x01\x93PPPP` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\r\xD9W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\r\xEDW=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x0E\x03W`\0\x80\xFD[PQ\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a\x0F9W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xA1g\x12\x95\x86\x86\x86`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x84`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x82b\xFF\xFF\xFF\x16\x81R` \x01\x93PPPP` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0E\xA2W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0E\xB6W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x0E\xCCW`\0\x80\xFD[PQ`@\x80Qc\xF67s\x1D`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R\x91Q\x92\x93P\x90\x83\x16\x91c\xF67s\x1D\x91`$\x80\x82\x01\x92`\0\x92\x90\x91\x90\x82\x90\x03\x01\x81\x83\x87\x80;\x15\x80\x15a\x0F\x1CW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0F0W=`\0\x80>=`\0\xFD[PPPPa\x10\x1AV[`\0\x81`\x01`\x01`\xA0\x1B\x03\x16c8P\xC7\xBD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\xE0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0FtW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x0F\x88W=`\0\x80>=`\0\xFD[PPPP`@Q=`\xE0\x81\x10\x15a\x0F\x9EW`\0\x80\xFD[PQ\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a\x10\x18W\x81`\x01`\x01`\xA0\x1B\x03\x16c\xF67s\x1D\x84`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x82`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x91PP`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0F\xFFW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x10\x13W=`\0\x80>=`\0\xFD[PPPP[P[\x94\x93PPPPV[`\0a\x10.`\x02a3VV[\x90P\x90V[`\0\x80`\0\x83`\xA0\x015\x80a\x10Fa0~V[\x11\x15a\x10\x8FW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr\x15\x1C\x98[\x9C\xD8X\xDD\x1A[\xDB\x88\x1D\x1B\xDB\xC8\x1B\xDB\x19`j\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x845`\0\x90\x81R`\x0C` \x90\x81R`@\x80\x83 `\x01\x80\x82\x01T`\x01`\x01`P\x1B\x03\x81\x16\x86R`\x0B\x85R\x83\x86 \x84Q``\x80\x82\x01\x87R\x82T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83R\x92\x90\x94\x01T\x80\x83\x16\x82\x89\x01\x90\x81Rb\xFF\xFF\xFF`\x01`\xA0\x1B\x90\x92\x04\x82\x16\x83\x89\x01\x90\x81R\x88Qa\x01@\x81\x01\x8AR\x84Q\x86\x16\x81R\x91Q\x90\x94\x16\x81\x8A\x01R\x92Q\x16\x82\x87\x01R0\x82\x85\x01R`\x01`P\x1B\x83\x04`\x02\x90\x81\x0B\x81\x0B`\x80\x80\x85\x01\x91\x90\x91R`\x01`h\x1B\x90\x94\x04\x81\x0B\x90\x0B`\xA0\x83\x01R\x95\x8C\x015`\xC0\x82\x01R\x93\x8B\x015`\xE0\x85\x01R\x90\x8A\x015a\x01\0\x84\x01R\x89\x015a\x01 \x83\x01R\x92\x90a\x11q\x90a3aV[`\x01\x87\x01T\x93\x9AP\x91\x98P\x96P\x91P`\0\x90a\x11\xA4\x900\x90`\x01`P\x1B\x81\x04`\x02\x90\x81\x0B\x91`\x01`h\x1B\x90\x04\x90\x0Ba1cV[\x90P`\0\x80\x83`\x01`\x01`\xA0\x1B\x03\x16cQN\xA4\xBF\x84`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x11\xD5\x91\x90aT\x14V[`\xA0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x11\xEDW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x12\x01W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12%\x91\x90aP\xB6V[PP\x92P\x92PPa\x12a\x86`\x02\x01T\x83\x03\x87`\x01\x01`\x10\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\x80\x1B\x03\x16`\x01`\x01`\x80\x1B\x03\x16`\x01`\x80\x1Ba1\xB8V[`\x04\x87\x01\x80T`\x01`\x01`\x80\x1B\x03\x80\x82\x16\x90\x93\x01\x83\x16`\x01`\x01`\x80\x1B\x03\x19\x90\x91\x16\x17\x90U`\x03\x87\x01T`\x01\x88\x01Ta\x12\xA8\x92\x91\x84\x03\x91`\x01`\x80\x1B\x91\x82\x90\x04\x16\x90a1\xB8V[`\x04\x87\x01\x80T`\x01`\x01`\x80\x1B\x03`\x01`\x80\x1B\x80\x83\x04\x82\x16\x90\x94\x01\x81\x16\x84\x02\x91\x81\x16\x91\x90\x91\x17\x90\x91U`\x02\x88\x01\x84\x90U`\x03\x88\x01\x83\x90U`\x01\x88\x01\x80T\x83\x81\x04\x83\x16\x8E\x01\x83\x16\x90\x93\x02\x92\x90\x91\x16\x91\x90\x91\x17\x90U`@Q\x8B5\x90\x7F0g\x04\x8B\xEE\xE3\x1B%\xB2\xF1h\x1F\x88\xDA\xC88\xC8\xBB\xA3j\xF2[\xFB+|\xF7G:XG\xE3_\x90a\x132\x90\x8D\x90\x8D\x90\x8D\x90aU\x9EV[`@Q\x80\x91\x03\x90\xA2PPPPPPP\x91\x93\x90\x92PV[a\x13Ya\x13Sa/hV[\x82a/\xE2V[a\x13\x94W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`1\x81R` \x01\x80aY\x81`1\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[a\x08\xCF\x83\x83\x83a5\x9CV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x01` R`@\x81 a\x13\xC1\x90\x83a6\xE8V[\x90P[\x92\x91PPV[\x7FI\xEC\xF33\xE5\xB8\xC9\\@\xFD\xAF\xC9\\\x1A\xD16\xE8\x91J\x8F\xB5^\x9D\xC8\xBB\x01\xEA\xA8:-\xF9\xAD\x81V[`\0\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x14[a6\xF4V[0`@Q` \x01\x80\x86\x81R` \x01\x85\x81R` \x01\x84\x81R` \x01\x83\x81R` \x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x95PPPPPP`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[`\x0EUV[a\x08\xCF\x83\x83\x83`@Q\x80` \x01`@R\x80`\0\x81RPa&\xD3V[\x80a\x14\xD73\x82a/\xE2V[a\x14\xF3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xD2\x90aToV[`\0\x82\x81R`\x0C` R`@\x90 `\x01\x81\x01T`\x01`\x80\x1B\x90\x04`\x01`\x01`\x80\x1B\x03\x16\x15\x80\x15a\x15.WP`\x04\x81\x01T`\x01`\x01`\x80\x1B\x03\x16\x15[\x80\x15a\x15LWP`\x04\x81\x01T`\x01`\x80\x1B\x90\x04`\x01`\x01`\x80\x1B\x03\x16\x15[a\x15hW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xD2\x90aU9V[`\0\x83\x81R`\x0C` R`@\x81 \x81\x81U`\x01\x81\x01\x82\x90U`\x02\x81\x01\x82\x90U`\x03\x81\x01\x82\x90U`\x04\x01Ua\x08\xCF\x83a6\xF8V[`@\x80Qc#\xF2\xEB\xC3`\xE2\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x87\x90R`d\x81\x01\x86\x90R`\x01`\x84\x82\x01R`\xFF\x85\x16`\xA4\x82\x01R`\xC4\x81\x01\x84\x90R`\xE4\x81\x01\x83\x90R\x90Q`\x01`\x01`\xA0\x1B\x03\x88\x16\x91c\x8F\xCB\xAF\x0C\x91a\x01\x04\x80\x83\x01\x92`\0\x92\x91\x90\x82\x90\x03\x01\x81\x83\x87\x80;\x15\x80\x15a\x16\x15W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x16)W=`\0\x80>=`\0\xFD[PPPPPPPPPPV[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16cp\xA0\x8210`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x82`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x91PP` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x16\xA4W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x16\xB8W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x16\xCEW`\0\x80\xFD[PQ\x90P\x82\x81\x10\x15a\x17\x1CW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01RqInsufficient WETH9`p\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x80\x15a\x08\xCFW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c.\x1A}M\x82`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x82\x81R` \x01\x91PP`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x17\x88W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x17\x9CW=`\0\x80>=`\0\xFD[PPPPa\x08\xCF\x82\x82a2gV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0\x80a\x17\xDC`\x02\x84a7\xC5V[P\x93\x92PPPV[`\0a\x13\xC4\x82`@Q\x80``\x01`@R\x80`)\x81R` \x01aX\xEC`)\x919`\x02\x91\x90a7\xE3V[``\x90V[`\0`\x01`\x01`\xA0\x1B\x03\x82\x16a\x18XW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`*\x81R` \x01\x80aX\xC2`*\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x01` R`@\x90 a\x13\xC4\x90a3VV[\x83a\x18\x82a0~V[\x11\x15a\x18\xC6W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rm\x14\x19\\\x9BZ]\x08\x19^\x1C\x1A\\\x99Y`\x92\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\0a\x18\xD0a\x13\xEEV[\x7FI\xEC\xF33\xE5\xB8\xC9\\@\xFD\xAF\xC9\\\x1A\xD16\xE8\x91J\x8F\xB5^\x9D\xC8\xBB\x01\xEA\xA8:-\xF9\xAD\x88\x88a\x18\xFC\x81a7\xF0V[`@\x80Q` \x80\x82\x01\x96\x90\x96R`\x01`\x01`\xA0\x1B\x03\x90\x94\x16\x84\x82\x01R``\x84\x01\x92\x90\x92R`\x80\x83\x01R`\xA0\x80\x83\x01\x8A\x90R\x81Q\x80\x84\x03\x90\x91\x01\x81R`\xC0\x83\x01\x82R\x80Q\x90\x84\x01 a\x19\x01`\xF0\x1B`\xE0\x84\x01R`\xE2\x83\x01\x94\x90\x94Ra\x01\x02\x80\x83\x01\x94\x90\x94R\x80Q\x80\x83\x03\x90\x94\x01\x84Ra\x01\"\x90\x91\x01\x90R\x81Q\x91\x01 \x90P`\0a\x19\x84\x87a\x17\xE4V[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16\x88`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a\x19\xD7W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`'\x81R` \x01\x80aW\xED`'\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[a\x19\xE0\x81a8%V[\x15a\x1BHW`@\x80Q` \x80\x82\x01\x87\x90R\x81\x83\x01\x86\x90R`\x01`\x01`\xF8\x1B\x03\x19`\xF8\x89\x90\x1B\x16``\x83\x01R\x82Q`A\x81\x84\x03\x01\x81R`a\x83\x01\x80\x85Rc\x0B\x13]?`\xE1\x1B\x90R`e\x83\x01\x86\x81R`\x85\x84\x01\x94\x85R\x81Q`\xA5\x85\x01R\x81Q`\x01`\x01`\xA0\x1B\x03\x87\x16\x95c\x16&\xBA~\x95\x89\x95\x91\x92`\xC5\x90\x91\x01\x91\x85\x01\x90\x80\x83\x83`\0[\x83\x81\x10\x15a\x1AyW\x81\x81\x01Q\x83\x82\x01R` \x01a\x1AaV[PPPP\x90P\x90\x81\x01\x90`\x1F\x16\x80\x15a\x1A\xA6W\x80\x82\x03\x80Q`\x01\x83` \x03a\x01\0\n\x03\x19\x16\x81R` \x01\x91P[P\x93PPPP` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1A\xC4W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x1A\xD8W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x1A\xEEW`\0\x80\xFD[PQ`\x01`\x01`\xE0\x1B\x03\x19\x16c\x0B\x13]?`\xE1\x1B\x14a\x1BCW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01Rk\x15[\x98]]\x1A\x1B\xDC\x9A^\x99Y`\xA2\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[a\x1CWV[`\0`\x01\x83\x87\x87\x87`@Q`\0\x81R` \x01`@R`@Q\x80\x85\x81R` \x01\x84`\xFF\x16\x81R` \x01\x83\x81R` \x01\x82\x81R` \x01\x94PPPPP` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x1B\xA4W=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a\x1C\0W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01RpInvalid signature`x\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x81`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1CUW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01Rk\x15[\x98]]\x1A\x1B\xDC\x9A^\x99Y`\xA2\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[P[a\x1Ca\x88\x88a/lV[PPPPPPPPV[`\0\x80`\0\x80\x84a\x01@\x015\x80a\x1C\x80a0~V[\x11\x15a\x1C\xC9W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr\x15\x1C\x98[\x9C\xD8X\xDD\x1A[\xDB\x88\x1D\x1B\xDB\xC8\x1B\xDB\x19`j\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`@\x80Qa\x01@\x81\x01\x90\x91R`\0\x90a\x1D\x95\x90\x80a\x1C\xEA` \x8B\x01\x8BaK\xD1V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x89` \x01` \x81\x01\x90a\x1D\x0B\x91\x90aK\xD1V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01a\x1D)``\x8B\x01`@\x8C\x01aQ\xA8V[b\xFF\xFF\xFF\x16\x81R0` \x82\x01R`@\x01a\x1DI`\x80\x8B\x01``\x8C\x01aN\xF0V[`\x02\x0B\x81R` \x01a\x1Da`\xA0\x8B\x01`\x80\x8C\x01aN\xF0V[`\x02\x0B\x81R` \x01\x89`\xA0\x015\x81R` \x01\x89`\xC0\x015\x81R` \x01\x89`\xE0\x015\x81R` \x01\x89a\x01\0\x015\x81RPa3aV[\x92\x97P\x90\x95P\x93P\x90Pa\x1D\xE2a\x1D\xB4a\x01@\x89\x01a\x01 \x8A\x01aK\xD1V[`\r\x80T`\x01`\x01`\xB0\x1B\x03\x19\x81\x16`\x01`\x01`\x01`\xB0\x1B\x03\x92\x83\x16\x90\x81\x01\x90\x92\x16\x17\x90\x91U\x97P\x87a8+V[`\0a\x1E\r0a\x1D\xF8`\x80\x8B\x01``\x8C\x01aN\xF0V[a\x1E\x08`\xA0\x8C\x01`\x80\x8D\x01aN\xF0V[a1cV[\x90P`\0\x80\x83`\x01`\x01`\xA0\x1B\x03\x16cQN\xA4\xBF\x84`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1E>\x91\x90aT\x14V[`\xA0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1EVW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x1EjW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E\x8E\x91\x90aP\xB6V[PP\x92P\x92PP`\0a\x1F\x07\x85`@Q\x80``\x01`@R\x80\x8E`\0\x01` \x81\x01\x90a\x1E\xB9\x91\x90aK\xD1V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8E` \x01` \x81\x01\x90a\x1E\xDA\x91\x90aK\xD1V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8E`@\x01` \x81\x01\x90a\x1E\xFB\x91\x90aQ\xA8V[b\xFF\xFF\xFF\x16\x90Ra9YV[\x90P`@Q\x80a\x01@\x01`@R\x80`\0`\x01`\x01``\x1B\x03\x16\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x82`\x01`\x01`P\x1B\x03\x16\x81R` \x01\x8C``\x01` \x81\x01\x90a\x1FW\x91\x90aN\xF0V[`\x02\x0B\x81R` \x01a\x1Fo`\xA0\x8E\x01`\x80\x8F\x01aN\xF0V[`\x02\x0B\x81R` \x01\x8A`\x01`\x01`\x80\x1B\x03\x16\x81R` \x01\x84\x81R` \x01\x83\x81R` \x01`\0`\x01`\x01`\x80\x1B\x03\x16\x81R` \x01`\0`\x01`\x01`\x80\x1B\x03\x16\x81RP`\x0C`\0\x8C\x81R` \x01\x90\x81R` \x01`\0 `\0\x82\x01Q\x81`\0\x01`\0a\x01\0\n\x81T\x81`\x01`\x01``\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01``\x1B\x03\x16\x02\x17\x90UP` \x82\x01Q\x81`\0\x01`\x0Ca\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP`@\x82\x01Q\x81`\x01\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`P\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`P\x1B\x03\x16\x02\x17\x90UP``\x82\x01Q\x81`\x01\x01`\na\x01\0\n\x81T\x81b\xFF\xFF\xFF\x02\x19\x16\x90\x83`\x02\x0Bb\xFF\xFF\xFF\x16\x02\x17\x90UP`\x80\x82\x01Q\x81`\x01\x01`\ra\x01\0\n\x81T\x81b\xFF\xFF\xFF\x02\x19\x16\x90\x83`\x02\x0Bb\xFF\xFF\xFF\x16\x02\x17\x90UP`\xA0\x82\x01Q\x81`\x01\x01`\x10a\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP`\xC0\x82\x01Q\x81`\x02\x01U`\xE0\x82\x01Q\x81`\x03\x01Ua\x01\0\x82\x01Q\x81`\x04\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UPa\x01 \x82\x01Q\x81`\x04\x01`\x10a\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP\x90PP\x89\x7F0g\x04\x8B\xEE\xE3\x1B%\xB2\xF1h\x1F\x88\xDA\xC88\xC8\xBB\xA3j\xF2[\xFB+|\xF7G:XG\xE3_\x8A\x8A\x8A`@Qa!n\x93\x92\x91\x90aU\x9EV[`@Q\x80\x91\x03\x90\xA2PPPPPP\x91\x93P\x91\x93V[`\x07\x80T`@\x80Q` `\x1F`\x02`\0\x19a\x01\0`\x01\x88\x16\x15\x02\x01\x90\x95\x16\x94\x90\x94\x04\x93\x84\x01\x81\x90\x04\x81\x02\x82\x01\x81\x01\x90\x92R\x82\x81R``\x93\x90\x92\x90\x91\x83\x01\x82\x82\x80\x15a\x07\xA1W\x80`\x1F\x10a\x07vWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x07\xA1V[`\0\x81\x81R`\x0C` \x90\x81R`@\x80\x83 \x81Qa\x01@\x81\x01\x83R\x81T`\x01`\x01``\x1B\x03\x81\x16\x82R`\x01`\x01`\xA0\x1B\x03`\x01``\x1B\x90\x91\x04\x16\x93\x81\x01\x93\x90\x93R`\x01\x81\x01T`\x01`\x01`P\x1B\x03\x81\x16\x92\x84\x01\x83\x90R`\x01`P\x1B\x81\x04`\x02\x90\x81\x0B\x81\x0B\x81\x0B``\x86\x01R`\x01`h\x1B\x82\x04\x81\x0B\x81\x0B\x81\x0B`\x80\x86\x01R`\x01`\x01`\x80\x1B\x03`\x01`\x80\x1B\x92\x83\x90\x04\x81\x16`\xA0\x87\x01R\x90\x83\x01T`\xC0\x86\x01R`\x03\x83\x01T`\xE0\x86\x01R`\x04\x90\x92\x01T\x80\x83\x16a\x01\0\x86\x01R\x04\x16a\x01 \x83\x01R\x82\x91\x82\x91\x82\x91\x82\x91\x82\x91\x82\x91\x82\x91\x82\x91\x82\x91\x82\x91\x82\x91\x90a\"\xD5W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xD2\x90aU\x0FV[`\0`\x0B`\0\x83`@\x01Q`\x01`\x01`P\x1B\x03\x16`\x01`\x01`P\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `@Q\x80``\x01`@R\x90\x81`\0\x82\x01`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\x01\x82\x01`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\x01\x82\x01`\x14\x90T\x90a\x01\0\n\x90\x04b\xFF\xFF\xFF\x16b\xFF\xFF\xFF\x16b\xFF\xFF\xFF\x16\x81RPP\x90P\x81`\0\x01Q\x82` \x01Q\x82`\0\x01Q\x83` \x01Q\x84`@\x01Q\x86``\x01Q\x87`\x80\x01Q\x88`\xA0\x01Q\x89`\xC0\x01Q\x8A`\xE0\x01Q\x8Ba\x01\0\x01Q\x8Ca\x01 \x01Q\x9DP\x9DP\x9DP\x9DP\x9DP\x9DP\x9DP\x9DP\x9DP\x9DP\x9DP\x9DPPP\x91\x93\x95\x97\x99\x9BP\x91\x93\x95\x97\x99\x9BV[a#\xFDa/hV[`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a$cW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FERC721: approve to caller\0\0\0\0\0\0\0`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x80`\x05`\0a$pa/hV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R` \x80\x83\x01\x93\x90\x93R`@\x91\x82\x01`\0\x90\x81 \x91\x87\x16\x80\x82R\x91\x90\x93R\x91 \x80T`\xFF\x19\x16\x92\x15\x15\x92\x90\x92\x17\x90\x91Ua$\xB4a/hV[`\x01`\x01`\xA0\x1B\x03\x16\x7F\x170~\xAB9\xABa\x07\xE8\x89\x98E\xAD=Y\xBD\x96S\xF2\0\xF2 \x92\x04\x89\xCA+Y7il1\x83`@Q\x80\x82\x15\x15\x81R` \x01\x91PP`@Q\x80\x91\x03\x90\xA3PPV[`@\x80Qcn\xB1v\x9F`\xE1\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R\x90Q`\0\x19\x91`\x01`\x01`\xA0\x1B\x03\x89\x16\x91c\xDDb\xED>\x91`D\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a%KW`\0\x80\xFD[PZ\xFA\x15\x80\x15a%_W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a%uW`\0\x80\xFD[PQ\x10\x15a%\x8BWa%\x8B\x86\x86\x86\x86\x86\x86a\x15\x9BV[PPPPPPV[``\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x80\x15a%\xACW`\0\x80\xFD[P`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a%\xE0W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a%\xCBW\x90P[P\x90P`\0[\x82\x81\x10\x15a&\xCCW`\0\x800\x86\x86\x85\x81\x81\x10a%\xFEW\xFE[\x90P` \x02\x81\x01\x90a&\x10\x91\x90aV\x8BV[`@Qa&\x1E\x92\x91\x90aR\xDCV[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a&YW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a&^V[``\x91P[P\x91P\x91P\x81a&\xAAW`D\x81Q\x10\x15a&wW`\0\x80\xFD[`\x04\x81\x01\x90P\x80\x80` \x01\x90Q\x81\x01\x90a&\x91\x91\x90aO\x0CV[`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xD2\x91\x90aT\\V[\x80\x84\x84\x81Q\x81\x10a&\xB7W\xFE[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a%\xE6V[P\x92\x91PPV[a&\xE4a&\xDEa/hV[\x83a/\xE2V[a'\x1FW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`1\x81R` \x01\x80aY\x81`1\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[a'+\x84\x84\x84\x84a:;V[PPPPV[`@\x80Qcn\xB1v\x9F`\xE1\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R\x90Q\x86\x91`\x01`\x01`\xA0\x1B\x03\x89\x16\x91c\xDDb\xED>\x91`D\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a'\x80W`\0\x80\xFD[PZ\xFA\x15\x80\x15a'\x94W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a'\xAAW`\0\x80\xFD[PQ\x10\x15a%\x8BWa%\x8B\x86\x86\x86\x86\x86\x86a*\x1EV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[``a'\xEF\x82a/[V[a'\xF8W`\0\x80\xFD[`@Qc\xE9\xDCcu`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xE9\xDCcu\x90a(F\x900\x90\x86\x90`\x04\x01aT\x1DV[`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a(^W`\0\x80\xFD[PZ\xFA\x15\x80\x15a(rW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x13\xC4\x91\x90\x81\x01\x90aO\x0CV[`\0a(\xA8\x82\x84\x01\x84aO\xAFV[\x90Pa(\xD8\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82`\0\x01Qa:\x8DV[P\x84\x15a(\xF3W\x80QQ` \x82\x01Qa(\xF3\x91\x903\x88a:\xB0V[\x83\x15a)\x11Wa)\x11\x81`\0\x01Q` \x01Q\x82` \x01Q3\x87a:\xB0V[PPPPPV[`\0\x83`\x01`\x01`\xA0\x1B\x03\x16cp\xA0\x8210`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x82`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x91PP` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a)gW`\0\x80\xFD[PZ\xFA\x15\x80\x15a){W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a)\x91W`\0\x80\xFD[PQ\x90P\x82\x81\x10\x15a)\xDFW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01Rq$\xB79\xBA\xB334\xB1\xB4\xB2\xB7:\x10:7\xB5\xB2\xB7`q\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x80\x15a'+Wa'+\x84\x83\x83a<@V[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\0\x90\x81R`\x05` \x90\x81R`@\x80\x83 \x93\x90\x94\x16\x82R\x91\x90\x91R T`\xFF\x16\x90V[`@\x80Qc\xD5\x05\xAC\xCF`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x87\x90R`d\x81\x01\x86\x90R`\xFF\x85\x16`\x84\x82\x01R`\xA4\x81\x01\x84\x90R`\xC4\x81\x01\x83\x90R\x90Q`\x01`\x01`\xA0\x1B\x03\x88\x16\x91c\xD5\x05\xAC\xCF\x91`\xE4\x80\x83\x01\x92`\0\x92\x91\x90\x82\x90\x03\x01\x81\x83\x87\x80;\x15\x80\x15a\x16\x15W`\0\x80\xFD[`\0\x80\x825a*\x9F3\x82a/\xE2V[a*\xBBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xD2\x90aToV[`\0a*\xCD``\x86\x01`@\x87\x01aPlV[`\x01`\x01`\x80\x1B\x03\x16\x11\x80a*\xFAWP`\0a*\xEF`\x80\x86\x01``\x87\x01aPlV[`\x01`\x01`\x80\x1B\x03\x16\x11[a+\x03W`\0\x80\xFD[`\0\x80a+\x16`@\x87\x01` \x88\x01aK\xD1V[`\x01`\x01`\xA0\x1B\x03\x16\x14a+9Wa+4`@\x86\x01` \x87\x01aK\xD1V[a+;V[0[\x855`\0\x90\x81R`\x0C` \x90\x81R`@\x80\x83 `\x01\x80\x82\x01T`\x01`\x01`P\x1B\x03\x16\x85R`\x0B\x84R\x82\x85 \x83Q``\x81\x01\x85R\x81T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R\x91\x90\x92\x01T\x90\x81\x16\x94\x82\x01\x94\x90\x94R`\x01`\xA0\x1B\x90\x93\x04b\xFF\xFF\xFF\x16\x91\x83\x01\x91\x90\x91R\x92\x93P\x90a+\xCE\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83a0\x84V[`\x04\x84\x01T`\x01\x85\x01T\x91\x92P`\x01`\x01`\x80\x1B\x03\x80\x82\x16\x92`\x01`\x80\x1B\x92\x83\x90\x04\x82\x16\x92\x90\x04\x16\x15a-\xC4W`\x01\x85\x01T`@Qc\xA3A#\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16\x91c\xA3A#\xA7\x91a,E\x91`\x01`P\x1B\x81\x04`\x02\x90\x81\x0B\x92`\x01`h\x1B\x90\x92\x04\x90\x0B\x90`\0\x90`\x04\x01aT6V[`@\x80Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a,^W`\0\x80\xFD[PZ\xF1\x15\x80\x15a,rW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a,\x96\x91\x90aQ\xFEV[PP`\x01\x85\x01T`\0\x90\x81\x90`\x01`\x01`\xA0\x1B\x03\x86\x16\x90cQN\xA4\xBF\x90a,\xD4\x900\x90`\x01`P\x1B\x81\x04`\x02\x90\x81\x0B\x91`\x01`h\x1B\x90\x04\x90\x0Ba1cV[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a,\xF0\x91\x90aT\x14V[`\xA0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a-\x08W`\0\x80\xFD[PZ\xFA\x15\x80\x15a-\x1CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a-@\x91\x90aP\xB6V[PP\x92P\x92PPa-|\x87`\x02\x01T\x83\x03\x88`\x01\x01`\x10\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\x80\x1B\x03\x16`\x01`\x01`\x80\x1B\x03\x16`\x01`\x80\x1Ba1\xB8V[\x84\x01\x93Pa-\xB5\x87`\x03\x01T\x82\x03\x88`\x01\x01`\x10\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\x80\x1B\x03\x16`\x01`\x01`\x80\x1B\x03\x16`\x01`\x80\x1Ba1\xB8V[`\x02\x88\x01\x92\x90\x92U`\x03\x87\x01U\x01[`\0\x80`\x01`\x01`\x80\x1B\x03\x84\x16a-\xE1``\x8E\x01`@\x8F\x01aPlV[`\x01`\x01`\x80\x1B\x03\x16\x11a.\x04Wa-\xFF``\x8D\x01`@\x8E\x01aPlV[a.\x06V[\x83[\x83`\x01`\x01`\x80\x1B\x03\x16\x8D``\x01` \x81\x01\x90a.#\x91\x90aPlV[`\x01`\x01`\x80\x1B\x03\x16\x11a.FWa.A`\x80\x8E\x01``\x8F\x01aPlV[a.HV[\x83[`\x01\x89\x01T`@Qc\t\xE3\xD6{`\xE3\x1B\x81R\x92\x94P\x90\x92P`\x01`\x01`\xA0\x1B\x03\x87\x16\x91cO\x1E\xB3\xD8\x91a.\x9B\x91\x8C\x91`\x01`P\x1B\x81\x04`\x02\x90\x81\x0B\x92`\x01`h\x1B\x90\x92\x04\x90\x0B\x90\x88\x90\x88\x90`\x04\x01aSBV[`@\x80Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a.\xB4W`\0\x80\xFD[PZ\xF1\x15\x80\x15a.\xC8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a.\xEC\x91\x90aP\x88V[`\x04\x89\x01\x80T`\x01`\x01`\x80\x1B\x03\x19`\x01`\x01`\x80\x1B\x03\x91\x82\x16`\x01`\x80\x1B\x87\x8A\x03\x84\x16\x02\x17\x16\x86\x89\x03\x82\x16\x17\x90\x91U`@Q\x92\x81\x16\x9DP\x16\x9AP\x8C5\x90\x7F@\xD0\xEF\xD1\xA5=`\xEC\xBF@\x97\x1B\x9D\xAF}\xC9\x01x\xC3\xAA\xDCz\xAB\x17ec'8\xFA\x8B\x8F\x01\x90a\r\x06\x90\x8B\x90\x86\x90\x86\x90aS\x7FV[`\0a\x13\xC4`\x02\x83a=\x87V[3\x90V[`\0\x81\x81R`\x0C` R`@\x90 \x80T`\x01`\x01``\x1B\x03\x16`\x01``\x1B`\x01`\x01`\xA0\x1B\x03\x85\x16\x90\x81\x02\x91\x90\x91\x17\x90\x91U\x81\x90a/\xA9\x82a\x17\xE4V[`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%`@Q`@Q\x80\x91\x03\x90\xA4PPV[`\0a/\xED\x82a/[V[a0(W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`,\x81R` \x01\x80aX^`,\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[`\0a03\x83a\x17\xE4V[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x14\x80a0nWP\x83`\x01`\x01`\xA0\x1B\x03\x16a0c\x84a\x07\xABV[`\x01`\x01`\xA0\x1B\x03\x16\x14[\x80a\x10\x1AWPa\x10\x1A\x81\x85a)\xF0V[`\x0ET\x90V[`\0\x81` \x01Q`\x01`\x01`\xA0\x1B\x03\x16\x82`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x10a0\xACW`\0\x80\xFD[P\x80Q` \x80\x83\x01Q`@\x93\x84\x01Q\x84Q`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x81\x85\x01R\x93\x90\x91\x16\x83\x85\x01Rb\xFF\xFF\xFF\x16``\x80\x84\x01\x91\x90\x91R\x83Q\x80\x84\x03\x82\x01\x81R`\x80\x84\x01\x85R\x80Q\x90\x83\x01 `\x01`\x01`\xF8\x1B\x03\x19`\xA0\x85\x01R\x94\x90\x1B`\x01`\x01``\x1B\x03\x19\x16`\xA1\x83\x01R`\xB5\x82\x01\x93\x90\x93R\x7F\xE3O\x19\x9B\x19\xB2\xB4\xF4\x7FhD&\x19\xD5UR}$Ox\xA3)~\xA8\x93%\xF8C\xF8{\x8BT`\xD5\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\xF5\x90\x91\x01\x90\x91R\x80Q\x91\x01 \x90V[`@\x80Q`\x01`\x01``\x1B\x03\x19``\x86\x90\x1B\x16` \x80\x83\x01\x91\x90\x91R`\x02\x85\x81\x0B`\xE8\x90\x81\x1B`4\x85\x01R\x90\x85\x90\x0B\x90\x1B`7\x83\x01R\x82Q`\x1A\x81\x84\x03\x01\x81R`:\x90\x92\x01\x90\x92R\x80Q\x91\x01 [\x93\x92PPPV[`\0\x80\x80`\0\x19\x85\x87\t\x86\x86\x02\x92P\x82\x81\x10\x90\x83\x90\x03\x03\x90P\x80a1\xEEW`\0\x84\x11a1\xE3W`\0\x80\xFD[P\x82\x90\x04\x90Pa1\xB1V[\x80\x84\x11a1\xFAW`\0\x80\xFD[`\0\x84\x86\x88\t`\0\x86\x81\x03\x87\x16\x96\x87\x90\x04\x96`\x02`\x03\x89\x02\x81\x18\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x90\x91\x03\x02\x91\x81\x90\x03\x81\x90\x04`\x01\x01\x86\x84\x11\x90\x95\x03\x94\x90\x94\x02\x91\x90\x94\x03\x92\x90\x92\x04\x91\x90\x91\x17\x91\x90\x91\x02\x91PP\x93\x92PPPV[`@\x80Q`\0\x80\x82R` \x82\x01\x90\x92R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x83\x90`@Q\x80\x82\x80Q\x90` \x01\x90\x80\x83\x83[` \x83\x10a2\xB3W\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01a2\x94V[`\x01\x83` \x03a\x01\0\n\x03\x80\x19\x82Q\x16\x81\x84Q\x16\x80\x82\x17\x85RPPPPPP\x90P\x01\x91PP`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a3\x15W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a3\x1AV[``\x91P[PP\x90P\x80a\x08\xCFW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x03`$\x82\x01RbSTE`\xE8\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\0a\x13\xC4\x82a=\x93V[`\0\x80`\0\x80`\0`@Q\x80``\x01`@R\x80\x87`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x87` \x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x87`@\x01Qb\xFF\xFF\xFF\x16\x81RP\x90Pa3\xD3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82a0\x84V[\x91P`\0\x82`\x01`\x01`\xA0\x1B\x03\x16c8P\xC7\xBD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\xE0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a4\x10W`\0\x80\xFD[PZ\xFA\x15\x80\x15a4$W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a4H\x91\x90aQ\x17V[PPPPPP\x90P`\0a4_\x88`\x80\x01Qa=\x97V[\x90P`\0a4p\x89`\xA0\x01Qa=\x97V[\x90Pa4\x87\x83\x83\x83\x8C`\xC0\x01Q\x8D`\xE0\x01Qa@\xC9V[\x97PPPP\x81`\x01`\x01`\xA0\x1B\x03\x16c<\x8A}\x8D\x87``\x01Q\x88`\x80\x01Q\x89`\xA0\x01Q\x89`@Q\x80`@\x01`@R\x80\x88\x81R` \x013`\x01`\x01`\xA0\x1B\x03\x16\x81RP`@Q` \x01a4\xD9\x91\x90aU^V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a5\x08\x95\x94\x93\x92\x91\x90aS\0V[`@\x80Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a5!W`\0\x80\xFD[PZ\xF1\x15\x80\x15a55W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a5Y\x91\x90aQ\xFEV[a\x01\0\x88\x01Q\x91\x95P\x93P\x84\x10\x80\x15\x90a5xWP\x85a\x01 \x01Q\x83\x10\x15[a5\x94W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xD2\x90aT\xE1V[P\x91\x93P\x91\x93V[\x82`\x01`\x01`\xA0\x1B\x03\x16a5\xAF\x82a\x17\xE4V[`\x01`\x01`\xA0\x1B\x03\x16\x14a5\xF4W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`)\x81R` \x01\x80aY7`)\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x82\x16a69W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`$\x81R` \x01\x80aX\x14`$\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[a6D\x83\x83\x83a\x08\xCFV[a6O`\0\x82a/lV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x01` R`@\x90 a6q\x90\x82aA\x8DV[P`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x01` R`@\x90 a6\x94\x90\x82aA\x99V[Pa6\xA1`\x02\x82\x84aA\xA5V[P\x80\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF`@Q`@Q\x80\x91\x03\x90\xA4PPPV[`\0a\x13\xC1\x83\x83aA\xBBV[F\x90V[`\0a7\x03\x82a\x17\xE4V[\x90Pa7\x11\x81`\0\x84a\x08\xCFV[a7\x1C`\0\x83a/lV[`\0\x82\x81R`\x08` R`@\x90 T`\x02`\0\x19a\x01\0`\x01\x84\x16\x15\x02\x01\x90\x91\x16\x04\x15a7ZW`\0\x82\x81R`\x08` R`@\x81 a7Z\x91aKAV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x01` R`@\x90 a7|\x90\x83aA\x8DV[Pa7\x88`\x02\x83aB\x1FV[P`@Q\x82\x90`\0\x90`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90\x83\x90\xA4PPV[`\0\x80\x80\x80a7\xD4\x86\x86aB+V[\x90\x94P\x92PPP[\x92P\x92\x90PV[`\0a\x10\x1A\x84\x84\x84aB\xA6V[`\0\x90\x81R`\x0C` R`@\x90 \x80T`\x01`\x01``\x1B\x03\x19\x81\x16`\x01`\x01`\x01``\x1B\x03\x92\x83\x16\x90\x81\x01\x90\x92\x16\x17\x90\x91U\x90V[;\x15\x15\x90V[`\x01`\x01`\xA0\x1B\x03\x82\x16a8\x86W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FERC721: mint to the zero address`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[a8\x8F\x81a/[V[\x15a8\xE1W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FERC721: token already minted\0\0\0\0`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[a8\xED`\0\x83\x83a\x08\xCFV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x01` R`@\x90 a9\x0F\x90\x82aA\x99V[Pa9\x1C`\x02\x82\x84aA\xA5V[P`@Q\x81\x90`\x01`\x01`\xA0\x1B\x03\x84\x16\x90`\0\x90\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90\x82\x90\xA4PPV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\n` R`@\x90 T`\x01`\x01`P\x1B\x03\x16\x80a\x13\xC4WP`\r\x80T`\x01`\x01`\x01`P\x1B\x03`\x01`\xB0\x1B\x80\x84\x04\x82\x16\x83\x81\x01\x90\x92\x16\x02`\x01`\x01`\xB0\x1B\x03\x90\x93\x16\x92\x90\x92\x17\x90\x92U`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x90\x81R`\n` \x90\x81R`@\x80\x83 \x80Ti\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x86\x17\x90U\x84\x83R`\x0B\x82R\x91\x82\x90 \x86Q\x81T\x90\x85\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x17\x82U\x91\x87\x01Q\x95\x01\x80T\x92\x87\x01Qb\xFF\xFF\xFF\x16`\x01`\xA0\x1B\x02b\xFF\xFF\xFF`\xA0\x1B\x19\x96\x90\x94\x16\x92\x90\x91\x16\x91\x90\x91\x17\x93\x90\x93\x16\x17\x90\x91U\x92\x91PPV[a:F\x84\x84\x84a5\x9CV[a:R\x84\x84\x84\x84aCpV[a'+W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`2\x81R` \x01\x80aW\xBB`2\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[`\0a:\x99\x83\x83a0\x84V[\x90P3`\x01`\x01`\xA0\x1B\x03\x82\x16\x14a\x13\xC4W`\0\x80\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x14\x80\x15a:\xF1WP\x80G\x10\x15[\x15a<\x13W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xD0\xE3\r\xB0\x82`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a;QW`\0\x80\xFD[PZ\xF1\x15\x80\x15a;eW=`\0\x80>=`\0\xFD[PPPPP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xA9\x05\x9C\xBB\x83\x83`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x83`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x82\x81R` \x01\x92PPP` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a;\xE1W`\0\x80\xFD[PZ\xF1\x15\x80\x15a;\xF5W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a<\x0BW`\0\x80\xFD[Pa'+\x90PV[`\x01`\x01`\xA0\x1B\x03\x83\x160\x14\x15a<4Wa</\x84\x83\x83a<@V[a'+V[a'+\x84\x84\x84\x84aD\xD8V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`$\x83\x01R`D\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`d\x90\x92\x01\x83R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\xA9\x05\x9C\xBB`\xE0\x1B\x17\x81R\x92Q\x82Q`\0\x94\x85\x94\x93\x89\x16\x93\x92\x91\x82\x91\x90\x80\x83\x83[` \x83\x10a<\xBCW\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01a<\x9DV[`\x01\x83` \x03a\x01\0\n\x03\x80\x19\x82Q\x16\x81\x84Q\x16\x80\x82\x17\x85RPPPPPP\x90P\x01\x91PP`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a=\x1EW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a=#V[``\x91P[P\x91P\x91P\x81\x80\x15a=QWP\x80Q\x15\x80a=QWP\x80\x80` \x01\x90Q` \x81\x10\x15a=NW`\0\x80\xFD[PQ[a)\x11W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x02`$\x82\x01Ra\x14\xD5`\xF2\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\0a\x13\xC1\x83\x83aF(V[T\x90V[`\0\x80`\0\x83`\x02\x0B\x12a=\xAEW\x82`\x02\x0Ba=\xB6V[\x82`\x02\x0B`\0\x03[\x90Pb\r\x89\xE8\x81\x11\x15a=\xF4W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x01`$\x82\x01R`\x15`\xFA\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\0`\x01\x82\x16a>\x08W`\x01`\x80\x1Ba>\x1AV[o\xFF\xFC\xB93\xBDo\xAD7\xAA-\x16-\x1AY@\x01[p\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P`\x02\x82\x16\x15a>NWo\xFF\xF9rr7=A2Y\xA4i\x90X\x0E!:\x02`\x80\x1C[`\x04\x82\x16\x15a>mWo\xFF\xF2\xE5\x0F_ei2\xEF\x125|\xF3\xC7\xFD\xCC\x02`\x80\x1C[`\x08\x82\x16\x15a>\x8CWo\xFF\xE5\xCA\xCA~\x10\xE4\xE6\x1C6$\xEA\xA0\x94\x1C\xD0\x02`\x80\x1C[`\x10\x82\x16\x15a>\xABWo\xFF\xCB\x98C\xD6\x0FaY\xC9\xDBX\x83\\\x92fD\x02`\x80\x1C[` \x82\x16\x15a>\xCAWo\xFF\x97;A\xFA\x98\xC0\x81G.h\x96\xDF\xB2T\xC0\x02`\x80\x1C[`@\x82\x16\x15a>\xE9Wo\xFF.\xA1df\xC9j8C\xECx\xB3&\xB5(a\x02`\x80\x1C[`\x80\x82\x16\x15a?\x08Wo\xFE]\xEE\x04j\x99\xA2\xA8\x11\xC4a\xF1\x96\x9C0S\x02`\x80\x1C[a\x01\0\x82\x16\x15a?(Wo\xFC\xBE\x86\xC7\x90\n\x88\xAE\xDC\xFF\xC8;G\x9A\xA3\xA4\x02`\x80\x1C[a\x02\0\x82\x16\x15a?HWo\xF9\x87\xA7%:\xC4\x13\x17o+\x07L\xF7\x81^T\x02`\x80\x1C[a\x04\0\x82\x16\x15a?hWo\xF39+\x08\"\xB7\0\x05\x94\x0Cz9\x8EKp\xF3\x02`\x80\x1C[a\x08\0\x82\x16\x15a?\x88Wo\xE7\x15\x94u\xA2\xC2\x9BtC\xB2\x9C\x7F\xA6\xE8\x89\xD9\x02`\x80\x1C[a\x10\0\x82\x16\x15a?\xA8Wo\xD0\x97\xF3\xBD\xFD \"\xB8\x84Z\xD8\xF7\x92\xAAX%\x02`\x80\x1C[a \0\x82\x16\x15a?\xC8Wo\xA9\xF7FF-\x87\x0F\xDF\x8Ae\xDC\x1F\x90\xE0a\xE5\x02`\x80\x1C[a@\0\x82\x16\x15a?\xE8Wop\xD8i\xA1V\xD2\xA1\xB8\x90\xBB=\xF6+\xAF2\xF7\x02`\x80\x1C[a\x80\0\x82\x16\x15a@\x08Wo1\xBE\x13_\x97\xD0\x8F\xD9\x81#\x15\x05T/\xCF\xA6\x02`\x80\x1C[b\x01\0\0\x82\x16\x15a@)Wo\t\xAAP\x8B[z\x84\xE1\xC6w\xDET\xF3\xE9\x9B\xC9\x02`\x80\x1C[b\x02\0\0\x82\x16\x15a@IWn]j\xF8\xDE\xDB\x81\x19f\x99\xC3)\"^\xE6\x04\x02`\x80\x1C[b\x04\0\0\x82\x16\x15a@hWm\"\x16\xE5\x84\xF5\xFA\x1E\xA9&\x04\x1B\xED\xFE\x98\x02`\x80\x1C[b\x08\0\0\x82\x16\x15a@\x85Wk\x04\x8A\x17\x03\x91\xF7\xDCBDN\x8F\xA2\x02`\x80\x1C[`\0\x84`\x02\x0B\x13\x15a@\xA0W\x80`\0\x19\x81a@\x9CW\xFE[\x04\x90P[d\x01\0\0\0\0\x81\x06\x15a@\xB4W`\x01a@\xB7V[`\0[`\xFF\x16` \x82\x90\x1C\x01\x92PPP\x91\x90PV[`\0\x83`\x01`\x01`\xA0\x1B\x03\x16\x85`\x01`\x01`\xA0\x1B\x03\x16\x11\x15a@\xE9W\x92\x93\x92[\x84`\x01`\x01`\xA0\x1B\x03\x16\x86`\x01`\x01`\xA0\x1B\x03\x16\x11aA\x14WaA\r\x85\x85\x85aF@V[\x90PaA\x84V[\x83`\x01`\x01`\xA0\x1B\x03\x16\x86`\x01`\x01`\xA0\x1B\x03\x16\x10\x15aAvW`\0aA;\x87\x86\x86aF@V[\x90P`\0aAJ\x87\x89\x86aF\xA3V[\x90P\x80`\x01`\x01`\x80\x1B\x03\x16\x82`\x01`\x01`\x80\x1B\x03\x16\x10aAkW\x80aAmV[\x81[\x92PPPaA\x84V[aA\x81\x85\x85\x84aF\xA3V[\x90P[\x95\x94PPPPPV[`\0a\x13\xC1\x83\x83aF\xE0V[`\0a\x13\xC1\x83\x83aG\xA6V[`\0a\x10\x1A\x84\x84`\x01`\x01`\xA0\x1B\x03\x85\x16aG\xF0V[\x81T`\0\x90\x82\x10aA\xFDW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`\"\x81R` \x01\x80aW\x99`\"\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[\x82`\0\x01\x82\x81T\x81\x10aB\x0CW\xFE[\x90`\0R` `\0 \x01T\x90P\x92\x91PPV[`\0a\x13\xC1\x83\x83aH\x87V[\x81T`\0\x90\x81\x90\x83\x10aBoW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`\"\x81R` \x01\x80aY\x15`\"\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[`\0\x84`\0\x01\x84\x81T\x81\x10aB\x80W\xFE[\x90`\0R` `\0 \x90`\x02\x02\x01\x90P\x80`\0\x01T\x81`\x01\x01T\x92P\x92PP\x92P\x92\x90PV[`\0\x82\x81R`\x01\x84\x01` R`@\x81 T\x82\x81aCAW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R\x83\x81\x81Q\x81R` \x01\x91P\x80Q\x90` \x01\x90\x80\x83\x83`\0[\x83\x81\x10\x15aC\x06W\x81\x81\x01Q\x83\x82\x01R` \x01aB\xEEV[PPPP\x90P\x90\x81\x01\x90`\x1F\x16\x80\x15aC3W\x80\x82\x03\x80Q`\x01\x83` \x03a\x01\0\n\x03\x19\x16\x81R` \x01\x91P[P\x92PPP`@Q\x80\x91\x03\x90\xFD[P\x84`\0\x01`\x01\x82\x03\x81T\x81\x10aCTW\xFE[\x90`\0R` `\0 \x90`\x02\x02\x01`\x01\x01T\x91PP\x93\x92PPPV[`\0aC\x84\x84`\x01`\x01`\xA0\x1B\x03\x16a8%V[aC\x90WP`\x01a\x10\x1AV[`\0aD\x9Ec\n\x85\xBD\x01`\xE1\x1BaC\xA5a/hV[\x88\x87\x87`@Q`$\x01\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x84`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83\x81R` \x01\x80` \x01\x82\x81\x03\x82R\x83\x81\x81Q\x81R` \x01\x91P\x80Q\x90` \x01\x90\x80\x83\x83`\0[\x83\x81\x10\x15aD\x0CW\x81\x81\x01Q\x83\x82\x01R` \x01aC\xF4V[PPPP\x90P\x90\x81\x01\x90`\x1F\x16\x80\x15aD9W\x80\x82\x03\x80Q`\x01\x83` \x03a\x01\0\n\x03\x19\x16\x81R` \x01\x91P[P\x95PPPPPP`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90`\x01`\x01`\xE0\x1B\x03\x19\x16` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x83\x81\x83\x16\x17\x83RPPPP`@Q\x80``\x01`@R\x80`2\x81R` \x01aW\xBB`2\x919`\x01`\x01`\xA0\x1B\x03\x88\x16\x91\x90aI[V[\x90P`\0\x81\x80` \x01\x90Q` \x81\x10\x15aD\xB7W`\0\x80\xFD[PQ`\x01`\x01`\xE0\x1B\x03\x19\x16c\n\x85\xBD\x01`\xE1\x1B\x14\x92PPP\x94\x93PPPPV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`$\x83\x01R\x84\x81\x16`D\x83\x01R`d\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`\x84\x90\x92\x01\x83R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c#\xB8r\xDD`\xE0\x1B\x17\x81R\x92Q\x82Q`\0\x94\x85\x94\x93\x8A\x16\x93\x92\x91\x82\x91\x90\x80\x83\x83[` \x83\x10aE\\W\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01aE=V[`\x01\x83` \x03a\x01\0\n\x03\x80\x19\x82Q\x16\x81\x84Q\x16\x80\x82\x17\x85RPPPPPP\x90P\x01\x91PP`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14aE\xBEW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>aE\xC3V[``\x91P[P\x91P\x91P\x81\x80\x15aE\xF1WP\x80Q\x15\x80aE\xF1WP\x80\x80` \x01\x90Q` \x81\x10\x15aE\xEEW`\0\x80\xFD[PQ[a%\x8BW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x03`$\x82\x01Rb)\xAA#`\xE9\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\0\x90\x81R`\x01\x91\x90\x91\x01` R`@\x90 T\x15\x15\x90V[`\0\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x11\x15aF`W\x91\x92\x91[`\0aF\x83\x85`\x01`\x01`\xA0\x1B\x03\x16\x85`\x01`\x01`\xA0\x1B\x03\x16`\x01``\x1Ba1\xB8V[\x90PaA\x84aF\x9E\x84\x83\x88\x88\x03`\x01`\x01`\xA0\x1B\x03\x16a1\xB8V[aIjV[`\0\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x11\x15aF\xC3W\x91\x92\x91[a\x10\x1AaF\x9E\x83`\x01``\x1B\x87\x87\x03`\x01`\x01`\xA0\x1B\x03\x16a1\xB8V[`\0\x81\x81R`\x01\x83\x01` R`@\x81 T\x80\x15aG\x9CW\x83T`\0\x19\x80\x83\x01\x91\x90\x81\x01\x90`\0\x90\x87\x90\x83\x90\x81\x10aG\x13W\xFE[\x90`\0R` `\0 \x01T\x90P\x80\x87`\0\x01\x84\x81T\x81\x10aG0W\xFE[`\0\x91\x82R` \x80\x83 \x90\x91\x01\x92\x90\x92U\x82\x81R`\x01\x89\x81\x01\x90\x92R`@\x90 \x90\x84\x01\x90U\x86T\x87\x90\x80aG`W\xFE[`\x01\x90\x03\x81\x81\x90`\0R` `\0 \x01`\0\x90U\x90U\x86`\x01\x01`\0\x87\x81R` \x01\x90\x81R` \x01`\0 `\0\x90U`\x01\x94PPPPPa\x13\xC4V[`\0\x91PPa\x13\xC4V[`\0aG\xB2\x83\x83aF(V[aG\xE8WP\x81T`\x01\x81\x81\x01\x84U`\0\x84\x81R` \x80\x82 \x90\x93\x01\x84\x90U\x84T\x84\x82R\x82\x86\x01\x90\x93R`@\x90 \x91\x90\x91Ua\x13\xC4V[P`\0a\x13\xC4V[`\0\x82\x81R`\x01\x84\x01` R`@\x81 T\x80aHUWPP`@\x80Q\x80\x82\x01\x82R\x83\x81R` \x80\x82\x01\x84\x81R\x86T`\x01\x81\x81\x01\x89U`\0\x89\x81R\x84\x81 \x95Q`\x02\x90\x93\x02\x90\x95\x01\x91\x82U\x91Q\x90\x82\x01U\x86T\x86\x84R\x81\x88\x01\x90\x92R\x92\x90\x91 Ua1\xB1V[\x82\x85`\0\x01`\x01\x83\x03\x81T\x81\x10aHhW\xFE[\x90`\0R` `\0 \x90`\x02\x02\x01`\x01\x01\x81\x90UP`\0\x91PPa1\xB1V[`\0\x81\x81R`\x01\x83\x01` R`@\x81 T\x80\x15aG\x9CW\x83T`\0\x19\x80\x83\x01\x91\x90\x81\x01\x90`\0\x90\x87\x90\x83\x90\x81\x10aH\xBAW\xFE[\x90`\0R` `\0 \x90`\x02\x02\x01\x90P\x80\x87`\0\x01\x84\x81T\x81\x10aH\xDAW\xFE[`\0\x91\x82R` \x80\x83 \x84T`\x02\x90\x93\x02\x01\x91\x82U`\x01\x93\x84\x01T\x91\x84\x01\x91\x90\x91U\x83T\x82R\x89\x83\x01\x90R`@\x90 \x90\x84\x01\x90U\x86T\x87\x90\x80aI\x19W\xFE[`\0\x82\x81R` \x80\x82 `\x02`\0\x19\x90\x94\x01\x93\x84\x02\x01\x82\x81U`\x01\x90\x81\x01\x83\x90U\x92\x90\x93U\x88\x81R\x89\x82\x01\x90\x92R`@\x82 \x91\x90\x91U\x94Pa\x13\xC4\x93PPPPV[``a\x10\x1A\x84\x84`\0\x85aI\x80V[\x80`\x01`\x01`\x80\x1B\x03\x81\x16\x81\x14a\x07\x10W`\0\x80\xFD[``\x82G\x10\x15aI\xC1W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`&\x81R` \x01\x80aX8`&\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[aI\xCA\x85a8%V[aJ\x1BW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\0\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@Q\x80\x82\x80Q\x90` \x01\x90\x80\x83\x83[` \x83\x10aJYW\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01aJ:V[`\x01\x83` \x03a\x01\0\n\x03\x80\x19\x82Q\x16\x81\x84Q\x16\x80\x82\x17\x85RPPPPPP\x90P\x01\x91PP`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14aJ\xBBW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>aJ\xC0V[``\x91P[P\x91P\x91PaJ\xD0\x82\x82\x86aJ\xDBV[\x97\x96PPPPPPPV[``\x83\x15aJ\xEAWP\x81a1\xB1V[\x82Q\x15aJ\xFAW\x82Q\x80\x84` \x01\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x81R\x84Q`$\x84\x01R\x84Q\x85\x93\x91\x92\x83\x92`D\x01\x91\x90\x85\x01\x90\x80\x83\x83`\0\x83\x15aC\x06W\x81\x81\x01Q\x83\x82\x01R` \x01aB\xEEV[P\x80T`\x01\x81`\x01\x16\x15a\x01\0\x02\x03\x16`\x02\x90\x04`\0\x82U\x80`\x1F\x10aKgWPaK\x85V[`\x1F\x01` \x90\x04\x90`\0R` `\0 \x90\x81\x01\x90aK\x85\x91\x90aK\x88V[PV[[\x80\x82\x11\x15aK\x9DW`\0\x81U`\x01\x01aK\x89V[P\x90V[\x805a\x07\x10\x81aWBV[\x80Qa\xFF\xFF\x81\x16\x81\x14a\x07\x10W`\0\x80\xFD[\x805b\xFF\xFF\xFF\x81\x16\x81\x14a\x07\x10W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aK\xE2W\x80\x81\xFD[\x815a1\xB1\x81aWBV[`\0\x80`@\x83\x85\x03\x12\x15aK\xFFW\x80\x81\xFD[\x825aL\n\x81aWBV[\x91P` \x83\x015aL\x1A\x81aWBV[\x80\x91PP\x92P\x92\x90PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15aL:W\x81\x82\xFD[\x845aLE\x81aWBV[\x93P` \x85\x015aLU\x81aWBV[\x92PaLc`@\x86\x01aK\xBEV[\x91P``\x85\x015aLs\x81aWBV[\x93\x96\x92\x95P\x90\x93PPV[`\0\x80`\0``\x84\x86\x03\x12\x15aL\x92W\x80\x81\xFD[\x835aL\x9D\x81aWBV[\x92P` \x84\x015aL\xAD\x81aWBV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15aL\xD3W\x81\x82\xFD[\x845aL\xDE\x81aWBV[\x93P` \x85\x015aL\xEE\x81aWBV[\x92P`@\x85\x015\x91P``\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aM\x10W\x81\x82\xFD[\x85\x01`\x1F\x81\x01\x87\x13aM W\x81\x82\xFD[\x805aM3aM.\x82aV\xF4V[aV\xD0V[\x81\x81R\x88` \x83\x85\x01\x01\x11\x15aMGW\x83\x84\xFD[\x81` \x84\x01` \x83\x017\x90\x81\x01` \x01\x92\x90\x92RP\x92\x95\x91\x94P\x92PV[`\0\x80`@\x83\x85\x03\x12\x15aMwW\x81\x82\xFD[\x825aM\x82\x81aWBV[\x91P` \x83\x015aL\x1A\x81aWWV[`\0\x80`@\x83\x85\x03\x12\x15aM\xA4W\x81\x82\xFD[\x825aM\xAF\x81aWBV[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15aM\xD1W\x80\x81\xFD[\x835aM\xDC\x81aWBV[\x92P` \x84\x015\x91P`@\x84\x015aM\xF3\x81aWBV[\x80\x91PP\x92P\x92P\x92V[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15aN\x16W\x83\x84\xFD[\x865aN!\x81aWBV[\x95P` \x87\x015\x94P`@\x87\x015\x93P``\x87\x015aN?\x81aW\x89V[\x95\x98\x94\x97P\x92\x95`\x80\x81\x015\x94`\xA0\x90\x91\x015\x93P\x91PPV[`\0\x80` \x83\x85\x03\x12\x15aNkW\x81\x82\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15aN\x82W\x83\x84\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12aN\x95W\x83\x84\xFD[\x815\x81\x81\x11\x15aN\xA3W\x84\x85\xFD[\x86` \x80\x83\x02\x85\x01\x01\x11\x15aN\xB6W\x84\x85\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[`\0` \x82\x84\x03\x12\x15aN\xD9W\x80\x81\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a1\xB1W\x81\x82\xFD[`\0` \x82\x84\x03\x12\x15aO\x01W\x80\x81\xFD[\x815a1\xB1\x81aWeV[`\0` \x82\x84\x03\x12\x15aO\x1DW\x80\x81\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aO3W\x81\x82\xFD[\x82\x01`\x1F\x81\x01\x84\x13aOCW\x81\x82\xFD[\x80QaOQaM.\x82aV\xF4V[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15aOeW\x83\x84\xFD[aA\x84\x82` \x83\x01` \x86\x01aW\x16V[`\0`\x80\x82\x84\x03\x12\x15aO\x87W\x80\x81\xFD[P\x91\x90PV[`\0`\xA0\x82\x84\x03\x12\x15aO\x87W\x80\x81\xFD[`\0`\xC0\x82\x84\x03\x12\x15aO\x87W\x80\x81\xFD[`\0\x81\x83\x03`\x80\x81\x12\x15aO\xC1W\x81\x82\xFD[`@Q`@\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x82\x10\x81\x83\x11\x17\x15aO\xDFW\xFE[\x81`@R``\x84\x12\x15aO\xF0W\x84\x85\xFD[`\xA0\x83\x01\x93P\x81\x84\x10\x81\x85\x11\x17\x15aP\x04W\xFE[P\x82`@R\x845\x92PaP\x16\x83aWBV[\x91\x82R` \x84\x015\x91aP(\x83aWBV[\x82``\x83\x01RaP:`@\x86\x01aK\xBEV[`\x80\x83\x01R\x81RaPM``\x85\x01aK\xA1V[` \x82\x01R\x94\x93PPPPV[`\0a\x01`\x82\x84\x03\x12\x15aO\x87W\x80\x81\xFD[`\0` \x82\x84\x03\x12\x15aP}W\x80\x81\xFD[\x815a1\xB1\x81aWtV[`\0\x80`@\x83\x85\x03\x12\x15aP\x9AW\x81\x82\xFD[\x82QaP\xA5\x81aWtV[` \x84\x01Q\x90\x92PaL\x1A\x81aWtV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15aP\xCDW\x82\x83\xFD[\x85QaP\xD8\x81aWtV[\x80\x95PP` \x86\x01Q\x93P`@\x86\x01Q\x92P``\x86\x01QaP\xF8\x81aWtV[`\x80\x87\x01Q\x90\x92PaQ\t\x81aWtV[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[`\0\x80`\0\x80`\0\x80`\0`\xE0\x88\x8A\x03\x12\x15aQ1W\x84\x85\xFD[\x87QaQ<\x81aWBV[` \x89\x01Q\x90\x97PaQM\x81aWeV[\x95PaQ[`@\x89\x01aK\xACV[\x94PaQi``\x89\x01aK\xACV[\x93PaQw`\x80\x89\x01aK\xACV[\x92P`\xA0\x88\x01QaQ\x87\x81aW\x89V[`\xC0\x89\x01Q\x90\x92PaQ\x98\x81aWWV[\x80\x91PP\x92\x95\x98\x91\x94\x97P\x92\x95PV[`\0` \x82\x84\x03\x12\x15aQ\xB9W\x80\x81\xFD[a\x13\xC1\x82aK\xBEV[`\0` \x82\x84\x03\x12\x15aQ\xD3W\x80\x81\xFD[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15aQ\xECW\x81\x82\xFD[\x825\x91P` \x83\x015aL\x1A\x81aWBV[`\0\x80`@\x83\x85\x03\x12\x15aR\x10W\x81\x82\xFD[PP\x80Q` \x90\x91\x01Q\x90\x92\x90\x91PV[`\0\x80`\0\x80``\x85\x87\x03\x12\x15aR6W\x81\x82\xFD[\x845\x93P` \x85\x015\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15aR[W\x83\x84\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12aRnW\x83\x84\xFD[\x815\x81\x81\x11\x15aR|W\x84\x85\xFD[\x88` \x82\x85\x01\x01\x11\x15aR\x8DW\x84\x85\xFD[\x95\x98\x94\x97PP` \x01\x94PPPV[`\0\x81Q\x80\x84RaR\xB4\x81` \x86\x01` \x86\x01aW\x16V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\x02\x0B\x90RV[`\x01`\x01`\x80\x1B\x03\x16\x90RV[`\0\x82\x84\x837\x91\x01\x90\x81R\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x81R` \x01\x90V[`\0`\x01\x80`\xA0\x1B\x03\x87\x16\x82R\x85`\x02\x0B` \x83\x01R\x84`\x02\x0B`@\x83\x01R`\x01`\x01`\x80\x1B\x03\x84\x16``\x83\x01R`\xA0`\x80\x83\x01RaJ\xD0`\xA0\x83\x01\x84aR\x9CV[`\x01`\x01`\xA0\x1B\x03\x95\x90\x95\x16\x85R`\x02\x93\x84\x0B` \x86\x01R\x91\x90\x92\x0B`@\x84\x01R`\x01`\x01`\x80\x1B\x03\x91\x82\x16``\x84\x01R\x16`\x80\x82\x01R`\xA0\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x83R`\x01`\x01`\x80\x1B\x03\x91\x82\x16` \x84\x01R\x16`@\x82\x01R``\x01\x90V[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x86\x01\x91P`@\x84\x82\x02\x87\x01\x01\x92P\x83\x87\x01\x85[\x82\x81\x10\x15aS\xFCW`?\x19\x88\x86\x03\x01\x84RaS\xEA\x85\x83QaR\x9CV[\x94P\x92\x85\x01\x92\x90\x85\x01\x90`\x01\x01aS\xCEV[P\x92\x97\x96PPPPPPPV[\x90\x15\x15\x81R` \x01\x90V[\x90\x81R` \x01\x90V[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x82R` \x82\x01R`@\x01\x90V[`\x02\x93\x84\x0B\x81R\x91\x90\x92\x0B` \x82\x01R`\x01`\x01`\x80\x1B\x03\x90\x91\x16`@\x82\x01R``\x01\x90V[`\0` \x82Ra\x13\xC1` \x83\x01\x84aR\x9CV[` \x80\x82R`\x0C\x90\x82\x01Rk\x13\x9B\xDD\x08\x18\\\x1C\x1C\x9B\xDD\x99Y`\xA2\x1B`@\x82\x01R``\x01\x90V[` \x80\x82R`,\x90\x82\x01R\x7FERC721: approved query for nonex`@\x82\x01Rk4\xB9\xBA2\xB7:\x10:7\xB5\xB2\xB7`\xA1\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`\x14\x90\x82\x01RsPrice slippage check``\x1B`@\x82\x01R``\x01\x90V[` \x80\x82R`\x10\x90\x82\x01Ro\x12[\x9D\x98[\x1AY\x08\x1D\x1B\xDA\xD9[\x88\x12Q`\x82\x1B`@\x82\x01R``\x01\x90V[` \x80\x82R`\x0B\x90\x82\x01Rj\x13\x9B\xDD\x08\x18\xDB\x19X\\\x99Y`\xAA\x1B`@\x82\x01R``\x01\x90V[\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83R` \x80\x83\x01Q\x82\x16\x81\x85\x01R`@\x92\x83\x01Qb\xFF\xFF\xFF\x16\x92\x84\x01\x92\x90\x92R\x92\x01Q\x90\x91\x16``\x82\x01R`\x80\x01\x90V[`\x01`\x01`\x80\x1B\x03\x93\x90\x93\x16\x83R` \x83\x01\x91\x90\x91R`@\x82\x01R``\x01\x90V[\x93\x84R`\x01`\x01`\x80\x1B\x03\x92\x90\x92\x16` \x84\x01R`@\x83\x01R``\x82\x01R`\x80\x01\x90V[\x91\x82R` \x82\x01R`@\x01\x90V[`\x01`\x01``\x1B\x03\x8D\x16\x81R`\x01`\x01`\xA0\x1B\x03\x8C\x81\x16` \x83\x01R\x8B\x81\x16`@\x83\x01R\x8A\x16``\x82\x01Rb\xFF\xFF\xFF\x89\x16`\x80\x82\x01R`\x02\x88\x90\x0B`\xA0\x82\x01Ra\x01\x80\x81\x01aVC`\xC0\x83\x01\x89aR\xC8V[aVP`\xE0\x83\x01\x88aR\xCFV[\x85a\x01\0\x83\x01R\x84a\x01 \x83\x01RaVla\x01@\x83\x01\x85aR\xCFV[aVza\x01`\x83\x01\x84aR\xCFV[\x9D\x9CPPPPPPPPPPPPPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aV\xA1W\x82\x83\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aV\xBBW\x82\x83\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a7\xDCW`\0\x80\xFD[`@Q\x81\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15aV\xECW\xFE[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aW\x08W\xFE[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0[\x83\x81\x10\x15aW1W\x81\x81\x01Q\x83\x82\x01R` \x01aW\x19V[\x83\x81\x11\x15a'+WPP`\0\x91\x01RV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14aK\x85W`\0\x80\xFD[\x80\x15\x15\x81\x14aK\x85W`\0\x80\xFD[\x80`\x02\x0B\x81\x14aK\x85W`\0\x80\xFD[`\x01`\x01`\x80\x1B\x03\x81\x16\x81\x14aK\x85W`\0\x80\xFD[`\xFF\x81\x16\x81\x14aK\x85W`\0\x80\xFD\xFEEnumerableSet: index out of boundsERC721: transfer to non ERC721Receiver implementerERC721Permit: approval to current ownerERC721: transfer to the zero addressAddress: insufficient balance for callERC721: operator query for nonexistent tokenERC721: approve caller is not owner nor approved for allERC721: balance query for the zero addressERC721: owner query for nonexistent tokenEnumerableMap: index out of boundsERC721: transfer of token that is not ownERC721: approval to current ownerERC721: transfer caller is not owner nor approved\xA2dipfsX\"\x12 K\x9DR\xEF)\xEAKT\x99\xCC\x04\xD2\xA7\x9A\xD4\xA4\x9B\x17\x1E\xE4\x97\xEA\xF2\x1B IU\x82\x9A\xF5j\xAAdsolcC\0\x07\x06\x003";
    /// The bytecode of the contract.
    pub static MOCKTIMENONFUNGIBLEPOSITIONMANAGER_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\x024W`\x005`\xE0\x1C\x80cOl\xCC\xE7\x11a\x01.W\x80c\xAC\x96P\xD8\x11a\0\xABW\x80c\xD3Hy\x97\x11a\0oW\x80c\xD3Hy\x97\x14a\x06yW\x80c\xDF*\xB5\xBB\x14a\x06\x99W\x80c\xE9\x85\xE9\xC5\x14a\x06\xACW\x80c\xF3\x99\\g\x14a\x06\xCCW\x80c\xFCoxe\x14a\x06\xDFWa\x02\xA4V[\x80c\xAC\x96P\xD8\x14a\x05\xF1W\x80c\xB8\x8DO\xDE\x14a\x06\x11W\x80c\xC2\xE3\x14\n\x14a\x061W\x80c\xC4Z\x01U\x14a\x06DW\x80c\xC8{V\xDD\x14a\x06YWa\x02\xA4V[\x80c\x881dV\x11a\0\xF2W\x80c\x881dV\x14a\x05NW\x80c\x95\xD8\x9BA\x14a\x05qW\x80c\x99\xFB\xAB\x88\x14a\x05\x86W\x80c\xA2,\xB4e\x14a\x05\xBEW\x80c\xA4\xA7\x8F\x0C\x14a\x05\xDEWa\x02\xA4V[\x80cOl\xCC\xE7\x14a\x04\xC6W\x80ccR!\x1E\x14a\x04\xE6W\x80cl\x03`\xEB\x14a\x05\x06W\x80cp\xA0\x821\x14a\x05\x1BW\x80cz\xC2\xFF{\x14a\x05;Wa\x02\xA4V[\x80c#\xB8r\xDD\x11a\x01\xBCW\x80cB\x84.\x0E\x11a\x01\x80W\x80cB\x84.\x0E\x14a\x04XW\x80cB\x96lh\x14a\x04xW\x80cFY\xA4\x94\x14a\x04\x8BW\x80cI@K|\x14a\x04\x9EW\x80cJ\xA4\xA4\xFC\x14a\x04\xB1Wa\x02\xA4V[\x80c#\xB8r\xDD\x14a\x03\xCEW\x80c/t\\Y\x14a\x03\xEEW\x80c0\xAD\xF8\x1F\x14a\x04\x0EW\x80c6D\xE5\x15\x14a\x04#W\x80c;\xEB&\xC4\x14a\x048Wa\x02\xA4V[\x80c\x0CI\xCC\xBE\x11a\x02\x03W\x80c\x0CI\xCC\xBE\x14a\x03NW\x80c\x12!\x0E\x8A\x14a\x03oW\x80c\x13\xEA\xD5b\x14a\x03wW\x80c\x18\x16\r\xDD\x14a\x03\x8AW\x80c!\x9F]\x17\x14a\x03\xACWa\x02\xA4V[\x80c\x01\xFF\xC9\xA7\x14a\x02\xA9W\x80c\x06\xFD\xDE\x03\x14a\x02\xDFW\x80c\x08\x18\x12\xFC\x14a\x03\x01W\x80c\t^\xA7\xB3\x14a\x03.Wa\x02\xA4V[6a\x02\xA4W3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x02\xA2W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01RhNot WETH9`\xB8\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\0[`\0\x80\xFD[4\x80\x15a\x02\xB5W`\0\x80\xFD[Pa\x02\xC9a\x02\xC46`\x04aN\xC8V[a\x06\xF2V[`@Qa\x02\xD6\x91\x90aT\tV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\xEBW`\0\x80\xFD[Pa\x02\xF4a\x07\x15V[`@Qa\x02\xD6\x91\x90aT\\V[4\x80\x15a\x03\rW`\0\x80\xFD[Pa\x03!a\x03\x1C6`\x04aQ\xC2V[a\x07\xABV[`@Qa\x02\xD6\x91\x90aR\xECV[4\x80\x15a\x03:W`\0\x80\xFD[Pa\x02\xA2a\x03I6`\x04aM\x92V[a\x07\xFEV[a\x03aa\x03\\6`\x04aO\x8DV[a\x08\xD4V[`@Qa\x02\xD6\x92\x91\x90aU\xE3V[a\x02\xA2a\r\x1CV[a\x03!a\x03\x856`\x04aL%V[a\r.V[4\x80\x15a\x03\x96W`\0\x80\xFD[Pa\x03\x9Fa\x10\"V[`@Qa\x02\xD6\x91\x90aT\x14V[a\x03\xBFa\x03\xBA6`\x04aO\x9EV[a\x103V[`@Qa\x02\xD6\x93\x92\x91\x90aU\x9EV[4\x80\x15a\x03\xDAW`\0\x80\xFD[Pa\x02\xA2a\x03\xE96`\x04aL~V[a\x13HV[4\x80\x15a\x03\xFAW`\0\x80\xFD[Pa\x03\x9Fa\x04\t6`\x04aM\x92V[a\x13\x9FV[4\x80\x15a\x04\x1AW`\0\x80\xFD[Pa\x03\x9Fa\x13\xCAV[4\x80\x15a\x04/W`\0\x80\xFD[Pa\x03\x9Fa\x13\xEEV[4\x80\x15a\x04DW`\0\x80\xFD[Pa\x02\xA2a\x04S6`\x04aQ\xC2V[a\x14\xACV[4\x80\x15a\x04dW`\0\x80\xFD[Pa\x02\xA2a\x04s6`\x04aL~V[a\x14\xB1V[a\x02\xA2a\x04\x866`\x04aQ\xC2V[a\x14\xCCV[a\x02\xA2a\x04\x996`\x04aM\xFEV[a\x15\x9BV[a\x02\xA2a\x04\xAC6`\x04aQ\xDAV[a\x165V[4\x80\x15a\x04\xBDW`\0\x80\xFD[Pa\x03!a\x17\xAAV[4\x80\x15a\x04\xD2W`\0\x80\xFD[Pa\x03\x9Fa\x04\xE16`\x04aQ\xC2V[a\x17\xCEV[4\x80\x15a\x04\xF2W`\0\x80\xFD[Pa\x03!a\x05\x016`\x04aQ\xC2V[a\x17\xE4V[4\x80\x15a\x05\x12W`\0\x80\xFD[Pa\x02\xF4a\x18\x0CV[4\x80\x15a\x05'W`\0\x80\xFD[Pa\x03\x9Fa\x0566`\x04aK\xD1V[a\x18\x11V[a\x02\xA2a\x05I6`\x04aM\xFEV[a\x18yV[a\x05aa\x05\\6`\x04aPZV[a\x1CkV[`@Qa\x02\xD6\x94\x93\x92\x91\x90aU\xBFV[4\x80\x15a\x05}W`\0\x80\xFD[Pa\x02\xF4a!\x83V[4\x80\x15a\x05\x92W`\0\x80\xFD[Pa\x05\xA6a\x05\xA16`\x04aQ\xC2V[a!\xE4V[`@Qa\x02\xD6\x9C\x9B\x9A\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90aU\xF1V[4\x80\x15a\x05\xCAW`\0\x80\xFD[Pa\x02\xA2a\x05\xD96`\x04aMeV[a#\xF5V[a\x02\xA2a\x05\xEC6`\x04aM\xFEV[a$\xFAV[a\x06\x04a\x05\xFF6`\x04aNYV[a%\x93V[`@Qa\x02\xD6\x91\x90aS\xA9V[4\x80\x15a\x06\x1DW`\0\x80\xFD[Pa\x02\xA2a\x06,6`\x04aL\xBEV[a&\xD3V[a\x02\xA2a\x06?6`\x04aM\xFEV[a'1V[4\x80\x15a\x06PW`\0\x80\xFD[Pa\x03!a'\xC0V[4\x80\x15a\x06eW`\0\x80\xFD[Pa\x02\xF4a\x06t6`\x04aQ\xC2V[a'\xE4V[4\x80\x15a\x06\x85W`\0\x80\xFD[Pa\x02\xA2a\x06\x946`\x04aR!V[a(\x9AV[a\x02\xA2a\x06\xA76`\x04aM\xBDV[a)\x18V[4\x80\x15a\x06\xB8W`\0\x80\xFD[Pa\x02\xC9a\x06\xC76`\x04aK\xEDV[a)\xF0V[a\x02\xA2a\x06\xDA6`\x04aM\xFEV[a*\x1EV[a\x03aa\x06\xED6`\x04aOvV[a*\x90V[`\x01`\x01`\xE0\x1B\x03\x19\x81\x16`\0\x90\x81R` \x81\x90R`@\x90 T`\xFF\x16[\x91\x90PV[`\x06\x80T`@\x80Q` `\x1F`\x02`\0\x19a\x01\0`\x01\x88\x16\x15\x02\x01\x90\x95\x16\x94\x90\x94\x04\x93\x84\x01\x81\x90\x04\x81\x02\x82\x01\x81\x01\x90\x92R\x82\x81R``\x93\x90\x92\x90\x91\x83\x01\x82\x82\x80\x15a\x07\xA1W\x80`\x1F\x10a\x07vWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x07\xA1V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x07\x84W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[`\0a\x07\xB6\x82a/[V[a\x07\xDBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xD2\x90aT\x95V[`@Q\x80\x91\x03\x90\xFD[P`\0\x90\x81R`\x0C` R`@\x90 T`\x01``\x1B\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\0a\x08\t\x82a\x17\xE4V[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a\x08\\W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`!\x81R` \x01\x80aY``!\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[\x80`\x01`\x01`\xA0\x1B\x03\x16a\x08na/hV[`\x01`\x01`\xA0\x1B\x03\x16\x14\x80a\x08\x8AWPa\x08\x8A\x81a\x06\xC7a/hV[a\x08\xC5W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`8\x81R` \x01\x80aX\x8A`8\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[a\x08\xCF\x83\x83a/lV[PPPV[`\0\x80\x825a\x08\xE33\x82a/\xE2V[a\x08\xFFW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xD2\x90aToV[\x83`\x80\x015\x80a\t\ra0~V[\x11\x15a\tVW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr\x15\x1C\x98[\x9C\xD8X\xDD\x1A[\xDB\x88\x1D\x1B\xDB\xC8\x1B\xDB\x19`j\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\0a\th`@\x87\x01` \x88\x01aPlV[`\x01`\x01`\x80\x1B\x03\x16\x11a\t{W`\0\x80\xFD[\x845`\0\x90\x81R`\x0C` \x90\x81R`@\x91\x82\x90 `\x01\x81\x01T\x90\x92`\x01`\x80\x1B\x90\x91\x04`\x01`\x01`\x80\x1B\x03\x16\x91a\t\xB6\x91\x89\x01\x90\x89\x01aPlV[`\x01`\x01`\x80\x1B\x03\x16\x81`\x01`\x01`\x80\x1B\x03\x16\x10\x15a\t\xD4W`\0\x80\xFD[`\x01\x82\x81\x01T`\x01`\x01`P\x1B\x03\x16`\0\x90\x81R`\x0B` \x90\x81R`@\x80\x83 \x81Q``\x81\x01\x83R\x81T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R\x91\x90\x95\x01T\x90\x81\x16\x92\x85\x01\x92\x90\x92R`\x01`\xA0\x1B\x90\x91\x04b\xFF\xFF\xFF\x16\x90\x83\x01Ra\nU\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83a0\x84V[`\x01\x85\x01T\x90\x91P`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\xA3A#\xA7\x90`\x01`P\x1B\x81\x04`\x02\x90\x81\x0B\x91`\x01`h\x1B\x90\x04\x90\x0Ba\n\x94`@\x8E\x01` \x8F\x01aPlV[`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\n\xB2\x93\x92\x91\x90aT6V[`@\x80Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\n\xCBW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\n\xDFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\x03\x91\x90aQ\xFEV[\x90\x98P\x96P`@\x89\x015\x88\x10\x80\x15\x90a\x0B WP\x88``\x015\x87\x10\x15[a\x0B<W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xD2\x90aT\xE1V[`\x01\x84\x01T`\0\x90a\x0Be\x900\x90`\x01`P\x1B\x81\x04`\x02\x90\x81\x0B\x91`\x01`h\x1B\x90\x04\x90\x0Ba1cV[\x90P`\0\x80\x83`\x01`\x01`\xA0\x1B\x03\x16cQN\xA4\xBF\x84`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0B\x96\x91\x90aT\x14V[`\xA0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0B\xAEW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x0B\xC2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xE6\x91\x90aP\xB6V[PP\x92P\x92PPa\x0C\x0B\x87`\x02\x01T\x83\x03\x87`\x01`\x01`\x80\x1B\x03\x16`\x01`\x80\x1Ba1\xB8V[`\x04\x88\x01\x80T`\x01`\x01`\x80\x1B\x03\x19\x81\x16\x92\x8E\x01`\x01`\x01`\x80\x1B\x03\x91\x82\x16\x01\x81\x16\x92\x90\x92\x17\x90U`\x03\x88\x01Ta\x0CL\x91\x90\x83\x03\x90\x88\x16`\x01`\x80\x1Ba1\xB8V[`\x04\x88\x01\x80T`\x01`\x01`\x80\x1B\x03\x80\x82\x16\x93\x8E\x01`\x01`\x80\x1B\x92\x83\x90\x04\x82\x16\x01\x16\x02\x91\x90\x91\x17\x90U`\x02\x87\x01\x82\x90U`\x03\x87\x01\x81\x90Ua\x0C\x92`@\x8D\x01` \x8E\x01aPlV[\x86\x03\x87`\x01\x01`\x10a\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP\x8B`\0\x015\x7F&\xF6\xA0H\xEE\x918\xF2\xC0\xCE&o2,\xB9\x92(\xE8\xD6\x19\xAE+\xFF0\xC6\x7F\x8D\xCF\x9D#w\xB4\x8D` \x01` \x81\x01\x90a\x0C\xF5\x91\x90aPlV[\x8D\x8D`@Qa\r\x06\x93\x92\x91\x90aU\x9EV[`@Q\x80\x91\x03\x90\xA2PPPPPPPPP\x91P\x91V[G\x15a\r,Wa\r,3Ga2gV[V[`\0\x83`\x01`\x01`\xA0\x1B\x03\x16\x85`\x01`\x01`\xA0\x1B\x03\x16\x10a\rNW`\0\x80\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x16\x98\xEE\x82\x86\x86\x86`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x84`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x82b\xFF\xFF\xFF\x16\x81R` \x01\x93PPPP` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\r\xD9W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\r\xEDW=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x0E\x03W`\0\x80\xFD[PQ\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a\x0F9W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xA1g\x12\x95\x86\x86\x86`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x84`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x82b\xFF\xFF\xFF\x16\x81R` \x01\x93PPPP` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0E\xA2W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0E\xB6W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x0E\xCCW`\0\x80\xFD[PQ`@\x80Qc\xF67s\x1D`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R\x91Q\x92\x93P\x90\x83\x16\x91c\xF67s\x1D\x91`$\x80\x82\x01\x92`\0\x92\x90\x91\x90\x82\x90\x03\x01\x81\x83\x87\x80;\x15\x80\x15a\x0F\x1CW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0F0W=`\0\x80>=`\0\xFD[PPPPa\x10\x1AV[`\0\x81`\x01`\x01`\xA0\x1B\x03\x16c8P\xC7\xBD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\xE0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0FtW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x0F\x88W=`\0\x80>=`\0\xFD[PPPP`@Q=`\xE0\x81\x10\x15a\x0F\x9EW`\0\x80\xFD[PQ\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a\x10\x18W\x81`\x01`\x01`\xA0\x1B\x03\x16c\xF67s\x1D\x84`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x82`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x91PP`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0F\xFFW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x10\x13W=`\0\x80>=`\0\xFD[PPPP[P[\x94\x93PPPPV[`\0a\x10.`\x02a3VV[\x90P\x90V[`\0\x80`\0\x83`\xA0\x015\x80a\x10Fa0~V[\x11\x15a\x10\x8FW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr\x15\x1C\x98[\x9C\xD8X\xDD\x1A[\xDB\x88\x1D\x1B\xDB\xC8\x1B\xDB\x19`j\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x845`\0\x90\x81R`\x0C` \x90\x81R`@\x80\x83 `\x01\x80\x82\x01T`\x01`\x01`P\x1B\x03\x81\x16\x86R`\x0B\x85R\x83\x86 \x84Q``\x80\x82\x01\x87R\x82T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83R\x92\x90\x94\x01T\x80\x83\x16\x82\x89\x01\x90\x81Rb\xFF\xFF\xFF`\x01`\xA0\x1B\x90\x92\x04\x82\x16\x83\x89\x01\x90\x81R\x88Qa\x01@\x81\x01\x8AR\x84Q\x86\x16\x81R\x91Q\x90\x94\x16\x81\x8A\x01R\x92Q\x16\x82\x87\x01R0\x82\x85\x01R`\x01`P\x1B\x83\x04`\x02\x90\x81\x0B\x81\x0B`\x80\x80\x85\x01\x91\x90\x91R`\x01`h\x1B\x90\x94\x04\x81\x0B\x90\x0B`\xA0\x83\x01R\x95\x8C\x015`\xC0\x82\x01R\x93\x8B\x015`\xE0\x85\x01R\x90\x8A\x015a\x01\0\x84\x01R\x89\x015a\x01 \x83\x01R\x92\x90a\x11q\x90a3aV[`\x01\x87\x01T\x93\x9AP\x91\x98P\x96P\x91P`\0\x90a\x11\xA4\x900\x90`\x01`P\x1B\x81\x04`\x02\x90\x81\x0B\x91`\x01`h\x1B\x90\x04\x90\x0Ba1cV[\x90P`\0\x80\x83`\x01`\x01`\xA0\x1B\x03\x16cQN\xA4\xBF\x84`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x11\xD5\x91\x90aT\x14V[`\xA0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x11\xEDW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x12\x01W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12%\x91\x90aP\xB6V[PP\x92P\x92PPa\x12a\x86`\x02\x01T\x83\x03\x87`\x01\x01`\x10\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\x80\x1B\x03\x16`\x01`\x01`\x80\x1B\x03\x16`\x01`\x80\x1Ba1\xB8V[`\x04\x87\x01\x80T`\x01`\x01`\x80\x1B\x03\x80\x82\x16\x90\x93\x01\x83\x16`\x01`\x01`\x80\x1B\x03\x19\x90\x91\x16\x17\x90U`\x03\x87\x01T`\x01\x88\x01Ta\x12\xA8\x92\x91\x84\x03\x91`\x01`\x80\x1B\x91\x82\x90\x04\x16\x90a1\xB8V[`\x04\x87\x01\x80T`\x01`\x01`\x80\x1B\x03`\x01`\x80\x1B\x80\x83\x04\x82\x16\x90\x94\x01\x81\x16\x84\x02\x91\x81\x16\x91\x90\x91\x17\x90\x91U`\x02\x88\x01\x84\x90U`\x03\x88\x01\x83\x90U`\x01\x88\x01\x80T\x83\x81\x04\x83\x16\x8E\x01\x83\x16\x90\x93\x02\x92\x90\x91\x16\x91\x90\x91\x17\x90U`@Q\x8B5\x90\x7F0g\x04\x8B\xEE\xE3\x1B%\xB2\xF1h\x1F\x88\xDA\xC88\xC8\xBB\xA3j\xF2[\xFB+|\xF7G:XG\xE3_\x90a\x132\x90\x8D\x90\x8D\x90\x8D\x90aU\x9EV[`@Q\x80\x91\x03\x90\xA2PPPPPPP\x91\x93\x90\x92PV[a\x13Ya\x13Sa/hV[\x82a/\xE2V[a\x13\x94W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`1\x81R` \x01\x80aY\x81`1\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[a\x08\xCF\x83\x83\x83a5\x9CV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x01` R`@\x81 a\x13\xC1\x90\x83a6\xE8V[\x90P[\x92\x91PPV[\x7FI\xEC\xF33\xE5\xB8\xC9\\@\xFD\xAF\xC9\\\x1A\xD16\xE8\x91J\x8F\xB5^\x9D\xC8\xBB\x01\xEA\xA8:-\xF9\xAD\x81V[`\0\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x14[a6\xF4V[0`@Q` \x01\x80\x86\x81R` \x01\x85\x81R` \x01\x84\x81R` \x01\x83\x81R` \x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x95PPPPPP`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[`\x0EUV[a\x08\xCF\x83\x83\x83`@Q\x80` \x01`@R\x80`\0\x81RPa&\xD3V[\x80a\x14\xD73\x82a/\xE2V[a\x14\xF3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xD2\x90aToV[`\0\x82\x81R`\x0C` R`@\x90 `\x01\x81\x01T`\x01`\x80\x1B\x90\x04`\x01`\x01`\x80\x1B\x03\x16\x15\x80\x15a\x15.WP`\x04\x81\x01T`\x01`\x01`\x80\x1B\x03\x16\x15[\x80\x15a\x15LWP`\x04\x81\x01T`\x01`\x80\x1B\x90\x04`\x01`\x01`\x80\x1B\x03\x16\x15[a\x15hW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xD2\x90aU9V[`\0\x83\x81R`\x0C` R`@\x81 \x81\x81U`\x01\x81\x01\x82\x90U`\x02\x81\x01\x82\x90U`\x03\x81\x01\x82\x90U`\x04\x01Ua\x08\xCF\x83a6\xF8V[`@\x80Qc#\xF2\xEB\xC3`\xE2\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x87\x90R`d\x81\x01\x86\x90R`\x01`\x84\x82\x01R`\xFF\x85\x16`\xA4\x82\x01R`\xC4\x81\x01\x84\x90R`\xE4\x81\x01\x83\x90R\x90Q`\x01`\x01`\xA0\x1B\x03\x88\x16\x91c\x8F\xCB\xAF\x0C\x91a\x01\x04\x80\x83\x01\x92`\0\x92\x91\x90\x82\x90\x03\x01\x81\x83\x87\x80;\x15\x80\x15a\x16\x15W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x16)W=`\0\x80>=`\0\xFD[PPPPPPPPPPV[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16cp\xA0\x8210`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x82`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x91PP` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x16\xA4W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x16\xB8W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x16\xCEW`\0\x80\xFD[PQ\x90P\x82\x81\x10\x15a\x17\x1CW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01RqInsufficient WETH9`p\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x80\x15a\x08\xCFW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c.\x1A}M\x82`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x82\x81R` \x01\x91PP`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x17\x88W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x17\x9CW=`\0\x80>=`\0\xFD[PPPPa\x08\xCF\x82\x82a2gV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0\x80a\x17\xDC`\x02\x84a7\xC5V[P\x93\x92PPPV[`\0a\x13\xC4\x82`@Q\x80``\x01`@R\x80`)\x81R` \x01aX\xEC`)\x919`\x02\x91\x90a7\xE3V[``\x90V[`\0`\x01`\x01`\xA0\x1B\x03\x82\x16a\x18XW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`*\x81R` \x01\x80aX\xC2`*\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x01` R`@\x90 a\x13\xC4\x90a3VV[\x83a\x18\x82a0~V[\x11\x15a\x18\xC6W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rm\x14\x19\\\x9BZ]\x08\x19^\x1C\x1A\\\x99Y`\x92\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\0a\x18\xD0a\x13\xEEV[\x7FI\xEC\xF33\xE5\xB8\xC9\\@\xFD\xAF\xC9\\\x1A\xD16\xE8\x91J\x8F\xB5^\x9D\xC8\xBB\x01\xEA\xA8:-\xF9\xAD\x88\x88a\x18\xFC\x81a7\xF0V[`@\x80Q` \x80\x82\x01\x96\x90\x96R`\x01`\x01`\xA0\x1B\x03\x90\x94\x16\x84\x82\x01R``\x84\x01\x92\x90\x92R`\x80\x83\x01R`\xA0\x80\x83\x01\x8A\x90R\x81Q\x80\x84\x03\x90\x91\x01\x81R`\xC0\x83\x01\x82R\x80Q\x90\x84\x01 a\x19\x01`\xF0\x1B`\xE0\x84\x01R`\xE2\x83\x01\x94\x90\x94Ra\x01\x02\x80\x83\x01\x94\x90\x94R\x80Q\x80\x83\x03\x90\x94\x01\x84Ra\x01\"\x90\x91\x01\x90R\x81Q\x91\x01 \x90P`\0a\x19\x84\x87a\x17\xE4V[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16\x88`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a\x19\xD7W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`'\x81R` \x01\x80aW\xED`'\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[a\x19\xE0\x81a8%V[\x15a\x1BHW`@\x80Q` \x80\x82\x01\x87\x90R\x81\x83\x01\x86\x90R`\x01`\x01`\xF8\x1B\x03\x19`\xF8\x89\x90\x1B\x16``\x83\x01R\x82Q`A\x81\x84\x03\x01\x81R`a\x83\x01\x80\x85Rc\x0B\x13]?`\xE1\x1B\x90R`e\x83\x01\x86\x81R`\x85\x84\x01\x94\x85R\x81Q`\xA5\x85\x01R\x81Q`\x01`\x01`\xA0\x1B\x03\x87\x16\x95c\x16&\xBA~\x95\x89\x95\x91\x92`\xC5\x90\x91\x01\x91\x85\x01\x90\x80\x83\x83`\0[\x83\x81\x10\x15a\x1AyW\x81\x81\x01Q\x83\x82\x01R` \x01a\x1AaV[PPPP\x90P\x90\x81\x01\x90`\x1F\x16\x80\x15a\x1A\xA6W\x80\x82\x03\x80Q`\x01\x83` \x03a\x01\0\n\x03\x19\x16\x81R` \x01\x91P[P\x93PPPP` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1A\xC4W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x1A\xD8W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x1A\xEEW`\0\x80\xFD[PQ`\x01`\x01`\xE0\x1B\x03\x19\x16c\x0B\x13]?`\xE1\x1B\x14a\x1BCW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01Rk\x15[\x98]]\x1A\x1B\xDC\x9A^\x99Y`\xA2\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[a\x1CWV[`\0`\x01\x83\x87\x87\x87`@Q`\0\x81R` \x01`@R`@Q\x80\x85\x81R` \x01\x84`\xFF\x16\x81R` \x01\x83\x81R` \x01\x82\x81R` \x01\x94PPPPP` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x1B\xA4W=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a\x1C\0W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01RpInvalid signature`x\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x81`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1CUW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01Rk\x15[\x98]]\x1A\x1B\xDC\x9A^\x99Y`\xA2\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[P[a\x1Ca\x88\x88a/lV[PPPPPPPPV[`\0\x80`\0\x80\x84a\x01@\x015\x80a\x1C\x80a0~V[\x11\x15a\x1C\xC9W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr\x15\x1C\x98[\x9C\xD8X\xDD\x1A[\xDB\x88\x1D\x1B\xDB\xC8\x1B\xDB\x19`j\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`@\x80Qa\x01@\x81\x01\x90\x91R`\0\x90a\x1D\x95\x90\x80a\x1C\xEA` \x8B\x01\x8BaK\xD1V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x89` \x01` \x81\x01\x90a\x1D\x0B\x91\x90aK\xD1V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01a\x1D)``\x8B\x01`@\x8C\x01aQ\xA8V[b\xFF\xFF\xFF\x16\x81R0` \x82\x01R`@\x01a\x1DI`\x80\x8B\x01``\x8C\x01aN\xF0V[`\x02\x0B\x81R` \x01a\x1Da`\xA0\x8B\x01`\x80\x8C\x01aN\xF0V[`\x02\x0B\x81R` \x01\x89`\xA0\x015\x81R` \x01\x89`\xC0\x015\x81R` \x01\x89`\xE0\x015\x81R` \x01\x89a\x01\0\x015\x81RPa3aV[\x92\x97P\x90\x95P\x93P\x90Pa\x1D\xE2a\x1D\xB4a\x01@\x89\x01a\x01 \x8A\x01aK\xD1V[`\r\x80T`\x01`\x01`\xB0\x1B\x03\x19\x81\x16`\x01`\x01`\x01`\xB0\x1B\x03\x92\x83\x16\x90\x81\x01\x90\x92\x16\x17\x90\x91U\x97P\x87a8+V[`\0a\x1E\r0a\x1D\xF8`\x80\x8B\x01``\x8C\x01aN\xF0V[a\x1E\x08`\xA0\x8C\x01`\x80\x8D\x01aN\xF0V[a1cV[\x90P`\0\x80\x83`\x01`\x01`\xA0\x1B\x03\x16cQN\xA4\xBF\x84`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1E>\x91\x90aT\x14V[`\xA0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1EVW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x1EjW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E\x8E\x91\x90aP\xB6V[PP\x92P\x92PP`\0a\x1F\x07\x85`@Q\x80``\x01`@R\x80\x8E`\0\x01` \x81\x01\x90a\x1E\xB9\x91\x90aK\xD1V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8E` \x01` \x81\x01\x90a\x1E\xDA\x91\x90aK\xD1V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8E`@\x01` \x81\x01\x90a\x1E\xFB\x91\x90aQ\xA8V[b\xFF\xFF\xFF\x16\x90Ra9YV[\x90P`@Q\x80a\x01@\x01`@R\x80`\0`\x01`\x01``\x1B\x03\x16\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x82`\x01`\x01`P\x1B\x03\x16\x81R` \x01\x8C``\x01` \x81\x01\x90a\x1FW\x91\x90aN\xF0V[`\x02\x0B\x81R` \x01a\x1Fo`\xA0\x8E\x01`\x80\x8F\x01aN\xF0V[`\x02\x0B\x81R` \x01\x8A`\x01`\x01`\x80\x1B\x03\x16\x81R` \x01\x84\x81R` \x01\x83\x81R` \x01`\0`\x01`\x01`\x80\x1B\x03\x16\x81R` \x01`\0`\x01`\x01`\x80\x1B\x03\x16\x81RP`\x0C`\0\x8C\x81R` \x01\x90\x81R` \x01`\0 `\0\x82\x01Q\x81`\0\x01`\0a\x01\0\n\x81T\x81`\x01`\x01``\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01``\x1B\x03\x16\x02\x17\x90UP` \x82\x01Q\x81`\0\x01`\x0Ca\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP`@\x82\x01Q\x81`\x01\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`P\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`P\x1B\x03\x16\x02\x17\x90UP``\x82\x01Q\x81`\x01\x01`\na\x01\0\n\x81T\x81b\xFF\xFF\xFF\x02\x19\x16\x90\x83`\x02\x0Bb\xFF\xFF\xFF\x16\x02\x17\x90UP`\x80\x82\x01Q\x81`\x01\x01`\ra\x01\0\n\x81T\x81b\xFF\xFF\xFF\x02\x19\x16\x90\x83`\x02\x0Bb\xFF\xFF\xFF\x16\x02\x17\x90UP`\xA0\x82\x01Q\x81`\x01\x01`\x10a\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP`\xC0\x82\x01Q\x81`\x02\x01U`\xE0\x82\x01Q\x81`\x03\x01Ua\x01\0\x82\x01Q\x81`\x04\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UPa\x01 \x82\x01Q\x81`\x04\x01`\x10a\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP\x90PP\x89\x7F0g\x04\x8B\xEE\xE3\x1B%\xB2\xF1h\x1F\x88\xDA\xC88\xC8\xBB\xA3j\xF2[\xFB+|\xF7G:XG\xE3_\x8A\x8A\x8A`@Qa!n\x93\x92\x91\x90aU\x9EV[`@Q\x80\x91\x03\x90\xA2PPPPPP\x91\x93P\x91\x93V[`\x07\x80T`@\x80Q` `\x1F`\x02`\0\x19a\x01\0`\x01\x88\x16\x15\x02\x01\x90\x95\x16\x94\x90\x94\x04\x93\x84\x01\x81\x90\x04\x81\x02\x82\x01\x81\x01\x90\x92R\x82\x81R``\x93\x90\x92\x90\x91\x83\x01\x82\x82\x80\x15a\x07\xA1W\x80`\x1F\x10a\x07vWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x07\xA1V[`\0\x81\x81R`\x0C` \x90\x81R`@\x80\x83 \x81Qa\x01@\x81\x01\x83R\x81T`\x01`\x01``\x1B\x03\x81\x16\x82R`\x01`\x01`\xA0\x1B\x03`\x01``\x1B\x90\x91\x04\x16\x93\x81\x01\x93\x90\x93R`\x01\x81\x01T`\x01`\x01`P\x1B\x03\x81\x16\x92\x84\x01\x83\x90R`\x01`P\x1B\x81\x04`\x02\x90\x81\x0B\x81\x0B\x81\x0B``\x86\x01R`\x01`h\x1B\x82\x04\x81\x0B\x81\x0B\x81\x0B`\x80\x86\x01R`\x01`\x01`\x80\x1B\x03`\x01`\x80\x1B\x92\x83\x90\x04\x81\x16`\xA0\x87\x01R\x90\x83\x01T`\xC0\x86\x01R`\x03\x83\x01T`\xE0\x86\x01R`\x04\x90\x92\x01T\x80\x83\x16a\x01\0\x86\x01R\x04\x16a\x01 \x83\x01R\x82\x91\x82\x91\x82\x91\x82\x91\x82\x91\x82\x91\x82\x91\x82\x91\x82\x91\x82\x91\x82\x91\x90a\"\xD5W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xD2\x90aU\x0FV[`\0`\x0B`\0\x83`@\x01Q`\x01`\x01`P\x1B\x03\x16`\x01`\x01`P\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `@Q\x80``\x01`@R\x90\x81`\0\x82\x01`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\x01\x82\x01`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\x01\x82\x01`\x14\x90T\x90a\x01\0\n\x90\x04b\xFF\xFF\xFF\x16b\xFF\xFF\xFF\x16b\xFF\xFF\xFF\x16\x81RPP\x90P\x81`\0\x01Q\x82` \x01Q\x82`\0\x01Q\x83` \x01Q\x84`@\x01Q\x86``\x01Q\x87`\x80\x01Q\x88`\xA0\x01Q\x89`\xC0\x01Q\x8A`\xE0\x01Q\x8Ba\x01\0\x01Q\x8Ca\x01 \x01Q\x9DP\x9DP\x9DP\x9DP\x9DP\x9DP\x9DP\x9DP\x9DP\x9DP\x9DP\x9DPPP\x91\x93\x95\x97\x99\x9BP\x91\x93\x95\x97\x99\x9BV[a#\xFDa/hV[`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a$cW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FERC721: approve to caller\0\0\0\0\0\0\0`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x80`\x05`\0a$pa/hV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R` \x80\x83\x01\x93\x90\x93R`@\x91\x82\x01`\0\x90\x81 \x91\x87\x16\x80\x82R\x91\x90\x93R\x91 \x80T`\xFF\x19\x16\x92\x15\x15\x92\x90\x92\x17\x90\x91Ua$\xB4a/hV[`\x01`\x01`\xA0\x1B\x03\x16\x7F\x170~\xAB9\xABa\x07\xE8\x89\x98E\xAD=Y\xBD\x96S\xF2\0\xF2 \x92\x04\x89\xCA+Y7il1\x83`@Q\x80\x82\x15\x15\x81R` \x01\x91PP`@Q\x80\x91\x03\x90\xA3PPV[`@\x80Qcn\xB1v\x9F`\xE1\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R\x90Q`\0\x19\x91`\x01`\x01`\xA0\x1B\x03\x89\x16\x91c\xDDb\xED>\x91`D\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a%KW`\0\x80\xFD[PZ\xFA\x15\x80\x15a%_W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a%uW`\0\x80\xFD[PQ\x10\x15a%\x8BWa%\x8B\x86\x86\x86\x86\x86\x86a\x15\x9BV[PPPPPPV[``\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x80\x15a%\xACW`\0\x80\xFD[P`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a%\xE0W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a%\xCBW\x90P[P\x90P`\0[\x82\x81\x10\x15a&\xCCW`\0\x800\x86\x86\x85\x81\x81\x10a%\xFEW\xFE[\x90P` \x02\x81\x01\x90a&\x10\x91\x90aV\x8BV[`@Qa&\x1E\x92\x91\x90aR\xDCV[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a&YW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a&^V[``\x91P[P\x91P\x91P\x81a&\xAAW`D\x81Q\x10\x15a&wW`\0\x80\xFD[`\x04\x81\x01\x90P\x80\x80` \x01\x90Q\x81\x01\x90a&\x91\x91\x90aO\x0CV[`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xD2\x91\x90aT\\V[\x80\x84\x84\x81Q\x81\x10a&\xB7W\xFE[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a%\xE6V[P\x92\x91PPV[a&\xE4a&\xDEa/hV[\x83a/\xE2V[a'\x1FW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`1\x81R` \x01\x80aY\x81`1\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[a'+\x84\x84\x84\x84a:;V[PPPPV[`@\x80Qcn\xB1v\x9F`\xE1\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R\x90Q\x86\x91`\x01`\x01`\xA0\x1B\x03\x89\x16\x91c\xDDb\xED>\x91`D\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a'\x80W`\0\x80\xFD[PZ\xFA\x15\x80\x15a'\x94W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a'\xAAW`\0\x80\xFD[PQ\x10\x15a%\x8BWa%\x8B\x86\x86\x86\x86\x86\x86a*\x1EV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[``a'\xEF\x82a/[V[a'\xF8W`\0\x80\xFD[`@Qc\xE9\xDCcu`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xE9\xDCcu\x90a(F\x900\x90\x86\x90`\x04\x01aT\x1DV[`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a(^W`\0\x80\xFD[PZ\xFA\x15\x80\x15a(rW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x13\xC4\x91\x90\x81\x01\x90aO\x0CV[`\0a(\xA8\x82\x84\x01\x84aO\xAFV[\x90Pa(\xD8\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82`\0\x01Qa:\x8DV[P\x84\x15a(\xF3W\x80QQ` \x82\x01Qa(\xF3\x91\x903\x88a:\xB0V[\x83\x15a)\x11Wa)\x11\x81`\0\x01Q` \x01Q\x82` \x01Q3\x87a:\xB0V[PPPPPV[`\0\x83`\x01`\x01`\xA0\x1B\x03\x16cp\xA0\x8210`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x82`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x91PP` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a)gW`\0\x80\xFD[PZ\xFA\x15\x80\x15a){W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a)\x91W`\0\x80\xFD[PQ\x90P\x82\x81\x10\x15a)\xDFW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01Rq$\xB79\xBA\xB334\xB1\xB4\xB2\xB7:\x10:7\xB5\xB2\xB7`q\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x80\x15a'+Wa'+\x84\x83\x83a<@V[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\0\x90\x81R`\x05` \x90\x81R`@\x80\x83 \x93\x90\x94\x16\x82R\x91\x90\x91R T`\xFF\x16\x90V[`@\x80Qc\xD5\x05\xAC\xCF`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x87\x90R`d\x81\x01\x86\x90R`\xFF\x85\x16`\x84\x82\x01R`\xA4\x81\x01\x84\x90R`\xC4\x81\x01\x83\x90R\x90Q`\x01`\x01`\xA0\x1B\x03\x88\x16\x91c\xD5\x05\xAC\xCF\x91`\xE4\x80\x83\x01\x92`\0\x92\x91\x90\x82\x90\x03\x01\x81\x83\x87\x80;\x15\x80\x15a\x16\x15W`\0\x80\xFD[`\0\x80\x825a*\x9F3\x82a/\xE2V[a*\xBBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xD2\x90aToV[`\0a*\xCD``\x86\x01`@\x87\x01aPlV[`\x01`\x01`\x80\x1B\x03\x16\x11\x80a*\xFAWP`\0a*\xEF`\x80\x86\x01``\x87\x01aPlV[`\x01`\x01`\x80\x1B\x03\x16\x11[a+\x03W`\0\x80\xFD[`\0\x80a+\x16`@\x87\x01` \x88\x01aK\xD1V[`\x01`\x01`\xA0\x1B\x03\x16\x14a+9Wa+4`@\x86\x01` \x87\x01aK\xD1V[a+;V[0[\x855`\0\x90\x81R`\x0C` \x90\x81R`@\x80\x83 `\x01\x80\x82\x01T`\x01`\x01`P\x1B\x03\x16\x85R`\x0B\x84R\x82\x85 \x83Q``\x81\x01\x85R\x81T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R\x91\x90\x92\x01T\x90\x81\x16\x94\x82\x01\x94\x90\x94R`\x01`\xA0\x1B\x90\x93\x04b\xFF\xFF\xFF\x16\x91\x83\x01\x91\x90\x91R\x92\x93P\x90a+\xCE\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83a0\x84V[`\x04\x84\x01T`\x01\x85\x01T\x91\x92P`\x01`\x01`\x80\x1B\x03\x80\x82\x16\x92`\x01`\x80\x1B\x92\x83\x90\x04\x82\x16\x92\x90\x04\x16\x15a-\xC4W`\x01\x85\x01T`@Qc\xA3A#\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16\x91c\xA3A#\xA7\x91a,E\x91`\x01`P\x1B\x81\x04`\x02\x90\x81\x0B\x92`\x01`h\x1B\x90\x92\x04\x90\x0B\x90`\0\x90`\x04\x01aT6V[`@\x80Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a,^W`\0\x80\xFD[PZ\xF1\x15\x80\x15a,rW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a,\x96\x91\x90aQ\xFEV[PP`\x01\x85\x01T`\0\x90\x81\x90`\x01`\x01`\xA0\x1B\x03\x86\x16\x90cQN\xA4\xBF\x90a,\xD4\x900\x90`\x01`P\x1B\x81\x04`\x02\x90\x81\x0B\x91`\x01`h\x1B\x90\x04\x90\x0Ba1cV[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a,\xF0\x91\x90aT\x14V[`\xA0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a-\x08W`\0\x80\xFD[PZ\xFA\x15\x80\x15a-\x1CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a-@\x91\x90aP\xB6V[PP\x92P\x92PPa-|\x87`\x02\x01T\x83\x03\x88`\x01\x01`\x10\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\x80\x1B\x03\x16`\x01`\x01`\x80\x1B\x03\x16`\x01`\x80\x1Ba1\xB8V[\x84\x01\x93Pa-\xB5\x87`\x03\x01T\x82\x03\x88`\x01\x01`\x10\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\x80\x1B\x03\x16`\x01`\x01`\x80\x1B\x03\x16`\x01`\x80\x1Ba1\xB8V[`\x02\x88\x01\x92\x90\x92U`\x03\x87\x01U\x01[`\0\x80`\x01`\x01`\x80\x1B\x03\x84\x16a-\xE1``\x8E\x01`@\x8F\x01aPlV[`\x01`\x01`\x80\x1B\x03\x16\x11a.\x04Wa-\xFF``\x8D\x01`@\x8E\x01aPlV[a.\x06V[\x83[\x83`\x01`\x01`\x80\x1B\x03\x16\x8D``\x01` \x81\x01\x90a.#\x91\x90aPlV[`\x01`\x01`\x80\x1B\x03\x16\x11a.FWa.A`\x80\x8E\x01``\x8F\x01aPlV[a.HV[\x83[`\x01\x89\x01T`@Qc\t\xE3\xD6{`\xE3\x1B\x81R\x92\x94P\x90\x92P`\x01`\x01`\xA0\x1B\x03\x87\x16\x91cO\x1E\xB3\xD8\x91a.\x9B\x91\x8C\x91`\x01`P\x1B\x81\x04`\x02\x90\x81\x0B\x92`\x01`h\x1B\x90\x92\x04\x90\x0B\x90\x88\x90\x88\x90`\x04\x01aSBV[`@\x80Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a.\xB4W`\0\x80\xFD[PZ\xF1\x15\x80\x15a.\xC8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a.\xEC\x91\x90aP\x88V[`\x04\x89\x01\x80T`\x01`\x01`\x80\x1B\x03\x19`\x01`\x01`\x80\x1B\x03\x91\x82\x16`\x01`\x80\x1B\x87\x8A\x03\x84\x16\x02\x17\x16\x86\x89\x03\x82\x16\x17\x90\x91U`@Q\x92\x81\x16\x9DP\x16\x9AP\x8C5\x90\x7F@\xD0\xEF\xD1\xA5=`\xEC\xBF@\x97\x1B\x9D\xAF}\xC9\x01x\xC3\xAA\xDCz\xAB\x17ec'8\xFA\x8B\x8F\x01\x90a\r\x06\x90\x8B\x90\x86\x90\x86\x90aS\x7FV[`\0a\x13\xC4`\x02\x83a=\x87V[3\x90V[`\0\x81\x81R`\x0C` R`@\x90 \x80T`\x01`\x01``\x1B\x03\x16`\x01``\x1B`\x01`\x01`\xA0\x1B\x03\x85\x16\x90\x81\x02\x91\x90\x91\x17\x90\x91U\x81\x90a/\xA9\x82a\x17\xE4V[`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%`@Q`@Q\x80\x91\x03\x90\xA4PPV[`\0a/\xED\x82a/[V[a0(W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`,\x81R` \x01\x80aX^`,\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[`\0a03\x83a\x17\xE4V[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x14\x80a0nWP\x83`\x01`\x01`\xA0\x1B\x03\x16a0c\x84a\x07\xABV[`\x01`\x01`\xA0\x1B\x03\x16\x14[\x80a\x10\x1AWPa\x10\x1A\x81\x85a)\xF0V[`\x0ET\x90V[`\0\x81` \x01Q`\x01`\x01`\xA0\x1B\x03\x16\x82`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x10a0\xACW`\0\x80\xFD[P\x80Q` \x80\x83\x01Q`@\x93\x84\x01Q\x84Q`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x81\x85\x01R\x93\x90\x91\x16\x83\x85\x01Rb\xFF\xFF\xFF\x16``\x80\x84\x01\x91\x90\x91R\x83Q\x80\x84\x03\x82\x01\x81R`\x80\x84\x01\x85R\x80Q\x90\x83\x01 `\x01`\x01`\xF8\x1B\x03\x19`\xA0\x85\x01R\x94\x90\x1B`\x01`\x01``\x1B\x03\x19\x16`\xA1\x83\x01R`\xB5\x82\x01\x93\x90\x93R\x7F\xE3O\x19\x9B\x19\xB2\xB4\xF4\x7FhD&\x19\xD5UR}$Ox\xA3)~\xA8\x93%\xF8C\xF8{\x8BT`\xD5\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\xF5\x90\x91\x01\x90\x91R\x80Q\x91\x01 \x90V[`@\x80Q`\x01`\x01``\x1B\x03\x19``\x86\x90\x1B\x16` \x80\x83\x01\x91\x90\x91R`\x02\x85\x81\x0B`\xE8\x90\x81\x1B`4\x85\x01R\x90\x85\x90\x0B\x90\x1B`7\x83\x01R\x82Q`\x1A\x81\x84\x03\x01\x81R`:\x90\x92\x01\x90\x92R\x80Q\x91\x01 [\x93\x92PPPV[`\0\x80\x80`\0\x19\x85\x87\t\x86\x86\x02\x92P\x82\x81\x10\x90\x83\x90\x03\x03\x90P\x80a1\xEEW`\0\x84\x11a1\xE3W`\0\x80\xFD[P\x82\x90\x04\x90Pa1\xB1V[\x80\x84\x11a1\xFAW`\0\x80\xFD[`\0\x84\x86\x88\t`\0\x86\x81\x03\x87\x16\x96\x87\x90\x04\x96`\x02`\x03\x89\x02\x81\x18\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x90\x91\x03\x02\x91\x81\x90\x03\x81\x90\x04`\x01\x01\x86\x84\x11\x90\x95\x03\x94\x90\x94\x02\x91\x90\x94\x03\x92\x90\x92\x04\x91\x90\x91\x17\x91\x90\x91\x02\x91PP\x93\x92PPPV[`@\x80Q`\0\x80\x82R` \x82\x01\x90\x92R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x83\x90`@Q\x80\x82\x80Q\x90` \x01\x90\x80\x83\x83[` \x83\x10a2\xB3W\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01a2\x94V[`\x01\x83` \x03a\x01\0\n\x03\x80\x19\x82Q\x16\x81\x84Q\x16\x80\x82\x17\x85RPPPPPP\x90P\x01\x91PP`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a3\x15W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a3\x1AV[``\x91P[PP\x90P\x80a\x08\xCFW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x03`$\x82\x01RbSTE`\xE8\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\0a\x13\xC4\x82a=\x93V[`\0\x80`\0\x80`\0`@Q\x80``\x01`@R\x80\x87`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x87` \x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x87`@\x01Qb\xFF\xFF\xFF\x16\x81RP\x90Pa3\xD3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82a0\x84V[\x91P`\0\x82`\x01`\x01`\xA0\x1B\x03\x16c8P\xC7\xBD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\xE0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a4\x10W`\0\x80\xFD[PZ\xFA\x15\x80\x15a4$W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a4H\x91\x90aQ\x17V[PPPPPP\x90P`\0a4_\x88`\x80\x01Qa=\x97V[\x90P`\0a4p\x89`\xA0\x01Qa=\x97V[\x90Pa4\x87\x83\x83\x83\x8C`\xC0\x01Q\x8D`\xE0\x01Qa@\xC9V[\x97PPPP\x81`\x01`\x01`\xA0\x1B\x03\x16c<\x8A}\x8D\x87``\x01Q\x88`\x80\x01Q\x89`\xA0\x01Q\x89`@Q\x80`@\x01`@R\x80\x88\x81R` \x013`\x01`\x01`\xA0\x1B\x03\x16\x81RP`@Q` \x01a4\xD9\x91\x90aU^V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a5\x08\x95\x94\x93\x92\x91\x90aS\0V[`@\x80Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a5!W`\0\x80\xFD[PZ\xF1\x15\x80\x15a55W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a5Y\x91\x90aQ\xFEV[a\x01\0\x88\x01Q\x91\x95P\x93P\x84\x10\x80\x15\x90a5xWP\x85a\x01 \x01Q\x83\x10\x15[a5\x94W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xD2\x90aT\xE1V[P\x91\x93P\x91\x93V[\x82`\x01`\x01`\xA0\x1B\x03\x16a5\xAF\x82a\x17\xE4V[`\x01`\x01`\xA0\x1B\x03\x16\x14a5\xF4W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`)\x81R` \x01\x80aY7`)\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x82\x16a69W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`$\x81R` \x01\x80aX\x14`$\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[a6D\x83\x83\x83a\x08\xCFV[a6O`\0\x82a/lV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x01` R`@\x90 a6q\x90\x82aA\x8DV[P`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x01` R`@\x90 a6\x94\x90\x82aA\x99V[Pa6\xA1`\x02\x82\x84aA\xA5V[P\x80\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF`@Q`@Q\x80\x91\x03\x90\xA4PPPV[`\0a\x13\xC1\x83\x83aA\xBBV[F\x90V[`\0a7\x03\x82a\x17\xE4V[\x90Pa7\x11\x81`\0\x84a\x08\xCFV[a7\x1C`\0\x83a/lV[`\0\x82\x81R`\x08` R`@\x90 T`\x02`\0\x19a\x01\0`\x01\x84\x16\x15\x02\x01\x90\x91\x16\x04\x15a7ZW`\0\x82\x81R`\x08` R`@\x81 a7Z\x91aKAV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x01` R`@\x90 a7|\x90\x83aA\x8DV[Pa7\x88`\x02\x83aB\x1FV[P`@Q\x82\x90`\0\x90`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90\x83\x90\xA4PPV[`\0\x80\x80\x80a7\xD4\x86\x86aB+V[\x90\x94P\x92PPP[\x92P\x92\x90PV[`\0a\x10\x1A\x84\x84\x84aB\xA6V[`\0\x90\x81R`\x0C` R`@\x90 \x80T`\x01`\x01``\x1B\x03\x19\x81\x16`\x01`\x01`\x01``\x1B\x03\x92\x83\x16\x90\x81\x01\x90\x92\x16\x17\x90\x91U\x90V[;\x15\x15\x90V[`\x01`\x01`\xA0\x1B\x03\x82\x16a8\x86W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FERC721: mint to the zero address`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[a8\x8F\x81a/[V[\x15a8\xE1W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FERC721: token already minted\0\0\0\0`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[a8\xED`\0\x83\x83a\x08\xCFV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x01` R`@\x90 a9\x0F\x90\x82aA\x99V[Pa9\x1C`\x02\x82\x84aA\xA5V[P`@Q\x81\x90`\x01`\x01`\xA0\x1B\x03\x84\x16\x90`\0\x90\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90\x82\x90\xA4PPV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\n` R`@\x90 T`\x01`\x01`P\x1B\x03\x16\x80a\x13\xC4WP`\r\x80T`\x01`\x01`\x01`P\x1B\x03`\x01`\xB0\x1B\x80\x84\x04\x82\x16\x83\x81\x01\x90\x92\x16\x02`\x01`\x01`\xB0\x1B\x03\x90\x93\x16\x92\x90\x92\x17\x90\x92U`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x90\x81R`\n` \x90\x81R`@\x80\x83 \x80Ti\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x86\x17\x90U\x84\x83R`\x0B\x82R\x91\x82\x90 \x86Q\x81T\x90\x85\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x17\x82U\x91\x87\x01Q\x95\x01\x80T\x92\x87\x01Qb\xFF\xFF\xFF\x16`\x01`\xA0\x1B\x02b\xFF\xFF\xFF`\xA0\x1B\x19\x96\x90\x94\x16\x92\x90\x91\x16\x91\x90\x91\x17\x93\x90\x93\x16\x17\x90\x91U\x92\x91PPV[a:F\x84\x84\x84a5\x9CV[a:R\x84\x84\x84\x84aCpV[a'+W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`2\x81R` \x01\x80aW\xBB`2\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[`\0a:\x99\x83\x83a0\x84V[\x90P3`\x01`\x01`\xA0\x1B\x03\x82\x16\x14a\x13\xC4W`\0\x80\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x14\x80\x15a:\xF1WP\x80G\x10\x15[\x15a<\x13W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xD0\xE3\r\xB0\x82`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a;QW`\0\x80\xFD[PZ\xF1\x15\x80\x15a;eW=`\0\x80>=`\0\xFD[PPPPP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xA9\x05\x9C\xBB\x83\x83`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x83`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x82\x81R` \x01\x92PPP` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a;\xE1W`\0\x80\xFD[PZ\xF1\x15\x80\x15a;\xF5W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a<\x0BW`\0\x80\xFD[Pa'+\x90PV[`\x01`\x01`\xA0\x1B\x03\x83\x160\x14\x15a<4Wa</\x84\x83\x83a<@V[a'+V[a'+\x84\x84\x84\x84aD\xD8V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`$\x83\x01R`D\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`d\x90\x92\x01\x83R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\xA9\x05\x9C\xBB`\xE0\x1B\x17\x81R\x92Q\x82Q`\0\x94\x85\x94\x93\x89\x16\x93\x92\x91\x82\x91\x90\x80\x83\x83[` \x83\x10a<\xBCW\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01a<\x9DV[`\x01\x83` \x03a\x01\0\n\x03\x80\x19\x82Q\x16\x81\x84Q\x16\x80\x82\x17\x85RPPPPPP\x90P\x01\x91PP`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a=\x1EW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a=#V[``\x91P[P\x91P\x91P\x81\x80\x15a=QWP\x80Q\x15\x80a=QWP\x80\x80` \x01\x90Q` \x81\x10\x15a=NW`\0\x80\xFD[PQ[a)\x11W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x02`$\x82\x01Ra\x14\xD5`\xF2\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\0a\x13\xC1\x83\x83aF(V[T\x90V[`\0\x80`\0\x83`\x02\x0B\x12a=\xAEW\x82`\x02\x0Ba=\xB6V[\x82`\x02\x0B`\0\x03[\x90Pb\r\x89\xE8\x81\x11\x15a=\xF4W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x01`$\x82\x01R`\x15`\xFA\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\0`\x01\x82\x16a>\x08W`\x01`\x80\x1Ba>\x1AV[o\xFF\xFC\xB93\xBDo\xAD7\xAA-\x16-\x1AY@\x01[p\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P`\x02\x82\x16\x15a>NWo\xFF\xF9rr7=A2Y\xA4i\x90X\x0E!:\x02`\x80\x1C[`\x04\x82\x16\x15a>mWo\xFF\xF2\xE5\x0F_ei2\xEF\x125|\xF3\xC7\xFD\xCC\x02`\x80\x1C[`\x08\x82\x16\x15a>\x8CWo\xFF\xE5\xCA\xCA~\x10\xE4\xE6\x1C6$\xEA\xA0\x94\x1C\xD0\x02`\x80\x1C[`\x10\x82\x16\x15a>\xABWo\xFF\xCB\x98C\xD6\x0FaY\xC9\xDBX\x83\\\x92fD\x02`\x80\x1C[` \x82\x16\x15a>\xCAWo\xFF\x97;A\xFA\x98\xC0\x81G.h\x96\xDF\xB2T\xC0\x02`\x80\x1C[`@\x82\x16\x15a>\xE9Wo\xFF.\xA1df\xC9j8C\xECx\xB3&\xB5(a\x02`\x80\x1C[`\x80\x82\x16\x15a?\x08Wo\xFE]\xEE\x04j\x99\xA2\xA8\x11\xC4a\xF1\x96\x9C0S\x02`\x80\x1C[a\x01\0\x82\x16\x15a?(Wo\xFC\xBE\x86\xC7\x90\n\x88\xAE\xDC\xFF\xC8;G\x9A\xA3\xA4\x02`\x80\x1C[a\x02\0\x82\x16\x15a?HWo\xF9\x87\xA7%:\xC4\x13\x17o+\x07L\xF7\x81^T\x02`\x80\x1C[a\x04\0\x82\x16\x15a?hWo\xF39+\x08\"\xB7\0\x05\x94\x0Cz9\x8EKp\xF3\x02`\x80\x1C[a\x08\0\x82\x16\x15a?\x88Wo\xE7\x15\x94u\xA2\xC2\x9BtC\xB2\x9C\x7F\xA6\xE8\x89\xD9\x02`\x80\x1C[a\x10\0\x82\x16\x15a?\xA8Wo\xD0\x97\xF3\xBD\xFD \"\xB8\x84Z\xD8\xF7\x92\xAAX%\x02`\x80\x1C[a \0\x82\x16\x15a?\xC8Wo\xA9\xF7FF-\x87\x0F\xDF\x8Ae\xDC\x1F\x90\xE0a\xE5\x02`\x80\x1C[a@\0\x82\x16\x15a?\xE8Wop\xD8i\xA1V\xD2\xA1\xB8\x90\xBB=\xF6+\xAF2\xF7\x02`\x80\x1C[a\x80\0\x82\x16\x15a@\x08Wo1\xBE\x13_\x97\xD0\x8F\xD9\x81#\x15\x05T/\xCF\xA6\x02`\x80\x1C[b\x01\0\0\x82\x16\x15a@)Wo\t\xAAP\x8B[z\x84\xE1\xC6w\xDET\xF3\xE9\x9B\xC9\x02`\x80\x1C[b\x02\0\0\x82\x16\x15a@IWn]j\xF8\xDE\xDB\x81\x19f\x99\xC3)\"^\xE6\x04\x02`\x80\x1C[b\x04\0\0\x82\x16\x15a@hWm\"\x16\xE5\x84\xF5\xFA\x1E\xA9&\x04\x1B\xED\xFE\x98\x02`\x80\x1C[b\x08\0\0\x82\x16\x15a@\x85Wk\x04\x8A\x17\x03\x91\xF7\xDCBDN\x8F\xA2\x02`\x80\x1C[`\0\x84`\x02\x0B\x13\x15a@\xA0W\x80`\0\x19\x81a@\x9CW\xFE[\x04\x90P[d\x01\0\0\0\0\x81\x06\x15a@\xB4W`\x01a@\xB7V[`\0[`\xFF\x16` \x82\x90\x1C\x01\x92PPP\x91\x90PV[`\0\x83`\x01`\x01`\xA0\x1B\x03\x16\x85`\x01`\x01`\xA0\x1B\x03\x16\x11\x15a@\xE9W\x92\x93\x92[\x84`\x01`\x01`\xA0\x1B\x03\x16\x86`\x01`\x01`\xA0\x1B\x03\x16\x11aA\x14WaA\r\x85\x85\x85aF@V[\x90PaA\x84V[\x83`\x01`\x01`\xA0\x1B\x03\x16\x86`\x01`\x01`\xA0\x1B\x03\x16\x10\x15aAvW`\0aA;\x87\x86\x86aF@V[\x90P`\0aAJ\x87\x89\x86aF\xA3V[\x90P\x80`\x01`\x01`\x80\x1B\x03\x16\x82`\x01`\x01`\x80\x1B\x03\x16\x10aAkW\x80aAmV[\x81[\x92PPPaA\x84V[aA\x81\x85\x85\x84aF\xA3V[\x90P[\x95\x94PPPPPV[`\0a\x13\xC1\x83\x83aF\xE0V[`\0a\x13\xC1\x83\x83aG\xA6V[`\0a\x10\x1A\x84\x84`\x01`\x01`\xA0\x1B\x03\x85\x16aG\xF0V[\x81T`\0\x90\x82\x10aA\xFDW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`\"\x81R` \x01\x80aW\x99`\"\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[\x82`\0\x01\x82\x81T\x81\x10aB\x0CW\xFE[\x90`\0R` `\0 \x01T\x90P\x92\x91PPV[`\0a\x13\xC1\x83\x83aH\x87V[\x81T`\0\x90\x81\x90\x83\x10aBoW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`\"\x81R` \x01\x80aY\x15`\"\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[`\0\x84`\0\x01\x84\x81T\x81\x10aB\x80W\xFE[\x90`\0R` `\0 \x90`\x02\x02\x01\x90P\x80`\0\x01T\x81`\x01\x01T\x92P\x92PP\x92P\x92\x90PV[`\0\x82\x81R`\x01\x84\x01` R`@\x81 T\x82\x81aCAW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R\x83\x81\x81Q\x81R` \x01\x91P\x80Q\x90` \x01\x90\x80\x83\x83`\0[\x83\x81\x10\x15aC\x06W\x81\x81\x01Q\x83\x82\x01R` \x01aB\xEEV[PPPP\x90P\x90\x81\x01\x90`\x1F\x16\x80\x15aC3W\x80\x82\x03\x80Q`\x01\x83` \x03a\x01\0\n\x03\x19\x16\x81R` \x01\x91P[P\x92PPP`@Q\x80\x91\x03\x90\xFD[P\x84`\0\x01`\x01\x82\x03\x81T\x81\x10aCTW\xFE[\x90`\0R` `\0 \x90`\x02\x02\x01`\x01\x01T\x91PP\x93\x92PPPV[`\0aC\x84\x84`\x01`\x01`\xA0\x1B\x03\x16a8%V[aC\x90WP`\x01a\x10\x1AV[`\0aD\x9Ec\n\x85\xBD\x01`\xE1\x1BaC\xA5a/hV[\x88\x87\x87`@Q`$\x01\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x84`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83\x81R` \x01\x80` \x01\x82\x81\x03\x82R\x83\x81\x81Q\x81R` \x01\x91P\x80Q\x90` \x01\x90\x80\x83\x83`\0[\x83\x81\x10\x15aD\x0CW\x81\x81\x01Q\x83\x82\x01R` \x01aC\xF4V[PPPP\x90P\x90\x81\x01\x90`\x1F\x16\x80\x15aD9W\x80\x82\x03\x80Q`\x01\x83` \x03a\x01\0\n\x03\x19\x16\x81R` \x01\x91P[P\x95PPPPPP`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90`\x01`\x01`\xE0\x1B\x03\x19\x16` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x83\x81\x83\x16\x17\x83RPPPP`@Q\x80``\x01`@R\x80`2\x81R` \x01aW\xBB`2\x919`\x01`\x01`\xA0\x1B\x03\x88\x16\x91\x90aI[V[\x90P`\0\x81\x80` \x01\x90Q` \x81\x10\x15aD\xB7W`\0\x80\xFD[PQ`\x01`\x01`\xE0\x1B\x03\x19\x16c\n\x85\xBD\x01`\xE1\x1B\x14\x92PPP\x94\x93PPPPV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`$\x83\x01R\x84\x81\x16`D\x83\x01R`d\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`\x84\x90\x92\x01\x83R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c#\xB8r\xDD`\xE0\x1B\x17\x81R\x92Q\x82Q`\0\x94\x85\x94\x93\x8A\x16\x93\x92\x91\x82\x91\x90\x80\x83\x83[` \x83\x10aE\\W\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01aE=V[`\x01\x83` \x03a\x01\0\n\x03\x80\x19\x82Q\x16\x81\x84Q\x16\x80\x82\x17\x85RPPPPPP\x90P\x01\x91PP`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14aE\xBEW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>aE\xC3V[``\x91P[P\x91P\x91P\x81\x80\x15aE\xF1WP\x80Q\x15\x80aE\xF1WP\x80\x80` \x01\x90Q` \x81\x10\x15aE\xEEW`\0\x80\xFD[PQ[a%\x8BW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x03`$\x82\x01Rb)\xAA#`\xE9\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\0\x90\x81R`\x01\x91\x90\x91\x01` R`@\x90 T\x15\x15\x90V[`\0\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x11\x15aF`W\x91\x92\x91[`\0aF\x83\x85`\x01`\x01`\xA0\x1B\x03\x16\x85`\x01`\x01`\xA0\x1B\x03\x16`\x01``\x1Ba1\xB8V[\x90PaA\x84aF\x9E\x84\x83\x88\x88\x03`\x01`\x01`\xA0\x1B\x03\x16a1\xB8V[aIjV[`\0\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x11\x15aF\xC3W\x91\x92\x91[a\x10\x1AaF\x9E\x83`\x01``\x1B\x87\x87\x03`\x01`\x01`\xA0\x1B\x03\x16a1\xB8V[`\0\x81\x81R`\x01\x83\x01` R`@\x81 T\x80\x15aG\x9CW\x83T`\0\x19\x80\x83\x01\x91\x90\x81\x01\x90`\0\x90\x87\x90\x83\x90\x81\x10aG\x13W\xFE[\x90`\0R` `\0 \x01T\x90P\x80\x87`\0\x01\x84\x81T\x81\x10aG0W\xFE[`\0\x91\x82R` \x80\x83 \x90\x91\x01\x92\x90\x92U\x82\x81R`\x01\x89\x81\x01\x90\x92R`@\x90 \x90\x84\x01\x90U\x86T\x87\x90\x80aG`W\xFE[`\x01\x90\x03\x81\x81\x90`\0R` `\0 \x01`\0\x90U\x90U\x86`\x01\x01`\0\x87\x81R` \x01\x90\x81R` \x01`\0 `\0\x90U`\x01\x94PPPPPa\x13\xC4V[`\0\x91PPa\x13\xC4V[`\0aG\xB2\x83\x83aF(V[aG\xE8WP\x81T`\x01\x81\x81\x01\x84U`\0\x84\x81R` \x80\x82 \x90\x93\x01\x84\x90U\x84T\x84\x82R\x82\x86\x01\x90\x93R`@\x90 \x91\x90\x91Ua\x13\xC4V[P`\0a\x13\xC4V[`\0\x82\x81R`\x01\x84\x01` R`@\x81 T\x80aHUWPP`@\x80Q\x80\x82\x01\x82R\x83\x81R` \x80\x82\x01\x84\x81R\x86T`\x01\x81\x81\x01\x89U`\0\x89\x81R\x84\x81 \x95Q`\x02\x90\x93\x02\x90\x95\x01\x91\x82U\x91Q\x90\x82\x01U\x86T\x86\x84R\x81\x88\x01\x90\x92R\x92\x90\x91 Ua1\xB1V[\x82\x85`\0\x01`\x01\x83\x03\x81T\x81\x10aHhW\xFE[\x90`\0R` `\0 \x90`\x02\x02\x01`\x01\x01\x81\x90UP`\0\x91PPa1\xB1V[`\0\x81\x81R`\x01\x83\x01` R`@\x81 T\x80\x15aG\x9CW\x83T`\0\x19\x80\x83\x01\x91\x90\x81\x01\x90`\0\x90\x87\x90\x83\x90\x81\x10aH\xBAW\xFE[\x90`\0R` `\0 \x90`\x02\x02\x01\x90P\x80\x87`\0\x01\x84\x81T\x81\x10aH\xDAW\xFE[`\0\x91\x82R` \x80\x83 \x84T`\x02\x90\x93\x02\x01\x91\x82U`\x01\x93\x84\x01T\x91\x84\x01\x91\x90\x91U\x83T\x82R\x89\x83\x01\x90R`@\x90 \x90\x84\x01\x90U\x86T\x87\x90\x80aI\x19W\xFE[`\0\x82\x81R` \x80\x82 `\x02`\0\x19\x90\x94\x01\x93\x84\x02\x01\x82\x81U`\x01\x90\x81\x01\x83\x90U\x92\x90\x93U\x88\x81R\x89\x82\x01\x90\x92R`@\x82 \x91\x90\x91U\x94Pa\x13\xC4\x93PPPPV[``a\x10\x1A\x84\x84`\0\x85aI\x80V[\x80`\x01`\x01`\x80\x1B\x03\x81\x16\x81\x14a\x07\x10W`\0\x80\xFD[``\x82G\x10\x15aI\xC1W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`&\x81R` \x01\x80aX8`&\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[aI\xCA\x85a8%V[aJ\x1BW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\0\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@Q\x80\x82\x80Q\x90` \x01\x90\x80\x83\x83[` \x83\x10aJYW\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01aJ:V[`\x01\x83` \x03a\x01\0\n\x03\x80\x19\x82Q\x16\x81\x84Q\x16\x80\x82\x17\x85RPPPPPP\x90P\x01\x91PP`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14aJ\xBBW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>aJ\xC0V[``\x91P[P\x91P\x91PaJ\xD0\x82\x82\x86aJ\xDBV[\x97\x96PPPPPPPV[``\x83\x15aJ\xEAWP\x81a1\xB1V[\x82Q\x15aJ\xFAW\x82Q\x80\x84` \x01\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x81R\x84Q`$\x84\x01R\x84Q\x85\x93\x91\x92\x83\x92`D\x01\x91\x90\x85\x01\x90\x80\x83\x83`\0\x83\x15aC\x06W\x81\x81\x01Q\x83\x82\x01R` \x01aB\xEEV[P\x80T`\x01\x81`\x01\x16\x15a\x01\0\x02\x03\x16`\x02\x90\x04`\0\x82U\x80`\x1F\x10aKgWPaK\x85V[`\x1F\x01` \x90\x04\x90`\0R` `\0 \x90\x81\x01\x90aK\x85\x91\x90aK\x88V[PV[[\x80\x82\x11\x15aK\x9DW`\0\x81U`\x01\x01aK\x89V[P\x90V[\x805a\x07\x10\x81aWBV[\x80Qa\xFF\xFF\x81\x16\x81\x14a\x07\x10W`\0\x80\xFD[\x805b\xFF\xFF\xFF\x81\x16\x81\x14a\x07\x10W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aK\xE2W\x80\x81\xFD[\x815a1\xB1\x81aWBV[`\0\x80`@\x83\x85\x03\x12\x15aK\xFFW\x80\x81\xFD[\x825aL\n\x81aWBV[\x91P` \x83\x015aL\x1A\x81aWBV[\x80\x91PP\x92P\x92\x90PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15aL:W\x81\x82\xFD[\x845aLE\x81aWBV[\x93P` \x85\x015aLU\x81aWBV[\x92PaLc`@\x86\x01aK\xBEV[\x91P``\x85\x015aLs\x81aWBV[\x93\x96\x92\x95P\x90\x93PPV[`\0\x80`\0``\x84\x86\x03\x12\x15aL\x92W\x80\x81\xFD[\x835aL\x9D\x81aWBV[\x92P` \x84\x015aL\xAD\x81aWBV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15aL\xD3W\x81\x82\xFD[\x845aL\xDE\x81aWBV[\x93P` \x85\x015aL\xEE\x81aWBV[\x92P`@\x85\x015\x91P``\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aM\x10W\x81\x82\xFD[\x85\x01`\x1F\x81\x01\x87\x13aM W\x81\x82\xFD[\x805aM3aM.\x82aV\xF4V[aV\xD0V[\x81\x81R\x88` \x83\x85\x01\x01\x11\x15aMGW\x83\x84\xFD[\x81` \x84\x01` \x83\x017\x90\x81\x01` \x01\x92\x90\x92RP\x92\x95\x91\x94P\x92PV[`\0\x80`@\x83\x85\x03\x12\x15aMwW\x81\x82\xFD[\x825aM\x82\x81aWBV[\x91P` \x83\x015aL\x1A\x81aWWV[`\0\x80`@\x83\x85\x03\x12\x15aM\xA4W\x81\x82\xFD[\x825aM\xAF\x81aWBV[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15aM\xD1W\x80\x81\xFD[\x835aM\xDC\x81aWBV[\x92P` \x84\x015\x91P`@\x84\x015aM\xF3\x81aWBV[\x80\x91PP\x92P\x92P\x92V[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15aN\x16W\x83\x84\xFD[\x865aN!\x81aWBV[\x95P` \x87\x015\x94P`@\x87\x015\x93P``\x87\x015aN?\x81aW\x89V[\x95\x98\x94\x97P\x92\x95`\x80\x81\x015\x94`\xA0\x90\x91\x015\x93P\x91PPV[`\0\x80` \x83\x85\x03\x12\x15aNkW\x81\x82\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15aN\x82W\x83\x84\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12aN\x95W\x83\x84\xFD[\x815\x81\x81\x11\x15aN\xA3W\x84\x85\xFD[\x86` \x80\x83\x02\x85\x01\x01\x11\x15aN\xB6W\x84\x85\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[`\0` \x82\x84\x03\x12\x15aN\xD9W\x80\x81\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a1\xB1W\x81\x82\xFD[`\0` \x82\x84\x03\x12\x15aO\x01W\x80\x81\xFD[\x815a1\xB1\x81aWeV[`\0` \x82\x84\x03\x12\x15aO\x1DW\x80\x81\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aO3W\x81\x82\xFD[\x82\x01`\x1F\x81\x01\x84\x13aOCW\x81\x82\xFD[\x80QaOQaM.\x82aV\xF4V[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15aOeW\x83\x84\xFD[aA\x84\x82` \x83\x01` \x86\x01aW\x16V[`\0`\x80\x82\x84\x03\x12\x15aO\x87W\x80\x81\xFD[P\x91\x90PV[`\0`\xA0\x82\x84\x03\x12\x15aO\x87W\x80\x81\xFD[`\0`\xC0\x82\x84\x03\x12\x15aO\x87W\x80\x81\xFD[`\0\x81\x83\x03`\x80\x81\x12\x15aO\xC1W\x81\x82\xFD[`@Q`@\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x82\x10\x81\x83\x11\x17\x15aO\xDFW\xFE[\x81`@R``\x84\x12\x15aO\xF0W\x84\x85\xFD[`\xA0\x83\x01\x93P\x81\x84\x10\x81\x85\x11\x17\x15aP\x04W\xFE[P\x82`@R\x845\x92PaP\x16\x83aWBV[\x91\x82R` \x84\x015\x91aP(\x83aWBV[\x82``\x83\x01RaP:`@\x86\x01aK\xBEV[`\x80\x83\x01R\x81RaPM``\x85\x01aK\xA1V[` \x82\x01R\x94\x93PPPPV[`\0a\x01`\x82\x84\x03\x12\x15aO\x87W\x80\x81\xFD[`\0` \x82\x84\x03\x12\x15aP}W\x80\x81\xFD[\x815a1\xB1\x81aWtV[`\0\x80`@\x83\x85\x03\x12\x15aP\x9AW\x81\x82\xFD[\x82QaP\xA5\x81aWtV[` \x84\x01Q\x90\x92PaL\x1A\x81aWtV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15aP\xCDW\x82\x83\xFD[\x85QaP\xD8\x81aWtV[\x80\x95PP` \x86\x01Q\x93P`@\x86\x01Q\x92P``\x86\x01QaP\xF8\x81aWtV[`\x80\x87\x01Q\x90\x92PaQ\t\x81aWtV[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[`\0\x80`\0\x80`\0\x80`\0`\xE0\x88\x8A\x03\x12\x15aQ1W\x84\x85\xFD[\x87QaQ<\x81aWBV[` \x89\x01Q\x90\x97PaQM\x81aWeV[\x95PaQ[`@\x89\x01aK\xACV[\x94PaQi``\x89\x01aK\xACV[\x93PaQw`\x80\x89\x01aK\xACV[\x92P`\xA0\x88\x01QaQ\x87\x81aW\x89V[`\xC0\x89\x01Q\x90\x92PaQ\x98\x81aWWV[\x80\x91PP\x92\x95\x98\x91\x94\x97P\x92\x95PV[`\0` \x82\x84\x03\x12\x15aQ\xB9W\x80\x81\xFD[a\x13\xC1\x82aK\xBEV[`\0` \x82\x84\x03\x12\x15aQ\xD3W\x80\x81\xFD[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15aQ\xECW\x81\x82\xFD[\x825\x91P` \x83\x015aL\x1A\x81aWBV[`\0\x80`@\x83\x85\x03\x12\x15aR\x10W\x81\x82\xFD[PP\x80Q` \x90\x91\x01Q\x90\x92\x90\x91PV[`\0\x80`\0\x80``\x85\x87\x03\x12\x15aR6W\x81\x82\xFD[\x845\x93P` \x85\x015\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15aR[W\x83\x84\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12aRnW\x83\x84\xFD[\x815\x81\x81\x11\x15aR|W\x84\x85\xFD[\x88` \x82\x85\x01\x01\x11\x15aR\x8DW\x84\x85\xFD[\x95\x98\x94\x97PP` \x01\x94PPPV[`\0\x81Q\x80\x84RaR\xB4\x81` \x86\x01` \x86\x01aW\x16V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\x02\x0B\x90RV[`\x01`\x01`\x80\x1B\x03\x16\x90RV[`\0\x82\x84\x837\x91\x01\x90\x81R\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x81R` \x01\x90V[`\0`\x01\x80`\xA0\x1B\x03\x87\x16\x82R\x85`\x02\x0B` \x83\x01R\x84`\x02\x0B`@\x83\x01R`\x01`\x01`\x80\x1B\x03\x84\x16``\x83\x01R`\xA0`\x80\x83\x01RaJ\xD0`\xA0\x83\x01\x84aR\x9CV[`\x01`\x01`\xA0\x1B\x03\x95\x90\x95\x16\x85R`\x02\x93\x84\x0B` \x86\x01R\x91\x90\x92\x0B`@\x84\x01R`\x01`\x01`\x80\x1B\x03\x91\x82\x16``\x84\x01R\x16`\x80\x82\x01R`\xA0\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x83R`\x01`\x01`\x80\x1B\x03\x91\x82\x16` \x84\x01R\x16`@\x82\x01R``\x01\x90V[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x86\x01\x91P`@\x84\x82\x02\x87\x01\x01\x92P\x83\x87\x01\x85[\x82\x81\x10\x15aS\xFCW`?\x19\x88\x86\x03\x01\x84RaS\xEA\x85\x83QaR\x9CV[\x94P\x92\x85\x01\x92\x90\x85\x01\x90`\x01\x01aS\xCEV[P\x92\x97\x96PPPPPPPV[\x90\x15\x15\x81R` \x01\x90V[\x90\x81R` \x01\x90V[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x82R` \x82\x01R`@\x01\x90V[`\x02\x93\x84\x0B\x81R\x91\x90\x92\x0B` \x82\x01R`\x01`\x01`\x80\x1B\x03\x90\x91\x16`@\x82\x01R``\x01\x90V[`\0` \x82Ra\x13\xC1` \x83\x01\x84aR\x9CV[` \x80\x82R`\x0C\x90\x82\x01Rk\x13\x9B\xDD\x08\x18\\\x1C\x1C\x9B\xDD\x99Y`\xA2\x1B`@\x82\x01R``\x01\x90V[` \x80\x82R`,\x90\x82\x01R\x7FERC721: approved query for nonex`@\x82\x01Rk4\xB9\xBA2\xB7:\x10:7\xB5\xB2\xB7`\xA1\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`\x14\x90\x82\x01RsPrice slippage check``\x1B`@\x82\x01R``\x01\x90V[` \x80\x82R`\x10\x90\x82\x01Ro\x12[\x9D\x98[\x1AY\x08\x1D\x1B\xDA\xD9[\x88\x12Q`\x82\x1B`@\x82\x01R``\x01\x90V[` \x80\x82R`\x0B\x90\x82\x01Rj\x13\x9B\xDD\x08\x18\xDB\x19X\\\x99Y`\xAA\x1B`@\x82\x01R``\x01\x90V[\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83R` \x80\x83\x01Q\x82\x16\x81\x85\x01R`@\x92\x83\x01Qb\xFF\xFF\xFF\x16\x92\x84\x01\x92\x90\x92R\x92\x01Q\x90\x91\x16``\x82\x01R`\x80\x01\x90V[`\x01`\x01`\x80\x1B\x03\x93\x90\x93\x16\x83R` \x83\x01\x91\x90\x91R`@\x82\x01R``\x01\x90V[\x93\x84R`\x01`\x01`\x80\x1B\x03\x92\x90\x92\x16` \x84\x01R`@\x83\x01R``\x82\x01R`\x80\x01\x90V[\x91\x82R` \x82\x01R`@\x01\x90V[`\x01`\x01``\x1B\x03\x8D\x16\x81R`\x01`\x01`\xA0\x1B\x03\x8C\x81\x16` \x83\x01R\x8B\x81\x16`@\x83\x01R\x8A\x16``\x82\x01Rb\xFF\xFF\xFF\x89\x16`\x80\x82\x01R`\x02\x88\x90\x0B`\xA0\x82\x01Ra\x01\x80\x81\x01aVC`\xC0\x83\x01\x89aR\xC8V[aVP`\xE0\x83\x01\x88aR\xCFV[\x85a\x01\0\x83\x01R\x84a\x01 \x83\x01RaVla\x01@\x83\x01\x85aR\xCFV[aVza\x01`\x83\x01\x84aR\xCFV[\x9D\x9CPPPPPPPPPPPPPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aV\xA1W\x82\x83\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aV\xBBW\x82\x83\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a7\xDCW`\0\x80\xFD[`@Q\x81\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15aV\xECW\xFE[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aW\x08W\xFE[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0[\x83\x81\x10\x15aW1W\x81\x81\x01Q\x83\x82\x01R` \x01aW\x19V[\x83\x81\x11\x15a'+WPP`\0\x91\x01RV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14aK\x85W`\0\x80\xFD[\x80\x15\x15\x81\x14aK\x85W`\0\x80\xFD[\x80`\x02\x0B\x81\x14aK\x85W`\0\x80\xFD[`\x01`\x01`\x80\x1B\x03\x81\x16\x81\x14aK\x85W`\0\x80\xFD[`\xFF\x81\x16\x81\x14aK\x85W`\0\x80\xFD\xFEEnumerableSet: index out of boundsERC721: transfer to non ERC721Receiver implementerERC721Permit: approval to current ownerERC721: transfer to the zero addressAddress: insufficient balance for callERC721: operator query for nonexistent tokenERC721: approve caller is not owner nor approved for allERC721: balance query for the zero addressERC721: owner query for nonexistent tokenEnumerableMap: index out of boundsERC721: transfer of token that is not ownERC721: approval to current ownerERC721: transfer caller is not owner nor approved\xA2dipfsX\"\x12 K\x9DR\xEF)\xEAKT\x99\xCC\x04\xD2\xA7\x9A\xD4\xA4\x9B\x17\x1E\xE4\x97\xEA\xF2\x1B IU\x82\x9A\xF5j\xAAdsolcC\0\x07\x06\x003";
    /// The deployed bytecode of the contract.
    pub static MOCKTIMENONFUNGIBLEPOSITIONMANAGER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct MockTimeNonfungiblePositionManager<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MockTimeNonfungiblePositionManager<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MockTimeNonfungiblePositionManager<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MockTimeNonfungiblePositionManager<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MockTimeNonfungiblePositionManager<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MockTimeNonfungiblePositionManager))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MockTimeNonfungiblePositionManager<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                MOCKTIMENONFUNGIBLEPOSITIONMANAGER_ABI.clone(),
                client,
            ))
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
                MOCKTIMENONFUNGIBLEPOSITIONMANAGER_ABI.clone(),
                MOCKTIMENONFUNGIBLEPOSITIONMANAGER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `DOMAIN_SEPARATOR` (0x3644e515) function
        pub fn domain_separator(&self) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([54, 68, 229, 21], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `PERMIT_TYPEHASH` (0x30adf81f) function
        pub fn permit_typehash(&self) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([48, 173, 248, 31], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `WETH9` (0x4aa4a4fc) function
        pub fn weth9(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([74, 164, 164, 252], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `approve` (0x095ea7b3) function
        pub fn approve(
            &self,
            to: ::ethers::core::types::Address,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([9, 94, 167, 179], (to, token_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `balanceOf` (0x70a08231) function
        pub fn balance_of(
            &self,
            owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `baseURI` (0x6c0360eb) function
        pub fn base_uri(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([108, 3, 96, 235], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `burn` (0x42966c68) function
        pub fn burn(
            &self,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([66, 150, 108, 104], token_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `collect` (0xfc6f7865) function
        pub fn collect(
            &self,
            params: CollectParams,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([252, 111, 120, 101], (params,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createAndInitializePoolIfNecessary` (0x13ead562) function
        pub fn create_and_initialize_pool_if_necessary(
            &self,
            token_0: ::ethers::core::types::Address,
            token_1: ::ethers::core::types::Address,
            fee: u32,
            sqrt_price_x96: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([19, 234, 213, 98], (token_0, token_1, fee, sqrt_price_x96))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `decreaseLiquidity` (0x0c49ccbe) function
        pub fn decrease_liquidity(
            &self,
            params: DecreaseLiquidityParams,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([12, 73, 204, 190], (params,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `factory` (0xc45a0155) function
        pub fn factory(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([196, 90, 1, 85], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getApproved` (0x081812fc) function
        pub fn get_approved(
            &self,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([8, 24, 18, 252], token_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `increaseLiquidity` (0x219f5d17) function
        pub fn increase_liquidity(
            &self,
            params: IncreaseLiquidityParams,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                u128,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([33, 159, 93, 23], (params,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isApprovedForAll` (0xe985e9c5) function
        pub fn is_approved_for_all(
            &self,
            owner: ::ethers::core::types::Address,
            operator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([233, 133, 233, 197], (owner, operator))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mint` (0x88316456) function
        pub fn mint(
            &self,
            params: MintParams,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                u128,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([136, 49, 100, 86], (params,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `multicall` (0xac9650d8) function
        pub fn multicall(
            &self,
            data: ::std::vec::Vec<::ethers::core::types::Bytes>,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Bytes>,
        > {
            self.0
                .method_hash([172, 150, 80, 216], data)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `name` (0x06fdde03) function
        pub fn name(&self) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ownerOf` (0x6352211e) function
        pub fn owner_of(
            &self,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([99, 82, 33, 30], token_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `permit` (0x7ac2ff7b) function
        pub fn permit(
            &self,
            spender: ::ethers::core::types::Address,
            token_id: ::ethers::core::types::U256,
            deadline: ::ethers::core::types::U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([122, 194, 255, 123], (spender, token_id, deadline, v, r, s))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `positions` (0x99fbab88) function
        pub fn positions(
            &self,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                u128,
                ::ethers::core::types::Address,
                ::ethers::core::types::Address,
                ::ethers::core::types::Address,
                u32,
                i32,
                i32,
                u128,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                u128,
                u128,
            ),
        > {
            self.0
                .method_hash([153, 251, 171, 136], token_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `refundETH` (0x12210e8a) function
        pub fn refund_eth(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([18, 33, 14, 138], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `safeTransferFrom` (0x42842e0e) function
        pub fn safe_transfer_from(
            &self,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([66, 132, 46, 14], (from, to, token_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `safeTransferFrom` (0xb88d4fde) function
        pub fn safe_transfer_from_with_from_and_to_and_data(
            &self,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            token_id: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([184, 141, 79, 222], (from, to, token_id, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `selfPermit` (0xf3995c67) function
        pub fn self_permit(
            &self,
            token: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            deadline: ::ethers::core::types::U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([243, 153, 92, 103], (token, value, deadline, v, r, s))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `selfPermitAllowed` (0x4659a494) function
        pub fn self_permit_allowed(
            &self,
            token: ::ethers::core::types::Address,
            nonce: ::ethers::core::types::U256,
            expiry: ::ethers::core::types::U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([70, 89, 164, 148], (token, nonce, expiry, v, r, s))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `selfPermitAllowedIfNecessary` (0xa4a78f0c) function
        pub fn self_permit_allowed_if_necessary(
            &self,
            token: ::ethers::core::types::Address,
            nonce: ::ethers::core::types::U256,
            expiry: ::ethers::core::types::U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([164, 167, 143, 12], (token, nonce, expiry, v, r, s))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `selfPermitIfNecessary` (0xc2e3140a) function
        pub fn self_permit_if_necessary(
            &self,
            token: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            deadline: ::ethers::core::types::U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([194, 227, 20, 10], (token, value, deadline, v, r, s))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setApprovalForAll` (0xa22cb465) function
        pub fn set_approval_for_all(
            &self,
            operator: ::ethers::core::types::Address,
            approved: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([162, 44, 180, 101], (operator, approved))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setTime` (0x3beb26c4) function
        pub fn set_time(
            &self,
            time: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([59, 235, 38, 196], time)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `supportsInterface` (0x01ffc9a7) function
        pub fn supports_interface(
            &self,
            interface_id: [u8; 4],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([1, 255, 201, 167], interface_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sweepToken` (0xdf2ab5bb) function
        pub fn sweep_token(
            &self,
            token: ::ethers::core::types::Address,
            amount_minimum: ::ethers::core::types::U256,
            recipient: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([223, 42, 181, 187], (token, amount_minimum, recipient))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `symbol` (0x95d89b41) function
        pub fn symbol(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([149, 216, 155, 65], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `tokenByIndex` (0x4f6ccce7) function
        pub fn token_by_index(
            &self,
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([79, 108, 204, 231], index)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `tokenOfOwnerByIndex` (0x2f745c59) function
        pub fn token_of_owner_by_index(
            &self,
            owner: ::ethers::core::types::Address,
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([47, 116, 92, 89], (owner, index))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `tokenURI` (0xc87b56dd) function
        pub fn token_uri(
            &self,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([200, 123, 86, 221], token_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `totalSupply` (0x18160ddd) function
        pub fn total_supply(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([24, 22, 13, 221], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferFrom` (0x23b872dd) function
        pub fn transfer_from(
            &self,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([35, 184, 114, 221], (from, to, token_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `uniswapV3MintCallback` (0xd3487997) function
        pub fn uniswap_v3_mint_callback(
            &self,
            amount_0_owed: ::ethers::core::types::U256,
            amount_1_owed: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([211, 72, 121, 151], (amount_0_owed, amount_1_owed, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `unwrapWETH9` (0x49404b7c) function
        pub fn unwrap_weth9(
            &self,
            amount_minimum: ::ethers::core::types::U256,
            recipient: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([73, 64, 75, 124], (amount_minimum, recipient))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Approval` event
        pub fn approval_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ApprovalFilter> {
            self.0.event()
        }
        ///Gets the contract's `ApprovalForAll` event
        pub fn approval_for_all_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ApprovalForAllFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `Collect` event
        pub fn collect_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, CollectFilter> {
            self.0.event()
        }
        ///Gets the contract's `DecreaseLiquidity` event
        pub fn decrease_liquidity_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, DecreaseLiquidityFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `IncreaseLiquidity` event
        pub fn increase_liquidity_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, IncreaseLiquidityFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `Transfer` event
        pub fn transfer_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, TransferFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            MockTimeNonfungiblePositionManagerEvents,
        > {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for MockTimeNonfungiblePositionManager<M>
    {
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
        Hash,
    )]
    #[ethevent(name = "Approval", abi = "Approval(address,address,uint256)")]
    pub struct ApprovalFilter {
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub approved: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token_id: ::ethers::core::types::U256,
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
        Hash,
    )]
    #[ethevent(name = "ApprovalForAll", abi = "ApprovalForAll(address,address,bool)")]
    pub struct ApprovalForAllFilter {
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
        pub approved: bool,
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
        Hash,
    )]
    #[ethevent(name = "Collect", abi = "Collect(uint256,address,uint256,uint256)")]
    pub struct CollectFilter {
        #[ethevent(indexed)]
        pub token_id: ::ethers::core::types::U256,
        pub recipient: ::ethers::core::types::Address,
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
        Hash,
    )]
    #[ethevent(
        name = "DecreaseLiquidity",
        abi = "DecreaseLiquidity(uint256,uint128,uint256,uint256)"
    )]
    pub struct DecreaseLiquidityFilter {
        #[ethevent(indexed)]
        pub token_id: ::ethers::core::types::U256,
        pub liquidity: u128,
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
        Hash,
    )]
    #[ethevent(
        name = "IncreaseLiquidity",
        abi = "IncreaseLiquidity(uint256,uint128,uint256,uint256)"
    )]
    pub struct IncreaseLiquidityFilter {
        #[ethevent(indexed)]
        pub token_id: ::ethers::core::types::U256,
        pub liquidity: u128,
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
        Hash,
    )]
    #[ethevent(name = "Transfer", abi = "Transfer(address,address,uint256)")]
    pub struct TransferFilter {
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token_id: ::ethers::core::types::U256,
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
        Hash,
    )]
    pub enum MockTimeNonfungiblePositionManagerEvents {
        ApprovalFilter(ApprovalFilter),
        ApprovalForAllFilter(ApprovalForAllFilter),
        CollectFilter(CollectFilter),
        DecreaseLiquidityFilter(DecreaseLiquidityFilter),
        IncreaseLiquidityFilter(IncreaseLiquidityFilter),
        TransferFilter(TransferFilter),
    }
    impl ::ethers::contract::EthLogDecode for MockTimeNonfungiblePositionManagerEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(MockTimeNonfungiblePositionManagerEvents::ApprovalFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = ApprovalForAllFilter::decode_log(log) {
                return Ok(MockTimeNonfungiblePositionManagerEvents::ApprovalForAllFilter(decoded));
            }
            if let Ok(decoded) = CollectFilter::decode_log(log) {
                return Ok(MockTimeNonfungiblePositionManagerEvents::CollectFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = DecreaseLiquidityFilter::decode_log(log) {
                return Ok(
                    MockTimeNonfungiblePositionManagerEvents::DecreaseLiquidityFilter(decoded),
                );
            }
            if let Ok(decoded) = IncreaseLiquidityFilter::decode_log(log) {
                return Ok(
                    MockTimeNonfungiblePositionManagerEvents::IncreaseLiquidityFilter(decoded),
                );
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(MockTimeNonfungiblePositionManagerEvents::TransferFilter(
                    decoded,
                ));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for MockTimeNonfungiblePositionManagerEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ApprovalFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ApprovalForAllFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::CollectFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::DecreaseLiquidityFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::IncreaseLiquidityFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ApprovalFilter> for MockTimeNonfungiblePositionManagerEvents {
        fn from(value: ApprovalFilter) -> Self {
            Self::ApprovalFilter(value)
        }
    }
    impl ::core::convert::From<ApprovalForAllFilter> for MockTimeNonfungiblePositionManagerEvents {
        fn from(value: ApprovalForAllFilter) -> Self {
            Self::ApprovalForAllFilter(value)
        }
    }
    impl ::core::convert::From<CollectFilter> for MockTimeNonfungiblePositionManagerEvents {
        fn from(value: CollectFilter) -> Self {
            Self::CollectFilter(value)
        }
    }
    impl ::core::convert::From<DecreaseLiquidityFilter> for MockTimeNonfungiblePositionManagerEvents {
        fn from(value: DecreaseLiquidityFilter) -> Self {
            Self::DecreaseLiquidityFilter(value)
        }
    }
    impl ::core::convert::From<IncreaseLiquidityFilter> for MockTimeNonfungiblePositionManagerEvents {
        fn from(value: IncreaseLiquidityFilter) -> Self {
            Self::IncreaseLiquidityFilter(value)
        }
    }
    impl ::core::convert::From<TransferFilter> for MockTimeNonfungiblePositionManagerEvents {
        fn from(value: TransferFilter) -> Self {
            Self::TransferFilter(value)
        }
    }
    ///Container type for all input parameters for the `DOMAIN_SEPARATOR` function with signature `DOMAIN_SEPARATOR()` and selector `0x3644e515`
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
        Hash,
    )]
    #[ethcall(name = "DOMAIN_SEPARATOR", abi = "DOMAIN_SEPARATOR()")]
    pub struct DomainSeparatorCall;
    ///Container type for all input parameters for the `PERMIT_TYPEHASH` function with signature `PERMIT_TYPEHASH()` and selector `0x30adf81f`
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
        Hash,
    )]
    #[ethcall(name = "PERMIT_TYPEHASH", abi = "PERMIT_TYPEHASH()")]
    pub struct PermitTypehashCall;
    ///Container type for all input parameters for the `WETH9` function with signature `WETH9()` and selector `0x4aa4a4fc`
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
        Hash,
    )]
    #[ethcall(name = "WETH9", abi = "WETH9()")]
    pub struct Weth9Call;
    ///Container type for all input parameters for the `approve` function with signature `approve(address,uint256)` and selector `0x095ea7b3`
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
        Hash,
    )]
    #[ethcall(name = "approve", abi = "approve(address,uint256)")]
    pub struct ApproveCall {
        pub to: ::ethers::core::types::Address,
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
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
        Hash,
    )]
    #[ethcall(name = "balanceOf", abi = "balanceOf(address)")]
    pub struct BalanceOfCall {
        pub owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `baseURI` function with signature `baseURI()` and selector `0x6c0360eb`
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
        Hash,
    )]
    #[ethcall(name = "baseURI", abi = "baseURI()")]
    pub struct BaseURICall;
    ///Container type for all input parameters for the `burn` function with signature `burn(uint256)` and selector `0x42966c68`
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
        Hash,
    )]
    #[ethcall(name = "burn", abi = "burn(uint256)")]
    pub struct BurnCall {
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `collect` function with signature `collect((uint256,address,uint128,uint128))` and selector `0xfc6f7865`
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
        Hash,
    )]
    #[ethcall(name = "collect", abi = "collect((uint256,address,uint128,uint128))")]
    pub struct CollectCall {
        pub params: CollectParams,
    }
    ///Container type for all input parameters for the `createAndInitializePoolIfNecessary` function with signature `createAndInitializePoolIfNecessary(address,address,uint24,uint160)` and selector `0x13ead562`
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
        Hash,
    )]
    #[ethcall(
        name = "createAndInitializePoolIfNecessary",
        abi = "createAndInitializePoolIfNecessary(address,address,uint24,uint160)"
    )]
    pub struct CreateAndInitializePoolIfNecessaryCall {
        pub token_0: ::ethers::core::types::Address,
        pub token_1: ::ethers::core::types::Address,
        pub fee: u32,
        pub sqrt_price_x96: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `decreaseLiquidity` function with signature `decreaseLiquidity((uint256,uint128,uint256,uint256,uint256))` and selector `0x0c49ccbe`
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
        Hash,
    )]
    #[ethcall(
        name = "decreaseLiquidity",
        abi = "decreaseLiquidity((uint256,uint128,uint256,uint256,uint256))"
    )]
    pub struct DecreaseLiquidityCall {
        pub params: DecreaseLiquidityParams,
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
        Hash,
    )]
    #[ethcall(name = "factory", abi = "factory()")]
    pub struct FactoryCall;
    ///Container type for all input parameters for the `getApproved` function with signature `getApproved(uint256)` and selector `0x081812fc`
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
        Hash,
    )]
    #[ethcall(name = "getApproved", abi = "getApproved(uint256)")]
    pub struct GetApprovedCall {
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `increaseLiquidity` function with signature `increaseLiquidity((uint256,uint256,uint256,uint256,uint256,uint256))` and selector `0x219f5d17`
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
        Hash,
    )]
    #[ethcall(
        name = "increaseLiquidity",
        abi = "increaseLiquidity((uint256,uint256,uint256,uint256,uint256,uint256))"
    )]
    pub struct IncreaseLiquidityCall {
        pub params: IncreaseLiquidityParams,
    }
    ///Container type for all input parameters for the `isApprovedForAll` function with signature `isApprovedForAll(address,address)` and selector `0xe985e9c5`
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
        Hash,
    )]
    #[ethcall(name = "isApprovedForAll", abi = "isApprovedForAll(address,address)")]
    pub struct IsApprovedForAllCall {
        pub owner: ::ethers::core::types::Address,
        pub operator: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `mint` function with signature `mint((address,address,uint24,int24,int24,uint256,uint256,uint256,uint256,address,uint256))` and selector `0x88316456`
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
        Hash,
    )]
    #[ethcall(
        name = "mint",
        abi = "mint((address,address,uint24,int24,int24,uint256,uint256,uint256,uint256,address,uint256))"
    )]
    pub struct MintCall {
        pub params: MintParams,
    }
    ///Container type for all input parameters for the `multicall` function with signature `multicall(bytes[])` and selector `0xac9650d8`
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
        Hash,
    )]
    #[ethcall(name = "multicall", abi = "multicall(bytes[])")]
    pub struct MulticallCall {
        pub data: ::std::vec::Vec<::ethers::core::types::Bytes>,
    }
    ///Container type for all input parameters for the `name` function with signature `name()` and selector `0x06fdde03`
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
        Hash,
    )]
    #[ethcall(name = "name", abi = "name()")]
    pub struct NameCall;
    ///Container type for all input parameters for the `ownerOf` function with signature `ownerOf(uint256)` and selector `0x6352211e`
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
        Hash,
    )]
    #[ethcall(name = "ownerOf", abi = "ownerOf(uint256)")]
    pub struct OwnerOfCall {
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `permit` function with signature `permit(address,uint256,uint256,uint8,bytes32,bytes32)` and selector `0x7ac2ff7b`
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
        Hash,
    )]
    #[ethcall(
        name = "permit",
        abi = "permit(address,uint256,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct PermitCall {
        pub spender: ::ethers::core::types::Address,
        pub token_id: ::ethers::core::types::U256,
        pub deadline: ::ethers::core::types::U256,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    ///Container type for all input parameters for the `positions` function with signature `positions(uint256)` and selector `0x99fbab88`
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
        Hash,
    )]
    #[ethcall(name = "positions", abi = "positions(uint256)")]
    pub struct PositionsCall {
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `refundETH` function with signature `refundETH()` and selector `0x12210e8a`
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
        Hash,
    )]
    #[ethcall(name = "refundETH", abi = "refundETH()")]
    pub struct RefundETHCall;
    ///Container type for all input parameters for the `safeTransferFrom` function with signature `safeTransferFrom(address,address,uint256)` and selector `0x42842e0e`
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
        Hash,
    )]
    #[ethcall(
        name = "safeTransferFrom",
        abi = "safeTransferFrom(address,address,uint256)"
    )]
    pub struct SafeTransferFromCall {
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `safeTransferFrom` function with signature `safeTransferFrom(address,address,uint256,bytes)` and selector `0xb88d4fde`
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
        Hash,
    )]
    #[ethcall(
        name = "safeTransferFrom",
        abi = "safeTransferFrom(address,address,uint256,bytes)"
    )]
    pub struct SafeTransferFromWithFromAndToAndDataCall {
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub token_id: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `selfPermit` function with signature `selfPermit(address,uint256,uint256,uint8,bytes32,bytes32)` and selector `0xf3995c67`
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
        Hash,
    )]
    #[ethcall(
        name = "selfPermit",
        abi = "selfPermit(address,uint256,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct SelfPermitCall {
        pub token: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub deadline: ::ethers::core::types::U256,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    ///Container type for all input parameters for the `selfPermitAllowed` function with signature `selfPermitAllowed(address,uint256,uint256,uint8,bytes32,bytes32)` and selector `0x4659a494`
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
        Hash,
    )]
    #[ethcall(
        name = "selfPermitAllowed",
        abi = "selfPermitAllowed(address,uint256,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct SelfPermitAllowedCall {
        pub token: ::ethers::core::types::Address,
        pub nonce: ::ethers::core::types::U256,
        pub expiry: ::ethers::core::types::U256,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    ///Container type for all input parameters for the `selfPermitAllowedIfNecessary` function with signature `selfPermitAllowedIfNecessary(address,uint256,uint256,uint8,bytes32,bytes32)` and selector `0xa4a78f0c`
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
        Hash,
    )]
    #[ethcall(
        name = "selfPermitAllowedIfNecessary",
        abi = "selfPermitAllowedIfNecessary(address,uint256,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct SelfPermitAllowedIfNecessaryCall {
        pub token: ::ethers::core::types::Address,
        pub nonce: ::ethers::core::types::U256,
        pub expiry: ::ethers::core::types::U256,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    ///Container type for all input parameters for the `selfPermitIfNecessary` function with signature `selfPermitIfNecessary(address,uint256,uint256,uint8,bytes32,bytes32)` and selector `0xc2e3140a`
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
        Hash,
    )]
    #[ethcall(
        name = "selfPermitIfNecessary",
        abi = "selfPermitIfNecessary(address,uint256,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct SelfPermitIfNecessaryCall {
        pub token: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub deadline: ::ethers::core::types::U256,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    ///Container type for all input parameters for the `setApprovalForAll` function with signature `setApprovalForAll(address,bool)` and selector `0xa22cb465`
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
        Hash,
    )]
    #[ethcall(name = "setApprovalForAll", abi = "setApprovalForAll(address,bool)")]
    pub struct SetApprovalForAllCall {
        pub operator: ::ethers::core::types::Address,
        pub approved: bool,
    }
    ///Container type for all input parameters for the `setTime` function with signature `setTime(uint256)` and selector `0x3beb26c4`
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
        Hash,
    )]
    #[ethcall(name = "setTime", abi = "setTime(uint256)")]
    pub struct SetTimeCall {
        pub time: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`
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
        Hash,
    )]
    #[ethcall(name = "supportsInterface", abi = "supportsInterface(bytes4)")]
    pub struct SupportsInterfaceCall {
        pub interface_id: [u8; 4],
    }
    ///Container type for all input parameters for the `sweepToken` function with signature `sweepToken(address,uint256,address)` and selector `0xdf2ab5bb`
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
        Hash,
    )]
    #[ethcall(name = "sweepToken", abi = "sweepToken(address,uint256,address)")]
    pub struct SweepTokenCall {
        pub token: ::ethers::core::types::Address,
        pub amount_minimum: ::ethers::core::types::U256,
        pub recipient: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `symbol` function with signature `symbol()` and selector `0x95d89b41`
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
        Hash,
    )]
    #[ethcall(name = "symbol", abi = "symbol()")]
    pub struct SymbolCall;
    ///Container type for all input parameters for the `tokenByIndex` function with signature `tokenByIndex(uint256)` and selector `0x4f6ccce7`
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
        Hash,
    )]
    #[ethcall(name = "tokenByIndex", abi = "tokenByIndex(uint256)")]
    pub struct TokenByIndexCall {
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `tokenOfOwnerByIndex` function with signature `tokenOfOwnerByIndex(address,uint256)` and selector `0x2f745c59`
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
        Hash,
    )]
    #[ethcall(
        name = "tokenOfOwnerByIndex",
        abi = "tokenOfOwnerByIndex(address,uint256)"
    )]
    pub struct TokenOfOwnerByIndexCall {
        pub owner: ::ethers::core::types::Address,
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `tokenURI` function with signature `tokenURI(uint256)` and selector `0xc87b56dd`
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
        Hash,
    )]
    #[ethcall(name = "tokenURI", abi = "tokenURI(uint256)")]
    pub struct TokenURICall {
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `totalSupply` function with signature `totalSupply()` and selector `0x18160ddd`
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
        Hash,
    )]
    #[ethcall(name = "totalSupply", abi = "totalSupply()")]
    pub struct TotalSupplyCall;
    ///Container type for all input parameters for the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `0x23b872dd`
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
        Hash,
    )]
    #[ethcall(name = "transferFrom", abi = "transferFrom(address,address,uint256)")]
    pub struct TransferFromCall {
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `uniswapV3MintCallback` function with signature `uniswapV3MintCallback(uint256,uint256,bytes)` and selector `0xd3487997`
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
        Hash,
    )]
    #[ethcall(
        name = "uniswapV3MintCallback",
        abi = "uniswapV3MintCallback(uint256,uint256,bytes)"
    )]
    pub struct UniswapV3MintCallbackCall {
        pub amount_0_owed: ::ethers::core::types::U256,
        pub amount_1_owed: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `unwrapWETH9` function with signature `unwrapWETH9(uint256,address)` and selector `0x49404b7c`
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
        Hash,
    )]
    #[ethcall(name = "unwrapWETH9", abi = "unwrapWETH9(uint256,address)")]
    pub struct UnwrapWETH9Call {
        pub amount_minimum: ::ethers::core::types::U256,
        pub recipient: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub enum MockTimeNonfungiblePositionManagerCalls {
        DomainSeparator(DomainSeparatorCall),
        PermitTypehash(PermitTypehashCall),
        Weth9(Weth9Call),
        Approve(ApproveCall),
        BalanceOf(BalanceOfCall),
        BaseURI(BaseURICall),
        Burn(BurnCall),
        Collect(CollectCall),
        CreateAndInitializePoolIfNecessary(CreateAndInitializePoolIfNecessaryCall),
        DecreaseLiquidity(DecreaseLiquidityCall),
        Factory(FactoryCall),
        GetApproved(GetApprovedCall),
        IncreaseLiquidity(IncreaseLiquidityCall),
        IsApprovedForAll(IsApprovedForAllCall),
        Mint(MintCall),
        Multicall(MulticallCall),
        Name(NameCall),
        OwnerOf(OwnerOfCall),
        Permit(PermitCall),
        Positions(PositionsCall),
        RefundETH(RefundETHCall),
        SafeTransferFrom(SafeTransferFromCall),
        SafeTransferFromWithFromAndToAndData(SafeTransferFromWithFromAndToAndDataCall),
        SelfPermit(SelfPermitCall),
        SelfPermitAllowed(SelfPermitAllowedCall),
        SelfPermitAllowedIfNecessary(SelfPermitAllowedIfNecessaryCall),
        SelfPermitIfNecessary(SelfPermitIfNecessaryCall),
        SetApprovalForAll(SetApprovalForAllCall),
        SetTime(SetTimeCall),
        SupportsInterface(SupportsInterfaceCall),
        SweepToken(SweepTokenCall),
        Symbol(SymbolCall),
        TokenByIndex(TokenByIndexCall),
        TokenOfOwnerByIndex(TokenOfOwnerByIndexCall),
        TokenURI(TokenURICall),
        TotalSupply(TotalSupplyCall),
        TransferFrom(TransferFromCall),
        UniswapV3MintCallback(UniswapV3MintCallbackCall),
        UnwrapWETH9(UnwrapWETH9Call),
    }
    impl ::ethers::core::abi::AbiDecode for MockTimeNonfungiblePositionManagerCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <DomainSeparatorCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DomainSeparator(decoded));
            }
            if let Ok(decoded) =
                <PermitTypehashCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PermitTypehash(decoded));
            }
            if let Ok(decoded) = <Weth9Call as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Weth9(decoded));
            }
            if let Ok(decoded) = <ApproveCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Approve(decoded));
            }
            if let Ok(decoded) = <BalanceOfCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BalanceOf(decoded));
            }
            if let Ok(decoded) = <BaseURICall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BaseURI(decoded));
            }
            if let Ok(decoded) = <BurnCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Burn(decoded));
            }
            if let Ok(decoded) = <CollectCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Collect(decoded));
            }
            if let Ok(decoded) =
                <CreateAndInitializePoolIfNecessaryCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::CreateAndInitializePoolIfNecessary(decoded));
            }
            if let Ok(decoded) =
                <DecreaseLiquidityCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DecreaseLiquidity(decoded));
            }
            if let Ok(decoded) = <FactoryCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Factory(decoded));
            }
            if let Ok(decoded) = <GetApprovedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetApproved(decoded));
            }
            if let Ok(decoded) =
                <IncreaseLiquidityCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::IncreaseLiquidity(decoded));
            }
            if let Ok(decoded) =
                <IsApprovedForAllCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::IsApprovedForAll(decoded));
            }
            if let Ok(decoded) = <MintCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Mint(decoded));
            }
            if let Ok(decoded) = <MulticallCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Multicall(decoded));
            }
            if let Ok(decoded) = <NameCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Name(decoded));
            }
            if let Ok(decoded) = <OwnerOfCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::OwnerOf(decoded));
            }
            if let Ok(decoded) = <PermitCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Permit(decoded));
            }
            if let Ok(decoded) = <PositionsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Positions(decoded));
            }
            if let Ok(decoded) = <RefundETHCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RefundETH(decoded));
            }
            if let Ok(decoded) =
                <SafeTransferFromCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SafeTransferFrom(decoded));
            }
            if let Ok(decoded) =
                <SafeTransferFromWithFromAndToAndDataCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::SafeTransferFromWithFromAndToAndData(decoded));
            }
            if let Ok(decoded) = <SelfPermitCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SelfPermit(decoded));
            }
            if let Ok(decoded) =
                <SelfPermitAllowedCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SelfPermitAllowed(decoded));
            }
            if let Ok(decoded) =
                <SelfPermitAllowedIfNecessaryCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SelfPermitAllowedIfNecessary(decoded));
            }
            if let Ok(decoded) =
                <SelfPermitIfNecessaryCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SelfPermitIfNecessary(decoded));
            }
            if let Ok(decoded) =
                <SetApprovalForAllCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetApprovalForAll(decoded));
            }
            if let Ok(decoded) = <SetTimeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetTime(decoded));
            }
            if let Ok(decoded) =
                <SupportsInterfaceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SupportsInterface(decoded));
            }
            if let Ok(decoded) = <SweepTokenCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SweepToken(decoded));
            }
            if let Ok(decoded) = <SymbolCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Symbol(decoded));
            }
            if let Ok(decoded) = <TokenByIndexCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TokenByIndex(decoded));
            }
            if let Ok(decoded) =
                <TokenOfOwnerByIndexCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TokenOfOwnerByIndex(decoded));
            }
            if let Ok(decoded) = <TokenURICall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TokenURI(decoded));
            }
            if let Ok(decoded) = <TotalSupplyCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TotalSupply(decoded));
            }
            if let Ok(decoded) = <TransferFromCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TransferFrom(decoded));
            }
            if let Ok(decoded) =
                <UniswapV3MintCallbackCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UniswapV3MintCallback(decoded));
            }
            if let Ok(decoded) = <UnwrapWETH9Call as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UnwrapWETH9(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MockTimeNonfungiblePositionManagerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::DomainSeparator(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PermitTypehash(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Weth9(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Approve(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BalanceOf(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BaseURI(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Burn(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Collect(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CreateAndInitializePoolIfNecessary(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DecreaseLiquidity(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Factory(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetApproved(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IncreaseLiquidity(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsApprovedForAll(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Mint(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Multicall(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Name(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OwnerOf(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Permit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Positions(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RefundETH(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SafeTransferFrom(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SafeTransferFromWithFromAndToAndData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SelfPermit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SelfPermitAllowed(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SelfPermitAllowedIfNecessary(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SelfPermitIfNecessary(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetApprovalForAll(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetTime(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SupportsInterface(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SweepToken(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Symbol(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TokenByIndex(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TokenOfOwnerByIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TokenURI(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TotalSupply(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TransferFrom(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UniswapV3MintCallback(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnwrapWETH9(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for MockTimeNonfungiblePositionManagerCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DomainSeparator(element) => ::core::fmt::Display::fmt(element, f),
                Self::PermitTypehash(element) => ::core::fmt::Display::fmt(element, f),
                Self::Weth9(element) => ::core::fmt::Display::fmt(element, f),
                Self::Approve(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::BaseURI(element) => ::core::fmt::Display::fmt(element, f),
                Self::Burn(element) => ::core::fmt::Display::fmt(element, f),
                Self::Collect(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreateAndInitializePoolIfNecessary(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DecreaseLiquidity(element) => ::core::fmt::Display::fmt(element, f),
                Self::Factory(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetApproved(element) => ::core::fmt::Display::fmt(element, f),
                Self::IncreaseLiquidity(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsApprovedForAll(element) => ::core::fmt::Display::fmt(element, f),
                Self::Mint(element) => ::core::fmt::Display::fmt(element, f),
                Self::Multicall(element) => ::core::fmt::Display::fmt(element, f),
                Self::Name(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnerOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::Permit(element) => ::core::fmt::Display::fmt(element, f),
                Self::Positions(element) => ::core::fmt::Display::fmt(element, f),
                Self::RefundETH(element) => ::core::fmt::Display::fmt(element, f),
                Self::SafeTransferFrom(element) => ::core::fmt::Display::fmt(element, f),
                Self::SafeTransferFromWithFromAndToAndData(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SelfPermit(element) => ::core::fmt::Display::fmt(element, f),
                Self::SelfPermitAllowed(element) => ::core::fmt::Display::fmt(element, f),
                Self::SelfPermitAllowedIfNecessary(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SelfPermitIfNecessary(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetApprovalForAll(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetTime(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupportsInterface(element) => ::core::fmt::Display::fmt(element, f),
                Self::SweepToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::Symbol(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenByIndex(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenOfOwnerByIndex(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenURI(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalSupply(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFrom(element) => ::core::fmt::Display::fmt(element, f),
                Self::UniswapV3MintCallback(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnwrapWETH9(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DomainSeparatorCall> for MockTimeNonfungiblePositionManagerCalls {
        fn from(value: DomainSeparatorCall) -> Self {
            Self::DomainSeparator(value)
        }
    }
    impl ::core::convert::From<PermitTypehashCall> for MockTimeNonfungiblePositionManagerCalls {
        fn from(value: PermitTypehashCall) -> Self {
            Self::PermitTypehash(value)
        }
    }
    impl ::core::convert::From<Weth9Call> for MockTimeNonfungiblePositionManagerCalls {
        fn from(value: Weth9Call) -> Self {
            Self::Weth9(value)
        }
    }
    impl ::core::convert::From<ApproveCall> for MockTimeNonfungiblePositionManagerCalls {
        fn from(value: ApproveCall) -> Self {
            Self::Approve(value)
        }
    }
    impl ::core::convert::From<BalanceOfCall> for MockTimeNonfungiblePositionManagerCalls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<BaseURICall> for MockTimeNonfungiblePositionManagerCalls {
        fn from(value: BaseURICall) -> Self {
            Self::BaseURI(value)
        }
    }
    impl ::core::convert::From<BurnCall> for MockTimeNonfungiblePositionManagerCalls {
        fn from(value: BurnCall) -> Self {
            Self::Burn(value)
        }
    }
    impl ::core::convert::From<CollectCall> for MockTimeNonfungiblePositionManagerCalls {
        fn from(value: CollectCall) -> Self {
            Self::Collect(value)
        }
    }
    impl ::core::convert::From<CreateAndInitializePoolIfNecessaryCall>
        for MockTimeNonfungiblePositionManagerCalls
    {
        fn from(value: CreateAndInitializePoolIfNecessaryCall) -> Self {
            Self::CreateAndInitializePoolIfNecessary(value)
        }
    }
    impl ::core::convert::From<DecreaseLiquidityCall> for MockTimeNonfungiblePositionManagerCalls {
        fn from(value: DecreaseLiquidityCall) -> Self {
            Self::DecreaseLiquidity(value)
        }
    }
    impl ::core::convert::From<FactoryCall> for MockTimeNonfungiblePositionManagerCalls {
        fn from(value: FactoryCall) -> Self {
            Self::Factory(value)
        }
    }
    impl ::core::convert::From<GetApprovedCall> for MockTimeNonfungiblePositionManagerCalls {
        fn from(value: GetApprovedCall) -> Self {
            Self::GetApproved(value)
        }
    }
    impl ::core::convert::From<IncreaseLiquidityCall> for MockTimeNonfungiblePositionManagerCalls {
        fn from(value: IncreaseLiquidityCall) -> Self {
            Self::IncreaseLiquidity(value)
        }
    }
    impl ::core::convert::From<IsApprovedForAllCall> for MockTimeNonfungiblePositionManagerCalls {
        fn from(value: IsApprovedForAllCall) -> Self {
            Self::IsApprovedForAll(value)
        }
    }
    impl ::core::convert::From<MintCall> for MockTimeNonfungiblePositionManagerCalls {
        fn from(value: MintCall) -> Self {
            Self::Mint(value)
        }
    }
    impl ::core::convert::From<MulticallCall> for MockTimeNonfungiblePositionManagerCalls {
        fn from(value: MulticallCall) -> Self {
            Self::Multicall(value)
        }
    }
    impl ::core::convert::From<NameCall> for MockTimeNonfungiblePositionManagerCalls {
        fn from(value: NameCall) -> Self {
            Self::Name(value)
        }
    }
    impl ::core::convert::From<OwnerOfCall> for MockTimeNonfungiblePositionManagerCalls {
        fn from(value: OwnerOfCall) -> Self {
            Self::OwnerOf(value)
        }
    }
    impl ::core::convert::From<PermitCall> for MockTimeNonfungiblePositionManagerCalls {
        fn from(value: PermitCall) -> Self {
            Self::Permit(value)
        }
    }
    impl ::core::convert::From<PositionsCall> for MockTimeNonfungiblePositionManagerCalls {
        fn from(value: PositionsCall) -> Self {
            Self::Positions(value)
        }
    }
    impl ::core::convert::From<RefundETHCall> for MockTimeNonfungiblePositionManagerCalls {
        fn from(value: RefundETHCall) -> Self {
            Self::RefundETH(value)
        }
    }
    impl ::core::convert::From<SafeTransferFromCall> for MockTimeNonfungiblePositionManagerCalls {
        fn from(value: SafeTransferFromCall) -> Self {
            Self::SafeTransferFrom(value)
        }
    }
    impl ::core::convert::From<SafeTransferFromWithFromAndToAndDataCall>
        for MockTimeNonfungiblePositionManagerCalls
    {
        fn from(value: SafeTransferFromWithFromAndToAndDataCall) -> Self {
            Self::SafeTransferFromWithFromAndToAndData(value)
        }
    }
    impl ::core::convert::From<SelfPermitCall> for MockTimeNonfungiblePositionManagerCalls {
        fn from(value: SelfPermitCall) -> Self {
            Self::SelfPermit(value)
        }
    }
    impl ::core::convert::From<SelfPermitAllowedCall> for MockTimeNonfungiblePositionManagerCalls {
        fn from(value: SelfPermitAllowedCall) -> Self {
            Self::SelfPermitAllowed(value)
        }
    }
    impl ::core::convert::From<SelfPermitAllowedIfNecessaryCall>
        for MockTimeNonfungiblePositionManagerCalls
    {
        fn from(value: SelfPermitAllowedIfNecessaryCall) -> Self {
            Self::SelfPermitAllowedIfNecessary(value)
        }
    }
    impl ::core::convert::From<SelfPermitIfNecessaryCall> for MockTimeNonfungiblePositionManagerCalls {
        fn from(value: SelfPermitIfNecessaryCall) -> Self {
            Self::SelfPermitIfNecessary(value)
        }
    }
    impl ::core::convert::From<SetApprovalForAllCall> for MockTimeNonfungiblePositionManagerCalls {
        fn from(value: SetApprovalForAllCall) -> Self {
            Self::SetApprovalForAll(value)
        }
    }
    impl ::core::convert::From<SetTimeCall> for MockTimeNonfungiblePositionManagerCalls {
        fn from(value: SetTimeCall) -> Self {
            Self::SetTime(value)
        }
    }
    impl ::core::convert::From<SupportsInterfaceCall> for MockTimeNonfungiblePositionManagerCalls {
        fn from(value: SupportsInterfaceCall) -> Self {
            Self::SupportsInterface(value)
        }
    }
    impl ::core::convert::From<SweepTokenCall> for MockTimeNonfungiblePositionManagerCalls {
        fn from(value: SweepTokenCall) -> Self {
            Self::SweepToken(value)
        }
    }
    impl ::core::convert::From<SymbolCall> for MockTimeNonfungiblePositionManagerCalls {
        fn from(value: SymbolCall) -> Self {
            Self::Symbol(value)
        }
    }
    impl ::core::convert::From<TokenByIndexCall> for MockTimeNonfungiblePositionManagerCalls {
        fn from(value: TokenByIndexCall) -> Self {
            Self::TokenByIndex(value)
        }
    }
    impl ::core::convert::From<TokenOfOwnerByIndexCall> for MockTimeNonfungiblePositionManagerCalls {
        fn from(value: TokenOfOwnerByIndexCall) -> Self {
            Self::TokenOfOwnerByIndex(value)
        }
    }
    impl ::core::convert::From<TokenURICall> for MockTimeNonfungiblePositionManagerCalls {
        fn from(value: TokenURICall) -> Self {
            Self::TokenURI(value)
        }
    }
    impl ::core::convert::From<TotalSupplyCall> for MockTimeNonfungiblePositionManagerCalls {
        fn from(value: TotalSupplyCall) -> Self {
            Self::TotalSupply(value)
        }
    }
    impl ::core::convert::From<TransferFromCall> for MockTimeNonfungiblePositionManagerCalls {
        fn from(value: TransferFromCall) -> Self {
            Self::TransferFrom(value)
        }
    }
    impl ::core::convert::From<UniswapV3MintCallbackCall> for MockTimeNonfungiblePositionManagerCalls {
        fn from(value: UniswapV3MintCallbackCall) -> Self {
            Self::UniswapV3MintCallback(value)
        }
    }
    impl ::core::convert::From<UnwrapWETH9Call> for MockTimeNonfungiblePositionManagerCalls {
        fn from(value: UnwrapWETH9Call) -> Self {
            Self::UnwrapWETH9(value)
        }
    }
    ///Container type for all return fields from the `DOMAIN_SEPARATOR` function with signature `DOMAIN_SEPARATOR()` and selector `0x3644e515`
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
    pub struct DomainSeparatorReturn(pub [u8; 32]);
    ///Container type for all return fields from the `PERMIT_TYPEHASH` function with signature `PERMIT_TYPEHASH()` and selector `0x30adf81f`
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
    pub struct PermitTypehashReturn(pub [u8; 32]);
    ///Container type for all return fields from the `WETH9` function with signature `WETH9()` and selector `0x4aa4a4fc`
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
    pub struct Weth9Return(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
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
    pub struct BalanceOfReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `baseURI` function with signature `baseURI()` and selector `0x6c0360eb`
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
    pub struct BaseURIReturn(pub ::std::string::String);
    ///Container type for all return fields from the `collect` function with signature `collect((uint256,address,uint128,uint128))` and selector `0xfc6f7865`
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
    pub struct CollectReturn {
        pub amount_0: ::ethers::core::types::U256,
        pub amount_1: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `createAndInitializePoolIfNecessary` function with signature `createAndInitializePoolIfNecessary(address,address,uint24,uint160)` and selector `0x13ead562`
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
    pub struct CreateAndInitializePoolIfNecessaryReturn {
        pub pool: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `decreaseLiquidity` function with signature `decreaseLiquidity((uint256,uint128,uint256,uint256,uint256))` and selector `0x0c49ccbe`
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
    pub struct DecreaseLiquidityReturn {
        pub amount_0: ::ethers::core::types::U256,
        pub amount_1: ::ethers::core::types::U256,
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
        Hash,
    )]
    pub struct FactoryReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getApproved` function with signature `getApproved(uint256)` and selector `0x081812fc`
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
    pub struct GetApprovedReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `increaseLiquidity` function with signature `increaseLiquidity((uint256,uint256,uint256,uint256,uint256,uint256))` and selector `0x219f5d17`
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
    pub struct IncreaseLiquidityReturn {
        pub liquidity: u128,
        pub amount_0: ::ethers::core::types::U256,
        pub amount_1: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `isApprovedForAll` function with signature `isApprovedForAll(address,address)` and selector `0xe985e9c5`
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
    pub struct IsApprovedForAllReturn(pub bool);
    ///Container type for all return fields from the `mint` function with signature `mint((address,address,uint24,int24,int24,uint256,uint256,uint256,uint256,address,uint256))` and selector `0x88316456`
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
    pub struct MintReturn {
        pub token_id: ::ethers::core::types::U256,
        pub liquidity: u128,
        pub amount_0: ::ethers::core::types::U256,
        pub amount_1: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `multicall` function with signature `multicall(bytes[])` and selector `0xac9650d8`
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
    pub struct MulticallReturn {
        pub results: ::std::vec::Vec<::ethers::core::types::Bytes>,
    }
    ///Container type for all return fields from the `name` function with signature `name()` and selector `0x06fdde03`
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
    pub struct NameReturn(pub ::std::string::String);
    ///Container type for all return fields from the `ownerOf` function with signature `ownerOf(uint256)` and selector `0x6352211e`
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
    pub struct OwnerOfReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `positions` function with signature `positions(uint256)` and selector `0x99fbab88`
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
    pub struct PositionsReturn {
        pub nonce: u128,
        pub operator: ::ethers::core::types::Address,
        pub token_0: ::ethers::core::types::Address,
        pub token_1: ::ethers::core::types::Address,
        pub fee: u32,
        pub tick_lower: i32,
        pub tick_upper: i32,
        pub liquidity: u128,
        pub fee_growth_inside_0_last_x128: ::ethers::core::types::U256,
        pub fee_growth_inside_1_last_x128: ::ethers::core::types::U256,
        pub tokens_owed_0: u128,
        pub tokens_owed_1: u128,
    }
    ///Container type for all return fields from the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`
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
    pub struct SupportsInterfaceReturn(pub bool);
    ///Container type for all return fields from the `symbol` function with signature `symbol()` and selector `0x95d89b41`
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
    pub struct SymbolReturn(pub ::std::string::String);
    ///Container type for all return fields from the `tokenByIndex` function with signature `tokenByIndex(uint256)` and selector `0x4f6ccce7`
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
    pub struct TokenByIndexReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `tokenOfOwnerByIndex` function with signature `tokenOfOwnerByIndex(address,uint256)` and selector `0x2f745c59`
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
    pub struct TokenOfOwnerByIndexReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `tokenURI` function with signature `tokenURI(uint256)` and selector `0xc87b56dd`
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
    pub struct TokenURIReturn(pub ::std::string::String);
    ///Container type for all return fields from the `totalSupply` function with signature `totalSupply()` and selector `0x18160ddd`
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
    pub struct TotalSupplyReturn(pub ::ethers::core::types::U256);
}
