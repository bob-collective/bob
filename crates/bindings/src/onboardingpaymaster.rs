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
///Module containing a contract's types and functions.
/**

```solidity
library IPaymaster {
    struct GasAndDataLimits { uint256 acceptanceBudget; uint256 preRelayedCallGasLimit; uint256 postRelayedCallGasLimit; uint256 calldataSizeLimit; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style
)]
pub mod IPaymaster {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
struct GasAndDataLimits { uint256 acceptanceBudget; uint256 preRelayedCallGasLimit; uint256 postRelayedCallGasLimit; uint256 calldataSizeLimit; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct GasAndDataLimits {
        pub acceptanceBudget: alloy::sol_types::private::primitives::aliases::U256,
        pub preRelayedCallGasLimit: alloy::sol_types::private::primitives::aliases::U256,
        pub postRelayedCallGasLimit: alloy::sol_types::private::primitives::aliases::U256,
        pub calldataSizeLimit: alloy::sol_types::private::primitives::aliases::U256,
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
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<GasAndDataLimits> for UnderlyingRustTuple<'_> {
            fn from(value: GasAndDataLimits) -> Self {
                (
                    value.acceptanceBudget,
                    value.preRelayedCallGasLimit,
                    value.postRelayedCallGasLimit,
                    value.calldataSizeLimit,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for GasAndDataLimits {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    acceptanceBudget: tuple.0,
                    preRelayedCallGasLimit: tuple.1,
                    postRelayedCallGasLimit: tuple.2,
                    calldataSizeLimit: tuple.3,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for GasAndDataLimits {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for GasAndDataLimits {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.acceptanceBudget),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.preRelayedCallGasLimit,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.postRelayedCallGasLimit,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.calldataSizeLimit),
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
        impl alloy_sol_types::SolType for GasAndDataLimits {
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
        impl alloy_sol_types::SolStruct for GasAndDataLimits {
            const NAME: &'static str = "GasAndDataLimits";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "GasAndDataLimits(uint256 acceptanceBudget,uint256 preRelayedCallGasLimit,uint256 postRelayedCallGasLimit,uint256 calldataSizeLimit)",
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
                            &self.acceptanceBudget,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.preRelayedCallGasLimit,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.postRelayedCallGasLimit,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.calldataSizeLimit,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for GasAndDataLimits {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.acceptanceBudget,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.preRelayedCallGasLimit,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.postRelayedCallGasLimit,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.calldataSizeLimit,
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
                    &rust.acceptanceBudget,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.preRelayedCallGasLimit,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.postRelayedCallGasLimit,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.calldataSizeLimit,
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
    /**Creates a new wrapper around an on-chain [`IPaymaster`](self) contract instance.

See the [wrapper's documentation](`IPaymasterInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IPaymasterInstance<T, P, N> {
        IPaymasterInstance::<T, P, N>::new(address, provider)
    }
    /**A [`IPaymaster`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`IPaymaster`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IPaymasterInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for IPaymasterInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IPaymasterInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IPaymasterInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`IPaymaster`](self) contract instance.

See the [wrapper's documentation](`IPaymasterInstance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> IPaymasterInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IPaymasterInstance<T, P, N> {
            IPaymasterInstance {
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
    > IPaymasterInstance<T, P, N> {
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
    > IPaymasterInstance<T, P, N> {
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

library IPaymaster {
    struct GasAndDataLimits {
        uint256 acceptanceBudget;
        uint256 preRelayedCallGasLimit;
        uint256 postRelayedCallGasLimit;
        uint256 calldataSizeLimit;
    }
}

interface OnboardingPaymaster {
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    event PostRelay(address indexed sender);
    event PreRelay(address indexed sender);

    constructor(address _whitelistedContract, uint32 _whitelistedSelector);

    receive() external payable;

    function CALLDATA_SIZE_LIMIT() external view returns (uint256);
    function FORWARDER_HUB_OVERHEAD() external view returns (uint256);
    function PAYMASTER_ACCEPTANCE_BUDGET() external view returns (uint256);
    function POST_RELAYED_CALL_GAS_LIMIT() external view returns (uint256);
    function PRE_RELAYED_CALL_GAS_LIMIT() external view returns (uint256);
    function gasUsedByPost() external view returns (uint256);
    function getGasAndDataLimits() external view returns (IPaymaster.GasAndDataLimits memory limits);
    function getRelayHub() external view returns (address);
    function getSelector(bytes memory call) external pure returns (uint32);
    function getTrustedForwarder() external view returns (address);
    function owner() external view returns (address);
    function postRelayedCall(bytes memory context, bool success, uint256 gasUseWithoutPost, GsnTypes.RelayData memory relayData) external;
    function preRelayedCall(GsnTypes.RelayRequest memory relayRequest, bytes memory signature, bytes memory approvalData, uint256 maxPossibleGas) external returns (bytes memory, bool);
    function renounceOwnership() external;
    function setPostGasUsage(uint256 _gasUsedByPost) external;
    function setRelayHub(address hub) external;
    function setTrustedForwarder(address forwarder) external;
    function supportsInterface(bytes4 interfaceId) external view returns (bool);
    function transferOwnership(address newOwner) external;
    function versionPaymaster() external view returns (string memory);
    function whitelistedContract() external view returns (address);
    function whitelistedSelector() external view returns (uint32);
    function withdrawRelayHubDepositTo(uint256 amount, address payable target) external;
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [
      {
        "name": "_whitelistedContract",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "_whitelistedSelector",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "receive",
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "CALLDATA_SIZE_LIMIT",
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
    "name": "FORWARDER_HUB_OVERHEAD",
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
    "name": "PAYMASTER_ACCEPTANCE_BUDGET",
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
    "name": "POST_RELAYED_CALL_GAS_LIMIT",
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
    "name": "PRE_RELAYED_CALL_GAS_LIMIT",
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
    "name": "gasUsedByPost",
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
    "name": "getGasAndDataLimits",
    "inputs": [],
    "outputs": [
      {
        "name": "limits",
        "type": "tuple",
        "internalType": "struct IPaymaster.GasAndDataLimits",
        "components": [
          {
            "name": "acceptanceBudget",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "preRelayedCallGasLimit",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "postRelayedCallGasLimit",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "calldataSizeLimit",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getRelayHub",
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
    "name": "getSelector",
    "inputs": [
      {
        "name": "call",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "stateMutability": "pure"
  },
  {
    "type": "function",
    "name": "getTrustedForwarder",
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
    "name": "owner",
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
    "name": "postRelayedCall",
    "inputs": [
      {
        "name": "context",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "success",
        "type": "bool",
        "internalType": "bool"
      },
      {
        "name": "gasUseWithoutPost",
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
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "preRelayedCall",
    "inputs": [
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
      },
      {
        "name": "maxPossibleGas",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "renounceOwnership",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setPostGasUsage",
    "inputs": [
      {
        "name": "_gasUsedByPost",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setRelayHub",
    "inputs": [
      {
        "name": "hub",
        "type": "address",
        "internalType": "contract IRelayHub"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setTrustedForwarder",
    "inputs": [
      {
        "name": "forwarder",
        "type": "address",
        "internalType": "address"
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
    "name": "transferOwnership",
    "inputs": [
      {
        "name": "newOwner",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "versionPaymaster",
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
    "name": "whitelistedContract",
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
    "name": "whitelistedSelector",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "withdrawRelayHubDepositTo",
    "inputs": [
      {
        "name": "amount",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "target",
        "type": "address",
        "internalType": "address payable"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "event",
    "name": "OwnershipTransferred",
    "inputs": [
      {
        "name": "previousOwner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "newOwner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "PostRelay",
    "inputs": [
      {
        "name": "sender",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "PreRelay",
    "inputs": [
      {
        "name": "sender",
        "type": "address",
        "indexed": true,
        "internalType": "address"
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
pub mod OnboardingPaymaster {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b50604051611a68380380611a6883398101604081905261002e916100bb565b6100373361006c565b6003805463ffffffff909216600160a01b026001600160c01b03199092166001600160a01b0390931692909217179055610106565b5f80546001600160a01b038381166001600160a01b0319831681178455604051919092169283917f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09190a35050565b5f5f604083850312156100cc575f5ffd5b82516001600160a01b03811681146100e2575f5ffd5b602084015190925063ffffffff811681146100fb575f5ffd5b809150509250929050565b611955806101135f395ff3fe60806040526004361061017a575f3560e01c80638da5cb5b116100d1578063c3a3e2fa1161007c578063df463a6611610057578063df463a6614610525578063f2fde38b14610539578063f9c002f714610558575f5ffd5b8063c3a3e2fa146104b5578063ce1b815f146104e9578063da74222814610506575f5ffd5b8063b039a88f116100ac578063b039a88f14610443578063b90b41cf1461048a578063bbdaa3c91461049f575f5ffd5b80638da5cb5b146103f1578063921276ea1461040d578063ad12e50e1461042e575f5ffd5b80635c5e3db11161013157806376fa01c31161010c57806376fa01c3146103965780637bb05264146103b55780637bdf2ec7146103d4575f5ffd5b80635c5e3db1146103405780636d7c3e2b14610363578063715018a614610382575f5ffd5b80630cbd17c8116101615780630cbd17c8146102b657806318e45427146102ea5780632d14c4b714610321575f5ffd5b8062be5dd41461025157806301ffc9a714610287575f5ffd5b3661024d576001546001600160a01b03166101dc5760405162461bcd60e51b815260206004820152601960248201527f72656c6179206875622061646472657373206e6f74207365740000000000000060448201526064015b60405180910390fd5b6001546040517faa67c9190000000000000000000000000000000000000000000000000000000081523060048201526001600160a01b039091169063aa67c9199034906024015f604051808303818588803b158015610239575f5ffd5b505af115801561024b573d5f5f3e3d5ffd5b005b5f5ffd5b34801561025c575f5ffd5b5061027061026b366004611499565b61056e565b60405161027e92919061156d565b60405180910390f35b348015610292575f5ffd5b506102a66102a1366004611590565b6105bb565b604051901515815260200161027e565b3480156102c1575f5ffd5b506102d56102d03660046115cf565b61069f565b60405163ffffffff909116815260200161027e565b3480156102f5575f5ffd5b50600354610309906001600160a01b031681565b6040516001600160a01b03909116815260200161027e565b34801561032c575f5ffd5b5061024b61033b366004611622565b6106c2565b34801561034b575f5ffd5b5061035561290481565b60405190815260200161027e565b34801561036e575f5ffd5b5061024b61037d366004611650565b610799565b34801561038d575f5ffd5b5061024b6107f7565b3480156103a1575f5ffd5b5061024b6103b0366004611674565b61085b565b3480156103c0575f5ffd5b5061024b6103cf366004611700565b610877565b3480156103df575f5ffd5b506001546001600160a01b0316610309565b3480156103fc575f5ffd5b505f546001600160a01b0316610309565b348015610418575f5ffd5b50610421610989565b60405161027e919061171b565b348015610439575f5ffd5b5061035560045481565b34801561044e575f5ffd5b506104576109a9565b60405161027e91908151815260208083015190820152604080830151908201526060918201519181019190915260800190565b348015610495575f5ffd5b5061035561c35081565b3480156104aa575f5ffd5b506103556201adb081565b3480156104c0575f5ffd5b506003546102d59074010000000000000000000000000000000000000000900463ffffffff1681565b3480156104f4575f5ffd5b506002546001600160a01b0316610309565b348015610511575f5ffd5b5061024b610520366004611700565b610a0e565b348015610530575f5ffd5b50610355610b20565b348015610544575f5ffd5b5061024b610553366004611700565b610b32565b348015610563575f5ffd5b50610355620186a081565b60605f610579610c13565b61058288610c6d565b61058b88610d05565b61059488610d60565b61059e8585610dca565b6105ac888888888888610e1c565b91509150965096945050505050565b5f7fffffffff0000000000000000000000000000000000000000000000000000000082167fe1ab2dea00000000000000000000000000000000000000000000000000000000148061064d57507fffffffff0000000000000000000000000000000000000000000000000000000082167f0e08307600000000000000000000000000000000000000000000000000000000145b8061069957507f01ffc9a7000000000000000000000000000000000000000000000000000000007fffffffff000000000000000000000000000000000000000000000000000000008316145b92915050565b5f806106ae600482858761172d565b6106b791611754565b60e01c949350505050565b5f546001600160a01b0316331461071b5760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e657260448201526064016101d3565b6001546040517ff3fef3a30000000000000000000000000000000000000000000000000000000081526001600160a01b038381166004830152602482018590529091169063f3fef3a3906044015f604051808303815f87803b15801561077f575f5ffd5b505af1158015610791573d5f5f3e3d5ffd5b505050505050565b5f546001600160a01b031633146107f25760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e657260448201526064016101d3565b600455565b5f546001600160a01b031633146108505760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e657260448201526064016101d3565b6108595f610fc8565b565b610863610c13565b610870858585858561102f565b5050505050565b5f546001600160a01b031633146108d05760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e657260448201526064016101d3565b6109036001600160a01b0382167fe9fb30f70000000000000000000000000000000000000000000000000000000061107a565b61094f5760405162461bcd60e51b815260206004820152601f60248201527f746172676574206973206e6f7420612076616c6964204952656c61794875620060448201526064016101d3565b600180547fffffffffffffffffffffffff0000000000000000000000000000000000000000166001600160a01b0392909216919091179055565b60606040518060600160405280602d81526020016118f3602d9139905090565b6109d060405180608001604052805f81526020015f81526020015f81526020015f81525090565b604051806080016040528061c350620186a06109ec91906117ba565b8152602001620186a081526020016201adb08152602001612904815250905090565b5f546001600160a01b03163314610a675760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e657260448201526064016101d3565b610a9a6001600160a01b0382167f25e23e640000000000000000000000000000000000000000000000000000000061107a565b610ae65760405162461bcd60e51b815260206004820181905260248201527f746172676574206973206e6f7420612076616c69642049466f7277617264657260448201526064016101d3565b600280547fffffffffffffffffffffffff0000000000000000000000000000000000000000166001600160a01b0392909216919091179055565b610b2f61c350620186a06117ba565b81565b5f546001600160a01b03163314610b8b5760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e657260448201526064016101d3565b6001600160a01b038116610c075760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201527f646472657373000000000000000000000000000000000000000000000000000060648201526084016101d3565b610c1081610fc8565b50565b6001546001600160a01b031633146108595760405162461bcd60e51b815260206004820152601e60248201527f63616e206f6e6c792062652063616c6c65642062792052656c6179487562000060448201526064016101d3565b610c7a60208201826117f2565b610c8b9060c081019060a001611700565b6001600160a01b0316610ca66002546001600160a01b031690565b6001600160a01b031614610cfc5760405162461bcd60e51b815260206004820152601860248201527f466f72776172646572206973206e6f742074727573746564000000000000000060448201526064016101d3565b610c108161109c565b610d0f818061182e565b6040013515610c105760405162461bcd60e51b815260206004820152601c60248201527f76616c7565207472616e73666572206e6f7420737570706f727465640000000060448201526064016101d3565b610d6d60208201826117f2565b610d7b9060c0810190611860565b159050610c105760405162461bcd60e51b815260206004820152601c60248201527f73686f756c642068617665206e6f207061796d6173746572446174610000000060448201526064016101d3565b8015610e185760405162461bcd60e51b815260206004820152601b60248201527f73686f756c642068617665206e6f20617070726f76616c44617461000000000060448201526064016101d3565b5050565b6003546060905f906001600160a01b0316610e37898061182e565b610e48906040810190602001611700565b6001600160a01b031614610e9e5760405162461bcd60e51b815260206004820152601c60248201527f526563697069656e74206973206e6f742077686974656c69737465640000000060448201526064016101d3565b5f610eba610eac8a8061182e565b6102d09060a0810190611860565b60035490915063ffffffff808316740100000000000000000000000000000000000000009092041614610f2f5760405162461bcd60e51b815260206004820152601b60248201527f53656c6563746f72206973206e6f742077686974656c6973746564000000000060448201526064016101d3565b610f39898061182e565b610f47906020810190611700565b6001600160a01b03167f717bf6fb351c316ed19e6418337257bf2c5a7a12eca78e4d26d10d94c1c497cf60405160405180910390a2610f86898061182e565b610f94906020810190611700565b604080516001600160a01b0390921660208301520160408051601f19818403018152919052995f9950975050505050505050565b5f80546001600160a01b038381167fffffffffffffffffffffffff0000000000000000000000000000000000000000831681178455604051919092169283917f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09190a35050565b5f61103c85870187611700565b6040519091506001600160a01b038216907f318bf636e31eb4adc594f14c43ac0b50bf449c959e8344adc6368012d8b83c5f905f90a2505050505050565b5f611084836112c7565b80156110955750611095838361132a565b9392505050565b5f806110a8838061182e565b6110b9906040810190602001611700565b6001600160a01b03167f572b6c05000000000000000000000000000000000000000000000000000000006110f060208601866117f2565b6111019060c081019060a001611700565b6040516001600160a01b03909116602482015260440160408051601f198184030181529181526020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167fffffffff0000000000000000000000000000000000000000000000000000000090941693909317909252905161118291906118c1565b5f60405180830381855afa9150503d805f81146111ba576040519150601f19603f3d011682016040523d82523d5f602084013e6111bf565b606091505b5091509150816112115760405162461bcd60e51b815260206004820152601c60248201527f697354727573746564466f727761726465723a2072657665727465640000000060448201526064016101d3565b80516020146112625760405162461bcd60e51b815260206004820181905260248201527f697354727573746564466f727761726465723a2062616420726573706f6e736560448201526064016101d3565b8080602001905181019061127691906118d7565b6112c25760405162461bcd60e51b815260206004820152601f60248201527f696e76616c696420666f7277617264657220666f7220726563697069656e740060448201526064016101d3565b505050565b5f6112f2827f01ffc9a70000000000000000000000000000000000000000000000000000000061132a565b80156106995750611323827fffffffff0000000000000000000000000000000000000000000000000000000061132a565b1592915050565b604080517fffffffff00000000000000000000000000000000000000000000000000000000831660248083019190915282518083039091018152604490910182526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f01ffc9a70000000000000000000000000000000000000000000000000000000017905290515f9190829081906001600160a01b03871690617530906113d69086906118c1565b5f604051808303818686fa925050503d805f811461140f576040519150601f19603f3d011682016040523d82523d5f602084013e611414565b606091505b509150915060208151101561142e575f9350505050610699565b81801561144a57508080602001905181019061144a91906118d7565b9695505050505050565b5f5f83601f840112611464575f5ffd5b50813567ffffffffffffffff81111561147b575f5ffd5b602083019150836020828501011115611492575f5ffd5b9250929050565b5f5f5f5f5f5f608087890312156114ae575f5ffd5b863567ffffffffffffffff8111156114c4575f5ffd5b87016040818a0312156114d5575f5ffd5b9550602087013567ffffffffffffffff8111156114f0575f5ffd5b6114fc89828a01611454565b909650945050604087013567ffffffffffffffff81111561151b575f5ffd5b61152789828a01611454565b979a9699509497949695606090950135949350505050565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b604081525f61157f604083018561153f565b905082151560208301529392505050565b5f602082840312156115a0575f5ffd5b81357fffffffff0000000000000000000000000000000000000000000000000000000081168114611095575f5ffd5b5f5f602083850312156115e0575f5ffd5b823567ffffffffffffffff8111156115f6575f5ffd5b61160285828601611454565b90969095509350505050565b6001600160a01b0381168114610c10575f5ffd5b5f5f60408385031215611633575f5ffd5b8235915060208301356116458161160e565b809150509250929050565b5f60208284031215611660575f5ffd5b5035919050565b8015158114610c10575f5ffd5b5f5f5f5f5f60808688031215611688575f5ffd5b853567ffffffffffffffff81111561169e575f5ffd5b6116aa88828901611454565b90965094505060208601356116be81611667565b925060408601359150606086013567ffffffffffffffff8111156116e0575f5ffd5b860161010081890312156116f2575f5ffd5b809150509295509295909350565b5f60208284031215611710575f5ffd5b81356110958161160e565b602081525f611095602083018461153f565b5f5f8585111561173b575f5ffd5b83861115611747575f5ffd5b5050820193919092039150565b80357fffffffff0000000000000000000000000000000000000000000000000000000081169060048410156117b3577fffffffff00000000000000000000000000000000000000000000000000000000808560040360031b1b82161691505b5092915050565b80820180821115610699577f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f82357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff01833603018112611824575f5ffd5b9190910192915050565b5f82357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff21833603018112611824575f5ffd5b5f5f83357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe1843603018112611893575f5ffd5b83018035915067ffffffffffffffff8211156118ad575f5ffd5b602001915036819003821315611492575f5ffd5b5f82518060208501845e5f920191825250919050565b5f602082840312156118e7575f5ffd5b81516110958161166756fe332e302e302d626574612e31302b6f70656e67736e2e6f7261636c652e746f6b656e2e697061796d6173746572a2646970667358221220fff5f3500f92f1da9e4ce14440022195c63be4f0490434f07077f3ebafaabd1864736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`@Qa\x1Ah8\x03\x80a\x1Ah\x839\x81\x01`@\x81\x90Ra\0.\x91a\0\xBBV[a\x0073a\0lV[`\x03\x80Tc\xFF\xFF\xFF\xFF\x90\x92\x16`\x01`\xA0\x1B\x02`\x01`\x01`\xC0\x1B\x03\x19\x90\x92\x16`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92\x90\x92\x17\x17\x90Ua\x01\x06V[_\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[__`@\x83\x85\x03\x12\x15a\0\xCCW__\xFD[\x82Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0\xE2W__\xFD[` \x84\x01Q\x90\x92Pc\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\0\xFBW__\xFD[\x80\x91PP\x92P\x92\x90PV[a\x19U\x80a\x01\x13_9_\xF3\xFE`\x80`@R`\x046\x10a\x01zW_5`\xE0\x1C\x80c\x8D\xA5\xCB[\x11a\0\xD1W\x80c\xC3\xA3\xE2\xFA\x11a\0|W\x80c\xDFF:f\x11a\0WW\x80c\xDFF:f\x14a\x05%W\x80c\xF2\xFD\xE3\x8B\x14a\x059W\x80c\xF9\xC0\x02\xF7\x14a\x05XW__\xFD[\x80c\xC3\xA3\xE2\xFA\x14a\x04\xB5W\x80c\xCE\x1B\x81_\x14a\x04\xE9W\x80c\xDAt\"(\x14a\x05\x06W__\xFD[\x80c\xB09\xA8\x8F\x11a\0\xACW\x80c\xB09\xA8\x8F\x14a\x04CW\x80c\xB9\x0BA\xCF\x14a\x04\x8AW\x80c\xBB\xDA\xA3\xC9\x14a\x04\x9FW__\xFD[\x80c\x8D\xA5\xCB[\x14a\x03\xF1W\x80c\x92\x12v\xEA\x14a\x04\rW\x80c\xAD\x12\xE5\x0E\x14a\x04.W__\xFD[\x80c\\^=\xB1\x11a\x011W\x80cv\xFA\x01\xC3\x11a\x01\x0CW\x80cv\xFA\x01\xC3\x14a\x03\x96W\x80c{\xB0Rd\x14a\x03\xB5W\x80c{\xDF.\xC7\x14a\x03\xD4W__\xFD[\x80c\\^=\xB1\x14a\x03@W\x80cm|>+\x14a\x03cW\x80cqP\x18\xA6\x14a\x03\x82W__\xFD[\x80c\x0C\xBD\x17\xC8\x11a\x01aW\x80c\x0C\xBD\x17\xC8\x14a\x02\xB6W\x80c\x18\xE4T'\x14a\x02\xEAW\x80c-\x14\xC4\xB7\x14a\x03!W__\xFD[\x80b\xBE]\xD4\x14a\x02QW\x80c\x01\xFF\xC9\xA7\x14a\x02\x87W__\xFD[6a\x02MW`\x01T`\x01`\x01`\xA0\x1B\x03\x16a\x01\xDCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7Frelay hub address not set\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\x01T`@Q\x7F\xAAg\xC9\x19\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xAAg\xC9\x19\x904\x90`$\x01_`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\x029W__\xFD[PZ\xF1\x15\x80\x15a\x02KW=__>=_\xFD[\0[__\xFD[4\x80\x15a\x02\\W__\xFD[Pa\x02pa\x02k6`\x04a\x14\x99V[a\x05nV[`@Qa\x02~\x92\x91\x90a\x15mV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\x92W__\xFD[Pa\x02\xA6a\x02\xA16`\x04a\x15\x90V[a\x05\xBBV[`@Q\x90\x15\x15\x81R` \x01a\x02~V[4\x80\x15a\x02\xC1W__\xFD[Pa\x02\xD5a\x02\xD06`\x04a\x15\xCFV[a\x06\x9FV[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02~V[4\x80\x15a\x02\xF5W__\xFD[P`\x03Ta\x03\t\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02~V[4\x80\x15a\x03,W__\xFD[Pa\x02Ka\x03;6`\x04a\x16\"V[a\x06\xC2V[4\x80\x15a\x03KW__\xFD[Pa\x03Ua)\x04\x81V[`@Q\x90\x81R` \x01a\x02~V[4\x80\x15a\x03nW__\xFD[Pa\x02Ka\x03}6`\x04a\x16PV[a\x07\x99V[4\x80\x15a\x03\x8DW__\xFD[Pa\x02Ka\x07\xF7V[4\x80\x15a\x03\xA1W__\xFD[Pa\x02Ka\x03\xB06`\x04a\x16tV[a\x08[V[4\x80\x15a\x03\xC0W__\xFD[Pa\x02Ka\x03\xCF6`\x04a\x17\0V[a\x08wV[4\x80\x15a\x03\xDFW__\xFD[P`\x01T`\x01`\x01`\xA0\x1B\x03\x16a\x03\tV[4\x80\x15a\x03\xFCW__\xFD[P_T`\x01`\x01`\xA0\x1B\x03\x16a\x03\tV[4\x80\x15a\x04\x18W__\xFD[Pa\x04!a\t\x89V[`@Qa\x02~\x91\x90a\x17\x1BV[4\x80\x15a\x049W__\xFD[Pa\x03U`\x04T\x81V[4\x80\x15a\x04NW__\xFD[Pa\x04Wa\t\xA9V[`@Qa\x02~\x91\x90\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x80\x83\x01Q\x90\x82\x01R``\x91\x82\x01Q\x91\x81\x01\x91\x90\x91R`\x80\x01\x90V[4\x80\x15a\x04\x95W__\xFD[Pa\x03Ua\xC3P\x81V[4\x80\x15a\x04\xAAW__\xFD[Pa\x03Ub\x01\xAD\xB0\x81V[4\x80\x15a\x04\xC0W__\xFD[P`\x03Ta\x02\xD5\x90t\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[4\x80\x15a\x04\xF4W__\xFD[P`\x02T`\x01`\x01`\xA0\x1B\x03\x16a\x03\tV[4\x80\x15a\x05\x11W__\xFD[Pa\x02Ka\x05 6`\x04a\x17\0V[a\n\x0EV[4\x80\x15a\x050W__\xFD[Pa\x03Ua\x0B V[4\x80\x15a\x05DW__\xFD[Pa\x02Ka\x05S6`\x04a\x17\0V[a\x0B2V[4\x80\x15a\x05cW__\xFD[Pa\x03Ub\x01\x86\xA0\x81V[``_a\x05ya\x0C\x13V[a\x05\x82\x88a\x0CmV[a\x05\x8B\x88a\r\x05V[a\x05\x94\x88a\r`V[a\x05\x9E\x85\x85a\r\xCAV[a\x05\xAC\x88\x88\x88\x88\x88\x88a\x0E\x1CV[\x91P\x91P\x96P\x96\x94PPPPPV[_\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x7F\xE1\xAB-\xEA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x14\x80a\x06MWP\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x7F\x0E\x080v\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x14[\x80a\x06\x99WP\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x14[\x92\x91PPV[_\x80a\x06\xAE`\x04\x82\x85\x87a\x17-V[a\x06\xB7\x91a\x17TV[`\xE0\x1C\x94\x93PPPPV[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x07\x1BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x01\xD3V[`\x01T`@Q\x7F\xF3\xFE\xF3\xA3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R`$\x82\x01\x85\x90R\x90\x91\x16\x90c\xF3\xFE\xF3\xA3\x90`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x07\x7FW__\xFD[PZ\xF1\x15\x80\x15a\x07\x91W=__>=_\xFD[PPPPPPV[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x07\xF2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x01\xD3V[`\x04UV[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x08PW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x01\xD3V[a\x08Y_a\x0F\xC8V[V[a\x08ca\x0C\x13V[a\x08p\x85\x85\x85\x85\x85a\x10/V[PPPPPV[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x08\xD0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x01\xD3V[a\t\x03`\x01`\x01`\xA0\x1B\x03\x82\x16\x7F\xE9\xFB0\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x10zV[a\tOW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7Ftarget is not a valid IRelayHub\0`D\x82\x01R`d\x01a\x01\xD3V[`\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[```@Q\x80``\x01`@R\x80`-\x81R` \x01a\x18\xF3`-\x919\x90P\x90V[a\t\xD0`@Q\x80`\x80\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80`\x80\x01`@R\x80a\xC3Pb\x01\x86\xA0a\t\xEC\x91\x90a\x17\xBAV[\x81R` \x01b\x01\x86\xA0\x81R` \x01b\x01\xAD\xB0\x81R` \x01a)\x04\x81RP\x90P\x90V[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\ngW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x01\xD3V[a\n\x9A`\x01`\x01`\xA0\x1B\x03\x82\x16\x7F%\xE2>d\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x10zV[a\n\xE6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7Ftarget is not a valid IForwarder`D\x82\x01R`d\x01a\x01\xD3V[`\x02\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[a\x0B/a\xC3Pb\x01\x86\xA0a\x17\xBAV[\x81V[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0B\x8BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x01\xD3V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x0C\x07W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x01\xD3V[a\x0C\x10\x81a\x0F\xC8V[PV[`\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x08YW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7Fcan only be called by RelayHub\0\0`D\x82\x01R`d\x01a\x01\xD3V[a\x0Cz` \x82\x01\x82a\x17\xF2V[a\x0C\x8B\x90`\xC0\x81\x01\x90`\xA0\x01a\x17\0V[`\x01`\x01`\xA0\x1B\x03\x16a\x0C\xA6`\x02T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0C\xFCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FForwarder is not trusted\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x01\xD3V[a\x0C\x10\x81a\x10\x9CV[a\r\x0F\x81\x80a\x18.V[`@\x015\x15a\x0C\x10W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7Fvalue transfer not supported\0\0\0\0`D\x82\x01R`d\x01a\x01\xD3V[a\rm` \x82\x01\x82a\x17\xF2V[a\r{\x90`\xC0\x81\x01\x90a\x18`V[\x15\x90Pa\x0C\x10W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7Fshould have no paymasterData\0\0\0\0`D\x82\x01R`d\x01a\x01\xD3V[\x80\x15a\x0E\x18W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7Fshould have no approvalData\0\0\0\0\0`D\x82\x01R`d\x01a\x01\xD3V[PPV[`\x03T``\x90_\x90`\x01`\x01`\xA0\x1B\x03\x16a\x0E7\x89\x80a\x18.V[a\x0EH\x90`@\x81\x01\x90` \x01a\x17\0V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0E\x9EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FRecipient is not whitelisted\0\0\0\0`D\x82\x01R`d\x01a\x01\xD3V[_a\x0E\xBAa\x0E\xAC\x8A\x80a\x18.V[a\x02\xD0\x90`\xA0\x81\x01\x90a\x18`V[`\x03T\x90\x91Pc\xFF\xFF\xFF\xFF\x80\x83\x16t\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x92\x04\x16\x14a\x0F/W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FSelector is not whitelisted\0\0\0\0\0`D\x82\x01R`d\x01a\x01\xD3V[a\x0F9\x89\x80a\x18.V[a\x0FG\x90` \x81\x01\x90a\x17\0V[`\x01`\x01`\xA0\x1B\x03\x16\x7Fq{\xF6\xFB5\x1C1n\xD1\x9Ed\x183rW\xBF,Zz\x12\xEC\xA7\x8EM&\xD1\r\x94\xC1\xC4\x97\xCF`@Q`@Q\x80\x91\x03\x90\xA2a\x0F\x86\x89\x80a\x18.V[a\x0F\x94\x90` \x81\x01\x90a\x17\0V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x83\x01R\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x99_\x99P\x97PPPPPPPPV[_\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[_a\x10<\x85\x87\x01\x87a\x17\0V[`@Q\x90\x91P`\x01`\x01`\xA0\x1B\x03\x82\x16\x90\x7F1\x8B\xF66\xE3\x1E\xB4\xAD\xC5\x94\xF1LC\xAC\x0BP\xBFD\x9C\x95\x9E\x83D\xAD\xC66\x80\x12\xD8\xB8<_\x90_\x90\xA2PPPPPPV[_a\x10\x84\x83a\x12\xC7V[\x80\x15a\x10\x95WPa\x10\x95\x83\x83a\x13*V[\x93\x92PPPV[_\x80a\x10\xA8\x83\x80a\x18.V[a\x10\xB9\x90`@\x81\x01\x90` \x01a\x17\0V[`\x01`\x01`\xA0\x1B\x03\x16\x7FW+l\x05\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x10\xF0` \x86\x01\x86a\x17\xF2V[a\x11\x01\x90`\xC0\x81\x01\x90`\xA0\x01a\x17\0V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`$\x82\x01R`D\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x94\x16\x93\x90\x93\x17\x90\x92R\x90Qa\x11\x82\x91\x90a\x18\xC1V[_`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80_\x81\x14a\x11\xBAW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x11\xBFV[``\x91P[P\x91P\x91P\x81a\x12\x11W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FisTrustedForwarder: reverted\0\0\0\0`D\x82\x01R`d\x01a\x01\xD3V[\x80Q` \x14a\x12bW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FisTrustedForwarder: bad response`D\x82\x01R`d\x01a\x01\xD3V[\x80\x80` \x01\x90Q\x81\x01\x90a\x12v\x91\x90a\x18\xD7V[a\x12\xC2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7Finvalid forwarder for recipient\0`D\x82\x01R`d\x01a\x01\xD3V[PPPV[_a\x12\xF2\x82\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x13*V[\x80\x15a\x06\x99WPa\x13#\x82\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x13*V[\x15\x92\x91PPV[`@\x80Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16`$\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`D\x90\x91\x01\x82R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90R\x90Q_\x91\x90\x82\x90\x81\x90`\x01`\x01`\xA0\x1B\x03\x87\x16\x90au0\x90a\x13\xD6\x90\x86\x90a\x18\xC1V[_`@Q\x80\x83\x03\x81\x86\x86\xFA\x92PPP=\x80_\x81\x14a\x14\x0FW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x14\x14V[``\x91P[P\x91P\x91P` \x81Q\x10\x15a\x14.W_\x93PPPPa\x06\x99V[\x81\x80\x15a\x14JWP\x80\x80` \x01\x90Q\x81\x01\x90a\x14J\x91\x90a\x18\xD7V[\x96\x95PPPPPPV[__\x83`\x1F\x84\x01\x12a\x14dW__\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x14{W__\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x14\x92W__\xFD[\x92P\x92\x90PV[______`\x80\x87\x89\x03\x12\x15a\x14\xAEW__\xFD[\x865g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x14\xC4W__\xFD[\x87\x01`@\x81\x8A\x03\x12\x15a\x14\xD5W__\xFD[\x95P` \x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x14\xF0W__\xFD[a\x14\xFC\x89\x82\x8A\x01a\x14TV[\x90\x96P\x94PP`@\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x15\x1BW__\xFD[a\x15'\x89\x82\x8A\x01a\x14TV[\x97\x9A\x96\x99P\x94\x97\x94\x96\x95``\x90\x95\x015\x94\x93PPPPV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[`@\x81R_a\x15\x7F`@\x83\x01\x85a\x15?V[\x90P\x82\x15\x15` \x83\x01R\x93\x92PPPV[_` \x82\x84\x03\x12\x15a\x15\xA0W__\xFD[\x815\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x81\x14a\x10\x95W__\xFD[__` \x83\x85\x03\x12\x15a\x15\xE0W__\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x15\xF6W__\xFD[a\x16\x02\x85\x82\x86\x01a\x14TV[\x90\x96\x90\x95P\x93PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0C\x10W__\xFD[__`@\x83\x85\x03\x12\x15a\x163W__\xFD[\x825\x91P` \x83\x015a\x16E\x81a\x16\x0EV[\x80\x91PP\x92P\x92\x90PV[_` \x82\x84\x03\x12\x15a\x16`W__\xFD[P5\x91\x90PV[\x80\x15\x15\x81\x14a\x0C\x10W__\xFD[_____`\x80\x86\x88\x03\x12\x15a\x16\x88W__\xFD[\x855g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x16\x9EW__\xFD[a\x16\xAA\x88\x82\x89\x01a\x14TV[\x90\x96P\x94PP` \x86\x015a\x16\xBE\x81a\x16gV[\x92P`@\x86\x015\x91P``\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x16\xE0W__\xFD[\x86\x01a\x01\0\x81\x89\x03\x12\x15a\x16\xF2W__\xFD[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[_` \x82\x84\x03\x12\x15a\x17\x10W__\xFD[\x815a\x10\x95\x81a\x16\x0EV[` \x81R_a\x10\x95` \x83\x01\x84a\x15?V[__\x85\x85\x11\x15a\x17;W__\xFD[\x83\x86\x11\x15a\x17GW__\xFD[PP\x82\x01\x93\x91\x90\x92\x03\x91PV[\x805\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x90`\x04\x84\x10\x15a\x17\xB3W\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x85`\x04\x03`\x03\x1B\x1B\x82\x16\x16\x91P[P\x92\x91PPV[\x80\x82\x01\x80\x82\x11\x15a\x06\x99W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_\x825\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x01\x836\x03\x01\x81\x12a\x18$W__\xFD[\x91\x90\x91\x01\x92\x91PPV[_\x825\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF!\x836\x03\x01\x81\x12a\x18$W__\xFD[__\x835\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x846\x03\x01\x81\x12a\x18\x93W__\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x18\xADW__\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x14\x92W__\xFD[_\x82Q\x80` \x85\x01\x84^_\x92\x01\x91\x82RP\x91\x90PV[_` \x82\x84\x03\x12\x15a\x18\xE7W__\xFD[\x81Qa\x10\x95\x81a\x16gV\xFE3.0.0-beta.10+opengsn.oracle.token.ipaymaster\xA2dipfsX\"\x12 \xFF\xF5\xF3P\x0F\x92\xF1\xDA\x9EL\xE1D@\x02!\x95\xC6;\xE4\xF0I\x044\xF0pw\xF3\xEB\xAF\xAA\xBD\x18dsolcC\0\x08\x1B\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361061017a575f3560e01c80638da5cb5b116100d1578063c3a3e2fa1161007c578063df463a6611610057578063df463a6614610525578063f2fde38b14610539578063f9c002f714610558575f5ffd5b8063c3a3e2fa146104b5578063ce1b815f146104e9578063da74222814610506575f5ffd5b8063b039a88f116100ac578063b039a88f14610443578063b90b41cf1461048a578063bbdaa3c91461049f575f5ffd5b80638da5cb5b146103f1578063921276ea1461040d578063ad12e50e1461042e575f5ffd5b80635c5e3db11161013157806376fa01c31161010c57806376fa01c3146103965780637bb05264146103b55780637bdf2ec7146103d4575f5ffd5b80635c5e3db1146103405780636d7c3e2b14610363578063715018a614610382575f5ffd5b80630cbd17c8116101615780630cbd17c8146102b657806318e45427146102ea5780632d14c4b714610321575f5ffd5b8062be5dd41461025157806301ffc9a714610287575f5ffd5b3661024d576001546001600160a01b03166101dc5760405162461bcd60e51b815260206004820152601960248201527f72656c6179206875622061646472657373206e6f74207365740000000000000060448201526064015b60405180910390fd5b6001546040517faa67c9190000000000000000000000000000000000000000000000000000000081523060048201526001600160a01b039091169063aa67c9199034906024015f604051808303818588803b158015610239575f5ffd5b505af115801561024b573d5f5f3e3d5ffd5b005b5f5ffd5b34801561025c575f5ffd5b5061027061026b366004611499565b61056e565b60405161027e92919061156d565b60405180910390f35b348015610292575f5ffd5b506102a66102a1366004611590565b6105bb565b604051901515815260200161027e565b3480156102c1575f5ffd5b506102d56102d03660046115cf565b61069f565b60405163ffffffff909116815260200161027e565b3480156102f5575f5ffd5b50600354610309906001600160a01b031681565b6040516001600160a01b03909116815260200161027e565b34801561032c575f5ffd5b5061024b61033b366004611622565b6106c2565b34801561034b575f5ffd5b5061035561290481565b60405190815260200161027e565b34801561036e575f5ffd5b5061024b61037d366004611650565b610799565b34801561038d575f5ffd5b5061024b6107f7565b3480156103a1575f5ffd5b5061024b6103b0366004611674565b61085b565b3480156103c0575f5ffd5b5061024b6103cf366004611700565b610877565b3480156103df575f5ffd5b506001546001600160a01b0316610309565b3480156103fc575f5ffd5b505f546001600160a01b0316610309565b348015610418575f5ffd5b50610421610989565b60405161027e919061171b565b348015610439575f5ffd5b5061035560045481565b34801561044e575f5ffd5b506104576109a9565b60405161027e91908151815260208083015190820152604080830151908201526060918201519181019190915260800190565b348015610495575f5ffd5b5061035561c35081565b3480156104aa575f5ffd5b506103556201adb081565b3480156104c0575f5ffd5b506003546102d59074010000000000000000000000000000000000000000900463ffffffff1681565b3480156104f4575f5ffd5b506002546001600160a01b0316610309565b348015610511575f5ffd5b5061024b610520366004611700565b610a0e565b348015610530575f5ffd5b50610355610b20565b348015610544575f5ffd5b5061024b610553366004611700565b610b32565b348015610563575f5ffd5b50610355620186a081565b60605f610579610c13565b61058288610c6d565b61058b88610d05565b61059488610d60565b61059e8585610dca565b6105ac888888888888610e1c565b91509150965096945050505050565b5f7fffffffff0000000000000000000000000000000000000000000000000000000082167fe1ab2dea00000000000000000000000000000000000000000000000000000000148061064d57507fffffffff0000000000000000000000000000000000000000000000000000000082167f0e08307600000000000000000000000000000000000000000000000000000000145b8061069957507f01ffc9a7000000000000000000000000000000000000000000000000000000007fffffffff000000000000000000000000000000000000000000000000000000008316145b92915050565b5f806106ae600482858761172d565b6106b791611754565b60e01c949350505050565b5f546001600160a01b0316331461071b5760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e657260448201526064016101d3565b6001546040517ff3fef3a30000000000000000000000000000000000000000000000000000000081526001600160a01b038381166004830152602482018590529091169063f3fef3a3906044015f604051808303815f87803b15801561077f575f5ffd5b505af1158015610791573d5f5f3e3d5ffd5b505050505050565b5f546001600160a01b031633146107f25760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e657260448201526064016101d3565b600455565b5f546001600160a01b031633146108505760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e657260448201526064016101d3565b6108595f610fc8565b565b610863610c13565b610870858585858561102f565b5050505050565b5f546001600160a01b031633146108d05760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e657260448201526064016101d3565b6109036001600160a01b0382167fe9fb30f70000000000000000000000000000000000000000000000000000000061107a565b61094f5760405162461bcd60e51b815260206004820152601f60248201527f746172676574206973206e6f7420612076616c6964204952656c61794875620060448201526064016101d3565b600180547fffffffffffffffffffffffff0000000000000000000000000000000000000000166001600160a01b0392909216919091179055565b60606040518060600160405280602d81526020016118f3602d9139905090565b6109d060405180608001604052805f81526020015f81526020015f81526020015f81525090565b604051806080016040528061c350620186a06109ec91906117ba565b8152602001620186a081526020016201adb08152602001612904815250905090565b5f546001600160a01b03163314610a675760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e657260448201526064016101d3565b610a9a6001600160a01b0382167f25e23e640000000000000000000000000000000000000000000000000000000061107a565b610ae65760405162461bcd60e51b815260206004820181905260248201527f746172676574206973206e6f7420612076616c69642049466f7277617264657260448201526064016101d3565b600280547fffffffffffffffffffffffff0000000000000000000000000000000000000000166001600160a01b0392909216919091179055565b610b2f61c350620186a06117ba565b81565b5f546001600160a01b03163314610b8b5760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e657260448201526064016101d3565b6001600160a01b038116610c075760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201527f646472657373000000000000000000000000000000000000000000000000000060648201526084016101d3565b610c1081610fc8565b50565b6001546001600160a01b031633146108595760405162461bcd60e51b815260206004820152601e60248201527f63616e206f6e6c792062652063616c6c65642062792052656c6179487562000060448201526064016101d3565b610c7a60208201826117f2565b610c8b9060c081019060a001611700565b6001600160a01b0316610ca66002546001600160a01b031690565b6001600160a01b031614610cfc5760405162461bcd60e51b815260206004820152601860248201527f466f72776172646572206973206e6f742074727573746564000000000000000060448201526064016101d3565b610c108161109c565b610d0f818061182e565b6040013515610c105760405162461bcd60e51b815260206004820152601c60248201527f76616c7565207472616e73666572206e6f7420737570706f727465640000000060448201526064016101d3565b610d6d60208201826117f2565b610d7b9060c0810190611860565b159050610c105760405162461bcd60e51b815260206004820152601c60248201527f73686f756c642068617665206e6f207061796d6173746572446174610000000060448201526064016101d3565b8015610e185760405162461bcd60e51b815260206004820152601b60248201527f73686f756c642068617665206e6f20617070726f76616c44617461000000000060448201526064016101d3565b5050565b6003546060905f906001600160a01b0316610e37898061182e565b610e48906040810190602001611700565b6001600160a01b031614610e9e5760405162461bcd60e51b815260206004820152601c60248201527f526563697069656e74206973206e6f742077686974656c69737465640000000060448201526064016101d3565b5f610eba610eac8a8061182e565b6102d09060a0810190611860565b60035490915063ffffffff808316740100000000000000000000000000000000000000009092041614610f2f5760405162461bcd60e51b815260206004820152601b60248201527f53656c6563746f72206973206e6f742077686974656c6973746564000000000060448201526064016101d3565b610f39898061182e565b610f47906020810190611700565b6001600160a01b03167f717bf6fb351c316ed19e6418337257bf2c5a7a12eca78e4d26d10d94c1c497cf60405160405180910390a2610f86898061182e565b610f94906020810190611700565b604080516001600160a01b0390921660208301520160408051601f19818403018152919052995f9950975050505050505050565b5f80546001600160a01b038381167fffffffffffffffffffffffff0000000000000000000000000000000000000000831681178455604051919092169283917f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09190a35050565b5f61103c85870187611700565b6040519091506001600160a01b038216907f318bf636e31eb4adc594f14c43ac0b50bf449c959e8344adc6368012d8b83c5f905f90a2505050505050565b5f611084836112c7565b80156110955750611095838361132a565b9392505050565b5f806110a8838061182e565b6110b9906040810190602001611700565b6001600160a01b03167f572b6c05000000000000000000000000000000000000000000000000000000006110f060208601866117f2565b6111019060c081019060a001611700565b6040516001600160a01b03909116602482015260440160408051601f198184030181529181526020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167fffffffff0000000000000000000000000000000000000000000000000000000090941693909317909252905161118291906118c1565b5f60405180830381855afa9150503d805f81146111ba576040519150601f19603f3d011682016040523d82523d5f602084013e6111bf565b606091505b5091509150816112115760405162461bcd60e51b815260206004820152601c60248201527f697354727573746564466f727761726465723a2072657665727465640000000060448201526064016101d3565b80516020146112625760405162461bcd60e51b815260206004820181905260248201527f697354727573746564466f727761726465723a2062616420726573706f6e736560448201526064016101d3565b8080602001905181019061127691906118d7565b6112c25760405162461bcd60e51b815260206004820152601f60248201527f696e76616c696420666f7277617264657220666f7220726563697069656e740060448201526064016101d3565b505050565b5f6112f2827f01ffc9a70000000000000000000000000000000000000000000000000000000061132a565b80156106995750611323827fffffffff0000000000000000000000000000000000000000000000000000000061132a565b1592915050565b604080517fffffffff00000000000000000000000000000000000000000000000000000000831660248083019190915282518083039091018152604490910182526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f01ffc9a70000000000000000000000000000000000000000000000000000000017905290515f9190829081906001600160a01b03871690617530906113d69086906118c1565b5f604051808303818686fa925050503d805f811461140f576040519150601f19603f3d011682016040523d82523d5f602084013e611414565b606091505b509150915060208151101561142e575f9350505050610699565b81801561144a57508080602001905181019061144a91906118d7565b9695505050505050565b5f5f83601f840112611464575f5ffd5b50813567ffffffffffffffff81111561147b575f5ffd5b602083019150836020828501011115611492575f5ffd5b9250929050565b5f5f5f5f5f5f608087890312156114ae575f5ffd5b863567ffffffffffffffff8111156114c4575f5ffd5b87016040818a0312156114d5575f5ffd5b9550602087013567ffffffffffffffff8111156114f0575f5ffd5b6114fc89828a01611454565b909650945050604087013567ffffffffffffffff81111561151b575f5ffd5b61152789828a01611454565b979a9699509497949695606090950135949350505050565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b604081525f61157f604083018561153f565b905082151560208301529392505050565b5f602082840312156115a0575f5ffd5b81357fffffffff0000000000000000000000000000000000000000000000000000000081168114611095575f5ffd5b5f5f602083850312156115e0575f5ffd5b823567ffffffffffffffff8111156115f6575f5ffd5b61160285828601611454565b90969095509350505050565b6001600160a01b0381168114610c10575f5ffd5b5f5f60408385031215611633575f5ffd5b8235915060208301356116458161160e565b809150509250929050565b5f60208284031215611660575f5ffd5b5035919050565b8015158114610c10575f5ffd5b5f5f5f5f5f60808688031215611688575f5ffd5b853567ffffffffffffffff81111561169e575f5ffd5b6116aa88828901611454565b90965094505060208601356116be81611667565b925060408601359150606086013567ffffffffffffffff8111156116e0575f5ffd5b860161010081890312156116f2575f5ffd5b809150509295509295909350565b5f60208284031215611710575f5ffd5b81356110958161160e565b602081525f611095602083018461153f565b5f5f8585111561173b575f5ffd5b83861115611747575f5ffd5b5050820193919092039150565b80357fffffffff0000000000000000000000000000000000000000000000000000000081169060048410156117b3577fffffffff00000000000000000000000000000000000000000000000000000000808560040360031b1b82161691505b5092915050565b80820180821115610699577f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f82357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff01833603018112611824575f5ffd5b9190910192915050565b5f82357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff21833603018112611824575f5ffd5b5f5f83357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe1843603018112611893575f5ffd5b83018035915067ffffffffffffffff8211156118ad575f5ffd5b602001915036819003821315611492575f5ffd5b5f82518060208501845e5f920191825250919050565b5f602082840312156118e7575f5ffd5b81516110958161166756fe332e302e302d626574612e31302b6f70656e67736e2e6f7261636c652e746f6b656e2e697061796d6173746572a2646970667358221220fff5f3500f92f1da9e4ce14440022195c63be4f0490434f07077f3ebafaabd1864736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10a\x01zW_5`\xE0\x1C\x80c\x8D\xA5\xCB[\x11a\0\xD1W\x80c\xC3\xA3\xE2\xFA\x11a\0|W\x80c\xDFF:f\x11a\0WW\x80c\xDFF:f\x14a\x05%W\x80c\xF2\xFD\xE3\x8B\x14a\x059W\x80c\xF9\xC0\x02\xF7\x14a\x05XW__\xFD[\x80c\xC3\xA3\xE2\xFA\x14a\x04\xB5W\x80c\xCE\x1B\x81_\x14a\x04\xE9W\x80c\xDAt\"(\x14a\x05\x06W__\xFD[\x80c\xB09\xA8\x8F\x11a\0\xACW\x80c\xB09\xA8\x8F\x14a\x04CW\x80c\xB9\x0BA\xCF\x14a\x04\x8AW\x80c\xBB\xDA\xA3\xC9\x14a\x04\x9FW__\xFD[\x80c\x8D\xA5\xCB[\x14a\x03\xF1W\x80c\x92\x12v\xEA\x14a\x04\rW\x80c\xAD\x12\xE5\x0E\x14a\x04.W__\xFD[\x80c\\^=\xB1\x11a\x011W\x80cv\xFA\x01\xC3\x11a\x01\x0CW\x80cv\xFA\x01\xC3\x14a\x03\x96W\x80c{\xB0Rd\x14a\x03\xB5W\x80c{\xDF.\xC7\x14a\x03\xD4W__\xFD[\x80c\\^=\xB1\x14a\x03@W\x80cm|>+\x14a\x03cW\x80cqP\x18\xA6\x14a\x03\x82W__\xFD[\x80c\x0C\xBD\x17\xC8\x11a\x01aW\x80c\x0C\xBD\x17\xC8\x14a\x02\xB6W\x80c\x18\xE4T'\x14a\x02\xEAW\x80c-\x14\xC4\xB7\x14a\x03!W__\xFD[\x80b\xBE]\xD4\x14a\x02QW\x80c\x01\xFF\xC9\xA7\x14a\x02\x87W__\xFD[6a\x02MW`\x01T`\x01`\x01`\xA0\x1B\x03\x16a\x01\xDCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7Frelay hub address not set\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\x01T`@Q\x7F\xAAg\xC9\x19\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xAAg\xC9\x19\x904\x90`$\x01_`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\x029W__\xFD[PZ\xF1\x15\x80\x15a\x02KW=__>=_\xFD[\0[__\xFD[4\x80\x15a\x02\\W__\xFD[Pa\x02pa\x02k6`\x04a\x14\x99V[a\x05nV[`@Qa\x02~\x92\x91\x90a\x15mV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\x92W__\xFD[Pa\x02\xA6a\x02\xA16`\x04a\x15\x90V[a\x05\xBBV[`@Q\x90\x15\x15\x81R` \x01a\x02~V[4\x80\x15a\x02\xC1W__\xFD[Pa\x02\xD5a\x02\xD06`\x04a\x15\xCFV[a\x06\x9FV[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02~V[4\x80\x15a\x02\xF5W__\xFD[P`\x03Ta\x03\t\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02~V[4\x80\x15a\x03,W__\xFD[Pa\x02Ka\x03;6`\x04a\x16\"V[a\x06\xC2V[4\x80\x15a\x03KW__\xFD[Pa\x03Ua)\x04\x81V[`@Q\x90\x81R` \x01a\x02~V[4\x80\x15a\x03nW__\xFD[Pa\x02Ka\x03}6`\x04a\x16PV[a\x07\x99V[4\x80\x15a\x03\x8DW__\xFD[Pa\x02Ka\x07\xF7V[4\x80\x15a\x03\xA1W__\xFD[Pa\x02Ka\x03\xB06`\x04a\x16tV[a\x08[V[4\x80\x15a\x03\xC0W__\xFD[Pa\x02Ka\x03\xCF6`\x04a\x17\0V[a\x08wV[4\x80\x15a\x03\xDFW__\xFD[P`\x01T`\x01`\x01`\xA0\x1B\x03\x16a\x03\tV[4\x80\x15a\x03\xFCW__\xFD[P_T`\x01`\x01`\xA0\x1B\x03\x16a\x03\tV[4\x80\x15a\x04\x18W__\xFD[Pa\x04!a\t\x89V[`@Qa\x02~\x91\x90a\x17\x1BV[4\x80\x15a\x049W__\xFD[Pa\x03U`\x04T\x81V[4\x80\x15a\x04NW__\xFD[Pa\x04Wa\t\xA9V[`@Qa\x02~\x91\x90\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x80\x83\x01Q\x90\x82\x01R``\x91\x82\x01Q\x91\x81\x01\x91\x90\x91R`\x80\x01\x90V[4\x80\x15a\x04\x95W__\xFD[Pa\x03Ua\xC3P\x81V[4\x80\x15a\x04\xAAW__\xFD[Pa\x03Ub\x01\xAD\xB0\x81V[4\x80\x15a\x04\xC0W__\xFD[P`\x03Ta\x02\xD5\x90t\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[4\x80\x15a\x04\xF4W__\xFD[P`\x02T`\x01`\x01`\xA0\x1B\x03\x16a\x03\tV[4\x80\x15a\x05\x11W__\xFD[Pa\x02Ka\x05 6`\x04a\x17\0V[a\n\x0EV[4\x80\x15a\x050W__\xFD[Pa\x03Ua\x0B V[4\x80\x15a\x05DW__\xFD[Pa\x02Ka\x05S6`\x04a\x17\0V[a\x0B2V[4\x80\x15a\x05cW__\xFD[Pa\x03Ub\x01\x86\xA0\x81V[``_a\x05ya\x0C\x13V[a\x05\x82\x88a\x0CmV[a\x05\x8B\x88a\r\x05V[a\x05\x94\x88a\r`V[a\x05\x9E\x85\x85a\r\xCAV[a\x05\xAC\x88\x88\x88\x88\x88\x88a\x0E\x1CV[\x91P\x91P\x96P\x96\x94PPPPPV[_\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x7F\xE1\xAB-\xEA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x14\x80a\x06MWP\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x7F\x0E\x080v\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x14[\x80a\x06\x99WP\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x14[\x92\x91PPV[_\x80a\x06\xAE`\x04\x82\x85\x87a\x17-V[a\x06\xB7\x91a\x17TV[`\xE0\x1C\x94\x93PPPPV[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x07\x1BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x01\xD3V[`\x01T`@Q\x7F\xF3\xFE\xF3\xA3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R`$\x82\x01\x85\x90R\x90\x91\x16\x90c\xF3\xFE\xF3\xA3\x90`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x07\x7FW__\xFD[PZ\xF1\x15\x80\x15a\x07\x91W=__>=_\xFD[PPPPPPV[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x07\xF2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x01\xD3V[`\x04UV[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x08PW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x01\xD3V[a\x08Y_a\x0F\xC8V[V[a\x08ca\x0C\x13V[a\x08p\x85\x85\x85\x85\x85a\x10/V[PPPPPV[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x08\xD0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x01\xD3V[a\t\x03`\x01`\x01`\xA0\x1B\x03\x82\x16\x7F\xE9\xFB0\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x10zV[a\tOW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7Ftarget is not a valid IRelayHub\0`D\x82\x01R`d\x01a\x01\xD3V[`\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[```@Q\x80``\x01`@R\x80`-\x81R` \x01a\x18\xF3`-\x919\x90P\x90V[a\t\xD0`@Q\x80`\x80\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80`\x80\x01`@R\x80a\xC3Pb\x01\x86\xA0a\t\xEC\x91\x90a\x17\xBAV[\x81R` \x01b\x01\x86\xA0\x81R` \x01b\x01\xAD\xB0\x81R` \x01a)\x04\x81RP\x90P\x90V[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\ngW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x01\xD3V[a\n\x9A`\x01`\x01`\xA0\x1B\x03\x82\x16\x7F%\xE2>d\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x10zV[a\n\xE6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7Ftarget is not a valid IForwarder`D\x82\x01R`d\x01a\x01\xD3V[`\x02\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[a\x0B/a\xC3Pb\x01\x86\xA0a\x17\xBAV[\x81V[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0B\x8BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x01\xD3V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x0C\x07W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x01\xD3V[a\x0C\x10\x81a\x0F\xC8V[PV[`\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x08YW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7Fcan only be called by RelayHub\0\0`D\x82\x01R`d\x01a\x01\xD3V[a\x0Cz` \x82\x01\x82a\x17\xF2V[a\x0C\x8B\x90`\xC0\x81\x01\x90`\xA0\x01a\x17\0V[`\x01`\x01`\xA0\x1B\x03\x16a\x0C\xA6`\x02T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0C\xFCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FForwarder is not trusted\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x01\xD3V[a\x0C\x10\x81a\x10\x9CV[a\r\x0F\x81\x80a\x18.V[`@\x015\x15a\x0C\x10W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7Fvalue transfer not supported\0\0\0\0`D\x82\x01R`d\x01a\x01\xD3V[a\rm` \x82\x01\x82a\x17\xF2V[a\r{\x90`\xC0\x81\x01\x90a\x18`V[\x15\x90Pa\x0C\x10W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7Fshould have no paymasterData\0\0\0\0`D\x82\x01R`d\x01a\x01\xD3V[\x80\x15a\x0E\x18W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7Fshould have no approvalData\0\0\0\0\0`D\x82\x01R`d\x01a\x01\xD3V[PPV[`\x03T``\x90_\x90`\x01`\x01`\xA0\x1B\x03\x16a\x0E7\x89\x80a\x18.V[a\x0EH\x90`@\x81\x01\x90` \x01a\x17\0V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0E\x9EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FRecipient is not whitelisted\0\0\0\0`D\x82\x01R`d\x01a\x01\xD3V[_a\x0E\xBAa\x0E\xAC\x8A\x80a\x18.V[a\x02\xD0\x90`\xA0\x81\x01\x90a\x18`V[`\x03T\x90\x91Pc\xFF\xFF\xFF\xFF\x80\x83\x16t\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x92\x04\x16\x14a\x0F/W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FSelector is not whitelisted\0\0\0\0\0`D\x82\x01R`d\x01a\x01\xD3V[a\x0F9\x89\x80a\x18.V[a\x0FG\x90` \x81\x01\x90a\x17\0V[`\x01`\x01`\xA0\x1B\x03\x16\x7Fq{\xF6\xFB5\x1C1n\xD1\x9Ed\x183rW\xBF,Zz\x12\xEC\xA7\x8EM&\xD1\r\x94\xC1\xC4\x97\xCF`@Q`@Q\x80\x91\x03\x90\xA2a\x0F\x86\x89\x80a\x18.V[a\x0F\x94\x90` \x81\x01\x90a\x17\0V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x83\x01R\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x99_\x99P\x97PPPPPPPPV[_\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[_a\x10<\x85\x87\x01\x87a\x17\0V[`@Q\x90\x91P`\x01`\x01`\xA0\x1B\x03\x82\x16\x90\x7F1\x8B\xF66\xE3\x1E\xB4\xAD\xC5\x94\xF1LC\xAC\x0BP\xBFD\x9C\x95\x9E\x83D\xAD\xC66\x80\x12\xD8\xB8<_\x90_\x90\xA2PPPPPPV[_a\x10\x84\x83a\x12\xC7V[\x80\x15a\x10\x95WPa\x10\x95\x83\x83a\x13*V[\x93\x92PPPV[_\x80a\x10\xA8\x83\x80a\x18.V[a\x10\xB9\x90`@\x81\x01\x90` \x01a\x17\0V[`\x01`\x01`\xA0\x1B\x03\x16\x7FW+l\x05\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x10\xF0` \x86\x01\x86a\x17\xF2V[a\x11\x01\x90`\xC0\x81\x01\x90`\xA0\x01a\x17\0V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`$\x82\x01R`D\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x94\x16\x93\x90\x93\x17\x90\x92R\x90Qa\x11\x82\x91\x90a\x18\xC1V[_`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80_\x81\x14a\x11\xBAW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x11\xBFV[``\x91P[P\x91P\x91P\x81a\x12\x11W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FisTrustedForwarder: reverted\0\0\0\0`D\x82\x01R`d\x01a\x01\xD3V[\x80Q` \x14a\x12bW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FisTrustedForwarder: bad response`D\x82\x01R`d\x01a\x01\xD3V[\x80\x80` \x01\x90Q\x81\x01\x90a\x12v\x91\x90a\x18\xD7V[a\x12\xC2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7Finvalid forwarder for recipient\0`D\x82\x01R`d\x01a\x01\xD3V[PPPV[_a\x12\xF2\x82\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x13*V[\x80\x15a\x06\x99WPa\x13#\x82\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x13*V[\x15\x92\x91PPV[`@\x80Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16`$\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`D\x90\x91\x01\x82R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90R\x90Q_\x91\x90\x82\x90\x81\x90`\x01`\x01`\xA0\x1B\x03\x87\x16\x90au0\x90a\x13\xD6\x90\x86\x90a\x18\xC1V[_`@Q\x80\x83\x03\x81\x86\x86\xFA\x92PPP=\x80_\x81\x14a\x14\x0FW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x14\x14V[``\x91P[P\x91P\x91P` \x81Q\x10\x15a\x14.W_\x93PPPPa\x06\x99V[\x81\x80\x15a\x14JWP\x80\x80` \x01\x90Q\x81\x01\x90a\x14J\x91\x90a\x18\xD7V[\x96\x95PPPPPPV[__\x83`\x1F\x84\x01\x12a\x14dW__\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x14{W__\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x14\x92W__\xFD[\x92P\x92\x90PV[______`\x80\x87\x89\x03\x12\x15a\x14\xAEW__\xFD[\x865g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x14\xC4W__\xFD[\x87\x01`@\x81\x8A\x03\x12\x15a\x14\xD5W__\xFD[\x95P` \x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x14\xF0W__\xFD[a\x14\xFC\x89\x82\x8A\x01a\x14TV[\x90\x96P\x94PP`@\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x15\x1BW__\xFD[a\x15'\x89\x82\x8A\x01a\x14TV[\x97\x9A\x96\x99P\x94\x97\x94\x96\x95``\x90\x95\x015\x94\x93PPPPV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[`@\x81R_a\x15\x7F`@\x83\x01\x85a\x15?V[\x90P\x82\x15\x15` \x83\x01R\x93\x92PPPV[_` \x82\x84\x03\x12\x15a\x15\xA0W__\xFD[\x815\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x81\x14a\x10\x95W__\xFD[__` \x83\x85\x03\x12\x15a\x15\xE0W__\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x15\xF6W__\xFD[a\x16\x02\x85\x82\x86\x01a\x14TV[\x90\x96\x90\x95P\x93PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0C\x10W__\xFD[__`@\x83\x85\x03\x12\x15a\x163W__\xFD[\x825\x91P` \x83\x015a\x16E\x81a\x16\x0EV[\x80\x91PP\x92P\x92\x90PV[_` \x82\x84\x03\x12\x15a\x16`W__\xFD[P5\x91\x90PV[\x80\x15\x15\x81\x14a\x0C\x10W__\xFD[_____`\x80\x86\x88\x03\x12\x15a\x16\x88W__\xFD[\x855g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x16\x9EW__\xFD[a\x16\xAA\x88\x82\x89\x01a\x14TV[\x90\x96P\x94PP` \x86\x015a\x16\xBE\x81a\x16gV[\x92P`@\x86\x015\x91P``\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x16\xE0W__\xFD[\x86\x01a\x01\0\x81\x89\x03\x12\x15a\x16\xF2W__\xFD[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[_` \x82\x84\x03\x12\x15a\x17\x10W__\xFD[\x815a\x10\x95\x81a\x16\x0EV[` \x81R_a\x10\x95` \x83\x01\x84a\x15?V[__\x85\x85\x11\x15a\x17;W__\xFD[\x83\x86\x11\x15a\x17GW__\xFD[PP\x82\x01\x93\x91\x90\x92\x03\x91PV[\x805\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x90`\x04\x84\x10\x15a\x17\xB3W\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x85`\x04\x03`\x03\x1B\x1B\x82\x16\x16\x91P[P\x92\x91PPV[\x80\x82\x01\x80\x82\x11\x15a\x06\x99W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_\x825\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x01\x836\x03\x01\x81\x12a\x18$W__\xFD[\x91\x90\x91\x01\x92\x91PPV[_\x825\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF!\x836\x03\x01\x81\x12a\x18$W__\xFD[__\x835\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x846\x03\x01\x81\x12a\x18\x93W__\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x18\xADW__\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x14\x92W__\xFD[_\x82Q\x80` \x85\x01\x84^_\x92\x01\x91\x82RP\x91\x90PV[_` \x82\x84\x03\x12\x15a\x18\xE7W__\xFD[\x81Qa\x10\x95\x81a\x16gV\xFE3.0.0-beta.10+opengsn.oracle.token.ipaymaster\xA2dipfsX\"\x12 \xFF\xF5\xF3P\x0F\x92\xF1\xDA\x9EL\xE1D@\x02!\x95\xC6;\xE4\xF0I\x044\xF0pw\xF3\xEB\xAF\xAA\xBD\x18dsolcC\0\x08\x1B\x003",
    );
    /**Event with signature `OwnershipTransferred(address,address)` and selector `0x8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0`.
```solidity
event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OwnershipTransferred {
        #[allow(missing_docs)]
        pub previousOwner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newOwner: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for OwnershipTransferred {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OwnershipTransferred(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                139u8,
                224u8,
                7u8,
                156u8,
                83u8,
                22u8,
                89u8,
                20u8,
                19u8,
                68u8,
                205u8,
                31u8,
                208u8,
                164u8,
                242u8,
                132u8,
                25u8,
                73u8,
                127u8,
                151u8,
                34u8,
                163u8,
                218u8,
                175u8,
                227u8,
                180u8,
                24u8,
                111u8,
                107u8,
                100u8,
                87u8,
                224u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    previousOwner: topics.1,
                    newOwner: topics.2,
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
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.previousOwner.clone(),
                    self.newOwner.clone(),
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
                    &self.previousOwner,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.newOwner,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OwnershipTransferred {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OwnershipTransferred> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OwnershipTransferred) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `PostRelay(address)` and selector `0x318bf636e31eb4adc594f14c43ac0b50bf449c959e8344adc6368012d8b83c5f`.
```solidity
event PostRelay(address indexed sender);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct PostRelay {
        #[allow(missing_docs)]
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for PostRelay {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "PostRelay(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                49u8,
                139u8,
                246u8,
                54u8,
                227u8,
                30u8,
                180u8,
                173u8,
                197u8,
                148u8,
                241u8,
                76u8,
                67u8,
                172u8,
                11u8,
                80u8,
                191u8,
                68u8,
                156u8,
                149u8,
                158u8,
                131u8,
                68u8,
                173u8,
                198u8,
                54u8,
                128u8,
                18u8,
                216u8,
                184u8,
                60u8,
                95u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { sender: topics.1 }
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
                (Self::SIGNATURE_HASH.into(), self.sender.clone())
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
                    &self.sender,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for PostRelay {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&PostRelay> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &PostRelay) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `PreRelay(address)` and selector `0x717bf6fb351c316ed19e6418337257bf2c5a7a12eca78e4d26d10d94c1c497cf`.
```solidity
event PreRelay(address indexed sender);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct PreRelay {
        #[allow(missing_docs)]
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for PreRelay {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "PreRelay(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                113u8,
                123u8,
                246u8,
                251u8,
                53u8,
                28u8,
                49u8,
                110u8,
                209u8,
                158u8,
                100u8,
                24u8,
                51u8,
                114u8,
                87u8,
                191u8,
                44u8,
                90u8,
                122u8,
                18u8,
                236u8,
                167u8,
                142u8,
                77u8,
                38u8,
                209u8,
                13u8,
                148u8,
                193u8,
                196u8,
                151u8,
                207u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { sender: topics.1 }
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
                (Self::SIGNATURE_HASH.into(), self.sender.clone())
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
                    &self.sender,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for PreRelay {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&PreRelay> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &PreRelay) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Constructor`.
```solidity
constructor(address _whitelistedContract, uint32 _whitelistedSelector);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        pub _whitelistedContract: alloy::sol_types::private::Address,
        pub _whitelistedSelector: u32,
    }
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address, u32);
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
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    (value._whitelistedContract, value._whitelistedSelector)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _whitelistedContract: tuple.0,
                        _whitelistedSelector: tuple.1,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                        &self._whitelistedContract,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self._whitelistedSelector),
                )
            }
        }
    };
    /**Function with signature `CALLDATA_SIZE_LIMIT()` and selector `0x5c5e3db1`.
```solidity
function CALLDATA_SIZE_LIMIT() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CALLDATA_SIZE_LIMITCall {}
    ///Container type for the return parameters of the [`CALLDATA_SIZE_LIMIT()`](CALLDATA_SIZE_LIMITCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CALLDATA_SIZE_LIMITReturn {
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
            impl ::core::convert::From<CALLDATA_SIZE_LIMITCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: CALLDATA_SIZE_LIMITCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for CALLDATA_SIZE_LIMITCall {
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
            impl ::core::convert::From<CALLDATA_SIZE_LIMITReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: CALLDATA_SIZE_LIMITReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for CALLDATA_SIZE_LIMITReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for CALLDATA_SIZE_LIMITCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = CALLDATA_SIZE_LIMITReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CALLDATA_SIZE_LIMIT()";
            const SELECTOR: [u8; 4] = [92u8, 94u8, 61u8, 177u8];
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
    /**Function with signature `FORWARDER_HUB_OVERHEAD()` and selector `0xb90b41cf`.
```solidity
function FORWARDER_HUB_OVERHEAD() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct FORWARDER_HUB_OVERHEADCall {}
    ///Container type for the return parameters of the [`FORWARDER_HUB_OVERHEAD()`](FORWARDER_HUB_OVERHEADCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct FORWARDER_HUB_OVERHEADReturn {
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
            impl ::core::convert::From<FORWARDER_HUB_OVERHEADCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: FORWARDER_HUB_OVERHEADCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for FORWARDER_HUB_OVERHEADCall {
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
            impl ::core::convert::From<FORWARDER_HUB_OVERHEADReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: FORWARDER_HUB_OVERHEADReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for FORWARDER_HUB_OVERHEADReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for FORWARDER_HUB_OVERHEADCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = FORWARDER_HUB_OVERHEADReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "FORWARDER_HUB_OVERHEAD()";
            const SELECTOR: [u8; 4] = [185u8, 11u8, 65u8, 207u8];
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
    /**Function with signature `PAYMASTER_ACCEPTANCE_BUDGET()` and selector `0xdf463a66`.
```solidity
function PAYMASTER_ACCEPTANCE_BUDGET() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct PAYMASTER_ACCEPTANCE_BUDGETCall {}
    ///Container type for the return parameters of the [`PAYMASTER_ACCEPTANCE_BUDGET()`](PAYMASTER_ACCEPTANCE_BUDGETCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct PAYMASTER_ACCEPTANCE_BUDGETReturn {
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
            impl ::core::convert::From<PAYMASTER_ACCEPTANCE_BUDGETCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: PAYMASTER_ACCEPTANCE_BUDGETCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for PAYMASTER_ACCEPTANCE_BUDGETCall {
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
            impl ::core::convert::From<PAYMASTER_ACCEPTANCE_BUDGETReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: PAYMASTER_ACCEPTANCE_BUDGETReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for PAYMASTER_ACCEPTANCE_BUDGETReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for PAYMASTER_ACCEPTANCE_BUDGETCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = PAYMASTER_ACCEPTANCE_BUDGETReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "PAYMASTER_ACCEPTANCE_BUDGET()";
            const SELECTOR: [u8; 4] = [223u8, 70u8, 58u8, 102u8];
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
    /**Function with signature `POST_RELAYED_CALL_GAS_LIMIT()` and selector `0xbbdaa3c9`.
```solidity
function POST_RELAYED_CALL_GAS_LIMIT() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct POST_RELAYED_CALL_GAS_LIMITCall {}
    ///Container type for the return parameters of the [`POST_RELAYED_CALL_GAS_LIMIT()`](POST_RELAYED_CALL_GAS_LIMITCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct POST_RELAYED_CALL_GAS_LIMITReturn {
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
            impl ::core::convert::From<POST_RELAYED_CALL_GAS_LIMITCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: POST_RELAYED_CALL_GAS_LIMITCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for POST_RELAYED_CALL_GAS_LIMITCall {
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
            impl ::core::convert::From<POST_RELAYED_CALL_GAS_LIMITReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: POST_RELAYED_CALL_GAS_LIMITReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for POST_RELAYED_CALL_GAS_LIMITReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for POST_RELAYED_CALL_GAS_LIMITCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = POST_RELAYED_CALL_GAS_LIMITReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "POST_RELAYED_CALL_GAS_LIMIT()";
            const SELECTOR: [u8; 4] = [187u8, 218u8, 163u8, 201u8];
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
    /**Function with signature `PRE_RELAYED_CALL_GAS_LIMIT()` and selector `0xf9c002f7`.
```solidity
function PRE_RELAYED_CALL_GAS_LIMIT() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct PRE_RELAYED_CALL_GAS_LIMITCall {}
    ///Container type for the return parameters of the [`PRE_RELAYED_CALL_GAS_LIMIT()`](PRE_RELAYED_CALL_GAS_LIMITCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct PRE_RELAYED_CALL_GAS_LIMITReturn {
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
            impl ::core::convert::From<PRE_RELAYED_CALL_GAS_LIMITCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: PRE_RELAYED_CALL_GAS_LIMITCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for PRE_RELAYED_CALL_GAS_LIMITCall {
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
            impl ::core::convert::From<PRE_RELAYED_CALL_GAS_LIMITReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: PRE_RELAYED_CALL_GAS_LIMITReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for PRE_RELAYED_CALL_GAS_LIMITReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for PRE_RELAYED_CALL_GAS_LIMITCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = PRE_RELAYED_CALL_GAS_LIMITReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "PRE_RELAYED_CALL_GAS_LIMIT()";
            const SELECTOR: [u8; 4] = [249u8, 192u8, 2u8, 247u8];
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
    /**Function with signature `gasUsedByPost()` and selector `0xad12e50e`.
```solidity
function gasUsedByPost() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct gasUsedByPostCall {}
    ///Container type for the return parameters of the [`gasUsedByPost()`](gasUsedByPostCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct gasUsedByPostReturn {
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
            impl ::core::convert::From<gasUsedByPostCall> for UnderlyingRustTuple<'_> {
                fn from(value: gasUsedByPostCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for gasUsedByPostCall {
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
            impl ::core::convert::From<gasUsedByPostReturn> for UnderlyingRustTuple<'_> {
                fn from(value: gasUsedByPostReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for gasUsedByPostReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for gasUsedByPostCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = gasUsedByPostReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "gasUsedByPost()";
            const SELECTOR: [u8; 4] = [173u8, 18u8, 229u8, 14u8];
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
    /**Function with signature `getGasAndDataLimits()` and selector `0xb039a88f`.
```solidity
function getGasAndDataLimits() external view returns (IPaymaster.GasAndDataLimits memory limits);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getGasAndDataLimitsCall {}
    ///Container type for the return parameters of the [`getGasAndDataLimits()`](getGasAndDataLimitsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getGasAndDataLimitsReturn {
        pub limits: <IPaymaster::GasAndDataLimits as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getGasAndDataLimitsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getGasAndDataLimitsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getGasAndDataLimitsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (IPaymaster::GasAndDataLimits,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IPaymaster::GasAndDataLimits as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getGasAndDataLimitsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getGasAndDataLimitsReturn) -> Self {
                    (value.limits,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getGasAndDataLimitsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { limits: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getGasAndDataLimitsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getGasAndDataLimitsReturn;
            type ReturnTuple<'a> = (IPaymaster::GasAndDataLimits,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getGasAndDataLimits()";
            const SELECTOR: [u8; 4] = [176u8, 57u8, 168u8, 143u8];
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
    /**Function with signature `getRelayHub()` and selector `0x7bdf2ec7`.
```solidity
function getRelayHub() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRelayHubCall {}
    ///Container type for the return parameters of the [`getRelayHub()`](getRelayHubCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRelayHubReturn {
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
            impl ::core::convert::From<getRelayHubCall> for UnderlyingRustTuple<'_> {
                fn from(value: getRelayHubCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getRelayHubCall {
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
            impl ::core::convert::From<getRelayHubReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getRelayHubReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getRelayHubReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getRelayHubCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getRelayHubReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getRelayHub()";
            const SELECTOR: [u8; 4] = [123u8, 223u8, 46u8, 199u8];
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
    /**Function with signature `getSelector(bytes)` and selector `0x0cbd17c8`.
```solidity
function getSelector(bytes memory call) external pure returns (uint32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getSelectorCall {
        pub call: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`getSelector(bytes)`](getSelectorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getSelectorReturn {
        pub _0: u32,
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
            impl ::core::convert::From<getSelectorCall> for UnderlyingRustTuple<'_> {
                fn from(value: getSelectorCall) -> Self {
                    (value.call,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getSelectorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { call: tuple.0 }
                }
            }
        }
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
            impl ::core::convert::From<getSelectorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getSelectorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getSelectorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getSelectorCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Bytes,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getSelectorReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getSelector(bytes)";
            const SELECTOR: [u8; 4] = [12u8, 189u8, 23u8, 200u8];
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
                        &self.call,
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
    /**Function with signature `getTrustedForwarder()` and selector `0xce1b815f`.
```solidity
function getTrustedForwarder() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTrustedForwarderCall {}
    ///Container type for the return parameters of the [`getTrustedForwarder()`](getTrustedForwarderCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTrustedForwarderReturn {
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
            impl ::core::convert::From<getTrustedForwarderCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getTrustedForwarderCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getTrustedForwarderCall {
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
            impl ::core::convert::From<getTrustedForwarderReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getTrustedForwarderReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getTrustedForwarderReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getTrustedForwarderCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getTrustedForwarderReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getTrustedForwarder()";
            const SELECTOR: [u8; 4] = [206u8, 27u8, 129u8, 95u8];
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
    /**Function with signature `owner()` and selector `0x8da5cb5b`.
```solidity
function owner() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ownerCall {}
    ///Container type for the return parameters of the [`owner()`](ownerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ownerReturn {
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
            impl ::core::convert::From<ownerCall> for UnderlyingRustTuple<'_> {
                fn from(value: ownerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ownerCall {
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
            impl ::core::convert::From<ownerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: ownerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ownerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for ownerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = ownerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "owner()";
            const SELECTOR: [u8; 4] = [141u8, 165u8, 203u8, 91u8];
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
    /**Function with signature `postRelayedCall(bytes,bool,uint256,(uint256,uint256,uint256,address,address,address,bytes,uint256))` and selector `0x76fa01c3`.
```solidity
function postRelayedCall(bytes memory context, bool success, uint256 gasUseWithoutPost, GsnTypes.RelayData memory relayData) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct postRelayedCallCall {
        pub context: alloy::sol_types::private::Bytes,
        pub success: bool,
        pub gasUseWithoutPost: alloy::sol_types::private::primitives::aliases::U256,
        pub relayData: <GsnTypes::RelayData as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`postRelayedCall(bytes,bool,uint256,(uint256,uint256,uint256,address,address,address,bytes,uint256))`](postRelayedCallCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct postRelayedCallReturn {}
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
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Uint<256>,
                GsnTypes::RelayData,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Bytes,
                bool,
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
            impl ::core::convert::From<postRelayedCallCall> for UnderlyingRustTuple<'_> {
                fn from(value: postRelayedCallCall) -> Self {
                    (
                        value.context,
                        value.success,
                        value.gasUseWithoutPost,
                        value.relayData,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for postRelayedCallCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        context: tuple.0,
                        success: tuple.1,
                        gasUseWithoutPost: tuple.2,
                        relayData: tuple.3,
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
            impl ::core::convert::From<postRelayedCallReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: postRelayedCallReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for postRelayedCallReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for postRelayedCallCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Uint<256>,
                GsnTypes::RelayData,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = postRelayedCallReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "postRelayedCall(bytes,bool,uint256,(uint256,uint256,uint256,address,address,address,bytes,uint256))";
            const SELECTOR: [u8; 4] = [118u8, 250u8, 1u8, 195u8];
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
                        &self.context,
                    ),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.success,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.gasUseWithoutPost),
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
    /**Function with signature `preRelayedCall(((address,address,uint256,uint256,uint256,bytes,uint256),(uint256,uint256,uint256,address,address,address,bytes,uint256)),bytes,bytes,uint256)` and selector `0x00be5dd4`.
```solidity
function preRelayedCall(GsnTypes.RelayRequest memory relayRequest, bytes memory signature, bytes memory approvalData, uint256 maxPossibleGas) external returns (bytes memory, bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct preRelayedCallCall {
        pub relayRequest: <GsnTypes::RelayRequest as alloy::sol_types::SolType>::RustType,
        pub signature: alloy::sol_types::private::Bytes,
        pub approvalData: alloy::sol_types::private::Bytes,
        pub maxPossibleGas: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`preRelayedCall(((address,address,uint256,uint256,uint256,bytes,uint256),(uint256,uint256,uint256,address,address,address,bytes,uint256)),bytes,bytes,uint256)`](preRelayedCallCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct preRelayedCallReturn {
        pub _0: alloy::sol_types::private::Bytes,
        pub _1: bool,
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
                GsnTypes::RelayRequest,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <GsnTypes::RelayRequest as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::Bytes,
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
            impl ::core::convert::From<preRelayedCallCall> for UnderlyingRustTuple<'_> {
                fn from(value: preRelayedCallCall) -> Self {
                    (
                        value.relayRequest,
                        value.signature,
                        value.approvalData,
                        value.maxPossibleGas,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for preRelayedCallCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        relayRequest: tuple.0,
                        signature: tuple.1,
                        approvalData: tuple.2,
                        maxPossibleGas: tuple.3,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bool,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Bytes, bool);
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
            impl ::core::convert::From<preRelayedCallReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: preRelayedCallReturn) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for preRelayedCallReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0, _1: tuple.1 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for preRelayedCallCall {
            type Parameters<'a> = (
                GsnTypes::RelayRequest,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = preRelayedCallReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bool,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "preRelayedCall(((address,address,uint256,uint256,uint256,bytes,uint256),(uint256,uint256,uint256,address,address,address,bytes,uint256)),bytes,bytes,uint256)";
            const SELECTOR: [u8; 4] = [0u8, 190u8, 93u8, 212u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <GsnTypes::RelayRequest as alloy_sol_types::SolType>::tokenize(
                        &self.relayRequest,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.signature,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.approvalData,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.maxPossibleGas),
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
    /**Function with signature `renounceOwnership()` and selector `0x715018a6`.
```solidity
function renounceOwnership() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct renounceOwnershipCall {}
    ///Container type for the return parameters of the [`renounceOwnership()`](renounceOwnershipCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct renounceOwnershipReturn {}
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
            impl ::core::convert::From<renounceOwnershipCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: renounceOwnershipCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for renounceOwnershipCall {
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
            impl ::core::convert::From<renounceOwnershipReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: renounceOwnershipReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for renounceOwnershipReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for renounceOwnershipCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = renounceOwnershipReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "renounceOwnership()";
            const SELECTOR: [u8; 4] = [113u8, 80u8, 24u8, 166u8];
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
    /**Function with signature `setPostGasUsage(uint256)` and selector `0x6d7c3e2b`.
```solidity
function setPostGasUsage(uint256 _gasUsedByPost) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setPostGasUsageCall {
        pub _gasUsedByPost: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`setPostGasUsage(uint256)`](setPostGasUsageCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setPostGasUsageReturn {}
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
            impl ::core::convert::From<setPostGasUsageCall> for UnderlyingRustTuple<'_> {
                fn from(value: setPostGasUsageCall) -> Self {
                    (value._gasUsedByPost,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setPostGasUsageCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _gasUsedByPost: tuple.0 }
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
            impl ::core::convert::From<setPostGasUsageReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setPostGasUsageReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setPostGasUsageReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setPostGasUsageCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setPostGasUsageReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setPostGasUsage(uint256)";
            const SELECTOR: [u8; 4] = [109u8, 124u8, 62u8, 43u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self._gasUsedByPost),
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
    /**Function with signature `setRelayHub(address)` and selector `0x7bb05264`.
```solidity
function setRelayHub(address hub) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setRelayHubCall {
        pub hub: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`setRelayHub(address)`](setRelayHubCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setRelayHubReturn {}
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
            impl ::core::convert::From<setRelayHubCall> for UnderlyingRustTuple<'_> {
                fn from(value: setRelayHubCall) -> Self {
                    (value.hub,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setRelayHubCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { hub: tuple.0 }
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
            impl ::core::convert::From<setRelayHubReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setRelayHubReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setRelayHubReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setRelayHubCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setRelayHubReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setRelayHub(address)";
            const SELECTOR: [u8; 4] = [123u8, 176u8, 82u8, 100u8];
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
                        &self.hub,
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
    /**Function with signature `setTrustedForwarder(address)` and selector `0xda742228`.
```solidity
function setTrustedForwarder(address forwarder) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setTrustedForwarderCall {
        pub forwarder: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`setTrustedForwarder(address)`](setTrustedForwarderCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setTrustedForwarderReturn {}
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
            impl ::core::convert::From<setTrustedForwarderCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: setTrustedForwarderCall) -> Self {
                    (value.forwarder,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setTrustedForwarderCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { forwarder: tuple.0 }
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
            impl ::core::convert::From<setTrustedForwarderReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setTrustedForwarderReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setTrustedForwarderReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setTrustedForwarderCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setTrustedForwarderReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setTrustedForwarder(address)";
            const SELECTOR: [u8; 4] = [218u8, 116u8, 34u8, 40u8];
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
                        &self.forwarder,
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
    /**Function with signature `transferOwnership(address)` and selector `0xf2fde38b`.
```solidity
function transferOwnership(address newOwner) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transferOwnershipCall {
        pub newOwner: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`transferOwnership(address)`](transferOwnershipCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transferOwnershipReturn {}
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
            impl ::core::convert::From<transferOwnershipCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: transferOwnershipCall) -> Self {
                    (value.newOwner,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for transferOwnershipCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { newOwner: tuple.0 }
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
            impl ::core::convert::From<transferOwnershipReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: transferOwnershipReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for transferOwnershipReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for transferOwnershipCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = transferOwnershipReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "transferOwnership(address)";
            const SELECTOR: [u8; 4] = [242u8, 253u8, 227u8, 139u8];
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
                        &self.newOwner,
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
    /**Function with signature `versionPaymaster()` and selector `0x921276ea`.
```solidity
function versionPaymaster() external view returns (string memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct versionPaymasterCall {}
    ///Container type for the return parameters of the [`versionPaymaster()`](versionPaymasterCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct versionPaymasterReturn {
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
            impl ::core::convert::From<versionPaymasterCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: versionPaymasterCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for versionPaymasterCall {
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
            impl ::core::convert::From<versionPaymasterReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: versionPaymasterReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for versionPaymasterReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for versionPaymasterCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = versionPaymasterReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::String,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "versionPaymaster()";
            const SELECTOR: [u8; 4] = [146u8, 18u8, 118u8, 234u8];
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
    /**Function with signature `whitelistedContract()` and selector `0x18e45427`.
```solidity
function whitelistedContract() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct whitelistedContractCall {}
    ///Container type for the return parameters of the [`whitelistedContract()`](whitelistedContractCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct whitelistedContractReturn {
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
            impl ::core::convert::From<whitelistedContractCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: whitelistedContractCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for whitelistedContractCall {
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
            impl ::core::convert::From<whitelistedContractReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: whitelistedContractReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for whitelistedContractReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for whitelistedContractCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = whitelistedContractReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "whitelistedContract()";
            const SELECTOR: [u8; 4] = [24u8, 228u8, 84u8, 39u8];
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
    /**Function with signature `whitelistedSelector()` and selector `0xc3a3e2fa`.
```solidity
function whitelistedSelector() external view returns (uint32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct whitelistedSelectorCall {}
    ///Container type for the return parameters of the [`whitelistedSelector()`](whitelistedSelectorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct whitelistedSelectorReturn {
        pub _0: u32,
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
            impl ::core::convert::From<whitelistedSelectorCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: whitelistedSelectorCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for whitelistedSelectorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
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
            impl ::core::convert::From<whitelistedSelectorReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: whitelistedSelectorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for whitelistedSelectorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for whitelistedSelectorCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = whitelistedSelectorReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "whitelistedSelector()";
            const SELECTOR: [u8; 4] = [195u8, 163u8, 226u8, 250u8];
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
    /**Function with signature `withdrawRelayHubDepositTo(uint256,address)` and selector `0x2d14c4b7`.
```solidity
function withdrawRelayHubDepositTo(uint256 amount, address target) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct withdrawRelayHubDepositToCall {
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
        pub target: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`withdrawRelayHubDepositTo(uint256,address)`](withdrawRelayHubDepositToCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct withdrawRelayHubDepositToReturn {}
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
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<withdrawRelayHubDepositToCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: withdrawRelayHubDepositToCall) -> Self {
                    (value.amount, value.target)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for withdrawRelayHubDepositToCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        amount: tuple.0,
                        target: tuple.1,
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
            impl ::core::convert::From<withdrawRelayHubDepositToReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: withdrawRelayHubDepositToReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for withdrawRelayHubDepositToReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for withdrawRelayHubDepositToCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = withdrawRelayHubDepositToReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "withdrawRelayHubDepositTo(uint256,address)";
            const SELECTOR: [u8; 4] = [45u8, 20u8, 196u8, 183u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
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
    ///Container for all the [`OnboardingPaymaster`](self) function calls.
    pub enum OnboardingPaymasterCalls {
        CALLDATA_SIZE_LIMIT(CALLDATA_SIZE_LIMITCall),
        FORWARDER_HUB_OVERHEAD(FORWARDER_HUB_OVERHEADCall),
        PAYMASTER_ACCEPTANCE_BUDGET(PAYMASTER_ACCEPTANCE_BUDGETCall),
        POST_RELAYED_CALL_GAS_LIMIT(POST_RELAYED_CALL_GAS_LIMITCall),
        PRE_RELAYED_CALL_GAS_LIMIT(PRE_RELAYED_CALL_GAS_LIMITCall),
        gasUsedByPost(gasUsedByPostCall),
        getGasAndDataLimits(getGasAndDataLimitsCall),
        getRelayHub(getRelayHubCall),
        getSelector(getSelectorCall),
        getTrustedForwarder(getTrustedForwarderCall),
        owner(ownerCall),
        postRelayedCall(postRelayedCallCall),
        preRelayedCall(preRelayedCallCall),
        renounceOwnership(renounceOwnershipCall),
        setPostGasUsage(setPostGasUsageCall),
        setRelayHub(setRelayHubCall),
        setTrustedForwarder(setTrustedForwarderCall),
        supportsInterface(supportsInterfaceCall),
        transferOwnership(transferOwnershipCall),
        versionPaymaster(versionPaymasterCall),
        whitelistedContract(whitelistedContractCall),
        whitelistedSelector(whitelistedSelectorCall),
        withdrawRelayHubDepositTo(withdrawRelayHubDepositToCall),
    }
    #[automatically_derived]
    impl OnboardingPaymasterCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [0u8, 190u8, 93u8, 212u8],
            [1u8, 255u8, 201u8, 167u8],
            [12u8, 189u8, 23u8, 200u8],
            [24u8, 228u8, 84u8, 39u8],
            [45u8, 20u8, 196u8, 183u8],
            [92u8, 94u8, 61u8, 177u8],
            [109u8, 124u8, 62u8, 43u8],
            [113u8, 80u8, 24u8, 166u8],
            [118u8, 250u8, 1u8, 195u8],
            [123u8, 176u8, 82u8, 100u8],
            [123u8, 223u8, 46u8, 199u8],
            [141u8, 165u8, 203u8, 91u8],
            [146u8, 18u8, 118u8, 234u8],
            [173u8, 18u8, 229u8, 14u8],
            [176u8, 57u8, 168u8, 143u8],
            [185u8, 11u8, 65u8, 207u8],
            [187u8, 218u8, 163u8, 201u8],
            [195u8, 163u8, 226u8, 250u8],
            [206u8, 27u8, 129u8, 95u8],
            [218u8, 116u8, 34u8, 40u8],
            [223u8, 70u8, 58u8, 102u8],
            [242u8, 253u8, 227u8, 139u8],
            [249u8, 192u8, 2u8, 247u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for OnboardingPaymasterCalls {
        const NAME: &'static str = "OnboardingPaymasterCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 23usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::CALLDATA_SIZE_LIMIT(_) => {
                    <CALLDATA_SIZE_LIMITCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::FORWARDER_HUB_OVERHEAD(_) => {
                    <FORWARDER_HUB_OVERHEADCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::PAYMASTER_ACCEPTANCE_BUDGET(_) => {
                    <PAYMASTER_ACCEPTANCE_BUDGETCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::POST_RELAYED_CALL_GAS_LIMIT(_) => {
                    <POST_RELAYED_CALL_GAS_LIMITCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::PRE_RELAYED_CALL_GAS_LIMIT(_) => {
                    <PRE_RELAYED_CALL_GAS_LIMITCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::gasUsedByPost(_) => {
                    <gasUsedByPostCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getGasAndDataLimits(_) => {
                    <getGasAndDataLimitsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getRelayHub(_) => {
                    <getRelayHubCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getSelector(_) => {
                    <getSelectorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getTrustedForwarder(_) => {
                    <getTrustedForwarderCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::owner(_) => <ownerCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::postRelayedCall(_) => {
                    <postRelayedCallCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::preRelayedCall(_) => {
                    <preRelayedCallCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::renounceOwnership(_) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setPostGasUsage(_) => {
                    <setPostGasUsageCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setRelayHub(_) => {
                    <setRelayHubCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setTrustedForwarder(_) => {
                    <setTrustedForwarderCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::supportsInterface(_) => {
                    <supportsInterfaceCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::transferOwnership(_) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::versionPaymaster(_) => {
                    <versionPaymasterCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::whitelistedContract(_) => {
                    <whitelistedContractCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::whitelistedSelector(_) => {
                    <whitelistedSelectorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::withdrawRelayHubDepositTo(_) => {
                    <withdrawRelayHubDepositToCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<OnboardingPaymasterCalls>] = &[
                {
                    fn preRelayedCall(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OnboardingPaymasterCalls> {
                        <preRelayedCallCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OnboardingPaymasterCalls::preRelayedCall)
                    }
                    preRelayedCall
                },
                {
                    fn supportsInterface(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OnboardingPaymasterCalls> {
                        <supportsInterfaceCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OnboardingPaymasterCalls::supportsInterface)
                    }
                    supportsInterface
                },
                {
                    fn getSelector(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OnboardingPaymasterCalls> {
                        <getSelectorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OnboardingPaymasterCalls::getSelector)
                    }
                    getSelector
                },
                {
                    fn whitelistedContract(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OnboardingPaymasterCalls> {
                        <whitelistedContractCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OnboardingPaymasterCalls::whitelistedContract)
                    }
                    whitelistedContract
                },
                {
                    fn withdrawRelayHubDepositTo(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OnboardingPaymasterCalls> {
                        <withdrawRelayHubDepositToCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OnboardingPaymasterCalls::withdrawRelayHubDepositTo)
                    }
                    withdrawRelayHubDepositTo
                },
                {
                    fn CALLDATA_SIZE_LIMIT(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OnboardingPaymasterCalls> {
                        <CALLDATA_SIZE_LIMITCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OnboardingPaymasterCalls::CALLDATA_SIZE_LIMIT)
                    }
                    CALLDATA_SIZE_LIMIT
                },
                {
                    fn setPostGasUsage(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OnboardingPaymasterCalls> {
                        <setPostGasUsageCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OnboardingPaymasterCalls::setPostGasUsage)
                    }
                    setPostGasUsage
                },
                {
                    fn renounceOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OnboardingPaymasterCalls> {
                        <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OnboardingPaymasterCalls::renounceOwnership)
                    }
                    renounceOwnership
                },
                {
                    fn postRelayedCall(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OnboardingPaymasterCalls> {
                        <postRelayedCallCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OnboardingPaymasterCalls::postRelayedCall)
                    }
                    postRelayedCall
                },
                {
                    fn setRelayHub(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OnboardingPaymasterCalls> {
                        <setRelayHubCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OnboardingPaymasterCalls::setRelayHub)
                    }
                    setRelayHub
                },
                {
                    fn getRelayHub(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OnboardingPaymasterCalls> {
                        <getRelayHubCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OnboardingPaymasterCalls::getRelayHub)
                    }
                    getRelayHub
                },
                {
                    fn owner(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OnboardingPaymasterCalls> {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OnboardingPaymasterCalls::owner)
                    }
                    owner
                },
                {
                    fn versionPaymaster(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OnboardingPaymasterCalls> {
                        <versionPaymasterCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OnboardingPaymasterCalls::versionPaymaster)
                    }
                    versionPaymaster
                },
                {
                    fn gasUsedByPost(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OnboardingPaymasterCalls> {
                        <gasUsedByPostCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OnboardingPaymasterCalls::gasUsedByPost)
                    }
                    gasUsedByPost
                },
                {
                    fn getGasAndDataLimits(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OnboardingPaymasterCalls> {
                        <getGasAndDataLimitsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OnboardingPaymasterCalls::getGasAndDataLimits)
                    }
                    getGasAndDataLimits
                },
                {
                    fn FORWARDER_HUB_OVERHEAD(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OnboardingPaymasterCalls> {
                        <FORWARDER_HUB_OVERHEADCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OnboardingPaymasterCalls::FORWARDER_HUB_OVERHEAD)
                    }
                    FORWARDER_HUB_OVERHEAD
                },
                {
                    fn POST_RELAYED_CALL_GAS_LIMIT(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OnboardingPaymasterCalls> {
                        <POST_RELAYED_CALL_GAS_LIMITCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OnboardingPaymasterCalls::POST_RELAYED_CALL_GAS_LIMIT)
                    }
                    POST_RELAYED_CALL_GAS_LIMIT
                },
                {
                    fn whitelistedSelector(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OnboardingPaymasterCalls> {
                        <whitelistedSelectorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OnboardingPaymasterCalls::whitelistedSelector)
                    }
                    whitelistedSelector
                },
                {
                    fn getTrustedForwarder(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OnboardingPaymasterCalls> {
                        <getTrustedForwarderCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OnboardingPaymasterCalls::getTrustedForwarder)
                    }
                    getTrustedForwarder
                },
                {
                    fn setTrustedForwarder(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OnboardingPaymasterCalls> {
                        <setTrustedForwarderCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OnboardingPaymasterCalls::setTrustedForwarder)
                    }
                    setTrustedForwarder
                },
                {
                    fn PAYMASTER_ACCEPTANCE_BUDGET(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OnboardingPaymasterCalls> {
                        <PAYMASTER_ACCEPTANCE_BUDGETCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OnboardingPaymasterCalls::PAYMASTER_ACCEPTANCE_BUDGET)
                    }
                    PAYMASTER_ACCEPTANCE_BUDGET
                },
                {
                    fn transferOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OnboardingPaymasterCalls> {
                        <transferOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OnboardingPaymasterCalls::transferOwnership)
                    }
                    transferOwnership
                },
                {
                    fn PRE_RELAYED_CALL_GAS_LIMIT(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OnboardingPaymasterCalls> {
                        <PRE_RELAYED_CALL_GAS_LIMITCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OnboardingPaymasterCalls::PRE_RELAYED_CALL_GAS_LIMIT)
                    }
                    PRE_RELAYED_CALL_GAS_LIMIT
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
                Self::CALLDATA_SIZE_LIMIT(inner) => {
                    <CALLDATA_SIZE_LIMITCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::FORWARDER_HUB_OVERHEAD(inner) => {
                    <FORWARDER_HUB_OVERHEADCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::PAYMASTER_ACCEPTANCE_BUDGET(inner) => {
                    <PAYMASTER_ACCEPTANCE_BUDGETCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::POST_RELAYED_CALL_GAS_LIMIT(inner) => {
                    <POST_RELAYED_CALL_GAS_LIMITCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::PRE_RELAYED_CALL_GAS_LIMIT(inner) => {
                    <PRE_RELAYED_CALL_GAS_LIMITCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::gasUsedByPost(inner) => {
                    <gasUsedByPostCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getGasAndDataLimits(inner) => {
                    <getGasAndDataLimitsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getRelayHub(inner) => {
                    <getRelayHubCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getSelector(inner) => {
                    <getSelectorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getTrustedForwarder(inner) => {
                    <getTrustedForwarderCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::postRelayedCall(inner) => {
                    <postRelayedCallCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::preRelayedCall(inner) => {
                    <preRelayedCallCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::renounceOwnership(inner) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setPostGasUsage(inner) => {
                    <setPostGasUsageCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setRelayHub(inner) => {
                    <setRelayHubCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setTrustedForwarder(inner) => {
                    <setTrustedForwarderCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::supportsInterface(inner) => {
                    <supportsInterfaceCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::transferOwnership(inner) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::versionPaymaster(inner) => {
                    <versionPaymasterCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::whitelistedContract(inner) => {
                    <whitelistedContractCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::whitelistedSelector(inner) => {
                    <whitelistedSelectorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::withdrawRelayHubDepositTo(inner) => {
                    <withdrawRelayHubDepositToCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::CALLDATA_SIZE_LIMIT(inner) => {
                    <CALLDATA_SIZE_LIMITCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::FORWARDER_HUB_OVERHEAD(inner) => {
                    <FORWARDER_HUB_OVERHEADCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::PAYMASTER_ACCEPTANCE_BUDGET(inner) => {
                    <PAYMASTER_ACCEPTANCE_BUDGETCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::POST_RELAYED_CALL_GAS_LIMIT(inner) => {
                    <POST_RELAYED_CALL_GAS_LIMITCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::PRE_RELAYED_CALL_GAS_LIMIT(inner) => {
                    <PRE_RELAYED_CALL_GAS_LIMITCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::gasUsedByPost(inner) => {
                    <gasUsedByPostCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getGasAndDataLimits(inner) => {
                    <getGasAndDataLimitsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getRelayHub(inner) => {
                    <getRelayHubCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getSelector(inner) => {
                    <getSelectorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getTrustedForwarder(inner) => {
                    <getTrustedForwarderCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::postRelayedCall(inner) => {
                    <postRelayedCallCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::preRelayedCall(inner) => {
                    <preRelayedCallCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::renounceOwnership(inner) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setPostGasUsage(inner) => {
                    <setPostGasUsageCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setRelayHub(inner) => {
                    <setRelayHubCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setTrustedForwarder(inner) => {
                    <setTrustedForwarderCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::transferOwnership(inner) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::versionPaymaster(inner) => {
                    <versionPaymasterCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::whitelistedContract(inner) => {
                    <whitelistedContractCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::whitelistedSelector(inner) => {
                    <whitelistedSelectorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::withdrawRelayHubDepositTo(inner) => {
                    <withdrawRelayHubDepositToCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`OnboardingPaymaster`](self) events.
    pub enum OnboardingPaymasterEvents {
        OwnershipTransferred(OwnershipTransferred),
        PostRelay(PostRelay),
        PreRelay(PreRelay),
    }
    #[automatically_derived]
    impl OnboardingPaymasterEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                49u8,
                139u8,
                246u8,
                54u8,
                227u8,
                30u8,
                180u8,
                173u8,
                197u8,
                148u8,
                241u8,
                76u8,
                67u8,
                172u8,
                11u8,
                80u8,
                191u8,
                68u8,
                156u8,
                149u8,
                158u8,
                131u8,
                68u8,
                173u8,
                198u8,
                54u8,
                128u8,
                18u8,
                216u8,
                184u8,
                60u8,
                95u8,
            ],
            [
                113u8,
                123u8,
                246u8,
                251u8,
                53u8,
                28u8,
                49u8,
                110u8,
                209u8,
                158u8,
                100u8,
                24u8,
                51u8,
                114u8,
                87u8,
                191u8,
                44u8,
                90u8,
                122u8,
                18u8,
                236u8,
                167u8,
                142u8,
                77u8,
                38u8,
                209u8,
                13u8,
                148u8,
                193u8,
                196u8,
                151u8,
                207u8,
            ],
            [
                139u8,
                224u8,
                7u8,
                156u8,
                83u8,
                22u8,
                89u8,
                20u8,
                19u8,
                68u8,
                205u8,
                31u8,
                208u8,
                164u8,
                242u8,
                132u8,
                25u8,
                73u8,
                127u8,
                151u8,
                34u8,
                163u8,
                218u8,
                175u8,
                227u8,
                180u8,
                24u8,
                111u8,
                107u8,
                100u8,
                87u8,
                224u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for OnboardingPaymasterEvents {
        const NAME: &'static str = "OnboardingPaymasterEvents";
        const COUNT: usize = 3usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(
                    <OwnershipTransferred as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OwnershipTransferred as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::OwnershipTransferred)
                }
                Some(<PostRelay as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <PostRelay as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::PostRelay)
                }
                Some(<PreRelay as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <PreRelay as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::PreRelay)
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
    impl alloy_sol_types::private::IntoLogData for OnboardingPaymasterEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::PostRelay(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::PreRelay(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::PostRelay(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::PreRelay(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`OnboardingPaymaster`](self) contract instance.

See the [wrapper's documentation](`OnboardingPaymasterInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> OnboardingPaymasterInstance<T, P, N> {
        OnboardingPaymasterInstance::<T, P, N>::new(address, provider)
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
        _whitelistedContract: alloy::sol_types::private::Address,
        _whitelistedSelector: u32,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<OnboardingPaymasterInstance<T, P, N>>,
    > {
        OnboardingPaymasterInstance::<
            T,
            P,
            N,
        >::deploy(provider, _whitelistedContract, _whitelistedSelector)
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
    >(
        provider: P,
        _whitelistedContract: alloy::sol_types::private::Address,
        _whitelistedSelector: u32,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        OnboardingPaymasterInstance::<
            T,
            P,
            N,
        >::deploy_builder(provider, _whitelistedContract, _whitelistedSelector)
    }
    /**A [`OnboardingPaymaster`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`OnboardingPaymaster`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct OnboardingPaymasterInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for OnboardingPaymasterInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("OnboardingPaymasterInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > OnboardingPaymasterInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`OnboardingPaymaster`](self) contract instance.

See the [wrapper's documentation](`OnboardingPaymasterInstance`) for more details.*/
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
            _whitelistedContract: alloy::sol_types::private::Address,
            _whitelistedSelector: u32,
        ) -> alloy_contract::Result<OnboardingPaymasterInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(
                provider,
                _whitelistedContract,
                _whitelistedSelector,
            );
            let contract_address = call_builder.deploy().await?;
            Ok(Self::new(contract_address, call_builder.provider))
        }
        /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
        #[inline]
        pub fn deploy_builder(
            provider: P,
            _whitelistedContract: alloy::sol_types::private::Address,
            _whitelistedSelector: u32,
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall {
                            _whitelistedContract,
                            _whitelistedSelector,
                        },
                    )[..],
                ]
                    .concat()
                    .into(),
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
    impl<T, P: ::core::clone::Clone, N> OnboardingPaymasterInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> OnboardingPaymasterInstance<T, P, N> {
            OnboardingPaymasterInstance {
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
    > OnboardingPaymasterInstance<T, P, N> {
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
        ///Creates a new call builder for the [`CALLDATA_SIZE_LIMIT`] function.
        pub fn CALLDATA_SIZE_LIMIT(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, CALLDATA_SIZE_LIMITCall, N> {
            self.call_builder(&CALLDATA_SIZE_LIMITCall {})
        }
        ///Creates a new call builder for the [`FORWARDER_HUB_OVERHEAD`] function.
        pub fn FORWARDER_HUB_OVERHEAD(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, FORWARDER_HUB_OVERHEADCall, N> {
            self.call_builder(&FORWARDER_HUB_OVERHEADCall {})
        }
        ///Creates a new call builder for the [`PAYMASTER_ACCEPTANCE_BUDGET`] function.
        pub fn PAYMASTER_ACCEPTANCE_BUDGET(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, PAYMASTER_ACCEPTANCE_BUDGETCall, N> {
            self.call_builder(&PAYMASTER_ACCEPTANCE_BUDGETCall {})
        }
        ///Creates a new call builder for the [`POST_RELAYED_CALL_GAS_LIMIT`] function.
        pub fn POST_RELAYED_CALL_GAS_LIMIT(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, POST_RELAYED_CALL_GAS_LIMITCall, N> {
            self.call_builder(&POST_RELAYED_CALL_GAS_LIMITCall {})
        }
        ///Creates a new call builder for the [`PRE_RELAYED_CALL_GAS_LIMIT`] function.
        pub fn PRE_RELAYED_CALL_GAS_LIMIT(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, PRE_RELAYED_CALL_GAS_LIMITCall, N> {
            self.call_builder(&PRE_RELAYED_CALL_GAS_LIMITCall {})
        }
        ///Creates a new call builder for the [`gasUsedByPost`] function.
        pub fn gasUsedByPost(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, gasUsedByPostCall, N> {
            self.call_builder(&gasUsedByPostCall {})
        }
        ///Creates a new call builder for the [`getGasAndDataLimits`] function.
        pub fn getGasAndDataLimits(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getGasAndDataLimitsCall, N> {
            self.call_builder(&getGasAndDataLimitsCall {})
        }
        ///Creates a new call builder for the [`getRelayHub`] function.
        pub fn getRelayHub(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getRelayHubCall, N> {
            self.call_builder(&getRelayHubCall {})
        }
        ///Creates a new call builder for the [`getSelector`] function.
        pub fn getSelector(
            &self,
            call: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, getSelectorCall, N> {
            self.call_builder(&getSelectorCall { call })
        }
        ///Creates a new call builder for the [`getTrustedForwarder`] function.
        pub fn getTrustedForwarder(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getTrustedForwarderCall, N> {
            self.call_builder(&getTrustedForwarderCall {})
        }
        ///Creates a new call builder for the [`owner`] function.
        pub fn owner(&self) -> alloy_contract::SolCallBuilder<T, &P, ownerCall, N> {
            self.call_builder(&ownerCall {})
        }
        ///Creates a new call builder for the [`postRelayedCall`] function.
        pub fn postRelayedCall(
            &self,
            context: alloy::sol_types::private::Bytes,
            success: bool,
            gasUseWithoutPost: alloy::sol_types::private::primitives::aliases::U256,
            relayData: <GsnTypes::RelayData as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, postRelayedCallCall, N> {
            self.call_builder(
                &postRelayedCallCall {
                    context,
                    success,
                    gasUseWithoutPost,
                    relayData,
                },
            )
        }
        ///Creates a new call builder for the [`preRelayedCall`] function.
        pub fn preRelayedCall(
            &self,
            relayRequest: <GsnTypes::RelayRequest as alloy::sol_types::SolType>::RustType,
            signature: alloy::sol_types::private::Bytes,
            approvalData: alloy::sol_types::private::Bytes,
            maxPossibleGas: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, preRelayedCallCall, N> {
            self.call_builder(
                &preRelayedCallCall {
                    relayRequest,
                    signature,
                    approvalData,
                    maxPossibleGas,
                },
            )
        }
        ///Creates a new call builder for the [`renounceOwnership`] function.
        pub fn renounceOwnership(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, renounceOwnershipCall, N> {
            self.call_builder(&renounceOwnershipCall {})
        }
        ///Creates a new call builder for the [`setPostGasUsage`] function.
        pub fn setPostGasUsage(
            &self,
            _gasUsedByPost: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, setPostGasUsageCall, N> {
            self.call_builder(
                &setPostGasUsageCall {
                    _gasUsedByPost,
                },
            )
        }
        ///Creates a new call builder for the [`setRelayHub`] function.
        pub fn setRelayHub(
            &self,
            hub: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, setRelayHubCall, N> {
            self.call_builder(&setRelayHubCall { hub })
        }
        ///Creates a new call builder for the [`setTrustedForwarder`] function.
        pub fn setTrustedForwarder(
            &self,
            forwarder: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, setTrustedForwarderCall, N> {
            self.call_builder(
                &setTrustedForwarderCall {
                    forwarder,
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
        ///Creates a new call builder for the [`transferOwnership`] function.
        pub fn transferOwnership(
            &self,
            newOwner: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, transferOwnershipCall, N> {
            self.call_builder(&transferOwnershipCall { newOwner })
        }
        ///Creates a new call builder for the [`versionPaymaster`] function.
        pub fn versionPaymaster(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, versionPaymasterCall, N> {
            self.call_builder(&versionPaymasterCall {})
        }
        ///Creates a new call builder for the [`whitelistedContract`] function.
        pub fn whitelistedContract(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, whitelistedContractCall, N> {
            self.call_builder(&whitelistedContractCall {})
        }
        ///Creates a new call builder for the [`whitelistedSelector`] function.
        pub fn whitelistedSelector(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, whitelistedSelectorCall, N> {
            self.call_builder(&whitelistedSelectorCall {})
        }
        ///Creates a new call builder for the [`withdrawRelayHubDepositTo`] function.
        pub fn withdrawRelayHubDepositTo(
            &self,
            amount: alloy::sol_types::private::primitives::aliases::U256,
            target: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, withdrawRelayHubDepositToCall, N> {
            self.call_builder(
                &withdrawRelayHubDepositToCall {
                    amount,
                    target,
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
    > OnboardingPaymasterInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`OwnershipTransferred`] event.
        pub fn OwnershipTransferred_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OwnershipTransferred, N> {
            self.event_filter::<OwnershipTransferred>()
        }
        ///Creates a new event filter for the [`PostRelay`] event.
        pub fn PostRelay_filter(&self) -> alloy_contract::Event<T, &P, PostRelay, N> {
            self.event_filter::<PostRelay>()
        }
        ///Creates a new event filter for the [`PreRelay`] event.
        pub fn PreRelay_filter(&self) -> alloy_contract::Event<T, &P, PreRelay, N> {
            self.event_filter::<PreRelay>()
        }
    }
}
