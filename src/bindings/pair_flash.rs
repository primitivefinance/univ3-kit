pub use pair_flash::*;
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
pub mod pair_flash {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_swapRouter"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract ISwapRouter"),
                        ),
                    },
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
                    ::std::borrow::ToOwned::to_owned("initFlash"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("initFlash"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("params"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct PairFlash.FlashParams",),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
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
                    ::std::borrow::ToOwned::to_owned("swapRouter"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("swapRouter"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("contract ISwapRouter"),
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
                    ::std::borrow::ToOwned::to_owned("uniswapV3FlashCallback"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("uniswapV3FlashCallback",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("fee0"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("fee1"),
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
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: true,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static PAIRFLASH_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xE0`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0\x16\x1C8\x03\x80b\0\x16\x1C\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\0\\V[`\x01`\x01``\x1B\x03\x19``\x92\x83\x1B\x81\x16`\x80R\x90\x82\x1B\x81\x16`\xA0R\x91\x90\x1B\x16`\xC0Rb\0\0\xC8V[`\0\x80`\0``\x84\x86\x03\x12\x15b\0\0qW\x82\x83\xFD[\x83Qb\0\0~\x81b\0\0\xAFV[` \x85\x01Q\x90\x93Pb\0\0\x91\x81b\0\0\xAFV[`@\x85\x01Q\x90\x92Pb\0\0\xA4\x81b\0\0\xAFV[\x80\x91PP\x92P\x92P\x92V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\0\xC5W`\0\x80\xFD[PV[`\x80Q``\x1C`\xA0Q``\x1C`\xC0Q``\x1Ca\x14\xE3b\0\x019`\09\x80a\x04\xB8R\x80a\x06QR\x80a\x06~R\x80a\x07\x8DR\x80a\x08\x15RP\x80`\x8FR\x80a\x03\x1CR\x80a\x04\x07R\x80a\x04\x94R\x80a\r\xB5R\x80a\r\xFBR\x80a\x0EoRP\x80a\x02\x12R\x80a\x04\xDCR\x80a\x05\xF1RPa\x14\xE3`\0\xF3\xFE`\x80`@R`\x046\x10a\0\x7FW`\x005`\xE0\x1C\x80c\xC3\x1C\x9C\x07\x11a\0NW\x80c\xC3\x1C\x9C\x07\x14a\x01ZW\x80c\xC4Z\x01U\x14a\x01oW\x80c\xDF*\xB5\xBB\x14a\x01\x84W\x80c\xE9\xCB\xAF\xB0\x14a\x01\x97Wa\0\xEFV[\x80c\x12!\x0E\x8A\x14a\0\xF4W\x80c#\x84F;\x14a\0\xFCW\x80cI@K|\x14a\x01\x1CW\x80cJ\xA4\xA4\xFC\x14a\x01/Wa\0\xEFV[6a\0\xEFW3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\0\xEDW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01RhNot WETH9`\xB8\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\0[`\0\x80\xFD[a\0\xEDa\x01\xB7V[4\x80\x15a\x01\x08W`\0\x80\xFD[Pa\0\xEDa\x01\x176`\x04a\x11\xDEV[a\x01\xC9V[a\0\xEDa\x01*6`\x04a\x12\x91V[a\x03\x18V[4\x80\x15a\x01;W`\0\x80\xFD[Pa\x01Da\x04\x92V[`@Qa\x01Q\x91\x90a\x13@V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01fW`\0\x80\xFD[Pa\x01Da\x04\xB6V[4\x80\x15a\x01{W`\0\x80\xFD[Pa\x01Da\x04\xDAV[a\0\xEDa\x01\x926`\x04a\x10\xC5V[a\x04\xFEV[4\x80\x15a\x01\xA3W`\0\x80\xFD[Pa\0\xEDa\x01\xB26`\x04a\x12\xBCV[a\x05\xDCV[G\x15a\x01\xC7Wa\x01\xC73Ga\t\x12V[V[`\0`@Q\x80``\x01`@R\x80\x83`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83` \x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83`@\x01Qb\xFF\xFF\xFF\x16\x81RP\x90P`\0a\x027\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83a\n\x01V[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16cI\x0El\xBC0\x85``\x01Q\x86`\x80\x01Q`@Q\x80`\xC0\x01`@R\x80\x89``\x01Q\x81R` \x01\x89`\x80\x01Q\x81R` \x013`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x88\x81R` \x01\x89`\xA0\x01Qb\xFF\xFF\xFF\x16\x81R` \x01\x89`\xC0\x01Qb\xFF\xFF\xFF\x16\x81RP`@Q` \x01a\x02\xB3\x91\x90a\x14.V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x02\xE1\x94\x93\x92\x91\x90a\x13TV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x02\xFBW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x03\x0FW=`\0\x80>=`\0\xFD[PPPPPPPV[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16cp\xA0\x8210`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x82`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x91PP` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x03\x87W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x03\x9BW=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x03\xB1W`\0\x80\xFD[PQ\x90P\x82\x81\x10\x15a\x03\xFFW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01RqInsufficient WETH9`p\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x80\x15a\x04\x8DW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c.\x1A}M\x82`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x82\x81R` \x01\x91PP`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04kW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x04\x7FW=`\0\x80>=`\0\xFD[PPPPa\x04\x8D\x82\x82a\t\x12V[PPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0\x83`\x01`\x01`\xA0\x1B\x03\x16cp\xA0\x8210`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x82`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x91PP` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x05MW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x05aW=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x05wW`\0\x80\xFD[PQ\x90P\x82\x81\x10\x15a\x05\xC5W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01Rq$\xB79\xBA\xB334\xB1\xB4\xB2\xB7:\x10:7\xB5\xB2\xB7`q\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x80\x15a\x05\xD6Wa\x05\xD6\x84\x83\x83a\n\xE5V[PPPPV[`\0a\x05\xEA\x82\x84\x01\x84a\x11\0V[\x90Pa\x06\x1A\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82``\x01Qa\x0C3V[P``\x81\x01Q\x80Q` \x90\x91\x01Q\x82Q`\0\x90a\x067\x90\x89a\x0C\\V[\x90P`\0a\x06I\x85` \x01Q\x89a\x0C\\V[\x90Pa\x06z\x83\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x87` \x01Qa\x0ClV[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16cAK\xF3\x89`@Q\x80a\x01\0\x01`@R\x80\x87`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x88`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x89`\x80\x01Qb\xFF\xFF\xFF\x16\x81R` \x010`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01B\x81R` \x01\x89` \x01Q\x81R` \x01\x86\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81RP`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x073\x91\x90a\x13\xC5V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x07MW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x07aW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x85\x91\x90a\x12yV[\x90Pa\x07\xB6\x85\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x88`\0\x01Qa\x0ClV[`@\x80Qa\x01\0\x81\x01\x82R`\x01`\x01`\xA0\x1B\x03\x87\x81\x16\x82R\x86\x81\x16` \x83\x01R`\xA0\x89\x81\x01Qb\xFF\xFF\xFF\x16\x83\x85\x01R0``\x84\x01RB`\x80\x84\x01R\x89Q\x90\x83\x01R`\xC0\x82\x01\x85\x90R`\0`\xE0\x83\x01\x81\x90R\x92QcAK\xF3\x89`\xE0\x1B\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x91cAK\xF3\x89\x91a\x08K\x91\x90`\x04\x01a\x13\xC5V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x08eW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x08yW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\x9D\x91\x90a\x12yV[\x90P\x83\x15a\x08\xB1Wa\x08\xB1\x8603\x87a\r\xB3V[\x82\x15a\x08\xC3Wa\x08\xC3\x8503\x86a\r\xB3V[\x83\x82\x11\x15a\x08\xE4W`\0\x84\x83\x03\x90Pa\x08\xE2\x870\x8A`@\x01Q\x84a\r\xB3V[P[\x82\x81\x11\x15a\t\x05W`\0\x83\x82\x03\x90Pa\t\x03\x860\x8A`@\x01Q\x84a\r\xB3V[P[PPPPPPPPPPPV[`@\x80Q`\0\x80\x82R` \x82\x01\x90\x92R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x83\x90`@Q\x80\x82\x80Q\x90` \x01\x90\x80\x83\x83[` \x83\x10a\t^W\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01a\t?V[`\x01\x83` \x03a\x01\0\n\x03\x80\x19\x82Q\x16\x81\x84Q\x16\x80\x82\x17\x85RPPPPPP\x90P\x01\x91PP`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\t\xC0W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\t\xC5V[``\x91P[PP\x90P\x80a\x04\x8DW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x03`$\x82\x01RbSTE`\xE8\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\0\x81` \x01Q`\x01`\x01`\xA0\x1B\x03\x16\x82`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x10a\n)W`\0\x80\xFD[P\x80Q` \x80\x83\x01Q`@\x93\x84\x01Q\x84Q`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x81\x85\x01R\x93\x90\x91\x16\x83\x85\x01Rb\xFF\xFF\xFF\x16``\x80\x84\x01\x91\x90\x91R\x83Q\x80\x84\x03\x82\x01\x81R`\x80\x84\x01\x85R\x80Q\x90\x83\x01 `\x01`\x01`\xF8\x1B\x03\x19`\xA0\x85\x01R\x94\x90\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\xA1\x83\x01R`\xB5\x82\x01\x93\x90\x93R\x7F\xE3O\x19\x9B\x19\xB2\xB4\xF4\x7FhD&\x19\xD5UR}$Ox\xA3)~\xA8\x93%\xF8C\xF8{\x8BT`\xD5\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\xF5\x90\x91\x01\x90\x91R\x80Q\x91\x01 \x90V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`$\x83\x01R`D\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`d\x90\x92\x01\x83R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\xA9\x05\x9C\xBB`\xE0\x1B\x17\x81R\x92Q\x82Q`\0\x94\x85\x94\x93\x89\x16\x93\x92\x91\x82\x91\x90\x80\x83\x83[` \x83\x10a\x0BaW\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01a\x0BBV[`\x01\x83` \x03a\x01\0\n\x03\x80\x19\x82Q\x16\x81\x84Q\x16\x80\x82\x17\x85RPPPPPP\x90P\x01\x91PP`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x0B\xC3W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x0B\xC8V[``\x91P[P\x91P\x91P\x81\x80\x15a\x0B\xF6WP\x80Q\x15\x80a\x0B\xF6WP\x80\x80` \x01\x90Q` \x81\x10\x15a\x0B\xF3W`\0\x80\xFD[PQ[a\x0C,W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x02`$\x82\x01Ra\x14\xD5`\xF2\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[PPPPPV[`\0a\x0C?\x83\x83a\n\x01V[\x90P3`\x01`\x01`\xA0\x1B\x03\x82\x16\x14a\x0CVW`\0\x80\xFD[\x92\x91PPV[\x80\x82\x01\x82\x81\x10\x15a\x0CVW`\0\x80\xFD[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`$\x83\x01R`D\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`d\x90\x92\x01\x83R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\t^\xA7\xB3`\xE0\x1B\x17\x81R\x92Q\x82Q`\0\x94\x85\x94\x93\x89\x16\x93\x92\x91\x82\x91\x90\x80\x83\x83[` \x83\x10a\x0C\xE8W\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01a\x0C\xC9V[`\x01\x83` \x03a\x01\0\n\x03\x80\x19\x82Q\x16\x81\x84Q\x16\x80\x82\x17\x85RPPPPPP\x90P\x01\x91PP`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\rJW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\rOV[``\x91P[P\x91P\x91P\x81\x80\x15a\r}WP\x80Q\x15\x80a\r}WP\x80\x80` \x01\x90Q` \x81\x10\x15a\rzW`\0\x80\xFD[PQ[a\x0C,W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x02`$\x82\x01RaSA`\xF0\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x14\x80\x15a\r\xF4WP\x80G\x10\x15[\x15a\x0F\x16W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xD0\xE3\r\xB0\x82`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\x0ETW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0EhW=`\0\x80>=`\0\xFD[PPPPP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xA9\x05\x9C\xBB\x83\x83`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x83`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x82\x81R` \x01\x92PPP` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0E\xE4W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0E\xF8W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x0F\x0EW`\0\x80\xFD[Pa\x05\xD6\x90PV[`\x01`\x01`\xA0\x1B\x03\x83\x160\x14\x15a\x0F7Wa\x0F2\x84\x83\x83a\n\xE5V[a\x05\xD6V[a\x05\xD6\x84\x84\x84\x84`@\x80Q`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`$\x83\x01R\x84\x81\x16`D\x83\x01R`d\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`\x84\x90\x92\x01\x83R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c#\xB8r\xDD`\xE0\x1B\x17\x81R\x92Q\x82Q`\0\x94\x85\x94\x93\x8A\x16\x93\x92\x91\x82\x91\x90\x80\x83\x83[` \x83\x10a\x0F\xC2W\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01a\x0F\xA3V[`\x01\x83` \x03a\x01\0\n\x03\x80\x19\x82Q\x16\x81\x84Q\x16\x80\x82\x17\x85RPPPPPP\x90P\x01\x91PP`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x10$W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x10)V[``\x91P[P\x91P\x91P\x81\x80\x15a\x10WWP\x80Q\x15\x80a\x10WWP\x80\x80` \x01\x90Q` \x81\x10\x15a\x10TW`\0\x80\xFD[PQ[a\x10\x8EW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x03`$\x82\x01Rb)\xAA#`\xE9\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[PPPPPPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x10\xADW`\0\x80\xFD[\x91\x90PV[\x805b\xFF\xFF\xFF\x81\x16\x81\x14a\x10\xADW`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a\x10\xD9W\x82\x83\xFD[a\x10\xE2\x84a\x10\x96V[\x92P` \x84\x015\x91Pa\x10\xF7`@\x85\x01a\x10\x96V[\x90P\x92P\x92P\x92V[`\0\x81\x83\x03a\x01\0\x81\x12\x15a\x11\x13W\x81\x82\xFD[`@\x80Q`\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x82\x10\x81\x83\x11\x17\x15a\x112W\xFE[\x81\x84R\x865\x83R` \x87\x015` \x84\x01Ra\x11N\x84\x88\x01a\x10\x96V[\x83\x85\x01R```_\x19\x86\x01\x12\x15a\x11cW\x85\x86\xFD[\x83Q\x94P``\x85\x01\x91P\x84\x82\x10\x81\x83\x11\x17\x15a\x11{W\xFE[P\x82Ra\x11\x8A``\x86\x01a\x10\x96V[\x83Ra\x11\x98`\x80\x86\x01a\x10\x96V[` \x84\x01Ra\x11\xA9`\xA0\x86\x01a\x10\xB2V[\x82\x84\x01R\x82``\x82\x01Ra\x11\xBF`\xC0\x86\x01a\x10\xB2V[`\x80\x82\x01Ra\x11\xD0`\xE0\x86\x01a\x10\xB2V[`\xA0\x82\x01R\x95\x94PPPPPV[`\0`\xE0\x82\x84\x03\x12\x15a\x11\xEFW\x80\x81\xFD[`@Q`\xE0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x12\x0CW\xFE[`@Ra\x12\x18\x83a\x10\x96V[\x81Ra\x12&` \x84\x01a\x10\x96V[` \x82\x01Ra\x127`@\x84\x01a\x10\xB2V[`@\x82\x01R``\x83\x015``\x82\x01R`\x80\x83\x015`\x80\x82\x01Ra\x12\\`\xA0\x84\x01a\x10\xB2V[`\xA0\x82\x01Ra\x12m`\xC0\x84\x01a\x10\xB2V[`\xC0\x82\x01R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x12\x8AW\x80\x81\xFD[PQ\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x12\xA3W\x81\x82\xFD[\x825\x91Pa\x12\xB3` \x84\x01a\x10\x96V[\x90P\x92P\x92\x90PV[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a\x12\xD1W\x80\x81\xFD[\x845\x93P` \x85\x015\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x12\xF6W\x82\x83\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12a\x13\tW\x82\x83\xFD[\x815\x81\x81\x11\x15a\x13\x17W\x83\x84\xFD[\x88` \x82\x85\x01\x01\x11\x15a\x13(W\x83\x84\xFD[\x95\x98\x94\x97PP` \x01\x94PPPV[b\xFF\xFF\xFF\x16\x90RV[`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x81R` \x01\x90V[`\0`\x01\x80`\xA0\x1B\x03\x86\x16\x82R` \x85\x81\x84\x01R\x84`@\x84\x01R`\x80``\x84\x01R\x83Q\x80`\x80\x85\x01R\x82[\x81\x81\x10\x15a\x13\x9BW\x85\x81\x01\x83\x01Q\x85\x82\x01`\xA0\x01R\x82\x01a\x13\x7FV[\x81\x81\x11\x15a\x13\xACW\x83`\xA0\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01`\xA0\x01\x96\x95PPPPPPV[\x81Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R` \x80\x84\x01Q\x82\x16\x90\x83\x01R`@\x80\x84\x01Qb\xFF\xFF\xFF\x16\x90\x83\x01R``\x80\x84\x01Q\x82\x16\x90\x83\x01R`\x80\x80\x84\x01Q\x90\x83\x01R`\xA0\x83\x81\x01Q\x90\x83\x01R`\xC0\x80\x84\x01Q\x90\x83\x01R`\xE0\x92\x83\x01Q\x16\x91\x81\x01\x91\x90\x91Ra\x01\0\x01\x90V[\x81Q\x81R` \x80\x83\x01Q\x81\x83\x01R`@\x80\x84\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82\x85\x01R``\x80\x86\x01Q\x80Q\x83\x16\x91\x86\x01\x91\x90\x91R\x92\x83\x01Q\x16`\x80\x80\x85\x01\x91\x90\x91R\x91\x01Qb\xFF\xFF\xFF\x16`\xA0\x83\x01R\x82\x01Qa\x01\0\x82\x01\x90a\x14\x93`\xC0\x84\x01\x82a\x137V[P`\xA0\x83\x01Qa\x14\xA6`\xE0\x84\x01\x82a\x137V[P\x92\x91PPV\xFE\xA2dipfsX\"\x12 \x967\xA6\xF2U\xEAN\xC9\xC4^\xC7\x1E\x94v[\xE3!V\x96\xA2\x1A\xD5\xD6\x13\x87W'\xA8%\xDF\xD11dsolcC\0\x07\x06\x003";
    /// The bytecode of the contract.
    pub static PAIRFLASH_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0\x7FW`\x005`\xE0\x1C\x80c\xC3\x1C\x9C\x07\x11a\0NW\x80c\xC3\x1C\x9C\x07\x14a\x01ZW\x80c\xC4Z\x01U\x14a\x01oW\x80c\xDF*\xB5\xBB\x14a\x01\x84W\x80c\xE9\xCB\xAF\xB0\x14a\x01\x97Wa\0\xEFV[\x80c\x12!\x0E\x8A\x14a\0\xF4W\x80c#\x84F;\x14a\0\xFCW\x80cI@K|\x14a\x01\x1CW\x80cJ\xA4\xA4\xFC\x14a\x01/Wa\0\xEFV[6a\0\xEFW3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\0\xEDW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01RhNot WETH9`\xB8\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\0[`\0\x80\xFD[a\0\xEDa\x01\xB7V[4\x80\x15a\x01\x08W`\0\x80\xFD[Pa\0\xEDa\x01\x176`\x04a\x11\xDEV[a\x01\xC9V[a\0\xEDa\x01*6`\x04a\x12\x91V[a\x03\x18V[4\x80\x15a\x01;W`\0\x80\xFD[Pa\x01Da\x04\x92V[`@Qa\x01Q\x91\x90a\x13@V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01fW`\0\x80\xFD[Pa\x01Da\x04\xB6V[4\x80\x15a\x01{W`\0\x80\xFD[Pa\x01Da\x04\xDAV[a\0\xEDa\x01\x926`\x04a\x10\xC5V[a\x04\xFEV[4\x80\x15a\x01\xA3W`\0\x80\xFD[Pa\0\xEDa\x01\xB26`\x04a\x12\xBCV[a\x05\xDCV[G\x15a\x01\xC7Wa\x01\xC73Ga\t\x12V[V[`\0`@Q\x80``\x01`@R\x80\x83`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83` \x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83`@\x01Qb\xFF\xFF\xFF\x16\x81RP\x90P`\0a\x027\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83a\n\x01V[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16cI\x0El\xBC0\x85``\x01Q\x86`\x80\x01Q`@Q\x80`\xC0\x01`@R\x80\x89``\x01Q\x81R` \x01\x89`\x80\x01Q\x81R` \x013`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x88\x81R` \x01\x89`\xA0\x01Qb\xFF\xFF\xFF\x16\x81R` \x01\x89`\xC0\x01Qb\xFF\xFF\xFF\x16\x81RP`@Q` \x01a\x02\xB3\x91\x90a\x14.V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x02\xE1\x94\x93\x92\x91\x90a\x13TV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x02\xFBW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x03\x0FW=`\0\x80>=`\0\xFD[PPPPPPPV[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16cp\xA0\x8210`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x82`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x91PP` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x03\x87W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x03\x9BW=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x03\xB1W`\0\x80\xFD[PQ\x90P\x82\x81\x10\x15a\x03\xFFW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01RqInsufficient WETH9`p\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x80\x15a\x04\x8DW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c.\x1A}M\x82`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x82\x81R` \x01\x91PP`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04kW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x04\x7FW=`\0\x80>=`\0\xFD[PPPPa\x04\x8D\x82\x82a\t\x12V[PPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0\x83`\x01`\x01`\xA0\x1B\x03\x16cp\xA0\x8210`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x82`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x91PP` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x05MW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x05aW=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x05wW`\0\x80\xFD[PQ\x90P\x82\x81\x10\x15a\x05\xC5W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01Rq$\xB79\xBA\xB334\xB1\xB4\xB2\xB7:\x10:7\xB5\xB2\xB7`q\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x80\x15a\x05\xD6Wa\x05\xD6\x84\x83\x83a\n\xE5V[PPPPV[`\0a\x05\xEA\x82\x84\x01\x84a\x11\0V[\x90Pa\x06\x1A\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82``\x01Qa\x0C3V[P``\x81\x01Q\x80Q` \x90\x91\x01Q\x82Q`\0\x90a\x067\x90\x89a\x0C\\V[\x90P`\0a\x06I\x85` \x01Q\x89a\x0C\\V[\x90Pa\x06z\x83\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x87` \x01Qa\x0ClV[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16cAK\xF3\x89`@Q\x80a\x01\0\x01`@R\x80\x87`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x88`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x89`\x80\x01Qb\xFF\xFF\xFF\x16\x81R` \x010`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01B\x81R` \x01\x89` \x01Q\x81R` \x01\x86\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81RP`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x073\x91\x90a\x13\xC5V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x07MW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x07aW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x85\x91\x90a\x12yV[\x90Pa\x07\xB6\x85\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x88`\0\x01Qa\x0ClV[`@\x80Qa\x01\0\x81\x01\x82R`\x01`\x01`\xA0\x1B\x03\x87\x81\x16\x82R\x86\x81\x16` \x83\x01R`\xA0\x89\x81\x01Qb\xFF\xFF\xFF\x16\x83\x85\x01R0``\x84\x01RB`\x80\x84\x01R\x89Q\x90\x83\x01R`\xC0\x82\x01\x85\x90R`\0`\xE0\x83\x01\x81\x90R\x92QcAK\xF3\x89`\xE0\x1B\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x91cAK\xF3\x89\x91a\x08K\x91\x90`\x04\x01a\x13\xC5V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x08eW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x08yW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\x9D\x91\x90a\x12yV[\x90P\x83\x15a\x08\xB1Wa\x08\xB1\x8603\x87a\r\xB3V[\x82\x15a\x08\xC3Wa\x08\xC3\x8503\x86a\r\xB3V[\x83\x82\x11\x15a\x08\xE4W`\0\x84\x83\x03\x90Pa\x08\xE2\x870\x8A`@\x01Q\x84a\r\xB3V[P[\x82\x81\x11\x15a\t\x05W`\0\x83\x82\x03\x90Pa\t\x03\x860\x8A`@\x01Q\x84a\r\xB3V[P[PPPPPPPPPPPV[`@\x80Q`\0\x80\x82R` \x82\x01\x90\x92R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x83\x90`@Q\x80\x82\x80Q\x90` \x01\x90\x80\x83\x83[` \x83\x10a\t^W\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01a\t?V[`\x01\x83` \x03a\x01\0\n\x03\x80\x19\x82Q\x16\x81\x84Q\x16\x80\x82\x17\x85RPPPPPP\x90P\x01\x91PP`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\t\xC0W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\t\xC5V[``\x91P[PP\x90P\x80a\x04\x8DW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x03`$\x82\x01RbSTE`\xE8\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\0\x81` \x01Q`\x01`\x01`\xA0\x1B\x03\x16\x82`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x10a\n)W`\0\x80\xFD[P\x80Q` \x80\x83\x01Q`@\x93\x84\x01Q\x84Q`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x81\x85\x01R\x93\x90\x91\x16\x83\x85\x01Rb\xFF\xFF\xFF\x16``\x80\x84\x01\x91\x90\x91R\x83Q\x80\x84\x03\x82\x01\x81R`\x80\x84\x01\x85R\x80Q\x90\x83\x01 `\x01`\x01`\xF8\x1B\x03\x19`\xA0\x85\x01R\x94\x90\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\xA1\x83\x01R`\xB5\x82\x01\x93\x90\x93R\x7F\xE3O\x19\x9B\x19\xB2\xB4\xF4\x7FhD&\x19\xD5UR}$Ox\xA3)~\xA8\x93%\xF8C\xF8{\x8BT`\xD5\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\xF5\x90\x91\x01\x90\x91R\x80Q\x91\x01 \x90V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`$\x83\x01R`D\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`d\x90\x92\x01\x83R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\xA9\x05\x9C\xBB`\xE0\x1B\x17\x81R\x92Q\x82Q`\0\x94\x85\x94\x93\x89\x16\x93\x92\x91\x82\x91\x90\x80\x83\x83[` \x83\x10a\x0BaW\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01a\x0BBV[`\x01\x83` \x03a\x01\0\n\x03\x80\x19\x82Q\x16\x81\x84Q\x16\x80\x82\x17\x85RPPPPPP\x90P\x01\x91PP`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x0B\xC3W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x0B\xC8V[``\x91P[P\x91P\x91P\x81\x80\x15a\x0B\xF6WP\x80Q\x15\x80a\x0B\xF6WP\x80\x80` \x01\x90Q` \x81\x10\x15a\x0B\xF3W`\0\x80\xFD[PQ[a\x0C,W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x02`$\x82\x01Ra\x14\xD5`\xF2\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[PPPPPV[`\0a\x0C?\x83\x83a\n\x01V[\x90P3`\x01`\x01`\xA0\x1B\x03\x82\x16\x14a\x0CVW`\0\x80\xFD[\x92\x91PPV[\x80\x82\x01\x82\x81\x10\x15a\x0CVW`\0\x80\xFD[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`$\x83\x01R`D\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`d\x90\x92\x01\x83R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\t^\xA7\xB3`\xE0\x1B\x17\x81R\x92Q\x82Q`\0\x94\x85\x94\x93\x89\x16\x93\x92\x91\x82\x91\x90\x80\x83\x83[` \x83\x10a\x0C\xE8W\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01a\x0C\xC9V[`\x01\x83` \x03a\x01\0\n\x03\x80\x19\x82Q\x16\x81\x84Q\x16\x80\x82\x17\x85RPPPPPP\x90P\x01\x91PP`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\rJW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\rOV[``\x91P[P\x91P\x91P\x81\x80\x15a\r}WP\x80Q\x15\x80a\r}WP\x80\x80` \x01\x90Q` \x81\x10\x15a\rzW`\0\x80\xFD[PQ[a\x0C,W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x02`$\x82\x01RaSA`\xF0\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x14\x80\x15a\r\xF4WP\x80G\x10\x15[\x15a\x0F\x16W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xD0\xE3\r\xB0\x82`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\x0ETW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0EhW=`\0\x80>=`\0\xFD[PPPPP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xA9\x05\x9C\xBB\x83\x83`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x83`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x82\x81R` \x01\x92PPP` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0E\xE4W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0E\xF8W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x0F\x0EW`\0\x80\xFD[Pa\x05\xD6\x90PV[`\x01`\x01`\xA0\x1B\x03\x83\x160\x14\x15a\x0F7Wa\x0F2\x84\x83\x83a\n\xE5V[a\x05\xD6V[a\x05\xD6\x84\x84\x84\x84`@\x80Q`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`$\x83\x01R\x84\x81\x16`D\x83\x01R`d\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`\x84\x90\x92\x01\x83R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c#\xB8r\xDD`\xE0\x1B\x17\x81R\x92Q\x82Q`\0\x94\x85\x94\x93\x8A\x16\x93\x92\x91\x82\x91\x90\x80\x83\x83[` \x83\x10a\x0F\xC2W\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01a\x0F\xA3V[`\x01\x83` \x03a\x01\0\n\x03\x80\x19\x82Q\x16\x81\x84Q\x16\x80\x82\x17\x85RPPPPPP\x90P\x01\x91PP`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x10$W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x10)V[``\x91P[P\x91P\x91P\x81\x80\x15a\x10WWP\x80Q\x15\x80a\x10WWP\x80\x80` \x01\x90Q` \x81\x10\x15a\x10TW`\0\x80\xFD[PQ[a\x10\x8EW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x03`$\x82\x01Rb)\xAA#`\xE9\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[PPPPPPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x10\xADW`\0\x80\xFD[\x91\x90PV[\x805b\xFF\xFF\xFF\x81\x16\x81\x14a\x10\xADW`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a\x10\xD9W\x82\x83\xFD[a\x10\xE2\x84a\x10\x96V[\x92P` \x84\x015\x91Pa\x10\xF7`@\x85\x01a\x10\x96V[\x90P\x92P\x92P\x92V[`\0\x81\x83\x03a\x01\0\x81\x12\x15a\x11\x13W\x81\x82\xFD[`@\x80Q`\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x82\x10\x81\x83\x11\x17\x15a\x112W\xFE[\x81\x84R\x865\x83R` \x87\x015` \x84\x01Ra\x11N\x84\x88\x01a\x10\x96V[\x83\x85\x01R```_\x19\x86\x01\x12\x15a\x11cW\x85\x86\xFD[\x83Q\x94P``\x85\x01\x91P\x84\x82\x10\x81\x83\x11\x17\x15a\x11{W\xFE[P\x82Ra\x11\x8A``\x86\x01a\x10\x96V[\x83Ra\x11\x98`\x80\x86\x01a\x10\x96V[` \x84\x01Ra\x11\xA9`\xA0\x86\x01a\x10\xB2V[\x82\x84\x01R\x82``\x82\x01Ra\x11\xBF`\xC0\x86\x01a\x10\xB2V[`\x80\x82\x01Ra\x11\xD0`\xE0\x86\x01a\x10\xB2V[`\xA0\x82\x01R\x95\x94PPPPPV[`\0`\xE0\x82\x84\x03\x12\x15a\x11\xEFW\x80\x81\xFD[`@Q`\xE0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x12\x0CW\xFE[`@Ra\x12\x18\x83a\x10\x96V[\x81Ra\x12&` \x84\x01a\x10\x96V[` \x82\x01Ra\x127`@\x84\x01a\x10\xB2V[`@\x82\x01R``\x83\x015``\x82\x01R`\x80\x83\x015`\x80\x82\x01Ra\x12\\`\xA0\x84\x01a\x10\xB2V[`\xA0\x82\x01Ra\x12m`\xC0\x84\x01a\x10\xB2V[`\xC0\x82\x01R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x12\x8AW\x80\x81\xFD[PQ\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x12\xA3W\x81\x82\xFD[\x825\x91Pa\x12\xB3` \x84\x01a\x10\x96V[\x90P\x92P\x92\x90PV[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a\x12\xD1W\x80\x81\xFD[\x845\x93P` \x85\x015\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x12\xF6W\x82\x83\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12a\x13\tW\x82\x83\xFD[\x815\x81\x81\x11\x15a\x13\x17W\x83\x84\xFD[\x88` \x82\x85\x01\x01\x11\x15a\x13(W\x83\x84\xFD[\x95\x98\x94\x97PP` \x01\x94PPPV[b\xFF\xFF\xFF\x16\x90RV[`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x81R` \x01\x90V[`\0`\x01\x80`\xA0\x1B\x03\x86\x16\x82R` \x85\x81\x84\x01R\x84`@\x84\x01R`\x80``\x84\x01R\x83Q\x80`\x80\x85\x01R\x82[\x81\x81\x10\x15a\x13\x9BW\x85\x81\x01\x83\x01Q\x85\x82\x01`\xA0\x01R\x82\x01a\x13\x7FV[\x81\x81\x11\x15a\x13\xACW\x83`\xA0\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01`\xA0\x01\x96\x95PPPPPPV[\x81Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R` \x80\x84\x01Q\x82\x16\x90\x83\x01R`@\x80\x84\x01Qb\xFF\xFF\xFF\x16\x90\x83\x01R``\x80\x84\x01Q\x82\x16\x90\x83\x01R`\x80\x80\x84\x01Q\x90\x83\x01R`\xA0\x83\x81\x01Q\x90\x83\x01R`\xC0\x80\x84\x01Q\x90\x83\x01R`\xE0\x92\x83\x01Q\x16\x91\x81\x01\x91\x90\x91Ra\x01\0\x01\x90V[\x81Q\x81R` \x80\x83\x01Q\x81\x83\x01R`@\x80\x84\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82\x85\x01R``\x80\x86\x01Q\x80Q\x83\x16\x91\x86\x01\x91\x90\x91R\x92\x83\x01Q\x16`\x80\x80\x85\x01\x91\x90\x91R\x91\x01Qb\xFF\xFF\xFF\x16`\xA0\x83\x01R\x82\x01Qa\x01\0\x82\x01\x90a\x14\x93`\xC0\x84\x01\x82a\x137V[P`\xA0\x83\x01Qa\x14\xA6`\xE0\x84\x01\x82a\x137V[P\x92\x91PPV\xFE\xA2dipfsX\"\x12 \x967\xA6\xF2U\xEAN\xC9\xC4^\xC7\x1E\x94v[\xE3!V\x96\xA2\x1A\xD5\xD6\x13\x87W'\xA8%\xDF\xD11dsolcC\0\x07\x06\x003";
    /// The deployed bytecode of the contract.
    pub static PAIRFLASH_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct PairFlash<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for PairFlash<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for PairFlash<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for PairFlash<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for PairFlash<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(PairFlash))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> PairFlash<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                PAIRFLASH_ABI.clone(),
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
                PAIRFLASH_ABI.clone(),
                PAIRFLASH_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `WETH9` (0x4aa4a4fc) function
        pub fn weth9(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([74, 164, 164, 252], ())
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
        ///Calls the contract's `initFlash` (0x2384463b) function
        pub fn init_flash(
            &self,
            params: FlashParams,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([35, 132, 70, 59], (params,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `refundETH` (0x12210e8a) function
        pub fn refund_eth(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([18, 33, 14, 138], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swapRouter` (0xc31c9c07) function
        pub fn swap_router(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([195, 28, 156, 7], ())
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
        ///Calls the contract's `uniswapV3FlashCallback` (0xe9cbafb0) function
        pub fn uniswap_v3_flash_callback(
            &self,
            fee_0: ::ethers::core::types::U256,
            fee_1: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([233, 203, 175, 176], (fee_0, fee_1, data))
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
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for PairFlash<M> {
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
        Hash,
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
        Hash,
    )]
    #[ethcall(name = "factory", abi = "factory()")]
    pub struct FactoryCall;
    ///Container type for all input parameters for the `initFlash` function with signature `initFlash((address,address,uint24,uint256,uint256,uint24,uint24))` and selector `0x2384463b`
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
        name = "initFlash",
        abi = "initFlash((address,address,uint24,uint256,uint256,uint24,uint24))"
    )]
    pub struct InitFlashCall {
        pub params: FlashParams,
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
    ///Container type for all input parameters for the `swapRouter` function with signature `swapRouter()` and selector `0xc31c9c07`
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
    #[ethcall(name = "swapRouter", abi = "swapRouter()")]
    pub struct SwapRouterCall;
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
    ///Container type for all input parameters for the `uniswapV3FlashCallback` function with signature `uniswapV3FlashCallback(uint256,uint256,bytes)` and selector `0xe9cbafb0`
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
        name = "uniswapV3FlashCallback",
        abi = "uniswapV3FlashCallback(uint256,uint256,bytes)"
    )]
    pub struct UniswapV3FlashCallbackCall {
        pub fee_0: ::ethers::core::types::U256,
        pub fee_1: ::ethers::core::types::U256,
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
    pub enum PairFlashCalls {
        Weth9(Weth9Call),
        Factory(FactoryCall),
        InitFlash(InitFlashCall),
        RefundETH(RefundETHCall),
        SwapRouter(SwapRouterCall),
        SweepToken(SweepTokenCall),
        UniswapV3FlashCallback(UniswapV3FlashCallbackCall),
        UnwrapWETH9(UnwrapWETH9Call),
    }
    impl ::ethers::core::abi::AbiDecode for PairFlashCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <Weth9Call as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Weth9(decoded));
            }
            if let Ok(decoded) = <FactoryCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Factory(decoded));
            }
            if let Ok(decoded) = <InitFlashCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InitFlash(decoded));
            }
            if let Ok(decoded) = <RefundETHCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RefundETH(decoded));
            }
            if let Ok(decoded) = <SwapRouterCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SwapRouter(decoded));
            }
            if let Ok(decoded) = <SweepTokenCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SweepToken(decoded));
            }
            if let Ok(decoded) =
                <UniswapV3FlashCallbackCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UniswapV3FlashCallback(decoded));
            }
            if let Ok(decoded) = <UnwrapWETH9Call as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UnwrapWETH9(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for PairFlashCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Weth9(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Factory(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InitFlash(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RefundETH(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SwapRouter(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SweepToken(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UniswapV3FlashCallback(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnwrapWETH9(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for PairFlashCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Weth9(element) => ::core::fmt::Display::fmt(element, f),
                Self::Factory(element) => ::core::fmt::Display::fmt(element, f),
                Self::InitFlash(element) => ::core::fmt::Display::fmt(element, f),
                Self::RefundETH(element) => ::core::fmt::Display::fmt(element, f),
                Self::SwapRouter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SweepToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::UniswapV3FlashCallback(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnwrapWETH9(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<Weth9Call> for PairFlashCalls {
        fn from(value: Weth9Call) -> Self {
            Self::Weth9(value)
        }
    }
    impl ::core::convert::From<FactoryCall> for PairFlashCalls {
        fn from(value: FactoryCall) -> Self {
            Self::Factory(value)
        }
    }
    impl ::core::convert::From<InitFlashCall> for PairFlashCalls {
        fn from(value: InitFlashCall) -> Self {
            Self::InitFlash(value)
        }
    }
    impl ::core::convert::From<RefundETHCall> for PairFlashCalls {
        fn from(value: RefundETHCall) -> Self {
            Self::RefundETH(value)
        }
    }
    impl ::core::convert::From<SwapRouterCall> for PairFlashCalls {
        fn from(value: SwapRouterCall) -> Self {
            Self::SwapRouter(value)
        }
    }
    impl ::core::convert::From<SweepTokenCall> for PairFlashCalls {
        fn from(value: SweepTokenCall) -> Self {
            Self::SweepToken(value)
        }
    }
    impl ::core::convert::From<UniswapV3FlashCallbackCall> for PairFlashCalls {
        fn from(value: UniswapV3FlashCallbackCall) -> Self {
            Self::UniswapV3FlashCallback(value)
        }
    }
    impl ::core::convert::From<UnwrapWETH9Call> for PairFlashCalls {
        fn from(value: UnwrapWETH9Call) -> Self {
            Self::UnwrapWETH9(value)
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
        Hash,
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
        Hash,
    )]
    pub struct FactoryReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `swapRouter` function with signature `swapRouter()` and selector `0xc31c9c07`
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
    pub struct SwapRouterReturn(pub ::ethers::core::types::Address);
    ///`FlashParams(address,address,uint24,uint256,uint256,uint24,uint24)`
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
    pub struct FlashParams {
        pub token_0: ::ethers::core::types::Address,
        pub token_1: ::ethers::core::types::Address,
        pub fee_1: u32,
        pub amount_0: ::ethers::core::types::U256,
        pub amount_1: ::ethers::core::types::U256,
        pub fee_2: u32,
        pub fee_3: u32,
    }
}
