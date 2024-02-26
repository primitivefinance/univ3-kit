pub use tick_lens::*;
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
pub mod tick_lens {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([(
                ::std::borrow::ToOwned::to_owned("getPopulatedTicksInWord"),
                ::std::vec![::ethers::core::abi::ethabi::Function {
                    name: ::std::borrow::ToOwned::to_owned("getPopulatedTicksInWord",),
                    inputs: ::std::vec![
                        ::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("pool"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },
                        ::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("tickBitmapIndex"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Int(16usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("int16"),
                            ),
                        },
                    ],
                    outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("populatedTicks"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Array(
                            ::std::boxed::Box::new(::ethers::core::abi::ethabi::ParamType::Tuple(
                                ::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Int(24usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                ],
                            ),),
                        ),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("struct ITickLens.PopulatedTick[]",),
                        ),
                    },],
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
    pub static TICKLENS_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x05*\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0+W`\x005`\xE0\x1C\x80c5\x1F\xB4x\x14a\x000W[`\0\x80\xFD[a\0Ca\0>6`\x04a\x033V[a\0YV[`@Qa\0P\x91\x90a\x04XV[`@Q\x80\x91\x03\x90\xF3[```\0\x83`\x01`\x01`\xA0\x1B\x03\x16cS9\xC2\x96\x84`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\0\x89\x91\x90a\x04\xC0V[` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\0\xA1W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\0\xB5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\0\xD9\x91\x90a\x04@V[\x90P`\0\x80[a\x01\0\x81\x10\x15a\x01\x03W`\x01\x81\x1B\x83\x16\x15a\0\xFBW`\x01\x90\x91\x01\x90[`\x01\x01a\0\xDFV[P`\0\x85`\x01`\x01`\xA0\x1B\x03\x16c\xD0\xC9:|`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x01?W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x01SW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01w\x91\x90a\x03qV[\x90P\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x80\x15a\x01\x90W`\0\x80\xFD[P`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x01\xCAW\x81` \x01[a\x01\xB7a\x02\xDFV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x01\xAFW\x90P[P\x93P`\0[a\x01\0\x81\x10\x15a\x02\xD5W`\x01\x81\x1B\x84\x16\x15a\x02\xCDW`@Qc\xF3\r\xBA\x93`\xE0\x1B\x81R`\x01\x87\x90\x0B`\x02\x0B`\x08\x1B\x82\x01\x83\x02\x90`\0\x90\x81\x90`\x01`\x01`\xA0\x1B\x03\x8B\x16\x90c\xF3\r\xBA\x93\x90a\x02&\x90\x86\x90`\x04\x01a\x04\xCEV[a\x01\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x02?W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x02SW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02w\x91\x90a\x03\x99V[PPPPPP\x91P\x91P`@Q\x80``\x01`@R\x80\x84`\x02\x0B\x81R` \x01\x82`\x0F\x0B\x81R` \x01\x83`\x01`\x01`\x80\x1B\x03\x16\x81RP\x88\x87`\x01\x90\x03\x97P\x87\x81Q\x81\x10a\x02\xBEW\xFE[` \x02` \x01\x01\x81\x90RPPPP[`\x01\x01a\x01\xD0V[PPPP\x92\x91PPV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R\x90V[\x80Q\x80\x15\x15\x81\x14a\x03\x0FW`\0\x80\xFD[\x91\x90PV[\x80Qa\x03\x0F\x81a\x04\xDCV[\x80Qc\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x03\x0FW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x03EW\x81\x82\xFD[\x825a\x03P\x81a\x04\xDCV[\x91P` \x83\x015`\x01\x81\x90\x0B\x81\x14a\x03fW\x81\x82\xFD[\x80\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x03\x82W\x80\x81\xFD[\x81Q\x80`\x02\x0B\x81\x14a\x03\x92W\x81\x82\xFD[\x93\x92PPPV[`\0\x80`\0\x80`\0\x80`\0\x80a\x01\0\x89\x8B\x03\x12\x15a\x03\xB5W\x83\x84\xFD[\x88Q`\x01`\x01`\x80\x1B\x03\x81\x16\x81\x14a\x03\xCBW\x84\x85\xFD[\x80\x98PP` \x89\x01Q\x80`\x0F\x0B\x81\x14a\x03\xE2W\x84\x85\xFD[\x80\x97PP`@\x89\x01Q\x95P``\x89\x01Q\x94P`\x80\x89\x01Q\x80`\x06\x0B\x81\x14a\x04\x07W\x84\x85\xFD[\x93Pa\x04\x15`\xA0\x8A\x01a\x03\x14V[\x92Pa\x04#`\xC0\x8A\x01a\x03\x1FV[\x91Pa\x041`\xE0\x8A\x01a\x02\xFFV[\x90P\x92\x95\x98P\x92\x95\x98\x90\x93\x96PV[`\0` \x82\x84\x03\x12\x15a\x04QW\x80\x81\xFD[PQ\x91\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90`@\x90\x81\x85\x01\x90\x86\x84\x01\x85[\x82\x81\x10\x15a\x04\xB3W\x81Q\x80Q`\x02\x0B\x85R\x86\x81\x01Q`\x0F\x0B\x87\x86\x01R\x85\x01Q`\x01`\x01`\x80\x1B\x03\x16\x85\x85\x01R``\x90\x93\x01\x92\x90\x85\x01\x90`\x01\x01a\x04uV[P\x91\x97\x96PPPPPPPV[`\x01\x91\x90\x91\x0B\x81R` \x01\x90V[`\x02\x91\x90\x91\x0B\x81R` \x01\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x04\xF1W`\0\x80\xFD[PV\xFE\xA2dipfsX\"\x12 \x1A\xD3\xCE\xA2\x9A\x8C\xAB\xB2\x88\xE3\xC4p\x81&&V\t\x10\x1D\xD7[\x1C\xB9\x02\xE2\x14\xDC0\xFF\xCC\x92\xDAdsolcC\0\x07\x06\x003";
    /// The bytecode of the contract.
    pub static TICKLENS_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0+W`\x005`\xE0\x1C\x80c5\x1F\xB4x\x14a\x000W[`\0\x80\xFD[a\0Ca\0>6`\x04a\x033V[a\0YV[`@Qa\0P\x91\x90a\x04XV[`@Q\x80\x91\x03\x90\xF3[```\0\x83`\x01`\x01`\xA0\x1B\x03\x16cS9\xC2\x96\x84`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\0\x89\x91\x90a\x04\xC0V[` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\0\xA1W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\0\xB5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\0\xD9\x91\x90a\x04@V[\x90P`\0\x80[a\x01\0\x81\x10\x15a\x01\x03W`\x01\x81\x1B\x83\x16\x15a\0\xFBW`\x01\x90\x91\x01\x90[`\x01\x01a\0\xDFV[P`\0\x85`\x01`\x01`\xA0\x1B\x03\x16c\xD0\xC9:|`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x01?W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x01SW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01w\x91\x90a\x03qV[\x90P\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x80\x15a\x01\x90W`\0\x80\xFD[P`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x01\xCAW\x81` \x01[a\x01\xB7a\x02\xDFV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x01\xAFW\x90P[P\x93P`\0[a\x01\0\x81\x10\x15a\x02\xD5W`\x01\x81\x1B\x84\x16\x15a\x02\xCDW`@Qc\xF3\r\xBA\x93`\xE0\x1B\x81R`\x01\x87\x90\x0B`\x02\x0B`\x08\x1B\x82\x01\x83\x02\x90`\0\x90\x81\x90`\x01`\x01`\xA0\x1B\x03\x8B\x16\x90c\xF3\r\xBA\x93\x90a\x02&\x90\x86\x90`\x04\x01a\x04\xCEV[a\x01\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x02?W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x02SW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02w\x91\x90a\x03\x99V[PPPPPP\x91P\x91P`@Q\x80``\x01`@R\x80\x84`\x02\x0B\x81R` \x01\x82`\x0F\x0B\x81R` \x01\x83`\x01`\x01`\x80\x1B\x03\x16\x81RP\x88\x87`\x01\x90\x03\x97P\x87\x81Q\x81\x10a\x02\xBEW\xFE[` \x02` \x01\x01\x81\x90RPPPP[`\x01\x01a\x01\xD0V[PPPP\x92\x91PPV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R\x90V[\x80Q\x80\x15\x15\x81\x14a\x03\x0FW`\0\x80\xFD[\x91\x90PV[\x80Qa\x03\x0F\x81a\x04\xDCV[\x80Qc\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x03\x0FW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x03EW\x81\x82\xFD[\x825a\x03P\x81a\x04\xDCV[\x91P` \x83\x015`\x01\x81\x90\x0B\x81\x14a\x03fW\x81\x82\xFD[\x80\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x03\x82W\x80\x81\xFD[\x81Q\x80`\x02\x0B\x81\x14a\x03\x92W\x81\x82\xFD[\x93\x92PPPV[`\0\x80`\0\x80`\0\x80`\0\x80a\x01\0\x89\x8B\x03\x12\x15a\x03\xB5W\x83\x84\xFD[\x88Q`\x01`\x01`\x80\x1B\x03\x81\x16\x81\x14a\x03\xCBW\x84\x85\xFD[\x80\x98PP` \x89\x01Q\x80`\x0F\x0B\x81\x14a\x03\xE2W\x84\x85\xFD[\x80\x97PP`@\x89\x01Q\x95P``\x89\x01Q\x94P`\x80\x89\x01Q\x80`\x06\x0B\x81\x14a\x04\x07W\x84\x85\xFD[\x93Pa\x04\x15`\xA0\x8A\x01a\x03\x14V[\x92Pa\x04#`\xC0\x8A\x01a\x03\x1FV[\x91Pa\x041`\xE0\x8A\x01a\x02\xFFV[\x90P\x92\x95\x98P\x92\x95\x98\x90\x93\x96PV[`\0` \x82\x84\x03\x12\x15a\x04QW\x80\x81\xFD[PQ\x91\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90`@\x90\x81\x85\x01\x90\x86\x84\x01\x85[\x82\x81\x10\x15a\x04\xB3W\x81Q\x80Q`\x02\x0B\x85R\x86\x81\x01Q`\x0F\x0B\x87\x86\x01R\x85\x01Q`\x01`\x01`\x80\x1B\x03\x16\x85\x85\x01R``\x90\x93\x01\x92\x90\x85\x01\x90`\x01\x01a\x04uV[P\x91\x97\x96PPPPPPPV[`\x01\x91\x90\x91\x0B\x81R` \x01\x90V[`\x02\x91\x90\x91\x0B\x81R` \x01\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x04\xF1W`\0\x80\xFD[PV\xFE\xA2dipfsX\"\x12 \x1A\xD3\xCE\xA2\x9A\x8C\xAB\xB2\x88\xE3\xC4p\x81&&V\t\x10\x1D\xD7[\x1C\xB9\x02\xE2\x14\xDC0\xFF\xCC\x92\xDAdsolcC\0\x07\x06\x003";
    /// The deployed bytecode of the contract.
    pub static TICKLENS_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct TickLens<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for TickLens<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for TickLens<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for TickLens<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for TickLens<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(TickLens))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> TickLens<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                TICKLENS_ABI.clone(),
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
                TICKLENS_ABI.clone(),
                TICKLENS_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `getPopulatedTicksInWord` (0x351fb478) function
        pub fn get_populated_ticks_in_word(
            &self,
            pool: ::ethers::core::types::Address,
            tick_bitmap_index: i16,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<PopulatedTick>> {
            self.0
                .method_hash([53, 31, 180, 120], (pool, tick_bitmap_index))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for TickLens<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `getPopulatedTicksInWord` function with signature `getPopulatedTicksInWord(address,int16)` and selector `0x351fb478`
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
        name = "getPopulatedTicksInWord",
        abi = "getPopulatedTicksInWord(address,int16)"
    )]
    pub struct GetPopulatedTicksInWordCall {
        pub pool: ::ethers::core::types::Address,
        pub tick_bitmap_index: i16,
    }
    ///Container type for all return fields from the `getPopulatedTicksInWord` function with signature `getPopulatedTicksInWord(address,int16)` and selector `0x351fb478`
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
    pub struct GetPopulatedTicksInWordReturn {
        pub populated_ticks: ::std::vec::Vec<PopulatedTick>,
    }
}
