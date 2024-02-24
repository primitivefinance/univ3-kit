pub use uniswap_interface_multicall::*;
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
pub mod uniswap_interface_multicall {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("getCurrentBlockTimestamp"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getCurrentBlockTimestamp",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("timestamp"),
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
                    ::std::borrow::ToOwned::to_owned("getEthBalance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getEthBalance"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("addr"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("balance"),
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
                    ::std::borrow::ToOwned::to_owned("multicall"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("multicall"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("calls"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct UniswapInterfaceMulticall.Call[]",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("blockNumber"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("returnData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct UniswapInterfaceMulticall.Result[]",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
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
    pub static UNISWAPINTERFACEMULTICALL_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x05\x0F\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0AW`\x005`\xE0\x1C\x80c\x0F(\xC9}\x14a\0FW\x80c\x17I\xE1\xE3\x14a\0dW\x80cM#\x01\xCC\x14a\0\x85W[`\0\x80\xFD[a\0Na\0\x98V[`@Qa\0[\x91\x90a\x03\xDAV[`@Q\x80\x91\x03\x90\xF3[a\0wa\0r6`\x04a\x02\x80V[a\0\x9CV[`@Qa\0[\x92\x91\x90a\x03\xE3V[a\0Na\0\x936`\x04a\x02_V[a\x02\x13V[B\x90V[\x80QC\x90``\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x80\x15a\0\xB9W`\0\x80\xFD[P`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\0\xF3W\x81` \x01[a\0\xE0a\x02 V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\0\xD8W\x90P[P\x90P`\0[\x83Q\x81\x10\x15a\x02\rW`\0\x80`\0\x86\x84\x81Q\x81\x10a\x01\x13W\xFE[` \x02` \x01\x01Q`\0\x01Q\x87\x85\x81Q\x81\x10a\x01+W\xFE[` \x02` \x01\x01Q` \x01Q\x88\x86\x81Q\x81\x10a\x01CW\xFE[` \x02` \x01\x01Q`@\x01Q\x92P\x92P\x92P`\0Z\x90P`\0\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x85\x85`@Qa\x01v\x91\x90a\x03\xBEV[`\0`@Q\x80\x83\x03\x81`\0\x87\x87\xF1\x92PPP=\x80`\0\x81\x14a\x01\xB4W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x01\xB9V[``\x91P[P\x91P\x91P`\0Z\x84\x03\x90P`@Q\x80``\x01`@R\x80\x84\x15\x15\x81R` \x01\x82\x81R` \x01\x83\x81RP\x89\x89\x81Q\x81\x10a\x01\xEEW\xFE[` \x02` \x01\x01\x81\x90RPPPPPPPP\x80\x80`\x01\x01\x91PPa\0\xF9V[P\x91P\x91V[`\x01`\x01`\xA0\x1B\x03\x161\x90V[`@Q\x80``\x01`@R\x80`\0\x15\x15\x81R` \x01`\0\x81R` \x01``\x81RP\x90V[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x02ZW`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x02pW\x80\x81\xFD[a\x02y\x82a\x02CV[\x93\x92PPPV[`\0` \x80\x83\x85\x03\x12\x15a\x02\x92W\x81\x82\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x02\xA9W\x83\x84\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x02\xBCW\x83\x84\xFD[\x815\x81\x81\x11\x15a\x02\xC8W\xFE[a\x02\xD5\x84\x85\x83\x02\x01a\x04\x85V[\x81\x81R\x84\x81\x01\x90\x84\x86\x01\x87[\x84\x81\x10\x15a\x03\xAFW\x815\x87\x01`\x1F\x19``\x82\x8E\x03\x82\x01\x12\x15a\x03\x01W\x8A\x8B\xFD[`@\x80Q``\x81\x01\x81\x81\x10\x8B\x82\x11\x17\x15a\x03\x17W\xFE[\x82Ra\x03$\x84\x8D\x01a\x02CV[\x81R\x81\x84\x015\x8C\x82\x01R``\x84\x015\x8A\x81\x11\x15a\x03?W\x8D\x8E\xFD[\x80\x85\x01\x94PP\x8E`?\x85\x01\x12a\x03SW\x8C\x8D\xFD[\x8B\x84\x015\x8A\x81\x11\x15a\x03aW\xFE[a\x03q\x8D\x85`\x1F\x84\x01\x16\x01a\x04\x85V[\x93P\x80\x84R\x8F\x83\x82\x87\x01\x01\x11\x15a\x03\x86W\x8D\x8E\xFD[\x80\x83\x86\x01\x8E\x86\x017\x83\x01\x8C\x01\x8D\x90R\x90\x81\x01\x91\x90\x91R\x85RP\x92\x87\x01\x92\x90\x87\x01\x90`\x01\x01a\x02\xE1V[P\x90\x99\x98PPPPPPPPPV[`\0\x82Qa\x03\xD0\x81\x84` \x87\x01a\x04\xA9V[\x91\x90\x91\x01\x92\x91PPV[\x90\x81R` \x01\x90V[`\0`@\x80\x83\x01\x85\x84R` \x82\x81\x86\x01R\x81\x86Q\x80\x84R``\x93P\x83\x87\x01\x91P\x83\x83\x82\x02\x88\x01\x01\x83\x89\x01\x87[\x83\x81\x10\x15a\x04uW\x89\x83\x03`_\x19\x01\x85R\x81Q\x80Q\x15\x15\x84R\x86\x81\x01Q\x87\x85\x01R\x88\x01Q\x88\x84\x01\x88\x90R\x80Q\x88\x85\x01\x81\x90R`\x80a\x04R\x82\x82\x88\x01\x85\x8C\x01a\x04\xA9V[\x96\x88\x01\x96`\x1F\x91\x90\x91\x01`\x1F\x19\x16\x94\x90\x94\x01\x90\x93\x01\x92P\x90\x85\x01\x90`\x01\x01a\x04\x0FV[P\x90\x9A\x99PPPPPPPPPPV[`@Q\x81\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x04\xA1W\xFE[`@R\x91\x90PV[`\0[\x83\x81\x10\x15a\x04\xC4W\x81\x81\x01Q\x83\x82\x01R` \x01a\x04\xACV[\x83\x81\x11\x15a\x04\xD3W`\0\x84\x84\x01R[PPPPV\xFE\xA2dipfsX\"\x12 \xC8K\x9C\xDC+\x95(\x89\xE8\x95a8]RM:\xE7\xC8\xD6\xE8;;\xC7\xC91\xF1\x17\xA7x\xDE\x96WdsolcC\0\x07\x06\x003";
    /// The bytecode of the contract.
    pub static UNISWAPINTERFACEMULTICALL_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0AW`\x005`\xE0\x1C\x80c\x0F(\xC9}\x14a\0FW\x80c\x17I\xE1\xE3\x14a\0dW\x80cM#\x01\xCC\x14a\0\x85W[`\0\x80\xFD[a\0Na\0\x98V[`@Qa\0[\x91\x90a\x03\xDAV[`@Q\x80\x91\x03\x90\xF3[a\0wa\0r6`\x04a\x02\x80V[a\0\x9CV[`@Qa\0[\x92\x91\x90a\x03\xE3V[a\0Na\0\x936`\x04a\x02_V[a\x02\x13V[B\x90V[\x80QC\x90``\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x80\x15a\0\xB9W`\0\x80\xFD[P`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\0\xF3W\x81` \x01[a\0\xE0a\x02 V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\0\xD8W\x90P[P\x90P`\0[\x83Q\x81\x10\x15a\x02\rW`\0\x80`\0\x86\x84\x81Q\x81\x10a\x01\x13W\xFE[` \x02` \x01\x01Q`\0\x01Q\x87\x85\x81Q\x81\x10a\x01+W\xFE[` \x02` \x01\x01Q` \x01Q\x88\x86\x81Q\x81\x10a\x01CW\xFE[` \x02` \x01\x01Q`@\x01Q\x92P\x92P\x92P`\0Z\x90P`\0\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x85\x85`@Qa\x01v\x91\x90a\x03\xBEV[`\0`@Q\x80\x83\x03\x81`\0\x87\x87\xF1\x92PPP=\x80`\0\x81\x14a\x01\xB4W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x01\xB9V[``\x91P[P\x91P\x91P`\0Z\x84\x03\x90P`@Q\x80``\x01`@R\x80\x84\x15\x15\x81R` \x01\x82\x81R` \x01\x83\x81RP\x89\x89\x81Q\x81\x10a\x01\xEEW\xFE[` \x02` \x01\x01\x81\x90RPPPPPPPP\x80\x80`\x01\x01\x91PPa\0\xF9V[P\x91P\x91V[`\x01`\x01`\xA0\x1B\x03\x161\x90V[`@Q\x80``\x01`@R\x80`\0\x15\x15\x81R` \x01`\0\x81R` \x01``\x81RP\x90V[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x02ZW`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x02pW\x80\x81\xFD[a\x02y\x82a\x02CV[\x93\x92PPPV[`\0` \x80\x83\x85\x03\x12\x15a\x02\x92W\x81\x82\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x02\xA9W\x83\x84\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x02\xBCW\x83\x84\xFD[\x815\x81\x81\x11\x15a\x02\xC8W\xFE[a\x02\xD5\x84\x85\x83\x02\x01a\x04\x85V[\x81\x81R\x84\x81\x01\x90\x84\x86\x01\x87[\x84\x81\x10\x15a\x03\xAFW\x815\x87\x01`\x1F\x19``\x82\x8E\x03\x82\x01\x12\x15a\x03\x01W\x8A\x8B\xFD[`@\x80Q``\x81\x01\x81\x81\x10\x8B\x82\x11\x17\x15a\x03\x17W\xFE[\x82Ra\x03$\x84\x8D\x01a\x02CV[\x81R\x81\x84\x015\x8C\x82\x01R``\x84\x015\x8A\x81\x11\x15a\x03?W\x8D\x8E\xFD[\x80\x85\x01\x94PP\x8E`?\x85\x01\x12a\x03SW\x8C\x8D\xFD[\x8B\x84\x015\x8A\x81\x11\x15a\x03aW\xFE[a\x03q\x8D\x85`\x1F\x84\x01\x16\x01a\x04\x85V[\x93P\x80\x84R\x8F\x83\x82\x87\x01\x01\x11\x15a\x03\x86W\x8D\x8E\xFD[\x80\x83\x86\x01\x8E\x86\x017\x83\x01\x8C\x01\x8D\x90R\x90\x81\x01\x91\x90\x91R\x85RP\x92\x87\x01\x92\x90\x87\x01\x90`\x01\x01a\x02\xE1V[P\x90\x99\x98PPPPPPPPPV[`\0\x82Qa\x03\xD0\x81\x84` \x87\x01a\x04\xA9V[\x91\x90\x91\x01\x92\x91PPV[\x90\x81R` \x01\x90V[`\0`@\x80\x83\x01\x85\x84R` \x82\x81\x86\x01R\x81\x86Q\x80\x84R``\x93P\x83\x87\x01\x91P\x83\x83\x82\x02\x88\x01\x01\x83\x89\x01\x87[\x83\x81\x10\x15a\x04uW\x89\x83\x03`_\x19\x01\x85R\x81Q\x80Q\x15\x15\x84R\x86\x81\x01Q\x87\x85\x01R\x88\x01Q\x88\x84\x01\x88\x90R\x80Q\x88\x85\x01\x81\x90R`\x80a\x04R\x82\x82\x88\x01\x85\x8C\x01a\x04\xA9V[\x96\x88\x01\x96`\x1F\x91\x90\x91\x01`\x1F\x19\x16\x94\x90\x94\x01\x90\x93\x01\x92P\x90\x85\x01\x90`\x01\x01a\x04\x0FV[P\x90\x9A\x99PPPPPPPPPPV[`@Q\x81\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x04\xA1W\xFE[`@R\x91\x90PV[`\0[\x83\x81\x10\x15a\x04\xC4W\x81\x81\x01Q\x83\x82\x01R` \x01a\x04\xACV[\x83\x81\x11\x15a\x04\xD3W`\0\x84\x84\x01R[PPPPV\xFE\xA2dipfsX\"\x12 \xC8K\x9C\xDC+\x95(\x89\xE8\x95a8]RM:\xE7\xC8\xD6\xE8;;\xC7\xC91\xF1\x17\xA7x\xDE\x96WdsolcC\0\x07\x06\x003";
    /// The deployed bytecode of the contract.
    pub static UNISWAPINTERFACEMULTICALL_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct UniswapInterfaceMulticall<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for UniswapInterfaceMulticall<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for UniswapInterfaceMulticall<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for UniswapInterfaceMulticall<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for UniswapInterfaceMulticall<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(UniswapInterfaceMulticall))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> UniswapInterfaceMulticall<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    UNISWAPINTERFACEMULTICALL_ABI.clone(),
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
                UNISWAPINTERFACEMULTICALL_ABI.clone(),
                UNISWAPINTERFACEMULTICALL_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `getCurrentBlockTimestamp` (0x0f28c97d) function
        pub fn get_current_block_timestamp(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([15, 40, 201, 125], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getEthBalance` (0x4d2301cc) function
        pub fn get_eth_balance(
            &self,
            addr: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([77, 35, 1, 204], addr)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `multicall` (0x1749e1e3) function
        pub fn multicall(
            &self,
            calls: ::std::vec::Vec<Call>,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::std::vec::Vec<Result>),
        > {
            self.0
                .method_hash([23, 73, 225, 227], calls)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for UniswapInterfaceMulticall<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `getCurrentBlockTimestamp` function with signature `getCurrentBlockTimestamp()` and selector `0x0f28c97d`
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
    #[ethcall(name = "getCurrentBlockTimestamp", abi = "getCurrentBlockTimestamp()")]
    pub struct GetCurrentBlockTimestampCall;
    ///Container type for all input parameters for the `getEthBalance` function with signature `getEthBalance(address)` and selector `0x4d2301cc`
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
    #[ethcall(name = "getEthBalance", abi = "getEthBalance(address)")]
    pub struct GetEthBalanceCall {
        pub addr: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `multicall` function with signature `multicall((address,uint256,bytes)[])` and selector `0x1749e1e3`
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
    #[ethcall(name = "multicall", abi = "multicall((address,uint256,bytes)[])")]
    pub struct MulticallCall {
        pub calls: ::std::vec::Vec<Call>,
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
    pub enum UniswapInterfaceMulticallCalls {
        GetCurrentBlockTimestamp(GetCurrentBlockTimestampCall),
        GetEthBalance(GetEthBalanceCall),
        Multicall(MulticallCall),
    }
    impl ::ethers::core::abi::AbiDecode for UniswapInterfaceMulticallCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <GetCurrentBlockTimestampCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetCurrentBlockTimestamp(decoded));
            }
            if let Ok(decoded) = <GetEthBalanceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetEthBalance(decoded));
            }
            if let Ok(decoded) = <MulticallCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Multicall(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for UniswapInterfaceMulticallCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::GetCurrentBlockTimestamp(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetEthBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Multicall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for UniswapInterfaceMulticallCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::GetCurrentBlockTimestamp(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetEthBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::Multicall(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<GetCurrentBlockTimestampCall>
    for UniswapInterfaceMulticallCalls {
        fn from(value: GetCurrentBlockTimestampCall) -> Self {
            Self::GetCurrentBlockTimestamp(value)
        }
    }
    impl ::core::convert::From<GetEthBalanceCall> for UniswapInterfaceMulticallCalls {
        fn from(value: GetEthBalanceCall) -> Self {
            Self::GetEthBalance(value)
        }
    }
    impl ::core::convert::From<MulticallCall> for UniswapInterfaceMulticallCalls {
        fn from(value: MulticallCall) -> Self {
            Self::Multicall(value)
        }
    }
    ///Container type for all return fields from the `getCurrentBlockTimestamp` function with signature `getCurrentBlockTimestamp()` and selector `0x0f28c97d`
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
    pub struct GetCurrentBlockTimestampReturn {
        pub timestamp: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getEthBalance` function with signature `getEthBalance(address)` and selector `0x4d2301cc`
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
    pub struct GetEthBalanceReturn {
        pub balance: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `multicall` function with signature `multicall((address,uint256,bytes)[])` and selector `0x1749e1e3`
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
        pub block_number: ::ethers::core::types::U256,
        pub return_data: ::std::vec::Vec<Result>,
    }
    ///`Call(address,uint256,bytes)`
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
    pub struct Call {
        pub target: ::ethers::core::types::Address,
        pub gas_limit: ::ethers::core::types::U256,
        pub call_data: ::ethers::core::types::Bytes,
    }
    ///`Result(bool,uint256,bytes)`
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
    pub struct Result {
        pub success: bool,
        pub gas_used: ::ethers::core::types::U256,
        pub return_data: ::ethers::core::types::Bytes,
    }
}
