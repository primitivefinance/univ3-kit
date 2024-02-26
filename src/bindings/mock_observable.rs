pub use mock_observable::*;
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
pub mod mock_observable {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("secondsAgos"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Array(
                            ::std::boxed::Box::new(::ethers::core::abi::ethabi::ParamType::Uint(
                                32usize
                            ),),
                        ),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint32[]"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("tickCumulatives"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Array(
                            ::std::boxed::Box::new(::ethers::core::abi::ethabi::ParamType::Int(
                                56usize
                            ),),
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
                            ::std::boxed::Box::new(::ethers::core::abi::ethabi::ParamType::Uint(
                                160usize
                            ),),
                        ),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint160[]"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([(
                ::std::borrow::ToOwned::to_owned("observe"),
                ::std::vec![::ethers::core::abi::ethabi::Function {
                    name: ::std::borrow::ToOwned::to_owned("observe"),
                    inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("secondsAgos"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Array(
                            ::std::boxed::Box::new(::ethers::core::abi::ethabi::ParamType::Uint(
                                32usize
                            ),),
                        ),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint32[]"),
                        ),
                    },],
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
                },],
            )]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static MOCKOBSERVABLE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x07L8\x03\x80a\x07L\x839\x81\x81\x01`@R``\x81\x10\x15a\x003W`\0\x80\xFD[\x81\x01\x90\x80\x80Q`@Q\x93\x92\x91\x90\x84d\x01\0\0\0\0\x82\x11\x15a\0SW`\0\x80\xFD[\x90\x83\x01\x90` \x82\x01\x85\x81\x11\x15a\0hW`\0\x80\xFD[\x82Q\x86` \x82\x02\x83\x01\x11d\x01\0\0\0\0\x82\x11\x17\x15a\0\x85W`\0\x80\xFD[\x82RP\x81Q` \x91\x82\x01\x92\x82\x01\x91\x02\x80\x83\x83`\0[\x83\x81\x10\x15a\0\xB2W\x81\x81\x01Q\x83\x82\x01R` \x01a\0\x9AV[PPPP\x90P\x01`@R` \x01\x80Q`@Q\x93\x92\x91\x90\x84d\x01\0\0\0\0\x82\x11\x15a\0\xDBW`\0\x80\xFD[\x90\x83\x01\x90` \x82\x01\x85\x81\x11\x15a\0\xF0W`\0\x80\xFD[\x82Q\x86` \x82\x02\x83\x01\x11d\x01\0\0\0\0\x82\x11\x17\x15a\x01\rW`\0\x80\xFD[\x82RP\x81Q` \x91\x82\x01\x92\x82\x01\x91\x02\x80\x83\x83`\0[\x83\x81\x10\x15a\x01:W\x81\x81\x01Q\x83\x82\x01R` \x01a\x01\"V[PPPP\x90P\x01`@R` \x01\x80Q`@Q\x93\x92\x91\x90\x84d\x01\0\0\0\0\x82\x11\x15a\x01cW`\0\x80\xFD[\x90\x83\x01\x90` \x82\x01\x85\x81\x11\x15a\x01xW`\0\x80\xFD[\x82Q\x86` \x82\x02\x83\x01\x11d\x01\0\0\0\0\x82\x11\x17\x15a\x01\x95W`\0\x80\xFD[\x82RP\x81Q` \x91\x82\x01\x92\x82\x01\x91\x02\x80\x83\x83`\0[\x83\x81\x10\x15a\x01\xC2W\x81\x81\x01Q\x83\x82\x01R` \x01a\x01\xAAV[PPPP\x90P\x01`@RPPP\x82Q`\x02\x14\x80\x15a\x01\xE1WP\x81Q`\x02\x14[\x80\x15a\x01\xEEWP\x80Q`\x02\x14[a\x02?W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01R\x7FInvalid test case size\0\0\0\0\0\0\0\0\0\0`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`@Q\x80``\x01`@R\x80\x84`\0\x81Q\x81\x10a\x02WW\xFE[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x81R` \x01\x83`\0\x81Q\x81\x10a\x02wW\xFE[` \x02` \x01\x01Q`\x06\x0B\x81R` \x01\x82`\0\x81Q\x81\x10a\x02\x94W\xFE[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x90\x92R\x82Q`\0\x80T\x92\x85\x01Q`@\x95\x86\x01Q\x90\x94\x16k\x01\0\0\0\0\0\0\0\0\0\0\0\x02`\x01`X\x1B`\x01`\xF8\x1B\x03\x19`\x06\x95\x90\x95\x0Bf\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16d\x01\0\0\0\0\x02f\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x1B\x19c\xFF\xFF\xFF\xFF\x90\x94\x16c\xFF\xFF\xFF\xFF\x19\x90\x95\x16\x94\x90\x94\x17\x92\x90\x92\x16\x92\x90\x92\x17\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80Q``\x81\x01\x90\x91R\x83Q\x81\x90\x85\x90`\x01\x90\x81\x10a\x03>W\xFE[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x81R` \x01\x83`\x01\x81Q\x81\x10a\x03^W\xFE[` \x02` \x01\x01Q`\x06\x0B\x81R` \x01\x82`\x01\x81Q\x81\x10a\x03{W\xFE[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x90\x92R\x82Q`\x01\x80T\x92\x85\x01Q`@\x90\x95\x01Q\x90\x93\x16k\x01\0\0\0\0\0\0\0\0\0\0\0\x02`\x01`X\x1B`\x01`\xF8\x1B\x03\x19`\x06\x95\x90\x95\x0Bf\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16d\x01\0\0\0\0\x02f\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x1B\x19c\xFF\xFF\xFF\xFF\x90\x93\x16c\xFF\xFF\xFF\xFF\x19\x90\x94\x16\x93\x90\x93\x17\x91\x90\x91\x16\x91\x90\x91\x17\x92\x90\x92\x16\x91\x90\x91\x17\x90UPPPa\x03/\x80a\x04\x1D`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0+W`\x005`\xE0\x1C\x80c\x88;\xDB\xFD\x14a\x000W[`\0\x80\xFD[a\0\xA0`\x04\x806\x03` \x81\x10\x15a\0FW`\0\x80\xFD[\x81\x01\x90` \x81\x01\x815d\x01\0\0\0\0\x81\x11\x15a\0aW`\0\x80\xFD[\x82\x01\x83` \x82\x01\x11\x15a\0sW`\0\x80\xFD[\x805\x90` \x01\x91\x84` \x83\x02\x84\x01\x11d\x01\0\0\0\0\x83\x11\x17\x15a\0\x95W`\0\x80\xFD[P\x90\x92P\x90Pa\x019V[`@Q\x80\x80` \x01\x80` \x01\x83\x81\x03\x83R\x85\x81\x81Q\x81R` \x01\x91P\x80Q\x90` \x01\x90` \x02\x80\x83\x83`\0[\x83\x81\x10\x15a\0\xE4W\x81\x81\x01Q\x83\x82\x01R` \x01a\0\xCCV[PPPP\x90P\x01\x83\x81\x03\x82R\x84\x81\x81Q\x81R` \x01\x91P\x80Q\x90` \x01\x90` \x02\x80\x83\x83`\0[\x83\x81\x10\x15a\x01#W\x81\x81\x01Q\x83\x82\x01R` \x01a\x01\x0BV[PPPP\x90P\x01\x94PPPPP`@Q\x80\x91\x03\x90\xF3[`\0\x80T``\x91\x82\x91c\xFF\xFF\xFF\xFF\x16\x90\x85\x90\x85\x90\x81a\x01TW\xFE[\x90P` \x02\x015c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x14\x80\x15a\x01\x9CWP`\x01\x80Tc\xFF\xFF\xFF\xFF\x16\x90\x85\x90\x85\x90\x81\x81\x10a\x01\x87W\xFE[\x90P` \x02\x015c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x14[a\x01\xE1W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01RpInvalid test case`x\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`@\x80Q`\x02\x80\x82R``\x82\x01\x83R`\0\x92` \x83\x01\x90\x806\x837PP`\0\x80T\x83Q\x93\x94Pd\x01\0\0\0\0\x90\x04`\x06\x0B\x92\x84\x92Pa\x02\x1CW\xFE[`\x06\x92\x83\x0B\x83\x0B` \x91\x82\x02\x92\x90\x92\x01\x01R`\x01\x80T\x83Qd\x01\0\0\0\0\x90\x91\x04\x90\x92\x0B\x91\x83\x91\x90\x81\x10a\x02LW\xFE[`\x06\x92\x83\x0B\x90\x92\x0B` \x92\x83\x02\x91\x90\x91\x01\x82\x01R`@\x80Q`\x02\x80\x82R``\x82\x01\x83R`\0\x93\x91\x92\x90\x91\x83\x01\x90\x806\x837PP`\0\x80T\x83Q\x93\x94P`\x01`X\x1B\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x92\x84\x92Pa\x02\xA2W\xFE[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16` \x91\x82\x02\x92\x90\x92\x01\x01R`\x01\x80T\x83Q`\x01`X\x1B\x90\x91\x04\x90\x92\x16\x91\x83\x91\x90\x81\x10a\x02\xD5W\xFE[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x90\x92P\x90P\x92P\x92\x90PV\xFE\xA2dipfsX\"\x12 \xB3\x9A\xBD\xDE\xE0B\xFEs\x1D$F\x18V4\x9BB\xB36\xA8\xFC_1T3M\x15\xC9*x\xA7TJdsolcC\0\x07\x06\x003";
    /// The bytecode of the contract.
    pub static MOCKOBSERVABLE_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0+W`\x005`\xE0\x1C\x80c\x88;\xDB\xFD\x14a\x000W[`\0\x80\xFD[a\0\xA0`\x04\x806\x03` \x81\x10\x15a\0FW`\0\x80\xFD[\x81\x01\x90` \x81\x01\x815d\x01\0\0\0\0\x81\x11\x15a\0aW`\0\x80\xFD[\x82\x01\x83` \x82\x01\x11\x15a\0sW`\0\x80\xFD[\x805\x90` \x01\x91\x84` \x83\x02\x84\x01\x11d\x01\0\0\0\0\x83\x11\x17\x15a\0\x95W`\0\x80\xFD[P\x90\x92P\x90Pa\x019V[`@Q\x80\x80` \x01\x80` \x01\x83\x81\x03\x83R\x85\x81\x81Q\x81R` \x01\x91P\x80Q\x90` \x01\x90` \x02\x80\x83\x83`\0[\x83\x81\x10\x15a\0\xE4W\x81\x81\x01Q\x83\x82\x01R` \x01a\0\xCCV[PPPP\x90P\x01\x83\x81\x03\x82R\x84\x81\x81Q\x81R` \x01\x91P\x80Q\x90` \x01\x90` \x02\x80\x83\x83`\0[\x83\x81\x10\x15a\x01#W\x81\x81\x01Q\x83\x82\x01R` \x01a\x01\x0BV[PPPP\x90P\x01\x94PPPPP`@Q\x80\x91\x03\x90\xF3[`\0\x80T``\x91\x82\x91c\xFF\xFF\xFF\xFF\x16\x90\x85\x90\x85\x90\x81a\x01TW\xFE[\x90P` \x02\x015c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x14\x80\x15a\x01\x9CWP`\x01\x80Tc\xFF\xFF\xFF\xFF\x16\x90\x85\x90\x85\x90\x81\x81\x10a\x01\x87W\xFE[\x90P` \x02\x015c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x14[a\x01\xE1W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01RpInvalid test case`x\x1B`D\x82\x01R\x90Q\x90\x81\x90\x03`d\x01\x90\xFD[`@\x80Q`\x02\x80\x82R``\x82\x01\x83R`\0\x92` \x83\x01\x90\x806\x837PP`\0\x80T\x83Q\x93\x94Pd\x01\0\0\0\0\x90\x04`\x06\x0B\x92\x84\x92Pa\x02\x1CW\xFE[`\x06\x92\x83\x0B\x83\x0B` \x91\x82\x02\x92\x90\x92\x01\x01R`\x01\x80T\x83Qd\x01\0\0\0\0\x90\x91\x04\x90\x92\x0B\x91\x83\x91\x90\x81\x10a\x02LW\xFE[`\x06\x92\x83\x0B\x90\x92\x0B` \x92\x83\x02\x91\x90\x91\x01\x82\x01R`@\x80Q`\x02\x80\x82R``\x82\x01\x83R`\0\x93\x91\x92\x90\x91\x83\x01\x90\x806\x837PP`\0\x80T\x83Q\x93\x94P`\x01`X\x1B\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x92\x84\x92Pa\x02\xA2W\xFE[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16` \x91\x82\x02\x92\x90\x92\x01\x01R`\x01\x80T\x83Q`\x01`X\x1B\x90\x91\x04\x90\x92\x16\x91\x83\x91\x90\x81\x10a\x02\xD5W\xFE[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x90\x92P\x90P\x92P\x92\x90PV\xFE\xA2dipfsX\"\x12 \xB3\x9A\xBD\xDE\xE0B\xFEs\x1D$F\x18V4\x9BB\xB36\xA8\xFC_1T3M\x15\xC9*x\xA7TJdsolcC\0\x07\x06\x003";
    /// The deployed bytecode of the contract.
    pub static MOCKOBSERVABLE_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct MockObservable<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MockObservable<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MockObservable<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MockObservable<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MockObservable<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MockObservable))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MockObservable<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                MOCKOBSERVABLE_ABI.clone(),
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
                MOCKOBSERVABLE_ABI.clone(),
                MOCKOBSERVABLE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `observe` (0x883bdbfd) function
        pub fn observe(
            &self,
            seconds_agos: ::std::vec::Vec<u32>,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::std::vec::Vec<i64>,
                ::std::vec::Vec<::ethers::core::types::U256>,
            ),
        > {
            self.0
                .method_hash([136, 59, 219, 253], seconds_agos)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for MockObservable<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
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
        Hash,
    )]
    #[ethcall(name = "observe", abi = "observe(uint32[])")]
    pub struct ObserveCall {
        pub seconds_agos: ::std::vec::Vec<u32>,
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
        Hash,
    )]
    pub struct ObserveReturn {
        pub tick_cumulatives: ::std::vec::Vec<i64>,
        pub seconds_per_liquidity_cumulative_x12_8s: ::std::vec::Vec<::ethers::core::types::U256>,
    }
}
