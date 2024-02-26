pub use erc721::*;
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
pub mod erc721 {
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
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static ERC721_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0\x1C\xEE8\x03\x80b\0\x1C\xEE\x839\x81\x81\x01`@R`@\x81\x10\x15b\0\x007W`\0\x80\xFD[\x81\x01\x90\x80\x80Q`@Q\x93\x92\x91\x90\x84d\x01\0\0\0\0\x82\x11\x15b\0\0XW`\0\x80\xFD[\x90\x83\x01\x90` \x82\x01\x85\x81\x11\x15b\0\0nW`\0\x80\xFD[\x82Qd\x01\0\0\0\0\x81\x11\x82\x82\x01\x88\x10\x17\x15b\0\0\x89W`\0\x80\xFD[\x82RP\x81Q` \x91\x82\x01\x92\x90\x91\x01\x90\x80\x83\x83`\0[\x83\x81\x10\x15b\0\0\xB8W\x81\x81\x01Q\x83\x82\x01R` \x01b\0\0\x9EV[PPPP\x90P\x90\x81\x01\x90`\x1F\x16\x80\x15b\0\0\xE6W\x80\x82\x03\x80Q`\x01\x83` \x03a\x01\0\n\x03\x19\x16\x81R` \x01\x91P[P`@R` \x01\x80Q`@Q\x93\x92\x91\x90\x84d\x01\0\0\0\0\x82\x11\x15b\0\x01\nW`\0\x80\xFD[\x90\x83\x01\x90` \x82\x01\x85\x81\x11\x15b\0\x01 W`\0\x80\xFD[\x82Qd\x01\0\0\0\0\x81\x11\x82\x82\x01\x88\x10\x17\x15b\0\x01;W`\0\x80\xFD[\x82RP\x81Q` \x91\x82\x01\x92\x90\x91\x01\x90\x80\x83\x83`\0[\x83\x81\x10\x15b\0\x01jW\x81\x81\x01Q\x83\x82\x01R` \x01b\0\x01PV[PPPP\x90P\x90\x81\x01\x90`\x1F\x16\x80\x15b\0\x01\x98W\x80\x82\x03\x80Q`\x01\x83` \x03a\x01\0\n\x03\x19\x16\x81R` \x01\x91P[P`@RPb\0\x01\xB3\x91Pc\x01\xFF\xC9\xA7`\xE0\x1B\x90Pb\0\x02\x1DV[\x81Qb\0\x01\xC8\x90`\x06\x90` \x85\x01\x90b\0\x02\xA2V[P\x80Qb\0\x01\xDE\x90`\x07\x90` \x84\x01\x90b\0\x02\xA2V[Pb\0\x01\xF1c\x80\xACX\xCD`\xE0\x1Bb\0\x02\x1DV[b\0\x02\x03c[^\x13\x9F`\xE0\x1Bb\0\x02\x1DV[b\0\x02\x15cx\x0E\x9Dc`\xE0\x1Bb\0\x02\x1DV[PPb\0\x03NV[`\x01`\x01`\xE0\x1B\x03\x19\x80\x82\x16\x14\x15b\0\x02}W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FERC165: invalid interface id\0\0\0\0`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\x01`\x01`\xE0\x1B\x03\x19\x16`\0\x90\x81R` \x81\x90R`@\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UV[\x82\x80T`\x01\x81`\x01\x16\x15a\x01\0\x02\x03\x16`\x02\x90\x04\x90`\0R` `\0 \x90`\x1F\x01` \x90\x04\x81\x01\x92\x82b\0\x02\xDAW`\0\x85Ub\0\x03%V[\x82`\x1F\x10b\0\x02\xF5W\x80Q`\xFF\x19\x16\x83\x80\x01\x17\x85Ub\0\x03%V[\x82\x80\x01`\x01\x01\x85U\x82\x15b\0\x03%W\x91\x82\x01[\x82\x81\x11\x15b\0\x03%W\x82Q\x82U\x91` \x01\x91\x90`\x01\x01\x90b\0\x03\x08V[Pb\0\x033\x92\x91Pb\0\x037V[P\x90V[[\x80\x82\x11\x15b\0\x033W`\0\x81U`\x01\x01b\0\x038V[a\x19\x90\x80b\0\x03^`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\x0BW`\x005`\xE0\x1C\x80cOl\xCC\xE7\x11a\0\xA2W\x80c\x95\xD8\x9BA\x11a\0qW\x80c\x95\xD8\x9BA\x14a\x03IW\x80c\xA2,\xB4e\x14a\x03QW\x80c\xB8\x8DO\xDE\x14a\x03\x7FW\x80c\xC8{V\xDD\x14a\x04EW\x80c\xE9\x85\xE9\xC5\x14a\x04bWa\x01\x0BV[\x80cOl\xCC\xE7\x14a\x02\xE1W\x80ccR!\x1E\x14a\x02\xFEW\x80cl\x03`\xEB\x14a\x03\x1BW\x80cp\xA0\x821\x14a\x03#Wa\x01\x0BV[\x80c\x18\x16\r\xDD\x11a\0\xDEW\x80c\x18\x16\r\xDD\x14a\x02/W\x80c#\xB8r\xDD\x14a\x02IW\x80c/t\\Y\x14a\x02\x7FW\x80cB\x84.\x0E\x14a\x02\xABWa\x01\x0BV[\x80c\x01\xFF\xC9\xA7\x14a\x01\x10W\x80c\x06\xFD\xDE\x03\x14a\x01KW\x80c\x08\x18\x12\xFC\x14a\x01\xC8W\x80c\t^\xA7\xB3\x14a\x02\x01W[`\0\x80\xFD[a\x017`\x04\x806\x03` \x81\x10\x15a\x01&W`\0\x80\xFD[P5`\x01`\x01`\xE0\x1B\x03\x19\x16a\x04\x90V[`@\x80Q\x91\x15\x15\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[a\x01Sa\x04\xB3V[`@\x80Q` \x80\x82R\x83Q\x81\x83\x01R\x83Q\x91\x92\x83\x92\x90\x83\x01\x91\x85\x01\x90\x80\x83\x83`\0[\x83\x81\x10\x15a\x01\x8DW\x81\x81\x01Q\x83\x82\x01R` \x01a\x01uV[PPPP\x90P\x90\x81\x01\x90`\x1F\x16\x80\x15a\x01\xBAW\x80\x82\x03\x80Q`\x01\x83` \x03a\x01\0\n\x03\x19\x16\x81R` \x01\x91P[P\x92PPP`@Q\x80\x91\x03\x90\xF3[a\x01\xE5`\x04\x806\x03` \x81\x10\x15a\x01\xDEW`\0\x80\xFD[P5a\x05IV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[a\x02-`\x04\x806\x03`@\x81\x10\x15a\x02\x17W`\0\x80\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x16\x90` \x015a\x05\xABV[\0[a\x027a\x06\x86V[`@\x80Q\x91\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[a\x02-`\x04\x806\x03``\x81\x10\x15a\x02_W`\0\x80\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x81\x16\x91` \x81\x015\x90\x91\x16\x90`@\x015a\x06\x97V[a\x027`\x04\x806\x03`@\x81\x10\x15a\x02\x95W`\0\x80\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x16\x90` \x015a\x06\xEEV[a\x02-`\x04\x806\x03``\x81\x10\x15a\x02\xC1W`\0\x80\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x81\x16\x91` \x81\x015\x90\x91\x16\x90`@\x015a\x07\x19V[a\x027`\x04\x806\x03` \x81\x10\x15a\x02\xF7W`\0\x80\xFD[P5a\x074V[a\x01\xE5`\x04\x806\x03` \x81\x10\x15a\x03\x14W`\0\x80\xFD[P5a\x07JV[a\x01Sa\x07rV[a\x027`\x04\x806\x03` \x81\x10\x15a\x039W`\0\x80\xFD[P5`\x01`\x01`\xA0\x1B\x03\x16a\x07\xD3V[a\x01Sa\x08;V[a\x02-`\x04\x806\x03`@\x81\x10\x15a\x03gW`\0\x80\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x16\x90` \x015\x15\x15a\x08\x9CV[a\x02-`\x04\x806\x03`\x80\x81\x10\x15a\x03\x95W`\0\x80\xFD[`\x01`\x01`\xA0\x1B\x03\x825\x81\x16\x92` \x81\x015\x90\x91\x16\x91`@\x82\x015\x91\x90\x81\x01\x90`\x80\x81\x01``\x82\x015d\x01\0\0\0\0\x81\x11\x15a\x03\xD0W`\0\x80\xFD[\x82\x01\x83` \x82\x01\x11\x15a\x03\xE2W`\0\x80\xFD[\x805\x90` \x01\x91\x84`\x01\x83\x02\x84\x01\x11d\x01\0\0\0\0\x83\x11\x17\x15a\x04\x04W`\0\x80\xFD[\x91\x90\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x92\x95Pa\t\xA1\x94PPPPPV[a\x01S`\x04\x806\x03` \x81\x10\x15a\x04[W`\0\x80\xFD[P5a\t\xFFV[a\x017`\x04\x806\x03`@\x81\x10\x15a\x04xW`\0\x80\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x81\x16\x91` \x015\x16a\x0C\x80V[`\x01`\x01`\xE0\x1B\x03\x19\x81\x16`\0\x90\x81R` \x81\x90R`@\x90 T`\xFF\x16[\x91\x90PV[`\x06\x80T`@\x80Q` `\x1F`\x02`\0\x19a\x01\0`\x01\x88\x16\x15\x02\x01\x90\x95\x16\x94\x90\x94\x04\x93\x84\x01\x81\x90\x04\x81\x02\x82\x01\x81\x01\x90\x92R\x82\x81R``\x93\x90\x92\x90\x91\x83\x01\x82\x82\x80\x15a\x05?W\x80`\x1F\x10a\x05\x14Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x05?V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x05\"W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[`\0a\x05T\x82a\x0C\xAEV[a\x05\x8FW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`,\x81R` \x01\x80a\x18\x85`,\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[P`\0\x90\x81R`\x04` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\0a\x05\xB6\x82a\x07JV[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a\x06\tW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`!\x81R` \x01\x80a\x19\t`!\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[\x80`\x01`\x01`\xA0\x1B\x03\x16a\x06\x1Ba\x0C\xBBV[`\x01`\x01`\xA0\x1B\x03\x16\x14\x80a\x06<WPa\x06<\x81a\x067a\x0C\xBBV[a\x0C\x80V[a\x06wW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`8\x81R` \x01\x80a\x17\xD8`8\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[a\x06\x81\x83\x83a\x0C\xBFV[PPPV[`\0a\x06\x92`\x02a\r-V[\x90P\x90V[a\x06\xA8a\x06\xA2a\x0C\xBBV[\x82a\r8V[a\x06\xE3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`1\x81R` \x01\x80a\x19*`1\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[a\x06\x81\x83\x83\x83a\r\xDCV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x01` R`@\x81 a\x07\x10\x90\x83a\x0F(V[\x90P[\x92\x91PPV[a\x06\x81\x83\x83\x83`@Q\x80` \x01`@R\x80`\0\x81RPa\t\xA1V[`\0\x80a\x07B`\x02\x84a\x0F4V[P\x93\x92PPPV[`\0a\x07\x13\x82`@Q\x80``\x01`@R\x80`)\x81R` \x01a\x18:`)\x919`\x02\x91\x90a\x0FPV[`\t\x80T`@\x80Q` `\x1F`\x02`\0\x19a\x01\0`\x01\x88\x16\x15\x02\x01\x90\x95\x16\x94\x90\x94\x04\x93\x84\x01\x81\x90\x04\x81\x02\x82\x01\x81\x01\x90\x92R\x82\x81R``\x93\x90\x92\x90\x91\x83\x01\x82\x82\x80\x15a\x05?W\x80`\x1F\x10a\x05\x14Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x05?V[`\0`\x01`\x01`\xA0\x1B\x03\x82\x16a\x08\x1AW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`*\x81R` \x01\x80a\x18\x10`*\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x01` R`@\x90 a\x07\x13\x90a\r-V[`\x07\x80T`@\x80Q` `\x1F`\x02`\0\x19a\x01\0`\x01\x88\x16\x15\x02\x01\x90\x95\x16\x94\x90\x94\x04\x93\x84\x01\x81\x90\x04\x81\x02\x82\x01\x81\x01\x90\x92R\x82\x81R``\x93\x90\x92\x90\x91\x83\x01\x82\x82\x80\x15a\x05?W\x80`\x1F\x10a\x05\x14Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x05?V[a\x08\xA4a\x0C\xBBV[`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a\t\nW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FERC721: approve to caller\0\0\0\0\0\0\0`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x80`\x05`\0a\t\x17a\x0C\xBBV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R` \x80\x83\x01\x93\x90\x93R`@\x91\x82\x01`\0\x90\x81 \x91\x87\x16\x80\x82R\x91\x90\x93R\x91 \x80T`\xFF\x19\x16\x92\x15\x15\x92\x90\x92\x17\x90\x91Ua\t[a\x0C\xBBV[`\x01`\x01`\xA0\x1B\x03\x16\x7F\x170~\xAB9\xABa\x07\xE8\x89\x98E\xAD=Y\xBD\x96S\xF2\0\xF2 \x92\x04\x89\xCA+Y7il1\x83`@Q\x80\x82\x15\x15\x81R` \x01\x91PP`@Q\x80\x91\x03\x90\xA3PPV[a\t\xB2a\t\xACa\x0C\xBBV[\x83a\r8V[a\t\xEDW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`1\x81R` \x01\x80a\x19*`1\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[a\t\xF9\x84\x84\x84\x84a\x0FgV[PPPPV[``a\n\n\x82a\x0C\xAEV[a\nEW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`/\x81R` \x01\x80a\x18\xDA`/\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[`\0\x82\x81R`\x08` \x90\x81R`@\x80\x83 \x80T\x82Q`\x02`\x01\x83\x16\x15a\x01\0\x02`\0\x19\x01\x90\x92\x16\x91\x90\x91\x04`\x1F\x81\x01\x85\x90\x04\x85\x02\x82\x01\x85\x01\x90\x93R\x82\x81R\x92\x90\x91\x90\x83\x01\x82\x82\x80\x15a\n\xD8W\x80`\x1F\x10a\n\xADWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\n\xD8V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\n\xBBW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P`\0a\n\xE9a\x07rV[\x90P\x80Q`\0\x14\x15a\n\xFDWP\x90Pa\x04\xAEV[\x81Q\x15a\x0B\xBEW\x80\x82`@Q` \x01\x80\x83\x80Q\x90` \x01\x90\x80\x83\x83[` \x83\x10a\x0B8W\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01a\x0B\x19V[Q\x81Q` \x93\x84\x03a\x01\0\n`\0\x19\x01\x80\x19\x90\x92\x16\x91\x16\x17\x90R\x85Q\x91\x90\x93\x01\x92\x85\x01\x91P\x80\x83\x83[` \x83\x10a\x0B\x80W\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01a\x0BaV[`\x01\x83` \x03a\x01\0\n\x03\x80\x19\x82Q\x16\x81\x84Q\x16\x80\x82\x17\x85RPPPPPP\x90P\x01\x92PPP`@Q` \x81\x83\x03\x03\x81R\x90`@R\x92PPPa\x04\xAEV[\x80a\x0B\xC8\x85a\x0F\xB9V[`@Q` \x01\x80\x83\x80Q\x90` \x01\x90\x80\x83\x83[` \x83\x10a\x0B\xFAW\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01a\x0B\xDBV[Q\x81Q` \x93\x84\x03a\x01\0\n`\0\x19\x01\x80\x19\x90\x92\x16\x91\x16\x17\x90R\x85Q\x91\x90\x93\x01\x92\x85\x01\x91P\x80\x83\x83[` \x83\x10a\x0CBW\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01a\x0C#V[`\x01\x83` \x03a\x01\0\n\x03\x80\x19\x82Q\x16\x81\x84Q\x16\x80\x82\x17\x85RPPPPPP\x90P\x01\x92PPP`@Q` \x81\x83\x03\x03\x81R\x90`@R\x92PPP\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\0\x90\x81R`\x05` \x90\x81R`@\x80\x83 \x93\x90\x94\x16\x82R\x91\x90\x91R T`\xFF\x16\x90V[`\0a\x07\x13`\x02\x83a\x10\x94V[3\x90V[`\0\x81\x81R`\x04` R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x81\x17\x90\x91U\x81\x90a\x0C\xF4\x82a\x07JV[`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%`@Q`@Q\x80\x91\x03\x90\xA4PPV[`\0a\x07\x13\x82a\x10\xA0V[`\0a\rC\x82a\x0C\xAEV[a\r~W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`,\x81R` \x01\x80a\x17\xAC`,\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[`\0a\r\x89\x83a\x07JV[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x14\x80a\r\xC4WP\x83`\x01`\x01`\xA0\x1B\x03\x16a\r\xB9\x84a\x05IV[`\x01`\x01`\xA0\x1B\x03\x16\x14[\x80a\r\xD4WPa\r\xD4\x81\x85a\x0C\x80V[\x94\x93PPPPV[\x82`\x01`\x01`\xA0\x1B\x03\x16a\r\xEF\x82a\x07JV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0E4W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`)\x81R` \x01\x80a\x18\xB1`)\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x0EyW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`$\x81R` \x01\x80a\x17\x88`$\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[a\x0E\x84\x83\x83\x83a\x06\x81V[a\x0E\x8F`\0\x82a\x0C\xBFV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x01` R`@\x90 a\x0E\xB1\x90\x82a\x10\xA4V[P`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x01` R`@\x90 a\x0E\xD4\x90\x82a\x10\xB0V[Pa\x0E\xE1`\x02\x82\x84a\x10\xBCV[P\x80\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF`@Q`@Q\x80\x91\x03\x90\xA4PPPV[`\0a\x07\x10\x83\x83a\x10\xD2V[`\0\x80\x80\x80a\x0FC\x86\x86a\x116V[\x90\x97\x90\x96P\x94PPPPPV[`\0a\x0F]\x84\x84\x84a\x11\xB1V[\x90P[\x93\x92PPPV[a\x0Fr\x84\x84\x84a\r\xDCV[a\x0F~\x84\x84\x84\x84a\x12{V[a\t\xF9W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`2\x81R` \x01\x80a\x17V`2\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[``\x81a\x0F\xDEWP`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R`\x03`\xFC\x1B` \x82\x01Ra\x04\xAEV[\x81`\0[\x81\x15a\x0F\xF6W`\x01\x01`\n\x82\x04\x91Pa\x0F\xE2V[`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x80\x15a\x10\x0FW`\0\x80\xFD[P`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x10:W` \x82\x01\x81\x806\x837\x01\x90P[P\x85\x93P\x90P`\0\x19\x82\x01[\x83\x15a\x10\x8BW`\n\x84\x06`0\x01`\xF8\x1B\x82\x82\x80`\x01\x90\x03\x93P\x81Q\x81\x10a\x10iW\xFE[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\n\x84\x04\x93Pa\x10FV[P\x94\x93PPPPV[`\0a\x07\x10\x83\x83a\x13\xE3V[T\x90V[`\0a\x07\x10\x83\x83a\x13\xFBV[`\0a\x07\x10\x83\x83a\x14\xC1V[`\0a\x0F]\x84\x84`\x01`\x01`\xA0\x1B\x03\x85\x16a\x15\x0BV[\x81T`\0\x90\x82\x10a\x11\x14W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`\"\x81R` \x01\x80a\x174`\"\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[\x82`\0\x01\x82\x81T\x81\x10a\x11#W\xFE[\x90`\0R` `\0 \x01T\x90P\x92\x91PPV[\x81T`\0\x90\x81\x90\x83\x10a\x11zW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`\"\x81R` \x01\x80a\x18c`\"\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[`\0\x84`\0\x01\x84\x81T\x81\x10a\x11\x8BW\xFE[\x90`\0R` `\0 \x90`\x02\x02\x01\x90P\x80`\0\x01T\x81`\x01\x01T\x92P\x92PP\x92P\x92\x90PV[`\0\x82\x81R`\x01\x84\x01` R`@\x81 T\x82\x81a\x12LW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R\x83\x81\x81Q\x81R` \x01\x91P\x80Q\x90` \x01\x90\x80\x83\x83`\0[\x83\x81\x10\x15a\x12\x11W\x81\x81\x01Q\x83\x82\x01R` \x01a\x11\xF9V[PPPP\x90P\x90\x81\x01\x90`\x1F\x16\x80\x15a\x12>W\x80\x82\x03\x80Q`\x01\x83` \x03a\x01\0\n\x03\x19\x16\x81R` \x01\x91P[P\x92PPP`@Q\x80\x91\x03\x90\xFD[P\x84`\0\x01`\x01\x82\x03\x81T\x81\x10a\x12_W\xFE[\x90`\0R` `\0 \x90`\x02\x02\x01`\x01\x01T\x91PP\x93\x92PPPV[`\0a\x12\x8F\x84`\x01`\x01`\xA0\x1B\x03\x16a\x15\xA2V[a\x12\x9BWP`\x01a\r\xD4V[`\0a\x13\xA9c\n\x85\xBD\x01`\xE1\x1Ba\x12\xB0a\x0C\xBBV[\x88\x87\x87`@Q`$\x01\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x84`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83\x81R` \x01\x80` \x01\x82\x81\x03\x82R\x83\x81\x81Q\x81R` \x01\x91P\x80Q\x90` \x01\x90\x80\x83\x83`\0[\x83\x81\x10\x15a\x13\x17W\x81\x81\x01Q\x83\x82\x01R` \x01a\x12\xFFV[PPPP\x90P\x90\x81\x01\x90`\x1F\x16\x80\x15a\x13DW\x80\x82\x03\x80Q`\x01\x83` \x03a\x01\0\n\x03\x19\x16\x81R` \x01\x91P[P\x95PPPPPP`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90`\x01`\x01`\xE0\x1B\x03\x19\x16` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x83\x81\x83\x16\x17\x83RPPPP`@Q\x80``\x01`@R\x80`2\x81R` \x01a\x17V`2\x919`\x01`\x01`\xA0\x1B\x03\x88\x16\x91\x90a\x15\xA8V[\x90P`\0\x81\x80` \x01\x90Q` \x81\x10\x15a\x13\xC2W`\0\x80\xFD[PQ`\x01`\x01`\xE0\x1B\x03\x19\x16c\n\x85\xBD\x01`\xE1\x1B\x14\x92PPP\x94\x93PPPPV[`\0\x90\x81R`\x01\x91\x90\x91\x01` R`@\x90 T\x15\x15\x90V[`\0\x81\x81R`\x01\x83\x01` R`@\x81 T\x80\x15a\x14\xB7W\x83T`\0\x19\x80\x83\x01\x91\x90\x81\x01\x90`\0\x90\x87\x90\x83\x90\x81\x10a\x14.W\xFE[\x90`\0R` `\0 \x01T\x90P\x80\x87`\0\x01\x84\x81T\x81\x10a\x14KW\xFE[`\0\x91\x82R` \x80\x83 \x90\x91\x01\x92\x90\x92U\x82\x81R`\x01\x89\x81\x01\x90\x92R`@\x90 \x90\x84\x01\x90U\x86T\x87\x90\x80a\x14{W\xFE[`\x01\x90\x03\x81\x81\x90`\0R` `\0 \x01`\0\x90U\x90U\x86`\x01\x01`\0\x87\x81R` \x01\x90\x81R` \x01`\0 `\0\x90U`\x01\x94PPPPPa\x07\x13V[`\0\x91PPa\x07\x13V[`\0a\x14\xCD\x83\x83a\x13\xE3V[a\x15\x03WP\x81T`\x01\x81\x81\x01\x84U`\0\x84\x81R` \x80\x82 \x90\x93\x01\x84\x90U\x84T\x84\x82R\x82\x86\x01\x90\x93R`@\x90 \x91\x90\x91Ua\x07\x13V[P`\0a\x07\x13V[`\0\x82\x81R`\x01\x84\x01` R`@\x81 T\x80a\x15pWPP`@\x80Q\x80\x82\x01\x82R\x83\x81R` \x80\x82\x01\x84\x81R\x86T`\x01\x81\x81\x01\x89U`\0\x89\x81R\x84\x81 \x95Q`\x02\x90\x93\x02\x90\x95\x01\x91\x82U\x91Q\x90\x82\x01U\x86T\x86\x84R\x81\x88\x01\x90\x92R\x92\x90\x91 Ua\x0F`V[\x82\x85`\0\x01`\x01\x83\x03\x81T\x81\x10a\x15\x83W\xFE[\x90`\0R` `\0 \x90`\x02\x02\x01`\x01\x01\x81\x90UP`\0\x91PPa\x0F`V[;\x15\x15\x90V[``a\x0F]\x84\x84`\0\x85\x85a\x15\xBC\x85a\x15\xA2V[a\x16\rW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\0\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@Q\x80\x82\x80Q\x90` \x01\x90\x80\x83\x83[` \x83\x10a\x16KW\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01a\x16,V[`\x01\x83` \x03a\x01\0\n\x03\x80\x19\x82Q\x16\x81\x84Q\x16\x80\x82\x17\x85RPPPPPP\x90P\x01\x91PP`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x16\xADW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x16\xB2V[``\x91P[P\x91P\x91Pa\x16\xC2\x82\x82\x86a\x16\xCDV[\x97\x96PPPPPPPV[``\x83\x15a\x16\xDCWP\x81a\x0F`V[\x82Q\x15a\x16\xECW\x82Q\x80\x84` \x01\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x81R\x84Q`$\x84\x01R\x84Q\x85\x93\x91\x92\x83\x92`D\x01\x91\x90\x85\x01\x90\x80\x83\x83`\0\x83\x15a\x12\x11W\x81\x81\x01Q\x83\x82\x01R` \x01a\x11\xF9V\xFEEnumerableSet: index out of boundsERC721: transfer to non ERC721Receiver implementerERC721: transfer to the zero addressERC721: operator query for nonexistent tokenERC721: approve caller is not owner nor approved for allERC721: balance query for the zero addressERC721: owner query for nonexistent tokenEnumerableMap: index out of boundsERC721: approved query for nonexistent tokenERC721: transfer of token that is not ownERC721Metadata: URI query for nonexistent tokenERC721: approval to current ownerERC721: transfer caller is not owner nor approved\xA2dipfsX\"\x12 \x11g\x04\xE0@\xE6K>5\xBD\x846\xC7\x03\xED\xF8\xB6\x08\x83W|\x10\xDD\xE2\xFF\x114&\xE3%r3dsolcC\0\x07\x06\x003";
    /// The bytecode of the contract.
    pub static ERC721_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\x0BW`\x005`\xE0\x1C\x80cOl\xCC\xE7\x11a\0\xA2W\x80c\x95\xD8\x9BA\x11a\0qW\x80c\x95\xD8\x9BA\x14a\x03IW\x80c\xA2,\xB4e\x14a\x03QW\x80c\xB8\x8DO\xDE\x14a\x03\x7FW\x80c\xC8{V\xDD\x14a\x04EW\x80c\xE9\x85\xE9\xC5\x14a\x04bWa\x01\x0BV[\x80cOl\xCC\xE7\x14a\x02\xE1W\x80ccR!\x1E\x14a\x02\xFEW\x80cl\x03`\xEB\x14a\x03\x1BW\x80cp\xA0\x821\x14a\x03#Wa\x01\x0BV[\x80c\x18\x16\r\xDD\x11a\0\xDEW\x80c\x18\x16\r\xDD\x14a\x02/W\x80c#\xB8r\xDD\x14a\x02IW\x80c/t\\Y\x14a\x02\x7FW\x80cB\x84.\x0E\x14a\x02\xABWa\x01\x0BV[\x80c\x01\xFF\xC9\xA7\x14a\x01\x10W\x80c\x06\xFD\xDE\x03\x14a\x01KW\x80c\x08\x18\x12\xFC\x14a\x01\xC8W\x80c\t^\xA7\xB3\x14a\x02\x01W[`\0\x80\xFD[a\x017`\x04\x806\x03` \x81\x10\x15a\x01&W`\0\x80\xFD[P5`\x01`\x01`\xE0\x1B\x03\x19\x16a\x04\x90V[`@\x80Q\x91\x15\x15\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[a\x01Sa\x04\xB3V[`@\x80Q` \x80\x82R\x83Q\x81\x83\x01R\x83Q\x91\x92\x83\x92\x90\x83\x01\x91\x85\x01\x90\x80\x83\x83`\0[\x83\x81\x10\x15a\x01\x8DW\x81\x81\x01Q\x83\x82\x01R` \x01a\x01uV[PPPP\x90P\x90\x81\x01\x90`\x1F\x16\x80\x15a\x01\xBAW\x80\x82\x03\x80Q`\x01\x83` \x03a\x01\0\n\x03\x19\x16\x81R` \x01\x91P[P\x92PPP`@Q\x80\x91\x03\x90\xF3[a\x01\xE5`\x04\x806\x03` \x81\x10\x15a\x01\xDEW`\0\x80\xFD[P5a\x05IV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[a\x02-`\x04\x806\x03`@\x81\x10\x15a\x02\x17W`\0\x80\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x16\x90` \x015a\x05\xABV[\0[a\x027a\x06\x86V[`@\x80Q\x91\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[a\x02-`\x04\x806\x03``\x81\x10\x15a\x02_W`\0\x80\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x81\x16\x91` \x81\x015\x90\x91\x16\x90`@\x015a\x06\x97V[a\x027`\x04\x806\x03`@\x81\x10\x15a\x02\x95W`\0\x80\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x16\x90` \x015a\x06\xEEV[a\x02-`\x04\x806\x03``\x81\x10\x15a\x02\xC1W`\0\x80\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x81\x16\x91` \x81\x015\x90\x91\x16\x90`@\x015a\x07\x19V[a\x027`\x04\x806\x03` \x81\x10\x15a\x02\xF7W`\0\x80\xFD[P5a\x074V[a\x01\xE5`\x04\x806\x03` \x81\x10\x15a\x03\x14W`\0\x80\xFD[P5a\x07JV[a\x01Sa\x07rV[a\x027`\x04\x806\x03` \x81\x10\x15a\x039W`\0\x80\xFD[P5`\x01`\x01`\xA0\x1B\x03\x16a\x07\xD3V[a\x01Sa\x08;V[a\x02-`\x04\x806\x03`@\x81\x10\x15a\x03gW`\0\x80\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x16\x90` \x015\x15\x15a\x08\x9CV[a\x02-`\x04\x806\x03`\x80\x81\x10\x15a\x03\x95W`\0\x80\xFD[`\x01`\x01`\xA0\x1B\x03\x825\x81\x16\x92` \x81\x015\x90\x91\x16\x91`@\x82\x015\x91\x90\x81\x01\x90`\x80\x81\x01``\x82\x015d\x01\0\0\0\0\x81\x11\x15a\x03\xD0W`\0\x80\xFD[\x82\x01\x83` \x82\x01\x11\x15a\x03\xE2W`\0\x80\xFD[\x805\x90` \x01\x91\x84`\x01\x83\x02\x84\x01\x11d\x01\0\0\0\0\x83\x11\x17\x15a\x04\x04W`\0\x80\xFD[\x91\x90\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x92\x95Pa\t\xA1\x94PPPPPV[a\x01S`\x04\x806\x03` \x81\x10\x15a\x04[W`\0\x80\xFD[P5a\t\xFFV[a\x017`\x04\x806\x03`@\x81\x10\x15a\x04xW`\0\x80\xFD[P`\x01`\x01`\xA0\x1B\x03\x815\x81\x16\x91` \x015\x16a\x0C\x80V[`\x01`\x01`\xE0\x1B\x03\x19\x81\x16`\0\x90\x81R` \x81\x90R`@\x90 T`\xFF\x16[\x91\x90PV[`\x06\x80T`@\x80Q` `\x1F`\x02`\0\x19a\x01\0`\x01\x88\x16\x15\x02\x01\x90\x95\x16\x94\x90\x94\x04\x93\x84\x01\x81\x90\x04\x81\x02\x82\x01\x81\x01\x90\x92R\x82\x81R``\x93\x90\x92\x90\x91\x83\x01\x82\x82\x80\x15a\x05?W\x80`\x1F\x10a\x05\x14Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x05?V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x05\"W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[`\0a\x05T\x82a\x0C\xAEV[a\x05\x8FW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`,\x81R` \x01\x80a\x18\x85`,\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[P`\0\x90\x81R`\x04` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\0a\x05\xB6\x82a\x07JV[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a\x06\tW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`!\x81R` \x01\x80a\x19\t`!\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[\x80`\x01`\x01`\xA0\x1B\x03\x16a\x06\x1Ba\x0C\xBBV[`\x01`\x01`\xA0\x1B\x03\x16\x14\x80a\x06<WPa\x06<\x81a\x067a\x0C\xBBV[a\x0C\x80V[a\x06wW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`8\x81R` \x01\x80a\x17\xD8`8\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[a\x06\x81\x83\x83a\x0C\xBFV[PPPV[`\0a\x06\x92`\x02a\r-V[\x90P\x90V[a\x06\xA8a\x06\xA2a\x0C\xBBV[\x82a\r8V[a\x06\xE3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`1\x81R` \x01\x80a\x19*`1\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[a\x06\x81\x83\x83\x83a\r\xDCV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x01` R`@\x81 a\x07\x10\x90\x83a\x0F(V[\x90P[\x92\x91PPV[a\x06\x81\x83\x83\x83`@Q\x80` \x01`@R\x80`\0\x81RPa\t\xA1V[`\0\x80a\x07B`\x02\x84a\x0F4V[P\x93\x92PPPV[`\0a\x07\x13\x82`@Q\x80``\x01`@R\x80`)\x81R` \x01a\x18:`)\x919`\x02\x91\x90a\x0FPV[`\t\x80T`@\x80Q` `\x1F`\x02`\0\x19a\x01\0`\x01\x88\x16\x15\x02\x01\x90\x95\x16\x94\x90\x94\x04\x93\x84\x01\x81\x90\x04\x81\x02\x82\x01\x81\x01\x90\x92R\x82\x81R``\x93\x90\x92\x90\x91\x83\x01\x82\x82\x80\x15a\x05?W\x80`\x1F\x10a\x05\x14Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x05?V[`\0`\x01`\x01`\xA0\x1B\x03\x82\x16a\x08\x1AW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`*\x81R` \x01\x80a\x18\x10`*\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x01` R`@\x90 a\x07\x13\x90a\r-V[`\x07\x80T`@\x80Q` `\x1F`\x02`\0\x19a\x01\0`\x01\x88\x16\x15\x02\x01\x90\x95\x16\x94\x90\x94\x04\x93\x84\x01\x81\x90\x04\x81\x02\x82\x01\x81\x01\x90\x92R\x82\x81R``\x93\x90\x92\x90\x91\x83\x01\x82\x82\x80\x15a\x05?W\x80`\x1F\x10a\x05\x14Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x05?V[a\x08\xA4a\x0C\xBBV[`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a\t\nW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FERC721: approve to caller\0\0\0\0\0\0\0`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[\x80`\x05`\0a\t\x17a\x0C\xBBV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R` \x80\x83\x01\x93\x90\x93R`@\x91\x82\x01`\0\x90\x81 \x91\x87\x16\x80\x82R\x91\x90\x93R\x91 \x80T`\xFF\x19\x16\x92\x15\x15\x92\x90\x92\x17\x90\x91Ua\t[a\x0C\xBBV[`\x01`\x01`\xA0\x1B\x03\x16\x7F\x170~\xAB9\xABa\x07\xE8\x89\x98E\xAD=Y\xBD\x96S\xF2\0\xF2 \x92\x04\x89\xCA+Y7il1\x83`@Q\x80\x82\x15\x15\x81R` \x01\x91PP`@Q\x80\x91\x03\x90\xA3PPV[a\t\xB2a\t\xACa\x0C\xBBV[\x83a\r8V[a\t\xEDW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`1\x81R` \x01\x80a\x19*`1\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[a\t\xF9\x84\x84\x84\x84a\x0FgV[PPPPV[``a\n\n\x82a\x0C\xAEV[a\nEW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`/\x81R` \x01\x80a\x18\xDA`/\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[`\0\x82\x81R`\x08` \x90\x81R`@\x80\x83 \x80T\x82Q`\x02`\x01\x83\x16\x15a\x01\0\x02`\0\x19\x01\x90\x92\x16\x91\x90\x91\x04`\x1F\x81\x01\x85\x90\x04\x85\x02\x82\x01\x85\x01\x90\x93R\x82\x81R\x92\x90\x91\x90\x83\x01\x82\x82\x80\x15a\n\xD8W\x80`\x1F\x10a\n\xADWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\n\xD8V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\n\xBBW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P`\0a\n\xE9a\x07rV[\x90P\x80Q`\0\x14\x15a\n\xFDWP\x90Pa\x04\xAEV[\x81Q\x15a\x0B\xBEW\x80\x82`@Q` \x01\x80\x83\x80Q\x90` \x01\x90\x80\x83\x83[` \x83\x10a\x0B8W\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01a\x0B\x19V[Q\x81Q` \x93\x84\x03a\x01\0\n`\0\x19\x01\x80\x19\x90\x92\x16\x91\x16\x17\x90R\x85Q\x91\x90\x93\x01\x92\x85\x01\x91P\x80\x83\x83[` \x83\x10a\x0B\x80W\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01a\x0BaV[`\x01\x83` \x03a\x01\0\n\x03\x80\x19\x82Q\x16\x81\x84Q\x16\x80\x82\x17\x85RPPPPPP\x90P\x01\x92PPP`@Q` \x81\x83\x03\x03\x81R\x90`@R\x92PPPa\x04\xAEV[\x80a\x0B\xC8\x85a\x0F\xB9V[`@Q` \x01\x80\x83\x80Q\x90` \x01\x90\x80\x83\x83[` \x83\x10a\x0B\xFAW\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01a\x0B\xDBV[Q\x81Q` \x93\x84\x03a\x01\0\n`\0\x19\x01\x80\x19\x90\x92\x16\x91\x16\x17\x90R\x85Q\x91\x90\x93\x01\x92\x85\x01\x91P\x80\x83\x83[` \x83\x10a\x0CBW\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01a\x0C#V[`\x01\x83` \x03a\x01\0\n\x03\x80\x19\x82Q\x16\x81\x84Q\x16\x80\x82\x17\x85RPPPPPP\x90P\x01\x92PPP`@Q` \x81\x83\x03\x03\x81R\x90`@R\x92PPP\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\0\x90\x81R`\x05` \x90\x81R`@\x80\x83 \x93\x90\x94\x16\x82R\x91\x90\x91R T`\xFF\x16\x90V[`\0a\x07\x13`\x02\x83a\x10\x94V[3\x90V[`\0\x81\x81R`\x04` R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x81\x17\x90\x91U\x81\x90a\x0C\xF4\x82a\x07JV[`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%`@Q`@Q\x80\x91\x03\x90\xA4PPV[`\0a\x07\x13\x82a\x10\xA0V[`\0a\rC\x82a\x0C\xAEV[a\r~W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`,\x81R` \x01\x80a\x17\xAC`,\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[`\0a\r\x89\x83a\x07JV[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x14\x80a\r\xC4WP\x83`\x01`\x01`\xA0\x1B\x03\x16a\r\xB9\x84a\x05IV[`\x01`\x01`\xA0\x1B\x03\x16\x14[\x80a\r\xD4WPa\r\xD4\x81\x85a\x0C\x80V[\x94\x93PPPPV[\x82`\x01`\x01`\xA0\x1B\x03\x16a\r\xEF\x82a\x07JV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0E4W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`)\x81R` \x01\x80a\x18\xB1`)\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x0EyW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`$\x81R` \x01\x80a\x17\x88`$\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[a\x0E\x84\x83\x83\x83a\x06\x81V[a\x0E\x8F`\0\x82a\x0C\xBFV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x01` R`@\x90 a\x0E\xB1\x90\x82a\x10\xA4V[P`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x01` R`@\x90 a\x0E\xD4\x90\x82a\x10\xB0V[Pa\x0E\xE1`\x02\x82\x84a\x10\xBCV[P\x80\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF`@Q`@Q\x80\x91\x03\x90\xA4PPPV[`\0a\x07\x10\x83\x83a\x10\xD2V[`\0\x80\x80\x80a\x0FC\x86\x86a\x116V[\x90\x97\x90\x96P\x94PPPPPV[`\0a\x0F]\x84\x84\x84a\x11\xB1V[\x90P[\x93\x92PPPV[a\x0Fr\x84\x84\x84a\r\xDCV[a\x0F~\x84\x84\x84\x84a\x12{V[a\t\xF9W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`2\x81R` \x01\x80a\x17V`2\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[``\x81a\x0F\xDEWP`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R`\x03`\xFC\x1B` \x82\x01Ra\x04\xAEV[\x81`\0[\x81\x15a\x0F\xF6W`\x01\x01`\n\x82\x04\x91Pa\x0F\xE2V[`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x80\x15a\x10\x0FW`\0\x80\xFD[P`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x10:W` \x82\x01\x81\x806\x837\x01\x90P[P\x85\x93P\x90P`\0\x19\x82\x01[\x83\x15a\x10\x8BW`\n\x84\x06`0\x01`\xF8\x1B\x82\x82\x80`\x01\x90\x03\x93P\x81Q\x81\x10a\x10iW\xFE[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\n\x84\x04\x93Pa\x10FV[P\x94\x93PPPPV[`\0a\x07\x10\x83\x83a\x13\xE3V[T\x90V[`\0a\x07\x10\x83\x83a\x13\xFBV[`\0a\x07\x10\x83\x83a\x14\xC1V[`\0a\x0F]\x84\x84`\x01`\x01`\xA0\x1B\x03\x85\x16a\x15\x0BV[\x81T`\0\x90\x82\x10a\x11\x14W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`\"\x81R` \x01\x80a\x174`\"\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[\x82`\0\x01\x82\x81T\x81\x10a\x11#W\xFE[\x90`\0R` `\0 \x01T\x90P\x92\x91PPV[\x81T`\0\x90\x81\x90\x83\x10a\x11zW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R`\"\x81R` \x01\x80a\x18c`\"\x919`@\x01\x91PP`@Q\x80\x91\x03\x90\xFD[`\0\x84`\0\x01\x84\x81T\x81\x10a\x11\x8BW\xFE[\x90`\0R` `\0 \x90`\x02\x02\x01\x90P\x80`\0\x01T\x81`\x01\x01T\x92P\x92PP\x92P\x92\x90PV[`\0\x82\x81R`\x01\x84\x01` R`@\x81 T\x82\x81a\x12LW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01\x80\x80` \x01\x82\x81\x03\x82R\x83\x81\x81Q\x81R` \x01\x91P\x80Q\x90` \x01\x90\x80\x83\x83`\0[\x83\x81\x10\x15a\x12\x11W\x81\x81\x01Q\x83\x82\x01R` \x01a\x11\xF9V[PPPP\x90P\x90\x81\x01\x90`\x1F\x16\x80\x15a\x12>W\x80\x82\x03\x80Q`\x01\x83` \x03a\x01\0\n\x03\x19\x16\x81R` \x01\x91P[P\x92PPP`@Q\x80\x91\x03\x90\xFD[P\x84`\0\x01`\x01\x82\x03\x81T\x81\x10a\x12_W\xFE[\x90`\0R` `\0 \x90`\x02\x02\x01`\x01\x01T\x91PP\x93\x92PPPV[`\0a\x12\x8F\x84`\x01`\x01`\xA0\x1B\x03\x16a\x15\xA2V[a\x12\x9BWP`\x01a\r\xD4V[`\0a\x13\xA9c\n\x85\xBD\x01`\xE1\x1Ba\x12\xB0a\x0C\xBBV[\x88\x87\x87`@Q`$\x01\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x84`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83\x81R` \x01\x80` \x01\x82\x81\x03\x82R\x83\x81\x81Q\x81R` \x01\x91P\x80Q\x90` \x01\x90\x80\x83\x83`\0[\x83\x81\x10\x15a\x13\x17W\x81\x81\x01Q\x83\x82\x01R` \x01a\x12\xFFV[PPPP\x90P\x90\x81\x01\x90`\x1F\x16\x80\x15a\x13DW\x80\x82\x03\x80Q`\x01\x83` \x03a\x01\0\n\x03\x19\x16\x81R` \x01\x91P[P\x95PPPPPP`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90`\x01`\x01`\xE0\x1B\x03\x19\x16` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x83\x81\x83\x16\x17\x83RPPPP`@Q\x80``\x01`@R\x80`2\x81R` \x01a\x17V`2\x919`\x01`\x01`\xA0\x1B\x03\x88\x16\x91\x90a\x15\xA8V[\x90P`\0\x81\x80` \x01\x90Q` \x81\x10\x15a\x13\xC2W`\0\x80\xFD[PQ`\x01`\x01`\xE0\x1B\x03\x19\x16c\n\x85\xBD\x01`\xE1\x1B\x14\x92PPP\x94\x93PPPPV[`\0\x90\x81R`\x01\x91\x90\x91\x01` R`@\x90 T\x15\x15\x90V[`\0\x81\x81R`\x01\x83\x01` R`@\x81 T\x80\x15a\x14\xB7W\x83T`\0\x19\x80\x83\x01\x91\x90\x81\x01\x90`\0\x90\x87\x90\x83\x90\x81\x10a\x14.W\xFE[\x90`\0R` `\0 \x01T\x90P\x80\x87`\0\x01\x84\x81T\x81\x10a\x14KW\xFE[`\0\x91\x82R` \x80\x83 \x90\x91\x01\x92\x90\x92U\x82\x81R`\x01\x89\x81\x01\x90\x92R`@\x90 \x90\x84\x01\x90U\x86T\x87\x90\x80a\x14{W\xFE[`\x01\x90\x03\x81\x81\x90`\0R` `\0 \x01`\0\x90U\x90U\x86`\x01\x01`\0\x87\x81R` \x01\x90\x81R` \x01`\0 `\0\x90U`\x01\x94PPPPPa\x07\x13V[`\0\x91PPa\x07\x13V[`\0a\x14\xCD\x83\x83a\x13\xE3V[a\x15\x03WP\x81T`\x01\x81\x81\x01\x84U`\0\x84\x81R` \x80\x82 \x90\x93\x01\x84\x90U\x84T\x84\x82R\x82\x86\x01\x90\x93R`@\x90 \x91\x90\x91Ua\x07\x13V[P`\0a\x07\x13V[`\0\x82\x81R`\x01\x84\x01` R`@\x81 T\x80a\x15pWPP`@\x80Q\x80\x82\x01\x82R\x83\x81R` \x80\x82\x01\x84\x81R\x86T`\x01\x81\x81\x01\x89U`\0\x89\x81R\x84\x81 \x95Q`\x02\x90\x93\x02\x90\x95\x01\x91\x82U\x91Q\x90\x82\x01U\x86T\x86\x84R\x81\x88\x01\x90\x92R\x92\x90\x91 Ua\x0F`V[\x82\x85`\0\x01`\x01\x83\x03\x81T\x81\x10a\x15\x83W\xFE[\x90`\0R` `\0 \x90`\x02\x02\x01`\x01\x01\x81\x90UP`\0\x91PPa\x0F`V[;\x15\x15\x90V[``a\x0F]\x84\x84`\0\x85\x85a\x15\xBC\x85a\x15\xA2V[a\x16\rW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`\0\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@Q\x80\x82\x80Q\x90` \x01\x90\x80\x83\x83[` \x83\x10a\x16KW\x80Q\x82R`\x1F\x19\x90\x92\x01\x91` \x91\x82\x01\x91\x01a\x16,V[`\x01\x83` \x03a\x01\0\n\x03\x80\x19\x82Q\x16\x81\x84Q\x16\x80\x82\x17\x85RPPPPPP\x90P\x01\x91PP`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x16\xADW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x16\xB2V[``\x91P[P\x91P\x91Pa\x16\xC2\x82\x82\x86a\x16\xCDV[\x97\x96PPPPPPPV[``\x83\x15a\x16\xDCWP\x81a\x0F`V[\x82Q\x15a\x16\xECW\x82Q\x80\x84` \x01\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x81R\x84Q`$\x84\x01R\x84Q\x85\x93\x91\x92\x83\x92`D\x01\x91\x90\x85\x01\x90\x80\x83\x83`\0\x83\x15a\x12\x11W\x81\x81\x01Q\x83\x82\x01R` \x01a\x11\xF9V\xFEEnumerableSet: index out of boundsERC721: transfer to non ERC721Receiver implementerERC721: transfer to the zero addressERC721: operator query for nonexistent tokenERC721: approve caller is not owner nor approved for allERC721: balance query for the zero addressERC721: owner query for nonexistent tokenEnumerableMap: index out of boundsERC721: approved query for nonexistent tokenERC721: transfer of token that is not ownERC721Metadata: URI query for nonexistent tokenERC721: approval to current ownerERC721: transfer caller is not owner nor approved\xA2dipfsX\"\x12 \x11g\x04\xE0@\xE6K>5\xBD\x846\xC7\x03\xED\xF8\xB6\x08\x83W|\x10\xDD\xE2\xFF\x114&\xE3%r3dsolcC\0\x07\x06\x003";
    /// The deployed bytecode of the contract.
    pub static ERC721_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct ERC721<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ERC721<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ERC721<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ERC721<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ERC721<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(ERC721))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ERC721<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                ERC721_ABI.clone(),
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
                ERC721_ABI.clone(),
                ERC721_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
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
        ///Calls the contract's `getApproved` (0x081812fc) function
        pub fn get_approved(
            &self,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([8, 24, 18, 252], token_id)
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
        ///Calls the contract's `supportsInterface` (0x01ffc9a7) function
        pub fn supports_interface(
            &self,
            interface_id: [u8; 4],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([1, 255, 201, 167], interface_id)
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
        ///Gets the contract's `Transfer` event
        pub fn transfer_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, TransferFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ERC721Events> {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for ERC721<M> {
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
    pub enum ERC721Events {
        ApprovalFilter(ApprovalFilter),
        ApprovalForAllFilter(ApprovalForAllFilter),
        TransferFilter(TransferFilter),
    }
    impl ::ethers::contract::EthLogDecode for ERC721Events {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(ERC721Events::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = ApprovalForAllFilter::decode_log(log) {
                return Ok(ERC721Events::ApprovalForAllFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(ERC721Events::TransferFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for ERC721Events {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ApprovalFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ApprovalForAllFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ApprovalFilter> for ERC721Events {
        fn from(value: ApprovalFilter) -> Self {
            Self::ApprovalFilter(value)
        }
    }
    impl ::core::convert::From<ApprovalForAllFilter> for ERC721Events {
        fn from(value: ApprovalForAllFilter) -> Self {
            Self::ApprovalForAllFilter(value)
        }
    }
    impl ::core::convert::From<TransferFilter> for ERC721Events {
        fn from(value: TransferFilter) -> Self {
            Self::TransferFilter(value)
        }
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
    pub enum ERC721Calls {
        Approve(ApproveCall),
        BalanceOf(BalanceOfCall),
        BaseURI(BaseURICall),
        GetApproved(GetApprovedCall),
        IsApprovedForAll(IsApprovedForAllCall),
        Name(NameCall),
        OwnerOf(OwnerOfCall),
        SafeTransferFrom(SafeTransferFromCall),
        SafeTransferFromWithFromAndToAndData(SafeTransferFromWithFromAndToAndDataCall),
        SetApprovalForAll(SetApprovalForAllCall),
        SupportsInterface(SupportsInterfaceCall),
        Symbol(SymbolCall),
        TokenByIndex(TokenByIndexCall),
        TokenOfOwnerByIndex(TokenOfOwnerByIndexCall),
        TokenURI(TokenURICall),
        TotalSupply(TotalSupplyCall),
        TransferFrom(TransferFromCall),
    }
    impl ::ethers::core::abi::AbiDecode for ERC721Calls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <ApproveCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Approve(decoded));
            }
            if let Ok(decoded) = <BalanceOfCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BalanceOf(decoded));
            }
            if let Ok(decoded) = <BaseURICall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BaseURI(decoded));
            }
            if let Ok(decoded) = <GetApprovedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetApproved(decoded));
            }
            if let Ok(decoded) =
                <IsApprovedForAllCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::IsApprovedForAll(decoded));
            }
            if let Ok(decoded) = <NameCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Name(decoded));
            }
            if let Ok(decoded) = <OwnerOfCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::OwnerOf(decoded));
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
            if let Ok(decoded) =
                <SetApprovalForAllCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetApprovalForAll(decoded));
            }
            if let Ok(decoded) =
                <SupportsInterfaceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SupportsInterface(decoded));
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
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ERC721Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Approve(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BalanceOf(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BaseURI(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetApproved(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsApprovedForAll(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Name(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OwnerOf(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SafeTransferFrom(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SafeTransferFromWithFromAndToAndData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetApprovalForAll(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SupportsInterface(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Symbol(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TokenByIndex(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TokenOfOwnerByIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TokenURI(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TotalSupply(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TransferFrom(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for ERC721Calls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Approve(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::BaseURI(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetApproved(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsApprovedForAll(element) => ::core::fmt::Display::fmt(element, f),
                Self::Name(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnerOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::SafeTransferFrom(element) => ::core::fmt::Display::fmt(element, f),
                Self::SafeTransferFromWithFromAndToAndData(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetApprovalForAll(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupportsInterface(element) => ::core::fmt::Display::fmt(element, f),
                Self::Symbol(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenByIndex(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenOfOwnerByIndex(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenURI(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalSupply(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFrom(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ApproveCall> for ERC721Calls {
        fn from(value: ApproveCall) -> Self {
            Self::Approve(value)
        }
    }
    impl ::core::convert::From<BalanceOfCall> for ERC721Calls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<BaseURICall> for ERC721Calls {
        fn from(value: BaseURICall) -> Self {
            Self::BaseURI(value)
        }
    }
    impl ::core::convert::From<GetApprovedCall> for ERC721Calls {
        fn from(value: GetApprovedCall) -> Self {
            Self::GetApproved(value)
        }
    }
    impl ::core::convert::From<IsApprovedForAllCall> for ERC721Calls {
        fn from(value: IsApprovedForAllCall) -> Self {
            Self::IsApprovedForAll(value)
        }
    }
    impl ::core::convert::From<NameCall> for ERC721Calls {
        fn from(value: NameCall) -> Self {
            Self::Name(value)
        }
    }
    impl ::core::convert::From<OwnerOfCall> for ERC721Calls {
        fn from(value: OwnerOfCall) -> Self {
            Self::OwnerOf(value)
        }
    }
    impl ::core::convert::From<SafeTransferFromCall> for ERC721Calls {
        fn from(value: SafeTransferFromCall) -> Self {
            Self::SafeTransferFrom(value)
        }
    }
    impl ::core::convert::From<SafeTransferFromWithFromAndToAndDataCall> for ERC721Calls {
        fn from(value: SafeTransferFromWithFromAndToAndDataCall) -> Self {
            Self::SafeTransferFromWithFromAndToAndData(value)
        }
    }
    impl ::core::convert::From<SetApprovalForAllCall> for ERC721Calls {
        fn from(value: SetApprovalForAllCall) -> Self {
            Self::SetApprovalForAll(value)
        }
    }
    impl ::core::convert::From<SupportsInterfaceCall> for ERC721Calls {
        fn from(value: SupportsInterfaceCall) -> Self {
            Self::SupportsInterface(value)
        }
    }
    impl ::core::convert::From<SymbolCall> for ERC721Calls {
        fn from(value: SymbolCall) -> Self {
            Self::Symbol(value)
        }
    }
    impl ::core::convert::From<TokenByIndexCall> for ERC721Calls {
        fn from(value: TokenByIndexCall) -> Self {
            Self::TokenByIndex(value)
        }
    }
    impl ::core::convert::From<TokenOfOwnerByIndexCall> for ERC721Calls {
        fn from(value: TokenOfOwnerByIndexCall) -> Self {
            Self::TokenOfOwnerByIndex(value)
        }
    }
    impl ::core::convert::From<TokenURICall> for ERC721Calls {
        fn from(value: TokenURICall) -> Self {
            Self::TokenURI(value)
        }
    }
    impl ::core::convert::From<TotalSupplyCall> for ERC721Calls {
        fn from(value: TotalSupplyCall) -> Self {
            Self::TotalSupply(value)
        }
    }
    impl ::core::convert::From<TransferFromCall> for ERC721Calls {
        fn from(value: TransferFromCall) -> Self {
            Self::TransferFrom(value)
        }
    }
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
