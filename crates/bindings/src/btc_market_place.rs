///Module containing a contract's types and functions.
/**

```solidity
library BitcoinTx {
    struct Info { bytes4 version; bytes inputVector; bytes outputVector; bytes4 locktime; }
    struct Proof { bytes merkleProof; uint256 txIndexInBlock; bytes bitcoinHeaders; bytes32 coinbasePreimage; bytes coinbaseProof; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod BitcoinTx {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct Info { bytes4 version; bytes inputVector; bytes outputVector; bytes4 locktime; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Info {
        #[allow(missing_docs)]
        pub version: alloy::sol_types::private::FixedBytes<4>,
        #[allow(missing_docs)]
        pub inputVector: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub outputVector: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub locktime: alloy::sol_types::private::FixedBytes<4>,
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
            alloy::sol_types::sol_data::FixedBytes<4>,
            alloy::sol_types::sol_data::Bytes,
            alloy::sol_types::sol_data::Bytes,
            alloy::sol_types::sol_data::FixedBytes<4>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::FixedBytes<4>,
            alloy::sol_types::private::Bytes,
            alloy::sol_types::private::Bytes,
            alloy::sol_types::private::FixedBytes<4>,
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
        impl ::core::convert::From<Info> for UnderlyingRustTuple<'_> {
            fn from(value: Info) -> Self {
                (value.version, value.inputVector, value.outputVector, value.locktime)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Info {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    version: tuple.0,
                    inputVector: tuple.1,
                    outputVector: tuple.2,
                    locktime: tuple.3,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for Info {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for Info {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        4,
                    > as alloy_sol_types::SolType>::tokenize(&self.version),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.inputVector,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.outputVector,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        4,
                    > as alloy_sol_types::SolType>::tokenize(&self.locktime),
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
        impl alloy_sol_types::SolType for Info {
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
        impl alloy_sol_types::SolStruct for Info {
            const NAME: &'static str = "Info";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "Info(bytes4 version,bytes inputVector,bytes outputVector,bytes4 locktime)",
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
                    <alloy::sol_types::sol_data::FixedBytes<
                        4,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.version)
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.inputVector,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.outputVector,
                        )
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        4,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.locktime)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Info {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::FixedBytes<
                        4,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.version,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.inputVector,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.outputVector,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        4,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.locktime,
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
                <alloy::sol_types::sol_data::FixedBytes<
                    4,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.version,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.inputVector,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.outputVector,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    4,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.locktime,
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct Proof { bytes merkleProof; uint256 txIndexInBlock; bytes bitcoinHeaders; bytes32 coinbasePreimage; bytes coinbaseProof; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Proof {
        #[allow(missing_docs)]
        pub merkleProof: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub txIndexInBlock: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub bitcoinHeaders: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub coinbasePreimage: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub coinbaseProof: alloy::sol_types::private::Bytes,
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
            alloy::sol_types::sol_data::Bytes,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Bytes,
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::Bytes,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Bytes,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::Bytes,
            alloy::sol_types::private::FixedBytes<32>,
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
        impl ::core::convert::From<Proof> for UnderlyingRustTuple<'_> {
            fn from(value: Proof) -> Self {
                (
                    value.merkleProof,
                    value.txIndexInBlock,
                    value.bitcoinHeaders,
                    value.coinbasePreimage,
                    value.coinbaseProof,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Proof {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    merkleProof: tuple.0,
                    txIndexInBlock: tuple.1,
                    bitcoinHeaders: tuple.2,
                    coinbasePreimage: tuple.3,
                    coinbaseProof: tuple.4,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for Proof {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for Proof {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.merkleProof,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.txIndexInBlock),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.bitcoinHeaders,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.coinbasePreimage),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.coinbaseProof,
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
        impl alloy_sol_types::SolType for Proof {
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
        impl alloy_sol_types::SolStruct for Proof {
            const NAME: &'static str = "Proof";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "Proof(bytes merkleProof,uint256 txIndexInBlock,bytes bitcoinHeaders,bytes32 coinbasePreimage,bytes coinbaseProof)",
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
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.merkleProof,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.txIndexInBlock,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.bitcoinHeaders,
                        )
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.coinbasePreimage,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.coinbaseProof,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Proof {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.merkleProof,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.txIndexInBlock,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.bitcoinHeaders,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.coinbasePreimage,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.coinbaseProof,
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
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.merkleProof,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.txIndexInBlock,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.bitcoinHeaders,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.coinbasePreimage,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.coinbaseProof,
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
    /**Creates a new wrapper around an on-chain [`BitcoinTx`](self) contract instance.

See the [wrapper's documentation](`BitcoinTxInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> BitcoinTxInstance<P, N> {
        BitcoinTxInstance::<P, N>::new(address, provider)
    }
    /**A [`BitcoinTx`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`BitcoinTx`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct BitcoinTxInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for BitcoinTxInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("BitcoinTxInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > BitcoinTxInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`BitcoinTx`](self) contract instance.

See the [wrapper's documentation](`BitcoinTxInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            provider: P,
        ) -> Self {
            Self {
                address,
                provider,
                _network: ::core::marker::PhantomData,
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
    impl<P: ::core::clone::Clone, N> BitcoinTxInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> BitcoinTxInstance<P, N> {
            BitcoinTxInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > BitcoinTxInstance<P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<&P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > BitcoinTxInstance<P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
/**

Generated by the following Solidity interface...
```solidity
library BitcoinTx {
    struct Info {
        bytes4 version;
        bytes inputVector;
        bytes outputVector;
        bytes4 locktime;
    }
    struct Proof {
        bytes merkleProof;
        uint256 txIndexInBlock;
        bytes bitcoinHeaders;
        bytes32 coinbasePreimage;
        bytes coinbaseProof;
    }
}

interface BtcMarketPlace {
    struct AcceptedBtcBuyOrder {
        uint256 orderId;
        uint256 amountBtc;
        address ercToken;
        uint256 ercAmount;
        address requester;
        address accepter;
        uint256 acceptTime;
    }
    struct AcceptedBtcSellOrder {
        uint256 orderId;
        BitcoinAddress bitcoinAddress;
        uint256 amountBtc;
        address ercToken;
        uint256 ercAmount;
        address requester;
        address accepter;
        uint256 acceptTime;
    }
    struct BitcoinAddress {
        bytes scriptPubKey;
    }
    struct BtcBuyOrder {
        uint256 amountBtc;
        BitcoinAddress bitcoinAddress;
        address offeringToken;
        uint256 offeringAmount;
        address requester;
    }
    struct BtcSellOrder {
        uint256 amountBtc;
        address askingToken;
        uint256 askingAmount;
        address requester;
    }

    event acceptBtcBuyOrderEvent(uint256 indexed orderId, uint256 indexed acceptId, uint256 amountBtc, uint256 ercAmount, address ercToken);
    event acceptBtcSellOrderEvent(uint256 indexed id, uint256 indexed acceptId, BitcoinAddress bitcoinAddress, uint256 amountBtc, uint256 ercAmount, address ercToken);
    event cancelAcceptedBtcBuyOrderEvent(uint256 id);
    event cancelAcceptedBtcSellOrderEvent(uint256 id);
    event placeBtcBuyOrderEvent(uint256 amountBtc, BitcoinAddress bitcoinAddress, address sellingToken, uint256 saleAmount);
    event placeBtcSellOrderEvent(uint256 indexed orderId, uint256 amountBtc, address buyingToken, uint256 buyAmount);
    event proofBtcBuyOrderEvent(uint256 id);
    event proofBtcSellOrderEvent(uint256 id);
    event withdrawBtcBuyOrderEvent(uint256 id);
    event withdrawBtcSellOrderEvent(uint256 id);

    constructor(address _relay, address erc2771Forwarder);

    function REQUEST_EXPIRATION_SECONDS() external view returns (uint256);
    function acceptBtcBuyOrder(uint256 id, uint256 amountBtc) external returns (uint256);
    function acceptBtcSellOrder(uint256 id, BitcoinAddress memory bitcoinAddress, uint256 amountBtc) external returns (uint256);
    function acceptedBtcBuyOrders(uint256) external view returns (uint256 orderId, uint256 amountBtc, address ercToken, uint256 ercAmount, address requester, address accepter, uint256 acceptTime);
    function acceptedBtcSellOrders(uint256) external view returns (uint256 orderId, BitcoinAddress memory bitcoinAddress, uint256 amountBtc, address ercToken, uint256 ercAmount, address requester, address accepter, uint256 acceptTime);
    function btcBuyOrders(uint256) external view returns (uint256 amountBtc, BitcoinAddress memory bitcoinAddress, address offeringToken, uint256 offeringAmount, address requester);
    function btcSellOrders(uint256) external view returns (uint256 amountBtc, address askingToken, uint256 askingAmount, address requester);
    function cancelAcceptedBtcBuyOrder(uint256 id) external;
    function cancelAcceptedBtcSellOrder(uint256 id) external;
    function getOpenAcceptedBtcBuyOrders() external view returns (AcceptedBtcBuyOrder[] memory, uint256[] memory);
    function getOpenAcceptedBtcSellOrders() external view returns (AcceptedBtcSellOrder[] memory, uint256[] memory);
    function getOpenBtcBuyOrders() external view returns (BtcBuyOrder[] memory, uint256[] memory);
    function getOpenBtcSellOrders() external view returns (BtcSellOrder[] memory, uint256[] memory);
    function getTrustedForwarder() external view returns (address forwarder);
    function isTrustedForwarder(address forwarder) external view returns (bool);
    function placeBtcBuyOrder(uint256 amountBtc, BitcoinAddress memory bitcoinAddress, address sellingToken, uint256 saleAmount) external;
    function placeBtcSellOrder(uint256 amountBtc, address buyingToken, uint256 buyAmount) external;
    function proofBtcBuyOrder(uint256 id, BitcoinTx.Info memory transaction, BitcoinTx.Proof memory proof) external;
    function proofBtcSellOrder(uint256 id, BitcoinTx.Info memory transaction, BitcoinTx.Proof memory proof) external;
    function withdrawBtcBuyOrder(uint256 id) external;
    function withdrawBtcSellOrder(uint256 id) external;
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [
      {
        "name": "_relay",
        "type": "address",
        "internalType": "contract TestLightRelay"
      },
      {
        "name": "erc2771Forwarder",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "REQUEST_EXPIRATION_SECONDS",
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
    "name": "acceptBtcBuyOrder",
    "inputs": [
      {
        "name": "id",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "amountBtc",
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
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "acceptBtcSellOrder",
    "inputs": [
      {
        "name": "id",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "bitcoinAddress",
        "type": "tuple",
        "internalType": "struct BtcMarketPlace.BitcoinAddress",
        "components": [
          {
            "name": "scriptPubKey",
            "type": "bytes",
            "internalType": "bytes"
          }
        ]
      },
      {
        "name": "amountBtc",
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
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "acceptedBtcBuyOrders",
    "inputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "orderId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "amountBtc",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "ercToken",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "ercAmount",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "requester",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "accepter",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "acceptTime",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "acceptedBtcSellOrders",
    "inputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "orderId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "bitcoinAddress",
        "type": "tuple",
        "internalType": "struct BtcMarketPlace.BitcoinAddress",
        "components": [
          {
            "name": "scriptPubKey",
            "type": "bytes",
            "internalType": "bytes"
          }
        ]
      },
      {
        "name": "amountBtc",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "ercToken",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "ercAmount",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "requester",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "accepter",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "acceptTime",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "btcBuyOrders",
    "inputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "amountBtc",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "bitcoinAddress",
        "type": "tuple",
        "internalType": "struct BtcMarketPlace.BitcoinAddress",
        "components": [
          {
            "name": "scriptPubKey",
            "type": "bytes",
            "internalType": "bytes"
          }
        ]
      },
      {
        "name": "offeringToken",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "offeringAmount",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "requester",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "btcSellOrders",
    "inputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "amountBtc",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "askingToken",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "askingAmount",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "requester",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "cancelAcceptedBtcBuyOrder",
    "inputs": [
      {
        "name": "id",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "cancelAcceptedBtcSellOrder",
    "inputs": [
      {
        "name": "id",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getOpenAcceptedBtcBuyOrders",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "tuple[]",
        "internalType": "struct BtcMarketPlace.AcceptedBtcBuyOrder[]",
        "components": [
          {
            "name": "orderId",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "amountBtc",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "ercToken",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "ercAmount",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "requester",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "accepter",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "acceptTime",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      },
      {
        "name": "",
        "type": "uint256[]",
        "internalType": "uint256[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getOpenAcceptedBtcSellOrders",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "tuple[]",
        "internalType": "struct BtcMarketPlace.AcceptedBtcSellOrder[]",
        "components": [
          {
            "name": "orderId",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "bitcoinAddress",
            "type": "tuple",
            "internalType": "struct BtcMarketPlace.BitcoinAddress",
            "components": [
              {
                "name": "scriptPubKey",
                "type": "bytes",
                "internalType": "bytes"
              }
            ]
          },
          {
            "name": "amountBtc",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "ercToken",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "ercAmount",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "requester",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "accepter",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "acceptTime",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      },
      {
        "name": "",
        "type": "uint256[]",
        "internalType": "uint256[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getOpenBtcBuyOrders",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "tuple[]",
        "internalType": "struct BtcMarketPlace.BtcBuyOrder[]",
        "components": [
          {
            "name": "amountBtc",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "bitcoinAddress",
            "type": "tuple",
            "internalType": "struct BtcMarketPlace.BitcoinAddress",
            "components": [
              {
                "name": "scriptPubKey",
                "type": "bytes",
                "internalType": "bytes"
              }
            ]
          },
          {
            "name": "offeringToken",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "offeringAmount",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "requester",
            "type": "address",
            "internalType": "address"
          }
        ]
      },
      {
        "name": "",
        "type": "uint256[]",
        "internalType": "uint256[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getOpenBtcSellOrders",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "tuple[]",
        "internalType": "struct BtcMarketPlace.BtcSellOrder[]",
        "components": [
          {
            "name": "amountBtc",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "askingToken",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "askingAmount",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "requester",
            "type": "address",
            "internalType": "address"
          }
        ]
      },
      {
        "name": "",
        "type": "uint256[]",
        "internalType": "uint256[]"
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
        "name": "forwarder",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "isTrustedForwarder",
    "inputs": [
      {
        "name": "forwarder",
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
    "name": "placeBtcBuyOrder",
    "inputs": [
      {
        "name": "amountBtc",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "bitcoinAddress",
        "type": "tuple",
        "internalType": "struct BtcMarketPlace.BitcoinAddress",
        "components": [
          {
            "name": "scriptPubKey",
            "type": "bytes",
            "internalType": "bytes"
          }
        ]
      },
      {
        "name": "sellingToken",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "saleAmount",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "placeBtcSellOrder",
    "inputs": [
      {
        "name": "amountBtc",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "buyingToken",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "buyAmount",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "proofBtcBuyOrder",
    "inputs": [
      {
        "name": "id",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "transaction",
        "type": "tuple",
        "internalType": "struct BitcoinTx.Info",
        "components": [
          {
            "name": "version",
            "type": "bytes4",
            "internalType": "bytes4"
          },
          {
            "name": "inputVector",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "outputVector",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "locktime",
            "type": "bytes4",
            "internalType": "bytes4"
          }
        ]
      },
      {
        "name": "proof",
        "type": "tuple",
        "internalType": "struct BitcoinTx.Proof",
        "components": [
          {
            "name": "merkleProof",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "txIndexInBlock",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "bitcoinHeaders",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "coinbasePreimage",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "coinbaseProof",
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
    "name": "proofBtcSellOrder",
    "inputs": [
      {
        "name": "id",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "transaction",
        "type": "tuple",
        "internalType": "struct BitcoinTx.Info",
        "components": [
          {
            "name": "version",
            "type": "bytes4",
            "internalType": "bytes4"
          },
          {
            "name": "inputVector",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "outputVector",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "locktime",
            "type": "bytes4",
            "internalType": "bytes4"
          }
        ]
      },
      {
        "name": "proof",
        "type": "tuple",
        "internalType": "struct BitcoinTx.Proof",
        "components": [
          {
            "name": "merkleProof",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "txIndexInBlock",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "bitcoinHeaders",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "coinbasePreimage",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "coinbaseProof",
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
    "name": "withdrawBtcBuyOrder",
    "inputs": [
      {
        "name": "id",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "withdrawBtcSellOrder",
    "inputs": [
      {
        "name": "id",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "event",
    "name": "acceptBtcBuyOrderEvent",
    "inputs": [
      {
        "name": "orderId",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "acceptId",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "amountBtc",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "ercAmount",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "ercToken",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "acceptBtcSellOrderEvent",
    "inputs": [
      {
        "name": "id",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "acceptId",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "bitcoinAddress",
        "type": "tuple",
        "indexed": false,
        "internalType": "struct BtcMarketPlace.BitcoinAddress",
        "components": [
          {
            "name": "scriptPubKey",
            "type": "bytes",
            "internalType": "bytes"
          }
        ]
      },
      {
        "name": "amountBtc",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "ercAmount",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "ercToken",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "cancelAcceptedBtcBuyOrderEvent",
    "inputs": [
      {
        "name": "id",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "cancelAcceptedBtcSellOrderEvent",
    "inputs": [
      {
        "name": "id",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "placeBtcBuyOrderEvent",
    "inputs": [
      {
        "name": "amountBtc",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "bitcoinAddress",
        "type": "tuple",
        "indexed": false,
        "internalType": "struct BtcMarketPlace.BitcoinAddress",
        "components": [
          {
            "name": "scriptPubKey",
            "type": "bytes",
            "internalType": "bytes"
          }
        ]
      },
      {
        "name": "sellingToken",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "saleAmount",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "placeBtcSellOrderEvent",
    "inputs": [
      {
        "name": "orderId",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "amountBtc",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "buyingToken",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "buyAmount",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "proofBtcBuyOrderEvent",
    "inputs": [
      {
        "name": "id",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "proofBtcSellOrderEvent",
    "inputs": [
      {
        "name": "id",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "withdrawBtcBuyOrderEvent",
    "inputs": [
      {
        "name": "id",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "withdrawBtcSellOrderEvent",
    "inputs": [
      {
        "name": "id",
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
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod BtcMarketPlace {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x6080604052348015600e575f5ffd5b506040516145ba3803806145ba833981016040819052602b916081565b5f80546001600160a01b0319166001600160a01b038316179055600680546001600160a01b0319166001600160a01b0384161790555050600160075560b4565b6001600160a01b0381168114607e575f5ffd5b50565b5f5f604083850312156091575f5ffd5b8251609a81606b565b602084015190925060a981606b565b809150509250929050565b6144f9806100c15f395ff3fe608060405234801561000f575f5ffd5b506004361061016e575f3560e01c80636a8cde3a116100d2578063c56a452611610088578063df69b14f11610063578063df69b14f146103b2578063ecca2c36146103c5578063fd3fc24514610432575f5ffd5b8063c56a45261461037c578063ce1b815f1461038f578063d1920ff0146103a9575f5ffd5b8063a383013b116100b8578063a383013b146102ba578063b223d976146102cd578063bd2a7e3e146102e0575f5ffd5b80636a8cde3a1461028e5780639cc6722e146102a4575f5ffd5b80634145640a11610127578063572b6c051161010d578063572b6c05146102345780635b8fe042146102655780636811a31114610278575f5ffd5b80634145640a146101fa578063506a109d14610221575f5ffd5b8063210ec18111610157578063210ec181146101ae578063364f1ec0146101c15780633af3fc7e146101d6575f5ffd5b806311c137aa146101725780631dfe759514610198575b5f5ffd5b61018561018036600461354f565b610445565b6040519081526020015b60405180910390f35b6101a061063e565b60405161018f9291906135eb565b6101856101bc3660046136d9565b6108a0565b6101d46101cf366004613741565b610ad7565b005b6101e96101e436600461379c565b610c2c565b60405161018f9594939291906137b3565b61020d61020836600461379c565b610cfe565b60405161018f9897969594939291906137fb565b6101d461022f36600461379c565b610def565b610255610242366004613850565b5f546001600160a01b0391821691161490565b604051901515815260200161018f565b6101d4610273366004613869565b610ed4565b610280610ff0565b60405161018f92919061389c565b6102966111b4565b60405161018f92919061392b565b6102ac6113ba565b60405161018f9291906139c3565b6101d46102c836600461379c565b61161d565b6101d46102db366004613ab1565b6116c1565b6103396102ee36600461379c565b600260208190525f918252604090912080546001820154928201546003830154600484015460058501546006909501549395946001600160a01b039384169492939182169291169087565b6040805197885260208801969096526001600160a01b03948516958701959095526060860192909252821660808501521660a083015260c082015260e00161018f565b6101d461038a36600461379c565b61185d565b5f546040516001600160a01b03909116815260200161018f565b61018561546081565b6101d46103c036600461379c565b611940565b6104076103d336600461379c565b600360208190525f9182526040909120805460018201546002830154929093015490926001600160a01b0390811692911684565b604080519485526001600160a01b03938416602086015284019190915216606082015260800161018f565b6101d4610440366004613ab1565b611a53565b5f828152600160205260408120805483111561045f575f5ffd5b5f831161046a575f5ffd5b805460038201545f919061047e9086613b55565b6104889190613b99565b90505f811161049957610499613bac565b80826003015410156104ad576104ad613bac565b80826003015f8282546104c09190613bd9565b90915550508154849083905f906104d8908490613bd9565b90915550506040805160e0810182528681526020810186905260028401546001600160a01b039081169282019290925260608101839052600484015490911660808201525f9060a0810161052a611be0565b6001600160a01b0316815242602090910152600580549192505f91908261055083613bec565b909155505f818152600260208181526040928390208651815586820151600182015586840151818401805473ffffffffffffffffffffffffffffffffffffffff199081166001600160a01b03938416179091556060808a0151600385015560808a0151600485018054841691851691909117905560a08a01516005850180549093169084161790915560c08901516006909301929092559289015484518c815292830189905290921692810192909252919250829189917fc39a1a5ddc0e85c955fe2e1abeb43c94ce18322e75bb3d44e80f759ff9d034b9910160405180910390a393505050505b92915050565b6060805f805b600554811015610683575f818152600160205260409020600401546001600160a01b03161561067b578161067781613bec565b9250505b600101610644565b505f8167ffffffffffffffff81111561069e5761069e613c04565b6040519080825280602002602001820160405280156106d757816020015b6106c4613465565b8152602001906001900390816106bc5790505b5090505f8267ffffffffffffffff8111156106f4576106f4613c04565b60405190808252806020026020018201604052801561071d578160200160208202803683370190505b5090505f805b600554811015610894575f818152600160205260409020600401546001600160a01b03161561088c5760015f8281526020019081526020015f206040518060a00160405290815f8201548152602001600182016040518060200160405290815f8201805461079090613c31565b80601f01602080910402602001604051908101604052809291908181526020018280546107bc90613c31565b80156108075780601f106107de57610100808354040283529160200191610807565b820191905f5260205f20905b8154815290600101906020018083116107ea57829003601f168201915b50505091909252505050815260028201546001600160a01b03908116602083015260038301546040830152600490920154909116606090910152845185908490811061085557610855613c7c565b60200260200101819052508083838151811061087357610873613c7c565b60209081029190910101528161088881613bec565b9250505b600101610723565b50919590945092505050565b5f838152600360205260408120826108b6575f5ffd5b80548311156108c3575f5ffd5b805460028201545f91906108d79086613b55565b6108e19190613b99565b90505f81116108f2576108f2613bac565b808260020154101561090657610906613bac565b80826002015f8282546109199190613bd9565b90915550508154849083905f90610931908490613bd9565b909155506109589050610942611be0565b60018401546001600160a01b0316903084611c30565b600580545f918261096883613bec565b9190505590506040518061010001604052808881526020018761098a90613d5d565b81526020810187905260018501546001600160a01b03908116604083015260608201859052600386015416608082015260a0016109c5611be0565b6001600160a01b03168152426020918201525f838152600482526040902082518155908201518051600183019081906109fe9082613e02565b5050506040828101516002830155606083015160038301805473ffffffffffffffffffffffffffffffffffffffff199081166001600160a01b03938416179091556080850151600485015560a0850151600585018054831691841691909117905560c085015160068501805490921690831617905560e0909301516007909201919091556001850154905183928a927f653e0d81f2c99beba359dfb17b499a5cff2be9d950514852224df8c097c2192192610ac3928c928c928a929190911690613f54565b60405180910390a3925050505b9392505050565b6001600160a01b038216610ae9575f5ffd5b610b06610af4611be0565b6001600160a01b038416903084611c30565b600580545f9182610b1683613bec565b9190505590506040518060a0016040528086815260200185610b3790613d5d565b8152602001846001600160a01b03168152602001838152602001610b59611be0565b6001600160a01b031690525f8281526001602081815260409092208351815591830151805190918301908190610b8f9082613e02565b50505060408281015160028301805473ffffffffffffffffffffffffffffffffffffffff199081166001600160a01b03938416179091556060850151600385015560809094015160049093018054909416921691909117909155517f98c7c680403d47403dea4a570d0e6c5716538c49420ef471cec428f5a5852c0690610c1d908790879087908790613f8b565b60405180910390a15050505050565b600160208181525f92835260409283902080548451928301909452918201805482908290610c5990613c31565b80601f0160208091040260200160405190810160405280929190818152602001828054610c8590613c31565b8015610cd05780601f10610ca757610100808354040283529160200191610cd0565b820191905f5260205f20905b815481529060010190602001808311610cb357829003601f168201915b505050919092525050506002820154600383015460049093015491926001600160a01b039182169290911685565b6004602052805f5260405f205f91509050805f015490806001016040518060200160405290815f82018054610d3290613c31565b80601f0160208091040260200160405190810160405280929190818152602001828054610d5e90613c31565b8015610da95780601f10610d8057610100808354040283529160200191610da9565b820191905f5260205f20905b815481529060010190602001808311610d8c57829003601f168201915b5050509190925250505060028201546003830154600484015460058501546006860154600790960154949593946001600160a01b03938416949293918216929091169088565b5f818152600160205260409020610e04611be0565b60048201546001600160a01b03908116911614610e1f575f5ffd5b610e44610e2a611be0565b600383015460028401546001600160a01b03169190611ce7565b5f82815260016020819052604082208281559190820181610e6582826134a6565b50505060028101805473ffffffffffffffffffffffffffffffffffffffff199081169091555f60038301556004909101805490911690556040518281527fc340e7ac48dc80ee793fc6266960bd5f1bd21be91c8a95e218178113f79e17b4906020015b60405180910390a15050565b6001600160a01b038216610ee6575f5ffd5b5f8311610ef1575f5ffd5b5f8111610efc575f5ffd5b600580545f9182610f0c83613bec565b9190505590506040518060800160405280858152602001846001600160a01b03168152602001838152602001610f40611be0565b6001600160a01b039081169091525f83815260036020818152604092839020855181558582015160018201805491871673ffffffffffffffffffffffffffffffffffffffff199283161790558685015160028301556060968701519190930180549186169190931617909155815188815292871690830152810184905282917fff1ce210defcd3ba1adf76c9419a0758fa60fd3eb38c7bd9418f60b575b76e24910160405180910390a250505050565b6060805f805b600554811015611036575f81815260036020819052604090912001546001600160a01b03161561102e578161102a81613bec565b9250505b600101610ff6565b505f8167ffffffffffffffff81111561105157611051613c04565b6040519080825280602002602001820160405280156110a157816020015b604080516080810182525f8082526020808301829052928201819052606082015282525f1990920191018161106f5790505b5090505f8267ffffffffffffffff8111156110be576110be613c04565b6040519080825280602002602001820160405280156110e7578160200160208202803683370190505b5090505f805b600554811015610894575f81815260036020819052604090912001546001600160a01b0316156111ac575f8181526003602081815260409283902083516080810185528154815260018201546001600160a01b039081169382019390935260028201549481019490945290910154166060820152845185908490811061117557611175613c7c565b60200260200101819052508083838151811061119357611193613c7c565b6020908102919091010152816111a881613bec565b9250505b6001016110ed565b6060805f805b6005548110156111f0575f81815260026020526040902060010154156111e857816111e481613bec565b9250505b6001016111ba565b505f8167ffffffffffffffff81111561120b5761120b613c04565b60405190808252806020026020018201604052801561129057816020015b61127d6040518060e001604052805f81526020015f81526020015f6001600160a01b031681526020015f81526020015f6001600160a01b031681526020015f6001600160a01b031681526020015f81525090565b8152602001906001900390816112295790505b5090505f8267ffffffffffffffff8111156112ad576112ad613c04565b6040519080825280602002602001820160405280156112d6578160200160208202803683370190505b5090505f805b600554811015610894575f81815260026020526040902060010154156113b2575f81815260026020818152604092839020835160e08101855281548152600182015492810192909252918201546001600160a01b039081169382019390935260038201546060820152600482015483166080820152600582015490921660a08301526006015460c0820152845185908490811061137b5761137b613c7c565b60200260200101819052508083838151811061139957611399613c7c565b6020908102919091010152816113ae81613bec565b9250505b6001016112dc565b6060805f805b6005548110156113f6575f81815260046020526040902060020154156113ee57816113ea81613bec565b9250505b6001016113c0565b505f8167ffffffffffffffff81111561141157611411613c04565b60405190808252806020026020018201604052801561144a57816020015b6114376134e0565b81526020019060019003908161142f5790505b5090505f8267ffffffffffffffff81111561146757611467613c04565b604051908082528060200260200182016040528015611490578160200160208202803683370190505b5090505f805b600554811015610894575f81815260046020526040902060020154156116155760045f8281526020019081526020015f20604051806101000160405290815f8201548152602001600182016040518060200160405290815f820180546114fb90613c31565b80601f016020809104026020016040519081016040528092919081815260200182805461152790613c31565b80156115725780601f1061154957610100808354040283529160200191611572565b820191905f5260205f20905b81548152906001019060200180831161155557829003601f168201915b5050509190925250505081526002820154602082015260038201546001600160a01b0390811660408301526004830154606083015260058301548116608083015260068301541660a082015260079091015460c09091015284518590849081106115de576115de613c7c565b6020026020010181905250808383815181106115fc576115fc613c7c565b60209081029190910101528161161181613bec565b9250505b600101611496565b5f818152600360205260409020611632611be0565b60038201546001600160a01b0390811691161461164d575f5ffd5b5f82815260036020818152604080842084815560018101805473ffffffffffffffffffffffffffffffffffffffff1990811690915560028201959095559092018054909316909255518381527f3cd475b092e8b379f6ba0d9e0e0c8f30705e73321dc5c9f80ce4ad38db7be1aa9101610ec8565b5f8381526002602052604090206116d6611be0565b60058201546001600160a01b039081169116146116f1575f5ffd5b6006546001600160a01b031663d38c29a161170f6040850185613fbf565b6040518363ffffffff1660e01b815260040161172c929190614020565b5f604051808303815f87803b158015611743575f5ffd5b505af1158015611755573d5f5f3e3d5ffd5b505050506117866007548461176990614062565b61177285614110565b6006546001600160a01b0316929190611d35565b5080545f908152600160208190526040909120805490916117aa9190830186611d62565b6005820154600383015460028401546117d1926001600160a01b0391821692911690611ce7565b5f85815260026020818152604080842084815560018101859055928301805473ffffffffffffffffffffffffffffffffffffffff1990811690915560038401859055600484018054821690556005840180549091169055600690920192909255518681527fb4c98de210696b3cf21e99335c1ee3a0ae34a26713412a4adde8af596176f37e9101610c1d565b5f818152600260205260409020611872611be0565b60048201546001600160a01b0390811691161461188d575f5ffd5b615460816006015461189f91906141bd565b42116118a9575f5ffd5b6118b4610e2a611be0565b5f82815260026020818152604080842084815560018101859055928301805473ffffffffffffffffffffffffffffffffffffffff1990811690915560038401859055600484018054821690556005840180549091169055600690920192909255518381527f3e5ea358e9eb4cdf44cdc77938ade8074b1240a6d8c0fd13728671b82e800ad69101610ec8565b5f818152600460205260409020600781015461195f90615460906141bd565b4211611969575f5ffd5b611971611be0565b60068201546001600160a01b0390811691161461198c575f5ffd5b6119b1611997611be0565b600483015460038401546001600160a01b03169190611ce7565b5f8281526004602052604081208181559060018201816119d182826134a6565b50505f6002830181905560038301805473ffffffffffffffffffffffffffffffffffffffff1990811690915560048401829055600584018054821690556006840180549091169055600790920191909155506040518281527f78f51f62f7cf1381c673c27eae187dd6c588dc6624ce59697dbb3e1d7c1bbcdf90602001610ec8565b5f838152600460205260409020611a68611be0565b60058201546001600160a01b03908116911614611a83575f5ffd5b6006546001600160a01b031663d38c29a1611aa16040850185613fbf565b6040518363ffffffff1660e01b8152600401611abe929190614020565b5f604051808303815f87803b158015611ad5575f5ffd5b505af1158015611ae7573d5f5f3e3d5ffd5b50505050611afb6007548461176990614062565b50611b0e81600201548260010185611d62565b600581015460048201546003830154611b35926001600160a01b0391821692911690611ce7565b5f848152600460205260408120818155906001820181611b5582826134a6565b50505f6002830181905560038301805473ffffffffffffffffffffffffffffffffffffffff1990811690915560048401829055600584018054821690556006840180549091169055600790920191909155506040518481527fcf561061db78f7bc518d37fe86718514c640ccc5c3f1293828b955e68f19f5fb9060200160405180910390a150505050565b5f60143610801590611bfb57505f546001600160a01b031633145b15611c2b57507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffec36013560601c90565b503390565b6040516001600160a01b0380851660248301528316604482015260648101829052611ce19085907f23b872dd00000000000000000000000000000000000000000000000000000000906084015b60408051601f198184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167fffffffff0000000000000000000000000000000000000000000000000000000090931692909217909152611e79565b50505050565b6040516001600160a01b038316602482015260448101829052611d309084907fa9059cbb0000000000000000000000000000000000000000000000000000000090606401611c7d565b505050565b5f611d3f83611f5d565b9050611d4b818361204d565b611d5a858584604001516122b1565b949350505050565b5f825f018054611d7190613c31565b604051611d83925085906020016141d0565b6040516020818303038152906040528051906020012090505f611dea838060400190611daf9190613fbf565b8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f92019190915250869250612601915050565b5167ffffffffffffffff16905084811015611e725760405162461bcd60e51b815260206004820152603b60248201527f426974636f696e207472616e73616374696f6e20616d6f756e74206973206c6f60448201527f776572207468616e20696e206163636570746564206f726465722e000000000060648201526084015b60405180910390fd5b5050505050565b5f611ecd826040518060400160405280602081526020017f5361666545524332303a206c6f772d6c6576656c2063616c6c206661696c6564815250856001600160a01b031661279f9092919063ffffffff16565b805190915015611d305780806020019051810190611eeb9190614297565b611d305760405162461bcd60e51b815260206004820152602a60248201527f5361666545524332303a204552433230206f7065726174696f6e20646964206e60448201527f6f742073756363656564000000000000000000000000000000000000000000006064820152608401611e69565b5f611f6b82602001516127ad565b611fb75760405162461bcd60e51b815260206004820152601d60248201527f496e76616c696420696e70757420766563746f722070726f76696465640000006044820152606401611e69565b611fc48260400151612847565b6120105760405162461bcd60e51b815260206004820152601e60248201527f496e76616c6964206f757470757420766563746f722070726f766964656400006044820152606401611e69565b610638825f015183602001518460400151856060015160405160200161203994939291906142cd565b6040516020818303038152906040526128d4565b8051612058906128f6565b6120a45760405162461bcd60e51b815260206004820152601660248201527f426164206d65726b6c652061727261792070726f6f66000000000000000000006044820152606401611e69565b608081015151815151146121205760405162461bcd60e51b815260206004820152602f60248201527f5478206e6f74206f6e2073616d65206c6576656c206f66206d65726b6c65207460448201527f72656520617320636f696e6261736500000000000000000000000000000000006064820152608401611e69565b5f61212e826040015161290c565b825160208401519192506121459185918491612918565b6121b75760405162461bcd60e51b815260206004820152603c60248201527f5478206d65726b6c652070726f6f66206973206e6f742076616c696420666f7260448201527f2070726f76696465642068656164657220616e642074782068617368000000006064820152608401611e69565b5f600283606001516040516020016121d191815260200190565b60408051601f19818403018152908290526121eb9161433c565b602060405180830381855afa158015612206573d5f5f3e3d5ffd5b5050506040513d601f19601f820116820180604052508101906122299190614347565b608084015190915061223f90829084905f612918565b611ce15760405162461bcd60e51b815260206004820152603f60248201527f436f696e62617365206d65726b6c652070726f6f66206973206e6f742076616c60448201527f696420666f722070726f76696465642068656164657220616e642068617368006064820152608401611e69565b5f836001600160a01b031663113764be6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156122ee573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906123129190614347565b90505f846001600160a01b0316632b97be246040518163ffffffff1660e01b8152600401602060405180830381865afa158015612351573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906123759190614347565b90505f8061238a61238586612953565b61295e565b905083810361239b57839150612418565b8281036123aa57829150612418565b60405162461bcd60e51b815260206004820152602560248201527f4e6f742061742063757272656e74206f722070726576696f757320646966666960448201527f63756c74790000000000000000000000000000000000000000000000000000006064820152608401611e69565b5f61242286612985565b90505f19810361249a5760405162461bcd60e51b815260206004820152602360248201527f496e76616c6964206c656e677468206f6620746865206865616465727320636860448201527f61696e00000000000000000000000000000000000000000000000000000000006064820152608401611e69565b7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81036125095760405162461bcd60e51b815260206004820152601560248201527f496e76616c6964206865616465727320636861696e00000000000000000000006044820152606401611e69565b7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffd81036125785760405162461bcd60e51b815260206004820152601d60248201527f496e73756666696369656e7420776f726b20696e2061206865616465720000006044820152606401611e69565b6125828784613b55565b8110156125f75760405162461bcd60e51b815260206004820152603360248201527f496e73756666696369656e7420616363756d756c61746564206469666669637560448201527f6c747920696e2068656164657220636861696e000000000000000000000000006064820152608401611e69565b5050505050505050565b604080516060810182525f808252602080830182905282840182905283518085019094528184528301529061263584612ba9565b60208301528082528161264782613bec565b9052505f805b82602001518110156127495782515f90612668908890612bbe565b84519091505f9061267a908990612c1e565b90505f612688600884613bd9565b86519091505f9061269a9060086141bd565b8a8101602001839020909150808a036126d4576001965083895f018181516126c2919061435e565b67ffffffffffffffff16905250612724565b5f6126e28c8a5f0151612c94565b90506001600160a01b03811615612703576001600160a01b03811660208b01525b5f6127118d8b5f0151612d74565b905080156127215760408b018190525b50505b84885f0181815161273591906141bd565b905250506001909401935061264d92505050565b50806127975760405162461bcd60e51b815260206004820181905260248201527f4e6f206f757470757420666f756e6420666f72207363726970745075624b65796044820152606401611e69565b505092915050565b6060611d5a84845f85612e54565b5f5f5f6127b984612ba9565b90925090508015806127cb57505f1982145b156127d957505f9392505050565b5f6127e58360016141bd565b90505f5b8281101561283a578551821061280457505f95945050505050565b5f61280f8784612f98565b90505f19810361282557505f9695505050505050565b61282f81846141bd565b9250506001016127e9565b5093519093149392505050565b5f5f5f61285384612ba9565b909250905080158061286557505f1982145b1561287357505f9392505050565b5f61287f8360016141bd565b90505f5b8281101561283a578551821061289e57505f95945050505050565b5f6128a98784612bbe565b90505f1981036128bf57505f9695505050505050565b6128c981846141bd565b925050600101612883565b5f60205f83516020850160025afa5060205f60205f60025afa50505f51919050565b5f60208251612905919061437e565b1592915050565b60448101515f90610638565b5f8385148015612926575081155b801561293157508251155b1561293e57506001611d5a565b61294a85848685612fde565b95945050505050565b5f610638825f613083565b5f6106387bffff00000000000000000000000000000000000000000000000000008361311c565b5f60508251612994919061437e565b156129a157505f19919050565b505f80805b8351811015612ba25780156129ed576129c0848284613127565b6129ed57507ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe9392505050565b5f6129f88583613083565b9050612a0685836050613150565b925080612b49845f8190506008817eff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff16901b600882901c7eff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff161790506010817dffff0000ffff0000ffff0000ffff0000ffff0000ffff0000ffff0000ffff16901b601082901c7dffff0000ffff0000ffff0000ffff0000ffff0000ffff0000ffff0000ffff161790506020817bffffffff00000000ffffffff00000000ffffffff00000000ffffffff16901b602082901c7bffffffff00000000ffffffff00000000ffffffff00000000ffffffff1617905060408177ffffffffffffffff0000000000000000ffffffffffffffff16901b604082901c77ffffffffffffffff0000000000000000ffffffffffffffff16179050608081901b608082901c179050919050565b1115612b7957507ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffd949350505050565b612b828161295e565b612b8c90856141bd565b9350612b9b90506050826141bd565b90506129a6565b5050919050565b5f5f612bb5835f613175565b91509150915091565b5f612bca8260096141bd565b83511015612bda57505f19610638565b5f80612bf085612beb8660086141bd565b613175565b909250905060018201612c08575f1992505050610638565b80612c148360096141bd565b61294a91906141bd565b5f80612c2a8484613312565b60c01c90505f61294a8264ff000000ff600882811c91821665ff000000ff009390911b92831617601090811b67ffffffffffffffff1666ff00ff00ff00ff9290921667ff00ff00ff00ff009093169290921790911c65ffff0000ffff1617602081811c91901b1790565b5f82612ca18360096141bd565b81518110612cb157612cb1613c7c565b6020910101517fff00000000000000000000000000000000000000000000000000000000000000167f6a0000000000000000000000000000000000000000000000000000000000000014612d0657505f610638565b5f83612d1384600a6141bd565b81518110612d2357612d23613c7c565b01602001517fff000000000000000000000000000000000000000000000000000000000000008116915060f81c601403612d6d575f612d6384600b6141bd565b8501601401519250505b5092915050565b5f82612d818360096141bd565b81518110612d9157612d91613c7c565b6020910101517fff00000000000000000000000000000000000000000000000000000000000000167f6a0000000000000000000000000000000000000000000000000000000000000014612de657505f610638565b5f83612df384600a6141bd565b81518110612e0357612e03613c7c565b016020908101517fff000000000000000000000000000000000000000000000000000000000000008116925060f81c9003612d6d575f612e4484600b6141bd565b8501602001519250505092915050565b606082471015612ecc5760405162461bcd60e51b815260206004820152602660248201527f416464726573733a20696e73756666696369656e742062616c616e636520666f60448201527f722063616c6c00000000000000000000000000000000000000000000000000006064820152608401611e69565b6001600160a01b0385163b612f235760405162461bcd60e51b815260206004820152601d60248201527f416464726573733a2063616c6c20746f206e6f6e2d636f6e74726163740000006044820152606401611e69565b5f5f866001600160a01b03168587604051612f3e919061433c565b5f6040518083038185875af1925050503d805f8114612f78576040519150601f19603f3d011682016040523d82523d5f602084013e612f7d565b606091505b5091509150612f8d828286613320565b979650505050505050565b5f5f5f612fa58585613359565b909250905060018201612fbd575f1992505050610638565b80612fc98360256141bd565b612fd391906141bd565b61294a9060046141bd565b5f60208451612fed919061437e565b15612ff957505f611d5a565b83515f0361300857505f611d5a565b81855f5b86518110156130765761302060028461437e565b6001036130445761303d6130378883016020015190565b83613397565b915061305d565b61305a826130558984016020015190565b613397565b91505b60019290921c9161306f6020826141bd565b905061300c565b5090931495945050505050565b5f8061309a6130938460486141bd565b8590613312565b60e81c90505f846130ac85604b6141bd565b815181106130bc576130bc613c7c565b016020015160f81c90505f6130ee835f60108262ffffff16901c8261ff001660108462ffffff16901b17179050919050565b62ffffff1690505f613101600384614391565b60ff1690506131128161010061448d565b612f8d9083613b55565b5f610ad08284613b99565b5f8061313385856133a2565b9050828114613145575f915050610ad0565b506001949350505050565b5f60205f8385602001870160025afa5060205f60205f60025afa50505f519392505050565b5f5f5f61318285856133ba565b90508060ff165f036131b5575f8585815181106131a1576131a1613c7c565b016020015190935060f81c915061330b9050565b836131c1826001614498565b60ff166131ce91906141bd565b855110156131e3575f195f925092505061330b565b5f8160ff166002036132265761321b6132076132008760016141bd565b8890613312565b62ffff0060e882901c1660f89190911c1790565b61ffff169050613301565b8160ff16600403613275576132686132426132008760016141bd565b60d881901c63ff00ff001662ff00ff60e89290921c9190911617601081811b91901c1790565b63ffffffff169050613301565b8160ff16600803613301576132f46132916132008760016141bd565b60c01c64ff000000ff600882811c91821665ff000000ff009390911b92831617601090811b67ffffffffffffffff1666ff00ff00ff00ff9290921667ff00ff00ff00ff009093169290921790911c65ffff0000ffff1617602081811c91901b1790565b67ffffffffffffffff1690505b60ff909116925090505b9250929050565b5f610ad08383016020015190565b6060831561332f575081610ad0565b82511561333f5782518084602001fd5b8160405162461bcd60e51b8152600401611e6991906144b1565b5f806133668360256141bd565b8451101561337957505f1990505f61330b565b5f8061338a86612beb8760246141bd565b9097909650945050505050565b5f610ad0838361343e565b5f610ad06133b18360046141bd565b84016020015190565b5f8282815181106133cd576133cd613c7c565b016020015160f81c60ff036133e457506008610638565b8282815181106133f6576133f6613c7c565b016020015160f81c60fe0361340d57506004610638565b82828151811061341f5761341f613c7c565b016020015160f81c60fd0361343657506002610638565b505f92915050565b5f825f528160205260205f60405f60025afa5060205f60205f60025afa50505f5192915050565b6040518060a001604052805f815260200161348c6040518060200160405280606081525090565b81525f602082018190526040820181905260609091015290565b5080546134b290613c31565b5f825580601f106134c1575050565b601f0160209004905f5260205f20908101906134dd9190613537565b50565b6040518061010001604052805f81526020016135086040518060200160405280606081525090565b81525f6020820181905260408201819052606082018190526080820181905260a0820181905260c09091015290565b5b8082111561354b575f8155600101613538565b5090565b5f5f60408385031215613560575f5ffd5b50508035926020909101359150565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b5f815160208452611d5a602085018261356f565b5f8151808452602084019350602083015f5b828110156135e15781518652602095860195909101906001016135c3565b5093949350505050565b5f604082016040835280855180835260608501915060608160051b8601019250602087015f5b828110156136ad577fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa0878603018452815180518652602081015160a0602088015261365f60a088018261359d565b90506001600160a01b036040830151166040880152606082015160608801526001600160a01b0360808301511660808801528096505050602082019150602084019350600181019050613611565b50505050828103602084015261294a81856135b1565b5f602082840312156136d3575f5ffd5b50919050565b5f5f5f606084860312156136eb575f5ffd5b83359250602084013567ffffffffffffffff811115613708575f5ffd5b613714868287016136c3565b93969395505050506040919091013590565b80356001600160a01b038116811461373c575f5ffd5b919050565b5f5f5f5f60808587031215613754575f5ffd5b84359350602085013567ffffffffffffffff811115613771575f5ffd5b61377d878288016136c3565b93505061378c60408601613726565b9396929550929360600135925050565b5f602082840312156137ac575f5ffd5b5035919050565b85815260a060208201525f6137cb60a083018761359d565b90506001600160a01b03851660408301528360608301526001600160a01b03831660808301529695505050505050565b88815261010060208201525f61381561010083018a61359d565b6040830198909852506001600160a01b039586166060820152608081019490945291841660a084015290921660c082015260e0015292915050565b5f60208284031215613860575f5ffd5b610ad082613726565b5f5f5f6060848603121561387b575f5ffd5b8335925061388b60208501613726565b929592945050506040919091013590565b604080825283519082018190525f9060208501906060840190835b8181101561390d578351805184526001600160a01b036020820151166020850152604081015160408501526001600160a01b036060820151166060850152506080830192506020840193506001810190506138b7565b5050838103602085015261392181866135b1565b9695505050505050565b604080825283519082018190525f9060208501906060840190835b8181101561390d57835180518452602081015160208501526001600160a01b036040820151166040850152606081015160608501526001600160a01b0360808201511660808501526001600160a01b0360a08201511660a085015260c081015160c08501525060e083019250602084019350600181019050613946565b5f604082016040835280855180835260608501915060608160051b8601019250602087015f5b828110156136ad577fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa087860301845281518051865260208101516101006020880152613a3961010088018261359d565b9050604082015160408801526001600160a01b036060830151166060880152608082015160808801526001600160a01b0360a08301511660a088015260c0820151613a8f60c08901826001600160a01b03169052565b5060e091820151969091019590955260209384019391909101906001016139e9565b5f5f5f60608486031215613ac3575f5ffd5b83359250602084013567ffffffffffffffff811115613ae0575f5ffd5b840160808187031215613af1575f5ffd5b9150604084013567ffffffffffffffff811115613b0c575f5ffd5b840160a08187031215613b1d575f5ffd5b809150509250925092565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b808202811582820484141761063857610638613b28565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f82613ba757613ba7613b6c565b500490565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52600160045260245ffd5b8181038181111561063857610638613b28565b5f5f198203613bfd57613bfd613b28565b5060010190565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b600181811c90821680613c4557607f821691505b6020821081036136d3577f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b60405160a0810167ffffffffffffffff81118282101715613ccc57613ccc613c04565b60405290565b5f82601f830112613ce1575f5ffd5b813567ffffffffffffffff811115613cfb57613cfb613c04565b604051601f8201601f19908116603f0116810167ffffffffffffffff81118282101715613d2a57613d2a613c04565b604052818152838201602001851015613d41575f5ffd5b816020850160208301375f918101602001919091529392505050565b5f60208236031215613d6d575f5ffd5b6040516020810167ffffffffffffffff81118282101715613d9057613d90613c04565b604052823567ffffffffffffffff811115613da9575f5ffd5b613db536828601613cd2565b82525092915050565b601f821115611d3057805f5260205f20601f840160051c81016020851015613de35750805b601f840160051c820191505b81811015611e72575f8155600101613def565b815167ffffffffffffffff811115613e1c57613e1c613c04565b613e3081613e2a8454613c31565b84613dbe565b6020601f821160018114613e62575f8315613e4b5750848201515b5f19600385901b1c1916600184901b178455611e72565b5f84815260208120601f198516915b82811015613e915787850151825560209485019460019092019101613e71565b5084821015613eae57868401515f19600387901b60f8161c191681555b50505050600190811b01905550565b81835281816020850137505f602082840101525f6020601f19601f840116840101905092915050565b5f81357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe1833603018112613f18575f5ffd5b820160208101903567ffffffffffffffff811115613f34575f5ffd5b803603821315613f42575f5ffd5b6020855261294a602086018284613ebd565b608081525f613f666080830187613ee6565b60208301959095525060408101929092526001600160a01b0316606090910152919050565b848152608060208201525f613fa36080830186613ee6565b6001600160a01b03949094166040830152506060015292915050565b5f5f83357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe1843603018112613ff2575f5ffd5b83018035915067ffffffffffffffff82111561400c575f5ffd5b60200191503681900382131561330b575f5ffd5b602081525f611d5a602083018486613ebd565b80357fffffffff000000000000000000000000000000000000000000000000000000008116811461373c575f5ffd5b5f60808236031215614072575f5ffd5b6040516080810167ffffffffffffffff8111828210171561409557614095613c04565b6040526140a183614033565b8152602083013567ffffffffffffffff8111156140bc575f5ffd5b6140c836828601613cd2565b602083015250604083013567ffffffffffffffff8111156140e7575f5ffd5b6140f336828601613cd2565b60408301525061410560608401614033565b606082015292915050565b5f60a08236031215614120575f5ffd5b614128613ca9565b823567ffffffffffffffff81111561413e575f5ffd5b61414a36828601613cd2565b82525060208381013590820152604083013567ffffffffffffffff811115614170575f5ffd5b61417c36828601613cd2565b60408301525060608381013590820152608083013567ffffffffffffffff8111156141a5575f5ffd5b6141b136828601613cd2565b60808301525092915050565b8082018082111561063857610638613b28565b7fff000000000000000000000000000000000000000000000000000000000000008360f81b1681525f5f835461420581613c31565b60018216801561421c57600181146142555761428b565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff008316600187015260018215158302870101935061428b565b865f5260205f205f5b838110156142805781546001828a01015260018201915060208101905061425e565b505060018287010193505b50919695505050505050565b5f602082840312156142a7575f5ffd5b81518015158114610ad0575f5ffd5b5f81518060208401855e5f93019283525090919050565b7fffffffff00000000000000000000000000000000000000000000000000000000851681525f61430961430360048401876142b6565b856142b6565b7fffffffff0000000000000000000000000000000000000000000000000000000093909316835250506004019392505050565b5f610ad082846142b6565b5f60208284031215614357575f5ffd5b5051919050565b67ffffffffffffffff818116838216019081111561063857610638613b28565b5f8261438c5761438c613b6c565b500690565b60ff828116828216039081111561063857610638613b28565b6001815b60018411156143e5578085048111156143c9576143c9613b28565b60018416156143d757908102905b60019390931c9280026143ae565b935093915050565b5f826143fb57506001610638565b8161440757505f610638565b816001811461441d576002811461442757614443565b6001915050610638565b60ff84111561443857614438613b28565b50506001821b610638565b5060208310610133831016604e8410600b8410161715614466575081810a610638565b6144725f1984846143aa565b805f190482111561448557614485613b28565b029392505050565b5f610ad083836143ed565b60ff818116838216019081111561063857610638613b28565b602081525f610ad0602083018461356f56fea264697066735822122041df18e578a041c2e52d8f05eb633ba96c00113b60873f78cbe14aff1cb17f0764736f6c634300081c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15`\x0EW__\xFD[P`@QaE\xBA8\x03\x80aE\xBA\x839\x81\x01`@\x81\x90R`+\x91`\x81V[_\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x17\x90U`\x06\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x17\x90UPP`\x01`\x07U`\xB4V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14`~W__\xFD[PV[__`@\x83\x85\x03\x12\x15`\x91W__\xFD[\x82Q`\x9A\x81`kV[` \x84\x01Q\x90\x92P`\xA9\x81`kV[\x80\x91PP\x92P\x92\x90PV[aD\xF9\x80a\0\xC1_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x01nW_5`\xE0\x1C\x80cj\x8C\xDE:\x11a\0\xD2W\x80c\xC5jE&\x11a\0\x88W\x80c\xDFi\xB1O\x11a\0cW\x80c\xDFi\xB1O\x14a\x03\xB2W\x80c\xEC\xCA,6\x14a\x03\xC5W\x80c\xFD?\xC2E\x14a\x042W__\xFD[\x80c\xC5jE&\x14a\x03|W\x80c\xCE\x1B\x81_\x14a\x03\x8FW\x80c\xD1\x92\x0F\xF0\x14a\x03\xA9W__\xFD[\x80c\xA3\x83\x01;\x11a\0\xB8W\x80c\xA3\x83\x01;\x14a\x02\xBAW\x80c\xB2#\xD9v\x14a\x02\xCDW\x80c\xBD*~>\x14a\x02\xE0W__\xFD[\x80cj\x8C\xDE:\x14a\x02\x8EW\x80c\x9C\xC6r.\x14a\x02\xA4W__\xFD[\x80cAEd\n\x11a\x01'W\x80cW+l\x05\x11a\x01\rW\x80cW+l\x05\x14a\x024W\x80c[\x8F\xE0B\x14a\x02eW\x80ch\x11\xA3\x11\x14a\x02xW__\xFD[\x80cAEd\n\x14a\x01\xFAW\x80cPj\x10\x9D\x14a\x02!W__\xFD[\x80c!\x0E\xC1\x81\x11a\x01WW\x80c!\x0E\xC1\x81\x14a\x01\xAEW\x80c6O\x1E\xC0\x14a\x01\xC1W\x80c:\xF3\xFC~\x14a\x01\xD6W__\xFD[\x80c\x11\xC17\xAA\x14a\x01rW\x80c\x1D\xFEu\x95\x14a\x01\x98W[__\xFD[a\x01\x85a\x01\x806`\x04a5OV[a\x04EV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\xA0a\x06>V[`@Qa\x01\x8F\x92\x91\x90a5\xEBV[a\x01\x85a\x01\xBC6`\x04a6\xD9V[a\x08\xA0V[a\x01\xD4a\x01\xCF6`\x04a7AV[a\n\xD7V[\0[a\x01\xE9a\x01\xE46`\x04a7\x9CV[a\x0C,V[`@Qa\x01\x8F\x95\x94\x93\x92\x91\x90a7\xB3V[a\x02\ra\x02\x086`\x04a7\x9CV[a\x0C\xFEV[`@Qa\x01\x8F\x98\x97\x96\x95\x94\x93\x92\x91\x90a7\xFBV[a\x01\xD4a\x02/6`\x04a7\x9CV[a\r\xEFV[a\x02Ua\x02B6`\x04a8PV[_T`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x91\x16\x14\x90V[`@Q\x90\x15\x15\x81R` \x01a\x01\x8FV[a\x01\xD4a\x02s6`\x04a8iV[a\x0E\xD4V[a\x02\x80a\x0F\xF0V[`@Qa\x01\x8F\x92\x91\x90a8\x9CV[a\x02\x96a\x11\xB4V[`@Qa\x01\x8F\x92\x91\x90a9+V[a\x02\xACa\x13\xBAV[`@Qa\x01\x8F\x92\x91\x90a9\xC3V[a\x01\xD4a\x02\xC86`\x04a7\x9CV[a\x16\x1DV[a\x01\xD4a\x02\xDB6`\x04a:\xB1V[a\x16\xC1V[a\x039a\x02\xEE6`\x04a7\x9CV[`\x02` \x81\x90R_\x91\x82R`@\x90\x91 \x80T`\x01\x82\x01T\x92\x82\x01T`\x03\x83\x01T`\x04\x84\x01T`\x05\x85\x01T`\x06\x90\x95\x01T\x93\x95\x94`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x94\x92\x93\x91\x82\x16\x92\x91\x16\x90\x87V[`@\x80Q\x97\x88R` \x88\x01\x96\x90\x96R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x95\x87\x01\x95\x90\x95R``\x86\x01\x92\x90\x92R\x82\x16`\x80\x85\x01R\x16`\xA0\x83\x01R`\xC0\x82\x01R`\xE0\x01a\x01\x8FV[a\x01\xD4a\x03\x8A6`\x04a7\x9CV[a\x18]V[_T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\x8FV[a\x01\x85aT`\x81V[a\x01\xD4a\x03\xC06`\x04a7\x9CV[a\x19@V[a\x04\x07a\x03\xD36`\x04a7\x9CV[`\x03` \x81\x90R_\x91\x82R`@\x90\x91 \x80T`\x01\x82\x01T`\x02\x83\x01T\x92\x90\x93\x01T\x90\x92`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x92\x91\x16\x84V[`@\x80Q\x94\x85R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16` \x86\x01R\x84\x01\x91\x90\x91R\x16``\x82\x01R`\x80\x01a\x01\x8FV[a\x01\xD4a\x04@6`\x04a:\xB1V[a\x1ASV[_\x82\x81R`\x01` R`@\x81 \x80T\x83\x11\x15a\x04_W__\xFD[_\x83\x11a\x04jW__\xFD[\x80T`\x03\x82\x01T_\x91\x90a\x04~\x90\x86a;UV[a\x04\x88\x91\x90a;\x99V[\x90P_\x81\x11a\x04\x99Wa\x04\x99a;\xACV[\x80\x82`\x03\x01T\x10\x15a\x04\xADWa\x04\xADa;\xACV[\x80\x82`\x03\x01_\x82\x82Ta\x04\xC0\x91\x90a;\xD9V[\x90\x91UPP\x81T\x84\x90\x83\x90_\x90a\x04\xD8\x90\x84\x90a;\xD9V[\x90\x91UPP`@\x80Q`\xE0\x81\x01\x82R\x86\x81R` \x81\x01\x86\x90R`\x02\x84\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x83\x90R`\x04\x84\x01T\x90\x91\x16`\x80\x82\x01R_\x90`\xA0\x81\x01a\x05*a\x1B\xE0V[`\x01`\x01`\xA0\x1B\x03\x16\x81RB` \x90\x91\x01R`\x05\x80T\x91\x92P_\x91\x90\x82a\x05P\x83a;\xECV[\x90\x91UP_\x81\x81R`\x02` \x81\x81R`@\x92\x83\x90 \x86Q\x81U\x86\x82\x01Q`\x01\x82\x01U\x86\x84\x01Q\x81\x84\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x17\x90\x91U``\x80\x8A\x01Q`\x03\x85\x01U`\x80\x8A\x01Q`\x04\x85\x01\x80T\x84\x16\x91\x85\x16\x91\x90\x91\x17\x90U`\xA0\x8A\x01Q`\x05\x85\x01\x80T\x90\x93\x16\x90\x84\x16\x17\x90\x91U`\xC0\x89\x01Q`\x06\x90\x93\x01\x92\x90\x92U\x92\x89\x01T\x84Q\x8C\x81R\x92\x83\x01\x89\x90R\x90\x92\x16\x92\x81\x01\x92\x90\x92R\x91\x92P\x82\x91\x89\x91\x7F\xC3\x9A\x1A]\xDC\x0E\x85\xC9U\xFE.\x1A\xBE\xB4<\x94\xCE\x182.u\xBB=D\xE8\x0Fu\x9F\xF9\xD04\xB9\x91\x01`@Q\x80\x91\x03\x90\xA3\x93PPPP[\x92\x91PPV[``\x80_\x80[`\x05T\x81\x10\x15a\x06\x83W_\x81\x81R`\x01` R`@\x90 `\x04\x01T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x06{W\x81a\x06w\x81a;\xECV[\x92PP[`\x01\x01a\x06DV[P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06\x9EWa\x06\x9Ea<\x04V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x06\xD7W\x81` \x01[a\x06\xC4a4eV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x06\xBCW\x90P[P\x90P_\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06\xF4Wa\x06\xF4a<\x04V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x07\x1DW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_\x80[`\x05T\x81\x10\x15a\x08\x94W_\x81\x81R`\x01` R`@\x90 `\x04\x01T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x08\x8CW`\x01_\x82\x81R` \x01\x90\x81R` \x01_ `@Q\x80`\xA0\x01`@R\x90\x81_\x82\x01T\x81R` \x01`\x01\x82\x01`@Q\x80` \x01`@R\x90\x81_\x82\x01\x80Ta\x07\x90\x90a<1V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x07\xBC\x90a<1V[\x80\x15a\x08\x07W\x80`\x1F\x10a\x07\xDEWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x08\x07V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x07\xEAW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x90\x92RPPP\x81R`\x02\x82\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16` \x83\x01R`\x03\x83\x01T`@\x83\x01R`\x04\x90\x92\x01T\x90\x91\x16``\x90\x91\x01R\x84Q\x85\x90\x84\x90\x81\x10a\x08UWa\x08Ua<|V[` \x02` \x01\x01\x81\x90RP\x80\x83\x83\x81Q\x81\x10a\x08sWa\x08sa<|V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x81a\x08\x88\x81a;\xECV[\x92PP[`\x01\x01a\x07#V[P\x91\x95\x90\x94P\x92PPPV[_\x83\x81R`\x03` R`@\x81 \x82a\x08\xB6W__\xFD[\x80T\x83\x11\x15a\x08\xC3W__\xFD[\x80T`\x02\x82\x01T_\x91\x90a\x08\xD7\x90\x86a;UV[a\x08\xE1\x91\x90a;\x99V[\x90P_\x81\x11a\x08\xF2Wa\x08\xF2a;\xACV[\x80\x82`\x02\x01T\x10\x15a\t\x06Wa\t\x06a;\xACV[\x80\x82`\x02\x01_\x82\x82Ta\t\x19\x91\x90a;\xD9V[\x90\x91UPP\x81T\x84\x90\x83\x90_\x90a\t1\x90\x84\x90a;\xD9V[\x90\x91UPa\tX\x90Pa\tBa\x1B\xE0V[`\x01\x84\x01T`\x01`\x01`\xA0\x1B\x03\x16\x900\x84a\x1C0V[`\x05\x80T_\x91\x82a\th\x83a;\xECV[\x91\x90PU\x90P`@Q\x80a\x01\0\x01`@R\x80\x88\x81R` \x01\x87a\t\x8A\x90a=]V[\x81R` \x81\x01\x87\x90R`\x01\x85\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`@\x83\x01R``\x82\x01\x85\x90R`\x03\x86\x01T\x16`\x80\x82\x01R`\xA0\x01a\t\xC5a\x1B\xE0V[`\x01`\x01`\xA0\x1B\x03\x16\x81RB` \x91\x82\x01R_\x83\x81R`\x04\x82R`@\x90 \x82Q\x81U\x90\x82\x01Q\x80Q`\x01\x83\x01\x90\x81\x90a\t\xFE\x90\x82a>\x02V[PPP`@\x82\x81\x01Q`\x02\x83\x01U``\x83\x01Q`\x03\x83\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x17\x90\x91U`\x80\x85\x01Q`\x04\x85\x01U`\xA0\x85\x01Q`\x05\x85\x01\x80T\x83\x16\x91\x84\x16\x91\x90\x91\x17\x90U`\xC0\x85\x01Q`\x06\x85\x01\x80T\x90\x92\x16\x90\x83\x16\x17\x90U`\xE0\x90\x93\x01Q`\x07\x90\x92\x01\x91\x90\x91U`\x01\x85\x01T\x90Q\x83\x92\x8A\x92\x7Fe>\r\x81\xF2\xC9\x9B\xEB\xA3Y\xDF\xB1{I\x9A\\\xFF+\xE9\xD9PQHR\"M\xF8\xC0\x97\xC2\x19!\x92a\n\xC3\x92\x8C\x92\x8C\x92\x8A\x92\x91\x90\x91\x16\x90a?TV[`@Q\x80\x91\x03\x90\xA3\x92PPP[\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\n\xE9W__\xFD[a\x0B\x06a\n\xF4a\x1B\xE0V[`\x01`\x01`\xA0\x1B\x03\x84\x16\x900\x84a\x1C0V[`\x05\x80T_\x91\x82a\x0B\x16\x83a;\xECV[\x91\x90PU\x90P`@Q\x80`\xA0\x01`@R\x80\x86\x81R` \x01\x85a\x0B7\x90a=]V[\x81R` \x01\x84`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83\x81R` \x01a\x0BYa\x1B\xE0V[`\x01`\x01`\xA0\x1B\x03\x16\x90R_\x82\x81R`\x01` \x81\x81R`@\x90\x92 \x83Q\x81U\x91\x83\x01Q\x80Q\x90\x91\x83\x01\x90\x81\x90a\x0B\x8F\x90\x82a>\x02V[PPP`@\x82\x81\x01Q`\x02\x83\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x17\x90\x91U``\x85\x01Q`\x03\x85\x01U`\x80\x90\x94\x01Q`\x04\x90\x93\x01\x80T\x90\x94\x16\x92\x16\x91\x90\x91\x17\x90\x91UQ\x7F\x98\xC7\xC6\x80@=G@=\xEAJW\r\x0ElW\x16S\x8CIB\x0E\xF4q\xCE\xC4(\xF5\xA5\x85,\x06\x90a\x0C\x1D\x90\x87\x90\x87\x90\x87\x90\x87\x90a?\x8BV[`@Q\x80\x91\x03\x90\xA1PPPPPV[`\x01` \x81\x81R_\x92\x83R`@\x92\x83\x90 \x80T\x84Q\x92\x83\x01\x90\x94R\x91\x82\x01\x80T\x82\x90\x82\x90a\x0CY\x90a<1V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0C\x85\x90a<1V[\x80\x15a\x0C\xD0W\x80`\x1F\x10a\x0C\xA7Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0C\xD0V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0C\xB3W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x90\x92RPPP`\x02\x82\x01T`\x03\x83\x01T`\x04\x90\x93\x01T\x91\x92`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x92\x90\x91\x16\x85V[`\x04` R\x80_R`@_ _\x91P\x90P\x80_\x01T\x90\x80`\x01\x01`@Q\x80` \x01`@R\x90\x81_\x82\x01\x80Ta\r2\x90a<1V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\r^\x90a<1V[\x80\x15a\r\xA9W\x80`\x1F\x10a\r\x80Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\r\xA9V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\r\x8CW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x90\x92RPPP`\x02\x82\x01T`\x03\x83\x01T`\x04\x84\x01T`\x05\x85\x01T`\x06\x86\x01T`\x07\x90\x96\x01T\x94\x95\x93\x94`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x94\x92\x93\x91\x82\x16\x92\x90\x91\x16\x90\x88V[_\x81\x81R`\x01` R`@\x90 a\x0E\x04a\x1B\xE0V[`\x04\x82\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x16\x14a\x0E\x1FW__\xFD[a\x0EDa\x0E*a\x1B\xE0V[`\x03\x83\x01T`\x02\x84\x01T`\x01`\x01`\xA0\x1B\x03\x16\x91\x90a\x1C\xE7V[_\x82\x81R`\x01` \x81\x90R`@\x82 \x82\x81U\x91\x90\x82\x01\x81a\x0Ee\x82\x82a4\xA6V[PPP`\x02\x81\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x81\x16\x90\x91U_`\x03\x83\x01U`\x04\x90\x91\x01\x80T\x90\x91\x16\x90U`@Q\x82\x81R\x7F\xC3@\xE7\xACH\xDC\x80\xEEy?\xC6&i`\xBD_\x1B\xD2\x1B\xE9\x1C\x8A\x95\xE2\x18\x17\x81\x13\xF7\x9E\x17\xB4\x90` \x01[`@Q\x80\x91\x03\x90\xA1PPV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x0E\xE6W__\xFD[_\x83\x11a\x0E\xF1W__\xFD[_\x81\x11a\x0E\xFCW__\xFD[`\x05\x80T_\x91\x82a\x0F\x0C\x83a;\xECV[\x91\x90PU\x90P`@Q\x80`\x80\x01`@R\x80\x85\x81R` \x01\x84`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83\x81R` \x01a\x0F@a\x1B\xE0V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x90\x91R_\x83\x81R`\x03` \x81\x81R`@\x92\x83\x90 \x85Q\x81U\x85\x82\x01Q`\x01\x82\x01\x80T\x91\x87\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x92\x83\x16\x17\x90U\x86\x85\x01Q`\x02\x83\x01U``\x96\x87\x01Q\x91\x90\x93\x01\x80T\x91\x86\x16\x91\x90\x93\x16\x17\x90\x91U\x81Q\x88\x81R\x92\x87\x16\x90\x83\x01R\x81\x01\x84\x90R\x82\x91\x7F\xFF\x1C\xE2\x10\xDE\xFC\xD3\xBA\x1A\xDFv\xC9A\x9A\x07X\xFA`\xFD>\xB3\x8C{\xD9A\x8F`\xB5u\xB7n$\x91\x01`@Q\x80\x91\x03\x90\xA2PPPPV[``\x80_\x80[`\x05T\x81\x10\x15a\x106W_\x81\x81R`\x03` \x81\x90R`@\x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x10.W\x81a\x10*\x81a;\xECV[\x92PP[`\x01\x01a\x0F\xF6V[P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x10QWa\x10Qa<\x04V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x10\xA1W\x81` \x01[`@\x80Q`\x80\x81\x01\x82R_\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01\x81\x90R``\x82\x01R\x82R_\x19\x90\x92\x01\x91\x01\x81a\x10oW\x90P[P\x90P_\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x10\xBEWa\x10\xBEa<\x04V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x10\xE7W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_\x80[`\x05T\x81\x10\x15a\x08\x94W_\x81\x81R`\x03` \x81\x90R`@\x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x11\xACW_\x81\x81R`\x03` \x81\x81R`@\x92\x83\x90 \x83Q`\x80\x81\x01\x85R\x81T\x81R`\x01\x82\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x93\x82\x01\x93\x90\x93R`\x02\x82\x01T\x94\x81\x01\x94\x90\x94R\x90\x91\x01T\x16``\x82\x01R\x84Q\x85\x90\x84\x90\x81\x10a\x11uWa\x11ua<|V[` \x02` \x01\x01\x81\x90RP\x80\x83\x83\x81Q\x81\x10a\x11\x93Wa\x11\x93a<|V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x81a\x11\xA8\x81a;\xECV[\x92PP[`\x01\x01a\x10\xEDV[``\x80_\x80[`\x05T\x81\x10\x15a\x11\xF0W_\x81\x81R`\x02` R`@\x90 `\x01\x01T\x15a\x11\xE8W\x81a\x11\xE4\x81a;\xECV[\x92PP[`\x01\x01a\x11\xBAV[P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x12\x0BWa\x12\x0Ba<\x04V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x12\x90W\x81` \x01[a\x12}`@Q\x80`\xE0\x01`@R\x80_\x81R` \x01_\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x12)W\x90P[P\x90P_\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x12\xADWa\x12\xADa<\x04V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x12\xD6W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_\x80[`\x05T\x81\x10\x15a\x08\x94W_\x81\x81R`\x02` R`@\x90 `\x01\x01T\x15a\x13\xB2W_\x81\x81R`\x02` \x81\x81R`@\x92\x83\x90 \x83Q`\xE0\x81\x01\x85R\x81T\x81R`\x01\x82\x01T\x92\x81\x01\x92\x90\x92R\x91\x82\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x93\x82\x01\x93\x90\x93R`\x03\x82\x01T``\x82\x01R`\x04\x82\x01T\x83\x16`\x80\x82\x01R`\x05\x82\x01T\x90\x92\x16`\xA0\x83\x01R`\x06\x01T`\xC0\x82\x01R\x84Q\x85\x90\x84\x90\x81\x10a\x13{Wa\x13{a<|V[` \x02` \x01\x01\x81\x90RP\x80\x83\x83\x81Q\x81\x10a\x13\x99Wa\x13\x99a<|V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x81a\x13\xAE\x81a;\xECV[\x92PP[`\x01\x01a\x12\xDCV[``\x80_\x80[`\x05T\x81\x10\x15a\x13\xF6W_\x81\x81R`\x04` R`@\x90 `\x02\x01T\x15a\x13\xEEW\x81a\x13\xEA\x81a;\xECV[\x92PP[`\x01\x01a\x13\xC0V[P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x14\x11Wa\x14\x11a<\x04V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x14JW\x81` \x01[a\x147a4\xE0V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x14/W\x90P[P\x90P_\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x14gWa\x14ga<\x04V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x14\x90W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_\x80[`\x05T\x81\x10\x15a\x08\x94W_\x81\x81R`\x04` R`@\x90 `\x02\x01T\x15a\x16\x15W`\x04_\x82\x81R` \x01\x90\x81R` \x01_ `@Q\x80a\x01\0\x01`@R\x90\x81_\x82\x01T\x81R` \x01`\x01\x82\x01`@Q\x80` \x01`@R\x90\x81_\x82\x01\x80Ta\x14\xFB\x90a<1V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x15'\x90a<1V[\x80\x15a\x15rW\x80`\x1F\x10a\x15IWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x15rV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x15UW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x90\x92RPPP\x81R`\x02\x82\x01T` \x82\x01R`\x03\x82\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`@\x83\x01R`\x04\x83\x01T``\x83\x01R`\x05\x83\x01T\x81\x16`\x80\x83\x01R`\x06\x83\x01T\x16`\xA0\x82\x01R`\x07\x90\x91\x01T`\xC0\x90\x91\x01R\x84Q\x85\x90\x84\x90\x81\x10a\x15\xDEWa\x15\xDEa<|V[` \x02` \x01\x01\x81\x90RP\x80\x83\x83\x81Q\x81\x10a\x15\xFCWa\x15\xFCa<|V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x81a\x16\x11\x81a;\xECV[\x92PP[`\x01\x01a\x14\x96V[_\x81\x81R`\x03` R`@\x90 a\x162a\x1B\xE0V[`\x03\x82\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x16\x14a\x16MW__\xFD[_\x82\x81R`\x03` \x81\x81R`@\x80\x84 \x84\x81U`\x01\x81\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x81\x16\x90\x91U`\x02\x82\x01\x95\x90\x95U\x90\x92\x01\x80T\x90\x93\x16\x90\x92UQ\x83\x81R\x7F<\xD4u\xB0\x92\xE8\xB3y\xF6\xBA\r\x9E\x0E\x0C\x8F0p^s2\x1D\xC5\xC9\xF8\x0C\xE4\xAD8\xDB{\xE1\xAA\x91\x01a\x0E\xC8V[_\x83\x81R`\x02` R`@\x90 a\x16\xD6a\x1B\xE0V[`\x05\x82\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x16\x14a\x16\xF1W__\xFD[`\x06T`\x01`\x01`\xA0\x1B\x03\x16c\xD3\x8C)\xA1a\x17\x0F`@\x85\x01\x85a?\xBFV[`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x17,\x92\x91\x90a@ V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x17CW__\xFD[PZ\xF1\x15\x80\x15a\x17UW=__>=_\xFD[PPPPa\x17\x86`\x07T\x84a\x17i\x90a@bV[a\x17r\x85aA\x10V[`\x06T`\x01`\x01`\xA0\x1B\x03\x16\x92\x91\x90a\x1D5V[P\x80T_\x90\x81R`\x01` \x81\x90R`@\x90\x91 \x80T\x90\x91a\x17\xAA\x91\x90\x83\x01\x86a\x1DbV[`\x05\x82\x01T`\x03\x83\x01T`\x02\x84\x01Ta\x17\xD1\x92`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x92\x91\x16\x90a\x1C\xE7V[_\x85\x81R`\x02` \x81\x81R`@\x80\x84 \x84\x81U`\x01\x81\x01\x85\x90U\x92\x83\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x81\x16\x90\x91U`\x03\x84\x01\x85\x90U`\x04\x84\x01\x80T\x82\x16\x90U`\x05\x84\x01\x80T\x90\x91\x16\x90U`\x06\x90\x92\x01\x92\x90\x92UQ\x86\x81R\x7F\xB4\xC9\x8D\xE2\x10ik<\xF2\x1E\x993\\\x1E\xE3\xA0\xAE4\xA2g\x13A*J\xDD\xE8\xAFYav\xF3~\x91\x01a\x0C\x1DV[_\x81\x81R`\x02` R`@\x90 a\x18ra\x1B\xE0V[`\x04\x82\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x16\x14a\x18\x8DW__\xFD[aT`\x81`\x06\x01Ta\x18\x9F\x91\x90aA\xBDV[B\x11a\x18\xA9W__\xFD[a\x18\xB4a\x0E*a\x1B\xE0V[_\x82\x81R`\x02` \x81\x81R`@\x80\x84 \x84\x81U`\x01\x81\x01\x85\x90U\x92\x83\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x81\x16\x90\x91U`\x03\x84\x01\x85\x90U`\x04\x84\x01\x80T\x82\x16\x90U`\x05\x84\x01\x80T\x90\x91\x16\x90U`\x06\x90\x92\x01\x92\x90\x92UQ\x83\x81R\x7F>^\xA3X\xE9\xEBL\xDFD\xCD\xC7y8\xAD\xE8\x07K\x12@\xA6\xD8\xC0\xFD\x13r\x86q\xB8.\x80\n\xD6\x91\x01a\x0E\xC8V[_\x81\x81R`\x04` R`@\x90 `\x07\x81\x01Ta\x19_\x90aT`\x90aA\xBDV[B\x11a\x19iW__\xFD[a\x19qa\x1B\xE0V[`\x06\x82\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x16\x14a\x19\x8CW__\xFD[a\x19\xB1a\x19\x97a\x1B\xE0V[`\x04\x83\x01T`\x03\x84\x01T`\x01`\x01`\xA0\x1B\x03\x16\x91\x90a\x1C\xE7V[_\x82\x81R`\x04` R`@\x81 \x81\x81U\x90`\x01\x82\x01\x81a\x19\xD1\x82\x82a4\xA6V[PP_`\x02\x83\x01\x81\x90U`\x03\x83\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x81\x16\x90\x91U`\x04\x84\x01\x82\x90U`\x05\x84\x01\x80T\x82\x16\x90U`\x06\x84\x01\x80T\x90\x91\x16\x90U`\x07\x90\x92\x01\x91\x90\x91UP`@Q\x82\x81R\x7Fx\xF5\x1Fb\xF7\xCF\x13\x81\xC6s\xC2~\xAE\x18}\xD6\xC5\x88\xDCf$\xCEYi}\xBB>\x1D|\x1B\xBC\xDF\x90` \x01a\x0E\xC8V[_\x83\x81R`\x04` R`@\x90 a\x1Aha\x1B\xE0V[`\x05\x82\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x16\x14a\x1A\x83W__\xFD[`\x06T`\x01`\x01`\xA0\x1B\x03\x16c\xD3\x8C)\xA1a\x1A\xA1`@\x85\x01\x85a?\xBFV[`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1A\xBE\x92\x91\x90a@ V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1A\xD5W__\xFD[PZ\xF1\x15\x80\x15a\x1A\xE7W=__>=_\xFD[PPPPa\x1A\xFB`\x07T\x84a\x17i\x90a@bV[Pa\x1B\x0E\x81`\x02\x01T\x82`\x01\x01\x85a\x1DbV[`\x05\x81\x01T`\x04\x82\x01T`\x03\x83\x01Ta\x1B5\x92`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x92\x91\x16\x90a\x1C\xE7V[_\x84\x81R`\x04` R`@\x81 \x81\x81U\x90`\x01\x82\x01\x81a\x1BU\x82\x82a4\xA6V[PP_`\x02\x83\x01\x81\x90U`\x03\x83\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x81\x16\x90\x91U`\x04\x84\x01\x82\x90U`\x05\x84\x01\x80T\x82\x16\x90U`\x06\x84\x01\x80T\x90\x91\x16\x90U`\x07\x90\x92\x01\x91\x90\x91UP`@Q\x84\x81R\x7F\xCFV\x10a\xDBx\xF7\xBCQ\x8D7\xFE\x86q\x85\x14\xC6@\xCC\xC5\xC3\xF1)8(\xB9U\xE6\x8F\x19\xF5\xFB\x90` \x01`@Q\x80\x91\x03\x90\xA1PPPPV[_`\x146\x10\x80\x15\x90a\x1B\xFBWP_T`\x01`\x01`\xA0\x1B\x03\x163\x14[\x15a\x1C+WP\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xEC6\x015``\x1C\x90V[P3\x90V[`@Q`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`$\x83\x01R\x83\x16`D\x82\x01R`d\x81\x01\x82\x90Ra\x1C\xE1\x90\x85\x90\x7F#\xB8r\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90`\x84\x01[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x93\x16\x92\x90\x92\x17\x90\x91Ra\x1EyV[PPPPV[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16`$\x82\x01R`D\x81\x01\x82\x90Ra\x1D0\x90\x84\x90\x7F\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90`d\x01a\x1C}V[PPPV[_a\x1D?\x83a\x1F]V[\x90Pa\x1DK\x81\x83a MV[a\x1DZ\x85\x85\x84`@\x01Qa\"\xB1V[\x94\x93PPPPV[_\x82_\x01\x80Ta\x1Dq\x90a<1V[`@Qa\x1D\x83\x92P\x85\x90` \x01aA\xD0V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P_a\x1D\xEA\x83\x80`@\x01\x90a\x1D\xAF\x91\x90a?\xBFV[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RP\x86\x92Pa&\x01\x91PPV[Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x84\x81\x10\x15a\x1ErW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`;`$\x82\x01R\x7FBitcoin transaction amount is lo`D\x82\x01R\x7Fwer than in accepted order.\0\0\0\0\0`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[PPPPPV[_a\x1E\xCD\x82`@Q\x80`@\x01`@R\x80` \x81R` \x01\x7FSafeERC20: low-level call failed\x81RP\x85`\x01`\x01`\xA0\x1B\x03\x16a'\x9F\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x80Q\x90\x91P\x15a\x1D0W\x80\x80` \x01\x90Q\x81\x01\x90a\x1E\xEB\x91\x90aB\x97V[a\x1D0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01R\x7Fot succeed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x1EiV[_a\x1Fk\x82` \x01Qa'\xADV[a\x1F\xB7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FInvalid input vector provided\0\0\0`D\x82\x01R`d\x01a\x1EiV[a\x1F\xC4\x82`@\x01Qa(GV[a \x10W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FInvalid output vector provided\0\0`D\x82\x01R`d\x01a\x1EiV[a\x068\x82_\x01Q\x83` \x01Q\x84`@\x01Q\x85``\x01Q`@Q` \x01a 9\x94\x93\x92\x91\x90aB\xCDV[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra(\xD4V[\x80Qa X\x90a(\xF6V[a \xA4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01R\x7FBad merkle array proof\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x1EiV[`\x80\x81\x01QQ\x81QQ\x14a! W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FTx not on same level of merkle t`D\x82\x01R\x7Free as coinbase\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x1EiV[_a!.\x82`@\x01Qa)\x0CV[\x82Q` \x84\x01Q\x91\x92Pa!E\x91\x85\x91\x84\x91a)\x18V[a!\xB7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`<`$\x82\x01R\x7FTx merkle proof is not valid for`D\x82\x01R\x7F provided header and tx hash\0\0\0\0`d\x82\x01R`\x84\x01a\x1EiV[_`\x02\x83``\x01Q`@Q` \x01a!\xD1\x91\x81R` \x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra!\xEB\x91aC<V[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\"\x06W=__>=_\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\")\x91\x90aCGV[`\x80\x84\x01Q\x90\x91Pa\"?\x90\x82\x90\x84\x90_a)\x18V[a\x1C\xE1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`?`$\x82\x01R\x7FCoinbase merkle proof is not val`D\x82\x01R\x7Fid for provided header and hash\0`d\x82\x01R`\x84\x01a\x1EiV[_\x83`\x01`\x01`\xA0\x1B\x03\x16c\x117d\xBE`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\"\xEEW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a#\x12\x91\x90aCGV[\x90P_\x84`\x01`\x01`\xA0\x1B\x03\x16c+\x97\xBE$`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a#QW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a#u\x91\x90aCGV[\x90P_\x80a#\x8Aa#\x85\x86a)SV[a)^V[\x90P\x83\x81\x03a#\x9BW\x83\x91Pa$\x18V[\x82\x81\x03a#\xAAW\x82\x91Pa$\x18V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FNot at current or previous diffi`D\x82\x01R\x7Fculty\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x1EiV[_a$\"\x86a)\x85V[\x90P_\x19\x81\x03a$\x9AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FInvalid length of the headers ch`D\x82\x01R\x7Fain\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x1EiV[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x03a%\tW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01R\x7FInvalid headers chain\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x1EiV[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFD\x81\x03a%xW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FInsufficient work in a header\0\0\0`D\x82\x01R`d\x01a\x1EiV[a%\x82\x87\x84a;UV[\x81\x10\x15a%\xF7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`3`$\x82\x01R\x7FInsufficient accumulated difficu`D\x82\x01R\x7Flty in header chain\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x1EiV[PPPPPPPPV[`@\x80Q``\x81\x01\x82R_\x80\x82R` \x80\x83\x01\x82\x90R\x82\x84\x01\x82\x90R\x83Q\x80\x85\x01\x90\x94R\x81\x84R\x83\x01R\x90a&5\x84a+\xA9V[` \x83\x01R\x80\x82R\x81a&G\x82a;\xECV[\x90RP_\x80[\x82` \x01Q\x81\x10\x15a'IW\x82Q_\x90a&h\x90\x88\x90a+\xBEV[\x84Q\x90\x91P_\x90a&z\x90\x89\x90a,\x1EV[\x90P_a&\x88`\x08\x84a;\xD9V[\x86Q\x90\x91P_\x90a&\x9A\x90`\x08aA\xBDV[\x8A\x81\x01` \x01\x83\x90 \x90\x91P\x80\x8A\x03a&\xD4W`\x01\x96P\x83\x89_\x01\x81\x81Qa&\xC2\x91\x90aC^V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90RPa'$V[_a&\xE2\x8C\x8A_\x01Qa,\x94V[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a'\x03W`\x01`\x01`\xA0\x1B\x03\x81\x16` \x8B\x01R[_a'\x11\x8D\x8B_\x01Qa-tV[\x90P\x80\x15a'!W`@\x8B\x01\x81\x90R[PP[\x84\x88_\x01\x81\x81Qa'5\x91\x90aA\xBDV[\x90RPP`\x01\x90\x94\x01\x93Pa&M\x92PPPV[P\x80a'\x97W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FNo output found for scriptPubKey`D\x82\x01R`d\x01a\x1EiV[PP\x92\x91PPV[``a\x1DZ\x84\x84_\x85a.TV[___a'\xB9\x84a+\xA9V[\x90\x92P\x90P\x80\x15\x80a'\xCBWP_\x19\x82\x14[\x15a'\xD9WP_\x93\x92PPPV[_a'\xE5\x83`\x01aA\xBDV[\x90P_[\x82\x81\x10\x15a(:W\x85Q\x82\x10a(\x04WP_\x95\x94PPPPPV[_a(\x0F\x87\x84a/\x98V[\x90P_\x19\x81\x03a(%WP_\x96\x95PPPPPPV[a(/\x81\x84aA\xBDV[\x92PP`\x01\x01a'\xE9V[P\x93Q\x90\x93\x14\x93\x92PPPV[___a(S\x84a+\xA9V[\x90\x92P\x90P\x80\x15\x80a(eWP_\x19\x82\x14[\x15a(sWP_\x93\x92PPPV[_a(\x7F\x83`\x01aA\xBDV[\x90P_[\x82\x81\x10\x15a(:W\x85Q\x82\x10a(\x9EWP_\x95\x94PPPPPV[_a(\xA9\x87\x84a+\xBEV[\x90P_\x19\x81\x03a(\xBFWP_\x96\x95PPPPPPV[a(\xC9\x81\x84aA\xBDV[\x92PP`\x01\x01a(\x83V[_` _\x83Q` \x85\x01`\x02Z\xFAP` _` _`\x02Z\xFAPP_Q\x91\x90PV[_` \x82Qa)\x05\x91\x90aC~V[\x15\x92\x91PPV[`D\x81\x01Q_\x90a\x068V[_\x83\x85\x14\x80\x15a)&WP\x81\x15[\x80\x15a)1WP\x82Q\x15[\x15a)>WP`\x01a\x1DZV[a)J\x85\x84\x86\x85a/\xDEV[\x95\x94PPPPPV[_a\x068\x82_a0\x83V[_a\x068{\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83a1\x1CV[_`P\x82Qa)\x94\x91\x90aC~V[\x15a)\xA1WP_\x19\x91\x90PV[P_\x80\x80[\x83Q\x81\x10\x15a+\xA2W\x80\x15a)\xEDWa)\xC0\x84\x82\x84a1'V[a)\xEDWP\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x93\x92PPPV[_a)\xF8\x85\x83a0\x83V[\x90Pa*\x06\x85\x83`Pa1PV[\x92P\x80a+I\x84_\x81\x90P`\x08\x81~\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\x16\x90\x1B`\x08\x82\x90\x1C~\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\x16\x17\x90P`\x10\x81}\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\x16\x90\x1B`\x10\x82\x90\x1C}\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\x16\x17\x90P` \x81{\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\x16\x90\x1B` \x82\x90\x1C{\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\x16\x17\x90P`@\x81w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x1B`@\x82\x90\x1Cw\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x17\x90P`\x80\x81\x90\x1B`\x80\x82\x90\x1C\x17\x90P\x91\x90PV[\x11\x15a+yWP\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFD\x94\x93PPPPV[a+\x82\x81a)^V[a+\x8C\x90\x85aA\xBDV[\x93Pa+\x9B\x90P`P\x82aA\xBDV[\x90Pa)\xA6V[PP\x91\x90PV[__a+\xB5\x83_a1uV[\x91P\x91P\x91P\x91V[_a+\xCA\x82`\taA\xBDV[\x83Q\x10\x15a+\xDAWP_\x19a\x068V[_\x80a+\xF0\x85a+\xEB\x86`\x08aA\xBDV[a1uV[\x90\x92P\x90P`\x01\x82\x01a,\x08W_\x19\x92PPPa\x068V[\x80a,\x14\x83`\taA\xBDV[a)J\x91\x90aA\xBDV[_\x80a,*\x84\x84a3\x12V[`\xC0\x1C\x90P_a)J\x82d\xFF\0\0\0\xFF`\x08\x82\x81\x1C\x91\x82\x16e\xFF\0\0\0\xFF\0\x93\x90\x91\x1B\x92\x83\x16\x17`\x10\x90\x81\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16f\xFF\0\xFF\0\xFF\0\xFF\x92\x90\x92\x16g\xFF\0\xFF\0\xFF\0\xFF\0\x90\x93\x16\x92\x90\x92\x17\x90\x91\x1Ce\xFF\xFF\0\0\xFF\xFF\x16\x17` \x81\x81\x1C\x91\x90\x1B\x17\x90V[_\x82a,\xA1\x83`\taA\xBDV[\x81Q\x81\x10a,\xB1Wa,\xB1a<|V[` \x91\x01\x01Q\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x7Fj\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x14a-\x06WP_a\x068V[_\x83a-\x13\x84`\naA\xBDV[\x81Q\x81\x10a-#Wa-#a<|V[\x01` \x01Q\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x91P`\xF8\x1C`\x14\x03a-mW_a-c\x84`\x0BaA\xBDV[\x85\x01`\x14\x01Q\x92PP[P\x92\x91PPV[_\x82a-\x81\x83`\taA\xBDV[\x81Q\x81\x10a-\x91Wa-\x91a<|V[` \x91\x01\x01Q\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x7Fj\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x14a-\xE6WP_a\x068V[_\x83a-\xF3\x84`\naA\xBDV[\x81Q\x81\x10a.\x03Wa.\x03a<|V[\x01` \x90\x81\x01Q\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x92P`\xF8\x1C\x90\x03a-mW_a.D\x84`\x0BaA\xBDV[\x85\x01` \x01Q\x92PPP\x92\x91PPV[``\x82G\x10\x15a.\xCCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FAddress: insufficient balance fo`D\x82\x01R\x7Fr call\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x1EiV[`\x01`\x01`\xA0\x1B\x03\x85\x16;a/#W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x1EiV[__\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@Qa/>\x91\x90aC<V[_`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80_\x81\x14a/xW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a/}V[``\x91P[P\x91P\x91Pa/\x8D\x82\x82\x86a3 V[\x97\x96PPPPPPPV[___a/\xA5\x85\x85a3YV[\x90\x92P\x90P`\x01\x82\x01a/\xBDW_\x19\x92PPPa\x068V[\x80a/\xC9\x83`%aA\xBDV[a/\xD3\x91\x90aA\xBDV[a)J\x90`\x04aA\xBDV[_` \x84Qa/\xED\x91\x90aC~V[\x15a/\xF9WP_a\x1DZV[\x83Q_\x03a0\x08WP_a\x1DZV[\x81\x85_[\x86Q\x81\x10\x15a0vWa0 `\x02\x84aC~V[`\x01\x03a0DWa0=a07\x88\x83\x01` \x01Q\x90V[\x83a3\x97V[\x91Pa0]V[a0Z\x82a0U\x89\x84\x01` \x01Q\x90V[a3\x97V[\x91P[`\x01\x92\x90\x92\x1C\x91a0o` \x82aA\xBDV[\x90Pa0\x0CV[P\x90\x93\x14\x95\x94PPPPPV[_\x80a0\x9Aa0\x93\x84`HaA\xBDV[\x85\x90a3\x12V[`\xE8\x1C\x90P_\x84a0\xAC\x85`KaA\xBDV[\x81Q\x81\x10a0\xBCWa0\xBCa<|V[\x01` \x01Q`\xF8\x1C\x90P_a0\xEE\x83_`\x10\x82b\xFF\xFF\xFF\x16\x90\x1C\x82a\xFF\0\x16`\x10\x84b\xFF\xFF\xFF\x16\x90\x1B\x17\x17\x90P\x91\x90PV[b\xFF\xFF\xFF\x16\x90P_a1\x01`\x03\x84aC\x91V[`\xFF\x16\x90Pa1\x12\x81a\x01\0aD\x8DV[a/\x8D\x90\x83a;UV[_a\n\xD0\x82\x84a;\x99V[_\x80a13\x85\x85a3\xA2V[\x90P\x82\x81\x14a1EW_\x91PPa\n\xD0V[P`\x01\x94\x93PPPPV[_` _\x83\x85` \x01\x87\x01`\x02Z\xFAP` _` _`\x02Z\xFAPP_Q\x93\x92PPPV[___a1\x82\x85\x85a3\xBAV[\x90P\x80`\xFF\x16_\x03a1\xB5W_\x85\x85\x81Q\x81\x10a1\xA1Wa1\xA1a<|V[\x01` \x01Q\x90\x93P`\xF8\x1C\x91Pa3\x0B\x90PV[\x83a1\xC1\x82`\x01aD\x98V[`\xFF\x16a1\xCE\x91\x90aA\xBDV[\x85Q\x10\x15a1\xE3W_\x19_\x92P\x92PPa3\x0BV[_\x81`\xFF\x16`\x02\x03a2&Wa2\x1Ba2\x07a2\0\x87`\x01aA\xBDV[\x88\x90a3\x12V[b\xFF\xFF\0`\xE8\x82\x90\x1C\x16`\xF8\x91\x90\x91\x1C\x17\x90V[a\xFF\xFF\x16\x90Pa3\x01V[\x81`\xFF\x16`\x04\x03a2uWa2ha2Ba2\0\x87`\x01aA\xBDV[`\xD8\x81\x90\x1Cc\xFF\0\xFF\0\x16b\xFF\0\xFF`\xE8\x92\x90\x92\x1C\x91\x90\x91\x16\x17`\x10\x81\x81\x1B\x91\x90\x1C\x17\x90V[c\xFF\xFF\xFF\xFF\x16\x90Pa3\x01V[\x81`\xFF\x16`\x08\x03a3\x01Wa2\xF4a2\x91a2\0\x87`\x01aA\xBDV[`\xC0\x1Cd\xFF\0\0\0\xFF`\x08\x82\x81\x1C\x91\x82\x16e\xFF\0\0\0\xFF\0\x93\x90\x91\x1B\x92\x83\x16\x17`\x10\x90\x81\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16f\xFF\0\xFF\0\xFF\0\xFF\x92\x90\x92\x16g\xFF\0\xFF\0\xFF\0\xFF\0\x90\x93\x16\x92\x90\x92\x17\x90\x91\x1Ce\xFF\xFF\0\0\xFF\xFF\x16\x17` \x81\x81\x1C\x91\x90\x1B\x17\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P[`\xFF\x90\x91\x16\x92P\x90P[\x92P\x92\x90PV[_a\n\xD0\x83\x83\x01` \x01Q\x90V[``\x83\x15a3/WP\x81a\n\xD0V[\x82Q\x15a3?W\x82Q\x80\x84` \x01\xFD[\x81`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x1Ei\x91\x90aD\xB1V[_\x80a3f\x83`%aA\xBDV[\x84Q\x10\x15a3yWP_\x19\x90P_a3\x0BV[_\x80a3\x8A\x86a+\xEB\x87`$aA\xBDV[\x90\x97\x90\x96P\x94PPPPPV[_a\n\xD0\x83\x83a4>V[_a\n\xD0a3\xB1\x83`\x04aA\xBDV[\x84\x01` \x01Q\x90V[_\x82\x82\x81Q\x81\x10a3\xCDWa3\xCDa<|V[\x01` \x01Q`\xF8\x1C`\xFF\x03a3\xE4WP`\x08a\x068V[\x82\x82\x81Q\x81\x10a3\xF6Wa3\xF6a<|V[\x01` \x01Q`\xF8\x1C`\xFE\x03a4\rWP`\x04a\x068V[\x82\x82\x81Q\x81\x10a4\x1FWa4\x1Fa<|V[\x01` \x01Q`\xF8\x1C`\xFD\x03a46WP`\x02a\x068V[P_\x92\x91PPV[_\x82_R\x81` R` _`@_`\x02Z\xFAP` _` _`\x02Z\xFAPP_Q\x92\x91PPV[`@Q\x80`\xA0\x01`@R\x80_\x81R` \x01a4\x8C`@Q\x80` \x01`@R\x80``\x81RP\x90V[\x81R_` \x82\x01\x81\x90R`@\x82\x01\x81\x90R``\x90\x91\x01R\x90V[P\x80Ta4\xB2\x90a<1V[_\x82U\x80`\x1F\x10a4\xC1WPPV[`\x1F\x01` \x90\x04\x90_R` _ \x90\x81\x01\x90a4\xDD\x91\x90a57V[PV[`@Q\x80a\x01\0\x01`@R\x80_\x81R` \x01a5\x08`@Q\x80` \x01`@R\x80``\x81RP\x90V[\x81R_` \x82\x01\x81\x90R`@\x82\x01\x81\x90R``\x82\x01\x81\x90R`\x80\x82\x01\x81\x90R`\xA0\x82\x01\x81\x90R`\xC0\x90\x91\x01R\x90V[[\x80\x82\x11\x15a5KW_\x81U`\x01\x01a58V[P\x90V[__`@\x83\x85\x03\x12\x15a5`W__\xFD[PP\x805\x92` \x90\x91\x015\x91PV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[_\x81Q` \x84Ra\x1DZ` \x85\x01\x82a5oV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15a5\xE1W\x81Q\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01a5\xC3V[P\x93\x94\x93PPPPV[_`@\x82\x01`@\x83R\x80\x85Q\x80\x83R``\x85\x01\x91P``\x81`\x05\x1B\x86\x01\x01\x92P` \x87\x01_[\x82\x81\x10\x15a6\xADW\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA0\x87\x86\x03\x01\x84R\x81Q\x80Q\x86R` \x81\x01Q`\xA0` \x88\x01Ra6_`\xA0\x88\x01\x82a5\x9DV[\x90P`\x01`\x01`\xA0\x1B\x03`@\x83\x01Q\x16`@\x88\x01R``\x82\x01Q``\x88\x01R`\x01`\x01`\xA0\x1B\x03`\x80\x83\x01Q\x16`\x80\x88\x01R\x80\x96PPP` \x82\x01\x91P` \x84\x01\x93P`\x01\x81\x01\x90Pa6\x11V[PPPP\x82\x81\x03` \x84\x01Ra)J\x81\x85a5\xB1V[_` \x82\x84\x03\x12\x15a6\xD3W__\xFD[P\x91\x90PV[___``\x84\x86\x03\x12\x15a6\xEBW__\xFD[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a7\x08W__\xFD[a7\x14\x86\x82\x87\x01a6\xC3V[\x93\x96\x93\x95PPPP`@\x91\x90\x91\x015\x90V[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a7<W__\xFD[\x91\x90PV[____`\x80\x85\x87\x03\x12\x15a7TW__\xFD[\x845\x93P` \x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a7qW__\xFD[a7}\x87\x82\x88\x01a6\xC3V[\x93PPa7\x8C`@\x86\x01a7&V[\x93\x96\x92\x95P\x92\x93``\x015\x92PPV[_` \x82\x84\x03\x12\x15a7\xACW__\xFD[P5\x91\x90PV[\x85\x81R`\xA0` \x82\x01R_a7\xCB`\xA0\x83\x01\x87a5\x9DV[\x90P`\x01`\x01`\xA0\x1B\x03\x85\x16`@\x83\x01R\x83``\x83\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x80\x83\x01R\x96\x95PPPPPPV[\x88\x81Ra\x01\0` \x82\x01R_a8\x15a\x01\0\x83\x01\x8Aa5\x9DV[`@\x83\x01\x98\x90\x98RP`\x01`\x01`\xA0\x1B\x03\x95\x86\x16``\x82\x01R`\x80\x81\x01\x94\x90\x94R\x91\x84\x16`\xA0\x84\x01R\x90\x92\x16`\xC0\x82\x01R`\xE0\x01R\x92\x91PPV[_` \x82\x84\x03\x12\x15a8`W__\xFD[a\n\xD0\x82a7&V[___``\x84\x86\x03\x12\x15a8{W__\xFD[\x835\x92Pa8\x8B` \x85\x01a7&V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`@\x80\x82R\x83Q\x90\x82\x01\x81\x90R_\x90` \x85\x01\x90``\x84\x01\x90\x83[\x81\x81\x10\x15a9\rW\x83Q\x80Q\x84R`\x01`\x01`\xA0\x1B\x03` \x82\x01Q\x16` \x85\x01R`@\x81\x01Q`@\x85\x01R`\x01`\x01`\xA0\x1B\x03``\x82\x01Q\x16``\x85\x01RP`\x80\x83\x01\x92P` \x84\x01\x93P`\x01\x81\x01\x90Pa8\xB7V[PP\x83\x81\x03` \x85\x01Ra9!\x81\x86a5\xB1V[\x96\x95PPPPPPV[`@\x80\x82R\x83Q\x90\x82\x01\x81\x90R_\x90` \x85\x01\x90``\x84\x01\x90\x83[\x81\x81\x10\x15a9\rW\x83Q\x80Q\x84R` \x81\x01Q` \x85\x01R`\x01`\x01`\xA0\x1B\x03`@\x82\x01Q\x16`@\x85\x01R``\x81\x01Q``\x85\x01R`\x01`\x01`\xA0\x1B\x03`\x80\x82\x01Q\x16`\x80\x85\x01R`\x01`\x01`\xA0\x1B\x03`\xA0\x82\x01Q\x16`\xA0\x85\x01R`\xC0\x81\x01Q`\xC0\x85\x01RP`\xE0\x83\x01\x92P` \x84\x01\x93P`\x01\x81\x01\x90Pa9FV[_`@\x82\x01`@\x83R\x80\x85Q\x80\x83R``\x85\x01\x91P``\x81`\x05\x1B\x86\x01\x01\x92P` \x87\x01_[\x82\x81\x10\x15a6\xADW\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA0\x87\x86\x03\x01\x84R\x81Q\x80Q\x86R` \x81\x01Qa\x01\0` \x88\x01Ra:9a\x01\0\x88\x01\x82a5\x9DV[\x90P`@\x82\x01Q`@\x88\x01R`\x01`\x01`\xA0\x1B\x03``\x83\x01Q\x16``\x88\x01R`\x80\x82\x01Q`\x80\x88\x01R`\x01`\x01`\xA0\x1B\x03`\xA0\x83\x01Q\x16`\xA0\x88\x01R`\xC0\x82\x01Qa:\x8F`\xC0\x89\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`\xE0\x91\x82\x01Q\x96\x90\x91\x01\x95\x90\x95R` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a9\xE9V[___``\x84\x86\x03\x12\x15a:\xC3W__\xFD[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a:\xE0W__\xFD[\x84\x01`\x80\x81\x87\x03\x12\x15a:\xF1W__\xFD[\x91P`@\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a;\x0CW__\xFD[\x84\x01`\xA0\x81\x87\x03\x12\x15a;\x1DW__\xFD[\x80\x91PP\x92P\x92P\x92V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x068Wa\x068a;(V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_\x82a;\xA7Wa;\xA7a;lV[P\x04\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x01`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x068Wa\x068a;(V[__\x19\x82\x03a;\xFDWa;\xFDa;(V[P`\x01\x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[`\x01\x81\x81\x1C\x90\x82\x16\x80a<EW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a6\xD3W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a<\xCCWa<\xCCa<\x04V[`@R\x90V[_\x82`\x1F\x83\x01\x12a<\xE1W__\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a<\xFBWa<\xFBa<\x04V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a=*Wa=*a<\x04V[`@R\x81\x81R\x83\x82\x01` \x01\x85\x10\x15a=AW__\xFD[\x81` \x85\x01` \x83\x017_\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[_` \x826\x03\x12\x15a=mW__\xFD[`@Q` \x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a=\x90Wa=\x90a<\x04V[`@R\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a=\xA9W__\xFD[a=\xB56\x82\x86\x01a<\xD2V[\x82RP\x92\x91PPV[`\x1F\x82\x11\x15a\x1D0W\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a=\xE3WP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x1ErW_\x81U`\x01\x01a=\xEFV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a>\x1CWa>\x1Ca<\x04V[a>0\x81a>*\x84Ta<1V[\x84a=\xBEV[` `\x1F\x82\x11`\x01\x81\x14a>bW_\x83\x15a>KWP\x84\x82\x01Q[_\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua\x1ErV[_\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a>\x91W\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a>qV[P\x84\x82\x10\x15a>\xAEW\x86\x84\x01Q_\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[\x81\x83R\x81\x81` \x85\x017P_` \x82\x84\x01\x01R_` `\x1F\x19`\x1F\x84\x01\x16\x84\x01\x01\x90P\x92\x91PPV[_\x815\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x836\x03\x01\x81\x12a?\x18W__\xFD[\x82\x01` \x81\x01\x905g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a?4W__\xFD[\x806\x03\x82\x13\x15a?BW__\xFD[` \x85Ra)J` \x86\x01\x82\x84a>\xBDV[`\x80\x81R_a?f`\x80\x83\x01\x87a>\xE6V[` \x83\x01\x95\x90\x95RP`@\x81\x01\x92\x90\x92R`\x01`\x01`\xA0\x1B\x03\x16``\x90\x91\x01R\x91\x90PV[\x84\x81R`\x80` \x82\x01R_a?\xA3`\x80\x83\x01\x86a>\xE6V[`\x01`\x01`\xA0\x1B\x03\x94\x90\x94\x16`@\x83\x01RP``\x01R\x92\x91PPV[__\x835\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x846\x03\x01\x81\x12a?\xF2W__\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a@\x0CW__\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a3\x0BW__\xFD[` \x81R_a\x1DZ` \x83\x01\x84\x86a>\xBDV[\x805\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x81\x14a7<W__\xFD[_`\x80\x826\x03\x12\x15a@rW__\xFD[`@Q`\x80\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a@\x95Wa@\x95a<\x04V[`@Ra@\xA1\x83a@3V[\x81R` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a@\xBCW__\xFD[a@\xC86\x82\x86\x01a<\xD2V[` \x83\x01RP`@\x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a@\xE7W__\xFD[a@\xF36\x82\x86\x01a<\xD2V[`@\x83\x01RPaA\x05``\x84\x01a@3V[``\x82\x01R\x92\x91PPV[_`\xA0\x826\x03\x12\x15aA W__\xFD[aA(a<\xA9V[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aA>W__\xFD[aAJ6\x82\x86\x01a<\xD2V[\x82RP` \x83\x81\x015\x90\x82\x01R`@\x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aApW__\xFD[aA|6\x82\x86\x01a<\xD2V[`@\x83\x01RP``\x83\x81\x015\x90\x82\x01R`\x80\x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aA\xA5W__\xFD[aA\xB16\x82\x86\x01a<\xD2V[`\x80\x83\x01RP\x92\x91PPV[\x80\x82\x01\x80\x82\x11\x15a\x068Wa\x068a;(V[\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83`\xF8\x1B\x16\x81R__\x83TaB\x05\x81a<1V[`\x01\x82\x16\x80\x15aB\x1CW`\x01\x81\x14aBUWaB\x8BV[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x83\x16`\x01\x87\x01R`\x01\x82\x15\x15\x83\x02\x87\x01\x01\x93PaB\x8BV[\x86_R` _ _[\x83\x81\x10\x15aB\x80W\x81T`\x01\x82\x8A\x01\x01R`\x01\x82\x01\x91P` \x81\x01\x90PaB^V[PP`\x01\x82\x87\x01\x01\x93P[P\x91\x96\x95PPPPPPV[_` \x82\x84\x03\x12\x15aB\xA7W__\xFD[\x81Q\x80\x15\x15\x81\x14a\n\xD0W__\xFD[_\x81Q\x80` \x84\x01\x85^_\x93\x01\x92\x83RP\x90\x91\x90PV[\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85\x16\x81R_aC\taC\x03`\x04\x84\x01\x87aB\xB6V[\x85aB\xB6V[\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x93\x90\x93\x16\x83RPP`\x04\x01\x93\x92PPPV[_a\n\xD0\x82\x84aB\xB6V[_` \x82\x84\x03\x12\x15aCWW__\xFD[PQ\x91\x90PV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x068Wa\x068a;(V[_\x82aC\x8CWaC\x8Ca;lV[P\x06\x90V[`\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x068Wa\x068a;(V[`\x01\x81[`\x01\x84\x11\x15aC\xE5W\x80\x85\x04\x81\x11\x15aC\xC9WaC\xC9a;(V[`\x01\x84\x16\x15aC\xD7W\x90\x81\x02\x90[`\x01\x93\x90\x93\x1C\x92\x80\x02aC\xAEV[\x93P\x93\x91PPV[_\x82aC\xFBWP`\x01a\x068V[\x81aD\x07WP_a\x068V[\x81`\x01\x81\x14aD\x1DW`\x02\x81\x14aD'WaDCV[`\x01\x91PPa\x068V[`\xFF\x84\x11\x15aD8WaD8a;(V[PP`\x01\x82\x1Ba\x068V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15aDfWP\x81\x81\na\x068V[aDr_\x19\x84\x84aC\xAAV[\x80_\x19\x04\x82\x11\x15aD\x85WaD\x85a;(V[\x02\x93\x92PPPV[_a\n\xD0\x83\x83aC\xEDV[`\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x068Wa\x068a;(V[` \x81R_a\n\xD0` \x83\x01\x84a5oV\xFE\xA2dipfsX\"\x12 A\xDF\x18\xE5x\xA0A\xC2\xE5-\x8F\x05\xEBc;\xA9l\0\x11;`\x87?x\xCB\xE1J\xFF\x1C\xB1\x7F\x07dsolcC\0\x08\x1C\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b506004361061016e575f3560e01c80636a8cde3a116100d2578063c56a452611610088578063df69b14f11610063578063df69b14f146103b2578063ecca2c36146103c5578063fd3fc24514610432575f5ffd5b8063c56a45261461037c578063ce1b815f1461038f578063d1920ff0146103a9575f5ffd5b8063a383013b116100b8578063a383013b146102ba578063b223d976146102cd578063bd2a7e3e146102e0575f5ffd5b80636a8cde3a1461028e5780639cc6722e146102a4575f5ffd5b80634145640a11610127578063572b6c051161010d578063572b6c05146102345780635b8fe042146102655780636811a31114610278575f5ffd5b80634145640a146101fa578063506a109d14610221575f5ffd5b8063210ec18111610157578063210ec181146101ae578063364f1ec0146101c15780633af3fc7e146101d6575f5ffd5b806311c137aa146101725780631dfe759514610198575b5f5ffd5b61018561018036600461354f565b610445565b6040519081526020015b60405180910390f35b6101a061063e565b60405161018f9291906135eb565b6101856101bc3660046136d9565b6108a0565b6101d46101cf366004613741565b610ad7565b005b6101e96101e436600461379c565b610c2c565b60405161018f9594939291906137b3565b61020d61020836600461379c565b610cfe565b60405161018f9897969594939291906137fb565b6101d461022f36600461379c565b610def565b610255610242366004613850565b5f546001600160a01b0391821691161490565b604051901515815260200161018f565b6101d4610273366004613869565b610ed4565b610280610ff0565b60405161018f92919061389c565b6102966111b4565b60405161018f92919061392b565b6102ac6113ba565b60405161018f9291906139c3565b6101d46102c836600461379c565b61161d565b6101d46102db366004613ab1565b6116c1565b6103396102ee36600461379c565b600260208190525f918252604090912080546001820154928201546003830154600484015460058501546006909501549395946001600160a01b039384169492939182169291169087565b6040805197885260208801969096526001600160a01b03948516958701959095526060860192909252821660808501521660a083015260c082015260e00161018f565b6101d461038a36600461379c565b61185d565b5f546040516001600160a01b03909116815260200161018f565b61018561546081565b6101d46103c036600461379c565b611940565b6104076103d336600461379c565b600360208190525f9182526040909120805460018201546002830154929093015490926001600160a01b0390811692911684565b604080519485526001600160a01b03938416602086015284019190915216606082015260800161018f565b6101d4610440366004613ab1565b611a53565b5f828152600160205260408120805483111561045f575f5ffd5b5f831161046a575f5ffd5b805460038201545f919061047e9086613b55565b6104889190613b99565b90505f811161049957610499613bac565b80826003015410156104ad576104ad613bac565b80826003015f8282546104c09190613bd9565b90915550508154849083905f906104d8908490613bd9565b90915550506040805160e0810182528681526020810186905260028401546001600160a01b039081169282019290925260608101839052600484015490911660808201525f9060a0810161052a611be0565b6001600160a01b0316815242602090910152600580549192505f91908261055083613bec565b909155505f818152600260208181526040928390208651815586820151600182015586840151818401805473ffffffffffffffffffffffffffffffffffffffff199081166001600160a01b03938416179091556060808a0151600385015560808a0151600485018054841691851691909117905560a08a01516005850180549093169084161790915560c08901516006909301929092559289015484518c815292830189905290921692810192909252919250829189917fc39a1a5ddc0e85c955fe2e1abeb43c94ce18322e75bb3d44e80f759ff9d034b9910160405180910390a393505050505b92915050565b6060805f805b600554811015610683575f818152600160205260409020600401546001600160a01b03161561067b578161067781613bec565b9250505b600101610644565b505f8167ffffffffffffffff81111561069e5761069e613c04565b6040519080825280602002602001820160405280156106d757816020015b6106c4613465565b8152602001906001900390816106bc5790505b5090505f8267ffffffffffffffff8111156106f4576106f4613c04565b60405190808252806020026020018201604052801561071d578160200160208202803683370190505b5090505f805b600554811015610894575f818152600160205260409020600401546001600160a01b03161561088c5760015f8281526020019081526020015f206040518060a00160405290815f8201548152602001600182016040518060200160405290815f8201805461079090613c31565b80601f01602080910402602001604051908101604052809291908181526020018280546107bc90613c31565b80156108075780601f106107de57610100808354040283529160200191610807565b820191905f5260205f20905b8154815290600101906020018083116107ea57829003601f168201915b50505091909252505050815260028201546001600160a01b03908116602083015260038301546040830152600490920154909116606090910152845185908490811061085557610855613c7c565b60200260200101819052508083838151811061087357610873613c7c565b60209081029190910101528161088881613bec565b9250505b600101610723565b50919590945092505050565b5f838152600360205260408120826108b6575f5ffd5b80548311156108c3575f5ffd5b805460028201545f91906108d79086613b55565b6108e19190613b99565b90505f81116108f2576108f2613bac565b808260020154101561090657610906613bac565b80826002015f8282546109199190613bd9565b90915550508154849083905f90610931908490613bd9565b909155506109589050610942611be0565b60018401546001600160a01b0316903084611c30565b600580545f918261096883613bec565b9190505590506040518061010001604052808881526020018761098a90613d5d565b81526020810187905260018501546001600160a01b03908116604083015260608201859052600386015416608082015260a0016109c5611be0565b6001600160a01b03168152426020918201525f838152600482526040902082518155908201518051600183019081906109fe9082613e02565b5050506040828101516002830155606083015160038301805473ffffffffffffffffffffffffffffffffffffffff199081166001600160a01b03938416179091556080850151600485015560a0850151600585018054831691841691909117905560c085015160068501805490921690831617905560e0909301516007909201919091556001850154905183928a927f653e0d81f2c99beba359dfb17b499a5cff2be9d950514852224df8c097c2192192610ac3928c928c928a929190911690613f54565b60405180910390a3925050505b9392505050565b6001600160a01b038216610ae9575f5ffd5b610b06610af4611be0565b6001600160a01b038416903084611c30565b600580545f9182610b1683613bec565b9190505590506040518060a0016040528086815260200185610b3790613d5d565b8152602001846001600160a01b03168152602001838152602001610b59611be0565b6001600160a01b031690525f8281526001602081815260409092208351815591830151805190918301908190610b8f9082613e02565b50505060408281015160028301805473ffffffffffffffffffffffffffffffffffffffff199081166001600160a01b03938416179091556060850151600385015560809094015160049093018054909416921691909117909155517f98c7c680403d47403dea4a570d0e6c5716538c49420ef471cec428f5a5852c0690610c1d908790879087908790613f8b565b60405180910390a15050505050565b600160208181525f92835260409283902080548451928301909452918201805482908290610c5990613c31565b80601f0160208091040260200160405190810160405280929190818152602001828054610c8590613c31565b8015610cd05780601f10610ca757610100808354040283529160200191610cd0565b820191905f5260205f20905b815481529060010190602001808311610cb357829003601f168201915b505050919092525050506002820154600383015460049093015491926001600160a01b039182169290911685565b6004602052805f5260405f205f91509050805f015490806001016040518060200160405290815f82018054610d3290613c31565b80601f0160208091040260200160405190810160405280929190818152602001828054610d5e90613c31565b8015610da95780601f10610d8057610100808354040283529160200191610da9565b820191905f5260205f20905b815481529060010190602001808311610d8c57829003601f168201915b5050509190925250505060028201546003830154600484015460058501546006860154600790960154949593946001600160a01b03938416949293918216929091169088565b5f818152600160205260409020610e04611be0565b60048201546001600160a01b03908116911614610e1f575f5ffd5b610e44610e2a611be0565b600383015460028401546001600160a01b03169190611ce7565b5f82815260016020819052604082208281559190820181610e6582826134a6565b50505060028101805473ffffffffffffffffffffffffffffffffffffffff199081169091555f60038301556004909101805490911690556040518281527fc340e7ac48dc80ee793fc6266960bd5f1bd21be91c8a95e218178113f79e17b4906020015b60405180910390a15050565b6001600160a01b038216610ee6575f5ffd5b5f8311610ef1575f5ffd5b5f8111610efc575f5ffd5b600580545f9182610f0c83613bec565b9190505590506040518060800160405280858152602001846001600160a01b03168152602001838152602001610f40611be0565b6001600160a01b039081169091525f83815260036020818152604092839020855181558582015160018201805491871673ffffffffffffffffffffffffffffffffffffffff199283161790558685015160028301556060968701519190930180549186169190931617909155815188815292871690830152810184905282917fff1ce210defcd3ba1adf76c9419a0758fa60fd3eb38c7bd9418f60b575b76e24910160405180910390a250505050565b6060805f805b600554811015611036575f81815260036020819052604090912001546001600160a01b03161561102e578161102a81613bec565b9250505b600101610ff6565b505f8167ffffffffffffffff81111561105157611051613c04565b6040519080825280602002602001820160405280156110a157816020015b604080516080810182525f8082526020808301829052928201819052606082015282525f1990920191018161106f5790505b5090505f8267ffffffffffffffff8111156110be576110be613c04565b6040519080825280602002602001820160405280156110e7578160200160208202803683370190505b5090505f805b600554811015610894575f81815260036020819052604090912001546001600160a01b0316156111ac575f8181526003602081815260409283902083516080810185528154815260018201546001600160a01b039081169382019390935260028201549481019490945290910154166060820152845185908490811061117557611175613c7c565b60200260200101819052508083838151811061119357611193613c7c565b6020908102919091010152816111a881613bec565b9250505b6001016110ed565b6060805f805b6005548110156111f0575f81815260026020526040902060010154156111e857816111e481613bec565b9250505b6001016111ba565b505f8167ffffffffffffffff81111561120b5761120b613c04565b60405190808252806020026020018201604052801561129057816020015b61127d6040518060e001604052805f81526020015f81526020015f6001600160a01b031681526020015f81526020015f6001600160a01b031681526020015f6001600160a01b031681526020015f81525090565b8152602001906001900390816112295790505b5090505f8267ffffffffffffffff8111156112ad576112ad613c04565b6040519080825280602002602001820160405280156112d6578160200160208202803683370190505b5090505f805b600554811015610894575f81815260026020526040902060010154156113b2575f81815260026020818152604092839020835160e08101855281548152600182015492810192909252918201546001600160a01b039081169382019390935260038201546060820152600482015483166080820152600582015490921660a08301526006015460c0820152845185908490811061137b5761137b613c7c565b60200260200101819052508083838151811061139957611399613c7c565b6020908102919091010152816113ae81613bec565b9250505b6001016112dc565b6060805f805b6005548110156113f6575f81815260046020526040902060020154156113ee57816113ea81613bec565b9250505b6001016113c0565b505f8167ffffffffffffffff81111561141157611411613c04565b60405190808252806020026020018201604052801561144a57816020015b6114376134e0565b81526020019060019003908161142f5790505b5090505f8267ffffffffffffffff81111561146757611467613c04565b604051908082528060200260200182016040528015611490578160200160208202803683370190505b5090505f805b600554811015610894575f81815260046020526040902060020154156116155760045f8281526020019081526020015f20604051806101000160405290815f8201548152602001600182016040518060200160405290815f820180546114fb90613c31565b80601f016020809104026020016040519081016040528092919081815260200182805461152790613c31565b80156115725780601f1061154957610100808354040283529160200191611572565b820191905f5260205f20905b81548152906001019060200180831161155557829003601f168201915b5050509190925250505081526002820154602082015260038201546001600160a01b0390811660408301526004830154606083015260058301548116608083015260068301541660a082015260079091015460c09091015284518590849081106115de576115de613c7c565b6020026020010181905250808383815181106115fc576115fc613c7c565b60209081029190910101528161161181613bec565b9250505b600101611496565b5f818152600360205260409020611632611be0565b60038201546001600160a01b0390811691161461164d575f5ffd5b5f82815260036020818152604080842084815560018101805473ffffffffffffffffffffffffffffffffffffffff1990811690915560028201959095559092018054909316909255518381527f3cd475b092e8b379f6ba0d9e0e0c8f30705e73321dc5c9f80ce4ad38db7be1aa9101610ec8565b5f8381526002602052604090206116d6611be0565b60058201546001600160a01b039081169116146116f1575f5ffd5b6006546001600160a01b031663d38c29a161170f6040850185613fbf565b6040518363ffffffff1660e01b815260040161172c929190614020565b5f604051808303815f87803b158015611743575f5ffd5b505af1158015611755573d5f5f3e3d5ffd5b505050506117866007548461176990614062565b61177285614110565b6006546001600160a01b0316929190611d35565b5080545f908152600160208190526040909120805490916117aa9190830186611d62565b6005820154600383015460028401546117d1926001600160a01b0391821692911690611ce7565b5f85815260026020818152604080842084815560018101859055928301805473ffffffffffffffffffffffffffffffffffffffff1990811690915560038401859055600484018054821690556005840180549091169055600690920192909255518681527fb4c98de210696b3cf21e99335c1ee3a0ae34a26713412a4adde8af596176f37e9101610c1d565b5f818152600260205260409020611872611be0565b60048201546001600160a01b0390811691161461188d575f5ffd5b615460816006015461189f91906141bd565b42116118a9575f5ffd5b6118b4610e2a611be0565b5f82815260026020818152604080842084815560018101859055928301805473ffffffffffffffffffffffffffffffffffffffff1990811690915560038401859055600484018054821690556005840180549091169055600690920192909255518381527f3e5ea358e9eb4cdf44cdc77938ade8074b1240a6d8c0fd13728671b82e800ad69101610ec8565b5f818152600460205260409020600781015461195f90615460906141bd565b4211611969575f5ffd5b611971611be0565b60068201546001600160a01b0390811691161461198c575f5ffd5b6119b1611997611be0565b600483015460038401546001600160a01b03169190611ce7565b5f8281526004602052604081208181559060018201816119d182826134a6565b50505f6002830181905560038301805473ffffffffffffffffffffffffffffffffffffffff1990811690915560048401829055600584018054821690556006840180549091169055600790920191909155506040518281527f78f51f62f7cf1381c673c27eae187dd6c588dc6624ce59697dbb3e1d7c1bbcdf90602001610ec8565b5f838152600460205260409020611a68611be0565b60058201546001600160a01b03908116911614611a83575f5ffd5b6006546001600160a01b031663d38c29a1611aa16040850185613fbf565b6040518363ffffffff1660e01b8152600401611abe929190614020565b5f604051808303815f87803b158015611ad5575f5ffd5b505af1158015611ae7573d5f5f3e3d5ffd5b50505050611afb6007548461176990614062565b50611b0e81600201548260010185611d62565b600581015460048201546003830154611b35926001600160a01b0391821692911690611ce7565b5f848152600460205260408120818155906001820181611b5582826134a6565b50505f6002830181905560038301805473ffffffffffffffffffffffffffffffffffffffff1990811690915560048401829055600584018054821690556006840180549091169055600790920191909155506040518481527fcf561061db78f7bc518d37fe86718514c640ccc5c3f1293828b955e68f19f5fb9060200160405180910390a150505050565b5f60143610801590611bfb57505f546001600160a01b031633145b15611c2b57507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffec36013560601c90565b503390565b6040516001600160a01b0380851660248301528316604482015260648101829052611ce19085907f23b872dd00000000000000000000000000000000000000000000000000000000906084015b60408051601f198184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167fffffffff0000000000000000000000000000000000000000000000000000000090931692909217909152611e79565b50505050565b6040516001600160a01b038316602482015260448101829052611d309084907fa9059cbb0000000000000000000000000000000000000000000000000000000090606401611c7d565b505050565b5f611d3f83611f5d565b9050611d4b818361204d565b611d5a858584604001516122b1565b949350505050565b5f825f018054611d7190613c31565b604051611d83925085906020016141d0565b6040516020818303038152906040528051906020012090505f611dea838060400190611daf9190613fbf565b8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f92019190915250869250612601915050565b5167ffffffffffffffff16905084811015611e725760405162461bcd60e51b815260206004820152603b60248201527f426974636f696e207472616e73616374696f6e20616d6f756e74206973206c6f60448201527f776572207468616e20696e206163636570746564206f726465722e000000000060648201526084015b60405180910390fd5b5050505050565b5f611ecd826040518060400160405280602081526020017f5361666545524332303a206c6f772d6c6576656c2063616c6c206661696c6564815250856001600160a01b031661279f9092919063ffffffff16565b805190915015611d305780806020019051810190611eeb9190614297565b611d305760405162461bcd60e51b815260206004820152602a60248201527f5361666545524332303a204552433230206f7065726174696f6e20646964206e60448201527f6f742073756363656564000000000000000000000000000000000000000000006064820152608401611e69565b5f611f6b82602001516127ad565b611fb75760405162461bcd60e51b815260206004820152601d60248201527f496e76616c696420696e70757420766563746f722070726f76696465640000006044820152606401611e69565b611fc48260400151612847565b6120105760405162461bcd60e51b815260206004820152601e60248201527f496e76616c6964206f757470757420766563746f722070726f766964656400006044820152606401611e69565b610638825f015183602001518460400151856060015160405160200161203994939291906142cd565b6040516020818303038152906040526128d4565b8051612058906128f6565b6120a45760405162461bcd60e51b815260206004820152601660248201527f426164206d65726b6c652061727261792070726f6f66000000000000000000006044820152606401611e69565b608081015151815151146121205760405162461bcd60e51b815260206004820152602f60248201527f5478206e6f74206f6e2073616d65206c6576656c206f66206d65726b6c65207460448201527f72656520617320636f696e6261736500000000000000000000000000000000006064820152608401611e69565b5f61212e826040015161290c565b825160208401519192506121459185918491612918565b6121b75760405162461bcd60e51b815260206004820152603c60248201527f5478206d65726b6c652070726f6f66206973206e6f742076616c696420666f7260448201527f2070726f76696465642068656164657220616e642074782068617368000000006064820152608401611e69565b5f600283606001516040516020016121d191815260200190565b60408051601f19818403018152908290526121eb9161433c565b602060405180830381855afa158015612206573d5f5f3e3d5ffd5b5050506040513d601f19601f820116820180604052508101906122299190614347565b608084015190915061223f90829084905f612918565b611ce15760405162461bcd60e51b815260206004820152603f60248201527f436f696e62617365206d65726b6c652070726f6f66206973206e6f742076616c60448201527f696420666f722070726f76696465642068656164657220616e642068617368006064820152608401611e69565b5f836001600160a01b031663113764be6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156122ee573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906123129190614347565b90505f846001600160a01b0316632b97be246040518163ffffffff1660e01b8152600401602060405180830381865afa158015612351573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906123759190614347565b90505f8061238a61238586612953565b61295e565b905083810361239b57839150612418565b8281036123aa57829150612418565b60405162461bcd60e51b815260206004820152602560248201527f4e6f742061742063757272656e74206f722070726576696f757320646966666960448201527f63756c74790000000000000000000000000000000000000000000000000000006064820152608401611e69565b5f61242286612985565b90505f19810361249a5760405162461bcd60e51b815260206004820152602360248201527f496e76616c6964206c656e677468206f6620746865206865616465727320636860448201527f61696e00000000000000000000000000000000000000000000000000000000006064820152608401611e69565b7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81036125095760405162461bcd60e51b815260206004820152601560248201527f496e76616c6964206865616465727320636861696e00000000000000000000006044820152606401611e69565b7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffd81036125785760405162461bcd60e51b815260206004820152601d60248201527f496e73756666696369656e7420776f726b20696e2061206865616465720000006044820152606401611e69565b6125828784613b55565b8110156125f75760405162461bcd60e51b815260206004820152603360248201527f496e73756666696369656e7420616363756d756c61746564206469666669637560448201527f6c747920696e2068656164657220636861696e000000000000000000000000006064820152608401611e69565b5050505050505050565b604080516060810182525f808252602080830182905282840182905283518085019094528184528301529061263584612ba9565b60208301528082528161264782613bec565b9052505f805b82602001518110156127495782515f90612668908890612bbe565b84519091505f9061267a908990612c1e565b90505f612688600884613bd9565b86519091505f9061269a9060086141bd565b8a8101602001839020909150808a036126d4576001965083895f018181516126c2919061435e565b67ffffffffffffffff16905250612724565b5f6126e28c8a5f0151612c94565b90506001600160a01b03811615612703576001600160a01b03811660208b01525b5f6127118d8b5f0151612d74565b905080156127215760408b018190525b50505b84885f0181815161273591906141bd565b905250506001909401935061264d92505050565b50806127975760405162461bcd60e51b815260206004820181905260248201527f4e6f206f757470757420666f756e6420666f72207363726970745075624b65796044820152606401611e69565b505092915050565b6060611d5a84845f85612e54565b5f5f5f6127b984612ba9565b90925090508015806127cb57505f1982145b156127d957505f9392505050565b5f6127e58360016141bd565b90505f5b8281101561283a578551821061280457505f95945050505050565b5f61280f8784612f98565b90505f19810361282557505f9695505050505050565b61282f81846141bd565b9250506001016127e9565b5093519093149392505050565b5f5f5f61285384612ba9565b909250905080158061286557505f1982145b1561287357505f9392505050565b5f61287f8360016141bd565b90505f5b8281101561283a578551821061289e57505f95945050505050565b5f6128a98784612bbe565b90505f1981036128bf57505f9695505050505050565b6128c981846141bd565b925050600101612883565b5f60205f83516020850160025afa5060205f60205f60025afa50505f51919050565b5f60208251612905919061437e565b1592915050565b60448101515f90610638565b5f8385148015612926575081155b801561293157508251155b1561293e57506001611d5a565b61294a85848685612fde565b95945050505050565b5f610638825f613083565b5f6106387bffff00000000000000000000000000000000000000000000000000008361311c565b5f60508251612994919061437e565b156129a157505f19919050565b505f80805b8351811015612ba25780156129ed576129c0848284613127565b6129ed57507ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe9392505050565b5f6129f88583613083565b9050612a0685836050613150565b925080612b49845f8190506008817eff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff16901b600882901c7eff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff161790506010817dffff0000ffff0000ffff0000ffff0000ffff0000ffff0000ffff0000ffff16901b601082901c7dffff0000ffff0000ffff0000ffff0000ffff0000ffff0000ffff0000ffff161790506020817bffffffff00000000ffffffff00000000ffffffff00000000ffffffff16901b602082901c7bffffffff00000000ffffffff00000000ffffffff00000000ffffffff1617905060408177ffffffffffffffff0000000000000000ffffffffffffffff16901b604082901c77ffffffffffffffff0000000000000000ffffffffffffffff16179050608081901b608082901c179050919050565b1115612b7957507ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffd949350505050565b612b828161295e565b612b8c90856141bd565b9350612b9b90506050826141bd565b90506129a6565b5050919050565b5f5f612bb5835f613175565b91509150915091565b5f612bca8260096141bd565b83511015612bda57505f19610638565b5f80612bf085612beb8660086141bd565b613175565b909250905060018201612c08575f1992505050610638565b80612c148360096141bd565b61294a91906141bd565b5f80612c2a8484613312565b60c01c90505f61294a8264ff000000ff600882811c91821665ff000000ff009390911b92831617601090811b67ffffffffffffffff1666ff00ff00ff00ff9290921667ff00ff00ff00ff009093169290921790911c65ffff0000ffff1617602081811c91901b1790565b5f82612ca18360096141bd565b81518110612cb157612cb1613c7c565b6020910101517fff00000000000000000000000000000000000000000000000000000000000000167f6a0000000000000000000000000000000000000000000000000000000000000014612d0657505f610638565b5f83612d1384600a6141bd565b81518110612d2357612d23613c7c565b01602001517fff000000000000000000000000000000000000000000000000000000000000008116915060f81c601403612d6d575f612d6384600b6141bd565b8501601401519250505b5092915050565b5f82612d818360096141bd565b81518110612d9157612d91613c7c565b6020910101517fff00000000000000000000000000000000000000000000000000000000000000167f6a0000000000000000000000000000000000000000000000000000000000000014612de657505f610638565b5f83612df384600a6141bd565b81518110612e0357612e03613c7c565b016020908101517fff000000000000000000000000000000000000000000000000000000000000008116925060f81c9003612d6d575f612e4484600b6141bd565b8501602001519250505092915050565b606082471015612ecc5760405162461bcd60e51b815260206004820152602660248201527f416464726573733a20696e73756666696369656e742062616c616e636520666f60448201527f722063616c6c00000000000000000000000000000000000000000000000000006064820152608401611e69565b6001600160a01b0385163b612f235760405162461bcd60e51b815260206004820152601d60248201527f416464726573733a2063616c6c20746f206e6f6e2d636f6e74726163740000006044820152606401611e69565b5f5f866001600160a01b03168587604051612f3e919061433c565b5f6040518083038185875af1925050503d805f8114612f78576040519150601f19603f3d011682016040523d82523d5f602084013e612f7d565b606091505b5091509150612f8d828286613320565b979650505050505050565b5f5f5f612fa58585613359565b909250905060018201612fbd575f1992505050610638565b80612fc98360256141bd565b612fd391906141bd565b61294a9060046141bd565b5f60208451612fed919061437e565b15612ff957505f611d5a565b83515f0361300857505f611d5a565b81855f5b86518110156130765761302060028461437e565b6001036130445761303d6130378883016020015190565b83613397565b915061305d565b61305a826130558984016020015190565b613397565b91505b60019290921c9161306f6020826141bd565b905061300c565b5090931495945050505050565b5f8061309a6130938460486141bd565b8590613312565b60e81c90505f846130ac85604b6141bd565b815181106130bc576130bc613c7c565b016020015160f81c90505f6130ee835f60108262ffffff16901c8261ff001660108462ffffff16901b17179050919050565b62ffffff1690505f613101600384614391565b60ff1690506131128161010061448d565b612f8d9083613b55565b5f610ad08284613b99565b5f8061313385856133a2565b9050828114613145575f915050610ad0565b506001949350505050565b5f60205f8385602001870160025afa5060205f60205f60025afa50505f519392505050565b5f5f5f61318285856133ba565b90508060ff165f036131b5575f8585815181106131a1576131a1613c7c565b016020015190935060f81c915061330b9050565b836131c1826001614498565b60ff166131ce91906141bd565b855110156131e3575f195f925092505061330b565b5f8160ff166002036132265761321b6132076132008760016141bd565b8890613312565b62ffff0060e882901c1660f89190911c1790565b61ffff169050613301565b8160ff16600403613275576132686132426132008760016141bd565b60d881901c63ff00ff001662ff00ff60e89290921c9190911617601081811b91901c1790565b63ffffffff169050613301565b8160ff16600803613301576132f46132916132008760016141bd565b60c01c64ff000000ff600882811c91821665ff000000ff009390911b92831617601090811b67ffffffffffffffff1666ff00ff00ff00ff9290921667ff00ff00ff00ff009093169290921790911c65ffff0000ffff1617602081811c91901b1790565b67ffffffffffffffff1690505b60ff909116925090505b9250929050565b5f610ad08383016020015190565b6060831561332f575081610ad0565b82511561333f5782518084602001fd5b8160405162461bcd60e51b8152600401611e6991906144b1565b5f806133668360256141bd565b8451101561337957505f1990505f61330b565b5f8061338a86612beb8760246141bd565b9097909650945050505050565b5f610ad0838361343e565b5f610ad06133b18360046141bd565b84016020015190565b5f8282815181106133cd576133cd613c7c565b016020015160f81c60ff036133e457506008610638565b8282815181106133f6576133f6613c7c565b016020015160f81c60fe0361340d57506004610638565b82828151811061341f5761341f613c7c565b016020015160f81c60fd0361343657506002610638565b505f92915050565b5f825f528160205260205f60405f60025afa5060205f60205f60025afa50505f5192915050565b6040518060a001604052805f815260200161348c6040518060200160405280606081525090565b81525f602082018190526040820181905260609091015290565b5080546134b290613c31565b5f825580601f106134c1575050565b601f0160209004905f5260205f20908101906134dd9190613537565b50565b6040518061010001604052805f81526020016135086040518060200160405280606081525090565b81525f6020820181905260408201819052606082018190526080820181905260a0820181905260c09091015290565b5b8082111561354b575f8155600101613538565b5090565b5f5f60408385031215613560575f5ffd5b50508035926020909101359150565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b5f815160208452611d5a602085018261356f565b5f8151808452602084019350602083015f5b828110156135e15781518652602095860195909101906001016135c3565b5093949350505050565b5f604082016040835280855180835260608501915060608160051b8601019250602087015f5b828110156136ad577fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa0878603018452815180518652602081015160a0602088015261365f60a088018261359d565b90506001600160a01b036040830151166040880152606082015160608801526001600160a01b0360808301511660808801528096505050602082019150602084019350600181019050613611565b50505050828103602084015261294a81856135b1565b5f602082840312156136d3575f5ffd5b50919050565b5f5f5f606084860312156136eb575f5ffd5b83359250602084013567ffffffffffffffff811115613708575f5ffd5b613714868287016136c3565b93969395505050506040919091013590565b80356001600160a01b038116811461373c575f5ffd5b919050565b5f5f5f5f60808587031215613754575f5ffd5b84359350602085013567ffffffffffffffff811115613771575f5ffd5b61377d878288016136c3565b93505061378c60408601613726565b9396929550929360600135925050565b5f602082840312156137ac575f5ffd5b5035919050565b85815260a060208201525f6137cb60a083018761359d565b90506001600160a01b03851660408301528360608301526001600160a01b03831660808301529695505050505050565b88815261010060208201525f61381561010083018a61359d565b6040830198909852506001600160a01b039586166060820152608081019490945291841660a084015290921660c082015260e0015292915050565b5f60208284031215613860575f5ffd5b610ad082613726565b5f5f5f6060848603121561387b575f5ffd5b8335925061388b60208501613726565b929592945050506040919091013590565b604080825283519082018190525f9060208501906060840190835b8181101561390d578351805184526001600160a01b036020820151166020850152604081015160408501526001600160a01b036060820151166060850152506080830192506020840193506001810190506138b7565b5050838103602085015261392181866135b1565b9695505050505050565b604080825283519082018190525f9060208501906060840190835b8181101561390d57835180518452602081015160208501526001600160a01b036040820151166040850152606081015160608501526001600160a01b0360808201511660808501526001600160a01b0360a08201511660a085015260c081015160c08501525060e083019250602084019350600181019050613946565b5f604082016040835280855180835260608501915060608160051b8601019250602087015f5b828110156136ad577fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa087860301845281518051865260208101516101006020880152613a3961010088018261359d565b9050604082015160408801526001600160a01b036060830151166060880152608082015160808801526001600160a01b0360a08301511660a088015260c0820151613a8f60c08901826001600160a01b03169052565b5060e091820151969091019590955260209384019391909101906001016139e9565b5f5f5f60608486031215613ac3575f5ffd5b83359250602084013567ffffffffffffffff811115613ae0575f5ffd5b840160808187031215613af1575f5ffd5b9150604084013567ffffffffffffffff811115613b0c575f5ffd5b840160a08187031215613b1d575f5ffd5b809150509250925092565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b808202811582820484141761063857610638613b28565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f82613ba757613ba7613b6c565b500490565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52600160045260245ffd5b8181038181111561063857610638613b28565b5f5f198203613bfd57613bfd613b28565b5060010190565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b600181811c90821680613c4557607f821691505b6020821081036136d3577f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b60405160a0810167ffffffffffffffff81118282101715613ccc57613ccc613c04565b60405290565b5f82601f830112613ce1575f5ffd5b813567ffffffffffffffff811115613cfb57613cfb613c04565b604051601f8201601f19908116603f0116810167ffffffffffffffff81118282101715613d2a57613d2a613c04565b604052818152838201602001851015613d41575f5ffd5b816020850160208301375f918101602001919091529392505050565b5f60208236031215613d6d575f5ffd5b6040516020810167ffffffffffffffff81118282101715613d9057613d90613c04565b604052823567ffffffffffffffff811115613da9575f5ffd5b613db536828601613cd2565b82525092915050565b601f821115611d3057805f5260205f20601f840160051c81016020851015613de35750805b601f840160051c820191505b81811015611e72575f8155600101613def565b815167ffffffffffffffff811115613e1c57613e1c613c04565b613e3081613e2a8454613c31565b84613dbe565b6020601f821160018114613e62575f8315613e4b5750848201515b5f19600385901b1c1916600184901b178455611e72565b5f84815260208120601f198516915b82811015613e915787850151825560209485019460019092019101613e71565b5084821015613eae57868401515f19600387901b60f8161c191681555b50505050600190811b01905550565b81835281816020850137505f602082840101525f6020601f19601f840116840101905092915050565b5f81357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe1833603018112613f18575f5ffd5b820160208101903567ffffffffffffffff811115613f34575f5ffd5b803603821315613f42575f5ffd5b6020855261294a602086018284613ebd565b608081525f613f666080830187613ee6565b60208301959095525060408101929092526001600160a01b0316606090910152919050565b848152608060208201525f613fa36080830186613ee6565b6001600160a01b03949094166040830152506060015292915050565b5f5f83357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe1843603018112613ff2575f5ffd5b83018035915067ffffffffffffffff82111561400c575f5ffd5b60200191503681900382131561330b575f5ffd5b602081525f611d5a602083018486613ebd565b80357fffffffff000000000000000000000000000000000000000000000000000000008116811461373c575f5ffd5b5f60808236031215614072575f5ffd5b6040516080810167ffffffffffffffff8111828210171561409557614095613c04565b6040526140a183614033565b8152602083013567ffffffffffffffff8111156140bc575f5ffd5b6140c836828601613cd2565b602083015250604083013567ffffffffffffffff8111156140e7575f5ffd5b6140f336828601613cd2565b60408301525061410560608401614033565b606082015292915050565b5f60a08236031215614120575f5ffd5b614128613ca9565b823567ffffffffffffffff81111561413e575f5ffd5b61414a36828601613cd2565b82525060208381013590820152604083013567ffffffffffffffff811115614170575f5ffd5b61417c36828601613cd2565b60408301525060608381013590820152608083013567ffffffffffffffff8111156141a5575f5ffd5b6141b136828601613cd2565b60808301525092915050565b8082018082111561063857610638613b28565b7fff000000000000000000000000000000000000000000000000000000000000008360f81b1681525f5f835461420581613c31565b60018216801561421c57600181146142555761428b565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff008316600187015260018215158302870101935061428b565b865f5260205f205f5b838110156142805781546001828a01015260018201915060208101905061425e565b505060018287010193505b50919695505050505050565b5f602082840312156142a7575f5ffd5b81518015158114610ad0575f5ffd5b5f81518060208401855e5f93019283525090919050565b7fffffffff00000000000000000000000000000000000000000000000000000000851681525f61430961430360048401876142b6565b856142b6565b7fffffffff0000000000000000000000000000000000000000000000000000000093909316835250506004019392505050565b5f610ad082846142b6565b5f60208284031215614357575f5ffd5b5051919050565b67ffffffffffffffff818116838216019081111561063857610638613b28565b5f8261438c5761438c613b6c565b500690565b60ff828116828216039081111561063857610638613b28565b6001815b60018411156143e5578085048111156143c9576143c9613b28565b60018416156143d757908102905b60019390931c9280026143ae565b935093915050565b5f826143fb57506001610638565b8161440757505f610638565b816001811461441d576002811461442757614443565b6001915050610638565b60ff84111561443857614438613b28565b50506001821b610638565b5060208310610133831016604e8410600b8410161715614466575081810a610638565b6144725f1984846143aa565b805f190482111561448557614485613b28565b029392505050565b5f610ad083836143ed565b60ff818116838216019081111561063857610638613b28565b602081525f610ad0602083018461356f56fea264697066735822122041df18e578a041c2e52d8f05eb633ba96c00113b60873f78cbe14aff1cb17f0764736f6c634300081c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x01nW_5`\xE0\x1C\x80cj\x8C\xDE:\x11a\0\xD2W\x80c\xC5jE&\x11a\0\x88W\x80c\xDFi\xB1O\x11a\0cW\x80c\xDFi\xB1O\x14a\x03\xB2W\x80c\xEC\xCA,6\x14a\x03\xC5W\x80c\xFD?\xC2E\x14a\x042W__\xFD[\x80c\xC5jE&\x14a\x03|W\x80c\xCE\x1B\x81_\x14a\x03\x8FW\x80c\xD1\x92\x0F\xF0\x14a\x03\xA9W__\xFD[\x80c\xA3\x83\x01;\x11a\0\xB8W\x80c\xA3\x83\x01;\x14a\x02\xBAW\x80c\xB2#\xD9v\x14a\x02\xCDW\x80c\xBD*~>\x14a\x02\xE0W__\xFD[\x80cj\x8C\xDE:\x14a\x02\x8EW\x80c\x9C\xC6r.\x14a\x02\xA4W__\xFD[\x80cAEd\n\x11a\x01'W\x80cW+l\x05\x11a\x01\rW\x80cW+l\x05\x14a\x024W\x80c[\x8F\xE0B\x14a\x02eW\x80ch\x11\xA3\x11\x14a\x02xW__\xFD[\x80cAEd\n\x14a\x01\xFAW\x80cPj\x10\x9D\x14a\x02!W__\xFD[\x80c!\x0E\xC1\x81\x11a\x01WW\x80c!\x0E\xC1\x81\x14a\x01\xAEW\x80c6O\x1E\xC0\x14a\x01\xC1W\x80c:\xF3\xFC~\x14a\x01\xD6W__\xFD[\x80c\x11\xC17\xAA\x14a\x01rW\x80c\x1D\xFEu\x95\x14a\x01\x98W[__\xFD[a\x01\x85a\x01\x806`\x04a5OV[a\x04EV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\xA0a\x06>V[`@Qa\x01\x8F\x92\x91\x90a5\xEBV[a\x01\x85a\x01\xBC6`\x04a6\xD9V[a\x08\xA0V[a\x01\xD4a\x01\xCF6`\x04a7AV[a\n\xD7V[\0[a\x01\xE9a\x01\xE46`\x04a7\x9CV[a\x0C,V[`@Qa\x01\x8F\x95\x94\x93\x92\x91\x90a7\xB3V[a\x02\ra\x02\x086`\x04a7\x9CV[a\x0C\xFEV[`@Qa\x01\x8F\x98\x97\x96\x95\x94\x93\x92\x91\x90a7\xFBV[a\x01\xD4a\x02/6`\x04a7\x9CV[a\r\xEFV[a\x02Ua\x02B6`\x04a8PV[_T`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x91\x16\x14\x90V[`@Q\x90\x15\x15\x81R` \x01a\x01\x8FV[a\x01\xD4a\x02s6`\x04a8iV[a\x0E\xD4V[a\x02\x80a\x0F\xF0V[`@Qa\x01\x8F\x92\x91\x90a8\x9CV[a\x02\x96a\x11\xB4V[`@Qa\x01\x8F\x92\x91\x90a9+V[a\x02\xACa\x13\xBAV[`@Qa\x01\x8F\x92\x91\x90a9\xC3V[a\x01\xD4a\x02\xC86`\x04a7\x9CV[a\x16\x1DV[a\x01\xD4a\x02\xDB6`\x04a:\xB1V[a\x16\xC1V[a\x039a\x02\xEE6`\x04a7\x9CV[`\x02` \x81\x90R_\x91\x82R`@\x90\x91 \x80T`\x01\x82\x01T\x92\x82\x01T`\x03\x83\x01T`\x04\x84\x01T`\x05\x85\x01T`\x06\x90\x95\x01T\x93\x95\x94`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x94\x92\x93\x91\x82\x16\x92\x91\x16\x90\x87V[`@\x80Q\x97\x88R` \x88\x01\x96\x90\x96R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x95\x87\x01\x95\x90\x95R``\x86\x01\x92\x90\x92R\x82\x16`\x80\x85\x01R\x16`\xA0\x83\x01R`\xC0\x82\x01R`\xE0\x01a\x01\x8FV[a\x01\xD4a\x03\x8A6`\x04a7\x9CV[a\x18]V[_T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\x8FV[a\x01\x85aT`\x81V[a\x01\xD4a\x03\xC06`\x04a7\x9CV[a\x19@V[a\x04\x07a\x03\xD36`\x04a7\x9CV[`\x03` \x81\x90R_\x91\x82R`@\x90\x91 \x80T`\x01\x82\x01T`\x02\x83\x01T\x92\x90\x93\x01T\x90\x92`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x92\x91\x16\x84V[`@\x80Q\x94\x85R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16` \x86\x01R\x84\x01\x91\x90\x91R\x16``\x82\x01R`\x80\x01a\x01\x8FV[a\x01\xD4a\x04@6`\x04a:\xB1V[a\x1ASV[_\x82\x81R`\x01` R`@\x81 \x80T\x83\x11\x15a\x04_W__\xFD[_\x83\x11a\x04jW__\xFD[\x80T`\x03\x82\x01T_\x91\x90a\x04~\x90\x86a;UV[a\x04\x88\x91\x90a;\x99V[\x90P_\x81\x11a\x04\x99Wa\x04\x99a;\xACV[\x80\x82`\x03\x01T\x10\x15a\x04\xADWa\x04\xADa;\xACV[\x80\x82`\x03\x01_\x82\x82Ta\x04\xC0\x91\x90a;\xD9V[\x90\x91UPP\x81T\x84\x90\x83\x90_\x90a\x04\xD8\x90\x84\x90a;\xD9V[\x90\x91UPP`@\x80Q`\xE0\x81\x01\x82R\x86\x81R` \x81\x01\x86\x90R`\x02\x84\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x83\x90R`\x04\x84\x01T\x90\x91\x16`\x80\x82\x01R_\x90`\xA0\x81\x01a\x05*a\x1B\xE0V[`\x01`\x01`\xA0\x1B\x03\x16\x81RB` \x90\x91\x01R`\x05\x80T\x91\x92P_\x91\x90\x82a\x05P\x83a;\xECV[\x90\x91UP_\x81\x81R`\x02` \x81\x81R`@\x92\x83\x90 \x86Q\x81U\x86\x82\x01Q`\x01\x82\x01U\x86\x84\x01Q\x81\x84\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x17\x90\x91U``\x80\x8A\x01Q`\x03\x85\x01U`\x80\x8A\x01Q`\x04\x85\x01\x80T\x84\x16\x91\x85\x16\x91\x90\x91\x17\x90U`\xA0\x8A\x01Q`\x05\x85\x01\x80T\x90\x93\x16\x90\x84\x16\x17\x90\x91U`\xC0\x89\x01Q`\x06\x90\x93\x01\x92\x90\x92U\x92\x89\x01T\x84Q\x8C\x81R\x92\x83\x01\x89\x90R\x90\x92\x16\x92\x81\x01\x92\x90\x92R\x91\x92P\x82\x91\x89\x91\x7F\xC3\x9A\x1A]\xDC\x0E\x85\xC9U\xFE.\x1A\xBE\xB4<\x94\xCE\x182.u\xBB=D\xE8\x0Fu\x9F\xF9\xD04\xB9\x91\x01`@Q\x80\x91\x03\x90\xA3\x93PPPP[\x92\x91PPV[``\x80_\x80[`\x05T\x81\x10\x15a\x06\x83W_\x81\x81R`\x01` R`@\x90 `\x04\x01T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x06{W\x81a\x06w\x81a;\xECV[\x92PP[`\x01\x01a\x06DV[P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06\x9EWa\x06\x9Ea<\x04V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x06\xD7W\x81` \x01[a\x06\xC4a4eV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x06\xBCW\x90P[P\x90P_\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06\xF4Wa\x06\xF4a<\x04V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x07\x1DW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_\x80[`\x05T\x81\x10\x15a\x08\x94W_\x81\x81R`\x01` R`@\x90 `\x04\x01T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x08\x8CW`\x01_\x82\x81R` \x01\x90\x81R` \x01_ `@Q\x80`\xA0\x01`@R\x90\x81_\x82\x01T\x81R` \x01`\x01\x82\x01`@Q\x80` \x01`@R\x90\x81_\x82\x01\x80Ta\x07\x90\x90a<1V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x07\xBC\x90a<1V[\x80\x15a\x08\x07W\x80`\x1F\x10a\x07\xDEWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x08\x07V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x07\xEAW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x90\x92RPPP\x81R`\x02\x82\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16` \x83\x01R`\x03\x83\x01T`@\x83\x01R`\x04\x90\x92\x01T\x90\x91\x16``\x90\x91\x01R\x84Q\x85\x90\x84\x90\x81\x10a\x08UWa\x08Ua<|V[` \x02` \x01\x01\x81\x90RP\x80\x83\x83\x81Q\x81\x10a\x08sWa\x08sa<|V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x81a\x08\x88\x81a;\xECV[\x92PP[`\x01\x01a\x07#V[P\x91\x95\x90\x94P\x92PPPV[_\x83\x81R`\x03` R`@\x81 \x82a\x08\xB6W__\xFD[\x80T\x83\x11\x15a\x08\xC3W__\xFD[\x80T`\x02\x82\x01T_\x91\x90a\x08\xD7\x90\x86a;UV[a\x08\xE1\x91\x90a;\x99V[\x90P_\x81\x11a\x08\xF2Wa\x08\xF2a;\xACV[\x80\x82`\x02\x01T\x10\x15a\t\x06Wa\t\x06a;\xACV[\x80\x82`\x02\x01_\x82\x82Ta\t\x19\x91\x90a;\xD9V[\x90\x91UPP\x81T\x84\x90\x83\x90_\x90a\t1\x90\x84\x90a;\xD9V[\x90\x91UPa\tX\x90Pa\tBa\x1B\xE0V[`\x01\x84\x01T`\x01`\x01`\xA0\x1B\x03\x16\x900\x84a\x1C0V[`\x05\x80T_\x91\x82a\th\x83a;\xECV[\x91\x90PU\x90P`@Q\x80a\x01\0\x01`@R\x80\x88\x81R` \x01\x87a\t\x8A\x90a=]V[\x81R` \x81\x01\x87\x90R`\x01\x85\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`@\x83\x01R``\x82\x01\x85\x90R`\x03\x86\x01T\x16`\x80\x82\x01R`\xA0\x01a\t\xC5a\x1B\xE0V[`\x01`\x01`\xA0\x1B\x03\x16\x81RB` \x91\x82\x01R_\x83\x81R`\x04\x82R`@\x90 \x82Q\x81U\x90\x82\x01Q\x80Q`\x01\x83\x01\x90\x81\x90a\t\xFE\x90\x82a>\x02V[PPP`@\x82\x81\x01Q`\x02\x83\x01U``\x83\x01Q`\x03\x83\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x17\x90\x91U`\x80\x85\x01Q`\x04\x85\x01U`\xA0\x85\x01Q`\x05\x85\x01\x80T\x83\x16\x91\x84\x16\x91\x90\x91\x17\x90U`\xC0\x85\x01Q`\x06\x85\x01\x80T\x90\x92\x16\x90\x83\x16\x17\x90U`\xE0\x90\x93\x01Q`\x07\x90\x92\x01\x91\x90\x91U`\x01\x85\x01T\x90Q\x83\x92\x8A\x92\x7Fe>\r\x81\xF2\xC9\x9B\xEB\xA3Y\xDF\xB1{I\x9A\\\xFF+\xE9\xD9PQHR\"M\xF8\xC0\x97\xC2\x19!\x92a\n\xC3\x92\x8C\x92\x8C\x92\x8A\x92\x91\x90\x91\x16\x90a?TV[`@Q\x80\x91\x03\x90\xA3\x92PPP[\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\n\xE9W__\xFD[a\x0B\x06a\n\xF4a\x1B\xE0V[`\x01`\x01`\xA0\x1B\x03\x84\x16\x900\x84a\x1C0V[`\x05\x80T_\x91\x82a\x0B\x16\x83a;\xECV[\x91\x90PU\x90P`@Q\x80`\xA0\x01`@R\x80\x86\x81R` \x01\x85a\x0B7\x90a=]V[\x81R` \x01\x84`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83\x81R` \x01a\x0BYa\x1B\xE0V[`\x01`\x01`\xA0\x1B\x03\x16\x90R_\x82\x81R`\x01` \x81\x81R`@\x90\x92 \x83Q\x81U\x91\x83\x01Q\x80Q\x90\x91\x83\x01\x90\x81\x90a\x0B\x8F\x90\x82a>\x02V[PPP`@\x82\x81\x01Q`\x02\x83\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x17\x90\x91U``\x85\x01Q`\x03\x85\x01U`\x80\x90\x94\x01Q`\x04\x90\x93\x01\x80T\x90\x94\x16\x92\x16\x91\x90\x91\x17\x90\x91UQ\x7F\x98\xC7\xC6\x80@=G@=\xEAJW\r\x0ElW\x16S\x8CIB\x0E\xF4q\xCE\xC4(\xF5\xA5\x85,\x06\x90a\x0C\x1D\x90\x87\x90\x87\x90\x87\x90\x87\x90a?\x8BV[`@Q\x80\x91\x03\x90\xA1PPPPPV[`\x01` \x81\x81R_\x92\x83R`@\x92\x83\x90 \x80T\x84Q\x92\x83\x01\x90\x94R\x91\x82\x01\x80T\x82\x90\x82\x90a\x0CY\x90a<1V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0C\x85\x90a<1V[\x80\x15a\x0C\xD0W\x80`\x1F\x10a\x0C\xA7Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0C\xD0V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0C\xB3W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x90\x92RPPP`\x02\x82\x01T`\x03\x83\x01T`\x04\x90\x93\x01T\x91\x92`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x92\x90\x91\x16\x85V[`\x04` R\x80_R`@_ _\x91P\x90P\x80_\x01T\x90\x80`\x01\x01`@Q\x80` \x01`@R\x90\x81_\x82\x01\x80Ta\r2\x90a<1V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\r^\x90a<1V[\x80\x15a\r\xA9W\x80`\x1F\x10a\r\x80Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\r\xA9V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\r\x8CW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x90\x92RPPP`\x02\x82\x01T`\x03\x83\x01T`\x04\x84\x01T`\x05\x85\x01T`\x06\x86\x01T`\x07\x90\x96\x01T\x94\x95\x93\x94`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x94\x92\x93\x91\x82\x16\x92\x90\x91\x16\x90\x88V[_\x81\x81R`\x01` R`@\x90 a\x0E\x04a\x1B\xE0V[`\x04\x82\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x16\x14a\x0E\x1FW__\xFD[a\x0EDa\x0E*a\x1B\xE0V[`\x03\x83\x01T`\x02\x84\x01T`\x01`\x01`\xA0\x1B\x03\x16\x91\x90a\x1C\xE7V[_\x82\x81R`\x01` \x81\x90R`@\x82 \x82\x81U\x91\x90\x82\x01\x81a\x0Ee\x82\x82a4\xA6V[PPP`\x02\x81\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x81\x16\x90\x91U_`\x03\x83\x01U`\x04\x90\x91\x01\x80T\x90\x91\x16\x90U`@Q\x82\x81R\x7F\xC3@\xE7\xACH\xDC\x80\xEEy?\xC6&i`\xBD_\x1B\xD2\x1B\xE9\x1C\x8A\x95\xE2\x18\x17\x81\x13\xF7\x9E\x17\xB4\x90` \x01[`@Q\x80\x91\x03\x90\xA1PPV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x0E\xE6W__\xFD[_\x83\x11a\x0E\xF1W__\xFD[_\x81\x11a\x0E\xFCW__\xFD[`\x05\x80T_\x91\x82a\x0F\x0C\x83a;\xECV[\x91\x90PU\x90P`@Q\x80`\x80\x01`@R\x80\x85\x81R` \x01\x84`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83\x81R` \x01a\x0F@a\x1B\xE0V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x90\x91R_\x83\x81R`\x03` \x81\x81R`@\x92\x83\x90 \x85Q\x81U\x85\x82\x01Q`\x01\x82\x01\x80T\x91\x87\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x92\x83\x16\x17\x90U\x86\x85\x01Q`\x02\x83\x01U``\x96\x87\x01Q\x91\x90\x93\x01\x80T\x91\x86\x16\x91\x90\x93\x16\x17\x90\x91U\x81Q\x88\x81R\x92\x87\x16\x90\x83\x01R\x81\x01\x84\x90R\x82\x91\x7F\xFF\x1C\xE2\x10\xDE\xFC\xD3\xBA\x1A\xDFv\xC9A\x9A\x07X\xFA`\xFD>\xB3\x8C{\xD9A\x8F`\xB5u\xB7n$\x91\x01`@Q\x80\x91\x03\x90\xA2PPPPV[``\x80_\x80[`\x05T\x81\x10\x15a\x106W_\x81\x81R`\x03` \x81\x90R`@\x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x10.W\x81a\x10*\x81a;\xECV[\x92PP[`\x01\x01a\x0F\xF6V[P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x10QWa\x10Qa<\x04V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x10\xA1W\x81` \x01[`@\x80Q`\x80\x81\x01\x82R_\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01\x81\x90R``\x82\x01R\x82R_\x19\x90\x92\x01\x91\x01\x81a\x10oW\x90P[P\x90P_\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x10\xBEWa\x10\xBEa<\x04V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x10\xE7W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_\x80[`\x05T\x81\x10\x15a\x08\x94W_\x81\x81R`\x03` \x81\x90R`@\x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x11\xACW_\x81\x81R`\x03` \x81\x81R`@\x92\x83\x90 \x83Q`\x80\x81\x01\x85R\x81T\x81R`\x01\x82\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x93\x82\x01\x93\x90\x93R`\x02\x82\x01T\x94\x81\x01\x94\x90\x94R\x90\x91\x01T\x16``\x82\x01R\x84Q\x85\x90\x84\x90\x81\x10a\x11uWa\x11ua<|V[` \x02` \x01\x01\x81\x90RP\x80\x83\x83\x81Q\x81\x10a\x11\x93Wa\x11\x93a<|V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x81a\x11\xA8\x81a;\xECV[\x92PP[`\x01\x01a\x10\xEDV[``\x80_\x80[`\x05T\x81\x10\x15a\x11\xF0W_\x81\x81R`\x02` R`@\x90 `\x01\x01T\x15a\x11\xE8W\x81a\x11\xE4\x81a;\xECV[\x92PP[`\x01\x01a\x11\xBAV[P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x12\x0BWa\x12\x0Ba<\x04V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x12\x90W\x81` \x01[a\x12}`@Q\x80`\xE0\x01`@R\x80_\x81R` \x01_\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x12)W\x90P[P\x90P_\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x12\xADWa\x12\xADa<\x04V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x12\xD6W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_\x80[`\x05T\x81\x10\x15a\x08\x94W_\x81\x81R`\x02` R`@\x90 `\x01\x01T\x15a\x13\xB2W_\x81\x81R`\x02` \x81\x81R`@\x92\x83\x90 \x83Q`\xE0\x81\x01\x85R\x81T\x81R`\x01\x82\x01T\x92\x81\x01\x92\x90\x92R\x91\x82\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x93\x82\x01\x93\x90\x93R`\x03\x82\x01T``\x82\x01R`\x04\x82\x01T\x83\x16`\x80\x82\x01R`\x05\x82\x01T\x90\x92\x16`\xA0\x83\x01R`\x06\x01T`\xC0\x82\x01R\x84Q\x85\x90\x84\x90\x81\x10a\x13{Wa\x13{a<|V[` \x02` \x01\x01\x81\x90RP\x80\x83\x83\x81Q\x81\x10a\x13\x99Wa\x13\x99a<|V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x81a\x13\xAE\x81a;\xECV[\x92PP[`\x01\x01a\x12\xDCV[``\x80_\x80[`\x05T\x81\x10\x15a\x13\xF6W_\x81\x81R`\x04` R`@\x90 `\x02\x01T\x15a\x13\xEEW\x81a\x13\xEA\x81a;\xECV[\x92PP[`\x01\x01a\x13\xC0V[P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x14\x11Wa\x14\x11a<\x04V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x14JW\x81` \x01[a\x147a4\xE0V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x14/W\x90P[P\x90P_\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x14gWa\x14ga<\x04V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x14\x90W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_\x80[`\x05T\x81\x10\x15a\x08\x94W_\x81\x81R`\x04` R`@\x90 `\x02\x01T\x15a\x16\x15W`\x04_\x82\x81R` \x01\x90\x81R` \x01_ `@Q\x80a\x01\0\x01`@R\x90\x81_\x82\x01T\x81R` \x01`\x01\x82\x01`@Q\x80` \x01`@R\x90\x81_\x82\x01\x80Ta\x14\xFB\x90a<1V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x15'\x90a<1V[\x80\x15a\x15rW\x80`\x1F\x10a\x15IWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x15rV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x15UW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x90\x92RPPP\x81R`\x02\x82\x01T` \x82\x01R`\x03\x82\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`@\x83\x01R`\x04\x83\x01T``\x83\x01R`\x05\x83\x01T\x81\x16`\x80\x83\x01R`\x06\x83\x01T\x16`\xA0\x82\x01R`\x07\x90\x91\x01T`\xC0\x90\x91\x01R\x84Q\x85\x90\x84\x90\x81\x10a\x15\xDEWa\x15\xDEa<|V[` \x02` \x01\x01\x81\x90RP\x80\x83\x83\x81Q\x81\x10a\x15\xFCWa\x15\xFCa<|V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x81a\x16\x11\x81a;\xECV[\x92PP[`\x01\x01a\x14\x96V[_\x81\x81R`\x03` R`@\x90 a\x162a\x1B\xE0V[`\x03\x82\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x16\x14a\x16MW__\xFD[_\x82\x81R`\x03` \x81\x81R`@\x80\x84 \x84\x81U`\x01\x81\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x81\x16\x90\x91U`\x02\x82\x01\x95\x90\x95U\x90\x92\x01\x80T\x90\x93\x16\x90\x92UQ\x83\x81R\x7F<\xD4u\xB0\x92\xE8\xB3y\xF6\xBA\r\x9E\x0E\x0C\x8F0p^s2\x1D\xC5\xC9\xF8\x0C\xE4\xAD8\xDB{\xE1\xAA\x91\x01a\x0E\xC8V[_\x83\x81R`\x02` R`@\x90 a\x16\xD6a\x1B\xE0V[`\x05\x82\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x16\x14a\x16\xF1W__\xFD[`\x06T`\x01`\x01`\xA0\x1B\x03\x16c\xD3\x8C)\xA1a\x17\x0F`@\x85\x01\x85a?\xBFV[`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x17,\x92\x91\x90a@ V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x17CW__\xFD[PZ\xF1\x15\x80\x15a\x17UW=__>=_\xFD[PPPPa\x17\x86`\x07T\x84a\x17i\x90a@bV[a\x17r\x85aA\x10V[`\x06T`\x01`\x01`\xA0\x1B\x03\x16\x92\x91\x90a\x1D5V[P\x80T_\x90\x81R`\x01` \x81\x90R`@\x90\x91 \x80T\x90\x91a\x17\xAA\x91\x90\x83\x01\x86a\x1DbV[`\x05\x82\x01T`\x03\x83\x01T`\x02\x84\x01Ta\x17\xD1\x92`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x92\x91\x16\x90a\x1C\xE7V[_\x85\x81R`\x02` \x81\x81R`@\x80\x84 \x84\x81U`\x01\x81\x01\x85\x90U\x92\x83\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x81\x16\x90\x91U`\x03\x84\x01\x85\x90U`\x04\x84\x01\x80T\x82\x16\x90U`\x05\x84\x01\x80T\x90\x91\x16\x90U`\x06\x90\x92\x01\x92\x90\x92UQ\x86\x81R\x7F\xB4\xC9\x8D\xE2\x10ik<\xF2\x1E\x993\\\x1E\xE3\xA0\xAE4\xA2g\x13A*J\xDD\xE8\xAFYav\xF3~\x91\x01a\x0C\x1DV[_\x81\x81R`\x02` R`@\x90 a\x18ra\x1B\xE0V[`\x04\x82\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x16\x14a\x18\x8DW__\xFD[aT`\x81`\x06\x01Ta\x18\x9F\x91\x90aA\xBDV[B\x11a\x18\xA9W__\xFD[a\x18\xB4a\x0E*a\x1B\xE0V[_\x82\x81R`\x02` \x81\x81R`@\x80\x84 \x84\x81U`\x01\x81\x01\x85\x90U\x92\x83\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x81\x16\x90\x91U`\x03\x84\x01\x85\x90U`\x04\x84\x01\x80T\x82\x16\x90U`\x05\x84\x01\x80T\x90\x91\x16\x90U`\x06\x90\x92\x01\x92\x90\x92UQ\x83\x81R\x7F>^\xA3X\xE9\xEBL\xDFD\xCD\xC7y8\xAD\xE8\x07K\x12@\xA6\xD8\xC0\xFD\x13r\x86q\xB8.\x80\n\xD6\x91\x01a\x0E\xC8V[_\x81\x81R`\x04` R`@\x90 `\x07\x81\x01Ta\x19_\x90aT`\x90aA\xBDV[B\x11a\x19iW__\xFD[a\x19qa\x1B\xE0V[`\x06\x82\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x16\x14a\x19\x8CW__\xFD[a\x19\xB1a\x19\x97a\x1B\xE0V[`\x04\x83\x01T`\x03\x84\x01T`\x01`\x01`\xA0\x1B\x03\x16\x91\x90a\x1C\xE7V[_\x82\x81R`\x04` R`@\x81 \x81\x81U\x90`\x01\x82\x01\x81a\x19\xD1\x82\x82a4\xA6V[PP_`\x02\x83\x01\x81\x90U`\x03\x83\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x81\x16\x90\x91U`\x04\x84\x01\x82\x90U`\x05\x84\x01\x80T\x82\x16\x90U`\x06\x84\x01\x80T\x90\x91\x16\x90U`\x07\x90\x92\x01\x91\x90\x91UP`@Q\x82\x81R\x7Fx\xF5\x1Fb\xF7\xCF\x13\x81\xC6s\xC2~\xAE\x18}\xD6\xC5\x88\xDCf$\xCEYi}\xBB>\x1D|\x1B\xBC\xDF\x90` \x01a\x0E\xC8V[_\x83\x81R`\x04` R`@\x90 a\x1Aha\x1B\xE0V[`\x05\x82\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x16\x14a\x1A\x83W__\xFD[`\x06T`\x01`\x01`\xA0\x1B\x03\x16c\xD3\x8C)\xA1a\x1A\xA1`@\x85\x01\x85a?\xBFV[`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1A\xBE\x92\x91\x90a@ V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1A\xD5W__\xFD[PZ\xF1\x15\x80\x15a\x1A\xE7W=__>=_\xFD[PPPPa\x1A\xFB`\x07T\x84a\x17i\x90a@bV[Pa\x1B\x0E\x81`\x02\x01T\x82`\x01\x01\x85a\x1DbV[`\x05\x81\x01T`\x04\x82\x01T`\x03\x83\x01Ta\x1B5\x92`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x92\x91\x16\x90a\x1C\xE7V[_\x84\x81R`\x04` R`@\x81 \x81\x81U\x90`\x01\x82\x01\x81a\x1BU\x82\x82a4\xA6V[PP_`\x02\x83\x01\x81\x90U`\x03\x83\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x81\x16\x90\x91U`\x04\x84\x01\x82\x90U`\x05\x84\x01\x80T\x82\x16\x90U`\x06\x84\x01\x80T\x90\x91\x16\x90U`\x07\x90\x92\x01\x91\x90\x91UP`@Q\x84\x81R\x7F\xCFV\x10a\xDBx\xF7\xBCQ\x8D7\xFE\x86q\x85\x14\xC6@\xCC\xC5\xC3\xF1)8(\xB9U\xE6\x8F\x19\xF5\xFB\x90` \x01`@Q\x80\x91\x03\x90\xA1PPPPV[_`\x146\x10\x80\x15\x90a\x1B\xFBWP_T`\x01`\x01`\xA0\x1B\x03\x163\x14[\x15a\x1C+WP\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xEC6\x015``\x1C\x90V[P3\x90V[`@Q`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`$\x83\x01R\x83\x16`D\x82\x01R`d\x81\x01\x82\x90Ra\x1C\xE1\x90\x85\x90\x7F#\xB8r\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90`\x84\x01[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x93\x16\x92\x90\x92\x17\x90\x91Ra\x1EyV[PPPPV[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16`$\x82\x01R`D\x81\x01\x82\x90Ra\x1D0\x90\x84\x90\x7F\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90`d\x01a\x1C}V[PPPV[_a\x1D?\x83a\x1F]V[\x90Pa\x1DK\x81\x83a MV[a\x1DZ\x85\x85\x84`@\x01Qa\"\xB1V[\x94\x93PPPPV[_\x82_\x01\x80Ta\x1Dq\x90a<1V[`@Qa\x1D\x83\x92P\x85\x90` \x01aA\xD0V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P_a\x1D\xEA\x83\x80`@\x01\x90a\x1D\xAF\x91\x90a?\xBFV[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RP\x86\x92Pa&\x01\x91PPV[Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x84\x81\x10\x15a\x1ErW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`;`$\x82\x01R\x7FBitcoin transaction amount is lo`D\x82\x01R\x7Fwer than in accepted order.\0\0\0\0\0`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[PPPPPV[_a\x1E\xCD\x82`@Q\x80`@\x01`@R\x80` \x81R` \x01\x7FSafeERC20: low-level call failed\x81RP\x85`\x01`\x01`\xA0\x1B\x03\x16a'\x9F\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x80Q\x90\x91P\x15a\x1D0W\x80\x80` \x01\x90Q\x81\x01\x90a\x1E\xEB\x91\x90aB\x97V[a\x1D0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01R\x7Fot succeed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x1EiV[_a\x1Fk\x82` \x01Qa'\xADV[a\x1F\xB7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FInvalid input vector provided\0\0\0`D\x82\x01R`d\x01a\x1EiV[a\x1F\xC4\x82`@\x01Qa(GV[a \x10W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FInvalid output vector provided\0\0`D\x82\x01R`d\x01a\x1EiV[a\x068\x82_\x01Q\x83` \x01Q\x84`@\x01Q\x85``\x01Q`@Q` \x01a 9\x94\x93\x92\x91\x90aB\xCDV[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra(\xD4V[\x80Qa X\x90a(\xF6V[a \xA4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01R\x7FBad merkle array proof\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x1EiV[`\x80\x81\x01QQ\x81QQ\x14a! W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FTx not on same level of merkle t`D\x82\x01R\x7Free as coinbase\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x1EiV[_a!.\x82`@\x01Qa)\x0CV[\x82Q` \x84\x01Q\x91\x92Pa!E\x91\x85\x91\x84\x91a)\x18V[a!\xB7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`<`$\x82\x01R\x7FTx merkle proof is not valid for`D\x82\x01R\x7F provided header and tx hash\0\0\0\0`d\x82\x01R`\x84\x01a\x1EiV[_`\x02\x83``\x01Q`@Q` \x01a!\xD1\x91\x81R` \x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra!\xEB\x91aC<V[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\"\x06W=__>=_\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\")\x91\x90aCGV[`\x80\x84\x01Q\x90\x91Pa\"?\x90\x82\x90\x84\x90_a)\x18V[a\x1C\xE1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`?`$\x82\x01R\x7FCoinbase merkle proof is not val`D\x82\x01R\x7Fid for provided header and hash\0`d\x82\x01R`\x84\x01a\x1EiV[_\x83`\x01`\x01`\xA0\x1B\x03\x16c\x117d\xBE`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\"\xEEW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a#\x12\x91\x90aCGV[\x90P_\x84`\x01`\x01`\xA0\x1B\x03\x16c+\x97\xBE$`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a#QW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a#u\x91\x90aCGV[\x90P_\x80a#\x8Aa#\x85\x86a)SV[a)^V[\x90P\x83\x81\x03a#\x9BW\x83\x91Pa$\x18V[\x82\x81\x03a#\xAAW\x82\x91Pa$\x18V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FNot at current or previous diffi`D\x82\x01R\x7Fculty\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x1EiV[_a$\"\x86a)\x85V[\x90P_\x19\x81\x03a$\x9AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FInvalid length of the headers ch`D\x82\x01R\x7Fain\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x1EiV[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x03a%\tW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01R\x7FInvalid headers chain\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x1EiV[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFD\x81\x03a%xW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FInsufficient work in a header\0\0\0`D\x82\x01R`d\x01a\x1EiV[a%\x82\x87\x84a;UV[\x81\x10\x15a%\xF7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`3`$\x82\x01R\x7FInsufficient accumulated difficu`D\x82\x01R\x7Flty in header chain\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x1EiV[PPPPPPPPV[`@\x80Q``\x81\x01\x82R_\x80\x82R` \x80\x83\x01\x82\x90R\x82\x84\x01\x82\x90R\x83Q\x80\x85\x01\x90\x94R\x81\x84R\x83\x01R\x90a&5\x84a+\xA9V[` \x83\x01R\x80\x82R\x81a&G\x82a;\xECV[\x90RP_\x80[\x82` \x01Q\x81\x10\x15a'IW\x82Q_\x90a&h\x90\x88\x90a+\xBEV[\x84Q\x90\x91P_\x90a&z\x90\x89\x90a,\x1EV[\x90P_a&\x88`\x08\x84a;\xD9V[\x86Q\x90\x91P_\x90a&\x9A\x90`\x08aA\xBDV[\x8A\x81\x01` \x01\x83\x90 \x90\x91P\x80\x8A\x03a&\xD4W`\x01\x96P\x83\x89_\x01\x81\x81Qa&\xC2\x91\x90aC^V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90RPa'$V[_a&\xE2\x8C\x8A_\x01Qa,\x94V[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a'\x03W`\x01`\x01`\xA0\x1B\x03\x81\x16` \x8B\x01R[_a'\x11\x8D\x8B_\x01Qa-tV[\x90P\x80\x15a'!W`@\x8B\x01\x81\x90R[PP[\x84\x88_\x01\x81\x81Qa'5\x91\x90aA\xBDV[\x90RPP`\x01\x90\x94\x01\x93Pa&M\x92PPPV[P\x80a'\x97W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FNo output found for scriptPubKey`D\x82\x01R`d\x01a\x1EiV[PP\x92\x91PPV[``a\x1DZ\x84\x84_\x85a.TV[___a'\xB9\x84a+\xA9V[\x90\x92P\x90P\x80\x15\x80a'\xCBWP_\x19\x82\x14[\x15a'\xD9WP_\x93\x92PPPV[_a'\xE5\x83`\x01aA\xBDV[\x90P_[\x82\x81\x10\x15a(:W\x85Q\x82\x10a(\x04WP_\x95\x94PPPPPV[_a(\x0F\x87\x84a/\x98V[\x90P_\x19\x81\x03a(%WP_\x96\x95PPPPPPV[a(/\x81\x84aA\xBDV[\x92PP`\x01\x01a'\xE9V[P\x93Q\x90\x93\x14\x93\x92PPPV[___a(S\x84a+\xA9V[\x90\x92P\x90P\x80\x15\x80a(eWP_\x19\x82\x14[\x15a(sWP_\x93\x92PPPV[_a(\x7F\x83`\x01aA\xBDV[\x90P_[\x82\x81\x10\x15a(:W\x85Q\x82\x10a(\x9EWP_\x95\x94PPPPPV[_a(\xA9\x87\x84a+\xBEV[\x90P_\x19\x81\x03a(\xBFWP_\x96\x95PPPPPPV[a(\xC9\x81\x84aA\xBDV[\x92PP`\x01\x01a(\x83V[_` _\x83Q` \x85\x01`\x02Z\xFAP` _` _`\x02Z\xFAPP_Q\x91\x90PV[_` \x82Qa)\x05\x91\x90aC~V[\x15\x92\x91PPV[`D\x81\x01Q_\x90a\x068V[_\x83\x85\x14\x80\x15a)&WP\x81\x15[\x80\x15a)1WP\x82Q\x15[\x15a)>WP`\x01a\x1DZV[a)J\x85\x84\x86\x85a/\xDEV[\x95\x94PPPPPV[_a\x068\x82_a0\x83V[_a\x068{\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83a1\x1CV[_`P\x82Qa)\x94\x91\x90aC~V[\x15a)\xA1WP_\x19\x91\x90PV[P_\x80\x80[\x83Q\x81\x10\x15a+\xA2W\x80\x15a)\xEDWa)\xC0\x84\x82\x84a1'V[a)\xEDWP\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x93\x92PPPV[_a)\xF8\x85\x83a0\x83V[\x90Pa*\x06\x85\x83`Pa1PV[\x92P\x80a+I\x84_\x81\x90P`\x08\x81~\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\x16\x90\x1B`\x08\x82\x90\x1C~\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\x16\x17\x90P`\x10\x81}\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\x16\x90\x1B`\x10\x82\x90\x1C}\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\x16\x17\x90P` \x81{\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\x16\x90\x1B` \x82\x90\x1C{\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\x16\x17\x90P`@\x81w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x1B`@\x82\x90\x1Cw\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x17\x90P`\x80\x81\x90\x1B`\x80\x82\x90\x1C\x17\x90P\x91\x90PV[\x11\x15a+yWP\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFD\x94\x93PPPPV[a+\x82\x81a)^V[a+\x8C\x90\x85aA\xBDV[\x93Pa+\x9B\x90P`P\x82aA\xBDV[\x90Pa)\xA6V[PP\x91\x90PV[__a+\xB5\x83_a1uV[\x91P\x91P\x91P\x91V[_a+\xCA\x82`\taA\xBDV[\x83Q\x10\x15a+\xDAWP_\x19a\x068V[_\x80a+\xF0\x85a+\xEB\x86`\x08aA\xBDV[a1uV[\x90\x92P\x90P`\x01\x82\x01a,\x08W_\x19\x92PPPa\x068V[\x80a,\x14\x83`\taA\xBDV[a)J\x91\x90aA\xBDV[_\x80a,*\x84\x84a3\x12V[`\xC0\x1C\x90P_a)J\x82d\xFF\0\0\0\xFF`\x08\x82\x81\x1C\x91\x82\x16e\xFF\0\0\0\xFF\0\x93\x90\x91\x1B\x92\x83\x16\x17`\x10\x90\x81\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16f\xFF\0\xFF\0\xFF\0\xFF\x92\x90\x92\x16g\xFF\0\xFF\0\xFF\0\xFF\0\x90\x93\x16\x92\x90\x92\x17\x90\x91\x1Ce\xFF\xFF\0\0\xFF\xFF\x16\x17` \x81\x81\x1C\x91\x90\x1B\x17\x90V[_\x82a,\xA1\x83`\taA\xBDV[\x81Q\x81\x10a,\xB1Wa,\xB1a<|V[` \x91\x01\x01Q\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x7Fj\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x14a-\x06WP_a\x068V[_\x83a-\x13\x84`\naA\xBDV[\x81Q\x81\x10a-#Wa-#a<|V[\x01` \x01Q\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x91P`\xF8\x1C`\x14\x03a-mW_a-c\x84`\x0BaA\xBDV[\x85\x01`\x14\x01Q\x92PP[P\x92\x91PPV[_\x82a-\x81\x83`\taA\xBDV[\x81Q\x81\x10a-\x91Wa-\x91a<|V[` \x91\x01\x01Q\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x7Fj\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x14a-\xE6WP_a\x068V[_\x83a-\xF3\x84`\naA\xBDV[\x81Q\x81\x10a.\x03Wa.\x03a<|V[\x01` \x90\x81\x01Q\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x92P`\xF8\x1C\x90\x03a-mW_a.D\x84`\x0BaA\xBDV[\x85\x01` \x01Q\x92PPP\x92\x91PPV[``\x82G\x10\x15a.\xCCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FAddress: insufficient balance fo`D\x82\x01R\x7Fr call\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x1EiV[`\x01`\x01`\xA0\x1B\x03\x85\x16;a/#W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x1EiV[__\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@Qa/>\x91\x90aC<V[_`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80_\x81\x14a/xW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a/}V[``\x91P[P\x91P\x91Pa/\x8D\x82\x82\x86a3 V[\x97\x96PPPPPPPV[___a/\xA5\x85\x85a3YV[\x90\x92P\x90P`\x01\x82\x01a/\xBDW_\x19\x92PPPa\x068V[\x80a/\xC9\x83`%aA\xBDV[a/\xD3\x91\x90aA\xBDV[a)J\x90`\x04aA\xBDV[_` \x84Qa/\xED\x91\x90aC~V[\x15a/\xF9WP_a\x1DZV[\x83Q_\x03a0\x08WP_a\x1DZV[\x81\x85_[\x86Q\x81\x10\x15a0vWa0 `\x02\x84aC~V[`\x01\x03a0DWa0=a07\x88\x83\x01` \x01Q\x90V[\x83a3\x97V[\x91Pa0]V[a0Z\x82a0U\x89\x84\x01` \x01Q\x90V[a3\x97V[\x91P[`\x01\x92\x90\x92\x1C\x91a0o` \x82aA\xBDV[\x90Pa0\x0CV[P\x90\x93\x14\x95\x94PPPPPV[_\x80a0\x9Aa0\x93\x84`HaA\xBDV[\x85\x90a3\x12V[`\xE8\x1C\x90P_\x84a0\xAC\x85`KaA\xBDV[\x81Q\x81\x10a0\xBCWa0\xBCa<|V[\x01` \x01Q`\xF8\x1C\x90P_a0\xEE\x83_`\x10\x82b\xFF\xFF\xFF\x16\x90\x1C\x82a\xFF\0\x16`\x10\x84b\xFF\xFF\xFF\x16\x90\x1B\x17\x17\x90P\x91\x90PV[b\xFF\xFF\xFF\x16\x90P_a1\x01`\x03\x84aC\x91V[`\xFF\x16\x90Pa1\x12\x81a\x01\0aD\x8DV[a/\x8D\x90\x83a;UV[_a\n\xD0\x82\x84a;\x99V[_\x80a13\x85\x85a3\xA2V[\x90P\x82\x81\x14a1EW_\x91PPa\n\xD0V[P`\x01\x94\x93PPPPV[_` _\x83\x85` \x01\x87\x01`\x02Z\xFAP` _` _`\x02Z\xFAPP_Q\x93\x92PPPV[___a1\x82\x85\x85a3\xBAV[\x90P\x80`\xFF\x16_\x03a1\xB5W_\x85\x85\x81Q\x81\x10a1\xA1Wa1\xA1a<|V[\x01` \x01Q\x90\x93P`\xF8\x1C\x91Pa3\x0B\x90PV[\x83a1\xC1\x82`\x01aD\x98V[`\xFF\x16a1\xCE\x91\x90aA\xBDV[\x85Q\x10\x15a1\xE3W_\x19_\x92P\x92PPa3\x0BV[_\x81`\xFF\x16`\x02\x03a2&Wa2\x1Ba2\x07a2\0\x87`\x01aA\xBDV[\x88\x90a3\x12V[b\xFF\xFF\0`\xE8\x82\x90\x1C\x16`\xF8\x91\x90\x91\x1C\x17\x90V[a\xFF\xFF\x16\x90Pa3\x01V[\x81`\xFF\x16`\x04\x03a2uWa2ha2Ba2\0\x87`\x01aA\xBDV[`\xD8\x81\x90\x1Cc\xFF\0\xFF\0\x16b\xFF\0\xFF`\xE8\x92\x90\x92\x1C\x91\x90\x91\x16\x17`\x10\x81\x81\x1B\x91\x90\x1C\x17\x90V[c\xFF\xFF\xFF\xFF\x16\x90Pa3\x01V[\x81`\xFF\x16`\x08\x03a3\x01Wa2\xF4a2\x91a2\0\x87`\x01aA\xBDV[`\xC0\x1Cd\xFF\0\0\0\xFF`\x08\x82\x81\x1C\x91\x82\x16e\xFF\0\0\0\xFF\0\x93\x90\x91\x1B\x92\x83\x16\x17`\x10\x90\x81\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16f\xFF\0\xFF\0\xFF\0\xFF\x92\x90\x92\x16g\xFF\0\xFF\0\xFF\0\xFF\0\x90\x93\x16\x92\x90\x92\x17\x90\x91\x1Ce\xFF\xFF\0\0\xFF\xFF\x16\x17` \x81\x81\x1C\x91\x90\x1B\x17\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P[`\xFF\x90\x91\x16\x92P\x90P[\x92P\x92\x90PV[_a\n\xD0\x83\x83\x01` \x01Q\x90V[``\x83\x15a3/WP\x81a\n\xD0V[\x82Q\x15a3?W\x82Q\x80\x84` \x01\xFD[\x81`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x1Ei\x91\x90aD\xB1V[_\x80a3f\x83`%aA\xBDV[\x84Q\x10\x15a3yWP_\x19\x90P_a3\x0BV[_\x80a3\x8A\x86a+\xEB\x87`$aA\xBDV[\x90\x97\x90\x96P\x94PPPPPV[_a\n\xD0\x83\x83a4>V[_a\n\xD0a3\xB1\x83`\x04aA\xBDV[\x84\x01` \x01Q\x90V[_\x82\x82\x81Q\x81\x10a3\xCDWa3\xCDa<|V[\x01` \x01Q`\xF8\x1C`\xFF\x03a3\xE4WP`\x08a\x068V[\x82\x82\x81Q\x81\x10a3\xF6Wa3\xF6a<|V[\x01` \x01Q`\xF8\x1C`\xFE\x03a4\rWP`\x04a\x068V[\x82\x82\x81Q\x81\x10a4\x1FWa4\x1Fa<|V[\x01` \x01Q`\xF8\x1C`\xFD\x03a46WP`\x02a\x068V[P_\x92\x91PPV[_\x82_R\x81` R` _`@_`\x02Z\xFAP` _` _`\x02Z\xFAPP_Q\x92\x91PPV[`@Q\x80`\xA0\x01`@R\x80_\x81R` \x01a4\x8C`@Q\x80` \x01`@R\x80``\x81RP\x90V[\x81R_` \x82\x01\x81\x90R`@\x82\x01\x81\x90R``\x90\x91\x01R\x90V[P\x80Ta4\xB2\x90a<1V[_\x82U\x80`\x1F\x10a4\xC1WPPV[`\x1F\x01` \x90\x04\x90_R` _ \x90\x81\x01\x90a4\xDD\x91\x90a57V[PV[`@Q\x80a\x01\0\x01`@R\x80_\x81R` \x01a5\x08`@Q\x80` \x01`@R\x80``\x81RP\x90V[\x81R_` \x82\x01\x81\x90R`@\x82\x01\x81\x90R``\x82\x01\x81\x90R`\x80\x82\x01\x81\x90R`\xA0\x82\x01\x81\x90R`\xC0\x90\x91\x01R\x90V[[\x80\x82\x11\x15a5KW_\x81U`\x01\x01a58V[P\x90V[__`@\x83\x85\x03\x12\x15a5`W__\xFD[PP\x805\x92` \x90\x91\x015\x91PV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[_\x81Q` \x84Ra\x1DZ` \x85\x01\x82a5oV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15a5\xE1W\x81Q\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01a5\xC3V[P\x93\x94\x93PPPPV[_`@\x82\x01`@\x83R\x80\x85Q\x80\x83R``\x85\x01\x91P``\x81`\x05\x1B\x86\x01\x01\x92P` \x87\x01_[\x82\x81\x10\x15a6\xADW\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA0\x87\x86\x03\x01\x84R\x81Q\x80Q\x86R` \x81\x01Q`\xA0` \x88\x01Ra6_`\xA0\x88\x01\x82a5\x9DV[\x90P`\x01`\x01`\xA0\x1B\x03`@\x83\x01Q\x16`@\x88\x01R``\x82\x01Q``\x88\x01R`\x01`\x01`\xA0\x1B\x03`\x80\x83\x01Q\x16`\x80\x88\x01R\x80\x96PPP` \x82\x01\x91P` \x84\x01\x93P`\x01\x81\x01\x90Pa6\x11V[PPPP\x82\x81\x03` \x84\x01Ra)J\x81\x85a5\xB1V[_` \x82\x84\x03\x12\x15a6\xD3W__\xFD[P\x91\x90PV[___``\x84\x86\x03\x12\x15a6\xEBW__\xFD[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a7\x08W__\xFD[a7\x14\x86\x82\x87\x01a6\xC3V[\x93\x96\x93\x95PPPP`@\x91\x90\x91\x015\x90V[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a7<W__\xFD[\x91\x90PV[____`\x80\x85\x87\x03\x12\x15a7TW__\xFD[\x845\x93P` \x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a7qW__\xFD[a7}\x87\x82\x88\x01a6\xC3V[\x93PPa7\x8C`@\x86\x01a7&V[\x93\x96\x92\x95P\x92\x93``\x015\x92PPV[_` \x82\x84\x03\x12\x15a7\xACW__\xFD[P5\x91\x90PV[\x85\x81R`\xA0` \x82\x01R_a7\xCB`\xA0\x83\x01\x87a5\x9DV[\x90P`\x01`\x01`\xA0\x1B\x03\x85\x16`@\x83\x01R\x83``\x83\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x80\x83\x01R\x96\x95PPPPPPV[\x88\x81Ra\x01\0` \x82\x01R_a8\x15a\x01\0\x83\x01\x8Aa5\x9DV[`@\x83\x01\x98\x90\x98RP`\x01`\x01`\xA0\x1B\x03\x95\x86\x16``\x82\x01R`\x80\x81\x01\x94\x90\x94R\x91\x84\x16`\xA0\x84\x01R\x90\x92\x16`\xC0\x82\x01R`\xE0\x01R\x92\x91PPV[_` \x82\x84\x03\x12\x15a8`W__\xFD[a\n\xD0\x82a7&V[___``\x84\x86\x03\x12\x15a8{W__\xFD[\x835\x92Pa8\x8B` \x85\x01a7&V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`@\x80\x82R\x83Q\x90\x82\x01\x81\x90R_\x90` \x85\x01\x90``\x84\x01\x90\x83[\x81\x81\x10\x15a9\rW\x83Q\x80Q\x84R`\x01`\x01`\xA0\x1B\x03` \x82\x01Q\x16` \x85\x01R`@\x81\x01Q`@\x85\x01R`\x01`\x01`\xA0\x1B\x03``\x82\x01Q\x16``\x85\x01RP`\x80\x83\x01\x92P` \x84\x01\x93P`\x01\x81\x01\x90Pa8\xB7V[PP\x83\x81\x03` \x85\x01Ra9!\x81\x86a5\xB1V[\x96\x95PPPPPPV[`@\x80\x82R\x83Q\x90\x82\x01\x81\x90R_\x90` \x85\x01\x90``\x84\x01\x90\x83[\x81\x81\x10\x15a9\rW\x83Q\x80Q\x84R` \x81\x01Q` \x85\x01R`\x01`\x01`\xA0\x1B\x03`@\x82\x01Q\x16`@\x85\x01R``\x81\x01Q``\x85\x01R`\x01`\x01`\xA0\x1B\x03`\x80\x82\x01Q\x16`\x80\x85\x01R`\x01`\x01`\xA0\x1B\x03`\xA0\x82\x01Q\x16`\xA0\x85\x01R`\xC0\x81\x01Q`\xC0\x85\x01RP`\xE0\x83\x01\x92P` \x84\x01\x93P`\x01\x81\x01\x90Pa9FV[_`@\x82\x01`@\x83R\x80\x85Q\x80\x83R``\x85\x01\x91P``\x81`\x05\x1B\x86\x01\x01\x92P` \x87\x01_[\x82\x81\x10\x15a6\xADW\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA0\x87\x86\x03\x01\x84R\x81Q\x80Q\x86R` \x81\x01Qa\x01\0` \x88\x01Ra:9a\x01\0\x88\x01\x82a5\x9DV[\x90P`@\x82\x01Q`@\x88\x01R`\x01`\x01`\xA0\x1B\x03``\x83\x01Q\x16``\x88\x01R`\x80\x82\x01Q`\x80\x88\x01R`\x01`\x01`\xA0\x1B\x03`\xA0\x83\x01Q\x16`\xA0\x88\x01R`\xC0\x82\x01Qa:\x8F`\xC0\x89\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`\xE0\x91\x82\x01Q\x96\x90\x91\x01\x95\x90\x95R` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a9\xE9V[___``\x84\x86\x03\x12\x15a:\xC3W__\xFD[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a:\xE0W__\xFD[\x84\x01`\x80\x81\x87\x03\x12\x15a:\xF1W__\xFD[\x91P`@\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a;\x0CW__\xFD[\x84\x01`\xA0\x81\x87\x03\x12\x15a;\x1DW__\xFD[\x80\x91PP\x92P\x92P\x92V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x068Wa\x068a;(V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_\x82a;\xA7Wa;\xA7a;lV[P\x04\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x01`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x068Wa\x068a;(V[__\x19\x82\x03a;\xFDWa;\xFDa;(V[P`\x01\x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[`\x01\x81\x81\x1C\x90\x82\x16\x80a<EW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a6\xD3W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a<\xCCWa<\xCCa<\x04V[`@R\x90V[_\x82`\x1F\x83\x01\x12a<\xE1W__\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a<\xFBWa<\xFBa<\x04V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a=*Wa=*a<\x04V[`@R\x81\x81R\x83\x82\x01` \x01\x85\x10\x15a=AW__\xFD[\x81` \x85\x01` \x83\x017_\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[_` \x826\x03\x12\x15a=mW__\xFD[`@Q` \x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a=\x90Wa=\x90a<\x04V[`@R\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a=\xA9W__\xFD[a=\xB56\x82\x86\x01a<\xD2V[\x82RP\x92\x91PPV[`\x1F\x82\x11\x15a\x1D0W\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a=\xE3WP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x1ErW_\x81U`\x01\x01a=\xEFV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a>\x1CWa>\x1Ca<\x04V[a>0\x81a>*\x84Ta<1V[\x84a=\xBEV[` `\x1F\x82\x11`\x01\x81\x14a>bW_\x83\x15a>KWP\x84\x82\x01Q[_\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua\x1ErV[_\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a>\x91W\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a>qV[P\x84\x82\x10\x15a>\xAEW\x86\x84\x01Q_\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[\x81\x83R\x81\x81` \x85\x017P_` \x82\x84\x01\x01R_` `\x1F\x19`\x1F\x84\x01\x16\x84\x01\x01\x90P\x92\x91PPV[_\x815\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x836\x03\x01\x81\x12a?\x18W__\xFD[\x82\x01` \x81\x01\x905g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a?4W__\xFD[\x806\x03\x82\x13\x15a?BW__\xFD[` \x85Ra)J` \x86\x01\x82\x84a>\xBDV[`\x80\x81R_a?f`\x80\x83\x01\x87a>\xE6V[` \x83\x01\x95\x90\x95RP`@\x81\x01\x92\x90\x92R`\x01`\x01`\xA0\x1B\x03\x16``\x90\x91\x01R\x91\x90PV[\x84\x81R`\x80` \x82\x01R_a?\xA3`\x80\x83\x01\x86a>\xE6V[`\x01`\x01`\xA0\x1B\x03\x94\x90\x94\x16`@\x83\x01RP``\x01R\x92\x91PPV[__\x835\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x846\x03\x01\x81\x12a?\xF2W__\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a@\x0CW__\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a3\x0BW__\xFD[` \x81R_a\x1DZ` \x83\x01\x84\x86a>\xBDV[\x805\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x81\x14a7<W__\xFD[_`\x80\x826\x03\x12\x15a@rW__\xFD[`@Q`\x80\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a@\x95Wa@\x95a<\x04V[`@Ra@\xA1\x83a@3V[\x81R` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a@\xBCW__\xFD[a@\xC86\x82\x86\x01a<\xD2V[` \x83\x01RP`@\x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a@\xE7W__\xFD[a@\xF36\x82\x86\x01a<\xD2V[`@\x83\x01RPaA\x05``\x84\x01a@3V[``\x82\x01R\x92\x91PPV[_`\xA0\x826\x03\x12\x15aA W__\xFD[aA(a<\xA9V[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aA>W__\xFD[aAJ6\x82\x86\x01a<\xD2V[\x82RP` \x83\x81\x015\x90\x82\x01R`@\x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aApW__\xFD[aA|6\x82\x86\x01a<\xD2V[`@\x83\x01RP``\x83\x81\x015\x90\x82\x01R`\x80\x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aA\xA5W__\xFD[aA\xB16\x82\x86\x01a<\xD2V[`\x80\x83\x01RP\x92\x91PPV[\x80\x82\x01\x80\x82\x11\x15a\x068Wa\x068a;(V[\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83`\xF8\x1B\x16\x81R__\x83TaB\x05\x81a<1V[`\x01\x82\x16\x80\x15aB\x1CW`\x01\x81\x14aBUWaB\x8BV[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x83\x16`\x01\x87\x01R`\x01\x82\x15\x15\x83\x02\x87\x01\x01\x93PaB\x8BV[\x86_R` _ _[\x83\x81\x10\x15aB\x80W\x81T`\x01\x82\x8A\x01\x01R`\x01\x82\x01\x91P` \x81\x01\x90PaB^V[PP`\x01\x82\x87\x01\x01\x93P[P\x91\x96\x95PPPPPPV[_` \x82\x84\x03\x12\x15aB\xA7W__\xFD[\x81Q\x80\x15\x15\x81\x14a\n\xD0W__\xFD[_\x81Q\x80` \x84\x01\x85^_\x93\x01\x92\x83RP\x90\x91\x90PV[\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85\x16\x81R_aC\taC\x03`\x04\x84\x01\x87aB\xB6V[\x85aB\xB6V[\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x93\x90\x93\x16\x83RPP`\x04\x01\x93\x92PPPV[_a\n\xD0\x82\x84aB\xB6V[_` \x82\x84\x03\x12\x15aCWW__\xFD[PQ\x91\x90PV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x068Wa\x068a;(V[_\x82aC\x8CWaC\x8Ca;lV[P\x06\x90V[`\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x068Wa\x068a;(V[`\x01\x81[`\x01\x84\x11\x15aC\xE5W\x80\x85\x04\x81\x11\x15aC\xC9WaC\xC9a;(V[`\x01\x84\x16\x15aC\xD7W\x90\x81\x02\x90[`\x01\x93\x90\x93\x1C\x92\x80\x02aC\xAEV[\x93P\x93\x91PPV[_\x82aC\xFBWP`\x01a\x068V[\x81aD\x07WP_a\x068V[\x81`\x01\x81\x14aD\x1DW`\x02\x81\x14aD'WaDCV[`\x01\x91PPa\x068V[`\xFF\x84\x11\x15aD8WaD8a;(V[PP`\x01\x82\x1Ba\x068V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15aDfWP\x81\x81\na\x068V[aDr_\x19\x84\x84aC\xAAV[\x80_\x19\x04\x82\x11\x15aD\x85WaD\x85a;(V[\x02\x93\x92PPPV[_a\n\xD0\x83\x83aC\xEDV[`\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x068Wa\x068a;(V[` \x81R_a\n\xD0` \x83\x01\x84a5oV\xFE\xA2dipfsX\"\x12 A\xDF\x18\xE5x\xA0A\xC2\xE5-\x8F\x05\xEBc;\xA9l\0\x11;`\x87?x\xCB\xE1J\xFF\x1C\xB1\x7F\x07dsolcC\0\x08\x1C\x003",
    );
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct AcceptedBtcBuyOrder { uint256 orderId; uint256 amountBtc; address ercToken; uint256 ercAmount; address requester; address accepter; uint256 acceptTime; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AcceptedBtcBuyOrder {
        #[allow(missing_docs)]
        pub orderId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub amountBtc: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub ercToken: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub ercAmount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub requester: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub accepter: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub acceptTime: alloy::sol_types::private::primitives::aliases::U256,
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
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::Address,
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
        impl ::core::convert::From<AcceptedBtcBuyOrder> for UnderlyingRustTuple<'_> {
            fn from(value: AcceptedBtcBuyOrder) -> Self {
                (
                    value.orderId,
                    value.amountBtc,
                    value.ercToken,
                    value.ercAmount,
                    value.requester,
                    value.accepter,
                    value.acceptTime,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for AcceptedBtcBuyOrder {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    orderId: tuple.0,
                    amountBtc: tuple.1,
                    ercToken: tuple.2,
                    ercAmount: tuple.3,
                    requester: tuple.4,
                    accepter: tuple.5,
                    acceptTime: tuple.6,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for AcceptedBtcBuyOrder {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for AcceptedBtcBuyOrder {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.orderId),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amountBtc),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.ercToken,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.ercAmount),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.requester,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.accepter,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.acceptTime),
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
        impl alloy_sol_types::SolType for AcceptedBtcBuyOrder {
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
        impl alloy_sol_types::SolStruct for AcceptedBtcBuyOrder {
            const NAME: &'static str = "AcceptedBtcBuyOrder";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "AcceptedBtcBuyOrder(uint256 orderId,uint256 amountBtc,address ercToken,uint256 ercAmount,address requester,address accepter,uint256 acceptTime)",
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
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.orderId)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.amountBtc)
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.ercToken,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.ercAmount)
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.requester,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.accepter,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.acceptTime)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for AcceptedBtcBuyOrder {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.orderId,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.amountBtc,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.ercToken,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.ercAmount,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.requester,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.accepter,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.acceptTime,
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
                    &rust.orderId,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.amountBtc,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.ercToken,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.ercAmount,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.requester,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.accepter,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.acceptTime,
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct AcceptedBtcSellOrder { uint256 orderId; BitcoinAddress bitcoinAddress; uint256 amountBtc; address ercToken; uint256 ercAmount; address requester; address accepter; uint256 acceptTime; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AcceptedBtcSellOrder {
        #[allow(missing_docs)]
        pub orderId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub bitcoinAddress: <BitcoinAddress as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub amountBtc: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub ercToken: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub ercAmount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub requester: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub accepter: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub acceptTime: alloy::sol_types::private::primitives::aliases::U256,
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
            BitcoinAddress,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            <BitcoinAddress as alloy::sol_types::SolType>::RustType,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::Address,
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
        impl ::core::convert::From<AcceptedBtcSellOrder> for UnderlyingRustTuple<'_> {
            fn from(value: AcceptedBtcSellOrder) -> Self {
                (
                    value.orderId,
                    value.bitcoinAddress,
                    value.amountBtc,
                    value.ercToken,
                    value.ercAmount,
                    value.requester,
                    value.accepter,
                    value.acceptTime,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for AcceptedBtcSellOrder {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    orderId: tuple.0,
                    bitcoinAddress: tuple.1,
                    amountBtc: tuple.2,
                    ercToken: tuple.3,
                    ercAmount: tuple.4,
                    requester: tuple.5,
                    accepter: tuple.6,
                    acceptTime: tuple.7,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for AcceptedBtcSellOrder {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for AcceptedBtcSellOrder {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.orderId),
                    <BitcoinAddress as alloy_sol_types::SolType>::tokenize(
                        &self.bitcoinAddress,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amountBtc),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.ercToken,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.ercAmount),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.requester,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.accepter,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.acceptTime),
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
        impl alloy_sol_types::SolType for AcceptedBtcSellOrder {
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
        impl alloy_sol_types::SolStruct for AcceptedBtcSellOrder {
            const NAME: &'static str = "AcceptedBtcSellOrder";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "AcceptedBtcSellOrder(uint256 orderId,BitcoinAddress bitcoinAddress,uint256 amountBtc,address ercToken,uint256 ercAmount,address requester,address accepter,uint256 acceptTime)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(1);
                components
                    .push(
                        <BitcoinAddress as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <BitcoinAddress as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.orderId)
                        .0,
                    <BitcoinAddress as alloy_sol_types::SolType>::eip712_data_word(
                            &self.bitcoinAddress,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.amountBtc)
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.ercToken,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.ercAmount)
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.requester,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.accepter,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.acceptTime)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for AcceptedBtcSellOrder {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.orderId,
                    )
                    + <BitcoinAddress as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.bitcoinAddress,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.amountBtc,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.ercToken,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.ercAmount,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.requester,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.accepter,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.acceptTime,
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
                    &rust.orderId,
                    out,
                );
                <BitcoinAddress as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.bitcoinAddress,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.amountBtc,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.ercToken,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.ercAmount,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.requester,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.accepter,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.acceptTime,
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct BitcoinAddress { bytes scriptPubKey; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BitcoinAddress {
        #[allow(missing_docs)]
        pub scriptPubKey: alloy::sol_types::private::Bytes,
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
        impl ::core::convert::From<BitcoinAddress> for UnderlyingRustTuple<'_> {
            fn from(value: BitcoinAddress) -> Self {
                (value.scriptPubKey,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for BitcoinAddress {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { scriptPubKey: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for BitcoinAddress {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for BitcoinAddress {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.scriptPubKey,
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
        impl alloy_sol_types::SolType for BitcoinAddress {
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
        impl alloy_sol_types::SolStruct for BitcoinAddress {
            const NAME: &'static str = "BitcoinAddress";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "BitcoinAddress(bytes scriptPubKey)",
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
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                        &self.scriptPubKey,
                    )
                    .0
                    .to_vec()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for BitcoinAddress {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.scriptPubKey,
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
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.scriptPubKey,
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct BtcBuyOrder { uint256 amountBtc; BitcoinAddress bitcoinAddress; address offeringToken; uint256 offeringAmount; address requester; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BtcBuyOrder {
        #[allow(missing_docs)]
        pub amountBtc: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub bitcoinAddress: <BitcoinAddress as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub offeringToken: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub offeringAmount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub requester: alloy::sol_types::private::Address,
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
            BitcoinAddress,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Address,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            <BitcoinAddress as alloy::sol_types::SolType>::RustType,
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
        impl ::core::convert::From<BtcBuyOrder> for UnderlyingRustTuple<'_> {
            fn from(value: BtcBuyOrder) -> Self {
                (
                    value.amountBtc,
                    value.bitcoinAddress,
                    value.offeringToken,
                    value.offeringAmount,
                    value.requester,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for BtcBuyOrder {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    amountBtc: tuple.0,
                    bitcoinAddress: tuple.1,
                    offeringToken: tuple.2,
                    offeringAmount: tuple.3,
                    requester: tuple.4,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for BtcBuyOrder {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for BtcBuyOrder {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amountBtc),
                    <BitcoinAddress as alloy_sol_types::SolType>::tokenize(
                        &self.bitcoinAddress,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.offeringToken,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.offeringAmount),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.requester,
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
        impl alloy_sol_types::SolType for BtcBuyOrder {
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
        impl alloy_sol_types::SolStruct for BtcBuyOrder {
            const NAME: &'static str = "BtcBuyOrder";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "BtcBuyOrder(uint256 amountBtc,BitcoinAddress bitcoinAddress,address offeringToken,uint256 offeringAmount,address requester)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(1);
                components
                    .push(
                        <BitcoinAddress as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <BitcoinAddress as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.amountBtc)
                        .0,
                    <BitcoinAddress as alloy_sol_types::SolType>::eip712_data_word(
                            &self.bitcoinAddress,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.offeringToken,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.offeringAmount,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.requester,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for BtcBuyOrder {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.amountBtc,
                    )
                    + <BitcoinAddress as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.bitcoinAddress,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.offeringToken,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.offeringAmount,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.requester,
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
                    &rust.amountBtc,
                    out,
                );
                <BitcoinAddress as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.bitcoinAddress,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.offeringToken,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.offeringAmount,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.requester,
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct BtcSellOrder { uint256 amountBtc; address askingToken; uint256 askingAmount; address requester; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BtcSellOrder {
        #[allow(missing_docs)]
        pub amountBtc: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub askingToken: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub askingAmount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub requester: alloy::sol_types::private::Address,
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
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Address,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<BtcSellOrder> for UnderlyingRustTuple<'_> {
            fn from(value: BtcSellOrder) -> Self {
                (value.amountBtc, value.askingToken, value.askingAmount, value.requester)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for BtcSellOrder {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    amountBtc: tuple.0,
                    askingToken: tuple.1,
                    askingAmount: tuple.2,
                    requester: tuple.3,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for BtcSellOrder {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for BtcSellOrder {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amountBtc),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.askingToken,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.askingAmount),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.requester,
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
        impl alloy_sol_types::SolType for BtcSellOrder {
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
        impl alloy_sol_types::SolStruct for BtcSellOrder {
            const NAME: &'static str = "BtcSellOrder";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "BtcSellOrder(uint256 amountBtc,address askingToken,uint256 askingAmount,address requester)",
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
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.amountBtc)
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.askingToken,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.askingAmount)
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.requester,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for BtcSellOrder {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.amountBtc,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.askingToken,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.askingAmount,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.requester,
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
                    &rust.amountBtc,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.askingToken,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.askingAmount,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.requester,
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `acceptBtcBuyOrderEvent(uint256,uint256,uint256,uint256,address)` and selector `0xc39a1a5ddc0e85c955fe2e1abeb43c94ce18322e75bb3d44e80f759ff9d034b9`.
```solidity
event acceptBtcBuyOrderEvent(uint256 indexed orderId, uint256 indexed acceptId, uint256 amountBtc, uint256 ercAmount, address ercToken);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct acceptBtcBuyOrderEvent {
        #[allow(missing_docs)]
        pub orderId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub acceptId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub amountBtc: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub ercAmount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub ercToken: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for acceptBtcBuyOrderEvent {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "acceptBtcBuyOrderEvent(uint256,uint256,uint256,uint256,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                195u8, 154u8, 26u8, 93u8, 220u8, 14u8, 133u8, 201u8, 85u8, 254u8, 46u8,
                26u8, 190u8, 180u8, 60u8, 148u8, 206u8, 24u8, 50u8, 46u8, 117u8, 187u8,
                61u8, 68u8, 232u8, 15u8, 117u8, 159u8, 249u8, 208u8, 52u8, 185u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    orderId: topics.1,
                    acceptId: topics.2,
                    amountBtc: data.0,
                    ercAmount: data.1,
                    ercToken: data.2,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.amountBtc),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.ercAmount),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.ercToken,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.orderId.clone(),
                    self.acceptId.clone(),
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
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.orderId);
                out[2usize] = <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.acceptId);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for acceptBtcBuyOrderEvent {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&acceptBtcBuyOrderEvent> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &acceptBtcBuyOrderEvent) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `acceptBtcSellOrderEvent(uint256,uint256,(bytes),uint256,uint256,address)` and selector `0x653e0d81f2c99beba359dfb17b499a5cff2be9d950514852224df8c097c21921`.
```solidity
event acceptBtcSellOrderEvent(uint256 indexed id, uint256 indexed acceptId, BitcoinAddress bitcoinAddress, uint256 amountBtc, uint256 ercAmount, address ercToken);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct acceptBtcSellOrderEvent {
        #[allow(missing_docs)]
        pub id: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub acceptId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub bitcoinAddress: <BitcoinAddress as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub amountBtc: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub ercAmount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub ercToken: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for acceptBtcSellOrderEvent {
            type DataTuple<'a> = (
                BitcoinAddress,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "acceptBtcSellOrderEvent(uint256,uint256,(bytes),uint256,uint256,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                101u8, 62u8, 13u8, 129u8, 242u8, 201u8, 155u8, 235u8, 163u8, 89u8, 223u8,
                177u8, 123u8, 73u8, 154u8, 92u8, 255u8, 43u8, 233u8, 217u8, 80u8, 81u8,
                72u8, 82u8, 34u8, 77u8, 248u8, 192u8, 151u8, 194u8, 25u8, 33u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    id: topics.1,
                    acceptId: topics.2,
                    bitcoinAddress: data.0,
                    amountBtc: data.1,
                    ercAmount: data.2,
                    ercToken: data.3,
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
                    <BitcoinAddress as alloy_sol_types::SolType>::tokenize(
                        &self.bitcoinAddress,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amountBtc),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.ercAmount),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.ercToken,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.id.clone(), self.acceptId.clone())
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
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.id);
                out[2usize] = <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.acceptId);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for acceptBtcSellOrderEvent {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&acceptBtcSellOrderEvent> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &acceptBtcSellOrderEvent,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `cancelAcceptedBtcBuyOrderEvent(uint256)` and selector `0x3e5ea358e9eb4cdf44cdc77938ade8074b1240a6d8c0fd13728671b82e800ad6`.
```solidity
event cancelAcceptedBtcBuyOrderEvent(uint256 id);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct cancelAcceptedBtcBuyOrderEvent {
        #[allow(missing_docs)]
        pub id: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for cancelAcceptedBtcBuyOrderEvent {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "cancelAcceptedBtcBuyOrderEvent(uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                62u8, 94u8, 163u8, 88u8, 233u8, 235u8, 76u8, 223u8, 68u8, 205u8, 199u8,
                121u8, 56u8, 173u8, 232u8, 7u8, 75u8, 18u8, 64u8, 166u8, 216u8, 192u8,
                253u8, 19u8, 114u8, 134u8, 113u8, 184u8, 46u8, 128u8, 10u8, 214u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { id: data.0 }
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
                    > as alloy_sol_types::SolType>::tokenize(&self.id),
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
        impl alloy_sol_types::private::IntoLogData for cancelAcceptedBtcBuyOrderEvent {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&cancelAcceptedBtcBuyOrderEvent>
        for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &cancelAcceptedBtcBuyOrderEvent,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `cancelAcceptedBtcSellOrderEvent(uint256)` and selector `0x78f51f62f7cf1381c673c27eae187dd6c588dc6624ce59697dbb3e1d7c1bbcdf`.
```solidity
event cancelAcceptedBtcSellOrderEvent(uint256 id);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct cancelAcceptedBtcSellOrderEvent {
        #[allow(missing_docs)]
        pub id: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for cancelAcceptedBtcSellOrderEvent {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "cancelAcceptedBtcSellOrderEvent(uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                120u8, 245u8, 31u8, 98u8, 247u8, 207u8, 19u8, 129u8, 198u8, 115u8, 194u8,
                126u8, 174u8, 24u8, 125u8, 214u8, 197u8, 136u8, 220u8, 102u8, 36u8,
                206u8, 89u8, 105u8, 125u8, 187u8, 62u8, 29u8, 124u8, 27u8, 188u8, 223u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { id: data.0 }
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
                    > as alloy_sol_types::SolType>::tokenize(&self.id),
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
        impl alloy_sol_types::private::IntoLogData for cancelAcceptedBtcSellOrderEvent {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&cancelAcceptedBtcSellOrderEvent>
        for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &cancelAcceptedBtcSellOrderEvent,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `placeBtcBuyOrderEvent(uint256,(bytes),address,uint256)` and selector `0x98c7c680403d47403dea4a570d0e6c5716538c49420ef471cec428f5a5852c06`.
```solidity
event placeBtcBuyOrderEvent(uint256 amountBtc, BitcoinAddress bitcoinAddress, address sellingToken, uint256 saleAmount);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct placeBtcBuyOrderEvent {
        #[allow(missing_docs)]
        pub amountBtc: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub bitcoinAddress: <BitcoinAddress as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub sellingToken: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub saleAmount: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for placeBtcBuyOrderEvent {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                BitcoinAddress,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "placeBtcBuyOrderEvent(uint256,(bytes),address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                152u8, 199u8, 198u8, 128u8, 64u8, 61u8, 71u8, 64u8, 61u8, 234u8, 74u8,
                87u8, 13u8, 14u8, 108u8, 87u8, 22u8, 83u8, 140u8, 73u8, 66u8, 14u8,
                244u8, 113u8, 206u8, 196u8, 40u8, 245u8, 165u8, 133u8, 44u8, 6u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    amountBtc: data.0,
                    bitcoinAddress: data.1,
                    sellingToken: data.2,
                    saleAmount: data.3,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.amountBtc),
                    <BitcoinAddress as alloy_sol_types::SolType>::tokenize(
                        &self.bitcoinAddress,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.sellingToken,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.saleAmount),
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
        impl alloy_sol_types::private::IntoLogData for placeBtcBuyOrderEvent {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&placeBtcBuyOrderEvent> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &placeBtcBuyOrderEvent) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `placeBtcSellOrderEvent(uint256,uint256,address,uint256)` and selector `0xff1ce210defcd3ba1adf76c9419a0758fa60fd3eb38c7bd9418f60b575b76e24`.
```solidity
event placeBtcSellOrderEvent(uint256 indexed orderId, uint256 amountBtc, address buyingToken, uint256 buyAmount);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct placeBtcSellOrderEvent {
        #[allow(missing_docs)]
        pub orderId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub amountBtc: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub buyingToken: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub buyAmount: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for placeBtcSellOrderEvent {
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
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "placeBtcSellOrderEvent(uint256,uint256,address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                255u8, 28u8, 226u8, 16u8, 222u8, 252u8, 211u8, 186u8, 26u8, 223u8, 118u8,
                201u8, 65u8, 154u8, 7u8, 88u8, 250u8, 96u8, 253u8, 62u8, 179u8, 140u8,
                123u8, 217u8, 65u8, 143u8, 96u8, 181u8, 117u8, 183u8, 110u8, 36u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    orderId: topics.1,
                    amountBtc: data.0,
                    buyingToken: data.1,
                    buyAmount: data.2,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.amountBtc),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.buyingToken,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.buyAmount),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.orderId.clone())
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
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.orderId);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for placeBtcSellOrderEvent {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&placeBtcSellOrderEvent> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &placeBtcSellOrderEvent) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `proofBtcBuyOrderEvent(uint256)` and selector `0xb4c98de210696b3cf21e99335c1ee3a0ae34a26713412a4adde8af596176f37e`.
```solidity
event proofBtcBuyOrderEvent(uint256 id);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct proofBtcBuyOrderEvent {
        #[allow(missing_docs)]
        pub id: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for proofBtcBuyOrderEvent {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "proofBtcBuyOrderEvent(uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                180u8, 201u8, 141u8, 226u8, 16u8, 105u8, 107u8, 60u8, 242u8, 30u8, 153u8,
                51u8, 92u8, 30u8, 227u8, 160u8, 174u8, 52u8, 162u8, 103u8, 19u8, 65u8,
                42u8, 74u8, 221u8, 232u8, 175u8, 89u8, 97u8, 118u8, 243u8, 126u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { id: data.0 }
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
                    > as alloy_sol_types::SolType>::tokenize(&self.id),
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
        impl alloy_sol_types::private::IntoLogData for proofBtcBuyOrderEvent {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&proofBtcBuyOrderEvent> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &proofBtcBuyOrderEvent) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `proofBtcSellOrderEvent(uint256)` and selector `0xcf561061db78f7bc518d37fe86718514c640ccc5c3f1293828b955e68f19f5fb`.
```solidity
event proofBtcSellOrderEvent(uint256 id);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct proofBtcSellOrderEvent {
        #[allow(missing_docs)]
        pub id: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for proofBtcSellOrderEvent {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "proofBtcSellOrderEvent(uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                207u8, 86u8, 16u8, 97u8, 219u8, 120u8, 247u8, 188u8, 81u8, 141u8, 55u8,
                254u8, 134u8, 113u8, 133u8, 20u8, 198u8, 64u8, 204u8, 197u8, 195u8,
                241u8, 41u8, 56u8, 40u8, 185u8, 85u8, 230u8, 143u8, 25u8, 245u8, 251u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { id: data.0 }
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
                    > as alloy_sol_types::SolType>::tokenize(&self.id),
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
        impl alloy_sol_types::private::IntoLogData for proofBtcSellOrderEvent {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&proofBtcSellOrderEvent> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &proofBtcSellOrderEvent) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `withdrawBtcBuyOrderEvent(uint256)` and selector `0xc340e7ac48dc80ee793fc6266960bd5f1bd21be91c8a95e218178113f79e17b4`.
```solidity
event withdrawBtcBuyOrderEvent(uint256 id);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct withdrawBtcBuyOrderEvent {
        #[allow(missing_docs)]
        pub id: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for withdrawBtcBuyOrderEvent {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "withdrawBtcBuyOrderEvent(uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                195u8, 64u8, 231u8, 172u8, 72u8, 220u8, 128u8, 238u8, 121u8, 63u8, 198u8,
                38u8, 105u8, 96u8, 189u8, 95u8, 27u8, 210u8, 27u8, 233u8, 28u8, 138u8,
                149u8, 226u8, 24u8, 23u8, 129u8, 19u8, 247u8, 158u8, 23u8, 180u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { id: data.0 }
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
                    > as alloy_sol_types::SolType>::tokenize(&self.id),
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
        impl alloy_sol_types::private::IntoLogData for withdrawBtcBuyOrderEvent {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&withdrawBtcBuyOrderEvent> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &withdrawBtcBuyOrderEvent,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `withdrawBtcSellOrderEvent(uint256)` and selector `0x3cd475b092e8b379f6ba0d9e0e0c8f30705e73321dc5c9f80ce4ad38db7be1aa`.
```solidity
event withdrawBtcSellOrderEvent(uint256 id);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct withdrawBtcSellOrderEvent {
        #[allow(missing_docs)]
        pub id: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for withdrawBtcSellOrderEvent {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "withdrawBtcSellOrderEvent(uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                60u8, 212u8, 117u8, 176u8, 146u8, 232u8, 179u8, 121u8, 246u8, 186u8,
                13u8, 158u8, 14u8, 12u8, 143u8, 48u8, 112u8, 94u8, 115u8, 50u8, 29u8,
                197u8, 201u8, 248u8, 12u8, 228u8, 173u8, 56u8, 219u8, 123u8, 225u8, 170u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { id: data.0 }
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
                    > as alloy_sol_types::SolType>::tokenize(&self.id),
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
        impl alloy_sol_types::private::IntoLogData for withdrawBtcSellOrderEvent {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&withdrawBtcSellOrderEvent> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &withdrawBtcSellOrderEvent,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Constructor`.
```solidity
constructor(address _relay, address erc2771Forwarder);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        #[allow(missing_docs)]
        pub _relay: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub erc2771Forwarder: alloy::sol_types::private::Address,
    }
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
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    (value._relay, value.erc2771Forwarder)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _relay: tuple.0,
                        erc2771Forwarder: tuple.1,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
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
                        &self._relay,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.erc2771Forwarder,
                    ),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `REQUEST_EXPIRATION_SECONDS()` and selector `0xd1920ff0`.
```solidity
function REQUEST_EXPIRATION_SECONDS() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct REQUEST_EXPIRATION_SECONDSCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`REQUEST_EXPIRATION_SECONDS()`](REQUEST_EXPIRATION_SECONDSCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct REQUEST_EXPIRATION_SECONDSReturn {
        #[allow(missing_docs)]
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
            impl ::core::convert::From<REQUEST_EXPIRATION_SECONDSCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: REQUEST_EXPIRATION_SECONDSCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for REQUEST_EXPIRATION_SECONDSCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            impl ::core::convert::From<REQUEST_EXPIRATION_SECONDSReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: REQUEST_EXPIRATION_SECONDSReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for REQUEST_EXPIRATION_SECONDSReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for REQUEST_EXPIRATION_SECONDSCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "REQUEST_EXPIRATION_SECONDS()";
            const SELECTOR: [u8; 4] = [209u8, 146u8, 15u8, 240u8];
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: REQUEST_EXPIRATION_SECONDSReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: REQUEST_EXPIRATION_SECONDSReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `acceptBtcBuyOrder(uint256,uint256)` and selector `0x11c137aa`.
```solidity
function acceptBtcBuyOrder(uint256 id, uint256 amountBtc) external returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct acceptBtcBuyOrderCall {
        #[allow(missing_docs)]
        pub id: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub amountBtc: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`acceptBtcBuyOrder(uint256,uint256)`](acceptBtcBuyOrderCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct acceptBtcBuyOrderReturn {
        #[allow(missing_docs)]
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
            impl ::core::convert::From<acceptBtcBuyOrderCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: acceptBtcBuyOrderCall) -> Self {
                    (value.id, value.amountBtc)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for acceptBtcBuyOrderCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        id: tuple.0,
                        amountBtc: tuple.1,
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
            impl ::core::convert::From<acceptBtcBuyOrderReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: acceptBtcBuyOrderReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for acceptBtcBuyOrderReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for acceptBtcBuyOrderCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "acceptBtcBuyOrder(uint256,uint256)";
            const SELECTOR: [u8; 4] = [17u8, 193u8, 55u8, 170u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.id),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amountBtc),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: acceptBtcBuyOrderReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: acceptBtcBuyOrderReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `acceptBtcSellOrder(uint256,(bytes),uint256)` and selector `0x210ec181`.
```solidity
function acceptBtcSellOrder(uint256 id, BitcoinAddress memory bitcoinAddress, uint256 amountBtc) external returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct acceptBtcSellOrderCall {
        #[allow(missing_docs)]
        pub id: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub bitcoinAddress: <BitcoinAddress as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub amountBtc: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`acceptBtcSellOrder(uint256,(bytes),uint256)`](acceptBtcSellOrderCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct acceptBtcSellOrderReturn {
        #[allow(missing_docs)]
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
                BitcoinAddress,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                <BitcoinAddress as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<acceptBtcSellOrderCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: acceptBtcSellOrderCall) -> Self {
                    (value.id, value.bitcoinAddress, value.amountBtc)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for acceptBtcSellOrderCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        id: tuple.0,
                        bitcoinAddress: tuple.1,
                        amountBtc: tuple.2,
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
            impl ::core::convert::From<acceptBtcSellOrderReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: acceptBtcSellOrderReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for acceptBtcSellOrderReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for acceptBtcSellOrderCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                BitcoinAddress,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "acceptBtcSellOrder(uint256,(bytes),uint256)";
            const SELECTOR: [u8; 4] = [33u8, 14u8, 193u8, 129u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.id),
                    <BitcoinAddress as alloy_sol_types::SolType>::tokenize(
                        &self.bitcoinAddress,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amountBtc),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: acceptBtcSellOrderReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: acceptBtcSellOrderReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `acceptedBtcBuyOrders(uint256)` and selector `0xbd2a7e3e`.
```solidity
function acceptedBtcBuyOrders(uint256) external view returns (uint256 orderId, uint256 amountBtc, address ercToken, uint256 ercAmount, address requester, address accepter, uint256 acceptTime);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct acceptedBtcBuyOrdersCall(
        pub alloy::sol_types::private::primitives::aliases::U256,
    );
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`acceptedBtcBuyOrders(uint256)`](acceptedBtcBuyOrdersCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct acceptedBtcBuyOrdersReturn {
        #[allow(missing_docs)]
        pub orderId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub amountBtc: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub ercToken: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub ercAmount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub requester: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub accepter: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub acceptTime: alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<acceptedBtcBuyOrdersCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: acceptedBtcBuyOrdersCall) -> Self {
                    (value.0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for acceptedBtcBuyOrdersCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self(tuple.0)
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<acceptedBtcBuyOrdersReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: acceptedBtcBuyOrdersReturn) -> Self {
                    (
                        value.orderId,
                        value.amountBtc,
                        value.ercToken,
                        value.ercAmount,
                        value.requester,
                        value.accepter,
                        value.acceptTime,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for acceptedBtcBuyOrdersReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        orderId: tuple.0,
                        amountBtc: tuple.1,
                        ercToken: tuple.2,
                        ercAmount: tuple.3,
                        requester: tuple.4,
                        accepter: tuple.5,
                        acceptTime: tuple.6,
                    }
                }
            }
        }
        impl acceptedBtcBuyOrdersReturn {
            fn _tokenize(
                &self,
            ) -> <acceptedBtcBuyOrdersCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.orderId),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amountBtc),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.ercToken,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.ercAmount),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.requester,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.accepter,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.acceptTime),
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for acceptedBtcBuyOrdersCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = acceptedBtcBuyOrdersReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "acceptedBtcBuyOrders(uint256)";
            const SELECTOR: [u8; 4] = [189u8, 42u8, 126u8, 62u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.0),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                acceptedBtcBuyOrdersReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `acceptedBtcSellOrders(uint256)` and selector `0x4145640a`.
```solidity
function acceptedBtcSellOrders(uint256) external view returns (uint256 orderId, BitcoinAddress memory bitcoinAddress, uint256 amountBtc, address ercToken, uint256 ercAmount, address requester, address accepter, uint256 acceptTime);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct acceptedBtcSellOrdersCall(
        pub alloy::sol_types::private::primitives::aliases::U256,
    );
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`acceptedBtcSellOrders(uint256)`](acceptedBtcSellOrdersCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct acceptedBtcSellOrdersReturn {
        #[allow(missing_docs)]
        pub orderId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub bitcoinAddress: <BitcoinAddress as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub amountBtc: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub ercToken: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub ercAmount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub requester: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub accepter: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub acceptTime: alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<acceptedBtcSellOrdersCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: acceptedBtcSellOrdersCall) -> Self {
                    (value.0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for acceptedBtcSellOrdersCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self(tuple.0)
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                BitcoinAddress,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                <BitcoinAddress as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<acceptedBtcSellOrdersReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: acceptedBtcSellOrdersReturn) -> Self {
                    (
                        value.orderId,
                        value.bitcoinAddress,
                        value.amountBtc,
                        value.ercToken,
                        value.ercAmount,
                        value.requester,
                        value.accepter,
                        value.acceptTime,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for acceptedBtcSellOrdersReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        orderId: tuple.0,
                        bitcoinAddress: tuple.1,
                        amountBtc: tuple.2,
                        ercToken: tuple.3,
                        ercAmount: tuple.4,
                        requester: tuple.5,
                        accepter: tuple.6,
                        acceptTime: tuple.7,
                    }
                }
            }
        }
        impl acceptedBtcSellOrdersReturn {
            fn _tokenize(
                &self,
            ) -> <acceptedBtcSellOrdersCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.orderId),
                    <BitcoinAddress as alloy_sol_types::SolType>::tokenize(
                        &self.bitcoinAddress,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amountBtc),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.ercToken,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.ercAmount),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.requester,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.accepter,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.acceptTime),
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for acceptedBtcSellOrdersCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = acceptedBtcSellOrdersReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                BitcoinAddress,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "acceptedBtcSellOrders(uint256)";
            const SELECTOR: [u8; 4] = [65u8, 69u8, 100u8, 10u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.0),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                acceptedBtcSellOrdersReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `btcBuyOrders(uint256)` and selector `0x3af3fc7e`.
```solidity
function btcBuyOrders(uint256) external view returns (uint256 amountBtc, BitcoinAddress memory bitcoinAddress, address offeringToken, uint256 offeringAmount, address requester);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct btcBuyOrdersCall(
        pub alloy::sol_types::private::primitives::aliases::U256,
    );
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`btcBuyOrders(uint256)`](btcBuyOrdersCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct btcBuyOrdersReturn {
        #[allow(missing_docs)]
        pub amountBtc: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub bitcoinAddress: <BitcoinAddress as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub offeringToken: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub offeringAmount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub requester: alloy::sol_types::private::Address,
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
            impl ::core::convert::From<btcBuyOrdersCall> for UnderlyingRustTuple<'_> {
                fn from(value: btcBuyOrdersCall) -> Self {
                    (value.0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for btcBuyOrdersCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self(tuple.0)
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                BitcoinAddress,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                <BitcoinAddress as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<btcBuyOrdersReturn> for UnderlyingRustTuple<'_> {
                fn from(value: btcBuyOrdersReturn) -> Self {
                    (
                        value.amountBtc,
                        value.bitcoinAddress,
                        value.offeringToken,
                        value.offeringAmount,
                        value.requester,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for btcBuyOrdersReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        amountBtc: tuple.0,
                        bitcoinAddress: tuple.1,
                        offeringToken: tuple.2,
                        offeringAmount: tuple.3,
                        requester: tuple.4,
                    }
                }
            }
        }
        impl btcBuyOrdersReturn {
            fn _tokenize(
                &self,
            ) -> <btcBuyOrdersCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amountBtc),
                    <BitcoinAddress as alloy_sol_types::SolType>::tokenize(
                        &self.bitcoinAddress,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.offeringToken,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.offeringAmount),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.requester,
                    ),
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for btcBuyOrdersCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = btcBuyOrdersReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                BitcoinAddress,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "btcBuyOrders(uint256)";
            const SELECTOR: [u8; 4] = [58u8, 243u8, 252u8, 126u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.0),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                btcBuyOrdersReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `btcSellOrders(uint256)` and selector `0xecca2c36`.
```solidity
function btcSellOrders(uint256) external view returns (uint256 amountBtc, address askingToken, uint256 askingAmount, address requester);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct btcSellOrdersCall(
        pub alloy::sol_types::private::primitives::aliases::U256,
    );
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`btcSellOrders(uint256)`](btcSellOrdersCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct btcSellOrdersReturn {
        #[allow(missing_docs)]
        pub amountBtc: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub askingToken: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub askingAmount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub requester: alloy::sol_types::private::Address,
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
            impl ::core::convert::From<btcSellOrdersCall> for UnderlyingRustTuple<'_> {
                fn from(value: btcSellOrdersCall) -> Self {
                    (value.0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for btcSellOrdersCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self(tuple.0)
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<btcSellOrdersReturn> for UnderlyingRustTuple<'_> {
                fn from(value: btcSellOrdersReturn) -> Self {
                    (
                        value.amountBtc,
                        value.askingToken,
                        value.askingAmount,
                        value.requester,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for btcSellOrdersReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        amountBtc: tuple.0,
                        askingToken: tuple.1,
                        askingAmount: tuple.2,
                        requester: tuple.3,
                    }
                }
            }
        }
        impl btcSellOrdersReturn {
            fn _tokenize(
                &self,
            ) -> <btcSellOrdersCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amountBtc),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.askingToken,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.askingAmount),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.requester,
                    ),
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for btcSellOrdersCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = btcSellOrdersReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "btcSellOrders(uint256)";
            const SELECTOR: [u8; 4] = [236u8, 202u8, 44u8, 54u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.0),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                btcSellOrdersReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `cancelAcceptedBtcBuyOrder(uint256)` and selector `0xc56a4526`.
```solidity
function cancelAcceptedBtcBuyOrder(uint256 id) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct cancelAcceptedBtcBuyOrderCall {
        #[allow(missing_docs)]
        pub id: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`cancelAcceptedBtcBuyOrder(uint256)`](cancelAcceptedBtcBuyOrderCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct cancelAcceptedBtcBuyOrderReturn {}
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
            impl ::core::convert::From<cancelAcceptedBtcBuyOrderCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: cancelAcceptedBtcBuyOrderCall) -> Self {
                    (value.id,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for cancelAcceptedBtcBuyOrderCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { id: tuple.0 }
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
            impl ::core::convert::From<cancelAcceptedBtcBuyOrderReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: cancelAcceptedBtcBuyOrderReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for cancelAcceptedBtcBuyOrderReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl cancelAcceptedBtcBuyOrderReturn {
            fn _tokenize(
                &self,
            ) -> <cancelAcceptedBtcBuyOrderCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for cancelAcceptedBtcBuyOrderCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = cancelAcceptedBtcBuyOrderReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "cancelAcceptedBtcBuyOrder(uint256)";
            const SELECTOR: [u8; 4] = [197u8, 106u8, 69u8, 38u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.id),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                cancelAcceptedBtcBuyOrderReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `cancelAcceptedBtcSellOrder(uint256)` and selector `0xdf69b14f`.
```solidity
function cancelAcceptedBtcSellOrder(uint256 id) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct cancelAcceptedBtcSellOrderCall {
        #[allow(missing_docs)]
        pub id: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`cancelAcceptedBtcSellOrder(uint256)`](cancelAcceptedBtcSellOrderCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct cancelAcceptedBtcSellOrderReturn {}
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
            impl ::core::convert::From<cancelAcceptedBtcSellOrderCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: cancelAcceptedBtcSellOrderCall) -> Self {
                    (value.id,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for cancelAcceptedBtcSellOrderCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { id: tuple.0 }
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
            impl ::core::convert::From<cancelAcceptedBtcSellOrderReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: cancelAcceptedBtcSellOrderReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for cancelAcceptedBtcSellOrderReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl cancelAcceptedBtcSellOrderReturn {
            fn _tokenize(
                &self,
            ) -> <cancelAcceptedBtcSellOrderCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for cancelAcceptedBtcSellOrderCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = cancelAcceptedBtcSellOrderReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "cancelAcceptedBtcSellOrder(uint256)";
            const SELECTOR: [u8; 4] = [223u8, 105u8, 177u8, 79u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.id),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                cancelAcceptedBtcSellOrderReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getOpenAcceptedBtcBuyOrders()` and selector `0x6a8cde3a`.
```solidity
function getOpenAcceptedBtcBuyOrders() external view returns (AcceptedBtcBuyOrder[] memory, uint256[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOpenAcceptedBtcBuyOrdersCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getOpenAcceptedBtcBuyOrders()`](getOpenAcceptedBtcBuyOrdersCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOpenAcceptedBtcBuyOrdersReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Vec<
            <AcceptedBtcBuyOrder as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub _1: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
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
            impl ::core::convert::From<getOpenAcceptedBtcBuyOrdersCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOpenAcceptedBtcBuyOrdersCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOpenAcceptedBtcBuyOrdersCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<AcceptedBtcBuyOrder>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <AcceptedBtcBuyOrder as alloy::sol_types::SolType>::RustType,
                >,
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
            impl ::core::convert::From<getOpenAcceptedBtcBuyOrdersReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOpenAcceptedBtcBuyOrdersReturn) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOpenAcceptedBtcBuyOrdersReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0, _1: tuple.1 }
                }
            }
        }
        impl getOpenAcceptedBtcBuyOrdersReturn {
            fn _tokenize(
                &self,
            ) -> <getOpenAcceptedBtcBuyOrdersCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                (
                    <alloy::sol_types::sol_data::Array<
                        AcceptedBtcBuyOrder,
                    > as alloy_sol_types::SolType>::tokenize(&self._0),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self._1),
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOpenAcceptedBtcBuyOrdersCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOpenAcceptedBtcBuyOrdersReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<AcceptedBtcBuyOrder>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOpenAcceptedBtcBuyOrders()";
            const SELECTOR: [u8; 4] = [106u8, 140u8, 222u8, 58u8];
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                getOpenAcceptedBtcBuyOrdersReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getOpenAcceptedBtcSellOrders()` and selector `0x9cc6722e`.
```solidity
function getOpenAcceptedBtcSellOrders() external view returns (AcceptedBtcSellOrder[] memory, uint256[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOpenAcceptedBtcSellOrdersCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getOpenAcceptedBtcSellOrders()`](getOpenAcceptedBtcSellOrdersCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOpenAcceptedBtcSellOrdersReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Vec<
            <AcceptedBtcSellOrder as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub _1: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
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
            impl ::core::convert::From<getOpenAcceptedBtcSellOrdersCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOpenAcceptedBtcSellOrdersCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOpenAcceptedBtcSellOrdersCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<AcceptedBtcSellOrder>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <AcceptedBtcSellOrder as alloy::sol_types::SolType>::RustType,
                >,
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
            impl ::core::convert::From<getOpenAcceptedBtcSellOrdersReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOpenAcceptedBtcSellOrdersReturn) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOpenAcceptedBtcSellOrdersReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0, _1: tuple.1 }
                }
            }
        }
        impl getOpenAcceptedBtcSellOrdersReturn {
            fn _tokenize(
                &self,
            ) -> <getOpenAcceptedBtcSellOrdersCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                (
                    <alloy::sol_types::sol_data::Array<
                        AcceptedBtcSellOrder,
                    > as alloy_sol_types::SolType>::tokenize(&self._0),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self._1),
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOpenAcceptedBtcSellOrdersCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOpenAcceptedBtcSellOrdersReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<AcceptedBtcSellOrder>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOpenAcceptedBtcSellOrders()";
            const SELECTOR: [u8; 4] = [156u8, 198u8, 114u8, 46u8];
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                getOpenAcceptedBtcSellOrdersReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getOpenBtcBuyOrders()` and selector `0x1dfe7595`.
```solidity
function getOpenBtcBuyOrders() external view returns (BtcBuyOrder[] memory, uint256[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOpenBtcBuyOrdersCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getOpenBtcBuyOrders()`](getOpenBtcBuyOrdersCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOpenBtcBuyOrdersReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Vec<
            <BtcBuyOrder as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub _1: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
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
            impl ::core::convert::From<getOpenBtcBuyOrdersCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOpenBtcBuyOrdersCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOpenBtcBuyOrdersCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<BtcBuyOrder>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <BtcBuyOrder as alloy::sol_types::SolType>::RustType,
                >,
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
            impl ::core::convert::From<getOpenBtcBuyOrdersReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOpenBtcBuyOrdersReturn) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOpenBtcBuyOrdersReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0, _1: tuple.1 }
                }
            }
        }
        impl getOpenBtcBuyOrdersReturn {
            fn _tokenize(
                &self,
            ) -> <getOpenBtcBuyOrdersCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        BtcBuyOrder,
                    > as alloy_sol_types::SolType>::tokenize(&self._0),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self._1),
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOpenBtcBuyOrdersCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOpenBtcBuyOrdersReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<BtcBuyOrder>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOpenBtcBuyOrders()";
            const SELECTOR: [u8; 4] = [29u8, 254u8, 117u8, 149u8];
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                getOpenBtcBuyOrdersReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getOpenBtcSellOrders()` and selector `0x6811a311`.
```solidity
function getOpenBtcSellOrders() external view returns (BtcSellOrder[] memory, uint256[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOpenBtcSellOrdersCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getOpenBtcSellOrders()`](getOpenBtcSellOrdersCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOpenBtcSellOrdersReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Vec<
            <BtcSellOrder as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub _1: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
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
            impl ::core::convert::From<getOpenBtcSellOrdersCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOpenBtcSellOrdersCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOpenBtcSellOrdersCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<BtcSellOrder>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <BtcSellOrder as alloy::sol_types::SolType>::RustType,
                >,
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
            impl ::core::convert::From<getOpenBtcSellOrdersReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOpenBtcSellOrdersReturn) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOpenBtcSellOrdersReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0, _1: tuple.1 }
                }
            }
        }
        impl getOpenBtcSellOrdersReturn {
            fn _tokenize(
                &self,
            ) -> <getOpenBtcSellOrdersCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                (
                    <alloy::sol_types::sol_data::Array<
                        BtcSellOrder,
                    > as alloy_sol_types::SolType>::tokenize(&self._0),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self._1),
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOpenBtcSellOrdersCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOpenBtcSellOrdersReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<BtcSellOrder>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOpenBtcSellOrders()";
            const SELECTOR: [u8; 4] = [104u8, 17u8, 163u8, 17u8];
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                getOpenBtcSellOrdersReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getTrustedForwarder()` and selector `0xce1b815f`.
```solidity
function getTrustedForwarder() external view returns (address forwarder);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTrustedForwarderCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getTrustedForwarder()`](getTrustedForwarderCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTrustedForwarderReturn {
        #[allow(missing_docs)]
        pub forwarder: alloy::sol_types::private::Address,
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
                    Self
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
                    (value.forwarder,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getTrustedForwarderReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { forwarder: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getTrustedForwarderCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getTrustedForwarderReturn = r.into();
                        r.forwarder
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: getTrustedForwarderReturn = r.into();
                        r.forwarder
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `isTrustedForwarder(address)` and selector `0x572b6c05`.
```solidity
function isTrustedForwarder(address forwarder) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isTrustedForwarderCall {
        #[allow(missing_docs)]
        pub forwarder: alloy::sol_types::private::Address,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`isTrustedForwarder(address)`](isTrustedForwarderCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isTrustedForwarderReturn {
        #[allow(missing_docs)]
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
            impl ::core::convert::From<isTrustedForwarderCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: isTrustedForwarderCall) -> Self {
                    (value.forwarder,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isTrustedForwarderCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { forwarder: tuple.0 }
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
            impl ::core::convert::From<isTrustedForwarderReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: isTrustedForwarderReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isTrustedForwarderReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isTrustedForwarderCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isTrustedForwarder(address)";
            const SELECTOR: [u8; 4] = [87u8, 43u8, 108u8, 5u8];
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: isTrustedForwarderReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: isTrustedForwarderReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `placeBtcBuyOrder(uint256,(bytes),address,uint256)` and selector `0x364f1ec0`.
```solidity
function placeBtcBuyOrder(uint256 amountBtc, BitcoinAddress memory bitcoinAddress, address sellingToken, uint256 saleAmount) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct placeBtcBuyOrderCall {
        #[allow(missing_docs)]
        pub amountBtc: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub bitcoinAddress: <BitcoinAddress as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub sellingToken: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub saleAmount: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`placeBtcBuyOrder(uint256,(bytes),address,uint256)`](placeBtcBuyOrderCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct placeBtcBuyOrderReturn {}
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
                BitcoinAddress,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                <BitcoinAddress as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<placeBtcBuyOrderCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: placeBtcBuyOrderCall) -> Self {
                    (
                        value.amountBtc,
                        value.bitcoinAddress,
                        value.sellingToken,
                        value.saleAmount,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for placeBtcBuyOrderCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        amountBtc: tuple.0,
                        bitcoinAddress: tuple.1,
                        sellingToken: tuple.2,
                        saleAmount: tuple.3,
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
            impl ::core::convert::From<placeBtcBuyOrderReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: placeBtcBuyOrderReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for placeBtcBuyOrderReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl placeBtcBuyOrderReturn {
            fn _tokenize(
                &self,
            ) -> <placeBtcBuyOrderCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for placeBtcBuyOrderCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                BitcoinAddress,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = placeBtcBuyOrderReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "placeBtcBuyOrder(uint256,(bytes),address,uint256)";
            const SELECTOR: [u8; 4] = [54u8, 79u8, 30u8, 192u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.amountBtc),
                    <BitcoinAddress as alloy_sol_types::SolType>::tokenize(
                        &self.bitcoinAddress,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.sellingToken,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.saleAmount),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                placeBtcBuyOrderReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `placeBtcSellOrder(uint256,address,uint256)` and selector `0x5b8fe042`.
```solidity
function placeBtcSellOrder(uint256 amountBtc, address buyingToken, uint256 buyAmount) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct placeBtcSellOrderCall {
        #[allow(missing_docs)]
        pub amountBtc: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub buyingToken: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub buyAmount: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`placeBtcSellOrder(uint256,address,uint256)`](placeBtcSellOrderCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct placeBtcSellOrderReturn {}
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
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<placeBtcSellOrderCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: placeBtcSellOrderCall) -> Self {
                    (value.amountBtc, value.buyingToken, value.buyAmount)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for placeBtcSellOrderCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        amountBtc: tuple.0,
                        buyingToken: tuple.1,
                        buyAmount: tuple.2,
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
            impl ::core::convert::From<placeBtcSellOrderReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: placeBtcSellOrderReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for placeBtcSellOrderReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl placeBtcSellOrderReturn {
            fn _tokenize(
                &self,
            ) -> <placeBtcSellOrderCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for placeBtcSellOrderCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = placeBtcSellOrderReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "placeBtcSellOrder(uint256,address,uint256)";
            const SELECTOR: [u8; 4] = [91u8, 143u8, 224u8, 66u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.amountBtc),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.buyingToken,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.buyAmount),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                placeBtcSellOrderReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `proofBtcBuyOrder(uint256,(bytes4,bytes,bytes,bytes4),(bytes,uint256,bytes,bytes32,bytes))` and selector `0xb223d976`.
```solidity
function proofBtcBuyOrder(uint256 id, BitcoinTx.Info memory transaction, BitcoinTx.Proof memory proof) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proofBtcBuyOrderCall {
        #[allow(missing_docs)]
        pub id: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub transaction: <BitcoinTx::Info as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub proof: <BitcoinTx::Proof as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`proofBtcBuyOrder(uint256,(bytes4,bytes,bytes,bytes4),(bytes,uint256,bytes,bytes32,bytes))`](proofBtcBuyOrderCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proofBtcBuyOrderReturn {}
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
                BitcoinTx::Info,
                BitcoinTx::Proof,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                <BitcoinTx::Info as alloy::sol_types::SolType>::RustType,
                <BitcoinTx::Proof as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<proofBtcBuyOrderCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: proofBtcBuyOrderCall) -> Self {
                    (value.id, value.transaction, value.proof)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for proofBtcBuyOrderCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        id: tuple.0,
                        transaction: tuple.1,
                        proof: tuple.2,
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
            impl ::core::convert::From<proofBtcBuyOrderReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: proofBtcBuyOrderReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for proofBtcBuyOrderReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl proofBtcBuyOrderReturn {
            fn _tokenize(
                &self,
            ) -> <proofBtcBuyOrderCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for proofBtcBuyOrderCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                BitcoinTx::Info,
                BitcoinTx::Proof,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = proofBtcBuyOrderReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "proofBtcBuyOrder(uint256,(bytes4,bytes,bytes,bytes4),(bytes,uint256,bytes,bytes32,bytes))";
            const SELECTOR: [u8; 4] = [178u8, 35u8, 217u8, 118u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.id),
                    <BitcoinTx::Info as alloy_sol_types::SolType>::tokenize(
                        &self.transaction,
                    ),
                    <BitcoinTx::Proof as alloy_sol_types::SolType>::tokenize(&self.proof),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                proofBtcBuyOrderReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `proofBtcSellOrder(uint256,(bytes4,bytes,bytes,bytes4),(bytes,uint256,bytes,bytes32,bytes))` and selector `0xfd3fc245`.
```solidity
function proofBtcSellOrder(uint256 id, BitcoinTx.Info memory transaction, BitcoinTx.Proof memory proof) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proofBtcSellOrderCall {
        #[allow(missing_docs)]
        pub id: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub transaction: <BitcoinTx::Info as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub proof: <BitcoinTx::Proof as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`proofBtcSellOrder(uint256,(bytes4,bytes,bytes,bytes4),(bytes,uint256,bytes,bytes32,bytes))`](proofBtcSellOrderCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proofBtcSellOrderReturn {}
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
                BitcoinTx::Info,
                BitcoinTx::Proof,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                <BitcoinTx::Info as alloy::sol_types::SolType>::RustType,
                <BitcoinTx::Proof as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<proofBtcSellOrderCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: proofBtcSellOrderCall) -> Self {
                    (value.id, value.transaction, value.proof)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for proofBtcSellOrderCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        id: tuple.0,
                        transaction: tuple.1,
                        proof: tuple.2,
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
            impl ::core::convert::From<proofBtcSellOrderReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: proofBtcSellOrderReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for proofBtcSellOrderReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl proofBtcSellOrderReturn {
            fn _tokenize(
                &self,
            ) -> <proofBtcSellOrderCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for proofBtcSellOrderCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                BitcoinTx::Info,
                BitcoinTx::Proof,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = proofBtcSellOrderReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "proofBtcSellOrder(uint256,(bytes4,bytes,bytes,bytes4),(bytes,uint256,bytes,bytes32,bytes))";
            const SELECTOR: [u8; 4] = [253u8, 63u8, 194u8, 69u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.id),
                    <BitcoinTx::Info as alloy_sol_types::SolType>::tokenize(
                        &self.transaction,
                    ),
                    <BitcoinTx::Proof as alloy_sol_types::SolType>::tokenize(&self.proof),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                proofBtcSellOrderReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `withdrawBtcBuyOrder(uint256)` and selector `0x506a109d`.
```solidity
function withdrawBtcBuyOrder(uint256 id) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct withdrawBtcBuyOrderCall {
        #[allow(missing_docs)]
        pub id: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`withdrawBtcBuyOrder(uint256)`](withdrawBtcBuyOrderCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct withdrawBtcBuyOrderReturn {}
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
            impl ::core::convert::From<withdrawBtcBuyOrderCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: withdrawBtcBuyOrderCall) -> Self {
                    (value.id,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for withdrawBtcBuyOrderCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { id: tuple.0 }
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
            impl ::core::convert::From<withdrawBtcBuyOrderReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: withdrawBtcBuyOrderReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for withdrawBtcBuyOrderReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl withdrawBtcBuyOrderReturn {
            fn _tokenize(
                &self,
            ) -> <withdrawBtcBuyOrderCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for withdrawBtcBuyOrderCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = withdrawBtcBuyOrderReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "withdrawBtcBuyOrder(uint256)";
            const SELECTOR: [u8; 4] = [80u8, 106u8, 16u8, 157u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.id),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                withdrawBtcBuyOrderReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `withdrawBtcSellOrder(uint256)` and selector `0xa383013b`.
```solidity
function withdrawBtcSellOrder(uint256 id) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct withdrawBtcSellOrderCall {
        #[allow(missing_docs)]
        pub id: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`withdrawBtcSellOrder(uint256)`](withdrawBtcSellOrderCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct withdrawBtcSellOrderReturn {}
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
            impl ::core::convert::From<withdrawBtcSellOrderCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: withdrawBtcSellOrderCall) -> Self {
                    (value.id,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for withdrawBtcSellOrderCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { id: tuple.0 }
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
            impl ::core::convert::From<withdrawBtcSellOrderReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: withdrawBtcSellOrderReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for withdrawBtcSellOrderReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl withdrawBtcSellOrderReturn {
            fn _tokenize(
                &self,
            ) -> <withdrawBtcSellOrderCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for withdrawBtcSellOrderCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = withdrawBtcSellOrderReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "withdrawBtcSellOrder(uint256)";
            const SELECTOR: [u8; 4] = [163u8, 131u8, 1u8, 59u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.id),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                withdrawBtcSellOrderReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    ///Container for all the [`BtcMarketPlace`](self) function calls.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum BtcMarketPlaceCalls {
        #[allow(missing_docs)]
        REQUEST_EXPIRATION_SECONDS(REQUEST_EXPIRATION_SECONDSCall),
        #[allow(missing_docs)]
        acceptBtcBuyOrder(acceptBtcBuyOrderCall),
        #[allow(missing_docs)]
        acceptBtcSellOrder(acceptBtcSellOrderCall),
        #[allow(missing_docs)]
        acceptedBtcBuyOrders(acceptedBtcBuyOrdersCall),
        #[allow(missing_docs)]
        acceptedBtcSellOrders(acceptedBtcSellOrdersCall),
        #[allow(missing_docs)]
        btcBuyOrders(btcBuyOrdersCall),
        #[allow(missing_docs)]
        btcSellOrders(btcSellOrdersCall),
        #[allow(missing_docs)]
        cancelAcceptedBtcBuyOrder(cancelAcceptedBtcBuyOrderCall),
        #[allow(missing_docs)]
        cancelAcceptedBtcSellOrder(cancelAcceptedBtcSellOrderCall),
        #[allow(missing_docs)]
        getOpenAcceptedBtcBuyOrders(getOpenAcceptedBtcBuyOrdersCall),
        #[allow(missing_docs)]
        getOpenAcceptedBtcSellOrders(getOpenAcceptedBtcSellOrdersCall),
        #[allow(missing_docs)]
        getOpenBtcBuyOrders(getOpenBtcBuyOrdersCall),
        #[allow(missing_docs)]
        getOpenBtcSellOrders(getOpenBtcSellOrdersCall),
        #[allow(missing_docs)]
        getTrustedForwarder(getTrustedForwarderCall),
        #[allow(missing_docs)]
        isTrustedForwarder(isTrustedForwarderCall),
        #[allow(missing_docs)]
        placeBtcBuyOrder(placeBtcBuyOrderCall),
        #[allow(missing_docs)]
        placeBtcSellOrder(placeBtcSellOrderCall),
        #[allow(missing_docs)]
        proofBtcBuyOrder(proofBtcBuyOrderCall),
        #[allow(missing_docs)]
        proofBtcSellOrder(proofBtcSellOrderCall),
        #[allow(missing_docs)]
        withdrawBtcBuyOrder(withdrawBtcBuyOrderCall),
        #[allow(missing_docs)]
        withdrawBtcSellOrder(withdrawBtcSellOrderCall),
    }
    #[automatically_derived]
    impl BtcMarketPlaceCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [17u8, 193u8, 55u8, 170u8],
            [29u8, 254u8, 117u8, 149u8],
            [33u8, 14u8, 193u8, 129u8],
            [54u8, 79u8, 30u8, 192u8],
            [58u8, 243u8, 252u8, 126u8],
            [65u8, 69u8, 100u8, 10u8],
            [80u8, 106u8, 16u8, 157u8],
            [87u8, 43u8, 108u8, 5u8],
            [91u8, 143u8, 224u8, 66u8],
            [104u8, 17u8, 163u8, 17u8],
            [106u8, 140u8, 222u8, 58u8],
            [156u8, 198u8, 114u8, 46u8],
            [163u8, 131u8, 1u8, 59u8],
            [178u8, 35u8, 217u8, 118u8],
            [189u8, 42u8, 126u8, 62u8],
            [197u8, 106u8, 69u8, 38u8],
            [206u8, 27u8, 129u8, 95u8],
            [209u8, 146u8, 15u8, 240u8],
            [223u8, 105u8, 177u8, 79u8],
            [236u8, 202u8, 44u8, 54u8],
            [253u8, 63u8, 194u8, 69u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for BtcMarketPlaceCalls {
        const NAME: &'static str = "BtcMarketPlaceCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 21usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::REQUEST_EXPIRATION_SECONDS(_) => {
                    <REQUEST_EXPIRATION_SECONDSCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::acceptBtcBuyOrder(_) => {
                    <acceptBtcBuyOrderCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::acceptBtcSellOrder(_) => {
                    <acceptBtcSellOrderCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::acceptedBtcBuyOrders(_) => {
                    <acceptedBtcBuyOrdersCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::acceptedBtcSellOrders(_) => {
                    <acceptedBtcSellOrdersCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::btcBuyOrders(_) => {
                    <btcBuyOrdersCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::btcSellOrders(_) => {
                    <btcSellOrdersCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::cancelAcceptedBtcBuyOrder(_) => {
                    <cancelAcceptedBtcBuyOrderCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::cancelAcceptedBtcSellOrder(_) => {
                    <cancelAcceptedBtcSellOrderCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOpenAcceptedBtcBuyOrders(_) => {
                    <getOpenAcceptedBtcBuyOrdersCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOpenAcceptedBtcSellOrders(_) => {
                    <getOpenAcceptedBtcSellOrdersCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOpenBtcBuyOrders(_) => {
                    <getOpenBtcBuyOrdersCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOpenBtcSellOrders(_) => {
                    <getOpenBtcSellOrdersCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getTrustedForwarder(_) => {
                    <getTrustedForwarderCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isTrustedForwarder(_) => {
                    <isTrustedForwarderCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::placeBtcBuyOrder(_) => {
                    <placeBtcBuyOrderCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::placeBtcSellOrder(_) => {
                    <placeBtcSellOrderCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::proofBtcBuyOrder(_) => {
                    <proofBtcBuyOrderCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::proofBtcSellOrder(_) => {
                    <proofBtcSellOrderCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::withdrawBtcBuyOrder(_) => {
                    <withdrawBtcBuyOrderCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::withdrawBtcSellOrder(_) => {
                    <withdrawBtcSellOrderCall as alloy_sol_types::SolCall>::SELECTOR
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
        #[allow(non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
            ) -> alloy_sol_types::Result<BtcMarketPlaceCalls>] = &[
                {
                    fn acceptBtcBuyOrder(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BtcMarketPlaceCalls> {
                        <acceptBtcBuyOrderCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(BtcMarketPlaceCalls::acceptBtcBuyOrder)
                    }
                    acceptBtcBuyOrder
                },
                {
                    fn getOpenBtcBuyOrders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BtcMarketPlaceCalls> {
                        <getOpenBtcBuyOrdersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(BtcMarketPlaceCalls::getOpenBtcBuyOrders)
                    }
                    getOpenBtcBuyOrders
                },
                {
                    fn acceptBtcSellOrder(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BtcMarketPlaceCalls> {
                        <acceptBtcSellOrderCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(BtcMarketPlaceCalls::acceptBtcSellOrder)
                    }
                    acceptBtcSellOrder
                },
                {
                    fn placeBtcBuyOrder(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BtcMarketPlaceCalls> {
                        <placeBtcBuyOrderCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(BtcMarketPlaceCalls::placeBtcBuyOrder)
                    }
                    placeBtcBuyOrder
                },
                {
                    fn btcBuyOrders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BtcMarketPlaceCalls> {
                        <btcBuyOrdersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(BtcMarketPlaceCalls::btcBuyOrders)
                    }
                    btcBuyOrders
                },
                {
                    fn acceptedBtcSellOrders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BtcMarketPlaceCalls> {
                        <acceptedBtcSellOrdersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(BtcMarketPlaceCalls::acceptedBtcSellOrders)
                    }
                    acceptedBtcSellOrders
                },
                {
                    fn withdrawBtcBuyOrder(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BtcMarketPlaceCalls> {
                        <withdrawBtcBuyOrderCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(BtcMarketPlaceCalls::withdrawBtcBuyOrder)
                    }
                    withdrawBtcBuyOrder
                },
                {
                    fn isTrustedForwarder(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BtcMarketPlaceCalls> {
                        <isTrustedForwarderCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(BtcMarketPlaceCalls::isTrustedForwarder)
                    }
                    isTrustedForwarder
                },
                {
                    fn placeBtcSellOrder(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BtcMarketPlaceCalls> {
                        <placeBtcSellOrderCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(BtcMarketPlaceCalls::placeBtcSellOrder)
                    }
                    placeBtcSellOrder
                },
                {
                    fn getOpenBtcSellOrders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BtcMarketPlaceCalls> {
                        <getOpenBtcSellOrdersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(BtcMarketPlaceCalls::getOpenBtcSellOrders)
                    }
                    getOpenBtcSellOrders
                },
                {
                    fn getOpenAcceptedBtcBuyOrders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BtcMarketPlaceCalls> {
                        <getOpenAcceptedBtcBuyOrdersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(BtcMarketPlaceCalls::getOpenAcceptedBtcBuyOrders)
                    }
                    getOpenAcceptedBtcBuyOrders
                },
                {
                    fn getOpenAcceptedBtcSellOrders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BtcMarketPlaceCalls> {
                        <getOpenAcceptedBtcSellOrdersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(BtcMarketPlaceCalls::getOpenAcceptedBtcSellOrders)
                    }
                    getOpenAcceptedBtcSellOrders
                },
                {
                    fn withdrawBtcSellOrder(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BtcMarketPlaceCalls> {
                        <withdrawBtcSellOrderCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(BtcMarketPlaceCalls::withdrawBtcSellOrder)
                    }
                    withdrawBtcSellOrder
                },
                {
                    fn proofBtcBuyOrder(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BtcMarketPlaceCalls> {
                        <proofBtcBuyOrderCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(BtcMarketPlaceCalls::proofBtcBuyOrder)
                    }
                    proofBtcBuyOrder
                },
                {
                    fn acceptedBtcBuyOrders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BtcMarketPlaceCalls> {
                        <acceptedBtcBuyOrdersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(BtcMarketPlaceCalls::acceptedBtcBuyOrders)
                    }
                    acceptedBtcBuyOrders
                },
                {
                    fn cancelAcceptedBtcBuyOrder(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BtcMarketPlaceCalls> {
                        <cancelAcceptedBtcBuyOrderCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(BtcMarketPlaceCalls::cancelAcceptedBtcBuyOrder)
                    }
                    cancelAcceptedBtcBuyOrder
                },
                {
                    fn getTrustedForwarder(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BtcMarketPlaceCalls> {
                        <getTrustedForwarderCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(BtcMarketPlaceCalls::getTrustedForwarder)
                    }
                    getTrustedForwarder
                },
                {
                    fn REQUEST_EXPIRATION_SECONDS(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BtcMarketPlaceCalls> {
                        <REQUEST_EXPIRATION_SECONDSCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(BtcMarketPlaceCalls::REQUEST_EXPIRATION_SECONDS)
                    }
                    REQUEST_EXPIRATION_SECONDS
                },
                {
                    fn cancelAcceptedBtcSellOrder(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BtcMarketPlaceCalls> {
                        <cancelAcceptedBtcSellOrderCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(BtcMarketPlaceCalls::cancelAcceptedBtcSellOrder)
                    }
                    cancelAcceptedBtcSellOrder
                },
                {
                    fn btcSellOrders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BtcMarketPlaceCalls> {
                        <btcSellOrdersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(BtcMarketPlaceCalls::btcSellOrders)
                    }
                    btcSellOrders
                },
                {
                    fn proofBtcSellOrder(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BtcMarketPlaceCalls> {
                        <proofBtcSellOrderCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(BtcMarketPlaceCalls::proofBtcSellOrder)
                    }
                    proofBtcSellOrder
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
            DECODE_SHIMS[idx](data)
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw_validate(
            selector: [u8; 4],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_VALIDATE_SHIMS: &[fn(
                &[u8],
            ) -> alloy_sol_types::Result<BtcMarketPlaceCalls>] = &[
                {
                    fn acceptBtcBuyOrder(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BtcMarketPlaceCalls> {
                        <acceptBtcBuyOrderCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(BtcMarketPlaceCalls::acceptBtcBuyOrder)
                    }
                    acceptBtcBuyOrder
                },
                {
                    fn getOpenBtcBuyOrders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BtcMarketPlaceCalls> {
                        <getOpenBtcBuyOrdersCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(BtcMarketPlaceCalls::getOpenBtcBuyOrders)
                    }
                    getOpenBtcBuyOrders
                },
                {
                    fn acceptBtcSellOrder(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BtcMarketPlaceCalls> {
                        <acceptBtcSellOrderCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(BtcMarketPlaceCalls::acceptBtcSellOrder)
                    }
                    acceptBtcSellOrder
                },
                {
                    fn placeBtcBuyOrder(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BtcMarketPlaceCalls> {
                        <placeBtcBuyOrderCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(BtcMarketPlaceCalls::placeBtcBuyOrder)
                    }
                    placeBtcBuyOrder
                },
                {
                    fn btcBuyOrders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BtcMarketPlaceCalls> {
                        <btcBuyOrdersCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(BtcMarketPlaceCalls::btcBuyOrders)
                    }
                    btcBuyOrders
                },
                {
                    fn acceptedBtcSellOrders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BtcMarketPlaceCalls> {
                        <acceptedBtcSellOrdersCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(BtcMarketPlaceCalls::acceptedBtcSellOrders)
                    }
                    acceptedBtcSellOrders
                },
                {
                    fn withdrawBtcBuyOrder(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BtcMarketPlaceCalls> {
                        <withdrawBtcBuyOrderCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(BtcMarketPlaceCalls::withdrawBtcBuyOrder)
                    }
                    withdrawBtcBuyOrder
                },
                {
                    fn isTrustedForwarder(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BtcMarketPlaceCalls> {
                        <isTrustedForwarderCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(BtcMarketPlaceCalls::isTrustedForwarder)
                    }
                    isTrustedForwarder
                },
                {
                    fn placeBtcSellOrder(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BtcMarketPlaceCalls> {
                        <placeBtcSellOrderCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(BtcMarketPlaceCalls::placeBtcSellOrder)
                    }
                    placeBtcSellOrder
                },
                {
                    fn getOpenBtcSellOrders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BtcMarketPlaceCalls> {
                        <getOpenBtcSellOrdersCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(BtcMarketPlaceCalls::getOpenBtcSellOrders)
                    }
                    getOpenBtcSellOrders
                },
                {
                    fn getOpenAcceptedBtcBuyOrders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BtcMarketPlaceCalls> {
                        <getOpenAcceptedBtcBuyOrdersCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(BtcMarketPlaceCalls::getOpenAcceptedBtcBuyOrders)
                    }
                    getOpenAcceptedBtcBuyOrders
                },
                {
                    fn getOpenAcceptedBtcSellOrders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BtcMarketPlaceCalls> {
                        <getOpenAcceptedBtcSellOrdersCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(BtcMarketPlaceCalls::getOpenAcceptedBtcSellOrders)
                    }
                    getOpenAcceptedBtcSellOrders
                },
                {
                    fn withdrawBtcSellOrder(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BtcMarketPlaceCalls> {
                        <withdrawBtcSellOrderCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(BtcMarketPlaceCalls::withdrawBtcSellOrder)
                    }
                    withdrawBtcSellOrder
                },
                {
                    fn proofBtcBuyOrder(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BtcMarketPlaceCalls> {
                        <proofBtcBuyOrderCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(BtcMarketPlaceCalls::proofBtcBuyOrder)
                    }
                    proofBtcBuyOrder
                },
                {
                    fn acceptedBtcBuyOrders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BtcMarketPlaceCalls> {
                        <acceptedBtcBuyOrdersCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(BtcMarketPlaceCalls::acceptedBtcBuyOrders)
                    }
                    acceptedBtcBuyOrders
                },
                {
                    fn cancelAcceptedBtcBuyOrder(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BtcMarketPlaceCalls> {
                        <cancelAcceptedBtcBuyOrderCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(BtcMarketPlaceCalls::cancelAcceptedBtcBuyOrder)
                    }
                    cancelAcceptedBtcBuyOrder
                },
                {
                    fn getTrustedForwarder(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BtcMarketPlaceCalls> {
                        <getTrustedForwarderCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(BtcMarketPlaceCalls::getTrustedForwarder)
                    }
                    getTrustedForwarder
                },
                {
                    fn REQUEST_EXPIRATION_SECONDS(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BtcMarketPlaceCalls> {
                        <REQUEST_EXPIRATION_SECONDSCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(BtcMarketPlaceCalls::REQUEST_EXPIRATION_SECONDS)
                    }
                    REQUEST_EXPIRATION_SECONDS
                },
                {
                    fn cancelAcceptedBtcSellOrder(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BtcMarketPlaceCalls> {
                        <cancelAcceptedBtcSellOrderCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(BtcMarketPlaceCalls::cancelAcceptedBtcSellOrder)
                    }
                    cancelAcceptedBtcSellOrder
                },
                {
                    fn btcSellOrders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BtcMarketPlaceCalls> {
                        <btcSellOrdersCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(BtcMarketPlaceCalls::btcSellOrders)
                    }
                    btcSellOrders
                },
                {
                    fn proofBtcSellOrder(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BtcMarketPlaceCalls> {
                        <proofBtcSellOrderCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(BtcMarketPlaceCalls::proofBtcSellOrder)
                    }
                    proofBtcSellOrder
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
            DECODE_VALIDATE_SHIMS[idx](data)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::REQUEST_EXPIRATION_SECONDS(inner) => {
                    <REQUEST_EXPIRATION_SECONDSCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::acceptBtcBuyOrder(inner) => {
                    <acceptBtcBuyOrderCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::acceptBtcSellOrder(inner) => {
                    <acceptBtcSellOrderCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::acceptedBtcBuyOrders(inner) => {
                    <acceptedBtcBuyOrdersCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::acceptedBtcSellOrders(inner) => {
                    <acceptedBtcSellOrdersCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::btcBuyOrders(inner) => {
                    <btcBuyOrdersCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::btcSellOrders(inner) => {
                    <btcSellOrdersCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::cancelAcceptedBtcBuyOrder(inner) => {
                    <cancelAcceptedBtcBuyOrderCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::cancelAcceptedBtcSellOrder(inner) => {
                    <cancelAcceptedBtcSellOrderCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOpenAcceptedBtcBuyOrders(inner) => {
                    <getOpenAcceptedBtcBuyOrdersCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOpenAcceptedBtcSellOrders(inner) => {
                    <getOpenAcceptedBtcSellOrdersCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOpenBtcBuyOrders(inner) => {
                    <getOpenBtcBuyOrdersCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOpenBtcSellOrders(inner) => {
                    <getOpenBtcSellOrdersCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getTrustedForwarder(inner) => {
                    <getTrustedForwarderCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::isTrustedForwarder(inner) => {
                    <isTrustedForwarderCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::placeBtcBuyOrder(inner) => {
                    <placeBtcBuyOrderCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::placeBtcSellOrder(inner) => {
                    <placeBtcSellOrderCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::proofBtcBuyOrder(inner) => {
                    <proofBtcBuyOrderCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::proofBtcSellOrder(inner) => {
                    <proofBtcSellOrderCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::withdrawBtcBuyOrder(inner) => {
                    <withdrawBtcBuyOrderCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::withdrawBtcSellOrder(inner) => {
                    <withdrawBtcSellOrderCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::REQUEST_EXPIRATION_SECONDS(inner) => {
                    <REQUEST_EXPIRATION_SECONDSCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::acceptBtcBuyOrder(inner) => {
                    <acceptBtcBuyOrderCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::acceptBtcSellOrder(inner) => {
                    <acceptBtcSellOrderCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::acceptedBtcBuyOrders(inner) => {
                    <acceptedBtcBuyOrdersCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::acceptedBtcSellOrders(inner) => {
                    <acceptedBtcSellOrdersCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::btcBuyOrders(inner) => {
                    <btcBuyOrdersCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::btcSellOrders(inner) => {
                    <btcSellOrdersCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::cancelAcceptedBtcBuyOrder(inner) => {
                    <cancelAcceptedBtcBuyOrderCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::cancelAcceptedBtcSellOrder(inner) => {
                    <cancelAcceptedBtcSellOrderCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getOpenAcceptedBtcBuyOrders(inner) => {
                    <getOpenAcceptedBtcBuyOrdersCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getOpenAcceptedBtcSellOrders(inner) => {
                    <getOpenAcceptedBtcSellOrdersCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getOpenBtcBuyOrders(inner) => {
                    <getOpenBtcBuyOrdersCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getOpenBtcSellOrders(inner) => {
                    <getOpenBtcSellOrdersCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::isTrustedForwarder(inner) => {
                    <isTrustedForwarderCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::placeBtcBuyOrder(inner) => {
                    <placeBtcBuyOrderCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::placeBtcSellOrder(inner) => {
                    <placeBtcSellOrderCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::proofBtcBuyOrder(inner) => {
                    <proofBtcBuyOrderCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::proofBtcSellOrder(inner) => {
                    <proofBtcSellOrderCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::withdrawBtcBuyOrder(inner) => {
                    <withdrawBtcBuyOrderCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::withdrawBtcSellOrder(inner) => {
                    <withdrawBtcSellOrderCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`BtcMarketPlace`](self) events.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum BtcMarketPlaceEvents {
        #[allow(missing_docs)]
        acceptBtcBuyOrderEvent(acceptBtcBuyOrderEvent),
        #[allow(missing_docs)]
        acceptBtcSellOrderEvent(acceptBtcSellOrderEvent),
        #[allow(missing_docs)]
        cancelAcceptedBtcBuyOrderEvent(cancelAcceptedBtcBuyOrderEvent),
        #[allow(missing_docs)]
        cancelAcceptedBtcSellOrderEvent(cancelAcceptedBtcSellOrderEvent),
        #[allow(missing_docs)]
        placeBtcBuyOrderEvent(placeBtcBuyOrderEvent),
        #[allow(missing_docs)]
        placeBtcSellOrderEvent(placeBtcSellOrderEvent),
        #[allow(missing_docs)]
        proofBtcBuyOrderEvent(proofBtcBuyOrderEvent),
        #[allow(missing_docs)]
        proofBtcSellOrderEvent(proofBtcSellOrderEvent),
        #[allow(missing_docs)]
        withdrawBtcBuyOrderEvent(withdrawBtcBuyOrderEvent),
        #[allow(missing_docs)]
        withdrawBtcSellOrderEvent(withdrawBtcSellOrderEvent),
    }
    #[automatically_derived]
    impl BtcMarketPlaceEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                60u8, 212u8, 117u8, 176u8, 146u8, 232u8, 179u8, 121u8, 246u8, 186u8,
                13u8, 158u8, 14u8, 12u8, 143u8, 48u8, 112u8, 94u8, 115u8, 50u8, 29u8,
                197u8, 201u8, 248u8, 12u8, 228u8, 173u8, 56u8, 219u8, 123u8, 225u8, 170u8,
            ],
            [
                62u8, 94u8, 163u8, 88u8, 233u8, 235u8, 76u8, 223u8, 68u8, 205u8, 199u8,
                121u8, 56u8, 173u8, 232u8, 7u8, 75u8, 18u8, 64u8, 166u8, 216u8, 192u8,
                253u8, 19u8, 114u8, 134u8, 113u8, 184u8, 46u8, 128u8, 10u8, 214u8,
            ],
            [
                101u8, 62u8, 13u8, 129u8, 242u8, 201u8, 155u8, 235u8, 163u8, 89u8, 223u8,
                177u8, 123u8, 73u8, 154u8, 92u8, 255u8, 43u8, 233u8, 217u8, 80u8, 81u8,
                72u8, 82u8, 34u8, 77u8, 248u8, 192u8, 151u8, 194u8, 25u8, 33u8,
            ],
            [
                120u8, 245u8, 31u8, 98u8, 247u8, 207u8, 19u8, 129u8, 198u8, 115u8, 194u8,
                126u8, 174u8, 24u8, 125u8, 214u8, 197u8, 136u8, 220u8, 102u8, 36u8,
                206u8, 89u8, 105u8, 125u8, 187u8, 62u8, 29u8, 124u8, 27u8, 188u8, 223u8,
            ],
            [
                152u8, 199u8, 198u8, 128u8, 64u8, 61u8, 71u8, 64u8, 61u8, 234u8, 74u8,
                87u8, 13u8, 14u8, 108u8, 87u8, 22u8, 83u8, 140u8, 73u8, 66u8, 14u8,
                244u8, 113u8, 206u8, 196u8, 40u8, 245u8, 165u8, 133u8, 44u8, 6u8,
            ],
            [
                180u8, 201u8, 141u8, 226u8, 16u8, 105u8, 107u8, 60u8, 242u8, 30u8, 153u8,
                51u8, 92u8, 30u8, 227u8, 160u8, 174u8, 52u8, 162u8, 103u8, 19u8, 65u8,
                42u8, 74u8, 221u8, 232u8, 175u8, 89u8, 97u8, 118u8, 243u8, 126u8,
            ],
            [
                195u8, 64u8, 231u8, 172u8, 72u8, 220u8, 128u8, 238u8, 121u8, 63u8, 198u8,
                38u8, 105u8, 96u8, 189u8, 95u8, 27u8, 210u8, 27u8, 233u8, 28u8, 138u8,
                149u8, 226u8, 24u8, 23u8, 129u8, 19u8, 247u8, 158u8, 23u8, 180u8,
            ],
            [
                195u8, 154u8, 26u8, 93u8, 220u8, 14u8, 133u8, 201u8, 85u8, 254u8, 46u8,
                26u8, 190u8, 180u8, 60u8, 148u8, 206u8, 24u8, 50u8, 46u8, 117u8, 187u8,
                61u8, 68u8, 232u8, 15u8, 117u8, 159u8, 249u8, 208u8, 52u8, 185u8,
            ],
            [
                207u8, 86u8, 16u8, 97u8, 219u8, 120u8, 247u8, 188u8, 81u8, 141u8, 55u8,
                254u8, 134u8, 113u8, 133u8, 20u8, 198u8, 64u8, 204u8, 197u8, 195u8,
                241u8, 41u8, 56u8, 40u8, 185u8, 85u8, 230u8, 143u8, 25u8, 245u8, 251u8,
            ],
            [
                255u8, 28u8, 226u8, 16u8, 222u8, 252u8, 211u8, 186u8, 26u8, 223u8, 118u8,
                201u8, 65u8, 154u8, 7u8, 88u8, 250u8, 96u8, 253u8, 62u8, 179u8, 140u8,
                123u8, 217u8, 65u8, 143u8, 96u8, 181u8, 117u8, 183u8, 110u8, 36u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for BtcMarketPlaceEvents {
        const NAME: &'static str = "BtcMarketPlaceEvents";
        const COUNT: usize = 10usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(
                    <acceptBtcBuyOrderEvent as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <acceptBtcBuyOrderEvent as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::acceptBtcBuyOrderEvent)
                }
                Some(
                    <acceptBtcSellOrderEvent as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <acceptBtcSellOrderEvent as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::acceptBtcSellOrderEvent)
                }
                Some(
                    <cancelAcceptedBtcBuyOrderEvent as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <cancelAcceptedBtcBuyOrderEvent as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::cancelAcceptedBtcBuyOrderEvent)
                }
                Some(
                    <cancelAcceptedBtcSellOrderEvent as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <cancelAcceptedBtcSellOrderEvent as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::cancelAcceptedBtcSellOrderEvent)
                }
                Some(
                    <placeBtcBuyOrderEvent as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <placeBtcBuyOrderEvent as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::placeBtcBuyOrderEvent)
                }
                Some(
                    <placeBtcSellOrderEvent as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <placeBtcSellOrderEvent as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::placeBtcSellOrderEvent)
                }
                Some(
                    <proofBtcBuyOrderEvent as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <proofBtcBuyOrderEvent as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::proofBtcBuyOrderEvent)
                }
                Some(
                    <proofBtcSellOrderEvent as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <proofBtcSellOrderEvent as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::proofBtcSellOrderEvent)
                }
                Some(
                    <withdrawBtcBuyOrderEvent as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <withdrawBtcBuyOrderEvent as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::withdrawBtcBuyOrderEvent)
                }
                Some(
                    <withdrawBtcSellOrderEvent as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <withdrawBtcSellOrderEvent as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::withdrawBtcSellOrderEvent)
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
    impl alloy_sol_types::private::IntoLogData for BtcMarketPlaceEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::acceptBtcBuyOrderEvent(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::acceptBtcSellOrderEvent(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::cancelAcceptedBtcBuyOrderEvent(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::cancelAcceptedBtcSellOrderEvent(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::placeBtcBuyOrderEvent(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::placeBtcSellOrderEvent(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::proofBtcBuyOrderEvent(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::proofBtcSellOrderEvent(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::withdrawBtcBuyOrderEvent(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::withdrawBtcSellOrderEvent(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::acceptBtcBuyOrderEvent(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::acceptBtcSellOrderEvent(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::cancelAcceptedBtcBuyOrderEvent(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::cancelAcceptedBtcSellOrderEvent(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::placeBtcBuyOrderEvent(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::placeBtcSellOrderEvent(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::proofBtcBuyOrderEvent(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::proofBtcSellOrderEvent(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::withdrawBtcBuyOrderEvent(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::withdrawBtcSellOrderEvent(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`BtcMarketPlace`](self) contract instance.

See the [wrapper's documentation](`BtcMarketPlaceInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> BtcMarketPlaceInstance<P, N> {
        BtcMarketPlaceInstance::<P, N>::new(address, provider)
    }
    /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
    #[inline]
    pub fn deploy<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        provider: P,
        _relay: alloy::sol_types::private::Address,
        erc2771Forwarder: alloy::sol_types::private::Address,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<BtcMarketPlaceInstance<P, N>>,
    > {
        BtcMarketPlaceInstance::<P, N>::deploy(provider, _relay, erc2771Forwarder)
    }
    /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
    #[inline]
    pub fn deploy_builder<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        provider: P,
        _relay: alloy::sol_types::private::Address,
        erc2771Forwarder: alloy::sol_types::private::Address,
    ) -> alloy_contract::RawCallBuilder<P, N> {
        BtcMarketPlaceInstance::<
            P,
            N,
        >::deploy_builder(provider, _relay, erc2771Forwarder)
    }
    /**A [`BtcMarketPlace`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`BtcMarketPlace`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct BtcMarketPlaceInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for BtcMarketPlaceInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("BtcMarketPlaceInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > BtcMarketPlaceInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`BtcMarketPlace`](self) contract instance.

See the [wrapper's documentation](`BtcMarketPlaceInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            provider: P,
        ) -> Self {
            Self {
                address,
                provider,
                _network: ::core::marker::PhantomData,
            }
        }
        /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
        #[inline]
        pub async fn deploy(
            provider: P,
            _relay: alloy::sol_types::private::Address,
            erc2771Forwarder: alloy::sol_types::private::Address,
        ) -> alloy_contract::Result<BtcMarketPlaceInstance<P, N>> {
            let call_builder = Self::deploy_builder(provider, _relay, erc2771Forwarder);
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
            _relay: alloy::sol_types::private::Address,
            erc2771Forwarder: alloy::sol_types::private::Address,
        ) -> alloy_contract::RawCallBuilder<P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall {
                            _relay,
                            erc2771Forwarder,
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
    impl<P: ::core::clone::Clone, N> BtcMarketPlaceInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> BtcMarketPlaceInstance<P, N> {
            BtcMarketPlaceInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > BtcMarketPlaceInstance<P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<&P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
        ///Creates a new call builder for the [`REQUEST_EXPIRATION_SECONDS`] function.
        pub fn REQUEST_EXPIRATION_SECONDS(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, REQUEST_EXPIRATION_SECONDSCall, N> {
            self.call_builder(&REQUEST_EXPIRATION_SECONDSCall)
        }
        ///Creates a new call builder for the [`acceptBtcBuyOrder`] function.
        pub fn acceptBtcBuyOrder(
            &self,
            id: alloy::sol_types::private::primitives::aliases::U256,
            amountBtc: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, acceptBtcBuyOrderCall, N> {
            self.call_builder(
                &acceptBtcBuyOrderCall {
                    id,
                    amountBtc,
                },
            )
        }
        ///Creates a new call builder for the [`acceptBtcSellOrder`] function.
        pub fn acceptBtcSellOrder(
            &self,
            id: alloy::sol_types::private::primitives::aliases::U256,
            bitcoinAddress: <BitcoinAddress as alloy::sol_types::SolType>::RustType,
            amountBtc: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, acceptBtcSellOrderCall, N> {
            self.call_builder(
                &acceptBtcSellOrderCall {
                    id,
                    bitcoinAddress,
                    amountBtc,
                },
            )
        }
        ///Creates a new call builder for the [`acceptedBtcBuyOrders`] function.
        pub fn acceptedBtcBuyOrders(
            &self,
            _0: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, acceptedBtcBuyOrdersCall, N> {
            self.call_builder(&acceptedBtcBuyOrdersCall(_0))
        }
        ///Creates a new call builder for the [`acceptedBtcSellOrders`] function.
        pub fn acceptedBtcSellOrders(
            &self,
            _0: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, acceptedBtcSellOrdersCall, N> {
            self.call_builder(&acceptedBtcSellOrdersCall(_0))
        }
        ///Creates a new call builder for the [`btcBuyOrders`] function.
        pub fn btcBuyOrders(
            &self,
            _0: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, btcBuyOrdersCall, N> {
            self.call_builder(&btcBuyOrdersCall(_0))
        }
        ///Creates a new call builder for the [`btcSellOrders`] function.
        pub fn btcSellOrders(
            &self,
            _0: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, btcSellOrdersCall, N> {
            self.call_builder(&btcSellOrdersCall(_0))
        }
        ///Creates a new call builder for the [`cancelAcceptedBtcBuyOrder`] function.
        pub fn cancelAcceptedBtcBuyOrder(
            &self,
            id: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, cancelAcceptedBtcBuyOrderCall, N> {
            self.call_builder(
                &cancelAcceptedBtcBuyOrderCall {
                    id,
                },
            )
        }
        ///Creates a new call builder for the [`cancelAcceptedBtcSellOrder`] function.
        pub fn cancelAcceptedBtcSellOrder(
            &self,
            id: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, cancelAcceptedBtcSellOrderCall, N> {
            self.call_builder(
                &cancelAcceptedBtcSellOrderCall {
                    id,
                },
            )
        }
        ///Creates a new call builder for the [`getOpenAcceptedBtcBuyOrders`] function.
        pub fn getOpenAcceptedBtcBuyOrders(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getOpenAcceptedBtcBuyOrdersCall, N> {
            self.call_builder(&getOpenAcceptedBtcBuyOrdersCall)
        }
        ///Creates a new call builder for the [`getOpenAcceptedBtcSellOrders`] function.
        pub fn getOpenAcceptedBtcSellOrders(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getOpenAcceptedBtcSellOrdersCall, N> {
            self.call_builder(&getOpenAcceptedBtcSellOrdersCall)
        }
        ///Creates a new call builder for the [`getOpenBtcBuyOrders`] function.
        pub fn getOpenBtcBuyOrders(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getOpenBtcBuyOrdersCall, N> {
            self.call_builder(&getOpenBtcBuyOrdersCall)
        }
        ///Creates a new call builder for the [`getOpenBtcSellOrders`] function.
        pub fn getOpenBtcSellOrders(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getOpenBtcSellOrdersCall, N> {
            self.call_builder(&getOpenBtcSellOrdersCall)
        }
        ///Creates a new call builder for the [`getTrustedForwarder`] function.
        pub fn getTrustedForwarder(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getTrustedForwarderCall, N> {
            self.call_builder(&getTrustedForwarderCall)
        }
        ///Creates a new call builder for the [`isTrustedForwarder`] function.
        pub fn isTrustedForwarder(
            &self,
            forwarder: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, isTrustedForwarderCall, N> {
            self.call_builder(
                &isTrustedForwarderCall {
                    forwarder,
                },
            )
        }
        ///Creates a new call builder for the [`placeBtcBuyOrder`] function.
        pub fn placeBtcBuyOrder(
            &self,
            amountBtc: alloy::sol_types::private::primitives::aliases::U256,
            bitcoinAddress: <BitcoinAddress as alloy::sol_types::SolType>::RustType,
            sellingToken: alloy::sol_types::private::Address,
            saleAmount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, placeBtcBuyOrderCall, N> {
            self.call_builder(
                &placeBtcBuyOrderCall {
                    amountBtc,
                    bitcoinAddress,
                    sellingToken,
                    saleAmount,
                },
            )
        }
        ///Creates a new call builder for the [`placeBtcSellOrder`] function.
        pub fn placeBtcSellOrder(
            &self,
            amountBtc: alloy::sol_types::private::primitives::aliases::U256,
            buyingToken: alloy::sol_types::private::Address,
            buyAmount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, placeBtcSellOrderCall, N> {
            self.call_builder(
                &placeBtcSellOrderCall {
                    amountBtc,
                    buyingToken,
                    buyAmount,
                },
            )
        }
        ///Creates a new call builder for the [`proofBtcBuyOrder`] function.
        pub fn proofBtcBuyOrder(
            &self,
            id: alloy::sol_types::private::primitives::aliases::U256,
            transaction: <BitcoinTx::Info as alloy::sol_types::SolType>::RustType,
            proof: <BitcoinTx::Proof as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, proofBtcBuyOrderCall, N> {
            self.call_builder(
                &proofBtcBuyOrderCall {
                    id,
                    transaction,
                    proof,
                },
            )
        }
        ///Creates a new call builder for the [`proofBtcSellOrder`] function.
        pub fn proofBtcSellOrder(
            &self,
            id: alloy::sol_types::private::primitives::aliases::U256,
            transaction: <BitcoinTx::Info as alloy::sol_types::SolType>::RustType,
            proof: <BitcoinTx::Proof as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, proofBtcSellOrderCall, N> {
            self.call_builder(
                &proofBtcSellOrderCall {
                    id,
                    transaction,
                    proof,
                },
            )
        }
        ///Creates a new call builder for the [`withdrawBtcBuyOrder`] function.
        pub fn withdrawBtcBuyOrder(
            &self,
            id: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, withdrawBtcBuyOrderCall, N> {
            self.call_builder(&withdrawBtcBuyOrderCall { id })
        }
        ///Creates a new call builder for the [`withdrawBtcSellOrder`] function.
        pub fn withdrawBtcSellOrder(
            &self,
            id: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, withdrawBtcSellOrderCall, N> {
            self.call_builder(&withdrawBtcSellOrderCall { id })
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > BtcMarketPlaceInstance<P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`acceptBtcBuyOrderEvent`] event.
        pub fn acceptBtcBuyOrderEvent_filter(
            &self,
        ) -> alloy_contract::Event<&P, acceptBtcBuyOrderEvent, N> {
            self.event_filter::<acceptBtcBuyOrderEvent>()
        }
        ///Creates a new event filter for the [`acceptBtcSellOrderEvent`] event.
        pub fn acceptBtcSellOrderEvent_filter(
            &self,
        ) -> alloy_contract::Event<&P, acceptBtcSellOrderEvent, N> {
            self.event_filter::<acceptBtcSellOrderEvent>()
        }
        ///Creates a new event filter for the [`cancelAcceptedBtcBuyOrderEvent`] event.
        pub fn cancelAcceptedBtcBuyOrderEvent_filter(
            &self,
        ) -> alloy_contract::Event<&P, cancelAcceptedBtcBuyOrderEvent, N> {
            self.event_filter::<cancelAcceptedBtcBuyOrderEvent>()
        }
        ///Creates a new event filter for the [`cancelAcceptedBtcSellOrderEvent`] event.
        pub fn cancelAcceptedBtcSellOrderEvent_filter(
            &self,
        ) -> alloy_contract::Event<&P, cancelAcceptedBtcSellOrderEvent, N> {
            self.event_filter::<cancelAcceptedBtcSellOrderEvent>()
        }
        ///Creates a new event filter for the [`placeBtcBuyOrderEvent`] event.
        pub fn placeBtcBuyOrderEvent_filter(
            &self,
        ) -> alloy_contract::Event<&P, placeBtcBuyOrderEvent, N> {
            self.event_filter::<placeBtcBuyOrderEvent>()
        }
        ///Creates a new event filter for the [`placeBtcSellOrderEvent`] event.
        pub fn placeBtcSellOrderEvent_filter(
            &self,
        ) -> alloy_contract::Event<&P, placeBtcSellOrderEvent, N> {
            self.event_filter::<placeBtcSellOrderEvent>()
        }
        ///Creates a new event filter for the [`proofBtcBuyOrderEvent`] event.
        pub fn proofBtcBuyOrderEvent_filter(
            &self,
        ) -> alloy_contract::Event<&P, proofBtcBuyOrderEvent, N> {
            self.event_filter::<proofBtcBuyOrderEvent>()
        }
        ///Creates a new event filter for the [`proofBtcSellOrderEvent`] event.
        pub fn proofBtcSellOrderEvent_filter(
            &self,
        ) -> alloy_contract::Event<&P, proofBtcSellOrderEvent, N> {
            self.event_filter::<proofBtcSellOrderEvent>()
        }
        ///Creates a new event filter for the [`withdrawBtcBuyOrderEvent`] event.
        pub fn withdrawBtcBuyOrderEvent_filter(
            &self,
        ) -> alloy_contract::Event<&P, withdrawBtcBuyOrderEvent, N> {
            self.event_filter::<withdrawBtcBuyOrderEvent>()
        }
        ///Creates a new event filter for the [`withdrawBtcSellOrderEvent`] event.
        pub fn withdrawBtcSellOrderEvent_filter(
            &self,
        ) -> alloy_contract::Event<&P, withdrawBtcSellOrderEvent, N> {
            self.event_filter::<withdrawBtcSellOrderEvent>()
        }
    }
}
