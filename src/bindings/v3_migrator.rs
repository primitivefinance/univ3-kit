pub use v3_migrator::*;
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
pub mod v3_migrator {
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
                        name: ::std::borrow::ToOwned::to_owned(
                            "_nonfungiblePositionManager",
                        ),
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
                    ::std::borrow::ToOwned::to_owned(
                        "createAndInitializePoolIfNecessary",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
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
                                    name: ::std::borrow::ToOwned::to_owned("pool"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
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
                    ::std::borrow::ToOwned::to_owned("migrate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("migrate"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(24usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(24usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IV3Migrator.MigrateParams",
                                        ),
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
                    ::std::borrow::ToOwned::to_owned("multicall"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("multicall"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("results"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("nonfungiblePositionManager"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "nonfungiblePositionManager",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("selfPermit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("deadline"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("s"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("selfPermitAllowed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("expiry"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("s"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("selfPermitAllowedIfNecessary"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "selfPermitAllowedIfNecessary",
                            ),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("expiry"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("s"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("selfPermitIfNecessary"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "selfPermitIfNecessary",
                            ),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("deadline"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("s"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: true,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static V3MIGRATOR_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xE0`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0\x1A68\x03\x80b\0\x1A6\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\0yV[`\x01`\x01``\x1B\x03\x19``\x93\x84\x1B\x81\x16`\x80R\x91\x83\x1B\x82\x16`\xA0R\x90\x91\x1B\x16`\xC0Rb\0\0\xC2V[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\0tW`\0\x80\xFD[\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15b\0\0\x8EW\x82\x83\xFD[b\0\0\x99\x84b\0\0\\V[\x92Pb\0\0\xA9` \x85\x01b\0\0\\V[\x91Pb\0\0\xB9`@\x85\x01b\0\0\\V[\x90P\x92P\x92P\x92V[`\x80Q``\x1C`\xA0Q``\x1C`\xC0Q``\x1Ca\x19\x08b\0\x01.`\09\x80a\x07sR\x80a\n>R\x80a\nxR\x80a\n\xA2R\x80a\x0CKRP\x80`\xA5R\x80a\x05vR\x80a\x0C\x97R\x80a\x0C\xEER\x80a\r\xC9R\x80a\x0E RP\x80a\x02\x08R\x80a\x02\xCFR\x80a\x08&RPa\x19\x08`\0\xF3\xFE`\x80`@R`\x046\x10a\0\x95W`\x005`\xE0\x1C\x80c\xB4J'\"\x11a\0YW\x80c\xB4J'\"\x14a\x01vW\x80c\xC2\xE3\x14\n\x14a\x01\x8BW\x80c\xC4Z\x01U\x14a\x01\x9EW\x80c\xD4O+\xF2\x14a\x01\xB3W\x80c\xF3\x99\\g\x14a\x01\xD3Wa\0\xEDV[\x80c\x13\xEA\xD5b\x14a\0\xF2W\x80cFY\xA4\x94\x14a\x01\x1BW\x80cJ\xA4\xA4\xFC\x14a\x01.W\x80c\xA4\xA7\x8F\x0C\x14a\x01CW\x80c\xAC\x96P\xD8\x14a\x01VWa\0\xEDV[6a\0\xEDW3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\0\xEBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\0\xE2\x90a\x17(V[`@Q\x80\x91\x03\x90\xFD[\0[`\0\x80\xFD[a\x01\x05a\x01\x006`\x04a\x13%V[a\x01\xE6V[`@Qa\x01\x12\x91\x90a\x16OV[`@Q\x80\x91\x03\x90\xF3[a\0\xEBa\x01)6`\x04a\x13~V[a\x04\xDAV[4\x80\x15a\x01:W`\0\x80\xFD[Pa\x01\x05a\x05tV[a\0\xEBa\x01Q6`\x04a\x13~V[a\x05\x98V[a\x01ia\x01d6`\x04a\x13\xD7V[a\x061V[`@Qa\x01\x12\x91\x90a\x16\x87V[4\x80\x15a\x01\x82W`\0\x80\xFD[Pa\x01\x05a\x07qV[a\0\xEBa\x01\x996`\x04a\x13~V[a\x07\x95V[4\x80\x15a\x01\xAAW`\0\x80\xFD[Pa\x01\x05a\x08$V[4\x80\x15a\x01\xBFW`\0\x80\xFD[Pa\0\xEBa\x01\xCE6`\x04a\x156V[a\x08HV[a\0\xEBa\x01\xE16`\x04a\x13~V[a\x0E\xB4V[`\0\x83`\x01`\x01`\xA0\x1B\x03\x16\x85`\x01`\x01`\xA0\x1B\x03\x16\x10a\x02\x06W`\0\x80\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x16\x98\xEE\x82\x86\x86\x86`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x84`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x82b\xFF\xFF\xFF\x16\x81R` \x01\x93PPPP` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x02\x91W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x02\xA5W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x02\xBBW`\0\x80\xFD[PQ\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a\x03\xF1W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xA1g\x12\x95\x86\x86\x86`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x84`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x82b\xFF\xFF\xFF\x16\x81R` \x01\x93PPPP` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x03ZW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x03nW=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x03\x84W`\0\x80\xFD[PQ`@\x80Qc\xF67s\x1D`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R\x91Q\x92\x93P\x90\x83\x16\x91c\xF67s\x1D\x91`$\x80\x82\x01\x92`\0\x92\x90\x91\x90\x82\x90\x03\x01\x81\x83\x87\x80;\x15\x80\x15a\x03\xD4W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x03\xE8W=`\0\x80>=`\0\xFD[PPPPa\x04\xD2V[`\0\x81`\x01`\x01`\xA0\x1B\x03\x16c8P\xC7\xBD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\xE0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x04,W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x04@W=`\0\x80>=`\0\xFD[PPPP`@Q=`\xE0\x81\x10\x15a\x04VW`\0\x80\xFD[PQ\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a\x04\xD0W\x81`\x01`\x01`\xA0\x1B\x03\x16c\xF67s\x1D\x84`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x82`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x91PP`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04\xB7W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x04\xCBW=`\0\x80>=`\0\xFD[PPPP[P[\x94\x93PPPPV[`@\x80Qc#\xF2\xEB\xC3`\xE2\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x87\x90R`d\x81\x01\x86\x90R`\x01`\x84\x82\x01R`\xFF\x85\x16`\xA4\x82\x01R`\xC4\x81\x01\x84\x90R`\xE4\x81\x01\x83\x90R\x90Q`\x01`\x01`\xA0\x1B\x03\x88\x16\x91c\x8F\xCB\xAF\x0C\x91a\x01\x04\x80\x83\x01\x92`\0\x92\x91\x90\x82\x90\x03\x01\x81\x83\x87\x80;\x15\x80\x15a\x05TW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x05hW=`\0\x80>=`\0\xFD[PPPPPPPPPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@\x80Qcn\xB1v\x9F`\xE1\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R\x90Q`\0\x19\x91`\x01`\x01`\xA0\x1B\x03\x89\x16\x91c\xDDb\xED>\x91`D\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x05\xE9W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x05\xFDW=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x06\x13W`\0\x80\xFD[PQ\x10\x15a\x06)Wa\x06)\x86\x86\x86\x86\x86\x86a\x04\xDAV[PPPPPPV[``\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x80\x15a\x06JW`\0\x80\xFD[P`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x06~W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x06iW\x90P[P\x90P`\0[\x82\x81\x10\x15a\x07jW`\0\x800\x86\x86\x85\x81\x81\x10a\x06\x9CW\xFE[\x90P` \x02\x81\x01\x90a\x06\xAE\x91\x90a\x180V[`@Qa\x06\xBC\x92\x91\x90a\x16?V[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a\x06\xF7W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x06\xFCV[``\x91P[P\x91P\x91P\x81a\x07HW`D\x81Q\x10\x15a\x07\x15W`\0\x80\xFD[`\x04\x81\x01\x90P\x80\x80` \x01\x90Q\x81\x01\x90a\x07/\x91\x90a\x14\x9FV[`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\0\xE2\x91\x90a\x16\xE7V[\x80\x84\x84\x81Q\x81\x10a\x07UW\xFE[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a\x06\x84V[P\x92\x91PPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@\x80Qcn\xB1v\x9F`\xE1\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R\x90Q\x86\x91`\x01`\x01`\xA0\x1B\x03\x89\x16\x91c\xDDb\xED>\x91`D\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x07\xE4W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x07\xF8W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x08\x0EW`\0\x80\xFD[PQ\x10\x15a\x06)Wa\x06)\x86\x86\x86\x86\x86\x86a\x0E\xB4V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0a\x08Z``\x83\x01`@\x84\x01a\x15\xDCV[`\xFF\x16\x11a\x08zW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\0\xE2\x90a\x16\xFAV[`da\x08\x8C``\x83\x01`@\x84\x01a\x15\xDCV[`\xFF\x16\x11\x15a\x08\xADW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\0\xE2\x90a\x17KV[a\x08\xBA` \x82\x01\x82a\x13\x02V[`\x01`\x01`\xA0\x1B\x03\x16c#\xB8r\xDD3a\x08\xD6` \x85\x01\x85a\x13\x02V[\x84` \x015`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x08\xF9\x93\x92\x91\x90a\x16cV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\t\x13W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\t'W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\tK\x91\x90a\x14bV[P`\0\x80a\t\\` \x84\x01\x84a\x13\x02V[`\x01`\x01`\xA0\x1B\x03\x16c\x89\xAF\xCBD0`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\t\x87\x91\x90a\x16OV[`@\x80Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\t\xA0W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\t\xB4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xD8\x91\x90a\x15\xB9V[\x90\x92P\x90P`\0`da\t\xFEa\t\xF4``\x87\x01`@\x88\x01a\x15\xDCV[\x85\x90`\xFF\x16a\x0F&V[\x81a\n\x05W\xFE[\x04\x90P`\0`da\n\x1Fa\t\xF4``\x88\x01`@\x89\x01a\x15\xDCV[\x81a\n&W\xFE[\x04\x90Pa\nca\n<`\x80\x87\x01``\x88\x01a\x13\x02V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84a\x0FPV[a\n\x9Da\nv`\xA0\x87\x01`\x80\x88\x01a\x13\x02V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83a\x0FPV[`\0\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x881dV`@Q\x80a\x01`\x01`@R\x80\x8A``\x01` \x81\x01\x90a\n\xEE\x91\x90a\x13\x02V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01a\x0B\x0C`\xA0\x8C\x01`\x80\x8D\x01a\x13\x02V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01a\x0B*`\xC0\x8C\x01`\xA0\x8D\x01a\x15NV[b\xFF\xFF\xFF\x16\x81R` \x01a\x0BD`\xE0\x8C\x01`\xC0\x8D\x01a\x14~V[`\x02\x0B\x81R` \x01a\x0B]a\x01\0\x8C\x01`\xE0\x8D\x01a\x14~V[`\x02\x0B\x81R` \x81\x01\x88\x90R`@\x81\x01\x87\x90Ra\x01\0\x8B\x015``\x82\x01Ra\x01 \x8B\x015`\x80\x82\x01R`\xA0\x01a\x0B\x9Ba\x01`\x8C\x01a\x01@\x8D\x01a\x13\x02V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8Aa\x01`\x015\x81RP`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0B\xCE\x91\x90a\x17yV[`\x80`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0B\xE8W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0B\xFCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C \x91\x90a\x15hV[\x93P\x93PPP\x85\x82\x10\x15a\r\x80W\x83\x82\x10\x15a\x0CqWa\x0Cqa\x0CI`\x80\x89\x01``\x8A\x01a\x13\x02V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0a\x0FPV[\x81\x86\x03a\x0C\x86a\x01\xA0\x89\x01a\x01\x80\x8A\x01a\x14FV[\x80\x15a\x0C\xD2WP`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16a\x0C\xC7`\x80\x8A\x01``\x8B\x01a\x13\x02V[`\x01`\x01`\xA0\x1B\x03\x16\x14[\x15a\rdW`@Qc.\x1A}M`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c.\x1A}M\x90a\r#\x90\x84\x90`\x04\x01a\x18'V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\r=W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\rQW=`\0\x80>=`\0\xFD[PPPPa\r_3\x82a\x10\x9EV[a\r~V[a\r~a\rw`\x80\x8A\x01``\x8B\x01a\x13\x02V[3\x83a\x11\x92V[P[\x84\x81\x10\x15a\x0E\xABW\x82\x81\x10\x15a\r\xA3Wa\r\xA3a\x0CI`\xA0\x89\x01`\x80\x8A\x01a\x13\x02V[\x80\x85\x03a\r\xB8a\x01\xA0\x89\x01a\x01\x80\x8A\x01a\x14FV[\x80\x15a\x0E\x04WP`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16a\r\xF9`\xA0\x8A\x01`\x80\x8B\x01a\x13\x02V[`\x01`\x01`\xA0\x1B\x03\x16\x14[\x15a\x0E\x96W`@Qc.\x1A}M`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c.\x1A}M\x90a\x0EU\x90\x84\x90`\x04\x01a\x18'V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0EoW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0E\x83W=`\0\x80>=`\0\xFD[PPPPa\x0E\x913\x82a\x10\x9EV[a\x0E\xA9V[a\x0E\xA9a\rw`\xA0\x8A\x01`\x80\x8B\x01a\x13\x02V[P[PPPPPPPV[`@\x80Qc\xD5\x05\xAC\xCF`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x87\x90R`d\x81\x01\x86\x90R`\xFF\x85\x16`\x84\x82\x01R`\xA4\x81\x01\x84\x90R`\xC4\x81\x01\x83\x90R\x90Q`\x01`\x01`\xA0\x1B\x03\x88\x16\x91c\xD5\x05\xAC\xCF\x91`\xE4\x80\x83\x01\x92`\0\x92\x91\x90\x82\x90\x03\x01\x81\x83\x87\x80;\x15\x80\x15a\x05TW`\0\x80\xFD[`\0\x82\x15\x80a\x0FAWPP\x81\x81\x02\x81\x83\x82\x81a\x0F>W\xFE[\x04\x14[a\x0FJW`\0\x80\xFD[\x92\x91PPV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`$\x83\x01R`D\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`d\x90\x92\x01\x83R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\t^\xA7\xB3`\xE0\x1B\x17\x81R\x92Q\x82Q`\0\x94\x85\x94\x93\x89\x16\x93\x92\x91\x82\x91\x90\x80\x83\x83[` \x83\x10a\x0F\xCCW\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01a\x0F\xADV[`\x01\x83` \x03a\x01\0\n\x03\x80\x19\x82Q\x16\x81\x84Q\x16\x80\x82\x17\x85RPPPPPP\x90P\x01\x91PP`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x10.W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x103V[``\x91P[P\x91P\x91P\x81\x80\x15a\x10aWP\x80Q\x15\x80a\x10aWP\x80\x80` \x01\x90Q` \x81\x10\x15a\x10^W`\0\x80\xFD[PQ[a\x10\x97W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x02`$\x82\x01RaSA`\xF0\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[PPPPPV[`@\x80Q`\0\x80\x82R` \x82\x01\x90\x92R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x83\x90`@Q\x80\x82\x80Q\x90` \x01\x90\x80\x83\x83[` \x83\x10a\x10\xEAW\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01a\x10\xCBV[`\x01\x83` \x03a\x01\0\n\x03\x80\x19\x82Q\x16\x81\x84Q\x16\x80\x82\x17\x85RPPPPPP\x90P\x01\x91PP`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x11LW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x11QV[``\x91P[PP\x90P\x80a\x11\x8DW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x03`$\x82\x01RbSTE`\xE8\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[PPPV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`$\x83\x01R`D\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`d\x90\x92\x01\x83R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\xA9\x05\x9C\xBB`\xE0\x1B\x17\x81R\x92Q\x82Q`\0\x94\x85\x94\x93\x89\x16\x93\x92\x91\x82\x91\x90\x80\x83\x83[` \x83\x10a\x12\x0EW\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01a\x11\xEFV[`\x01\x83` \x03a\x01\0\n\x03\x80\x19\x82Q\x16\x81\x84Q\x16\x80\x82\x17\x85RPPPPPP\x90P\x01\x91PP`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x12pW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x12uV[``\x91P[P\x91P\x91P\x81\x80\x15a\x12\xA3WP\x80Q\x15\x80a\x12\xA3WP\x80\x80` \x01\x90Q` \x81\x10\x15a\x12\xA0W`\0\x80\xFD[PQ[a\x10\x97W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x02`$\x82\x01Ra\x14\xD5`\xF2\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x805b\xFF\xFF\xFF\x81\x16\x81\x14a\x12\xECW`\0\x80\xFD[\x91\x90PV[\x805`\xFF\x81\x16\x81\x14a\x12\xECW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x13\x13W\x80\x81\xFD[\x815a\x13\x1E\x81a\x18\xACV[\x93\x92PPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x13:W\x82\x83\xFD[\x845a\x13E\x81a\x18\xACV[\x93P` \x85\x015a\x13U\x81a\x18\xACV[\x92Pa\x13c`@\x86\x01a\x12\xD9V[\x91P``\x85\x015a\x13s\x81a\x18\xACV[\x93\x96\x92\x95P\x90\x93PPV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\x13\x96W\x81\x82\xFD[\x865a\x13\xA1\x81a\x18\xACV[\x95P` \x87\x015\x94P`@\x87\x015\x93Pa\x13\xBD``\x88\x01a\x12\xF1V[\x92P`\x80\x87\x015\x91P`\xA0\x87\x015\x90P\x92\x95P\x92\x95P\x92\x95V[`\0\x80` \x83\x85\x03\x12\x15a\x13\xE9W\x81\x82\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x14\0W\x83\x84\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x14\x13W\x83\x84\xFD[\x815\x81\x81\x11\x15a\x14!W\x84\x85\xFD[\x86` \x80\x83\x02\x85\x01\x01\x11\x15a\x144W\x84\x85\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[`\0` \x82\x84\x03\x12\x15a\x14WW\x80\x81\xFD[\x815a\x13\x1E\x81a\x18\xC4V[`\0` \x82\x84\x03\x12\x15a\x14sW\x80\x81\xFD[\x81Qa\x13\x1E\x81a\x18\xC4V[`\0` \x82\x84\x03\x12\x15a\x14\x8FW\x80\x81\xFD[\x815\x80`\x02\x0B\x81\x14a\x13\x1EW\x81\x82\xFD[`\0` \x82\x84\x03\x12\x15a\x14\xB0W\x80\x81\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x14\xC7W\x82\x83\xFD[\x81\x84\x01\x91P\x84`\x1F\x83\x01\x12a\x14\xDAW\x82\x83\xFD[\x81Q\x81\x81\x11\x15a\x14\xE6W\xFE[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01` \x01\x83\x81\x11\x82\x82\x10\x17\x15a\x15\x04W\xFE[`@R\x81\x81R\x83\x82\x01` \x01\x87\x10\x15a\x15\x1BW\x84\x85\xFD[a\x15,\x82` \x83\x01` \x87\x01a\x18|V[\x96\x95PPPPPPV[`\0a\x01\xA0\x82\x84\x03\x12\x15a\x15HW\x80\x81\xFD[P\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x15_W\x80\x81\xFD[a\x13\x1E\x82a\x12\xD9V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x15}W\x83\x84\xFD[\x84Q\x93P` \x85\x01Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x15\xA3W\x83\x84\xFD[`@\x86\x01Q``\x90\x96\x01Q\x94\x97\x90\x96P\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x15\xCBW\x81\x82\xFD[PP\x80Q` \x90\x91\x01Q\x90\x92\x90\x91PV[`\0` \x82\x84\x03\x12\x15a\x15\xEDW\x80\x81\xFD[a\x13\x1E\x82a\x12\xF1V[`\x01`\x01`\xA0\x1B\x03\x16\x90RV[`\0\x81Q\x80\x84Ra\x16\x1B\x81` \x86\x01` \x86\x01a\x18|V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\x02\x0B\x90RV[b\xFF\xFF\xFF\x16\x90RV[`\0\x82\x84\x837\x91\x01\x90\x81R\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x81R` \x01\x90V[`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81R\x91\x90\x92\x16` \x82\x01R`@\x81\x01\x91\x90\x91R``\x01\x90V[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x86\x01\x91P`@\x84\x82\x02\x87\x01\x01\x92P\x83\x87\x01\x85[\x82\x81\x10\x15a\x16\xDAW`?\x19\x88\x86\x03\x01\x84Ra\x16\xC8\x85\x83Qa\x16\x03V[\x94P\x92\x85\x01\x92\x90\x85\x01\x90`\x01\x01a\x16\xACV[P\x92\x97\x96PPPPPPPV[`\0` \x82Ra\x13\x1E` \x83\x01\x84a\x16\x03V[` \x80\x82R`\x14\x90\x82\x01Rs\x14\x19\\\x98\xD9[\x9D\x18Y\xD9H\x1D\x1B\xDB\xC8\x1C\xDBX[\x1B`b\x1B`@\x82\x01R``\x01\x90V[` \x80\x82R`\t\x90\x82\x01RhNot WETH9`\xB8\x1B`@\x82\x01R``\x01\x90V[` \x80\x82R`\x14\x90\x82\x01RsPercentage too large``\x1B`@\x82\x01R``\x01\x90V[`\0a\x01`\x82\x01\x90Pa\x17\x8D\x82\x84Qa\x15\xF6V[` \x83\x01Qa\x17\x9F` \x84\x01\x82a\x15\xF6V[P`@\x83\x01Qa\x17\xB2`@\x84\x01\x82a\x166V[P``\x83\x01Qa\x17\xC5``\x84\x01\x82a\x16/V[P`\x80\x83\x01Qa\x17\xD8`\x80\x84\x01\x82a\x16/V[P`\xA0\x83\x01Q`\xA0\x83\x01R`\xC0\x83\x01Q`\xC0\x83\x01R`\xE0\x83\x01Q`\xE0\x83\x01Ra\x01\0\x80\x84\x01Q\x81\x84\x01RPa\x01 \x80\x84\x01Qa\x18\x16\x82\x85\x01\x82a\x15\xF6V[PPa\x01@\x92\x83\x01Q\x91\x90\x92\x01R\x90V[\x90\x81R` \x01\x90V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x18FW\x82\x83\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x18`W\x82\x83\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x18uW`\0\x80\xFD[\x92P\x92\x90PV[`\0[\x83\x81\x10\x15a\x18\x97W\x81\x81\x01Q\x83\x82\x01R` \x01a\x18\x7FV[\x83\x81\x11\x15a\x18\xA6W`\0\x84\x84\x01R[PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x18\xC1W`\0\x80\xFD[PV[\x80\x15\x15\x81\x14a\x18\xC1W`\0\x80\xFD\xFE\xA2dipfsX\"\x12 i\xF5l\x9F\xDAdM;j\x0B\xAF\x86rO\xFB\xC6\xC5.t\xBC\x7F]@\xFE\x8C\x8E.J\x1A\x8F3\xBEdsolcC\0\x07\x06\x003";
    /// The bytecode of the contract.
    pub static V3MIGRATOR_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0\x95W`\x005`\xE0\x1C\x80c\xB4J'\"\x11a\0YW\x80c\xB4J'\"\x14a\x01vW\x80c\xC2\xE3\x14\n\x14a\x01\x8BW\x80c\xC4Z\x01U\x14a\x01\x9EW\x80c\xD4O+\xF2\x14a\x01\xB3W\x80c\xF3\x99\\g\x14a\x01\xD3Wa\0\xEDV[\x80c\x13\xEA\xD5b\x14a\0\xF2W\x80cFY\xA4\x94\x14a\x01\x1BW\x80cJ\xA4\xA4\xFC\x14a\x01.W\x80c\xA4\xA7\x8F\x0C\x14a\x01CW\x80c\xAC\x96P\xD8\x14a\x01VWa\0\xEDV[6a\0\xEDW3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\0\xEBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\0\xE2\x90a\x17(V[`@Q\x80\x91\x03\x90\xFD[\0[`\0\x80\xFD[a\x01\x05a\x01\x006`\x04a\x13%V[a\x01\xE6V[`@Qa\x01\x12\x91\x90a\x16OV[`@Q\x80\x91\x03\x90\xF3[a\0\xEBa\x01)6`\x04a\x13~V[a\x04\xDAV[4\x80\x15a\x01:W`\0\x80\xFD[Pa\x01\x05a\x05tV[a\0\xEBa\x01Q6`\x04a\x13~V[a\x05\x98V[a\x01ia\x01d6`\x04a\x13\xD7V[a\x061V[`@Qa\x01\x12\x91\x90a\x16\x87V[4\x80\x15a\x01\x82W`\0\x80\xFD[Pa\x01\x05a\x07qV[a\0\xEBa\x01\x996`\x04a\x13~V[a\x07\x95V[4\x80\x15a\x01\xAAW`\0\x80\xFD[Pa\x01\x05a\x08$V[4\x80\x15a\x01\xBFW`\0\x80\xFD[Pa\0\xEBa\x01\xCE6`\x04a\x156V[a\x08HV[a\0\xEBa\x01\xE16`\x04a\x13~V[a\x0E\xB4V[`\0\x83`\x01`\x01`\xA0\x1B\x03\x16\x85`\x01`\x01`\xA0\x1B\x03\x16\x10a\x02\x06W`\0\x80\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x16\x98\xEE\x82\x86\x86\x86`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x84`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x82b\xFF\xFF\xFF\x16\x81R` \x01\x93PPPP` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x02\x91W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x02\xA5W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x02\xBBW`\0\x80\xFD[PQ\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a\x03\xF1W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xA1g\x12\x95\x86\x86\x86`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x84`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x82b\xFF\xFF\xFF\x16\x81R` \x01\x93PPPP` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x03ZW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x03nW=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x03\x84W`\0\x80\xFD[PQ`@\x80Qc\xF67s\x1D`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R\x91Q\x92\x93P\x90\x83\x16\x91c\xF67s\x1D\x91`$\x80\x82\x01\x92`\0\x92\x90\x91\x90\x82\x90\x03\x01\x81\x83\x87\x80;\x15\x80\x15a\x03\xD4W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x03\xE8W=`\0\x80>=`\0\xFD[PPPPa\x04\xD2V[`\0\x81`\x01`\x01`\xA0\x1B\x03\x16c8P\xC7\xBD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\xE0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x04,W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x04@W=`\0\x80>=`\0\xFD[PPPP`@Q=`\xE0\x81\x10\x15a\x04VW`\0\x80\xFD[PQ\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a\x04\xD0W\x81`\x01`\x01`\xA0\x1B\x03\x16c\xF67s\x1D\x84`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01\x80\x82`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x91PP`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04\xB7W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x04\xCBW=`\0\x80>=`\0\xFD[PPPP[P[\x94\x93PPPPV[`@\x80Qc#\xF2\xEB\xC3`\xE2\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x87\x90R`d\x81\x01\x86\x90R`\x01`\x84\x82\x01R`\xFF\x85\x16`\xA4\x82\x01R`\xC4\x81\x01\x84\x90R`\xE4\x81\x01\x83\x90R\x90Q`\x01`\x01`\xA0\x1B\x03\x88\x16\x91c\x8F\xCB\xAF\x0C\x91a\x01\x04\x80\x83\x01\x92`\0\x92\x91\x90\x82\x90\x03\x01\x81\x83\x87\x80;\x15\x80\x15a\x05TW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x05hW=`\0\x80>=`\0\xFD[PPPPPPPPPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@\x80Qcn\xB1v\x9F`\xE1\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R\x90Q`\0\x19\x91`\x01`\x01`\xA0\x1B\x03\x89\x16\x91c\xDDb\xED>\x91`D\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x05\xE9W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x05\xFDW=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x06\x13W`\0\x80\xFD[PQ\x10\x15a\x06)Wa\x06)\x86\x86\x86\x86\x86\x86a\x04\xDAV[PPPPPPV[``\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x80\x15a\x06JW`\0\x80\xFD[P`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x06~W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x06iW\x90P[P\x90P`\0[\x82\x81\x10\x15a\x07jW`\0\x800\x86\x86\x85\x81\x81\x10a\x06\x9CW\xFE[\x90P` \x02\x81\x01\x90a\x06\xAE\x91\x90a\x180V[`@Qa\x06\xBC\x92\x91\x90a\x16?V[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a\x06\xF7W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x06\xFCV[``\x91P[P\x91P\x91P\x81a\x07HW`D\x81Q\x10\x15a\x07\x15W`\0\x80\xFD[`\x04\x81\x01\x90P\x80\x80` \x01\x90Q\x81\x01\x90a\x07/\x91\x90a\x14\x9FV[`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\0\xE2\x91\x90a\x16\xE7V[\x80\x84\x84\x81Q\x81\x10a\x07UW\xFE[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a\x06\x84V[P\x92\x91PPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@\x80Qcn\xB1v\x9F`\xE1\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R\x90Q\x86\x91`\x01`\x01`\xA0\x1B\x03\x89\x16\x91c\xDDb\xED>\x91`D\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x07\xE4W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x07\xF8W=`\0\x80>=`\0\xFD[PPPP`@Q=` \x81\x10\x15a\x08\x0EW`\0\x80\xFD[PQ\x10\x15a\x06)Wa\x06)\x86\x86\x86\x86\x86\x86a\x0E\xB4V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0a\x08Z``\x83\x01`@\x84\x01a\x15\xDCV[`\xFF\x16\x11a\x08zW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\0\xE2\x90a\x16\xFAV[`da\x08\x8C``\x83\x01`@\x84\x01a\x15\xDCV[`\xFF\x16\x11\x15a\x08\xADW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\0\xE2\x90a\x17KV[a\x08\xBA` \x82\x01\x82a\x13\x02V[`\x01`\x01`\xA0\x1B\x03\x16c#\xB8r\xDD3a\x08\xD6` \x85\x01\x85a\x13\x02V[\x84` \x015`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x08\xF9\x93\x92\x91\x90a\x16cV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\t\x13W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\t'W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\tK\x91\x90a\x14bV[P`\0\x80a\t\\` \x84\x01\x84a\x13\x02V[`\x01`\x01`\xA0\x1B\x03\x16c\x89\xAF\xCBD0`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\t\x87\x91\x90a\x16OV[`@\x80Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\t\xA0W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\t\xB4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xD8\x91\x90a\x15\xB9V[\x90\x92P\x90P`\0`da\t\xFEa\t\xF4``\x87\x01`@\x88\x01a\x15\xDCV[\x85\x90`\xFF\x16a\x0F&V[\x81a\n\x05W\xFE[\x04\x90P`\0`da\n\x1Fa\t\xF4``\x88\x01`@\x89\x01a\x15\xDCV[\x81a\n&W\xFE[\x04\x90Pa\nca\n<`\x80\x87\x01``\x88\x01a\x13\x02V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84a\x0FPV[a\n\x9Da\nv`\xA0\x87\x01`\x80\x88\x01a\x13\x02V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83a\x0FPV[`\0\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x881dV`@Q\x80a\x01`\x01`@R\x80\x8A``\x01` \x81\x01\x90a\n\xEE\x91\x90a\x13\x02V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01a\x0B\x0C`\xA0\x8C\x01`\x80\x8D\x01a\x13\x02V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01a\x0B*`\xC0\x8C\x01`\xA0\x8D\x01a\x15NV[b\xFF\xFF\xFF\x16\x81R` \x01a\x0BD`\xE0\x8C\x01`\xC0\x8D\x01a\x14~V[`\x02\x0B\x81R` \x01a\x0B]a\x01\0\x8C\x01`\xE0\x8D\x01a\x14~V[`\x02\x0B\x81R` \x81\x01\x88\x90R`@\x81\x01\x87\x90Ra\x01\0\x8B\x015``\x82\x01Ra\x01 \x8B\x015`\x80\x82\x01R`\xA0\x01a\x0B\x9Ba\x01`\x8C\x01a\x01@\x8D\x01a\x13\x02V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8Aa\x01`\x015\x81RP`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0B\xCE\x91\x90a\x17yV[`\x80`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0B\xE8W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0B\xFCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C \x91\x90a\x15hV[\x93P\x93PPP\x85\x82\x10\x15a\r\x80W\x83\x82\x10\x15a\x0CqWa\x0Cqa\x0CI`\x80\x89\x01``\x8A\x01a\x13\x02V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0a\x0FPV[\x81\x86\x03a\x0C\x86a\x01\xA0\x89\x01a\x01\x80\x8A\x01a\x14FV[\x80\x15a\x0C\xD2WP`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16a\x0C\xC7`\x80\x8A\x01``\x8B\x01a\x13\x02V[`\x01`\x01`\xA0\x1B\x03\x16\x14[\x15a\rdW`@Qc.\x1A}M`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c.\x1A}M\x90a\r#\x90\x84\x90`\x04\x01a\x18'V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\r=W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\rQW=`\0\x80>=`\0\xFD[PPPPa\r_3\x82a\x10\x9EV[a\r~V[a\r~a\rw`\x80\x8A\x01``\x8B\x01a\x13\x02V[3\x83a\x11\x92V[P[\x84\x81\x10\x15a\x0E\xABW\x82\x81\x10\x15a\r\xA3Wa\r\xA3a\x0CI`\xA0\x89\x01`\x80\x8A\x01a\x13\x02V[\x80\x85\x03a\r\xB8a\x01\xA0\x89\x01a\x01\x80\x8A\x01a\x14FV[\x80\x15a\x0E\x04WP`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16a\r\xF9`\xA0\x8A\x01`\x80\x8B\x01a\x13\x02V[`\x01`\x01`\xA0\x1B\x03\x16\x14[\x15a\x0E\x96W`@Qc.\x1A}M`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c.\x1A}M\x90a\x0EU\x90\x84\x90`\x04\x01a\x18'V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0EoW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0E\x83W=`\0\x80>=`\0\xFD[PPPPa\x0E\x913\x82a\x10\x9EV[a\x0E\xA9V[a\x0E\xA9a\rw`\xA0\x8A\x01`\x80\x8B\x01a\x13\x02V[P[PPPPPPPV[`@\x80Qc\xD5\x05\xAC\xCF`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x87\x90R`d\x81\x01\x86\x90R`\xFF\x85\x16`\x84\x82\x01R`\xA4\x81\x01\x84\x90R`\xC4\x81\x01\x83\x90R\x90Q`\x01`\x01`\xA0\x1B\x03\x88\x16\x91c\xD5\x05\xAC\xCF\x91`\xE4\x80\x83\x01\x92`\0\x92\x91\x90\x82\x90\x03\x01\x81\x83\x87\x80;\x15\x80\x15a\x05TW`\0\x80\xFD[`\0\x82\x15\x80a\x0FAWPP\x81\x81\x02\x81\x83\x82\x81a\x0F>W\xFE[\x04\x14[a\x0FJW`\0\x80\xFD[\x92\x91PPV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`$\x83\x01R`D\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`d\x90\x92\x01\x83R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\t^\xA7\xB3`\xE0\x1B\x17\x81R\x92Q\x82Q`\0\x94\x85\x94\x93\x89\x16\x93\x92\x91\x82\x91\x90\x80\x83\x83[` \x83\x10a\x0F\xCCW\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01a\x0F\xADV[`\x01\x83` \x03a\x01\0\n\x03\x80\x19\x82Q\x16\x81\x84Q\x16\x80\x82\x17\x85RPPPPPP\x90P\x01\x91PP`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x10.W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x103V[``\x91P[P\x91P\x91P\x81\x80\x15a\x10aWP\x80Q\x15\x80a\x10aWP\x80\x80` \x01\x90Q` \x81\x10\x15a\x10^W`\0\x80\xFD[PQ[a\x10\x97W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x02`$\x82\x01RaSA`\xF0\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[PPPPPV[`@\x80Q`\0\x80\x82R` \x82\x01\x90\x92R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x83\x90`@Q\x80\x82\x80Q\x90` \x01\x90\x80\x83\x83[` \x83\x10a\x10\xEAW\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01a\x10\xCBV[`\x01\x83` \x03a\x01\0\n\x03\x80\x19\x82Q\x16\x81\x84Q\x16\x80\x82\x17\x85RPPPPPP\x90P\x01\x91PP`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x11LW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x11QV[``\x91P[PP\x90P\x80a\x11\x8DW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x03`$\x82\x01RbSTE`\xE8\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[PPPV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`$\x83\x01R`D\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`d\x90\x92\x01\x83R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\xA9\x05\x9C\xBB`\xE0\x1B\x17\x81R\x92Q\x82Q`\0\x94\x85\x94\x93\x89\x16\x93\x92\x91\x82\x91\x90\x80\x83\x83[` \x83\x10a\x12\x0EW\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01a\x11\xEFV[`\x01\x83` \x03a\x01\0\n\x03\x80\x19\x82Q\x16\x81\x84Q\x16\x80\x82\x17\x85RPPPPPP\x90P\x01\x91PP`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x12pW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x12uV[``\x91P[P\x91P\x91P\x81\x80\x15a\x12\xA3WP\x80Q\x15\x80a\x12\xA3WP\x80\x80` \x01\x90Q` \x81\x10\x15a\x12\xA0W`\0\x80\xFD[PQ[a\x10\x97W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x02`$\x82\x01Ra\x14\xD5`\xF2\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x805b\xFF\xFF\xFF\x81\x16\x81\x14a\x12\xECW`\0\x80\xFD[\x91\x90PV[\x805`\xFF\x81\x16\x81\x14a\x12\xECW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x13\x13W\x80\x81\xFD[\x815a\x13\x1E\x81a\x18\xACV[\x93\x92PPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x13:W\x82\x83\xFD[\x845a\x13E\x81a\x18\xACV[\x93P` \x85\x015a\x13U\x81a\x18\xACV[\x92Pa\x13c`@\x86\x01a\x12\xD9V[\x91P``\x85\x015a\x13s\x81a\x18\xACV[\x93\x96\x92\x95P\x90\x93PPV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\x13\x96W\x81\x82\xFD[\x865a\x13\xA1\x81a\x18\xACV[\x95P` \x87\x015\x94P`@\x87\x015\x93Pa\x13\xBD``\x88\x01a\x12\xF1V[\x92P`\x80\x87\x015\x91P`\xA0\x87\x015\x90P\x92\x95P\x92\x95P\x92\x95V[`\0\x80` \x83\x85\x03\x12\x15a\x13\xE9W\x81\x82\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x14\0W\x83\x84\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x14\x13W\x83\x84\xFD[\x815\x81\x81\x11\x15a\x14!W\x84\x85\xFD[\x86` \x80\x83\x02\x85\x01\x01\x11\x15a\x144W\x84\x85\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[`\0` \x82\x84\x03\x12\x15a\x14WW\x80\x81\xFD[\x815a\x13\x1E\x81a\x18\xC4V[`\0` \x82\x84\x03\x12\x15a\x14sW\x80\x81\xFD[\x81Qa\x13\x1E\x81a\x18\xC4V[`\0` \x82\x84\x03\x12\x15a\x14\x8FW\x80\x81\xFD[\x815\x80`\x02\x0B\x81\x14a\x13\x1EW\x81\x82\xFD[`\0` \x82\x84\x03\x12\x15a\x14\xB0W\x80\x81\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x14\xC7W\x82\x83\xFD[\x81\x84\x01\x91P\x84`\x1F\x83\x01\x12a\x14\xDAW\x82\x83\xFD[\x81Q\x81\x81\x11\x15a\x14\xE6W\xFE[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01` \x01\x83\x81\x11\x82\x82\x10\x17\x15a\x15\x04W\xFE[`@R\x81\x81R\x83\x82\x01` \x01\x87\x10\x15a\x15\x1BW\x84\x85\xFD[a\x15,\x82` \x83\x01` \x87\x01a\x18|V[\x96\x95PPPPPPV[`\0a\x01\xA0\x82\x84\x03\x12\x15a\x15HW\x80\x81\xFD[P\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x15_W\x80\x81\xFD[a\x13\x1E\x82a\x12\xD9V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x15}W\x83\x84\xFD[\x84Q\x93P` \x85\x01Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x15\xA3W\x83\x84\xFD[`@\x86\x01Q``\x90\x96\x01Q\x94\x97\x90\x96P\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x15\xCBW\x81\x82\xFD[PP\x80Q` \x90\x91\x01Q\x90\x92\x90\x91PV[`\0` \x82\x84\x03\x12\x15a\x15\xEDW\x80\x81\xFD[a\x13\x1E\x82a\x12\xF1V[`\x01`\x01`\xA0\x1B\x03\x16\x90RV[`\0\x81Q\x80\x84Ra\x16\x1B\x81` \x86\x01` \x86\x01a\x18|V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\x02\x0B\x90RV[b\xFF\xFF\xFF\x16\x90RV[`\0\x82\x84\x837\x91\x01\x90\x81R\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x81R` \x01\x90V[`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81R\x91\x90\x92\x16` \x82\x01R`@\x81\x01\x91\x90\x91R``\x01\x90V[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x86\x01\x91P`@\x84\x82\x02\x87\x01\x01\x92P\x83\x87\x01\x85[\x82\x81\x10\x15a\x16\xDAW`?\x19\x88\x86\x03\x01\x84Ra\x16\xC8\x85\x83Qa\x16\x03V[\x94P\x92\x85\x01\x92\x90\x85\x01\x90`\x01\x01a\x16\xACV[P\x92\x97\x96PPPPPPPV[`\0` \x82Ra\x13\x1E` \x83\x01\x84a\x16\x03V[` \x80\x82R`\x14\x90\x82\x01Rs\x14\x19\\\x98\xD9[\x9D\x18Y\xD9H\x1D\x1B\xDB\xC8\x1C\xDBX[\x1B`b\x1B`@\x82\x01R``\x01\x90V[` \x80\x82R`\t\x90\x82\x01RhNot WETH9`\xB8\x1B`@\x82\x01R``\x01\x90V[` \x80\x82R`\x14\x90\x82\x01RsPercentage too large``\x1B`@\x82\x01R``\x01\x90V[`\0a\x01`\x82\x01\x90Pa\x17\x8D\x82\x84Qa\x15\xF6V[` \x83\x01Qa\x17\x9F` \x84\x01\x82a\x15\xF6V[P`@\x83\x01Qa\x17\xB2`@\x84\x01\x82a\x166V[P``\x83\x01Qa\x17\xC5``\x84\x01\x82a\x16/V[P`\x80\x83\x01Qa\x17\xD8`\x80\x84\x01\x82a\x16/V[P`\xA0\x83\x01Q`\xA0\x83\x01R`\xC0\x83\x01Q`\xC0\x83\x01R`\xE0\x83\x01Q`\xE0\x83\x01Ra\x01\0\x80\x84\x01Q\x81\x84\x01RPa\x01 \x80\x84\x01Qa\x18\x16\x82\x85\x01\x82a\x15\xF6V[PPa\x01@\x92\x83\x01Q\x91\x90\x92\x01R\x90V[\x90\x81R` \x01\x90V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x18FW\x82\x83\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x18`W\x82\x83\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x18uW`\0\x80\xFD[\x92P\x92\x90PV[`\0[\x83\x81\x10\x15a\x18\x97W\x81\x81\x01Q\x83\x82\x01R` \x01a\x18\x7FV[\x83\x81\x11\x15a\x18\xA6W`\0\x84\x84\x01R[PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x18\xC1W`\0\x80\xFD[PV[\x80\x15\x15\x81\x14a\x18\xC1W`\0\x80\xFD\xFE\xA2dipfsX\"\x12 i\xF5l\x9F\xDAdM;j\x0B\xAF\x86rO\xFB\xC6\xC5.t\xBC\x7F]@\xFE\x8C\x8E.J\x1A\x8F3\xBEdsolcC\0\x07\x06\x003";
    /// The deployed bytecode of the contract.
    pub static V3MIGRATOR_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct V3Migrator<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for V3Migrator<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for V3Migrator<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for V3Migrator<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for V3Migrator<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(V3Migrator)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> V3Migrator<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    V3MIGRATOR_ABI.clone(),
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
                V3MIGRATOR_ABI.clone(),
                V3MIGRATOR_BYTECODE.clone().into(),
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
        ///Calls the contract's `createAndInitializePoolIfNecessary` (0x13ead562) function
        pub fn create_and_initialize_pool_if_necessary(
            &self,
            token_0: ::ethers::core::types::Address,
            token_1: ::ethers::core::types::Address,
            fee: u32,
            sqrt_price_x96: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([19, 234, 213, 98], (token_0, token_1, fee, sqrt_price_x96))
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
        ///Calls the contract's `migrate` (0xd44f2bf2) function
        pub fn migrate(
            &self,
            params: MigrateParams,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([212, 79, 43, 242], (params,))
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
        ///Calls the contract's `nonfungiblePositionManager` (0xb44a2722) function
        pub fn nonfungible_position_manager(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([180, 74, 39, 34], ())
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
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for V3Migrator<M> {
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
        Hash
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
    ///Container type for all input parameters for the `migrate` function with signature `migrate((address,uint256,uint8,address,address,uint24,int24,int24,uint256,uint256,address,uint256,bool))` and selector `0xd44f2bf2`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
    )]
    #[ethcall(
        name = "migrate",
        abi = "migrate((address,uint256,uint8,address,address,uint24,int24,int24,uint256,uint256,address,uint256,bool))"
    )]
    pub struct MigrateCall {
        pub params: MigrateParams,
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
        Hash
    )]
    #[ethcall(name = "multicall", abi = "multicall(bytes[])")]
    pub struct MulticallCall {
        pub data: ::std::vec::Vec<::ethers::core::types::Bytes>,
    }
    ///Container type for all input parameters for the `nonfungiblePositionManager` function with signature `nonfungiblePositionManager()` and selector `0xb44a2722`
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
    #[ethcall(name = "nonfungiblePositionManager", abi = "nonfungiblePositionManager()")]
    pub struct NonfungiblePositionManagerCall;
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
    ///Container type for all of the contract's call
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
    )]
    pub enum V3MigratorCalls {
        Weth9(Weth9Call),
        CreateAndInitializePoolIfNecessary(CreateAndInitializePoolIfNecessaryCall),
        Factory(FactoryCall),
        Migrate(MigrateCall),
        Multicall(MulticallCall),
        NonfungiblePositionManager(NonfungiblePositionManagerCall),
        SelfPermit(SelfPermitCall),
        SelfPermitAllowed(SelfPermitAllowedCall),
        SelfPermitAllowedIfNecessary(SelfPermitAllowedIfNecessaryCall),
        SelfPermitIfNecessary(SelfPermitIfNecessaryCall),
    }
    impl ::ethers::core::abi::AbiDecode for V3MigratorCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <Weth9Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Weth9(decoded));
            }
            if let Ok(decoded) = <CreateAndInitializePoolIfNecessaryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CreateAndInitializePoolIfNecessary(decoded));
            }
            if let Ok(decoded) = <FactoryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Factory(decoded));
            }
            if let Ok(decoded) = <MigrateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Migrate(decoded));
            }
            if let Ok(decoded) = <MulticallCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Multicall(decoded));
            }
            if let Ok(decoded) = <NonfungiblePositionManagerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NonfungiblePositionManager(decoded));
            }
            if let Ok(decoded) = <SelfPermitCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SelfPermit(decoded));
            }
            if let Ok(decoded) = <SelfPermitAllowedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SelfPermitAllowed(decoded));
            }
            if let Ok(decoded) = <SelfPermitAllowedIfNecessaryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SelfPermitAllowedIfNecessary(decoded));
            }
            if let Ok(decoded) = <SelfPermitIfNecessaryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SelfPermitIfNecessary(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for V3MigratorCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Weth9(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CreateAndInitializePoolIfNecessary(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Factory(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Migrate(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Multicall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NonfungiblePositionManager(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SelfPermit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SelfPermitAllowed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SelfPermitAllowedIfNecessary(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SelfPermitIfNecessary(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for V3MigratorCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Weth9(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreateAndInitializePoolIfNecessary(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Factory(element) => ::core::fmt::Display::fmt(element, f),
                Self::Migrate(element) => ::core::fmt::Display::fmt(element, f),
                Self::Multicall(element) => ::core::fmt::Display::fmt(element, f),
                Self::NonfungiblePositionManager(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SelfPermit(element) => ::core::fmt::Display::fmt(element, f),
                Self::SelfPermitAllowed(element) => ::core::fmt::Display::fmt(element, f),
                Self::SelfPermitAllowedIfNecessary(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SelfPermitIfNecessary(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<Weth9Call> for V3MigratorCalls {
        fn from(value: Weth9Call) -> Self {
            Self::Weth9(value)
        }
    }
    impl ::core::convert::From<CreateAndInitializePoolIfNecessaryCall>
    for V3MigratorCalls {
        fn from(value: CreateAndInitializePoolIfNecessaryCall) -> Self {
            Self::CreateAndInitializePoolIfNecessary(value)
        }
    }
    impl ::core::convert::From<FactoryCall> for V3MigratorCalls {
        fn from(value: FactoryCall) -> Self {
            Self::Factory(value)
        }
    }
    impl ::core::convert::From<MigrateCall> for V3MigratorCalls {
        fn from(value: MigrateCall) -> Self {
            Self::Migrate(value)
        }
    }
    impl ::core::convert::From<MulticallCall> for V3MigratorCalls {
        fn from(value: MulticallCall) -> Self {
            Self::Multicall(value)
        }
    }
    impl ::core::convert::From<NonfungiblePositionManagerCall> for V3MigratorCalls {
        fn from(value: NonfungiblePositionManagerCall) -> Self {
            Self::NonfungiblePositionManager(value)
        }
    }
    impl ::core::convert::From<SelfPermitCall> for V3MigratorCalls {
        fn from(value: SelfPermitCall) -> Self {
            Self::SelfPermit(value)
        }
    }
    impl ::core::convert::From<SelfPermitAllowedCall> for V3MigratorCalls {
        fn from(value: SelfPermitAllowedCall) -> Self {
            Self::SelfPermitAllowed(value)
        }
    }
    impl ::core::convert::From<SelfPermitAllowedIfNecessaryCall> for V3MigratorCalls {
        fn from(value: SelfPermitAllowedIfNecessaryCall) -> Self {
            Self::SelfPermitAllowedIfNecessary(value)
        }
    }
    impl ::core::convert::From<SelfPermitIfNecessaryCall> for V3MigratorCalls {
        fn from(value: SelfPermitIfNecessaryCall) -> Self {
            Self::SelfPermitIfNecessary(value)
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
        Hash
    )]
    pub struct CreateAndInitializePoolIfNecessaryReturn {
        pub pool: ::ethers::core::types::Address,
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
        Hash
    )]
    pub struct MulticallReturn {
        pub results: ::std::vec::Vec<::ethers::core::types::Bytes>,
    }
    ///Container type for all return fields from the `nonfungiblePositionManager` function with signature `nonfungiblePositionManager()` and selector `0xb44a2722`
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
    pub struct NonfungiblePositionManagerReturn(pub ::ethers::core::types::Address);
}
