pub use mock_observations::*;
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
pub mod mock_observations {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_blockTimestamps"),
                        kind: ::ethers::core::abi::ethabi::ParamType::FixedArray(
                            ::std::boxed::Box::new(::ethers::core::abi::ethabi::ParamType::Uint(
                                32usize
                            ),),
                            4usize,
                        ),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint32[4]"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_tickCumulatives"),
                        kind: ::ethers::core::abi::ethabi::ParamType::FixedArray(
                            ::std::boxed::Box::new(::ethers::core::abi::ethabi::ParamType::Int(
                                56usize
                            ),),
                            4usize,
                        ),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("int56[4]"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned(
                            "_secondsPerLiquidityCumulativeX128s",
                        ),
                        kind: ::ethers::core::abi::ethabi::ParamType::FixedArray(
                            ::std::boxed::Box::new(::ethers::core::abi::ethabi::ParamType::Uint(
                                128usize
                            ),),
                            4usize,
                        ),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint128[4]"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_initializeds"),
                        kind: ::ethers::core::abi::ethabi::ParamType::FixedArray(
                            ::std::boxed::Box::new(::ethers::core::abi::ethabi::ParamType::Bool,),
                            4usize,
                        ),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("bool[4]"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_tick"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Int(24usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("int24"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_observationCardinality",),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint16"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_observationIndex"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint16"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_lastObservationCurrentTimestamp",),
                        kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("bool"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_liquidity"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint128"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("liquidity"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("liquidity"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(128usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint128"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("observations"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("observations"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("index"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(56usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int56"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(160usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint160"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("slot0"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("slot0"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(160usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint160"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(24usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int24"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint16"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint16"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint16"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint8"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static MOCKOBSERVATIONS_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x04\xAC8\x03\x80a\x04\xAC\x839\x81\x81\x01`@Ra\x02\xA0\x81\x10\x15a\x004W`\0\x80\xFD[Pa\x02\0\x81\x01Qa\x02 \x82\x01Qa\x02@\x83\x01Qa\x02`\x84\x01Qa\x02\x80\x85\x01Q`\x80\x86\x01\x94a\x01\0\x87\x01\x94a\x01\x80\x88\x01\x94\x91\x93\x90\x92`\0[`\x04\x81\x10\x15a\x01\x9CW`@Q\x80`\x80\x01`@R\x80\x8B\x83`\x04\x81\x10a\0\x8BW\xFE[` \x02\x01Qc\xFF\xFF\xFF\xFF\x16\x81R` \x01\x8A\x83`\x04\x81\x10a\0\xA7W\xFE[` \x02\x01Q`\x06\x0B\x81R` \x01\x89\x83`\x04\x81\x10a\0\xC0W\xFE[` \x02\x01Q`\x01`\x01`\x80\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x88\x83`\x04\x81\x10a\0\xE8W\xFE[` \x02\x01Q\x15\x15\x90R`\0\x82`\x04\x81\x10a\0\xFEW\xFE[\x82Q\x91\x01\x80T` \x84\x01Q`@\x85\x01Q``\x90\x95\x01Q\x15\x15`\x01`\xF8\x1B\x02`\x01`\x01`\xF8\x1B\x03`\x01`\x01`\xA0\x1B\x03\x90\x96\x16k\x01\0\0\0\0\0\0\0\0\0\0\0\x02`\x01`X\x1B`\x01`\xF8\x1B\x03\x19`\x06\x93\x90\x93\x0Bf\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16d\x01\0\0\0\0\x02f\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x1B\x19c\xFF\xFF\xFF\xFF\x90\x97\x16c\xFF\xFF\xFF\xFF\x19\x90\x95\x16\x94\x90\x94\x17\x95\x90\x95\x16\x92\x90\x92\x17\x16\x92\x90\x92\x17\x92\x90\x92\x16\x17\x90U`\x01\x01a\0kV[P`\x04\x80Tb\xFF\xFF\xFF\x19\x16b\xFF\xFF\xFF`\x02\x97\x90\x97\x0B\x96\x90\x96\x16\x95\x90\x95\x17d\xFF\xFF\0\0\0\x19\x16c\x01\0\0\0a\xFF\xFF\x95\x86\x16\x02\x17a\xFF\xFF`(\x1B\x19\x16e\x01\0\0\0\0\0\x93\x90\x94\x16\x92\x90\x92\x02\x92\x90\x92\x17`\xFF`\xB8\x1B\x19\x16`\x01`\xB8\x1B\x92\x15\x15\x92\x90\x92\x02\x91\x90\x91\x17`\x01`8\x1B`\x01`\xB8\x1B\x03\x19\x16g\x01\0\0\0\0\0\0\0`\x01`\x01`\x80\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x02\x17\x90UPa\x02m\x92P\x82\x91Pa\x02?\x90P`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0AW`\x005`\xE0\x1C\x80c\x1Ahe\x02\x14a\0FW\x80c%,\t\xD7\x14a\0jW\x80c8P\xC7\xBD\x14a\0\xC1W[`\0\x80\xFD[a\0Na\x01\x1AV[`@\x80Q`\x01`\x01`\x80\x1B\x03\x90\x92\x16\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[a\0\x87`\x04\x806\x03` \x81\x10\x15a\0\x80W`\0\x80\xFD[P5a\x014V[`@\x80Qc\xFF\xFF\xFF\xFF\x90\x95\x16\x85R`\x06\x93\x90\x93\x0B` \x85\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x83\x83\x01R\x15\x15``\x83\x01RQ\x90\x81\x90\x03`\x80\x01\x90\xF3[a\0\xC9a\x02\tV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x98\x16\x88R`\x02\x96\x90\x96\x0B` \x88\x01Ra\xFF\xFF\x94\x85\x16\x87\x87\x01R\x92\x84\x16``\x87\x01R\x92\x16`\x80\x85\x01R`\xFF\x90\x91\x16`\xA0\x84\x01R\x15\x15`\xC0\x83\x01RQ\x90\x81\x90\x03`\xE0\x01\x90\xF3[`\x04Tg\x01\0\0\0\0\0\0\0\x90\x04`\x01`\x01`\x80\x1B\x03\x16\x81V[`\0\x80`\0\x80`\0\x80\x86`\x04\x81\x10a\x01HW\xFE[`@\x80Q`\x80\x81\x01\x82R\x92\x90\x91\x01Tc\xFF\xFF\xFF\xFF\x81\x16\x83Rd\x01\0\0\0\0\x81\x04`\x06\x90\x81\x0B\x81\x0B\x90\x0B` \x84\x01R`\x01`\x01`\xA0\x1B\x03`\x01`X\x1B\x82\x04\x16\x91\x83\x01\x91\x90\x91R`\xFF`\x01`\xF8\x1B\x90\x91\x04\x81\x16\x15\x15``\x83\x01R`\x04T\x91\x92P`\x01`\xB8\x1B\x90\x91\x04\x16\x15a\x01\xE8W\x80Q`\x04\x80T`\0\x91e\x01\0\0\0\0\0\x90\x91\x04a\xFF\xFF\x16\x90\x81\x10a\x01\xD4W\xFE[\x01Tc\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x90\x91\x03B\x03\x16\x81R[\x80Q` \x82\x01Q`@\x83\x01Q``\x90\x93\x01Q\x91\x98\x90\x97P\x91\x95P\x93P\x91PPV[`\x04T`\0\x90`\x02\x81\x90\x0B\x90a\xFF\xFFe\x01\0\0\0\0\0\x82\x04\x81\x16\x91c\x01\0\0\0\x90\x04\x16\x83\x80\x80\x91\x92\x93\x94\x95\x96V\xFE\xA2dipfsX\"\x12 ,\xA7!Dj\x18\x10\xCB\xF0GV\xF0+\xC2\xBB!\x9E\xDD\xEE\xE0>\xA2\xAA\xB3\n\xAFi?\x8902ddsolcC\0\x07\x06\x003";
    /// The bytecode of the contract.
    pub static MOCKOBSERVATIONS_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0AW`\x005`\xE0\x1C\x80c\x1Ahe\x02\x14a\0FW\x80c%,\t\xD7\x14a\0jW\x80c8P\xC7\xBD\x14a\0\xC1W[`\0\x80\xFD[a\0Na\x01\x1AV[`@\x80Q`\x01`\x01`\x80\x1B\x03\x90\x92\x16\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[a\0\x87`\x04\x806\x03` \x81\x10\x15a\0\x80W`\0\x80\xFD[P5a\x014V[`@\x80Qc\xFF\xFF\xFF\xFF\x90\x95\x16\x85R`\x06\x93\x90\x93\x0B` \x85\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x83\x83\x01R\x15\x15``\x83\x01RQ\x90\x81\x90\x03`\x80\x01\x90\xF3[a\0\xC9a\x02\tV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x98\x16\x88R`\x02\x96\x90\x96\x0B` \x88\x01Ra\xFF\xFF\x94\x85\x16\x87\x87\x01R\x92\x84\x16``\x87\x01R\x92\x16`\x80\x85\x01R`\xFF\x90\x91\x16`\xA0\x84\x01R\x15\x15`\xC0\x83\x01RQ\x90\x81\x90\x03`\xE0\x01\x90\xF3[`\x04Tg\x01\0\0\0\0\0\0\0\x90\x04`\x01`\x01`\x80\x1B\x03\x16\x81V[`\0\x80`\0\x80`\0\x80\x86`\x04\x81\x10a\x01HW\xFE[`@\x80Q`\x80\x81\x01\x82R\x92\x90\x91\x01Tc\xFF\xFF\xFF\xFF\x81\x16\x83Rd\x01\0\0\0\0\x81\x04`\x06\x90\x81\x0B\x81\x0B\x90\x0B` \x84\x01R`\x01`\x01`\xA0\x1B\x03`\x01`X\x1B\x82\x04\x16\x91\x83\x01\x91\x90\x91R`\xFF`\x01`\xF8\x1B\x90\x91\x04\x81\x16\x15\x15``\x83\x01R`\x04T\x91\x92P`\x01`\xB8\x1B\x90\x91\x04\x16\x15a\x01\xE8W\x80Q`\x04\x80T`\0\x91e\x01\0\0\0\0\0\x90\x91\x04a\xFF\xFF\x16\x90\x81\x10a\x01\xD4W\xFE[\x01Tc\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x90\x91\x03B\x03\x16\x81R[\x80Q` \x82\x01Q`@\x83\x01Q``\x90\x93\x01Q\x91\x98\x90\x97P\x91\x95P\x93P\x91PPV[`\x04T`\0\x90`\x02\x81\x90\x0B\x90a\xFF\xFFe\x01\0\0\0\0\0\x82\x04\x81\x16\x91c\x01\0\0\0\x90\x04\x16\x83\x80\x80\x91\x92\x93\x94\x95\x96V\xFE\xA2dipfsX\"\x12 ,\xA7!Dj\x18\x10\xCB\xF0GV\xF0+\xC2\xBB!\x9E\xDD\xEE\xE0>\xA2\xAA\xB3\n\xAFi?\x8902ddsolcC\0\x07\x06\x003";
    /// The deployed bytecode of the contract.
    pub static MOCKOBSERVATIONS_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct MockObservations<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MockObservations<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MockObservations<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MockObservations<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MockObservations<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MockObservations))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MockObservations<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                MOCKOBSERVATIONS_ABI.clone(),
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
                MOCKOBSERVATIONS_ABI.clone(),
                MOCKOBSERVATIONS_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `liquidity` (0x1a686502) function
        pub fn liquidity(&self) -> ::ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([26, 104, 101, 2], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `observations` (0x252c09d7) function
        pub fn observations(
            &self,
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (u32, i64, ::ethers::core::types::U256, bool),
        > {
            self.0
                .method_hash([37, 44, 9, 215], index)
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
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for MockObservations<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
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
        Hash,
    )]
    #[ethcall(name = "liquidity", abi = "liquidity()")]
    pub struct LiquidityCall;
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
        Hash,
    )]
    #[ethcall(name = "observations", abi = "observations(uint256)")]
    pub struct ObservationsCall {
        pub index: ::ethers::core::types::U256,
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
        Hash,
    )]
    #[ethcall(name = "slot0", abi = "slot0()")]
    pub struct Slot0Call;
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
    pub enum MockObservationsCalls {
        Liquidity(LiquidityCall),
        Observations(ObservationsCall),
        Slot0(Slot0Call),
    }
    impl ::ethers::core::abi::AbiDecode for MockObservationsCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <LiquidityCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Liquidity(decoded));
            }
            if let Ok(decoded) = <ObservationsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Observations(decoded));
            }
            if let Ok(decoded) = <Slot0Call as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Slot0(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MockObservationsCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Liquidity(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Observations(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Slot0(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for MockObservationsCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Liquidity(element) => ::core::fmt::Display::fmt(element, f),
                Self::Observations(element) => ::core::fmt::Display::fmt(element, f),
                Self::Slot0(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<LiquidityCall> for MockObservationsCalls {
        fn from(value: LiquidityCall) -> Self {
            Self::Liquidity(value)
        }
    }
    impl ::core::convert::From<ObservationsCall> for MockObservationsCalls {
        fn from(value: ObservationsCall) -> Self {
            Self::Observations(value)
        }
    }
    impl ::core::convert::From<Slot0Call> for MockObservationsCalls {
        fn from(value: Slot0Call) -> Self {
            Self::Slot0(value)
        }
    }
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
        Hash,
    )]
    pub struct LiquidityReturn(pub u128);
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
        Hash,
    )]
    pub struct ObservationsReturn(pub u32, pub i64, pub ::ethers::core::types::U256, pub bool);
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
        Hash,
    )]
    pub struct Slot0Return(
        pub ::ethers::core::types::U256,
        pub i32,
        pub u16,
        pub u16,
        pub u16,
        pub u8,
        pub bool,
    );
}
