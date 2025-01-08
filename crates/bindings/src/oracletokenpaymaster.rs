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

interface OracleTokenPaymaster {
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    event PostRelay(uint256 actualEthAmount, address token, uint256 actualTokenAmount, address payer);
    event PreRelayPayment(uint256 ethAmount, address token, uint256 tokenAmount, address indexed payer);

    constructor(address _nativeTokenOracle);

    receive() external payable;

    function CALLDATA_SIZE_LIMIT() external view returns (uint256);
    function FORWARDER_HUB_OVERHEAD() external view returns (uint256);
    function PAYMASTER_ACCEPTANCE_BUDGET() external view returns (uint256);
    function POST_RELAYED_CALL_GAS_LIMIT() external view returns (uint256);
    function PRE_RELAYED_CALL_GAS_LIMIT() external view returns (uint256);
    function __preRelayedCall(GsnTypes.RelayRequest memory relayRequest, bytes memory signature, bytes memory approvalData, uint256 maxPossibleGas) external;
    function addOracle(address _token, uint256 _decimals, address _oracle) external;
    function gasUsedByPost() external view returns (uint256);
    function getGasAndDataLimits() external view returns (IPaymaster.GasAndDataLimits memory limits);
    function getPayer(GsnTypes.RelayRequest memory relayRequest) external view returns (address);
    function getRelayHub() external view returns (address);
    function getTrustedForwarder() external view returns (address);
    function owner() external view returns (address);
    function postRelayedCall(bytes memory context, bool success, uint256 gasUseWithoutPost, GsnTypes.RelayData memory relayData) external;
    function preRelayedCall(GsnTypes.RelayRequest memory relayRequest, bytes memory signature, bytes memory approvalData, uint256 maxPossibleGas) external returns (bytes memory, bool);
    function renounceOwnership() external;
    function setPostGasUsage(uint256 _gasUsedByPost) external;
    function setRelayHub(address hub) external;
    function setTrustedForwarder(address forwarder) external;
    function supportsInterface(bytes4 interfaceId) external view returns (bool);
    function tokenOracles(address) external view returns (uint256 div, address oracle);
    function transferOwnership(address newOwner) external;
    function versionPaymaster() external view returns (string memory);
    function withdrawAll(address token) external;
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
        "name": "_nativeTokenOracle",
        "type": "address",
        "internalType": "contract IOracle"
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
    "name": "__preRelayedCall",
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
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "addOracle",
    "inputs": [
      {
        "name": "_token",
        "type": "address",
        "internalType": "contract IERC20"
      },
      {
        "name": "_decimals",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "_oracle",
        "type": "address",
        "internalType": "contract IOracle"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
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
    "name": "getPayer",
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
    "name": "tokenOracles",
    "inputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IERC20"
      }
    ],
    "outputs": [
      {
        "name": "div",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "oracle",
        "type": "address",
        "internalType": "contract IOracle"
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
    "name": "withdrawAll",
    "inputs": [
      {
        "name": "token",
        "type": "address",
        "internalType": "contract IERC20"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
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
        "name": "actualEthAmount",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "token",
        "type": "address",
        "indexed": false,
        "internalType": "contract IERC20"
      },
      {
        "name": "actualTokenAmount",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "payer",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "PreRelayPayment",
    "inputs": [
      {
        "name": "ethAmount",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "token",
        "type": "address",
        "indexed": false,
        "internalType": "contract IERC20"
      },
      {
        "name": "tokenAmount",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "payer",
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
pub mod OracleTokenPaymaster {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b506040516129f63803806129f683398101604081905261002e91610175565b61003733610126565b806001600160a01b031663313ce5676040518163ffffffff1660e01b8152600401602060405180830381865afa158015610073573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061009791906101a2565b60ff166008146101015760405162461bcd60e51b815260206004820152602b60248201527f4f54503a206e617469766520746f6b656e206f7261636c6520646563696d616c60448201526a0e640daeae6e840c4ca40760ab1b606482015260840160405180910390fd5b600380546001600160a01b0319166001600160a01b03929092169190911790556101c2565b5f80546001600160a01b038381166001600160a01b0319831681178455604051919092169283917f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09190a35050565b5f60208284031215610185575f5ffd5b81516001600160a01b038116811461019b575f5ffd5b9392505050565b5f602082840312156101b2575f5ffd5b815160ff8116811461019b575f5ffd5b612827806101cf5f395ff3fe608060405260043610610190575f3560e01c80639fbb0dca116100dc578063cd4bc61111610087578063df463a6611610062578063df463a6614610569578063f2fde38b1461057d578063f9c002f71461059c578063fa09e630146105b2575f5ffd5b8063cd4bc6111461050e578063ce1b815f1461052d578063da7422281461054a575f5ffd5b8063b90b41cf116100b7578063b90b41cf1461048b578063bbdaa3c9146104a0578063bd492432146104b6575f5ffd5b80639fbb0dca14610410578063ad12e50e1461042f578063b039a88f14610444575f5ffd5b8063715018a61161013c5780637bdf2ec7116101175780637bdf2ec7146103b65780638da5cb5b146103d3578063921276ea146103ef575f5ffd5b8063715018a61461036457806376fa01c3146103785780637bb0526414610397575f5ffd5b806341bbb7ca1161016c57806341bbb7ca146102eb5780635c5e3db1146103225780636d7c3e2b14610345575f5ffd5b8062be5dd41461026757806301ffc9a71461029d5780632d14c4b7146102cc575f5ffd5b36610263576001546001600160a01b03166101f25760405162461bcd60e51b815260206004820152601960248201527f72656c6179206875622061646472657373206e6f74207365740000000000000060448201526064015b60405180910390fd5b6001546040517faa67c9190000000000000000000000000000000000000000000000000000000081523060048201526001600160a01b039091169063aa67c9199034906024015f604051808303818588803b15801561024f575f5ffd5b505af1158015610261573d5f5f3e3d5ffd5b005b5f5ffd5b348015610272575f5ffd5b50610286610281366004611e34565b6105d1565b604051610294929190611f04565b60405180910390f35b3480156102a8575f5ffd5b506102bc6102b7366004611f27565b61061e565b6040519015158152602001610294565b3480156102d7575f5ffd5b506102616102e6366004611f7a565b610702565b3480156102f6575f5ffd5b5061030a610305366004611fa8565b6107d9565b6040516001600160a01b039091168152602001610294565b34801561032d575f5ffd5b5061033761290481565b604051908152602001610294565b348015610350575f5ffd5b5061026161035f366004611fe2565b6107f2565b34801561036f575f5ffd5b50610261610850565b348015610383575f5ffd5b50610261610392366004612006565b6108b4565b3480156103a2575f5ffd5b506102616103b1366004612092565b6108d0565b3480156103c1575f5ffd5b506001546001600160a01b031661030a565b3480156103de575f5ffd5b505f546001600160a01b031661030a565b3480156103fa575f5ffd5b506104036109e2565b60405161029491906120ad565b34801561041b575f5ffd5b5061026161042a366004611e34565b610a02565b34801561043a575f5ffd5b5061033760055481565b34801561044f575f5ffd5b50610458610a1a565b60405161029491908151815260208083015190820152604080830151908201526060918201519181019190915260800190565b348015610496575f5ffd5b5061033761c35081565b3480156104ab575f5ffd5b506103376201adb081565b3480156104c1575f5ffd5b506104f16104d0366004612092565b60046020525f9081526040902080546001909101546001600160a01b031682565b604080519283526001600160a01b03909116602083015201610294565b348015610519575f5ffd5b506102616105283660046120bf565b610a7f565b348015610538575f5ffd5b506002546001600160a01b031661030a565b348015610555575f5ffd5b50610261610564366004612092565b610c27565b348015610574575f5ffd5b50610337610d39565b348015610588575f5ffd5b50610261610597366004612092565b610d4b565b3480156105a7575f5ffd5b50610337620186a081565b3480156105bd575f5ffd5b506102616105cc366004612092565b610e2c565b60605f6105dc610f95565b6105e588610fef565b6105ee88611087565b6105f7886110e2565b610601858561114e565b61060f8888888888886111a0565b91509150965096945050505050565b5f7fffffffff0000000000000000000000000000000000000000000000000000000082167fe1ab2dea0000000000000000000000000000000000000000000000000000000014806106b057507fffffffff0000000000000000000000000000000000000000000000000000000082167f0e08307600000000000000000000000000000000000000000000000000000000145b806106fc57507f01ffc9a7000000000000000000000000000000000000000000000000000000007fffffffff000000000000000000000000000000000000000000000000000000008316145b92915050565b5f546001600160a01b0316331461075b5760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e657260448201526064016101e9565b6001546040517ff3fef3a30000000000000000000000000000000000000000000000000000000081526001600160a01b038381166004830152602482018590529091169063f3fef3a3906044015f604051808303815f87803b1580156107bf575f5ffd5b505af11580156107d1573d5f5f3e3d5ffd5b505050505050565b5f6107e4828061210e565b6106fc906020810190612092565b5f546001600160a01b0316331461084b5760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e657260448201526064016101e9565b600555565b5f546001600160a01b031633146108a95760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e657260448201526064016101e9565b6108b25f6113b0565b565b6108bc610f95565b6108c98585858585611417565b5050505050565b5f546001600160a01b031633146109295760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e657260448201526064016101e9565b61095c6001600160a01b0382167fe9fb30f70000000000000000000000000000000000000000000000000000000061143a565b6109a85760405162461bcd60e51b815260206004820152601f60248201527f746172676574206973206e6f7420612076616c6964204952656c61794875620060448201526064016101e9565b600180547fffffffffffffffffffffffff0000000000000000000000000000000000000000166001600160a01b0392909216919091179055565b60606040518060600160405280602d81526020016127c5602d9139905090565b610a108686868686866111a0565b5050505050505050565b610a4160405180608001604052805f81526020015f81526020015f81526020015f81525090565b604051806080016040528061c350620186a0610a5d9190612177565b8152602001620186a081526020016201adb08152602001612904815250905090565b5f546001600160a01b03163314610ad85760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e657260448201526064016101e9565b806001600160a01b031663313ce5676040518163ffffffff1660e01b8152600401602060405180830381865afa158015610b14573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610b38919061218a565b60ff16600814610baf5760405162461bcd60e51b8152602060048201526024808201527f4f54503a20746f6b656e206f7261636c6520646563696d616c73206d7573742060448201527f626520380000000000000000000000000000000000000000000000000000000060648201526084016101e9565b604051806040016040528083600a610bc7919061228d565b81526001600160a01b039283166020918201529382165f908152600485526040902081518155930151600190930180547fffffffffffffffffffffffff000000000000000000000000000000000000000016939091169290921790915550565b5f546001600160a01b03163314610c805760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e657260448201526064016101e9565b610cb36001600160a01b0382167f25e23e640000000000000000000000000000000000000000000000000000000061143a565b610cff5760405162461bcd60e51b815260206004820181905260248201527f746172676574206973206e6f7420612076616c69642049466f7277617264657260448201526064016101e9565b600280547fffffffffffffffffffffffff0000000000000000000000000000000000000000166001600160a01b0392909216919091179055565b610d4861c350620186a0612177565b81565b5f546001600160a01b03163314610da45760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e657260448201526064016101e9565b6001600160a01b038116610e205760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201527f646472657373000000000000000000000000000000000000000000000000000060648201526084016101e9565b610e29816113b0565b50565b5f546001600160a01b03163314610e855760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e657260448201526064016101e9565b6040517f70a082310000000000000000000000000000000000000000000000000000000081523060048201525f906001600160a01b038316906370a0823190602401602060405180830381865afa158015610ee2573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610f069190612298565b6040517fa9059cbb000000000000000000000000000000000000000000000000000000008152336004820152602481018290529091506001600160a01b0383169063a9059cbb906044016020604051808303815f875af1158015610f6c573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610f9091906122af565b505050565b6001546001600160a01b031633146108b25760405162461bcd60e51b815260206004820152601e60248201527f63616e206f6e6c792062652063616c6c65642062792052656c6179487562000060448201526064016101e9565b610ffc60208201826122ca565b61100d9060c081019060a001612092565b6001600160a01b03166110286002546001600160a01b031690565b6001600160a01b03161461107e5760405162461bcd60e51b815260206004820152601860248201527f466f72776172646572206973206e6f742074727573746564000000000000000060448201526064016101e9565b610e298161145c565b611091818061210e565b6040013515610e295760405162461bcd60e51b815260206004820152601c60248201527f76616c7565207472616e73666572206e6f7420737570706f727465640000000060448201526064016101e9565b6110ef60208201826122ca565b6110fd9060c08101906122fc565b9050604014610e295760405162461bcd60e51b815260206004820152601d60248201527f7061796d6173746572446174613a20696e76616c6964206c656e67746800000060448201526064016101e9565b801561119c5760405162461bcd60e51b815260206004820152601b60248201527f73686f756c642068617665206e6f20617070726f76616c44617461000000000060448201526064016101e9565b5050565b60605f80806111fc6111b560208c018c6122ca565b6111c39060c08101906122fc565b8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f9201919091525061168292505050565b915091505f5f5f61120e858e8a6116a6565b9250925092508381111561128a5760405162461bcd60e51b815260206004820152602960248201527f547820636f7374206d6f7265207468616e2074686520757365722d737570706c60448201527f696564206c696d6974000000000000000000000000000000000000000000000060648201526084016101e9565b6040517f23b872dd0000000000000000000000000000000000000000000000000000000081526001600160a01b038481166004830152306024830152604482018390528616906323b872dd906064016020604051808303815f875af11580156112f5573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061131991906122af565b50604080518381526001600160a01b038781166020830152918101839052908416907fc5784efefb9e3c3bdecec21cdd61e420e96858d8f1aaaa6dbbe30b052075e1999060600160405180910390a2604080516001600160a01b038086166020830152918101839052908616606082015260800160408051601f198184030181529190529d5f9d509b505050505050505050505050565b5f80546001600160a01b038381167fffffffffffffffffffffffff0000000000000000000000000000000000000000831681178455604051919092169283917f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09190a35050565b5f8080611426878901896120bf565b925092509250610a1083835f8888866117dc565b5f611444836119b3565b801561145557506114558383611a16565b9392505050565b5f80611468838061210e565b611479906040810190602001612092565b6001600160a01b03167f572b6c05000000000000000000000000000000000000000000000000000000006114b060208601866122ca565b6114c19060c081019060a001612092565b6040516001600160a01b03909116602482015260440160408051601f198184030181529181526020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167fffffffff00000000000000000000000000000000000000000000000000000000909416939093179092529051611542919061235d565b5f60405180830381855afa9150503d805f811461157a576040519150601f19603f3d011682016040523d82523d5f602084013e61157f565b606091505b5091509150816115d15760405162461bcd60e51b815260206004820152601c60248201527f697354727573746564466f727761726465723a2072657665727465640000000060448201526064016101e9565b80516020146116225760405162461bcd60e51b815260206004820181905260248201527f697354727573746564466f727761726465723a2062616420726573706f6e736560448201526064016101e9565b8080602001905181019061163691906122af565b610f905760405162461bcd60e51b815260206004820152601f60248201527f696e76616c696420666f7277617264657220666f7220726563697069656e740060448201526064016101e9565b5f5f5f5f8480602001905181019061169a9190612373565b90969095509350505050565b6040517f41bbb7ca0000000000000000000000000000000000000000000000000000000081525f908190819030906341bbb7ca906116e8908890600401612506565b602060405180830381865afa158015611703573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061172791906125ff565b6001549093505f906001600160a01b0316638e53548b8661174b60208a018a6122ca565b6040518363ffffffff1660e01b815260040161176892919061261a565b602060405180830381865afa158015611783573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906117a79190612298565b90506117b3868061210e565b6117c1906040013582612177565b90506117cd8782611b40565b91508092505093509350939050565b6001546005545f916001600160a01b031690638e53548b906117fe9087612177565b856040518363ffffffff1660e01b815260040161181c92919061261a565b602060405180830381865afa158015611837573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061185b9190612298565b90505f6118718361186c8489612177565b611b40565b90505f61187e8289612632565b604080518581526001600160a01b038088166020830152918101859052908b1660608201529091507fdb4e2886d87e892abf3e8d4277653ba2a26cb0130780415d341334bebf5edcca9060800160405180910390a16040517fa9059cbb0000000000000000000000000000000000000000000000000000000081526001600160a01b038a811660048301526024820183905285169063a9059cbb906044016020604051808303815f875af1158015611938573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061195c91906122af565b6119a85760405162461bcd60e51b815260206004820152600d60248201527f6661696c656420726566756e640000000000000000000000000000000000000060448201526064016101e9565b505050505050505050565b5f6119de827f01ffc9a700000000000000000000000000000000000000000000000000000000611a16565b80156106fc5750611a0f827fffffffff00000000000000000000000000000000000000000000000000000000611a16565b1592915050565b604080517fffffffff00000000000000000000000000000000000000000000000000000000831660248083019190915282518083039091018152604490910182526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f01ffc9a70000000000000000000000000000000000000000000000000000000017905290515f9190829081906001600160a01b0387169061753090611ac290869061235d565b5f604051808303818686fa925050503d805f8114611afb576040519150601f19603f3d011682016040523d82523d5f602084013e611b00565b606091505b5091509150602081511015611b1a575f93505050506106fc565b818015611b36575080806020019051810190611b3691906122af565b9695505050505050565b6001600160a01b038083165f90815260046020908152604080832081518083019092528054825260010154909316908301819052909190611bc35760405162461bcd60e51b815260206004820152601a60248201527f4f54503a204f7261636c6520646f6573206e6f7420657869737400000000000060448201526064016101e9565b5f611bd18260200151611c4e565b6003549091505f90611beb906001600160a01b0316611c4e565b90505f82845f015183611bfe9190612645565b611c0891906126e4565b90505f670de0b6b3a7640000611c3877ffffffffffffffffffffffffffffffffffffffffffffffff841689612733565b611c42919061274a565b98975050505050505050565b5f5f5f5f5f856001600160a01b031663feaf968c6040518163ffffffff1660e01b815260040160a060405180830381865afa158015611c8f573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611cb39190612776565b9450945050935093505f8313611d0b5760405162461bcd60e51b815260206004820152601a60248201527f4f5450203a20436861696e6c696e6b207072696365203c3d203000000000000060448201526064016101e9565b611d186202a30042612632565b821015611d675760405162461bcd60e51b815260206004820152601660248201527f4f5450203a20496e636f6d706c65746520726f756e640000000000000000000060448201526064016101e9565b8369ffffffffffffffffffff168169ffffffffffffffffffff161015611dcf5760405162461bcd60e51b815260206004820152601160248201527f4f5450203a205374616c6520707269636500000000000000000000000000000060448201526064016101e9565b5090949350505050565b5f60408284031215611de9575f5ffd5b50919050565b5f5f83601f840112611dff575f5ffd5b50813567ffffffffffffffff811115611e16575f5ffd5b602083019150836020828501011115611e2d575f5ffd5b9250929050565b5f5f5f5f5f5f60808789031215611e49575f5ffd5b863567ffffffffffffffff811115611e5f575f5ffd5b611e6b89828a01611dd9565b965050602087013567ffffffffffffffff811115611e87575f5ffd5b611e9389828a01611def565b909650945050604087013567ffffffffffffffff811115611eb2575f5ffd5b611ebe89828a01611def565b979a9699509497949695606090950135949350505050565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b604081525f611f166040830185611ed6565b905082151560208301529392505050565b5f60208284031215611f37575f5ffd5b81357fffffffff0000000000000000000000000000000000000000000000000000000081168114611455575f5ffd5b6001600160a01b0381168114610e29575f5ffd5b5f5f60408385031215611f8b575f5ffd5b823591506020830135611f9d81611f66565b809150509250929050565b5f60208284031215611fb8575f5ffd5b813567ffffffffffffffff811115611fce575f5ffd5b611fda84828501611dd9565b949350505050565b5f60208284031215611ff2575f5ffd5b5035919050565b8015158114610e29575f5ffd5b5f5f5f5f5f6080868803121561201a575f5ffd5b853567ffffffffffffffff811115612030575f5ffd5b61203c88828901611def565b909650945050602086013561205081611ff9565b925060408601359150606086013567ffffffffffffffff811115612072575f5ffd5b86016101008189031215612084575f5ffd5b809150509295509295909350565b5f602082840312156120a2575f5ffd5b813561145581611f66565b602081525f6114556020830184611ed6565b5f5f5f606084860312156120d1575f5ffd5b83356120dc81611f66565b92506020840135915060408401356120f381611f66565b809150509250925092565b803561210981611f66565b919050565b5f82357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff21833603018112612140575f5ffd5b9190910192915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b808201808211156106fc576106fc61214a565b5f6020828403121561219a575f5ffd5b815160ff81168114611455575f5ffd5b6001815b60018411156121e5578085048111156121c9576121c961214a565b60018416156121d757908102905b60019390931c9280026121ae565b935093915050565b5f826121fb575060016106fc565b8161220757505f6106fc565b816001811461221d576002811461222757612243565b60019150506106fc565b60ff8411156122385761223861214a565b50506001821b6106fc565b5060208310610133831016604e8410600b8410161715612266575081810a6106fc565b6122725f1984846121aa565b805f19048211156122855761228561214a565b029392505050565b5f61145583836121ed565b5f602082840312156122a8575f5ffd5b5051919050565b5f602082840312156122bf575f5ffd5b815161145581611ff9565b5f82357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff01833603018112612140575f5ffd5b5f5f83357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe184360301811261232f575f5ffd5b83018035915067ffffffffffffffff821115612349575f5ffd5b602001915036819003821315611e2d575f5ffd5b5f82518060208501845e5f920191825250919050565b5f5f60408385031215612384575f5ffd5b825161238f81611f66565b6020939093015192949293505050565b5f5f83357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe18436030181126123d2575f5ffd5b830160208101925035905067ffffffffffffffff8111156123f1575f5ffd5b803603821315611e2d575f5ffd5b81835281816020850137505f602082840101525f6020601f19601f840116840101905092915050565b5f82357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0183360301811261245a575f5ffd5b90910192915050565b8035825260208082013590830152604080820135908301525f606082013561248a81611f66565b6001600160a01b031660608401526124a4608083016120fe565b6001600160a01b031660808401526124be60a083016120fe565b6001600160a01b031660a08401526124d960c083018361239f565b61010060c08601526124f0610100860182846123ff565b60e0948501359590940194909452509092915050565b602081525f82357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff2184360301811261253c575f5ffd5b604060208401528301803561255081611f66565b6001600160a01b03166060840152602081013561256c81611f66565b6001600160a01b0316608084810191909152604082013560a080860191909152606083013560c08601529082013560e08501526125ab9082018261239f565b60e06101008601526125c2610140860182846123ff565b60c084013561012087015291506125de90506020860186612428565b9150601f198482030160408501526125f68183612463565b95945050505050565b5f6020828403121561260f575f5ffd5b815161145581611f66565b828152604060208201525f611fda6040830184612463565b818103818111156106fc576106fc61214a565b5f77ffffffffffffffffffffffffffffffffffffffffffffffff821677ffffffffffffffffffffffffffffffffffffffffffffffff841677ffffffffffffffffffffffffffffffffffffffffffffffff81830216925081830481148215176126af576126af61214a565b505092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f77ffffffffffffffffffffffffffffffffffffffffffffffff83168061270d5761270d6126b7565b8077ffffffffffffffffffffffffffffffffffffffffffffffff84160491505092915050565b80820281158282048414176106fc576106fc61214a565b5f82612758576127586126b7565b500490565b805169ffffffffffffffffffff81168114612109575f5ffd5b5f5f5f5f5f60a0868803121561278a575f5ffd5b6127938661275d565b602087015160408801516060890151929750909550935091506127b86080870161275d565b9050929550929590935056fe332e302e302d626574612e31302b6f70656e67736e2e6f7261636c652e746f6b656e2e697061796d6173746572a26469706673582212208ceb21cfc1616fc7596ba091f748e95c5ce7403c3f05538f3465a74cb367bd1064736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`@Qa)\xF68\x03\x80a)\xF6\x839\x81\x01`@\x81\x90Ra\0.\x91a\x01uV[a\x0073a\x01&V[\x80`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\0sW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\0\x97\x91\x90a\x01\xA2V[`\xFF\x16`\x08\x14a\x01\x01W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FOTP: native token oracle decimal`D\x82\x01Rj\x0Ed\r\xAE\xAEn\x84\x0CL\xA4\x07`\xAB\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[`\x03\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua\x01\xC2V[_\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[_` \x82\x84\x03\x12\x15a\x01\x85W__\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01\x9BW__\xFD[\x93\x92PPPV[_` \x82\x84\x03\x12\x15a\x01\xB2W__\xFD[\x81Q`\xFF\x81\x16\x81\x14a\x01\x9BW__\xFD[a('\x80a\x01\xCF_9_\xF3\xFE`\x80`@R`\x046\x10a\x01\x90W_5`\xE0\x1C\x80c\x9F\xBB\r\xCA\x11a\0\xDCW\x80c\xCDK\xC6\x11\x11a\0\x87W\x80c\xDFF:f\x11a\0bW\x80c\xDFF:f\x14a\x05iW\x80c\xF2\xFD\xE3\x8B\x14a\x05}W\x80c\xF9\xC0\x02\xF7\x14a\x05\x9CW\x80c\xFA\t\xE60\x14a\x05\xB2W__\xFD[\x80c\xCDK\xC6\x11\x14a\x05\x0EW\x80c\xCE\x1B\x81_\x14a\x05-W\x80c\xDAt\"(\x14a\x05JW__\xFD[\x80c\xB9\x0BA\xCF\x11a\0\xB7W\x80c\xB9\x0BA\xCF\x14a\x04\x8BW\x80c\xBB\xDA\xA3\xC9\x14a\x04\xA0W\x80c\xBDI$2\x14a\x04\xB6W__\xFD[\x80c\x9F\xBB\r\xCA\x14a\x04\x10W\x80c\xAD\x12\xE5\x0E\x14a\x04/W\x80c\xB09\xA8\x8F\x14a\x04DW__\xFD[\x80cqP\x18\xA6\x11a\x01<W\x80c{\xDF.\xC7\x11a\x01\x17W\x80c{\xDF.\xC7\x14a\x03\xB6W\x80c\x8D\xA5\xCB[\x14a\x03\xD3W\x80c\x92\x12v\xEA\x14a\x03\xEFW__\xFD[\x80cqP\x18\xA6\x14a\x03dW\x80cv\xFA\x01\xC3\x14a\x03xW\x80c{\xB0Rd\x14a\x03\x97W__\xFD[\x80cA\xBB\xB7\xCA\x11a\x01lW\x80cA\xBB\xB7\xCA\x14a\x02\xEBW\x80c\\^=\xB1\x14a\x03\"W\x80cm|>+\x14a\x03EW__\xFD[\x80b\xBE]\xD4\x14a\x02gW\x80c\x01\xFF\xC9\xA7\x14a\x02\x9DW\x80c-\x14\xC4\xB7\x14a\x02\xCCW__\xFD[6a\x02cW`\x01T`\x01`\x01`\xA0\x1B\x03\x16a\x01\xF2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7Frelay hub address not set\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\x01T`@Q\x7F\xAAg\xC9\x19\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xAAg\xC9\x19\x904\x90`$\x01_`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\x02OW__\xFD[PZ\xF1\x15\x80\x15a\x02aW=__>=_\xFD[\0[__\xFD[4\x80\x15a\x02rW__\xFD[Pa\x02\x86a\x02\x816`\x04a\x1E4V[a\x05\xD1V[`@Qa\x02\x94\x92\x91\x90a\x1F\x04V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\xA8W__\xFD[Pa\x02\xBCa\x02\xB76`\x04a\x1F'V[a\x06\x1EV[`@Q\x90\x15\x15\x81R` \x01a\x02\x94V[4\x80\x15a\x02\xD7W__\xFD[Pa\x02aa\x02\xE66`\x04a\x1FzV[a\x07\x02V[4\x80\x15a\x02\xF6W__\xFD[Pa\x03\na\x03\x056`\x04a\x1F\xA8V[a\x07\xD9V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\x94V[4\x80\x15a\x03-W__\xFD[Pa\x037a)\x04\x81V[`@Q\x90\x81R` \x01a\x02\x94V[4\x80\x15a\x03PW__\xFD[Pa\x02aa\x03_6`\x04a\x1F\xE2V[a\x07\xF2V[4\x80\x15a\x03oW__\xFD[Pa\x02aa\x08PV[4\x80\x15a\x03\x83W__\xFD[Pa\x02aa\x03\x926`\x04a \x06V[a\x08\xB4V[4\x80\x15a\x03\xA2W__\xFD[Pa\x02aa\x03\xB16`\x04a \x92V[a\x08\xD0V[4\x80\x15a\x03\xC1W__\xFD[P`\x01T`\x01`\x01`\xA0\x1B\x03\x16a\x03\nV[4\x80\x15a\x03\xDEW__\xFD[P_T`\x01`\x01`\xA0\x1B\x03\x16a\x03\nV[4\x80\x15a\x03\xFAW__\xFD[Pa\x04\x03a\t\xE2V[`@Qa\x02\x94\x91\x90a \xADV[4\x80\x15a\x04\x1BW__\xFD[Pa\x02aa\x04*6`\x04a\x1E4V[a\n\x02V[4\x80\x15a\x04:W__\xFD[Pa\x037`\x05T\x81V[4\x80\x15a\x04OW__\xFD[Pa\x04Xa\n\x1AV[`@Qa\x02\x94\x91\x90\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x80\x83\x01Q\x90\x82\x01R``\x91\x82\x01Q\x91\x81\x01\x91\x90\x91R`\x80\x01\x90V[4\x80\x15a\x04\x96W__\xFD[Pa\x037a\xC3P\x81V[4\x80\x15a\x04\xABW__\xFD[Pa\x037b\x01\xAD\xB0\x81V[4\x80\x15a\x04\xC1W__\xFD[Pa\x04\xF1a\x04\xD06`\x04a \x92V[`\x04` R_\x90\x81R`@\x90 \x80T`\x01\x90\x91\x01T`\x01`\x01`\xA0\x1B\x03\x16\x82V[`@\x80Q\x92\x83R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16` \x83\x01R\x01a\x02\x94V[4\x80\x15a\x05\x19W__\xFD[Pa\x02aa\x05(6`\x04a \xBFV[a\n\x7FV[4\x80\x15a\x058W__\xFD[P`\x02T`\x01`\x01`\xA0\x1B\x03\x16a\x03\nV[4\x80\x15a\x05UW__\xFD[Pa\x02aa\x05d6`\x04a \x92V[a\x0C'V[4\x80\x15a\x05tW__\xFD[Pa\x037a\r9V[4\x80\x15a\x05\x88W__\xFD[Pa\x02aa\x05\x976`\x04a \x92V[a\rKV[4\x80\x15a\x05\xA7W__\xFD[Pa\x037b\x01\x86\xA0\x81V[4\x80\x15a\x05\xBDW__\xFD[Pa\x02aa\x05\xCC6`\x04a \x92V[a\x0E,V[``_a\x05\xDCa\x0F\x95V[a\x05\xE5\x88a\x0F\xEFV[a\x05\xEE\x88a\x10\x87V[a\x05\xF7\x88a\x10\xE2V[a\x06\x01\x85\x85a\x11NV[a\x06\x0F\x88\x88\x88\x88\x88\x88a\x11\xA0V[\x91P\x91P\x96P\x96\x94PPPPPV[_\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x7F\xE1\xAB-\xEA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x14\x80a\x06\xB0WP\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x7F\x0E\x080v\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x14[\x80a\x06\xFCWP\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x14[\x92\x91PPV[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x07[W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x01\xE9V[`\x01T`@Q\x7F\xF3\xFE\xF3\xA3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R`$\x82\x01\x85\x90R\x90\x91\x16\x90c\xF3\xFE\xF3\xA3\x90`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x07\xBFW__\xFD[PZ\xF1\x15\x80\x15a\x07\xD1W=__>=_\xFD[PPPPPPV[_a\x07\xE4\x82\x80a!\x0EV[a\x06\xFC\x90` \x81\x01\x90a \x92V[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x08KW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x01\xE9V[`\x05UV[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x08\xA9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x01\xE9V[a\x08\xB2_a\x13\xB0V[V[a\x08\xBCa\x0F\x95V[a\x08\xC9\x85\x85\x85\x85\x85a\x14\x17V[PPPPPV[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\t)W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x01\xE9V[a\t\\`\x01`\x01`\xA0\x1B\x03\x82\x16\x7F\xE9\xFB0\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x14:V[a\t\xA8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7Ftarget is not a valid IRelayHub\0`D\x82\x01R`d\x01a\x01\xE9V[`\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[```@Q\x80``\x01`@R\x80`-\x81R` \x01a'\xC5`-\x919\x90P\x90V[a\n\x10\x86\x86\x86\x86\x86\x86a\x11\xA0V[PPPPPPPPV[a\nA`@Q\x80`\x80\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80`\x80\x01`@R\x80a\xC3Pb\x01\x86\xA0a\n]\x91\x90a!wV[\x81R` \x01b\x01\x86\xA0\x81R` \x01b\x01\xAD\xB0\x81R` \x01a)\x04\x81RP\x90P\x90V[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\n\xD8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x01\xE9V[\x80`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\x14W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B8\x91\x90a!\x8AV[`\xFF\x16`\x08\x14a\x0B\xAFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FOTP: token oracle decimals must `D\x82\x01R\x7Fbe 8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x01\xE9V[`@Q\x80`@\x01`@R\x80\x83`\na\x0B\xC7\x91\x90a\"\x8DV[\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16` \x91\x82\x01R\x93\x82\x16_\x90\x81R`\x04\x85R`@\x90 \x81Q\x81U\x93\x01Q`\x01\x90\x93\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x93\x90\x91\x16\x92\x90\x92\x17\x90\x91UPV[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0C\x80W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x01\xE9V[a\x0C\xB3`\x01`\x01`\xA0\x1B\x03\x82\x16\x7F%\xE2>d\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x14:V[a\x0C\xFFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7Ftarget is not a valid IForwarder`D\x82\x01R`d\x01a\x01\xE9V[`\x02\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[a\rHa\xC3Pb\x01\x86\xA0a!wV[\x81V[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\r\xA4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x01\xE9V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x0E W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x01\xE9V[a\x0E)\x81a\x13\xB0V[PV[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0E\x85W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x01\xE9V[`@Q\x7Fp\xA0\x821\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R0`\x04\x82\x01R_\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\xE2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\x06\x91\x90a\"\x98V[`@Q\x7F\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R3`\x04\x82\x01R`$\x81\x01\x82\x90R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x0FlW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\x90\x91\x90a\"\xAFV[PPPV[`\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x08\xB2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7Fcan only be called by RelayHub\0\0`D\x82\x01R`d\x01a\x01\xE9V[a\x0F\xFC` \x82\x01\x82a\"\xCAV[a\x10\r\x90`\xC0\x81\x01\x90`\xA0\x01a \x92V[`\x01`\x01`\xA0\x1B\x03\x16a\x10(`\x02T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x10~W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FForwarder is not trusted\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x01\xE9V[a\x0E)\x81a\x14\\V[a\x10\x91\x81\x80a!\x0EV[`@\x015\x15a\x0E)W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7Fvalue transfer not supported\0\0\0\0`D\x82\x01R`d\x01a\x01\xE9V[a\x10\xEF` \x82\x01\x82a\"\xCAV[a\x10\xFD\x90`\xC0\x81\x01\x90a\"\xFCV[\x90P`@\x14a\x0E)W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FpaymasterData: invalid length\0\0\0`D\x82\x01R`d\x01a\x01\xE9V[\x80\x15a\x11\x9CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7Fshould have no approvalData\0\0\0\0\0`D\x82\x01R`d\x01a\x01\xE9V[PPV[``_\x80\x80a\x11\xFCa\x11\xB5` \x8C\x01\x8Ca\"\xCAV[a\x11\xC3\x90`\xC0\x81\x01\x90a\"\xFCV[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPa\x16\x82\x92PPPV[\x91P\x91P___a\x12\x0E\x85\x8E\x8Aa\x16\xA6V[\x92P\x92P\x92P\x83\x81\x11\x15a\x12\x8AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`)`$\x82\x01R\x7FTx cost more than the user-suppl`D\x82\x01R\x7Fied limit\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x01\xE9V[`@Q\x7F#\xB8r\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R0`$\x83\x01R`D\x82\x01\x83\x90R\x86\x16\x90c#\xB8r\xDD\x90`d\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x12\xF5W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\x19\x91\x90a\"\xAFV[P`@\x80Q\x83\x81R`\x01`\x01`\xA0\x1B\x03\x87\x81\x16` \x83\x01R\x91\x81\x01\x83\x90R\x90\x84\x16\x90\x7F\xC5xN\xFE\xFB\x9E<;\xDE\xCE\xC2\x1C\xDDa\xE4 \xE9hX\xD8\xF1\xAA\xAAm\xBB\xE3\x0B\x05 u\xE1\x99\x90``\x01`@Q\x80\x91\x03\x90\xA2`@\x80Q`\x01`\x01`\xA0\x1B\x03\x80\x86\x16` \x83\x01R\x91\x81\x01\x83\x90R\x90\x86\x16``\x82\x01R`\x80\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x9D_\x9DP\x9BPPPPPPPPPPPPV[_\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[_\x80\x80a\x14&\x87\x89\x01\x89a \xBFV[\x92P\x92P\x92Pa\n\x10\x83\x83_\x88\x88\x86a\x17\xDCV[_a\x14D\x83a\x19\xB3V[\x80\x15a\x14UWPa\x14U\x83\x83a\x1A\x16V[\x93\x92PPPV[_\x80a\x14h\x83\x80a!\x0EV[a\x14y\x90`@\x81\x01\x90` \x01a \x92V[`\x01`\x01`\xA0\x1B\x03\x16\x7FW+l\x05\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x14\xB0` \x86\x01\x86a\"\xCAV[a\x14\xC1\x90`\xC0\x81\x01\x90`\xA0\x01a \x92V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`$\x82\x01R`D\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x94\x16\x93\x90\x93\x17\x90\x92R\x90Qa\x15B\x91\x90a#]V[_`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80_\x81\x14a\x15zW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x15\x7FV[``\x91P[P\x91P\x91P\x81a\x15\xD1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FisTrustedForwarder: reverted\0\0\0\0`D\x82\x01R`d\x01a\x01\xE9V[\x80Q` \x14a\x16\"W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FisTrustedForwarder: bad response`D\x82\x01R`d\x01a\x01\xE9V[\x80\x80` \x01\x90Q\x81\x01\x90a\x166\x91\x90a\"\xAFV[a\x0F\x90W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7Finvalid forwarder for recipient\0`D\x82\x01R`d\x01a\x01\xE9V[____\x84\x80` \x01\x90Q\x81\x01\x90a\x16\x9A\x91\x90a#sV[\x90\x96\x90\x95P\x93PPPPV[`@Q\x7FA\xBB\xB7\xCA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R_\x90\x81\x90\x81\x900\x90cA\xBB\xB7\xCA\x90a\x16\xE8\x90\x88\x90`\x04\x01a%\x06V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17\x03W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17'\x91\x90a%\xFFV[`\x01T\x90\x93P_\x90`\x01`\x01`\xA0\x1B\x03\x16c\x8EST\x8B\x86a\x17K` \x8A\x01\x8Aa\"\xCAV[`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x17h\x92\x91\x90a&\x1AV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17\x83W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\xA7\x91\x90a\"\x98V[\x90Pa\x17\xB3\x86\x80a!\x0EV[a\x17\xC1\x90`@\x015\x82a!wV[\x90Pa\x17\xCD\x87\x82a\x1B@V[\x91P\x80\x92PP\x93P\x93P\x93\x90PV[`\x01T`\x05T_\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\x8EST\x8B\x90a\x17\xFE\x90\x87a!wV[\x85`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x18\x1C\x92\x91\x90a&\x1AV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x187W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18[\x91\x90a\"\x98V[\x90P_a\x18q\x83a\x18l\x84\x89a!wV[a\x1B@V[\x90P_a\x18~\x82\x89a&2V[`@\x80Q\x85\x81R`\x01`\x01`\xA0\x1B\x03\x80\x88\x16` \x83\x01R\x91\x81\x01\x85\x90R\x90\x8B\x16``\x82\x01R\x90\x91P\x7F\xDBN(\x86\xD8~\x89*\xBF>\x8DBwe;\xA2\xA2l\xB0\x13\x07\x80A]4\x134\xBE\xBF^\xDC\xCA\x90`\x80\x01`@Q\x80\x91\x03\x90\xA1`@Q\x7F\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x8A\x81\x16`\x04\x83\x01R`$\x82\x01\x83\x90R\x85\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x198W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19\\\x91\x90a\"\xAFV[a\x19\xA8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01R\x7Ffailed refund\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x01\xE9V[PPPPPPPPPV[_a\x19\xDE\x82\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1A\x16V[\x80\x15a\x06\xFCWPa\x1A\x0F\x82\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1A\x16V[\x15\x92\x91PPV[`@\x80Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16`$\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`D\x90\x91\x01\x82R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90R\x90Q_\x91\x90\x82\x90\x81\x90`\x01`\x01`\xA0\x1B\x03\x87\x16\x90au0\x90a\x1A\xC2\x90\x86\x90a#]V[_`@Q\x80\x83\x03\x81\x86\x86\xFA\x92PPP=\x80_\x81\x14a\x1A\xFBW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x1B\0V[``\x91P[P\x91P\x91P` \x81Q\x10\x15a\x1B\x1AW_\x93PPPPa\x06\xFCV[\x81\x80\x15a\x1B6WP\x80\x80` \x01\x90Q\x81\x01\x90a\x1B6\x91\x90a\"\xAFV[\x96\x95PPPPPPV[`\x01`\x01`\xA0\x1B\x03\x80\x83\x16_\x90\x81R`\x04` \x90\x81R`@\x80\x83 \x81Q\x80\x83\x01\x90\x92R\x80T\x82R`\x01\x01T\x90\x93\x16\x90\x83\x01\x81\x90R\x90\x91\x90a\x1B\xC3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FOTP: Oracle does not exist\0\0\0\0\0\0`D\x82\x01R`d\x01a\x01\xE9V[_a\x1B\xD1\x82` \x01Qa\x1CNV[`\x03T\x90\x91P_\x90a\x1B\xEB\x90`\x01`\x01`\xA0\x1B\x03\x16a\x1CNV[\x90P_\x82\x84_\x01Q\x83a\x1B\xFE\x91\x90a&EV[a\x1C\x08\x91\x90a&\xE4V[\x90P_g\r\xE0\xB6\xB3\xA7d\0\0a\x1C8w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x89a'3V[a\x1CB\x91\x90a'JV[\x98\x97PPPPPPPPV[_____\x85`\x01`\x01`\xA0\x1B\x03\x16c\xFE\xAF\x96\x8C`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1C\x8FW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C\xB3\x91\x90a'vV[\x94P\x94PP\x93P\x93P_\x83\x13a\x1D\x0BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FOTP : Chainlink price <= 0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x01\xE9V[a\x1D\x18b\x02\xA3\0Ba&2V[\x82\x10\x15a\x1DgW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01R\x7FOTP : Incomplete round\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x01\xE9V[\x83i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15a\x1D\xCFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01R\x7FOTP : Stale price\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x01\xE9V[P\x90\x94\x93PPPPV[_`@\x82\x84\x03\x12\x15a\x1D\xE9W__\xFD[P\x91\x90PV[__\x83`\x1F\x84\x01\x12a\x1D\xFFW__\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1E\x16W__\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x1E-W__\xFD[\x92P\x92\x90PV[______`\x80\x87\x89\x03\x12\x15a\x1EIW__\xFD[\x865g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1E_W__\xFD[a\x1Ek\x89\x82\x8A\x01a\x1D\xD9V[\x96PP` \x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1E\x87W__\xFD[a\x1E\x93\x89\x82\x8A\x01a\x1D\xEFV[\x90\x96P\x94PP`@\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1E\xB2W__\xFD[a\x1E\xBE\x89\x82\x8A\x01a\x1D\xEFV[\x97\x9A\x96\x99P\x94\x97\x94\x96\x95``\x90\x95\x015\x94\x93PPPPV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[`@\x81R_a\x1F\x16`@\x83\x01\x85a\x1E\xD6V[\x90P\x82\x15\x15` \x83\x01R\x93\x92PPPV[_` \x82\x84\x03\x12\x15a\x1F7W__\xFD[\x815\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x81\x14a\x14UW__\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0E)W__\xFD[__`@\x83\x85\x03\x12\x15a\x1F\x8BW__\xFD[\x825\x91P` \x83\x015a\x1F\x9D\x81a\x1FfV[\x80\x91PP\x92P\x92\x90PV[_` \x82\x84\x03\x12\x15a\x1F\xB8W__\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1F\xCEW__\xFD[a\x1F\xDA\x84\x82\x85\x01a\x1D\xD9V[\x94\x93PPPPV[_` \x82\x84\x03\x12\x15a\x1F\xF2W__\xFD[P5\x91\x90PV[\x80\x15\x15\x81\x14a\x0E)W__\xFD[_____`\x80\x86\x88\x03\x12\x15a \x1AW__\xFD[\x855g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a 0W__\xFD[a <\x88\x82\x89\x01a\x1D\xEFV[\x90\x96P\x94PP` \x86\x015a P\x81a\x1F\xF9V[\x92P`@\x86\x015\x91P``\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a rW__\xFD[\x86\x01a\x01\0\x81\x89\x03\x12\x15a \x84W__\xFD[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[_` \x82\x84\x03\x12\x15a \xA2W__\xFD[\x815a\x14U\x81a\x1FfV[` \x81R_a\x14U` \x83\x01\x84a\x1E\xD6V[___``\x84\x86\x03\x12\x15a \xD1W__\xFD[\x835a \xDC\x81a\x1FfV[\x92P` \x84\x015\x91P`@\x84\x015a \xF3\x81a\x1FfV[\x80\x91PP\x92P\x92P\x92V[\x805a!\t\x81a\x1FfV[\x91\x90PV[_\x825\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF!\x836\x03\x01\x81\x12a!@W__\xFD[\x91\x90\x91\x01\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\x06\xFCWa\x06\xFCa!JV[_` \x82\x84\x03\x12\x15a!\x9AW__\xFD[\x81Q`\xFF\x81\x16\x81\x14a\x14UW__\xFD[`\x01\x81[`\x01\x84\x11\x15a!\xE5W\x80\x85\x04\x81\x11\x15a!\xC9Wa!\xC9a!JV[`\x01\x84\x16\x15a!\xD7W\x90\x81\x02\x90[`\x01\x93\x90\x93\x1C\x92\x80\x02a!\xAEV[\x93P\x93\x91PPV[_\x82a!\xFBWP`\x01a\x06\xFCV[\x81a\"\x07WP_a\x06\xFCV[\x81`\x01\x81\x14a\"\x1DW`\x02\x81\x14a\"'Wa\"CV[`\x01\x91PPa\x06\xFCV[`\xFF\x84\x11\x15a\"8Wa\"8a!JV[PP`\x01\x82\x1Ba\x06\xFCV[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a\"fWP\x81\x81\na\x06\xFCV[a\"r_\x19\x84\x84a!\xAAV[\x80_\x19\x04\x82\x11\x15a\"\x85Wa\"\x85a!JV[\x02\x93\x92PPPV[_a\x14U\x83\x83a!\xEDV[_` \x82\x84\x03\x12\x15a\"\xA8W__\xFD[PQ\x91\x90PV[_` \x82\x84\x03\x12\x15a\"\xBFW__\xFD[\x81Qa\x14U\x81a\x1F\xF9V[_\x825\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x01\x836\x03\x01\x81\x12a!@W__\xFD[__\x835\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x846\x03\x01\x81\x12a#/W__\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a#IW__\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x1E-W__\xFD[_\x82Q\x80` \x85\x01\x84^_\x92\x01\x91\x82RP\x91\x90PV[__`@\x83\x85\x03\x12\x15a#\x84W__\xFD[\x82Qa#\x8F\x81a\x1FfV[` \x93\x90\x93\x01Q\x92\x94\x92\x93PPPV[__\x835\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x846\x03\x01\x81\x12a#\xD2W__\xFD[\x83\x01` \x81\x01\x92P5\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a#\xF1W__\xFD[\x806\x03\x82\x13\x15a\x1E-W__\xFD[\x81\x83R\x81\x81` \x85\x017P_` \x82\x84\x01\x01R_` `\x1F\x19`\x1F\x84\x01\x16\x84\x01\x01\x90P\x92\x91PPV[_\x825\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x01\x836\x03\x01\x81\x12a$ZW__\xFD[\x90\x91\x01\x92\x91PPV[\x805\x82R` \x80\x82\x015\x90\x83\x01R`@\x80\x82\x015\x90\x83\x01R_``\x82\x015a$\x8A\x81a\x1FfV[`\x01`\x01`\xA0\x1B\x03\x16``\x84\x01Ra$\xA4`\x80\x83\x01a \xFEV[`\x01`\x01`\xA0\x1B\x03\x16`\x80\x84\x01Ra$\xBE`\xA0\x83\x01a \xFEV[`\x01`\x01`\xA0\x1B\x03\x16`\xA0\x84\x01Ra$\xD9`\xC0\x83\x01\x83a#\x9FV[a\x01\0`\xC0\x86\x01Ra$\xF0a\x01\0\x86\x01\x82\x84a#\xFFV[`\xE0\x94\x85\x015\x95\x90\x94\x01\x94\x90\x94RP\x90\x92\x91PPV[` \x81R_\x825\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF!\x846\x03\x01\x81\x12a%<W__\xFD[`@` \x84\x01R\x83\x01\x805a%P\x81a\x1FfV[`\x01`\x01`\xA0\x1B\x03\x16``\x84\x01R` \x81\x015a%l\x81a\x1FfV[`\x01`\x01`\xA0\x1B\x03\x16`\x80\x84\x81\x01\x91\x90\x91R`@\x82\x015`\xA0\x80\x86\x01\x91\x90\x91R``\x83\x015`\xC0\x86\x01R\x90\x82\x015`\xE0\x85\x01Ra%\xAB\x90\x82\x01\x82a#\x9FV[`\xE0a\x01\0\x86\x01Ra%\xC2a\x01@\x86\x01\x82\x84a#\xFFV[`\xC0\x84\x015a\x01 \x87\x01R\x91Pa%\xDE\x90P` \x86\x01\x86a$(V[\x91P`\x1F\x19\x84\x82\x03\x01`@\x85\x01Ra%\xF6\x81\x83a$cV[\x95\x94PPPPPV[_` \x82\x84\x03\x12\x15a&\x0FW__\xFD[\x81Qa\x14U\x81a\x1FfV[\x82\x81R`@` \x82\x01R_a\x1F\xDA`@\x83\x01\x84a$cV[\x81\x81\x03\x81\x81\x11\x15a\x06\xFCWa\x06\xFCa!JV[_w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x83\x02\x16\x92P\x81\x83\x04\x81\x14\x82\x15\x17a&\xAFWa&\xAFa!JV[PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x80a'\rWa'\ra&\xB7V[\x80w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x04\x91PP\x92\x91PPV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x06\xFCWa\x06\xFCa!JV[_\x82a'XWa'Xa&\xB7V[P\x04\x90V[\x80Qi\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a!\tW__\xFD[_____`\xA0\x86\x88\x03\x12\x15a'\x8AW__\xFD[a'\x93\x86a']V[` \x87\x01Q`@\x88\x01Q``\x89\x01Q\x92\x97P\x90\x95P\x93P\x91Pa'\xB8`\x80\x87\x01a']V[\x90P\x92\x95P\x92\x95\x90\x93PV\xFE3.0.0-beta.10+opengsn.oracle.token.ipaymaster\xA2dipfsX\"\x12 \x8C\xEB!\xCF\xC1ao\xC7Yk\xA0\x91\xF7H\xE9\\\\\xE7@<?\x05S\x8F4e\xA7L\xB3g\xBD\x10dsolcC\0\x08\x1B\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405260043610610190575f3560e01c80639fbb0dca116100dc578063cd4bc61111610087578063df463a6611610062578063df463a6614610569578063f2fde38b1461057d578063f9c002f71461059c578063fa09e630146105b2575f5ffd5b8063cd4bc6111461050e578063ce1b815f1461052d578063da7422281461054a575f5ffd5b8063b90b41cf116100b7578063b90b41cf1461048b578063bbdaa3c9146104a0578063bd492432146104b6575f5ffd5b80639fbb0dca14610410578063ad12e50e1461042f578063b039a88f14610444575f5ffd5b8063715018a61161013c5780637bdf2ec7116101175780637bdf2ec7146103b65780638da5cb5b146103d3578063921276ea146103ef575f5ffd5b8063715018a61461036457806376fa01c3146103785780637bb0526414610397575f5ffd5b806341bbb7ca1161016c57806341bbb7ca146102eb5780635c5e3db1146103225780636d7c3e2b14610345575f5ffd5b8062be5dd41461026757806301ffc9a71461029d5780632d14c4b7146102cc575f5ffd5b36610263576001546001600160a01b03166101f25760405162461bcd60e51b815260206004820152601960248201527f72656c6179206875622061646472657373206e6f74207365740000000000000060448201526064015b60405180910390fd5b6001546040517faa67c9190000000000000000000000000000000000000000000000000000000081523060048201526001600160a01b039091169063aa67c9199034906024015f604051808303818588803b15801561024f575f5ffd5b505af1158015610261573d5f5f3e3d5ffd5b005b5f5ffd5b348015610272575f5ffd5b50610286610281366004611e34565b6105d1565b604051610294929190611f04565b60405180910390f35b3480156102a8575f5ffd5b506102bc6102b7366004611f27565b61061e565b6040519015158152602001610294565b3480156102d7575f5ffd5b506102616102e6366004611f7a565b610702565b3480156102f6575f5ffd5b5061030a610305366004611fa8565b6107d9565b6040516001600160a01b039091168152602001610294565b34801561032d575f5ffd5b5061033761290481565b604051908152602001610294565b348015610350575f5ffd5b5061026161035f366004611fe2565b6107f2565b34801561036f575f5ffd5b50610261610850565b348015610383575f5ffd5b50610261610392366004612006565b6108b4565b3480156103a2575f5ffd5b506102616103b1366004612092565b6108d0565b3480156103c1575f5ffd5b506001546001600160a01b031661030a565b3480156103de575f5ffd5b505f546001600160a01b031661030a565b3480156103fa575f5ffd5b506104036109e2565b60405161029491906120ad565b34801561041b575f5ffd5b5061026161042a366004611e34565b610a02565b34801561043a575f5ffd5b5061033760055481565b34801561044f575f5ffd5b50610458610a1a565b60405161029491908151815260208083015190820152604080830151908201526060918201519181019190915260800190565b348015610496575f5ffd5b5061033761c35081565b3480156104ab575f5ffd5b506103376201adb081565b3480156104c1575f5ffd5b506104f16104d0366004612092565b60046020525f9081526040902080546001909101546001600160a01b031682565b604080519283526001600160a01b03909116602083015201610294565b348015610519575f5ffd5b506102616105283660046120bf565b610a7f565b348015610538575f5ffd5b506002546001600160a01b031661030a565b348015610555575f5ffd5b50610261610564366004612092565b610c27565b348015610574575f5ffd5b50610337610d39565b348015610588575f5ffd5b50610261610597366004612092565b610d4b565b3480156105a7575f5ffd5b50610337620186a081565b3480156105bd575f5ffd5b506102616105cc366004612092565b610e2c565b60605f6105dc610f95565b6105e588610fef565b6105ee88611087565b6105f7886110e2565b610601858561114e565b61060f8888888888886111a0565b91509150965096945050505050565b5f7fffffffff0000000000000000000000000000000000000000000000000000000082167fe1ab2dea0000000000000000000000000000000000000000000000000000000014806106b057507fffffffff0000000000000000000000000000000000000000000000000000000082167f0e08307600000000000000000000000000000000000000000000000000000000145b806106fc57507f01ffc9a7000000000000000000000000000000000000000000000000000000007fffffffff000000000000000000000000000000000000000000000000000000008316145b92915050565b5f546001600160a01b0316331461075b5760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e657260448201526064016101e9565b6001546040517ff3fef3a30000000000000000000000000000000000000000000000000000000081526001600160a01b038381166004830152602482018590529091169063f3fef3a3906044015f604051808303815f87803b1580156107bf575f5ffd5b505af11580156107d1573d5f5f3e3d5ffd5b505050505050565b5f6107e4828061210e565b6106fc906020810190612092565b5f546001600160a01b0316331461084b5760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e657260448201526064016101e9565b600555565b5f546001600160a01b031633146108a95760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e657260448201526064016101e9565b6108b25f6113b0565b565b6108bc610f95565b6108c98585858585611417565b5050505050565b5f546001600160a01b031633146109295760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e657260448201526064016101e9565b61095c6001600160a01b0382167fe9fb30f70000000000000000000000000000000000000000000000000000000061143a565b6109a85760405162461bcd60e51b815260206004820152601f60248201527f746172676574206973206e6f7420612076616c6964204952656c61794875620060448201526064016101e9565b600180547fffffffffffffffffffffffff0000000000000000000000000000000000000000166001600160a01b0392909216919091179055565b60606040518060600160405280602d81526020016127c5602d9139905090565b610a108686868686866111a0565b5050505050505050565b610a4160405180608001604052805f81526020015f81526020015f81526020015f81525090565b604051806080016040528061c350620186a0610a5d9190612177565b8152602001620186a081526020016201adb08152602001612904815250905090565b5f546001600160a01b03163314610ad85760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e657260448201526064016101e9565b806001600160a01b031663313ce5676040518163ffffffff1660e01b8152600401602060405180830381865afa158015610b14573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610b38919061218a565b60ff16600814610baf5760405162461bcd60e51b8152602060048201526024808201527f4f54503a20746f6b656e206f7261636c6520646563696d616c73206d7573742060448201527f626520380000000000000000000000000000000000000000000000000000000060648201526084016101e9565b604051806040016040528083600a610bc7919061228d565b81526001600160a01b039283166020918201529382165f908152600485526040902081518155930151600190930180547fffffffffffffffffffffffff000000000000000000000000000000000000000016939091169290921790915550565b5f546001600160a01b03163314610c805760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e657260448201526064016101e9565b610cb36001600160a01b0382167f25e23e640000000000000000000000000000000000000000000000000000000061143a565b610cff5760405162461bcd60e51b815260206004820181905260248201527f746172676574206973206e6f7420612076616c69642049466f7277617264657260448201526064016101e9565b600280547fffffffffffffffffffffffff0000000000000000000000000000000000000000166001600160a01b0392909216919091179055565b610d4861c350620186a0612177565b81565b5f546001600160a01b03163314610da45760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e657260448201526064016101e9565b6001600160a01b038116610e205760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201527f646472657373000000000000000000000000000000000000000000000000000060648201526084016101e9565b610e29816113b0565b50565b5f546001600160a01b03163314610e855760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e657260448201526064016101e9565b6040517f70a082310000000000000000000000000000000000000000000000000000000081523060048201525f906001600160a01b038316906370a0823190602401602060405180830381865afa158015610ee2573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610f069190612298565b6040517fa9059cbb000000000000000000000000000000000000000000000000000000008152336004820152602481018290529091506001600160a01b0383169063a9059cbb906044016020604051808303815f875af1158015610f6c573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610f9091906122af565b505050565b6001546001600160a01b031633146108b25760405162461bcd60e51b815260206004820152601e60248201527f63616e206f6e6c792062652063616c6c65642062792052656c6179487562000060448201526064016101e9565b610ffc60208201826122ca565b61100d9060c081019060a001612092565b6001600160a01b03166110286002546001600160a01b031690565b6001600160a01b03161461107e5760405162461bcd60e51b815260206004820152601860248201527f466f72776172646572206973206e6f742074727573746564000000000000000060448201526064016101e9565b610e298161145c565b611091818061210e565b6040013515610e295760405162461bcd60e51b815260206004820152601c60248201527f76616c7565207472616e73666572206e6f7420737570706f727465640000000060448201526064016101e9565b6110ef60208201826122ca565b6110fd9060c08101906122fc565b9050604014610e295760405162461bcd60e51b815260206004820152601d60248201527f7061796d6173746572446174613a20696e76616c6964206c656e67746800000060448201526064016101e9565b801561119c5760405162461bcd60e51b815260206004820152601b60248201527f73686f756c642068617665206e6f20617070726f76616c44617461000000000060448201526064016101e9565b5050565b60605f80806111fc6111b560208c018c6122ca565b6111c39060c08101906122fc565b8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f9201919091525061168292505050565b915091505f5f5f61120e858e8a6116a6565b9250925092508381111561128a5760405162461bcd60e51b815260206004820152602960248201527f547820636f7374206d6f7265207468616e2074686520757365722d737570706c60448201527f696564206c696d6974000000000000000000000000000000000000000000000060648201526084016101e9565b6040517f23b872dd0000000000000000000000000000000000000000000000000000000081526001600160a01b038481166004830152306024830152604482018390528616906323b872dd906064016020604051808303815f875af11580156112f5573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061131991906122af565b50604080518381526001600160a01b038781166020830152918101839052908416907fc5784efefb9e3c3bdecec21cdd61e420e96858d8f1aaaa6dbbe30b052075e1999060600160405180910390a2604080516001600160a01b038086166020830152918101839052908616606082015260800160408051601f198184030181529190529d5f9d509b505050505050505050505050565b5f80546001600160a01b038381167fffffffffffffffffffffffff0000000000000000000000000000000000000000831681178455604051919092169283917f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09190a35050565b5f8080611426878901896120bf565b925092509250610a1083835f8888866117dc565b5f611444836119b3565b801561145557506114558383611a16565b9392505050565b5f80611468838061210e565b611479906040810190602001612092565b6001600160a01b03167f572b6c05000000000000000000000000000000000000000000000000000000006114b060208601866122ca565b6114c19060c081019060a001612092565b6040516001600160a01b03909116602482015260440160408051601f198184030181529181526020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167fffffffff00000000000000000000000000000000000000000000000000000000909416939093179092529051611542919061235d565b5f60405180830381855afa9150503d805f811461157a576040519150601f19603f3d011682016040523d82523d5f602084013e61157f565b606091505b5091509150816115d15760405162461bcd60e51b815260206004820152601c60248201527f697354727573746564466f727761726465723a2072657665727465640000000060448201526064016101e9565b80516020146116225760405162461bcd60e51b815260206004820181905260248201527f697354727573746564466f727761726465723a2062616420726573706f6e736560448201526064016101e9565b8080602001905181019061163691906122af565b610f905760405162461bcd60e51b815260206004820152601f60248201527f696e76616c696420666f7277617264657220666f7220726563697069656e740060448201526064016101e9565b5f5f5f5f8480602001905181019061169a9190612373565b90969095509350505050565b6040517f41bbb7ca0000000000000000000000000000000000000000000000000000000081525f908190819030906341bbb7ca906116e8908890600401612506565b602060405180830381865afa158015611703573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061172791906125ff565b6001549093505f906001600160a01b0316638e53548b8661174b60208a018a6122ca565b6040518363ffffffff1660e01b815260040161176892919061261a565b602060405180830381865afa158015611783573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906117a79190612298565b90506117b3868061210e565b6117c1906040013582612177565b90506117cd8782611b40565b91508092505093509350939050565b6001546005545f916001600160a01b031690638e53548b906117fe9087612177565b856040518363ffffffff1660e01b815260040161181c92919061261a565b602060405180830381865afa158015611837573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061185b9190612298565b90505f6118718361186c8489612177565b611b40565b90505f61187e8289612632565b604080518581526001600160a01b038088166020830152918101859052908b1660608201529091507fdb4e2886d87e892abf3e8d4277653ba2a26cb0130780415d341334bebf5edcca9060800160405180910390a16040517fa9059cbb0000000000000000000000000000000000000000000000000000000081526001600160a01b038a811660048301526024820183905285169063a9059cbb906044016020604051808303815f875af1158015611938573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061195c91906122af565b6119a85760405162461bcd60e51b815260206004820152600d60248201527f6661696c656420726566756e640000000000000000000000000000000000000060448201526064016101e9565b505050505050505050565b5f6119de827f01ffc9a700000000000000000000000000000000000000000000000000000000611a16565b80156106fc5750611a0f827fffffffff00000000000000000000000000000000000000000000000000000000611a16565b1592915050565b604080517fffffffff00000000000000000000000000000000000000000000000000000000831660248083019190915282518083039091018152604490910182526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f01ffc9a70000000000000000000000000000000000000000000000000000000017905290515f9190829081906001600160a01b0387169061753090611ac290869061235d565b5f604051808303818686fa925050503d805f8114611afb576040519150601f19603f3d011682016040523d82523d5f602084013e611b00565b606091505b5091509150602081511015611b1a575f93505050506106fc565b818015611b36575080806020019051810190611b3691906122af565b9695505050505050565b6001600160a01b038083165f90815260046020908152604080832081518083019092528054825260010154909316908301819052909190611bc35760405162461bcd60e51b815260206004820152601a60248201527f4f54503a204f7261636c6520646f6573206e6f7420657869737400000000000060448201526064016101e9565b5f611bd18260200151611c4e565b6003549091505f90611beb906001600160a01b0316611c4e565b90505f82845f015183611bfe9190612645565b611c0891906126e4565b90505f670de0b6b3a7640000611c3877ffffffffffffffffffffffffffffffffffffffffffffffff841689612733565b611c42919061274a565b98975050505050505050565b5f5f5f5f5f856001600160a01b031663feaf968c6040518163ffffffff1660e01b815260040160a060405180830381865afa158015611c8f573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611cb39190612776565b9450945050935093505f8313611d0b5760405162461bcd60e51b815260206004820152601a60248201527f4f5450203a20436861696e6c696e6b207072696365203c3d203000000000000060448201526064016101e9565b611d186202a30042612632565b821015611d675760405162461bcd60e51b815260206004820152601660248201527f4f5450203a20496e636f6d706c65746520726f756e640000000000000000000060448201526064016101e9565b8369ffffffffffffffffffff168169ffffffffffffffffffff161015611dcf5760405162461bcd60e51b815260206004820152601160248201527f4f5450203a205374616c6520707269636500000000000000000000000000000060448201526064016101e9565b5090949350505050565b5f60408284031215611de9575f5ffd5b50919050565b5f5f83601f840112611dff575f5ffd5b50813567ffffffffffffffff811115611e16575f5ffd5b602083019150836020828501011115611e2d575f5ffd5b9250929050565b5f5f5f5f5f5f60808789031215611e49575f5ffd5b863567ffffffffffffffff811115611e5f575f5ffd5b611e6b89828a01611dd9565b965050602087013567ffffffffffffffff811115611e87575f5ffd5b611e9389828a01611def565b909650945050604087013567ffffffffffffffff811115611eb2575f5ffd5b611ebe89828a01611def565b979a9699509497949695606090950135949350505050565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b604081525f611f166040830185611ed6565b905082151560208301529392505050565b5f60208284031215611f37575f5ffd5b81357fffffffff0000000000000000000000000000000000000000000000000000000081168114611455575f5ffd5b6001600160a01b0381168114610e29575f5ffd5b5f5f60408385031215611f8b575f5ffd5b823591506020830135611f9d81611f66565b809150509250929050565b5f60208284031215611fb8575f5ffd5b813567ffffffffffffffff811115611fce575f5ffd5b611fda84828501611dd9565b949350505050565b5f60208284031215611ff2575f5ffd5b5035919050565b8015158114610e29575f5ffd5b5f5f5f5f5f6080868803121561201a575f5ffd5b853567ffffffffffffffff811115612030575f5ffd5b61203c88828901611def565b909650945050602086013561205081611ff9565b925060408601359150606086013567ffffffffffffffff811115612072575f5ffd5b86016101008189031215612084575f5ffd5b809150509295509295909350565b5f602082840312156120a2575f5ffd5b813561145581611f66565b602081525f6114556020830184611ed6565b5f5f5f606084860312156120d1575f5ffd5b83356120dc81611f66565b92506020840135915060408401356120f381611f66565b809150509250925092565b803561210981611f66565b919050565b5f82357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff21833603018112612140575f5ffd5b9190910192915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b808201808211156106fc576106fc61214a565b5f6020828403121561219a575f5ffd5b815160ff81168114611455575f5ffd5b6001815b60018411156121e5578085048111156121c9576121c961214a565b60018416156121d757908102905b60019390931c9280026121ae565b935093915050565b5f826121fb575060016106fc565b8161220757505f6106fc565b816001811461221d576002811461222757612243565b60019150506106fc565b60ff8411156122385761223861214a565b50506001821b6106fc565b5060208310610133831016604e8410600b8410161715612266575081810a6106fc565b6122725f1984846121aa565b805f19048211156122855761228561214a565b029392505050565b5f61145583836121ed565b5f602082840312156122a8575f5ffd5b5051919050565b5f602082840312156122bf575f5ffd5b815161145581611ff9565b5f82357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff01833603018112612140575f5ffd5b5f5f83357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe184360301811261232f575f5ffd5b83018035915067ffffffffffffffff821115612349575f5ffd5b602001915036819003821315611e2d575f5ffd5b5f82518060208501845e5f920191825250919050565b5f5f60408385031215612384575f5ffd5b825161238f81611f66565b6020939093015192949293505050565b5f5f83357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe18436030181126123d2575f5ffd5b830160208101925035905067ffffffffffffffff8111156123f1575f5ffd5b803603821315611e2d575f5ffd5b81835281816020850137505f602082840101525f6020601f19601f840116840101905092915050565b5f82357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0183360301811261245a575f5ffd5b90910192915050565b8035825260208082013590830152604080820135908301525f606082013561248a81611f66565b6001600160a01b031660608401526124a4608083016120fe565b6001600160a01b031660808401526124be60a083016120fe565b6001600160a01b031660a08401526124d960c083018361239f565b61010060c08601526124f0610100860182846123ff565b60e0948501359590940194909452509092915050565b602081525f82357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff2184360301811261253c575f5ffd5b604060208401528301803561255081611f66565b6001600160a01b03166060840152602081013561256c81611f66565b6001600160a01b0316608084810191909152604082013560a080860191909152606083013560c08601529082013560e08501526125ab9082018261239f565b60e06101008601526125c2610140860182846123ff565b60c084013561012087015291506125de90506020860186612428565b9150601f198482030160408501526125f68183612463565b95945050505050565b5f6020828403121561260f575f5ffd5b815161145581611f66565b828152604060208201525f611fda6040830184612463565b818103818111156106fc576106fc61214a565b5f77ffffffffffffffffffffffffffffffffffffffffffffffff821677ffffffffffffffffffffffffffffffffffffffffffffffff841677ffffffffffffffffffffffffffffffffffffffffffffffff81830216925081830481148215176126af576126af61214a565b505092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f77ffffffffffffffffffffffffffffffffffffffffffffffff83168061270d5761270d6126b7565b8077ffffffffffffffffffffffffffffffffffffffffffffffff84160491505092915050565b80820281158282048414176106fc576106fc61214a565b5f82612758576127586126b7565b500490565b805169ffffffffffffffffffff81168114612109575f5ffd5b5f5f5f5f5f60a0868803121561278a575f5ffd5b6127938661275d565b602087015160408801516060890151929750909550935091506127b86080870161275d565b9050929550929590935056fe332e302e302d626574612e31302b6f70656e67736e2e6f7261636c652e746f6b656e2e697061796d6173746572a26469706673582212208ceb21cfc1616fc7596ba091f748e95c5ce7403c3f05538f3465a74cb367bd1064736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10a\x01\x90W_5`\xE0\x1C\x80c\x9F\xBB\r\xCA\x11a\0\xDCW\x80c\xCDK\xC6\x11\x11a\0\x87W\x80c\xDFF:f\x11a\0bW\x80c\xDFF:f\x14a\x05iW\x80c\xF2\xFD\xE3\x8B\x14a\x05}W\x80c\xF9\xC0\x02\xF7\x14a\x05\x9CW\x80c\xFA\t\xE60\x14a\x05\xB2W__\xFD[\x80c\xCDK\xC6\x11\x14a\x05\x0EW\x80c\xCE\x1B\x81_\x14a\x05-W\x80c\xDAt\"(\x14a\x05JW__\xFD[\x80c\xB9\x0BA\xCF\x11a\0\xB7W\x80c\xB9\x0BA\xCF\x14a\x04\x8BW\x80c\xBB\xDA\xA3\xC9\x14a\x04\xA0W\x80c\xBDI$2\x14a\x04\xB6W__\xFD[\x80c\x9F\xBB\r\xCA\x14a\x04\x10W\x80c\xAD\x12\xE5\x0E\x14a\x04/W\x80c\xB09\xA8\x8F\x14a\x04DW__\xFD[\x80cqP\x18\xA6\x11a\x01<W\x80c{\xDF.\xC7\x11a\x01\x17W\x80c{\xDF.\xC7\x14a\x03\xB6W\x80c\x8D\xA5\xCB[\x14a\x03\xD3W\x80c\x92\x12v\xEA\x14a\x03\xEFW__\xFD[\x80cqP\x18\xA6\x14a\x03dW\x80cv\xFA\x01\xC3\x14a\x03xW\x80c{\xB0Rd\x14a\x03\x97W__\xFD[\x80cA\xBB\xB7\xCA\x11a\x01lW\x80cA\xBB\xB7\xCA\x14a\x02\xEBW\x80c\\^=\xB1\x14a\x03\"W\x80cm|>+\x14a\x03EW__\xFD[\x80b\xBE]\xD4\x14a\x02gW\x80c\x01\xFF\xC9\xA7\x14a\x02\x9DW\x80c-\x14\xC4\xB7\x14a\x02\xCCW__\xFD[6a\x02cW`\x01T`\x01`\x01`\xA0\x1B\x03\x16a\x01\xF2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7Frelay hub address not set\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\x01T`@Q\x7F\xAAg\xC9\x19\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xAAg\xC9\x19\x904\x90`$\x01_`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\x02OW__\xFD[PZ\xF1\x15\x80\x15a\x02aW=__>=_\xFD[\0[__\xFD[4\x80\x15a\x02rW__\xFD[Pa\x02\x86a\x02\x816`\x04a\x1E4V[a\x05\xD1V[`@Qa\x02\x94\x92\x91\x90a\x1F\x04V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\xA8W__\xFD[Pa\x02\xBCa\x02\xB76`\x04a\x1F'V[a\x06\x1EV[`@Q\x90\x15\x15\x81R` \x01a\x02\x94V[4\x80\x15a\x02\xD7W__\xFD[Pa\x02aa\x02\xE66`\x04a\x1FzV[a\x07\x02V[4\x80\x15a\x02\xF6W__\xFD[Pa\x03\na\x03\x056`\x04a\x1F\xA8V[a\x07\xD9V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\x94V[4\x80\x15a\x03-W__\xFD[Pa\x037a)\x04\x81V[`@Q\x90\x81R` \x01a\x02\x94V[4\x80\x15a\x03PW__\xFD[Pa\x02aa\x03_6`\x04a\x1F\xE2V[a\x07\xF2V[4\x80\x15a\x03oW__\xFD[Pa\x02aa\x08PV[4\x80\x15a\x03\x83W__\xFD[Pa\x02aa\x03\x926`\x04a \x06V[a\x08\xB4V[4\x80\x15a\x03\xA2W__\xFD[Pa\x02aa\x03\xB16`\x04a \x92V[a\x08\xD0V[4\x80\x15a\x03\xC1W__\xFD[P`\x01T`\x01`\x01`\xA0\x1B\x03\x16a\x03\nV[4\x80\x15a\x03\xDEW__\xFD[P_T`\x01`\x01`\xA0\x1B\x03\x16a\x03\nV[4\x80\x15a\x03\xFAW__\xFD[Pa\x04\x03a\t\xE2V[`@Qa\x02\x94\x91\x90a \xADV[4\x80\x15a\x04\x1BW__\xFD[Pa\x02aa\x04*6`\x04a\x1E4V[a\n\x02V[4\x80\x15a\x04:W__\xFD[Pa\x037`\x05T\x81V[4\x80\x15a\x04OW__\xFD[Pa\x04Xa\n\x1AV[`@Qa\x02\x94\x91\x90\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x80\x83\x01Q\x90\x82\x01R``\x91\x82\x01Q\x91\x81\x01\x91\x90\x91R`\x80\x01\x90V[4\x80\x15a\x04\x96W__\xFD[Pa\x037a\xC3P\x81V[4\x80\x15a\x04\xABW__\xFD[Pa\x037b\x01\xAD\xB0\x81V[4\x80\x15a\x04\xC1W__\xFD[Pa\x04\xF1a\x04\xD06`\x04a \x92V[`\x04` R_\x90\x81R`@\x90 \x80T`\x01\x90\x91\x01T`\x01`\x01`\xA0\x1B\x03\x16\x82V[`@\x80Q\x92\x83R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16` \x83\x01R\x01a\x02\x94V[4\x80\x15a\x05\x19W__\xFD[Pa\x02aa\x05(6`\x04a \xBFV[a\n\x7FV[4\x80\x15a\x058W__\xFD[P`\x02T`\x01`\x01`\xA0\x1B\x03\x16a\x03\nV[4\x80\x15a\x05UW__\xFD[Pa\x02aa\x05d6`\x04a \x92V[a\x0C'V[4\x80\x15a\x05tW__\xFD[Pa\x037a\r9V[4\x80\x15a\x05\x88W__\xFD[Pa\x02aa\x05\x976`\x04a \x92V[a\rKV[4\x80\x15a\x05\xA7W__\xFD[Pa\x037b\x01\x86\xA0\x81V[4\x80\x15a\x05\xBDW__\xFD[Pa\x02aa\x05\xCC6`\x04a \x92V[a\x0E,V[``_a\x05\xDCa\x0F\x95V[a\x05\xE5\x88a\x0F\xEFV[a\x05\xEE\x88a\x10\x87V[a\x05\xF7\x88a\x10\xE2V[a\x06\x01\x85\x85a\x11NV[a\x06\x0F\x88\x88\x88\x88\x88\x88a\x11\xA0V[\x91P\x91P\x96P\x96\x94PPPPPV[_\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x7F\xE1\xAB-\xEA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x14\x80a\x06\xB0WP\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x7F\x0E\x080v\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x14[\x80a\x06\xFCWP\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x14[\x92\x91PPV[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x07[W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x01\xE9V[`\x01T`@Q\x7F\xF3\xFE\xF3\xA3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R`$\x82\x01\x85\x90R\x90\x91\x16\x90c\xF3\xFE\xF3\xA3\x90`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x07\xBFW__\xFD[PZ\xF1\x15\x80\x15a\x07\xD1W=__>=_\xFD[PPPPPPV[_a\x07\xE4\x82\x80a!\x0EV[a\x06\xFC\x90` \x81\x01\x90a \x92V[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x08KW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x01\xE9V[`\x05UV[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x08\xA9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x01\xE9V[a\x08\xB2_a\x13\xB0V[V[a\x08\xBCa\x0F\x95V[a\x08\xC9\x85\x85\x85\x85\x85a\x14\x17V[PPPPPV[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\t)W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x01\xE9V[a\t\\`\x01`\x01`\xA0\x1B\x03\x82\x16\x7F\xE9\xFB0\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x14:V[a\t\xA8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7Ftarget is not a valid IRelayHub\0`D\x82\x01R`d\x01a\x01\xE9V[`\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[```@Q\x80``\x01`@R\x80`-\x81R` \x01a'\xC5`-\x919\x90P\x90V[a\n\x10\x86\x86\x86\x86\x86\x86a\x11\xA0V[PPPPPPPPV[a\nA`@Q\x80`\x80\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80`\x80\x01`@R\x80a\xC3Pb\x01\x86\xA0a\n]\x91\x90a!wV[\x81R` \x01b\x01\x86\xA0\x81R` \x01b\x01\xAD\xB0\x81R` \x01a)\x04\x81RP\x90P\x90V[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\n\xD8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x01\xE9V[\x80`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\x14W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B8\x91\x90a!\x8AV[`\xFF\x16`\x08\x14a\x0B\xAFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FOTP: token oracle decimals must `D\x82\x01R\x7Fbe 8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x01\xE9V[`@Q\x80`@\x01`@R\x80\x83`\na\x0B\xC7\x91\x90a\"\x8DV[\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16` \x91\x82\x01R\x93\x82\x16_\x90\x81R`\x04\x85R`@\x90 \x81Q\x81U\x93\x01Q`\x01\x90\x93\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x93\x90\x91\x16\x92\x90\x92\x17\x90\x91UPV[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0C\x80W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x01\xE9V[a\x0C\xB3`\x01`\x01`\xA0\x1B\x03\x82\x16\x7F%\xE2>d\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x14:V[a\x0C\xFFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7Ftarget is not a valid IForwarder`D\x82\x01R`d\x01a\x01\xE9V[`\x02\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[a\rHa\xC3Pb\x01\x86\xA0a!wV[\x81V[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\r\xA4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x01\xE9V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x0E W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x01\xE9V[a\x0E)\x81a\x13\xB0V[PV[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0E\x85W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x01\xE9V[`@Q\x7Fp\xA0\x821\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R0`\x04\x82\x01R_\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\xE2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\x06\x91\x90a\"\x98V[`@Q\x7F\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R3`\x04\x82\x01R`$\x81\x01\x82\x90R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x0FlW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\x90\x91\x90a\"\xAFV[PPPV[`\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x08\xB2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7Fcan only be called by RelayHub\0\0`D\x82\x01R`d\x01a\x01\xE9V[a\x0F\xFC` \x82\x01\x82a\"\xCAV[a\x10\r\x90`\xC0\x81\x01\x90`\xA0\x01a \x92V[`\x01`\x01`\xA0\x1B\x03\x16a\x10(`\x02T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x10~W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FForwarder is not trusted\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x01\xE9V[a\x0E)\x81a\x14\\V[a\x10\x91\x81\x80a!\x0EV[`@\x015\x15a\x0E)W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7Fvalue transfer not supported\0\0\0\0`D\x82\x01R`d\x01a\x01\xE9V[a\x10\xEF` \x82\x01\x82a\"\xCAV[a\x10\xFD\x90`\xC0\x81\x01\x90a\"\xFCV[\x90P`@\x14a\x0E)W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FpaymasterData: invalid length\0\0\0`D\x82\x01R`d\x01a\x01\xE9V[\x80\x15a\x11\x9CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7Fshould have no approvalData\0\0\0\0\0`D\x82\x01R`d\x01a\x01\xE9V[PPV[``_\x80\x80a\x11\xFCa\x11\xB5` \x8C\x01\x8Ca\"\xCAV[a\x11\xC3\x90`\xC0\x81\x01\x90a\"\xFCV[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPa\x16\x82\x92PPPV[\x91P\x91P___a\x12\x0E\x85\x8E\x8Aa\x16\xA6V[\x92P\x92P\x92P\x83\x81\x11\x15a\x12\x8AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`)`$\x82\x01R\x7FTx cost more than the user-suppl`D\x82\x01R\x7Fied limit\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x01\xE9V[`@Q\x7F#\xB8r\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R0`$\x83\x01R`D\x82\x01\x83\x90R\x86\x16\x90c#\xB8r\xDD\x90`d\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x12\xF5W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\x19\x91\x90a\"\xAFV[P`@\x80Q\x83\x81R`\x01`\x01`\xA0\x1B\x03\x87\x81\x16` \x83\x01R\x91\x81\x01\x83\x90R\x90\x84\x16\x90\x7F\xC5xN\xFE\xFB\x9E<;\xDE\xCE\xC2\x1C\xDDa\xE4 \xE9hX\xD8\xF1\xAA\xAAm\xBB\xE3\x0B\x05 u\xE1\x99\x90``\x01`@Q\x80\x91\x03\x90\xA2`@\x80Q`\x01`\x01`\xA0\x1B\x03\x80\x86\x16` \x83\x01R\x91\x81\x01\x83\x90R\x90\x86\x16``\x82\x01R`\x80\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x9D_\x9DP\x9BPPPPPPPPPPPPV[_\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[_\x80\x80a\x14&\x87\x89\x01\x89a \xBFV[\x92P\x92P\x92Pa\n\x10\x83\x83_\x88\x88\x86a\x17\xDCV[_a\x14D\x83a\x19\xB3V[\x80\x15a\x14UWPa\x14U\x83\x83a\x1A\x16V[\x93\x92PPPV[_\x80a\x14h\x83\x80a!\x0EV[a\x14y\x90`@\x81\x01\x90` \x01a \x92V[`\x01`\x01`\xA0\x1B\x03\x16\x7FW+l\x05\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x14\xB0` \x86\x01\x86a\"\xCAV[a\x14\xC1\x90`\xC0\x81\x01\x90`\xA0\x01a \x92V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`$\x82\x01R`D\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x94\x16\x93\x90\x93\x17\x90\x92R\x90Qa\x15B\x91\x90a#]V[_`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80_\x81\x14a\x15zW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x15\x7FV[``\x91P[P\x91P\x91P\x81a\x15\xD1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FisTrustedForwarder: reverted\0\0\0\0`D\x82\x01R`d\x01a\x01\xE9V[\x80Q` \x14a\x16\"W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FisTrustedForwarder: bad response`D\x82\x01R`d\x01a\x01\xE9V[\x80\x80` \x01\x90Q\x81\x01\x90a\x166\x91\x90a\"\xAFV[a\x0F\x90W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7Finvalid forwarder for recipient\0`D\x82\x01R`d\x01a\x01\xE9V[____\x84\x80` \x01\x90Q\x81\x01\x90a\x16\x9A\x91\x90a#sV[\x90\x96\x90\x95P\x93PPPPV[`@Q\x7FA\xBB\xB7\xCA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R_\x90\x81\x90\x81\x900\x90cA\xBB\xB7\xCA\x90a\x16\xE8\x90\x88\x90`\x04\x01a%\x06V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17\x03W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17'\x91\x90a%\xFFV[`\x01T\x90\x93P_\x90`\x01`\x01`\xA0\x1B\x03\x16c\x8EST\x8B\x86a\x17K` \x8A\x01\x8Aa\"\xCAV[`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x17h\x92\x91\x90a&\x1AV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17\x83W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\xA7\x91\x90a\"\x98V[\x90Pa\x17\xB3\x86\x80a!\x0EV[a\x17\xC1\x90`@\x015\x82a!wV[\x90Pa\x17\xCD\x87\x82a\x1B@V[\x91P\x80\x92PP\x93P\x93P\x93\x90PV[`\x01T`\x05T_\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\x8EST\x8B\x90a\x17\xFE\x90\x87a!wV[\x85`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x18\x1C\x92\x91\x90a&\x1AV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x187W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18[\x91\x90a\"\x98V[\x90P_a\x18q\x83a\x18l\x84\x89a!wV[a\x1B@V[\x90P_a\x18~\x82\x89a&2V[`@\x80Q\x85\x81R`\x01`\x01`\xA0\x1B\x03\x80\x88\x16` \x83\x01R\x91\x81\x01\x85\x90R\x90\x8B\x16``\x82\x01R\x90\x91P\x7F\xDBN(\x86\xD8~\x89*\xBF>\x8DBwe;\xA2\xA2l\xB0\x13\x07\x80A]4\x134\xBE\xBF^\xDC\xCA\x90`\x80\x01`@Q\x80\x91\x03\x90\xA1`@Q\x7F\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x8A\x81\x16`\x04\x83\x01R`$\x82\x01\x83\x90R\x85\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x198W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19\\\x91\x90a\"\xAFV[a\x19\xA8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01R\x7Ffailed refund\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x01\xE9V[PPPPPPPPPV[_a\x19\xDE\x82\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1A\x16V[\x80\x15a\x06\xFCWPa\x1A\x0F\x82\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1A\x16V[\x15\x92\x91PPV[`@\x80Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16`$\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`D\x90\x91\x01\x82R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90R\x90Q_\x91\x90\x82\x90\x81\x90`\x01`\x01`\xA0\x1B\x03\x87\x16\x90au0\x90a\x1A\xC2\x90\x86\x90a#]V[_`@Q\x80\x83\x03\x81\x86\x86\xFA\x92PPP=\x80_\x81\x14a\x1A\xFBW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x1B\0V[``\x91P[P\x91P\x91P` \x81Q\x10\x15a\x1B\x1AW_\x93PPPPa\x06\xFCV[\x81\x80\x15a\x1B6WP\x80\x80` \x01\x90Q\x81\x01\x90a\x1B6\x91\x90a\"\xAFV[\x96\x95PPPPPPV[`\x01`\x01`\xA0\x1B\x03\x80\x83\x16_\x90\x81R`\x04` \x90\x81R`@\x80\x83 \x81Q\x80\x83\x01\x90\x92R\x80T\x82R`\x01\x01T\x90\x93\x16\x90\x83\x01\x81\x90R\x90\x91\x90a\x1B\xC3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FOTP: Oracle does not exist\0\0\0\0\0\0`D\x82\x01R`d\x01a\x01\xE9V[_a\x1B\xD1\x82` \x01Qa\x1CNV[`\x03T\x90\x91P_\x90a\x1B\xEB\x90`\x01`\x01`\xA0\x1B\x03\x16a\x1CNV[\x90P_\x82\x84_\x01Q\x83a\x1B\xFE\x91\x90a&EV[a\x1C\x08\x91\x90a&\xE4V[\x90P_g\r\xE0\xB6\xB3\xA7d\0\0a\x1C8w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x89a'3V[a\x1CB\x91\x90a'JV[\x98\x97PPPPPPPPV[_____\x85`\x01`\x01`\xA0\x1B\x03\x16c\xFE\xAF\x96\x8C`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1C\x8FW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C\xB3\x91\x90a'vV[\x94P\x94PP\x93P\x93P_\x83\x13a\x1D\x0BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FOTP : Chainlink price <= 0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x01\xE9V[a\x1D\x18b\x02\xA3\0Ba&2V[\x82\x10\x15a\x1DgW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01R\x7FOTP : Incomplete round\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x01\xE9V[\x83i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15a\x1D\xCFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01R\x7FOTP : Stale price\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x01\xE9V[P\x90\x94\x93PPPPV[_`@\x82\x84\x03\x12\x15a\x1D\xE9W__\xFD[P\x91\x90PV[__\x83`\x1F\x84\x01\x12a\x1D\xFFW__\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1E\x16W__\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x1E-W__\xFD[\x92P\x92\x90PV[______`\x80\x87\x89\x03\x12\x15a\x1EIW__\xFD[\x865g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1E_W__\xFD[a\x1Ek\x89\x82\x8A\x01a\x1D\xD9V[\x96PP` \x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1E\x87W__\xFD[a\x1E\x93\x89\x82\x8A\x01a\x1D\xEFV[\x90\x96P\x94PP`@\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1E\xB2W__\xFD[a\x1E\xBE\x89\x82\x8A\x01a\x1D\xEFV[\x97\x9A\x96\x99P\x94\x97\x94\x96\x95``\x90\x95\x015\x94\x93PPPPV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[`@\x81R_a\x1F\x16`@\x83\x01\x85a\x1E\xD6V[\x90P\x82\x15\x15` \x83\x01R\x93\x92PPPV[_` \x82\x84\x03\x12\x15a\x1F7W__\xFD[\x815\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x81\x14a\x14UW__\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0E)W__\xFD[__`@\x83\x85\x03\x12\x15a\x1F\x8BW__\xFD[\x825\x91P` \x83\x015a\x1F\x9D\x81a\x1FfV[\x80\x91PP\x92P\x92\x90PV[_` \x82\x84\x03\x12\x15a\x1F\xB8W__\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1F\xCEW__\xFD[a\x1F\xDA\x84\x82\x85\x01a\x1D\xD9V[\x94\x93PPPPV[_` \x82\x84\x03\x12\x15a\x1F\xF2W__\xFD[P5\x91\x90PV[\x80\x15\x15\x81\x14a\x0E)W__\xFD[_____`\x80\x86\x88\x03\x12\x15a \x1AW__\xFD[\x855g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a 0W__\xFD[a <\x88\x82\x89\x01a\x1D\xEFV[\x90\x96P\x94PP` \x86\x015a P\x81a\x1F\xF9V[\x92P`@\x86\x015\x91P``\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a rW__\xFD[\x86\x01a\x01\0\x81\x89\x03\x12\x15a \x84W__\xFD[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[_` \x82\x84\x03\x12\x15a \xA2W__\xFD[\x815a\x14U\x81a\x1FfV[` \x81R_a\x14U` \x83\x01\x84a\x1E\xD6V[___``\x84\x86\x03\x12\x15a \xD1W__\xFD[\x835a \xDC\x81a\x1FfV[\x92P` \x84\x015\x91P`@\x84\x015a \xF3\x81a\x1FfV[\x80\x91PP\x92P\x92P\x92V[\x805a!\t\x81a\x1FfV[\x91\x90PV[_\x825\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF!\x836\x03\x01\x81\x12a!@W__\xFD[\x91\x90\x91\x01\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\x06\xFCWa\x06\xFCa!JV[_` \x82\x84\x03\x12\x15a!\x9AW__\xFD[\x81Q`\xFF\x81\x16\x81\x14a\x14UW__\xFD[`\x01\x81[`\x01\x84\x11\x15a!\xE5W\x80\x85\x04\x81\x11\x15a!\xC9Wa!\xC9a!JV[`\x01\x84\x16\x15a!\xD7W\x90\x81\x02\x90[`\x01\x93\x90\x93\x1C\x92\x80\x02a!\xAEV[\x93P\x93\x91PPV[_\x82a!\xFBWP`\x01a\x06\xFCV[\x81a\"\x07WP_a\x06\xFCV[\x81`\x01\x81\x14a\"\x1DW`\x02\x81\x14a\"'Wa\"CV[`\x01\x91PPa\x06\xFCV[`\xFF\x84\x11\x15a\"8Wa\"8a!JV[PP`\x01\x82\x1Ba\x06\xFCV[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a\"fWP\x81\x81\na\x06\xFCV[a\"r_\x19\x84\x84a!\xAAV[\x80_\x19\x04\x82\x11\x15a\"\x85Wa\"\x85a!JV[\x02\x93\x92PPPV[_a\x14U\x83\x83a!\xEDV[_` \x82\x84\x03\x12\x15a\"\xA8W__\xFD[PQ\x91\x90PV[_` \x82\x84\x03\x12\x15a\"\xBFW__\xFD[\x81Qa\x14U\x81a\x1F\xF9V[_\x825\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x01\x836\x03\x01\x81\x12a!@W__\xFD[__\x835\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x846\x03\x01\x81\x12a#/W__\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a#IW__\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x1E-W__\xFD[_\x82Q\x80` \x85\x01\x84^_\x92\x01\x91\x82RP\x91\x90PV[__`@\x83\x85\x03\x12\x15a#\x84W__\xFD[\x82Qa#\x8F\x81a\x1FfV[` \x93\x90\x93\x01Q\x92\x94\x92\x93PPPV[__\x835\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x846\x03\x01\x81\x12a#\xD2W__\xFD[\x83\x01` \x81\x01\x92P5\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a#\xF1W__\xFD[\x806\x03\x82\x13\x15a\x1E-W__\xFD[\x81\x83R\x81\x81` \x85\x017P_` \x82\x84\x01\x01R_` `\x1F\x19`\x1F\x84\x01\x16\x84\x01\x01\x90P\x92\x91PPV[_\x825\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x01\x836\x03\x01\x81\x12a$ZW__\xFD[\x90\x91\x01\x92\x91PPV[\x805\x82R` \x80\x82\x015\x90\x83\x01R`@\x80\x82\x015\x90\x83\x01R_``\x82\x015a$\x8A\x81a\x1FfV[`\x01`\x01`\xA0\x1B\x03\x16``\x84\x01Ra$\xA4`\x80\x83\x01a \xFEV[`\x01`\x01`\xA0\x1B\x03\x16`\x80\x84\x01Ra$\xBE`\xA0\x83\x01a \xFEV[`\x01`\x01`\xA0\x1B\x03\x16`\xA0\x84\x01Ra$\xD9`\xC0\x83\x01\x83a#\x9FV[a\x01\0`\xC0\x86\x01Ra$\xF0a\x01\0\x86\x01\x82\x84a#\xFFV[`\xE0\x94\x85\x015\x95\x90\x94\x01\x94\x90\x94RP\x90\x92\x91PPV[` \x81R_\x825\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF!\x846\x03\x01\x81\x12a%<W__\xFD[`@` \x84\x01R\x83\x01\x805a%P\x81a\x1FfV[`\x01`\x01`\xA0\x1B\x03\x16``\x84\x01R` \x81\x015a%l\x81a\x1FfV[`\x01`\x01`\xA0\x1B\x03\x16`\x80\x84\x81\x01\x91\x90\x91R`@\x82\x015`\xA0\x80\x86\x01\x91\x90\x91R``\x83\x015`\xC0\x86\x01R\x90\x82\x015`\xE0\x85\x01Ra%\xAB\x90\x82\x01\x82a#\x9FV[`\xE0a\x01\0\x86\x01Ra%\xC2a\x01@\x86\x01\x82\x84a#\xFFV[`\xC0\x84\x015a\x01 \x87\x01R\x91Pa%\xDE\x90P` \x86\x01\x86a$(V[\x91P`\x1F\x19\x84\x82\x03\x01`@\x85\x01Ra%\xF6\x81\x83a$cV[\x95\x94PPPPPV[_` \x82\x84\x03\x12\x15a&\x0FW__\xFD[\x81Qa\x14U\x81a\x1FfV[\x82\x81R`@` \x82\x01R_a\x1F\xDA`@\x83\x01\x84a$cV[\x81\x81\x03\x81\x81\x11\x15a\x06\xFCWa\x06\xFCa!JV[_w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x83\x02\x16\x92P\x81\x83\x04\x81\x14\x82\x15\x17a&\xAFWa&\xAFa!JV[PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x80a'\rWa'\ra&\xB7V[\x80w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x04\x91PP\x92\x91PPV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x06\xFCWa\x06\xFCa!JV[_\x82a'XWa'Xa&\xB7V[P\x04\x90V[\x80Qi\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a!\tW__\xFD[_____`\xA0\x86\x88\x03\x12\x15a'\x8AW__\xFD[a'\x93\x86a']V[` \x87\x01Q`@\x88\x01Q``\x89\x01Q\x92\x97P\x90\x95P\x93P\x91Pa'\xB8`\x80\x87\x01a']V[\x90P\x92\x95P\x92\x95\x90\x93PV\xFE3.0.0-beta.10+opengsn.oracle.token.ipaymaster\xA2dipfsX\"\x12 \x8C\xEB!\xCF\xC1ao\xC7Yk\xA0\x91\xF7H\xE9\\\\\xE7@<?\x05S\x8F4e\xA7L\xB3g\xBD\x10dsolcC\0\x08\x1B\x003",
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
    /**Event with signature `PostRelay(uint256,address,uint256,address)` and selector `0xdb4e2886d87e892abf3e8d4277653ba2a26cb0130780415d341334bebf5edcca`.
```solidity
event PostRelay(uint256 actualEthAmount, address token, uint256 actualTokenAmount, address payer);
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
        pub actualEthAmount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub token: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub actualTokenAmount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub payer: alloy::sol_types::private::Address,
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
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "PostRelay(uint256,address,uint256,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                219u8,
                78u8,
                40u8,
                134u8,
                216u8,
                126u8,
                137u8,
                42u8,
                191u8,
                62u8,
                141u8,
                66u8,
                119u8,
                101u8,
                59u8,
                162u8,
                162u8,
                108u8,
                176u8,
                19u8,
                7u8,
                128u8,
                65u8,
                93u8,
                52u8,
                19u8,
                52u8,
                190u8,
                191u8,
                94u8,
                220u8,
                202u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    actualEthAmount: data.0,
                    token: data.1,
                    actualTokenAmount: data.2,
                    payer: data.3,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.actualEthAmount),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.token,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.actualTokenAmount),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.payer,
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
    /**Event with signature `PreRelayPayment(uint256,address,uint256,address)` and selector `0xc5784efefb9e3c3bdecec21cdd61e420e96858d8f1aaaa6dbbe30b052075e199`.
```solidity
event PreRelayPayment(uint256 ethAmount, address token, uint256 tokenAmount, address indexed payer);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct PreRelayPayment {
        #[allow(missing_docs)]
        pub ethAmount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub token: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub tokenAmount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub payer: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for PreRelayPayment {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
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
            const SIGNATURE: &'static str = "PreRelayPayment(uint256,address,uint256,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                197u8,
                120u8,
                78u8,
                254u8,
                251u8,
                158u8,
                60u8,
                59u8,
                222u8,
                206u8,
                194u8,
                28u8,
                221u8,
                97u8,
                228u8,
                32u8,
                233u8,
                104u8,
                88u8,
                216u8,
                241u8,
                170u8,
                170u8,
                109u8,
                187u8,
                227u8,
                11u8,
                5u8,
                32u8,
                117u8,
                225u8,
                153u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    ethAmount: data.0,
                    token: data.1,
                    tokenAmount: data.2,
                    payer: topics.1,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.ethAmount),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.token,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.tokenAmount),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.payer.clone())
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
                    &self.payer,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for PreRelayPayment {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&PreRelayPayment> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &PreRelayPayment) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Constructor`.
```solidity
constructor(address _nativeTokenOracle);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        pub _nativeTokenOracle: alloy::sol_types::private::Address,
    }
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
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    (value._nativeTokenOracle,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _nativeTokenOracle: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
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
                        &self._nativeTokenOracle,
                    ),
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
    /**Function with signature `__preRelayedCall(((address,address,uint256,uint256,uint256,bytes,uint256),(uint256,uint256,uint256,address,address,address,bytes,uint256)),bytes,bytes,uint256)` and selector `0x9fbb0dca`.
```solidity
function __preRelayedCall(GsnTypes.RelayRequest memory relayRequest, bytes memory signature, bytes memory approvalData, uint256 maxPossibleGas) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct __preRelayedCallCall {
        pub relayRequest: <GsnTypes::RelayRequest as alloy::sol_types::SolType>::RustType,
        pub signature: alloy::sol_types::private::Bytes,
        pub approvalData: alloy::sol_types::private::Bytes,
        pub maxPossibleGas: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`__preRelayedCall(((address,address,uint256,uint256,uint256,bytes,uint256),(uint256,uint256,uint256,address,address,address,bytes,uint256)),bytes,bytes,uint256)`](__preRelayedCallCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct __preRelayedCallReturn {}
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
            impl ::core::convert::From<__preRelayedCallCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: __preRelayedCallCall) -> Self {
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
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for __preRelayedCallCall {
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
            impl ::core::convert::From<__preRelayedCallReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: __preRelayedCallReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for __preRelayedCallReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for __preRelayedCallCall {
            type Parameters<'a> = (
                GsnTypes::RelayRequest,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = __preRelayedCallReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "__preRelayedCall(((address,address,uint256,uint256,uint256,bytes,uint256),(uint256,uint256,uint256,address,address,address,bytes,uint256)),bytes,bytes,uint256)";
            const SELECTOR: [u8; 4] = [159u8, 187u8, 13u8, 202u8];
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
    /**Function with signature `addOracle(address,uint256,address)` and selector `0xcd4bc611`.
```solidity
function addOracle(address _token, uint256 _decimals, address _oracle) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addOracleCall {
        pub _token: alloy::sol_types::private::Address,
        pub _decimals: alloy::sol_types::private::primitives::aliases::U256,
        pub _oracle: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`addOracle(address,uint256,address)`](addOracleCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addOracleReturn {}
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
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<addOracleCall> for UnderlyingRustTuple<'_> {
                fn from(value: addOracleCall) -> Self {
                    (value._token, value._decimals, value._oracle)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for addOracleCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _token: tuple.0,
                        _decimals: tuple.1,
                        _oracle: tuple.2,
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
            impl ::core::convert::From<addOracleReturn> for UnderlyingRustTuple<'_> {
                fn from(value: addOracleReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for addOracleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for addOracleCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = addOracleReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "addOracle(address,uint256,address)";
            const SELECTOR: [u8; 4] = [205u8, 75u8, 198u8, 17u8];
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
                        &self._token,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._decimals),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._oracle,
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
    /**Function with signature `getPayer(((address,address,uint256,uint256,uint256,bytes,uint256),(uint256,uint256,uint256,address,address,address,bytes,uint256)))` and selector `0x41bbb7ca`.
```solidity
function getPayer(GsnTypes.RelayRequest memory relayRequest) external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getPayerCall {
        pub relayRequest: <GsnTypes::RelayRequest as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`getPayer(((address,address,uint256,uint256,uint256,bytes,uint256),(uint256,uint256,uint256,address,address,address,bytes,uint256)))`](getPayerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getPayerReturn {
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
            type UnderlyingSolTuple<'a> = (GsnTypes::RelayRequest,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <GsnTypes::RelayRequest as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getPayerCall> for UnderlyingRustTuple<'_> {
                fn from(value: getPayerCall) -> Self {
                    (value.relayRequest,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getPayerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { relayRequest: tuple.0 }
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
            impl ::core::convert::From<getPayerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getPayerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getPayerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getPayerCall {
            type Parameters<'a> = (GsnTypes::RelayRequest,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getPayerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getPayer(((address,address,uint256,uint256,uint256,bytes,uint256),(uint256,uint256,uint256,address,address,address,bytes,uint256)))";
            const SELECTOR: [u8; 4] = [65u8, 187u8, 183u8, 202u8];
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
    /**Function with signature `tokenOracles(address)` and selector `0xbd492432`.
```solidity
function tokenOracles(address) external view returns (uint256 div, address oracle);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct tokenOraclesCall {
        pub _0: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`tokenOracles(address)`](tokenOraclesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct tokenOraclesReturn {
        pub div: alloy::sol_types::private::primitives::aliases::U256,
        pub oracle: alloy::sol_types::private::Address,
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
            impl ::core::convert::From<tokenOraclesCall> for UnderlyingRustTuple<'_> {
                fn from(value: tokenOraclesCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for tokenOraclesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
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
            impl ::core::convert::From<tokenOraclesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: tokenOraclesReturn) -> Self {
                    (value.div, value.oracle)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for tokenOraclesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        div: tuple.0,
                        oracle: tuple.1,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for tokenOraclesCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = tokenOraclesReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "tokenOracles(address)";
            const SELECTOR: [u8; 4] = [189u8, 73u8, 36u8, 50u8];
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
                        &self._0,
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
    /**Function with signature `withdrawAll(address)` and selector `0xfa09e630`.
```solidity
function withdrawAll(address token) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct withdrawAllCall {
        pub token: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`withdrawAll(address)`](withdrawAllCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct withdrawAllReturn {}
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
            impl ::core::convert::From<withdrawAllCall> for UnderlyingRustTuple<'_> {
                fn from(value: withdrawAllCall) -> Self {
                    (value.token,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for withdrawAllCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { token: tuple.0 }
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
            impl ::core::convert::From<withdrawAllReturn> for UnderlyingRustTuple<'_> {
                fn from(value: withdrawAllReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for withdrawAllReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for withdrawAllCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = withdrawAllReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "withdrawAll(address)";
            const SELECTOR: [u8; 4] = [250u8, 9u8, 230u8, 48u8];
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
    ///Container for all the [`OracleTokenPaymaster`](self) function calls.
    pub enum OracleTokenPaymasterCalls {
        CALLDATA_SIZE_LIMIT(CALLDATA_SIZE_LIMITCall),
        FORWARDER_HUB_OVERHEAD(FORWARDER_HUB_OVERHEADCall),
        PAYMASTER_ACCEPTANCE_BUDGET(PAYMASTER_ACCEPTANCE_BUDGETCall),
        POST_RELAYED_CALL_GAS_LIMIT(POST_RELAYED_CALL_GAS_LIMITCall),
        PRE_RELAYED_CALL_GAS_LIMIT(PRE_RELAYED_CALL_GAS_LIMITCall),
        __preRelayedCall(__preRelayedCallCall),
        addOracle(addOracleCall),
        gasUsedByPost(gasUsedByPostCall),
        getGasAndDataLimits(getGasAndDataLimitsCall),
        getPayer(getPayerCall),
        getRelayHub(getRelayHubCall),
        getTrustedForwarder(getTrustedForwarderCall),
        owner(ownerCall),
        postRelayedCall(postRelayedCallCall),
        preRelayedCall(preRelayedCallCall),
        renounceOwnership(renounceOwnershipCall),
        setPostGasUsage(setPostGasUsageCall),
        setRelayHub(setRelayHubCall),
        setTrustedForwarder(setTrustedForwarderCall),
        supportsInterface(supportsInterfaceCall),
        tokenOracles(tokenOraclesCall),
        transferOwnership(transferOwnershipCall),
        versionPaymaster(versionPaymasterCall),
        withdrawAll(withdrawAllCall),
        withdrawRelayHubDepositTo(withdrawRelayHubDepositToCall),
    }
    #[automatically_derived]
    impl OracleTokenPaymasterCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [0u8, 190u8, 93u8, 212u8],
            [1u8, 255u8, 201u8, 167u8],
            [45u8, 20u8, 196u8, 183u8],
            [65u8, 187u8, 183u8, 202u8],
            [92u8, 94u8, 61u8, 177u8],
            [109u8, 124u8, 62u8, 43u8],
            [113u8, 80u8, 24u8, 166u8],
            [118u8, 250u8, 1u8, 195u8],
            [123u8, 176u8, 82u8, 100u8],
            [123u8, 223u8, 46u8, 199u8],
            [141u8, 165u8, 203u8, 91u8],
            [146u8, 18u8, 118u8, 234u8],
            [159u8, 187u8, 13u8, 202u8],
            [173u8, 18u8, 229u8, 14u8],
            [176u8, 57u8, 168u8, 143u8],
            [185u8, 11u8, 65u8, 207u8],
            [187u8, 218u8, 163u8, 201u8],
            [189u8, 73u8, 36u8, 50u8],
            [205u8, 75u8, 198u8, 17u8],
            [206u8, 27u8, 129u8, 95u8],
            [218u8, 116u8, 34u8, 40u8],
            [223u8, 70u8, 58u8, 102u8],
            [242u8, 253u8, 227u8, 139u8],
            [249u8, 192u8, 2u8, 247u8],
            [250u8, 9u8, 230u8, 48u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for OracleTokenPaymasterCalls {
        const NAME: &'static str = "OracleTokenPaymasterCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 25usize;
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
                Self::__preRelayedCall(_) => {
                    <__preRelayedCallCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::addOracle(_) => {
                    <addOracleCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::gasUsedByPost(_) => {
                    <gasUsedByPostCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getGasAndDataLimits(_) => {
                    <getGasAndDataLimitsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getPayer(_) => <getPayerCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::getRelayHub(_) => {
                    <getRelayHubCall as alloy_sol_types::SolCall>::SELECTOR
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
                Self::tokenOracles(_) => {
                    <tokenOraclesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::transferOwnership(_) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::versionPaymaster(_) => {
                    <versionPaymasterCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::withdrawAll(_) => {
                    <withdrawAllCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<OracleTokenPaymasterCalls>] = &[
                {
                    fn preRelayedCall(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OracleTokenPaymasterCalls> {
                        <preRelayedCallCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OracleTokenPaymasterCalls::preRelayedCall)
                    }
                    preRelayedCall
                },
                {
                    fn supportsInterface(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OracleTokenPaymasterCalls> {
                        <supportsInterfaceCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OracleTokenPaymasterCalls::supportsInterface)
                    }
                    supportsInterface
                },
                {
                    fn withdrawRelayHubDepositTo(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OracleTokenPaymasterCalls> {
                        <withdrawRelayHubDepositToCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OracleTokenPaymasterCalls::withdrawRelayHubDepositTo)
                    }
                    withdrawRelayHubDepositTo
                },
                {
                    fn getPayer(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OracleTokenPaymasterCalls> {
                        <getPayerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OracleTokenPaymasterCalls::getPayer)
                    }
                    getPayer
                },
                {
                    fn CALLDATA_SIZE_LIMIT(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OracleTokenPaymasterCalls> {
                        <CALLDATA_SIZE_LIMITCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OracleTokenPaymasterCalls::CALLDATA_SIZE_LIMIT)
                    }
                    CALLDATA_SIZE_LIMIT
                },
                {
                    fn setPostGasUsage(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OracleTokenPaymasterCalls> {
                        <setPostGasUsageCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OracleTokenPaymasterCalls::setPostGasUsage)
                    }
                    setPostGasUsage
                },
                {
                    fn renounceOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OracleTokenPaymasterCalls> {
                        <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OracleTokenPaymasterCalls::renounceOwnership)
                    }
                    renounceOwnership
                },
                {
                    fn postRelayedCall(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OracleTokenPaymasterCalls> {
                        <postRelayedCallCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OracleTokenPaymasterCalls::postRelayedCall)
                    }
                    postRelayedCall
                },
                {
                    fn setRelayHub(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OracleTokenPaymasterCalls> {
                        <setRelayHubCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OracleTokenPaymasterCalls::setRelayHub)
                    }
                    setRelayHub
                },
                {
                    fn getRelayHub(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OracleTokenPaymasterCalls> {
                        <getRelayHubCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OracleTokenPaymasterCalls::getRelayHub)
                    }
                    getRelayHub
                },
                {
                    fn owner(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OracleTokenPaymasterCalls> {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OracleTokenPaymasterCalls::owner)
                    }
                    owner
                },
                {
                    fn versionPaymaster(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OracleTokenPaymasterCalls> {
                        <versionPaymasterCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OracleTokenPaymasterCalls::versionPaymaster)
                    }
                    versionPaymaster
                },
                {
                    fn __preRelayedCall(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OracleTokenPaymasterCalls> {
                        <__preRelayedCallCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OracleTokenPaymasterCalls::__preRelayedCall)
                    }
                    __preRelayedCall
                },
                {
                    fn gasUsedByPost(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OracleTokenPaymasterCalls> {
                        <gasUsedByPostCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OracleTokenPaymasterCalls::gasUsedByPost)
                    }
                    gasUsedByPost
                },
                {
                    fn getGasAndDataLimits(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OracleTokenPaymasterCalls> {
                        <getGasAndDataLimitsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OracleTokenPaymasterCalls::getGasAndDataLimits)
                    }
                    getGasAndDataLimits
                },
                {
                    fn FORWARDER_HUB_OVERHEAD(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OracleTokenPaymasterCalls> {
                        <FORWARDER_HUB_OVERHEADCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OracleTokenPaymasterCalls::FORWARDER_HUB_OVERHEAD)
                    }
                    FORWARDER_HUB_OVERHEAD
                },
                {
                    fn POST_RELAYED_CALL_GAS_LIMIT(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OracleTokenPaymasterCalls> {
                        <POST_RELAYED_CALL_GAS_LIMITCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OracleTokenPaymasterCalls::POST_RELAYED_CALL_GAS_LIMIT)
                    }
                    POST_RELAYED_CALL_GAS_LIMIT
                },
                {
                    fn tokenOracles(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OracleTokenPaymasterCalls> {
                        <tokenOraclesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OracleTokenPaymasterCalls::tokenOracles)
                    }
                    tokenOracles
                },
                {
                    fn addOracle(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OracleTokenPaymasterCalls> {
                        <addOracleCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OracleTokenPaymasterCalls::addOracle)
                    }
                    addOracle
                },
                {
                    fn getTrustedForwarder(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OracleTokenPaymasterCalls> {
                        <getTrustedForwarderCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OracleTokenPaymasterCalls::getTrustedForwarder)
                    }
                    getTrustedForwarder
                },
                {
                    fn setTrustedForwarder(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OracleTokenPaymasterCalls> {
                        <setTrustedForwarderCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OracleTokenPaymasterCalls::setTrustedForwarder)
                    }
                    setTrustedForwarder
                },
                {
                    fn PAYMASTER_ACCEPTANCE_BUDGET(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OracleTokenPaymasterCalls> {
                        <PAYMASTER_ACCEPTANCE_BUDGETCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OracleTokenPaymasterCalls::PAYMASTER_ACCEPTANCE_BUDGET)
                    }
                    PAYMASTER_ACCEPTANCE_BUDGET
                },
                {
                    fn transferOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OracleTokenPaymasterCalls> {
                        <transferOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OracleTokenPaymasterCalls::transferOwnership)
                    }
                    transferOwnership
                },
                {
                    fn PRE_RELAYED_CALL_GAS_LIMIT(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OracleTokenPaymasterCalls> {
                        <PRE_RELAYED_CALL_GAS_LIMITCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OracleTokenPaymasterCalls::PRE_RELAYED_CALL_GAS_LIMIT)
                    }
                    PRE_RELAYED_CALL_GAS_LIMIT
                },
                {
                    fn withdrawAll(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OracleTokenPaymasterCalls> {
                        <withdrawAllCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OracleTokenPaymasterCalls::withdrawAll)
                    }
                    withdrawAll
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
                Self::__preRelayedCall(inner) => {
                    <__preRelayedCallCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::addOracle(inner) => {
                    <addOracleCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
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
                Self::getPayer(inner) => {
                    <getPayerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::getRelayHub(inner) => {
                    <getRelayHubCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::tokenOracles(inner) => {
                    <tokenOraclesCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::withdrawAll(inner) => {
                    <withdrawAllCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::__preRelayedCall(inner) => {
                    <__preRelayedCallCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::addOracle(inner) => {
                    <addOracleCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::getPayer(inner) => {
                    <getPayerCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::tokenOracles(inner) => {
                    <tokenOraclesCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::withdrawAll(inner) => {
                    <withdrawAllCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
    ///Container for all the [`OracleTokenPaymaster`](self) events.
    pub enum OracleTokenPaymasterEvents {
        OwnershipTransferred(OwnershipTransferred),
        PostRelay(PostRelay),
        PreRelayPayment(PreRelayPayment),
    }
    #[automatically_derived]
    impl OracleTokenPaymasterEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
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
            [
                197u8,
                120u8,
                78u8,
                254u8,
                251u8,
                158u8,
                60u8,
                59u8,
                222u8,
                206u8,
                194u8,
                28u8,
                221u8,
                97u8,
                228u8,
                32u8,
                233u8,
                104u8,
                88u8,
                216u8,
                241u8,
                170u8,
                170u8,
                109u8,
                187u8,
                227u8,
                11u8,
                5u8,
                32u8,
                117u8,
                225u8,
                153u8,
            ],
            [
                219u8,
                78u8,
                40u8,
                134u8,
                216u8,
                126u8,
                137u8,
                42u8,
                191u8,
                62u8,
                141u8,
                66u8,
                119u8,
                101u8,
                59u8,
                162u8,
                162u8,
                108u8,
                176u8,
                19u8,
                7u8,
                128u8,
                65u8,
                93u8,
                52u8,
                19u8,
                52u8,
                190u8,
                191u8,
                94u8,
                220u8,
                202u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for OracleTokenPaymasterEvents {
        const NAME: &'static str = "OracleTokenPaymasterEvents";
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
                Some(<PreRelayPayment as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <PreRelayPayment as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::PreRelayPayment)
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
    impl alloy_sol_types::private::IntoLogData for OracleTokenPaymasterEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::PostRelay(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::PreRelayPayment(inner) => {
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
                Self::PreRelayPayment(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`OracleTokenPaymaster`](self) contract instance.

See the [wrapper's documentation](`OracleTokenPaymasterInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> OracleTokenPaymasterInstance<T, P, N> {
        OracleTokenPaymasterInstance::<T, P, N>::new(address, provider)
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
        _nativeTokenOracle: alloy::sol_types::private::Address,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<OracleTokenPaymasterInstance<T, P, N>>,
    > {
        OracleTokenPaymasterInstance::<T, P, N>::deploy(provider, _nativeTokenOracle)
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
        _nativeTokenOracle: alloy::sol_types::private::Address,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        OracleTokenPaymasterInstance::<
            T,
            P,
            N,
        >::deploy_builder(provider, _nativeTokenOracle)
    }
    /**A [`OracleTokenPaymaster`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`OracleTokenPaymaster`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct OracleTokenPaymasterInstance<
        T,
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for OracleTokenPaymasterInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("OracleTokenPaymasterInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > OracleTokenPaymasterInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`OracleTokenPaymaster`](self) contract instance.

See the [wrapper's documentation](`OracleTokenPaymasterInstance`) for more details.*/
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
            _nativeTokenOracle: alloy::sol_types::private::Address,
        ) -> alloy_contract::Result<OracleTokenPaymasterInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(provider, _nativeTokenOracle);
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
            _nativeTokenOracle: alloy::sol_types::private::Address,
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall {
                            _nativeTokenOracle,
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
    impl<T, P: ::core::clone::Clone, N> OracleTokenPaymasterInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> OracleTokenPaymasterInstance<T, P, N> {
            OracleTokenPaymasterInstance {
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
    > OracleTokenPaymasterInstance<T, P, N> {
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
        ///Creates a new call builder for the [`__preRelayedCall`] function.
        pub fn __preRelayedCall(
            &self,
            relayRequest: <GsnTypes::RelayRequest as alloy::sol_types::SolType>::RustType,
            signature: alloy::sol_types::private::Bytes,
            approvalData: alloy::sol_types::private::Bytes,
            maxPossibleGas: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, __preRelayedCallCall, N> {
            self.call_builder(
                &__preRelayedCallCall {
                    relayRequest,
                    signature,
                    approvalData,
                    maxPossibleGas,
                },
            )
        }
        ///Creates a new call builder for the [`addOracle`] function.
        pub fn addOracle(
            &self,
            _token: alloy::sol_types::private::Address,
            _decimals: alloy::sol_types::private::primitives::aliases::U256,
            _oracle: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, addOracleCall, N> {
            self.call_builder(
                &addOracleCall {
                    _token,
                    _decimals,
                    _oracle,
                },
            )
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
        ///Creates a new call builder for the [`getPayer`] function.
        pub fn getPayer(
            &self,
            relayRequest: <GsnTypes::RelayRequest as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, getPayerCall, N> {
            self.call_builder(&getPayerCall { relayRequest })
        }
        ///Creates a new call builder for the [`getRelayHub`] function.
        pub fn getRelayHub(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getRelayHubCall, N> {
            self.call_builder(&getRelayHubCall {})
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
        ///Creates a new call builder for the [`tokenOracles`] function.
        pub fn tokenOracles(
            &self,
            _0: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, tokenOraclesCall, N> {
            self.call_builder(&tokenOraclesCall { _0 })
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
        ///Creates a new call builder for the [`withdrawAll`] function.
        pub fn withdrawAll(
            &self,
            token: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, withdrawAllCall, N> {
            self.call_builder(&withdrawAllCall { token })
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
    > OracleTokenPaymasterInstance<T, P, N> {
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
        ///Creates a new event filter for the [`PreRelayPayment`] event.
        pub fn PreRelayPayment_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, PreRelayPayment, N> {
            self.event_filter::<PreRelayPayment>()
        }
    }
}
