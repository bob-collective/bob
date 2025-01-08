///Module containing a contract's types and functions.
/**

```solidity
library GsnTypes {
    struct RelayData { uint256 maxFeePerGas; uint256 maxPriorityFeePerGas; uint256 transactionCalldataGasUsed; address relayWorker; address paymaster; address forwarder; bytes paymasterData; uint256 clientId; }
    struct RelayRequest { IForwarder.ForwardRequest request; RelayData relayData; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style
)]
pub mod GsnTypes {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
struct RelayData { uint256 maxFeePerGas; uint256 maxPriorityFeePerGas; uint256 transactionCalldataGasUsed; address relayWorker; address paymaster; address forwarder; bytes paymasterData; uint256 clientId; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct RelayData {
        pub maxFeePerGas: alloy::sol_types::private::primitives::aliases::U256,
        pub maxPriorityFeePerGas: alloy::sol_types::private::primitives::aliases::U256,
        pub transactionCalldataGasUsed: alloy::sol_types::private::primitives::aliases::U256,
        pub relayWorker: alloy::sol_types::private::Address,
        pub paymaster: alloy::sol_types::private::Address,
        pub forwarder: alloy::sol_types::private::Address,
        pub paymasterData: alloy::sol_types::private::Bytes,
        pub clientId: alloy::sol_types::private::primitives::aliases::U256,
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
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Bytes,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Bytes,
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
        impl ::core::convert::From<RelayData> for UnderlyingRustTuple<'_> {
            fn from(value: RelayData) -> Self {
                (
                    value.maxFeePerGas,
                    value.maxPriorityFeePerGas,
                    value.transactionCalldataGasUsed,
                    value.relayWorker,
                    value.paymaster,
                    value.forwarder,
                    value.paymasterData,
                    value.clientId,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for RelayData {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    maxFeePerGas: tuple.0,
                    maxPriorityFeePerGas: tuple.1,
                    transactionCalldataGasUsed: tuple.2,
                    relayWorker: tuple.3,
                    paymaster: tuple.4,
                    forwarder: tuple.5,
                    paymasterData: tuple.6,
                    clientId: tuple.7,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for RelayData {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for RelayData {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.maxFeePerGas),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.maxPriorityFeePerGas),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.transactionCalldataGasUsed,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.relayWorker,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.paymaster,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.forwarder,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.paymasterData,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.clientId),
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
        impl alloy_sol_types::SolType for RelayData {
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
        impl alloy_sol_types::SolStruct for RelayData {
            const NAME: &'static str = "RelayData";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "RelayData(uint256 maxFeePerGas,uint256 maxPriorityFeePerGas,uint256 transactionCalldataGasUsed,address relayWorker,address paymaster,address forwarder,bytes paymasterData,uint256 clientId)",
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
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.maxFeePerGas)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.maxPriorityFeePerGas,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.transactionCalldataGasUsed,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.relayWorker,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.paymaster,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.forwarder,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.paymasterData,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.clientId)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for RelayData {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
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
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.transactionCalldataGasUsed,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.relayWorker,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.paymaster,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.forwarder,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.paymasterData,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.clientId,
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
                    &rust.maxFeePerGas,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.maxPriorityFeePerGas,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.transactionCalldataGasUsed,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.relayWorker,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.paymaster,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.forwarder,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.paymasterData,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.clientId,
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
struct RelayRequest { IForwarder.ForwardRequest request; RelayData relayData; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct RelayRequest {
        pub request: <IForwarder::ForwardRequest as alloy::sol_types::SolType>::RustType,
        pub relayData: <RelayData as alloy::sol_types::SolType>::RustType,
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
        type UnderlyingSolTuple<'a> = (IForwarder::ForwardRequest, RelayData);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            <IForwarder::ForwardRequest as alloy::sol_types::SolType>::RustType,
            <RelayData as alloy::sol_types::SolType>::RustType,
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
        impl ::core::convert::From<RelayRequest> for UnderlyingRustTuple<'_> {
            fn from(value: RelayRequest) -> Self {
                (value.request, value.relayData)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for RelayRequest {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    request: tuple.0,
                    relayData: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for RelayRequest {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for RelayRequest {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <IForwarder::ForwardRequest as alloy_sol_types::SolType>::tokenize(
                        &self.request,
                    ),
                    <RelayData as alloy_sol_types::SolType>::tokenize(&self.relayData),
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
        impl alloy_sol_types::SolType for RelayRequest {
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
        impl alloy_sol_types::SolStruct for RelayRequest {
            const NAME: &'static str = "RelayRequest";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "RelayRequest(IForwarder.ForwardRequest request,RelayData relayData)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(2);
                components
                    .push(
                        <IForwarder::ForwardRequest as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <IForwarder::ForwardRequest as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
                    .push(<RelayData as alloy_sol_types::SolStruct>::eip712_root_type());
                components
                    .extend(
                        <RelayData as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <IForwarder::ForwardRequest as alloy_sol_types::SolType>::eip712_data_word(
                            &self.request,
                        )
                        .0,
                    <RelayData as alloy_sol_types::SolType>::eip712_data_word(
                            &self.relayData,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for RelayRequest {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <IForwarder::ForwardRequest as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.request,
                    )
                    + <RelayData as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.relayData,
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
                <IForwarder::ForwardRequest as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.request,
                    out,
                );
                <RelayData as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.relayData,
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
    /**Creates a new wrapper around an on-chain [`GsnTypes`](self) contract instance.

See the [wrapper's documentation](`GsnTypesInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> GsnTypesInstance<T, P, N> {
        GsnTypesInstance::<T, P, N>::new(address, provider)
    }
    /**A [`GsnTypes`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`GsnTypes`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct GsnTypesInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for GsnTypesInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("GsnTypesInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > GsnTypesInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`GsnTypes`](self) contract instance.

See the [wrapper's documentation](`GsnTypesInstance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> GsnTypesInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> GsnTypesInstance<T, P, N> {
            GsnTypesInstance {
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
    > GsnTypesInstance<T, P, N> {
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
    > GsnTypesInstance<T, P, N> {
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
///Module containing a contract's types and functions.
/**

```solidity
library IForwarder {
    struct ForwardRequest { address from; address to; uint256 value; uint256 gas; uint256 nonce; bytes data; uint256 validUntilTime; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style
)]
pub mod IForwarder {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
struct ForwardRequest { address from; address to; uint256 value; uint256 gas; uint256 nonce; bytes data; uint256 validUntilTime; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ForwardRequest {
        pub from: alloy::sol_types::private::Address,
        pub to: alloy::sol_types::private::Address,
        pub value: alloy::sol_types::private::primitives::aliases::U256,
        pub gas: alloy::sol_types::private::primitives::aliases::U256,
        pub nonce: alloy::sol_types::private::primitives::aliases::U256,
        pub data: alloy::sol_types::private::Bytes,
        pub validUntilTime: alloy::sol_types::private::primitives::aliases::U256,
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
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Bytes,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::Bytes,
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
        impl ::core::convert::From<ForwardRequest> for UnderlyingRustTuple<'_> {
            fn from(value: ForwardRequest) -> Self {
                (
                    value.from,
                    value.to,
                    value.value,
                    value.gas,
                    value.nonce,
                    value.data,
                    value.validUntilTime,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ForwardRequest {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    from: tuple.0,
                    to: tuple.1,
                    value: tuple.2,
                    gas: tuple.3,
                    nonce: tuple.4,
                    data: tuple.5,
                    validUntilTime: tuple.6,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for ForwardRequest {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for ForwardRequest {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.from,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.to,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.value),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.gas),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.nonce),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.data,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.validUntilTime),
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
        impl alloy_sol_types::SolType for ForwardRequest {
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
        impl alloy_sol_types::SolStruct for ForwardRequest {
            const NAME: &'static str = "ForwardRequest";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "ForwardRequest(address from,address to,uint256 value,uint256 gas,uint256 nonce,bytes data,uint256 validUntilTime)",
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
                            &self.from,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.to,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.value)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.gas)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.nonce)
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.data,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.validUntilTime,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for ForwardRequest {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.from,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.to,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.value)
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.gas)
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.nonce)
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.data,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.validUntilTime,
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
                    &rust.from,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.to,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.value,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.gas, out);
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.nonce,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.data,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.validUntilTime,
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
    /**Creates a new wrapper around an on-chain [`IForwarder`](self) contract instance.

See the [wrapper's documentation](`IForwarderInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IForwarderInstance<T, P, N> {
        IForwarderInstance::<T, P, N>::new(address, provider)
    }
    /**A [`IForwarder`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`IForwarder`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IForwarderInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for IForwarderInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IForwarderInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IForwarderInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`IForwarder`](self) contract instance.

See the [wrapper's documentation](`IForwarderInstance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> IForwarderInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IForwarderInstance<T, P, N> {
            IForwarderInstance {
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
    > IForwarderInstance<T, P, N> {
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
    > IForwarderInstance<T, P, N> {
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
library GsnTypes {
    struct RelayData {
        uint256 maxFeePerGas;
        uint256 maxPriorityFeePerGas;
        uint256 transactionCalldataGasUsed;
        address relayWorker;
        address paymaster;
        address forwarder;
        bytes paymasterData;
        uint256 clientId;
    }
    struct RelayRequest {
        IForwarder.ForwardRequest request;
        RelayData relayData;
    }
}

library IForwarder {
    struct ForwardRequest {
        address from;
        address to;
        uint256 value;
        uint256 gas;
        uint256 nonce;
        bytes data;
        uint256 validUntilTime;
    }
}

interface IRelayHub {
    type RelayCallStatus is uint8;
    struct RelayHubConfig {
        uint256 maxWorkerCount;
        uint256 gasReserve;
        uint256 postOverhead;
        uint256 gasOverhead;
        uint256 minimumUnstakeDelay;
        address devAddress;
        uint8 devFee;
        uint80 baseRelayFee;
        uint16 pctRelayFee;
    }

    event AbandonedRelayManagerBalanceEscheated(address indexed relayManager, uint256 balance);
    event Deposited(address indexed paymaster, address indexed from, uint256 amount);
    event HubDeprecated(uint256 deprecationTime);
    event RelayHubConfigured(RelayHubConfig config);
    event RelayWorkersAdded(address indexed relayManager, address[] newRelayWorkers, uint256 workersCount);
    event StakingTokenDataChanged(address token, uint256 minimumStake);
    event TransactionRejectedByPaymaster(address indexed relayManager, address indexed paymaster, bytes32 indexed relayRequestID, address from, address to, address relayWorker, bytes4 selector, uint256 innerGasUsed, bytes reason);
    event TransactionRelayed(address indexed relayManager, address indexed relayWorker, bytes32 indexed relayRequestID, address from, address to, address paymaster, bytes4 selector, RelayCallStatus status, uint256 charge);
    event TransactionResult(RelayCallStatus status, bytes returnValue);
    event Withdrawn(address indexed account, address indexed dest, uint256 amount);

    function addRelayWorkers(address[] memory newRelayWorkers) external;
    function aggregateGasleft() external view returns (uint256);
    function balanceOf(address target) external view returns (uint256);
    function calculateCharge(uint256 gasUsed, GsnTypes.RelayData memory relayData) external view returns (uint256);
    function calculateDevCharge(uint256 charge) external view returns (uint256);
    function depositFor(address target) external payable;
    function deprecateHub(uint256 _deprecationTime) external;
    function escheatAbandonedRelayBalance(address relayManager) external;
    function getBatchGateway() external view returns (address);
    function getConfiguration() external view returns (RelayHubConfig memory config);
    function getCreationBlock() external view returns (uint256);
    function getDeprecationTime() external view returns (uint256);
    function getMinimumStakePerToken(address token) external view returns (uint256);
    function getPenalizer() external view returns (address);
    function getRelayRegistrar() external view returns (address);
    function getStakeManager() external view returns (address);
    function getWorkerCount(address manager) external view returns (uint256);
    function getWorkerManager(address worker) external view returns (address);
    function isDeprecated() external view returns (bool);
    function isRelayEscheatable(address relayManager) external view returns (bool);
    function onRelayServerRegistered(address relayManager) external;
    function penalize(address relayWorker, address payable beneficiary) external;
    function relayCall(string memory domainSeparatorName, uint256 maxAcceptanceBudget, GsnTypes.RelayRequest memory relayRequest, bytes memory signature, bytes memory approvalData) external returns (bool paymasterAccepted, uint256 charge, RelayCallStatus status, bytes memory returnValue);
    function setConfiguration(RelayHubConfig memory _config) external;
    function setMinimumStakes(address[] memory token, uint256[] memory minimumStake) external;
    function supportsInterface(bytes4 interfaceId) external view returns (bool);
    function verifyRelayManagerStaked(address relayManager) external view;
    function versionHub() external view returns (string memory);
    function withdraw(address payable dest, uint256 amount) external;
    function withdrawMultiple(address payable[] memory dest, uint256[] memory amount) external;
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "function",
    "name": "addRelayWorkers",
    "inputs": [
      {
        "name": "newRelayWorkers",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "aggregateGasleft",
    "inputs": [],
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
    "name": "balanceOf",
    "inputs": [
      {
        "name": "target",
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
    "name": "calculateCharge",
    "inputs": [
      {
        "name": "gasUsed",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "relayData",
        "type": "tuple",
        "internalType": "struct GsnTypes.RelayData",
        "components": [
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
            "name": "transactionCalldataGasUsed",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "relayWorker",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "paymaster",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "forwarder",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "paymasterData",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "clientId",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
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
    "name": "calculateDevCharge",
    "inputs": [
      {
        "name": "charge",
        "type": "uint256",
        "internalType": "uint256"
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
    "name": "depositFor",
    "inputs": [
      {
        "name": "target",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "deprecateHub",
    "inputs": [
      {
        "name": "_deprecationTime",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "escheatAbandonedRelayBalance",
    "inputs": [
      {
        "name": "relayManager",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getBatchGateway",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getConfiguration",
    "inputs": [],
    "outputs": [
      {
        "name": "config",
        "type": "tuple",
        "internalType": "struct IRelayHub.RelayHubConfig",
        "components": [
          {
            "name": "maxWorkerCount",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "gasReserve",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "postOverhead",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "gasOverhead",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "minimumUnstakeDelay",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "devAddress",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "devFee",
            "type": "uint8",
            "internalType": "uint8"
          },
          {
            "name": "baseRelayFee",
            "type": "uint80",
            "internalType": "uint80"
          },
          {
            "name": "pctRelayFee",
            "type": "uint16",
            "internalType": "uint16"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getCreationBlock",
    "inputs": [],
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
    "name": "getDeprecationTime",
    "inputs": [],
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
    "name": "getMinimumStakePerToken",
    "inputs": [
      {
        "name": "token",
        "type": "address",
        "internalType": "contract IERC20"
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
    "name": "getPenalizer",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getRelayRegistrar",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getStakeManager",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IStakeManager"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getWorkerCount",
    "inputs": [
      {
        "name": "manager",
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
    "name": "getWorkerManager",
    "inputs": [
      {
        "name": "worker",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "isDeprecated",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "isRelayEscheatable",
    "inputs": [
      {
        "name": "relayManager",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "onRelayServerRegistered",
    "inputs": [
      {
        "name": "relayManager",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "penalize",
    "inputs": [
      {
        "name": "relayWorker",
        "type": "address",
        "internalType": "address"
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
    "name": "relayCall",
    "inputs": [
      {
        "name": "domainSeparatorName",
        "type": "string",
        "internalType": "string"
      },
      {
        "name": "maxAcceptanceBudget",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "relayRequest",
        "type": "tuple",
        "internalType": "struct GsnTypes.RelayRequest",
        "components": [
          {
            "name": "request",
            "type": "tuple",
            "internalType": "struct IForwarder.ForwardRequest",
            "components": [
              {
                "name": "from",
                "type": "address",
                "internalType": "address"
              },
              {
                "name": "to",
                "type": "address",
                "internalType": "address"
              },
              {
                "name": "value",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "gas",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "nonce",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "data",
                "type": "bytes",
                "internalType": "bytes"
              },
              {
                "name": "validUntilTime",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          },
          {
            "name": "relayData",
            "type": "tuple",
            "internalType": "struct GsnTypes.RelayData",
            "components": [
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
                "name": "transactionCalldataGasUsed",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "relayWorker",
                "type": "address",
                "internalType": "address"
              },
              {
                "name": "paymaster",
                "type": "address",
                "internalType": "address"
              },
              {
                "name": "forwarder",
                "type": "address",
                "internalType": "address"
              },
              {
                "name": "paymasterData",
                "type": "bytes",
                "internalType": "bytes"
              },
              {
                "name": "clientId",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          }
        ]
      },
      {
        "name": "signature",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "approvalData",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [
      {
        "name": "paymasterAccepted",
        "type": "bool",
        "internalType": "bool"
      },
      {
        "name": "charge",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "status",
        "type": "uint8",
        "internalType": "enum IRelayHub.RelayCallStatus"
      },
      {
        "name": "returnValue",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setConfiguration",
    "inputs": [
      {
        "name": "_config",
        "type": "tuple",
        "internalType": "struct IRelayHub.RelayHubConfig",
        "components": [
          {
            "name": "maxWorkerCount",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "gasReserve",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "postOverhead",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "gasOverhead",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "minimumUnstakeDelay",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "devAddress",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "devFee",
            "type": "uint8",
            "internalType": "uint8"
          },
          {
            "name": "baseRelayFee",
            "type": "uint80",
            "internalType": "uint80"
          },
          {
            "name": "pctRelayFee",
            "type": "uint16",
            "internalType": "uint16"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setMinimumStakes",
    "inputs": [
      {
        "name": "token",
        "type": "address[]",
        "internalType": "contract IERC20[]"
      },
      {
        "name": "minimumStake",
        "type": "uint256[]",
        "internalType": "uint256[]"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "supportsInterface",
    "inputs": [
      {
        "name": "interfaceId",
        "type": "bytes4",
        "internalType": "bytes4"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "verifyRelayManagerStaked",
    "inputs": [
      {
        "name": "relayManager",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "versionHub",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "string",
        "internalType": "string"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "withdraw",
    "inputs": [
      {
        "name": "dest",
        "type": "address",
        "internalType": "address payable"
      },
      {
        "name": "amount",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "withdrawMultiple",
    "inputs": [
      {
        "name": "dest",
        "type": "address[]",
        "internalType": "address payable[]"
      },
      {
        "name": "amount",
        "type": "uint256[]",
        "internalType": "uint256[]"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "event",
    "name": "AbandonedRelayManagerBalanceEscheated",
    "inputs": [
      {
        "name": "relayManager",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "balance",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Deposited",
    "inputs": [
      {
        "name": "paymaster",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "from",
        "type": "address",
        "indexed": true,
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
    "name": "HubDeprecated",
    "inputs": [
      {
        "name": "deprecationTime",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "RelayHubConfigured",
    "inputs": [
      {
        "name": "config",
        "type": "tuple",
        "indexed": false,
        "internalType": "struct IRelayHub.RelayHubConfig",
        "components": [
          {
            "name": "maxWorkerCount",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "gasReserve",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "postOverhead",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "gasOverhead",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "minimumUnstakeDelay",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "devAddress",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "devFee",
            "type": "uint8",
            "internalType": "uint8"
          },
          {
            "name": "baseRelayFee",
            "type": "uint80",
            "internalType": "uint80"
          },
          {
            "name": "pctRelayFee",
            "type": "uint16",
            "internalType": "uint16"
          }
        ]
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "RelayWorkersAdded",
    "inputs": [
      {
        "name": "relayManager",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "newRelayWorkers",
        "type": "address[]",
        "indexed": false,
        "internalType": "address[]"
      },
      {
        "name": "workersCount",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "StakingTokenDataChanged",
    "inputs": [
      {
        "name": "token",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "minimumStake",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "TransactionRejectedByPaymaster",
    "inputs": [
      {
        "name": "relayManager",
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
        "name": "relayRequestID",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "from",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "to",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "relayWorker",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "selector",
        "type": "bytes4",
        "indexed": false,
        "internalType": "bytes4"
      },
      {
        "name": "innerGasUsed",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "reason",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "TransactionRelayed",
    "inputs": [
      {
        "name": "relayManager",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "relayWorker",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "relayRequestID",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "from",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "to",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "paymaster",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "selector",
        "type": "bytes4",
        "indexed": false,
        "internalType": "bytes4"
      },
      {
        "name": "status",
        "type": "uint8",
        "indexed": false,
        "internalType": "enum IRelayHub.RelayCallStatus"
      },
      {
        "name": "charge",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "TransactionResult",
    "inputs": [
      {
        "name": "status",
        "type": "uint8",
        "indexed": false,
        "internalType": "enum IRelayHub.RelayCallStatus"
      },
      {
        "name": "returnValue",
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
        "name": "dest",
        "type": "address",
        "indexed": true,
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
  }
]
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style
)]
pub mod IRelayHub {
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
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct RelayCallStatus(u8);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<RelayCallStatus> for u8 {
            #[inline]
            fn stv_to_tokens(
                &self,
            ) -> <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::Token<'_> {
                alloy_sol_types::private::SolTypeValue::<
                    alloy::sol_types::sol_data::Uint<8>,
                >::stv_to_tokens(self)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::tokenize(self)
                    .0
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(self, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::abi_encoded_size(self)
            }
        }
        #[automatically_derived]
        impl RelayCallStatus {
            /// The Solidity type name.
            pub const NAME: &'static str = stringify!(@ name);
            /// Convert from the underlying value type.
            #[inline]
            pub const fn from(value: u8) -> Self {
                Self(value)
            }
            /// Return the underlying value.
            #[inline]
            pub const fn into(self) -> u8 {
                self.0
            }
            /// Return the single encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode(&self.0)
            }
            /// Return the packed encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode_packed(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode_packed(&self.0)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for RelayCallStatus {
            type RustType = u8;
            type Token<'a> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = Self::NAME;
            const ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                Self::type_check(token).is_ok()
            }
            #[inline]
            fn type_check(token: &Self::Token<'_>) -> alloy_sol_types::Result<()> {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::type_check(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::detokenize(token)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for RelayCallStatus {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::topic_preimage_length(rust)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, out)
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic(rust)
            }
        }
    };
    /**```solidity
struct RelayHubConfig { uint256 maxWorkerCount; uint256 gasReserve; uint256 postOverhead; uint256 gasOverhead; uint256 minimumUnstakeDelay; address devAddress; uint8 devFee; uint80 baseRelayFee; uint16 pctRelayFee; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct RelayHubConfig {
        pub maxWorkerCount: alloy::sol_types::private::primitives::aliases::U256,
        pub gasReserve: alloy::sol_types::private::primitives::aliases::U256,
        pub postOverhead: alloy::sol_types::private::primitives::aliases::U256,
        pub gasOverhead: alloy::sol_types::private::primitives::aliases::U256,
        pub minimumUnstakeDelay: alloy::sol_types::private::primitives::aliases::U256,
        pub devAddress: alloy::sol_types::private::Address,
        pub devFee: u8,
        pub baseRelayFee: alloy::sol_types::private::primitives::aliases::U80,
        pub pctRelayFee: u16,
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
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<8>,
            alloy::sol_types::sol_data::Uint<80>,
            alloy::sol_types::sol_data::Uint<16>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::Address,
            u8,
            alloy::sol_types::private::primitives::aliases::U80,
            u16,
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
        impl ::core::convert::From<RelayHubConfig> for UnderlyingRustTuple<'_> {
            fn from(value: RelayHubConfig) -> Self {
                (
                    value.maxWorkerCount,
                    value.gasReserve,
                    value.postOverhead,
                    value.gasOverhead,
                    value.minimumUnstakeDelay,
                    value.devAddress,
                    value.devFee,
                    value.baseRelayFee,
                    value.pctRelayFee,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for RelayHubConfig {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    maxWorkerCount: tuple.0,
                    gasReserve: tuple.1,
                    postOverhead: tuple.2,
                    gasOverhead: tuple.3,
                    minimumUnstakeDelay: tuple.4,
                    devAddress: tuple.5,
                    devFee: tuple.6,
                    baseRelayFee: tuple.7,
                    pctRelayFee: tuple.8,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for RelayHubConfig {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for RelayHubConfig {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.maxWorkerCount),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.gasReserve),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.postOverhead),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.gasOverhead),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.minimumUnstakeDelay),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.devAddress,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.devFee),
                    <alloy::sol_types::sol_data::Uint<
                        80,
                    > as alloy_sol_types::SolType>::tokenize(&self.baseRelayFee),
                    <alloy::sol_types::sol_data::Uint<
                        16,
                    > as alloy_sol_types::SolType>::tokenize(&self.pctRelayFee),
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
        impl alloy_sol_types::SolType for RelayHubConfig {
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
        impl alloy_sol_types::SolStruct for RelayHubConfig {
            const NAME: &'static str = "RelayHubConfig";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "RelayHubConfig(uint256 maxWorkerCount,uint256 gasReserve,uint256 postOverhead,uint256 gasOverhead,uint256 minimumUnstakeDelay,address devAddress,uint8 devFee,uint80 baseRelayFee,uint16 pctRelayFee)",
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
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.maxWorkerCount,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.gasReserve)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.postOverhead)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.gasOverhead)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.minimumUnstakeDelay,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.devAddress,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.devFee)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        80,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.baseRelayFee)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        16,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.pctRelayFee)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for RelayHubConfig {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.maxWorkerCount,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.gasReserve,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.postOverhead,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.gasOverhead,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.minimumUnstakeDelay,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.devAddress,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.devFee,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        80,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.baseRelayFee,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        16,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.pctRelayFee,
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
                    &rust.maxWorkerCount,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.gasReserve,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.postOverhead,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.gasOverhead,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.minimumUnstakeDelay,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.devAddress,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.devFee,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    80,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.baseRelayFee,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    16,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.pctRelayFee,
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
    /**Event with signature `AbandonedRelayManagerBalanceEscheated(address,uint256)` and selector `0x3fb672a061b9c3ab083062a9a8ff532d341e5fa42145b91f3922a711ae135659`.
```solidity
event AbandonedRelayManagerBalanceEscheated(address indexed relayManager, uint256 balance);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct AbandonedRelayManagerBalanceEscheated {
        #[allow(missing_docs)]
        pub relayManager: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub balance: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for AbandonedRelayManagerBalanceEscheated {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "AbandonedRelayManagerBalanceEscheated(address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                63u8,
                182u8,
                114u8,
                160u8,
                97u8,
                185u8,
                195u8,
                171u8,
                8u8,
                48u8,
                98u8,
                169u8,
                168u8,
                255u8,
                83u8,
                45u8,
                52u8,
                30u8,
                95u8,
                164u8,
                33u8,
                69u8,
                185u8,
                31u8,
                57u8,
                34u8,
                167u8,
                17u8,
                174u8,
                19u8,
                86u8,
                89u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    relayManager: topics.1,
                    balance: data.0,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.balance),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.relayManager.clone())
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
                    &self.relayManager,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData
        for AbandonedRelayManagerBalanceEscheated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&AbandonedRelayManagerBalanceEscheated>
        for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &AbandonedRelayManagerBalanceEscheated,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `Deposited(address,address,uint256)` and selector `0x8752a472e571a816aea92eec8dae9baf628e840f4929fbcc2d155e6233ff68a7`.
```solidity
event Deposited(address indexed paymaster, address indexed from, uint256 amount);
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
        pub paymaster: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub from: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for Deposited {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "Deposited(address,address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                135u8,
                82u8,
                164u8,
                114u8,
                229u8,
                113u8,
                168u8,
                22u8,
                174u8,
                169u8,
                46u8,
                236u8,
                141u8,
                174u8,
                155u8,
                175u8,
                98u8,
                142u8,
                132u8,
                15u8,
                73u8,
                41u8,
                251u8,
                204u8,
                45u8,
                21u8,
                94u8,
                98u8,
                51u8,
                255u8,
                104u8,
                167u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    paymaster: topics.1,
                    from: topics.2,
                    amount: data.0,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.paymaster.clone(), self.from.clone())
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
                    &self.paymaster,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.from,
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
    /**Event with signature `HubDeprecated(uint256)` and selector `0x1c0aa0c666483fbf0cf795d9d646ea3552d1e3008162ba9ab1d6d6dfd8c6ec6b`.
```solidity
event HubDeprecated(uint256 deprecationTime);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct HubDeprecated {
        #[allow(missing_docs)]
        pub deprecationTime: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for HubDeprecated {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "HubDeprecated(uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                28u8,
                10u8,
                160u8,
                198u8,
                102u8,
                72u8,
                63u8,
                191u8,
                12u8,
                247u8,
                149u8,
                217u8,
                214u8,
                70u8,
                234u8,
                53u8,
                82u8,
                209u8,
                227u8,
                0u8,
                129u8,
                98u8,
                186u8,
                154u8,
                177u8,
                214u8,
                214u8,
                223u8,
                216u8,
                198u8,
                236u8,
                107u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { deprecationTime: data.0 }
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
                    > as alloy_sol_types::SolType>::tokenize(&self.deprecationTime),
                )
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
        impl alloy_sol_types::private::IntoLogData for HubDeprecated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&HubDeprecated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &HubDeprecated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `RelayHubConfigured((uint256,uint256,uint256,uint256,uint256,address,uint8,uint80,uint16))` and selector `0x4812ada68f7c2cdc9f4a4a09e157ea6b924e0ef40a4fa7aa074fa8f70b1e7247`.
```solidity
event RelayHubConfigured(RelayHubConfig config);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct RelayHubConfigured {
        #[allow(missing_docs)]
        pub config: <RelayHubConfig as alloy::sol_types::SolType>::RustType,
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
        impl alloy_sol_types::SolEvent for RelayHubConfigured {
            type DataTuple<'a> = (RelayHubConfig,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "RelayHubConfigured((uint256,uint256,uint256,uint256,uint256,address,uint8,uint80,uint16))";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                72u8,
                18u8,
                173u8,
                166u8,
                143u8,
                124u8,
                44u8,
                220u8,
                159u8,
                74u8,
                74u8,
                9u8,
                225u8,
                87u8,
                234u8,
                107u8,
                146u8,
                78u8,
                14u8,
                244u8,
                10u8,
                79u8,
                167u8,
                170u8,
                7u8,
                79u8,
                168u8,
                247u8,
                11u8,
                30u8,
                114u8,
                71u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { config: data.0 }
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
                (<RelayHubConfig as alloy_sol_types::SolType>::tokenize(&self.config),)
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
        impl alloy_sol_types::private::IntoLogData for RelayHubConfigured {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&RelayHubConfigured> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &RelayHubConfigured) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `RelayWorkersAdded(address,address[],uint256)` and selector `0xebf4a9bffb39f7c5dbf3f65540183b9381ae226ac3d0a45b4cad484713bd4a28`.
```solidity
event RelayWorkersAdded(address indexed relayManager, address[] newRelayWorkers, uint256 workersCount);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct RelayWorkersAdded {
        #[allow(missing_docs)]
        pub relayManager: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newRelayWorkers: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        #[allow(missing_docs)]
        pub workersCount: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for RelayWorkersAdded {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "RelayWorkersAdded(address,address[],uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                235u8,
                244u8,
                169u8,
                191u8,
                251u8,
                57u8,
                247u8,
                197u8,
                219u8,
                243u8,
                246u8,
                85u8,
                64u8,
                24u8,
                59u8,
                147u8,
                129u8,
                174u8,
                34u8,
                106u8,
                195u8,
                208u8,
                164u8,
                91u8,
                76u8,
                173u8,
                72u8,
                71u8,
                19u8,
                189u8,
                74u8,
                40u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    relayManager: topics.1,
                    newRelayWorkers: data.0,
                    workersCount: data.1,
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
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.newRelayWorkers),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.workersCount),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.relayManager.clone())
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
                    &self.relayManager,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for RelayWorkersAdded {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&RelayWorkersAdded> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &RelayWorkersAdded) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `StakingTokenDataChanged(address,uint256)` and selector `0xfb7510fe755f485459ee8ed619d4a0addc092c230ff47e760a62aeba78ebb9e3`.
```solidity
event StakingTokenDataChanged(address token, uint256 minimumStake);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct StakingTokenDataChanged {
        #[allow(missing_docs)]
        pub token: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub minimumStake: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for StakingTokenDataChanged {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "StakingTokenDataChanged(address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                251u8,
                117u8,
                16u8,
                254u8,
                117u8,
                95u8,
                72u8,
                84u8,
                89u8,
                238u8,
                142u8,
                214u8,
                25u8,
                212u8,
                160u8,
                173u8,
                220u8,
                9u8,
                44u8,
                35u8,
                15u8,
                244u8,
                126u8,
                118u8,
                10u8,
                98u8,
                174u8,
                186u8,
                120u8,
                235u8,
                185u8,
                227u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    token: data.0,
                    minimumStake: data.1,
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
                        &self.token,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.minimumStake),
                )
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
        impl alloy_sol_types::private::IntoLogData for StakingTokenDataChanged {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&StakingTokenDataChanged> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &StakingTokenDataChanged,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `TransactionRejectedByPaymaster(address,address,bytes32,address,address,address,bytes4,uint256,bytes)` and selector `0x0c47510cb900901afca7c9e926892582a36fd3bb7f908142ccf0fb4382a2b684`.
```solidity
event TransactionRejectedByPaymaster(address indexed relayManager, address indexed paymaster, bytes32 indexed relayRequestID, address from, address to, address relayWorker, bytes4 selector, uint256 innerGasUsed, bytes reason);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct TransactionRejectedByPaymaster {
        #[allow(missing_docs)]
        pub relayManager: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub paymaster: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub relayRequestID: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub from: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub to: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub relayWorker: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub selector: alloy::sol_types::private::FixedBytes<4>,
        #[allow(missing_docs)]
        pub innerGasUsed: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub reason: alloy::sol_types::private::Bytes,
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
        impl alloy_sol_types::SolEvent for TransactionRejectedByPaymaster {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<4>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "TransactionRejectedByPaymaster(address,address,bytes32,address,address,address,bytes4,uint256,bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                12u8,
                71u8,
                81u8,
                12u8,
                185u8,
                0u8,
                144u8,
                26u8,
                252u8,
                167u8,
                201u8,
                233u8,
                38u8,
                137u8,
                37u8,
                130u8,
                163u8,
                111u8,
                211u8,
                187u8,
                127u8,
                144u8,
                129u8,
                66u8,
                204u8,
                240u8,
                251u8,
                67u8,
                130u8,
                162u8,
                182u8,
                132u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    relayManager: topics.1,
                    paymaster: topics.2,
                    relayRequestID: topics.3,
                    from: data.0,
                    to: data.1,
                    relayWorker: data.2,
                    selector: data.3,
                    innerGasUsed: data.4,
                    reason: data.5,
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
                        &self.from,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.to,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.relayWorker,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        4,
                    > as alloy_sol_types::SolType>::tokenize(&self.selector),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.innerGasUsed),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.reason,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.relayManager.clone(),
                    self.paymaster.clone(),
                    self.relayRequestID.clone(),
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
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.relayManager,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.paymaster,
                );
                out[3usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.relayRequestID);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for TransactionRejectedByPaymaster {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&TransactionRejectedByPaymaster>
        for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &TransactionRejectedByPaymaster,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `TransactionRelayed(address,address,bytes32,address,address,address,bytes4,uint8,uint256)` and selector `0xd51ac07012398c2059ec53c6005fa8639657917bd2dbd72bc489a3c0eaaa7f16`.
```solidity
event TransactionRelayed(address indexed relayManager, address indexed relayWorker, bytes32 indexed relayRequestID, address from, address to, address paymaster, bytes4 selector, RelayCallStatus status, uint256 charge);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct TransactionRelayed {
        #[allow(missing_docs)]
        pub relayManager: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub relayWorker: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub relayRequestID: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub from: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub to: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub paymaster: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub selector: alloy::sol_types::private::FixedBytes<4>,
        #[allow(missing_docs)]
        pub status: <RelayCallStatus as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub charge: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for TransactionRelayed {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<4>,
                RelayCallStatus,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "TransactionRelayed(address,address,bytes32,address,address,address,bytes4,uint8,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                213u8,
                26u8,
                192u8,
                112u8,
                18u8,
                57u8,
                140u8,
                32u8,
                89u8,
                236u8,
                83u8,
                198u8,
                0u8,
                95u8,
                168u8,
                99u8,
                150u8,
                87u8,
                145u8,
                123u8,
                210u8,
                219u8,
                215u8,
                43u8,
                196u8,
                137u8,
                163u8,
                192u8,
                234u8,
                170u8,
                127u8,
                22u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    relayManager: topics.1,
                    relayWorker: topics.2,
                    relayRequestID: topics.3,
                    from: data.0,
                    to: data.1,
                    paymaster: data.2,
                    selector: data.3,
                    status: data.4,
                    charge: data.5,
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
                        &self.from,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.to,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.paymaster,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        4,
                    > as alloy_sol_types::SolType>::tokenize(&self.selector),
                    <RelayCallStatus as alloy_sol_types::SolType>::tokenize(
                        &self.status,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.charge),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.relayManager.clone(),
                    self.relayWorker.clone(),
                    self.relayRequestID.clone(),
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
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.relayManager,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.relayWorker,
                );
                out[3usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.relayRequestID);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for TransactionRelayed {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&TransactionRelayed> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &TransactionRelayed) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `TransactionResult(uint8,bytes)` and selector `0xa1478a4242848419db824250a0dddc645dca0d6a9b12ab1fd79b00145a0ba98e`.
```solidity
event TransactionResult(RelayCallStatus status, bytes returnValue);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct TransactionResult {
        #[allow(missing_docs)]
        pub status: <RelayCallStatus as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub returnValue: alloy::sol_types::private::Bytes,
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
        impl alloy_sol_types::SolEvent for TransactionResult {
            type DataTuple<'a> = (RelayCallStatus, alloy::sol_types::sol_data::Bytes);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "TransactionResult(uint8,bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                161u8,
                71u8,
                138u8,
                66u8,
                66u8,
                132u8,
                132u8,
                25u8,
                219u8,
                130u8,
                66u8,
                80u8,
                160u8,
                221u8,
                220u8,
                100u8,
                93u8,
                202u8,
                13u8,
                106u8,
                155u8,
                18u8,
                171u8,
                31u8,
                215u8,
                155u8,
                0u8,
                20u8,
                90u8,
                11u8,
                169u8,
                142u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    status: data.0,
                    returnValue: data.1,
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
                    <RelayCallStatus as alloy_sol_types::SolType>::tokenize(
                        &self.status,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.returnValue,
                    ),
                )
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
        impl alloy_sol_types::private::IntoLogData for TransactionResult {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&TransactionResult> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &TransactionResult) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `Withdrawn(address,address,uint256)` and selector `0xd1c19fbcd4551a5edfb66d43d2e337c04837afda3482b42bdf569a8fccdae5fb`.
```solidity
event Withdrawn(address indexed account, address indexed dest, uint256 amount);
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
        pub dest: alloy::sol_types::private::Address,
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
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
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
                    dest: topics.2,
                    amount: data.0,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.account.clone(), self.dest.clone())
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
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.dest,
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
    /**Function with signature `addRelayWorkers(address[])` and selector `0xc2da0786`.
```solidity
function addRelayWorkers(address[] memory newRelayWorkers) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addRelayWorkersCall {
        pub newRelayWorkers: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
    }
    ///Container type for the return parameters of the [`addRelayWorkers(address[])`](addRelayWorkersCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addRelayWorkersReturn {}
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
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
            impl ::core::convert::From<addRelayWorkersCall> for UnderlyingRustTuple<'_> {
                fn from(value: addRelayWorkersCall) -> Self {
                    (value.newRelayWorkers,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for addRelayWorkersCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { newRelayWorkers: tuple.0 }
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
            impl ::core::convert::From<addRelayWorkersReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: addRelayWorkersReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for addRelayWorkersReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for addRelayWorkersCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = addRelayWorkersReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "addRelayWorkers(address[])";
            const SELECTOR: [u8; 4] = [194u8, 218u8, 7u8, 134u8];
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
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.newRelayWorkers),
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
    /**Function with signature `aggregateGasleft()` and selector `0x45bdade7`.
```solidity
function aggregateGasleft() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct aggregateGasleftCall {}
    ///Container type for the return parameters of the [`aggregateGasleft()`](aggregateGasleftCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct aggregateGasleftReturn {
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
            impl ::core::convert::From<aggregateGasleftCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: aggregateGasleftCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for aggregateGasleftCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
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
            impl ::core::convert::From<aggregateGasleftReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: aggregateGasleftReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for aggregateGasleftReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for aggregateGasleftCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = aggregateGasleftReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "aggregateGasleft()";
            const SELECTOR: [u8; 4] = [69u8, 189u8, 173u8, 231u8];
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
    /**Function with signature `balanceOf(address)` and selector `0x70a08231`.
```solidity
function balanceOf(address target) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct balanceOfCall {
        pub target: alloy::sol_types::private::Address,
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
                    (value.target,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for balanceOfCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { target: tuple.0 }
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
                        &self.target,
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
    /**Function with signature `calculateCharge(uint256,(uint256,uint256,uint256,address,address,address,bytes,uint256))` and selector `0x8e53548b`.
```solidity
function calculateCharge(uint256 gasUsed, GsnTypes.RelayData memory relayData) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct calculateChargeCall {
        pub gasUsed: alloy::sol_types::private::primitives::aliases::U256,
        pub relayData: <GsnTypes::RelayData as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`calculateCharge(uint256,(uint256,uint256,uint256,address,address,address,bytes,uint256))`](calculateChargeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct calculateChargeReturn {
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                GsnTypes::RelayData,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                <GsnTypes::RelayData as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<calculateChargeCall> for UnderlyingRustTuple<'_> {
                fn from(value: calculateChargeCall) -> Self {
                    (value.gasUsed, value.relayData)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for calculateChargeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        gasUsed: tuple.0,
                        relayData: tuple.1,
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
            impl ::core::convert::From<calculateChargeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: calculateChargeReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for calculateChargeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for calculateChargeCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                GsnTypes::RelayData,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = calculateChargeReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "calculateCharge(uint256,(uint256,uint256,uint256,address,address,address,bytes,uint256))";
            const SELECTOR: [u8; 4] = [142u8, 83u8, 84u8, 139u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.gasUsed),
                    <GsnTypes::RelayData as alloy_sol_types::SolType>::tokenize(
                        &self.relayData,
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
    /**Function with signature `calculateDevCharge(uint256)` and selector `0x8ac3b845`.
```solidity
function calculateDevCharge(uint256 charge) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct calculateDevChargeCall {
        pub charge: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`calculateDevCharge(uint256)`](calculateDevChargeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct calculateDevChargeReturn {
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
            impl ::core::convert::From<calculateDevChargeCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: calculateDevChargeCall) -> Self {
                    (value.charge,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for calculateDevChargeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { charge: tuple.0 }
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
            impl ::core::convert::From<calculateDevChargeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: calculateDevChargeReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for calculateDevChargeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for calculateDevChargeCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = calculateDevChargeReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "calculateDevCharge(uint256)";
            const SELECTOR: [u8; 4] = [138u8, 195u8, 184u8, 69u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.charge),
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
    /**Function with signature `depositFor(address)` and selector `0xaa67c919`.
```solidity
function depositFor(address target) external payable;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct depositForCall {
        pub target: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`depositFor(address)`](depositForCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct depositForReturn {}
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
            impl ::core::convert::From<depositForCall> for UnderlyingRustTuple<'_> {
                fn from(value: depositForCall) -> Self {
                    (value.target,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for depositForCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { target: tuple.0 }
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
            impl ::core::convert::From<depositForReturn> for UnderlyingRustTuple<'_> {
                fn from(value: depositForReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for depositForReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for depositForCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = depositForReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "depositFor(address)";
            const SELECTOR: [u8; 4] = [170u8, 103u8, 201u8, 25u8];
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
                        &self.target,
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
    /**Function with signature `deprecateHub(uint256)` and selector `0xaf595dfc`.
```solidity
function deprecateHub(uint256 _deprecationTime) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deprecateHubCall {
        pub _deprecationTime: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`deprecateHub(uint256)`](deprecateHubCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deprecateHubReturn {}
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
            impl ::core::convert::From<deprecateHubCall> for UnderlyingRustTuple<'_> {
                fn from(value: deprecateHubCall) -> Self {
                    (value._deprecationTime,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for deprecateHubCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _deprecationTime: tuple.0 }
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
            impl ::core::convert::From<deprecateHubReturn> for UnderlyingRustTuple<'_> {
                fn from(value: deprecateHubReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for deprecateHubReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for deprecateHubCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = deprecateHubReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "deprecateHub(uint256)";
            const SELECTOR: [u8; 4] = [175u8, 89u8, 93u8, 252u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self._deprecationTime),
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
    /**Function with signature `escheatAbandonedRelayBalance(address)` and selector `0xd26152f9`.
```solidity
function escheatAbandonedRelayBalance(address relayManager) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct escheatAbandonedRelayBalanceCall {
        pub relayManager: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`escheatAbandonedRelayBalance(address)`](escheatAbandonedRelayBalanceCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct escheatAbandonedRelayBalanceReturn {}
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
            impl ::core::convert::From<escheatAbandonedRelayBalanceCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: escheatAbandonedRelayBalanceCall) -> Self {
                    (value.relayManager,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for escheatAbandonedRelayBalanceCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { relayManager: tuple.0 }
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
            impl ::core::convert::From<escheatAbandonedRelayBalanceReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: escheatAbandonedRelayBalanceReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for escheatAbandonedRelayBalanceReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for escheatAbandonedRelayBalanceCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = escheatAbandonedRelayBalanceReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "escheatAbandonedRelayBalance(address)";
            const SELECTOR: [u8; 4] = [210u8, 97u8, 82u8, 249u8];
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
                        &self.relayManager,
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
    /**Function with signature `getBatchGateway()` and selector `0xf7908a74`.
```solidity
function getBatchGateway() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getBatchGatewayCall {}
    ///Container type for the return parameters of the [`getBatchGateway()`](getBatchGatewayCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getBatchGatewayReturn {
        pub _0: alloy::sol_types::private::Address,
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
            impl ::core::convert::From<getBatchGatewayCall> for UnderlyingRustTuple<'_> {
                fn from(value: getBatchGatewayCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getBatchGatewayCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
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
            impl ::core::convert::From<getBatchGatewayReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getBatchGatewayReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getBatchGatewayReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getBatchGatewayCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getBatchGatewayReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getBatchGateway()";
            const SELECTOR: [u8; 4] = [247u8, 144u8, 138u8, 116u8];
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
    /**Function with signature `getConfiguration()` and selector `0x6bd50cef`.
```solidity
function getConfiguration() external view returns (RelayHubConfig memory config);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getConfigurationCall {}
    ///Container type for the return parameters of the [`getConfiguration()`](getConfigurationCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getConfigurationReturn {
        pub config: <RelayHubConfig as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getConfigurationCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getConfigurationCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getConfigurationCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (RelayHubConfig,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <RelayHubConfig as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getConfigurationReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getConfigurationReturn) -> Self {
                    (value.config,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getConfigurationReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { config: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getConfigurationCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getConfigurationReturn;
            type ReturnTuple<'a> = (RelayHubConfig,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getConfiguration()";
            const SELECTOR: [u8; 4] = [107u8, 213u8, 12u8, 239u8];
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
    /**Function with signature `getCreationBlock()` and selector `0x20ffd56d`.
```solidity
function getCreationBlock() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCreationBlockCall {}
    ///Container type for the return parameters of the [`getCreationBlock()`](getCreationBlockCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCreationBlockReturn {
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
            impl ::core::convert::From<getCreationBlockCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCreationBlockCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCreationBlockCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
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
            impl ::core::convert::From<getCreationBlockReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCreationBlockReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCreationBlockReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getCreationBlockCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getCreationBlockReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getCreationBlock()";
            const SELECTOR: [u8; 4] = [32u8, 255u8, 213u8, 109u8];
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
    /**Function with signature `getDeprecationTime()` and selector `0x3c16e59a`.
```solidity
function getDeprecationTime() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getDeprecationTimeCall {}
    ///Container type for the return parameters of the [`getDeprecationTime()`](getDeprecationTimeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getDeprecationTimeReturn {
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
            impl ::core::convert::From<getDeprecationTimeCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getDeprecationTimeCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getDeprecationTimeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
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
            impl ::core::convert::From<getDeprecationTimeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getDeprecationTimeReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getDeprecationTimeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getDeprecationTimeCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getDeprecationTimeReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getDeprecationTime()";
            const SELECTOR: [u8; 4] = [60u8, 22u8, 229u8, 154u8];
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
    /**Function with signature `getMinimumStakePerToken(address)` and selector `0xddf73789`.
```solidity
function getMinimumStakePerToken(address token) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getMinimumStakePerTokenCall {
        pub token: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getMinimumStakePerToken(address)`](getMinimumStakePerTokenCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getMinimumStakePerTokenReturn {
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
            impl ::core::convert::From<getMinimumStakePerTokenCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getMinimumStakePerTokenCall) -> Self {
                    (value.token,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getMinimumStakePerTokenCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { token: tuple.0 }
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
            impl ::core::convert::From<getMinimumStakePerTokenReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getMinimumStakePerTokenReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getMinimumStakePerTokenReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getMinimumStakePerTokenCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getMinimumStakePerTokenReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getMinimumStakePerToken(address)";
            const SELECTOR: [u8; 4] = [221u8, 247u8, 55u8, 137u8];
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
                        &self.token,
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
    /**Function with signature `getPenalizer()` and selector `0x75dc7b4e`.
```solidity
function getPenalizer() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getPenalizerCall {}
    ///Container type for the return parameters of the [`getPenalizer()`](getPenalizerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getPenalizerReturn {
        pub _0: alloy::sol_types::private::Address,
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
            impl ::core::convert::From<getPenalizerCall> for UnderlyingRustTuple<'_> {
                fn from(value: getPenalizerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getPenalizerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
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
            impl ::core::convert::From<getPenalizerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getPenalizerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getPenalizerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getPenalizerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getPenalizerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getPenalizer()";
            const SELECTOR: [u8; 4] = [117u8, 220u8, 123u8, 78u8];
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
    /**Function with signature `getRelayRegistrar()` and selector `0x51d85156`.
```solidity
function getRelayRegistrar() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRelayRegistrarCall {}
    ///Container type for the return parameters of the [`getRelayRegistrar()`](getRelayRegistrarCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRelayRegistrarReturn {
        pub _0: alloy::sol_types::private::Address,
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
            impl ::core::convert::From<getRelayRegistrarCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getRelayRegistrarCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getRelayRegistrarCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
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
            impl ::core::convert::From<getRelayRegistrarReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getRelayRegistrarReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getRelayRegistrarReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getRelayRegistrarCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getRelayRegistrarReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getRelayRegistrar()";
            const SELECTOR: [u8; 4] = [81u8, 216u8, 81u8, 86u8];
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
    /**Function with signature `getStakeManager()` and selector `0xca64f9e7`.
```solidity
function getStakeManager() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getStakeManagerCall {}
    ///Container type for the return parameters of the [`getStakeManager()`](getStakeManagerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getStakeManagerReturn {
        pub _0: alloy::sol_types::private::Address,
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
            impl ::core::convert::From<getStakeManagerCall> for UnderlyingRustTuple<'_> {
                fn from(value: getStakeManagerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getStakeManagerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
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
            impl ::core::convert::From<getStakeManagerReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getStakeManagerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getStakeManagerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getStakeManagerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getStakeManagerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getStakeManager()";
            const SELECTOR: [u8; 4] = [202u8, 100u8, 249u8, 231u8];
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
    /**Function with signature `getWorkerCount(address)` and selector `0x59c4afc0`.
```solidity
function getWorkerCount(address manager) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getWorkerCountCall {
        pub manager: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getWorkerCount(address)`](getWorkerCountCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getWorkerCountReturn {
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
            impl ::core::convert::From<getWorkerCountCall> for UnderlyingRustTuple<'_> {
                fn from(value: getWorkerCountCall) -> Self {
                    (value.manager,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getWorkerCountCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { manager: tuple.0 }
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
            impl ::core::convert::From<getWorkerCountReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getWorkerCountReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getWorkerCountReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getWorkerCountCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getWorkerCountReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getWorkerCount(address)";
            const SELECTOR: [u8; 4] = [89u8, 196u8, 175u8, 192u8];
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
                        &self.manager,
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
    /**Function with signature `getWorkerManager(address)` and selector `0xbe7c76ad`.
```solidity
function getWorkerManager(address worker) external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getWorkerManagerCall {
        pub worker: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getWorkerManager(address)`](getWorkerManagerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getWorkerManagerReturn {
        pub _0: alloy::sol_types::private::Address,
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
            impl ::core::convert::From<getWorkerManagerCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getWorkerManagerCall) -> Self {
                    (value.worker,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getWorkerManagerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { worker: tuple.0 }
                }
            }
        }
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
            impl ::core::convert::From<getWorkerManagerReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getWorkerManagerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getWorkerManagerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getWorkerManagerCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getWorkerManagerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getWorkerManager(address)";
            const SELECTOR: [u8; 4] = [190u8, 124u8, 118u8, 173u8];
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
                        &self.worker,
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
    /**Function with signature `isDeprecated()` and selector `0xc7178230`.
```solidity
function isDeprecated() external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isDeprecatedCall {}
    ///Container type for the return parameters of the [`isDeprecated()`](isDeprecatedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isDeprecatedReturn {
        pub _0: bool,
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
            impl ::core::convert::From<isDeprecatedCall> for UnderlyingRustTuple<'_> {
                fn from(value: isDeprecatedCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isDeprecatedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
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
            impl ::core::convert::From<isDeprecatedReturn> for UnderlyingRustTuple<'_> {
                fn from(value: isDeprecatedReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isDeprecatedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isDeprecatedCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = isDeprecatedReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isDeprecated()";
            const SELECTOR: [u8; 4] = [199u8, 23u8, 130u8, 48u8];
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
    /**Function with signature `isRelayEscheatable(address)` and selector `0x39622167`.
```solidity
function isRelayEscheatable(address relayManager) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isRelayEscheatableCall {
        pub relayManager: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`isRelayEscheatable(address)`](isRelayEscheatableCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isRelayEscheatableReturn {
        pub _0: bool,
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
            impl ::core::convert::From<isRelayEscheatableCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: isRelayEscheatableCall) -> Self {
                    (value.relayManager,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isRelayEscheatableCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { relayManager: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
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
            impl ::core::convert::From<isRelayEscheatableReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: isRelayEscheatableReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isRelayEscheatableReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isRelayEscheatableCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = isRelayEscheatableReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isRelayEscheatable(address)";
            const SELECTOR: [u8; 4] = [57u8, 98u8, 33u8, 103u8];
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
                        &self.relayManager,
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
    /**Function with signature `onRelayServerRegistered(address)` and selector `0x4f7de031`.
```solidity
function onRelayServerRegistered(address relayManager) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct onRelayServerRegisteredCall {
        pub relayManager: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`onRelayServerRegistered(address)`](onRelayServerRegisteredCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct onRelayServerRegisteredReturn {}
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
            impl ::core::convert::From<onRelayServerRegisteredCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: onRelayServerRegisteredCall) -> Self {
                    (value.relayManager,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for onRelayServerRegisteredCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { relayManager: tuple.0 }
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
            impl ::core::convert::From<onRelayServerRegisteredReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: onRelayServerRegisteredReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for onRelayServerRegisteredReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for onRelayServerRegisteredCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = onRelayServerRegisteredReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "onRelayServerRegistered(address)";
            const SELECTOR: [u8; 4] = [79u8, 125u8, 224u8, 49u8];
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
                        &self.relayManager,
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
    /**Function with signature `penalize(address,address)` and selector `0xebcd31ac`.
```solidity
function penalize(address relayWorker, address beneficiary) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct penalizeCall {
        pub relayWorker: alloy::sol_types::private::Address,
        pub beneficiary: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`penalize(address,address)`](penalizeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct penalizeReturn {}
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
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<penalizeCall> for UnderlyingRustTuple<'_> {
                fn from(value: penalizeCall) -> Self {
                    (value.relayWorker, value.beneficiary)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for penalizeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        relayWorker: tuple.0,
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
            impl ::core::convert::From<penalizeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: penalizeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for penalizeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for penalizeCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = penalizeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "penalize(address,address)";
            const SELECTOR: [u8; 4] = [235u8, 205u8, 49u8, 172u8];
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
                        &self.relayWorker,
                    ),
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
    /**Function with signature `relayCall(string,uint256,((address,address,uint256,uint256,uint256,bytes,uint256),(uint256,uint256,uint256,address,address,address,bytes,uint256)),bytes,bytes)` and selector `0x6ca862e2`.
```solidity
function relayCall(string memory domainSeparatorName, uint256 maxAcceptanceBudget, GsnTypes.RelayRequest memory relayRequest, bytes memory signature, bytes memory approvalData) external returns (bool paymasterAccepted, uint256 charge, RelayCallStatus status, bytes memory returnValue);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct relayCallCall {
        pub domainSeparatorName: alloy::sol_types::private::String,
        pub maxAcceptanceBudget: alloy::sol_types::private::primitives::aliases::U256,
        pub relayRequest: <GsnTypes::RelayRequest as alloy::sol_types::SolType>::RustType,
        pub signature: alloy::sol_types::private::Bytes,
        pub approvalData: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`relayCall(string,uint256,((address,address,uint256,uint256,uint256,bytes,uint256),(uint256,uint256,uint256,address,address,address,bytes,uint256)),bytes,bytes)`](relayCallCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct relayCallReturn {
        pub paymasterAccepted: bool,
        pub charge: alloy::sol_types::private::primitives::aliases::U256,
        pub status: <RelayCallStatus as alloy::sol_types::SolType>::RustType,
        pub returnValue: alloy::sol_types::private::Bytes,
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
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Uint<256>,
                GsnTypes::RelayRequest,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::String,
                alloy::sol_types::private::primitives::aliases::U256,
                <GsnTypes::RelayRequest as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<relayCallCall> for UnderlyingRustTuple<'_> {
                fn from(value: relayCallCall) -> Self {
                    (
                        value.domainSeparatorName,
                        value.maxAcceptanceBudget,
                        value.relayRequest,
                        value.signature,
                        value.approvalData,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for relayCallCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        domainSeparatorName: tuple.0,
                        maxAcceptanceBudget: tuple.1,
                        relayRequest: tuple.2,
                        signature: tuple.3,
                        approvalData: tuple.4,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Uint<256>,
                RelayCallStatus,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                bool,
                alloy::sol_types::private::primitives::aliases::U256,
                <RelayCallStatus as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<relayCallReturn> for UnderlyingRustTuple<'_> {
                fn from(value: relayCallReturn) -> Self {
                    (
                        value.paymasterAccepted,
                        value.charge,
                        value.status,
                        value.returnValue,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for relayCallReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        paymasterAccepted: tuple.0,
                        charge: tuple.1,
                        status: tuple.2,
                        returnValue: tuple.3,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for relayCallCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Uint<256>,
                GsnTypes::RelayRequest,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = relayCallReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Uint<256>,
                RelayCallStatus,
                alloy::sol_types::sol_data::Bytes,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "relayCall(string,uint256,((address,address,uint256,uint256,uint256,bytes,uint256),(uint256,uint256,uint256,address,address,address,bytes,uint256)),bytes,bytes)";
            const SELECTOR: [u8; 4] = [108u8, 168u8, 98u8, 226u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.domainSeparatorName,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.maxAcceptanceBudget),
                    <GsnTypes::RelayRequest as alloy_sol_types::SolType>::tokenize(
                        &self.relayRequest,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.signature,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.approvalData,
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
    /**Function with signature `setConfiguration((uint256,uint256,uint256,uint256,uint256,address,uint8,uint80,uint16))` and selector `0x9bcde3f3`.
```solidity
function setConfiguration(RelayHubConfig memory _config) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setConfigurationCall {
        pub _config: <RelayHubConfig as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`setConfiguration((uint256,uint256,uint256,uint256,uint256,address,uint8,uint80,uint16))`](setConfigurationCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setConfigurationReturn {}
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
            type UnderlyingSolTuple<'a> = (RelayHubConfig,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <RelayHubConfig as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<setConfigurationCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: setConfigurationCall) -> Self {
                    (value._config,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setConfigurationCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _config: tuple.0 }
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
            impl ::core::convert::From<setConfigurationReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setConfigurationReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setConfigurationReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setConfigurationCall {
            type Parameters<'a> = (RelayHubConfig,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setConfigurationReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setConfiguration((uint256,uint256,uint256,uint256,uint256,address,uint8,uint80,uint16))";
            const SELECTOR: [u8; 4] = [155u8, 205u8, 227u8, 243u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<RelayHubConfig as alloy_sol_types::SolType>::tokenize(&self._config),)
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
    /**Function with signature `setMinimumStakes(address[],uint256[])` and selector `0x8a09fb56`.
```solidity
function setMinimumStakes(address[] memory token, uint256[] memory minimumStake) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setMinimumStakesCall {
        pub token: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        pub minimumStake: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
    }
    ///Container type for the return parameters of the [`setMinimumStakes(address[],uint256[])`](setMinimumStakesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setMinimumStakesReturn {}
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
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U256,
                >,
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
            impl ::core::convert::From<setMinimumStakesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: setMinimumStakesCall) -> Self {
                    (value.token, value.minimumStake)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setMinimumStakesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        token: tuple.0,
                        minimumStake: tuple.1,
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
            impl ::core::convert::From<setMinimumStakesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setMinimumStakesReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setMinimumStakesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setMinimumStakesCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setMinimumStakesReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setMinimumStakes(address[],uint256[])";
            const SELECTOR: [u8; 4] = [138u8, 9u8, 251u8, 86u8];
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
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.token),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.minimumStake),
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
    /**Function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`.
```solidity
function supportsInterface(bytes4 interfaceId) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct supportsInterfaceCall {
        pub interfaceId: alloy::sol_types::private::FixedBytes<4>,
    }
    ///Container type for the return parameters of the [`supportsInterface(bytes4)`](supportsInterfaceCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct supportsInterfaceReturn {
        pub _0: bool,
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<4>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<4>,);
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
            impl ::core::convert::From<supportsInterfaceCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: supportsInterfaceCall) -> Self {
                    (value.interfaceId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for supportsInterfaceCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { interfaceId: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
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
            impl ::core::convert::From<supportsInterfaceReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: supportsInterfaceReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for supportsInterfaceReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for supportsInterfaceCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<4>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = supportsInterfaceReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "supportsInterface(bytes4)";
            const SELECTOR: [u8; 4] = [1u8, 255u8, 201u8, 167u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        4,
                    > as alloy_sol_types::SolType>::tokenize(&self.interfaceId),
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
    /**Function with signature `verifyRelayManagerStaked(address)` and selector `0xe6e22e12`.
```solidity
function verifyRelayManagerStaked(address relayManager) external view;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct verifyRelayManagerStakedCall {
        pub relayManager: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`verifyRelayManagerStaked(address)`](verifyRelayManagerStakedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct verifyRelayManagerStakedReturn {}
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
            impl ::core::convert::From<verifyRelayManagerStakedCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: verifyRelayManagerStakedCall) -> Self {
                    (value.relayManager,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for verifyRelayManagerStakedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { relayManager: tuple.0 }
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
            impl ::core::convert::From<verifyRelayManagerStakedReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: verifyRelayManagerStakedReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for verifyRelayManagerStakedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for verifyRelayManagerStakedCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = verifyRelayManagerStakedReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "verifyRelayManagerStaked(address)";
            const SELECTOR: [u8; 4] = [230u8, 226u8, 46u8, 18u8];
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
                        &self.relayManager,
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
    /**Function with signature `versionHub()` and selector `0xd904c732`.
```solidity
function versionHub() external view returns (string memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct versionHubCall {}
    ///Container type for the return parameters of the [`versionHub()`](versionHubCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct versionHubReturn {
        pub _0: alloy::sol_types::private::String,
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
            impl ::core::convert::From<versionHubCall> for UnderlyingRustTuple<'_> {
                fn from(value: versionHubCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for versionHubCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::String,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::String,);
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
            impl ::core::convert::From<versionHubReturn> for UnderlyingRustTuple<'_> {
                fn from(value: versionHubReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for versionHubReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for versionHubCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = versionHubReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::String,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "versionHub()";
            const SELECTOR: [u8; 4] = [217u8, 4u8, 199u8, 50u8];
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
    /**Function with signature `withdraw(address,uint256)` and selector `0xf3fef3a3`.
```solidity
function withdraw(address dest, uint256 amount) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct withdrawCall {
        pub dest: alloy::sol_types::private::Address,
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`withdraw(address,uint256)`](withdrawCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct withdrawReturn {}
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
            impl ::core::convert::From<withdrawCall> for UnderlyingRustTuple<'_> {
                fn from(value: withdrawCall) -> Self {
                    (value.dest, value.amount)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for withdrawCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        dest: tuple.0,
                        amount: tuple.1,
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
            impl ::core::convert::From<withdrawReturn> for UnderlyingRustTuple<'_> {
                fn from(value: withdrawReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for withdrawReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for withdrawCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = withdrawReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "withdraw(address,uint256)";
            const SELECTOR: [u8; 4] = [243u8, 254u8, 243u8, 163u8];
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
                        &self.dest,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
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
    /**Function with signature `withdrawMultiple(address[],uint256[])` and selector `0x72c1ab0c`.
```solidity
function withdrawMultiple(address[] memory dest, uint256[] memory amount) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct withdrawMultipleCall {
        pub dest: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        pub amount: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
    }
    ///Container type for the return parameters of the [`withdrawMultiple(address[],uint256[])`](withdrawMultipleCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct withdrawMultipleReturn {}
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
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U256,
                >,
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
            impl ::core::convert::From<withdrawMultipleCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: withdrawMultipleCall) -> Self {
                    (value.dest, value.amount)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for withdrawMultipleCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        dest: tuple.0,
                        amount: tuple.1,
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
            impl ::core::convert::From<withdrawMultipleReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: withdrawMultipleReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for withdrawMultipleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for withdrawMultipleCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = withdrawMultipleReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "withdrawMultiple(address[],uint256[])";
            const SELECTOR: [u8; 4] = [114u8, 193u8, 171u8, 12u8];
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
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.dest),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
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
    ///Container for all the [`IRelayHub`](self) function calls.
    pub enum IRelayHubCalls {
        addRelayWorkers(addRelayWorkersCall),
        aggregateGasleft(aggregateGasleftCall),
        balanceOf(balanceOfCall),
        calculateCharge(calculateChargeCall),
        calculateDevCharge(calculateDevChargeCall),
        depositFor(depositForCall),
        deprecateHub(deprecateHubCall),
        escheatAbandonedRelayBalance(escheatAbandonedRelayBalanceCall),
        getBatchGateway(getBatchGatewayCall),
        getConfiguration(getConfigurationCall),
        getCreationBlock(getCreationBlockCall),
        getDeprecationTime(getDeprecationTimeCall),
        getMinimumStakePerToken(getMinimumStakePerTokenCall),
        getPenalizer(getPenalizerCall),
        getRelayRegistrar(getRelayRegistrarCall),
        getStakeManager(getStakeManagerCall),
        getWorkerCount(getWorkerCountCall),
        getWorkerManager(getWorkerManagerCall),
        isDeprecated(isDeprecatedCall),
        isRelayEscheatable(isRelayEscheatableCall),
        onRelayServerRegistered(onRelayServerRegisteredCall),
        penalize(penalizeCall),
        relayCall(relayCallCall),
        setConfiguration(setConfigurationCall),
        setMinimumStakes(setMinimumStakesCall),
        supportsInterface(supportsInterfaceCall),
        verifyRelayManagerStaked(verifyRelayManagerStakedCall),
        versionHub(versionHubCall),
        withdraw(withdrawCall),
        withdrawMultiple(withdrawMultipleCall),
    }
    #[automatically_derived]
    impl IRelayHubCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [1u8, 255u8, 201u8, 167u8],
            [32u8, 255u8, 213u8, 109u8],
            [57u8, 98u8, 33u8, 103u8],
            [60u8, 22u8, 229u8, 154u8],
            [69u8, 189u8, 173u8, 231u8],
            [79u8, 125u8, 224u8, 49u8],
            [81u8, 216u8, 81u8, 86u8],
            [89u8, 196u8, 175u8, 192u8],
            [107u8, 213u8, 12u8, 239u8],
            [108u8, 168u8, 98u8, 226u8],
            [112u8, 160u8, 130u8, 49u8],
            [114u8, 193u8, 171u8, 12u8],
            [117u8, 220u8, 123u8, 78u8],
            [138u8, 9u8, 251u8, 86u8],
            [138u8, 195u8, 184u8, 69u8],
            [142u8, 83u8, 84u8, 139u8],
            [155u8, 205u8, 227u8, 243u8],
            [170u8, 103u8, 201u8, 25u8],
            [175u8, 89u8, 93u8, 252u8],
            [190u8, 124u8, 118u8, 173u8],
            [194u8, 218u8, 7u8, 134u8],
            [199u8, 23u8, 130u8, 48u8],
            [202u8, 100u8, 249u8, 231u8],
            [210u8, 97u8, 82u8, 249u8],
            [217u8, 4u8, 199u8, 50u8],
            [221u8, 247u8, 55u8, 137u8],
            [230u8, 226u8, 46u8, 18u8],
            [235u8, 205u8, 49u8, 172u8],
            [243u8, 254u8, 243u8, 163u8],
            [247u8, 144u8, 138u8, 116u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for IRelayHubCalls {
        const NAME: &'static str = "IRelayHubCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 30usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::addRelayWorkers(_) => {
                    <addRelayWorkersCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::aggregateGasleft(_) => {
                    <aggregateGasleftCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::balanceOf(_) => {
                    <balanceOfCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::calculateCharge(_) => {
                    <calculateChargeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::calculateDevCharge(_) => {
                    <calculateDevChargeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::depositFor(_) => {
                    <depositForCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::deprecateHub(_) => {
                    <deprecateHubCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::escheatAbandonedRelayBalance(_) => {
                    <escheatAbandonedRelayBalanceCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getBatchGateway(_) => {
                    <getBatchGatewayCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getConfiguration(_) => {
                    <getConfigurationCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getCreationBlock(_) => {
                    <getCreationBlockCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getDeprecationTime(_) => {
                    <getDeprecationTimeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getMinimumStakePerToken(_) => {
                    <getMinimumStakePerTokenCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getPenalizer(_) => {
                    <getPenalizerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getRelayRegistrar(_) => {
                    <getRelayRegistrarCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getStakeManager(_) => {
                    <getStakeManagerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getWorkerCount(_) => {
                    <getWorkerCountCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getWorkerManager(_) => {
                    <getWorkerManagerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isDeprecated(_) => {
                    <isDeprecatedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isRelayEscheatable(_) => {
                    <isRelayEscheatableCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::onRelayServerRegistered(_) => {
                    <onRelayServerRegisteredCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::penalize(_) => <penalizeCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::relayCall(_) => {
                    <relayCallCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setConfiguration(_) => {
                    <setConfigurationCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setMinimumStakes(_) => {
                    <setMinimumStakesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::supportsInterface(_) => {
                    <supportsInterfaceCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::verifyRelayManagerStaked(_) => {
                    <verifyRelayManagerStakedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::versionHub(_) => {
                    <versionHubCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::withdraw(_) => <withdrawCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::withdrawMultiple(_) => {
                    <withdrawMultipleCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<IRelayHubCalls>] = &[
                {
                    fn supportsInterface(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IRelayHubCalls> {
                        <supportsInterfaceCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IRelayHubCalls::supportsInterface)
                    }
                    supportsInterface
                },
                {
                    fn getCreationBlock(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IRelayHubCalls> {
                        <getCreationBlockCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IRelayHubCalls::getCreationBlock)
                    }
                    getCreationBlock
                },
                {
                    fn isRelayEscheatable(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IRelayHubCalls> {
                        <isRelayEscheatableCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IRelayHubCalls::isRelayEscheatable)
                    }
                    isRelayEscheatable
                },
                {
                    fn getDeprecationTime(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IRelayHubCalls> {
                        <getDeprecationTimeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IRelayHubCalls::getDeprecationTime)
                    }
                    getDeprecationTime
                },
                {
                    fn aggregateGasleft(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IRelayHubCalls> {
                        <aggregateGasleftCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IRelayHubCalls::aggregateGasleft)
                    }
                    aggregateGasleft
                },
                {
                    fn onRelayServerRegistered(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IRelayHubCalls> {
                        <onRelayServerRegisteredCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IRelayHubCalls::onRelayServerRegistered)
                    }
                    onRelayServerRegistered
                },
                {
                    fn getRelayRegistrar(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IRelayHubCalls> {
                        <getRelayRegistrarCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IRelayHubCalls::getRelayRegistrar)
                    }
                    getRelayRegistrar
                },
                {
                    fn getWorkerCount(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IRelayHubCalls> {
                        <getWorkerCountCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IRelayHubCalls::getWorkerCount)
                    }
                    getWorkerCount
                },
                {
                    fn getConfiguration(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IRelayHubCalls> {
                        <getConfigurationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IRelayHubCalls::getConfiguration)
                    }
                    getConfiguration
                },
                {
                    fn relayCall(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IRelayHubCalls> {
                        <relayCallCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IRelayHubCalls::relayCall)
                    }
                    relayCall
                },
                {
                    fn balanceOf(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IRelayHubCalls> {
                        <balanceOfCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IRelayHubCalls::balanceOf)
                    }
                    balanceOf
                },
                {
                    fn withdrawMultiple(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IRelayHubCalls> {
                        <withdrawMultipleCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IRelayHubCalls::withdrawMultiple)
                    }
                    withdrawMultiple
                },
                {
                    fn getPenalizer(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IRelayHubCalls> {
                        <getPenalizerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IRelayHubCalls::getPenalizer)
                    }
                    getPenalizer
                },
                {
                    fn setMinimumStakes(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IRelayHubCalls> {
                        <setMinimumStakesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IRelayHubCalls::setMinimumStakes)
                    }
                    setMinimumStakes
                },
                {
                    fn calculateDevCharge(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IRelayHubCalls> {
                        <calculateDevChargeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IRelayHubCalls::calculateDevCharge)
                    }
                    calculateDevCharge
                },
                {
                    fn calculateCharge(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IRelayHubCalls> {
                        <calculateChargeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IRelayHubCalls::calculateCharge)
                    }
                    calculateCharge
                },
                {
                    fn setConfiguration(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IRelayHubCalls> {
                        <setConfigurationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IRelayHubCalls::setConfiguration)
                    }
                    setConfiguration
                },
                {
                    fn depositFor(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IRelayHubCalls> {
                        <depositForCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IRelayHubCalls::depositFor)
                    }
                    depositFor
                },
                {
                    fn deprecateHub(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IRelayHubCalls> {
                        <deprecateHubCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IRelayHubCalls::deprecateHub)
                    }
                    deprecateHub
                },
                {
                    fn getWorkerManager(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IRelayHubCalls> {
                        <getWorkerManagerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IRelayHubCalls::getWorkerManager)
                    }
                    getWorkerManager
                },
                {
                    fn addRelayWorkers(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IRelayHubCalls> {
                        <addRelayWorkersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IRelayHubCalls::addRelayWorkers)
                    }
                    addRelayWorkers
                },
                {
                    fn isDeprecated(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IRelayHubCalls> {
                        <isDeprecatedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IRelayHubCalls::isDeprecated)
                    }
                    isDeprecated
                },
                {
                    fn getStakeManager(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IRelayHubCalls> {
                        <getStakeManagerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IRelayHubCalls::getStakeManager)
                    }
                    getStakeManager
                },
                {
                    fn escheatAbandonedRelayBalance(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IRelayHubCalls> {
                        <escheatAbandonedRelayBalanceCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IRelayHubCalls::escheatAbandonedRelayBalance)
                    }
                    escheatAbandonedRelayBalance
                },
                {
                    fn versionHub(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IRelayHubCalls> {
                        <versionHubCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IRelayHubCalls::versionHub)
                    }
                    versionHub
                },
                {
                    fn getMinimumStakePerToken(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IRelayHubCalls> {
                        <getMinimumStakePerTokenCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IRelayHubCalls::getMinimumStakePerToken)
                    }
                    getMinimumStakePerToken
                },
                {
                    fn verifyRelayManagerStaked(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IRelayHubCalls> {
                        <verifyRelayManagerStakedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IRelayHubCalls::verifyRelayManagerStaked)
                    }
                    verifyRelayManagerStaked
                },
                {
                    fn penalize(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IRelayHubCalls> {
                        <penalizeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IRelayHubCalls::penalize)
                    }
                    penalize
                },
                {
                    fn withdraw(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IRelayHubCalls> {
                        <withdrawCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IRelayHubCalls::withdraw)
                    }
                    withdraw
                },
                {
                    fn getBatchGateway(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IRelayHubCalls> {
                        <getBatchGatewayCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IRelayHubCalls::getBatchGateway)
                    }
                    getBatchGateway
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
                Self::addRelayWorkers(inner) => {
                    <addRelayWorkersCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::aggregateGasleft(inner) => {
                    <aggregateGasleftCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::balanceOf(inner) => {
                    <balanceOfCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::calculateCharge(inner) => {
                    <calculateChargeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::calculateDevCharge(inner) => {
                    <calculateDevChargeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::depositFor(inner) => {
                    <depositForCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::deprecateHub(inner) => {
                    <deprecateHubCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::escheatAbandonedRelayBalance(inner) => {
                    <escheatAbandonedRelayBalanceCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getBatchGateway(inner) => {
                    <getBatchGatewayCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getConfiguration(inner) => {
                    <getConfigurationCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getCreationBlock(inner) => {
                    <getCreationBlockCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getDeprecationTime(inner) => {
                    <getDeprecationTimeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getMinimumStakePerToken(inner) => {
                    <getMinimumStakePerTokenCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getPenalizer(inner) => {
                    <getPenalizerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getRelayRegistrar(inner) => {
                    <getRelayRegistrarCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getStakeManager(inner) => {
                    <getStakeManagerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getWorkerCount(inner) => {
                    <getWorkerCountCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getWorkerManager(inner) => {
                    <getWorkerManagerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::isDeprecated(inner) => {
                    <isDeprecatedCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::isRelayEscheatable(inner) => {
                    <isRelayEscheatableCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::onRelayServerRegistered(inner) => {
                    <onRelayServerRegisteredCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::penalize(inner) => {
                    <penalizeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::relayCall(inner) => {
                    <relayCallCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::setConfiguration(inner) => {
                    <setConfigurationCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setMinimumStakes(inner) => {
                    <setMinimumStakesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::supportsInterface(inner) => {
                    <supportsInterfaceCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::verifyRelayManagerStaked(inner) => {
                    <verifyRelayManagerStakedCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::versionHub(inner) => {
                    <versionHubCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::withdraw(inner) => {
                    <withdrawCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::withdrawMultiple(inner) => {
                    <withdrawMultipleCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::addRelayWorkers(inner) => {
                    <addRelayWorkersCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::aggregateGasleft(inner) => {
                    <aggregateGasleftCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::calculateCharge(inner) => {
                    <calculateChargeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::calculateDevCharge(inner) => {
                    <calculateDevChargeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::depositFor(inner) => {
                    <depositForCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::deprecateHub(inner) => {
                    <deprecateHubCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::escheatAbandonedRelayBalance(inner) => {
                    <escheatAbandonedRelayBalanceCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getBatchGateway(inner) => {
                    <getBatchGatewayCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getConfiguration(inner) => {
                    <getConfigurationCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getCreationBlock(inner) => {
                    <getCreationBlockCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getDeprecationTime(inner) => {
                    <getDeprecationTimeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getMinimumStakePerToken(inner) => {
                    <getMinimumStakePerTokenCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getPenalizer(inner) => {
                    <getPenalizerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getRelayRegistrar(inner) => {
                    <getRelayRegistrarCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getStakeManager(inner) => {
                    <getStakeManagerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getWorkerCount(inner) => {
                    <getWorkerCountCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getWorkerManager(inner) => {
                    <getWorkerManagerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isDeprecated(inner) => {
                    <isDeprecatedCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isRelayEscheatable(inner) => {
                    <isRelayEscheatableCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::onRelayServerRegistered(inner) => {
                    <onRelayServerRegisteredCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::penalize(inner) => {
                    <penalizeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::relayCall(inner) => {
                    <relayCallCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setConfiguration(inner) => {
                    <setConfigurationCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setMinimumStakes(inner) => {
                    <setMinimumStakesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::supportsInterface(inner) => {
                    <supportsInterfaceCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::verifyRelayManagerStaked(inner) => {
                    <verifyRelayManagerStakedCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::versionHub(inner) => {
                    <versionHubCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::withdraw(inner) => {
                    <withdrawCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::withdrawMultiple(inner) => {
                    <withdrawMultipleCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`IRelayHub`](self) events.
    pub enum IRelayHubEvents {
        AbandonedRelayManagerBalanceEscheated(AbandonedRelayManagerBalanceEscheated),
        Deposited(Deposited),
        HubDeprecated(HubDeprecated),
        RelayHubConfigured(RelayHubConfigured),
        RelayWorkersAdded(RelayWorkersAdded),
        StakingTokenDataChanged(StakingTokenDataChanged),
        TransactionRejectedByPaymaster(TransactionRejectedByPaymaster),
        TransactionRelayed(TransactionRelayed),
        TransactionResult(TransactionResult),
        Withdrawn(Withdrawn),
    }
    #[automatically_derived]
    impl IRelayHubEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                12u8,
                71u8,
                81u8,
                12u8,
                185u8,
                0u8,
                144u8,
                26u8,
                252u8,
                167u8,
                201u8,
                233u8,
                38u8,
                137u8,
                37u8,
                130u8,
                163u8,
                111u8,
                211u8,
                187u8,
                127u8,
                144u8,
                129u8,
                66u8,
                204u8,
                240u8,
                251u8,
                67u8,
                130u8,
                162u8,
                182u8,
                132u8,
            ],
            [
                28u8,
                10u8,
                160u8,
                198u8,
                102u8,
                72u8,
                63u8,
                191u8,
                12u8,
                247u8,
                149u8,
                217u8,
                214u8,
                70u8,
                234u8,
                53u8,
                82u8,
                209u8,
                227u8,
                0u8,
                129u8,
                98u8,
                186u8,
                154u8,
                177u8,
                214u8,
                214u8,
                223u8,
                216u8,
                198u8,
                236u8,
                107u8,
            ],
            [
                63u8,
                182u8,
                114u8,
                160u8,
                97u8,
                185u8,
                195u8,
                171u8,
                8u8,
                48u8,
                98u8,
                169u8,
                168u8,
                255u8,
                83u8,
                45u8,
                52u8,
                30u8,
                95u8,
                164u8,
                33u8,
                69u8,
                185u8,
                31u8,
                57u8,
                34u8,
                167u8,
                17u8,
                174u8,
                19u8,
                86u8,
                89u8,
            ],
            [
                72u8,
                18u8,
                173u8,
                166u8,
                143u8,
                124u8,
                44u8,
                220u8,
                159u8,
                74u8,
                74u8,
                9u8,
                225u8,
                87u8,
                234u8,
                107u8,
                146u8,
                78u8,
                14u8,
                244u8,
                10u8,
                79u8,
                167u8,
                170u8,
                7u8,
                79u8,
                168u8,
                247u8,
                11u8,
                30u8,
                114u8,
                71u8,
            ],
            [
                135u8,
                82u8,
                164u8,
                114u8,
                229u8,
                113u8,
                168u8,
                22u8,
                174u8,
                169u8,
                46u8,
                236u8,
                141u8,
                174u8,
                155u8,
                175u8,
                98u8,
                142u8,
                132u8,
                15u8,
                73u8,
                41u8,
                251u8,
                204u8,
                45u8,
                21u8,
                94u8,
                98u8,
                51u8,
                255u8,
                104u8,
                167u8,
            ],
            [
                161u8,
                71u8,
                138u8,
                66u8,
                66u8,
                132u8,
                132u8,
                25u8,
                219u8,
                130u8,
                66u8,
                80u8,
                160u8,
                221u8,
                220u8,
                100u8,
                93u8,
                202u8,
                13u8,
                106u8,
                155u8,
                18u8,
                171u8,
                31u8,
                215u8,
                155u8,
                0u8,
                20u8,
                90u8,
                11u8,
                169u8,
                142u8,
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
                192u8,
                112u8,
                18u8,
                57u8,
                140u8,
                32u8,
                89u8,
                236u8,
                83u8,
                198u8,
                0u8,
                95u8,
                168u8,
                99u8,
                150u8,
                87u8,
                145u8,
                123u8,
                210u8,
                219u8,
                215u8,
                43u8,
                196u8,
                137u8,
                163u8,
                192u8,
                234u8,
                170u8,
                127u8,
                22u8,
            ],
            [
                235u8,
                244u8,
                169u8,
                191u8,
                251u8,
                57u8,
                247u8,
                197u8,
                219u8,
                243u8,
                246u8,
                85u8,
                64u8,
                24u8,
                59u8,
                147u8,
                129u8,
                174u8,
                34u8,
                106u8,
                195u8,
                208u8,
                164u8,
                91u8,
                76u8,
                173u8,
                72u8,
                71u8,
                19u8,
                189u8,
                74u8,
                40u8,
            ],
            [
                251u8,
                117u8,
                16u8,
                254u8,
                117u8,
                95u8,
                72u8,
                84u8,
                89u8,
                238u8,
                142u8,
                214u8,
                25u8,
                212u8,
                160u8,
                173u8,
                220u8,
                9u8,
                44u8,
                35u8,
                15u8,
                244u8,
                126u8,
                118u8,
                10u8,
                98u8,
                174u8,
                186u8,
                120u8,
                235u8,
                185u8,
                227u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for IRelayHubEvents {
        const NAME: &'static str = "IRelayHubEvents";
        const COUNT: usize = 10usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(
                    <AbandonedRelayManagerBalanceEscheated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <AbandonedRelayManagerBalanceEscheated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::AbandonedRelayManagerBalanceEscheated)
                }
                Some(<Deposited as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Deposited as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::Deposited)
                }
                Some(<HubDeprecated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <HubDeprecated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::HubDeprecated)
                }
                Some(
                    <RelayHubConfigured as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <RelayHubConfigured as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::RelayHubConfigured)
                }
                Some(
                    <RelayWorkersAdded as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <RelayWorkersAdded as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::RelayWorkersAdded)
                }
                Some(
                    <StakingTokenDataChanged as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <StakingTokenDataChanged as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::StakingTokenDataChanged)
                }
                Some(
                    <TransactionRejectedByPaymaster as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <TransactionRejectedByPaymaster as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::TransactionRejectedByPaymaster)
                }
                Some(
                    <TransactionRelayed as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <TransactionRelayed as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::TransactionRelayed)
                }
                Some(
                    <TransactionResult as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <TransactionResult as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::TransactionResult)
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
    impl alloy_sol_types::private::IntoLogData for IRelayHubEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::AbandonedRelayManagerBalanceEscheated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Deposited(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::HubDeprecated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::RelayHubConfigured(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::RelayWorkersAdded(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::StakingTokenDataChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::TransactionRejectedByPaymaster(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::TransactionRelayed(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::TransactionResult(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Withdrawn(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::AbandonedRelayManagerBalanceEscheated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Deposited(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::HubDeprecated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::RelayHubConfigured(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::RelayWorkersAdded(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::StakingTokenDataChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::TransactionRejectedByPaymaster(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::TransactionRelayed(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::TransactionResult(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Withdrawn(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`IRelayHub`](self) contract instance.

See the [wrapper's documentation](`IRelayHubInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IRelayHubInstance<T, P, N> {
        IRelayHubInstance::<T, P, N>::new(address, provider)
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
        Output = alloy_contract::Result<IRelayHubInstance<T, P, N>>,
    > {
        IRelayHubInstance::<T, P, N>::deploy(provider)
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
        IRelayHubInstance::<T, P, N>::deploy_builder(provider)
    }
    /**A [`IRelayHub`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`IRelayHub`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IRelayHubInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for IRelayHubInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IRelayHubInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IRelayHubInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`IRelayHub`](self) contract instance.

See the [wrapper's documentation](`IRelayHubInstance`) for more details.*/
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
        ) -> alloy_contract::Result<IRelayHubInstance<T, P, N>> {
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
    impl<T, P: ::core::clone::Clone, N> IRelayHubInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IRelayHubInstance<T, P, N> {
            IRelayHubInstance {
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
    > IRelayHubInstance<T, P, N> {
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
        ///Creates a new call builder for the [`addRelayWorkers`] function.
        pub fn addRelayWorkers(
            &self,
            newRelayWorkers: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, addRelayWorkersCall, N> {
            self.call_builder(
                &addRelayWorkersCall {
                    newRelayWorkers,
                },
            )
        }
        ///Creates a new call builder for the [`aggregateGasleft`] function.
        pub fn aggregateGasleft(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, aggregateGasleftCall, N> {
            self.call_builder(&aggregateGasleftCall {})
        }
        ///Creates a new call builder for the [`balanceOf`] function.
        pub fn balanceOf(
            &self,
            target: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, balanceOfCall, N> {
            self.call_builder(&balanceOfCall { target })
        }
        ///Creates a new call builder for the [`calculateCharge`] function.
        pub fn calculateCharge(
            &self,
            gasUsed: alloy::sol_types::private::primitives::aliases::U256,
            relayData: <GsnTypes::RelayData as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, calculateChargeCall, N> {
            self.call_builder(
                &calculateChargeCall {
                    gasUsed,
                    relayData,
                },
            )
        }
        ///Creates a new call builder for the [`calculateDevCharge`] function.
        pub fn calculateDevCharge(
            &self,
            charge: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, calculateDevChargeCall, N> {
            self.call_builder(&calculateDevChargeCall { charge })
        }
        ///Creates a new call builder for the [`depositFor`] function.
        pub fn depositFor(
            &self,
            target: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, depositForCall, N> {
            self.call_builder(&depositForCall { target })
        }
        ///Creates a new call builder for the [`deprecateHub`] function.
        pub fn deprecateHub(
            &self,
            _deprecationTime: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, deprecateHubCall, N> {
            self.call_builder(
                &deprecateHubCall {
                    _deprecationTime,
                },
            )
        }
        ///Creates a new call builder for the [`escheatAbandonedRelayBalance`] function.
        pub fn escheatAbandonedRelayBalance(
            &self,
            relayManager: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, escheatAbandonedRelayBalanceCall, N> {
            self.call_builder(
                &escheatAbandonedRelayBalanceCall {
                    relayManager,
                },
            )
        }
        ///Creates a new call builder for the [`getBatchGateway`] function.
        pub fn getBatchGateway(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getBatchGatewayCall, N> {
            self.call_builder(&getBatchGatewayCall {})
        }
        ///Creates a new call builder for the [`getConfiguration`] function.
        pub fn getConfiguration(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getConfigurationCall, N> {
            self.call_builder(&getConfigurationCall {})
        }
        ///Creates a new call builder for the [`getCreationBlock`] function.
        pub fn getCreationBlock(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getCreationBlockCall, N> {
            self.call_builder(&getCreationBlockCall {})
        }
        ///Creates a new call builder for the [`getDeprecationTime`] function.
        pub fn getDeprecationTime(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getDeprecationTimeCall, N> {
            self.call_builder(&getDeprecationTimeCall {})
        }
        ///Creates a new call builder for the [`getMinimumStakePerToken`] function.
        pub fn getMinimumStakePerToken(
            &self,
            token: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getMinimumStakePerTokenCall, N> {
            self.call_builder(
                &getMinimumStakePerTokenCall {
                    token,
                },
            )
        }
        ///Creates a new call builder for the [`getPenalizer`] function.
        pub fn getPenalizer(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getPenalizerCall, N> {
            self.call_builder(&getPenalizerCall {})
        }
        ///Creates a new call builder for the [`getRelayRegistrar`] function.
        pub fn getRelayRegistrar(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getRelayRegistrarCall, N> {
            self.call_builder(&getRelayRegistrarCall {})
        }
        ///Creates a new call builder for the [`getStakeManager`] function.
        pub fn getStakeManager(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getStakeManagerCall, N> {
            self.call_builder(&getStakeManagerCall {})
        }
        ///Creates a new call builder for the [`getWorkerCount`] function.
        pub fn getWorkerCount(
            &self,
            manager: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getWorkerCountCall, N> {
            self.call_builder(&getWorkerCountCall { manager })
        }
        ///Creates a new call builder for the [`getWorkerManager`] function.
        pub fn getWorkerManager(
            &self,
            worker: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getWorkerManagerCall, N> {
            self.call_builder(&getWorkerManagerCall { worker })
        }
        ///Creates a new call builder for the [`isDeprecated`] function.
        pub fn isDeprecated(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, isDeprecatedCall, N> {
            self.call_builder(&isDeprecatedCall {})
        }
        ///Creates a new call builder for the [`isRelayEscheatable`] function.
        pub fn isRelayEscheatable(
            &self,
            relayManager: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, isRelayEscheatableCall, N> {
            self.call_builder(
                &isRelayEscheatableCall {
                    relayManager,
                },
            )
        }
        ///Creates a new call builder for the [`onRelayServerRegistered`] function.
        pub fn onRelayServerRegistered(
            &self,
            relayManager: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, onRelayServerRegisteredCall, N> {
            self.call_builder(
                &onRelayServerRegisteredCall {
                    relayManager,
                },
            )
        }
        ///Creates a new call builder for the [`penalize`] function.
        pub fn penalize(
            &self,
            relayWorker: alloy::sol_types::private::Address,
            beneficiary: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, penalizeCall, N> {
            self.call_builder(
                &penalizeCall {
                    relayWorker,
                    beneficiary,
                },
            )
        }
        ///Creates a new call builder for the [`relayCall`] function.
        pub fn relayCall(
            &self,
            domainSeparatorName: alloy::sol_types::private::String,
            maxAcceptanceBudget: alloy::sol_types::private::primitives::aliases::U256,
            relayRequest: <GsnTypes::RelayRequest as alloy::sol_types::SolType>::RustType,
            signature: alloy::sol_types::private::Bytes,
            approvalData: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, relayCallCall, N> {
            self.call_builder(
                &relayCallCall {
                    domainSeparatorName,
                    maxAcceptanceBudget,
                    relayRequest,
                    signature,
                    approvalData,
                },
            )
        }
        ///Creates a new call builder for the [`setConfiguration`] function.
        pub fn setConfiguration(
            &self,
            _config: <RelayHubConfig as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, setConfigurationCall, N> {
            self.call_builder(&setConfigurationCall { _config })
        }
        ///Creates a new call builder for the [`setMinimumStakes`] function.
        pub fn setMinimumStakes(
            &self,
            token: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            minimumStake: alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U256,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, setMinimumStakesCall, N> {
            self.call_builder(
                &setMinimumStakesCall {
                    token,
                    minimumStake,
                },
            )
        }
        ///Creates a new call builder for the [`supportsInterface`] function.
        pub fn supportsInterface(
            &self,
            interfaceId: alloy::sol_types::private::FixedBytes<4>,
        ) -> alloy_contract::SolCallBuilder<T, &P, supportsInterfaceCall, N> {
            self.call_builder(
                &supportsInterfaceCall {
                    interfaceId,
                },
            )
        }
        ///Creates a new call builder for the [`verifyRelayManagerStaked`] function.
        pub fn verifyRelayManagerStaked(
            &self,
            relayManager: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, verifyRelayManagerStakedCall, N> {
            self.call_builder(
                &verifyRelayManagerStakedCall {
                    relayManager,
                },
            )
        }
        ///Creates a new call builder for the [`versionHub`] function.
        pub fn versionHub(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, versionHubCall, N> {
            self.call_builder(&versionHubCall {})
        }
        ///Creates a new call builder for the [`withdraw`] function.
        pub fn withdraw(
            &self,
            dest: alloy::sol_types::private::Address,
            amount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, withdrawCall, N> {
            self.call_builder(&withdrawCall { dest, amount })
        }
        ///Creates a new call builder for the [`withdrawMultiple`] function.
        pub fn withdrawMultiple(
            &self,
            dest: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            amount: alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U256,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, withdrawMultipleCall, N> {
            self.call_builder(
                &withdrawMultipleCall {
                    dest,
                    amount,
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
    > IRelayHubInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`AbandonedRelayManagerBalanceEscheated`] event.
        pub fn AbandonedRelayManagerBalanceEscheated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, AbandonedRelayManagerBalanceEscheated, N> {
            self.event_filter::<AbandonedRelayManagerBalanceEscheated>()
        }
        ///Creates a new event filter for the [`Deposited`] event.
        pub fn Deposited_filter(&self) -> alloy_contract::Event<T, &P, Deposited, N> {
            self.event_filter::<Deposited>()
        }
        ///Creates a new event filter for the [`HubDeprecated`] event.
        pub fn HubDeprecated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, HubDeprecated, N> {
            self.event_filter::<HubDeprecated>()
        }
        ///Creates a new event filter for the [`RelayHubConfigured`] event.
        pub fn RelayHubConfigured_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, RelayHubConfigured, N> {
            self.event_filter::<RelayHubConfigured>()
        }
        ///Creates a new event filter for the [`RelayWorkersAdded`] event.
        pub fn RelayWorkersAdded_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, RelayWorkersAdded, N> {
            self.event_filter::<RelayWorkersAdded>()
        }
        ///Creates a new event filter for the [`StakingTokenDataChanged`] event.
        pub fn StakingTokenDataChanged_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, StakingTokenDataChanged, N> {
            self.event_filter::<StakingTokenDataChanged>()
        }
        ///Creates a new event filter for the [`TransactionRejectedByPaymaster`] event.
        pub fn TransactionRejectedByPaymaster_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, TransactionRejectedByPaymaster, N> {
            self.event_filter::<TransactionRejectedByPaymaster>()
        }
        ///Creates a new event filter for the [`TransactionRelayed`] event.
        pub fn TransactionRelayed_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, TransactionRelayed, N> {
            self.event_filter::<TransactionRelayed>()
        }
        ///Creates a new event filter for the [`TransactionResult`] event.
        pub fn TransactionResult_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, TransactionResult, N> {
            self.event_filter::<TransactionResult>()
        }
        ///Creates a new event filter for the [`Withdrawn`] event.
        pub fn Withdrawn_filter(&self) -> alloy_contract::Event<T, &P, Withdrawn, N> {
            self.event_filter::<Withdrawn>()
        }
    }
}
