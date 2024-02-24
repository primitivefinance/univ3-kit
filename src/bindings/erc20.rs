pub use erc20::*;
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
pub mod erc20 {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("name_"),
                        kind: ::ethers::core::abi::ethabi::ParamType::String,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("string"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("symbol_"),
                        kind: ::ethers::core::abi::ethabi::ParamType::String,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("string"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("allowance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("allowance"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("spender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("approve"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("approve"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("spender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("balanceOf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("balanceOf"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("decimals"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("decimals"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("decreaseAllowance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("decreaseAllowance"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("spender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("subtractedValue"),
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
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("increaseAllowance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("increaseAllowance"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("spender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("addedValue"),
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
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("name"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("name"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("symbol"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("symbol"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("totalSupply"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("totalSupply"),
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
                    ::std::borrow::ToOwned::to_owned("transfer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transfer"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transferFrom"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transferFrom"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Approval"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Approval"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("spender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
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
                    ::std::borrow::ToOwned::to_owned("Transfer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
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
                                    name: ::std::borrow::ToOwned::to_owned("value"),
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
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static ERC20_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0\x0C\x8E8\x03\x80b\0\x0C\x8E\x839\x81\x81\x01`@R`@\x81\x10\x15b\0\x007W`\0\x80\xFD[\x81\x01\x90\x80\x80Q`@Q\x93\x92\x91\x90\x84d\x01\0\0\0\0\x82\x11\x15b\0\0XW`\0\x80\xFD[\x90\x83\x01\x90` \x82\x01\x85\x81\x11\x15b\0\0nW`\0\x80\xFD[\x82Qd\x01\0\0\0\0\x81\x11\x82\x82\x01\x88\x10\x17\x15b\0\0\x89W`\0\x80\xFD[\x82RP\x81Q` \x91\x82\x01\x92\x90\x91\x01\x90\x80\x83\x83`\0[\x83\x81\x10\x15b\0\0\xB8W\x81\x81\x01Q\x83\x82\x01R` \x01b\0\0\x9EV[PPPP\x90P\x90\x81\x01\x90`\x1F\x16\x80\x15b\0\0\xE6W\x80\x82\x03\x80Q`\x01\x83` \x03a\x01\0\n\x03\x19\x16\x81R` \x01\x91P[P`@R` \x01\x80Q`@Q\x93\x92\x91\x90\x84d\x01\0\0\0\0\x82\x11\x15b\0\x01\nW`\0\x80\xFD[\x90\x83\x01\x90` \x82\x01\x85\x81\x11\x15b\0\x01 W`\0\x80\xFD[\x82Qd\x01\0\0\0\0\x81\x11\x82\x82\x01\x88\x10\x17\x15b\0\x01;W`\0\x80\xFD[\x82RP\x81Q` \x91\x82\x01\x92\x90\x91\x01\x90\x80\x83\x83`\0[\x83\x81\x10\x15b\0\x01jW\x81\x81\x01Q\x83\x82\x01R` \x01b\0\x01PV[PPPP\x90P\x90\x81\x01\x90`\x1F\x16\x80\x15b\0\x01\x98W\x80\x82\x03\x80Q`\x01\x83` \x03a\x01\0\n\x03\x19\x16\x81R` \x01\x91P[P`@RPP\x82Qb\0\x01\xB4\x91P`\x03\x90` \x85\x01\x90b\0\x01\xE0V[P\x80Qb\0\x01\xCA\x90`\x04\x90` \x84\x01\x90b\0\x01\xE0V[PP`\x05\x80T`\xFF\x19\x16`\x12\x17\x90UPb\0\x02\x8CV[\x82\x80T`\x01\x81`\x01\x16\x15a\x01\0\x02\x03\x16`\x02\x90\x04\x90`\0R` `\0 \x90`\x1F\x01` \x90\x04\x81\x01\x92\x82b\0\x02\x18W`\0\x85Ub\0\x02cV[\x82`\x1F\x10b\0\x023W\x80Q`\xFF\x19\x16\x83\x80\x01\x17\x85Ub\0\x02cV[\x82\x80\x01`\x01\x01\x85U\x82\x15b\0\x02cW\x91\x82\x01[\x82\x81\x11\x15b\0\x02cW\x82Q\x82U\x91` \x01\x91\x90`\x01\x01\x90b\0\x02FV[Pb\0\x02q\x92\x91Pb\0\x02uV[P\x90V[[\x80\x82\x11\x15b\0\x02qW`\0\x81U`\x01\x01b\0\x02vV[a\t\xF2\x80b\0\x02\x9C`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xA9W`\x005`\xE0\x1C\x80c9P\x93Q\x11a\0qW\x80c9P\x93Q\x14a\x01\xD9W\x80cp\xA0\x821\x14a\x02\x05W\x80c\x95\xD8\x9BA\x14a\x02+W\x80c\xA4W\xC2\xD7\x14a\x023W\x80c\xA9\x05\x9C\xBB\x14a\x02_W\x80c\xDDb\xED>\x14a\x02\x8BWa\0\xA9V[\x80c\x06\xFD\xDE\x03\x14a\0\xAEW\x80c\t^\xA7\xB3\x14a\x01+W\x80c\x18\x16\r\xDD\x14a\x01kW\x80c#\xB8r\xDD\x14a\x01\x85W\x80c1<\xE5g\x14a\x01\xBBW[`\0\x80\xFD[a\0\xB6a\x02\xB9V[`@\x80Q` \x80\x82R\x83Q\x81\x83\x01R\x83Q\x91\x92\x83\x92\x90\x83\x01\x91\x85\x01\x90\x80\x83\x83`\0[\x83\x81\x10\x15a\0\xF0W\x81\x81\x01Q\x83\x82\x01R` \x01a\0\xD8V[PPPP\x90P\x90\x81\x01\x90`\x1F\x16\x80\x15a\x01\x1DW\x80\x82\x03\x80Q`\x01\x83` \x03a\x01\0\n\x03\x19\x16\x81R` \x01\x91P[P\x92PPP`@Q\x80\x91\x03\x90\xF3[a\x01W`\x04\x806\x03`@\x81\x10\x15a\x01AW`\0\x80\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x16\x90` \x015a\x03OV[`@\x80Q\x91\x15\x15\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[a\x01sa\x03lV[`@\x80Q\x91\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[a\x01W`\x04\x806\x03``\x81\x10\x15a\x01\x9BW`\0\x80\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x81\x16\x91` \x81\x015\x90\x91\x16\x90`@\x015a\x03rV[a\x01\xC3a\x03\xF9V[`@\x80Q`\xFF\x90\x92\x16\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[a\x01W`\x04\x806\x03`@\x81\x10\x15a\x01\xEFW`\0\x80\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x16\x90` \x015a\x04\x02V[a\x01s`\x04\x806\x03` \x81\x10\x15a\x02\x1BW`\0\x80\xFD[P5`\x01`\x01`\xA0\x1B\x03\x16a\x04PV[a\0\xB6a\x04kV[a\x01W`\x04\x806\x03`@\x81\x10\x15a\x02IW`\0\x80\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x16\x90` \x015a\x04\xCCV[a\x01W`\x04\x806\x03`@\x81\x10\x15a\x02uW`\0\x80\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x16\x90` \x015a\x054V[a\x01s`\x04\x806\x03`@\x81\x10\x15a\x02\xA1W`\0\x80\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x81\x16\x91` \x015\x16a\x05HV[`\x03\x80T`@\x80Q` `\x1F`\x02`\0\x19a\x01\0`\x01\x88\x16\x15\x02\x01\x90\x95\x16\x94\x90\x94\x04\x93\x84\x01\x81\x90\x04\x81\x02\x82\x01\x81\x01\x90\x92R\x82\x81R``\x93\x90\x92\x90\x91\x83\x01\x82\x82\x80\x15a\x03EW\x80`\x1F\x10a\x03\x1AWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x03EV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x03(W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[`\0a\x03ca\x03\\a\x05sV[\x84\x84a\x05wV[P`\x01\x92\x91PPV[`\x02T\x90V[`\0a\x03\x7F\x84\x84\x84a\x06cV[a\x03\xEF\x84a\x03\x8Ba\x05sV[a\x03\xEA\x85`@Q\x80``\x01`@R\x80`(\x81R` \x01a\t'`(\x919`\x01`\x01`\xA0\x1B\x03\x8A\x16`\0\x90\x81R`\x01` R`@\x81 \x90a\x03\xC9a\x05sV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 T\x91\x90a\x07\xBEV[a\x05wV[P`\x01\x93\x92PPPV[`\x05T`\xFF\x16\x90V[`\0a\x03ca\x04\x0Fa\x05sV[\x84a\x03\xEA\x85`\x01`\0a\x04 a\x05sV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R` \x80\x83\x01\x93\x90\x93R`@\x91\x82\x01`\0\x90\x81 \x91\x8C\x16\x81R\x92R\x90 T\x90a\x08UV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x90V[`\x04\x80T`@\x80Q` `\x1F`\x02`\0\x19a\x01\0`\x01\x88\x16\x15\x02\x01\x90\x95\x16\x94\x90\x94\x04\x93\x84\x01\x81\x90\x04\x81\x02\x82\x01\x81\x01\x90\x92R\x82\x81R``\x93\x90\x92\x90\x91\x83\x01\x82\x82\x80\x15a\x03EW\x80`\x1F\x10a\x03\x1AWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x03EV[`\0a\x03ca\x04\xD9a\x05sV[\x84a\x03\xEA\x85`@Q\x80``\x01`@R\x80`%\x81R` \x01a\t\x98`%\x919`\x01`\0a\x05\x03a\x05sV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R` \x80\x83\x01\x93\x90\x93R`@\x91\x82\x01`\0\x90\x81 \x91\x8D\x16\x81R\x92R\x90 T\x91\x90a\x07\xBEV[`\0a\x03ca\x05Aa\x05sV[\x84\x84a\x06cV[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\0\x90\x81R`\x01` \x90\x81R`@\x80\x83 \x93\x90\x94\x16\x82R\x91\x90\x91R T\x90V[3\x90V[`\x01`\x01`\xA0\x1B\x03\x83\x16a\x05\xBCW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`$\x81R` \x01\x80a\tt`$\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x06\x01W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`\"\x81R` \x01\x80a\x08\xDF`\"\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\0\x81\x81R`\x01` \x90\x81R`@\x80\x83 \x94\x87\x16\x80\x84R\x94\x82R\x91\x82\x90 \x85\x90U\x81Q\x85\x81R\x91Q\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x92\x81\x90\x03\x90\x91\x01\x90\xA3PPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16a\x06\xA8W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`%\x81R` \x01\x80a\tO`%\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x06\xEDW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`#\x81R` \x01\x80a\x08\xBC`#\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[a\x06\xF8\x83\x83\x83a\x08\xB6V[a\x075\x81`@Q\x80``\x01`@R\x80`&\x81R` \x01a\t\x01`&\x919`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x91\x90a\x07\xBEV[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x90\x81R` \x81\x90R`@\x80\x82 \x93\x90\x93U\x90\x84\x16\x81R Ta\x07d\x90\x82a\x08UV[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\0\x81\x81R` \x81\x81R`@\x91\x82\x90 \x94\x90\x94U\x80Q\x85\x81R\x90Q\x91\x93\x92\x87\x16\x92\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x92\x91\x82\x90\x03\x01\x90\xA3PPPV[`\0\x81\x84\x84\x11\x15a\x08MW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R\x83\x81\x81Q\x81R` \x01\x91P\x80Q\x90` \x01\x90\x80\x83\x83`\0[\x83\x81\x10\x15a\x08\x12W\x81\x81\x01Q\x83\x82\x01R` \x01a\x07\xFAV[PPPP\x90P\x90\x81\x01\x90`\x1F\x16\x80\x15a\x08?W\x80\x82\x03\x80Q`\x01\x83` \x03a\x01\0\n\x03\x19\x16\x81R` \x01\x91P[P\x92PPP`@Q\x80\x91\x03\x90\xFD[PPP\x90\x03\x90V[`\0\x82\x82\x01\x83\x81\x10\x15a\x08\xAFW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FSafeMath: addition overflow\0\0\0\0\0`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x93\x92PPPV[PPPV\xFEERC20: transfer to the zero addressERC20: approve to the zero addressERC20: transfer amount exceeds balanceERC20: transfer amount exceeds allowanceERC20: transfer from the zero addressERC20: approve from the zero addressERC20: decreased allowance below zero\xA2dipfsX\"\x12 \x88t\t\xCC\xF8\xCC\xF5g\xCC\x82r\xCB:\xEBG\x84\xBC\x1D\xFATu_h\xBD\xC7\xCC\xEB9\xD8x\xB9)dsolcC\0\x07\x06\x003";
    /// The bytecode of the contract.
    pub static ERC20_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xA9W`\x005`\xE0\x1C\x80c9P\x93Q\x11a\0qW\x80c9P\x93Q\x14a\x01\xD9W\x80cp\xA0\x821\x14a\x02\x05W\x80c\x95\xD8\x9BA\x14a\x02+W\x80c\xA4W\xC2\xD7\x14a\x023W\x80c\xA9\x05\x9C\xBB\x14a\x02_W\x80c\xDDb\xED>\x14a\x02\x8BWa\0\xA9V[\x80c\x06\xFD\xDE\x03\x14a\0\xAEW\x80c\t^\xA7\xB3\x14a\x01+W\x80c\x18\x16\r\xDD\x14a\x01kW\x80c#\xB8r\xDD\x14a\x01\x85W\x80c1<\xE5g\x14a\x01\xBBW[`\0\x80\xFD[a\0\xB6a\x02\xB9V[`@\x80Q` \x80\x82R\x83Q\x81\x83\x01R\x83Q\x91\x92\x83\x92\x90\x83\x01\x91\x85\x01\x90\x80\x83\x83`\0[\x83\x81\x10\x15a\0\xF0W\x81\x81\x01Q\x83\x82\x01R` \x01a\0\xD8V[PPPP\x90P\x90\x81\x01\x90`\x1F\x16\x80\x15a\x01\x1DW\x80\x82\x03\x80Q`\x01\x83` \x03a\x01\0\n\x03\x19\x16\x81R` \x01\x91P[P\x92PPP`@Q\x80\x91\x03\x90\xF3[a\x01W`\x04\x806\x03`@\x81\x10\x15a\x01AW`\0\x80\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x16\x90` \x015a\x03OV[`@\x80Q\x91\x15\x15\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[a\x01sa\x03lV[`@\x80Q\x91\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[a\x01W`\x04\x806\x03``\x81\x10\x15a\x01\x9BW`\0\x80\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x81\x16\x91` \x81\x015\x90\x91\x16\x90`@\x015a\x03rV[a\x01\xC3a\x03\xF9V[`@\x80Q`\xFF\x90\x92\x16\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[a\x01W`\x04\x806\x03`@\x81\x10\x15a\x01\xEFW`\0\x80\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x16\x90` \x015a\x04\x02V[a\x01s`\x04\x806\x03` \x81\x10\x15a\x02\x1BW`\0\x80\xFD[P5`\x01`\x01`\xA0\x1B\x03\x16a\x04PV[a\0\xB6a\x04kV[a\x01W`\x04\x806\x03`@\x81\x10\x15a\x02IW`\0\x80\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x16\x90` \x015a\x04\xCCV[a\x01W`\x04\x806\x03`@\x81\x10\x15a\x02uW`\0\x80\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x16\x90` \x015a\x054V[a\x01s`\x04\x806\x03`@\x81\x10\x15a\x02\xA1W`\0\x80\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x81\x16\x91` \x015\x16a\x05HV[`\x03\x80T`@\x80Q` `\x1F`\x02`\0\x19a\x01\0`\x01\x88\x16\x15\x02\x01\x90\x95\x16\x94\x90\x94\x04\x93\x84\x01\x81\x90\x04\x81\x02\x82\x01\x81\x01\x90\x92R\x82\x81R``\x93\x90\x92\x90\x91\x83\x01\x82\x82\x80\x15a\x03EW\x80`\x1F\x10a\x03\x1AWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x03EV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x03(W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[`\0a\x03ca\x03\\a\x05sV[\x84\x84a\x05wV[P`\x01\x92\x91PPV[`\x02T\x90V[`\0a\x03\x7F\x84\x84\x84a\x06cV[a\x03\xEF\x84a\x03\x8Ba\x05sV[a\x03\xEA\x85`@Q\x80``\x01`@R\x80`(\x81R` \x01a\t'`(\x919`\x01`\x01`\xA0\x1B\x03\x8A\x16`\0\x90\x81R`\x01` R`@\x81 \x90a\x03\xC9a\x05sV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 T\x91\x90a\x07\xBEV[a\x05wV[P`\x01\x93\x92PPPV[`\x05T`\xFF\x16\x90V[`\0a\x03ca\x04\x0Fa\x05sV[\x84a\x03\xEA\x85`\x01`\0a\x04 a\x05sV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R` \x80\x83\x01\x93\x90\x93R`@\x91\x82\x01`\0\x90\x81 \x91\x8C\x16\x81R\x92R\x90 T\x90a\x08UV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x90V[`\x04\x80T`@\x80Q` `\x1F`\x02`\0\x19a\x01\0`\x01\x88\x16\x15\x02\x01\x90\x95\x16\x94\x90\x94\x04\x93\x84\x01\x81\x90\x04\x81\x02\x82\x01\x81\x01\x90\x92R\x82\x81R``\x93\x90\x92\x90\x91\x83\x01\x82\x82\x80\x15a\x03EW\x80`\x1F\x10a\x03\x1AWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x03EV[`\0a\x03ca\x04\xD9a\x05sV[\x84a\x03\xEA\x85`@Q\x80``\x01`@R\x80`%\x81R` \x01a\t\x98`%\x919`\x01`\0a\x05\x03a\x05sV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R` \x80\x83\x01\x93\x90\x93R`@\x91\x82\x01`\0\x90\x81 \x91\x8D\x16\x81R\x92R\x90 T\x91\x90a\x07\xBEV[`\0a\x03ca\x05Aa\x05sV[\x84\x84a\x06cV[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\0\x90\x81R`\x01` \x90\x81R`@\x80\x83 \x93\x90\x94\x16\x82R\x91\x90\x91R T\x90V[3\x90V[`\x01`\x01`\xA0\x1B\x03\x83\x16a\x05\xBCW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`$\x81R` \x01\x80a\tt`$\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x06\x01W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`\"\x81R` \x01\x80a\x08\xDF`\"\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\0\x81\x81R`\x01` \x90\x81R`@\x80\x83 \x94\x87\x16\x80\x84R\x94\x82R\x91\x82\x90 \x85\x90U\x81Q\x85\x81R\x91Q\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x92\x81\x90\x03\x90\x91\x01\x90\xA3PPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16a\x06\xA8W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`%\x81R` \x01\x80a\tO`%\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x06\xEDW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`#\x81R` \x01\x80a\x08\xBC`#\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[a\x06\xF8\x83\x83\x83a\x08\xB6V[a\x075\x81`@Q\x80``\x01`@R\x80`&\x81R` \x01a\t\x01`&\x919`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x91\x90a\x07\xBEV[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x90\x81R` \x81\x90R`@\x80\x82 \x93\x90\x93U\x90\x84\x16\x81R Ta\x07d\x90\x82a\x08UV[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\0\x81\x81R` \x81\x81R`@\x91\x82\x90 \x94\x90\x94U\x80Q\x85\x81R\x90Q\x91\x93\x92\x87\x16\x92\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x92\x91\x82\x90\x03\x01\x90\xA3PPPV[`\0\x81\x84\x84\x11\x15a\x08MW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R\x83\x81\x81Q\x81R` \x01\x91P\x80Q\x90` \x01\x90\x80\x83\x83`\0[\x83\x81\x10\x15a\x08\x12W\x81\x81\x01Q\x83\x82\x01R` \x01a\x07\xFAV[PPPP\x90P\x90\x81\x01\x90`\x1F\x16\x80\x15a\x08?W\x80\x82\x03\x80Q`\x01\x83` \x03a\x01\0\n\x03\x19\x16\x81R` \x01\x91P[P\x92PPP`@Q\x80\x91\x03\x90\xFD[PPP\x90\x03\x90V[`\0\x82\x82\x01\x83\x81\x10\x15a\x08\xAFW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FSafeMath: addition overflow\0\0\0\0\0`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x93\x92PPPV[PPPV\xFEERC20: transfer to the zero addressERC20: approve to the zero addressERC20: transfer amount exceeds balanceERC20: transfer amount exceeds allowanceERC20: transfer from the zero addressERC20: approve from the zero addressERC20: decreased allowance below zero\xA2dipfsX\"\x12 \x88t\t\xCC\xF8\xCC\xF5g\xCC\x82r\xCB:\xEBG\x84\xBC\x1D\xFATu_h\xBD\xC7\xCC\xEB9\xD8x\xB9)dsolcC\0\x07\x06\x003";
    /// The deployed bytecode of the contract.
    pub static ERC20_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct ERC20<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ERC20<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ERC20<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ERC20<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ERC20<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(ERC20)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ERC20<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ERC20_ABI.clone(),
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
                ERC20_ABI.clone(),
                ERC20_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `allowance` (0xdd62ed3e) function
        pub fn allowance(
            &self,
            owner: ::ethers::core::types::Address,
            spender: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([221, 98, 237, 62], (owner, spender))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `approve` (0x095ea7b3) function
        pub fn approve(
            &self,
            spender: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([9, 94, 167, 179], (spender, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `balanceOf` (0x70a08231) function
        pub fn balance_of(
            &self,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], account)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `decimals` (0x313ce567) function
        pub fn decimals(&self) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([49, 60, 229, 103], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `decreaseAllowance` (0xa457c2d7) function
        pub fn decrease_allowance(
            &self,
            spender: ::ethers::core::types::Address,
            subtracted_value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([164, 87, 194, 215], (spender, subtracted_value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `increaseAllowance` (0x39509351) function
        pub fn increase_allowance(
            &self,
            spender: ::ethers::core::types::Address,
            added_value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([57, 80, 147, 81], (spender, added_value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `name` (0x06fdde03) function
        pub fn name(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
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
        ///Calls the contract's `totalSupply` (0x18160ddd) function
        pub fn total_supply(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([24, 22, 13, 221], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transfer` (0xa9059cbb) function
        pub fn transfer(
            &self,
            recipient: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([169, 5, 156, 187], (recipient, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferFrom` (0x23b872dd) function
        pub fn transfer_from(
            &self,
            sender: ::ethers::core::types::Address,
            recipient: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([35, 184, 114, 221], (sender, recipient, amount))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Approval` event
        pub fn approval_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ApprovalFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Transfer` event
        pub fn transfer_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TransferFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ERC20Events> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for ERC20<M> {
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
    #[ethevent(name = "Approval", abi = "Approval(address,address,uint256)")]
    pub struct ApprovalFilter {
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub spender: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
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
    #[ethevent(name = "Transfer", abi = "Transfer(address,address,uint256)")]
    pub struct TransferFilter {
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
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
    pub enum ERC20Events {
        ApprovalFilter(ApprovalFilter),
        TransferFilter(TransferFilter),
    }
    impl ::ethers::contract::EthLogDecode for ERC20Events {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(ERC20Events::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(ERC20Events::TransferFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for ERC20Events {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ApprovalFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ApprovalFilter> for ERC20Events {
        fn from(value: ApprovalFilter) -> Self {
            Self::ApprovalFilter(value)
        }
    }
    impl ::core::convert::From<TransferFilter> for ERC20Events {
        fn from(value: TransferFilter) -> Self {
            Self::TransferFilter(value)
        }
    }
    ///Container type for all input parameters for the `allowance` function with signature `allowance(address,address)` and selector `0xdd62ed3e`
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
    #[ethcall(name = "allowance", abi = "allowance(address,address)")]
    pub struct AllowanceCall {
        pub owner: ::ethers::core::types::Address,
        pub spender: ::ethers::core::types::Address,
    }
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
        Hash
    )]
    #[ethcall(name = "approve", abi = "approve(address,uint256)")]
    pub struct ApproveCall {
        pub spender: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
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
        Hash
    )]
    #[ethcall(name = "balanceOf", abi = "balanceOf(address)")]
    pub struct BalanceOfCall {
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `decimals` function with signature `decimals()` and selector `0x313ce567`
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
    #[ethcall(name = "decimals", abi = "decimals()")]
    pub struct DecimalsCall;
    ///Container type for all input parameters for the `decreaseAllowance` function with signature `decreaseAllowance(address,uint256)` and selector `0xa457c2d7`
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
    #[ethcall(name = "decreaseAllowance", abi = "decreaseAllowance(address,uint256)")]
    pub struct DecreaseAllowanceCall {
        pub spender: ::ethers::core::types::Address,
        pub subtracted_value: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `increaseAllowance` function with signature `increaseAllowance(address,uint256)` and selector `0x39509351`
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
    #[ethcall(name = "increaseAllowance", abi = "increaseAllowance(address,uint256)")]
    pub struct IncreaseAllowanceCall {
        pub spender: ::ethers::core::types::Address,
        pub added_value: ::ethers::core::types::U256,
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
        Hash
    )]
    #[ethcall(name = "name", abi = "name()")]
    pub struct NameCall;
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
        Hash
    )]
    #[ethcall(name = "symbol", abi = "symbol()")]
    pub struct SymbolCall;
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
        Hash
    )]
    #[ethcall(name = "totalSupply", abi = "totalSupply()")]
    pub struct TotalSupplyCall;
    ///Container type for all input parameters for the `transfer` function with signature `transfer(address,uint256)` and selector `0xa9059cbb`
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
    #[ethcall(name = "transfer", abi = "transfer(address,uint256)")]
    pub struct TransferCall {
        pub recipient: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
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
        Hash
    )]
    #[ethcall(name = "transferFrom", abi = "transferFrom(address,address,uint256)")]
    pub struct TransferFromCall {
        pub sender: ::ethers::core::types::Address,
        pub recipient: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
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
    pub enum ERC20Calls {
        Allowance(AllowanceCall),
        Approve(ApproveCall),
        BalanceOf(BalanceOfCall),
        Decimals(DecimalsCall),
        DecreaseAllowance(DecreaseAllowanceCall),
        IncreaseAllowance(IncreaseAllowanceCall),
        Name(NameCall),
        Symbol(SymbolCall),
        TotalSupply(TotalSupplyCall),
        Transfer(TransferCall),
        TransferFrom(TransferFromCall),
    }
    impl ::ethers::core::abi::AbiDecode for ERC20Calls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AllowanceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Allowance(decoded));
            }
            if let Ok(decoded) = <ApproveCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Approve(decoded));
            }
            if let Ok(decoded) = <BalanceOfCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BalanceOf(decoded));
            }
            if let Ok(decoded) = <DecimalsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Decimals(decoded));
            }
            if let Ok(decoded) = <DecreaseAllowanceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DecreaseAllowance(decoded));
            }
            if let Ok(decoded) = <IncreaseAllowanceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IncreaseAllowance(decoded));
            }
            if let Ok(decoded) = <NameCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Name(decoded));
            }
            if let Ok(decoded) = <SymbolCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Symbol(decoded));
            }
            if let Ok(decoded) = <TotalSupplyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TotalSupply(decoded));
            }
            if let Ok(decoded) = <TransferCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Transfer(decoded));
            }
            if let Ok(decoded) = <TransferFromCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferFrom(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ERC20Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Allowance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Approve(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BalanceOf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Decimals(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DecreaseAllowance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IncreaseAllowance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Name(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Symbol(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TotalSupply(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Transfer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferFrom(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for ERC20Calls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Allowance(element) => ::core::fmt::Display::fmt(element, f),
                Self::Approve(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::Decimals(element) => ::core::fmt::Display::fmt(element, f),
                Self::DecreaseAllowance(element) => ::core::fmt::Display::fmt(element, f),
                Self::IncreaseAllowance(element) => ::core::fmt::Display::fmt(element, f),
                Self::Name(element) => ::core::fmt::Display::fmt(element, f),
                Self::Symbol(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalSupply(element) => ::core::fmt::Display::fmt(element, f),
                Self::Transfer(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFrom(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AllowanceCall> for ERC20Calls {
        fn from(value: AllowanceCall) -> Self {
            Self::Allowance(value)
        }
    }
    impl ::core::convert::From<ApproveCall> for ERC20Calls {
        fn from(value: ApproveCall) -> Self {
            Self::Approve(value)
        }
    }
    impl ::core::convert::From<BalanceOfCall> for ERC20Calls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<DecimalsCall> for ERC20Calls {
        fn from(value: DecimalsCall) -> Self {
            Self::Decimals(value)
        }
    }
    impl ::core::convert::From<DecreaseAllowanceCall> for ERC20Calls {
        fn from(value: DecreaseAllowanceCall) -> Self {
            Self::DecreaseAllowance(value)
        }
    }
    impl ::core::convert::From<IncreaseAllowanceCall> for ERC20Calls {
        fn from(value: IncreaseAllowanceCall) -> Self {
            Self::IncreaseAllowance(value)
        }
    }
    impl ::core::convert::From<NameCall> for ERC20Calls {
        fn from(value: NameCall) -> Self {
            Self::Name(value)
        }
    }
    impl ::core::convert::From<SymbolCall> for ERC20Calls {
        fn from(value: SymbolCall) -> Self {
            Self::Symbol(value)
        }
    }
    impl ::core::convert::From<TotalSupplyCall> for ERC20Calls {
        fn from(value: TotalSupplyCall) -> Self {
            Self::TotalSupply(value)
        }
    }
    impl ::core::convert::From<TransferCall> for ERC20Calls {
        fn from(value: TransferCall) -> Self {
            Self::Transfer(value)
        }
    }
    impl ::core::convert::From<TransferFromCall> for ERC20Calls {
        fn from(value: TransferFromCall) -> Self {
            Self::TransferFrom(value)
        }
    }
    ///Container type for all return fields from the `allowance` function with signature `allowance(address,address)` and selector `0xdd62ed3e`
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
    pub struct AllowanceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `approve` function with signature `approve(address,uint256)` and selector `0x095ea7b3`
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
    pub struct ApproveReturn(pub bool);
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
        Hash
    )]
    pub struct BalanceOfReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `decimals` function with signature `decimals()` and selector `0x313ce567`
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
    pub struct DecimalsReturn(pub u8);
    ///Container type for all return fields from the `decreaseAllowance` function with signature `decreaseAllowance(address,uint256)` and selector `0xa457c2d7`
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
    pub struct DecreaseAllowanceReturn(pub bool);
    ///Container type for all return fields from the `increaseAllowance` function with signature `increaseAllowance(address,uint256)` and selector `0x39509351`
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
    pub struct IncreaseAllowanceReturn(pub bool);
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
        Hash
    )]
    pub struct NameReturn(pub ::std::string::String);
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
        Hash
    )]
    pub struct SymbolReturn(pub ::std::string::String);
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
        Hash
    )]
    pub struct TotalSupplyReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `transfer` function with signature `transfer(address,uint256)` and selector `0xa9059cbb`
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
    pub struct TransferReturn(pub bool);
    ///Container type for all return fields from the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `0x23b872dd`
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
    pub struct TransferFromReturn(pub bool);
}
