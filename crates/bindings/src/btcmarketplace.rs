///Module containing a contract's types and functions.
/**

```solidity
library BitcoinTx {
    struct Info { bytes4 version; bytes inputVector; bytes outputVector; bytes4 locktime; }
    struct Proof { bytes merkleProof; uint256 txIndexInBlock; bytes bitcoinHeaders; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style
)]
pub mod BitcoinTx {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
struct Info { bytes4 version; bytes inputVector; bytes outputVector; bytes4 locktime; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Info {
        pub version: alloy::sol_types::private::FixedBytes<4>,
        pub inputVector: alloy::sol_types::private::Bytes,
        pub outputVector: alloy::sol_types::private::Bytes,
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
    /**```solidity
struct Proof { bytes merkleProof; uint256 txIndexInBlock; bytes bitcoinHeaders; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Proof {
        pub merkleProof: alloy::sol_types::private::Bytes,
        pub txIndexInBlock: alloy::sol_types::private::primitives::aliases::U256,
        pub bitcoinHeaders: alloy::sol_types::private::Bytes,
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
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Bytes,
            alloy::sol_types::private::primitives::aliases::U256,
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
                (value.merkleProof, value.txIndexInBlock, value.bitcoinHeaders)
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
                    "Proof(bytes merkleProof,uint256 txIndexInBlock,bytes bitcoinHeaders)",
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
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> BitcoinTxInstance<T, P, N> {
        BitcoinTxInstance::<T, P, N>::new(address, provider)
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
    pub struct BitcoinTxInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for BitcoinTxInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("BitcoinTxInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > BitcoinTxInstance<T, P, N> {
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
    impl<T, P: ::core::clone::Clone, N> BitcoinTxInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> BitcoinTxInstance<T, P, N> {
            BitcoinTxInstance {
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
    > BitcoinTxInstance<T, P, N> {
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
    > BitcoinTxInstance<T, P, N> {
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
    clippy::style
)]
pub mod BtcMarketPlace {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x6080604052348015600e575f5ffd5b5060405161437e38038061437e833981016040819052602b916081565b5f80546001600160a01b0319166001600160a01b038316179055600680546001600160a01b0319166001600160a01b0384161790555050600160075560b4565b6001600160a01b0381168114607e575f5ffd5b50565b5f5f604083850312156091575f5ffd5b8251609a81606b565b602084015190925060a981606b565b809150509250929050565b6142bd806100c15f395ff3fe608060405234801561000f575f5ffd5b506004361061016e575f3560e01c80636a8cde3a116100d2578063bd2a7e3e11610088578063d1920ff011610063578063d1920ff0146103bc578063df69b14f146103c5578063ecca2c36146103d8575f5ffd5b8063bd2a7e3e146102f3578063c56a45261461038f578063ce1b815f146103a2575f5ffd5b80639cc6722e116100b85780639cc6722e146102b7578063a383013b146102cd578063b522c133146102e0575f5ffd5b80636a8cde3a1461028e57806372a546c6146102a4575f5ffd5b80634145640a11610127578063572b6c051161010d578063572b6c05146102345780635b8fe042146102655780636811a31114610278575f5ffd5b80634145640a146101fa578063506a109d14610221575f5ffd5b8063210ec18111610157578063210ec181146101ae578063364f1ec0146101c15780633af3fc7e146101d6575f5ffd5b806311c137aa146101725780631dfe759514610198575b5f5ffd5b610185610180366004613353565b610445565b6040519081526020015b60405180910390f35b6101a061063e565b60405161018f9291906133ef565b6101856101bc3660046134dd565b6108a0565b6101d46101cf366004613545565b610ad7565b005b6101e96101e43660046135a0565b610c2c565b60405161018f9594939291906135b7565b61020d6102083660046135a0565b610cfe565b60405161018f9897969594939291906135ff565b6101d461022f3660046135a0565b610def565b610255610242366004613654565b5f546001600160a01b0391821691161490565b604051901515815260200161018f565b6101d461027336600461366d565b610ed4565b610280610ff0565b60405161018f9291906136a0565b6102966111b4565b60405161018f92919061372f565b6101d46102b23660046137c7565b6113ba565b6102bf611564565b60405161018f92919061383e565b6101d46102db3660046135a0565b6117c7565b6101d46102ee3660046137c7565b61186b565b61034c6103013660046135a0565b600260208190525f918252604090912080546001820154928201546003830154600484015460058501546006909501549395946001600160a01b039384169492939182169291169087565b6040805197885260208801969096526001600160a01b03948516958701959095526060860192909252821660808501521660a083015260c082015260e00161018f565b6101d461039d3660046135a0565b6119ea565b5f546040516001600160a01b03909116815260200161018f565b61018561546081565b6101d46103d33660046135a0565b611acd565b61041a6103e63660046135a0565b600360208190525f9182526040909120805460018201546002830154929093015490926001600160a01b0390811692911684565b604080519485526001600160a01b03938416602086015284019190915216606082015260800161018f565b5f828152600160205260408120805483111561045f575f5ffd5b5f831161046a575f5ffd5b805460038201545f919061047e9086613959565b610488919061399d565b90505f8111610499576104996139b0565b80826003015410156104ad576104ad6139b0565b80826003015f8282546104c091906139dd565b90915550508154849083905f906104d89084906139dd565b90915550506040805160e0810182528681526020810186905260028401546001600160a01b039081169282019290925260608101839052600484015490911660808201525f9060a0810161052a611be0565b6001600160a01b0316815242602090910152600580549192505f919082610550836139f0565b909155505f818152600260208181526040928390208651815586820151600182015586840151818401805473ffffffffffffffffffffffffffffffffffffffff199081166001600160a01b03938416179091556060808a0151600385015560808a0151600485018054841691851691909117905560a08a01516005850180549093169084161790915560c08901516006909301929092559289015484518c815292830189905290921692810192909252919250829189917fc39a1a5ddc0e85c955fe2e1abeb43c94ce18322e75bb3d44e80f759ff9d034b9910160405180910390a393505050505b92915050565b6060805f805b600554811015610683575f818152600160205260409020600401546001600160a01b03161561067b5781610677816139f0565b9250505b600101610644565b505f8167ffffffffffffffff81111561069e5761069e613a08565b6040519080825280602002602001820160405280156106d757816020015b6106c4613269565b8152602001906001900390816106bc5790505b5090505f8267ffffffffffffffff8111156106f4576106f4613a08565b60405190808252806020026020018201604052801561071d578160200160208202803683370190505b5090505f805b600554811015610894575f818152600160205260409020600401546001600160a01b03161561088c5760015f8281526020019081526020015f206040518060a00160405290815f8201548152602001600182016040518060200160405290815f8201805461079090613a35565b80601f01602080910402602001604051908101604052809291908181526020018280546107bc90613a35565b80156108075780601f106107de57610100808354040283529160200191610807565b820191905f5260205f20905b8154815290600101906020018083116107ea57829003601f168201915b50505091909252505050815260028201546001600160a01b03908116602083015260038301546040830152600490920154909116606090910152845185908490811061085557610855613a80565b60200260200101819052508083838151811061087357610873613a80565b602090810291909101015281610888816139f0565b9250505b600101610723565b50919590945092505050565b5f838152600360205260408120826108b6575f5ffd5b80548311156108c3575f5ffd5b805460028201545f91906108d79086613959565b6108e1919061399d565b90505f81116108f2576108f26139b0565b8082600201541015610906576109066139b0565b80826002015f82825461091991906139dd565b90915550508154849083905f906109319084906139dd565b909155506109589050610942611be0565b60018401546001600160a01b0316903084611c30565b600580545f9182610968836139f0565b9190505590506040518061010001604052808881526020018761098a90613b38565b81526020810187905260018501546001600160a01b03908116604083015260608201859052600386015416608082015260a0016109c5611be0565b6001600160a01b03168152426020918201525f838152600482526040902082518155908201518051600183019081906109fe9082613bdd565b5050506040828101516002830155606083015160038301805473ffffffffffffffffffffffffffffffffffffffff199081166001600160a01b03938416179091556080850151600485015560a0850151600585018054831691841691909117905560c085015160068501805490921690831617905560e0909301516007909201919091556001850154905183928a927f653e0d81f2c99beba359dfb17b499a5cff2be9d950514852224df8c097c2192192610ac3928c928c928a929190911690613d2f565b60405180910390a3925050505b9392505050565b6001600160a01b038216610ae9575f5ffd5b610b06610af4611be0565b6001600160a01b038416903084611c30565b600580545f9182610b16836139f0565b9190505590506040518060a0016040528086815260200185610b3790613b38565b8152602001846001600160a01b03168152602001838152602001610b59611be0565b6001600160a01b031690525f8281526001602081815260409092208351815591830151805190918301908190610b8f9082613bdd565b50505060408281015160028301805473ffffffffffffffffffffffffffffffffffffffff199081166001600160a01b03938416179091556060850151600385015560809094015160049093018054909416921691909117909155517f98c7c680403d47403dea4a570d0e6c5716538c49420ef471cec428f5a5852c0690610c1d908790879087908790613d66565b60405180910390a15050505050565b600160208181525f92835260409283902080548451928301909452918201805482908290610c5990613a35565b80601f0160208091040260200160405190810160405280929190818152602001828054610c8590613a35565b8015610cd05780601f10610ca757610100808354040283529160200191610cd0565b820191905f5260205f20905b815481529060010190602001808311610cb357829003601f168201915b505050919092525050506002820154600383015460049093015491926001600160a01b039182169290911685565b6004602052805f5260405f205f91509050805f015490806001016040518060200160405290815f82018054610d3290613a35565b80601f0160208091040260200160405190810160405280929190818152602001828054610d5e90613a35565b8015610da95780601f10610d8057610100808354040283529160200191610da9565b820191905f5260205f20905b815481529060010190602001808311610d8c57829003601f168201915b5050509190925250505060028201546003830154600484015460058501546006860154600790960154949593946001600160a01b03938416949293918216929091169088565b5f818152600160205260409020610e04611be0565b60048201546001600160a01b03908116911614610e1f575f5ffd5b610e44610e2a611be0565b600383015460028401546001600160a01b03169190611ce7565b5f82815260016020819052604082208281559190820181610e6582826132aa565b50505060028101805473ffffffffffffffffffffffffffffffffffffffff199081169091555f60038301556004909101805490911690556040518281527fc340e7ac48dc80ee793fc6266960bd5f1bd21be91c8a95e218178113f79e17b4906020015b60405180910390a15050565b6001600160a01b038216610ee6575f5ffd5b5f8311610ef1575f5ffd5b5f8111610efc575f5ffd5b600580545f9182610f0c836139f0565b9190505590506040518060800160405280858152602001846001600160a01b03168152602001838152602001610f40611be0565b6001600160a01b039081169091525f83815260036020818152604092839020855181558582015160018201805491871673ffffffffffffffffffffffffffffffffffffffff199283161790558685015160028301556060968701519190930180549186169190931617909155815188815292871690830152810184905282917fff1ce210defcd3ba1adf76c9419a0758fa60fd3eb38c7bd9418f60b575b76e24910160405180910390a250505050565b6060805f805b600554811015611036575f81815260036020819052604090912001546001600160a01b03161561102e578161102a816139f0565b9250505b600101610ff6565b505f8167ffffffffffffffff81111561105157611051613a08565b6040519080825280602002602001820160405280156110a157816020015b604080516080810182525f8082526020808301829052928201819052606082015282525f1990920191018161106f5790505b5090505f8267ffffffffffffffff8111156110be576110be613a08565b6040519080825280602002602001820160405280156110e7578160200160208202803683370190505b5090505f805b600554811015610894575f81815260036020819052604090912001546001600160a01b0316156111ac575f8181526003602081815260409283902083516080810185528154815260018201546001600160a01b039081169382019390935260028201549481019490945290910154166060820152845185908490811061117557611175613a80565b60200260200101819052508083838151811061119357611193613a80565b6020908102919091010152816111a8816139f0565b9250505b6001016110ed565b6060805f805b6005548110156111f0575f81815260026020526040902060010154156111e857816111e4816139f0565b9250505b6001016111ba565b505f8167ffffffffffffffff81111561120b5761120b613a08565b60405190808252806020026020018201604052801561129057816020015b61127d6040518060e001604052805f81526020015f81526020015f6001600160a01b031681526020015f81526020015f6001600160a01b031681526020015f6001600160a01b031681526020015f81525090565b8152602001906001900390816112295790505b5090505f8267ffffffffffffffff8111156112ad576112ad613a08565b6040519080825280602002602001820160405280156112d6578160200160208202803683370190505b5090505f805b600554811015610894575f81815260026020526040902060010154156113b2575f81815260026020818152604092839020835160e08101855281548152600182015492810192909252918201546001600160a01b039081169382019390935260038201546060820152600482015483166080820152600582015490921660a08301526006015460c0820152845185908490811061137b5761137b613a80565b60200260200101819052508083838151811061139957611399613a80565b6020908102919091010152816113ae816139f0565b9250505b6001016112dc565b5f8381526004602052604090206113cf611be0565b60058201546001600160a01b039081169116146113ea575f5ffd5b6006546001600160a01b031663d38c29a16114086040850185613d9a565b6040518363ffffffff1660e01b8152600401611425929190613dfb565b5f604051808303815f87803b15801561143c575f5ffd5b505af115801561144e573d5f5f3e3d5ffd5b5050505061147f6007548461146290613e3d565b61146b85613eeb565b6006546001600160a01b0316929190611d35565b5061149281600201548260010185611ed5565b6005810154600482015460038301546114b9926001600160a01b0391821692911690611ce7565b5f8481526004602052604081208181559060018201816114d982826132aa565b50505f6002830181905560038301805473ffffffffffffffffffffffffffffffffffffffff1990811690915560048401829055600584018054821690556006840180549091169055600790920191909155506040518481527fcf561061db78f7bc518d37fe86718514c640ccc5c3f1293828b955e68f19f5fb9060200160405180910390a150505050565b6060805f805b6005548110156115a0575f81815260046020526040902060020154156115985781611594816139f0565b9250505b60010161156a565b505f8167ffffffffffffffff8111156115bb576115bb613a08565b6040519080825280602002602001820160405280156115f457816020015b6115e16132e4565b8152602001906001900390816115d95790505b5090505f8267ffffffffffffffff81111561161157611611613a08565b60405190808252806020026020018201604052801561163a578160200160208202803683370190505b5090505f805b600554811015610894575f81815260046020526040902060020154156117bf5760045f8281526020019081526020015f20604051806101000160405290815f8201548152602001600182016040518060200160405290815f820180546116a590613a35565b80601f01602080910402602001604051908101604052809291908181526020018280546116d190613a35565b801561171c5780601f106116f35761010080835404028352916020019161171c565b820191905f5260205f20905b8154815290600101906020018083116116ff57829003601f168201915b5050509190925250505081526002820154602082015260038201546001600160a01b0390811660408301526004830154606083015260058301548116608083015260068301541660a082015260079091015460c090910152845185908490811061178857611788613a80565b6020026020010181905250808383815181106117a6576117a6613a80565b6020908102919091010152816117bb816139f0565b9250505b600101611640565b5f8181526003602052604090206117dc611be0565b60038201546001600160a01b039081169116146117f7575f5ffd5b5f82815260036020818152604080842084815560018101805473ffffffffffffffffffffffffffffffffffffffff1990811690915560028201959095559092018054909316909255518381527f3cd475b092e8b379f6ba0d9e0e0c8f30705e73321dc5c9f80ce4ad38db7be1aa9101610ec8565b5f838152600260205260409020611880611be0565b60058201546001600160a01b0390811691161461189b575f5ffd5b6006546001600160a01b031663d38c29a16118b96040850185613d9a565b6040518363ffffffff1660e01b81526004016118d6929190613dfb565b5f604051808303815f87803b1580156118ed575f5ffd5b505af11580156118ff573d5f5f3e3d5ffd5b505050506119136007548461146290613e3d565b5080545f908152600160208190526040909120805490916119379190830186611ed5565b60058201546003830154600284015461195e926001600160a01b0391821692911690611ce7565b5f85815260026020818152604080842084815560018101859055928301805473ffffffffffffffffffffffffffffffffffffffff1990811690915560038401859055600484018054821690556005840180549091169055600690920192909255518681527fb4c98de210696b3cf21e99335c1ee3a0ae34a26713412a4adde8af596176f37e9101610c1d565b5f8181526002602052604090206119ff611be0565b60048201546001600160a01b03908116911614611a1a575f5ffd5b6154608160060154611a2c9190613f81565b4211611a36575f5ffd5b611a41610e2a611be0565b5f82815260026020818152604080842084815560018101859055928301805473ffffffffffffffffffffffffffffffffffffffff1990811690915560038401859055600484018054821690556005840180549091169055600690920192909255518381527f3e5ea358e9eb4cdf44cdc77938ade8074b1240a6d8c0fd13728671b82e800ad69101610ec8565b5f8181526004602052604090206007810154611aec9061546090613f81565b4211611af6575f5ffd5b611afe611be0565b60068201546001600160a01b03908116911614611b19575f5ffd5b611b3e611b24611be0565b600483015460038401546001600160a01b03169190611ce7565b5f828152600460205260408120818155906001820181611b5e82826132aa565b50505f6002830181905560038301805473ffffffffffffffffffffffffffffffffffffffff1990811690915560048401829055600584018054821690556006840180549091169055600790920191909155506040518281527f78f51f62f7cf1381c673c27eae187dd6c588dc6624ce59697dbb3e1d7c1bbcdf90602001610ec8565b5f60143610801590611bfb57505f546001600160a01b031633145b15611c2b57507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffec36013560601c90565b503390565b6040516001600160a01b0380851660248301528316604482015260648101829052611ce19085907f23b872dd00000000000000000000000000000000000000000000000000000000906084015b60408051601f198184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167fffffffff0000000000000000000000000000000000000000000000000000000090931692909217909152611fe7565b50505050565b6040516001600160a01b038316602482015260448101829052611d309084907fa9059cbb0000000000000000000000000000000000000000000000000000000090606401611c7d565b505050565b5f611d4383602001516120cb565b611d945760405162461bcd60e51b815260206004820152601d60248201527f496e76616c696420696e70757420766563746f722070726f766964656400000060448201526064015b60405180910390fd5b611da18360400151612165565b611ded5760405162461bcd60e51b815260206004820152601e60248201527f496e76616c6964206f757470757420766563746f722070726f766964656400006044820152606401611d8b565b611e2a835f0151846020015185604001518660600151604051602001611e169493929190613fab565b6040516020818303038152906040526121f2565b9050611e4c611e3c8360400151612214565b8351602085015184929190612220565b611ebe5760405162461bcd60e51b815260206004820152603c60248201527f5478206d65726b6c652070726f6f66206973206e6f742076616c696420666f7260448201527f2070726f76696465642068656164657220616e642074782068617368000000006064820152608401611d8b565b611ecd8585846040015161225b565b949350505050565b5f825f018054611ee490613a35565b604051611ef69250859060200161401a565b6040516020818303038152906040528051906020012090505f611f5d838060400190611f229190613d9a565b8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f920191909152508692506125ab915050565b5167ffffffffffffffff16905084811015611fe05760405162461bcd60e51b815260206004820152603b60248201527f426974636f696e207472616e73616374696f6e20616d6f756e74206973206c6f60448201527f776572207468616e20696e206163636570746564206f726465722e00000000006064820152608401611d8b565b5050505050565b5f61203b826040518060400160405280602081526020017f5361666545524332303a206c6f772d6c6576656c2063616c6c206661696c6564815250856001600160a01b03166127499092919063ffffffff16565b805190915015611d30578080602001905181019061205991906140e1565b611d305760405162461bcd60e51b815260206004820152602a60248201527f5361666545524332303a204552433230206f7065726174696f6e20646964206e60448201527f6f742073756363656564000000000000000000000000000000000000000000006064820152608401611d8b565b5f5f5f6120d784612757565b90925090508015806120e957505f1982145b156120f757505f9392505050565b5f612103836001613f81565b90505f5b82811015612158578551821061212257505f95945050505050565b5f61212d878461276c565b90505f19810361214357505f9695505050505050565b61214d8184613f81565b925050600101612107565b5093519093149392505050565b5f5f5f61217184612757565b909250905080158061218357505f1982145b1561219157505f9392505050565b5f61219d836001613f81565b90505f5b8281101561215857855182106121bc57505f95945050505050565b5f6121c787846127b2565b90505f1981036121dd57505f9695505050505050565b6121e78184613f81565b9250506001016121a1565b5f60205f83516020850160025afa5060205f60205f60025afa50505f51919050565b60448101515f90610638565b5f838514801561222e575081155b801561223957508251155b1561224657506001611ecd565b61225285848685612812565b95945050505050565b5f836001600160a01b031663113764be6040518163ffffffff1660e01b8152600401602060405180830381865afa158015612298573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906122bc9190614100565b90505f846001600160a01b0316632b97be246040518163ffffffff1660e01b8152600401602060405180830381865afa1580156122fb573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061231f9190614100565b90505f8061233461232f866128b7565b6128c2565b9050838103612345578391506123c2565b828103612354578291506123c2565b60405162461bcd60e51b815260206004820152602560248201527f4e6f742061742063757272656e74206f722070726576696f757320646966666960448201527f63756c74790000000000000000000000000000000000000000000000000000006064820152608401611d8b565b5f6123cc866128e9565b90505f1981036124445760405162461bcd60e51b815260206004820152602360248201527f496e76616c6964206c656e677468206f6620746865206865616465727320636860448201527f61696e00000000000000000000000000000000000000000000000000000000006064820152608401611d8b565b7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81036124b35760405162461bcd60e51b815260206004820152601560248201527f496e76616c6964206865616465727320636861696e00000000000000000000006044820152606401611d8b565b7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffd81036125225760405162461bcd60e51b815260206004820152601d60248201527f496e73756666696369656e7420776f726b20696e2061206865616465720000006044820152606401611d8b565b61252c8784613959565b8110156125a15760405162461bcd60e51b815260206004820152603360248201527f496e73756666696369656e7420616363756d756c61746564206469666669637560448201527f6c747920696e2068656164657220636861696e000000000000000000000000006064820152608401611d8b565b5050505050505050565b604080516060810182525f80825260208083018290528284018290528351808501909452818452830152906125df84612757565b6020830152808252816125f1826139f0565b9052505f805b82602001518110156126f35782515f906126129088906127b2565b84519091505f90612624908990612b0d565b90505f6126326008846139dd565b86519091505f90612644906008613f81565b8a8101602001839020909150808a0361267e576001965083895f0181815161266c9190614117565b67ffffffffffffffff169052506126ce565b5f61268c8c8a5f0151612b83565b90506001600160a01b038116156126ad576001600160a01b03811660208b01525b5f6126bb8d8b5f0151612c63565b905080156126cb5760408b018190525b50505b84885f018181516126df9190613f81565b90525050600190940193506125f792505050565b50806127415760405162461bcd60e51b815260206004820181905260248201527f4e6f206f757470757420666f756e6420666f72207363726970745075624b65796044820152606401611d8b565b505092915050565b6060611ecd84845f85612d43565b5f5f612763835f612e87565b91509150915091565b5f5f5f6127798585613024565b909250905060018201612791575f1992505050610638565b8061279d836025613f81565b6127a79190613f81565b612252906004613f81565b5f6127be826009613f81565b835110156127ce57505f19610638565b5f806127e4856127df866008613f81565b612e87565b9092509050600182016127fc575f1992505050610638565b80612808836009613f81565b6122529190613f81565b5f602084516128219190614137565b1561282d57505f611ecd565b83515f0361283c57505f611ecd565b81855f5b86518110156128aa57612854600284614137565b6001036128785761287161286b8883016020015190565b83613062565b9150612891565b61288e826128898984016020015190565b613062565b91505b60019290921c916128a3602082613f81565b9050612840565b5090931495945050505050565b5f610638825f61306d565b5f6106387bffff000000000000000000000000000000000000000000000000000083613106565b5f605082516128f89190614137565b1561290557505f19919050565b505f80805b8351811015612b0657801561295157612924848284613111565b61295157507ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe9392505050565b5f61295c858361306d565b905061296a8583605061313a565b925080612aad845f8190506008817eff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff16901b600882901c7eff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff161790506010817dffff0000ffff0000ffff0000ffff0000ffff0000ffff0000ffff0000ffff16901b601082901c7dffff0000ffff0000ffff0000ffff0000ffff0000ffff0000ffff0000ffff161790506020817bffffffff00000000ffffffff00000000ffffffff00000000ffffffff16901b602082901c7bffffffff00000000ffffffff00000000ffffffff00000000ffffffff1617905060408177ffffffffffffffff0000000000000000ffffffffffffffff16901b604082901c77ffffffffffffffff0000000000000000ffffffffffffffff16179050608081901b608082901c179050919050565b1115612add57507ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffd949350505050565b612ae6816128c2565b612af09085613f81565b9350612aff9050605082613f81565b905061290a565b5050919050565b5f80612b19848461315f565b60c01c90505f6122528264ff000000ff600882811c91821665ff000000ff009390911b92831617601090811b67ffffffffffffffff1666ff00ff00ff00ff9290921667ff00ff00ff00ff009093169290921790911c65ffff0000ffff1617602081811c91901b1790565b5f82612b90836009613f81565b81518110612ba057612ba0613a80565b6020910101517fff00000000000000000000000000000000000000000000000000000000000000167f6a0000000000000000000000000000000000000000000000000000000000000014612bf557505f610638565b5f83612c0284600a613f81565b81518110612c1257612c12613a80565b01602001517fff000000000000000000000000000000000000000000000000000000000000008116915060f81c601403612c5c575f612c5284600b613f81565b8501601401519250505b5092915050565b5f82612c70836009613f81565b81518110612c8057612c80613a80565b6020910101517fff00000000000000000000000000000000000000000000000000000000000000167f6a0000000000000000000000000000000000000000000000000000000000000014612cd557505f610638565b5f83612ce284600a613f81565b81518110612cf257612cf2613a80565b016020908101517fff000000000000000000000000000000000000000000000000000000000000008116925060f81c9003612c5c575f612d3384600b613f81565b8501602001519250505092915050565b606082471015612dbb5760405162461bcd60e51b815260206004820152602660248201527f416464726573733a20696e73756666696369656e742062616c616e636520666f60448201527f722063616c6c00000000000000000000000000000000000000000000000000006064820152608401611d8b565b6001600160a01b0385163b612e125760405162461bcd60e51b815260206004820152601d60248201527f416464726573733a2063616c6c20746f206e6f6e2d636f6e74726163740000006044820152606401611d8b565b5f5f866001600160a01b03168587604051612e2d919061414a565b5f6040518083038185875af1925050503d805f8114612e67576040519150601f19603f3d011682016040523d82523d5f602084013e612e6c565b606091505b5091509150612e7c82828661316d565b979650505050505050565b5f5f5f612e9485856131a6565b90508060ff165f03612ec7575f858581518110612eb357612eb3613a80565b016020015190935060f81c915061301d9050565b83612ed3826001614155565b60ff16612ee09190613f81565b85511015612ef5575f195f925092505061301d565b5f8160ff16600203612f3857612f2d612f19612f12876001613f81565b889061315f565b62ffff0060e882901c1660f89190911c1790565b61ffff169050613013565b8160ff16600403612f8757612f7a612f54612f12876001613f81565b60d881901c63ff00ff001662ff00ff60e89290921c9190911617601081811b91901c1790565b63ffffffff169050613013565b8160ff1660080361301357613006612fa3612f12876001613f81565b60c01c64ff000000ff600882811c91821665ff000000ff009390911b92831617601090811b67ffffffffffffffff1666ff00ff00ff00ff9290921667ff00ff00ff00ff009093169290921790911c65ffff0000ffff1617602081811c91901b1790565b67ffffffffffffffff1690505b60ff909116925090505b9250929050565b5f80613031836025613f81565b8451101561304457505f1990505f61301d565b5f80613055866127df876024613f81565b9097909650945050505050565b5f610ad0838361322a565b5f8061308461307d846048613f81565b859061315f565b60e81c90505f8461309685604b613f81565b815181106130a6576130a6613a80565b016020015160f81c90505f6130d8835f60108262ffffff16901c8261ff001660108462ffffff16901b17179050919050565b62ffffff1690505f6130eb60038461416e565b60ff1690506130fc8161010061426a565b612e7c9083613959565b5f610ad0828461399d565b5f8061311d8585613251565b905082811461312f575f915050610ad0565b506001949350505050565b5f60205f8385602001870160025afa5060205f60205f60025afa50505f519392505050565b5f610ad08383016020015190565b6060831561317c575081610ad0565b82511561318c5782518084602001fd5b8160405162461bcd60e51b8152600401611d8b9190614275565b5f8282815181106131b9576131b9613a80565b016020015160f81c60ff036131d057506008610638565b8282815181106131e2576131e2613a80565b016020015160f81c60fe036131f957506004610638565b82828151811061320b5761320b613a80565b016020015160f81c60fd0361322257506002610638565b505f92915050565b5f825f528160205260205f60405f60025afa5060205f60205f60025afa50505f5192915050565b5f610ad0613260836004613f81565b84016020015190565b6040518060a001604052805f81526020016132906040518060200160405280606081525090565b81525f602082018190526040820181905260609091015290565b5080546132b690613a35565b5f825580601f106132c5575050565b601f0160209004905f5260205f20908101906132e1919061333b565b50565b6040518061010001604052805f815260200161330c6040518060200160405280606081525090565b81525f6020820181905260408201819052606082018190526080820181905260a0820181905260c09091015290565b5b8082111561334f575f815560010161333c565b5090565b5f5f60408385031215613364575f5ffd5b50508035926020909101359150565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b5f815160208452611ecd6020850182613373565b5f8151808452602084019350602083015f5b828110156133e55781518652602095860195909101906001016133c7565b5093949350505050565b5f604082016040835280855180835260608501915060608160051b8601019250602087015f5b828110156134b1577fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa0878603018452815180518652602081015160a0602088015261346360a08801826133a1565b90506001600160a01b036040830151166040880152606082015160608801526001600160a01b0360808301511660808801528096505050602082019150602084019350600181019050613415565b50505050828103602084015261225281856133b5565b5f602082840312156134d7575f5ffd5b50919050565b5f5f5f606084860312156134ef575f5ffd5b83359250602084013567ffffffffffffffff81111561350c575f5ffd5b613518868287016134c7565b93969395505050506040919091013590565b80356001600160a01b0381168114613540575f5ffd5b919050565b5f5f5f5f60808587031215613558575f5ffd5b84359350602085013567ffffffffffffffff811115613575575f5ffd5b613581878288016134c7565b9350506135906040860161352a565b9396929550929360600135925050565b5f602082840312156135b0575f5ffd5b5035919050565b85815260a060208201525f6135cf60a08301876133a1565b90506001600160a01b03851660408301528360608301526001600160a01b03831660808301529695505050505050565b88815261010060208201525f61361961010083018a6133a1565b6040830198909852506001600160a01b039586166060820152608081019490945291841660a084015290921660c082015260e0015292915050565b5f60208284031215613664575f5ffd5b610ad08261352a565b5f5f5f6060848603121561367f575f5ffd5b8335925061368f6020850161352a565b929592945050506040919091013590565b604080825283519082018190525f9060208501906060840190835b81811015613711578351805184526001600160a01b036020820151166020850152604081015160408501526001600160a01b036060820151166060850152506080830192506020840193506001810190506136bb565b5050838103602085015261372581866133b5565b9695505050505050565b604080825283519082018190525f9060208501906060840190835b8181101561371157835180518452602081015160208501526001600160a01b036040820151166040850152606081015160608501526001600160a01b0360808201511660808501526001600160a01b0360a08201511660a085015260c081015160c08501525060e08301925060208401935060018101905061374a565b5f5f5f606084860312156137d9575f5ffd5b83359250602084013567ffffffffffffffff8111156137f6575f5ffd5b840160808187031215613807575f5ffd5b9150604084013567ffffffffffffffff811115613822575f5ffd5b840160608187031215613833575f5ffd5b809150509250925092565b5f604082016040835280855180835260608501915060608160051b8601019250602087015f5b828110156134b1577fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa0878603018452815180518652602081015161010060208801526138b46101008801826133a1565b9050604082015160408801526001600160a01b036060830151166060880152608082015160808801526001600160a01b0360a08301511660a088015260c082015161390a60c08901826001600160a01b03169052565b5060e09182015196909101959095526020938401939190910190600101613864565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b80820281158282048414176106385761063861392c565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f826139ab576139ab613970565b500490565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52600160045260245ffd5b818103818111156106385761063861392c565b5f5f198203613a0157613a0161392c565b5060010190565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b600181811c90821680613a4957607f821691505b6020821081036134d7577f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f82601f830112613abc575f5ffd5b813567ffffffffffffffff811115613ad657613ad6613a08565b604051601f8201601f19908116603f0116810167ffffffffffffffff81118282101715613b0557613b05613a08565b604052818152838201602001851015613b1c575f5ffd5b816020850160208301375f918101602001919091529392505050565b5f60208236031215613b48575f5ffd5b6040516020810167ffffffffffffffff81118282101715613b6b57613b6b613a08565b604052823567ffffffffffffffff811115613b84575f5ffd5b613b9036828601613aad565b82525092915050565b601f821115611d3057805f5260205f20601f840160051c81016020851015613bbe5750805b601f840160051c820191505b81811015611fe0575f8155600101613bca565b815167ffffffffffffffff811115613bf757613bf7613a08565b613c0b81613c058454613a35565b84613b99565b6020601f821160018114613c3d575f8315613c265750848201515b5f19600385901b1c1916600184901b178455611fe0565b5f84815260208120601f198516915b82811015613c6c5787850151825560209485019460019092019101613c4c565b5084821015613c8957868401515f19600387901b60f8161c191681555b50505050600190811b01905550565b81835281816020850137505f602082840101525f6020601f19601f840116840101905092915050565b5f81357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe1833603018112613cf3575f5ffd5b820160208101903567ffffffffffffffff811115613d0f575f5ffd5b803603821315613d1d575f5ffd5b60208552612252602086018284613c98565b608081525f613d416080830187613cc1565b60208301959095525060408101929092526001600160a01b0316606090910152919050565b848152608060208201525f613d7e6080830186613cc1565b6001600160a01b03949094166040830152506060015292915050565b5f5f83357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe1843603018112613dcd575f5ffd5b83018035915067ffffffffffffffff821115613de7575f5ffd5b60200191503681900382131561301d575f5ffd5b602081525f611ecd602083018486613c98565b80357fffffffff0000000000000000000000000000000000000000000000000000000081168114613540575f5ffd5b5f60808236031215613e4d575f5ffd5b6040516080810167ffffffffffffffff81118282101715613e7057613e70613a08565b604052613e7c83613e0e565b8152602083013567ffffffffffffffff811115613e97575f5ffd5b613ea336828601613aad565b602083015250604083013567ffffffffffffffff811115613ec2575f5ffd5b613ece36828601613aad565b604083015250613ee060608401613e0e565b606082015292915050565b5f60608236031215613efb575f5ffd5b6040516060810167ffffffffffffffff81118282101715613f1e57613f1e613a08565b604052823567ffffffffffffffff811115613f37575f5ffd5b613f4336828601613aad565b82525060208381013590820152604083013567ffffffffffffffff811115613f69575f5ffd5b613f7536828601613aad565b60408301525092915050565b808201808211156106385761063861392c565b5f81518060208401855e5f93019283525090919050565b7fffffffff00000000000000000000000000000000000000000000000000000000851681525f613fe7613fe16004840187613f94565b85613f94565b7fffffffff0000000000000000000000000000000000000000000000000000000093909316835250506004019392505050565b7fff000000000000000000000000000000000000000000000000000000000000008360f81b1681525f5f835461404f81613a35565b600182168015614066576001811461409f576140d5565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00831660018701526001821515830287010193506140d5565b865f5260205f205f5b838110156140ca5781546001828a0101526001820191506020810190506140a8565b505060018287010193505b50919695505050505050565b5f602082840312156140f1575f5ffd5b81518015158114610ad0575f5ffd5b5f60208284031215614110575f5ffd5b5051919050565b67ffffffffffffffff81811683821601908111156106385761063861392c565b5f8261414557614145613970565b500690565b5f610ad08284613f94565b60ff81811683821601908111156106385761063861392c565b60ff82811682821603908111156106385761063861392c565b6001815b60018411156141c2578085048111156141a6576141a661392c565b60018416156141b457908102905b60019390931c92800261418b565b935093915050565b5f826141d857506001610638565b816141e457505f610638565b81600181146141fa576002811461420457614220565b6001915050610638565b60ff8411156142155761421561392c565b50506001821b610638565b5060208310610133831016604e8410600b8410161715614243575081810a610638565b61424f5f198484614187565b805f19048211156142625761426261392c565b029392505050565b5f610ad083836141ca565b602081525f610ad0602083018461337356fea264697066735822122031f69b286db991c7beeb2413733418fc26ca867d0fb291fb51a46c037e52769864736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15`\x0EW__\xFD[P`@QaC~8\x03\x80aC~\x839\x81\x01`@\x81\x90R`+\x91`\x81V[_\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x17\x90U`\x06\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x17\x90UPP`\x01`\x07U`\xB4V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14`~W__\xFD[PV[__`@\x83\x85\x03\x12\x15`\x91W__\xFD[\x82Q`\x9A\x81`kV[` \x84\x01Q\x90\x92P`\xA9\x81`kV[\x80\x91PP\x92P\x92\x90PV[aB\xBD\x80a\0\xC1_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x01nW_5`\xE0\x1C\x80cj\x8C\xDE:\x11a\0\xD2W\x80c\xBD*~>\x11a\0\x88W\x80c\xD1\x92\x0F\xF0\x11a\0cW\x80c\xD1\x92\x0F\xF0\x14a\x03\xBCW\x80c\xDFi\xB1O\x14a\x03\xC5W\x80c\xEC\xCA,6\x14a\x03\xD8W__\xFD[\x80c\xBD*~>\x14a\x02\xF3W\x80c\xC5jE&\x14a\x03\x8FW\x80c\xCE\x1B\x81_\x14a\x03\xA2W__\xFD[\x80c\x9C\xC6r.\x11a\0\xB8W\x80c\x9C\xC6r.\x14a\x02\xB7W\x80c\xA3\x83\x01;\x14a\x02\xCDW\x80c\xB5\"\xC13\x14a\x02\xE0W__\xFD[\x80cj\x8C\xDE:\x14a\x02\x8EW\x80cr\xA5F\xC6\x14a\x02\xA4W__\xFD[\x80cAEd\n\x11a\x01'W\x80cW+l\x05\x11a\x01\rW\x80cW+l\x05\x14a\x024W\x80c[\x8F\xE0B\x14a\x02eW\x80ch\x11\xA3\x11\x14a\x02xW__\xFD[\x80cAEd\n\x14a\x01\xFAW\x80cPj\x10\x9D\x14a\x02!W__\xFD[\x80c!\x0E\xC1\x81\x11a\x01WW\x80c!\x0E\xC1\x81\x14a\x01\xAEW\x80c6O\x1E\xC0\x14a\x01\xC1W\x80c:\xF3\xFC~\x14a\x01\xD6W__\xFD[\x80c\x11\xC17\xAA\x14a\x01rW\x80c\x1D\xFEu\x95\x14a\x01\x98W[__\xFD[a\x01\x85a\x01\x806`\x04a3SV[a\x04EV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\xA0a\x06>V[`@Qa\x01\x8F\x92\x91\x90a3\xEFV[a\x01\x85a\x01\xBC6`\x04a4\xDDV[a\x08\xA0V[a\x01\xD4a\x01\xCF6`\x04a5EV[a\n\xD7V[\0[a\x01\xE9a\x01\xE46`\x04a5\xA0V[a\x0C,V[`@Qa\x01\x8F\x95\x94\x93\x92\x91\x90a5\xB7V[a\x02\ra\x02\x086`\x04a5\xA0V[a\x0C\xFEV[`@Qa\x01\x8F\x98\x97\x96\x95\x94\x93\x92\x91\x90a5\xFFV[a\x01\xD4a\x02/6`\x04a5\xA0V[a\r\xEFV[a\x02Ua\x02B6`\x04a6TV[_T`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x91\x16\x14\x90V[`@Q\x90\x15\x15\x81R` \x01a\x01\x8FV[a\x01\xD4a\x02s6`\x04a6mV[a\x0E\xD4V[a\x02\x80a\x0F\xF0V[`@Qa\x01\x8F\x92\x91\x90a6\xA0V[a\x02\x96a\x11\xB4V[`@Qa\x01\x8F\x92\x91\x90a7/V[a\x01\xD4a\x02\xB26`\x04a7\xC7V[a\x13\xBAV[a\x02\xBFa\x15dV[`@Qa\x01\x8F\x92\x91\x90a8>V[a\x01\xD4a\x02\xDB6`\x04a5\xA0V[a\x17\xC7V[a\x01\xD4a\x02\xEE6`\x04a7\xC7V[a\x18kV[a\x03La\x03\x016`\x04a5\xA0V[`\x02` \x81\x90R_\x91\x82R`@\x90\x91 \x80T`\x01\x82\x01T\x92\x82\x01T`\x03\x83\x01T`\x04\x84\x01T`\x05\x85\x01T`\x06\x90\x95\x01T\x93\x95\x94`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x94\x92\x93\x91\x82\x16\x92\x91\x16\x90\x87V[`@\x80Q\x97\x88R` \x88\x01\x96\x90\x96R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x95\x87\x01\x95\x90\x95R``\x86\x01\x92\x90\x92R\x82\x16`\x80\x85\x01R\x16`\xA0\x83\x01R`\xC0\x82\x01R`\xE0\x01a\x01\x8FV[a\x01\xD4a\x03\x9D6`\x04a5\xA0V[a\x19\xEAV[_T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\x8FV[a\x01\x85aT`\x81V[a\x01\xD4a\x03\xD36`\x04a5\xA0V[a\x1A\xCDV[a\x04\x1Aa\x03\xE66`\x04a5\xA0V[`\x03` \x81\x90R_\x91\x82R`@\x90\x91 \x80T`\x01\x82\x01T`\x02\x83\x01T\x92\x90\x93\x01T\x90\x92`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x92\x91\x16\x84V[`@\x80Q\x94\x85R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16` \x86\x01R\x84\x01\x91\x90\x91R\x16``\x82\x01R`\x80\x01a\x01\x8FV[_\x82\x81R`\x01` R`@\x81 \x80T\x83\x11\x15a\x04_W__\xFD[_\x83\x11a\x04jW__\xFD[\x80T`\x03\x82\x01T_\x91\x90a\x04~\x90\x86a9YV[a\x04\x88\x91\x90a9\x9DV[\x90P_\x81\x11a\x04\x99Wa\x04\x99a9\xB0V[\x80\x82`\x03\x01T\x10\x15a\x04\xADWa\x04\xADa9\xB0V[\x80\x82`\x03\x01_\x82\x82Ta\x04\xC0\x91\x90a9\xDDV[\x90\x91UPP\x81T\x84\x90\x83\x90_\x90a\x04\xD8\x90\x84\x90a9\xDDV[\x90\x91UPP`@\x80Q`\xE0\x81\x01\x82R\x86\x81R` \x81\x01\x86\x90R`\x02\x84\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x83\x90R`\x04\x84\x01T\x90\x91\x16`\x80\x82\x01R_\x90`\xA0\x81\x01a\x05*a\x1B\xE0V[`\x01`\x01`\xA0\x1B\x03\x16\x81RB` \x90\x91\x01R`\x05\x80T\x91\x92P_\x91\x90\x82a\x05P\x83a9\xF0V[\x90\x91UP_\x81\x81R`\x02` \x81\x81R`@\x92\x83\x90 \x86Q\x81U\x86\x82\x01Q`\x01\x82\x01U\x86\x84\x01Q\x81\x84\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x17\x90\x91U``\x80\x8A\x01Q`\x03\x85\x01U`\x80\x8A\x01Q`\x04\x85\x01\x80T\x84\x16\x91\x85\x16\x91\x90\x91\x17\x90U`\xA0\x8A\x01Q`\x05\x85\x01\x80T\x90\x93\x16\x90\x84\x16\x17\x90\x91U`\xC0\x89\x01Q`\x06\x90\x93\x01\x92\x90\x92U\x92\x89\x01T\x84Q\x8C\x81R\x92\x83\x01\x89\x90R\x90\x92\x16\x92\x81\x01\x92\x90\x92R\x91\x92P\x82\x91\x89\x91\x7F\xC3\x9A\x1A]\xDC\x0E\x85\xC9U\xFE.\x1A\xBE\xB4<\x94\xCE\x182.u\xBB=D\xE8\x0Fu\x9F\xF9\xD04\xB9\x91\x01`@Q\x80\x91\x03\x90\xA3\x93PPPP[\x92\x91PPV[``\x80_\x80[`\x05T\x81\x10\x15a\x06\x83W_\x81\x81R`\x01` R`@\x90 `\x04\x01T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x06{W\x81a\x06w\x81a9\xF0V[\x92PP[`\x01\x01a\x06DV[P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06\x9EWa\x06\x9Ea:\x08V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x06\xD7W\x81` \x01[a\x06\xC4a2iV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x06\xBCW\x90P[P\x90P_\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06\xF4Wa\x06\xF4a:\x08V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x07\x1DW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_\x80[`\x05T\x81\x10\x15a\x08\x94W_\x81\x81R`\x01` R`@\x90 `\x04\x01T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x08\x8CW`\x01_\x82\x81R` \x01\x90\x81R` \x01_ `@Q\x80`\xA0\x01`@R\x90\x81_\x82\x01T\x81R` \x01`\x01\x82\x01`@Q\x80` \x01`@R\x90\x81_\x82\x01\x80Ta\x07\x90\x90a:5V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x07\xBC\x90a:5V[\x80\x15a\x08\x07W\x80`\x1F\x10a\x07\xDEWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x08\x07V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x07\xEAW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x90\x92RPPP\x81R`\x02\x82\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16` \x83\x01R`\x03\x83\x01T`@\x83\x01R`\x04\x90\x92\x01T\x90\x91\x16``\x90\x91\x01R\x84Q\x85\x90\x84\x90\x81\x10a\x08UWa\x08Ua:\x80V[` \x02` \x01\x01\x81\x90RP\x80\x83\x83\x81Q\x81\x10a\x08sWa\x08sa:\x80V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x81a\x08\x88\x81a9\xF0V[\x92PP[`\x01\x01a\x07#V[P\x91\x95\x90\x94P\x92PPPV[_\x83\x81R`\x03` R`@\x81 \x82a\x08\xB6W__\xFD[\x80T\x83\x11\x15a\x08\xC3W__\xFD[\x80T`\x02\x82\x01T_\x91\x90a\x08\xD7\x90\x86a9YV[a\x08\xE1\x91\x90a9\x9DV[\x90P_\x81\x11a\x08\xF2Wa\x08\xF2a9\xB0V[\x80\x82`\x02\x01T\x10\x15a\t\x06Wa\t\x06a9\xB0V[\x80\x82`\x02\x01_\x82\x82Ta\t\x19\x91\x90a9\xDDV[\x90\x91UPP\x81T\x84\x90\x83\x90_\x90a\t1\x90\x84\x90a9\xDDV[\x90\x91UPa\tX\x90Pa\tBa\x1B\xE0V[`\x01\x84\x01T`\x01`\x01`\xA0\x1B\x03\x16\x900\x84a\x1C0V[`\x05\x80T_\x91\x82a\th\x83a9\xF0V[\x91\x90PU\x90P`@Q\x80a\x01\0\x01`@R\x80\x88\x81R` \x01\x87a\t\x8A\x90a;8V[\x81R` \x81\x01\x87\x90R`\x01\x85\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`@\x83\x01R``\x82\x01\x85\x90R`\x03\x86\x01T\x16`\x80\x82\x01R`\xA0\x01a\t\xC5a\x1B\xE0V[`\x01`\x01`\xA0\x1B\x03\x16\x81RB` \x91\x82\x01R_\x83\x81R`\x04\x82R`@\x90 \x82Q\x81U\x90\x82\x01Q\x80Q`\x01\x83\x01\x90\x81\x90a\t\xFE\x90\x82a;\xDDV[PPP`@\x82\x81\x01Q`\x02\x83\x01U``\x83\x01Q`\x03\x83\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x17\x90\x91U`\x80\x85\x01Q`\x04\x85\x01U`\xA0\x85\x01Q`\x05\x85\x01\x80T\x83\x16\x91\x84\x16\x91\x90\x91\x17\x90U`\xC0\x85\x01Q`\x06\x85\x01\x80T\x90\x92\x16\x90\x83\x16\x17\x90U`\xE0\x90\x93\x01Q`\x07\x90\x92\x01\x91\x90\x91U`\x01\x85\x01T\x90Q\x83\x92\x8A\x92\x7Fe>\r\x81\xF2\xC9\x9B\xEB\xA3Y\xDF\xB1{I\x9A\\\xFF+\xE9\xD9PQHR\"M\xF8\xC0\x97\xC2\x19!\x92a\n\xC3\x92\x8C\x92\x8C\x92\x8A\x92\x91\x90\x91\x16\x90a=/V[`@Q\x80\x91\x03\x90\xA3\x92PPP[\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\n\xE9W__\xFD[a\x0B\x06a\n\xF4a\x1B\xE0V[`\x01`\x01`\xA0\x1B\x03\x84\x16\x900\x84a\x1C0V[`\x05\x80T_\x91\x82a\x0B\x16\x83a9\xF0V[\x91\x90PU\x90P`@Q\x80`\xA0\x01`@R\x80\x86\x81R` \x01\x85a\x0B7\x90a;8V[\x81R` \x01\x84`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83\x81R` \x01a\x0BYa\x1B\xE0V[`\x01`\x01`\xA0\x1B\x03\x16\x90R_\x82\x81R`\x01` \x81\x81R`@\x90\x92 \x83Q\x81U\x91\x83\x01Q\x80Q\x90\x91\x83\x01\x90\x81\x90a\x0B\x8F\x90\x82a;\xDDV[PPP`@\x82\x81\x01Q`\x02\x83\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x17\x90\x91U``\x85\x01Q`\x03\x85\x01U`\x80\x90\x94\x01Q`\x04\x90\x93\x01\x80T\x90\x94\x16\x92\x16\x91\x90\x91\x17\x90\x91UQ\x7F\x98\xC7\xC6\x80@=G@=\xEAJW\r\x0ElW\x16S\x8CIB\x0E\xF4q\xCE\xC4(\xF5\xA5\x85,\x06\x90a\x0C\x1D\x90\x87\x90\x87\x90\x87\x90\x87\x90a=fV[`@Q\x80\x91\x03\x90\xA1PPPPPV[`\x01` \x81\x81R_\x92\x83R`@\x92\x83\x90 \x80T\x84Q\x92\x83\x01\x90\x94R\x91\x82\x01\x80T\x82\x90\x82\x90a\x0CY\x90a:5V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0C\x85\x90a:5V[\x80\x15a\x0C\xD0W\x80`\x1F\x10a\x0C\xA7Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0C\xD0V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0C\xB3W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x90\x92RPPP`\x02\x82\x01T`\x03\x83\x01T`\x04\x90\x93\x01T\x91\x92`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x92\x90\x91\x16\x85V[`\x04` R\x80_R`@_ _\x91P\x90P\x80_\x01T\x90\x80`\x01\x01`@Q\x80` \x01`@R\x90\x81_\x82\x01\x80Ta\r2\x90a:5V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\r^\x90a:5V[\x80\x15a\r\xA9W\x80`\x1F\x10a\r\x80Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\r\xA9V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\r\x8CW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x90\x92RPPP`\x02\x82\x01T`\x03\x83\x01T`\x04\x84\x01T`\x05\x85\x01T`\x06\x86\x01T`\x07\x90\x96\x01T\x94\x95\x93\x94`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x94\x92\x93\x91\x82\x16\x92\x90\x91\x16\x90\x88V[_\x81\x81R`\x01` R`@\x90 a\x0E\x04a\x1B\xE0V[`\x04\x82\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x16\x14a\x0E\x1FW__\xFD[a\x0EDa\x0E*a\x1B\xE0V[`\x03\x83\x01T`\x02\x84\x01T`\x01`\x01`\xA0\x1B\x03\x16\x91\x90a\x1C\xE7V[_\x82\x81R`\x01` \x81\x90R`@\x82 \x82\x81U\x91\x90\x82\x01\x81a\x0Ee\x82\x82a2\xAAV[PPP`\x02\x81\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x81\x16\x90\x91U_`\x03\x83\x01U`\x04\x90\x91\x01\x80T\x90\x91\x16\x90U`@Q\x82\x81R\x7F\xC3@\xE7\xACH\xDC\x80\xEEy?\xC6&i`\xBD_\x1B\xD2\x1B\xE9\x1C\x8A\x95\xE2\x18\x17\x81\x13\xF7\x9E\x17\xB4\x90` \x01[`@Q\x80\x91\x03\x90\xA1PPV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x0E\xE6W__\xFD[_\x83\x11a\x0E\xF1W__\xFD[_\x81\x11a\x0E\xFCW__\xFD[`\x05\x80T_\x91\x82a\x0F\x0C\x83a9\xF0V[\x91\x90PU\x90P`@Q\x80`\x80\x01`@R\x80\x85\x81R` \x01\x84`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83\x81R` \x01a\x0F@a\x1B\xE0V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x90\x91R_\x83\x81R`\x03` \x81\x81R`@\x92\x83\x90 \x85Q\x81U\x85\x82\x01Q`\x01\x82\x01\x80T\x91\x87\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x92\x83\x16\x17\x90U\x86\x85\x01Q`\x02\x83\x01U``\x96\x87\x01Q\x91\x90\x93\x01\x80T\x91\x86\x16\x91\x90\x93\x16\x17\x90\x91U\x81Q\x88\x81R\x92\x87\x16\x90\x83\x01R\x81\x01\x84\x90R\x82\x91\x7F\xFF\x1C\xE2\x10\xDE\xFC\xD3\xBA\x1A\xDFv\xC9A\x9A\x07X\xFA`\xFD>\xB3\x8C{\xD9A\x8F`\xB5u\xB7n$\x91\x01`@Q\x80\x91\x03\x90\xA2PPPPV[``\x80_\x80[`\x05T\x81\x10\x15a\x106W_\x81\x81R`\x03` \x81\x90R`@\x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x10.W\x81a\x10*\x81a9\xF0V[\x92PP[`\x01\x01a\x0F\xF6V[P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x10QWa\x10Qa:\x08V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x10\xA1W\x81` \x01[`@\x80Q`\x80\x81\x01\x82R_\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01\x81\x90R``\x82\x01R\x82R_\x19\x90\x92\x01\x91\x01\x81a\x10oW\x90P[P\x90P_\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x10\xBEWa\x10\xBEa:\x08V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x10\xE7W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_\x80[`\x05T\x81\x10\x15a\x08\x94W_\x81\x81R`\x03` \x81\x90R`@\x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x11\xACW_\x81\x81R`\x03` \x81\x81R`@\x92\x83\x90 \x83Q`\x80\x81\x01\x85R\x81T\x81R`\x01\x82\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x93\x82\x01\x93\x90\x93R`\x02\x82\x01T\x94\x81\x01\x94\x90\x94R\x90\x91\x01T\x16``\x82\x01R\x84Q\x85\x90\x84\x90\x81\x10a\x11uWa\x11ua:\x80V[` \x02` \x01\x01\x81\x90RP\x80\x83\x83\x81Q\x81\x10a\x11\x93Wa\x11\x93a:\x80V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x81a\x11\xA8\x81a9\xF0V[\x92PP[`\x01\x01a\x10\xEDV[``\x80_\x80[`\x05T\x81\x10\x15a\x11\xF0W_\x81\x81R`\x02` R`@\x90 `\x01\x01T\x15a\x11\xE8W\x81a\x11\xE4\x81a9\xF0V[\x92PP[`\x01\x01a\x11\xBAV[P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x12\x0BWa\x12\x0Ba:\x08V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x12\x90W\x81` \x01[a\x12}`@Q\x80`\xE0\x01`@R\x80_\x81R` \x01_\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x12)W\x90P[P\x90P_\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x12\xADWa\x12\xADa:\x08V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x12\xD6W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_\x80[`\x05T\x81\x10\x15a\x08\x94W_\x81\x81R`\x02` R`@\x90 `\x01\x01T\x15a\x13\xB2W_\x81\x81R`\x02` \x81\x81R`@\x92\x83\x90 \x83Q`\xE0\x81\x01\x85R\x81T\x81R`\x01\x82\x01T\x92\x81\x01\x92\x90\x92R\x91\x82\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x93\x82\x01\x93\x90\x93R`\x03\x82\x01T``\x82\x01R`\x04\x82\x01T\x83\x16`\x80\x82\x01R`\x05\x82\x01T\x90\x92\x16`\xA0\x83\x01R`\x06\x01T`\xC0\x82\x01R\x84Q\x85\x90\x84\x90\x81\x10a\x13{Wa\x13{a:\x80V[` \x02` \x01\x01\x81\x90RP\x80\x83\x83\x81Q\x81\x10a\x13\x99Wa\x13\x99a:\x80V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x81a\x13\xAE\x81a9\xF0V[\x92PP[`\x01\x01a\x12\xDCV[_\x83\x81R`\x04` R`@\x90 a\x13\xCFa\x1B\xE0V[`\x05\x82\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x16\x14a\x13\xEAW__\xFD[`\x06T`\x01`\x01`\xA0\x1B\x03\x16c\xD3\x8C)\xA1a\x14\x08`@\x85\x01\x85a=\x9AV[`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x14%\x92\x91\x90a=\xFBV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x14<W__\xFD[PZ\xF1\x15\x80\x15a\x14NW=__>=_\xFD[PPPPa\x14\x7F`\x07T\x84a\x14b\x90a>=V[a\x14k\x85a>\xEBV[`\x06T`\x01`\x01`\xA0\x1B\x03\x16\x92\x91\x90a\x1D5V[Pa\x14\x92\x81`\x02\x01T\x82`\x01\x01\x85a\x1E\xD5V[`\x05\x81\x01T`\x04\x82\x01T`\x03\x83\x01Ta\x14\xB9\x92`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x92\x91\x16\x90a\x1C\xE7V[_\x84\x81R`\x04` R`@\x81 \x81\x81U\x90`\x01\x82\x01\x81a\x14\xD9\x82\x82a2\xAAV[PP_`\x02\x83\x01\x81\x90U`\x03\x83\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x81\x16\x90\x91U`\x04\x84\x01\x82\x90U`\x05\x84\x01\x80T\x82\x16\x90U`\x06\x84\x01\x80T\x90\x91\x16\x90U`\x07\x90\x92\x01\x91\x90\x91UP`@Q\x84\x81R\x7F\xCFV\x10a\xDBx\xF7\xBCQ\x8D7\xFE\x86q\x85\x14\xC6@\xCC\xC5\xC3\xF1)8(\xB9U\xE6\x8F\x19\xF5\xFB\x90` \x01`@Q\x80\x91\x03\x90\xA1PPPPV[``\x80_\x80[`\x05T\x81\x10\x15a\x15\xA0W_\x81\x81R`\x04` R`@\x90 `\x02\x01T\x15a\x15\x98W\x81a\x15\x94\x81a9\xF0V[\x92PP[`\x01\x01a\x15jV[P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x15\xBBWa\x15\xBBa:\x08V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x15\xF4W\x81` \x01[a\x15\xE1a2\xE4V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x15\xD9W\x90P[P\x90P_\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x16\x11Wa\x16\x11a:\x08V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x16:W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_\x80[`\x05T\x81\x10\x15a\x08\x94W_\x81\x81R`\x04` R`@\x90 `\x02\x01T\x15a\x17\xBFW`\x04_\x82\x81R` \x01\x90\x81R` \x01_ `@Q\x80a\x01\0\x01`@R\x90\x81_\x82\x01T\x81R` \x01`\x01\x82\x01`@Q\x80` \x01`@R\x90\x81_\x82\x01\x80Ta\x16\xA5\x90a:5V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x16\xD1\x90a:5V[\x80\x15a\x17\x1CW\x80`\x1F\x10a\x16\xF3Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x17\x1CV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x16\xFFW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x90\x92RPPP\x81R`\x02\x82\x01T` \x82\x01R`\x03\x82\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`@\x83\x01R`\x04\x83\x01T``\x83\x01R`\x05\x83\x01T\x81\x16`\x80\x83\x01R`\x06\x83\x01T\x16`\xA0\x82\x01R`\x07\x90\x91\x01T`\xC0\x90\x91\x01R\x84Q\x85\x90\x84\x90\x81\x10a\x17\x88Wa\x17\x88a:\x80V[` \x02` \x01\x01\x81\x90RP\x80\x83\x83\x81Q\x81\x10a\x17\xA6Wa\x17\xA6a:\x80V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x81a\x17\xBB\x81a9\xF0V[\x92PP[`\x01\x01a\x16@V[_\x81\x81R`\x03` R`@\x90 a\x17\xDCa\x1B\xE0V[`\x03\x82\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x16\x14a\x17\xF7W__\xFD[_\x82\x81R`\x03` \x81\x81R`@\x80\x84 \x84\x81U`\x01\x81\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x81\x16\x90\x91U`\x02\x82\x01\x95\x90\x95U\x90\x92\x01\x80T\x90\x93\x16\x90\x92UQ\x83\x81R\x7F<\xD4u\xB0\x92\xE8\xB3y\xF6\xBA\r\x9E\x0E\x0C\x8F0p^s2\x1D\xC5\xC9\xF8\x0C\xE4\xAD8\xDB{\xE1\xAA\x91\x01a\x0E\xC8V[_\x83\x81R`\x02` R`@\x90 a\x18\x80a\x1B\xE0V[`\x05\x82\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x16\x14a\x18\x9BW__\xFD[`\x06T`\x01`\x01`\xA0\x1B\x03\x16c\xD3\x8C)\xA1a\x18\xB9`@\x85\x01\x85a=\x9AV[`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x18\xD6\x92\x91\x90a=\xFBV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x18\xEDW__\xFD[PZ\xF1\x15\x80\x15a\x18\xFFW=__>=_\xFD[PPPPa\x19\x13`\x07T\x84a\x14b\x90a>=V[P\x80T_\x90\x81R`\x01` \x81\x90R`@\x90\x91 \x80T\x90\x91a\x197\x91\x90\x83\x01\x86a\x1E\xD5V[`\x05\x82\x01T`\x03\x83\x01T`\x02\x84\x01Ta\x19^\x92`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x92\x91\x16\x90a\x1C\xE7V[_\x85\x81R`\x02` \x81\x81R`@\x80\x84 \x84\x81U`\x01\x81\x01\x85\x90U\x92\x83\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x81\x16\x90\x91U`\x03\x84\x01\x85\x90U`\x04\x84\x01\x80T\x82\x16\x90U`\x05\x84\x01\x80T\x90\x91\x16\x90U`\x06\x90\x92\x01\x92\x90\x92UQ\x86\x81R\x7F\xB4\xC9\x8D\xE2\x10ik<\xF2\x1E\x993\\\x1E\xE3\xA0\xAE4\xA2g\x13A*J\xDD\xE8\xAFYav\xF3~\x91\x01a\x0C\x1DV[_\x81\x81R`\x02` R`@\x90 a\x19\xFFa\x1B\xE0V[`\x04\x82\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x16\x14a\x1A\x1AW__\xFD[aT`\x81`\x06\x01Ta\x1A,\x91\x90a?\x81V[B\x11a\x1A6W__\xFD[a\x1AAa\x0E*a\x1B\xE0V[_\x82\x81R`\x02` \x81\x81R`@\x80\x84 \x84\x81U`\x01\x81\x01\x85\x90U\x92\x83\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x81\x16\x90\x91U`\x03\x84\x01\x85\x90U`\x04\x84\x01\x80T\x82\x16\x90U`\x05\x84\x01\x80T\x90\x91\x16\x90U`\x06\x90\x92\x01\x92\x90\x92UQ\x83\x81R\x7F>^\xA3X\xE9\xEBL\xDFD\xCD\xC7y8\xAD\xE8\x07K\x12@\xA6\xD8\xC0\xFD\x13r\x86q\xB8.\x80\n\xD6\x91\x01a\x0E\xC8V[_\x81\x81R`\x04` R`@\x90 `\x07\x81\x01Ta\x1A\xEC\x90aT`\x90a?\x81V[B\x11a\x1A\xF6W__\xFD[a\x1A\xFEa\x1B\xE0V[`\x06\x82\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x16\x14a\x1B\x19W__\xFD[a\x1B>a\x1B$a\x1B\xE0V[`\x04\x83\x01T`\x03\x84\x01T`\x01`\x01`\xA0\x1B\x03\x16\x91\x90a\x1C\xE7V[_\x82\x81R`\x04` R`@\x81 \x81\x81U\x90`\x01\x82\x01\x81a\x1B^\x82\x82a2\xAAV[PP_`\x02\x83\x01\x81\x90U`\x03\x83\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x81\x16\x90\x91U`\x04\x84\x01\x82\x90U`\x05\x84\x01\x80T\x82\x16\x90U`\x06\x84\x01\x80T\x90\x91\x16\x90U`\x07\x90\x92\x01\x91\x90\x91UP`@Q\x82\x81R\x7Fx\xF5\x1Fb\xF7\xCF\x13\x81\xC6s\xC2~\xAE\x18}\xD6\xC5\x88\xDCf$\xCEYi}\xBB>\x1D|\x1B\xBC\xDF\x90` \x01a\x0E\xC8V[_`\x146\x10\x80\x15\x90a\x1B\xFBWP_T`\x01`\x01`\xA0\x1B\x03\x163\x14[\x15a\x1C+WP\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xEC6\x015``\x1C\x90V[P3\x90V[`@Q`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`$\x83\x01R\x83\x16`D\x82\x01R`d\x81\x01\x82\x90Ra\x1C\xE1\x90\x85\x90\x7F#\xB8r\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90`\x84\x01[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x93\x16\x92\x90\x92\x17\x90\x91Ra\x1F\xE7V[PPPPV[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16`$\x82\x01R`D\x81\x01\x82\x90Ra\x1D0\x90\x84\x90\x7F\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90`d\x01a\x1C}V[PPPV[_a\x1DC\x83` \x01Qa \xCBV[a\x1D\x94W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FInvalid input vector provided\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[a\x1D\xA1\x83`@\x01Qa!eV[a\x1D\xEDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FInvalid output vector provided\0\0`D\x82\x01R`d\x01a\x1D\x8BV[a\x1E*\x83_\x01Q\x84` \x01Q\x85`@\x01Q\x86``\x01Q`@Q` \x01a\x1E\x16\x94\x93\x92\x91\x90a?\xABV[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra!\xF2V[\x90Pa\x1ELa\x1E<\x83`@\x01Qa\"\x14V[\x83Q` \x85\x01Q\x84\x92\x91\x90a\" V[a\x1E\xBEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`<`$\x82\x01R\x7FTx merkle proof is not valid for`D\x82\x01R\x7F provided header and tx hash\0\0\0\0`d\x82\x01R`\x84\x01a\x1D\x8BV[a\x1E\xCD\x85\x85\x84`@\x01Qa\"[V[\x94\x93PPPPV[_\x82_\x01\x80Ta\x1E\xE4\x90a:5V[`@Qa\x1E\xF6\x92P\x85\x90` \x01a@\x1AV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P_a\x1F]\x83\x80`@\x01\x90a\x1F\"\x91\x90a=\x9AV[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RP\x86\x92Pa%\xAB\x91PPV[Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x84\x81\x10\x15a\x1F\xE0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`;`$\x82\x01R\x7FBitcoin transaction amount is lo`D\x82\x01R\x7Fwer than in accepted order.\0\0\0\0\0`d\x82\x01R`\x84\x01a\x1D\x8BV[PPPPPV[_a ;\x82`@Q\x80`@\x01`@R\x80` \x81R` \x01\x7FSafeERC20: low-level call failed\x81RP\x85`\x01`\x01`\xA0\x1B\x03\x16a'I\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x80Q\x90\x91P\x15a\x1D0W\x80\x80` \x01\x90Q\x81\x01\x90a Y\x91\x90a@\xE1V[a\x1D0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01R\x7Fot succeed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x1D\x8BV[___a \xD7\x84a'WV[\x90\x92P\x90P\x80\x15\x80a \xE9WP_\x19\x82\x14[\x15a \xF7WP_\x93\x92PPPV[_a!\x03\x83`\x01a?\x81V[\x90P_[\x82\x81\x10\x15a!XW\x85Q\x82\x10a!\"WP_\x95\x94PPPPPV[_a!-\x87\x84a'lV[\x90P_\x19\x81\x03a!CWP_\x96\x95PPPPPPV[a!M\x81\x84a?\x81V[\x92PP`\x01\x01a!\x07V[P\x93Q\x90\x93\x14\x93\x92PPPV[___a!q\x84a'WV[\x90\x92P\x90P\x80\x15\x80a!\x83WP_\x19\x82\x14[\x15a!\x91WP_\x93\x92PPPV[_a!\x9D\x83`\x01a?\x81V[\x90P_[\x82\x81\x10\x15a!XW\x85Q\x82\x10a!\xBCWP_\x95\x94PPPPPV[_a!\xC7\x87\x84a'\xB2V[\x90P_\x19\x81\x03a!\xDDWP_\x96\x95PPPPPPV[a!\xE7\x81\x84a?\x81V[\x92PP`\x01\x01a!\xA1V[_` _\x83Q` \x85\x01`\x02Z\xFAP` _` _`\x02Z\xFAPP_Q\x91\x90PV[`D\x81\x01Q_\x90a\x068V[_\x83\x85\x14\x80\x15a\".WP\x81\x15[\x80\x15a\"9WP\x82Q\x15[\x15a\"FWP`\x01a\x1E\xCDV[a\"R\x85\x84\x86\x85a(\x12V[\x95\x94PPPPPV[_\x83`\x01`\x01`\xA0\x1B\x03\x16c\x117d\xBE`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\"\x98W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\"\xBC\x91\x90aA\0V[\x90P_\x84`\x01`\x01`\xA0\x1B\x03\x16c+\x97\xBE$`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\"\xFBW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a#\x1F\x91\x90aA\0V[\x90P_\x80a#4a#/\x86a(\xB7V[a(\xC2V[\x90P\x83\x81\x03a#EW\x83\x91Pa#\xC2V[\x82\x81\x03a#TW\x82\x91Pa#\xC2V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FNot at current or previous diffi`D\x82\x01R\x7Fculty\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x1D\x8BV[_a#\xCC\x86a(\xE9V[\x90P_\x19\x81\x03a$DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FInvalid length of the headers ch`D\x82\x01R\x7Fain\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x1D\x8BV[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x03a$\xB3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01R\x7FInvalid headers chain\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x1D\x8BV[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFD\x81\x03a%\"W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FInsufficient work in a header\0\0\0`D\x82\x01R`d\x01a\x1D\x8BV[a%,\x87\x84a9YV[\x81\x10\x15a%\xA1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`3`$\x82\x01R\x7FInsufficient accumulated difficu`D\x82\x01R\x7Flty in header chain\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x1D\x8BV[PPPPPPPPV[`@\x80Q``\x81\x01\x82R_\x80\x82R` \x80\x83\x01\x82\x90R\x82\x84\x01\x82\x90R\x83Q\x80\x85\x01\x90\x94R\x81\x84R\x83\x01R\x90a%\xDF\x84a'WV[` \x83\x01R\x80\x82R\x81a%\xF1\x82a9\xF0V[\x90RP_\x80[\x82` \x01Q\x81\x10\x15a&\xF3W\x82Q_\x90a&\x12\x90\x88\x90a'\xB2V[\x84Q\x90\x91P_\x90a&$\x90\x89\x90a+\rV[\x90P_a&2`\x08\x84a9\xDDV[\x86Q\x90\x91P_\x90a&D\x90`\x08a?\x81V[\x8A\x81\x01` \x01\x83\x90 \x90\x91P\x80\x8A\x03a&~W`\x01\x96P\x83\x89_\x01\x81\x81Qa&l\x91\x90aA\x17V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90RPa&\xCEV[_a&\x8C\x8C\x8A_\x01Qa+\x83V[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a&\xADW`\x01`\x01`\xA0\x1B\x03\x81\x16` \x8B\x01R[_a&\xBB\x8D\x8B_\x01Qa,cV[\x90P\x80\x15a&\xCBW`@\x8B\x01\x81\x90R[PP[\x84\x88_\x01\x81\x81Qa&\xDF\x91\x90a?\x81V[\x90RPP`\x01\x90\x94\x01\x93Pa%\xF7\x92PPPV[P\x80a'AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FNo output found for scriptPubKey`D\x82\x01R`d\x01a\x1D\x8BV[PP\x92\x91PPV[``a\x1E\xCD\x84\x84_\x85a-CV[__a'c\x83_a.\x87V[\x91P\x91P\x91P\x91V[___a'y\x85\x85a0$V[\x90\x92P\x90P`\x01\x82\x01a'\x91W_\x19\x92PPPa\x068V[\x80a'\x9D\x83`%a?\x81V[a'\xA7\x91\x90a?\x81V[a\"R\x90`\x04a?\x81V[_a'\xBE\x82`\ta?\x81V[\x83Q\x10\x15a'\xCEWP_\x19a\x068V[_\x80a'\xE4\x85a'\xDF\x86`\x08a?\x81V[a.\x87V[\x90\x92P\x90P`\x01\x82\x01a'\xFCW_\x19\x92PPPa\x068V[\x80a(\x08\x83`\ta?\x81V[a\"R\x91\x90a?\x81V[_` \x84Qa(!\x91\x90aA7V[\x15a(-WP_a\x1E\xCDV[\x83Q_\x03a(<WP_a\x1E\xCDV[\x81\x85_[\x86Q\x81\x10\x15a(\xAAWa(T`\x02\x84aA7V[`\x01\x03a(xWa(qa(k\x88\x83\x01` \x01Q\x90V[\x83a0bV[\x91Pa(\x91V[a(\x8E\x82a(\x89\x89\x84\x01` \x01Q\x90V[a0bV[\x91P[`\x01\x92\x90\x92\x1C\x91a(\xA3` \x82a?\x81V[\x90Pa(@V[P\x90\x93\x14\x95\x94PPPPPV[_a\x068\x82_a0mV[_a\x068{\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83a1\x06V[_`P\x82Qa(\xF8\x91\x90aA7V[\x15a)\x05WP_\x19\x91\x90PV[P_\x80\x80[\x83Q\x81\x10\x15a+\x06W\x80\x15a)QWa)$\x84\x82\x84a1\x11V[a)QWP\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x93\x92PPPV[_a)\\\x85\x83a0mV[\x90Pa)j\x85\x83`Pa1:V[\x92P\x80a*\xAD\x84_\x81\x90P`\x08\x81~\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\x16\x90\x1B`\x08\x82\x90\x1C~\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\x16\x17\x90P`\x10\x81}\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\x16\x90\x1B`\x10\x82\x90\x1C}\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\x16\x17\x90P` \x81{\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\x16\x90\x1B` \x82\x90\x1C{\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\x16\x17\x90P`@\x81w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x1B`@\x82\x90\x1Cw\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x17\x90P`\x80\x81\x90\x1B`\x80\x82\x90\x1C\x17\x90P\x91\x90PV[\x11\x15a*\xDDWP\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFD\x94\x93PPPPV[a*\xE6\x81a(\xC2V[a*\xF0\x90\x85a?\x81V[\x93Pa*\xFF\x90P`P\x82a?\x81V[\x90Pa)\nV[PP\x91\x90PV[_\x80a+\x19\x84\x84a1_V[`\xC0\x1C\x90P_a\"R\x82d\xFF\0\0\0\xFF`\x08\x82\x81\x1C\x91\x82\x16e\xFF\0\0\0\xFF\0\x93\x90\x91\x1B\x92\x83\x16\x17`\x10\x90\x81\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16f\xFF\0\xFF\0\xFF\0\xFF\x92\x90\x92\x16g\xFF\0\xFF\0\xFF\0\xFF\0\x90\x93\x16\x92\x90\x92\x17\x90\x91\x1Ce\xFF\xFF\0\0\xFF\xFF\x16\x17` \x81\x81\x1C\x91\x90\x1B\x17\x90V[_\x82a+\x90\x83`\ta?\x81V[\x81Q\x81\x10a+\xA0Wa+\xA0a:\x80V[` \x91\x01\x01Q\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x7Fj\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x14a+\xF5WP_a\x068V[_\x83a,\x02\x84`\na?\x81V[\x81Q\x81\x10a,\x12Wa,\x12a:\x80V[\x01` \x01Q\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x91P`\xF8\x1C`\x14\x03a,\\W_a,R\x84`\x0Ba?\x81V[\x85\x01`\x14\x01Q\x92PP[P\x92\x91PPV[_\x82a,p\x83`\ta?\x81V[\x81Q\x81\x10a,\x80Wa,\x80a:\x80V[` \x91\x01\x01Q\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x7Fj\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x14a,\xD5WP_a\x068V[_\x83a,\xE2\x84`\na?\x81V[\x81Q\x81\x10a,\xF2Wa,\xF2a:\x80V[\x01` \x90\x81\x01Q\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x92P`\xF8\x1C\x90\x03a,\\W_a-3\x84`\x0Ba?\x81V[\x85\x01` \x01Q\x92PPP\x92\x91PPV[``\x82G\x10\x15a-\xBBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FAddress: insufficient balance fo`D\x82\x01R\x7Fr call\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x1D\x8BV[`\x01`\x01`\xA0\x1B\x03\x85\x16;a.\x12W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x1D\x8BV[__\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@Qa.-\x91\x90aAJV[_`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80_\x81\x14a.gW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a.lV[``\x91P[P\x91P\x91Pa.|\x82\x82\x86a1mV[\x97\x96PPPPPPPV[___a.\x94\x85\x85a1\xA6V[\x90P\x80`\xFF\x16_\x03a.\xC7W_\x85\x85\x81Q\x81\x10a.\xB3Wa.\xB3a:\x80V[\x01` \x01Q\x90\x93P`\xF8\x1C\x91Pa0\x1D\x90PV[\x83a.\xD3\x82`\x01aAUV[`\xFF\x16a.\xE0\x91\x90a?\x81V[\x85Q\x10\x15a.\xF5W_\x19_\x92P\x92PPa0\x1DV[_\x81`\xFF\x16`\x02\x03a/8Wa/-a/\x19a/\x12\x87`\x01a?\x81V[\x88\x90a1_V[b\xFF\xFF\0`\xE8\x82\x90\x1C\x16`\xF8\x91\x90\x91\x1C\x17\x90V[a\xFF\xFF\x16\x90Pa0\x13V[\x81`\xFF\x16`\x04\x03a/\x87Wa/za/Ta/\x12\x87`\x01a?\x81V[`\xD8\x81\x90\x1Cc\xFF\0\xFF\0\x16b\xFF\0\xFF`\xE8\x92\x90\x92\x1C\x91\x90\x91\x16\x17`\x10\x81\x81\x1B\x91\x90\x1C\x17\x90V[c\xFF\xFF\xFF\xFF\x16\x90Pa0\x13V[\x81`\xFF\x16`\x08\x03a0\x13Wa0\x06a/\xA3a/\x12\x87`\x01a?\x81V[`\xC0\x1Cd\xFF\0\0\0\xFF`\x08\x82\x81\x1C\x91\x82\x16e\xFF\0\0\0\xFF\0\x93\x90\x91\x1B\x92\x83\x16\x17`\x10\x90\x81\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16f\xFF\0\xFF\0\xFF\0\xFF\x92\x90\x92\x16g\xFF\0\xFF\0\xFF\0\xFF\0\x90\x93\x16\x92\x90\x92\x17\x90\x91\x1Ce\xFF\xFF\0\0\xFF\xFF\x16\x17` \x81\x81\x1C\x91\x90\x1B\x17\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P[`\xFF\x90\x91\x16\x92P\x90P[\x92P\x92\x90PV[_\x80a01\x83`%a?\x81V[\x84Q\x10\x15a0DWP_\x19\x90P_a0\x1DV[_\x80a0U\x86a'\xDF\x87`$a?\x81V[\x90\x97\x90\x96P\x94PPPPPV[_a\n\xD0\x83\x83a2*V[_\x80a0\x84a0}\x84`Ha?\x81V[\x85\x90a1_V[`\xE8\x1C\x90P_\x84a0\x96\x85`Ka?\x81V[\x81Q\x81\x10a0\xA6Wa0\xA6a:\x80V[\x01` \x01Q`\xF8\x1C\x90P_a0\xD8\x83_`\x10\x82b\xFF\xFF\xFF\x16\x90\x1C\x82a\xFF\0\x16`\x10\x84b\xFF\xFF\xFF\x16\x90\x1B\x17\x17\x90P\x91\x90PV[b\xFF\xFF\xFF\x16\x90P_a0\xEB`\x03\x84aAnV[`\xFF\x16\x90Pa0\xFC\x81a\x01\0aBjV[a.|\x90\x83a9YV[_a\n\xD0\x82\x84a9\x9DV[_\x80a1\x1D\x85\x85a2QV[\x90P\x82\x81\x14a1/W_\x91PPa\n\xD0V[P`\x01\x94\x93PPPPV[_` _\x83\x85` \x01\x87\x01`\x02Z\xFAP` _` _`\x02Z\xFAPP_Q\x93\x92PPPV[_a\n\xD0\x83\x83\x01` \x01Q\x90V[``\x83\x15a1|WP\x81a\n\xD0V[\x82Q\x15a1\x8CW\x82Q\x80\x84` \x01\xFD[\x81`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x1D\x8B\x91\x90aBuV[_\x82\x82\x81Q\x81\x10a1\xB9Wa1\xB9a:\x80V[\x01` \x01Q`\xF8\x1C`\xFF\x03a1\xD0WP`\x08a\x068V[\x82\x82\x81Q\x81\x10a1\xE2Wa1\xE2a:\x80V[\x01` \x01Q`\xF8\x1C`\xFE\x03a1\xF9WP`\x04a\x068V[\x82\x82\x81Q\x81\x10a2\x0BWa2\x0Ba:\x80V[\x01` \x01Q`\xF8\x1C`\xFD\x03a2\"WP`\x02a\x068V[P_\x92\x91PPV[_\x82_R\x81` R` _`@_`\x02Z\xFAP` _` _`\x02Z\xFAPP_Q\x92\x91PPV[_a\n\xD0a2`\x83`\x04a?\x81V[\x84\x01` \x01Q\x90V[`@Q\x80`\xA0\x01`@R\x80_\x81R` \x01a2\x90`@Q\x80` \x01`@R\x80``\x81RP\x90V[\x81R_` \x82\x01\x81\x90R`@\x82\x01\x81\x90R``\x90\x91\x01R\x90V[P\x80Ta2\xB6\x90a:5V[_\x82U\x80`\x1F\x10a2\xC5WPPV[`\x1F\x01` \x90\x04\x90_R` _ \x90\x81\x01\x90a2\xE1\x91\x90a3;V[PV[`@Q\x80a\x01\0\x01`@R\x80_\x81R` \x01a3\x0C`@Q\x80` \x01`@R\x80``\x81RP\x90V[\x81R_` \x82\x01\x81\x90R`@\x82\x01\x81\x90R``\x82\x01\x81\x90R`\x80\x82\x01\x81\x90R`\xA0\x82\x01\x81\x90R`\xC0\x90\x91\x01R\x90V[[\x80\x82\x11\x15a3OW_\x81U`\x01\x01a3<V[P\x90V[__`@\x83\x85\x03\x12\x15a3dW__\xFD[PP\x805\x92` \x90\x91\x015\x91PV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[_\x81Q` \x84Ra\x1E\xCD` \x85\x01\x82a3sV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15a3\xE5W\x81Q\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01a3\xC7V[P\x93\x94\x93PPPPV[_`@\x82\x01`@\x83R\x80\x85Q\x80\x83R``\x85\x01\x91P``\x81`\x05\x1B\x86\x01\x01\x92P` \x87\x01_[\x82\x81\x10\x15a4\xB1W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA0\x87\x86\x03\x01\x84R\x81Q\x80Q\x86R` \x81\x01Q`\xA0` \x88\x01Ra4c`\xA0\x88\x01\x82a3\xA1V[\x90P`\x01`\x01`\xA0\x1B\x03`@\x83\x01Q\x16`@\x88\x01R``\x82\x01Q``\x88\x01R`\x01`\x01`\xA0\x1B\x03`\x80\x83\x01Q\x16`\x80\x88\x01R\x80\x96PPP` \x82\x01\x91P` \x84\x01\x93P`\x01\x81\x01\x90Pa4\x15V[PPPP\x82\x81\x03` \x84\x01Ra\"R\x81\x85a3\xB5V[_` \x82\x84\x03\x12\x15a4\xD7W__\xFD[P\x91\x90PV[___``\x84\x86\x03\x12\x15a4\xEFW__\xFD[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a5\x0CW__\xFD[a5\x18\x86\x82\x87\x01a4\xC7V[\x93\x96\x93\x95PPPP`@\x91\x90\x91\x015\x90V[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a5@W__\xFD[\x91\x90PV[____`\x80\x85\x87\x03\x12\x15a5XW__\xFD[\x845\x93P` \x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a5uW__\xFD[a5\x81\x87\x82\x88\x01a4\xC7V[\x93PPa5\x90`@\x86\x01a5*V[\x93\x96\x92\x95P\x92\x93``\x015\x92PPV[_` \x82\x84\x03\x12\x15a5\xB0W__\xFD[P5\x91\x90PV[\x85\x81R`\xA0` \x82\x01R_a5\xCF`\xA0\x83\x01\x87a3\xA1V[\x90P`\x01`\x01`\xA0\x1B\x03\x85\x16`@\x83\x01R\x83``\x83\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x80\x83\x01R\x96\x95PPPPPPV[\x88\x81Ra\x01\0` \x82\x01R_a6\x19a\x01\0\x83\x01\x8Aa3\xA1V[`@\x83\x01\x98\x90\x98RP`\x01`\x01`\xA0\x1B\x03\x95\x86\x16``\x82\x01R`\x80\x81\x01\x94\x90\x94R\x91\x84\x16`\xA0\x84\x01R\x90\x92\x16`\xC0\x82\x01R`\xE0\x01R\x92\x91PPV[_` \x82\x84\x03\x12\x15a6dW__\xFD[a\n\xD0\x82a5*V[___``\x84\x86\x03\x12\x15a6\x7FW__\xFD[\x835\x92Pa6\x8F` \x85\x01a5*V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`@\x80\x82R\x83Q\x90\x82\x01\x81\x90R_\x90` \x85\x01\x90``\x84\x01\x90\x83[\x81\x81\x10\x15a7\x11W\x83Q\x80Q\x84R`\x01`\x01`\xA0\x1B\x03` \x82\x01Q\x16` \x85\x01R`@\x81\x01Q`@\x85\x01R`\x01`\x01`\xA0\x1B\x03``\x82\x01Q\x16``\x85\x01RP`\x80\x83\x01\x92P` \x84\x01\x93P`\x01\x81\x01\x90Pa6\xBBV[PP\x83\x81\x03` \x85\x01Ra7%\x81\x86a3\xB5V[\x96\x95PPPPPPV[`@\x80\x82R\x83Q\x90\x82\x01\x81\x90R_\x90` \x85\x01\x90``\x84\x01\x90\x83[\x81\x81\x10\x15a7\x11W\x83Q\x80Q\x84R` \x81\x01Q` \x85\x01R`\x01`\x01`\xA0\x1B\x03`@\x82\x01Q\x16`@\x85\x01R``\x81\x01Q``\x85\x01R`\x01`\x01`\xA0\x1B\x03`\x80\x82\x01Q\x16`\x80\x85\x01R`\x01`\x01`\xA0\x1B\x03`\xA0\x82\x01Q\x16`\xA0\x85\x01R`\xC0\x81\x01Q`\xC0\x85\x01RP`\xE0\x83\x01\x92P` \x84\x01\x93P`\x01\x81\x01\x90Pa7JV[___``\x84\x86\x03\x12\x15a7\xD9W__\xFD[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a7\xF6W__\xFD[\x84\x01`\x80\x81\x87\x03\x12\x15a8\x07W__\xFD[\x91P`@\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a8\"W__\xFD[\x84\x01``\x81\x87\x03\x12\x15a83W__\xFD[\x80\x91PP\x92P\x92P\x92V[_`@\x82\x01`@\x83R\x80\x85Q\x80\x83R``\x85\x01\x91P``\x81`\x05\x1B\x86\x01\x01\x92P` \x87\x01_[\x82\x81\x10\x15a4\xB1W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA0\x87\x86\x03\x01\x84R\x81Q\x80Q\x86R` \x81\x01Qa\x01\0` \x88\x01Ra8\xB4a\x01\0\x88\x01\x82a3\xA1V[\x90P`@\x82\x01Q`@\x88\x01R`\x01`\x01`\xA0\x1B\x03``\x83\x01Q\x16``\x88\x01R`\x80\x82\x01Q`\x80\x88\x01R`\x01`\x01`\xA0\x1B\x03`\xA0\x83\x01Q\x16`\xA0\x88\x01R`\xC0\x82\x01Qa9\n`\xC0\x89\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`\xE0\x91\x82\x01Q\x96\x90\x91\x01\x95\x90\x95R` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a8dV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x068Wa\x068a9,V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_\x82a9\xABWa9\xABa9pV[P\x04\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x01`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x068Wa\x068a9,V[__\x19\x82\x03a:\x01Wa:\x01a9,V[P`\x01\x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[`\x01\x81\x81\x1C\x90\x82\x16\x80a:IW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a4\xD7W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_\x82`\x1F\x83\x01\x12a:\xBCW__\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a:\xD6Wa:\xD6a:\x08V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a;\x05Wa;\x05a:\x08V[`@R\x81\x81R\x83\x82\x01` \x01\x85\x10\x15a;\x1CW__\xFD[\x81` \x85\x01` \x83\x017_\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[_` \x826\x03\x12\x15a;HW__\xFD[`@Q` \x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a;kWa;ka:\x08V[`@R\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a;\x84W__\xFD[a;\x906\x82\x86\x01a:\xADV[\x82RP\x92\x91PPV[`\x1F\x82\x11\x15a\x1D0W\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a;\xBEWP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x1F\xE0W_\x81U`\x01\x01a;\xCAV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a;\xF7Wa;\xF7a:\x08V[a<\x0B\x81a<\x05\x84Ta:5V[\x84a;\x99V[` `\x1F\x82\x11`\x01\x81\x14a<=W_\x83\x15a<&WP\x84\x82\x01Q[_\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua\x1F\xE0V[_\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a<lW\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a<LV[P\x84\x82\x10\x15a<\x89W\x86\x84\x01Q_\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[\x81\x83R\x81\x81` \x85\x017P_` \x82\x84\x01\x01R_` `\x1F\x19`\x1F\x84\x01\x16\x84\x01\x01\x90P\x92\x91PPV[_\x815\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x836\x03\x01\x81\x12a<\xF3W__\xFD[\x82\x01` \x81\x01\x905g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a=\x0FW__\xFD[\x806\x03\x82\x13\x15a=\x1DW__\xFD[` \x85Ra\"R` \x86\x01\x82\x84a<\x98V[`\x80\x81R_a=A`\x80\x83\x01\x87a<\xC1V[` \x83\x01\x95\x90\x95RP`@\x81\x01\x92\x90\x92R`\x01`\x01`\xA0\x1B\x03\x16``\x90\x91\x01R\x91\x90PV[\x84\x81R`\x80` \x82\x01R_a=~`\x80\x83\x01\x86a<\xC1V[`\x01`\x01`\xA0\x1B\x03\x94\x90\x94\x16`@\x83\x01RP``\x01R\x92\x91PPV[__\x835\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x846\x03\x01\x81\x12a=\xCDW__\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a=\xE7W__\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a0\x1DW__\xFD[` \x81R_a\x1E\xCD` \x83\x01\x84\x86a<\x98V[\x805\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x81\x14a5@W__\xFD[_`\x80\x826\x03\x12\x15a>MW__\xFD[`@Q`\x80\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a>pWa>pa:\x08V[`@Ra>|\x83a>\x0EV[\x81R` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a>\x97W__\xFD[a>\xA36\x82\x86\x01a:\xADV[` \x83\x01RP`@\x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a>\xC2W__\xFD[a>\xCE6\x82\x86\x01a:\xADV[`@\x83\x01RPa>\xE0``\x84\x01a>\x0EV[``\x82\x01R\x92\x91PPV[_``\x826\x03\x12\x15a>\xFBW__\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a?\x1EWa?\x1Ea:\x08V[`@R\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a?7W__\xFD[a?C6\x82\x86\x01a:\xADV[\x82RP` \x83\x81\x015\x90\x82\x01R`@\x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a?iW__\xFD[a?u6\x82\x86\x01a:\xADV[`@\x83\x01RP\x92\x91PPV[\x80\x82\x01\x80\x82\x11\x15a\x068Wa\x068a9,V[_\x81Q\x80` \x84\x01\x85^_\x93\x01\x92\x83RP\x90\x91\x90PV[\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85\x16\x81R_a?\xE7a?\xE1`\x04\x84\x01\x87a?\x94V[\x85a?\x94V[\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x93\x90\x93\x16\x83RPP`\x04\x01\x93\x92PPPV[\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83`\xF8\x1B\x16\x81R__\x83Ta@O\x81a:5V[`\x01\x82\x16\x80\x15a@fW`\x01\x81\x14a@\x9FWa@\xD5V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x83\x16`\x01\x87\x01R`\x01\x82\x15\x15\x83\x02\x87\x01\x01\x93Pa@\xD5V[\x86_R` _ _[\x83\x81\x10\x15a@\xCAW\x81T`\x01\x82\x8A\x01\x01R`\x01\x82\x01\x91P` \x81\x01\x90Pa@\xA8V[PP`\x01\x82\x87\x01\x01\x93P[P\x91\x96\x95PPPPPPV[_` \x82\x84\x03\x12\x15a@\xF1W__\xFD[\x81Q\x80\x15\x15\x81\x14a\n\xD0W__\xFD[_` \x82\x84\x03\x12\x15aA\x10W__\xFD[PQ\x91\x90PV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x068Wa\x068a9,V[_\x82aAEWaAEa9pV[P\x06\x90V[_a\n\xD0\x82\x84a?\x94V[`\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x068Wa\x068a9,V[`\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x068Wa\x068a9,V[`\x01\x81[`\x01\x84\x11\x15aA\xC2W\x80\x85\x04\x81\x11\x15aA\xA6WaA\xA6a9,V[`\x01\x84\x16\x15aA\xB4W\x90\x81\x02\x90[`\x01\x93\x90\x93\x1C\x92\x80\x02aA\x8BV[\x93P\x93\x91PPV[_\x82aA\xD8WP`\x01a\x068V[\x81aA\xE4WP_a\x068V[\x81`\x01\x81\x14aA\xFAW`\x02\x81\x14aB\x04WaB V[`\x01\x91PPa\x068V[`\xFF\x84\x11\x15aB\x15WaB\x15a9,V[PP`\x01\x82\x1Ba\x068V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15aBCWP\x81\x81\na\x068V[aBO_\x19\x84\x84aA\x87V[\x80_\x19\x04\x82\x11\x15aBbWaBba9,V[\x02\x93\x92PPPV[_a\n\xD0\x83\x83aA\xCAV[` \x81R_a\n\xD0` \x83\x01\x84a3sV\xFE\xA2dipfsX\"\x12 1\xF6\x9B(m\xB9\x91\xC7\xBE\xEB$\x13s4\x18\xFC&\xCA\x86}\x0F\xB2\x91\xFBQ\xA4l\x03~Rv\x98dsolcC\0\x08\x1B\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b506004361061016e575f3560e01c80636a8cde3a116100d2578063bd2a7e3e11610088578063d1920ff011610063578063d1920ff0146103bc578063df69b14f146103c5578063ecca2c36146103d8575f5ffd5b8063bd2a7e3e146102f3578063c56a45261461038f578063ce1b815f146103a2575f5ffd5b80639cc6722e116100b85780639cc6722e146102b7578063a383013b146102cd578063b522c133146102e0575f5ffd5b80636a8cde3a1461028e57806372a546c6146102a4575f5ffd5b80634145640a11610127578063572b6c051161010d578063572b6c05146102345780635b8fe042146102655780636811a31114610278575f5ffd5b80634145640a146101fa578063506a109d14610221575f5ffd5b8063210ec18111610157578063210ec181146101ae578063364f1ec0146101c15780633af3fc7e146101d6575f5ffd5b806311c137aa146101725780631dfe759514610198575b5f5ffd5b610185610180366004613353565b610445565b6040519081526020015b60405180910390f35b6101a061063e565b60405161018f9291906133ef565b6101856101bc3660046134dd565b6108a0565b6101d46101cf366004613545565b610ad7565b005b6101e96101e43660046135a0565b610c2c565b60405161018f9594939291906135b7565b61020d6102083660046135a0565b610cfe565b60405161018f9897969594939291906135ff565b6101d461022f3660046135a0565b610def565b610255610242366004613654565b5f546001600160a01b0391821691161490565b604051901515815260200161018f565b6101d461027336600461366d565b610ed4565b610280610ff0565b60405161018f9291906136a0565b6102966111b4565b60405161018f92919061372f565b6101d46102b23660046137c7565b6113ba565b6102bf611564565b60405161018f92919061383e565b6101d46102db3660046135a0565b6117c7565b6101d46102ee3660046137c7565b61186b565b61034c6103013660046135a0565b600260208190525f918252604090912080546001820154928201546003830154600484015460058501546006909501549395946001600160a01b039384169492939182169291169087565b6040805197885260208801969096526001600160a01b03948516958701959095526060860192909252821660808501521660a083015260c082015260e00161018f565b6101d461039d3660046135a0565b6119ea565b5f546040516001600160a01b03909116815260200161018f565b61018561546081565b6101d46103d33660046135a0565b611acd565b61041a6103e63660046135a0565b600360208190525f9182526040909120805460018201546002830154929093015490926001600160a01b0390811692911684565b604080519485526001600160a01b03938416602086015284019190915216606082015260800161018f565b5f828152600160205260408120805483111561045f575f5ffd5b5f831161046a575f5ffd5b805460038201545f919061047e9086613959565b610488919061399d565b90505f8111610499576104996139b0565b80826003015410156104ad576104ad6139b0565b80826003015f8282546104c091906139dd565b90915550508154849083905f906104d89084906139dd565b90915550506040805160e0810182528681526020810186905260028401546001600160a01b039081169282019290925260608101839052600484015490911660808201525f9060a0810161052a611be0565b6001600160a01b0316815242602090910152600580549192505f919082610550836139f0565b909155505f818152600260208181526040928390208651815586820151600182015586840151818401805473ffffffffffffffffffffffffffffffffffffffff199081166001600160a01b03938416179091556060808a0151600385015560808a0151600485018054841691851691909117905560a08a01516005850180549093169084161790915560c08901516006909301929092559289015484518c815292830189905290921692810192909252919250829189917fc39a1a5ddc0e85c955fe2e1abeb43c94ce18322e75bb3d44e80f759ff9d034b9910160405180910390a393505050505b92915050565b6060805f805b600554811015610683575f818152600160205260409020600401546001600160a01b03161561067b5781610677816139f0565b9250505b600101610644565b505f8167ffffffffffffffff81111561069e5761069e613a08565b6040519080825280602002602001820160405280156106d757816020015b6106c4613269565b8152602001906001900390816106bc5790505b5090505f8267ffffffffffffffff8111156106f4576106f4613a08565b60405190808252806020026020018201604052801561071d578160200160208202803683370190505b5090505f805b600554811015610894575f818152600160205260409020600401546001600160a01b03161561088c5760015f8281526020019081526020015f206040518060a00160405290815f8201548152602001600182016040518060200160405290815f8201805461079090613a35565b80601f01602080910402602001604051908101604052809291908181526020018280546107bc90613a35565b80156108075780601f106107de57610100808354040283529160200191610807565b820191905f5260205f20905b8154815290600101906020018083116107ea57829003601f168201915b50505091909252505050815260028201546001600160a01b03908116602083015260038301546040830152600490920154909116606090910152845185908490811061085557610855613a80565b60200260200101819052508083838151811061087357610873613a80565b602090810291909101015281610888816139f0565b9250505b600101610723565b50919590945092505050565b5f838152600360205260408120826108b6575f5ffd5b80548311156108c3575f5ffd5b805460028201545f91906108d79086613959565b6108e1919061399d565b90505f81116108f2576108f26139b0565b8082600201541015610906576109066139b0565b80826002015f82825461091991906139dd565b90915550508154849083905f906109319084906139dd565b909155506109589050610942611be0565b60018401546001600160a01b0316903084611c30565b600580545f9182610968836139f0565b9190505590506040518061010001604052808881526020018761098a90613b38565b81526020810187905260018501546001600160a01b03908116604083015260608201859052600386015416608082015260a0016109c5611be0565b6001600160a01b03168152426020918201525f838152600482526040902082518155908201518051600183019081906109fe9082613bdd565b5050506040828101516002830155606083015160038301805473ffffffffffffffffffffffffffffffffffffffff199081166001600160a01b03938416179091556080850151600485015560a0850151600585018054831691841691909117905560c085015160068501805490921690831617905560e0909301516007909201919091556001850154905183928a927f653e0d81f2c99beba359dfb17b499a5cff2be9d950514852224df8c097c2192192610ac3928c928c928a929190911690613d2f565b60405180910390a3925050505b9392505050565b6001600160a01b038216610ae9575f5ffd5b610b06610af4611be0565b6001600160a01b038416903084611c30565b600580545f9182610b16836139f0565b9190505590506040518060a0016040528086815260200185610b3790613b38565b8152602001846001600160a01b03168152602001838152602001610b59611be0565b6001600160a01b031690525f8281526001602081815260409092208351815591830151805190918301908190610b8f9082613bdd565b50505060408281015160028301805473ffffffffffffffffffffffffffffffffffffffff199081166001600160a01b03938416179091556060850151600385015560809094015160049093018054909416921691909117909155517f98c7c680403d47403dea4a570d0e6c5716538c49420ef471cec428f5a5852c0690610c1d908790879087908790613d66565b60405180910390a15050505050565b600160208181525f92835260409283902080548451928301909452918201805482908290610c5990613a35565b80601f0160208091040260200160405190810160405280929190818152602001828054610c8590613a35565b8015610cd05780601f10610ca757610100808354040283529160200191610cd0565b820191905f5260205f20905b815481529060010190602001808311610cb357829003601f168201915b505050919092525050506002820154600383015460049093015491926001600160a01b039182169290911685565b6004602052805f5260405f205f91509050805f015490806001016040518060200160405290815f82018054610d3290613a35565b80601f0160208091040260200160405190810160405280929190818152602001828054610d5e90613a35565b8015610da95780601f10610d8057610100808354040283529160200191610da9565b820191905f5260205f20905b815481529060010190602001808311610d8c57829003601f168201915b5050509190925250505060028201546003830154600484015460058501546006860154600790960154949593946001600160a01b03938416949293918216929091169088565b5f818152600160205260409020610e04611be0565b60048201546001600160a01b03908116911614610e1f575f5ffd5b610e44610e2a611be0565b600383015460028401546001600160a01b03169190611ce7565b5f82815260016020819052604082208281559190820181610e6582826132aa565b50505060028101805473ffffffffffffffffffffffffffffffffffffffff199081169091555f60038301556004909101805490911690556040518281527fc340e7ac48dc80ee793fc6266960bd5f1bd21be91c8a95e218178113f79e17b4906020015b60405180910390a15050565b6001600160a01b038216610ee6575f5ffd5b5f8311610ef1575f5ffd5b5f8111610efc575f5ffd5b600580545f9182610f0c836139f0565b9190505590506040518060800160405280858152602001846001600160a01b03168152602001838152602001610f40611be0565b6001600160a01b039081169091525f83815260036020818152604092839020855181558582015160018201805491871673ffffffffffffffffffffffffffffffffffffffff199283161790558685015160028301556060968701519190930180549186169190931617909155815188815292871690830152810184905282917fff1ce210defcd3ba1adf76c9419a0758fa60fd3eb38c7bd9418f60b575b76e24910160405180910390a250505050565b6060805f805b600554811015611036575f81815260036020819052604090912001546001600160a01b03161561102e578161102a816139f0565b9250505b600101610ff6565b505f8167ffffffffffffffff81111561105157611051613a08565b6040519080825280602002602001820160405280156110a157816020015b604080516080810182525f8082526020808301829052928201819052606082015282525f1990920191018161106f5790505b5090505f8267ffffffffffffffff8111156110be576110be613a08565b6040519080825280602002602001820160405280156110e7578160200160208202803683370190505b5090505f805b600554811015610894575f81815260036020819052604090912001546001600160a01b0316156111ac575f8181526003602081815260409283902083516080810185528154815260018201546001600160a01b039081169382019390935260028201549481019490945290910154166060820152845185908490811061117557611175613a80565b60200260200101819052508083838151811061119357611193613a80565b6020908102919091010152816111a8816139f0565b9250505b6001016110ed565b6060805f805b6005548110156111f0575f81815260026020526040902060010154156111e857816111e4816139f0565b9250505b6001016111ba565b505f8167ffffffffffffffff81111561120b5761120b613a08565b60405190808252806020026020018201604052801561129057816020015b61127d6040518060e001604052805f81526020015f81526020015f6001600160a01b031681526020015f81526020015f6001600160a01b031681526020015f6001600160a01b031681526020015f81525090565b8152602001906001900390816112295790505b5090505f8267ffffffffffffffff8111156112ad576112ad613a08565b6040519080825280602002602001820160405280156112d6578160200160208202803683370190505b5090505f805b600554811015610894575f81815260026020526040902060010154156113b2575f81815260026020818152604092839020835160e08101855281548152600182015492810192909252918201546001600160a01b039081169382019390935260038201546060820152600482015483166080820152600582015490921660a08301526006015460c0820152845185908490811061137b5761137b613a80565b60200260200101819052508083838151811061139957611399613a80565b6020908102919091010152816113ae816139f0565b9250505b6001016112dc565b5f8381526004602052604090206113cf611be0565b60058201546001600160a01b039081169116146113ea575f5ffd5b6006546001600160a01b031663d38c29a16114086040850185613d9a565b6040518363ffffffff1660e01b8152600401611425929190613dfb565b5f604051808303815f87803b15801561143c575f5ffd5b505af115801561144e573d5f5f3e3d5ffd5b5050505061147f6007548461146290613e3d565b61146b85613eeb565b6006546001600160a01b0316929190611d35565b5061149281600201548260010185611ed5565b6005810154600482015460038301546114b9926001600160a01b0391821692911690611ce7565b5f8481526004602052604081208181559060018201816114d982826132aa565b50505f6002830181905560038301805473ffffffffffffffffffffffffffffffffffffffff1990811690915560048401829055600584018054821690556006840180549091169055600790920191909155506040518481527fcf561061db78f7bc518d37fe86718514c640ccc5c3f1293828b955e68f19f5fb9060200160405180910390a150505050565b6060805f805b6005548110156115a0575f81815260046020526040902060020154156115985781611594816139f0565b9250505b60010161156a565b505f8167ffffffffffffffff8111156115bb576115bb613a08565b6040519080825280602002602001820160405280156115f457816020015b6115e16132e4565b8152602001906001900390816115d95790505b5090505f8267ffffffffffffffff81111561161157611611613a08565b60405190808252806020026020018201604052801561163a578160200160208202803683370190505b5090505f805b600554811015610894575f81815260046020526040902060020154156117bf5760045f8281526020019081526020015f20604051806101000160405290815f8201548152602001600182016040518060200160405290815f820180546116a590613a35565b80601f01602080910402602001604051908101604052809291908181526020018280546116d190613a35565b801561171c5780601f106116f35761010080835404028352916020019161171c565b820191905f5260205f20905b8154815290600101906020018083116116ff57829003601f168201915b5050509190925250505081526002820154602082015260038201546001600160a01b0390811660408301526004830154606083015260058301548116608083015260068301541660a082015260079091015460c090910152845185908490811061178857611788613a80565b6020026020010181905250808383815181106117a6576117a6613a80565b6020908102919091010152816117bb816139f0565b9250505b600101611640565b5f8181526003602052604090206117dc611be0565b60038201546001600160a01b039081169116146117f7575f5ffd5b5f82815260036020818152604080842084815560018101805473ffffffffffffffffffffffffffffffffffffffff1990811690915560028201959095559092018054909316909255518381527f3cd475b092e8b379f6ba0d9e0e0c8f30705e73321dc5c9f80ce4ad38db7be1aa9101610ec8565b5f838152600260205260409020611880611be0565b60058201546001600160a01b0390811691161461189b575f5ffd5b6006546001600160a01b031663d38c29a16118b96040850185613d9a565b6040518363ffffffff1660e01b81526004016118d6929190613dfb565b5f604051808303815f87803b1580156118ed575f5ffd5b505af11580156118ff573d5f5f3e3d5ffd5b505050506119136007548461146290613e3d565b5080545f908152600160208190526040909120805490916119379190830186611ed5565b60058201546003830154600284015461195e926001600160a01b0391821692911690611ce7565b5f85815260026020818152604080842084815560018101859055928301805473ffffffffffffffffffffffffffffffffffffffff1990811690915560038401859055600484018054821690556005840180549091169055600690920192909255518681527fb4c98de210696b3cf21e99335c1ee3a0ae34a26713412a4adde8af596176f37e9101610c1d565b5f8181526002602052604090206119ff611be0565b60048201546001600160a01b03908116911614611a1a575f5ffd5b6154608160060154611a2c9190613f81565b4211611a36575f5ffd5b611a41610e2a611be0565b5f82815260026020818152604080842084815560018101859055928301805473ffffffffffffffffffffffffffffffffffffffff1990811690915560038401859055600484018054821690556005840180549091169055600690920192909255518381527f3e5ea358e9eb4cdf44cdc77938ade8074b1240a6d8c0fd13728671b82e800ad69101610ec8565b5f8181526004602052604090206007810154611aec9061546090613f81565b4211611af6575f5ffd5b611afe611be0565b60068201546001600160a01b03908116911614611b19575f5ffd5b611b3e611b24611be0565b600483015460038401546001600160a01b03169190611ce7565b5f828152600460205260408120818155906001820181611b5e82826132aa565b50505f6002830181905560038301805473ffffffffffffffffffffffffffffffffffffffff1990811690915560048401829055600584018054821690556006840180549091169055600790920191909155506040518281527f78f51f62f7cf1381c673c27eae187dd6c588dc6624ce59697dbb3e1d7c1bbcdf90602001610ec8565b5f60143610801590611bfb57505f546001600160a01b031633145b15611c2b57507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffec36013560601c90565b503390565b6040516001600160a01b0380851660248301528316604482015260648101829052611ce19085907f23b872dd00000000000000000000000000000000000000000000000000000000906084015b60408051601f198184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167fffffffff0000000000000000000000000000000000000000000000000000000090931692909217909152611fe7565b50505050565b6040516001600160a01b038316602482015260448101829052611d309084907fa9059cbb0000000000000000000000000000000000000000000000000000000090606401611c7d565b505050565b5f611d4383602001516120cb565b611d945760405162461bcd60e51b815260206004820152601d60248201527f496e76616c696420696e70757420766563746f722070726f766964656400000060448201526064015b60405180910390fd5b611da18360400151612165565b611ded5760405162461bcd60e51b815260206004820152601e60248201527f496e76616c6964206f757470757420766563746f722070726f766964656400006044820152606401611d8b565b611e2a835f0151846020015185604001518660600151604051602001611e169493929190613fab565b6040516020818303038152906040526121f2565b9050611e4c611e3c8360400151612214565b8351602085015184929190612220565b611ebe5760405162461bcd60e51b815260206004820152603c60248201527f5478206d65726b6c652070726f6f66206973206e6f742076616c696420666f7260448201527f2070726f76696465642068656164657220616e642074782068617368000000006064820152608401611d8b565b611ecd8585846040015161225b565b949350505050565b5f825f018054611ee490613a35565b604051611ef69250859060200161401a565b6040516020818303038152906040528051906020012090505f611f5d838060400190611f229190613d9a565b8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f920191909152508692506125ab915050565b5167ffffffffffffffff16905084811015611fe05760405162461bcd60e51b815260206004820152603b60248201527f426974636f696e207472616e73616374696f6e20616d6f756e74206973206c6f60448201527f776572207468616e20696e206163636570746564206f726465722e00000000006064820152608401611d8b565b5050505050565b5f61203b826040518060400160405280602081526020017f5361666545524332303a206c6f772d6c6576656c2063616c6c206661696c6564815250856001600160a01b03166127499092919063ffffffff16565b805190915015611d30578080602001905181019061205991906140e1565b611d305760405162461bcd60e51b815260206004820152602a60248201527f5361666545524332303a204552433230206f7065726174696f6e20646964206e60448201527f6f742073756363656564000000000000000000000000000000000000000000006064820152608401611d8b565b5f5f5f6120d784612757565b90925090508015806120e957505f1982145b156120f757505f9392505050565b5f612103836001613f81565b90505f5b82811015612158578551821061212257505f95945050505050565b5f61212d878461276c565b90505f19810361214357505f9695505050505050565b61214d8184613f81565b925050600101612107565b5093519093149392505050565b5f5f5f61217184612757565b909250905080158061218357505f1982145b1561219157505f9392505050565b5f61219d836001613f81565b90505f5b8281101561215857855182106121bc57505f95945050505050565b5f6121c787846127b2565b90505f1981036121dd57505f9695505050505050565b6121e78184613f81565b9250506001016121a1565b5f60205f83516020850160025afa5060205f60205f60025afa50505f51919050565b60448101515f90610638565b5f838514801561222e575081155b801561223957508251155b1561224657506001611ecd565b61225285848685612812565b95945050505050565b5f836001600160a01b031663113764be6040518163ffffffff1660e01b8152600401602060405180830381865afa158015612298573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906122bc9190614100565b90505f846001600160a01b0316632b97be246040518163ffffffff1660e01b8152600401602060405180830381865afa1580156122fb573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061231f9190614100565b90505f8061233461232f866128b7565b6128c2565b9050838103612345578391506123c2565b828103612354578291506123c2565b60405162461bcd60e51b815260206004820152602560248201527f4e6f742061742063757272656e74206f722070726576696f757320646966666960448201527f63756c74790000000000000000000000000000000000000000000000000000006064820152608401611d8b565b5f6123cc866128e9565b90505f1981036124445760405162461bcd60e51b815260206004820152602360248201527f496e76616c6964206c656e677468206f6620746865206865616465727320636860448201527f61696e00000000000000000000000000000000000000000000000000000000006064820152608401611d8b565b7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81036124b35760405162461bcd60e51b815260206004820152601560248201527f496e76616c6964206865616465727320636861696e00000000000000000000006044820152606401611d8b565b7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffd81036125225760405162461bcd60e51b815260206004820152601d60248201527f496e73756666696369656e7420776f726b20696e2061206865616465720000006044820152606401611d8b565b61252c8784613959565b8110156125a15760405162461bcd60e51b815260206004820152603360248201527f496e73756666696369656e7420616363756d756c61746564206469666669637560448201527f6c747920696e2068656164657220636861696e000000000000000000000000006064820152608401611d8b565b5050505050505050565b604080516060810182525f80825260208083018290528284018290528351808501909452818452830152906125df84612757565b6020830152808252816125f1826139f0565b9052505f805b82602001518110156126f35782515f906126129088906127b2565b84519091505f90612624908990612b0d565b90505f6126326008846139dd565b86519091505f90612644906008613f81565b8a8101602001839020909150808a0361267e576001965083895f0181815161266c9190614117565b67ffffffffffffffff169052506126ce565b5f61268c8c8a5f0151612b83565b90506001600160a01b038116156126ad576001600160a01b03811660208b01525b5f6126bb8d8b5f0151612c63565b905080156126cb5760408b018190525b50505b84885f018181516126df9190613f81565b90525050600190940193506125f792505050565b50806127415760405162461bcd60e51b815260206004820181905260248201527f4e6f206f757470757420666f756e6420666f72207363726970745075624b65796044820152606401611d8b565b505092915050565b6060611ecd84845f85612d43565b5f5f612763835f612e87565b91509150915091565b5f5f5f6127798585613024565b909250905060018201612791575f1992505050610638565b8061279d836025613f81565b6127a79190613f81565b612252906004613f81565b5f6127be826009613f81565b835110156127ce57505f19610638565b5f806127e4856127df866008613f81565b612e87565b9092509050600182016127fc575f1992505050610638565b80612808836009613f81565b6122529190613f81565b5f602084516128219190614137565b1561282d57505f611ecd565b83515f0361283c57505f611ecd565b81855f5b86518110156128aa57612854600284614137565b6001036128785761287161286b8883016020015190565b83613062565b9150612891565b61288e826128898984016020015190565b613062565b91505b60019290921c916128a3602082613f81565b9050612840565b5090931495945050505050565b5f610638825f61306d565b5f6106387bffff000000000000000000000000000000000000000000000000000083613106565b5f605082516128f89190614137565b1561290557505f19919050565b505f80805b8351811015612b0657801561295157612924848284613111565b61295157507ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe9392505050565b5f61295c858361306d565b905061296a8583605061313a565b925080612aad845f8190506008817eff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff16901b600882901c7eff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff161790506010817dffff0000ffff0000ffff0000ffff0000ffff0000ffff0000ffff0000ffff16901b601082901c7dffff0000ffff0000ffff0000ffff0000ffff0000ffff0000ffff0000ffff161790506020817bffffffff00000000ffffffff00000000ffffffff00000000ffffffff16901b602082901c7bffffffff00000000ffffffff00000000ffffffff00000000ffffffff1617905060408177ffffffffffffffff0000000000000000ffffffffffffffff16901b604082901c77ffffffffffffffff0000000000000000ffffffffffffffff16179050608081901b608082901c179050919050565b1115612add57507ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffd949350505050565b612ae6816128c2565b612af09085613f81565b9350612aff9050605082613f81565b905061290a565b5050919050565b5f80612b19848461315f565b60c01c90505f6122528264ff000000ff600882811c91821665ff000000ff009390911b92831617601090811b67ffffffffffffffff1666ff00ff00ff00ff9290921667ff00ff00ff00ff009093169290921790911c65ffff0000ffff1617602081811c91901b1790565b5f82612b90836009613f81565b81518110612ba057612ba0613a80565b6020910101517fff00000000000000000000000000000000000000000000000000000000000000167f6a0000000000000000000000000000000000000000000000000000000000000014612bf557505f610638565b5f83612c0284600a613f81565b81518110612c1257612c12613a80565b01602001517fff000000000000000000000000000000000000000000000000000000000000008116915060f81c601403612c5c575f612c5284600b613f81565b8501601401519250505b5092915050565b5f82612c70836009613f81565b81518110612c8057612c80613a80565b6020910101517fff00000000000000000000000000000000000000000000000000000000000000167f6a0000000000000000000000000000000000000000000000000000000000000014612cd557505f610638565b5f83612ce284600a613f81565b81518110612cf257612cf2613a80565b016020908101517fff000000000000000000000000000000000000000000000000000000000000008116925060f81c9003612c5c575f612d3384600b613f81565b8501602001519250505092915050565b606082471015612dbb5760405162461bcd60e51b815260206004820152602660248201527f416464726573733a20696e73756666696369656e742062616c616e636520666f60448201527f722063616c6c00000000000000000000000000000000000000000000000000006064820152608401611d8b565b6001600160a01b0385163b612e125760405162461bcd60e51b815260206004820152601d60248201527f416464726573733a2063616c6c20746f206e6f6e2d636f6e74726163740000006044820152606401611d8b565b5f5f866001600160a01b03168587604051612e2d919061414a565b5f6040518083038185875af1925050503d805f8114612e67576040519150601f19603f3d011682016040523d82523d5f602084013e612e6c565b606091505b5091509150612e7c82828661316d565b979650505050505050565b5f5f5f612e9485856131a6565b90508060ff165f03612ec7575f858581518110612eb357612eb3613a80565b016020015190935060f81c915061301d9050565b83612ed3826001614155565b60ff16612ee09190613f81565b85511015612ef5575f195f925092505061301d565b5f8160ff16600203612f3857612f2d612f19612f12876001613f81565b889061315f565b62ffff0060e882901c1660f89190911c1790565b61ffff169050613013565b8160ff16600403612f8757612f7a612f54612f12876001613f81565b60d881901c63ff00ff001662ff00ff60e89290921c9190911617601081811b91901c1790565b63ffffffff169050613013565b8160ff1660080361301357613006612fa3612f12876001613f81565b60c01c64ff000000ff600882811c91821665ff000000ff009390911b92831617601090811b67ffffffffffffffff1666ff00ff00ff00ff9290921667ff00ff00ff00ff009093169290921790911c65ffff0000ffff1617602081811c91901b1790565b67ffffffffffffffff1690505b60ff909116925090505b9250929050565b5f80613031836025613f81565b8451101561304457505f1990505f61301d565b5f80613055866127df876024613f81565b9097909650945050505050565b5f610ad0838361322a565b5f8061308461307d846048613f81565b859061315f565b60e81c90505f8461309685604b613f81565b815181106130a6576130a6613a80565b016020015160f81c90505f6130d8835f60108262ffffff16901c8261ff001660108462ffffff16901b17179050919050565b62ffffff1690505f6130eb60038461416e565b60ff1690506130fc8161010061426a565b612e7c9083613959565b5f610ad0828461399d565b5f8061311d8585613251565b905082811461312f575f915050610ad0565b506001949350505050565b5f60205f8385602001870160025afa5060205f60205f60025afa50505f519392505050565b5f610ad08383016020015190565b6060831561317c575081610ad0565b82511561318c5782518084602001fd5b8160405162461bcd60e51b8152600401611d8b9190614275565b5f8282815181106131b9576131b9613a80565b016020015160f81c60ff036131d057506008610638565b8282815181106131e2576131e2613a80565b016020015160f81c60fe036131f957506004610638565b82828151811061320b5761320b613a80565b016020015160f81c60fd0361322257506002610638565b505f92915050565b5f825f528160205260205f60405f60025afa5060205f60205f60025afa50505f5192915050565b5f610ad0613260836004613f81565b84016020015190565b6040518060a001604052805f81526020016132906040518060200160405280606081525090565b81525f602082018190526040820181905260609091015290565b5080546132b690613a35565b5f825580601f106132c5575050565b601f0160209004905f5260205f20908101906132e1919061333b565b50565b6040518061010001604052805f815260200161330c6040518060200160405280606081525090565b81525f6020820181905260408201819052606082018190526080820181905260a0820181905260c09091015290565b5b8082111561334f575f815560010161333c565b5090565b5f5f60408385031215613364575f5ffd5b50508035926020909101359150565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b5f815160208452611ecd6020850182613373565b5f8151808452602084019350602083015f5b828110156133e55781518652602095860195909101906001016133c7565b5093949350505050565b5f604082016040835280855180835260608501915060608160051b8601019250602087015f5b828110156134b1577fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa0878603018452815180518652602081015160a0602088015261346360a08801826133a1565b90506001600160a01b036040830151166040880152606082015160608801526001600160a01b0360808301511660808801528096505050602082019150602084019350600181019050613415565b50505050828103602084015261225281856133b5565b5f602082840312156134d7575f5ffd5b50919050565b5f5f5f606084860312156134ef575f5ffd5b83359250602084013567ffffffffffffffff81111561350c575f5ffd5b613518868287016134c7565b93969395505050506040919091013590565b80356001600160a01b0381168114613540575f5ffd5b919050565b5f5f5f5f60808587031215613558575f5ffd5b84359350602085013567ffffffffffffffff811115613575575f5ffd5b613581878288016134c7565b9350506135906040860161352a565b9396929550929360600135925050565b5f602082840312156135b0575f5ffd5b5035919050565b85815260a060208201525f6135cf60a08301876133a1565b90506001600160a01b03851660408301528360608301526001600160a01b03831660808301529695505050505050565b88815261010060208201525f61361961010083018a6133a1565b6040830198909852506001600160a01b039586166060820152608081019490945291841660a084015290921660c082015260e0015292915050565b5f60208284031215613664575f5ffd5b610ad08261352a565b5f5f5f6060848603121561367f575f5ffd5b8335925061368f6020850161352a565b929592945050506040919091013590565b604080825283519082018190525f9060208501906060840190835b81811015613711578351805184526001600160a01b036020820151166020850152604081015160408501526001600160a01b036060820151166060850152506080830192506020840193506001810190506136bb565b5050838103602085015261372581866133b5565b9695505050505050565b604080825283519082018190525f9060208501906060840190835b8181101561371157835180518452602081015160208501526001600160a01b036040820151166040850152606081015160608501526001600160a01b0360808201511660808501526001600160a01b0360a08201511660a085015260c081015160c08501525060e08301925060208401935060018101905061374a565b5f5f5f606084860312156137d9575f5ffd5b83359250602084013567ffffffffffffffff8111156137f6575f5ffd5b840160808187031215613807575f5ffd5b9150604084013567ffffffffffffffff811115613822575f5ffd5b840160608187031215613833575f5ffd5b809150509250925092565b5f604082016040835280855180835260608501915060608160051b8601019250602087015f5b828110156134b1577fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa0878603018452815180518652602081015161010060208801526138b46101008801826133a1565b9050604082015160408801526001600160a01b036060830151166060880152608082015160808801526001600160a01b0360a08301511660a088015260c082015161390a60c08901826001600160a01b03169052565b5060e09182015196909101959095526020938401939190910190600101613864565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b80820281158282048414176106385761063861392c565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f826139ab576139ab613970565b500490565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52600160045260245ffd5b818103818111156106385761063861392c565b5f5f198203613a0157613a0161392c565b5060010190565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b600181811c90821680613a4957607f821691505b6020821081036134d7577f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f82601f830112613abc575f5ffd5b813567ffffffffffffffff811115613ad657613ad6613a08565b604051601f8201601f19908116603f0116810167ffffffffffffffff81118282101715613b0557613b05613a08565b604052818152838201602001851015613b1c575f5ffd5b816020850160208301375f918101602001919091529392505050565b5f60208236031215613b48575f5ffd5b6040516020810167ffffffffffffffff81118282101715613b6b57613b6b613a08565b604052823567ffffffffffffffff811115613b84575f5ffd5b613b9036828601613aad565b82525092915050565b601f821115611d3057805f5260205f20601f840160051c81016020851015613bbe5750805b601f840160051c820191505b81811015611fe0575f8155600101613bca565b815167ffffffffffffffff811115613bf757613bf7613a08565b613c0b81613c058454613a35565b84613b99565b6020601f821160018114613c3d575f8315613c265750848201515b5f19600385901b1c1916600184901b178455611fe0565b5f84815260208120601f198516915b82811015613c6c5787850151825560209485019460019092019101613c4c565b5084821015613c8957868401515f19600387901b60f8161c191681555b50505050600190811b01905550565b81835281816020850137505f602082840101525f6020601f19601f840116840101905092915050565b5f81357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe1833603018112613cf3575f5ffd5b820160208101903567ffffffffffffffff811115613d0f575f5ffd5b803603821315613d1d575f5ffd5b60208552612252602086018284613c98565b608081525f613d416080830187613cc1565b60208301959095525060408101929092526001600160a01b0316606090910152919050565b848152608060208201525f613d7e6080830186613cc1565b6001600160a01b03949094166040830152506060015292915050565b5f5f83357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe1843603018112613dcd575f5ffd5b83018035915067ffffffffffffffff821115613de7575f5ffd5b60200191503681900382131561301d575f5ffd5b602081525f611ecd602083018486613c98565b80357fffffffff0000000000000000000000000000000000000000000000000000000081168114613540575f5ffd5b5f60808236031215613e4d575f5ffd5b6040516080810167ffffffffffffffff81118282101715613e7057613e70613a08565b604052613e7c83613e0e565b8152602083013567ffffffffffffffff811115613e97575f5ffd5b613ea336828601613aad565b602083015250604083013567ffffffffffffffff811115613ec2575f5ffd5b613ece36828601613aad565b604083015250613ee060608401613e0e565b606082015292915050565b5f60608236031215613efb575f5ffd5b6040516060810167ffffffffffffffff81118282101715613f1e57613f1e613a08565b604052823567ffffffffffffffff811115613f37575f5ffd5b613f4336828601613aad565b82525060208381013590820152604083013567ffffffffffffffff811115613f69575f5ffd5b613f7536828601613aad565b60408301525092915050565b808201808211156106385761063861392c565b5f81518060208401855e5f93019283525090919050565b7fffffffff00000000000000000000000000000000000000000000000000000000851681525f613fe7613fe16004840187613f94565b85613f94565b7fffffffff0000000000000000000000000000000000000000000000000000000093909316835250506004019392505050565b7fff000000000000000000000000000000000000000000000000000000000000008360f81b1681525f5f835461404f81613a35565b600182168015614066576001811461409f576140d5565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00831660018701526001821515830287010193506140d5565b865f5260205f205f5b838110156140ca5781546001828a0101526001820191506020810190506140a8565b505060018287010193505b50919695505050505050565b5f602082840312156140f1575f5ffd5b81518015158114610ad0575f5ffd5b5f60208284031215614110575f5ffd5b5051919050565b67ffffffffffffffff81811683821601908111156106385761063861392c565b5f8261414557614145613970565b500690565b5f610ad08284613f94565b60ff81811683821601908111156106385761063861392c565b60ff82811682821603908111156106385761063861392c565b6001815b60018411156141c2578085048111156141a6576141a661392c565b60018416156141b457908102905b60019390931c92800261418b565b935093915050565b5f826141d857506001610638565b816141e457505f610638565b81600181146141fa576002811461420457614220565b6001915050610638565b60ff8411156142155761421561392c565b50506001821b610638565b5060208310610133831016604e8410600b8410161715614243575081810a610638565b61424f5f198484614187565b805f19048211156142625761426261392c565b029392505050565b5f610ad083836141ca565b602081525f610ad0602083018461337356fea264697066735822122031f69b286db991c7beeb2413733418fc26ca867d0fb291fb51a46c037e52769864736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x01nW_5`\xE0\x1C\x80cj\x8C\xDE:\x11a\0\xD2W\x80c\xBD*~>\x11a\0\x88W\x80c\xD1\x92\x0F\xF0\x11a\0cW\x80c\xD1\x92\x0F\xF0\x14a\x03\xBCW\x80c\xDFi\xB1O\x14a\x03\xC5W\x80c\xEC\xCA,6\x14a\x03\xD8W__\xFD[\x80c\xBD*~>\x14a\x02\xF3W\x80c\xC5jE&\x14a\x03\x8FW\x80c\xCE\x1B\x81_\x14a\x03\xA2W__\xFD[\x80c\x9C\xC6r.\x11a\0\xB8W\x80c\x9C\xC6r.\x14a\x02\xB7W\x80c\xA3\x83\x01;\x14a\x02\xCDW\x80c\xB5\"\xC13\x14a\x02\xE0W__\xFD[\x80cj\x8C\xDE:\x14a\x02\x8EW\x80cr\xA5F\xC6\x14a\x02\xA4W__\xFD[\x80cAEd\n\x11a\x01'W\x80cW+l\x05\x11a\x01\rW\x80cW+l\x05\x14a\x024W\x80c[\x8F\xE0B\x14a\x02eW\x80ch\x11\xA3\x11\x14a\x02xW__\xFD[\x80cAEd\n\x14a\x01\xFAW\x80cPj\x10\x9D\x14a\x02!W__\xFD[\x80c!\x0E\xC1\x81\x11a\x01WW\x80c!\x0E\xC1\x81\x14a\x01\xAEW\x80c6O\x1E\xC0\x14a\x01\xC1W\x80c:\xF3\xFC~\x14a\x01\xD6W__\xFD[\x80c\x11\xC17\xAA\x14a\x01rW\x80c\x1D\xFEu\x95\x14a\x01\x98W[__\xFD[a\x01\x85a\x01\x806`\x04a3SV[a\x04EV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\xA0a\x06>V[`@Qa\x01\x8F\x92\x91\x90a3\xEFV[a\x01\x85a\x01\xBC6`\x04a4\xDDV[a\x08\xA0V[a\x01\xD4a\x01\xCF6`\x04a5EV[a\n\xD7V[\0[a\x01\xE9a\x01\xE46`\x04a5\xA0V[a\x0C,V[`@Qa\x01\x8F\x95\x94\x93\x92\x91\x90a5\xB7V[a\x02\ra\x02\x086`\x04a5\xA0V[a\x0C\xFEV[`@Qa\x01\x8F\x98\x97\x96\x95\x94\x93\x92\x91\x90a5\xFFV[a\x01\xD4a\x02/6`\x04a5\xA0V[a\r\xEFV[a\x02Ua\x02B6`\x04a6TV[_T`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x91\x16\x14\x90V[`@Q\x90\x15\x15\x81R` \x01a\x01\x8FV[a\x01\xD4a\x02s6`\x04a6mV[a\x0E\xD4V[a\x02\x80a\x0F\xF0V[`@Qa\x01\x8F\x92\x91\x90a6\xA0V[a\x02\x96a\x11\xB4V[`@Qa\x01\x8F\x92\x91\x90a7/V[a\x01\xD4a\x02\xB26`\x04a7\xC7V[a\x13\xBAV[a\x02\xBFa\x15dV[`@Qa\x01\x8F\x92\x91\x90a8>V[a\x01\xD4a\x02\xDB6`\x04a5\xA0V[a\x17\xC7V[a\x01\xD4a\x02\xEE6`\x04a7\xC7V[a\x18kV[a\x03La\x03\x016`\x04a5\xA0V[`\x02` \x81\x90R_\x91\x82R`@\x90\x91 \x80T`\x01\x82\x01T\x92\x82\x01T`\x03\x83\x01T`\x04\x84\x01T`\x05\x85\x01T`\x06\x90\x95\x01T\x93\x95\x94`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x94\x92\x93\x91\x82\x16\x92\x91\x16\x90\x87V[`@\x80Q\x97\x88R` \x88\x01\x96\x90\x96R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x95\x87\x01\x95\x90\x95R``\x86\x01\x92\x90\x92R\x82\x16`\x80\x85\x01R\x16`\xA0\x83\x01R`\xC0\x82\x01R`\xE0\x01a\x01\x8FV[a\x01\xD4a\x03\x9D6`\x04a5\xA0V[a\x19\xEAV[_T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\x8FV[a\x01\x85aT`\x81V[a\x01\xD4a\x03\xD36`\x04a5\xA0V[a\x1A\xCDV[a\x04\x1Aa\x03\xE66`\x04a5\xA0V[`\x03` \x81\x90R_\x91\x82R`@\x90\x91 \x80T`\x01\x82\x01T`\x02\x83\x01T\x92\x90\x93\x01T\x90\x92`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x92\x91\x16\x84V[`@\x80Q\x94\x85R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16` \x86\x01R\x84\x01\x91\x90\x91R\x16``\x82\x01R`\x80\x01a\x01\x8FV[_\x82\x81R`\x01` R`@\x81 \x80T\x83\x11\x15a\x04_W__\xFD[_\x83\x11a\x04jW__\xFD[\x80T`\x03\x82\x01T_\x91\x90a\x04~\x90\x86a9YV[a\x04\x88\x91\x90a9\x9DV[\x90P_\x81\x11a\x04\x99Wa\x04\x99a9\xB0V[\x80\x82`\x03\x01T\x10\x15a\x04\xADWa\x04\xADa9\xB0V[\x80\x82`\x03\x01_\x82\x82Ta\x04\xC0\x91\x90a9\xDDV[\x90\x91UPP\x81T\x84\x90\x83\x90_\x90a\x04\xD8\x90\x84\x90a9\xDDV[\x90\x91UPP`@\x80Q`\xE0\x81\x01\x82R\x86\x81R` \x81\x01\x86\x90R`\x02\x84\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x83\x90R`\x04\x84\x01T\x90\x91\x16`\x80\x82\x01R_\x90`\xA0\x81\x01a\x05*a\x1B\xE0V[`\x01`\x01`\xA0\x1B\x03\x16\x81RB` \x90\x91\x01R`\x05\x80T\x91\x92P_\x91\x90\x82a\x05P\x83a9\xF0V[\x90\x91UP_\x81\x81R`\x02` \x81\x81R`@\x92\x83\x90 \x86Q\x81U\x86\x82\x01Q`\x01\x82\x01U\x86\x84\x01Q\x81\x84\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x17\x90\x91U``\x80\x8A\x01Q`\x03\x85\x01U`\x80\x8A\x01Q`\x04\x85\x01\x80T\x84\x16\x91\x85\x16\x91\x90\x91\x17\x90U`\xA0\x8A\x01Q`\x05\x85\x01\x80T\x90\x93\x16\x90\x84\x16\x17\x90\x91U`\xC0\x89\x01Q`\x06\x90\x93\x01\x92\x90\x92U\x92\x89\x01T\x84Q\x8C\x81R\x92\x83\x01\x89\x90R\x90\x92\x16\x92\x81\x01\x92\x90\x92R\x91\x92P\x82\x91\x89\x91\x7F\xC3\x9A\x1A]\xDC\x0E\x85\xC9U\xFE.\x1A\xBE\xB4<\x94\xCE\x182.u\xBB=D\xE8\x0Fu\x9F\xF9\xD04\xB9\x91\x01`@Q\x80\x91\x03\x90\xA3\x93PPPP[\x92\x91PPV[``\x80_\x80[`\x05T\x81\x10\x15a\x06\x83W_\x81\x81R`\x01` R`@\x90 `\x04\x01T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x06{W\x81a\x06w\x81a9\xF0V[\x92PP[`\x01\x01a\x06DV[P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06\x9EWa\x06\x9Ea:\x08V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x06\xD7W\x81` \x01[a\x06\xC4a2iV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x06\xBCW\x90P[P\x90P_\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06\xF4Wa\x06\xF4a:\x08V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x07\x1DW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_\x80[`\x05T\x81\x10\x15a\x08\x94W_\x81\x81R`\x01` R`@\x90 `\x04\x01T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x08\x8CW`\x01_\x82\x81R` \x01\x90\x81R` \x01_ `@Q\x80`\xA0\x01`@R\x90\x81_\x82\x01T\x81R` \x01`\x01\x82\x01`@Q\x80` \x01`@R\x90\x81_\x82\x01\x80Ta\x07\x90\x90a:5V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x07\xBC\x90a:5V[\x80\x15a\x08\x07W\x80`\x1F\x10a\x07\xDEWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x08\x07V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x07\xEAW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x90\x92RPPP\x81R`\x02\x82\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16` \x83\x01R`\x03\x83\x01T`@\x83\x01R`\x04\x90\x92\x01T\x90\x91\x16``\x90\x91\x01R\x84Q\x85\x90\x84\x90\x81\x10a\x08UWa\x08Ua:\x80V[` \x02` \x01\x01\x81\x90RP\x80\x83\x83\x81Q\x81\x10a\x08sWa\x08sa:\x80V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x81a\x08\x88\x81a9\xF0V[\x92PP[`\x01\x01a\x07#V[P\x91\x95\x90\x94P\x92PPPV[_\x83\x81R`\x03` R`@\x81 \x82a\x08\xB6W__\xFD[\x80T\x83\x11\x15a\x08\xC3W__\xFD[\x80T`\x02\x82\x01T_\x91\x90a\x08\xD7\x90\x86a9YV[a\x08\xE1\x91\x90a9\x9DV[\x90P_\x81\x11a\x08\xF2Wa\x08\xF2a9\xB0V[\x80\x82`\x02\x01T\x10\x15a\t\x06Wa\t\x06a9\xB0V[\x80\x82`\x02\x01_\x82\x82Ta\t\x19\x91\x90a9\xDDV[\x90\x91UPP\x81T\x84\x90\x83\x90_\x90a\t1\x90\x84\x90a9\xDDV[\x90\x91UPa\tX\x90Pa\tBa\x1B\xE0V[`\x01\x84\x01T`\x01`\x01`\xA0\x1B\x03\x16\x900\x84a\x1C0V[`\x05\x80T_\x91\x82a\th\x83a9\xF0V[\x91\x90PU\x90P`@Q\x80a\x01\0\x01`@R\x80\x88\x81R` \x01\x87a\t\x8A\x90a;8V[\x81R` \x81\x01\x87\x90R`\x01\x85\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`@\x83\x01R``\x82\x01\x85\x90R`\x03\x86\x01T\x16`\x80\x82\x01R`\xA0\x01a\t\xC5a\x1B\xE0V[`\x01`\x01`\xA0\x1B\x03\x16\x81RB` \x91\x82\x01R_\x83\x81R`\x04\x82R`@\x90 \x82Q\x81U\x90\x82\x01Q\x80Q`\x01\x83\x01\x90\x81\x90a\t\xFE\x90\x82a;\xDDV[PPP`@\x82\x81\x01Q`\x02\x83\x01U``\x83\x01Q`\x03\x83\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x17\x90\x91U`\x80\x85\x01Q`\x04\x85\x01U`\xA0\x85\x01Q`\x05\x85\x01\x80T\x83\x16\x91\x84\x16\x91\x90\x91\x17\x90U`\xC0\x85\x01Q`\x06\x85\x01\x80T\x90\x92\x16\x90\x83\x16\x17\x90U`\xE0\x90\x93\x01Q`\x07\x90\x92\x01\x91\x90\x91U`\x01\x85\x01T\x90Q\x83\x92\x8A\x92\x7Fe>\r\x81\xF2\xC9\x9B\xEB\xA3Y\xDF\xB1{I\x9A\\\xFF+\xE9\xD9PQHR\"M\xF8\xC0\x97\xC2\x19!\x92a\n\xC3\x92\x8C\x92\x8C\x92\x8A\x92\x91\x90\x91\x16\x90a=/V[`@Q\x80\x91\x03\x90\xA3\x92PPP[\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\n\xE9W__\xFD[a\x0B\x06a\n\xF4a\x1B\xE0V[`\x01`\x01`\xA0\x1B\x03\x84\x16\x900\x84a\x1C0V[`\x05\x80T_\x91\x82a\x0B\x16\x83a9\xF0V[\x91\x90PU\x90P`@Q\x80`\xA0\x01`@R\x80\x86\x81R` \x01\x85a\x0B7\x90a;8V[\x81R` \x01\x84`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83\x81R` \x01a\x0BYa\x1B\xE0V[`\x01`\x01`\xA0\x1B\x03\x16\x90R_\x82\x81R`\x01` \x81\x81R`@\x90\x92 \x83Q\x81U\x91\x83\x01Q\x80Q\x90\x91\x83\x01\x90\x81\x90a\x0B\x8F\x90\x82a;\xDDV[PPP`@\x82\x81\x01Q`\x02\x83\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x17\x90\x91U``\x85\x01Q`\x03\x85\x01U`\x80\x90\x94\x01Q`\x04\x90\x93\x01\x80T\x90\x94\x16\x92\x16\x91\x90\x91\x17\x90\x91UQ\x7F\x98\xC7\xC6\x80@=G@=\xEAJW\r\x0ElW\x16S\x8CIB\x0E\xF4q\xCE\xC4(\xF5\xA5\x85,\x06\x90a\x0C\x1D\x90\x87\x90\x87\x90\x87\x90\x87\x90a=fV[`@Q\x80\x91\x03\x90\xA1PPPPPV[`\x01` \x81\x81R_\x92\x83R`@\x92\x83\x90 \x80T\x84Q\x92\x83\x01\x90\x94R\x91\x82\x01\x80T\x82\x90\x82\x90a\x0CY\x90a:5V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0C\x85\x90a:5V[\x80\x15a\x0C\xD0W\x80`\x1F\x10a\x0C\xA7Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0C\xD0V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0C\xB3W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x90\x92RPPP`\x02\x82\x01T`\x03\x83\x01T`\x04\x90\x93\x01T\x91\x92`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x92\x90\x91\x16\x85V[`\x04` R\x80_R`@_ _\x91P\x90P\x80_\x01T\x90\x80`\x01\x01`@Q\x80` \x01`@R\x90\x81_\x82\x01\x80Ta\r2\x90a:5V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\r^\x90a:5V[\x80\x15a\r\xA9W\x80`\x1F\x10a\r\x80Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\r\xA9V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\r\x8CW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x90\x92RPPP`\x02\x82\x01T`\x03\x83\x01T`\x04\x84\x01T`\x05\x85\x01T`\x06\x86\x01T`\x07\x90\x96\x01T\x94\x95\x93\x94`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x94\x92\x93\x91\x82\x16\x92\x90\x91\x16\x90\x88V[_\x81\x81R`\x01` R`@\x90 a\x0E\x04a\x1B\xE0V[`\x04\x82\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x16\x14a\x0E\x1FW__\xFD[a\x0EDa\x0E*a\x1B\xE0V[`\x03\x83\x01T`\x02\x84\x01T`\x01`\x01`\xA0\x1B\x03\x16\x91\x90a\x1C\xE7V[_\x82\x81R`\x01` \x81\x90R`@\x82 \x82\x81U\x91\x90\x82\x01\x81a\x0Ee\x82\x82a2\xAAV[PPP`\x02\x81\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x81\x16\x90\x91U_`\x03\x83\x01U`\x04\x90\x91\x01\x80T\x90\x91\x16\x90U`@Q\x82\x81R\x7F\xC3@\xE7\xACH\xDC\x80\xEEy?\xC6&i`\xBD_\x1B\xD2\x1B\xE9\x1C\x8A\x95\xE2\x18\x17\x81\x13\xF7\x9E\x17\xB4\x90` \x01[`@Q\x80\x91\x03\x90\xA1PPV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x0E\xE6W__\xFD[_\x83\x11a\x0E\xF1W__\xFD[_\x81\x11a\x0E\xFCW__\xFD[`\x05\x80T_\x91\x82a\x0F\x0C\x83a9\xF0V[\x91\x90PU\x90P`@Q\x80`\x80\x01`@R\x80\x85\x81R` \x01\x84`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83\x81R` \x01a\x0F@a\x1B\xE0V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x90\x91R_\x83\x81R`\x03` \x81\x81R`@\x92\x83\x90 \x85Q\x81U\x85\x82\x01Q`\x01\x82\x01\x80T\x91\x87\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x92\x83\x16\x17\x90U\x86\x85\x01Q`\x02\x83\x01U``\x96\x87\x01Q\x91\x90\x93\x01\x80T\x91\x86\x16\x91\x90\x93\x16\x17\x90\x91U\x81Q\x88\x81R\x92\x87\x16\x90\x83\x01R\x81\x01\x84\x90R\x82\x91\x7F\xFF\x1C\xE2\x10\xDE\xFC\xD3\xBA\x1A\xDFv\xC9A\x9A\x07X\xFA`\xFD>\xB3\x8C{\xD9A\x8F`\xB5u\xB7n$\x91\x01`@Q\x80\x91\x03\x90\xA2PPPPV[``\x80_\x80[`\x05T\x81\x10\x15a\x106W_\x81\x81R`\x03` \x81\x90R`@\x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x10.W\x81a\x10*\x81a9\xF0V[\x92PP[`\x01\x01a\x0F\xF6V[P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x10QWa\x10Qa:\x08V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x10\xA1W\x81` \x01[`@\x80Q`\x80\x81\x01\x82R_\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01\x81\x90R``\x82\x01R\x82R_\x19\x90\x92\x01\x91\x01\x81a\x10oW\x90P[P\x90P_\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x10\xBEWa\x10\xBEa:\x08V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x10\xE7W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_\x80[`\x05T\x81\x10\x15a\x08\x94W_\x81\x81R`\x03` \x81\x90R`@\x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x11\xACW_\x81\x81R`\x03` \x81\x81R`@\x92\x83\x90 \x83Q`\x80\x81\x01\x85R\x81T\x81R`\x01\x82\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x93\x82\x01\x93\x90\x93R`\x02\x82\x01T\x94\x81\x01\x94\x90\x94R\x90\x91\x01T\x16``\x82\x01R\x84Q\x85\x90\x84\x90\x81\x10a\x11uWa\x11ua:\x80V[` \x02` \x01\x01\x81\x90RP\x80\x83\x83\x81Q\x81\x10a\x11\x93Wa\x11\x93a:\x80V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x81a\x11\xA8\x81a9\xF0V[\x92PP[`\x01\x01a\x10\xEDV[``\x80_\x80[`\x05T\x81\x10\x15a\x11\xF0W_\x81\x81R`\x02` R`@\x90 `\x01\x01T\x15a\x11\xE8W\x81a\x11\xE4\x81a9\xF0V[\x92PP[`\x01\x01a\x11\xBAV[P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x12\x0BWa\x12\x0Ba:\x08V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x12\x90W\x81` \x01[a\x12}`@Q\x80`\xE0\x01`@R\x80_\x81R` \x01_\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x12)W\x90P[P\x90P_\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x12\xADWa\x12\xADa:\x08V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x12\xD6W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_\x80[`\x05T\x81\x10\x15a\x08\x94W_\x81\x81R`\x02` R`@\x90 `\x01\x01T\x15a\x13\xB2W_\x81\x81R`\x02` \x81\x81R`@\x92\x83\x90 \x83Q`\xE0\x81\x01\x85R\x81T\x81R`\x01\x82\x01T\x92\x81\x01\x92\x90\x92R\x91\x82\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x93\x82\x01\x93\x90\x93R`\x03\x82\x01T``\x82\x01R`\x04\x82\x01T\x83\x16`\x80\x82\x01R`\x05\x82\x01T\x90\x92\x16`\xA0\x83\x01R`\x06\x01T`\xC0\x82\x01R\x84Q\x85\x90\x84\x90\x81\x10a\x13{Wa\x13{a:\x80V[` \x02` \x01\x01\x81\x90RP\x80\x83\x83\x81Q\x81\x10a\x13\x99Wa\x13\x99a:\x80V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x81a\x13\xAE\x81a9\xF0V[\x92PP[`\x01\x01a\x12\xDCV[_\x83\x81R`\x04` R`@\x90 a\x13\xCFa\x1B\xE0V[`\x05\x82\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x16\x14a\x13\xEAW__\xFD[`\x06T`\x01`\x01`\xA0\x1B\x03\x16c\xD3\x8C)\xA1a\x14\x08`@\x85\x01\x85a=\x9AV[`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x14%\x92\x91\x90a=\xFBV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x14<W__\xFD[PZ\xF1\x15\x80\x15a\x14NW=__>=_\xFD[PPPPa\x14\x7F`\x07T\x84a\x14b\x90a>=V[a\x14k\x85a>\xEBV[`\x06T`\x01`\x01`\xA0\x1B\x03\x16\x92\x91\x90a\x1D5V[Pa\x14\x92\x81`\x02\x01T\x82`\x01\x01\x85a\x1E\xD5V[`\x05\x81\x01T`\x04\x82\x01T`\x03\x83\x01Ta\x14\xB9\x92`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x92\x91\x16\x90a\x1C\xE7V[_\x84\x81R`\x04` R`@\x81 \x81\x81U\x90`\x01\x82\x01\x81a\x14\xD9\x82\x82a2\xAAV[PP_`\x02\x83\x01\x81\x90U`\x03\x83\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x81\x16\x90\x91U`\x04\x84\x01\x82\x90U`\x05\x84\x01\x80T\x82\x16\x90U`\x06\x84\x01\x80T\x90\x91\x16\x90U`\x07\x90\x92\x01\x91\x90\x91UP`@Q\x84\x81R\x7F\xCFV\x10a\xDBx\xF7\xBCQ\x8D7\xFE\x86q\x85\x14\xC6@\xCC\xC5\xC3\xF1)8(\xB9U\xE6\x8F\x19\xF5\xFB\x90` \x01`@Q\x80\x91\x03\x90\xA1PPPPV[``\x80_\x80[`\x05T\x81\x10\x15a\x15\xA0W_\x81\x81R`\x04` R`@\x90 `\x02\x01T\x15a\x15\x98W\x81a\x15\x94\x81a9\xF0V[\x92PP[`\x01\x01a\x15jV[P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x15\xBBWa\x15\xBBa:\x08V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x15\xF4W\x81` \x01[a\x15\xE1a2\xE4V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x15\xD9W\x90P[P\x90P_\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x16\x11Wa\x16\x11a:\x08V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x16:W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_\x80[`\x05T\x81\x10\x15a\x08\x94W_\x81\x81R`\x04` R`@\x90 `\x02\x01T\x15a\x17\xBFW`\x04_\x82\x81R` \x01\x90\x81R` \x01_ `@Q\x80a\x01\0\x01`@R\x90\x81_\x82\x01T\x81R` \x01`\x01\x82\x01`@Q\x80` \x01`@R\x90\x81_\x82\x01\x80Ta\x16\xA5\x90a:5V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x16\xD1\x90a:5V[\x80\x15a\x17\x1CW\x80`\x1F\x10a\x16\xF3Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x17\x1CV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x16\xFFW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x90\x92RPPP\x81R`\x02\x82\x01T` \x82\x01R`\x03\x82\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`@\x83\x01R`\x04\x83\x01T``\x83\x01R`\x05\x83\x01T\x81\x16`\x80\x83\x01R`\x06\x83\x01T\x16`\xA0\x82\x01R`\x07\x90\x91\x01T`\xC0\x90\x91\x01R\x84Q\x85\x90\x84\x90\x81\x10a\x17\x88Wa\x17\x88a:\x80V[` \x02` \x01\x01\x81\x90RP\x80\x83\x83\x81Q\x81\x10a\x17\xA6Wa\x17\xA6a:\x80V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x81a\x17\xBB\x81a9\xF0V[\x92PP[`\x01\x01a\x16@V[_\x81\x81R`\x03` R`@\x90 a\x17\xDCa\x1B\xE0V[`\x03\x82\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x16\x14a\x17\xF7W__\xFD[_\x82\x81R`\x03` \x81\x81R`@\x80\x84 \x84\x81U`\x01\x81\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x81\x16\x90\x91U`\x02\x82\x01\x95\x90\x95U\x90\x92\x01\x80T\x90\x93\x16\x90\x92UQ\x83\x81R\x7F<\xD4u\xB0\x92\xE8\xB3y\xF6\xBA\r\x9E\x0E\x0C\x8F0p^s2\x1D\xC5\xC9\xF8\x0C\xE4\xAD8\xDB{\xE1\xAA\x91\x01a\x0E\xC8V[_\x83\x81R`\x02` R`@\x90 a\x18\x80a\x1B\xE0V[`\x05\x82\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x16\x14a\x18\x9BW__\xFD[`\x06T`\x01`\x01`\xA0\x1B\x03\x16c\xD3\x8C)\xA1a\x18\xB9`@\x85\x01\x85a=\x9AV[`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x18\xD6\x92\x91\x90a=\xFBV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x18\xEDW__\xFD[PZ\xF1\x15\x80\x15a\x18\xFFW=__>=_\xFD[PPPPa\x19\x13`\x07T\x84a\x14b\x90a>=V[P\x80T_\x90\x81R`\x01` \x81\x90R`@\x90\x91 \x80T\x90\x91a\x197\x91\x90\x83\x01\x86a\x1E\xD5V[`\x05\x82\x01T`\x03\x83\x01T`\x02\x84\x01Ta\x19^\x92`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x92\x91\x16\x90a\x1C\xE7V[_\x85\x81R`\x02` \x81\x81R`@\x80\x84 \x84\x81U`\x01\x81\x01\x85\x90U\x92\x83\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x81\x16\x90\x91U`\x03\x84\x01\x85\x90U`\x04\x84\x01\x80T\x82\x16\x90U`\x05\x84\x01\x80T\x90\x91\x16\x90U`\x06\x90\x92\x01\x92\x90\x92UQ\x86\x81R\x7F\xB4\xC9\x8D\xE2\x10ik<\xF2\x1E\x993\\\x1E\xE3\xA0\xAE4\xA2g\x13A*J\xDD\xE8\xAFYav\xF3~\x91\x01a\x0C\x1DV[_\x81\x81R`\x02` R`@\x90 a\x19\xFFa\x1B\xE0V[`\x04\x82\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x16\x14a\x1A\x1AW__\xFD[aT`\x81`\x06\x01Ta\x1A,\x91\x90a?\x81V[B\x11a\x1A6W__\xFD[a\x1AAa\x0E*a\x1B\xE0V[_\x82\x81R`\x02` \x81\x81R`@\x80\x84 \x84\x81U`\x01\x81\x01\x85\x90U\x92\x83\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x81\x16\x90\x91U`\x03\x84\x01\x85\x90U`\x04\x84\x01\x80T\x82\x16\x90U`\x05\x84\x01\x80T\x90\x91\x16\x90U`\x06\x90\x92\x01\x92\x90\x92UQ\x83\x81R\x7F>^\xA3X\xE9\xEBL\xDFD\xCD\xC7y8\xAD\xE8\x07K\x12@\xA6\xD8\xC0\xFD\x13r\x86q\xB8.\x80\n\xD6\x91\x01a\x0E\xC8V[_\x81\x81R`\x04` R`@\x90 `\x07\x81\x01Ta\x1A\xEC\x90aT`\x90a?\x81V[B\x11a\x1A\xF6W__\xFD[a\x1A\xFEa\x1B\xE0V[`\x06\x82\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x16\x14a\x1B\x19W__\xFD[a\x1B>a\x1B$a\x1B\xE0V[`\x04\x83\x01T`\x03\x84\x01T`\x01`\x01`\xA0\x1B\x03\x16\x91\x90a\x1C\xE7V[_\x82\x81R`\x04` R`@\x81 \x81\x81U\x90`\x01\x82\x01\x81a\x1B^\x82\x82a2\xAAV[PP_`\x02\x83\x01\x81\x90U`\x03\x83\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x81\x16\x90\x91U`\x04\x84\x01\x82\x90U`\x05\x84\x01\x80T\x82\x16\x90U`\x06\x84\x01\x80T\x90\x91\x16\x90U`\x07\x90\x92\x01\x91\x90\x91UP`@Q\x82\x81R\x7Fx\xF5\x1Fb\xF7\xCF\x13\x81\xC6s\xC2~\xAE\x18}\xD6\xC5\x88\xDCf$\xCEYi}\xBB>\x1D|\x1B\xBC\xDF\x90` \x01a\x0E\xC8V[_`\x146\x10\x80\x15\x90a\x1B\xFBWP_T`\x01`\x01`\xA0\x1B\x03\x163\x14[\x15a\x1C+WP\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xEC6\x015``\x1C\x90V[P3\x90V[`@Q`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`$\x83\x01R\x83\x16`D\x82\x01R`d\x81\x01\x82\x90Ra\x1C\xE1\x90\x85\x90\x7F#\xB8r\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90`\x84\x01[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x93\x16\x92\x90\x92\x17\x90\x91Ra\x1F\xE7V[PPPPV[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16`$\x82\x01R`D\x81\x01\x82\x90Ra\x1D0\x90\x84\x90\x7F\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90`d\x01a\x1C}V[PPPV[_a\x1DC\x83` \x01Qa \xCBV[a\x1D\x94W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FInvalid input vector provided\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[a\x1D\xA1\x83`@\x01Qa!eV[a\x1D\xEDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FInvalid output vector provided\0\0`D\x82\x01R`d\x01a\x1D\x8BV[a\x1E*\x83_\x01Q\x84` \x01Q\x85`@\x01Q\x86``\x01Q`@Q` \x01a\x1E\x16\x94\x93\x92\x91\x90a?\xABV[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra!\xF2V[\x90Pa\x1ELa\x1E<\x83`@\x01Qa\"\x14V[\x83Q` \x85\x01Q\x84\x92\x91\x90a\" V[a\x1E\xBEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`<`$\x82\x01R\x7FTx merkle proof is not valid for`D\x82\x01R\x7F provided header and tx hash\0\0\0\0`d\x82\x01R`\x84\x01a\x1D\x8BV[a\x1E\xCD\x85\x85\x84`@\x01Qa\"[V[\x94\x93PPPPV[_\x82_\x01\x80Ta\x1E\xE4\x90a:5V[`@Qa\x1E\xF6\x92P\x85\x90` \x01a@\x1AV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P_a\x1F]\x83\x80`@\x01\x90a\x1F\"\x91\x90a=\x9AV[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RP\x86\x92Pa%\xAB\x91PPV[Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x84\x81\x10\x15a\x1F\xE0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`;`$\x82\x01R\x7FBitcoin transaction amount is lo`D\x82\x01R\x7Fwer than in accepted order.\0\0\0\0\0`d\x82\x01R`\x84\x01a\x1D\x8BV[PPPPPV[_a ;\x82`@Q\x80`@\x01`@R\x80` \x81R` \x01\x7FSafeERC20: low-level call failed\x81RP\x85`\x01`\x01`\xA0\x1B\x03\x16a'I\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x80Q\x90\x91P\x15a\x1D0W\x80\x80` \x01\x90Q\x81\x01\x90a Y\x91\x90a@\xE1V[a\x1D0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01R\x7Fot succeed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x1D\x8BV[___a \xD7\x84a'WV[\x90\x92P\x90P\x80\x15\x80a \xE9WP_\x19\x82\x14[\x15a \xF7WP_\x93\x92PPPV[_a!\x03\x83`\x01a?\x81V[\x90P_[\x82\x81\x10\x15a!XW\x85Q\x82\x10a!\"WP_\x95\x94PPPPPV[_a!-\x87\x84a'lV[\x90P_\x19\x81\x03a!CWP_\x96\x95PPPPPPV[a!M\x81\x84a?\x81V[\x92PP`\x01\x01a!\x07V[P\x93Q\x90\x93\x14\x93\x92PPPV[___a!q\x84a'WV[\x90\x92P\x90P\x80\x15\x80a!\x83WP_\x19\x82\x14[\x15a!\x91WP_\x93\x92PPPV[_a!\x9D\x83`\x01a?\x81V[\x90P_[\x82\x81\x10\x15a!XW\x85Q\x82\x10a!\xBCWP_\x95\x94PPPPPV[_a!\xC7\x87\x84a'\xB2V[\x90P_\x19\x81\x03a!\xDDWP_\x96\x95PPPPPPV[a!\xE7\x81\x84a?\x81V[\x92PP`\x01\x01a!\xA1V[_` _\x83Q` \x85\x01`\x02Z\xFAP` _` _`\x02Z\xFAPP_Q\x91\x90PV[`D\x81\x01Q_\x90a\x068V[_\x83\x85\x14\x80\x15a\".WP\x81\x15[\x80\x15a\"9WP\x82Q\x15[\x15a\"FWP`\x01a\x1E\xCDV[a\"R\x85\x84\x86\x85a(\x12V[\x95\x94PPPPPV[_\x83`\x01`\x01`\xA0\x1B\x03\x16c\x117d\xBE`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\"\x98W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\"\xBC\x91\x90aA\0V[\x90P_\x84`\x01`\x01`\xA0\x1B\x03\x16c+\x97\xBE$`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\"\xFBW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a#\x1F\x91\x90aA\0V[\x90P_\x80a#4a#/\x86a(\xB7V[a(\xC2V[\x90P\x83\x81\x03a#EW\x83\x91Pa#\xC2V[\x82\x81\x03a#TW\x82\x91Pa#\xC2V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FNot at current or previous diffi`D\x82\x01R\x7Fculty\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x1D\x8BV[_a#\xCC\x86a(\xE9V[\x90P_\x19\x81\x03a$DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FInvalid length of the headers ch`D\x82\x01R\x7Fain\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x1D\x8BV[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x03a$\xB3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01R\x7FInvalid headers chain\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x1D\x8BV[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFD\x81\x03a%\"W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FInsufficient work in a header\0\0\0`D\x82\x01R`d\x01a\x1D\x8BV[a%,\x87\x84a9YV[\x81\x10\x15a%\xA1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`3`$\x82\x01R\x7FInsufficient accumulated difficu`D\x82\x01R\x7Flty in header chain\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x1D\x8BV[PPPPPPPPV[`@\x80Q``\x81\x01\x82R_\x80\x82R` \x80\x83\x01\x82\x90R\x82\x84\x01\x82\x90R\x83Q\x80\x85\x01\x90\x94R\x81\x84R\x83\x01R\x90a%\xDF\x84a'WV[` \x83\x01R\x80\x82R\x81a%\xF1\x82a9\xF0V[\x90RP_\x80[\x82` \x01Q\x81\x10\x15a&\xF3W\x82Q_\x90a&\x12\x90\x88\x90a'\xB2V[\x84Q\x90\x91P_\x90a&$\x90\x89\x90a+\rV[\x90P_a&2`\x08\x84a9\xDDV[\x86Q\x90\x91P_\x90a&D\x90`\x08a?\x81V[\x8A\x81\x01` \x01\x83\x90 \x90\x91P\x80\x8A\x03a&~W`\x01\x96P\x83\x89_\x01\x81\x81Qa&l\x91\x90aA\x17V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90RPa&\xCEV[_a&\x8C\x8C\x8A_\x01Qa+\x83V[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a&\xADW`\x01`\x01`\xA0\x1B\x03\x81\x16` \x8B\x01R[_a&\xBB\x8D\x8B_\x01Qa,cV[\x90P\x80\x15a&\xCBW`@\x8B\x01\x81\x90R[PP[\x84\x88_\x01\x81\x81Qa&\xDF\x91\x90a?\x81V[\x90RPP`\x01\x90\x94\x01\x93Pa%\xF7\x92PPPV[P\x80a'AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FNo output found for scriptPubKey`D\x82\x01R`d\x01a\x1D\x8BV[PP\x92\x91PPV[``a\x1E\xCD\x84\x84_\x85a-CV[__a'c\x83_a.\x87V[\x91P\x91P\x91P\x91V[___a'y\x85\x85a0$V[\x90\x92P\x90P`\x01\x82\x01a'\x91W_\x19\x92PPPa\x068V[\x80a'\x9D\x83`%a?\x81V[a'\xA7\x91\x90a?\x81V[a\"R\x90`\x04a?\x81V[_a'\xBE\x82`\ta?\x81V[\x83Q\x10\x15a'\xCEWP_\x19a\x068V[_\x80a'\xE4\x85a'\xDF\x86`\x08a?\x81V[a.\x87V[\x90\x92P\x90P`\x01\x82\x01a'\xFCW_\x19\x92PPPa\x068V[\x80a(\x08\x83`\ta?\x81V[a\"R\x91\x90a?\x81V[_` \x84Qa(!\x91\x90aA7V[\x15a(-WP_a\x1E\xCDV[\x83Q_\x03a(<WP_a\x1E\xCDV[\x81\x85_[\x86Q\x81\x10\x15a(\xAAWa(T`\x02\x84aA7V[`\x01\x03a(xWa(qa(k\x88\x83\x01` \x01Q\x90V[\x83a0bV[\x91Pa(\x91V[a(\x8E\x82a(\x89\x89\x84\x01` \x01Q\x90V[a0bV[\x91P[`\x01\x92\x90\x92\x1C\x91a(\xA3` \x82a?\x81V[\x90Pa(@V[P\x90\x93\x14\x95\x94PPPPPV[_a\x068\x82_a0mV[_a\x068{\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83a1\x06V[_`P\x82Qa(\xF8\x91\x90aA7V[\x15a)\x05WP_\x19\x91\x90PV[P_\x80\x80[\x83Q\x81\x10\x15a+\x06W\x80\x15a)QWa)$\x84\x82\x84a1\x11V[a)QWP\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x93\x92PPPV[_a)\\\x85\x83a0mV[\x90Pa)j\x85\x83`Pa1:V[\x92P\x80a*\xAD\x84_\x81\x90P`\x08\x81~\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\x16\x90\x1B`\x08\x82\x90\x1C~\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\x16\x17\x90P`\x10\x81}\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\x16\x90\x1B`\x10\x82\x90\x1C}\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\x16\x17\x90P` \x81{\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\x16\x90\x1B` \x82\x90\x1C{\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\x16\x17\x90P`@\x81w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x1B`@\x82\x90\x1Cw\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x17\x90P`\x80\x81\x90\x1B`\x80\x82\x90\x1C\x17\x90P\x91\x90PV[\x11\x15a*\xDDWP\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFD\x94\x93PPPPV[a*\xE6\x81a(\xC2V[a*\xF0\x90\x85a?\x81V[\x93Pa*\xFF\x90P`P\x82a?\x81V[\x90Pa)\nV[PP\x91\x90PV[_\x80a+\x19\x84\x84a1_V[`\xC0\x1C\x90P_a\"R\x82d\xFF\0\0\0\xFF`\x08\x82\x81\x1C\x91\x82\x16e\xFF\0\0\0\xFF\0\x93\x90\x91\x1B\x92\x83\x16\x17`\x10\x90\x81\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16f\xFF\0\xFF\0\xFF\0\xFF\x92\x90\x92\x16g\xFF\0\xFF\0\xFF\0\xFF\0\x90\x93\x16\x92\x90\x92\x17\x90\x91\x1Ce\xFF\xFF\0\0\xFF\xFF\x16\x17` \x81\x81\x1C\x91\x90\x1B\x17\x90V[_\x82a+\x90\x83`\ta?\x81V[\x81Q\x81\x10a+\xA0Wa+\xA0a:\x80V[` \x91\x01\x01Q\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x7Fj\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x14a+\xF5WP_a\x068V[_\x83a,\x02\x84`\na?\x81V[\x81Q\x81\x10a,\x12Wa,\x12a:\x80V[\x01` \x01Q\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x91P`\xF8\x1C`\x14\x03a,\\W_a,R\x84`\x0Ba?\x81V[\x85\x01`\x14\x01Q\x92PP[P\x92\x91PPV[_\x82a,p\x83`\ta?\x81V[\x81Q\x81\x10a,\x80Wa,\x80a:\x80V[` \x91\x01\x01Q\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x7Fj\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x14a,\xD5WP_a\x068V[_\x83a,\xE2\x84`\na?\x81V[\x81Q\x81\x10a,\xF2Wa,\xF2a:\x80V[\x01` \x90\x81\x01Q\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x92P`\xF8\x1C\x90\x03a,\\W_a-3\x84`\x0Ba?\x81V[\x85\x01` \x01Q\x92PPP\x92\x91PPV[``\x82G\x10\x15a-\xBBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FAddress: insufficient balance fo`D\x82\x01R\x7Fr call\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x1D\x8BV[`\x01`\x01`\xA0\x1B\x03\x85\x16;a.\x12W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x1D\x8BV[__\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@Qa.-\x91\x90aAJV[_`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80_\x81\x14a.gW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a.lV[``\x91P[P\x91P\x91Pa.|\x82\x82\x86a1mV[\x97\x96PPPPPPPV[___a.\x94\x85\x85a1\xA6V[\x90P\x80`\xFF\x16_\x03a.\xC7W_\x85\x85\x81Q\x81\x10a.\xB3Wa.\xB3a:\x80V[\x01` \x01Q\x90\x93P`\xF8\x1C\x91Pa0\x1D\x90PV[\x83a.\xD3\x82`\x01aAUV[`\xFF\x16a.\xE0\x91\x90a?\x81V[\x85Q\x10\x15a.\xF5W_\x19_\x92P\x92PPa0\x1DV[_\x81`\xFF\x16`\x02\x03a/8Wa/-a/\x19a/\x12\x87`\x01a?\x81V[\x88\x90a1_V[b\xFF\xFF\0`\xE8\x82\x90\x1C\x16`\xF8\x91\x90\x91\x1C\x17\x90V[a\xFF\xFF\x16\x90Pa0\x13V[\x81`\xFF\x16`\x04\x03a/\x87Wa/za/Ta/\x12\x87`\x01a?\x81V[`\xD8\x81\x90\x1Cc\xFF\0\xFF\0\x16b\xFF\0\xFF`\xE8\x92\x90\x92\x1C\x91\x90\x91\x16\x17`\x10\x81\x81\x1B\x91\x90\x1C\x17\x90V[c\xFF\xFF\xFF\xFF\x16\x90Pa0\x13V[\x81`\xFF\x16`\x08\x03a0\x13Wa0\x06a/\xA3a/\x12\x87`\x01a?\x81V[`\xC0\x1Cd\xFF\0\0\0\xFF`\x08\x82\x81\x1C\x91\x82\x16e\xFF\0\0\0\xFF\0\x93\x90\x91\x1B\x92\x83\x16\x17`\x10\x90\x81\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16f\xFF\0\xFF\0\xFF\0\xFF\x92\x90\x92\x16g\xFF\0\xFF\0\xFF\0\xFF\0\x90\x93\x16\x92\x90\x92\x17\x90\x91\x1Ce\xFF\xFF\0\0\xFF\xFF\x16\x17` \x81\x81\x1C\x91\x90\x1B\x17\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P[`\xFF\x90\x91\x16\x92P\x90P[\x92P\x92\x90PV[_\x80a01\x83`%a?\x81V[\x84Q\x10\x15a0DWP_\x19\x90P_a0\x1DV[_\x80a0U\x86a'\xDF\x87`$a?\x81V[\x90\x97\x90\x96P\x94PPPPPV[_a\n\xD0\x83\x83a2*V[_\x80a0\x84a0}\x84`Ha?\x81V[\x85\x90a1_V[`\xE8\x1C\x90P_\x84a0\x96\x85`Ka?\x81V[\x81Q\x81\x10a0\xA6Wa0\xA6a:\x80V[\x01` \x01Q`\xF8\x1C\x90P_a0\xD8\x83_`\x10\x82b\xFF\xFF\xFF\x16\x90\x1C\x82a\xFF\0\x16`\x10\x84b\xFF\xFF\xFF\x16\x90\x1B\x17\x17\x90P\x91\x90PV[b\xFF\xFF\xFF\x16\x90P_a0\xEB`\x03\x84aAnV[`\xFF\x16\x90Pa0\xFC\x81a\x01\0aBjV[a.|\x90\x83a9YV[_a\n\xD0\x82\x84a9\x9DV[_\x80a1\x1D\x85\x85a2QV[\x90P\x82\x81\x14a1/W_\x91PPa\n\xD0V[P`\x01\x94\x93PPPPV[_` _\x83\x85` \x01\x87\x01`\x02Z\xFAP` _` _`\x02Z\xFAPP_Q\x93\x92PPPV[_a\n\xD0\x83\x83\x01` \x01Q\x90V[``\x83\x15a1|WP\x81a\n\xD0V[\x82Q\x15a1\x8CW\x82Q\x80\x84` \x01\xFD[\x81`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x1D\x8B\x91\x90aBuV[_\x82\x82\x81Q\x81\x10a1\xB9Wa1\xB9a:\x80V[\x01` \x01Q`\xF8\x1C`\xFF\x03a1\xD0WP`\x08a\x068V[\x82\x82\x81Q\x81\x10a1\xE2Wa1\xE2a:\x80V[\x01` \x01Q`\xF8\x1C`\xFE\x03a1\xF9WP`\x04a\x068V[\x82\x82\x81Q\x81\x10a2\x0BWa2\x0Ba:\x80V[\x01` \x01Q`\xF8\x1C`\xFD\x03a2\"WP`\x02a\x068V[P_\x92\x91PPV[_\x82_R\x81` R` _`@_`\x02Z\xFAP` _` _`\x02Z\xFAPP_Q\x92\x91PPV[_a\n\xD0a2`\x83`\x04a?\x81V[\x84\x01` \x01Q\x90V[`@Q\x80`\xA0\x01`@R\x80_\x81R` \x01a2\x90`@Q\x80` \x01`@R\x80``\x81RP\x90V[\x81R_` \x82\x01\x81\x90R`@\x82\x01\x81\x90R``\x90\x91\x01R\x90V[P\x80Ta2\xB6\x90a:5V[_\x82U\x80`\x1F\x10a2\xC5WPPV[`\x1F\x01` \x90\x04\x90_R` _ \x90\x81\x01\x90a2\xE1\x91\x90a3;V[PV[`@Q\x80a\x01\0\x01`@R\x80_\x81R` \x01a3\x0C`@Q\x80` \x01`@R\x80``\x81RP\x90V[\x81R_` \x82\x01\x81\x90R`@\x82\x01\x81\x90R``\x82\x01\x81\x90R`\x80\x82\x01\x81\x90R`\xA0\x82\x01\x81\x90R`\xC0\x90\x91\x01R\x90V[[\x80\x82\x11\x15a3OW_\x81U`\x01\x01a3<V[P\x90V[__`@\x83\x85\x03\x12\x15a3dW__\xFD[PP\x805\x92` \x90\x91\x015\x91PV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[_\x81Q` \x84Ra\x1E\xCD` \x85\x01\x82a3sV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15a3\xE5W\x81Q\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01a3\xC7V[P\x93\x94\x93PPPPV[_`@\x82\x01`@\x83R\x80\x85Q\x80\x83R``\x85\x01\x91P``\x81`\x05\x1B\x86\x01\x01\x92P` \x87\x01_[\x82\x81\x10\x15a4\xB1W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA0\x87\x86\x03\x01\x84R\x81Q\x80Q\x86R` \x81\x01Q`\xA0` \x88\x01Ra4c`\xA0\x88\x01\x82a3\xA1V[\x90P`\x01`\x01`\xA0\x1B\x03`@\x83\x01Q\x16`@\x88\x01R``\x82\x01Q``\x88\x01R`\x01`\x01`\xA0\x1B\x03`\x80\x83\x01Q\x16`\x80\x88\x01R\x80\x96PPP` \x82\x01\x91P` \x84\x01\x93P`\x01\x81\x01\x90Pa4\x15V[PPPP\x82\x81\x03` \x84\x01Ra\"R\x81\x85a3\xB5V[_` \x82\x84\x03\x12\x15a4\xD7W__\xFD[P\x91\x90PV[___``\x84\x86\x03\x12\x15a4\xEFW__\xFD[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a5\x0CW__\xFD[a5\x18\x86\x82\x87\x01a4\xC7V[\x93\x96\x93\x95PPPP`@\x91\x90\x91\x015\x90V[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a5@W__\xFD[\x91\x90PV[____`\x80\x85\x87\x03\x12\x15a5XW__\xFD[\x845\x93P` \x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a5uW__\xFD[a5\x81\x87\x82\x88\x01a4\xC7V[\x93PPa5\x90`@\x86\x01a5*V[\x93\x96\x92\x95P\x92\x93``\x015\x92PPV[_` \x82\x84\x03\x12\x15a5\xB0W__\xFD[P5\x91\x90PV[\x85\x81R`\xA0` \x82\x01R_a5\xCF`\xA0\x83\x01\x87a3\xA1V[\x90P`\x01`\x01`\xA0\x1B\x03\x85\x16`@\x83\x01R\x83``\x83\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x80\x83\x01R\x96\x95PPPPPPV[\x88\x81Ra\x01\0` \x82\x01R_a6\x19a\x01\0\x83\x01\x8Aa3\xA1V[`@\x83\x01\x98\x90\x98RP`\x01`\x01`\xA0\x1B\x03\x95\x86\x16``\x82\x01R`\x80\x81\x01\x94\x90\x94R\x91\x84\x16`\xA0\x84\x01R\x90\x92\x16`\xC0\x82\x01R`\xE0\x01R\x92\x91PPV[_` \x82\x84\x03\x12\x15a6dW__\xFD[a\n\xD0\x82a5*V[___``\x84\x86\x03\x12\x15a6\x7FW__\xFD[\x835\x92Pa6\x8F` \x85\x01a5*V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`@\x80\x82R\x83Q\x90\x82\x01\x81\x90R_\x90` \x85\x01\x90``\x84\x01\x90\x83[\x81\x81\x10\x15a7\x11W\x83Q\x80Q\x84R`\x01`\x01`\xA0\x1B\x03` \x82\x01Q\x16` \x85\x01R`@\x81\x01Q`@\x85\x01R`\x01`\x01`\xA0\x1B\x03``\x82\x01Q\x16``\x85\x01RP`\x80\x83\x01\x92P` \x84\x01\x93P`\x01\x81\x01\x90Pa6\xBBV[PP\x83\x81\x03` \x85\x01Ra7%\x81\x86a3\xB5V[\x96\x95PPPPPPV[`@\x80\x82R\x83Q\x90\x82\x01\x81\x90R_\x90` \x85\x01\x90``\x84\x01\x90\x83[\x81\x81\x10\x15a7\x11W\x83Q\x80Q\x84R` \x81\x01Q` \x85\x01R`\x01`\x01`\xA0\x1B\x03`@\x82\x01Q\x16`@\x85\x01R``\x81\x01Q``\x85\x01R`\x01`\x01`\xA0\x1B\x03`\x80\x82\x01Q\x16`\x80\x85\x01R`\x01`\x01`\xA0\x1B\x03`\xA0\x82\x01Q\x16`\xA0\x85\x01R`\xC0\x81\x01Q`\xC0\x85\x01RP`\xE0\x83\x01\x92P` \x84\x01\x93P`\x01\x81\x01\x90Pa7JV[___``\x84\x86\x03\x12\x15a7\xD9W__\xFD[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a7\xF6W__\xFD[\x84\x01`\x80\x81\x87\x03\x12\x15a8\x07W__\xFD[\x91P`@\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a8\"W__\xFD[\x84\x01``\x81\x87\x03\x12\x15a83W__\xFD[\x80\x91PP\x92P\x92P\x92V[_`@\x82\x01`@\x83R\x80\x85Q\x80\x83R``\x85\x01\x91P``\x81`\x05\x1B\x86\x01\x01\x92P` \x87\x01_[\x82\x81\x10\x15a4\xB1W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA0\x87\x86\x03\x01\x84R\x81Q\x80Q\x86R` \x81\x01Qa\x01\0` \x88\x01Ra8\xB4a\x01\0\x88\x01\x82a3\xA1V[\x90P`@\x82\x01Q`@\x88\x01R`\x01`\x01`\xA0\x1B\x03``\x83\x01Q\x16``\x88\x01R`\x80\x82\x01Q`\x80\x88\x01R`\x01`\x01`\xA0\x1B\x03`\xA0\x83\x01Q\x16`\xA0\x88\x01R`\xC0\x82\x01Qa9\n`\xC0\x89\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`\xE0\x91\x82\x01Q\x96\x90\x91\x01\x95\x90\x95R` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a8dV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x068Wa\x068a9,V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_\x82a9\xABWa9\xABa9pV[P\x04\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x01`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x068Wa\x068a9,V[__\x19\x82\x03a:\x01Wa:\x01a9,V[P`\x01\x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[`\x01\x81\x81\x1C\x90\x82\x16\x80a:IW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a4\xD7W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_\x82`\x1F\x83\x01\x12a:\xBCW__\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a:\xD6Wa:\xD6a:\x08V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a;\x05Wa;\x05a:\x08V[`@R\x81\x81R\x83\x82\x01` \x01\x85\x10\x15a;\x1CW__\xFD[\x81` \x85\x01` \x83\x017_\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[_` \x826\x03\x12\x15a;HW__\xFD[`@Q` \x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a;kWa;ka:\x08V[`@R\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a;\x84W__\xFD[a;\x906\x82\x86\x01a:\xADV[\x82RP\x92\x91PPV[`\x1F\x82\x11\x15a\x1D0W\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a;\xBEWP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x1F\xE0W_\x81U`\x01\x01a;\xCAV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a;\xF7Wa;\xF7a:\x08V[a<\x0B\x81a<\x05\x84Ta:5V[\x84a;\x99V[` `\x1F\x82\x11`\x01\x81\x14a<=W_\x83\x15a<&WP\x84\x82\x01Q[_\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua\x1F\xE0V[_\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a<lW\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a<LV[P\x84\x82\x10\x15a<\x89W\x86\x84\x01Q_\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[\x81\x83R\x81\x81` \x85\x017P_` \x82\x84\x01\x01R_` `\x1F\x19`\x1F\x84\x01\x16\x84\x01\x01\x90P\x92\x91PPV[_\x815\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x836\x03\x01\x81\x12a<\xF3W__\xFD[\x82\x01` \x81\x01\x905g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a=\x0FW__\xFD[\x806\x03\x82\x13\x15a=\x1DW__\xFD[` \x85Ra\"R` \x86\x01\x82\x84a<\x98V[`\x80\x81R_a=A`\x80\x83\x01\x87a<\xC1V[` \x83\x01\x95\x90\x95RP`@\x81\x01\x92\x90\x92R`\x01`\x01`\xA0\x1B\x03\x16``\x90\x91\x01R\x91\x90PV[\x84\x81R`\x80` \x82\x01R_a=~`\x80\x83\x01\x86a<\xC1V[`\x01`\x01`\xA0\x1B\x03\x94\x90\x94\x16`@\x83\x01RP``\x01R\x92\x91PPV[__\x835\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x846\x03\x01\x81\x12a=\xCDW__\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a=\xE7W__\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a0\x1DW__\xFD[` \x81R_a\x1E\xCD` \x83\x01\x84\x86a<\x98V[\x805\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x81\x14a5@W__\xFD[_`\x80\x826\x03\x12\x15a>MW__\xFD[`@Q`\x80\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a>pWa>pa:\x08V[`@Ra>|\x83a>\x0EV[\x81R` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a>\x97W__\xFD[a>\xA36\x82\x86\x01a:\xADV[` \x83\x01RP`@\x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a>\xC2W__\xFD[a>\xCE6\x82\x86\x01a:\xADV[`@\x83\x01RPa>\xE0``\x84\x01a>\x0EV[``\x82\x01R\x92\x91PPV[_``\x826\x03\x12\x15a>\xFBW__\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a?\x1EWa?\x1Ea:\x08V[`@R\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a?7W__\xFD[a?C6\x82\x86\x01a:\xADV[\x82RP` \x83\x81\x015\x90\x82\x01R`@\x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a?iW__\xFD[a?u6\x82\x86\x01a:\xADV[`@\x83\x01RP\x92\x91PPV[\x80\x82\x01\x80\x82\x11\x15a\x068Wa\x068a9,V[_\x81Q\x80` \x84\x01\x85^_\x93\x01\x92\x83RP\x90\x91\x90PV[\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85\x16\x81R_a?\xE7a?\xE1`\x04\x84\x01\x87a?\x94V[\x85a?\x94V[\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x93\x90\x93\x16\x83RPP`\x04\x01\x93\x92PPPV[\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83`\xF8\x1B\x16\x81R__\x83Ta@O\x81a:5V[`\x01\x82\x16\x80\x15a@fW`\x01\x81\x14a@\x9FWa@\xD5V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x83\x16`\x01\x87\x01R`\x01\x82\x15\x15\x83\x02\x87\x01\x01\x93Pa@\xD5V[\x86_R` _ _[\x83\x81\x10\x15a@\xCAW\x81T`\x01\x82\x8A\x01\x01R`\x01\x82\x01\x91P` \x81\x01\x90Pa@\xA8V[PP`\x01\x82\x87\x01\x01\x93P[P\x91\x96\x95PPPPPPV[_` \x82\x84\x03\x12\x15a@\xF1W__\xFD[\x81Q\x80\x15\x15\x81\x14a\n\xD0W__\xFD[_` \x82\x84\x03\x12\x15aA\x10W__\xFD[PQ\x91\x90PV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x068Wa\x068a9,V[_\x82aAEWaAEa9pV[P\x06\x90V[_a\n\xD0\x82\x84a?\x94V[`\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x068Wa\x068a9,V[`\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x068Wa\x068a9,V[`\x01\x81[`\x01\x84\x11\x15aA\xC2W\x80\x85\x04\x81\x11\x15aA\xA6WaA\xA6a9,V[`\x01\x84\x16\x15aA\xB4W\x90\x81\x02\x90[`\x01\x93\x90\x93\x1C\x92\x80\x02aA\x8BV[\x93P\x93\x91PPV[_\x82aA\xD8WP`\x01a\x068V[\x81aA\xE4WP_a\x068V[\x81`\x01\x81\x14aA\xFAW`\x02\x81\x14aB\x04WaB V[`\x01\x91PPa\x068V[`\xFF\x84\x11\x15aB\x15WaB\x15a9,V[PP`\x01\x82\x1Ba\x068V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15aBCWP\x81\x81\na\x068V[aBO_\x19\x84\x84aA\x87V[\x80_\x19\x04\x82\x11\x15aBbWaBba9,V[\x02\x93\x92PPPV[_a\n\xD0\x83\x83aA\xCAV[` \x81R_a\n\xD0` \x83\x01\x84a3sV\xFE\xA2dipfsX\"\x12 1\xF6\x9B(m\xB9\x91\xC7\xBE\xEB$\x13s4\x18\xFC&\xCA\x86}\x0F\xB2\x91\xFBQ\xA4l\x03~Rv\x98dsolcC\0\x08\x1B\x003",
    );
    /**```solidity
struct AcceptedBtcBuyOrder { uint256 orderId; uint256 amountBtc; address ercToken; uint256 ercAmount; address requester; address accepter; uint256 acceptTime; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AcceptedBtcBuyOrder {
        pub orderId: alloy::sol_types::private::primitives::aliases::U256,
        pub amountBtc: alloy::sol_types::private::primitives::aliases::U256,
        pub ercToken: alloy::sol_types::private::Address,
        pub ercAmount: alloy::sol_types::private::primitives::aliases::U256,
        pub requester: alloy::sol_types::private::Address,
        pub accepter: alloy::sol_types::private::Address,
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
    /**```solidity
struct AcceptedBtcSellOrder { uint256 orderId; BitcoinAddress bitcoinAddress; uint256 amountBtc; address ercToken; uint256 ercAmount; address requester; address accepter; uint256 acceptTime; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AcceptedBtcSellOrder {
        pub orderId: alloy::sol_types::private::primitives::aliases::U256,
        pub bitcoinAddress: <BitcoinAddress as alloy::sol_types::SolType>::RustType,
        pub amountBtc: alloy::sol_types::private::primitives::aliases::U256,
        pub ercToken: alloy::sol_types::private::Address,
        pub ercAmount: alloy::sol_types::private::primitives::aliases::U256,
        pub requester: alloy::sol_types::private::Address,
        pub accepter: alloy::sol_types::private::Address,
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
    /**```solidity
struct BitcoinAddress { bytes scriptPubKey; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BitcoinAddress {
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
    /**```solidity
struct BtcBuyOrder { uint256 amountBtc; BitcoinAddress bitcoinAddress; address offeringToken; uint256 offeringAmount; address requester; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BtcBuyOrder {
        pub amountBtc: alloy::sol_types::private::primitives::aliases::U256,
        pub bitcoinAddress: <BitcoinAddress as alloy::sol_types::SolType>::RustType,
        pub offeringToken: alloy::sol_types::private::Address,
        pub offeringAmount: alloy::sol_types::private::primitives::aliases::U256,
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
    /**```solidity
struct BtcSellOrder { uint256 amountBtc; address askingToken; uint256 askingAmount; address requester; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BtcSellOrder {
        pub amountBtc: alloy::sol_types::private::primitives::aliases::U256,
        pub askingToken: alloy::sol_types::private::Address,
        pub askingAmount: alloy::sol_types::private::primitives::aliases::U256,
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
                195u8,
                154u8,
                26u8,
                93u8,
                220u8,
                14u8,
                133u8,
                201u8,
                85u8,
                254u8,
                46u8,
                26u8,
                190u8,
                180u8,
                60u8,
                148u8,
                206u8,
                24u8,
                50u8,
                46u8,
                117u8,
                187u8,
                61u8,
                68u8,
                232u8,
                15u8,
                117u8,
                159u8,
                249u8,
                208u8,
                52u8,
                185u8,
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
                101u8,
                62u8,
                13u8,
                129u8,
                242u8,
                201u8,
                155u8,
                235u8,
                163u8,
                89u8,
                223u8,
                177u8,
                123u8,
                73u8,
                154u8,
                92u8,
                255u8,
                43u8,
                233u8,
                217u8,
                80u8,
                81u8,
                72u8,
                82u8,
                34u8,
                77u8,
                248u8,
                192u8,
                151u8,
                194u8,
                25u8,
                33u8,
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
                62u8,
                94u8,
                163u8,
                88u8,
                233u8,
                235u8,
                76u8,
                223u8,
                68u8,
                205u8,
                199u8,
                121u8,
                56u8,
                173u8,
                232u8,
                7u8,
                75u8,
                18u8,
                64u8,
                166u8,
                216u8,
                192u8,
                253u8,
                19u8,
                114u8,
                134u8,
                113u8,
                184u8,
                46u8,
                128u8,
                10u8,
                214u8,
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
                120u8,
                245u8,
                31u8,
                98u8,
                247u8,
                207u8,
                19u8,
                129u8,
                198u8,
                115u8,
                194u8,
                126u8,
                174u8,
                24u8,
                125u8,
                214u8,
                197u8,
                136u8,
                220u8,
                102u8,
                36u8,
                206u8,
                89u8,
                105u8,
                125u8,
                187u8,
                62u8,
                29u8,
                124u8,
                27u8,
                188u8,
                223u8,
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
                152u8,
                199u8,
                198u8,
                128u8,
                64u8,
                61u8,
                71u8,
                64u8,
                61u8,
                234u8,
                74u8,
                87u8,
                13u8,
                14u8,
                108u8,
                87u8,
                22u8,
                83u8,
                140u8,
                73u8,
                66u8,
                14u8,
                244u8,
                113u8,
                206u8,
                196u8,
                40u8,
                245u8,
                165u8,
                133u8,
                44u8,
                6u8,
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
                255u8,
                28u8,
                226u8,
                16u8,
                222u8,
                252u8,
                211u8,
                186u8,
                26u8,
                223u8,
                118u8,
                201u8,
                65u8,
                154u8,
                7u8,
                88u8,
                250u8,
                96u8,
                253u8,
                62u8,
                179u8,
                140u8,
                123u8,
                217u8,
                65u8,
                143u8,
                96u8,
                181u8,
                117u8,
                183u8,
                110u8,
                36u8,
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
                180u8,
                201u8,
                141u8,
                226u8,
                16u8,
                105u8,
                107u8,
                60u8,
                242u8,
                30u8,
                153u8,
                51u8,
                92u8,
                30u8,
                227u8,
                160u8,
                174u8,
                52u8,
                162u8,
                103u8,
                19u8,
                65u8,
                42u8,
                74u8,
                221u8,
                232u8,
                175u8,
                89u8,
                97u8,
                118u8,
                243u8,
                126u8,
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
                207u8,
                86u8,
                16u8,
                97u8,
                219u8,
                120u8,
                247u8,
                188u8,
                81u8,
                141u8,
                55u8,
                254u8,
                134u8,
                113u8,
                133u8,
                20u8,
                198u8,
                64u8,
                204u8,
                197u8,
                195u8,
                241u8,
                41u8,
                56u8,
                40u8,
                185u8,
                85u8,
                230u8,
                143u8,
                25u8,
                245u8,
                251u8,
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
                195u8,
                64u8,
                231u8,
                172u8,
                72u8,
                220u8,
                128u8,
                238u8,
                121u8,
                63u8,
                198u8,
                38u8,
                105u8,
                96u8,
                189u8,
                95u8,
                27u8,
                210u8,
                27u8,
                233u8,
                28u8,
                138u8,
                149u8,
                226u8,
                24u8,
                23u8,
                129u8,
                19u8,
                247u8,
                158u8,
                23u8,
                180u8,
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
                60u8,
                212u8,
                117u8,
                176u8,
                146u8,
                232u8,
                179u8,
                121u8,
                246u8,
                186u8,
                13u8,
                158u8,
                14u8,
                12u8,
                143u8,
                48u8,
                112u8,
                94u8,
                115u8,
                50u8,
                29u8,
                197u8,
                201u8,
                248u8,
                12u8,
                228u8,
                173u8,
                56u8,
                219u8,
                123u8,
                225u8,
                170u8,
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
        pub _relay: alloy::sol_types::private::Address,
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
    /**Function with signature `REQUEST_EXPIRATION_SECONDS()` and selector `0xd1920ff0`.
```solidity
function REQUEST_EXPIRATION_SECONDS() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct REQUEST_EXPIRATION_SECONDSCall {}
    ///Container type for the return parameters of the [`REQUEST_EXPIRATION_SECONDS()`](REQUEST_EXPIRATION_SECONDSCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct REQUEST_EXPIRATION_SECONDSReturn {
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
            type Return = REQUEST_EXPIRATION_SECONDSReturn;
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
    /**Function with signature `acceptBtcBuyOrder(uint256,uint256)` and selector `0x11c137aa`.
```solidity
function acceptBtcBuyOrder(uint256 id, uint256 amountBtc) external returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct acceptBtcBuyOrderCall {
        pub id: alloy::sol_types::private::primitives::aliases::U256,
        pub amountBtc: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`acceptBtcBuyOrder(uint256,uint256)`](acceptBtcBuyOrderCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct acceptBtcBuyOrderReturn {
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
            type Return = acceptBtcBuyOrderReturn;
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
    /**Function with signature `acceptBtcSellOrder(uint256,(bytes),uint256)` and selector `0x210ec181`.
```solidity
function acceptBtcSellOrder(uint256 id, BitcoinAddress memory bitcoinAddress, uint256 amountBtc) external returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct acceptBtcSellOrderCall {
        pub id: alloy::sol_types::private::primitives::aliases::U256,
        pub bitcoinAddress: <BitcoinAddress as alloy::sol_types::SolType>::RustType,
        pub amountBtc: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`acceptBtcSellOrder(uint256,(bytes),uint256)`](acceptBtcSellOrderCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct acceptBtcSellOrderReturn {
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
            type Return = acceptBtcSellOrderReturn;
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
    /**Function with signature `acceptedBtcBuyOrders(uint256)` and selector `0xbd2a7e3e`.
```solidity
function acceptedBtcBuyOrders(uint256) external view returns (uint256 orderId, uint256 amountBtc, address ercToken, uint256 ercAmount, address requester, address accepter, uint256 acceptTime);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct acceptedBtcBuyOrdersCall {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`acceptedBtcBuyOrders(uint256)`](acceptedBtcBuyOrdersCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct acceptedBtcBuyOrdersReturn {
        pub orderId: alloy::sol_types::private::primitives::aliases::U256,
        pub amountBtc: alloy::sol_types::private::primitives::aliases::U256,
        pub ercToken: alloy::sol_types::private::Address,
        pub ercAmount: alloy::sol_types::private::primitives::aliases::U256,
        pub requester: alloy::sol_types::private::Address,
        pub accepter: alloy::sol_types::private::Address,
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
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for acceptedBtcBuyOrdersCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
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
                    > as alloy_sol_types::SolType>::tokenize(&self._0),
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
    /**Function with signature `acceptedBtcSellOrders(uint256)` and selector `0x4145640a`.
```solidity
function acceptedBtcSellOrders(uint256) external view returns (uint256 orderId, BitcoinAddress memory bitcoinAddress, uint256 amountBtc, address ercToken, uint256 ercAmount, address requester, address accepter, uint256 acceptTime);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct acceptedBtcSellOrdersCall {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`acceptedBtcSellOrders(uint256)`](acceptedBtcSellOrdersCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct acceptedBtcSellOrdersReturn {
        pub orderId: alloy::sol_types::private::primitives::aliases::U256,
        pub bitcoinAddress: <BitcoinAddress as alloy::sol_types::SolType>::RustType,
        pub amountBtc: alloy::sol_types::private::primitives::aliases::U256,
        pub ercToken: alloy::sol_types::private::Address,
        pub ercAmount: alloy::sol_types::private::primitives::aliases::U256,
        pub requester: alloy::sol_types::private::Address,
        pub accepter: alloy::sol_types::private::Address,
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
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for acceptedBtcSellOrdersCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
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
                    > as alloy_sol_types::SolType>::tokenize(&self._0),
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
    /**Function with signature `btcBuyOrders(uint256)` and selector `0x3af3fc7e`.
```solidity
function btcBuyOrders(uint256) external view returns (uint256 amountBtc, BitcoinAddress memory bitcoinAddress, address offeringToken, uint256 offeringAmount, address requester);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct btcBuyOrdersCall {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`btcBuyOrders(uint256)`](btcBuyOrdersCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct btcBuyOrdersReturn {
        pub amountBtc: alloy::sol_types::private::primitives::aliases::U256,
        pub bitcoinAddress: <BitcoinAddress as alloy::sol_types::SolType>::RustType,
        pub offeringToken: alloy::sol_types::private::Address,
        pub offeringAmount: alloy::sol_types::private::primitives::aliases::U256,
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
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for btcBuyOrdersCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
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
                    > as alloy_sol_types::SolType>::tokenize(&self._0),
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
    /**Function with signature `btcSellOrders(uint256)` and selector `0xecca2c36`.
```solidity
function btcSellOrders(uint256) external view returns (uint256 amountBtc, address askingToken, uint256 askingAmount, address requester);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct btcSellOrdersCall {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`btcSellOrders(uint256)`](btcSellOrdersCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct btcSellOrdersReturn {
        pub amountBtc: alloy::sol_types::private::primitives::aliases::U256,
        pub askingToken: alloy::sol_types::private::Address,
        pub askingAmount: alloy::sol_types::private::primitives::aliases::U256,
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
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for btcSellOrdersCall {
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
                    > as alloy_sol_types::SolType>::tokenize(&self._0),
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
    /**Function with signature `cancelAcceptedBtcBuyOrder(uint256)` and selector `0xc56a4526`.
```solidity
function cancelAcceptedBtcBuyOrder(uint256 id) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct cancelAcceptedBtcBuyOrderCall {
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
    /**Function with signature `cancelAcceptedBtcSellOrder(uint256)` and selector `0xdf69b14f`.
```solidity
function cancelAcceptedBtcSellOrder(uint256 id) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct cancelAcceptedBtcSellOrderCall {
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
    /**Function with signature `getOpenAcceptedBtcBuyOrders()` and selector `0x6a8cde3a`.
```solidity
function getOpenAcceptedBtcBuyOrders() external view returns (AcceptedBtcBuyOrder[] memory, uint256[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOpenAcceptedBtcBuyOrdersCall {}
    ///Container type for the return parameters of the [`getOpenAcceptedBtcBuyOrders()`](getOpenAcceptedBtcBuyOrdersCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOpenAcceptedBtcBuyOrdersReturn {
        pub _0: alloy::sol_types::private::Vec<
            <AcceptedBtcBuyOrder as alloy::sol_types::SolType>::RustType,
        >,
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
                    Self {}
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
    /**Function with signature `getOpenAcceptedBtcSellOrders()` and selector `0x9cc6722e`.
```solidity
function getOpenAcceptedBtcSellOrders() external view returns (AcceptedBtcSellOrder[] memory, uint256[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOpenAcceptedBtcSellOrdersCall {}
    ///Container type for the return parameters of the [`getOpenAcceptedBtcSellOrders()`](getOpenAcceptedBtcSellOrdersCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOpenAcceptedBtcSellOrdersReturn {
        pub _0: alloy::sol_types::private::Vec<
            <AcceptedBtcSellOrder as alloy::sol_types::SolType>::RustType,
        >,
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
                    Self {}
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
    /**Function with signature `getOpenBtcBuyOrders()` and selector `0x1dfe7595`.
```solidity
function getOpenBtcBuyOrders() external view returns (BtcBuyOrder[] memory, uint256[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOpenBtcBuyOrdersCall {}
    ///Container type for the return parameters of the [`getOpenBtcBuyOrders()`](getOpenBtcBuyOrdersCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOpenBtcBuyOrdersReturn {
        pub _0: alloy::sol_types::private::Vec<
            <BtcBuyOrder as alloy::sol_types::SolType>::RustType,
        >,
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
                    Self {}
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
    /**Function with signature `getOpenBtcSellOrders()` and selector `0x6811a311`.
```solidity
function getOpenBtcSellOrders() external view returns (BtcSellOrder[] memory, uint256[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOpenBtcSellOrdersCall {}
    ///Container type for the return parameters of the [`getOpenBtcSellOrders()`](getOpenBtcSellOrdersCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOpenBtcSellOrdersReturn {
        pub _0: alloy::sol_types::private::Vec<
            <BtcSellOrder as alloy::sol_types::SolType>::RustType,
        >,
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
                    Self {}
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
function getTrustedForwarder() external view returns (address forwarder);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTrustedForwarderCall {}
    ///Container type for the return parameters of the [`getTrustedForwarder()`](getTrustedForwarderCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTrustedForwarderReturn {
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
    /**Function with signature `isTrustedForwarder(address)` and selector `0x572b6c05`.
```solidity
function isTrustedForwarder(address forwarder) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isTrustedForwarderCall {
        pub forwarder: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`isTrustedForwarder(address)`](isTrustedForwarderCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isTrustedForwarderReturn {
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
            type Return = isTrustedForwarderReturn;
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
    /**Function with signature `placeBtcBuyOrder(uint256,(bytes),address,uint256)` and selector `0x364f1ec0`.
```solidity
function placeBtcBuyOrder(uint256 amountBtc, BitcoinAddress memory bitcoinAddress, address sellingToken, uint256 saleAmount) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct placeBtcBuyOrderCall {
        pub amountBtc: alloy::sol_types::private::primitives::aliases::U256,
        pub bitcoinAddress: <BitcoinAddress as alloy::sol_types::SolType>::RustType,
        pub sellingToken: alloy::sol_types::private::Address,
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
    /**Function with signature `placeBtcSellOrder(uint256,address,uint256)` and selector `0x5b8fe042`.
```solidity
function placeBtcSellOrder(uint256 amountBtc, address buyingToken, uint256 buyAmount) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct placeBtcSellOrderCall {
        pub amountBtc: alloy::sol_types::private::primitives::aliases::U256,
        pub buyingToken: alloy::sol_types::private::Address,
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
    /**Function with signature `proofBtcBuyOrder(uint256,(bytes4,bytes,bytes,bytes4),(bytes,uint256,bytes))` and selector `0xb522c133`.
```solidity
function proofBtcBuyOrder(uint256 id, BitcoinTx.Info memory transaction, BitcoinTx.Proof memory proof) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proofBtcBuyOrderCall {
        pub id: alloy::sol_types::private::primitives::aliases::U256,
        pub transaction: <BitcoinTx::Info as alloy::sol_types::SolType>::RustType,
        pub proof: <BitcoinTx::Proof as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`proofBtcBuyOrder(uint256,(bytes4,bytes,bytes,bytes4),(bytes,uint256,bytes))`](proofBtcBuyOrderCall) function.
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
            const SIGNATURE: &'static str = "proofBtcBuyOrder(uint256,(bytes4,bytes,bytes,bytes4),(bytes,uint256,bytes))";
            const SELECTOR: [u8; 4] = [181u8, 34u8, 193u8, 51u8];
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
    /**Function with signature `proofBtcSellOrder(uint256,(bytes4,bytes,bytes,bytes4),(bytes,uint256,bytes))` and selector `0x72a546c6`.
```solidity
function proofBtcSellOrder(uint256 id, BitcoinTx.Info memory transaction, BitcoinTx.Proof memory proof) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proofBtcSellOrderCall {
        pub id: alloy::sol_types::private::primitives::aliases::U256,
        pub transaction: <BitcoinTx::Info as alloy::sol_types::SolType>::RustType,
        pub proof: <BitcoinTx::Proof as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`proofBtcSellOrder(uint256,(bytes4,bytes,bytes,bytes4),(bytes,uint256,bytes))`](proofBtcSellOrderCall) function.
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
            const SIGNATURE: &'static str = "proofBtcSellOrder(uint256,(bytes4,bytes,bytes,bytes4),(bytes,uint256,bytes))";
            const SELECTOR: [u8; 4] = [114u8, 165u8, 70u8, 198u8];
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
    /**Function with signature `withdrawBtcBuyOrder(uint256)` and selector `0x506a109d`.
```solidity
function withdrawBtcBuyOrder(uint256 id) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct withdrawBtcBuyOrderCall {
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
    /**Function with signature `withdrawBtcSellOrder(uint256)` and selector `0xa383013b`.
```solidity
function withdrawBtcSellOrder(uint256 id) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct withdrawBtcSellOrderCall {
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
    ///Container for all the [`BtcMarketPlace`](self) function calls.
    pub enum BtcMarketPlaceCalls {
        REQUEST_EXPIRATION_SECONDS(REQUEST_EXPIRATION_SECONDSCall),
        acceptBtcBuyOrder(acceptBtcBuyOrderCall),
        acceptBtcSellOrder(acceptBtcSellOrderCall),
        acceptedBtcBuyOrders(acceptedBtcBuyOrdersCall),
        acceptedBtcSellOrders(acceptedBtcSellOrdersCall),
        btcBuyOrders(btcBuyOrdersCall),
        btcSellOrders(btcSellOrdersCall),
        cancelAcceptedBtcBuyOrder(cancelAcceptedBtcBuyOrderCall),
        cancelAcceptedBtcSellOrder(cancelAcceptedBtcSellOrderCall),
        getOpenAcceptedBtcBuyOrders(getOpenAcceptedBtcBuyOrdersCall),
        getOpenAcceptedBtcSellOrders(getOpenAcceptedBtcSellOrdersCall),
        getOpenBtcBuyOrders(getOpenBtcBuyOrdersCall),
        getOpenBtcSellOrders(getOpenBtcSellOrdersCall),
        getTrustedForwarder(getTrustedForwarderCall),
        isTrustedForwarder(isTrustedForwarderCall),
        placeBtcBuyOrder(placeBtcBuyOrderCall),
        placeBtcSellOrder(placeBtcSellOrderCall),
        proofBtcBuyOrder(proofBtcBuyOrderCall),
        proofBtcSellOrder(proofBtcSellOrderCall),
        withdrawBtcBuyOrder(withdrawBtcBuyOrderCall),
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
            [114u8, 165u8, 70u8, 198u8],
            [156u8, 198u8, 114u8, 46u8],
            [163u8, 131u8, 1u8, 59u8],
            [181u8, 34u8, 193u8, 51u8],
            [189u8, 42u8, 126u8, 62u8],
            [197u8, 106u8, 69u8, 38u8],
            [206u8, 27u8, 129u8, 95u8],
            [209u8, 146u8, 15u8, 240u8],
            [223u8, 105u8, 177u8, 79u8],
            [236u8, 202u8, 44u8, 54u8],
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
        #[allow(unsafe_code, non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
                bool,
            ) -> alloy_sol_types::Result<BtcMarketPlaceCalls>] = &[
                {
                    fn acceptBtcBuyOrder(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BtcMarketPlaceCalls> {
                        <acceptBtcBuyOrderCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BtcMarketPlaceCalls::acceptBtcBuyOrder)
                    }
                    acceptBtcBuyOrder
                },
                {
                    fn getOpenBtcBuyOrders(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BtcMarketPlaceCalls> {
                        <getOpenBtcBuyOrdersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BtcMarketPlaceCalls::getOpenBtcBuyOrders)
                    }
                    getOpenBtcBuyOrders
                },
                {
                    fn acceptBtcSellOrder(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BtcMarketPlaceCalls> {
                        <acceptBtcSellOrderCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BtcMarketPlaceCalls::acceptBtcSellOrder)
                    }
                    acceptBtcSellOrder
                },
                {
                    fn placeBtcBuyOrder(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BtcMarketPlaceCalls> {
                        <placeBtcBuyOrderCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BtcMarketPlaceCalls::placeBtcBuyOrder)
                    }
                    placeBtcBuyOrder
                },
                {
                    fn btcBuyOrders(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BtcMarketPlaceCalls> {
                        <btcBuyOrdersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BtcMarketPlaceCalls::btcBuyOrders)
                    }
                    btcBuyOrders
                },
                {
                    fn acceptedBtcSellOrders(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BtcMarketPlaceCalls> {
                        <acceptedBtcSellOrdersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BtcMarketPlaceCalls::acceptedBtcSellOrders)
                    }
                    acceptedBtcSellOrders
                },
                {
                    fn withdrawBtcBuyOrder(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BtcMarketPlaceCalls> {
                        <withdrawBtcBuyOrderCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BtcMarketPlaceCalls::withdrawBtcBuyOrder)
                    }
                    withdrawBtcBuyOrder
                },
                {
                    fn isTrustedForwarder(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BtcMarketPlaceCalls> {
                        <isTrustedForwarderCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BtcMarketPlaceCalls::isTrustedForwarder)
                    }
                    isTrustedForwarder
                },
                {
                    fn placeBtcSellOrder(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BtcMarketPlaceCalls> {
                        <placeBtcSellOrderCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BtcMarketPlaceCalls::placeBtcSellOrder)
                    }
                    placeBtcSellOrder
                },
                {
                    fn getOpenBtcSellOrders(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BtcMarketPlaceCalls> {
                        <getOpenBtcSellOrdersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BtcMarketPlaceCalls::getOpenBtcSellOrders)
                    }
                    getOpenBtcSellOrders
                },
                {
                    fn getOpenAcceptedBtcBuyOrders(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BtcMarketPlaceCalls> {
                        <getOpenAcceptedBtcBuyOrdersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BtcMarketPlaceCalls::getOpenAcceptedBtcBuyOrders)
                    }
                    getOpenAcceptedBtcBuyOrders
                },
                {
                    fn proofBtcSellOrder(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BtcMarketPlaceCalls> {
                        <proofBtcSellOrderCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BtcMarketPlaceCalls::proofBtcSellOrder)
                    }
                    proofBtcSellOrder
                },
                {
                    fn getOpenAcceptedBtcSellOrders(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BtcMarketPlaceCalls> {
                        <getOpenAcceptedBtcSellOrdersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BtcMarketPlaceCalls::getOpenAcceptedBtcSellOrders)
                    }
                    getOpenAcceptedBtcSellOrders
                },
                {
                    fn withdrawBtcSellOrder(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BtcMarketPlaceCalls> {
                        <withdrawBtcSellOrderCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BtcMarketPlaceCalls::withdrawBtcSellOrder)
                    }
                    withdrawBtcSellOrder
                },
                {
                    fn proofBtcBuyOrder(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BtcMarketPlaceCalls> {
                        <proofBtcBuyOrderCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BtcMarketPlaceCalls::proofBtcBuyOrder)
                    }
                    proofBtcBuyOrder
                },
                {
                    fn acceptedBtcBuyOrders(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BtcMarketPlaceCalls> {
                        <acceptedBtcBuyOrdersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BtcMarketPlaceCalls::acceptedBtcBuyOrders)
                    }
                    acceptedBtcBuyOrders
                },
                {
                    fn cancelAcceptedBtcBuyOrder(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BtcMarketPlaceCalls> {
                        <cancelAcceptedBtcBuyOrderCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BtcMarketPlaceCalls::cancelAcceptedBtcBuyOrder)
                    }
                    cancelAcceptedBtcBuyOrder
                },
                {
                    fn getTrustedForwarder(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BtcMarketPlaceCalls> {
                        <getTrustedForwarderCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BtcMarketPlaceCalls::getTrustedForwarder)
                    }
                    getTrustedForwarder
                },
                {
                    fn REQUEST_EXPIRATION_SECONDS(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BtcMarketPlaceCalls> {
                        <REQUEST_EXPIRATION_SECONDSCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BtcMarketPlaceCalls::REQUEST_EXPIRATION_SECONDS)
                    }
                    REQUEST_EXPIRATION_SECONDS
                },
                {
                    fn cancelAcceptedBtcSellOrder(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BtcMarketPlaceCalls> {
                        <cancelAcceptedBtcSellOrderCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BtcMarketPlaceCalls::cancelAcceptedBtcSellOrder)
                    }
                    cancelAcceptedBtcSellOrder
                },
                {
                    fn btcSellOrders(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BtcMarketPlaceCalls> {
                        <btcSellOrdersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BtcMarketPlaceCalls::btcSellOrders)
                    }
                    btcSellOrders
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
    pub enum BtcMarketPlaceEvents {
        acceptBtcBuyOrderEvent(acceptBtcBuyOrderEvent),
        acceptBtcSellOrderEvent(acceptBtcSellOrderEvent),
        cancelAcceptedBtcBuyOrderEvent(cancelAcceptedBtcBuyOrderEvent),
        cancelAcceptedBtcSellOrderEvent(cancelAcceptedBtcSellOrderEvent),
        placeBtcBuyOrderEvent(placeBtcBuyOrderEvent),
        placeBtcSellOrderEvent(placeBtcSellOrderEvent),
        proofBtcBuyOrderEvent(proofBtcBuyOrderEvent),
        proofBtcSellOrderEvent(proofBtcSellOrderEvent),
        withdrawBtcBuyOrderEvent(withdrawBtcBuyOrderEvent),
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
                60u8,
                212u8,
                117u8,
                176u8,
                146u8,
                232u8,
                179u8,
                121u8,
                246u8,
                186u8,
                13u8,
                158u8,
                14u8,
                12u8,
                143u8,
                48u8,
                112u8,
                94u8,
                115u8,
                50u8,
                29u8,
                197u8,
                201u8,
                248u8,
                12u8,
                228u8,
                173u8,
                56u8,
                219u8,
                123u8,
                225u8,
                170u8,
            ],
            [
                62u8,
                94u8,
                163u8,
                88u8,
                233u8,
                235u8,
                76u8,
                223u8,
                68u8,
                205u8,
                199u8,
                121u8,
                56u8,
                173u8,
                232u8,
                7u8,
                75u8,
                18u8,
                64u8,
                166u8,
                216u8,
                192u8,
                253u8,
                19u8,
                114u8,
                134u8,
                113u8,
                184u8,
                46u8,
                128u8,
                10u8,
                214u8,
            ],
            [
                101u8,
                62u8,
                13u8,
                129u8,
                242u8,
                201u8,
                155u8,
                235u8,
                163u8,
                89u8,
                223u8,
                177u8,
                123u8,
                73u8,
                154u8,
                92u8,
                255u8,
                43u8,
                233u8,
                217u8,
                80u8,
                81u8,
                72u8,
                82u8,
                34u8,
                77u8,
                248u8,
                192u8,
                151u8,
                194u8,
                25u8,
                33u8,
            ],
            [
                120u8,
                245u8,
                31u8,
                98u8,
                247u8,
                207u8,
                19u8,
                129u8,
                198u8,
                115u8,
                194u8,
                126u8,
                174u8,
                24u8,
                125u8,
                214u8,
                197u8,
                136u8,
                220u8,
                102u8,
                36u8,
                206u8,
                89u8,
                105u8,
                125u8,
                187u8,
                62u8,
                29u8,
                124u8,
                27u8,
                188u8,
                223u8,
            ],
            [
                152u8,
                199u8,
                198u8,
                128u8,
                64u8,
                61u8,
                71u8,
                64u8,
                61u8,
                234u8,
                74u8,
                87u8,
                13u8,
                14u8,
                108u8,
                87u8,
                22u8,
                83u8,
                140u8,
                73u8,
                66u8,
                14u8,
                244u8,
                113u8,
                206u8,
                196u8,
                40u8,
                245u8,
                165u8,
                133u8,
                44u8,
                6u8,
            ],
            [
                180u8,
                201u8,
                141u8,
                226u8,
                16u8,
                105u8,
                107u8,
                60u8,
                242u8,
                30u8,
                153u8,
                51u8,
                92u8,
                30u8,
                227u8,
                160u8,
                174u8,
                52u8,
                162u8,
                103u8,
                19u8,
                65u8,
                42u8,
                74u8,
                221u8,
                232u8,
                175u8,
                89u8,
                97u8,
                118u8,
                243u8,
                126u8,
            ],
            [
                195u8,
                64u8,
                231u8,
                172u8,
                72u8,
                220u8,
                128u8,
                238u8,
                121u8,
                63u8,
                198u8,
                38u8,
                105u8,
                96u8,
                189u8,
                95u8,
                27u8,
                210u8,
                27u8,
                233u8,
                28u8,
                138u8,
                149u8,
                226u8,
                24u8,
                23u8,
                129u8,
                19u8,
                247u8,
                158u8,
                23u8,
                180u8,
            ],
            [
                195u8,
                154u8,
                26u8,
                93u8,
                220u8,
                14u8,
                133u8,
                201u8,
                85u8,
                254u8,
                46u8,
                26u8,
                190u8,
                180u8,
                60u8,
                148u8,
                206u8,
                24u8,
                50u8,
                46u8,
                117u8,
                187u8,
                61u8,
                68u8,
                232u8,
                15u8,
                117u8,
                159u8,
                249u8,
                208u8,
                52u8,
                185u8,
            ],
            [
                207u8,
                86u8,
                16u8,
                97u8,
                219u8,
                120u8,
                247u8,
                188u8,
                81u8,
                141u8,
                55u8,
                254u8,
                134u8,
                113u8,
                133u8,
                20u8,
                198u8,
                64u8,
                204u8,
                197u8,
                195u8,
                241u8,
                41u8,
                56u8,
                40u8,
                185u8,
                85u8,
                230u8,
                143u8,
                25u8,
                245u8,
                251u8,
            ],
            [
                255u8,
                28u8,
                226u8,
                16u8,
                222u8,
                252u8,
                211u8,
                186u8,
                26u8,
                223u8,
                118u8,
                201u8,
                65u8,
                154u8,
                7u8,
                88u8,
                250u8,
                96u8,
                253u8,
                62u8,
                179u8,
                140u8,
                123u8,
                217u8,
                65u8,
                143u8,
                96u8,
                181u8,
                117u8,
                183u8,
                110u8,
                36u8,
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
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(
                    <acceptBtcBuyOrderEvent as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <acceptBtcBuyOrderEvent as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::acceptBtcBuyOrderEvent)
                }
                Some(
                    <acceptBtcSellOrderEvent as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <acceptBtcSellOrderEvent as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::acceptBtcSellOrderEvent)
                }
                Some(
                    <cancelAcceptedBtcBuyOrderEvent as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <cancelAcceptedBtcBuyOrderEvent as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::cancelAcceptedBtcBuyOrderEvent)
                }
                Some(
                    <cancelAcceptedBtcSellOrderEvent as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <cancelAcceptedBtcSellOrderEvent as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::cancelAcceptedBtcSellOrderEvent)
                }
                Some(
                    <placeBtcBuyOrderEvent as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <placeBtcBuyOrderEvent as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::placeBtcBuyOrderEvent)
                }
                Some(
                    <placeBtcSellOrderEvent as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <placeBtcSellOrderEvent as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::placeBtcSellOrderEvent)
                }
                Some(
                    <proofBtcBuyOrderEvent as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <proofBtcBuyOrderEvent as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::proofBtcBuyOrderEvent)
                }
                Some(
                    <proofBtcSellOrderEvent as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <proofBtcSellOrderEvent as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::proofBtcSellOrderEvent)
                }
                Some(
                    <withdrawBtcBuyOrderEvent as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <withdrawBtcBuyOrderEvent as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::withdrawBtcBuyOrderEvent)
                }
                Some(
                    <withdrawBtcSellOrderEvent as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <withdrawBtcSellOrderEvent as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
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
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> BtcMarketPlaceInstance<T, P, N> {
        BtcMarketPlaceInstance::<T, P, N>::new(address, provider)
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
        _relay: alloy::sol_types::private::Address,
        erc2771Forwarder: alloy::sol_types::private::Address,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<BtcMarketPlaceInstance<T, P, N>>,
    > {
        BtcMarketPlaceInstance::<T, P, N>::deploy(provider, _relay, erc2771Forwarder)
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
        _relay: alloy::sol_types::private::Address,
        erc2771Forwarder: alloy::sol_types::private::Address,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        BtcMarketPlaceInstance::<
            T,
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
    pub struct BtcMarketPlaceInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for BtcMarketPlaceInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("BtcMarketPlaceInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > BtcMarketPlaceInstance<T, P, N> {
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
                _network_transport: ::core::marker::PhantomData,
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
        ) -> alloy_contract::Result<BtcMarketPlaceInstance<T, P, N>> {
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
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
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
    impl<T, P: ::core::clone::Clone, N> BtcMarketPlaceInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> BtcMarketPlaceInstance<T, P, N> {
            BtcMarketPlaceInstance {
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
    > BtcMarketPlaceInstance<T, P, N> {
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
        ///Creates a new call builder for the [`REQUEST_EXPIRATION_SECONDS`] function.
        pub fn REQUEST_EXPIRATION_SECONDS(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, REQUEST_EXPIRATION_SECONDSCall, N> {
            self.call_builder(&REQUEST_EXPIRATION_SECONDSCall {})
        }
        ///Creates a new call builder for the [`acceptBtcBuyOrder`] function.
        pub fn acceptBtcBuyOrder(
            &self,
            id: alloy::sol_types::private::primitives::aliases::U256,
            amountBtc: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, acceptBtcBuyOrderCall, N> {
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
        ) -> alloy_contract::SolCallBuilder<T, &P, acceptBtcSellOrderCall, N> {
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
        ) -> alloy_contract::SolCallBuilder<T, &P, acceptedBtcBuyOrdersCall, N> {
            self.call_builder(&acceptedBtcBuyOrdersCall { _0 })
        }
        ///Creates a new call builder for the [`acceptedBtcSellOrders`] function.
        pub fn acceptedBtcSellOrders(
            &self,
            _0: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, acceptedBtcSellOrdersCall, N> {
            self.call_builder(&acceptedBtcSellOrdersCall { _0 })
        }
        ///Creates a new call builder for the [`btcBuyOrders`] function.
        pub fn btcBuyOrders(
            &self,
            _0: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, btcBuyOrdersCall, N> {
            self.call_builder(&btcBuyOrdersCall { _0 })
        }
        ///Creates a new call builder for the [`btcSellOrders`] function.
        pub fn btcSellOrders(
            &self,
            _0: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, btcSellOrdersCall, N> {
            self.call_builder(&btcSellOrdersCall { _0 })
        }
        ///Creates a new call builder for the [`cancelAcceptedBtcBuyOrder`] function.
        pub fn cancelAcceptedBtcBuyOrder(
            &self,
            id: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, cancelAcceptedBtcBuyOrderCall, N> {
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
        ) -> alloy_contract::SolCallBuilder<T, &P, cancelAcceptedBtcSellOrderCall, N> {
            self.call_builder(
                &cancelAcceptedBtcSellOrderCall {
                    id,
                },
            )
        }
        ///Creates a new call builder for the [`getOpenAcceptedBtcBuyOrders`] function.
        pub fn getOpenAcceptedBtcBuyOrders(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getOpenAcceptedBtcBuyOrdersCall, N> {
            self.call_builder(&getOpenAcceptedBtcBuyOrdersCall {})
        }
        ///Creates a new call builder for the [`getOpenAcceptedBtcSellOrders`] function.
        pub fn getOpenAcceptedBtcSellOrders(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getOpenAcceptedBtcSellOrdersCall, N> {
            self.call_builder(
                &getOpenAcceptedBtcSellOrdersCall {
                },
            )
        }
        ///Creates a new call builder for the [`getOpenBtcBuyOrders`] function.
        pub fn getOpenBtcBuyOrders(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getOpenBtcBuyOrdersCall, N> {
            self.call_builder(&getOpenBtcBuyOrdersCall {})
        }
        ///Creates a new call builder for the [`getOpenBtcSellOrders`] function.
        pub fn getOpenBtcSellOrders(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getOpenBtcSellOrdersCall, N> {
            self.call_builder(&getOpenBtcSellOrdersCall {})
        }
        ///Creates a new call builder for the [`getTrustedForwarder`] function.
        pub fn getTrustedForwarder(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getTrustedForwarderCall, N> {
            self.call_builder(&getTrustedForwarderCall {})
        }
        ///Creates a new call builder for the [`isTrustedForwarder`] function.
        pub fn isTrustedForwarder(
            &self,
            forwarder: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, isTrustedForwarderCall, N> {
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
        ) -> alloy_contract::SolCallBuilder<T, &P, placeBtcBuyOrderCall, N> {
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
        ) -> alloy_contract::SolCallBuilder<T, &P, placeBtcSellOrderCall, N> {
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
        ) -> alloy_contract::SolCallBuilder<T, &P, proofBtcBuyOrderCall, N> {
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
        ) -> alloy_contract::SolCallBuilder<T, &P, proofBtcSellOrderCall, N> {
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
        ) -> alloy_contract::SolCallBuilder<T, &P, withdrawBtcBuyOrderCall, N> {
            self.call_builder(&withdrawBtcBuyOrderCall { id })
        }
        ///Creates a new call builder for the [`withdrawBtcSellOrder`] function.
        pub fn withdrawBtcSellOrder(
            &self,
            id: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, withdrawBtcSellOrderCall, N> {
            self.call_builder(&withdrawBtcSellOrderCall { id })
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > BtcMarketPlaceInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`acceptBtcBuyOrderEvent`] event.
        pub fn acceptBtcBuyOrderEvent_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, acceptBtcBuyOrderEvent, N> {
            self.event_filter::<acceptBtcBuyOrderEvent>()
        }
        ///Creates a new event filter for the [`acceptBtcSellOrderEvent`] event.
        pub fn acceptBtcSellOrderEvent_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, acceptBtcSellOrderEvent, N> {
            self.event_filter::<acceptBtcSellOrderEvent>()
        }
        ///Creates a new event filter for the [`cancelAcceptedBtcBuyOrderEvent`] event.
        pub fn cancelAcceptedBtcBuyOrderEvent_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, cancelAcceptedBtcBuyOrderEvent, N> {
            self.event_filter::<cancelAcceptedBtcBuyOrderEvent>()
        }
        ///Creates a new event filter for the [`cancelAcceptedBtcSellOrderEvent`] event.
        pub fn cancelAcceptedBtcSellOrderEvent_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, cancelAcceptedBtcSellOrderEvent, N> {
            self.event_filter::<cancelAcceptedBtcSellOrderEvent>()
        }
        ///Creates a new event filter for the [`placeBtcBuyOrderEvent`] event.
        pub fn placeBtcBuyOrderEvent_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, placeBtcBuyOrderEvent, N> {
            self.event_filter::<placeBtcBuyOrderEvent>()
        }
        ///Creates a new event filter for the [`placeBtcSellOrderEvent`] event.
        pub fn placeBtcSellOrderEvent_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, placeBtcSellOrderEvent, N> {
            self.event_filter::<placeBtcSellOrderEvent>()
        }
        ///Creates a new event filter for the [`proofBtcBuyOrderEvent`] event.
        pub fn proofBtcBuyOrderEvent_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, proofBtcBuyOrderEvent, N> {
            self.event_filter::<proofBtcBuyOrderEvent>()
        }
        ///Creates a new event filter for the [`proofBtcSellOrderEvent`] event.
        pub fn proofBtcSellOrderEvent_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, proofBtcSellOrderEvent, N> {
            self.event_filter::<proofBtcSellOrderEvent>()
        }
        ///Creates a new event filter for the [`withdrawBtcBuyOrderEvent`] event.
        pub fn withdrawBtcBuyOrderEvent_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, withdrawBtcBuyOrderEvent, N> {
            self.event_filter::<withdrawBtcBuyOrderEvent>()
        }
        ///Creates a new event filter for the [`withdrawBtcSellOrderEvent`] event.
        pub fn withdrawBtcSellOrderEvent_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, withdrawBtcSellOrderEvent, N> {
            self.event_filter::<withdrawBtcSellOrderEvent>()
        }
    }
}
