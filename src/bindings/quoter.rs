pub use quoter::*;
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
pub mod quoter {
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
                                    name: ::std::borrow::ToOwned::to_owned("tokenIn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenOut"),
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
                                    name: ::std::borrow::ToOwned::to_owned("amountIn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                                    name: ::std::borrow::ToOwned::to_owned("tokenIn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenOut"),
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
                                    name: ::std::borrow::ToOwned::to_owned("amountOut"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
    pub static QUOTER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xC0`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x0F\x868\x03\x80a\x0F\x86\x839\x81\x01`@\x81\x90Ra\0/\x91a\0iV[`\x01`\x01``\x1B\x03\x19``\x92\x83\x1B\x81\x16`\x80R\x91\x1B\x16`\xA0Ra\0\x9BV[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0dW`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\0{W\x81\x82\xFD[a\0\x84\x83a\0MV[\x91Pa\0\x92` \x84\x01a\0MV[\x90P\x92P\x92\x90PV[`\x80Q``\x1C`\xA0Q``\x1Ca\x0E\xB8a\0\xCE`\09\x80a\x02\xF0RP\x80a\x03\x14R\x80a\x04\xEFR\x80a\x05\xECRPa\x0E\xB8`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0}W`\x005`\xE0\x1C\x80c\xC4Z\x01U\x11a\0[W\x80c\xC4Z\x01U\x14a\0\xD3W\x80c\xCD\xCA\x17S\x14a\0\xDBW\x80c\xF7r\x9DC\x14a\0\xEEW\x80c\xFAF\x1E3\x14a\x01\x01Wa\0}V[\x80c/\x80\xBB\x1D\x14a\0\x82W\x80c0\xD0\x7F!\x14a\0\xABW\x80cJ\xA4\xA4\xFC\x14a\0\xBEW[`\0\x80\xFD[a\0\x95a\0\x906`\x04a\x0B\xB0V[a\x01\x16V[`@Qa\0\xA2\x91\x90a\r\xEBV[`@Q\x80\x91\x03\x90\xF3[a\0\x95a\0\xB96`\x04a\x0BBV[a\x01{V[a\0\xC6a\x02\xEEV[`@Qa\0\xA2\x91\x90a\rMV[a\0\xC6a\x03\x12V[a\0\x95a\0\xE96`\x04a\x0B\xB0V[a\x036V[a\0\x95a\0\xFC6`\x04a\x0BBV[a\x03\x84V[a\x01\x14a\x01\x0F6`\x04a\x0C\x16V[a\x04\xBEV[\0[`\0[`\0a\x01$\x84a\x05\x95V[\x90P`\0\x80`\0a\x014\x87a\x05\x9DV[\x92P\x92P\x92Pa\x01H\x82\x84\x83\x89`\0a\x01{V[\x95P\x83\x15a\x01`Wa\x01Y\x87a\x05\xCEV[\x96Pa\x01lV[\x85\x94PPPPPa\x01uV[PPPPa\x01\x19V[\x92\x91PPV[`\0`\x01`\x01`\xA0\x1B\x03\x80\x86\x16\x87\x82\x16\x10\x90\x83\x16a\x01\x99W`\0\x84\x90U[a\x01\xA4\x87\x87\x87a\x05\xE5V[`\x01`\x01`\xA0\x1B\x03\x16c\x12\x8A\xCB\x080\x83a\x01\xBD\x88a\x06#V[`\0\x03`\x01`\x01`\xA0\x1B\x03\x88\x16\x15a\x01\xD5W\x87a\x01\xFBV[\x85a\x01\xF4Ws\xFF\xFD\x89c\xEF\xD1\xFCjPd\x88I]\x95\x1DRc\x98\x8D%a\x01\xFBV[d\x01\0\x02v\xA4[\x8B\x8B\x8E`@Q` \x01a\x02\x10\x93\x92\x91\x90a\r\x12V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x02?\x95\x94\x93\x92\x91\x90a\raV[`@\x80Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x02XW`\0\x80\xFD[PZ\xF1\x92PPP\x80\x15a\x02\x88WP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x02\x85\x91\x81\x01\x90a\x0B\xF3V[`\x01[a\x02\xE1W=\x80\x80\x15a\x02\xB6W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x02\xBBV[``\x91P[P`\x01`\x01`\xA0\x1B\x03\x84\x16a\x02\xCFW`\0\x80U[a\x02\xD8\x81a\x069V[\x92PPPa\x02\xE5V[PPP[\x95\x94PPPPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0[`\0a\x03D\x84a\x05\x95V[\x90P`\0\x80`\0a\x03T\x87a\x05\x9DV[\x92P\x92P\x92Pa\x03h\x83\x83\x83\x89`\0a\x03\x84V[\x95P\x83\x15a\x01`Wa\x03y\x87a\x05\xCEV[\x96PPPPPa\x039V[`\0`\x01`\x01`\xA0\x1B\x03\x80\x86\x16\x90\x87\x16\x10a\x03\xA0\x87\x87\x87a\x05\xE5V[`\x01`\x01`\xA0\x1B\x03\x16c\x12\x8A\xCB\x080\x83a\x03\xB9\x88a\x06#V[`\x01`\x01`\xA0\x1B\x03\x88\x16\x15a\x03\xCEW\x87a\x03\xF4V[\x85a\x03\xEDWs\xFF\xFD\x89c\xEF\xD1\xFCjPd\x88I]\x95\x1DRc\x98\x8D%a\x03\xF4V[d\x01\0\x02v\xA4[\x8C\x8B\x8D`@Q` \x01a\x04\t\x93\x92\x91\x90a\r\x12V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x048\x95\x94\x93\x92\x91\x90a\raV[`@\x80Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04QW`\0\x80\xFD[PZ\xF1\x92PPP\x80\x15a\x04\x81WP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x04~\x91\x81\x01\x90a\x0B\xF3V[`\x01[a\x02\xE1W=\x80\x80\x15a\x04\xAFW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x04\xB4V[``\x91P[Pa\x02\xD8\x81a\x069V[`\0\x83\x13\x80a\x04\xCDWP`\0\x82\x13[a\x04\xD6W`\0\x80\xFD[`\0\x80`\0a\x04\xE4\x84a\x05\x9DV[\x92P\x92P\x92Pa\x05\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84\x84\x84a\x06\xB6V[P`\0\x80`\0\x80\x89\x13a\x05BW\x85`\x01`\x01`\xA0\x1B\x03\x16\x85`\x01`\x01`\xA0\x1B\x03\x16\x10\x88\x8A`\0\x03a\x05]V[\x84`\x01`\x01`\xA0\x1B\x03\x16\x86`\x01`\x01`\xA0\x1B\x03\x16\x10\x89\x89`\0\x03[\x92P\x92P\x92P\x82\x15a\x05tW`@Q\x81\x81R` \x81\xFD[`\0T\x15a\x05\x8AW`\0T\x81\x14a\x05\x8AW`\0\x80\xFD[`@Q\x82\x81R` \x81\xFD[Q`B\x11\x15\x90V[`\0\x80\x80a\x05\xAB\x84\x82a\x06\xCCV[\x92Pa\x05\xB8\x84`\x14a\x07|V[\x90Pa\x05\xC5\x84`\x17a\x06\xCCV[\x91P\x91\x93\x90\x92PV[\x80Q``\x90a\x01u\x90\x83\x90`\x17\x90`\x16\x19\x01a\x08#V[`\0a\x06\x1B\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x06\x16\x86\x86\x86a\ttV[a\t\xCAV[\x94\x93PPPPV[`\0`\x01`\xFF\x1B\x82\x10a\x065W`\0\x80\xFD[P\x90V[`\0\x81Q` \x14a\x06\xA2W`D\x82Q\x10\x15a\x06oW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06f\x90a\r\xC1V[`@Q\x80\x91\x03\x90\xFD[`\x04\x82\x01\x91P\x81\x80` \x01\x90Q\x81\x01\x90a\x06\x89\x91\x90a\x0CdV[`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06f\x91\x90a\r\xA7V[\x81\x80` \x01\x90Q\x81\x01\x90a\x01u\x91\x90a\x0C\xCEV[`\0a\x02\xE5\x85a\x06\xC7\x86\x86\x86a\ttV[a\n\xAEV[`\0\x81\x82`\x14\x01\x10\x15a\x07\x1BW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01RqtoAddress_overflow`p\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x81`\x14\x01\x83Q\x10\x15a\x07lW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01RttoAddress_outOfBounds`X\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[P\x01` \x01Q`\x01``\x1B\x90\x04\x90V[`\0\x81\x82`\x03\x01\x10\x15a\x07\xCAW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01RptoUint24_overflow`x\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x81`\x03\x01\x83Q\x10\x15a\x08\x1AW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01RstoUint24_outOfBounds``\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[P\x01`\x03\x01Q\x90V[``\x81\x82`\x1F\x01\x10\x15a\x08nW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rmslice_overflow`\x90\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x82\x82\x84\x01\x10\x15a\x08\xB6W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rmslice_overflow`\x90\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x81\x83\x01\x84Q\x10\x15a\t\x02W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01Rpslice_outOfBounds`x\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[``\x82\x15\x80\x15a\t!W`@Q\x91P`\0\x82R` \x82\x01`@Ra\tkV[`@Q\x91P`\x1F\x84\x16\x80\x15` \x02\x81\x84\x01\x01\x85\x81\x01\x87\x83\x15` \x02\x84\x8B\x01\x01\x01[\x81\x83\x10\x15a\tZW\x80Q\x83R` \x92\x83\x01\x92\x01a\tBV[PP\x85\x84R`\x1F\x01`\x1F\x19\x16`@RP[P\x94\x93PPPPV[a\t|a\n\xD1V[\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x11\x15a\t\x9AW\x91\x92\x91[P`@\x80Q``\x81\x01\x82R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x81R\x92\x90\x93\x16` \x83\x01Rb\xFF\xFF\xFF\x16\x91\x81\x01\x91\x90\x91R\x90V[`\0\x81` \x01Q`\x01`\x01`\xA0\x1B\x03\x16\x82`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x10a\t\xF2W`\0\x80\xFD[P\x80Q` \x80\x83\x01Q`@\x93\x84\x01Q\x84Q`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x81\x85\x01R\x93\x90\x91\x16\x83\x85\x01Rb\xFF\xFF\xFF\x16``\x80\x84\x01\x91\x90\x91R\x83Q\x80\x84\x03\x82\x01\x81R`\x80\x84\x01\x85R\x80Q\x90\x83\x01 `\x01`\x01`\xF8\x1B\x03\x19`\xA0\x85\x01R\x94\x90\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\xA1\x83\x01R`\xB5\x82\x01\x93\x90\x93R\x7F\xE3O\x19\x9B\x19\xB2\xB4\xF4\x7FhD&\x19\xD5UR}$Ox\xA3)~\xA8\x93%\xF8C\xF8{\x8BT`\xD5\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\xF5\x90\x91\x01\x90\x91R\x80Q\x91\x01 \x90V[`\0a\n\xBA\x83\x83a\t\xCAV[\x90P3`\x01`\x01`\xA0\x1B\x03\x82\x16\x14a\x01uW`\0\x80\xFD[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R\x90V[`\0\x82`\x1F\x83\x01\x12a\x0B\x01W\x80\x81\xFD[\x815a\x0B\x14a\x0B\x0F\x82a\x0E\x18V[a\r\xF4V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x0B(W\x82\x83\xFD[\x81` \x85\x01` \x83\x017\x90\x81\x01` \x01\x91\x90\x91R\x92\x91PPV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x0BYW\x80\x81\xFD[\x855a\x0Bd\x81a\x0EjV[\x94P` \x86\x015a\x0Bt\x81a\x0EjV[\x93P`@\x86\x015b\xFF\xFF\xFF\x81\x16\x81\x14a\x0B\x8BW\x81\x82\xFD[\x92P``\x86\x015\x91P`\x80\x86\x015a\x0B\xA2\x81a\x0EjV[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[`\0\x80`@\x83\x85\x03\x12\x15a\x0B\xC2W\x81\x82\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0B\xD8W\x82\x83\xFD[a\x0B\xE4\x85\x82\x86\x01a\n\xF1V[\x95` \x94\x90\x94\x015\x94PPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x0C\x05W\x81\x82\xFD[PP\x80Q` \x90\x91\x01Q\x90\x92\x90\x91PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x0C*W\x82\x83\xFD[\x835\x92P` \x84\x015\x91P`@\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0CNW\x81\x82\xFD[a\x0CZ\x86\x82\x87\x01a\n\xF1V[\x91PP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\x0CuW\x80\x81\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0C\x8BW\x81\x82\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x0C\x9BW\x81\x82\xFD[\x80Qa\x0C\xA9a\x0B\x0F\x82a\x0E\x18V[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a\x0C\xBDW\x83\x84\xFD[a\x02\xE5\x82` \x83\x01` \x86\x01a\x0E:V[`\0` \x82\x84\x03\x12\x15a\x0C\xDFW\x80\x81\xFD[PQ\x91\x90PV[`\0\x81Q\x80\x84Ra\x0C\xFE\x81` \x86\x01` \x86\x01a\x0E:V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[``\x93\x84\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x81\x16\x82R`\xE8\x93\x90\x93\x1B`\x01`\x01`\xE8\x1B\x03\x19\x16`\x14\x82\x01R\x92\x1B\x16`\x17\x82\x01R`+\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x81R` \x01\x90V[`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x82R\x85\x15\x15` \x83\x01R`@\x82\x01\x85\x90R\x83\x16``\x82\x01R`\xA0`\x80\x82\x01\x81\x90R`\0\x90a\r\x9C\x90\x83\x01\x84a\x0C\xE6V[\x97\x96PPPPPPPV[`\0` \x82Ra\r\xBA` \x83\x01\x84a\x0C\xE6V[\x93\x92PPPV[` \x80\x82R`\x10\x90\x82\x01Ro*\xB72\xBC82\xB1\xBA2\xB2\x102\xB997\xB9`\x81\x1B`@\x82\x01R``\x01\x90V[\x90\x81R` \x01\x90V[`@Q\x81\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0E\x10W\xFE[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0E,W\xFE[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0[\x83\x81\x10\x15a\x0EUW\x81\x81\x01Q\x83\x82\x01R` \x01a\x0E=V[\x83\x81\x11\x15a\x0EdW`\0\x84\x84\x01R[PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0E\x7FW`\0\x80\xFD[PV\xFE\xA2dipfsX\"\x12 `\xFF\xF3\xF31\x8C\xED\x068Z\x0E\xE6k\x05\x0FO\x8C\xCD\xF1\x97\x13\xA8\x1Ap%~<\xD5\xB3\xBD0\xF6dsolcC\0\x07\x06\x003";
    /// The bytecode of the contract.
    pub static QUOTER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0}W`\x005`\xE0\x1C\x80c\xC4Z\x01U\x11a\0[W\x80c\xC4Z\x01U\x14a\0\xD3W\x80c\xCD\xCA\x17S\x14a\0\xDBW\x80c\xF7r\x9DC\x14a\0\xEEW\x80c\xFAF\x1E3\x14a\x01\x01Wa\0}V[\x80c/\x80\xBB\x1D\x14a\0\x82W\x80c0\xD0\x7F!\x14a\0\xABW\x80cJ\xA4\xA4\xFC\x14a\0\xBEW[`\0\x80\xFD[a\0\x95a\0\x906`\x04a\x0B\xB0V[a\x01\x16V[`@Qa\0\xA2\x91\x90a\r\xEBV[`@Q\x80\x91\x03\x90\xF3[a\0\x95a\0\xB96`\x04a\x0BBV[a\x01{V[a\0\xC6a\x02\xEEV[`@Qa\0\xA2\x91\x90a\rMV[a\0\xC6a\x03\x12V[a\0\x95a\0\xE96`\x04a\x0B\xB0V[a\x036V[a\0\x95a\0\xFC6`\x04a\x0BBV[a\x03\x84V[a\x01\x14a\x01\x0F6`\x04a\x0C\x16V[a\x04\xBEV[\0[`\0[`\0a\x01$\x84a\x05\x95V[\x90P`\0\x80`\0a\x014\x87a\x05\x9DV[\x92P\x92P\x92Pa\x01H\x82\x84\x83\x89`\0a\x01{V[\x95P\x83\x15a\x01`Wa\x01Y\x87a\x05\xCEV[\x96Pa\x01lV[\x85\x94PPPPPa\x01uV[PPPPa\x01\x19V[\x92\x91PPV[`\0`\x01`\x01`\xA0\x1B\x03\x80\x86\x16\x87\x82\x16\x10\x90\x83\x16a\x01\x99W`\0\x84\x90U[a\x01\xA4\x87\x87\x87a\x05\xE5V[`\x01`\x01`\xA0\x1B\x03\x16c\x12\x8A\xCB\x080\x83a\x01\xBD\x88a\x06#V[`\0\x03`\x01`\x01`\xA0\x1B\x03\x88\x16\x15a\x01\xD5W\x87a\x01\xFBV[\x85a\x01\xF4Ws\xFF\xFD\x89c\xEF\xD1\xFCjPd\x88I]\x95\x1DRc\x98\x8D%a\x01\xFBV[d\x01\0\x02v\xA4[\x8B\x8B\x8E`@Q` \x01a\x02\x10\x93\x92\x91\x90a\r\x12V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x02?\x95\x94\x93\x92\x91\x90a\raV[`@\x80Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x02XW`\0\x80\xFD[PZ\xF1\x92PPP\x80\x15a\x02\x88WP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x02\x85\x91\x81\x01\x90a\x0B\xF3V[`\x01[a\x02\xE1W=\x80\x80\x15a\x02\xB6W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x02\xBBV[``\x91P[P`\x01`\x01`\xA0\x1B\x03\x84\x16a\x02\xCFW`\0\x80U[a\x02\xD8\x81a\x069V[\x92PPPa\x02\xE5V[PPP[\x95\x94PPPPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0[`\0a\x03D\x84a\x05\x95V[\x90P`\0\x80`\0a\x03T\x87a\x05\x9DV[\x92P\x92P\x92Pa\x03h\x83\x83\x83\x89`\0a\x03\x84V[\x95P\x83\x15a\x01`Wa\x03y\x87a\x05\xCEV[\x96PPPPPa\x039V[`\0`\x01`\x01`\xA0\x1B\x03\x80\x86\x16\x90\x87\x16\x10a\x03\xA0\x87\x87\x87a\x05\xE5V[`\x01`\x01`\xA0\x1B\x03\x16c\x12\x8A\xCB\x080\x83a\x03\xB9\x88a\x06#V[`\x01`\x01`\xA0\x1B\x03\x88\x16\x15a\x03\xCEW\x87a\x03\xF4V[\x85a\x03\xEDWs\xFF\xFD\x89c\xEF\xD1\xFCjPd\x88I]\x95\x1DRc\x98\x8D%a\x03\xF4V[d\x01\0\x02v\xA4[\x8C\x8B\x8D`@Q` \x01a\x04\t\x93\x92\x91\x90a\r\x12V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x048\x95\x94\x93\x92\x91\x90a\raV[`@\x80Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04QW`\0\x80\xFD[PZ\xF1\x92PPP\x80\x15a\x04\x81WP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x04~\x91\x81\x01\x90a\x0B\xF3V[`\x01[a\x02\xE1W=\x80\x80\x15a\x04\xAFW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x04\xB4V[``\x91P[Pa\x02\xD8\x81a\x069V[`\0\x83\x13\x80a\x04\xCDWP`\0\x82\x13[a\x04\xD6W`\0\x80\xFD[`\0\x80`\0a\x04\xE4\x84a\x05\x9DV[\x92P\x92P\x92Pa\x05\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84\x84\x84a\x06\xB6V[P`\0\x80`\0\x80\x89\x13a\x05BW\x85`\x01`\x01`\xA0\x1B\x03\x16\x85`\x01`\x01`\xA0\x1B\x03\x16\x10\x88\x8A`\0\x03a\x05]V[\x84`\x01`\x01`\xA0\x1B\x03\x16\x86`\x01`\x01`\xA0\x1B\x03\x16\x10\x89\x89`\0\x03[\x92P\x92P\x92P\x82\x15a\x05tW`@Q\x81\x81R` \x81\xFD[`\0T\x15a\x05\x8AW`\0T\x81\x14a\x05\x8AW`\0\x80\xFD[`@Q\x82\x81R` \x81\xFD[Q`B\x11\x15\x90V[`\0\x80\x80a\x05\xAB\x84\x82a\x06\xCCV[\x92Pa\x05\xB8\x84`\x14a\x07|V[\x90Pa\x05\xC5\x84`\x17a\x06\xCCV[\x91P\x91\x93\x90\x92PV[\x80Q``\x90a\x01u\x90\x83\x90`\x17\x90`\x16\x19\x01a\x08#V[`\0a\x06\x1B\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x06\x16\x86\x86\x86a\ttV[a\t\xCAV[\x94\x93PPPPV[`\0`\x01`\xFF\x1B\x82\x10a\x065W`\0\x80\xFD[P\x90V[`\0\x81Q` \x14a\x06\xA2W`D\x82Q\x10\x15a\x06oW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06f\x90a\r\xC1V[`@Q\x80\x91\x03\x90\xFD[`\x04\x82\x01\x91P\x81\x80` \x01\x90Q\x81\x01\x90a\x06\x89\x91\x90a\x0CdV[`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06f\x91\x90a\r\xA7V[\x81\x80` \x01\x90Q\x81\x01\x90a\x01u\x91\x90a\x0C\xCEV[`\0a\x02\xE5\x85a\x06\xC7\x86\x86\x86a\ttV[a\n\xAEV[`\0\x81\x82`\x14\x01\x10\x15a\x07\x1BW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01RqtoAddress_overflow`p\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x81`\x14\x01\x83Q\x10\x15a\x07lW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01RttoAddress_outOfBounds`X\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[P\x01` \x01Q`\x01``\x1B\x90\x04\x90V[`\0\x81\x82`\x03\x01\x10\x15a\x07\xCAW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01RptoUint24_overflow`x\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x81`\x03\x01\x83Q\x10\x15a\x08\x1AW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01RstoUint24_outOfBounds``\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[P\x01`\x03\x01Q\x90V[``\x81\x82`\x1F\x01\x10\x15a\x08nW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rmslice_overflow`\x90\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x82\x82\x84\x01\x10\x15a\x08\xB6W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rmslice_overflow`\x90\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x81\x83\x01\x84Q\x10\x15a\t\x02W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01Rpslice_outOfBounds`x\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[``\x82\x15\x80\x15a\t!W`@Q\x91P`\0\x82R` \x82\x01`@Ra\tkV[`@Q\x91P`\x1F\x84\x16\x80\x15` \x02\x81\x84\x01\x01\x85\x81\x01\x87\x83\x15` \x02\x84\x8B\x01\x01\x01[\x81\x83\x10\x15a\tZW\x80Q\x83R` \x92\x83\x01\x92\x01a\tBV[PP\x85\x84R`\x1F\x01`\x1F\x19\x16`@RP[P\x94\x93PPPPV[a\t|a\n\xD1V[\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x11\x15a\t\x9AW\x91\x92\x91[P`@\x80Q``\x81\x01\x82R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x81R\x92\x90\x93\x16` \x83\x01Rb\xFF\xFF\xFF\x16\x91\x81\x01\x91\x90\x91R\x90V[`\0\x81` \x01Q`\x01`\x01`\xA0\x1B\x03\x16\x82`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x10a\t\xF2W`\0\x80\xFD[P\x80Q` \x80\x83\x01Q`@\x93\x84\x01Q\x84Q`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x81\x85\x01R\x93\x90\x91\x16\x83\x85\x01Rb\xFF\xFF\xFF\x16``\x80\x84\x01\x91\x90\x91R\x83Q\x80\x84\x03\x82\x01\x81R`\x80\x84\x01\x85R\x80Q\x90\x83\x01 `\x01`\x01`\xF8\x1B\x03\x19`\xA0\x85\x01R\x94\x90\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\xA1\x83\x01R`\xB5\x82\x01\x93\x90\x93R\x7F\xE3O\x19\x9B\x19\xB2\xB4\xF4\x7FhD&\x19\xD5UR}$Ox\xA3)~\xA8\x93%\xF8C\xF8{\x8BT`\xD5\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\xF5\x90\x91\x01\x90\x91R\x80Q\x91\x01 \x90V[`\0a\n\xBA\x83\x83a\t\xCAV[\x90P3`\x01`\x01`\xA0\x1B\x03\x82\x16\x14a\x01uW`\0\x80\xFD[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R\x90V[`\0\x82`\x1F\x83\x01\x12a\x0B\x01W\x80\x81\xFD[\x815a\x0B\x14a\x0B\x0F\x82a\x0E\x18V[a\r\xF4V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x0B(W\x82\x83\xFD[\x81` \x85\x01` \x83\x017\x90\x81\x01` \x01\x91\x90\x91R\x92\x91PPV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x0BYW\x80\x81\xFD[\x855a\x0Bd\x81a\x0EjV[\x94P` \x86\x015a\x0Bt\x81a\x0EjV[\x93P`@\x86\x015b\xFF\xFF\xFF\x81\x16\x81\x14a\x0B\x8BW\x81\x82\xFD[\x92P``\x86\x015\x91P`\x80\x86\x015a\x0B\xA2\x81a\x0EjV[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[`\0\x80`@\x83\x85\x03\x12\x15a\x0B\xC2W\x81\x82\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0B\xD8W\x82\x83\xFD[a\x0B\xE4\x85\x82\x86\x01a\n\xF1V[\x95` \x94\x90\x94\x015\x94PPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x0C\x05W\x81\x82\xFD[PP\x80Q` \x90\x91\x01Q\x90\x92\x90\x91PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x0C*W\x82\x83\xFD[\x835\x92P` \x84\x015\x91P`@\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0CNW\x81\x82\xFD[a\x0CZ\x86\x82\x87\x01a\n\xF1V[\x91PP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\x0CuW\x80\x81\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0C\x8BW\x81\x82\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x0C\x9BW\x81\x82\xFD[\x80Qa\x0C\xA9a\x0B\x0F\x82a\x0E\x18V[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a\x0C\xBDW\x83\x84\xFD[a\x02\xE5\x82` \x83\x01` \x86\x01a\x0E:V[`\0` \x82\x84\x03\x12\x15a\x0C\xDFW\x80\x81\xFD[PQ\x91\x90PV[`\0\x81Q\x80\x84Ra\x0C\xFE\x81` \x86\x01` \x86\x01a\x0E:V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[``\x93\x84\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x81\x16\x82R`\xE8\x93\x90\x93\x1B`\x01`\x01`\xE8\x1B\x03\x19\x16`\x14\x82\x01R\x92\x1B\x16`\x17\x82\x01R`+\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x81R` \x01\x90V[`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x82R\x85\x15\x15` \x83\x01R`@\x82\x01\x85\x90R\x83\x16``\x82\x01R`\xA0`\x80\x82\x01\x81\x90R`\0\x90a\r\x9C\x90\x83\x01\x84a\x0C\xE6V[\x97\x96PPPPPPPV[`\0` \x82Ra\r\xBA` \x83\x01\x84a\x0C\xE6V[\x93\x92PPPV[` \x80\x82R`\x10\x90\x82\x01Ro*\xB72\xBC82\xB1\xBA2\xB2\x102\xB997\xB9`\x81\x1B`@\x82\x01R``\x01\x90V[\x90\x81R` \x01\x90V[`@Q\x81\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0E\x10W\xFE[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0E,W\xFE[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0[\x83\x81\x10\x15a\x0EUW\x81\x81\x01Q\x83\x82\x01R` \x01a\x0E=V[\x83\x81\x11\x15a\x0EdW`\0\x84\x84\x01R[PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0E\x7FW`\0\x80\xFD[PV\xFE\xA2dipfsX\"\x12 `\xFF\xF3\xF31\x8C\xED\x068Z\x0E\xE6k\x05\x0FO\x8C\xCD\xF1\x97\x13\xA8\x1Ap%~<\xD5\xB3\xBD0\xF6dsolcC\0\x07\x06\x003";
    /// The deployed bytecode of the contract.
    pub static QUOTER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct Quoter<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Quoter<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Quoter<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Quoter<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Quoter<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Quoter)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Quoter<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    QUOTER_ABI.clone(),
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
                QUOTER_ABI.clone(),
                QUOTER_BYTECODE.clone().into(),
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
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([205, 202, 23, 83], (path, amount_in))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `quoteExactInputSingle` (0xf7729d43) function
        pub fn quote_exact_input_single(
            &self,
            token_in: ::ethers::core::types::Address,
            token_out: ::ethers::core::types::Address,
            fee: u32,
            amount_in: ::ethers::core::types::U256,
            sqrt_price_limit_x96: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [247, 114, 157, 67],
                    (token_in, token_out, fee, amount_in, sqrt_price_limit_x96),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `quoteExactOutput` (0x2f80bb1d) function
        pub fn quote_exact_output(
            &self,
            path: ::ethers::core::types::Bytes,
            amount_out: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([47, 128, 187, 29], (path, amount_out))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `quoteExactOutputSingle` (0x30d07f21) function
        pub fn quote_exact_output_single(
            &self,
            token_in: ::ethers::core::types::Address,
            token_out: ::ethers::core::types::Address,
            fee: u32,
            amount_out: ::ethers::core::types::U256,
            sqrt_price_limit_x96: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [48, 208, 127, 33],
                    (token_in, token_out, fee, amount_out, sqrt_price_limit_x96),
                )
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
    for Quoter<M> {
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
    ///Container type for all input parameters for the `quoteExactInputSingle` function with signature `quoteExactInputSingle(address,address,uint24,uint256,uint160)` and selector `0xf7729d43`
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
        abi = "quoteExactInputSingle(address,address,uint24,uint256,uint160)"
    )]
    pub struct QuoteExactInputSingleCall {
        pub token_in: ::ethers::core::types::Address,
        pub token_out: ::ethers::core::types::Address,
        pub fee: u32,
        pub amount_in: ::ethers::core::types::U256,
        pub sqrt_price_limit_x96: ::ethers::core::types::U256,
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
    ///Container type for all input parameters for the `quoteExactOutputSingle` function with signature `quoteExactOutputSingle(address,address,uint24,uint256,uint160)` and selector `0x30d07f21`
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
        abi = "quoteExactOutputSingle(address,address,uint24,uint256,uint160)"
    )]
    pub struct QuoteExactOutputSingleCall {
        pub token_in: ::ethers::core::types::Address,
        pub token_out: ::ethers::core::types::Address,
        pub fee: u32,
        pub amount_out: ::ethers::core::types::U256,
        pub sqrt_price_limit_x96: ::ethers::core::types::U256,
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
    pub enum QuoterCalls {
        Weth9(Weth9Call),
        Factory(FactoryCall),
        QuoteExactInput(QuoteExactInputCall),
        QuoteExactInputSingle(QuoteExactInputSingleCall),
        QuoteExactOutput(QuoteExactOutputCall),
        QuoteExactOutputSingle(QuoteExactOutputSingleCall),
        UniswapV3SwapCallback(UniswapV3SwapCallbackCall),
    }
    impl ::ethers::core::abi::AbiDecode for QuoterCalls {
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
    impl ::ethers::core::abi::AbiEncode for QuoterCalls {
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
    impl ::core::fmt::Display for QuoterCalls {
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
    impl ::core::convert::From<Weth9Call> for QuoterCalls {
        fn from(value: Weth9Call) -> Self {
            Self::Weth9(value)
        }
    }
    impl ::core::convert::From<FactoryCall> for QuoterCalls {
        fn from(value: FactoryCall) -> Self {
            Self::Factory(value)
        }
    }
    impl ::core::convert::From<QuoteExactInputCall> for QuoterCalls {
        fn from(value: QuoteExactInputCall) -> Self {
            Self::QuoteExactInput(value)
        }
    }
    impl ::core::convert::From<QuoteExactInputSingleCall> for QuoterCalls {
        fn from(value: QuoteExactInputSingleCall) -> Self {
            Self::QuoteExactInputSingle(value)
        }
    }
    impl ::core::convert::From<QuoteExactOutputCall> for QuoterCalls {
        fn from(value: QuoteExactOutputCall) -> Self {
            Self::QuoteExactOutput(value)
        }
    }
    impl ::core::convert::From<QuoteExactOutputSingleCall> for QuoterCalls {
        fn from(value: QuoteExactOutputSingleCall) -> Self {
            Self::QuoteExactOutputSingle(value)
        }
    }
    impl ::core::convert::From<UniswapV3SwapCallbackCall> for QuoterCalls {
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
    }
    ///Container type for all return fields from the `quoteExactInputSingle` function with signature `quoteExactInputSingle(address,address,uint24,uint256,uint160)` and selector `0xf7729d43`
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
    }
    ///Container type for all return fields from the `quoteExactOutputSingle` function with signature `quoteExactOutputSingle(address,address,uint24,uint256,uint160)` and selector `0x30d07f21`
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
    }
}
