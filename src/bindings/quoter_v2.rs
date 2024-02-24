pub use quoter_v2::*;
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
pub mod quoter_v2 {
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
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("WETH9"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("WETH9"),
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
                    ::std::borrow::ToOwned::to_owned("quoteExactInput"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("quoteExactInput"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("path"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountIn"),
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
                                    name: ::std::borrow::ToOwned::to_owned("amountOut"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "sqrtPriceX96AfterList",
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "initializedTicksCrossedList",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("gasEstimate"),
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
                    ::std::borrow::ToOwned::to_owned("quoteExactInputSingle"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "quoteExactInputSingle",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(160usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IQuoterV2.QuoteExactInputSingleParams",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountOut"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sqrtPriceX96After"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        160usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint160"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "initializedTicksCrossed",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("gasEstimate"),
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
                    ::std::borrow::ToOwned::to_owned("quoteExactOutput"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("quoteExactOutput"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("path"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountOut"),
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
                                    name: ::std::borrow::ToOwned::to_owned("amountIn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "sqrtPriceX96AfterList",
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "initializedTicksCrossedList",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("gasEstimate"),
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
                    ::std::borrow::ToOwned::to_owned("quoteExactOutputSingle"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "quoteExactOutputSingle",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(160usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IQuoterV2.QuoteExactOutputSingleParams",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountIn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sqrtPriceX96After"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        160usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint160"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "initializedTicksCrossed",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("gasEstimate"),
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
                    ::std::borrow::ToOwned::to_owned("uniswapV3SwapCallback"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "uniswapV3SwapCallback",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount0Delta"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount1Delta"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("path"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static QUOTERV2_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xC0`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0\x1B\xF28\x03\x80b\0\x1B\xF2\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\0pV[`\x01`\x01``\x1B\x03\x19``\x92\x83\x1B\x81\x16`\x80R\x91\x1B\x16`\xA0Rb\0\0\xA7V[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\0kW`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15b\0\0\x83W\x81\x82\xFD[b\0\0\x8E\x83b\0\0SV[\x91Pb\0\0\x9E` \x84\x01b\0\0SV[\x90P\x92P\x92\x90PV[`\x80Q``\x1C`\xA0Q``\x1Ca\x1B\x17b\0\0\xDB`\09\x80a\x02\xE0RP\x80a\x04\xD7R\x80a\x087R\x80a\t\xEFRPa\x1B\x17`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0}W`\x005`\xE0\x1C\x80c\xC4Z\x01U\x11a\0[W\x80c\xC4Z\x01U\x14a\0\xE6W\x80c\xC6\xA5\x02j\x14a\0\xEEW\x80c\xCD\xCA\x17S\x14a\x01\x01W\x80c\xFAF\x1E3\x14a\x01\x14Wa\0}V[\x80c/\x80\xBB\x1D\x14a\0\x82W\x80cJ\xA4\xA4\xFC\x14a\0\xAEW\x80c\xBD!pJ\x14a\0\xC3W[`\0\x80\xFD[a\0\x95a\0\x906`\x04a\x16|V[a\x01)V[`@Qa\0\xA5\x94\x93\x92\x91\x90a\x19\x8EV[`@Q\x80\x91\x03\x90\xF3[a\0\xB6a\x02\xDEV[`@Qa\0\xA5\x91\x90a\x18\xF7V[a\0\xD6a\0\xD16`\x04a\x17\x9AV[a\x03\x02V[`@Qa\0\xA5\x94\x93\x92\x91\x90a\x1A)V[a\0\xB6a\x04\xD5V[a\0\xD6a\0\xFC6`\x04a\x17\x9AV[a\x04\xF9V[a\0\x95a\x01\x0F6`\x04a\x16|V[a\x06oV[a\x01'a\x01\"6`\x04a\x16\xE2V[a\x08\x06V[\0[`\0``\x80`\0a\x019\x86a\t\x81V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x80\x15a\x01OW`\0\x80\xFD[P`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x01yW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x92Pa\x01\x85\x86a\t\x81V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x80\x15a\x01\x9BW`\0\x80\xFD[P`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x01\xC5W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x91P`\0[`\0\x80`\0a\x01\xD9\x8Aa\t\x92V[\x92P\x92P\x92P`\0\x80`\0\x80a\x025`@Q\x80`\xA0\x01`@R\x80\x88`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x89`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8F\x81R` \x01\x87b\xFF\xFF\xFF\x16\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81RPa\x03\x02V[\x93P\x93P\x93P\x93P\x82\x8B\x89\x81Q\x81\x10a\x02JW\xFE[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81\x8A\x89\x81Q\x81\x10a\x02wW\xFE[c\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x92\x9BP\x96\x82\x01\x96`\x01\x90\x96\x01\x95\x8B\x92a\x02\xA4\x8Ea\t\xC3V[\x15a\x02\xB9Wa\x02\xB2\x8Ea\t\xCBV[\x9DPa\x02\xC9V[\x8C\x9BPPPPPPPPPa\x02\xD5V[PPPPPPPa\x01\xCBV[\x92\x95\x91\x94P\x92PV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[` \x81\x01Q\x81Q``\x83\x01Q`\0\x92\x83\x92\x83\x92\x83\x92`\x01`\x01`\xA0\x1B\x03\x80\x82\x16\x90\x84\x16\x10\x92\x84\x92a\x033\x92\x90a\t\xE8V[\x90P\x86`\x80\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\0\x14\x15a\x03TW`@\x87\x01Q`\0U[`\0Z\x90P\x81`\x01`\x01`\xA0\x1B\x03\x16c\x12\x8A\xCB\x080\x85a\x03w\x8C`@\x01Qa\n&V[`\0\x03\x8C`\x80\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\0\x14a\x03\x99W\x8C`\x80\x01Qa\x03\xBFV[\x87a\x03\xB8Ws\xFF\xFD\x89c\xEF\xD1\xFCjPd\x88I]\x95\x1DRc\x98\x8D%a\x03\xBFV[d\x01\0\x02v\xA4[\x8D` \x01Q\x8E``\x01Q\x8F`\0\x01Q`@Q` \x01a\x03\xE0\x93\x92\x91\x90a\x18\xBCV[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04\x0F\x95\x94\x93\x92\x91\x90a\x19\x0BV[`@\x80Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04(W`\0\x80\xFD[PZ\xF1\x92PPP\x80\x15a\x04XWP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x04U\x91\x81\x01\x90a\x16\xBFV[`\x01[a\x04\xC8W=\x80\x80\x15a\x04\x86W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x04\x8BV[``\x91P[PZ\x82\x03\x94P\x88`\x80\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\0\x14\x15a\x04\xACW`\0\x80U[a\x04\xB7\x81\x84\x87a\n<V[\x97P\x97P\x97P\x97PPPPPa\x04\xCEV[PPPPP[\x91\x93P\x91\x93V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[` \x81\x01Q\x81Q``\x83\x01Q`\0\x92\x83\x92\x83\x92\x83\x92`\x01`\x01`\xA0\x1B\x03\x80\x82\x16\x90\x84\x16\x10\x92\x84\x92a\x05*\x92\x90a\t\xE8V[\x90P`\0Z\x90P\x81`\x01`\x01`\xA0\x1B\x03\x16c\x12\x8A\xCB\x080\x85a\x05O\x8C`@\x01Qa\n&V[`\x80\x8D\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x15a\x05lW\x8C`\x80\x01Qa\x05\x92V[\x87a\x05\x8BWs\xFF\xFD\x89c\xEF\xD1\xFCjPd\x88I]\x95\x1DRc\x98\x8D%a\x05\x92V[d\x01\0\x02v\xA4[\x8D`\0\x01Q\x8E``\x01Q\x8F` \x01Q`@Q` \x01a\x05\xB3\x93\x92\x91\x90a\x18\xBCV[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x05\xE2\x95\x94\x93\x92\x91\x90a\x19\x0BV[`@\x80Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x05\xFBW`\0\x80\xFD[PZ\xF1\x92PPP\x80\x15a\x06+WP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x06(\x91\x81\x01\x90a\x16\xBFV[`\x01[a\x04\xC8W=\x80\x80\x15a\x06YW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x06^V[``\x91P[PZ\x82\x03\x94Pa\x04\xB7\x81\x84\x87a\n<V[`\0``\x80`\0a\x06\x7F\x86a\t\x81V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x80\x15a\x06\x95W`\0\x80\xFD[P`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x06\xBFW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x92Pa\x06\xCB\x86a\t\x81V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x80\x15a\x06\xE1W`\0\x80\xFD[P`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x07\x0BW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x91P`\0[`\0\x80`\0a\x07\x1F\x8Aa\t\x92V[\x92P\x92P\x92P`\0\x80`\0\x80a\x07{`@Q\x80`\xA0\x01`@R\x80\x89`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x88`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8F\x81R` \x01\x87b\xFF\xFF\xFF\x16\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81RPa\x04\xF9V[\x93P\x93P\x93P\x93P\x82\x8B\x89\x81Q\x81\x10a\x07\x90W\xFE[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81\x8A\x89\x81Q\x81\x10a\x07\xBDW\xFE[c\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x92\x9BP\x96\x82\x01\x96`\x01\x90\x96\x01\x95\x8B\x92a\x07\xEA\x8Ea\t\xC3V[\x15a\x02\xB9Wa\x07\xF8\x8Ea\t\xCBV[\x9DPPPPPPPPa\x07\x11V[`\0\x83\x13\x80a\x08\x15WP`\0\x82\x13[a\x08\x1EW`\0\x80\xFD[`\0\x80`\0a\x08,\x84a\t\x92V[\x92P\x92P\x92Pa\x08^\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84\x84\x84a\n\xF6V[P`\0\x80`\0\x80\x89\x13a\x08\x8AW\x85`\x01`\x01`\xA0\x1B\x03\x16\x85`\x01`\x01`\xA0\x1B\x03\x16\x10\x88\x8A`\0\x03a\x08\xA5V[\x84`\x01`\x01`\xA0\x1B\x03\x16\x86`\x01`\x01`\xA0\x1B\x03\x16\x10\x89\x89`\0\x03[\x92P\x92P\x92P`\0a\x08\xB8\x87\x87\x87a\t\xE8V[\x90P`\0\x80\x82`\x01`\x01`\xA0\x1B\x03\x16c8P\xC7\xBD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\xE0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x08\xF6W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\t\nW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t.\x91\x90a\x17\xBCV[PPPPP\x91P\x91P\x85\x15a\tTW`@Q\x84\x81R\x82` \x82\x01R\x81`@\x82\x01R``\x81\xFD[`\0T\x15a\tjW`\0T\x84\x14a\tjW`\0\x80\xFD[`@Q\x85\x81R\x82` \x82\x01R\x81`@\x82\x01R``\x81\xFD[\x80Q`\x17`\x13\x19\x90\x91\x01\x04[\x91\x90PV[`\0\x80\x80a\t\xA0\x84\x82a\x0B\x15V[\x92Pa\t\xAD\x84`\x14a\x0B\xC5V[\x90Pa\t\xBA\x84`\x17a\x0B\x15V[\x91P\x91\x93\x90\x92PV[Q`B\x11\x15\x90V[\x80Q``\x90a\t\xE2\x90\x83\x90`\x17\x90`\x16\x19\x01a\x0ClV[\x92\x91PPV[`\0a\n\x1E\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\n\x19\x86\x86\x86a\r\xBDV[a\x0E\x13V[\x94\x93PPPPV[`\0`\x01`\xFF\x1B\x82\x10a\n8W`\0\x80\xFD[P\x90V[`\0\x80`\0\x80`\0\x80\x87`\x01`\x01`\xA0\x1B\x03\x16c8P\xC7\xBD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\xE0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\n~W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\n\x92W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xB6\x91\x90a\x17\xBCV[P\x93\x96Pa\n\xCB\x94P\x8D\x93Pa\x0E\xF7\x92PPPV[\x91\x97P\x95P\x90Pa\n\xE6`\x01`\x01`\xA0\x1B\x03\x89\x16\x83\x83a\x0F\x84V[\x93P\x86\x92PPP\x93P\x93P\x93P\x93V[`\0a\x0B\x0C\x85a\x0B\x07\x86\x86\x86a\r\xBDV[a\x15\rV[\x95\x94PPPPPV[`\0\x81\x82`\x14\x01\x10\x15a\x0BdW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01RqtoAddress_overflow`p\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x81`\x14\x01\x83Q\x10\x15a\x0B\xB5W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01RttoAddress_outOfBounds`X\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[P\x01` \x01Q`\x01``\x1B\x90\x04\x90V[`\0\x81\x82`\x03\x01\x10\x15a\x0C\x13W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01RptoUint24_overflow`x\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x81`\x03\x01\x83Q\x10\x15a\x0CcW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01RstoUint24_outOfBounds``\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[P\x01`\x03\x01Q\x90V[``\x81\x82`\x1F\x01\x10\x15a\x0C\xB7W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rmslice_overflow`\x90\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x82\x82\x84\x01\x10\x15a\x0C\xFFW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rmslice_overflow`\x90\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x81\x83\x01\x84Q\x10\x15a\rKW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01Rpslice_outOfBounds`x\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[``\x82\x15\x80\x15a\rjW`@Q\x91P`\0\x82R` \x82\x01`@Ra\r\xB4V[`@Q\x91P`\x1F\x84\x16\x80\x15` \x02\x81\x84\x01\x01\x85\x81\x01\x87\x83\x15` \x02\x84\x8B\x01\x01\x01[\x81\x83\x10\x15a\r\xA3W\x80Q\x83R` \x92\x83\x01\x92\x01a\r\x8BV[PP\x85\x84R`\x1F\x01`\x1F\x19\x16`@RP[P\x94\x93PPPPV[a\r\xC5a\x15KV[\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x11\x15a\r\xE3W\x91\x92\x91[P`@\x80Q``\x81\x01\x82R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x81R\x92\x90\x93\x16` \x83\x01Rb\xFF\xFF\xFF\x16\x91\x81\x01\x91\x90\x91R\x90V[`\0\x81` \x01Q`\x01`\x01`\xA0\x1B\x03\x16\x82`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x10a\x0E;W`\0\x80\xFD[P\x80Q` \x80\x83\x01Q`@\x93\x84\x01Q\x84Q`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x81\x85\x01R\x93\x90\x91\x16\x83\x85\x01Rb\xFF\xFF\xFF\x16``\x80\x84\x01\x91\x90\x91R\x83Q\x80\x84\x03\x82\x01\x81R`\x80\x84\x01\x85R\x80Q\x90\x83\x01 `\x01`\x01`\xF8\x1B\x03\x19`\xA0\x85\x01R\x94\x90\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\xA1\x83\x01R`\xB5\x82\x01\x93\x90\x93R\x7F\xE3O\x19\x9B\x19\xB2\xB4\xF4\x7FhD&\x19\xD5UR}$Ox\xA3)~\xA8\x93%\xF8C\xF8{\x8BT`\xD5\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\xF5\x90\x91\x01\x90\x91R\x80Q\x91\x01 \x90V[`\0\x80`\0\x83Q``\x14a\x0FcW`D\x84Q\x10\x15a\x0F0W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0F'\x90a\x19dV[`@Q\x80\x91\x03\x90\xFD[`\x04\x84\x01\x93P\x83\x80` \x01\x90Q\x81\x01\x90a\x0FJ\x91\x90a\x170V[`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0F'\x91\x90a\x19QV[\x83\x80` \x01\x90Q\x81\x01\x90a\x0Fw\x91\x90a\x18SV[\x92P\x92P\x92P\x91\x93\x90\x92PV[`\0\x80`\0\x80`\0\x80`\0\x80`\x08\x8B`\x01`\x01`\xA0\x1B\x03\x16c\xD0\xC9:|`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0F\xCBW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x0F\xDFW=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x0F\xF5W`\0\x80\xFD[PQ`\x02\x90\x81\x0B\x90\x8C\x90\x0B\x81a\x10\x07W\xFE[\x05`\x02\x0B\x90\x1D\x90P`\0a\x01\0\x8C`\x01`\x01`\xA0\x1B\x03\x16c\xD0\xC9:|`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x10MW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x10aW=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x10wW`\0\x80\xFD[PQ`\x02\x90\x81\x0B\x90\x8D\x90\x0B\x81a\x10\x89W\xFE[\x05`\x02\x0B\x81a\x10\x94W\xFE[\x07\x90P`\0`\x08\x8D`\x01`\x01`\xA0\x1B\x03\x16c\xD0\xC9:|`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x10\xD4W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x10\xE8W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x10\xFEW`\0\x80\xFD[PQ`\x02\x90\x81\x0B\x90\x8D\x90\x0B\x81a\x11\x10W\xFE[\x05`\x02\x0B\x90\x1D\x90P`\0a\x01\0\x8E`\x01`\x01`\xA0\x1B\x03\x16c\xD0\xC9:|`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x11VW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x11jW=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x11\x80W`\0\x80\xFD[PQ`\x02\x90\x81\x0B\x90\x8E\x90\x0B\x81a\x11\x92W\xFE[\x05`\x02\x0B\x81a\x11\x9DW\xFE[\x07\x90P`\0\x81`\xFF\x16`\x01\x90\x1B\x8F`\x01`\x01`\xA0\x1B\x03\x16cS9\xC2\x96\x85`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x82`\x01\x0B\x81R` \x01\x91PP` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x11\xF1W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x12\x05W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x12\x1BW`\0\x80\xFD[PQ\x16\x11\x80\x15a\x12\xA1WP\x8D`\x01`\x01`\xA0\x1B\x03\x16c\xD0\xC9:|`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x12_W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x12sW=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x12\x89W`\0\x80\xFD[PQ`\x02\x90\x81\x0B\x90\x8D\x90\x0B\x81a\x12\x9BW\xFE[\x07`\x02\x0B\x15[\x80\x15a\x12\xB2WP\x8B`\x02\x0B\x8D`\x02\x0B\x13[\x94P`\0\x83`\xFF\x16`\x01\x90\x1B\x8F`\x01`\x01`\xA0\x1B\x03\x16cS9\xC2\x96\x87`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x82`\x01\x0B\x81R` \x01\x91PP` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x13\x05W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x13\x19W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x13/W`\0\x80\xFD[PQ\x16\x11\x80\x15a\x13\xB5WP\x8D`\x01`\x01`\xA0\x1B\x03\x16c\xD0\xC9:|`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x13sW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x13\x87W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x13\x9DW`\0\x80\xFD[PQ`\x02\x90\x81\x0B\x90\x8E\x90\x0B\x81a\x13\xAFW\xFE[\x07`\x02\x0B\x15[\x80\x15a\x13\xC6WP\x8B`\x02\x0B\x8D`\x02\x0B\x12[\x95P\x81`\x01\x0B\x84`\x01\x0B\x12\x80a\x13\xF2WP\x81`\x01\x0B\x84`\x01\x0B\x14\x80\x15a\x13\xF2WP\x80`\xFF\x16\x83`\xFF\x16\x11\x15[\x15a\x14\x08W\x83\x99P\x82\x97P\x81\x98P\x80\x96Pa\x14\x15V[\x81\x99P\x80\x97P\x83\x98P\x82\x96P[PP`\0\x19`\xFF\x87\x16\x1B\x91PP[\x85`\x01\x0B\x87`\x01\x0B\x13a\x14\xE5W\x85`\x01\x0B\x87`\x01\x0B\x14\x15a\x14JW`\0\x19`\xFF\x85\x81\x03\x16\x1C\x16[`\0\x81\x8C`\x01`\x01`\xA0\x1B\x03\x16cS9\xC2\x96\x8A`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x82`\x01\x0B\x81R` \x01\x91PP` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x14\x94W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x14\xA8W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x14\xBEW`\0\x80\xFD[PQ\x16\x90Pa\x14\xCC\x81a\x150V[a\xFF\xFF\x16\x98\x90\x98\x01\x97PP`\x01\x90\x95\x01\x94`\0\x19a\x14#V[\x81\x15a\x14\xF2W`\x01\x88\x03\x97P[\x82\x15a\x14\xFFW`\x01\x88\x03\x97P[PPPPPPP\x93\x92PPPV[`\0a\x15\x19\x83\x83a\x0E\x13V[\x90P3`\x01`\x01`\xA0\x1B\x03\x82\x16\x14a\t\xE2W`\0\x80\xFD[`\0\x80[\x82\x15a\t\xE2W`\0\x19\x83\x01\x90\x92\x16\x91`\x01\x01a\x154V[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R\x90V[`\0\x82`\x1F\x83\x01\x12a\x15{W\x80\x81\xFD[\x815a\x15\x8Ea\x15\x89\x82a\x1AwV[a\x1ASV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x15\xA2W\x82\x83\xFD[\x81` \x85\x01` \x83\x017\x90\x81\x01` \x01\x91\x90\x91R\x92\x91PPV[\x80Q`\x02\x81\x90\x0B\x81\x14a\t\x8DW`\0\x80\xFD[`\0`\xA0\x82\x84\x03\x12\x15a\x15\xDFW\x80\x81\xFD[`@Q`\xA0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x15\xFCW\xFE[`@R\x90P\x80\x825a\x16\r\x81a\x1A\xC9V[\x81R` \x83\x015a\x16\x1D\x81a\x1A\xC9V[` \x82\x01R`@\x83\x81\x015\x90\x82\x01R``\x83\x015b\xFF\xFF\xFF\x81\x16\x81\x14a\x16BW`\0\x80\xFD[``\x82\x01Ra\x16S`\x80\x84\x01a\x16_V[`\x80\x82\x01RP\x92\x91PPV[\x805a\t\x8D\x81a\x1A\xC9V[\x80Qa\xFF\xFF\x81\x16\x81\x14a\t\x8DW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x16\x8EW\x81\x82\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x16\xA4W\x82\x83\xFD[a\x16\xB0\x85\x82\x86\x01a\x15kV[\x95` \x94\x90\x94\x015\x94PPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x16\xD1W\x81\x82\xFD[PP\x80Q` \x90\x91\x01Q\x90\x92\x90\x91PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x16\xF6W\x80\x81\xFD[\x835\x92P` \x84\x015\x91P`@\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x17\x1AW\x81\x82\xFD[a\x17&\x86\x82\x87\x01a\x15kV[\x91PP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\x17AW\x80\x81\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x17WW\x81\x82\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x17gW\x81\x82\xFD[\x80Qa\x17ua\x15\x89\x82a\x1AwV[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a\x17\x89W\x83\x84\xFD[a\x0B\x0C\x82` \x83\x01` \x86\x01a\x1A\x99V[`\0`\xA0\x82\x84\x03\x12\x15a\x17\xABW\x80\x81\xFD[a\x17\xB5\x83\x83a\x15\xCEV[\x93\x92PPPV[`\0\x80`\0\x80`\0\x80`\0`\xE0\x88\x8A\x03\x12\x15a\x17\xD6W\x82\x83\xFD[\x87Qa\x17\xE1\x81a\x1A\xC9V[\x96Pa\x17\xEF` \x89\x01a\x15\xBCV[\x95Pa\x17\xFD`@\x89\x01a\x16jV[\x94Pa\x18\x0B``\x89\x01a\x16jV[\x93Pa\x18\x19`\x80\x89\x01a\x16jV[\x92P`\xA0\x88\x01Q`\xFF\x81\x16\x81\x14a\x18.W\x82\x83\xFD[`\xC0\x89\x01Q\x90\x92P\x80\x15\x15\x81\x14a\x18CW\x81\x82\xFD[\x80\x91PP\x92\x95\x98\x91\x94\x97P\x92\x95PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x18gW\x80\x81\xFD[\x83Q\x92P` \x84\x01Qa\x18y\x81a\x1A\xC9V[\x91Pa\x18\x87`@\x85\x01a\x15\xBCV[\x90P\x92P\x92P\x92V[`\0\x81Q\x80\x84Ra\x18\xA8\x81` \x86\x01` \x86\x01a\x1A\x99V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[``\x93\x84\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x81\x16\x82R`\xE8\x93\x90\x93\x1B`\x01`\x01`\xE8\x1B\x03\x19\x16`\x14\x82\x01R\x92\x1B\x16`\x17\x82\x01R`+\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x81R` \x01\x90V[`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x82R\x85\x15\x15` \x83\x01R`@\x82\x01\x85\x90R\x83\x16``\x82\x01R`\xA0`\x80\x82\x01\x81\x90R`\0\x90a\x19F\x90\x83\x01\x84a\x18\x90V[\x97\x96PPPPPPPV[`\0` \x82Ra\x17\xB5` \x83\x01\x84a\x18\x90V[` \x80\x82R`\x10\x90\x82\x01Ro*\xB72\xBC82\xB1\xBA2\xB2\x102\xB997\xB9`\x81\x1B`@\x82\x01R``\x01\x90V[`\0`\x80\x82\x01\x86\x83R` `\x80\x81\x85\x01R\x81\x87Q\x80\x84R`\xA0\x86\x01\x91P\x82\x89\x01\x93P\x84[\x81\x81\x10\x15a\x19\xD7W\x84Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x93\x83\x01\x93\x91\x83\x01\x91`\x01\x01a\x19\xB2V[PP\x84\x81\x03`@\x86\x01R\x86Q\x80\x82R\x90\x82\x01\x92P\x81\x87\x01\x90\x84[\x81\x81\x10\x15a\x1A\x13W\x82Qc\xFF\xFF\xFF\xFF\x16\x85R\x93\x83\x01\x93\x91\x83\x01\x91`\x01\x01a\x19\xF1V[PPPP``\x92\x90\x92\x01\x92\x90\x92R\x94\x93PPPPV[\x93\x84R`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16` \x84\x01Rc\xFF\xFF\xFF\xFF\x16`@\x83\x01R``\x82\x01R`\x80\x01\x90V[`@Q\x81\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1AoW\xFE[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x1A\x8BW\xFE[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0[\x83\x81\x10\x15a\x1A\xB4W\x81\x81\x01Q\x83\x82\x01R` \x01a\x1A\x9CV[\x83\x81\x11\x15a\x1A\xC3W`\0\x84\x84\x01R[PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x1A\xDEW`\0\x80\xFD[PV\xFE\xA2dipfsX\"\x12 \x19\x9D\xA9\x13\x0B}\xA4Y0\x83\xA6^\xC9\xC0\xE8\x84\x0F\x8Ct]\xB2%\xA2o{\xC3\x17\x1E\xD7\xD8\x10pdsolcC\0\x07\x06\x003";
    /// The bytecode of the contract.
    pub static QUOTERV2_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0}W`\x005`\xE0\x1C\x80c\xC4Z\x01U\x11a\0[W\x80c\xC4Z\x01U\x14a\0\xE6W\x80c\xC6\xA5\x02j\x14a\0\xEEW\x80c\xCD\xCA\x17S\x14a\x01\x01W\x80c\xFAF\x1E3\x14a\x01\x14Wa\0}V[\x80c/\x80\xBB\x1D\x14a\0\x82W\x80cJ\xA4\xA4\xFC\x14a\0\xAEW\x80c\xBD!pJ\x14a\0\xC3W[`\0\x80\xFD[a\0\x95a\0\x906`\x04a\x16|V[a\x01)V[`@Qa\0\xA5\x94\x93\x92\x91\x90a\x19\x8EV[`@Q\x80\x91\x03\x90\xF3[a\0\xB6a\x02\xDEV[`@Qa\0\xA5\x91\x90a\x18\xF7V[a\0\xD6a\0\xD16`\x04a\x17\x9AV[a\x03\x02V[`@Qa\0\xA5\x94\x93\x92\x91\x90a\x1A)V[a\0\xB6a\x04\xD5V[a\0\xD6a\0\xFC6`\x04a\x17\x9AV[a\x04\xF9V[a\0\x95a\x01\x0F6`\x04a\x16|V[a\x06oV[a\x01'a\x01\"6`\x04a\x16\xE2V[a\x08\x06V[\0[`\0``\x80`\0a\x019\x86a\t\x81V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x80\x15a\x01OW`\0\x80\xFD[P`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x01yW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x92Pa\x01\x85\x86a\t\x81V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x80\x15a\x01\x9BW`\0\x80\xFD[P`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x01\xC5W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x91P`\0[`\0\x80`\0a\x01\xD9\x8Aa\t\x92V[\x92P\x92P\x92P`\0\x80`\0\x80a\x025`@Q\x80`\xA0\x01`@R\x80\x88`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x89`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8F\x81R` \x01\x87b\xFF\xFF\xFF\x16\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81RPa\x03\x02V[\x93P\x93P\x93P\x93P\x82\x8B\x89\x81Q\x81\x10a\x02JW\xFE[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81\x8A\x89\x81Q\x81\x10a\x02wW\xFE[c\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x92\x9BP\x96\x82\x01\x96`\x01\x90\x96\x01\x95\x8B\x92a\x02\xA4\x8Ea\t\xC3V[\x15a\x02\xB9Wa\x02\xB2\x8Ea\t\xCBV[\x9DPa\x02\xC9V[\x8C\x9BPPPPPPPPPa\x02\xD5V[PPPPPPPa\x01\xCBV[\x92\x95\x91\x94P\x92PV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[` \x81\x01Q\x81Q``\x83\x01Q`\0\x92\x83\x92\x83\x92\x83\x92`\x01`\x01`\xA0\x1B\x03\x80\x82\x16\x90\x84\x16\x10\x92\x84\x92a\x033\x92\x90a\t\xE8V[\x90P\x86`\x80\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\0\x14\x15a\x03TW`@\x87\x01Q`\0U[`\0Z\x90P\x81`\x01`\x01`\xA0\x1B\x03\x16c\x12\x8A\xCB\x080\x85a\x03w\x8C`@\x01Qa\n&V[`\0\x03\x8C`\x80\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\0\x14a\x03\x99W\x8C`\x80\x01Qa\x03\xBFV[\x87a\x03\xB8Ws\xFF\xFD\x89c\xEF\xD1\xFCjPd\x88I]\x95\x1DRc\x98\x8D%a\x03\xBFV[d\x01\0\x02v\xA4[\x8D` \x01Q\x8E``\x01Q\x8F`\0\x01Q`@Q` \x01a\x03\xE0\x93\x92\x91\x90a\x18\xBCV[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04\x0F\x95\x94\x93\x92\x91\x90a\x19\x0BV[`@\x80Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04(W`\0\x80\xFD[PZ\xF1\x92PPP\x80\x15a\x04XWP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x04U\x91\x81\x01\x90a\x16\xBFV[`\x01[a\x04\xC8W=\x80\x80\x15a\x04\x86W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x04\x8BV[``\x91P[PZ\x82\x03\x94P\x88`\x80\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\0\x14\x15a\x04\xACW`\0\x80U[a\x04\xB7\x81\x84\x87a\n<V[\x97P\x97P\x97P\x97PPPPPa\x04\xCEV[PPPPP[\x91\x93P\x91\x93V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[` \x81\x01Q\x81Q``\x83\x01Q`\0\x92\x83\x92\x83\x92\x83\x92`\x01`\x01`\xA0\x1B\x03\x80\x82\x16\x90\x84\x16\x10\x92\x84\x92a\x05*\x92\x90a\t\xE8V[\x90P`\0Z\x90P\x81`\x01`\x01`\xA0\x1B\x03\x16c\x12\x8A\xCB\x080\x85a\x05O\x8C`@\x01Qa\n&V[`\x80\x8D\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x15a\x05lW\x8C`\x80\x01Qa\x05\x92V[\x87a\x05\x8BWs\xFF\xFD\x89c\xEF\xD1\xFCjPd\x88I]\x95\x1DRc\x98\x8D%a\x05\x92V[d\x01\0\x02v\xA4[\x8D`\0\x01Q\x8E``\x01Q\x8F` \x01Q`@Q` \x01a\x05\xB3\x93\x92\x91\x90a\x18\xBCV[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x05\xE2\x95\x94\x93\x92\x91\x90a\x19\x0BV[`@\x80Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x05\xFBW`\0\x80\xFD[PZ\xF1\x92PPP\x80\x15a\x06+WP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x06(\x91\x81\x01\x90a\x16\xBFV[`\x01[a\x04\xC8W=\x80\x80\x15a\x06YW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x06^V[``\x91P[PZ\x82\x03\x94Pa\x04\xB7\x81\x84\x87a\n<V[`\0``\x80`\0a\x06\x7F\x86a\t\x81V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x80\x15a\x06\x95W`\0\x80\xFD[P`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x06\xBFW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x92Pa\x06\xCB\x86a\t\x81V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x80\x15a\x06\xE1W`\0\x80\xFD[P`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x07\x0BW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x91P`\0[`\0\x80`\0a\x07\x1F\x8Aa\t\x92V[\x92P\x92P\x92P`\0\x80`\0\x80a\x07{`@Q\x80`\xA0\x01`@R\x80\x89`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x88`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8F\x81R` \x01\x87b\xFF\xFF\xFF\x16\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81RPa\x04\xF9V[\x93P\x93P\x93P\x93P\x82\x8B\x89\x81Q\x81\x10a\x07\x90W\xFE[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81\x8A\x89\x81Q\x81\x10a\x07\xBDW\xFE[c\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x92\x9BP\x96\x82\x01\x96`\x01\x90\x96\x01\x95\x8B\x92a\x07\xEA\x8Ea\t\xC3V[\x15a\x02\xB9Wa\x07\xF8\x8Ea\t\xCBV[\x9DPPPPPPPPa\x07\x11V[`\0\x83\x13\x80a\x08\x15WP`\0\x82\x13[a\x08\x1EW`\0\x80\xFD[`\0\x80`\0a\x08,\x84a\t\x92V[\x92P\x92P\x92Pa\x08^\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84\x84\x84a\n\xF6V[P`\0\x80`\0\x80\x89\x13a\x08\x8AW\x85`\x01`\x01`\xA0\x1B\x03\x16\x85`\x01`\x01`\xA0\x1B\x03\x16\x10\x88\x8A`\0\x03a\x08\xA5V[\x84`\x01`\x01`\xA0\x1B\x03\x16\x86`\x01`\x01`\xA0\x1B\x03\x16\x10\x89\x89`\0\x03[\x92P\x92P\x92P`\0a\x08\xB8\x87\x87\x87a\t\xE8V[\x90P`\0\x80\x82`\x01`\x01`\xA0\x1B\x03\x16c8P\xC7\xBD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\xE0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x08\xF6W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\t\nW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t.\x91\x90a\x17\xBCV[PPPPP\x91P\x91P\x85\x15a\tTW`@Q\x84\x81R\x82` \x82\x01R\x81`@\x82\x01R``\x81\xFD[`\0T\x15a\tjW`\0T\x84\x14a\tjW`\0\x80\xFD[`@Q\x85\x81R\x82` \x82\x01R\x81`@\x82\x01R``\x81\xFD[\x80Q`\x17`\x13\x19\x90\x91\x01\x04[\x91\x90PV[`\0\x80\x80a\t\xA0\x84\x82a\x0B\x15V[\x92Pa\t\xAD\x84`\x14a\x0B\xC5V[\x90Pa\t\xBA\x84`\x17a\x0B\x15V[\x91P\x91\x93\x90\x92PV[Q`B\x11\x15\x90V[\x80Q``\x90a\t\xE2\x90\x83\x90`\x17\x90`\x16\x19\x01a\x0ClV[\x92\x91PPV[`\0a\n\x1E\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\n\x19\x86\x86\x86a\r\xBDV[a\x0E\x13V[\x94\x93PPPPV[`\0`\x01`\xFF\x1B\x82\x10a\n8W`\0\x80\xFD[P\x90V[`\0\x80`\0\x80`\0\x80\x87`\x01`\x01`\xA0\x1B\x03\x16c8P\xC7\xBD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\xE0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\n~W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\n\x92W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xB6\x91\x90a\x17\xBCV[P\x93\x96Pa\n\xCB\x94P\x8D\x93Pa\x0E\xF7\x92PPPV[\x91\x97P\x95P\x90Pa\n\xE6`\x01`\x01`\xA0\x1B\x03\x89\x16\x83\x83a\x0F\x84V[\x93P\x86\x92PPP\x93P\x93P\x93P\x93V[`\0a\x0B\x0C\x85a\x0B\x07\x86\x86\x86a\r\xBDV[a\x15\rV[\x95\x94PPPPPV[`\0\x81\x82`\x14\x01\x10\x15a\x0BdW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01RqtoAddress_overflow`p\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x81`\x14\x01\x83Q\x10\x15a\x0B\xB5W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01RttoAddress_outOfBounds`X\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[P\x01` \x01Q`\x01``\x1B\x90\x04\x90V[`\0\x81\x82`\x03\x01\x10\x15a\x0C\x13W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01RptoUint24_overflow`x\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x81`\x03\x01\x83Q\x10\x15a\x0CcW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01RstoUint24_outOfBounds``\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[P\x01`\x03\x01Q\x90V[``\x81\x82`\x1F\x01\x10\x15a\x0C\xB7W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rmslice_overflow`\x90\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x82\x82\x84\x01\x10\x15a\x0C\xFFW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rmslice_overflow`\x90\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x81\x83\x01\x84Q\x10\x15a\rKW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01Rpslice_outOfBounds`x\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[``\x82\x15\x80\x15a\rjW`@Q\x91P`\0\x82R` \x82\x01`@Ra\r\xB4V[`@Q\x91P`\x1F\x84\x16\x80\x15` \x02\x81\x84\x01\x01\x85\x81\x01\x87\x83\x15` \x02\x84\x8B\x01\x01\x01[\x81\x83\x10\x15a\r\xA3W\x80Q\x83R` \x92\x83\x01\x92\x01a\r\x8BV[PP\x85\x84R`\x1F\x01`\x1F\x19\x16`@RP[P\x94\x93PPPPV[a\r\xC5a\x15KV[\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x11\x15a\r\xE3W\x91\x92\x91[P`@\x80Q``\x81\x01\x82R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x81R\x92\x90\x93\x16` \x83\x01Rb\xFF\xFF\xFF\x16\x91\x81\x01\x91\x90\x91R\x90V[`\0\x81` \x01Q`\x01`\x01`\xA0\x1B\x03\x16\x82`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x10a\x0E;W`\0\x80\xFD[P\x80Q` \x80\x83\x01Q`@\x93\x84\x01Q\x84Q`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x81\x85\x01R\x93\x90\x91\x16\x83\x85\x01Rb\xFF\xFF\xFF\x16``\x80\x84\x01\x91\x90\x91R\x83Q\x80\x84\x03\x82\x01\x81R`\x80\x84\x01\x85R\x80Q\x90\x83\x01 `\x01`\x01`\xF8\x1B\x03\x19`\xA0\x85\x01R\x94\x90\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\xA1\x83\x01R`\xB5\x82\x01\x93\x90\x93R\x7F\xE3O\x19\x9B\x19\xB2\xB4\xF4\x7FhD&\x19\xD5UR}$Ox\xA3)~\xA8\x93%\xF8C\xF8{\x8BT`\xD5\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\xF5\x90\x91\x01\x90\x91R\x80Q\x91\x01 \x90V[`\0\x80`\0\x83Q``\x14a\x0FcW`D\x84Q\x10\x15a\x0F0W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0F'\x90a\x19dV[`@Q\x80\x91\x03\x90\xFD[`\x04\x84\x01\x93P\x83\x80` \x01\x90Q\x81\x01\x90a\x0FJ\x91\x90a\x170V[`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0F'\x91\x90a\x19QV[\x83\x80` \x01\x90Q\x81\x01\x90a\x0Fw\x91\x90a\x18SV[\x92P\x92P\x92P\x91\x93\x90\x92PV[`\0\x80`\0\x80`\0\x80`\0\x80`\x08\x8B`\x01`\x01`\xA0\x1B\x03\x16c\xD0\xC9:|`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0F\xCBW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x0F\xDFW=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x0F\xF5W`\0\x80\xFD[PQ`\x02\x90\x81\x0B\x90\x8C\x90\x0B\x81a\x10\x07W\xFE[\x05`\x02\x0B\x90\x1D\x90P`\0a\x01\0\x8C`\x01`\x01`\xA0\x1B\x03\x16c\xD0\xC9:|`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x10MW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x10aW=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x10wW`\0\x80\xFD[PQ`\x02\x90\x81\x0B\x90\x8D\x90\x0B\x81a\x10\x89W\xFE[\x05`\x02\x0B\x81a\x10\x94W\xFE[\x07\x90P`\0`\x08\x8D`\x01`\x01`\xA0\x1B\x03\x16c\xD0\xC9:|`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x10\xD4W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x10\xE8W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x10\xFEW`\0\x80\xFD[PQ`\x02\x90\x81\x0B\x90\x8D\x90\x0B\x81a\x11\x10W\xFE[\x05`\x02\x0B\x90\x1D\x90P`\0a\x01\0\x8E`\x01`\x01`\xA0\x1B\x03\x16c\xD0\xC9:|`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x11VW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x11jW=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x11\x80W`\0\x80\xFD[PQ`\x02\x90\x81\x0B\x90\x8E\x90\x0B\x81a\x11\x92W\xFE[\x05`\x02\x0B\x81a\x11\x9DW\xFE[\x07\x90P`\0\x81`\xFF\x16`\x01\x90\x1B\x8F`\x01`\x01`\xA0\x1B\x03\x16cS9\xC2\x96\x85`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x82`\x01\x0B\x81R` \x01\x91PP` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x11\xF1W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x12\x05W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x12\x1BW`\0\x80\xFD[PQ\x16\x11\x80\x15a\x12\xA1WP\x8D`\x01`\x01`\xA0\x1B\x03\x16c\xD0\xC9:|`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x12_W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x12sW=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x12\x89W`\0\x80\xFD[PQ`\x02\x90\x81\x0B\x90\x8D\x90\x0B\x81a\x12\x9BW\xFE[\x07`\x02\x0B\x15[\x80\x15a\x12\xB2WP\x8B`\x02\x0B\x8D`\x02\x0B\x13[\x94P`\0\x83`\xFF\x16`\x01\x90\x1B\x8F`\x01`\x01`\xA0\x1B\x03\x16cS9\xC2\x96\x87`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x82`\x01\x0B\x81R` \x01\x91PP` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x13\x05W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x13\x19W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x13/W`\0\x80\xFD[PQ\x16\x11\x80\x15a\x13\xB5WP\x8D`\x01`\x01`\xA0\x1B\x03\x16c\xD0\xC9:|`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x13sW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x13\x87W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x13\x9DW`\0\x80\xFD[PQ`\x02\x90\x81\x0B\x90\x8E\x90\x0B\x81a\x13\xAFW\xFE[\x07`\x02\x0B\x15[\x80\x15a\x13\xC6WP\x8B`\x02\x0B\x8D`\x02\x0B\x12[\x95P\x81`\x01\x0B\x84`\x01\x0B\x12\x80a\x13\xF2WP\x81`\x01\x0B\x84`\x01\x0B\x14\x80\x15a\x13\xF2WP\x80`\xFF\x16\x83`\xFF\x16\x11\x15[\x15a\x14\x08W\x83\x99P\x82\x97P\x81\x98P\x80\x96Pa\x14\x15V[\x81\x99P\x80\x97P\x83\x98P\x82\x96P[PP`\0\x19`\xFF\x87\x16\x1B\x91PP[\x85`\x01\x0B\x87`\x01\x0B\x13a\x14\xE5W\x85`\x01\x0B\x87`\x01\x0B\x14\x15a\x14JW`\0\x19`\xFF\x85\x81\x03\x16\x1C\x16[`\0\x81\x8C`\x01`\x01`\xA0\x1B\x03\x16cS9\xC2\x96\x8A`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x82`\x01\x0B\x81R` \x01\x91PP` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x14\x94W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x14\xA8W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x14\xBEW`\0\x80\xFD[PQ\x16\x90Pa\x14\xCC\x81a\x150V[a\xFF\xFF\x16\x98\x90\x98\x01\x97PP`\x01\x90\x95\x01\x94`\0\x19a\x14#V[\x81\x15a\x14\xF2W`\x01\x88\x03\x97P[\x82\x15a\x14\xFFW`\x01\x88\x03\x97P[PPPPPPP\x93\x92PPPV[`\0a\x15\x19\x83\x83a\x0E\x13V[\x90P3`\x01`\x01`\xA0\x1B\x03\x82\x16\x14a\t\xE2W`\0\x80\xFD[`\0\x80[\x82\x15a\t\xE2W`\0\x19\x83\x01\x90\x92\x16\x91`\x01\x01a\x154V[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R\x90V[`\0\x82`\x1F\x83\x01\x12a\x15{W\x80\x81\xFD[\x815a\x15\x8Ea\x15\x89\x82a\x1AwV[a\x1ASV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x15\xA2W\x82\x83\xFD[\x81` \x85\x01` \x83\x017\x90\x81\x01` \x01\x91\x90\x91R\x92\x91PPV[\x80Q`\x02\x81\x90\x0B\x81\x14a\t\x8DW`\0\x80\xFD[`\0`\xA0\x82\x84\x03\x12\x15a\x15\xDFW\x80\x81\xFD[`@Q`\xA0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x15\xFCW\xFE[`@R\x90P\x80\x825a\x16\r\x81a\x1A\xC9V[\x81R` \x83\x015a\x16\x1D\x81a\x1A\xC9V[` \x82\x01R`@\x83\x81\x015\x90\x82\x01R``\x83\x015b\xFF\xFF\xFF\x81\x16\x81\x14a\x16BW`\0\x80\xFD[``\x82\x01Ra\x16S`\x80\x84\x01a\x16_V[`\x80\x82\x01RP\x92\x91PPV[\x805a\t\x8D\x81a\x1A\xC9V[\x80Qa\xFF\xFF\x81\x16\x81\x14a\t\x8DW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x16\x8EW\x81\x82\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x16\xA4W\x82\x83\xFD[a\x16\xB0\x85\x82\x86\x01a\x15kV[\x95` \x94\x90\x94\x015\x94PPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x16\xD1W\x81\x82\xFD[PP\x80Q` \x90\x91\x01Q\x90\x92\x90\x91PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x16\xF6W\x80\x81\xFD[\x835\x92P` \x84\x015\x91P`@\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x17\x1AW\x81\x82\xFD[a\x17&\x86\x82\x87\x01a\x15kV[\x91PP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\x17AW\x80\x81\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x17WW\x81\x82\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x17gW\x81\x82\xFD[\x80Qa\x17ua\x15\x89\x82a\x1AwV[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a\x17\x89W\x83\x84\xFD[a\x0B\x0C\x82` \x83\x01` \x86\x01a\x1A\x99V[`\0`\xA0\x82\x84\x03\x12\x15a\x17\xABW\x80\x81\xFD[a\x17\xB5\x83\x83a\x15\xCEV[\x93\x92PPPV[`\0\x80`\0\x80`\0\x80`\0`\xE0\x88\x8A\x03\x12\x15a\x17\xD6W\x82\x83\xFD[\x87Qa\x17\xE1\x81a\x1A\xC9V[\x96Pa\x17\xEF` \x89\x01a\x15\xBCV[\x95Pa\x17\xFD`@\x89\x01a\x16jV[\x94Pa\x18\x0B``\x89\x01a\x16jV[\x93Pa\x18\x19`\x80\x89\x01a\x16jV[\x92P`\xA0\x88\x01Q`\xFF\x81\x16\x81\x14a\x18.W\x82\x83\xFD[`\xC0\x89\x01Q\x90\x92P\x80\x15\x15\x81\x14a\x18CW\x81\x82\xFD[\x80\x91PP\x92\x95\x98\x91\x94\x97P\x92\x95PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x18gW\x80\x81\xFD[\x83Q\x92P` \x84\x01Qa\x18y\x81a\x1A\xC9V[\x91Pa\x18\x87`@\x85\x01a\x15\xBCV[\x90P\x92P\x92P\x92V[`\0\x81Q\x80\x84Ra\x18\xA8\x81` \x86\x01` \x86\x01a\x1A\x99V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[``\x93\x84\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x81\x16\x82R`\xE8\x93\x90\x93\x1B`\x01`\x01`\xE8\x1B\x03\x19\x16`\x14\x82\x01R\x92\x1B\x16`\x17\x82\x01R`+\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x81R` \x01\x90V[`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x82R\x85\x15\x15` \x83\x01R`@\x82\x01\x85\x90R\x83\x16``\x82\x01R`\xA0`\x80\x82\x01\x81\x90R`\0\x90a\x19F\x90\x83\x01\x84a\x18\x90V[\x97\x96PPPPPPPV[`\0` \x82Ra\x17\xB5` \x83\x01\x84a\x18\x90V[` \x80\x82R`\x10\x90\x82\x01Ro*\xB72\xBC82\xB1\xBA2\xB2\x102\xB997\xB9`\x81\x1B`@\x82\x01R``\x01\x90V[`\0`\x80\x82\x01\x86\x83R` `\x80\x81\x85\x01R\x81\x87Q\x80\x84R`\xA0\x86\x01\x91P\x82\x89\x01\x93P\x84[\x81\x81\x10\x15a\x19\xD7W\x84Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x93\x83\x01\x93\x91\x83\x01\x91`\x01\x01a\x19\xB2V[PP\x84\x81\x03`@\x86\x01R\x86Q\x80\x82R\x90\x82\x01\x92P\x81\x87\x01\x90\x84[\x81\x81\x10\x15a\x1A\x13W\x82Qc\xFF\xFF\xFF\xFF\x16\x85R\x93\x83\x01\x93\x91\x83\x01\x91`\x01\x01a\x19\xF1V[PPPP``\x92\x90\x92\x01\x92\x90\x92R\x94\x93PPPPV[\x93\x84R`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16` \x84\x01Rc\xFF\xFF\xFF\xFF\x16`@\x83\x01R``\x82\x01R`\x80\x01\x90V[`@Q\x81\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1AoW\xFE[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x1A\x8BW\xFE[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0[\x83\x81\x10\x15a\x1A\xB4W\x81\x81\x01Q\x83\x82\x01R` \x01a\x1A\x9CV[\x83\x81\x11\x15a\x1A\xC3W`\0\x84\x84\x01R[PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x1A\xDEW`\0\x80\xFD[PV\xFE\xA2dipfsX\"\x12 \x19\x9D\xA9\x13\x0B}\xA4Y0\x83\xA6^\xC9\xC0\xE8\x84\x0F\x8Ct]\xB2%\xA2o{\xC3\x17\x1E\xD7\xD8\x10pdsolcC\0\x07\x06\x003";
    /// The deployed bytecode of the contract.
    pub static QUOTERV2_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct QuoterV2<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for QuoterV2<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for QuoterV2<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for QuoterV2<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for QuoterV2<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(QuoterV2)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> QuoterV2<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    QUOTERV2_ABI.clone(),
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
                QUOTERV2_ABI.clone(),
                QUOTERV2_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `WETH9` (0x4aa4a4fc) function
        pub fn weth9(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([74, 164, 164, 252], ())
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
        ///Calls the contract's `quoteExactInput` (0xcdca1753) function
        pub fn quote_exact_input(
            &self,
            path: ::ethers::core::types::Bytes,
            amount_in: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::std::vec::Vec<::ethers::core::types::U256>,
                ::std::vec::Vec<u32>,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([205, 202, 23, 83], (path, amount_in))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `quoteExactInputSingle` (0xc6a5026a) function
        pub fn quote_exact_input_single(
            &self,
            params: QuoteExactInputSingleParams,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                u32,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([198, 165, 2, 106], (params,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `quoteExactOutput` (0x2f80bb1d) function
        pub fn quote_exact_output(
            &self,
            path: ::ethers::core::types::Bytes,
            amount_out: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::std::vec::Vec<::ethers::core::types::U256>,
                ::std::vec::Vec<u32>,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([47, 128, 187, 29], (path, amount_out))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `quoteExactOutputSingle` (0xbd21704a) function
        pub fn quote_exact_output_single(
            &self,
            params: QuoteExactOutputSingleParams,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                u32,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([189, 33, 112, 74], (params,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `uniswapV3SwapCallback` (0xfa461e33) function
        pub fn uniswap_v3_swap_callback(
            &self,
            amount_0_delta: ::ethers::core::types::I256,
            amount_1_delta: ::ethers::core::types::I256,
            path: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([250, 70, 30, 51], (amount_0_delta, amount_1_delta, path))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for QuoterV2<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
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
        Hash
    )]
    #[ethcall(name = "WETH9", abi = "WETH9()")]
    pub struct Weth9Call;
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
    ///Container type for all input parameters for the `quoteExactInput` function with signature `quoteExactInput(bytes,uint256)` and selector `0xcdca1753`
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
    #[ethcall(name = "quoteExactInput", abi = "quoteExactInput(bytes,uint256)")]
    pub struct QuoteExactInputCall {
        pub path: ::ethers::core::types::Bytes,
        pub amount_in: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `quoteExactInputSingle` function with signature `quoteExactInputSingle((address,address,uint256,uint24,uint160))` and selector `0xc6a5026a`
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
        name = "quoteExactInputSingle",
        abi = "quoteExactInputSingle((address,address,uint256,uint24,uint160))"
    )]
    pub struct QuoteExactInputSingleCall {
        pub params: QuoteExactInputSingleParams,
    }
    ///Container type for all input parameters for the `quoteExactOutput` function with signature `quoteExactOutput(bytes,uint256)` and selector `0x2f80bb1d`
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
    #[ethcall(name = "quoteExactOutput", abi = "quoteExactOutput(bytes,uint256)")]
    pub struct QuoteExactOutputCall {
        pub path: ::ethers::core::types::Bytes,
        pub amount_out: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `quoteExactOutputSingle` function with signature `quoteExactOutputSingle((address,address,uint256,uint24,uint160))` and selector `0xbd21704a`
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
        name = "quoteExactOutputSingle",
        abi = "quoteExactOutputSingle((address,address,uint256,uint24,uint160))"
    )]
    pub struct QuoteExactOutputSingleCall {
        pub params: QuoteExactOutputSingleParams,
    }
    ///Container type for all input parameters for the `uniswapV3SwapCallback` function with signature `uniswapV3SwapCallback(int256,int256,bytes)` and selector `0xfa461e33`
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
        name = "uniswapV3SwapCallback",
        abi = "uniswapV3SwapCallback(int256,int256,bytes)"
    )]
    pub struct UniswapV3SwapCallbackCall {
        pub amount_0_delta: ::ethers::core::types::I256,
        pub amount_1_delta: ::ethers::core::types::I256,
        pub path: ::ethers::core::types::Bytes,
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
        Hash
    )]
    pub enum QuoterV2Calls {
        Weth9(Weth9Call),
        Factory(FactoryCall),
        QuoteExactInput(QuoteExactInputCall),
        QuoteExactInputSingle(QuoteExactInputSingleCall),
        QuoteExactOutput(QuoteExactOutputCall),
        QuoteExactOutputSingle(QuoteExactOutputSingleCall),
        UniswapV3SwapCallback(UniswapV3SwapCallbackCall),
    }
    impl ::ethers::core::abi::AbiDecode for QuoterV2Calls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <Weth9Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Weth9(decoded));
            }
            if let Ok(decoded) = <FactoryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Factory(decoded));
            }
            if let Ok(decoded) = <QuoteExactInputCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::QuoteExactInput(decoded));
            }
            if let Ok(decoded) = <QuoteExactInputSingleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::QuoteExactInputSingle(decoded));
            }
            if let Ok(decoded) = <QuoteExactOutputCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::QuoteExactOutput(decoded));
            }
            if let Ok(decoded) = <QuoteExactOutputSingleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::QuoteExactOutputSingle(decoded));
            }
            if let Ok(decoded) = <UniswapV3SwapCallbackCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UniswapV3SwapCallback(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for QuoterV2Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Weth9(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Factory(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::QuoteExactInput(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::QuoteExactInputSingle(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::QuoteExactOutput(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::QuoteExactOutputSingle(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UniswapV3SwapCallback(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for QuoterV2Calls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Weth9(element) => ::core::fmt::Display::fmt(element, f),
                Self::Factory(element) => ::core::fmt::Display::fmt(element, f),
                Self::QuoteExactInput(element) => ::core::fmt::Display::fmt(element, f),
                Self::QuoteExactInputSingle(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::QuoteExactOutput(element) => ::core::fmt::Display::fmt(element, f),
                Self::QuoteExactOutputSingle(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UniswapV3SwapCallback(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<Weth9Call> for QuoterV2Calls {
        fn from(value: Weth9Call) -> Self {
            Self::Weth9(value)
        }
    }
    impl ::core::convert::From<FactoryCall> for QuoterV2Calls {
        fn from(value: FactoryCall) -> Self {
            Self::Factory(value)
        }
    }
    impl ::core::convert::From<QuoteExactInputCall> for QuoterV2Calls {
        fn from(value: QuoteExactInputCall) -> Self {
            Self::QuoteExactInput(value)
        }
    }
    impl ::core::convert::From<QuoteExactInputSingleCall> for QuoterV2Calls {
        fn from(value: QuoteExactInputSingleCall) -> Self {
            Self::QuoteExactInputSingle(value)
        }
    }
    impl ::core::convert::From<QuoteExactOutputCall> for QuoterV2Calls {
        fn from(value: QuoteExactOutputCall) -> Self {
            Self::QuoteExactOutput(value)
        }
    }
    impl ::core::convert::From<QuoteExactOutputSingleCall> for QuoterV2Calls {
        fn from(value: QuoteExactOutputSingleCall) -> Self {
            Self::QuoteExactOutputSingle(value)
        }
    }
    impl ::core::convert::From<UniswapV3SwapCallbackCall> for QuoterV2Calls {
        fn from(value: UniswapV3SwapCallbackCall) -> Self {
            Self::UniswapV3SwapCallback(value)
        }
    }
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
        Hash
    )]
    pub struct Weth9Return(pub ::ethers::core::types::Address);
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
    ///Container type for all return fields from the `quoteExactInput` function with signature `quoteExactInput(bytes,uint256)` and selector `0xcdca1753`
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
    pub struct QuoteExactInputReturn {
        pub amount_out: ::ethers::core::types::U256,
        pub sqrt_price_x96_after_list: ::std::vec::Vec<::ethers::core::types::U256>,
        pub initialized_ticks_crossed_list: ::std::vec::Vec<u32>,
        pub gas_estimate: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `quoteExactInputSingle` function with signature `quoteExactInputSingle((address,address,uint256,uint24,uint160))` and selector `0xc6a5026a`
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
    pub struct QuoteExactInputSingleReturn {
        pub amount_out: ::ethers::core::types::U256,
        pub sqrt_price_x96_after: ::ethers::core::types::U256,
        pub initialized_ticks_crossed: u32,
        pub gas_estimate: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `quoteExactOutput` function with signature `quoteExactOutput(bytes,uint256)` and selector `0x2f80bb1d`
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
    pub struct QuoteExactOutputReturn {
        pub amount_in: ::ethers::core::types::U256,
        pub sqrt_price_x96_after_list: ::std::vec::Vec<::ethers::core::types::U256>,
        pub initialized_ticks_crossed_list: ::std::vec::Vec<u32>,
        pub gas_estimate: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `quoteExactOutputSingle` function with signature `quoteExactOutputSingle((address,address,uint256,uint24,uint160))` and selector `0xbd21704a`
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
    pub struct QuoteExactOutputSingleReturn {
        pub amount_in: ::ethers::core::types::U256,
        pub sqrt_price_x96_after: ::ethers::core::types::U256,
        pub initialized_ticks_crossed: u32,
        pub gas_estimate: ::ethers::core::types::U256,
    }
}
