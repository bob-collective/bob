///Module containing a contract's types and functions.
/**

```solidity
library IStakeManager {
    struct DepositInfo { uint112 deposit; bool staked; uint112 stake; uint32 unstakeDelaySec; uint48 withdrawTime; }
    struct StakeInfo { uint256 stake; uint256 unstakeDelaySec; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style
)]
pub mod IStakeManager {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
struct DepositInfo { uint112 deposit; bool staked; uint112 stake; uint32 unstakeDelaySec; uint48 withdrawTime; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DepositInfo {
        pub deposit: alloy::sol_types::private::primitives::aliases::U112,
        pub staked: bool,
        pub stake: alloy::sol_types::private::primitives::aliases::U112,
        pub unstakeDelaySec: u32,
        pub withdrawTime: alloy::sol_types::private::primitives::aliases::U48,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<112>,
            alloy::sol_types::sol_data::Bool,
            alloy::sol_types::sol_data::Uint<112>,
            alloy::sol_types::sol_data::Uint<32>,
            alloy::sol_types::sol_data::Uint<48>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U112,
            bool,
            alloy::sol_types::private::primitives::aliases::U112,
            u32,
            alloy::sol_types::private::primitives::aliases::U48,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<DepositInfo> for UnderlyingRustTuple<'_> {
            fn from(value: DepositInfo) -> Self {
                (
                    value.deposit,
                    value.staked,
                    value.stake,
                    value.unstakeDelaySec,
                    value.withdrawTime,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for DepositInfo {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    deposit: tuple.0,
                    staked: tuple.1,
                    stake: tuple.2,
                    unstakeDelaySec: tuple.3,
                    withdrawTime: tuple.4,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for DepositInfo {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for DepositInfo {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        112,
                    > as alloy_sol_types::SolType>::tokenize(&self.deposit),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.staked,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        112,
                    > as alloy_sol_types::SolType>::tokenize(&self.stake),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.unstakeDelaySec),
                    <alloy::sol_types::sol_data::Uint<
                        48,
                    > as alloy_sol_types::SolType>::tokenize(&self.withdrawTime),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for DepositInfo {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for DepositInfo {
            const NAME: &'static str = "DepositInfo";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "DepositInfo(uint112 deposit,bool staked,uint112 stake,uint32 unstakeDelaySec,uint48 withdrawTime)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Uint<
                        112,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.deposit)
                        .0,
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::eip712_data_word(
                            &self.staked,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        112,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.stake)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.unstakeDelaySec,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        48,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.withdrawTime)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for DepositInfo {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        112,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.deposit,
                    )
                    + <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.staked,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        112,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.stake)
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.unstakeDelaySec,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        48,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.withdrawTime,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Uint<
                    112,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.deposit,
                    out,
                );
                <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.staked,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    112,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.stake,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.unstakeDelaySec,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    48,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.withdrawTime,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    /**```solidity
struct StakeInfo { uint256 stake; uint256 unstakeDelaySec; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct StakeInfo {
        pub stake: alloy::sol_types::private::primitives::aliases::U256,
        pub unstakeDelaySec: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<StakeInfo> for UnderlyingRustTuple<'_> {
            fn from(value: StakeInfo) -> Self {
                (value.stake, value.unstakeDelaySec)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for StakeInfo {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    stake: tuple.0,
                    unstakeDelaySec: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for StakeInfo {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for StakeInfo {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.stake),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.unstakeDelaySec),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for StakeInfo {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for StakeInfo {
            const NAME: &'static str = "StakeInfo";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "StakeInfo(uint256 stake,uint256 unstakeDelaySec)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.stake)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.unstakeDelaySec,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for StakeInfo {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.stake)
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.unstakeDelaySec,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.stake,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.unstakeDelaySec,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`IStakeManager`](self) contract instance.

See the [wrapper's documentation](`IStakeManagerInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IStakeManagerInstance<T, P, N> {
        IStakeManagerInstance::<T, P, N>::new(address, provider)
    }
    /**A [`IStakeManager`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`IStakeManager`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IStakeManagerInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for IStakeManagerInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IStakeManagerInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IStakeManagerInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`IStakeManager`](self) contract instance.

See the [wrapper's documentation](`IStakeManagerInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            provider: P,
        ) -> Self {
            Self {
                address,
                provider,
                _network_transport: ::core::marker::PhantomData,
            }
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<T, P: ::core::clone::Clone, N> IStakeManagerInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IStakeManagerInstance<T, P, N> {
            IStakeManagerInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IStakeManagerInstance<T, P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IStakeManagerInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
/**

Generated by the following Solidity interface...
```solidity
library IStakeManager {
    struct DepositInfo {
        uint112 deposit;
        bool staked;
        uint112 stake;
        uint32 unstakeDelaySec;
        uint48 withdrawTime;
    }
    struct StakeInfo {
        uint256 stake;
        uint256 unstakeDelaySec;
    }
}

interface IEntryPoint {
    struct AggregatorStakeInfo {
        address aggregator;
        IStakeManager.StakeInfo stakeInfo;
    }
    struct ReturnInfo {
        uint256 preOpGas;
        uint256 prefund;
        bool sigFailed;
        uint48 validAfter;
        uint48 validUntil;
        bytes paymasterContext;
    }
    struct UserOperation {
        address sender;
        uint256 nonce;
        bytes initCode;
        bytes callData;
        uint256 callGasLimit;
        uint256 verificationGasLimit;
        uint256 preVerificationGas;
        uint256 maxFeePerGas;
        uint256 maxPriorityFeePerGas;
        bytes paymasterAndData;
        bytes signature;
    }
    struct UserOpsPerAggregator {
        UserOperation[] userOps;
        address aggregator;
        bytes signature;
    }

    error ExecutionResult(uint256 preOpGas, uint256 paid, uint48 validAfter, uint48 validUntil, bool targetSuccess, bytes targetResult);
    error FailedOp(uint256 opIndex, string reason);
    error SenderAddressResult(address sender);
    error SignatureValidationFailed(address aggregator);
    error ValidationResult(ReturnInfo returnInfo, IStakeManager.StakeInfo senderInfo, IStakeManager.StakeInfo factoryInfo, IStakeManager.StakeInfo paymasterInfo);
    error ValidationResultWithAggregation(ReturnInfo returnInfo, IStakeManager.StakeInfo senderInfo, IStakeManager.StakeInfo factoryInfo, IStakeManager.StakeInfo paymasterInfo, AggregatorStakeInfo aggregatorInfo);

    event AccountDeployed(bytes32 indexed userOpHash, address indexed sender, address factory, address paymaster);
    event BeforeExecution();
    event Deposited(address indexed account, uint256 totalDeposit);
    event SignatureAggregatorChanged(address indexed aggregator);
    event StakeLocked(address indexed account, uint256 totalStaked, uint256 unstakeDelaySec);
    event StakeUnlocked(address indexed account, uint256 withdrawTime);
    event StakeWithdrawn(address indexed account, address withdrawAddress, uint256 amount);
    event UserOperationEvent(bytes32 indexed userOpHash, address indexed sender, address indexed paymaster, uint256 nonce, bool success, uint256 actualGasCost, uint256 actualGasUsed);
    event UserOperationRevertReason(bytes32 indexed userOpHash, address indexed sender, uint256 nonce, bytes revertReason);
    event Withdrawn(address indexed account, address withdrawAddress, uint256 amount);

    function addStake(uint32 _unstakeDelaySec) external payable;
    function balanceOf(address account) external view returns (uint256);
    function depositTo(address account) external payable;
    function getDepositInfo(address account) external view returns (IStakeManager.DepositInfo memory info);
    function getNonce(address sender, uint192 key) external view returns (uint256 nonce);
    function getSenderAddress(bytes memory initCode) external;
    function getUserOpHash(UserOperation memory userOp) external view returns (bytes32);
    function handleAggregatedOps(UserOpsPerAggregator[] memory opsPerAggregator, address payable beneficiary) external;
    function handleOps(UserOperation[] memory ops, address payable beneficiary) external;
    function incrementNonce(uint192 key) external;
    function simulateHandleOp(UserOperation memory op, address target, bytes memory targetCallData) external;
    function simulateValidation(UserOperation memory userOp) external;
    function unlockStake() external;
    function withdrawStake(address payable withdrawAddress) external;
    function withdrawTo(address payable withdrawAddress, uint256 withdrawAmount) external;
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "function",
    "name": "addStake",
    "inputs": [
      {
        "name": "_unstakeDelaySec",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "balanceOf",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "depositTo",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "getDepositInfo",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "info",
        "type": "tuple",
        "internalType": "struct IStakeManager.DepositInfo",
        "components": [
          {
            "name": "deposit",
            "type": "uint112",
            "internalType": "uint112"
          },
          {
            "name": "staked",
            "type": "bool",
            "internalType": "bool"
          },
          {
            "name": "stake",
            "type": "uint112",
            "internalType": "uint112"
          },
          {
            "name": "unstakeDelaySec",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "withdrawTime",
            "type": "uint48",
            "internalType": "uint48"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getNonce",
    "inputs": [
      {
        "name": "sender",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "key",
        "type": "uint192",
        "internalType": "uint192"
      }
    ],
    "outputs": [
      {
        "name": "nonce",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getSenderAddress",
    "inputs": [
      {
        "name": "initCode",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getUserOpHash",
    "inputs": [
      {
        "name": "userOp",
        "type": "tuple",
        "internalType": "struct UserOperation",
        "components": [
          {
            "name": "sender",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "nonce",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "initCode",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "callData",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "callGasLimit",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "verificationGasLimit",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "preVerificationGas",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "maxFeePerGas",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "maxPriorityFeePerGas",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "paymasterAndData",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "signature",
            "type": "bytes",
            "internalType": "bytes"
          }
        ]
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "handleAggregatedOps",
    "inputs": [
      {
        "name": "opsPerAggregator",
        "type": "tuple[]",
        "internalType": "struct IEntryPoint.UserOpsPerAggregator[]",
        "components": [
          {
            "name": "userOps",
            "type": "tuple[]",
            "internalType": "struct UserOperation[]",
            "components": [
              {
                "name": "sender",
                "type": "address",
                "internalType": "address"
              },
              {
                "name": "nonce",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "initCode",
                "type": "bytes",
                "internalType": "bytes"
              },
              {
                "name": "callData",
                "type": "bytes",
                "internalType": "bytes"
              },
              {
                "name": "callGasLimit",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "verificationGasLimit",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "preVerificationGas",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "maxFeePerGas",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "maxPriorityFeePerGas",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "paymasterAndData",
                "type": "bytes",
                "internalType": "bytes"
              },
              {
                "name": "signature",
                "type": "bytes",
                "internalType": "bytes"
              }
            ]
          },
          {
            "name": "aggregator",
            "type": "address",
            "internalType": "contract IAggregator"
          },
          {
            "name": "signature",
            "type": "bytes",
            "internalType": "bytes"
          }
        ]
      },
      {
        "name": "beneficiary",
        "type": "address",
        "internalType": "address payable"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "handleOps",
    "inputs": [
      {
        "name": "ops",
        "type": "tuple[]",
        "internalType": "struct UserOperation[]",
        "components": [
          {
            "name": "sender",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "nonce",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "initCode",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "callData",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "callGasLimit",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "verificationGasLimit",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "preVerificationGas",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "maxFeePerGas",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "maxPriorityFeePerGas",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "paymasterAndData",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "signature",
            "type": "bytes",
            "internalType": "bytes"
          }
        ]
      },
      {
        "name": "beneficiary",
        "type": "address",
        "internalType": "address payable"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "incrementNonce",
    "inputs": [
      {
        "name": "key",
        "type": "uint192",
        "internalType": "uint192"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "simulateHandleOp",
    "inputs": [
      {
        "name": "op",
        "type": "tuple",
        "internalType": "struct UserOperation",
        "components": [
          {
            "name": "sender",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "nonce",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "initCode",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "callData",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "callGasLimit",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "verificationGasLimit",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "preVerificationGas",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "maxFeePerGas",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "maxPriorityFeePerGas",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "paymasterAndData",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "signature",
            "type": "bytes",
            "internalType": "bytes"
          }
        ]
      },
      {
        "name": "target",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "targetCallData",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "simulateValidation",
    "inputs": [
      {
        "name": "userOp",
        "type": "tuple",
        "internalType": "struct UserOperation",
        "components": [
          {
            "name": "sender",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "nonce",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "initCode",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "callData",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "callGasLimit",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "verificationGasLimit",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "preVerificationGas",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "maxFeePerGas",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "maxPriorityFeePerGas",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "paymasterAndData",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "signature",
            "type": "bytes",
            "internalType": "bytes"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "unlockStake",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "withdrawStake",
    "inputs": [
      {
        "name": "withdrawAddress",
        "type": "address",
        "internalType": "address payable"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "withdrawTo",
    "inputs": [
      {
        "name": "withdrawAddress",
        "type": "address",
        "internalType": "address payable"
      },
      {
        "name": "withdrawAmount",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "event",
    "name": "AccountDeployed",
    "inputs": [
      {
        "name": "userOpHash",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "sender",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "factory",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "paymaster",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "BeforeExecution",
    "inputs": [],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Deposited",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "totalDeposit",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "SignatureAggregatorChanged",
    "inputs": [
      {
        "name": "aggregator",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "StakeLocked",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "totalStaked",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "unstakeDelaySec",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "StakeUnlocked",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "withdrawTime",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "StakeWithdrawn",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "withdrawAddress",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "amount",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "UserOperationEvent",
    "inputs": [
      {
        "name": "userOpHash",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "sender",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "paymaster",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "nonce",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "success",
        "type": "bool",
        "indexed": false,
        "internalType": "bool"
      },
      {
        "name": "actualGasCost",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "actualGasUsed",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "UserOperationRevertReason",
    "inputs": [
      {
        "name": "userOpHash",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "sender",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "nonce",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "revertReason",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Withdrawn",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "withdrawAddress",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "amount",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "error",
    "name": "ExecutionResult",
    "inputs": [
      {
        "name": "preOpGas",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "paid",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "validAfter",
        "type": "uint48",
        "internalType": "uint48"
      },
      {
        "name": "validUntil",
        "type": "uint48",
        "internalType": "uint48"
      },
      {
        "name": "targetSuccess",
        "type": "bool",
        "internalType": "bool"
      },
      {
        "name": "targetResult",
        "type": "bytes",
        "internalType": "bytes"
      }
    ]
  },
  {
    "type": "error",
    "name": "FailedOp",
    "inputs": [
      {
        "name": "opIndex",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "reason",
        "type": "string",
        "internalType": "string"
      }
    ]
  },
  {
    "type": "error",
    "name": "SenderAddressResult",
    "inputs": [
      {
        "name": "sender",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "SignatureValidationFailed",
    "inputs": [
      {
        "name": "aggregator",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "ValidationResult",
    "inputs": [
      {
        "name": "returnInfo",
        "type": "tuple",
        "internalType": "struct IEntryPoint.ReturnInfo",
        "components": [
          {
            "name": "preOpGas",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "prefund",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "sigFailed",
            "type": "bool",
            "internalType": "bool"
          },
          {
            "name": "validAfter",
            "type": "uint48",
            "internalType": "uint48"
          },
          {
            "name": "validUntil",
            "type": "uint48",
            "internalType": "uint48"
          },
          {
            "name": "paymasterContext",
            "type": "bytes",
            "internalType": "bytes"
          }
        ]
      },
      {
        "name": "senderInfo",
        "type": "tuple",
        "internalType": "struct IStakeManager.StakeInfo",
        "components": [
          {
            "name": "stake",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "unstakeDelaySec",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      },
      {
        "name": "factoryInfo",
        "type": "tuple",
        "internalType": "struct IStakeManager.StakeInfo",
        "components": [
          {
            "name": "stake",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "unstakeDelaySec",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      },
      {
        "name": "paymasterInfo",
        "type": "tuple",
        "internalType": "struct IStakeManager.StakeInfo",
        "components": [
          {
            "name": "stake",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "unstakeDelaySec",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      }
    ]
  },
  {
    "type": "error",
    "name": "ValidationResultWithAggregation",
    "inputs": [
      {
        "name": "returnInfo",
        "type": "tuple",
        "internalType": "struct IEntryPoint.ReturnInfo",
        "components": [
          {
            "name": "preOpGas",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "prefund",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "sigFailed",
            "type": "bool",
            "internalType": "bool"
          },
          {
            "name": "validAfter",
            "type": "uint48",
            "internalType": "uint48"
          },
          {
            "name": "validUntil",
            "type": "uint48",
            "internalType": "uint48"
          },
          {
            "name": "paymasterContext",
            "type": "bytes",
            "internalType": "bytes"
          }
        ]
      },
      {
        "name": "senderInfo",
        "type": "tuple",
        "internalType": "struct IStakeManager.StakeInfo",
        "components": [
          {
            "name": "stake",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "unstakeDelaySec",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      },
      {
        "name": "factoryInfo",
        "type": "tuple",
        "internalType": "struct IStakeManager.StakeInfo",
        "components": [
          {
            "name": "stake",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "unstakeDelaySec",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      },
      {
        "name": "paymasterInfo",
        "type": "tuple",
        "internalType": "struct IStakeManager.StakeInfo",
        "components": [
          {
            "name": "stake",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "unstakeDelaySec",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      },
      {
        "name": "aggregatorInfo",
        "type": "tuple",
        "internalType": "struct IEntryPoint.AggregatorStakeInfo",
        "components": [
          {
            "name": "aggregator",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "stakeInfo",
            "type": "tuple",
            "internalType": "struct IStakeManager.StakeInfo",
            "components": [
              {
                "name": "stake",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "unstakeDelaySec",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          }
        ]
      }
    ]
  }
]
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style
)]
pub mod IEntryPoint {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"",
    );
    /**```solidity
struct AggregatorStakeInfo { address aggregator; IStakeManager.StakeInfo stakeInfo; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AggregatorStakeInfo {
        pub aggregator: alloy::sol_types::private::Address,
        pub stakeInfo: <IStakeManager::StakeInfo as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            IStakeManager::StakeInfo,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            <IStakeManager::StakeInfo as alloy::sol_types::SolType>::RustType,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<AggregatorStakeInfo> for UnderlyingRustTuple<'_> {
            fn from(value: AggregatorStakeInfo) -> Self {
                (value.aggregator, value.stakeInfo)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for AggregatorStakeInfo {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    aggregator: tuple.0,
                    stakeInfo: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for AggregatorStakeInfo {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for AggregatorStakeInfo {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.aggregator,
                    ),
                    <IStakeManager::StakeInfo as alloy_sol_types::SolType>::tokenize(
                        &self.stakeInfo,
                    ),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for AggregatorStakeInfo {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for AggregatorStakeInfo {
            const NAME: &'static str = "AggregatorStakeInfo";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "AggregatorStakeInfo(address aggregator,IStakeManager.StakeInfo stakeInfo)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(1);
                components
                    .push(
                        <IStakeManager::StakeInfo as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <IStakeManager::StakeInfo as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.aggregator,
                        )
                        .0,
                    <IStakeManager::StakeInfo as alloy_sol_types::SolType>::eip712_data_word(
                            &self.stakeInfo,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for AggregatorStakeInfo {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.aggregator,
                    )
                    + <IStakeManager::StakeInfo as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.stakeInfo,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.aggregator,
                    out,
                );
                <IStakeManager::StakeInfo as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.stakeInfo,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    /**```solidity
struct ReturnInfo { uint256 preOpGas; uint256 prefund; bool sigFailed; uint48 validAfter; uint48 validUntil; bytes paymasterContext; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ReturnInfo {
        pub preOpGas: alloy::sol_types::private::primitives::aliases::U256,
        pub prefund: alloy::sol_types::private::primitives::aliases::U256,
        pub sigFailed: bool,
        pub validAfter: alloy::sol_types::private::primitives::aliases::U48,
        pub validUntil: alloy::sol_types::private::primitives::aliases::U48,
        pub paymasterContext: alloy::sol_types::private::Bytes,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Bool,
            alloy::sol_types::sol_data::Uint<48>,
            alloy::sol_types::sol_data::Uint<48>,
            alloy::sol_types::sol_data::Bytes,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
            bool,
            alloy::sol_types::private::primitives::aliases::U48,
            alloy::sol_types::private::primitives::aliases::U48,
            alloy::sol_types::private::Bytes,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ReturnInfo> for UnderlyingRustTuple<'_> {
            fn from(value: ReturnInfo) -> Self {
                (
                    value.preOpGas,
                    value.prefund,
                    value.sigFailed,
                    value.validAfter,
                    value.validUntil,
                    value.paymasterContext,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ReturnInfo {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    preOpGas: tuple.0,
                    prefund: tuple.1,
                    sigFailed: tuple.2,
                    validAfter: tuple.3,
                    validUntil: tuple.4,
                    paymasterContext: tuple.5,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for ReturnInfo {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for ReturnInfo {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.preOpGas),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.prefund),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.sigFailed,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        48,
                    > as alloy_sol_types::SolType>::tokenize(&self.validAfter),
                    <alloy::sol_types::sol_data::Uint<
                        48,
                    > as alloy_sol_types::SolType>::tokenize(&self.validUntil),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.paymasterContext,
                    ),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for ReturnInfo {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for ReturnInfo {
            const NAME: &'static str = "ReturnInfo";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "ReturnInfo(uint256 preOpGas,uint256 prefund,bool sigFailed,uint48 validAfter,uint48 validUntil,bytes paymasterContext)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.preOpGas)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.prefund)
                        .0,
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::eip712_data_word(
                            &self.sigFailed,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        48,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.validAfter)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        48,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.validUntil)
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.paymasterContext,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for ReturnInfo {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.preOpGas,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.prefund,
                    )
                    + <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.sigFailed,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        48,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.validAfter,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        48,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.validUntil,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.paymasterContext,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.preOpGas,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.prefund,
                    out,
                );
                <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.sigFailed,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    48,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.validAfter,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    48,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.validUntil,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.paymasterContext,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    /**```solidity
struct UserOperation { address sender; uint256 nonce; bytes initCode; bytes callData; uint256 callGasLimit; uint256 verificationGasLimit; uint256 preVerificationGas; uint256 maxFeePerGas; uint256 maxPriorityFeePerGas; bytes paymasterAndData; bytes signature; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UserOperation {
        pub sender: alloy::sol_types::private::Address,
        pub nonce: alloy::sol_types::private::primitives::aliases::U256,
        pub initCode: alloy::sol_types::private::Bytes,
        pub callData: alloy::sol_types::private::Bytes,
        pub callGasLimit: alloy::sol_types::private::primitives::aliases::U256,
        pub verificationGasLimit: alloy::sol_types::private::primitives::aliases::U256,
        pub preVerificationGas: alloy::sol_types::private::primitives::aliases::U256,
        pub maxFeePerGas: alloy::sol_types::private::primitives::aliases::U256,
        pub maxPriorityFeePerGas: alloy::sol_types::private::primitives::aliases::U256,
        pub paymasterAndData: alloy::sol_types::private::Bytes,
        pub signature: alloy::sol_types::private::Bytes,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Bytes,
            alloy::sol_types::sol_data::Bytes,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Bytes,
            alloy::sol_types::sol_data::Bytes,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::Bytes,
            alloy::sol_types::private::Bytes,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::Bytes,
            alloy::sol_types::private::Bytes,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UserOperation> for UnderlyingRustTuple<'_> {
            fn from(value: UserOperation) -> Self {
                (
                    value.sender,
                    value.nonce,
                    value.initCode,
                    value.callData,
                    value.callGasLimit,
                    value.verificationGasLimit,
                    value.preVerificationGas,
                    value.maxFeePerGas,
                    value.maxPriorityFeePerGas,
                    value.paymasterAndData,
                    value.signature,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for UserOperation {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    sender: tuple.0,
                    nonce: tuple.1,
                    initCode: tuple.2,
                    callData: tuple.3,
                    callGasLimit: tuple.4,
                    verificationGasLimit: tuple.5,
                    preVerificationGas: tuple.6,
                    maxFeePerGas: tuple.7,
                    maxPriorityFeePerGas: tuple.8,
                    paymasterAndData: tuple.9,
                    signature: tuple.10,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for UserOperation {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for UserOperation {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.sender,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.nonce),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.initCode,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.callData,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.callGasLimit),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.verificationGasLimit),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.preVerificationGas),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.maxFeePerGas),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.maxPriorityFeePerGas),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.paymasterAndData,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.signature,
                    ),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for UserOperation {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for UserOperation {
            const NAME: &'static str = "UserOperation";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "UserOperation(address sender,uint256 nonce,bytes initCode,bytes callData,uint256 callGasLimit,uint256 verificationGasLimit,uint256 preVerificationGas,uint256 maxFeePerGas,uint256 maxPriorityFeePerGas,bytes paymasterAndData,bytes signature)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.sender,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.nonce)
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.initCode,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.callData,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.callGasLimit)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.verificationGasLimit,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.preVerificationGas,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.maxFeePerGas)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.maxPriorityFeePerGas,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.paymasterAndData,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.signature,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for UserOperation {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.sender,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.nonce)
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.initCode,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.callData,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.callGasLimit,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.verificationGasLimit,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.preVerificationGas,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.maxFeePerGas,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.maxPriorityFeePerGas,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.paymasterAndData,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.signature,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.sender,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.nonce,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.initCode,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.callData,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.callGasLimit,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.verificationGasLimit,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.preVerificationGas,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.maxFeePerGas,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.maxPriorityFeePerGas,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.paymasterAndData,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.signature,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    /**```solidity
struct UserOpsPerAggregator { UserOperation[] userOps; address aggregator; bytes signature; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UserOpsPerAggregator {
        pub userOps: alloy::sol_types::private::Vec<
            <UserOperation as alloy::sol_types::SolType>::RustType,
        >,
        pub aggregator: alloy::sol_types::private::Address,
        pub signature: alloy::sol_types::private::Bytes,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Array<UserOperation>,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Bytes,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Vec<
                <UserOperation as alloy::sol_types::SolType>::RustType,
            >,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Bytes,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UserOpsPerAggregator> for UnderlyingRustTuple<'_> {
            fn from(value: UserOpsPerAggregator) -> Self {
                (value.userOps, value.aggregator, value.signature)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for UserOpsPerAggregator {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    userOps: tuple.0,
                    aggregator: tuple.1,
                    signature: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for UserOpsPerAggregator {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for UserOpsPerAggregator {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        UserOperation,
                    > as alloy_sol_types::SolType>::tokenize(&self.userOps),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.aggregator,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.signature,
                    ),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for UserOpsPerAggregator {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for UserOpsPerAggregator {
            const NAME: &'static str = "UserOpsPerAggregator";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "UserOpsPerAggregator(UserOperation[] userOps,address aggregator,bytes signature)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(1);
                components
                    .push(
                        <UserOperation as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <UserOperation as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Array<
                        UserOperation,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.userOps)
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.aggregator,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.signature,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for UserOpsPerAggregator {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Array<
                        UserOperation,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.userOps,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.aggregator,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.signature,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Array<
                    UserOperation,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.userOps,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.aggregator,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.signature,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    /**Custom error with signature `ExecutionResult(uint256,uint256,uint48,uint48,bool,bytes)` and selector `0x8b7ac980`.
```solidity
error ExecutionResult(uint256 preOpGas, uint256 paid, uint48 validAfter, uint48 validUntil, bool targetSuccess, bytes targetResult);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ExecutionResult {
        pub preOpGas: alloy::sol_types::private::primitives::aliases::U256,
        pub paid: alloy::sol_types::private::primitives::aliases::U256,
        pub validAfter: alloy::sol_types::private::primitives::aliases::U48,
        pub validUntil: alloy::sol_types::private::primitives::aliases::U48,
        pub targetSuccess: bool,
        pub targetResult: alloy::sol_types::private::Bytes,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<48>,
            alloy::sol_types::sol_data::Uint<48>,
            alloy::sol_types::sol_data::Bool,
            alloy::sol_types::sol_data::Bytes,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U48,
            alloy::sol_types::private::primitives::aliases::U48,
            bool,
            alloy::sol_types::private::Bytes,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ExecutionResult> for UnderlyingRustTuple<'_> {
            fn from(value: ExecutionResult) -> Self {
                (
                    value.preOpGas,
                    value.paid,
                    value.validAfter,
                    value.validUntil,
                    value.targetSuccess,
                    value.targetResult,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ExecutionResult {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    preOpGas: tuple.0,
                    paid: tuple.1,
                    validAfter: tuple.2,
                    validUntil: tuple.3,
                    targetSuccess: tuple.4,
                    targetResult: tuple.5,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ExecutionResult {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ExecutionResult(uint256,uint256,uint48,uint48,bool,bytes)";
            const SELECTOR: [u8; 4] = [139u8, 122u8, 201u8, 128u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.preOpGas),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.paid),
                    <alloy::sol_types::sol_data::Uint<
                        48,
                    > as alloy_sol_types::SolType>::tokenize(&self.validAfter),
                    <alloy::sol_types::sol_data::Uint<
                        48,
                    > as alloy_sol_types::SolType>::tokenize(&self.validUntil),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.targetSuccess,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.targetResult,
                    ),
                )
            }
        }
    };
    /**Custom error with signature `FailedOp(uint256,string)` and selector `0x220266b6`.
```solidity
error FailedOp(uint256 opIndex, string reason);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct FailedOp {
        pub opIndex: alloy::sol_types::private::primitives::aliases::U256,
        pub reason: alloy::sol_types::private::String,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::String,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::String,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<FailedOp> for UnderlyingRustTuple<'_> {
            fn from(value: FailedOp) -> Self {
                (value.opIndex, value.reason)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for FailedOp {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    opIndex: tuple.0,
                    reason: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for FailedOp {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "FailedOp(uint256,string)";
            const SELECTOR: [u8; 4] = [34u8, 2u8, 102u8, 182u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.opIndex),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.reason,
                    ),
                )
            }
        }
    };
    /**Custom error with signature `SenderAddressResult(address)` and selector `0x6ca7b806`.
```solidity
error SenderAddressResult(address sender);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SenderAddressResult {
        pub sender: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<SenderAddressResult> for UnderlyingRustTuple<'_> {
            fn from(value: SenderAddressResult) -> Self {
                (value.sender,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for SenderAddressResult {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { sender: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for SenderAddressResult {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "SenderAddressResult(address)";
            const SELECTOR: [u8; 4] = [108u8, 167u8, 184u8, 6u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.sender,
                    ),
                )
            }
        }
    };
    /**Custom error with signature `SignatureValidationFailed(address)` and selector `0x86a9f750`.
```solidity
error SignatureValidationFailed(address aggregator);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SignatureValidationFailed {
        pub aggregator: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<SignatureValidationFailed>
        for UnderlyingRustTuple<'_> {
            fn from(value: SignatureValidationFailed) -> Self {
                (value.aggregator,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for SignatureValidationFailed {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { aggregator: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for SignatureValidationFailed {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "SignatureValidationFailed(address)";
            const SELECTOR: [u8; 4] = [134u8, 169u8, 247u8, 80u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.aggregator,
                    ),
                )
            }
        }
    };
    /**Custom error with signature `ValidationResult((uint256,uint256,bool,uint48,uint48,bytes),(uint256,uint256),(uint256,uint256),(uint256,uint256))` and selector `0xe0cff05f`.
```solidity
error ValidationResult(ReturnInfo returnInfo, IStakeManager.StakeInfo senderInfo, IStakeManager.StakeInfo factoryInfo, IStakeManager.StakeInfo paymasterInfo);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ValidationResult {
        pub returnInfo: <ReturnInfo as alloy::sol_types::SolType>::RustType,
        pub senderInfo: <IStakeManager::StakeInfo as alloy::sol_types::SolType>::RustType,
        pub factoryInfo: <IStakeManager::StakeInfo as alloy::sol_types::SolType>::RustType,
        pub paymasterInfo: <IStakeManager::StakeInfo as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            ReturnInfo,
            IStakeManager::StakeInfo,
            IStakeManager::StakeInfo,
            IStakeManager::StakeInfo,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            <ReturnInfo as alloy::sol_types::SolType>::RustType,
            <IStakeManager::StakeInfo as alloy::sol_types::SolType>::RustType,
            <IStakeManager::StakeInfo as alloy::sol_types::SolType>::RustType,
            <IStakeManager::StakeInfo as alloy::sol_types::SolType>::RustType,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ValidationResult> for UnderlyingRustTuple<'_> {
            fn from(value: ValidationResult) -> Self {
                (
                    value.returnInfo,
                    value.senderInfo,
                    value.factoryInfo,
                    value.paymasterInfo,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ValidationResult {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    returnInfo: tuple.0,
                    senderInfo: tuple.1,
                    factoryInfo: tuple.2,
                    paymasterInfo: tuple.3,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ValidationResult {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ValidationResult((uint256,uint256,bool,uint48,uint48,bytes),(uint256,uint256),(uint256,uint256),(uint256,uint256))";
            const SELECTOR: [u8; 4] = [224u8, 207u8, 240u8, 95u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <ReturnInfo as alloy_sol_types::SolType>::tokenize(&self.returnInfo),
                    <IStakeManager::StakeInfo as alloy_sol_types::SolType>::tokenize(
                        &self.senderInfo,
                    ),
                    <IStakeManager::StakeInfo as alloy_sol_types::SolType>::tokenize(
                        &self.factoryInfo,
                    ),
                    <IStakeManager::StakeInfo as alloy_sol_types::SolType>::tokenize(
                        &self.paymasterInfo,
                    ),
                )
            }
        }
    };
    /**Custom error with signature `ValidationResultWithAggregation((uint256,uint256,bool,uint48,uint48,bytes),(uint256,uint256),(uint256,uint256),(uint256,uint256),(address,(uint256,uint256)))` and selector `0xfaecb4e4`.
```solidity
error ValidationResultWithAggregation(ReturnInfo returnInfo, IStakeManager.StakeInfo senderInfo, IStakeManager.StakeInfo factoryInfo, IStakeManager.StakeInfo paymasterInfo, AggregatorStakeInfo aggregatorInfo);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ValidationResultWithAggregation {
        pub returnInfo: <ReturnInfo as alloy::sol_types::SolType>::RustType,
        pub senderInfo: <IStakeManager::StakeInfo as alloy::sol_types::SolType>::RustType,
        pub factoryInfo: <IStakeManager::StakeInfo as alloy::sol_types::SolType>::RustType,
        pub paymasterInfo: <IStakeManager::StakeInfo as alloy::sol_types::SolType>::RustType,
        pub aggregatorInfo: <AggregatorStakeInfo as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            ReturnInfo,
            IStakeManager::StakeInfo,
            IStakeManager::StakeInfo,
            IStakeManager::StakeInfo,
            AggregatorStakeInfo,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            <ReturnInfo as alloy::sol_types::SolType>::RustType,
            <IStakeManager::StakeInfo as alloy::sol_types::SolType>::RustType,
            <IStakeManager::StakeInfo as alloy::sol_types::SolType>::RustType,
            <IStakeManager::StakeInfo as alloy::sol_types::SolType>::RustType,
            <AggregatorStakeInfo as alloy::sol_types::SolType>::RustType,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ValidationResultWithAggregation>
        for UnderlyingRustTuple<'_> {
            fn from(value: ValidationResultWithAggregation) -> Self {
                (
                    value.returnInfo,
                    value.senderInfo,
                    value.factoryInfo,
                    value.paymasterInfo,
                    value.aggregatorInfo,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for ValidationResultWithAggregation {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    returnInfo: tuple.0,
                    senderInfo: tuple.1,
                    factoryInfo: tuple.2,
                    paymasterInfo: tuple.3,
                    aggregatorInfo: tuple.4,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ValidationResultWithAggregation {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ValidationResultWithAggregation((uint256,uint256,bool,uint48,uint48,bytes),(uint256,uint256),(uint256,uint256),(uint256,uint256),(address,(uint256,uint256)))";
            const SELECTOR: [u8; 4] = [250u8, 236u8, 180u8, 228u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <ReturnInfo as alloy_sol_types::SolType>::tokenize(&self.returnInfo),
                    <IStakeManager::StakeInfo as alloy_sol_types::SolType>::tokenize(
                        &self.senderInfo,
                    ),
                    <IStakeManager::StakeInfo as alloy_sol_types::SolType>::tokenize(
                        &self.factoryInfo,
                    ),
                    <IStakeManager::StakeInfo as alloy_sol_types::SolType>::tokenize(
                        &self.paymasterInfo,
                    ),
                    <AggregatorStakeInfo as alloy_sol_types::SolType>::tokenize(
                        &self.aggregatorInfo,
                    ),
                )
            }
        }
    };
    /**Event with signature `AccountDeployed(bytes32,address,address,address)` and selector `0xd51a9c61267aa6196961883ecf5ff2da6619c37dac0fa92122513fb32c032d2d`.
```solidity
event AccountDeployed(bytes32 indexed userOpHash, address indexed sender, address factory, address paymaster);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct AccountDeployed {
        #[allow(missing_docs)]
        pub userOpHash: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub sender: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub factory: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub paymaster: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for AccountDeployed {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "AccountDeployed(bytes32,address,address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                213u8,
                26u8,
                156u8,
                97u8,
                38u8,
                122u8,
                166u8,
                25u8,
                105u8,
                97u8,
                136u8,
                62u8,
                207u8,
                95u8,
                242u8,
                218u8,
                102u8,
                25u8,
                195u8,
                125u8,
                172u8,
                15u8,
                169u8,
                33u8,
                34u8,
                81u8,
                63u8,
                179u8,
                44u8,
                3u8,
                45u8,
                45u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    userOpHash: topics.1,
                    sender: topics.2,
                    factory: data.0,
                    paymaster: data.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.factory,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.paymaster,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.userOpHash.clone(),
                    self.sender.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.userOpHash);
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.sender,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for AccountDeployed {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&AccountDeployed> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &AccountDeployed) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `BeforeExecution()` and selector `0xbb47ee3e183a558b1a2ff0874b079f3fc5478b7454eacf2bfc5af2ff5878f972`.
```solidity
event BeforeExecution();
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct BeforeExecution {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for BeforeExecution {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "BeforeExecution()";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                187u8,
                71u8,
                238u8,
                62u8,
                24u8,
                58u8,
                85u8,
                139u8,
                26u8,
                47u8,
                240u8,
                135u8,
                75u8,
                7u8,
                159u8,
                63u8,
                197u8,
                71u8,
                139u8,
                116u8,
                84u8,
                234u8,
                207u8,
                43u8,
                252u8,
                90u8,
                242u8,
                255u8,
                88u8,
                120u8,
                249u8,
                114u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {}
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for BeforeExecution {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&BeforeExecution> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &BeforeExecution) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `Deposited(address,uint256)` and selector `0x2da466a7b24304f47e87fa2e1e5a81b9831ce54fec19055ce277ca2f39ba42c4`.
```solidity
event Deposited(address indexed account, uint256 totalDeposit);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Deposited {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub totalDeposit: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Deposited {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "Deposited(address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                45u8,
                164u8,
                102u8,
                167u8,
                178u8,
                67u8,
                4u8,
                244u8,
                126u8,
                135u8,
                250u8,
                46u8,
                30u8,
                90u8,
                129u8,
                185u8,
                131u8,
                28u8,
                229u8,
                79u8,
                236u8,
                25u8,
                5u8,
                92u8,
                226u8,
                119u8,
                202u8,
                47u8,
                57u8,
                186u8,
                66u8,
                196u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    account: topics.1,
                    totalDeposit: data.0,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.totalDeposit),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.account.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.account,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Deposited {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Deposited> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Deposited) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `SignatureAggregatorChanged(address)` and selector `0x575ff3acadd5ab348fe1855e217e0f3678f8d767d7494c9f9fefbee2e17cca4d`.
```solidity
event SignatureAggregatorChanged(address indexed aggregator);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct SignatureAggregatorChanged {
        #[allow(missing_docs)]
        pub aggregator: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for SignatureAggregatorChanged {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "SignatureAggregatorChanged(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                87u8,
                95u8,
                243u8,
                172u8,
                173u8,
                213u8,
                171u8,
                52u8,
                143u8,
                225u8,
                133u8,
                94u8,
                33u8,
                126u8,
                15u8,
                54u8,
                120u8,
                248u8,
                215u8,
                103u8,
                215u8,
                73u8,
                76u8,
                159u8,
                159u8,
                239u8,
                190u8,
                226u8,
                225u8,
                124u8,
                202u8,
                77u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { aggregator: topics.1 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.aggregator.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.aggregator,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for SignatureAggregatorChanged {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&SignatureAggregatorChanged> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &SignatureAggregatorChanged,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `StakeLocked(address,uint256,uint256)` and selector `0xa5ae833d0bb1dcd632d98a8b70973e8516812898e19bf27b70071ebc8dc52c01`.
```solidity
event StakeLocked(address indexed account, uint256 totalStaked, uint256 unstakeDelaySec);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct StakeLocked {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub totalStaked: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub unstakeDelaySec: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for StakeLocked {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "StakeLocked(address,uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                165u8,
                174u8,
                131u8,
                61u8,
                11u8,
                177u8,
                220u8,
                214u8,
                50u8,
                217u8,
                138u8,
                139u8,
                112u8,
                151u8,
                62u8,
                133u8,
                22u8,
                129u8,
                40u8,
                152u8,
                225u8,
                155u8,
                242u8,
                123u8,
                112u8,
                7u8,
                30u8,
                188u8,
                141u8,
                197u8,
                44u8,
                1u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    account: topics.1,
                    totalStaked: data.0,
                    unstakeDelaySec: data.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.totalStaked),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.unstakeDelaySec),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.account.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.account,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for StakeLocked {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&StakeLocked> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &StakeLocked) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `StakeUnlocked(address,uint256)` and selector `0xfa9b3c14cc825c412c9ed81b3ba365a5b459439403f18829e572ed53a4180f0a`.
```solidity
event StakeUnlocked(address indexed account, uint256 withdrawTime);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct StakeUnlocked {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub withdrawTime: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for StakeUnlocked {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "StakeUnlocked(address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                250u8,
                155u8,
                60u8,
                20u8,
                204u8,
                130u8,
                92u8,
                65u8,
                44u8,
                158u8,
                216u8,
                27u8,
                59u8,
                163u8,
                101u8,
                165u8,
                180u8,
                89u8,
                67u8,
                148u8,
                3u8,
                241u8,
                136u8,
                41u8,
                229u8,
                114u8,
                237u8,
                83u8,
                164u8,
                24u8,
                15u8,
                10u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    account: topics.1,
                    withdrawTime: data.0,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.withdrawTime),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.account.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.account,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for StakeUnlocked {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&StakeUnlocked> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &StakeUnlocked) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `StakeWithdrawn(address,address,uint256)` and selector `0xb7c918e0e249f999e965cafeb6c664271b3f4317d296461500e71da39f0cbda3`.
```solidity
event StakeWithdrawn(address indexed account, address withdrawAddress, uint256 amount);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct StakeWithdrawn {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub withdrawAddress: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for StakeWithdrawn {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "StakeWithdrawn(address,address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                183u8,
                201u8,
                24u8,
                224u8,
                226u8,
                73u8,
                249u8,
                153u8,
                233u8,
                101u8,
                202u8,
                254u8,
                182u8,
                198u8,
                100u8,
                39u8,
                27u8,
                63u8,
                67u8,
                23u8,
                210u8,
                150u8,
                70u8,
                21u8,
                0u8,
                231u8,
                29u8,
                163u8,
                159u8,
                12u8,
                189u8,
                163u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    account: topics.1,
                    withdrawAddress: data.0,
                    amount: data.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.withdrawAddress,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.account.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.account,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for StakeWithdrawn {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&StakeWithdrawn> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &StakeWithdrawn) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `UserOperationEvent(bytes32,address,address,uint256,bool,uint256,uint256)` and selector `0x49628fd1471006c1482da88028e9ce4dbb080b815c9b0344d39e5a8e6ec1419f`.
```solidity
event UserOperationEvent(bytes32 indexed userOpHash, address indexed sender, address indexed paymaster, uint256 nonce, bool success, uint256 actualGasCost, uint256 actualGasUsed);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct UserOperationEvent {
        #[allow(missing_docs)]
        pub userOpHash: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub sender: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub paymaster: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub nonce: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub success: bool,
        #[allow(missing_docs)]
        pub actualGasCost: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub actualGasUsed: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for UserOperationEvent {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "UserOperationEvent(bytes32,address,address,uint256,bool,uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                73u8,
                98u8,
                143u8,
                209u8,
                71u8,
                16u8,
                6u8,
                193u8,
                72u8,
                45u8,
                168u8,
                128u8,
                40u8,
                233u8,
                206u8,
                77u8,
                187u8,
                8u8,
                11u8,
                129u8,
                92u8,
                155u8,
                3u8,
                68u8,
                211u8,
                158u8,
                90u8,
                142u8,
                110u8,
                193u8,
                65u8,
                159u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    userOpHash: topics.1,
                    sender: topics.2,
                    paymaster: topics.3,
                    nonce: data.0,
                    success: data.1,
                    actualGasCost: data.2,
                    actualGasUsed: data.3,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.nonce),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.success,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.actualGasCost),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.actualGasUsed),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.userOpHash.clone(),
                    self.sender.clone(),
                    self.paymaster.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.userOpHash);
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.sender,
                );
                out[3usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.paymaster,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for UserOperationEvent {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&UserOperationEvent> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &UserOperationEvent) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `UserOperationRevertReason(bytes32,address,uint256,bytes)` and selector `0x1c4fada7374c0a9ee8841fc38afe82932dc0f8e69012e927f061a8bae611a201`.
```solidity
event UserOperationRevertReason(bytes32 indexed userOpHash, address indexed sender, uint256 nonce, bytes revertReason);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct UserOperationRevertReason {
        #[allow(missing_docs)]
        pub userOpHash: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub sender: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub nonce: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub revertReason: alloy::sol_types::private::Bytes,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for UserOperationRevertReason {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "UserOperationRevertReason(bytes32,address,uint256,bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                28u8,
                79u8,
                173u8,
                167u8,
                55u8,
                76u8,
                10u8,
                158u8,
                232u8,
                132u8,
                31u8,
                195u8,
                138u8,
                254u8,
                130u8,
                147u8,
                45u8,
                192u8,
                248u8,
                230u8,
                144u8,
                18u8,
                233u8,
                39u8,
                240u8,
                97u8,
                168u8,
                186u8,
                230u8,
                17u8,
                162u8,
                1u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    userOpHash: topics.1,
                    sender: topics.2,
                    nonce: data.0,
                    revertReason: data.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.nonce),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.revertReason,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.userOpHash.clone(),
                    self.sender.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.userOpHash);
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.sender,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for UserOperationRevertReason {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&UserOperationRevertReason> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &UserOperationRevertReason,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `Withdrawn(address,address,uint256)` and selector `0xd1c19fbcd4551a5edfb66d43d2e337c04837afda3482b42bdf569a8fccdae5fb`.
```solidity
event Withdrawn(address indexed account, address withdrawAddress, uint256 amount);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Withdrawn {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub withdrawAddress: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Withdrawn {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "Withdrawn(address,address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                209u8,
                193u8,
                159u8,
                188u8,
                212u8,
                85u8,
                26u8,
                94u8,
                223u8,
                182u8,
                109u8,
                67u8,
                210u8,
                227u8,
                55u8,
                192u8,
                72u8,
                55u8,
                175u8,
                218u8,
                52u8,
                130u8,
                180u8,
                43u8,
                223u8,
                86u8,
                154u8,
                143u8,
                204u8,
                218u8,
                229u8,
                251u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    account: topics.1,
                    withdrawAddress: data.0,
                    amount: data.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.withdrawAddress,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.account.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.account,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Withdrawn {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Withdrawn> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Withdrawn) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Function with signature `addStake(uint32)` and selector `0x0396cb60`.
```solidity
function addStake(uint32 _unstakeDelaySec) external payable;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addStakeCall {
        pub _unstakeDelaySec: u32,
    }
    ///Container type for the return parameters of the [`addStake(uint32)`](addStakeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addStakeReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u32,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<addStakeCall> for UnderlyingRustTuple<'_> {
                fn from(value: addStakeCall) -> Self {
                    (value._unstakeDelaySec,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for addStakeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _unstakeDelaySec: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<addStakeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: addStakeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for addStakeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for addStakeCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = addStakeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "addStake(uint32)";
            const SELECTOR: [u8; 4] = [3u8, 150u8, 203u8, 96u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self._unstakeDelaySec),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `balanceOf(address)` and selector `0x70a08231`.
```solidity
function balanceOf(address account) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct balanceOfCall {
        pub account: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`balanceOf(address)`](balanceOfCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct balanceOfReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<balanceOfCall> for UnderlyingRustTuple<'_> {
                fn from(value: balanceOfCall) -> Self {
                    (value.account,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for balanceOfCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { account: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<balanceOfReturn> for UnderlyingRustTuple<'_> {
                fn from(value: balanceOfReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for balanceOfReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for balanceOfCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = balanceOfReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "balanceOf(address)";
            const SELECTOR: [u8; 4] = [112u8, 160u8, 130u8, 49u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `depositTo(address)` and selector `0xb760faf9`.
```solidity
function depositTo(address account) external payable;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct depositToCall {
        pub account: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`depositTo(address)`](depositToCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct depositToReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<depositToCall> for UnderlyingRustTuple<'_> {
                fn from(value: depositToCall) -> Self {
                    (value.account,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for depositToCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { account: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<depositToReturn> for UnderlyingRustTuple<'_> {
                fn from(value: depositToReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for depositToReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for depositToCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = depositToReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "depositTo(address)";
            const SELECTOR: [u8; 4] = [183u8, 96u8, 250u8, 249u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `getDepositInfo(address)` and selector `0x5287ce12`.
```solidity
function getDepositInfo(address account) external view returns (IStakeManager.DepositInfo memory info);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getDepositInfoCall {
        pub account: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getDepositInfo(address)`](getDepositInfoCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getDepositInfoReturn {
        pub info: <IStakeManager::DepositInfo as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getDepositInfoCall> for UnderlyingRustTuple<'_> {
                fn from(value: getDepositInfoCall) -> Self {
                    (value.account,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getDepositInfoCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { account: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (IStakeManager::DepositInfo,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IStakeManager::DepositInfo as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getDepositInfoReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getDepositInfoReturn) -> Self {
                    (value.info,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getDepositInfoReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { info: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getDepositInfoCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getDepositInfoReturn;
            type ReturnTuple<'a> = (IStakeManager::DepositInfo,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getDepositInfo(address)";
            const SELECTOR: [u8; 4] = [82u8, 135u8, 206u8, 18u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `getNonce(address,uint192)` and selector `0x35567e1a`.
```solidity
function getNonce(address sender, uint192 key) external view returns (uint256 nonce);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getNonceCall {
        pub sender: alloy::sol_types::private::Address,
        pub key: alloy::sol_types::private::primitives::aliases::U192,
    }
    ///Container type for the return parameters of the [`getNonce(address,uint192)`](getNonceCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getNonceReturn {
        pub nonce: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<192>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U192,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getNonceCall> for UnderlyingRustTuple<'_> {
                fn from(value: getNonceCall) -> Self {
                    (value.sender, value.key)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getNonceCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        sender: tuple.0,
                        key: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getNonceReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getNonceReturn) -> Self {
                    (value.nonce,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getNonceReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { nonce: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getNonceCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<192>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getNonceReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getNonce(address,uint192)";
            const SELECTOR: [u8; 4] = [53u8, 86u8, 126u8, 26u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.sender,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        192,
                    > as alloy_sol_types::SolType>::tokenize(&self.key),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `getSenderAddress(bytes)` and selector `0x9b249f69`.
```solidity
function getSenderAddress(bytes memory initCode) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getSenderAddressCall {
        pub initCode: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`getSenderAddress(bytes)`](getSenderAddressCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getSenderAddressReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Bytes,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getSenderAddressCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getSenderAddressCall) -> Self {
                    (value.initCode,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getSenderAddressCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { initCode: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getSenderAddressReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getSenderAddressReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getSenderAddressReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getSenderAddressCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Bytes,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getSenderAddressReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getSenderAddress(bytes)";
            const SELECTOR: [u8; 4] = [155u8, 36u8, 159u8, 105u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.initCode,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `getUserOpHash((address,uint256,bytes,bytes,uint256,uint256,uint256,uint256,uint256,bytes,bytes))` and selector `0xa6193531`.
```solidity
function getUserOpHash(UserOperation memory userOp) external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getUserOpHashCall {
        pub userOp: <UserOperation as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`getUserOpHash((address,uint256,bytes,bytes,uint256,uint256,uint256,uint256,uint256,bytes,bytes))`](getUserOpHashCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getUserOpHashReturn {
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (UserOperation,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <UserOperation as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getUserOpHashCall> for UnderlyingRustTuple<'_> {
                fn from(value: getUserOpHashCall) -> Self {
                    (value.userOp,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getUserOpHashCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { userOp: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getUserOpHashReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getUserOpHashReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getUserOpHashReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getUserOpHashCall {
            type Parameters<'a> = (UserOperation,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getUserOpHashReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getUserOpHash((address,uint256,bytes,bytes,uint256,uint256,uint256,uint256,uint256,bytes,bytes))";
            const SELECTOR: [u8; 4] = [166u8, 25u8, 53u8, 49u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<UserOperation as alloy_sol_types::SolType>::tokenize(&self.userOp),)
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `handleAggregatedOps(((address,uint256,bytes,bytes,uint256,uint256,uint256,uint256,uint256,bytes,bytes)[],address,bytes)[],address)` and selector `0x4b1d7cf5`.
```solidity
function handleAggregatedOps(UserOpsPerAggregator[] memory opsPerAggregator, address beneficiary) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct handleAggregatedOpsCall {
        pub opsPerAggregator: alloy::sol_types::private::Vec<
            <UserOpsPerAggregator as alloy::sol_types::SolType>::RustType,
        >,
        pub beneficiary: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`handleAggregatedOps(((address,uint256,bytes,bytes,uint256,uint256,uint256,uint256,uint256,bytes,bytes)[],address,bytes)[],address)`](handleAggregatedOpsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct handleAggregatedOpsReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<UserOpsPerAggregator>,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <UserOpsPerAggregator as alloy::sol_types::SolType>::RustType,
                >,
                alloy::sol_types::private::Address,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<handleAggregatedOpsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: handleAggregatedOpsCall) -> Self {
                    (value.opsPerAggregator, value.beneficiary)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for handleAggregatedOpsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        opsPerAggregator: tuple.0,
                        beneficiary: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<handleAggregatedOpsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: handleAggregatedOpsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for handleAggregatedOpsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for handleAggregatedOpsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<UserOpsPerAggregator>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = handleAggregatedOpsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "handleAggregatedOps(((address,uint256,bytes,bytes,uint256,uint256,uint256,uint256,uint256,bytes,bytes)[],address,bytes)[],address)";
            const SELECTOR: [u8; 4] = [75u8, 29u8, 124u8, 245u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        UserOpsPerAggregator,
                    > as alloy_sol_types::SolType>::tokenize(&self.opsPerAggregator),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.beneficiary,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `handleOps((address,uint256,bytes,bytes,uint256,uint256,uint256,uint256,uint256,bytes,bytes)[],address)` and selector `0x1fad948c`.
```solidity
function handleOps(UserOperation[] memory ops, address beneficiary) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct handleOpsCall {
        pub ops: alloy::sol_types::private::Vec<
            <UserOperation as alloy::sol_types::SolType>::RustType,
        >,
        pub beneficiary: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`handleOps((address,uint256,bytes,bytes,uint256,uint256,uint256,uint256,uint256,bytes,bytes)[],address)`](handleOpsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct handleOpsReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<UserOperation>,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <UserOperation as alloy::sol_types::SolType>::RustType,
                >,
                alloy::sol_types::private::Address,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<handleOpsCall> for UnderlyingRustTuple<'_> {
                fn from(value: handleOpsCall) -> Self {
                    (value.ops, value.beneficiary)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for handleOpsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        ops: tuple.0,
                        beneficiary: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<handleOpsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: handleOpsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for handleOpsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for handleOpsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<UserOperation>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = handleOpsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "handleOps((address,uint256,bytes,bytes,uint256,uint256,uint256,uint256,uint256,bytes,bytes)[],address)";
            const SELECTOR: [u8; 4] = [31u8, 173u8, 148u8, 140u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        UserOperation,
                    > as alloy_sol_types::SolType>::tokenize(&self.ops),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.beneficiary,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `incrementNonce(uint192)` and selector `0x0bd28e3b`.
```solidity
function incrementNonce(uint192 key) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct incrementNonceCall {
        pub key: alloy::sol_types::private::primitives::aliases::U192,
    }
    ///Container type for the return parameters of the [`incrementNonce(uint192)`](incrementNonceCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct incrementNonceReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<192>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U192,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<incrementNonceCall> for UnderlyingRustTuple<'_> {
                fn from(value: incrementNonceCall) -> Self {
                    (value.key,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for incrementNonceCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { key: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<incrementNonceReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: incrementNonceReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for incrementNonceReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for incrementNonceCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<192>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = incrementNonceReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "incrementNonce(uint192)";
            const SELECTOR: [u8; 4] = [11u8, 210u8, 142u8, 59u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        192,
                    > as alloy_sol_types::SolType>::tokenize(&self.key),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `simulateHandleOp((address,uint256,bytes,bytes,uint256,uint256,uint256,uint256,uint256,bytes,bytes),address,bytes)` and selector `0xd6383f94`.
```solidity
function simulateHandleOp(UserOperation memory op, address target, bytes memory targetCallData) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct simulateHandleOpCall {
        pub op: <UserOperation as alloy::sol_types::SolType>::RustType,
        pub target: alloy::sol_types::private::Address,
        pub targetCallData: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`simulateHandleOp((address,uint256,bytes,bytes,uint256,uint256,uint256,uint256,uint256,bytes,bytes),address,bytes)`](simulateHandleOpCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct simulateHandleOpReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                UserOperation,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <UserOperation as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Bytes,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<simulateHandleOpCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: simulateHandleOpCall) -> Self {
                    (value.op, value.target, value.targetCallData)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for simulateHandleOpCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        op: tuple.0,
                        target: tuple.1,
                        targetCallData: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<simulateHandleOpReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: simulateHandleOpReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for simulateHandleOpReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for simulateHandleOpCall {
            type Parameters<'a> = (
                UserOperation,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = simulateHandleOpReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "simulateHandleOp((address,uint256,bytes,bytes,uint256,uint256,uint256,uint256,uint256,bytes,bytes),address,bytes)";
            const SELECTOR: [u8; 4] = [214u8, 56u8, 63u8, 148u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <UserOperation as alloy_sol_types::SolType>::tokenize(&self.op),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.target,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.targetCallData,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `simulateValidation((address,uint256,bytes,bytes,uint256,uint256,uint256,uint256,uint256,bytes,bytes))` and selector `0xee219423`.
```solidity
function simulateValidation(UserOperation memory userOp) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct simulateValidationCall {
        pub userOp: <UserOperation as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`simulateValidation((address,uint256,bytes,bytes,uint256,uint256,uint256,uint256,uint256,bytes,bytes))`](simulateValidationCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct simulateValidationReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (UserOperation,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <UserOperation as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<simulateValidationCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: simulateValidationCall) -> Self {
                    (value.userOp,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for simulateValidationCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { userOp: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<simulateValidationReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: simulateValidationReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for simulateValidationReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for simulateValidationCall {
            type Parameters<'a> = (UserOperation,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = simulateValidationReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "simulateValidation((address,uint256,bytes,bytes,uint256,uint256,uint256,uint256,uint256,bytes,bytes))";
            const SELECTOR: [u8; 4] = [238u8, 33u8, 148u8, 35u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<UserOperation as alloy_sol_types::SolType>::tokenize(&self.userOp),)
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `unlockStake()` and selector `0xbb9fe6bf`.
```solidity
function unlockStake() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct unlockStakeCall {}
    ///Container type for the return parameters of the [`unlockStake()`](unlockStakeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct unlockStakeReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<unlockStakeCall> for UnderlyingRustTuple<'_> {
                fn from(value: unlockStakeCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for unlockStakeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<unlockStakeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: unlockStakeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for unlockStakeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for unlockStakeCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = unlockStakeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "unlockStake()";
            const SELECTOR: [u8; 4] = [187u8, 159u8, 230u8, 191u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `withdrawStake(address)` and selector `0xc23a5cea`.
```solidity
function withdrawStake(address withdrawAddress) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct withdrawStakeCall {
        pub withdrawAddress: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`withdrawStake(address)`](withdrawStakeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct withdrawStakeReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<withdrawStakeCall> for UnderlyingRustTuple<'_> {
                fn from(value: withdrawStakeCall) -> Self {
                    (value.withdrawAddress,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for withdrawStakeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { withdrawAddress: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<withdrawStakeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: withdrawStakeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for withdrawStakeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for withdrawStakeCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = withdrawStakeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "withdrawStake(address)";
            const SELECTOR: [u8; 4] = [194u8, 58u8, 92u8, 234u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.withdrawAddress,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `withdrawTo(address,uint256)` and selector `0x205c2878`.
```solidity
function withdrawTo(address withdrawAddress, uint256 withdrawAmount) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct withdrawToCall {
        pub withdrawAddress: alloy::sol_types::private::Address,
        pub withdrawAmount: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`withdrawTo(address,uint256)`](withdrawToCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct withdrawToReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<withdrawToCall> for UnderlyingRustTuple<'_> {
                fn from(value: withdrawToCall) -> Self {
                    (value.withdrawAddress, value.withdrawAmount)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for withdrawToCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        withdrawAddress: tuple.0,
                        withdrawAmount: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<withdrawToReturn> for UnderlyingRustTuple<'_> {
                fn from(value: withdrawToReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for withdrawToReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for withdrawToCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = withdrawToReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "withdrawTo(address,uint256)";
            const SELECTOR: [u8; 4] = [32u8, 92u8, 40u8, 120u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.withdrawAddress,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.withdrawAmount),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    ///Container for all the [`IEntryPoint`](self) function calls.
    pub enum IEntryPointCalls {
        addStake(addStakeCall),
        balanceOf(balanceOfCall),
        depositTo(depositToCall),
        getDepositInfo(getDepositInfoCall),
        getNonce(getNonceCall),
        getSenderAddress(getSenderAddressCall),
        getUserOpHash(getUserOpHashCall),
        handleAggregatedOps(handleAggregatedOpsCall),
        handleOps(handleOpsCall),
        incrementNonce(incrementNonceCall),
        simulateHandleOp(simulateHandleOpCall),
        simulateValidation(simulateValidationCall),
        unlockStake(unlockStakeCall),
        withdrawStake(withdrawStakeCall),
        withdrawTo(withdrawToCall),
    }
    #[automatically_derived]
    impl IEntryPointCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [3u8, 150u8, 203u8, 96u8],
            [11u8, 210u8, 142u8, 59u8],
            [31u8, 173u8, 148u8, 140u8],
            [32u8, 92u8, 40u8, 120u8],
            [53u8, 86u8, 126u8, 26u8],
            [75u8, 29u8, 124u8, 245u8],
            [82u8, 135u8, 206u8, 18u8],
            [112u8, 160u8, 130u8, 49u8],
            [155u8, 36u8, 159u8, 105u8],
            [166u8, 25u8, 53u8, 49u8],
            [183u8, 96u8, 250u8, 249u8],
            [187u8, 159u8, 230u8, 191u8],
            [194u8, 58u8, 92u8, 234u8],
            [214u8, 56u8, 63u8, 148u8],
            [238u8, 33u8, 148u8, 35u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for IEntryPointCalls {
        const NAME: &'static str = "IEntryPointCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 15usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::addStake(_) => <addStakeCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::balanceOf(_) => {
                    <balanceOfCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::depositTo(_) => {
                    <depositToCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getDepositInfo(_) => {
                    <getDepositInfoCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getNonce(_) => <getNonceCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::getSenderAddress(_) => {
                    <getSenderAddressCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getUserOpHash(_) => {
                    <getUserOpHashCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::handleAggregatedOps(_) => {
                    <handleAggregatedOpsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::handleOps(_) => {
                    <handleOpsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::incrementNonce(_) => {
                    <incrementNonceCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::simulateHandleOp(_) => {
                    <simulateHandleOpCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::simulateValidation(_) => {
                    <simulateValidationCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::unlockStake(_) => {
                    <unlockStakeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::withdrawStake(_) => {
                    <withdrawStakeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::withdrawTo(_) => {
                    <withdrawToCall as alloy_sol_types::SolCall>::SELECTOR
                }
            }
        }
        #[inline]
        fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
            Self::SELECTORS.get(i).copied()
        }
        #[inline]
        fn valid_selector(selector: [u8; 4]) -> bool {
            Self::SELECTORS.binary_search(&selector).is_ok()
        }
        #[inline]
        #[allow(unsafe_code, non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
                bool,
            ) -> alloy_sol_types::Result<IEntryPointCalls>] = &[
                {
                    fn addStake(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IEntryPointCalls> {
                        <addStakeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IEntryPointCalls::addStake)
                    }
                    addStake
                },
                {
                    fn incrementNonce(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IEntryPointCalls> {
                        <incrementNonceCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IEntryPointCalls::incrementNonce)
                    }
                    incrementNonce
                },
                {
                    fn handleOps(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IEntryPointCalls> {
                        <handleOpsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IEntryPointCalls::handleOps)
                    }
                    handleOps
                },
                {
                    fn withdrawTo(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IEntryPointCalls> {
                        <withdrawToCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IEntryPointCalls::withdrawTo)
                    }
                    withdrawTo
                },
                {
                    fn getNonce(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IEntryPointCalls> {
                        <getNonceCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IEntryPointCalls::getNonce)
                    }
                    getNonce
                },
                {
                    fn handleAggregatedOps(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IEntryPointCalls> {
                        <handleAggregatedOpsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IEntryPointCalls::handleAggregatedOps)
                    }
                    handleAggregatedOps
                },
                {
                    fn getDepositInfo(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IEntryPointCalls> {
                        <getDepositInfoCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IEntryPointCalls::getDepositInfo)
                    }
                    getDepositInfo
                },
                {
                    fn balanceOf(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IEntryPointCalls> {
                        <balanceOfCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IEntryPointCalls::balanceOf)
                    }
                    balanceOf
                },
                {
                    fn getSenderAddress(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IEntryPointCalls> {
                        <getSenderAddressCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IEntryPointCalls::getSenderAddress)
                    }
                    getSenderAddress
                },
                {
                    fn getUserOpHash(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IEntryPointCalls> {
                        <getUserOpHashCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IEntryPointCalls::getUserOpHash)
                    }
                    getUserOpHash
                },
                {
                    fn depositTo(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IEntryPointCalls> {
                        <depositToCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IEntryPointCalls::depositTo)
                    }
                    depositTo
                },
                {
                    fn unlockStake(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IEntryPointCalls> {
                        <unlockStakeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IEntryPointCalls::unlockStake)
                    }
                    unlockStake
                },
                {
                    fn withdrawStake(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IEntryPointCalls> {
                        <withdrawStakeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IEntryPointCalls::withdrawStake)
                    }
                    withdrawStake
                },
                {
                    fn simulateHandleOp(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IEntryPointCalls> {
                        <simulateHandleOpCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IEntryPointCalls::simulateHandleOp)
                    }
                    simulateHandleOp
                },
                {
                    fn simulateValidation(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IEntryPointCalls> {
                        <simulateValidationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IEntryPointCalls::simulateValidation)
                    }
                    simulateValidation
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            (unsafe { DECODE_SHIMS.get_unchecked(idx) })(data, validate)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::addStake(inner) => {
                    <addStakeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::balanceOf(inner) => {
                    <balanceOfCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::depositTo(inner) => {
                    <depositToCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::getDepositInfo(inner) => {
                    <getDepositInfoCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getNonce(inner) => {
                    <getNonceCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::getSenderAddress(inner) => {
                    <getSenderAddressCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getUserOpHash(inner) => {
                    <getUserOpHashCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::handleAggregatedOps(inner) => {
                    <handleAggregatedOpsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::handleOps(inner) => {
                    <handleOpsCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::incrementNonce(inner) => {
                    <incrementNonceCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::simulateHandleOp(inner) => {
                    <simulateHandleOpCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::simulateValidation(inner) => {
                    <simulateValidationCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::unlockStake(inner) => {
                    <unlockStakeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::withdrawStake(inner) => {
                    <withdrawStakeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::withdrawTo(inner) => {
                    <withdrawToCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::addStake(inner) => {
                    <addStakeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::balanceOf(inner) => {
                    <balanceOfCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::depositTo(inner) => {
                    <depositToCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getDepositInfo(inner) => {
                    <getDepositInfoCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getNonce(inner) => {
                    <getNonceCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getSenderAddress(inner) => {
                    <getSenderAddressCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getUserOpHash(inner) => {
                    <getUserOpHashCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::handleAggregatedOps(inner) => {
                    <handleAggregatedOpsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::handleOps(inner) => {
                    <handleOpsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::incrementNonce(inner) => {
                    <incrementNonceCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::simulateHandleOp(inner) => {
                    <simulateHandleOpCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::simulateValidation(inner) => {
                    <simulateValidationCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::unlockStake(inner) => {
                    <unlockStakeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::withdrawStake(inner) => {
                    <withdrawStakeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::withdrawTo(inner) => {
                    <withdrawToCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`IEntryPoint`](self) custom errors.
    pub enum IEntryPointErrors {
        ExecutionResult(ExecutionResult),
        FailedOp(FailedOp),
        SenderAddressResult(SenderAddressResult),
        SignatureValidationFailed(SignatureValidationFailed),
        ValidationResult(ValidationResult),
        ValidationResultWithAggregation(ValidationResultWithAggregation),
    }
    #[automatically_derived]
    impl IEntryPointErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [34u8, 2u8, 102u8, 182u8],
            [108u8, 167u8, 184u8, 6u8],
            [134u8, 169u8, 247u8, 80u8],
            [139u8, 122u8, 201u8, 128u8],
            [224u8, 207u8, 240u8, 95u8],
            [250u8, 236u8, 180u8, 228u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for IEntryPointErrors {
        const NAME: &'static str = "IEntryPointErrors";
        const MIN_DATA_LENGTH: usize = 32usize;
        const COUNT: usize = 6usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::ExecutionResult(_) => {
                    <ExecutionResult as alloy_sol_types::SolError>::SELECTOR
                }
                Self::FailedOp(_) => <FailedOp as alloy_sol_types::SolError>::SELECTOR,
                Self::SenderAddressResult(_) => {
                    <SenderAddressResult as alloy_sol_types::SolError>::SELECTOR
                }
                Self::SignatureValidationFailed(_) => {
                    <SignatureValidationFailed as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ValidationResult(_) => {
                    <ValidationResult as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ValidationResultWithAggregation(_) => {
                    <ValidationResultWithAggregation as alloy_sol_types::SolError>::SELECTOR
                }
            }
        }
        #[inline]
        fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
            Self::SELECTORS.get(i).copied()
        }
        #[inline]
        fn valid_selector(selector: [u8; 4]) -> bool {
            Self::SELECTORS.binary_search(&selector).is_ok()
        }
        #[inline]
        #[allow(unsafe_code, non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
                bool,
            ) -> alloy_sol_types::Result<IEntryPointErrors>] = &[
                {
                    fn FailedOp(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IEntryPointErrors> {
                        <FailedOp as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IEntryPointErrors::FailedOp)
                    }
                    FailedOp
                },
                {
                    fn SenderAddressResult(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IEntryPointErrors> {
                        <SenderAddressResult as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IEntryPointErrors::SenderAddressResult)
                    }
                    SenderAddressResult
                },
                {
                    fn SignatureValidationFailed(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IEntryPointErrors> {
                        <SignatureValidationFailed as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IEntryPointErrors::SignatureValidationFailed)
                    }
                    SignatureValidationFailed
                },
                {
                    fn ExecutionResult(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IEntryPointErrors> {
                        <ExecutionResult as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IEntryPointErrors::ExecutionResult)
                    }
                    ExecutionResult
                },
                {
                    fn ValidationResult(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IEntryPointErrors> {
                        <ValidationResult as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IEntryPointErrors::ValidationResult)
                    }
                    ValidationResult
                },
                {
                    fn ValidationResultWithAggregation(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IEntryPointErrors> {
                        <ValidationResultWithAggregation as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IEntryPointErrors::ValidationResultWithAggregation)
                    }
                    ValidationResultWithAggregation
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            (unsafe { DECODE_SHIMS.get_unchecked(idx) })(data, validate)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::ExecutionResult(inner) => {
                    <ExecutionResult as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::FailedOp(inner) => {
                    <FailedOp as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::SenderAddressResult(inner) => {
                    <SenderAddressResult as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::SignatureValidationFailed(inner) => {
                    <SignatureValidationFailed as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ValidationResult(inner) => {
                    <ValidationResult as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ValidationResultWithAggregation(inner) => {
                    <ValidationResultWithAggregation as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::ExecutionResult(inner) => {
                    <ExecutionResult as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::FailedOp(inner) => {
                    <FailedOp as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::SenderAddressResult(inner) => {
                    <SenderAddressResult as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::SignatureValidationFailed(inner) => {
                    <SignatureValidationFailed as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ValidationResult(inner) => {
                    <ValidationResult as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ValidationResultWithAggregation(inner) => {
                    <ValidationResultWithAggregation as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`IEntryPoint`](self) events.
    pub enum IEntryPointEvents {
        AccountDeployed(AccountDeployed),
        BeforeExecution(BeforeExecution),
        Deposited(Deposited),
        SignatureAggregatorChanged(SignatureAggregatorChanged),
        StakeLocked(StakeLocked),
        StakeUnlocked(StakeUnlocked),
        StakeWithdrawn(StakeWithdrawn),
        UserOperationEvent(UserOperationEvent),
        UserOperationRevertReason(UserOperationRevertReason),
        Withdrawn(Withdrawn),
    }
    #[automatically_derived]
    impl IEntryPointEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                28u8,
                79u8,
                173u8,
                167u8,
                55u8,
                76u8,
                10u8,
                158u8,
                232u8,
                132u8,
                31u8,
                195u8,
                138u8,
                254u8,
                130u8,
                147u8,
                45u8,
                192u8,
                248u8,
                230u8,
                144u8,
                18u8,
                233u8,
                39u8,
                240u8,
                97u8,
                168u8,
                186u8,
                230u8,
                17u8,
                162u8,
                1u8,
            ],
            [
                45u8,
                164u8,
                102u8,
                167u8,
                178u8,
                67u8,
                4u8,
                244u8,
                126u8,
                135u8,
                250u8,
                46u8,
                30u8,
                90u8,
                129u8,
                185u8,
                131u8,
                28u8,
                229u8,
                79u8,
                236u8,
                25u8,
                5u8,
                92u8,
                226u8,
                119u8,
                202u8,
                47u8,
                57u8,
                186u8,
                66u8,
                196u8,
            ],
            [
                73u8,
                98u8,
                143u8,
                209u8,
                71u8,
                16u8,
                6u8,
                193u8,
                72u8,
                45u8,
                168u8,
                128u8,
                40u8,
                233u8,
                206u8,
                77u8,
                187u8,
                8u8,
                11u8,
                129u8,
                92u8,
                155u8,
                3u8,
                68u8,
                211u8,
                158u8,
                90u8,
                142u8,
                110u8,
                193u8,
                65u8,
                159u8,
            ],
            [
                87u8,
                95u8,
                243u8,
                172u8,
                173u8,
                213u8,
                171u8,
                52u8,
                143u8,
                225u8,
                133u8,
                94u8,
                33u8,
                126u8,
                15u8,
                54u8,
                120u8,
                248u8,
                215u8,
                103u8,
                215u8,
                73u8,
                76u8,
                159u8,
                159u8,
                239u8,
                190u8,
                226u8,
                225u8,
                124u8,
                202u8,
                77u8,
            ],
            [
                165u8,
                174u8,
                131u8,
                61u8,
                11u8,
                177u8,
                220u8,
                214u8,
                50u8,
                217u8,
                138u8,
                139u8,
                112u8,
                151u8,
                62u8,
                133u8,
                22u8,
                129u8,
                40u8,
                152u8,
                225u8,
                155u8,
                242u8,
                123u8,
                112u8,
                7u8,
                30u8,
                188u8,
                141u8,
                197u8,
                44u8,
                1u8,
            ],
            [
                183u8,
                201u8,
                24u8,
                224u8,
                226u8,
                73u8,
                249u8,
                153u8,
                233u8,
                101u8,
                202u8,
                254u8,
                182u8,
                198u8,
                100u8,
                39u8,
                27u8,
                63u8,
                67u8,
                23u8,
                210u8,
                150u8,
                70u8,
                21u8,
                0u8,
                231u8,
                29u8,
                163u8,
                159u8,
                12u8,
                189u8,
                163u8,
            ],
            [
                187u8,
                71u8,
                238u8,
                62u8,
                24u8,
                58u8,
                85u8,
                139u8,
                26u8,
                47u8,
                240u8,
                135u8,
                75u8,
                7u8,
                159u8,
                63u8,
                197u8,
                71u8,
                139u8,
                116u8,
                84u8,
                234u8,
                207u8,
                43u8,
                252u8,
                90u8,
                242u8,
                255u8,
                88u8,
                120u8,
                249u8,
                114u8,
            ],
            [
                209u8,
                193u8,
                159u8,
                188u8,
                212u8,
                85u8,
                26u8,
                94u8,
                223u8,
                182u8,
                109u8,
                67u8,
                210u8,
                227u8,
                55u8,
                192u8,
                72u8,
                55u8,
                175u8,
                218u8,
                52u8,
                130u8,
                180u8,
                43u8,
                223u8,
                86u8,
                154u8,
                143u8,
                204u8,
                218u8,
                229u8,
                251u8,
            ],
            [
                213u8,
                26u8,
                156u8,
                97u8,
                38u8,
                122u8,
                166u8,
                25u8,
                105u8,
                97u8,
                136u8,
                62u8,
                207u8,
                95u8,
                242u8,
                218u8,
                102u8,
                25u8,
                195u8,
                125u8,
                172u8,
                15u8,
                169u8,
                33u8,
                34u8,
                81u8,
                63u8,
                179u8,
                44u8,
                3u8,
                45u8,
                45u8,
            ],
            [
                250u8,
                155u8,
                60u8,
                20u8,
                204u8,
                130u8,
                92u8,
                65u8,
                44u8,
                158u8,
                216u8,
                27u8,
                59u8,
                163u8,
                101u8,
                165u8,
                180u8,
                89u8,
                67u8,
                148u8,
                3u8,
                241u8,
                136u8,
                41u8,
                229u8,
                114u8,
                237u8,
                83u8,
                164u8,
                24u8,
                15u8,
                10u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for IEntryPointEvents {
        const NAME: &'static str = "IEntryPointEvents";
        const COUNT: usize = 10usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<AccountDeployed as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <AccountDeployed as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::AccountDeployed)
                }
                Some(<BeforeExecution as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <BeforeExecution as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::BeforeExecution)
                }
                Some(<Deposited as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Deposited as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::Deposited)
                }
                Some(
                    <SignatureAggregatorChanged as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <SignatureAggregatorChanged as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::SignatureAggregatorChanged)
                }
                Some(<StakeLocked as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <StakeLocked as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::StakeLocked)
                }
                Some(<StakeUnlocked as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <StakeUnlocked as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::StakeUnlocked)
                }
                Some(<StakeWithdrawn as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <StakeWithdrawn as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::StakeWithdrawn)
                }
                Some(
                    <UserOperationEvent as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <UserOperationEvent as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::UserOperationEvent)
                }
                Some(
                    <UserOperationRevertReason as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <UserOperationRevertReason as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::UserOperationRevertReason)
                }
                Some(<Withdrawn as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Withdrawn as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::Withdrawn)
                }
                _ => {
                    alloy_sol_types::private::Err(alloy_sol_types::Error::InvalidLog {
                        name: <Self as alloy_sol_types::SolEventInterface>::NAME,
                        log: alloy_sol_types::private::Box::new(
                            alloy_sol_types::private::LogData::new_unchecked(
                                topics.to_vec(),
                                data.to_vec().into(),
                            ),
                        ),
                    })
                }
            }
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::private::IntoLogData for IEntryPointEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::AccountDeployed(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::BeforeExecution(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Deposited(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::SignatureAggregatorChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::StakeLocked(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::StakeUnlocked(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::StakeWithdrawn(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::UserOperationEvent(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::UserOperationRevertReason(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Withdrawn(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::AccountDeployed(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::BeforeExecution(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Deposited(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::SignatureAggregatorChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::StakeLocked(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::StakeUnlocked(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::StakeWithdrawn(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::UserOperationEvent(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::UserOperationRevertReason(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Withdrawn(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`IEntryPoint`](self) contract instance.

See the [wrapper's documentation](`IEntryPointInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IEntryPointInstance<T, P, N> {
        IEntryPointInstance::<T, P, N>::new(address, provider)
    }
    /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
    #[inline]
    pub fn deploy<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        provider: P,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<IEntryPointInstance<T, P, N>>,
    > {
        IEntryPointInstance::<T, P, N>::deploy(provider)
    }
    /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
    #[inline]
    pub fn deploy_builder<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(provider: P) -> alloy_contract::RawCallBuilder<T, P, N> {
        IEntryPointInstance::<T, P, N>::deploy_builder(provider)
    }
    /**A [`IEntryPoint`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`IEntryPoint`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IEntryPointInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for IEntryPointInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IEntryPointInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IEntryPointInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`IEntryPoint`](self) contract instance.

See the [wrapper's documentation](`IEntryPointInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            provider: P,
        ) -> Self {
            Self {
                address,
                provider,
                _network_transport: ::core::marker::PhantomData,
            }
        }
        /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
        #[inline]
        pub async fn deploy(
            provider: P,
        ) -> alloy_contract::Result<IEntryPointInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(provider);
            let contract_address = call_builder.deploy().await?;
            Ok(Self::new(contract_address, call_builder.provider))
        }
        /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
        #[inline]
        pub fn deploy_builder(provider: P) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                ::core::clone::Clone::clone(&BYTECODE),
            )
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<T, P: ::core::clone::Clone, N> IEntryPointInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IEntryPointInstance<T, P, N> {
            IEntryPointInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IEntryPointInstance<T, P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
        ///Creates a new call builder for the [`addStake`] function.
        pub fn addStake(
            &self,
            _unstakeDelaySec: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, addStakeCall, N> {
            self.call_builder(&addStakeCall { _unstakeDelaySec })
        }
        ///Creates a new call builder for the [`balanceOf`] function.
        pub fn balanceOf(
            &self,
            account: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, balanceOfCall, N> {
            self.call_builder(&balanceOfCall { account })
        }
        ///Creates a new call builder for the [`depositTo`] function.
        pub fn depositTo(
            &self,
            account: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, depositToCall, N> {
            self.call_builder(&depositToCall { account })
        }
        ///Creates a new call builder for the [`getDepositInfo`] function.
        pub fn getDepositInfo(
            &self,
            account: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getDepositInfoCall, N> {
            self.call_builder(&getDepositInfoCall { account })
        }
        ///Creates a new call builder for the [`getNonce`] function.
        pub fn getNonce(
            &self,
            sender: alloy::sol_types::private::Address,
            key: alloy::sol_types::private::primitives::aliases::U192,
        ) -> alloy_contract::SolCallBuilder<T, &P, getNonceCall, N> {
            self.call_builder(&getNonceCall { sender, key })
        }
        ///Creates a new call builder for the [`getSenderAddress`] function.
        pub fn getSenderAddress(
            &self,
            initCode: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, getSenderAddressCall, N> {
            self.call_builder(&getSenderAddressCall { initCode })
        }
        ///Creates a new call builder for the [`getUserOpHash`] function.
        pub fn getUserOpHash(
            &self,
            userOp: <UserOperation as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, getUserOpHashCall, N> {
            self.call_builder(&getUserOpHashCall { userOp })
        }
        ///Creates a new call builder for the [`handleAggregatedOps`] function.
        pub fn handleAggregatedOps(
            &self,
            opsPerAggregator: alloy::sol_types::private::Vec<
                <UserOpsPerAggregator as alloy::sol_types::SolType>::RustType,
            >,
            beneficiary: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, handleAggregatedOpsCall, N> {
            self.call_builder(
                &handleAggregatedOpsCall {
                    opsPerAggregator,
                    beneficiary,
                },
            )
        }
        ///Creates a new call builder for the [`handleOps`] function.
        pub fn handleOps(
            &self,
            ops: alloy::sol_types::private::Vec<
                <UserOperation as alloy::sol_types::SolType>::RustType,
            >,
            beneficiary: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, handleOpsCall, N> {
            self.call_builder(&handleOpsCall { ops, beneficiary })
        }
        ///Creates a new call builder for the [`incrementNonce`] function.
        pub fn incrementNonce(
            &self,
            key: alloy::sol_types::private::primitives::aliases::U192,
        ) -> alloy_contract::SolCallBuilder<T, &P, incrementNonceCall, N> {
            self.call_builder(&incrementNonceCall { key })
        }
        ///Creates a new call builder for the [`simulateHandleOp`] function.
        pub fn simulateHandleOp(
            &self,
            op: <UserOperation as alloy::sol_types::SolType>::RustType,
            target: alloy::sol_types::private::Address,
            targetCallData: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, simulateHandleOpCall, N> {
            self.call_builder(
                &simulateHandleOpCall {
                    op,
                    target,
                    targetCallData,
                },
            )
        }
        ///Creates a new call builder for the [`simulateValidation`] function.
        pub fn simulateValidation(
            &self,
            userOp: <UserOperation as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, simulateValidationCall, N> {
            self.call_builder(&simulateValidationCall { userOp })
        }
        ///Creates a new call builder for the [`unlockStake`] function.
        pub fn unlockStake(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, unlockStakeCall, N> {
            self.call_builder(&unlockStakeCall {})
        }
        ///Creates a new call builder for the [`withdrawStake`] function.
        pub fn withdrawStake(
            &self,
            withdrawAddress: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, withdrawStakeCall, N> {
            self.call_builder(
                &withdrawStakeCall {
                    withdrawAddress,
                },
            )
        }
        ///Creates a new call builder for the [`withdrawTo`] function.
        pub fn withdrawTo(
            &self,
            withdrawAddress: alloy::sol_types::private::Address,
            withdrawAmount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, withdrawToCall, N> {
            self.call_builder(
                &withdrawToCall {
                    withdrawAddress,
                    withdrawAmount,
                },
            )
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IEntryPointInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`AccountDeployed`] event.
        pub fn AccountDeployed_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, AccountDeployed, N> {
            self.event_filter::<AccountDeployed>()
        }
        ///Creates a new event filter for the [`BeforeExecution`] event.
        pub fn BeforeExecution_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, BeforeExecution, N> {
            self.event_filter::<BeforeExecution>()
        }
        ///Creates a new event filter for the [`Deposited`] event.
        pub fn Deposited_filter(&self) -> alloy_contract::Event<T, &P, Deposited, N> {
            self.event_filter::<Deposited>()
        }
        ///Creates a new event filter for the [`SignatureAggregatorChanged`] event.
        pub fn SignatureAggregatorChanged_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, SignatureAggregatorChanged, N> {
            self.event_filter::<SignatureAggregatorChanged>()
        }
        ///Creates a new event filter for the [`StakeLocked`] event.
        pub fn StakeLocked_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, StakeLocked, N> {
            self.event_filter::<StakeLocked>()
        }
        ///Creates a new event filter for the [`StakeUnlocked`] event.
        pub fn StakeUnlocked_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, StakeUnlocked, N> {
            self.event_filter::<StakeUnlocked>()
        }
        ///Creates a new event filter for the [`StakeWithdrawn`] event.
        pub fn StakeWithdrawn_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, StakeWithdrawn, N> {
            self.event_filter::<StakeWithdrawn>()
        }
        ///Creates a new event filter for the [`UserOperationEvent`] event.
        pub fn UserOperationEvent_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, UserOperationEvent, N> {
            self.event_filter::<UserOperationEvent>()
        }
        ///Creates a new event filter for the [`UserOperationRevertReason`] event.
        pub fn UserOperationRevertReason_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, UserOperationRevertReason, N> {
            self.event_filter::<UserOperationRevertReason>()
        }
        ///Creates a new event filter for the [`Withdrawn`] event.
        pub fn Withdrawn_filter(&self) -> alloy_contract::Event<T, &P, Withdrawn, N> {
            self.event_filter::<Withdrawn>()
        }
    }
}
