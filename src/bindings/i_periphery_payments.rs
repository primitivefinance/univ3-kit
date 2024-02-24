pub use i_periphery_payments::*;
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
pub mod i_periphery_payments {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("refundETH"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("refundETH"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("sweepToken"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
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
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("unwrapWETH9"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("unwrapWETH9"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountMinimum"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
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
    pub static IPERIPHERYPAYMENTS_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    pub struct IPeripheryPayments<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IPeripheryPayments<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IPeripheryPayments<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IPeripheryPayments<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IPeripheryPayments<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IPeripheryPayments))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IPeripheryPayments<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IPERIPHERYPAYMENTS_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `refundETH` (0x12210e8a) function
        pub fn refund_eth(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([18, 33, 14, 138], ())
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
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IPeripheryPayments<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
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
        Hash
    )]
    #[ethcall(name = "refundETH", abi = "refundETH()")]
    pub struct RefundETHCall;
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
        Hash
    )]
    #[ethcall(name = "sweepToken", abi = "sweepToken(address,uint256,address)")]
    pub struct SweepTokenCall {
        pub token: ::ethers::core::types::Address,
        pub amount_minimum: ::ethers::core::types::U256,
        pub recipient: ::ethers::core::types::Address,
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
        Hash
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
        Hash
    )]
    pub enum IPeripheryPaymentsCalls {
        RefundETH(RefundETHCall),
        SweepToken(SweepTokenCall),
        UnwrapWETH9(UnwrapWETH9Call),
    }
    impl ::ethers::core::abi::AbiDecode for IPeripheryPaymentsCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <RefundETHCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RefundETH(decoded));
            }
            if let Ok(decoded) = <SweepTokenCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SweepToken(decoded));
            }
            if let Ok(decoded) = <UnwrapWETH9Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UnwrapWETH9(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IPeripheryPaymentsCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::RefundETH(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SweepToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnwrapWETH9(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for IPeripheryPaymentsCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::RefundETH(element) => ::core::fmt::Display::fmt(element, f),
                Self::SweepToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnwrapWETH9(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<RefundETHCall> for IPeripheryPaymentsCalls {
        fn from(value: RefundETHCall) -> Self {
            Self::RefundETH(value)
        }
    }
    impl ::core::convert::From<SweepTokenCall> for IPeripheryPaymentsCalls {
        fn from(value: SweepTokenCall) -> Self {
            Self::SweepToken(value)
        }
    }
    impl ::core::convert::From<UnwrapWETH9Call> for IPeripheryPaymentsCalls {
        fn from(value: UnwrapWETH9Call) -> Self {
            Self::UnwrapWETH9(value)
        }
    }
}
