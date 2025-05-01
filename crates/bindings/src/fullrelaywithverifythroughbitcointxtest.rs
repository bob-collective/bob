///Module containing a contract's types and functions.
/**

```solidity
library StdInvariant {
    struct FuzzInterface { address addr; string[] artifacts; }
    struct FuzzSelector { address addr; bytes4[] selectors; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod StdInvariant {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
struct FuzzInterface { address addr; string[] artifacts; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct FuzzInterface {
        #[allow(missing_docs)]
        pub addr: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub artifacts: alloy::sol_types::private::Vec<alloy::sol_types::private::String>,
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
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::String>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Vec<alloy::sol_types::private::String>,
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
        impl ::core::convert::From<FuzzInterface> for UnderlyingRustTuple<'_> {
            fn from(value: FuzzInterface) -> Self {
                (value.addr, value.artifacts)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for FuzzInterface {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    addr: tuple.0,
                    artifacts: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for FuzzInterface {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for FuzzInterface {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.addr,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::String,
                    > as alloy_sol_types::SolType>::tokenize(&self.artifacts),
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
        impl alloy_sol_types::SolType for FuzzInterface {
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
        impl alloy_sol_types::SolStruct for FuzzInterface {
            const NAME: &'static str = "FuzzInterface";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "FuzzInterface(address addr,string[] artifacts)",
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
                            &self.addr,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::String,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.artifacts)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for FuzzInterface {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.addr,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::String,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.artifacts,
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
                    &rust.addr,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::String,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.artifacts,
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
struct FuzzSelector { address addr; bytes4[] selectors; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct FuzzSelector {
        #[allow(missing_docs)]
        pub addr: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub selectors: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<4>,
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
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::FixedBytes<4>>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<4>>,
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
        impl ::core::convert::From<FuzzSelector> for UnderlyingRustTuple<'_> {
            fn from(value: FuzzSelector) -> Self {
                (value.addr, value.selectors)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for FuzzSelector {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    addr: tuple.0,
                    selectors: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for FuzzSelector {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for FuzzSelector {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.addr,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<4>,
                    > as alloy_sol_types::SolType>::tokenize(&self.selectors),
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
        impl alloy_sol_types::SolType for FuzzSelector {
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
        impl alloy_sol_types::SolStruct for FuzzSelector {
            const NAME: &'static str = "FuzzSelector";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "FuzzSelector(address addr,bytes4[] selectors)",
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
                            &self.addr,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<4>,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.selectors)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for FuzzSelector {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.addr,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<4>,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.selectors,
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
                    &rust.addr,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<4>,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.selectors,
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
    /**Creates a new wrapper around an on-chain [`StdInvariant`](self) contract instance.

See the [wrapper's documentation](`StdInvariantInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> StdInvariantInstance<T, P, N> {
        StdInvariantInstance::<T, P, N>::new(address, provider)
    }
    /**A [`StdInvariant`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`StdInvariant`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct StdInvariantInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for StdInvariantInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("StdInvariantInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > StdInvariantInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`StdInvariant`](self) contract instance.

See the [wrapper's documentation](`StdInvariantInstance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> StdInvariantInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> StdInvariantInstance<T, P, N> {
            StdInvariantInstance {
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
    > StdInvariantInstance<T, P, N> {
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
    > StdInvariantInstance<T, P, N> {
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
library StdInvariant {
    struct FuzzInterface {
        address addr;
        string[] artifacts;
    }
    struct FuzzSelector {
        address addr;
        bytes4[] selectors;
    }
}

interface FullRelayWithVerifyThroughBitcoinTxTest {
    event log(string);
    event log_address(address);
    event log_array(uint256[] val);
    event log_array(int256[] val);
    event log_array(address[] val);
    event log_bytes(bytes);
    event log_bytes32(bytes32);
    event log_int(int256);
    event log_named_address(string key, address val);
    event log_named_array(string key, uint256[] val);
    event log_named_array(string key, int256[] val);
    event log_named_array(string key, address[] val);
    event log_named_bytes(string key, bytes val);
    event log_named_bytes32(string key, bytes32 val);
    event log_named_decimal_int(string key, int256 val, uint256 decimals);
    event log_named_decimal_uint(string key, uint256 val, uint256 decimals);
    event log_named_int(string key, int256 val);
    event log_named_string(string key, string val);
    event log_named_uint(string key, uint256 val);
    event log_string(string);
    event log_uint(uint256);
    event logs(bytes);

    function IS_TEST() external view returns (bool);
    function excludeArtifacts() external view returns (string[] memory excludedArtifacts_);
    function excludeContracts() external view returns (address[] memory excludedContracts_);
    function excludeSenders() external view returns (address[] memory excludedSenders_);
    function failed() external returns (bool);
    function getBlockHeights(string memory chainName, uint256 from, uint256 to) external view returns (uint256[] memory elements);
    function getDigestLes(string memory chainName, uint256 from, uint256 to) external view returns (bytes32[] memory elements);
    function getHeaderHexes(string memory chainName, uint256 from, uint256 to) external view returns (bytes[] memory elements);
    function getHeaders(string memory chainName, uint256 from, uint256 to) external view returns (bytes memory headers);
    function targetArtifactSelectors() external view returns (StdInvariant.FuzzSelector[] memory targetedArtifactSelectors_);
    function targetArtifacts() external view returns (string[] memory targetedArtifacts_);
    function targetContracts() external view returns (address[] memory targetedContracts_);
    function targetInterfaces() external view returns (StdInvariant.FuzzInterface[] memory targetedInterfaces_);
    function targetSelectors() external view returns (StdInvariant.FuzzSelector[] memory targetedSelectors_);
    function targetSenders() external view returns (address[] memory targetedSenders_);
    function testIncorrectInputVectorSupplied() external;
    function testIncorrectLocktimeSupplied() external;
    function testIncorrectOutputVectorSupplied() external;
    function testIncorrectProofSupplied() external;
    function testSuccessfullyVerify() external;
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "function",
    "name": "IS_TEST",
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
    "name": "excludeArtifacts",
    "inputs": [],
    "outputs": [
      {
        "name": "excludedArtifacts_",
        "type": "string[]",
        "internalType": "string[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "excludeContracts",
    "inputs": [],
    "outputs": [
      {
        "name": "excludedContracts_",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "excludeSenders",
    "inputs": [],
    "outputs": [
      {
        "name": "excludedSenders_",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "failed",
    "inputs": [],
    "outputs": [
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
    "name": "getBlockHeights",
    "inputs": [
      {
        "name": "chainName",
        "type": "string",
        "internalType": "string"
      },
      {
        "name": "from",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "to",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "elements",
        "type": "uint256[]",
        "internalType": "uint256[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getDigestLes",
    "inputs": [
      {
        "name": "chainName",
        "type": "string",
        "internalType": "string"
      },
      {
        "name": "from",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "to",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "elements",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getHeaderHexes",
    "inputs": [
      {
        "name": "chainName",
        "type": "string",
        "internalType": "string"
      },
      {
        "name": "from",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "to",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "elements",
        "type": "bytes[]",
        "internalType": "bytes[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getHeaders",
    "inputs": [
      {
        "name": "chainName",
        "type": "string",
        "internalType": "string"
      },
      {
        "name": "from",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "to",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "headers",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "targetArtifactSelectors",
    "inputs": [],
    "outputs": [
      {
        "name": "targetedArtifactSelectors_",
        "type": "tuple[]",
        "internalType": "struct StdInvariant.FuzzSelector[]",
        "components": [
          {
            "name": "addr",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "selectors",
            "type": "bytes4[]",
            "internalType": "bytes4[]"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "targetArtifacts",
    "inputs": [],
    "outputs": [
      {
        "name": "targetedArtifacts_",
        "type": "string[]",
        "internalType": "string[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "targetContracts",
    "inputs": [],
    "outputs": [
      {
        "name": "targetedContracts_",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "targetInterfaces",
    "inputs": [],
    "outputs": [
      {
        "name": "targetedInterfaces_",
        "type": "tuple[]",
        "internalType": "struct StdInvariant.FuzzInterface[]",
        "components": [
          {
            "name": "addr",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "artifacts",
            "type": "string[]",
            "internalType": "string[]"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "targetSelectors",
    "inputs": [],
    "outputs": [
      {
        "name": "targetedSelectors_",
        "type": "tuple[]",
        "internalType": "struct StdInvariant.FuzzSelector[]",
        "components": [
          {
            "name": "addr",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "selectors",
            "type": "bytes4[]",
            "internalType": "bytes4[]"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "targetSenders",
    "inputs": [],
    "outputs": [
      {
        "name": "targetedSenders_",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "testIncorrectInputVectorSupplied",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testIncorrectLocktimeSupplied",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testIncorrectOutputVectorSupplied",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testIncorrectProofSupplied",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testSuccessfullyVerify",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "event",
    "name": "log",
    "inputs": [
      {
        "name": "",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_address",
    "inputs": [
      {
        "name": "",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_array",
    "inputs": [
      {
        "name": "val",
        "type": "uint256[]",
        "indexed": false,
        "internalType": "uint256[]"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_array",
    "inputs": [
      {
        "name": "val",
        "type": "int256[]",
        "indexed": false,
        "internalType": "int256[]"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_array",
    "inputs": [
      {
        "name": "val",
        "type": "address[]",
        "indexed": false,
        "internalType": "address[]"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_bytes",
    "inputs": [
      {
        "name": "",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_bytes32",
    "inputs": [
      {
        "name": "",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_int",
    "inputs": [
      {
        "name": "",
        "type": "int256",
        "indexed": false,
        "internalType": "int256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_address",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_array",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "uint256[]",
        "indexed": false,
        "internalType": "uint256[]"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_array",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "int256[]",
        "indexed": false,
        "internalType": "int256[]"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_array",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "address[]",
        "indexed": false,
        "internalType": "address[]"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_bytes",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_bytes32",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_decimal_int",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "int256",
        "indexed": false,
        "internalType": "int256"
      },
      {
        "name": "decimals",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_decimal_uint",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "decimals",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_int",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "int256",
        "indexed": false,
        "internalType": "int256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_string",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_uint",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_string",
    "inputs": [
      {
        "name": "",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_uint",
    "inputs": [
      {
        "name": "",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "logs",
    "inputs": [
      {
        "name": "",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
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
pub mod FullRelayWithVerifyThroughBitcoinTxTest {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60078054600160ff199182168117909255600b805490911690911790556101006040526050608081815290616a6c60a039601e9061003d9082610942565b506040518061016001604052806101408152602001616b276101409139601f906100679082610942565b505f516020616b075f395f51905f5260205561011960215560405161008b90610890565b604051809103905ff0801580156100a4573d5f5f3e3d5ffd5b5060225f6101000a8154816001600160a01b0302191690836001600160a01b0316021790555060016023555f516020616b075f395f51905f526024556040518060800160405280600160f81b6001600160e01b03191681526020016040518060600160405280602a8152602001616c67602a913981526020016040518060800160405280604b8152602001616abc604b913981525f60209182015281516025805463ffffffff191660e09290921c919091178155908201516026906101699082610942565b506040820151600282019061017e9082610942565b506060820151816003015f6101000a81548163ffffffff021916908360e01c021790555050506040518060600160405280601f80546101bc906108be565b80601f01602080910402602001604051908101604052809291908181526020018280546101e8906108be565b80156102335780601f1061020a57610100808354040283529160200191610233565b820191905f5260205f20905b81548152906001019060200180831161021657829003601f168201915b505050505081526020016101198152602001601e8054610252906108be565b80601f016020809104026020016040519081016040528092919081815260200182805461027e906108be565b80156102c95780601f106102a0576101008083540402835291602001916102c9565b820191905f5260205f20905b8154815290600101906020018083116102ac57829003601f168201915b505050919092525050805160299081906102e39082610942565b5060208201516001820155604082015160028201906103029082610942565b505050348015610310575f5ffd5b506040518060400160405280600c81526020016b3432b0b232b939973539b7b760a11b8152506040518060400160405280600c81526020016b05ccecadccae6d2e65cd0caf60a31b8152506040518060400160405280600f81526020016e0b99d95b995cda5ccb9a195a59da1d608a1b815250604051806040016040528060128152602001712e67656e657369732e6469676573745f6c6560701b8152505f7f885cb69240a935d632d79c317109709ecfa91a80626ff3989d68f67f5b1dd12d5f1c6001600160a01b031663d930a0e66040518163ffffffff1660e01b81526004015f60405180830381865afa15801561040c573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526104339190810190610a71565b90505f8186604051602001610449929190610ad4565b60408051601f19818403018152908290526360f9bb1160e01b82529150737109709ecfa91a80626ff3989d68f67f5b1dd12d906360f9bb1190610490908490600401610b46565b5f60405180830381865afa1580156104aa573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526104d19190810190610a71565b601d906104de9082610942565b5061057585601d80546104f0906108be565b80601f016020809104026020016040519081016040528092919081815260200182805461051c906108be565b80156105675780601f1061053e57610100808354040283529160200191610567565b820191905f5260205f20905b81548152906001019060200180831161054a57829003601f168201915b509394935050610757915050565b61060b85601d8054610586906108be565b80601f01602080910402602001604051908101604052809291908181526020018280546105b2906108be565b80156105fd5780601f106105d4576101008083540402835291602001916105fd565b820191905f5260205f20905b8154815290600101906020018083116105e057829003601f168201915b5093949350506107db915050565b6106a185601d805461061c906108be565b80601f0160208091040260200160405190810160405280929190818152602001828054610648906108be565b80156106935780601f1061066a57610100808354040283529160200191610693565b820191905f5260205f20905b81548152906001019060200180831161067657829003601f168201915b509394935050610855915050565b6040516106ad9061089d565b6106b993929190610b58565b604051809103905ff0801580156106d2573d5f5f3e3d5ffd5b50601c80546001600160a01b0319166001600160a01b0392909216918217905560405163f58db06f60e01b8152600160048201819052602482015290965063f58db06f9550604401935061072592505050565b5f604051808303815f87803b15801561073c575f5ffd5b505af115801561074e573d5f5f3e3d5ffd5b50505050610bb7565b604051631fb2437d60e31b8152606090737109709ecfa91a80626ff3989d68f67f5b1dd12d9063fd921be8906107939086908690600401610b7c565b5f60405180830381865afa1580156107ad573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526107d49190810190610a71565b9392505050565b6040516356eef15b60e11b81525f90737109709ecfa91a80626ff3989d68f67f5b1dd12d9063addde2b6906108169086908690600401610b7c565b602060405180830381865afa158015610831573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906107d49190610ba0565b604051631777e59d60e01b81525f90737109709ecfa91a80626ff3989d68f67f5b1dd12d90631777e59d906108169086908690600401610b7c565b610b888061360c83390190565b6128d88061419483390190565b634e487b7160e01b5f52604160045260245ffd5b600181811c908216806108d257607f821691505b6020821081036108f057634e487b7160e01b5f52602260045260245ffd5b50919050565b601f82111561093d57805f5260205f20601f840160051c8101602085101561091b5750805b601f840160051c820191505b8181101561093a575f8155600101610927565b50505b505050565b81516001600160401b0381111561095b5761095b6108aa565b61096f8161096984546108be565b846108f6565b6020601f8211600181146109a1575f831561098a5750848201515b5f19600385901b1c1916600184901b17845561093a565b5f84815260208120601f198516915b828110156109d057878501518255602094850194600190920191016109b0565b50848210156109ed57868401515f19600387901b60f8161c191681555b50505050600190811b01905550565b5f806001600160401b03841115610a1557610a156108aa565b50604051601f19601f85018116603f011681018181106001600160401b0382111715610a4357610a436108aa565b604052838152905080828401851015610a5a575f5ffd5b8383602083015e5f60208583010152509392505050565b5f60208284031215610a81575f5ffd5b81516001600160401b03811115610a96575f5ffd5b8201601f81018413610aa6575f5ffd5b610ab5848251602084016109fc565b949350505050565b5f81518060208401855e5f93019283525090919050565b5f610adf8285610abd565b7f2f746573742f66756c6c52656c61792f74657374446174612f000000000000008152610b0f6019820185610abd565b95945050505050565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b602081525f6107d46020830184610b18565b606081525f610b6a6060830186610b18565b60208301949094525060400152919050565b604081525f610b8e6040830185610b18565b8281036020840152610b0f8185610b18565b5f60208284031215610bb0575f5ffd5b5051919050565b612a4880610bc45f395ff3fe608060405234801561000f575f5ffd5b5060043610610163575f3560e01c8063916a17c6116100c7578063e20c9f711161007d578063ef3e681211610063578063ef3e681214610286578063fa7626d41461028e578063fad06b8f1461029b575f5ffd5b8063e20c9f7114610276578063e89f84191461027e575f5ffd5b8063b5508aa9116100ad578063b5508aa91461024e578063ba414fa614610256578063c899d2411461026e575f5ffd5b8063916a17c61461023e578063b52b205814610246575f5ffd5b80633f7286f41161011c5780635df0d076116101025780635df0d0761461020a57806366d9a9a01461021457806385226c8114610229575f5ffd5b80633f7286f4146101e257806344badbb6146101ea575f5ffd5b80631ed7831c1161014c5780631ed7831c146101b05780632ade3880146101c55780633e5e3c23146101da575f5ffd5b80630813852a146101675780631c0da81f14610190575b5f5ffd5b61017a61017536600461200b565b6102ae565b60405161018791906120c0565b60405180910390f35b6101a361019e36600461200b565b6102f9565b6040516101879190612123565b6101b861036b565b6040516101879190612135565b6101cd6103d8565b60405161018791906121e7565b6101b8610521565b6101b861058c565b6101fd6101f836600461200b565b6105f7565b604051610187919061226b565b61021261063a565b005b61021c610932565b60405161018791906122a2565b610231610a35565b6040516101879190612373565b61021c610b00565b610212610c03565b610231610cba565b61025e610d85565b6040519015158152602001610187565b610212610ec2565b6101b86110f6565b610212611161565b6102126113c7565b60075461025e9060ff1681565b6101fd6102a936600461200b565b6115ec565b60606102f18484846040518060400160405280600381526020017f686578000000000000000000000000000000000000000000000000000000000081525061162f565b949350505050565b60605f6103078585856102ae565b90505f5b61031585856123b2565b811015610362578282828151811061032f5761032f6123c5565b6020026020010151604051602001610348929190612409565b60408051601f19818403018152919052925060010161030b565b50509392505050565b606060148054806020026020016040519081016040528092919081815260200182805480156103ce57602002820191905f5260205f20905b815473ffffffffffffffffffffffffffffffffffffffff1681526001909101906020018083116103a3575b5050505050905090565b6060601b805480602002602001604051908101604052809291908181526020015f905b82821015610518575f848152602080822060408051808201825260028702909201805473ffffffffffffffffffffffffffffffffffffffff168352600181018054835181870281018701909452808452939591948681019491929084015b82821015610501578382905f5260205f200180546104769061241d565b80601f01602080910402602001604051908101604052809291908181526020018280546104a29061241d565b80156104ed5780601f106104c4576101008083540402835291602001916104ed565b820191905f5260205f20905b8154815290600101906020018083116104d057829003601f168201915b505050505081526020019060010190610459565b5050505081525050815260200190600101906103fb565b50505050905090565b606060168054806020026020016040519081016040528092919081815260200182805480156103ce57602002820191905f5260205f2090815473ffffffffffffffffffffffffffffffffffffffff1681526001909101906020018083116103a3575050505050905090565b606060158054806020026020016040519081016040528092919081815260200182805480156103ce57602002820191905f5260205f2090815473ffffffffffffffffffffffffffffffffffffffff1681526001909101906020018083116103a3575050505050905090565b60606102f18484846040518060400160405280600981526020017f6469676573745f6c650000000000000000000000000000000000000000000000815250611790565b60408051608081019091526025805460e01b7fffffffff00000000000000000000000000000000000000000000000000000000168252602680545f93929160208401916106869061241d565b80601f01602080910402602001604051908101604052809291908181526020018280546106b29061241d565b80156106fd5780601f106106d4576101008083540402835291602001916106fd565b820191905f5260205f20905b8154815290600101906020018083116106e057829003601f168201915b505050505081526020016002820180546107169061241d565b80601f01602080910402602001604051908101604052809291908181526020018280546107429061241d565b801561078d5780601f106107645761010080835404028352916020019161078d565b820191905f5260205f20905b81548152906001019060200180831161077057829003601f168201915b50505091835250506003919091015460e01b7fffffffff0000000000000000000000000000000000000000000000000000000016602091820152604080518082018252600181525f818401528382015280518082018252601e81527f496e76616c6964206f757470757420766563746f722070726f7669646564000092810192909252517ff28dceb3000000000000000000000000000000000000000000000000000000008152919250737109709ecfa91a80626ff3989d68f67f5b1dd12d9163f28dceb39161085f91600401612123565b5f604051808303815f87803b158015610876575f5ffd5b505af1158015610888573d5f5f3e3d5ffd5b5050602254601c546023546040517f51588b8400000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff93841695506351588b8494506108ef93909216918690602990600401612574565b602060405180830381865afa15801561090a573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061092e9190612665565b5050565b60606019805480602002602001604051908101604052809291908181526020015f905b82821015610518575f84815260209081902060408051808201825260028602909201805473ffffffffffffffffffffffffffffffffffffffff168352600181018054835181870281018701909452808452939491938583019392830182828015610a1d57602002820191905f5260205f20905f905b82829054906101000a900460e01b7bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916815260200190600401906020826003010492830192600103820291508084116109ca5790505b50505050508152505081526020019060010190610955565b60606018805480602002602001604051908101604052809291908181526020015f905b82821015610518578382905f5260205f20018054610a759061241d565b80601f0160208091040260200160405190810160405280929190818152602001828054610aa19061241d565b8015610aec5780601f10610ac357610100808354040283529160200191610aec565b820191905f5260205f20905b815481529060010190602001808311610acf57829003601f168201915b505050505081526020019060010190610a58565b6060601a805480602002602001604051908101604052809291908181526020015f905b82821015610518575f84815260209081902060408051808201825260028602909201805473ffffffffffffffffffffffffffffffffffffffff168352600181018054835181870281018701909452808452939491938583019392830182828015610beb57602002820191905f5260205f20905f905b82829054906101000a900460e01b7bffffffffffffffffffffffffffffffffffffffffffffffffffffffff191681526020019060040190602082600301049283019260010382029150808411610b985790505b50505050508152505081526020019060010190610b23565b602254601c546023546040517f51588b840000000000000000000000000000000000000000000000000000000081525f9373ffffffffffffffffffffffffffffffffffffffff908116936351588b8493610c6a93929091169160259060299060040161270d565b602060405180830381865afa158015610c85573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610ca99190612665565b9050610cb7816024546118de565b50565b60606017805480602002602001604051908101604052809291908181526020015f905b82821015610518578382905f5260205f20018054610cfa9061241d565b80601f0160208091040260200160405190810160405280929190818152602001828054610d269061241d565b8015610d715780601f10610d4857610100808354040283529160200191610d71565b820191905f5260205f20905b815481529060010190602001808311610d5457829003601f168201915b505050505081526020019060010190610cdd565b6007545f90610100900460ff1615610da65750600754610100900460ff1690565b5f737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ebd5760408051737109709ecfa91a80626ff3989d68f67f5b1dd12d602082018190527f6661696c6564000000000000000000000000000000000000000000000000000082840152825180830384018152606083019093525f929091610e49917f667f9d70ca411d70ead50d8d5c22070dafc36ad75f3dcf5e7237b22ade9aecc491608001612753565b60408051601f1981840301815290829052610e6391612786565b5f604051808303815f865af19150503d805f8114610e9c576040519150601f19603f3d011682016040523d82523d5f602084013e610ea1565b606091505b5091505080806020019051810190610eb99190612791565b9150505b919050565b60408051608081019091526025805460e01b7fffffffff00000000000000000000000000000000000000000000000000000000168252602680545f9392916020840191610f0e9061241d565b80601f0160208091040260200160405190810160405280929190818152602001828054610f3a9061241d565b8015610f855780601f10610f5c57610100808354040283529160200191610f85565b820191905f5260205f20905b815481529060010190602001808311610f6857829003601f168201915b50505050508152602001600282018054610f9e9061241d565b80601f0160208091040260200160405190810160405280929190818152602001828054610fca9061241d565b80156110155780601f10610fec57610100808354040283529160200191611015565b820191905f5260205f20905b815481529060010190602001808311610ff857829003601f168201915b50505091835250506003919091015460e01b7fffffffff00000000000000000000000000000000000000000000000000000000166020918201527c01000000000000000000000000000000000000000000000000000000006060830152604080518082018252601381527f42616420696e636c7573696f6e2070726f6f660000000000000000000000000092810192909252517ff28dceb3000000000000000000000000000000000000000000000000000000008152919250737109709ecfa91a80626ff3989d68f67f5b1dd12d9163f28dceb39161085f91600401612123565b606060138054806020026020016040519081016040528092919081815260200182805480156103ce57602002820191905f5260205f2090815473ffffffffffffffffffffffffffffffffffffffff1681526001909101906020018083116103a3575050505050905090565b5f60296040518060600160405290815f8201805461117e9061241d565b80601f01602080910402602001604051908101604052809291908181526020018280546111aa9061241d565b80156111f55780601f106111cc576101008083540402835291602001916111f5565b820191905f5260205f20905b8154815290600101906020018083116111d857829003601f168201915b50505050508152602001600182015481526020016002820180546112189061241d565b80601f01602080910402602001604051908101604052809291908181526020018280546112449061241d565b801561128f5780601f106112665761010080835404028352916020019161128f565b820191905f5260205f20905b81548152906001019060200180831161127257829003601f168201915b505050919092525050604080518082018252600181525f60208083019190915290845281518083018352601681527f426164206d65726b6c652061727261792070726f6f66000000000000000000009181019190915290517ff28dceb3000000000000000000000000000000000000000000000000000000008152929350737109709ecfa91a80626ff3989d68f67f5b1dd12d9263f28dceb392506113379190600401612123565b5f604051808303815f87803b15801561134e575f5ffd5b505af1158015611360573d5f5f3e3d5ffd5b5050602254601c546023546040517f51588b8400000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff93841695506351588b8494506108ef939092169160259087906004016127b7565b60408051608081019091526025805460e01b7fffffffff00000000000000000000000000000000000000000000000000000000168252602680545f93929160208401916114139061241d565b80601f016020809104026020016040519081016040528092919081815260200182805461143f9061241d565b801561148a5780601f106114615761010080835404028352916020019161148a565b820191905f5260205f20905b81548152906001019060200180831161146d57829003601f168201915b505050505081526020016002820180546114a39061241d565b80601f01602080910402602001604051908101604052809291908181526020018280546114cf9061241d565b801561151a5780601f106114f15761010080835404028352916020019161151a565b820191905f5260205f20905b8154815290600101906020018083116114fd57829003601f168201915b50505091835250506003919091015460e01b7fffffffff0000000000000000000000000000000000000000000000000000000016602091820152604080518082018252600181525f818401528383015280518082018252601d81527f496e76616c696420696e70757420766563746f722070726f766964656400000092810192909252517ff28dceb3000000000000000000000000000000000000000000000000000000008152919250737109709ecfa91a80626ff3989d68f67f5b1dd12d9163f28dceb39161085f91600401612123565b60606102f18484846040518060400160405280600681526020017f6865696768740000000000000000000000000000000000000000000000000000815250611a46565b606061163b84846123b2565b67ffffffffffffffff81111561165357611653611f86565b60405190808252806020026020018201604052801561168657816020015b60608152602001906001900390816116715790505b509050835b8381101561178757611759866116a083611b94565b856040516020016116b393929190612836565b604051602081830303815290604052601d80546116cf9061241d565b80601f01602080910402602001604051908101604052809291908181526020018280546116fb9061241d565b80156117465780601f1061171d57610100808354040283529160200191611746565b820191905f5260205f20905b81548152906001019060200180831161172957829003601f168201915b5050505050611cc590919063ffffffff16565b8261176487846123b2565b81518110611774576117746123c5565b602090810291909101015260010161168b565b50949350505050565b606061179c84846123b2565b67ffffffffffffffff8111156117b4576117b4611f86565b6040519080825280602002602001820160405280156117dd578160200160208202803683370190505b509050835b83811015611787576118b0866117f783611b94565b8560405160200161180a93929190612836565b604051602081830303815290604052601d80546118269061241d565b80601f01602080910402602001604051908101604052809291908181526020018280546118529061241d565b801561189d5780601f106118745761010080835404028352916020019161189d565b820191905f5260205f20905b81548152906001019060200180831161188057829003601f168201915b5050505050611d6490919063ffffffff16565b826118bb87846123b2565b815181106118cb576118cb6123c5565b60209081029190910101526001016117e2565b80821461092e577f41304facd9323d75b11bcdd609cb38effffdb05710f7caf0e9b16c6d9d709f5060405161196a9060208082526025908201527f4572726f723a2061203d3d2062206e6f7420736174697366696564205b62797460408201527f657333325d000000000000000000000000000000000000000000000000000000606082015260800190565b60405180910390a160408051818152600a818301527f2020202020204c6566740000000000000000000000000000000000000000000060608201526020810184905290517fafb795c9c61e4fe7468c386f925d7a5429ecad9c0495ddb8d38d690614d32f999181900360800190a160408051818152600a818301527f202020202052696768740000000000000000000000000000000000000000000060608201526020810183905290517fafb795c9c61e4fe7468c386f925d7a5429ecad9c0495ddb8d38d690614d32f999181900360800190a161092e611df7565b6060611a5284846123b2565b67ffffffffffffffff811115611a6a57611a6a611f86565b604051908082528060200260200182016040528015611a93578160200160208202803683370190505b509050835b8381101561178757611b6686611aad83611b94565b85604051602001611ac093929190612836565b604051602081830303815290604052601d8054611adc9061241d565b80601f0160208091040260200160405190810160405280929190818152602001828054611b089061241d565b8015611b535780601f10611b2a57610100808354040283529160200191611b53565b820191905f5260205f20905b815481529060010190602001808311611b3657829003601f168201915b5050505050611f3290919063ffffffff16565b82611b7187846123b2565b81518110611b8157611b816123c5565b6020908102919091010152600101611a98565b6060815f03611bd657505060408051808201909152600181527f3000000000000000000000000000000000000000000000000000000000000000602082015290565b815f5b8115611bff5780611be9816128d3565b9150611bf89050600a83612937565b9150611bd9565b5f8167ffffffffffffffff811115611c1957611c19611f86565b6040519080825280601f01601f191660200182016040528015611c43576020820181803683370190505b5090505b84156102f157611c586001836123b2565b9150611c65600a8661294a565b611c7090603061295d565b60f81b818381518110611c8557611c856123c5565b60200101907effffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff191690815f1a905350611cbe600a86612937565b9450611c47565b6040517ffd921be8000000000000000000000000000000000000000000000000000000008152606090737109709ecfa91a80626ff3989d68f67f5b1dd12d9063fd921be890611d1a9086908690600401612970565b5f60405180830381865afa158015611d34573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052611d5b919081019061299d565b90505b92915050565b6040517f1777e59d0000000000000000000000000000000000000000000000000000000081525f90737109709ecfa91a80626ff3989d68f67f5b1dd12d90631777e59d90611db89086908690600401612970565b602060405180830381865afa158015611dd3573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611d5b9190612665565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15611f045760408051737109709ecfa91a80626ff3989d68f67f5b1dd12d602082018190527f6661696c6564000000000000000000000000000000000000000000000000000092820192909252600160608201525f91907f70ca10bbd0dbfd9020a9f4b13402c16cb120705e0d1c0aeab10fa353ae586fc49060800160408051601f1981840301815290829052611ea79291602001612753565b60408051601f1981840301815290829052611ec191612786565b5f604051808303815f865af19150503d805f8114611efa576040519150601f19603f3d011682016040523d82523d5f602084013e611eff565b606091505b505050505b600780547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00ff16610100179055565b6040517faddde2b60000000000000000000000000000000000000000000000000000000081525f90737109709ecfa91a80626ff3989d68f67f5b1dd12d9063addde2b690611db89086908690600401612970565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b604051601f8201601f1916810167ffffffffffffffff81118282101715611fdc57611fdc611f86565b604052919050565b5f67ffffffffffffffff821115611ffd57611ffd611f86565b50601f01601f191660200190565b5f5f5f6060848603121561201d575f5ffd5b833567ffffffffffffffff811115612033575f5ffd5b8401601f81018613612043575f5ffd5b803561205661205182611fe4565b611fb3565b81815287602083850101111561206a575f5ffd5b816020840160208301375f602092820183015297908601359650604090950135949350505050565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b8281101561211757603f19878603018452612102858351612092565b945060209384019391909101906001016120e6565b50929695505050505050565b602081525f611d5b6020830184612092565b602080825282518282018190525f918401906040840190835b8181101561218257835173ffffffffffffffffffffffffffffffffffffffff1683526020938401939092019160010161214e565b509095945050505050565b5f82825180855260208501945060208160051b830101602085015f5b838110156121db57601f198584030188526121c5838351612092565b60209889019890935091909101906001016121a9565b50909695505050505050565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b8281101561211757603f19878603018452815173ffffffffffffffffffffffffffffffffffffffff81511686526020810151905060406020870152612255604087018261218d565b955050602093840193919091019060010161220d565b602080825282518282018190525f918401906040840190835b81811015612182578351835260209384019390920191600101612284565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b8281101561211757868503603f190184528151805173ffffffffffffffffffffffffffffffffffffffff168652602090810151604082880181905281519088018190529101905f9060608801905b8083101561235b577fffffffff000000000000000000000000000000000000000000000000000000008451168252602082019150602084019350600183019250612316565b509650505060209384019391909101906001016122c8565b602081525f611d5b602083018461218d565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b81810381811115611d5e57611d5e612385565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f81518060208401855e5f93019283525090919050565b5f6102f161241783866123f2565b846123f2565b600181811c9082168061243157607f821691505b602082108103612468577f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b50919050565b80545f90600181811c9082168061248657607f821691505b6020821081036124bd577f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b818652602086018180156124d8576001811461250c57612538565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff008516825283151560051b82019550612538565b5f878152602090205f5b8581101561253257815484820152600190910190602001612516565b83019650505b505050505092915050565b606082525f612555606084018361246e565b6001830154602085015283810360408501526102f1816002850161246e565b73ffffffffffffffffffffffffffffffffffffffff85168152836020820152608060408201527fffffffff0000000000000000000000000000000000000000000000000000000083511660808201525f6020840151608060a08401526125de610100840182612092565b905060408501517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff808483030160c08501526126198282612092565b9150507fffffffff0000000000000000000000000000000000000000000000000000000060608601511660e0840152828103606084015261265a8185612543565b979650505050505050565b5f60208284031215612675575f5ffd5b5051919050565b7fffffffff00000000000000000000000000000000000000000000000000000000815460e01b168252608060208301525f6126bd608084016001840161246e565b83810360408501526126d2816002850161246e565b90507fffffffff00000000000000000000000000000000000000000000000000000000600384015460e01b1660608501528091505092915050565b73ffffffffffffffffffffffffffffffffffffffff85168152836020820152608060408201525f612741608083018561267c565b828103606084015261265a8185612543565b7fffffffff00000000000000000000000000000000000000000000000000000000831681525f6102f160048301846123f2565b5f611d5b82846123f2565b5f602082840312156127a1575f5ffd5b815180151581146127b0575f5ffd5b9392505050565b73ffffffffffffffffffffffffffffffffffffffff85168152836020820152608060408201525f6127eb608083018561267c565b82810360608401528351606082526128066060830182612092565b905060208501516020830152604085015182820360408401526128298282612092565b9998505050505050505050565b7f2e0000000000000000000000000000000000000000000000000000000000000081525f61286760018301866123f2565b7f5b00000000000000000000000000000000000000000000000000000000000000815261289760018201866123f2565b90507f5d2e00000000000000000000000000000000000000000000000000000000000081526128c960028201856123f2565b9695505050505050565b5f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff820361290357612903612385565b5060010190565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f826129455761294561290a565b500490565b5f826129585761295861290a565b500690565b80820180821115611d5e57611d5e612385565b604081525f6129826040830185612092565b82810360208401526129948185612092565b95945050505050565b5f602082840312156129ad575f5ffd5b815167ffffffffffffffff8111156129c3575f5ffd5b8201601f810184136129d3575f5ffd5b80516129e161205182611fe4565b8181528560208385010111156129f5575f5ffd5b8160208401602083015e5f9181016020019190915294935050505056fea26469706673582212201ca29e3f17a46aa00a3eab0db4101c405266208d3e68568fad37322f99cf475564736f6c634300081c00336080604052348015600e575f5ffd5b50610b6c8061001c5f395ff3fe608060405234801561000f575f5ffd5b5060043610610029575f3560e01c806351588b841461002d575b5f5ffd5b61004061003b366004610880565b610052565b60405190815260200160405180910390f35b5f61005f85858585610068565b95945050505050565b5f6100728361010c565b6040808401518451602086015192517f05d09a7000000000000000000000000000000000000000000000000000000000815293945073ffffffffffffffffffffffffffffffffffffffff8916936305d09a70936100d89392918791908b906004016109e5565b5f6040518083038186803b1580156100ee575f5ffd5b505afa158015610100573d5f5f3e3d5ffd5b50505050949350505050565b5f61011a826020015161023b565b610185576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601d60248201527f496e76616c696420696e70757420766563746f722070726f766964656400000060448201526064015b60405180910390fd5b61019282604001516102d5565b6101f8576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601e60248201527f496e76616c6964206f757470757420766563746f722070726f76696465640000604482015260640161017c565b610235825f01518360200151846040015185606001516040516020016102219493929190610a41565b604051602081830303815290604052610362565b92915050565b5f5f5f61024784610384565b909250905080158061025957505f1982145b1561026757505f9392505050565b5f610273836001610add565b90505f5b828110156102c8578551821061029257505f95945050505050565b5f61029d8784610399565b90505f1981036102b357505f9695505050505050565b6102bd8184610add565b925050600101610277565b5093519093149392505050565b5f5f5f6102e184610384565b90925090508015806102f357505f1982145b1561030157505f9392505050565b5f61030d836001610add565b90505f5b828110156102c8578551821061032c57505f95945050505050565b5f61033787846103df565b90505f19810361034d57505f9695505050505050565b6103578184610add565b925050600101610311565b5f60205f83516020850160025afa5060205f60205f60025afa50505f51919050565b5f5f610390835f61043f565b91509150915091565b5f5f5f6103a685856105dc565b9092509050600182016103be575f1992505050610235565b806103ca836025610add565b6103d49190610add565b61005f906004610add565b5f6103eb826009610add565b835110156103fb57505f19610235565b5f806104118561040c866008610add565b61043f565b909250905060018201610429575f1992505050610235565b80610435836009610add565b61005f9190610add565b5f5f5f61044c858561061a565b90508060ff165f0361047f575f85858151811061046b5761046b610af0565b016020015190935060f81c91506105d59050565b8361048b826001610b1d565b60ff166104989190610add565b855110156104ad575f195f92509250506105d5565b5f8160ff166002036104f0576104e56104d16104ca876001610add565b889061069e565b62ffff0060e882901c1660f89190911c1790565b61ffff1690506105cb565b8160ff1660040361053f5761053261050c6104ca876001610add565b60d881901c63ff00ff001662ff00ff60e89290921c9190911617601081811b91901c1790565b63ffffffff1690506105cb565b8160ff166008036105cb576105be61055b6104ca876001610add565b60c01c64ff000000ff600882811c91821665ff000000ff009390911b92831617601090811b67ffffffffffffffff1666ff00ff00ff00ff9290921667ff00ff00ff00ff009093169290921790911c65ffff0000ffff1617602081811c91901b1790565b67ffffffffffffffff1690505b60ff909116925090505b9250929050565b5f806105e9836025610add565b845110156105fc57505f1990505f6105d5565b5f8061060d8661040c876024610add565b9097909650945050505050565b5f82828151811061062d5761062d610af0565b016020015160f81c60ff0361064457506008610235565b82828151811061065657610656610af0565b016020015160f81c60fe0361066d57506004610235565b82828151811061067f5761067f610af0565b016020015160f81c60fd0361069657506002610235565b505f92915050565b5f6106ac8383016020015190565b9392505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b6040516080810167ffffffffffffffff81118282101715610703576107036106b3565b60405290565b80357fffffffff0000000000000000000000000000000000000000000000000000000081168114610738575f5ffd5b919050565b5f82601f83011261074c575f5ffd5b813567ffffffffffffffff811115610766576107666106b3565b604051601f82017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0908116603f0116810167ffffffffffffffff811182821017156107b3576107b36106b3565b6040528181528382016020018510156107ca575f5ffd5b816020850160208301375f918101602001919091529392505050565b5f606082840312156107f6575f5ffd5b6040516060810167ffffffffffffffff81118282101715610819576108196106b3565b604052905080823567ffffffffffffffff811115610835575f5ffd5b6108418582860161073d565b82525060208381013590820152604083013567ffffffffffffffff811115610867575f5ffd5b6108738582860161073d565b6040830152505092915050565b5f5f5f5f60808587031215610893575f5ffd5b843573ffffffffffffffffffffffffffffffffffffffff811681146108b6575f5ffd5b935060208501359250604085013567ffffffffffffffff8111156108d8575f5ffd5b8501608081880312156108e9575f5ffd5b6108f16106e0565b6108fa82610709565b8152602082013567ffffffffffffffff811115610915575f5ffd5b6109218982850161073d565b602083015250604082013567ffffffffffffffff811115610940575f5ffd5b61094c8982850161073d565b60408301525061095e60608301610709565b60608201528093505050606085013567ffffffffffffffff811115610981575f5ffd5b61098d878288016107e6565b91505092959194509250565b5f81518084528060208401602086015e5f6020828601015260207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f83011685010191505092915050565b60a081525f6109f760a0830188610999565b8281036020840152610a098188610999565b60408401969096525050606081019290925260ff1660809091015292915050565b5f81518060208401855e5f93019283525090919050565b7fffffffff00000000000000000000000000000000000000000000000000000000851681525f610a7d610a776004840187610a2a565b85610a2a565b7fffffffff0000000000000000000000000000000000000000000000000000000093909316835250506004019392505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b8082018082111561023557610235610ab0565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b60ff818116838216019081111561023557610235610ab056fea264697066735822122055377446e38cb0b3f5755b2fb2779901081f48819607b1b4a04553210cc6df9e64736f6c634300081c0033608060405234801561000f575f5ffd5b506040516128d83803806128d883398101604081905261002e9161032b565b82828282828261003f835160501490565b6100845760405162461bcd60e51b81526020600482015260116024820152704261642067656e6573697320626c6f636b60781b60448201526064015b60405180910390fd5b5f61008e84610166565b905062ffffff8216156101095760405162461bcd60e51b815260206004820152603d60248201527f506572696f64207374617274206861736820646f6573206e6f7420686176652060448201527f776f726b2e2048696e743a2077726f6e672062797465206f726465723f000000606482015260840161007b565b5f818155600182905560028290558181526004602052604090208390556101326107e0846103fe565b61013c9084610425565b5f8381526004602052604090205561015384610226565b600555506105bd98505050505050505050565b5f600280836040516101789190610438565b602060405180830381855afa158015610193573d5f5f3e3d5ffd5b5050506040513d601f19601f820116820180604052508101906101b6919061044e565b6040516020016101c891815260200190565b60408051601f19818403018152908290526101e291610438565b602060405180830381855afa1580156101fd573d5f5f3e3d5ffd5b5050506040513d601f19601f82011682018060405250810190610220919061044e565b92915050565b5f61022061023383610238565b610243565b5f6102208282610253565b5f61022061ffff60d01b836102f7565b5f8061026a610263846048610465565b8590610309565b60e81c90505f8461027c85604b610465565b8151811061028c5761028c610478565b016020015160f81c90505f6102be835f60108262ffffff16901c8261ff001660108462ffffff16901b17179050919050565b62ffffff1690505f6102d160038461048c565b60ff1690506102e281610100610588565b6102ec9083610593565b979650505050505050565b5f61030282846105aa565b9392505050565b5f6103028383016020015190565b634e487b7160e01b5f52604160045260245ffd5b5f5f5f6060848603121561033d575f5ffd5b83516001600160401b03811115610352575f5ffd5b8401601f81018613610362575f5ffd5b80516001600160401b0381111561037b5761037b610317565b604051601f8201601f19908116603f011681016001600160401b03811182821017156103a9576103a9610317565b6040528181528282016020018810156103c0575f5ffd5b8160208401602083015e5f6020928201830152908601516040909601519097959650949350505050565b634e487b7160e01b5f52601260045260245ffd5b5f8261040c5761040c6103ea565b500690565b634e487b7160e01b5f52601160045260245ffd5b8181038181111561022057610220610411565b5f82518060208501845e5f920191825250919050565b5f6020828403121561045e575f5ffd5b5051919050565b8082018082111561022057610220610411565b634e487b7160e01b5f52603260045260245ffd5b60ff828116828216039081111561022057610220610411565b6001815b60018411156104e0578085048111156104c4576104c4610411565b60018416156104d257908102905b60019390931c9280026104a9565b935093915050565b5f826104f657506001610220565b8161050257505f610220565b816001811461051857600281146105225761053e565b6001915050610220565b60ff84111561053357610533610411565b50506001821b610220565b5060208310610133831016604e8410600b8410161715610561575081810a610220565b61056d5f1984846104a5565b805f190482111561058057610580610411565b029392505050565b5f61030283836104e8565b808202811582820484141761022057610220610411565b5f826105b8576105b86103ea565b500490565b61230e806105ca5f395ff3fe608060405234801561000f575f5ffd5b50600436106100fb575f3560e01c806370d53c1811610093578063b985621a11610063578063b985621a146101ed578063c58242cd14610200578063e3d8d8d814610208578063f58db06f1461020f575f5ffd5b806370d53c181461019757806374c3a3a9146101b45780637fa637fc146101c7578063b25b9b00146101da575f5ffd5b80632e4f161a116100ce5780632e4f161a1461013b57806330017b3b1461015e57806360b5c3901461017157806365da41b914610184575f5ffd5b806305d09a70146100ff578063113764be146101145780631910d9731461012b5780632b97be2414610133575b5f5ffd5b61011261010d366004611d2b565b61027b565b005b6005545b6040519081526020015b60405180910390f35b600154610118565b600654610118565b61014e610149366004611dc5565b610564565b6040519015158152602001610122565b61011861016c366004611df4565b61057c565b61011861017f366004611e14565b610590565b61014e610192366004611e2b565b61059a565b61019f600481565b60405163ffffffff9091168152602001610122565b61014e6101c2366004611e97565b610746565b61014e6101d5366004611f18565b6108bb565b6101186101e8366004611fb7565b610a9a565b61014e6101fb366004612030565b610b17565b600254610118565b5f54610118565b61011261021d36600461206d565b600780547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000169215157fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00ff169290921761010091151591909102179055565b6102b987878080601f0160208091040260200160405190810160405280939291908181526020018383808284375f92019190915250610b2d92505050565b61030a5760405162461bcd60e51b815260206004820152601060248201527f4261642068656164657220626c6f636b0000000000000000000000000000000060448201526064015b60405180910390fd5b61034885858080601f0160208091040260200160405190810160405280939291908181526020018383808284375f92019190915250610b3492505050565b6103945760405162461bcd60e51b815260206004820152601660248201527f426164206d65726b6c652061727261792070726f6f66000000000000000000006044820152606401610301565b610413836103d689898080601f0160208091040260200160405190810160405280939291908181526020018383808284375f92019190915250610b4a92505050565b87878080601f0160208091040260200160405190810160405280939291908181526020018383808284375f92019190915250889250610b56915050565b61045f5760405162461bcd60e51b815260206004820152601360248201527f42616420696e636c7573696f6e2070726f6f66000000000000000000000000006044820152606401610301565b5f61049e88888080601f0160208091040260200160405190810160405280939291908181526020018383808284375f92019190915250610b8892505050565b90505f6104aa60025490565b90506104b98282610800610c60565b6105055760405162461bcd60e51b815260206004820152601b60248201527f47434420646f6573206e6f7420636f6e6669726d2068656164657200000000006044820152606401610301565b60ff8316600810156105595760405162461bcd60e51b815260206004820152601a60248201527f496e73756666696369656e7420636f6e6669726d6174696f6e730000000000006044820152606401610301565b505050505050505050565b5f61057185858585610c90565b90505b949350505050565b5f6105878383610d2a565b90505b92915050565b5f61058a82610d9c565b5f6105d983838080601f0160208091040260200160405190810160405280939291908181526020018383808284375f92019190915250610e4a92505050565b61064b5760405162461bcd60e51b815260206004820152602b60248201527f486561646572206172726179206c656e677468206d757374206265206469766960448201527f7369626c652062792038300000000000000000000000000000000000000000006064820152608401610301565b61068985858080601f0160208091040260200160405190810160405280939291908181526020018383808284375f92019190915250610b2d92505050565b6106d55760405162461bcd60e51b815260206004820152601760248201527f416e63686f72206d7573742062652038302062797465730000000000000000006044820152606401610301565b61057185858080601f0160208091040260200160405190810160405280939291908181526020018383808284375f9201919091525050604080516020601f890181900481028201810190925287815292508791508690819084018382808284375f9201829052509250610e59915050565b5f61078584848080601f0160208091040260200160405190810160405280939291908181526020018383808284375f92019190915250610b2d92505050565b80156107ca57506107ca86868080601f0160208091040260200160405190810160405280939291908181526020018383808284375f92019190915250610b2d92505050565b61083c5760405162461bcd60e51b815260206004820152602e60248201527f42616420617267732e20436865636b2068656164657220616e6420617272617960448201527f2062797465206c656e677468732e0000000000000000000000000000000000006064820152608401610301565b6108b08787878080601f0160208091040260200160405190810160405280939291908181526020018383808284375f9201919091525050604080516020601f8b0181900481028201810190925289815292508991508890819084018382808284375f92019190915250889250611246915050565b979650505050505050565b5f6108fa87878080601f0160208091040260200160405190810160405280939291908181526020018383808284375f92019190915250610b2d92505050565b801561093f575061093f85858080601f0160208091040260200160405190810160405280939291908181526020018383808284375f92019190915250610b2d92505050565b8015610984575061098483838080601f0160208091040260200160405190810160405280939291908181526020018383808284375f92019190915250610e4a92505050565b6109f65760405162461bcd60e51b815260206004820152602e60248201527f42616420617267732e20436865636b2068656164657220616e6420617272617960448201527f2062797465206c656e677468732e0000000000000000000000000000000000006064820152608401610301565b6108b087878080601f0160208091040260200160405190810160405280939291908181526020018383808284375f9201919091525050604080516020601f8b0181900481028201810190925289815292508991508890819084018382808284375f9201919091525050604080516020601f8a0181900481028201810190925288815292508891508790819084018382808284375f920191909152506114e392505050565b5f610b0d8686868080601f0160208091040260200160405190810160405280939291908181526020018383808284375f9201919091525050604080516020601f8a0181900481028201810190925288815292508891508790819084018382808284375f9201919091525061177592505050565b9695505050505050565b5f610b23848484610c60565b90505b9392505050565b5160501490565b5f60208251610b4391906120cb565b1592915050565b60448101515f9061058a565b5f8385148015610b64575081155b8015610b6f57508251155b15610b7c57506001610574565b61057185848685611906565b5f60028083604051610b9a91906120de565b602060405180830381855afa158015610bb5573d5f5f3e3d5ffd5b5050506040513d601f19601f82011682018060405250810190610bd891906120f4565b604051602001610bea91815260200190565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe081840301815290829052610c22916120de565b602060405180830381855afa158015610c3d573d5f5f3e3d5ffd5b5050506040513d601f19601f8201168201806040525081019061058a91906120f4565b6007545f9060ff1615610c7e5750600754610100900460ff16610b26565b610c898484846119ab565b9050610b26565b5f8385148015610c9f57508285145b15610cac57506001610574565b838381815f5b86811015610cf457898314610cd3575f838152600360205260409020549294505b898214610cec575f828152600360205260409020549193505b600101610cb2565b50828403610d08575f945050505050610574565b808214610d1b575f945050505050610574565b50600198975050505050505050565b5f82815b83811015610d4e575f918252600360205260409091205490600101610d2e565b50806105875760405162461bcd60e51b815260206004820152601060248201527f556e6b6e6f776e20616e636573746f72000000000000000000000000000000006044820152606401610301565b5f8082815b610dad60046001612138565b63ffffffff16811015610e01575f828152600460205260408120549350839003610de6575f918252600360205260409091205490610df9565b610df08184612154565b95945050505050565b600101610da1565b5060405162461bcd60e51b815260206004820152600d60248201527f556e6b6e6f776e20626c6f636b000000000000000000000000000000000000006044820152606401610301565b5f60508251610b4391906120cb565b5f5f610e6485610b88565b90505f610e7082610d9c565b90505f610e7c866119ec565b90508480610e91575080610e8f886119ec565b145b610f025760405162461bcd60e51b8152602060048201526024808201527f556e6578706563746564207265746172676574206f6e2065787465726e616c2060448201527f63616c6c000000000000000000000000000000000000000000000000000000006064820152608401610301565b85515f908190815b8181101561120357610f1d605082612167565b610f28906001612154565b610f329087612154565b9350610f408a8260506119f7565b5f818152600360205260409020549093506111165784611096845f8190506008817eff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff16901b600882901c7eff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff161790506010817dffff0000ffff0000ffff0000ffff0000ffff0000ffff0000ffff0000ffff16901b601082901c7dffff0000ffff0000ffff0000ffff0000ffff0000ffff0000ffff0000ffff161790506020817bffffffff00000000ffffffff00000000ffffffff00000000ffffffff16901b602082901c7bffffffff00000000ffffffff00000000ffffffff00000000ffffffff1617905060408177ffffffffffffffff0000000000000000ffffffffffffffff16901b604082901c77ffffffffffffffff0000000000000000ffffffffffffffff16179050608081901b608082901c179050919050565b11156110e45760405162461bcd60e51b815260206004820152601b60248201527f48656164657220776f726b20697320696e73756666696369656e7400000000006044820152606401610301565b5f8381526003602052604090208790556110ff6004856120cb565b5f03611116575f8381526004602052604090208490555b846111218b83611a1c565b1461116e5760405162461bcd60e51b815260206004820152601b60248201527f546172676574206368616e67656420756e65787065637465646c7900000000006044820152606401610301565b866111798b83611ab5565b146111ec5760405162461bcd60e51b815260206004820152602660248201527f4865616465727320646f206e6f7420666f726d206120636f6e73697374656e7460448201527f20636861696e00000000000000000000000000000000000000000000000000006064820152608401610301565b8296506050816111fc9190612154565b9050610f0a565b508161120e8b610b88565b6040517ff90e4f1d9cd0dd55e339411cbc9b152482307c3a23ed64715e4a2858f641a3f5905f90a35060019998505050505050505050565b5f6107e08211156112bf5760405162461bcd60e51b815260206004820152603360248201527f526571756573746564206c696d69742069732067726561746572207468616e2060448201527f3120646966666963756c747920706572696f64000000000000000000000000006064820152608401610301565b5f6112c984610b88565b90505f6112d586610b88565b905060015481146113285760405162461bcd60e51b815260206004820181905260248201527f50617373656420696e2062657374206973206e6f742062657374206b6e6f776e6044820152606401610301565b5f828152600360205260409020546113825760405162461bcd60e51b815260206004820152601360248201527f4e6577206265737420697320756e6b6e6f776e000000000000000000000000006044820152606401610301565b611390876001548487610c90565b6114025760405162461bcd60e51b815260206004820152602960248201527f416e636573746f72206d75737420626520686561766965737420636f6d6d6f6e60448201527f20616e636573746f7200000000000000000000000000000000000000000000006064820152608401610301565b8161140e888888611775565b146114815760405162461bcd60e51b815260206004820152603360248201527f4e65772062657374206861736820646f6573206e6f742068617665206d6f726560448201527f20776f726b207468616e2070726576696f7573000000000000000000000000006064820152608401610301565b600182905560028790555f61149586611acd565b905060055481146114a65760058190555b8783837f3cc13de64df0f0239626235c51a2da251bbc8c85664ecce39263da3ee03f606c60405160405180910390a4506001979650505050505050565b5f5f6114f66114f186610b88565b610d9c565b90505f6115056114f186610b88565b90506115136107e0826120cb565b6107df146115895760405162461bcd60e51b815260206004820152603d60248201527f4d7573742070726f7669646520746865206c61737420686561646572206f662060448201527f74686520636c6f73696e6720646966666963756c747920706572696f640000006064820152608401610301565b611595826107df612154565b81146116095760405162461bcd60e51b815260206004820152602860248201527f4d7573742070726f766964652065786163746c79203120646966666963756c7460448201527f7920706572696f640000000000000000000000000000000000000000000000006064820152608401610301565b61161285611acd565b61161b87611acd565b1461168e5760405162461bcd60e51b815260206004820152602760248201527f506572696f642068656164657220646966666963756c7469657320646f206e6f60448201527f74206d61746368000000000000000000000000000000000000000000000000006064820152608401610301565b5f611698856119ec565b90505f6116ca6116a7896119ec565b6116b08a611adf565b63ffffffff166116bf8a611adf565b63ffffffff16611b12565b9050818183161461171d5760405162461bcd60e51b815260206004820152601960248201527f496e76616c69642072657461726765742070726f7669646564000000000000006044820152606401610301565b5f61172789611acd565b9050806006541415801561175157506107e0611744600154610d9c565b61174e919061217a565b84115b1561175c5760068190555b61176888886001610e59565b9998505050505050505050565b5f5f61178085610d9c565b90505f61178f6114f186610b88565b90505f61179e6114f186610b88565b90508282101580156117b05750828110155b6118225760405162461bcd60e51b815260206004820152603060248201527f412064657363656e64616e74206865696768742069732062656c6f772074686560448201527f20616e636573746f7220686569676874000000000000000000000000000000006064820152608401610301565b5f61182f6107e0856120cb565b61183b856107e0612154565b611845919061217a565b90508083108183108115826118575750805b156118725761186589610b88565b9650505050505050610b26565b81801561187d575080155b1561188b5761186588610b88565b8180156118955750805b156118b957838510156118b0576118ab88610b88565b611865565b61186589610b88565b6118c288611acd565b6118ce6107e0866120cb565b6118d8919061218d565b6118e18a611acd565b6118ed6107e0886120cb565b6118f7919061218d565b10156118b05761186588610b88565b5f6020845161191591906120cb565b1561192157505f610574565b83515f0361193057505f610574565b81855f5b865181101561199e576119486002846120cb565b60010361196c5761196561195f8883016020015190565b83611b9a565b9150611985565b6119828261197d8984016020015190565b611b9a565b91505b60019290921c91611997602082612154565b9050611934565b5090931495945050505050565b5f82815b838110156119e1578582036119c957600192505050610b26565b5f9182526003602052604090912054906001016119af565b505f95945050505050565b5f61058a825f611a1c565b5f60205f8385602001870160025afa5060205f60205f60025afa50505f519392505050565b5f80611a33611a2c846048612154565b8590611ba5565b60e81c90505f84611a4585604b612154565b81518110611a5557611a556121a4565b016020015160f81c90505f611a87835f60108262ffffff16901c8261ff001660108462ffffff16901b17179050919050565b62ffffff1690505f611a9a6003846121d1565b60ff169050611aab816101006122cd565b6108b0908361218d565b5f610587611ac4836004612154565b84016020015190565b5f61058a611ada836119ec565b611bb3565b5f61058a611aec83611bda565b60d881901c63ff00ff001662ff00ff60e89290921c9190911617601081811b91901c1790565b5f80611b1e8385611be6565b9050611b2e621275006004611c41565b811015611b4657611b43621275006004611c41565b90505b611b54621275006004611c4c565b811115611b6c57611b69621275006004611c4c565b90505b5f611b8482611b7e8862010000611c41565b90611c4c565b9050610b0d62010000611b7e8362127500611c41565b5f6105878383611cbf565b5f6105878383016020015190565b5f61058a7bffff000000000000000000000000000000000000000000000000000083611c41565b5f61058a826044611ba5565b5f82821115611c375760405162461bcd60e51b815260206004820152601d60248201527f556e646572666c6f7720647572696e67207375627472616374696f6e2e0000006044820152606401610301565b610587828461217a565b5f6105878284612167565b5f825f03611c5b57505f61058a565b611c65828461218d565b905081611c728483612167565b1461058a5760405162461bcd60e51b815260206004820152601f60248201527f4f766572666c6f7720647572696e67206d756c7469706c69636174696f6e2e006044820152606401610301565b5f825f528160205260205f60405f60025afa5060205f60205f60025afa50505f5192915050565b5f5f83601f840112611cf6575f5ffd5b50813567ffffffffffffffff811115611d0d575f5ffd5b602083019150836020828501011115611d24575f5ffd5b9250929050565b5f5f5f5f5f5f5f60a0888a031215611d41575f5ffd5b873567ffffffffffffffff811115611d57575f5ffd5b611d638a828b01611ce6565b909850965050602088013567ffffffffffffffff811115611d82575f5ffd5b611d8e8a828b01611ce6565b9096509450506040880135925060608801359150608088013560ff81168114611db5575f5ffd5b8091505092959891949750929550565b5f5f5f5f60808587031215611dd8575f5ffd5b5050823594602084013594506040840135936060013592509050565b5f5f60408385031215611e05575f5ffd5b50508035926020909101359150565b5f60208284031215611e24575f5ffd5b5035919050565b5f5f5f5f60408587031215611e3e575f5ffd5b843567ffffffffffffffff811115611e54575f5ffd5b611e6087828801611ce6565b909550935050602085013567ffffffffffffffff811115611e7f575f5ffd5b611e8b87828801611ce6565b95989497509550505050565b5f5f5f5f5f5f60808789031215611eac575f5ffd5b86359550602087013567ffffffffffffffff811115611ec9575f5ffd5b611ed589828a01611ce6565b909650945050604087013567ffffffffffffffff811115611ef4575f5ffd5b611f0089828a01611ce6565b979a9699509497949695606090950135949350505050565b5f5f5f5f5f5f60608789031215611f2d575f5ffd5b863567ffffffffffffffff811115611f43575f5ffd5b611f4f89828a01611ce6565b909750955050602087013567ffffffffffffffff811115611f6e575f5ffd5b611f7a89828a01611ce6565b909550935050604087013567ffffffffffffffff811115611f99575f5ffd5b611fa589828a01611ce6565b979a9699509497509295939492505050565b5f5f5f5f5f60608688031215611fcb575f5ffd5b85359450602086013567ffffffffffffffff811115611fe8575f5ffd5b611ff488828901611ce6565b909550935050604086013567ffffffffffffffff811115612013575f5ffd5b61201f88828901611ce6565b969995985093965092949392505050565b5f5f5f60608486031215612042575f5ffd5b505081359360208301359350604090920135919050565b80358015158114612068575f5ffd5b919050565b5f5f6040838503121561207e575f5ffd5b61208783612059565b915061209560208401612059565b90509250929050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f826120d9576120d961209e565b500690565b5f82518060208501845e5f920191825250919050565b5f60208284031215612104575f5ffd5b5051919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b63ffffffff818116838216019081111561058a5761058a61210b565b8082018082111561058a5761058a61210b565b5f826121755761217561209e565b500490565b8181038181111561058a5761058a61210b565b808202811582820484141761058a5761058a61210b565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b60ff828116828216039081111561058a5761058a61210b565b6001815b6001841115612225578085048111156122095761220961210b565b600184161561221757908102905b60019390931c9280026121ee565b935093915050565b5f8261223b5750600161058a565b8161224757505f61058a565b816001811461225d576002811461226757612283565b600191505061058a565b60ff8411156122785761227861210b565b50506001821b61058a565b5060208310610133831016604e8410600b84101617156122a6575081810a61058a565b6122b25f1984846121ea565b805f19048211156122c5576122c561210b565b029392505050565b5f610587838361222d56fea2646970667358221220576a2c00826e6fb9bd085a811062dcfcb24df232a33748114708d8262ad50fad64736f6c634300081c00330000002073bd2184edd9c4fc76642ea6754ee40136970efc10c4190000000000000000000296ef123ea96da5cf695f22bf7d94be87d49db1ad7ac371ac43c4da4161c8c216349c5ba11928170d38782b024897070000000000220020a4333e5612ab1a1043b25755c89b16d55184a42f81799e623e6bc39db8539c180000000000000000166a14edb1b5c2f39af0fec151732585b1049b0789521148e5a1a0e616d8fd92b4ef228c424e0c816799a256c6a90892195ccfc53300d6e35a0d6de94b656694589964a252957e4673a9fb1d2f8b4a92e3f0a7bb654fddb94e5a1e6d7f7f499fd1be5dd30a73bf5584bf137da5fdd77cc21aeb95b9e35788894be019284bd4fbed6dd6118ac2cb6d26bc4be4e423f55a3a48f2874d8d02a65d9c87d07de21d4dfe7b0a9f4a23cc9a58373e9e6931fefdb5afade5df54c91104048df1ee999240617984e18b6f931e2373673d0195b8c6987d7ff7650d5ce53bcec46e13ab4f2da1146a7fc621ee672f62bc22742486392d75e55e67b09960c3386a0b49e75f1723d6ab28ac9a2028a0c72866e2111d79d4817b88e17c821937847768d92837bae3832bb8e5a4ab4434b97e00a6c10182f211f592409068d6f5652400d9a3d1cc150a7fb692e874cc42d76bdafc842f2fe0f835a7c24d2d60c109b187d64571efbaa8047be85821f8e67e0e85f2f5894bc63d00c2ed9d64011746bd867400f3494b8f44c24b83e1aa58c4f0ff25b4a61cffeffd4bc0f9ba300000000000ffffffff
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x07\x80T`\x01`\xFF\x19\x91\x82\x16\x81\x17\x90\x92U`\x0B\x80T\x90\x91\x16\x90\x91\x17\x90Ua\x01\0`@R`P`\x80\x81\x81R\x90ajl`\xA09`\x1E\x90a\0=\x90\x82a\tBV[P`@Q\x80a\x01`\x01`@R\x80a\x01@\x81R` \x01ak'a\x01@\x919`\x1F\x90a\0g\x90\x82a\tBV[P_Q` ak\x07_9_Q\x90_R` Ua\x01\x19`!U`@Qa\0\x8B\x90a\x08\x90V[`@Q\x80\x91\x03\x90_\xF0\x80\x15\x80\x15a\0\xA4W=__>=_\xFD[P`\"_a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP`\x01`#U_Q` ak\x07_9_Q\x90_R`$U`@Q\x80`\x80\x01`@R\x80`\x01`\xF8\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01`@Q\x80``\x01`@R\x80`*\x81R` \x01alg`*\x919\x81R` \x01`@Q\x80`\x80\x01`@R\x80`K\x81R` \x01aj\xBC`K\x919\x81R_` \x91\x82\x01R\x81Q`%\x80Tc\xFF\xFF\xFF\xFF\x19\x16`\xE0\x92\x90\x92\x1C\x91\x90\x91\x17\x81U\x90\x82\x01Q`&\x90a\x01i\x90\x82a\tBV[P`@\x82\x01Q`\x02\x82\x01\x90a\x01~\x90\x82a\tBV[P``\x82\x01Q\x81`\x03\x01_a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83`\xE0\x1C\x02\x17\x90UPPP`@Q\x80``\x01`@R\x80`\x1F\x80Ta\x01\xBC\x90a\x08\xBEV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x01\xE8\x90a\x08\xBEV[\x80\x15a\x023W\x80`\x1F\x10a\x02\nWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x023V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x02\x16W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01a\x01\x19\x81R` \x01`\x1E\x80Ta\x02R\x90a\x08\xBEV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x02~\x90a\x08\xBEV[\x80\x15a\x02\xC9W\x80`\x1F\x10a\x02\xA0Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x02\xC9V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x02\xACW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x90\x92RPP\x80Q`)\x90\x81\x90a\x02\xE3\x90\x82a\tBV[P` \x82\x01Q`\x01\x82\x01U`@\x82\x01Q`\x02\x82\x01\x90a\x03\x02\x90\x82a\tBV[PPP4\x80\x15a\x03\x10W__\xFD[P`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01k42\xB0\xB22\xB99\x9759\xB7\xB7`\xA1\x1B\x81RP`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01k\x05\xCC\xEC\xAD\xCC\xAEm.e\xCD\x0C\xAF`\xA3\x1B\x81RP`@Q\x80`@\x01`@R\x80`\x0F\x81R` \x01n\x0B\x99\xD9[\x99\\\xDA\\\xCB\x9A\x19ZY\xDA\x1D`\x8A\x1B\x81RP`@Q\x80`@\x01`@R\x80`\x12\x81R` \x01q.genesis.digest_le`p\x1B\x81RP_\x7F\x88\\\xB6\x92@\xA95\xD62\xD7\x9C1q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-_\x1C`\x01`\x01`\xA0\x1B\x03\x16c\xD90\xA0\xE6`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\x0CW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x043\x91\x90\x81\x01\x90a\nqV[\x90P_\x81\x86`@Q` \x01a\x04I\x92\x91\x90a\n\xD4V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Rc`\xF9\xBB\x11`\xE0\x1B\x82R\x91Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c`\xF9\xBB\x11\x90a\x04\x90\x90\x84\x90`\x04\x01a\x0BFV[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xAAW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x04\xD1\x91\x90\x81\x01\x90a\nqV[`\x1D\x90a\x04\xDE\x90\x82a\tBV[Pa\x05u\x85`\x1D\x80Ta\x04\xF0\x90a\x08\xBEV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x05\x1C\x90a\x08\xBEV[\x80\x15a\x05gW\x80`\x1F\x10a\x05>Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x05gV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x05JW\x82\x90\x03`\x1F\x16\x82\x01\x91[P\x93\x94\x93PPa\x07W\x91PPV[a\x06\x0B\x85`\x1D\x80Ta\x05\x86\x90a\x08\xBEV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x05\xB2\x90a\x08\xBEV[\x80\x15a\x05\xFDW\x80`\x1F\x10a\x05\xD4Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x05\xFDV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x05\xE0W\x82\x90\x03`\x1F\x16\x82\x01\x91[P\x93\x94\x93PPa\x07\xDB\x91PPV[a\x06\xA1\x85`\x1D\x80Ta\x06\x1C\x90a\x08\xBEV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x06H\x90a\x08\xBEV[\x80\x15a\x06\x93W\x80`\x1F\x10a\x06jWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x06\x93V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x06vW\x82\x90\x03`\x1F\x16\x82\x01\x91[P\x93\x94\x93PPa\x08U\x91PPV[`@Qa\x06\xAD\x90a\x08\x9DV[a\x06\xB9\x93\x92\x91\x90a\x0BXV[`@Q\x80\x91\x03\x90_\xF0\x80\x15\x80\x15a\x06\xD2W=__>=_\xFD[P`\x1C\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x82\x17\x90U`@Qc\xF5\x8D\xB0o`\xE0\x1B\x81R`\x01`\x04\x82\x01\x81\x90R`$\x82\x01R\x90\x96Pc\xF5\x8D\xB0o\x95P`D\x01\x93Pa\x07%\x92PPPV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x07<W__\xFD[PZ\xF1\x15\x80\x15a\x07NW=__>=_\xFD[PPPPa\x0B\xB7V[`@Qc\x1F\xB2C}`\xE3\x1B\x81R``\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\xFD\x92\x1B\xE8\x90a\x07\x93\x90\x86\x90\x86\x90`\x04\x01a\x0B|V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\xADW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x07\xD4\x91\x90\x81\x01\x90a\nqV[\x93\x92PPPV[`@QcV\xEE\xF1[`\xE1\x1B\x81R_\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\xAD\xDD\xE2\xB6\x90a\x08\x16\x90\x86\x90\x86\x90`\x04\x01a\x0B|V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x081W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xD4\x91\x90a\x0B\xA0V[`@Qc\x17w\xE5\x9D`\xE0\x1B\x81R_\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\x17w\xE5\x9D\x90a\x08\x16\x90\x86\x90\x86\x90`\x04\x01a\x0B|V[a\x0B\x88\x80a6\x0C\x839\x01\x90V[a(\xD8\x80aA\x94\x839\x01\x90V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x08\xD2W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x08\xF0WcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\t=W\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a\t\x1BWP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\t:W_\x81U`\x01\x01a\t'V[PP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\t[Wa\t[a\x08\xAAV[a\to\x81a\ti\x84Ta\x08\xBEV[\x84a\x08\xF6V[` `\x1F\x82\x11`\x01\x81\x14a\t\xA1W_\x83\x15a\t\x8AWP\x84\x82\x01Q[_\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua\t:V[_\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a\t\xD0W\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a\t\xB0V[P\x84\x82\x10\x15a\t\xEDW\x86\x84\x01Q_\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[_\x80`\x01`\x01`@\x1B\x03\x84\x11\x15a\n\x15Wa\n\x15a\x08\xAAV[P`@Q`\x1F\x19`\x1F\x85\x01\x81\x16`?\x01\x16\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a\nCWa\nCa\x08\xAAV[`@R\x83\x81R\x90P\x80\x82\x84\x01\x85\x10\x15a\nZW__\xFD[\x83\x83` \x83\x01^_` \x85\x83\x01\x01RP\x93\x92PPPV[_` \x82\x84\x03\x12\x15a\n\x81W__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\n\x96W__\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\n\xA6W__\xFD[a\n\xB5\x84\x82Q` \x84\x01a\t\xFCV[\x94\x93PPPPV[_\x81Q\x80` \x84\x01\x85^_\x93\x01\x92\x83RP\x90\x91\x90PV[_a\n\xDF\x82\x85a\n\xBDV[\x7F/test/fullRelay/testData/\0\0\0\0\0\0\0\x81Ra\x0B\x0F`\x19\x82\x01\x85a\n\xBDV[\x95\x94PPPPPV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R_a\x07\xD4` \x83\x01\x84a\x0B\x18V[``\x81R_a\x0Bj``\x83\x01\x86a\x0B\x18V[` \x83\x01\x94\x90\x94RP`@\x01R\x91\x90PV[`@\x81R_a\x0B\x8E`@\x83\x01\x85a\x0B\x18V[\x82\x81\x03` \x84\x01Ra\x0B\x0F\x81\x85a\x0B\x18V[_` \x82\x84\x03\x12\x15a\x0B\xB0W__\xFD[PQ\x91\x90PV[a*H\x80a\x0B\xC4_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x01cW_5`\xE0\x1C\x80c\x91j\x17\xC6\x11a\0\xC7W\x80c\xE2\x0C\x9Fq\x11a\0}W\x80c\xEF>h\x12\x11a\0cW\x80c\xEF>h\x12\x14a\x02\x86W\x80c\xFAv&\xD4\x14a\x02\x8EW\x80c\xFA\xD0k\x8F\x14a\x02\x9BW__\xFD[\x80c\xE2\x0C\x9Fq\x14a\x02vW\x80c\xE8\x9F\x84\x19\x14a\x02~W__\xFD[\x80c\xB5P\x8A\xA9\x11a\0\xADW\x80c\xB5P\x8A\xA9\x14a\x02NW\x80c\xBAAO\xA6\x14a\x02VW\x80c\xC8\x99\xD2A\x14a\x02nW__\xFD[\x80c\x91j\x17\xC6\x14a\x02>W\x80c\xB5+ X\x14a\x02FW__\xFD[\x80c?r\x86\xF4\x11a\x01\x1CW\x80c]\xF0\xD0v\x11a\x01\x02W\x80c]\xF0\xD0v\x14a\x02\nW\x80cf\xD9\xA9\xA0\x14a\x02\x14W\x80c\x85\"l\x81\x14a\x02)W__\xFD[\x80c?r\x86\xF4\x14a\x01\xE2W\x80cD\xBA\xDB\xB6\x14a\x01\xEAW__\xFD[\x80c\x1E\xD7\x83\x1C\x11a\x01LW\x80c\x1E\xD7\x83\x1C\x14a\x01\xB0W\x80c*\xDE8\x80\x14a\x01\xC5W\x80c>^<#\x14a\x01\xDAW__\xFD[\x80c\x08\x13\x85*\x14a\x01gW\x80c\x1C\r\xA8\x1F\x14a\x01\x90W[__\xFD[a\x01za\x01u6`\x04a \x0BV[a\x02\xAEV[`@Qa\x01\x87\x91\x90a \xC0V[`@Q\x80\x91\x03\x90\xF3[a\x01\xA3a\x01\x9E6`\x04a \x0BV[a\x02\xF9V[`@Qa\x01\x87\x91\x90a!#V[a\x01\xB8a\x03kV[`@Qa\x01\x87\x91\x90a!5V[a\x01\xCDa\x03\xD8V[`@Qa\x01\x87\x91\x90a!\xE7V[a\x01\xB8a\x05!V[a\x01\xB8a\x05\x8CV[a\x01\xFDa\x01\xF86`\x04a \x0BV[a\x05\xF7V[`@Qa\x01\x87\x91\x90a\"kV[a\x02\x12a\x06:V[\0[a\x02\x1Ca\t2V[`@Qa\x01\x87\x91\x90a\"\xA2V[a\x021a\n5V[`@Qa\x01\x87\x91\x90a#sV[a\x02\x1Ca\x0B\0V[a\x02\x12a\x0C\x03V[a\x021a\x0C\xBAV[a\x02^a\r\x85V[`@Q\x90\x15\x15\x81R` \x01a\x01\x87V[a\x02\x12a\x0E\xC2V[a\x01\xB8a\x10\xF6V[a\x02\x12a\x11aV[a\x02\x12a\x13\xC7V[`\x07Ta\x02^\x90`\xFF\x16\x81V[a\x01\xFDa\x02\xA96`\x04a \x0BV[a\x15\xECV[``a\x02\xF1\x84\x84\x84`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01\x7Fhex\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x16/V[\x94\x93PPPPV[``_a\x03\x07\x85\x85\x85a\x02\xAEV[\x90P_[a\x03\x15\x85\x85a#\xB2V[\x81\x10\x15a\x03bW\x82\x82\x82\x81Q\x81\x10a\x03/Wa\x03/a#\xC5V[` \x02` \x01\x01Q`@Q` \x01a\x03H\x92\x91\x90a$\tV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x92P`\x01\x01a\x03\x0BV[PP\x93\x92PPPV[```\x14\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x03\xCEW` \x02\x82\x01\x91\x90_R` _ \x90[\x81Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x03\xA3W[PPPPP\x90P\x90V[```\x1B\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x05\x18W_\x84\x81R` \x80\x82 `@\x80Q\x80\x82\x01\x82R`\x02\x87\x02\x90\x92\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x95\x91\x94\x86\x81\x01\x94\x91\x92\x90\x84\x01[\x82\x82\x10\x15a\x05\x01W\x83\x82\x90_R` _ \x01\x80Ta\x04v\x90a$\x1DV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x04\xA2\x90a$\x1DV[\x80\x15a\x04\xEDW\x80`\x1F\x10a\x04\xC4Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x04\xEDV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x04\xD0W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x04YV[PPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x03\xFBV[PPPP\x90P\x90V[```\x16\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x03\xCEW` \x02\x82\x01\x91\x90_R` _ \x90\x81Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x03\xA3WPPPPP\x90P\x90V[```\x15\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x03\xCEW` \x02\x82\x01\x91\x90_R` _ \x90\x81Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x03\xA3WPPPPP\x90P\x90V[``a\x02\xF1\x84\x84\x84`@Q\x80`@\x01`@R\x80`\t\x81R` \x01\x7Fdigest_le\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x17\x90V[`@\x80Q`\x80\x81\x01\x90\x91R`%\x80T`\xE0\x1B\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x82R`&\x80T_\x93\x92\x91` \x84\x01\x91a\x06\x86\x90a$\x1DV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x06\xB2\x90a$\x1DV[\x80\x15a\x06\xFDW\x80`\x1F\x10a\x06\xD4Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x06\xFDV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x06\xE0W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x02\x82\x01\x80Ta\x07\x16\x90a$\x1DV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x07B\x90a$\x1DV[\x80\x15a\x07\x8DW\x80`\x1F\x10a\x07dWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x07\x8DV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x07pW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x83RPP`\x03\x91\x90\x91\x01T`\xE0\x1B\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16` \x91\x82\x01R`@\x80Q\x80\x82\x01\x82R`\x01\x81R_\x81\x84\x01R\x83\x82\x01R\x80Q\x80\x82\x01\x82R`\x1E\x81R\x7FInvalid output vector provided\0\0\x92\x81\x01\x92\x90\x92RQ\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x91\x92Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x91c\xF2\x8D\xCE\xB3\x91a\x08_\x91`\x04\x01a!#V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x08vW__\xFD[PZ\xF1\x15\x80\x15a\x08\x88W=__>=_\xFD[PP`\"T`\x1CT`#T`@Q\x7FQX\x8B\x84\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x84\x16\x95PcQX\x8B\x84\x94Pa\x08\xEF\x93\x90\x92\x16\x91\x86\x90`)\x90`\x04\x01a%tV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\nW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t.\x91\x90a&eV[PPV[```\x19\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x05\x18W_\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\n\x1DW` \x02\x82\x01\x91\x90_R` _ \x90_\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\t\xCAW\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\tUV[```\x18\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x05\x18W\x83\x82\x90_R` _ \x01\x80Ta\nu\x90a$\x1DV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\n\xA1\x90a$\x1DV[\x80\x15a\n\xECW\x80`\x1F\x10a\n\xC3Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\n\xECV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\n\xCFW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\nXV[```\x1A\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x05\x18W_\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x0B\xEBW` \x02\x82\x01\x91\x90_R` _ \x90_\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x0B\x98W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x0B#V[`\"T`\x1CT`#T`@Q\x7FQX\x8B\x84\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R_\x93s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x93cQX\x8B\x84\x93a\x0Cj\x93\x92\x90\x91\x16\x91`%\x90`)\x90`\x04\x01a'\rV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\x85W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xA9\x91\x90a&eV[\x90Pa\x0C\xB7\x81`$Ta\x18\xDEV[PV[```\x17\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x05\x18W\x83\x82\x90_R` _ \x01\x80Ta\x0C\xFA\x90a$\x1DV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\r&\x90a$\x1DV[\x80\x15a\rqW\x80`\x1F\x10a\rHWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\rqV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\rTW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x0C\xDDV[`\x07T_\x90a\x01\0\x90\x04`\xFF\x16\x15a\r\xA6WP`\x07Ta\x01\0\x90\x04`\xFF\x16\x90V[_sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0E\xBDW`@\x80Qsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-` \x82\x01\x81\x90R\x7Ffailed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x84\x01R\x82Q\x80\x83\x03\x84\x01\x81R``\x83\x01\x90\x93R_\x92\x90\x91a\x0EI\x91\x7Ff\x7F\x9Dp\xCAA\x1Dp\xEA\xD5\r\x8D\\\"\x07\r\xAF\xC3j\xD7_=\xCF^r7\xB2*\xDE\x9A\xEC\xC4\x91`\x80\x01a'SV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x0Ec\x91a'\x86V[_`@Q\x80\x83\x03\x81_\x86Z\xF1\x91PP=\x80_\x81\x14a\x0E\x9CW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x0E\xA1V[``\x91P[P\x91PP\x80\x80` \x01\x90Q\x81\x01\x90a\x0E\xB9\x91\x90a'\x91V[\x91PP[\x91\x90PV[`@\x80Q`\x80\x81\x01\x90\x91R`%\x80T`\xE0\x1B\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x82R`&\x80T_\x93\x92\x91` \x84\x01\x91a\x0F\x0E\x90a$\x1DV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0F:\x90a$\x1DV[\x80\x15a\x0F\x85W\x80`\x1F\x10a\x0F\\Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0F\x85V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0FhW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x02\x82\x01\x80Ta\x0F\x9E\x90a$\x1DV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0F\xCA\x90a$\x1DV[\x80\x15a\x10\x15W\x80`\x1F\x10a\x0F\xECWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x10\x15V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0F\xF8W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x83RPP`\x03\x91\x90\x91\x01T`\xE0\x1B\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16` \x91\x82\x01R|\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x83\x01R`@\x80Q\x80\x82\x01\x82R`\x13\x81R\x7FBad inclusion proof\0\0\0\0\0\0\0\0\0\0\0\0\0\x92\x81\x01\x92\x90\x92RQ\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x91\x92Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x91c\xF2\x8D\xCE\xB3\x91a\x08_\x91`\x04\x01a!#V[```\x13\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x03\xCEW` \x02\x82\x01\x91\x90_R` _ \x90\x81Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x03\xA3WPPPPP\x90P\x90V[_`)`@Q\x80``\x01`@R\x90\x81_\x82\x01\x80Ta\x11~\x90a$\x1DV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x11\xAA\x90a$\x1DV[\x80\x15a\x11\xF5W\x80`\x1F\x10a\x11\xCCWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x11\xF5V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x11\xD8W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x01\x82\x01T\x81R` \x01`\x02\x82\x01\x80Ta\x12\x18\x90a$\x1DV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x12D\x90a$\x1DV[\x80\x15a\x12\x8FW\x80`\x1F\x10a\x12fWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x12\x8FV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x12rW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x90\x92RPP`@\x80Q\x80\x82\x01\x82R`\x01\x81R_` \x80\x83\x01\x91\x90\x91R\x90\x84R\x81Q\x80\x83\x01\x83R`\x16\x81R\x7FBad merkle array proof\0\0\0\0\0\0\0\0\0\0\x91\x81\x01\x91\x90\x91R\x90Q\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x92\x93Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x92c\xF2\x8D\xCE\xB3\x92Pa\x137\x91\x90`\x04\x01a!#V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x13NW__\xFD[PZ\xF1\x15\x80\x15a\x13`W=__>=_\xFD[PP`\"T`\x1CT`#T`@Q\x7FQX\x8B\x84\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x84\x16\x95PcQX\x8B\x84\x94Pa\x08\xEF\x93\x90\x92\x16\x91`%\x90\x87\x90`\x04\x01a'\xB7V[`@\x80Q`\x80\x81\x01\x90\x91R`%\x80T`\xE0\x1B\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x82R`&\x80T_\x93\x92\x91` \x84\x01\x91a\x14\x13\x90a$\x1DV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x14?\x90a$\x1DV[\x80\x15a\x14\x8AW\x80`\x1F\x10a\x14aWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x14\x8AV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x14mW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x02\x82\x01\x80Ta\x14\xA3\x90a$\x1DV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x14\xCF\x90a$\x1DV[\x80\x15a\x15\x1AW\x80`\x1F\x10a\x14\xF1Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x15\x1AV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x14\xFDW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x83RPP`\x03\x91\x90\x91\x01T`\xE0\x1B\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16` \x91\x82\x01R`@\x80Q\x80\x82\x01\x82R`\x01\x81R_\x81\x84\x01R\x83\x83\x01R\x80Q\x80\x82\x01\x82R`\x1D\x81R\x7FInvalid input vector provided\0\0\0\x92\x81\x01\x92\x90\x92RQ\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x91\x92Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x91c\xF2\x8D\xCE\xB3\x91a\x08_\x91`\x04\x01a!#V[``a\x02\xF1\x84\x84\x84`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01\x7Fheight\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x1AFV[``a\x16;\x84\x84a#\xB2V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x16SWa\x16Sa\x1F\x86V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x16\x86W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x16qW\x90P[P\x90P\x83[\x83\x81\x10\x15a\x17\x87Wa\x17Y\x86a\x16\xA0\x83a\x1B\x94V[\x85`@Q` \x01a\x16\xB3\x93\x92\x91\x90a(6V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`\x1D\x80Ta\x16\xCF\x90a$\x1DV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x16\xFB\x90a$\x1DV[\x80\x15a\x17FW\x80`\x1F\x10a\x17\x1DWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x17FV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x17)W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPPa\x1C\xC5\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x82a\x17d\x87\x84a#\xB2V[\x81Q\x81\x10a\x17tWa\x17ta#\xC5V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x16\x8BV[P\x94\x93PPPPV[``a\x17\x9C\x84\x84a#\xB2V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x17\xB4Wa\x17\xB4a\x1F\x86V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x17\xDDW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P\x83[\x83\x81\x10\x15a\x17\x87Wa\x18\xB0\x86a\x17\xF7\x83a\x1B\x94V[\x85`@Q` \x01a\x18\n\x93\x92\x91\x90a(6V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`\x1D\x80Ta\x18&\x90a$\x1DV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x18R\x90a$\x1DV[\x80\x15a\x18\x9DW\x80`\x1F\x10a\x18tWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x18\x9DV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x18\x80W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPPa\x1Dd\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x82a\x18\xBB\x87\x84a#\xB2V[\x81Q\x81\x10a\x18\xCBWa\x18\xCBa#\xC5V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x17\xE2V[\x80\x82\x14a\t.W\x7FA0O\xAC\xD92=u\xB1\x1B\xCD\xD6\t\xCB8\xEF\xFF\xFD\xB0W\x10\xF7\xCA\xF0\xE9\xB1lm\x9Dp\x9FP`@Qa\x19j\x90` \x80\x82R`%\x90\x82\x01R\x7FError: a == b not satisfied [byt`@\x82\x01R\x7Fes32]\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x82\x01R`\x80\x01\x90V[`@Q\x80\x91\x03\x90\xA1`@\x80Q\x81\x81R`\n\x81\x83\x01R\x7F      Left\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x82\x01R` \x81\x01\x84\x90R\x90Q\x7F\xAF\xB7\x95\xC9\xC6\x1EO\xE7F\x8C8o\x92]zT)\xEC\xAD\x9C\x04\x95\xDD\xB8\xD3\x8Di\x06\x14\xD3/\x99\x91\x81\x90\x03`\x80\x01\x90\xA1`@\x80Q\x81\x81R`\n\x81\x83\x01R\x7F     Right\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x82\x01R` \x81\x01\x83\x90R\x90Q\x7F\xAF\xB7\x95\xC9\xC6\x1EO\xE7F\x8C8o\x92]zT)\xEC\xAD\x9C\x04\x95\xDD\xB8\xD3\x8Di\x06\x14\xD3/\x99\x91\x81\x90\x03`\x80\x01\x90\xA1a\t.a\x1D\xF7V[``a\x1AR\x84\x84a#\xB2V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1AjWa\x1Aja\x1F\x86V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1A\x93W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P\x83[\x83\x81\x10\x15a\x17\x87Wa\x1Bf\x86a\x1A\xAD\x83a\x1B\x94V[\x85`@Q` \x01a\x1A\xC0\x93\x92\x91\x90a(6V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`\x1D\x80Ta\x1A\xDC\x90a$\x1DV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1B\x08\x90a$\x1DV[\x80\x15a\x1BSW\x80`\x1F\x10a\x1B*Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1BSV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1B6W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPPa\x1F2\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x82a\x1Bq\x87\x84a#\xB2V[\x81Q\x81\x10a\x1B\x81Wa\x1B\x81a#\xC5V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x1A\x98V[``\x81_\x03a\x1B\xD6WPP`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R\x7F0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x90V[\x81_[\x81\x15a\x1B\xFFW\x80a\x1B\xE9\x81a(\xD3V[\x91Pa\x1B\xF8\x90P`\n\x83a)7V[\x91Pa\x1B\xD9V[_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1C\x19Wa\x1C\x19a\x1F\x86V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x1CCW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P[\x84\x15a\x02\xF1Wa\x1CX`\x01\x83a#\xB2V[\x91Pa\x1Ce`\n\x86a)JV[a\x1Cp\x90`0a)]V[`\xF8\x1B\x81\x83\x81Q\x81\x10a\x1C\x85Wa\x1C\x85a#\xC5V[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81_\x1A\x90SPa\x1C\xBE`\n\x86a)7V[\x94Pa\x1CGV[`@Q\x7F\xFD\x92\x1B\xE8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R``\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\xFD\x92\x1B\xE8\x90a\x1D\x1A\x90\x86\x90\x86\x90`\x04\x01a)pV[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1D4W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1D[\x91\x90\x81\x01\x90a)\x9DV[\x90P[\x92\x91PPV[`@Q\x7F\x17w\xE5\x9D\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R_\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\x17w\xE5\x9D\x90a\x1D\xB8\x90\x86\x90\x86\x90`\x04\x01a)pV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1D\xD3W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D[\x91\x90a&eV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x1F\x04W`@\x80Qsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-` \x82\x01\x81\x90R\x7Ffailed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92\x82\x01\x92\x90\x92R`\x01``\x82\x01R_\x91\x90\x7Fp\xCA\x10\xBB\xD0\xDB\xFD\x90 \xA9\xF4\xB14\x02\xC1l\xB1 p^\r\x1C\n\xEA\xB1\x0F\xA3S\xAEXo\xC4\x90`\x80\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x1E\xA7\x92\x91` \x01a'SV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x1E\xC1\x91a'\x86V[_`@Q\x80\x83\x03\x81_\x86Z\xF1\x91PP=\x80_\x81\x14a\x1E\xFAW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x1E\xFFV[``\x91P[PPPP[`\x07\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\x16a\x01\0\x17\x90UV[`@Q\x7F\xAD\xDD\xE2\xB6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R_\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\xAD\xDD\xE2\xB6\x90a\x1D\xB8\x90\x86\x90\x86\x90`\x04\x01a)pV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1F\xDCWa\x1F\xDCa\x1F\x86V[`@R\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x1F\xFDWa\x1F\xFDa\x1F\x86V[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[___``\x84\x86\x03\x12\x15a \x1DW__\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a 3W__\xFD[\x84\x01`\x1F\x81\x01\x86\x13a CW__\xFD[\x805a Va Q\x82a\x1F\xE4V[a\x1F\xB3V[\x81\x81R\x87` \x83\x85\x01\x01\x11\x15a jW__\xFD[\x81` \x84\x01` \x83\x017_` \x92\x82\x01\x83\x01R\x97\x90\x86\x015\x96P`@\x90\x95\x015\x94\x93PPPPV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a!\x17W`?\x19\x87\x86\x03\x01\x84Ra!\x02\x85\x83Qa \x92V[\x94P` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a \xE6V[P\x92\x96\x95PPPPPPV[` \x81R_a\x1D[` \x83\x01\x84a \x92V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a!\x82W\x83Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a!NV[P\x90\x95\x94PPPPPV[_\x82\x82Q\x80\x85R` \x85\x01\x94P` \x81`\x05\x1B\x83\x01\x01` \x85\x01_[\x83\x81\x10\x15a!\xDBW`\x1F\x19\x85\x84\x03\x01\x88Ra!\xC5\x83\x83Qa \x92V[` \x98\x89\x01\x98\x90\x93P\x91\x90\x91\x01\x90`\x01\x01a!\xA9V[P\x90\x96\x95PPPPPPV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a!\x17W`?\x19\x87\x86\x03\x01\x84R\x81Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16\x86R` \x81\x01Q\x90P`@` \x87\x01Ra\"U`@\x87\x01\x82a!\x8DV[\x95PP` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a\"\rV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a!\x82W\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\"\x84V[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a!\x17W\x86\x85\x03`?\x19\x01\x84R\x81Q\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x86R` \x90\x81\x01Q`@\x82\x88\x01\x81\x90R\x81Q\x90\x88\x01\x81\x90R\x91\x01\x90_\x90``\x88\x01\x90[\x80\x83\x10\x15a#[W\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84Q\x16\x82R` \x82\x01\x91P` \x84\x01\x93P`\x01\x83\x01\x92Pa#\x16V[P\x96PPP` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a\"\xC8V[` \x81R_a\x1D[` \x83\x01\x84a!\x8DV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x1D^Wa\x1D^a#\x85V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_\x81Q\x80` \x84\x01\x85^_\x93\x01\x92\x83RP\x90\x91\x90PV[_a\x02\xF1a$\x17\x83\x86a#\xF2V[\x84a#\xF2V[`\x01\x81\x81\x1C\x90\x82\x16\x80a$1W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a$hW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[P\x91\x90PV[\x80T_\x90`\x01\x81\x81\x1C\x90\x82\x16\x80a$\x86W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a$\xBDW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[\x81\x86R` \x86\x01\x81\x80\x15a$\xD8W`\x01\x81\x14a%\x0CWa%8V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x85\x16\x82R\x83\x15\x15`\x05\x1B\x82\x01\x95Pa%8V[_\x87\x81R` \x90 _[\x85\x81\x10\x15a%2W\x81T\x84\x82\x01R`\x01\x90\x91\x01\x90` \x01a%\x16V[\x83\x01\x96PP[PPPPP\x92\x91PPV[``\x82R_a%U``\x84\x01\x83a$nV[`\x01\x83\x01T` \x85\x01R\x83\x81\x03`@\x85\x01Ra\x02\xF1\x81`\x02\x85\x01a$nV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x81R\x83` \x82\x01R`\x80`@\x82\x01R\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83Q\x16`\x80\x82\x01R_` \x84\x01Q`\x80`\xA0\x84\x01Ra%\xDEa\x01\0\x84\x01\x82a \x92V[\x90P`@\x85\x01Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x84\x83\x03\x01`\xC0\x85\x01Ra&\x19\x82\x82a \x92V[\x91PP\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x86\x01Q\x16`\xE0\x84\x01R\x82\x81\x03``\x84\x01Ra&Z\x81\x85a%CV[\x97\x96PPPPPPPV[_` \x82\x84\x03\x12\x15a&uW__\xFD[PQ\x91\x90PV[\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81T`\xE0\x1B\x16\x82R`\x80` \x83\x01R_a&\xBD`\x80\x84\x01`\x01\x84\x01a$nV[\x83\x81\x03`@\x85\x01Ra&\xD2\x81`\x02\x85\x01a$nV[\x90P\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x03\x84\x01T`\xE0\x1B\x16``\x85\x01R\x80\x91PP\x92\x91PPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x81R\x83` \x82\x01R`\x80`@\x82\x01R_a'A`\x80\x83\x01\x85a&|V[\x82\x81\x03``\x84\x01Ra&Z\x81\x85a%CV[\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x81R_a\x02\xF1`\x04\x83\x01\x84a#\xF2V[_a\x1D[\x82\x84a#\xF2V[_` \x82\x84\x03\x12\x15a'\xA1W__\xFD[\x81Q\x80\x15\x15\x81\x14a'\xB0W__\xFD[\x93\x92PPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x81R\x83` \x82\x01R`\x80`@\x82\x01R_a'\xEB`\x80\x83\x01\x85a&|V[\x82\x81\x03``\x84\x01R\x83Q``\x82Ra(\x06``\x83\x01\x82a \x92V[\x90P` \x85\x01Q` \x83\x01R`@\x85\x01Q\x82\x82\x03`@\x84\x01Ra()\x82\x82a \x92V[\x99\x98PPPPPPPPPV[\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R_a(g`\x01\x83\x01\x86a#\xF2V[\x7F[\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Ra(\x97`\x01\x82\x01\x86a#\xF2V[\x90P\x7F].\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Ra(\xC9`\x02\x82\x01\x85a#\xF2V[\x96\x95PPPPPPV[_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a)\x03Wa)\x03a#\x85V[P`\x01\x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_\x82a)EWa)Ea)\nV[P\x04\x90V[_\x82a)XWa)Xa)\nV[P\x06\x90V[\x80\x82\x01\x80\x82\x11\x15a\x1D^Wa\x1D^a#\x85V[`@\x81R_a)\x82`@\x83\x01\x85a \x92V[\x82\x81\x03` \x84\x01Ra)\x94\x81\x85a \x92V[\x95\x94PPPPPV[_` \x82\x84\x03\x12\x15a)\xADW__\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a)\xC3W__\xFD[\x82\x01`\x1F\x81\x01\x84\x13a)\xD3W__\xFD[\x80Qa)\xE1a Q\x82a\x1F\xE4V[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a)\xF5W__\xFD[\x81` \x84\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV\xFE\xA2dipfsX\"\x12 \x1C\xA2\x9E?\x17\xA4j\xA0\n>\xAB\r\xB4\x10\x1C@Rf \x8D>hV\x8F\xAD72/\x99\xCFGUdsolcC\0\x08\x1C\x003`\x80`@R4\x80\x15`\x0EW__\xFD[Pa\x0Bl\x80a\0\x1C_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0)W_5`\xE0\x1C\x80cQX\x8B\x84\x14a\0-W[__\xFD[a\0@a\0;6`\x04a\x08\x80V[a\0RV[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[_a\0_\x85\x85\x85\x85a\0hV[\x95\x94PPPPPV[_a\0r\x83a\x01\x0CV[`@\x80\x84\x01Q\x84Q` \x86\x01Q\x92Q\x7F\x05\xD0\x9Ap\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x93\x94Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x89\x16\x93c\x05\xD0\x9Ap\x93a\0\xD8\x93\x92\x91\x87\x91\x90\x8B\x90`\x04\x01a\t\xE5V[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\0\xEEW__\xFD[PZ\xFA\x15\x80\x15a\x01\0W=__>=_\xFD[PPPP\x94\x93PPPPV[_a\x01\x1A\x82` \x01Qa\x02;V[a\x01\x85W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FInvalid input vector provided\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[a\x01\x92\x82`@\x01Qa\x02\xD5V[a\x01\xF8W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FInvalid output vector provided\0\0`D\x82\x01R`d\x01a\x01|V[a\x025\x82_\x01Q\x83` \x01Q\x84`@\x01Q\x85``\x01Q`@Q` \x01a\x02!\x94\x93\x92\x91\x90a\nAV[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra\x03bV[\x92\x91PPV[___a\x02G\x84a\x03\x84V[\x90\x92P\x90P\x80\x15\x80a\x02YWP_\x19\x82\x14[\x15a\x02gWP_\x93\x92PPPV[_a\x02s\x83`\x01a\n\xDDV[\x90P_[\x82\x81\x10\x15a\x02\xC8W\x85Q\x82\x10a\x02\x92WP_\x95\x94PPPPPV[_a\x02\x9D\x87\x84a\x03\x99V[\x90P_\x19\x81\x03a\x02\xB3WP_\x96\x95PPPPPPV[a\x02\xBD\x81\x84a\n\xDDV[\x92PP`\x01\x01a\x02wV[P\x93Q\x90\x93\x14\x93\x92PPPV[___a\x02\xE1\x84a\x03\x84V[\x90\x92P\x90P\x80\x15\x80a\x02\xF3WP_\x19\x82\x14[\x15a\x03\x01WP_\x93\x92PPPV[_a\x03\r\x83`\x01a\n\xDDV[\x90P_[\x82\x81\x10\x15a\x02\xC8W\x85Q\x82\x10a\x03,WP_\x95\x94PPPPPV[_a\x037\x87\x84a\x03\xDFV[\x90P_\x19\x81\x03a\x03MWP_\x96\x95PPPPPPV[a\x03W\x81\x84a\n\xDDV[\x92PP`\x01\x01a\x03\x11V[_` _\x83Q` \x85\x01`\x02Z\xFAP` _` _`\x02Z\xFAPP_Q\x91\x90PV[__a\x03\x90\x83_a\x04?V[\x91P\x91P\x91P\x91V[___a\x03\xA6\x85\x85a\x05\xDCV[\x90\x92P\x90P`\x01\x82\x01a\x03\xBEW_\x19\x92PPPa\x025V[\x80a\x03\xCA\x83`%a\n\xDDV[a\x03\xD4\x91\x90a\n\xDDV[a\0_\x90`\x04a\n\xDDV[_a\x03\xEB\x82`\ta\n\xDDV[\x83Q\x10\x15a\x03\xFBWP_\x19a\x025V[_\x80a\x04\x11\x85a\x04\x0C\x86`\x08a\n\xDDV[a\x04?V[\x90\x92P\x90P`\x01\x82\x01a\x04)W_\x19\x92PPPa\x025V[\x80a\x045\x83`\ta\n\xDDV[a\0_\x91\x90a\n\xDDV[___a\x04L\x85\x85a\x06\x1AV[\x90P\x80`\xFF\x16_\x03a\x04\x7FW_\x85\x85\x81Q\x81\x10a\x04kWa\x04ka\n\xF0V[\x01` \x01Q\x90\x93P`\xF8\x1C\x91Pa\x05\xD5\x90PV[\x83a\x04\x8B\x82`\x01a\x0B\x1DV[`\xFF\x16a\x04\x98\x91\x90a\n\xDDV[\x85Q\x10\x15a\x04\xADW_\x19_\x92P\x92PPa\x05\xD5V[_\x81`\xFF\x16`\x02\x03a\x04\xF0Wa\x04\xE5a\x04\xD1a\x04\xCA\x87`\x01a\n\xDDV[\x88\x90a\x06\x9EV[b\xFF\xFF\0`\xE8\x82\x90\x1C\x16`\xF8\x91\x90\x91\x1C\x17\x90V[a\xFF\xFF\x16\x90Pa\x05\xCBV[\x81`\xFF\x16`\x04\x03a\x05?Wa\x052a\x05\x0Ca\x04\xCA\x87`\x01a\n\xDDV[`\xD8\x81\x90\x1Cc\xFF\0\xFF\0\x16b\xFF\0\xFF`\xE8\x92\x90\x92\x1C\x91\x90\x91\x16\x17`\x10\x81\x81\x1B\x91\x90\x1C\x17\x90V[c\xFF\xFF\xFF\xFF\x16\x90Pa\x05\xCBV[\x81`\xFF\x16`\x08\x03a\x05\xCBWa\x05\xBEa\x05[a\x04\xCA\x87`\x01a\n\xDDV[`\xC0\x1Cd\xFF\0\0\0\xFF`\x08\x82\x81\x1C\x91\x82\x16e\xFF\0\0\0\xFF\0\x93\x90\x91\x1B\x92\x83\x16\x17`\x10\x90\x81\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16f\xFF\0\xFF\0\xFF\0\xFF\x92\x90\x92\x16g\xFF\0\xFF\0\xFF\0\xFF\0\x90\x93\x16\x92\x90\x92\x17\x90\x91\x1Ce\xFF\xFF\0\0\xFF\xFF\x16\x17` \x81\x81\x1C\x91\x90\x1B\x17\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P[`\xFF\x90\x91\x16\x92P\x90P[\x92P\x92\x90PV[_\x80a\x05\xE9\x83`%a\n\xDDV[\x84Q\x10\x15a\x05\xFCWP_\x19\x90P_a\x05\xD5V[_\x80a\x06\r\x86a\x04\x0C\x87`$a\n\xDDV[\x90\x97\x90\x96P\x94PPPPPV[_\x82\x82\x81Q\x81\x10a\x06-Wa\x06-a\n\xF0V[\x01` \x01Q`\xF8\x1C`\xFF\x03a\x06DWP`\x08a\x025V[\x82\x82\x81Q\x81\x10a\x06VWa\x06Va\n\xF0V[\x01` \x01Q`\xF8\x1C`\xFE\x03a\x06mWP`\x04a\x025V[\x82\x82\x81Q\x81\x10a\x06\x7FWa\x06\x7Fa\n\xF0V[\x01` \x01Q`\xF8\x1C`\xFD\x03a\x06\x96WP`\x02a\x025V[P_\x92\x91PPV[_a\x06\xAC\x83\x83\x01` \x01Q\x90V[\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[`@Q`\x80\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x07\x03Wa\x07\x03a\x06\xB3V[`@R\x90V[\x805\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x81\x14a\x078W__\xFD[\x91\x90PV[_\x82`\x1F\x83\x01\x12a\x07LW__\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07fWa\x07fa\x06\xB3V[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x90\x81\x16`?\x01\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x07\xB3Wa\x07\xB3a\x06\xB3V[`@R\x81\x81R\x83\x82\x01` \x01\x85\x10\x15a\x07\xCAW__\xFD[\x81` \x85\x01` \x83\x017_\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[_``\x82\x84\x03\x12\x15a\x07\xF6W__\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x08\x19Wa\x08\x19a\x06\xB3V[`@R\x90P\x80\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x085W__\xFD[a\x08A\x85\x82\x86\x01a\x07=V[\x82RP` \x83\x81\x015\x90\x82\x01R`@\x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x08gW__\xFD[a\x08s\x85\x82\x86\x01a\x07=V[`@\x83\x01RPP\x92\x91PPV[____`\x80\x85\x87\x03\x12\x15a\x08\x93W__\xFD[\x845s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x08\xB6W__\xFD[\x93P` \x85\x015\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x08\xD8W__\xFD[\x85\x01`\x80\x81\x88\x03\x12\x15a\x08\xE9W__\xFD[a\x08\xF1a\x06\xE0V[a\x08\xFA\x82a\x07\tV[\x81R` \x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\t\x15W__\xFD[a\t!\x89\x82\x85\x01a\x07=V[` \x83\x01RP`@\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\t@W__\xFD[a\tL\x89\x82\x85\x01a\x07=V[`@\x83\x01RPa\t^``\x83\x01a\x07\tV[``\x82\x01R\x80\x93PPP``\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\t\x81W__\xFD[a\t\x8D\x87\x82\x88\x01a\x07\xE6V[\x91PP\x92\x95\x91\x94P\x92PV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[`\xA0\x81R_a\t\xF7`\xA0\x83\x01\x88a\t\x99V[\x82\x81\x03` \x84\x01Ra\n\t\x81\x88a\t\x99V[`@\x84\x01\x96\x90\x96RPP``\x81\x01\x92\x90\x92R`\xFF\x16`\x80\x90\x91\x01R\x92\x91PPV[_\x81Q\x80` \x84\x01\x85^_\x93\x01\x92\x83RP\x90\x91\x90PV[\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85\x16\x81R_a\n}a\nw`\x04\x84\x01\x87a\n*V[\x85a\n*V[\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x93\x90\x93\x16\x83RPP`\x04\x01\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\x025Wa\x025a\n\xB0V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[`\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x025Wa\x025a\n\xB0V\xFE\xA2dipfsX\"\x12 U7tF\xE3\x8C\xB0\xB3\xF5u[/\xB2w\x99\x01\x08\x1FH\x81\x96\x07\xB1\xB4\xA0ES!\x0C\xC6\xDF\x9EdsolcC\0\x08\x1C\x003`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`@Qa(\xD88\x03\x80a(\xD8\x839\x81\x01`@\x81\x90Ra\0.\x91a\x03+V[\x82\x82\x82\x82\x82\x82a\0?\x83Q`P\x14\x90V[a\0\x84W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01RpBad genesis block`x\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[_a\0\x8E\x84a\x01fV[\x90Pb\xFF\xFF\xFF\x82\x16\x15a\x01\tW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FPeriod start hash does not have `D\x82\x01R\x7Fwork. Hint: wrong byte order?\0\0\0`d\x82\x01R`\x84\x01a\0{V[_\x81\x81U`\x01\x82\x90U`\x02\x82\x90U\x81\x81R`\x04` R`@\x90 \x83\x90Ua\x012a\x07\xE0\x84a\x03\xFEV[a\x01<\x90\x84a\x04%V[_\x83\x81R`\x04` R`@\x90 Ua\x01S\x84a\x02&V[`\x05UPa\x05\xBD\x98PPPPPPPPPV[_`\x02\x80\x83`@Qa\x01x\x91\x90a\x048V[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\x01\x93W=__>=_\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xB6\x91\x90a\x04NV[`@Q` \x01a\x01\xC8\x91\x81R` \x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x01\xE2\x91a\x048V[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\x01\xFDW=__>=_\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02 \x91\x90a\x04NV[\x92\x91PPV[_a\x02 a\x023\x83a\x028V[a\x02CV[_a\x02 \x82\x82a\x02SV[_a\x02 a\xFF\xFF`\xD0\x1B\x83a\x02\xF7V[_\x80a\x02ja\x02c\x84`Ha\x04eV[\x85\x90a\x03\tV[`\xE8\x1C\x90P_\x84a\x02|\x85`Ka\x04eV[\x81Q\x81\x10a\x02\x8CWa\x02\x8Ca\x04xV[\x01` \x01Q`\xF8\x1C\x90P_a\x02\xBE\x83_`\x10\x82b\xFF\xFF\xFF\x16\x90\x1C\x82a\xFF\0\x16`\x10\x84b\xFF\xFF\xFF\x16\x90\x1B\x17\x17\x90P\x91\x90PV[b\xFF\xFF\xFF\x16\x90P_a\x02\xD1`\x03\x84a\x04\x8CV[`\xFF\x16\x90Pa\x02\xE2\x81a\x01\0a\x05\x88V[a\x02\xEC\x90\x83a\x05\x93V[\x97\x96PPPPPPPV[_a\x03\x02\x82\x84a\x05\xAAV[\x93\x92PPPV[_a\x03\x02\x83\x83\x01` \x01Q\x90V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[___``\x84\x86\x03\x12\x15a\x03=W__\xFD[\x83Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x03RW__\xFD[\x84\x01`\x1F\x81\x01\x86\x13a\x03bW__\xFD[\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x03{Wa\x03{a\x03\x17V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x03\xA9Wa\x03\xA9a\x03\x17V[`@R\x81\x81R\x82\x82\x01` \x01\x88\x10\x15a\x03\xC0W__\xFD[\x81` \x84\x01` \x83\x01^_` \x92\x82\x01\x83\x01R\x90\x86\x01Q`@\x90\x96\x01Q\x90\x97\x95\x96P\x94\x93PPPPV[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[_\x82a\x04\x0CWa\x04\x0Ca\x03\xEAV[P\x06\x90V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x02 Wa\x02 a\x04\x11V[_\x82Q\x80` \x85\x01\x84^_\x92\x01\x91\x82RP\x91\x90PV[_` \x82\x84\x03\x12\x15a\x04^W__\xFD[PQ\x91\x90PV[\x80\x82\x01\x80\x82\x11\x15a\x02 Wa\x02 a\x04\x11V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[`\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x02 Wa\x02 a\x04\x11V[`\x01\x81[`\x01\x84\x11\x15a\x04\xE0W\x80\x85\x04\x81\x11\x15a\x04\xC4Wa\x04\xC4a\x04\x11V[`\x01\x84\x16\x15a\x04\xD2W\x90\x81\x02\x90[`\x01\x93\x90\x93\x1C\x92\x80\x02a\x04\xA9V[\x93P\x93\x91PPV[_\x82a\x04\xF6WP`\x01a\x02 V[\x81a\x05\x02WP_a\x02 V[\x81`\x01\x81\x14a\x05\x18W`\x02\x81\x14a\x05\"Wa\x05>V[`\x01\x91PPa\x02 V[`\xFF\x84\x11\x15a\x053Wa\x053a\x04\x11V[PP`\x01\x82\x1Ba\x02 V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a\x05aWP\x81\x81\na\x02 V[a\x05m_\x19\x84\x84a\x04\xA5V[\x80_\x19\x04\x82\x11\x15a\x05\x80Wa\x05\x80a\x04\x11V[\x02\x93\x92PPPV[_a\x03\x02\x83\x83a\x04\xE8V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x02 Wa\x02 a\x04\x11V[_\x82a\x05\xB8Wa\x05\xB8a\x03\xEAV[P\x04\x90V[a#\x0E\x80a\x05\xCA_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0\xFBW_5`\xE0\x1C\x80cp\xD5<\x18\x11a\0\x93W\x80c\xB9\x85b\x1A\x11a\0cW\x80c\xB9\x85b\x1A\x14a\x01\xEDW\x80c\xC5\x82B\xCD\x14a\x02\0W\x80c\xE3\xD8\xD8\xD8\x14a\x02\x08W\x80c\xF5\x8D\xB0o\x14a\x02\x0FW__\xFD[\x80cp\xD5<\x18\x14a\x01\x97W\x80ct\xC3\xA3\xA9\x14a\x01\xB4W\x80c\x7F\xA67\xFC\x14a\x01\xC7W\x80c\xB2[\x9B\0\x14a\x01\xDAW__\xFD[\x80c.O\x16\x1A\x11a\0\xCEW\x80c.O\x16\x1A\x14a\x01;W\x80c0\x01{;\x14a\x01^W\x80c`\xB5\xC3\x90\x14a\x01qW\x80ce\xDAA\xB9\x14a\x01\x84W__\xFD[\x80c\x05\xD0\x9Ap\x14a\0\xFFW\x80c\x117d\xBE\x14a\x01\x14W\x80c\x19\x10\xD9s\x14a\x01+W\x80c+\x97\xBE$\x14a\x013W[__\xFD[a\x01\x12a\x01\r6`\x04a\x1D+V[a\x02{V[\0[`\x05T[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[`\x01Ta\x01\x18V[`\x06Ta\x01\x18V[a\x01Na\x01I6`\x04a\x1D\xC5V[a\x05dV[`@Q\x90\x15\x15\x81R` \x01a\x01\"V[a\x01\x18a\x01l6`\x04a\x1D\xF4V[a\x05|V[a\x01\x18a\x01\x7F6`\x04a\x1E\x14V[a\x05\x90V[a\x01Na\x01\x926`\x04a\x1E+V[a\x05\x9AV[a\x01\x9F`\x04\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01\"V[a\x01Na\x01\xC26`\x04a\x1E\x97V[a\x07FV[a\x01Na\x01\xD56`\x04a\x1F\x18V[a\x08\xBBV[a\x01\x18a\x01\xE86`\x04a\x1F\xB7V[a\n\x9AV[a\x01Na\x01\xFB6`\x04a 0V[a\x0B\x17V[`\x02Ta\x01\x18V[_Ta\x01\x18V[a\x01\x12a\x02\x1D6`\x04a mV[`\x07\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\x16\x92\x15\x15\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\x16\x92\x90\x92\x17a\x01\0\x91\x15\x15\x91\x90\x91\x02\x17\x90UV[a\x02\xB9\x87\x87\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPa\x0B-\x92PPPV[a\x03\nW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01R\x7FBad header block\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[a\x03H\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPa\x0B4\x92PPPV[a\x03\x94W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01R\x7FBad merkle array proof\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03\x01V[a\x04\x13\x83a\x03\xD6\x89\x89\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPa\x0BJ\x92PPPV[\x87\x87\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RP\x88\x92Pa\x0BV\x91PPV[a\x04_W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01R\x7FBad inclusion proof\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03\x01V[_a\x04\x9E\x88\x88\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPa\x0B\x88\x92PPPV[\x90P_a\x04\xAA`\x02T\x90V[\x90Pa\x04\xB9\x82\x82a\x08\0a\x0C`V[a\x05\x05W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FGCD does not confirm header\0\0\0\0\0`D\x82\x01R`d\x01a\x03\x01V[`\xFF\x83\x16`\x08\x10\x15a\x05YW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FInsufficient confirmations\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03\x01V[PPPPPPPPPV[_a\x05q\x85\x85\x85\x85a\x0C\x90V[\x90P[\x94\x93PPPPV[_a\x05\x87\x83\x83a\r*V[\x90P[\x92\x91PPV[_a\x05\x8A\x82a\r\x9CV[_a\x05\xD9\x83\x83\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPa\x0EJ\x92PPPV[a\x06KW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FHeader array length must be divi`D\x82\x01R\x7Fsible by 80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x03\x01V[a\x06\x89\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPa\x0B-\x92PPPV[a\x06\xD5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FAnchor must be 80 bytes\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03\x01V[a\x05q\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPP`@\x80Q` `\x1F\x89\x01\x81\x90\x04\x81\x02\x82\x01\x81\x01\x90\x92R\x87\x81R\x92P\x87\x91P\x86\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847_\x92\x01\x82\x90RP\x92Pa\x0EY\x91PPV[_a\x07\x85\x84\x84\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPa\x0B-\x92PPPV[\x80\x15a\x07\xCAWPa\x07\xCA\x86\x86\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPa\x0B-\x92PPPV[a\x08<W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FBad args. Check header and array`D\x82\x01R\x7F byte lengths.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x03\x01V[a\x08\xB0\x87\x87\x87\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPP`@\x80Q` `\x1F\x8B\x01\x81\x90\x04\x81\x02\x82\x01\x81\x01\x90\x92R\x89\x81R\x92P\x89\x91P\x88\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847_\x92\x01\x91\x90\x91RP\x88\x92Pa\x12F\x91PPV[\x97\x96PPPPPPPV[_a\x08\xFA\x87\x87\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPa\x0B-\x92PPPV[\x80\x15a\t?WPa\t?\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPa\x0B-\x92PPPV[\x80\x15a\t\x84WPa\t\x84\x83\x83\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPa\x0EJ\x92PPPV[a\t\xF6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FBad args. Check header and array`D\x82\x01R\x7F byte lengths.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x03\x01V[a\x08\xB0\x87\x87\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPP`@\x80Q` `\x1F\x8B\x01\x81\x90\x04\x81\x02\x82\x01\x81\x01\x90\x92R\x89\x81R\x92P\x89\x91P\x88\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847_\x92\x01\x91\x90\x91RPP`@\x80Q` `\x1F\x8A\x01\x81\x90\x04\x81\x02\x82\x01\x81\x01\x90\x92R\x88\x81R\x92P\x88\x91P\x87\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847_\x92\x01\x91\x90\x91RPa\x14\xE3\x92PPPV[_a\x0B\r\x86\x86\x86\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPP`@\x80Q` `\x1F\x8A\x01\x81\x90\x04\x81\x02\x82\x01\x81\x01\x90\x92R\x88\x81R\x92P\x88\x91P\x87\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847_\x92\x01\x91\x90\x91RPa\x17u\x92PPPV[\x96\x95PPPPPPV[_a\x0B#\x84\x84\x84a\x0C`V[\x90P[\x93\x92PPPV[Q`P\x14\x90V[_` \x82Qa\x0BC\x91\x90a \xCBV[\x15\x92\x91PPV[`D\x81\x01Q_\x90a\x05\x8AV[_\x83\x85\x14\x80\x15a\x0BdWP\x81\x15[\x80\x15a\x0BoWP\x82Q\x15[\x15a\x0B|WP`\x01a\x05tV[a\x05q\x85\x84\x86\x85a\x19\x06V[_`\x02\x80\x83`@Qa\x0B\x9A\x91\x90a \xDEV[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\x0B\xB5W=__>=_\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xD8\x91\x90a \xF4V[`@Q` \x01a\x0B\xEA\x91\x81R` \x01\x90V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x0C\"\x91a \xDEV[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\x0C=W=__>=_\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x8A\x91\x90a \xF4V[`\x07T_\x90`\xFF\x16\x15a\x0C~WP`\x07Ta\x01\0\x90\x04`\xFF\x16a\x0B&V[a\x0C\x89\x84\x84\x84a\x19\xABV[\x90Pa\x0B&V[_\x83\x85\x14\x80\x15a\x0C\x9FWP\x82\x85\x14[\x15a\x0C\xACWP`\x01a\x05tV[\x83\x83\x81\x81_[\x86\x81\x10\x15a\x0C\xF4W\x89\x83\x14a\x0C\xD3W_\x83\x81R`\x03` R`@\x90 T\x92\x94P[\x89\x82\x14a\x0C\xECW_\x82\x81R`\x03` R`@\x90 T\x91\x93P[`\x01\x01a\x0C\xB2V[P\x82\x84\x03a\r\x08W_\x94PPPPPa\x05tV[\x80\x82\x14a\r\x1BW_\x94PPPPPa\x05tV[P`\x01\x98\x97PPPPPPPPV[_\x82\x81[\x83\x81\x10\x15a\rNW_\x91\x82R`\x03` R`@\x90\x91 T\x90`\x01\x01a\r.V[P\x80a\x05\x87W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01R\x7FUnknown ancestor\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03\x01V[_\x80\x82\x81[a\r\xAD`\x04`\x01a!8V[c\xFF\xFF\xFF\xFF\x16\x81\x10\x15a\x0E\x01W_\x82\x81R`\x04` R`@\x81 T\x93P\x83\x90\x03a\r\xE6W_\x91\x82R`\x03` R`@\x90\x91 T\x90a\r\xF9V[a\r\xF0\x81\x84a!TV[\x95\x94PPPPPV[`\x01\x01a\r\xA1V[P`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01R\x7FUnknown block\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03\x01V[_`P\x82Qa\x0BC\x91\x90a \xCBV[__a\x0Ed\x85a\x0B\x88V[\x90P_a\x0Ep\x82a\r\x9CV[\x90P_a\x0E|\x86a\x19\xECV[\x90P\x84\x80a\x0E\x91WP\x80a\x0E\x8F\x88a\x19\xECV[\x14[a\x0F\x02W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FUnexpected retarget on external `D\x82\x01R\x7Fcall\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x03\x01V[\x85Q_\x90\x81\x90\x81[\x81\x81\x10\x15a\x12\x03Wa\x0F\x1D`P\x82a!gV[a\x0F(\x90`\x01a!TV[a\x0F2\x90\x87a!TV[\x93Pa\x0F@\x8A\x82`Pa\x19\xF7V[_\x81\x81R`\x03` R`@\x90 T\x90\x93Pa\x11\x16W\x84a\x10\x96\x84_\x81\x90P`\x08\x81~\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\x16\x90\x1B`\x08\x82\x90\x1C~\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\x16\x17\x90P`\x10\x81}\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\x16\x90\x1B`\x10\x82\x90\x1C}\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\x16\x17\x90P` \x81{\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\x16\x90\x1B` \x82\x90\x1C{\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\x16\x17\x90P`@\x81w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x1B`@\x82\x90\x1Cw\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x17\x90P`\x80\x81\x90\x1B`\x80\x82\x90\x1C\x17\x90P\x91\x90PV[\x11\x15a\x10\xE4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FHeader work is insufficient\0\0\0\0\0`D\x82\x01R`d\x01a\x03\x01V[_\x83\x81R`\x03` R`@\x90 \x87\x90Ua\x10\xFF`\x04\x85a \xCBV[_\x03a\x11\x16W_\x83\x81R`\x04` R`@\x90 \x84\x90U[\x84a\x11!\x8B\x83a\x1A\x1CV[\x14a\x11nW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FTarget changed unexpectedly\0\0\0\0\0`D\x82\x01R`d\x01a\x03\x01V[\x86a\x11y\x8B\x83a\x1A\xB5V[\x14a\x11\xECW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FHeaders do not form a consistent`D\x82\x01R\x7F chain\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x03\x01V[\x82\x96P`P\x81a\x11\xFC\x91\x90a!TV[\x90Pa\x0F\nV[P\x81a\x12\x0E\x8Ba\x0B\x88V[`@Q\x7F\xF9\x0EO\x1D\x9C\xD0\xDDU\xE39A\x1C\xBC\x9B\x15$\x820|:#\xEDdq^J(X\xF6A\xA3\xF5\x90_\x90\xA3P`\x01\x99\x98PPPPPPPPPV[_a\x07\xE0\x82\x11\x15a\x12\xBFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`3`$\x82\x01R\x7FRequested limit is greater than `D\x82\x01R\x7F1 difficulty period\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x03\x01V[_a\x12\xC9\x84a\x0B\x88V[\x90P_a\x12\xD5\x86a\x0B\x88V[\x90P`\x01T\x81\x14a\x13(W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FPassed in best is not best known`D\x82\x01R`d\x01a\x03\x01V[_\x82\x81R`\x03` R`@\x90 Ta\x13\x82W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01R\x7FNew best is unknown\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03\x01V[a\x13\x90\x87`\x01T\x84\x87a\x0C\x90V[a\x14\x02W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`)`$\x82\x01R\x7FAncestor must be heaviest common`D\x82\x01R\x7F ancestor\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x03\x01V[\x81a\x14\x0E\x88\x88\x88a\x17uV[\x14a\x14\x81W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`3`$\x82\x01R\x7FNew best hash does not have more`D\x82\x01R\x7F work than previous\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x03\x01V[`\x01\x82\x90U`\x02\x87\x90U_a\x14\x95\x86a\x1A\xCDV[\x90P`\x05T\x81\x14a\x14\xA6W`\x05\x81\x90U[\x87\x83\x83\x7F<\xC1=\xE6M\xF0\xF0#\x96&#\\Q\xA2\xDA%\x1B\xBC\x8C\x85fN\xCC\xE3\x92c\xDA>\xE0?`l`@Q`@Q\x80\x91\x03\x90\xA4P`\x01\x97\x96PPPPPPPV[__a\x14\xF6a\x14\xF1\x86a\x0B\x88V[a\r\x9CV[\x90P_a\x15\x05a\x14\xF1\x86a\x0B\x88V[\x90Pa\x15\x13a\x07\xE0\x82a \xCBV[a\x07\xDF\x14a\x15\x89W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FMust provide the last header of `D\x82\x01R\x7Fthe closing difficulty period\0\0\0`d\x82\x01R`\x84\x01a\x03\x01V[a\x15\x95\x82a\x07\xDFa!TV[\x81\x14a\x16\tW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`(`$\x82\x01R\x7FMust provide exactly 1 difficult`D\x82\x01R\x7Fy period\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x03\x01V[a\x16\x12\x85a\x1A\xCDV[a\x16\x1B\x87a\x1A\xCDV[\x14a\x16\x8EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FPeriod header difficulties do no`D\x82\x01R\x7Ft match\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x03\x01V[_a\x16\x98\x85a\x19\xECV[\x90P_a\x16\xCAa\x16\xA7\x89a\x19\xECV[a\x16\xB0\x8Aa\x1A\xDFV[c\xFF\xFF\xFF\xFF\x16a\x16\xBF\x8Aa\x1A\xDFV[c\xFF\xFF\xFF\xFF\x16a\x1B\x12V[\x90P\x81\x81\x83\x16\x14a\x17\x1DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FInvalid retarget provided\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03\x01V[_a\x17'\x89a\x1A\xCDV[\x90P\x80`\x06T\x14\x15\x80\x15a\x17QWPa\x07\xE0a\x17D`\x01Ta\r\x9CV[a\x17N\x91\x90a!zV[\x84\x11[\x15a\x17\\W`\x06\x81\x90U[a\x17h\x88\x88`\x01a\x0EYV[\x99\x98PPPPPPPPPV[__a\x17\x80\x85a\r\x9CV[\x90P_a\x17\x8Fa\x14\xF1\x86a\x0B\x88V[\x90P_a\x17\x9Ea\x14\xF1\x86a\x0B\x88V[\x90P\x82\x82\x10\x15\x80\x15a\x17\xB0WP\x82\x81\x10\x15[a\x18\"W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`0`$\x82\x01R\x7FA descendant height is below the`D\x82\x01R\x7F ancestor height\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x03\x01V[_a\x18/a\x07\xE0\x85a \xCBV[a\x18;\x85a\x07\xE0a!TV[a\x18E\x91\x90a!zV[\x90P\x80\x83\x10\x81\x83\x10\x81\x15\x82a\x18WWP\x80[\x15a\x18rWa\x18e\x89a\x0B\x88V[\x96PPPPPPPa\x0B&V[\x81\x80\x15a\x18}WP\x80\x15[\x15a\x18\x8BWa\x18e\x88a\x0B\x88V[\x81\x80\x15a\x18\x95WP\x80[\x15a\x18\xB9W\x83\x85\x10\x15a\x18\xB0Wa\x18\xAB\x88a\x0B\x88V[a\x18eV[a\x18e\x89a\x0B\x88V[a\x18\xC2\x88a\x1A\xCDV[a\x18\xCEa\x07\xE0\x86a \xCBV[a\x18\xD8\x91\x90a!\x8DV[a\x18\xE1\x8Aa\x1A\xCDV[a\x18\xEDa\x07\xE0\x88a \xCBV[a\x18\xF7\x91\x90a!\x8DV[\x10\x15a\x18\xB0Wa\x18e\x88a\x0B\x88V[_` \x84Qa\x19\x15\x91\x90a \xCBV[\x15a\x19!WP_a\x05tV[\x83Q_\x03a\x190WP_a\x05tV[\x81\x85_[\x86Q\x81\x10\x15a\x19\x9EWa\x19H`\x02\x84a \xCBV[`\x01\x03a\x19lWa\x19ea\x19_\x88\x83\x01` \x01Q\x90V[\x83a\x1B\x9AV[\x91Pa\x19\x85V[a\x19\x82\x82a\x19}\x89\x84\x01` \x01Q\x90V[a\x1B\x9AV[\x91P[`\x01\x92\x90\x92\x1C\x91a\x19\x97` \x82a!TV[\x90Pa\x194V[P\x90\x93\x14\x95\x94PPPPPV[_\x82\x81[\x83\x81\x10\x15a\x19\xE1W\x85\x82\x03a\x19\xC9W`\x01\x92PPPa\x0B&V[_\x91\x82R`\x03` R`@\x90\x91 T\x90`\x01\x01a\x19\xAFV[P_\x95\x94PPPPPV[_a\x05\x8A\x82_a\x1A\x1CV[_` _\x83\x85` \x01\x87\x01`\x02Z\xFAP` _` _`\x02Z\xFAPP_Q\x93\x92PPPV[_\x80a\x1A3a\x1A,\x84`Ha!TV[\x85\x90a\x1B\xA5V[`\xE8\x1C\x90P_\x84a\x1AE\x85`Ka!TV[\x81Q\x81\x10a\x1AUWa\x1AUa!\xA4V[\x01` \x01Q`\xF8\x1C\x90P_a\x1A\x87\x83_`\x10\x82b\xFF\xFF\xFF\x16\x90\x1C\x82a\xFF\0\x16`\x10\x84b\xFF\xFF\xFF\x16\x90\x1B\x17\x17\x90P\x91\x90PV[b\xFF\xFF\xFF\x16\x90P_a\x1A\x9A`\x03\x84a!\xD1V[`\xFF\x16\x90Pa\x1A\xAB\x81a\x01\0a\"\xCDV[a\x08\xB0\x90\x83a!\x8DV[_a\x05\x87a\x1A\xC4\x83`\x04a!TV[\x84\x01` \x01Q\x90V[_a\x05\x8Aa\x1A\xDA\x83a\x19\xECV[a\x1B\xB3V[_a\x05\x8Aa\x1A\xEC\x83a\x1B\xDAV[`\xD8\x81\x90\x1Cc\xFF\0\xFF\0\x16b\xFF\0\xFF`\xE8\x92\x90\x92\x1C\x91\x90\x91\x16\x17`\x10\x81\x81\x1B\x91\x90\x1C\x17\x90V[_\x80a\x1B\x1E\x83\x85a\x1B\xE6V[\x90Pa\x1B.b\x12u\0`\x04a\x1CAV[\x81\x10\x15a\x1BFWa\x1BCb\x12u\0`\x04a\x1CAV[\x90P[a\x1BTb\x12u\0`\x04a\x1CLV[\x81\x11\x15a\x1BlWa\x1Bib\x12u\0`\x04a\x1CLV[\x90P[_a\x1B\x84\x82a\x1B~\x88b\x01\0\0a\x1CAV[\x90a\x1CLV[\x90Pa\x0B\rb\x01\0\0a\x1B~\x83b\x12u\0a\x1CAV[_a\x05\x87\x83\x83a\x1C\xBFV[_a\x05\x87\x83\x83\x01` \x01Q\x90V[_a\x05\x8A{\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83a\x1CAV[_a\x05\x8A\x82`Da\x1B\xA5V[_\x82\x82\x11\x15a\x1C7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FUnderflow during subtraction.\0\0\0`D\x82\x01R`d\x01a\x03\x01V[a\x05\x87\x82\x84a!zV[_a\x05\x87\x82\x84a!gV[_\x82_\x03a\x1C[WP_a\x05\x8AV[a\x1Ce\x82\x84a!\x8DV[\x90P\x81a\x1Cr\x84\x83a!gV[\x14a\x05\x8AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FOverflow during multiplication.\0`D\x82\x01R`d\x01a\x03\x01V[_\x82_R\x81` R` _`@_`\x02Z\xFAP` _` _`\x02Z\xFAPP_Q\x92\x91PPV[__\x83`\x1F\x84\x01\x12a\x1C\xF6W__\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1D\rW__\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x1D$W__\xFD[\x92P\x92\x90PV[_______`\xA0\x88\x8A\x03\x12\x15a\x1DAW__\xFD[\x875g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1DWW__\xFD[a\x1Dc\x8A\x82\x8B\x01a\x1C\xE6V[\x90\x98P\x96PP` \x88\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1D\x82W__\xFD[a\x1D\x8E\x8A\x82\x8B\x01a\x1C\xE6V[\x90\x96P\x94PP`@\x88\x015\x92P``\x88\x015\x91P`\x80\x88\x015`\xFF\x81\x16\x81\x14a\x1D\xB5W__\xFD[\x80\x91PP\x92\x95\x98\x91\x94\x97P\x92\x95PV[____`\x80\x85\x87\x03\x12\x15a\x1D\xD8W__\xFD[PP\x825\x94` \x84\x015\x94P`@\x84\x015\x93``\x015\x92P\x90PV[__`@\x83\x85\x03\x12\x15a\x1E\x05W__\xFD[PP\x805\x92` \x90\x91\x015\x91PV[_` \x82\x84\x03\x12\x15a\x1E$W__\xFD[P5\x91\x90PV[____`@\x85\x87\x03\x12\x15a\x1E>W__\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1ETW__\xFD[a\x1E`\x87\x82\x88\x01a\x1C\xE6V[\x90\x95P\x93PP` \x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1E\x7FW__\xFD[a\x1E\x8B\x87\x82\x88\x01a\x1C\xE6V[\x95\x98\x94\x97P\x95PPPPV[______`\x80\x87\x89\x03\x12\x15a\x1E\xACW__\xFD[\x865\x95P` \x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1E\xC9W__\xFD[a\x1E\xD5\x89\x82\x8A\x01a\x1C\xE6V[\x90\x96P\x94PP`@\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1E\xF4W__\xFD[a\x1F\0\x89\x82\x8A\x01a\x1C\xE6V[\x97\x9A\x96\x99P\x94\x97\x94\x96\x95``\x90\x95\x015\x94\x93PPPPV[______``\x87\x89\x03\x12\x15a\x1F-W__\xFD[\x865g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1FCW__\xFD[a\x1FO\x89\x82\x8A\x01a\x1C\xE6V[\x90\x97P\x95PP` \x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1FnW__\xFD[a\x1Fz\x89\x82\x8A\x01a\x1C\xE6V[\x90\x95P\x93PP`@\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1F\x99W__\xFD[a\x1F\xA5\x89\x82\x8A\x01a\x1C\xE6V[\x97\x9A\x96\x99P\x94\x97P\x92\x95\x93\x94\x92PPPV[_____``\x86\x88\x03\x12\x15a\x1F\xCBW__\xFD[\x855\x94P` \x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1F\xE8W__\xFD[a\x1F\xF4\x88\x82\x89\x01a\x1C\xE6V[\x90\x95P\x93PP`@\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a \x13W__\xFD[a \x1F\x88\x82\x89\x01a\x1C\xE6V[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[___``\x84\x86\x03\x12\x15a BW__\xFD[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[\x805\x80\x15\x15\x81\x14a hW__\xFD[\x91\x90PV[__`@\x83\x85\x03\x12\x15a ~W__\xFD[a \x87\x83a YV[\x91Pa \x95` \x84\x01a YV[\x90P\x92P\x92\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_\x82a \xD9Wa \xD9a \x9EV[P\x06\x90V[_\x82Q\x80` \x85\x01\x84^_\x92\x01\x91\x82RP\x91\x90PV[_` \x82\x84\x03\x12\x15a!\x04W__\xFD[PQ\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[c\xFF\xFF\xFF\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x05\x8AWa\x05\x8Aa!\x0BV[\x80\x82\x01\x80\x82\x11\x15a\x05\x8AWa\x05\x8Aa!\x0BV[_\x82a!uWa!ua \x9EV[P\x04\x90V[\x81\x81\x03\x81\x81\x11\x15a\x05\x8AWa\x05\x8Aa!\x0BV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x05\x8AWa\x05\x8Aa!\x0BV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[`\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x05\x8AWa\x05\x8Aa!\x0BV[`\x01\x81[`\x01\x84\x11\x15a\"%W\x80\x85\x04\x81\x11\x15a\"\tWa\"\ta!\x0BV[`\x01\x84\x16\x15a\"\x17W\x90\x81\x02\x90[`\x01\x93\x90\x93\x1C\x92\x80\x02a!\xEEV[\x93P\x93\x91PPV[_\x82a\";WP`\x01a\x05\x8AV[\x81a\"GWP_a\x05\x8AV[\x81`\x01\x81\x14a\"]W`\x02\x81\x14a\"gWa\"\x83V[`\x01\x91PPa\x05\x8AV[`\xFF\x84\x11\x15a\"xWa\"xa!\x0BV[PP`\x01\x82\x1Ba\x05\x8AV[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a\"\xA6WP\x81\x81\na\x05\x8AV[a\"\xB2_\x19\x84\x84a!\xEAV[\x80_\x19\x04\x82\x11\x15a\"\xC5Wa\"\xC5a!\x0BV[\x02\x93\x92PPPV[_a\x05\x87\x83\x83a\"-V\xFE\xA2dipfsX\"\x12 Wj,\0\x82no\xB9\xBD\x08Z\x81\x10b\xDC\xFC\xB2M\xF22\xA37H\x11G\x08\xD8&*\xD5\x0F\xADdsolcC\0\x08\x1C\x003\0\0\0 s\xBD!\x84\xED\xD9\xC4\xFCvd.\xA6uN\xE4\x016\x97\x0E\xFC\x10\xC4\x19\0\0\0\0\0\0\0\0\0\x02\x96\xEF\x12>\xA9m\xA5\xCFi_\"\xBF}\x94\xBE\x87\xD4\x9D\xB1\xADz\xC3q\xACC\xC4\xDAAa\xC8\xC2\x164\x9C[\xA1\x19(\x17\r8x+\x02H\x97\x07\0\0\0\0\0\"\0 \xA43>V\x12\xAB\x1A\x10C\xB2WU\xC8\x9B\x16\xD5Q\x84\xA4/\x81y\x9Eb>k\xC3\x9D\xB8S\x9C\x18\0\0\0\0\0\0\0\0\x16j\x14\xED\xB1\xB5\xC2\xF3\x9A\xF0\xFE\xC1Qs%\x85\xB1\x04\x9B\x07\x89R\x11H\xE5\xA1\xA0\xE6\x16\xD8\xFD\x92\xB4\xEF\"\x8CBN\x0C\x81g\x99\xA2V\xC6\xA9\x08\x92\x19\\\xCF\xC53\0\xD6\xE3Z\rm\xE9Kef\x94X\x99d\xA2R\x95~Fs\xA9\xFB\x1D/\x8BJ\x92\xE3\xF0\xA7\xBBeO\xDD\xB9NZ\x1Em\x7F\x7FI\x9F\xD1\xBE]\xD3\ns\xBFU\x84\xBF\x13}\xA5\xFD\xD7|\xC2\x1A\xEB\x95\xB9\xE3W\x88\x89K\xE0\x19(K\xD4\xFB\xEDm\xD6\x11\x8A\xC2\xCBm&\xBCK\xE4\xE4#\xF5Z:H\xF2\x87M\x8D\x02\xA6]\x9C\x87\xD0}\xE2\x1DM\xFE{\n\x9FJ#\xCC\x9AX7>\x9Ei1\xFE\xFD\xB5\xAF\xAD\xE5\xDFT\xC9\x11\x04\x04\x8D\xF1\xEE\x99\x92@ay\x84\xE1\x8Bo\x93\x1E#sg=\x01\x95\xB8\xC6\x98}\x7F\xF7e\r\\\xE5;\xCE\xC4n\x13\xABO-\xA1\x14j\x7F\xC6!\xEEg/b\xBC\"t$\x869-u\xE5^g\xB0\x99`\xC38j\x0BI\xE7_\x17#\xD6\xAB(\xAC\x9A (\xA0\xC7(f\xE2\x11\x1Dy\xD4\x81{\x88\xE1|\x82\x197\x84wh\xD9(7\xBA\xE3\x83+\xB8\xE5\xA4\xABD4\xB9~\0\xA6\xC1\x01\x82\xF2\x11\xF5\x92@\x90h\xD6\xF5e$\0\xD9\xA3\xD1\xCC\x15\n\x7F\xB6\x92\xE8t\xCCB\xD7k\xDA\xFC\x84//\xE0\xF85\xA7\xC2M-`\xC1\t\xB1\x87\xD6Eq\xEF\xBA\xA8\x04{\xE8X!\xF8\xE6~\x0E\x85\xF2\xF5\x89K\xC6=\0\xC2\xED\x9Dd\x01\x17F\xBD\x86t\0\xF3IK\x8FD\xC2K\x83\xE1\xAAX\xC4\xF0\xFF%\xB4\xA6\x1C\xFF\xEF\xFDK\xC0\xF9\xBA0\0\0\0\0\0\xFF\xFF\xFF\xFF",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b5060043610610163575f3560e01c8063916a17c6116100c7578063e20c9f711161007d578063ef3e681211610063578063ef3e681214610286578063fa7626d41461028e578063fad06b8f1461029b575f5ffd5b8063e20c9f7114610276578063e89f84191461027e575f5ffd5b8063b5508aa9116100ad578063b5508aa91461024e578063ba414fa614610256578063c899d2411461026e575f5ffd5b8063916a17c61461023e578063b52b205814610246575f5ffd5b80633f7286f41161011c5780635df0d076116101025780635df0d0761461020a57806366d9a9a01461021457806385226c8114610229575f5ffd5b80633f7286f4146101e257806344badbb6146101ea575f5ffd5b80631ed7831c1161014c5780631ed7831c146101b05780632ade3880146101c55780633e5e3c23146101da575f5ffd5b80630813852a146101675780631c0da81f14610190575b5f5ffd5b61017a61017536600461200b565b6102ae565b60405161018791906120c0565b60405180910390f35b6101a361019e36600461200b565b6102f9565b6040516101879190612123565b6101b861036b565b6040516101879190612135565b6101cd6103d8565b60405161018791906121e7565b6101b8610521565b6101b861058c565b6101fd6101f836600461200b565b6105f7565b604051610187919061226b565b61021261063a565b005b61021c610932565b60405161018791906122a2565b610231610a35565b6040516101879190612373565b61021c610b00565b610212610c03565b610231610cba565b61025e610d85565b6040519015158152602001610187565b610212610ec2565b6101b86110f6565b610212611161565b6102126113c7565b60075461025e9060ff1681565b6101fd6102a936600461200b565b6115ec565b60606102f18484846040518060400160405280600381526020017f686578000000000000000000000000000000000000000000000000000000000081525061162f565b949350505050565b60605f6103078585856102ae565b90505f5b61031585856123b2565b811015610362578282828151811061032f5761032f6123c5565b6020026020010151604051602001610348929190612409565b60408051601f19818403018152919052925060010161030b565b50509392505050565b606060148054806020026020016040519081016040528092919081815260200182805480156103ce57602002820191905f5260205f20905b815473ffffffffffffffffffffffffffffffffffffffff1681526001909101906020018083116103a3575b5050505050905090565b6060601b805480602002602001604051908101604052809291908181526020015f905b82821015610518575f848152602080822060408051808201825260028702909201805473ffffffffffffffffffffffffffffffffffffffff168352600181018054835181870281018701909452808452939591948681019491929084015b82821015610501578382905f5260205f200180546104769061241d565b80601f01602080910402602001604051908101604052809291908181526020018280546104a29061241d565b80156104ed5780601f106104c4576101008083540402835291602001916104ed565b820191905f5260205f20905b8154815290600101906020018083116104d057829003601f168201915b505050505081526020019060010190610459565b5050505081525050815260200190600101906103fb565b50505050905090565b606060168054806020026020016040519081016040528092919081815260200182805480156103ce57602002820191905f5260205f2090815473ffffffffffffffffffffffffffffffffffffffff1681526001909101906020018083116103a3575050505050905090565b606060158054806020026020016040519081016040528092919081815260200182805480156103ce57602002820191905f5260205f2090815473ffffffffffffffffffffffffffffffffffffffff1681526001909101906020018083116103a3575050505050905090565b60606102f18484846040518060400160405280600981526020017f6469676573745f6c650000000000000000000000000000000000000000000000815250611790565b60408051608081019091526025805460e01b7fffffffff00000000000000000000000000000000000000000000000000000000168252602680545f93929160208401916106869061241d565b80601f01602080910402602001604051908101604052809291908181526020018280546106b29061241d565b80156106fd5780601f106106d4576101008083540402835291602001916106fd565b820191905f5260205f20905b8154815290600101906020018083116106e057829003601f168201915b505050505081526020016002820180546107169061241d565b80601f01602080910402602001604051908101604052809291908181526020018280546107429061241d565b801561078d5780601f106107645761010080835404028352916020019161078d565b820191905f5260205f20905b81548152906001019060200180831161077057829003601f168201915b50505091835250506003919091015460e01b7fffffffff0000000000000000000000000000000000000000000000000000000016602091820152604080518082018252600181525f818401528382015280518082018252601e81527f496e76616c6964206f757470757420766563746f722070726f7669646564000092810192909252517ff28dceb3000000000000000000000000000000000000000000000000000000008152919250737109709ecfa91a80626ff3989d68f67f5b1dd12d9163f28dceb39161085f91600401612123565b5f604051808303815f87803b158015610876575f5ffd5b505af1158015610888573d5f5f3e3d5ffd5b5050602254601c546023546040517f51588b8400000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff93841695506351588b8494506108ef93909216918690602990600401612574565b602060405180830381865afa15801561090a573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061092e9190612665565b5050565b60606019805480602002602001604051908101604052809291908181526020015f905b82821015610518575f84815260209081902060408051808201825260028602909201805473ffffffffffffffffffffffffffffffffffffffff168352600181018054835181870281018701909452808452939491938583019392830182828015610a1d57602002820191905f5260205f20905f905b82829054906101000a900460e01b7bffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916815260200190600401906020826003010492830192600103820291508084116109ca5790505b50505050508152505081526020019060010190610955565b60606018805480602002602001604051908101604052809291908181526020015f905b82821015610518578382905f5260205f20018054610a759061241d565b80601f0160208091040260200160405190810160405280929190818152602001828054610aa19061241d565b8015610aec5780601f10610ac357610100808354040283529160200191610aec565b820191905f5260205f20905b815481529060010190602001808311610acf57829003601f168201915b505050505081526020019060010190610a58565b6060601a805480602002602001604051908101604052809291908181526020015f905b82821015610518575f84815260209081902060408051808201825260028602909201805473ffffffffffffffffffffffffffffffffffffffff168352600181018054835181870281018701909452808452939491938583019392830182828015610beb57602002820191905f5260205f20905f905b82829054906101000a900460e01b7bffffffffffffffffffffffffffffffffffffffffffffffffffffffff191681526020019060040190602082600301049283019260010382029150808411610b985790505b50505050508152505081526020019060010190610b23565b602254601c546023546040517f51588b840000000000000000000000000000000000000000000000000000000081525f9373ffffffffffffffffffffffffffffffffffffffff908116936351588b8493610c6a93929091169160259060299060040161270d565b602060405180830381865afa158015610c85573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610ca99190612665565b9050610cb7816024546118de565b50565b60606017805480602002602001604051908101604052809291908181526020015f905b82821015610518578382905f5260205f20018054610cfa9061241d565b80601f0160208091040260200160405190810160405280929190818152602001828054610d269061241d565b8015610d715780601f10610d4857610100808354040283529160200191610d71565b820191905f5260205f20905b815481529060010190602001808311610d5457829003601f168201915b505050505081526020019060010190610cdd565b6007545f90610100900460ff1615610da65750600754610100900460ff1690565b5f737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610ebd5760408051737109709ecfa91a80626ff3989d68f67f5b1dd12d602082018190527f6661696c6564000000000000000000000000000000000000000000000000000082840152825180830384018152606083019093525f929091610e49917f667f9d70ca411d70ead50d8d5c22070dafc36ad75f3dcf5e7237b22ade9aecc491608001612753565b60408051601f1981840301815290829052610e6391612786565b5f604051808303815f865af19150503d805f8114610e9c576040519150601f19603f3d011682016040523d82523d5f602084013e610ea1565b606091505b5091505080806020019051810190610eb99190612791565b9150505b919050565b60408051608081019091526025805460e01b7fffffffff00000000000000000000000000000000000000000000000000000000168252602680545f9392916020840191610f0e9061241d565b80601f0160208091040260200160405190810160405280929190818152602001828054610f3a9061241d565b8015610f855780601f10610f5c57610100808354040283529160200191610f85565b820191905f5260205f20905b815481529060010190602001808311610f6857829003601f168201915b50505050508152602001600282018054610f9e9061241d565b80601f0160208091040260200160405190810160405280929190818152602001828054610fca9061241d565b80156110155780601f10610fec57610100808354040283529160200191611015565b820191905f5260205f20905b815481529060010190602001808311610ff857829003601f168201915b50505091835250506003919091015460e01b7fffffffff00000000000000000000000000000000000000000000000000000000166020918201527c01000000000000000000000000000000000000000000000000000000006060830152604080518082018252601381527f42616420696e636c7573696f6e2070726f6f660000000000000000000000000092810192909252517ff28dceb3000000000000000000000000000000000000000000000000000000008152919250737109709ecfa91a80626ff3989d68f67f5b1dd12d9163f28dceb39161085f91600401612123565b606060138054806020026020016040519081016040528092919081815260200182805480156103ce57602002820191905f5260205f2090815473ffffffffffffffffffffffffffffffffffffffff1681526001909101906020018083116103a3575050505050905090565b5f60296040518060600160405290815f8201805461117e9061241d565b80601f01602080910402602001604051908101604052809291908181526020018280546111aa9061241d565b80156111f55780601f106111cc576101008083540402835291602001916111f5565b820191905f5260205f20905b8154815290600101906020018083116111d857829003601f168201915b50505050508152602001600182015481526020016002820180546112189061241d565b80601f01602080910402602001604051908101604052809291908181526020018280546112449061241d565b801561128f5780601f106112665761010080835404028352916020019161128f565b820191905f5260205f20905b81548152906001019060200180831161127257829003601f168201915b505050919092525050604080518082018252600181525f60208083019190915290845281518083018352601681527f426164206d65726b6c652061727261792070726f6f66000000000000000000009181019190915290517ff28dceb3000000000000000000000000000000000000000000000000000000008152929350737109709ecfa91a80626ff3989d68f67f5b1dd12d9263f28dceb392506113379190600401612123565b5f604051808303815f87803b15801561134e575f5ffd5b505af1158015611360573d5f5f3e3d5ffd5b5050602254601c546023546040517f51588b8400000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff93841695506351588b8494506108ef939092169160259087906004016127b7565b60408051608081019091526025805460e01b7fffffffff00000000000000000000000000000000000000000000000000000000168252602680545f93929160208401916114139061241d565b80601f016020809104026020016040519081016040528092919081815260200182805461143f9061241d565b801561148a5780601f106114615761010080835404028352916020019161148a565b820191905f5260205f20905b81548152906001019060200180831161146d57829003601f168201915b505050505081526020016002820180546114a39061241d565b80601f01602080910402602001604051908101604052809291908181526020018280546114cf9061241d565b801561151a5780601f106114f15761010080835404028352916020019161151a565b820191905f5260205f20905b8154815290600101906020018083116114fd57829003601f168201915b50505091835250506003919091015460e01b7fffffffff0000000000000000000000000000000000000000000000000000000016602091820152604080518082018252600181525f818401528383015280518082018252601d81527f496e76616c696420696e70757420766563746f722070726f766964656400000092810192909252517ff28dceb3000000000000000000000000000000000000000000000000000000008152919250737109709ecfa91a80626ff3989d68f67f5b1dd12d9163f28dceb39161085f91600401612123565b60606102f18484846040518060400160405280600681526020017f6865696768740000000000000000000000000000000000000000000000000000815250611a46565b606061163b84846123b2565b67ffffffffffffffff81111561165357611653611f86565b60405190808252806020026020018201604052801561168657816020015b60608152602001906001900390816116715790505b509050835b8381101561178757611759866116a083611b94565b856040516020016116b393929190612836565b604051602081830303815290604052601d80546116cf9061241d565b80601f01602080910402602001604051908101604052809291908181526020018280546116fb9061241d565b80156117465780601f1061171d57610100808354040283529160200191611746565b820191905f5260205f20905b81548152906001019060200180831161172957829003601f168201915b5050505050611cc590919063ffffffff16565b8261176487846123b2565b81518110611774576117746123c5565b602090810291909101015260010161168b565b50949350505050565b606061179c84846123b2565b67ffffffffffffffff8111156117b4576117b4611f86565b6040519080825280602002602001820160405280156117dd578160200160208202803683370190505b509050835b83811015611787576118b0866117f783611b94565b8560405160200161180a93929190612836565b604051602081830303815290604052601d80546118269061241d565b80601f01602080910402602001604051908101604052809291908181526020018280546118529061241d565b801561189d5780601f106118745761010080835404028352916020019161189d565b820191905f5260205f20905b81548152906001019060200180831161188057829003601f168201915b5050505050611d6490919063ffffffff16565b826118bb87846123b2565b815181106118cb576118cb6123c5565b60209081029190910101526001016117e2565b80821461092e577f41304facd9323d75b11bcdd609cb38effffdb05710f7caf0e9b16c6d9d709f5060405161196a9060208082526025908201527f4572726f723a2061203d3d2062206e6f7420736174697366696564205b62797460408201527f657333325d000000000000000000000000000000000000000000000000000000606082015260800190565b60405180910390a160408051818152600a818301527f2020202020204c6566740000000000000000000000000000000000000000000060608201526020810184905290517fafb795c9c61e4fe7468c386f925d7a5429ecad9c0495ddb8d38d690614d32f999181900360800190a160408051818152600a818301527f202020202052696768740000000000000000000000000000000000000000000060608201526020810183905290517fafb795c9c61e4fe7468c386f925d7a5429ecad9c0495ddb8d38d690614d32f999181900360800190a161092e611df7565b6060611a5284846123b2565b67ffffffffffffffff811115611a6a57611a6a611f86565b604051908082528060200260200182016040528015611a93578160200160208202803683370190505b509050835b8381101561178757611b6686611aad83611b94565b85604051602001611ac093929190612836565b604051602081830303815290604052601d8054611adc9061241d565b80601f0160208091040260200160405190810160405280929190818152602001828054611b089061241d565b8015611b535780601f10611b2a57610100808354040283529160200191611b53565b820191905f5260205f20905b815481529060010190602001808311611b3657829003601f168201915b5050505050611f3290919063ffffffff16565b82611b7187846123b2565b81518110611b8157611b816123c5565b6020908102919091010152600101611a98565b6060815f03611bd657505060408051808201909152600181527f3000000000000000000000000000000000000000000000000000000000000000602082015290565b815f5b8115611bff5780611be9816128d3565b9150611bf89050600a83612937565b9150611bd9565b5f8167ffffffffffffffff811115611c1957611c19611f86565b6040519080825280601f01601f191660200182016040528015611c43576020820181803683370190505b5090505b84156102f157611c586001836123b2565b9150611c65600a8661294a565b611c7090603061295d565b60f81b818381518110611c8557611c856123c5565b60200101907effffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff191690815f1a905350611cbe600a86612937565b9450611c47565b6040517ffd921be8000000000000000000000000000000000000000000000000000000008152606090737109709ecfa91a80626ff3989d68f67f5b1dd12d9063fd921be890611d1a9086908690600401612970565b5f60405180830381865afa158015611d34573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052611d5b919081019061299d565b90505b92915050565b6040517f1777e59d0000000000000000000000000000000000000000000000000000000081525f90737109709ecfa91a80626ff3989d68f67f5b1dd12d90631777e59d90611db89086908690600401612970565b602060405180830381865afa158015611dd3573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611d5b9190612665565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15611f045760408051737109709ecfa91a80626ff3989d68f67f5b1dd12d602082018190527f6661696c6564000000000000000000000000000000000000000000000000000092820192909252600160608201525f91907f70ca10bbd0dbfd9020a9f4b13402c16cb120705e0d1c0aeab10fa353ae586fc49060800160408051601f1981840301815290829052611ea79291602001612753565b60408051601f1981840301815290829052611ec191612786565b5f604051808303815f865af19150503d805f8114611efa576040519150601f19603f3d011682016040523d82523d5f602084013e611eff565b606091505b505050505b600780547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00ff16610100179055565b6040517faddde2b60000000000000000000000000000000000000000000000000000000081525f90737109709ecfa91a80626ff3989d68f67f5b1dd12d9063addde2b690611db89086908690600401612970565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b604051601f8201601f1916810167ffffffffffffffff81118282101715611fdc57611fdc611f86565b604052919050565b5f67ffffffffffffffff821115611ffd57611ffd611f86565b50601f01601f191660200190565b5f5f5f6060848603121561201d575f5ffd5b833567ffffffffffffffff811115612033575f5ffd5b8401601f81018613612043575f5ffd5b803561205661205182611fe4565b611fb3565b81815287602083850101111561206a575f5ffd5b816020840160208301375f602092820183015297908601359650604090950135949350505050565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b8281101561211757603f19878603018452612102858351612092565b945060209384019391909101906001016120e6565b50929695505050505050565b602081525f611d5b6020830184612092565b602080825282518282018190525f918401906040840190835b8181101561218257835173ffffffffffffffffffffffffffffffffffffffff1683526020938401939092019160010161214e565b509095945050505050565b5f82825180855260208501945060208160051b830101602085015f5b838110156121db57601f198584030188526121c5838351612092565b60209889019890935091909101906001016121a9565b50909695505050505050565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b8281101561211757603f19878603018452815173ffffffffffffffffffffffffffffffffffffffff81511686526020810151905060406020870152612255604087018261218d565b955050602093840193919091019060010161220d565b602080825282518282018190525f918401906040840190835b81811015612182578351835260209384019390920191600101612284565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b8281101561211757868503603f190184528151805173ffffffffffffffffffffffffffffffffffffffff168652602090810151604082880181905281519088018190529101905f9060608801905b8083101561235b577fffffffff000000000000000000000000000000000000000000000000000000008451168252602082019150602084019350600183019250612316565b509650505060209384019391909101906001016122c8565b602081525f611d5b602083018461218d565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b81810381811115611d5e57611d5e612385565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f81518060208401855e5f93019283525090919050565b5f6102f161241783866123f2565b846123f2565b600181811c9082168061243157607f821691505b602082108103612468577f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b50919050565b80545f90600181811c9082168061248657607f821691505b6020821081036124bd577f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b818652602086018180156124d8576001811461250c57612538565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff008516825283151560051b82019550612538565b5f878152602090205f5b8581101561253257815484820152600190910190602001612516565b83019650505b505050505092915050565b606082525f612555606084018361246e565b6001830154602085015283810360408501526102f1816002850161246e565b73ffffffffffffffffffffffffffffffffffffffff85168152836020820152608060408201527fffffffff0000000000000000000000000000000000000000000000000000000083511660808201525f6020840151608060a08401526125de610100840182612092565b905060408501517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff808483030160c08501526126198282612092565b9150507fffffffff0000000000000000000000000000000000000000000000000000000060608601511660e0840152828103606084015261265a8185612543565b979650505050505050565b5f60208284031215612675575f5ffd5b5051919050565b7fffffffff00000000000000000000000000000000000000000000000000000000815460e01b168252608060208301525f6126bd608084016001840161246e565b83810360408501526126d2816002850161246e565b90507fffffffff00000000000000000000000000000000000000000000000000000000600384015460e01b1660608501528091505092915050565b73ffffffffffffffffffffffffffffffffffffffff85168152836020820152608060408201525f612741608083018561267c565b828103606084015261265a8185612543565b7fffffffff00000000000000000000000000000000000000000000000000000000831681525f6102f160048301846123f2565b5f611d5b82846123f2565b5f602082840312156127a1575f5ffd5b815180151581146127b0575f5ffd5b9392505050565b73ffffffffffffffffffffffffffffffffffffffff85168152836020820152608060408201525f6127eb608083018561267c565b82810360608401528351606082526128066060830182612092565b905060208501516020830152604085015182820360408401526128298282612092565b9998505050505050505050565b7f2e0000000000000000000000000000000000000000000000000000000000000081525f61286760018301866123f2565b7f5b00000000000000000000000000000000000000000000000000000000000000815261289760018201866123f2565b90507f5d2e00000000000000000000000000000000000000000000000000000000000081526128c960028201856123f2565b9695505050505050565b5f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff820361290357612903612385565b5060010190565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f826129455761294561290a565b500490565b5f826129585761295861290a565b500690565b80820180821115611d5e57611d5e612385565b604081525f6129826040830185612092565b82810360208401526129948185612092565b95945050505050565b5f602082840312156129ad575f5ffd5b815167ffffffffffffffff8111156129c3575f5ffd5b8201601f810184136129d3575f5ffd5b80516129e161205182611fe4565b8181528560208385010111156129f5575f5ffd5b8160208401602083015e5f9181016020019190915294935050505056fea26469706673582212201ca29e3f17a46aa00a3eab0db4101c405266208d3e68568fad37322f99cf475564736f6c634300081c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x01cW_5`\xE0\x1C\x80c\x91j\x17\xC6\x11a\0\xC7W\x80c\xE2\x0C\x9Fq\x11a\0}W\x80c\xEF>h\x12\x11a\0cW\x80c\xEF>h\x12\x14a\x02\x86W\x80c\xFAv&\xD4\x14a\x02\x8EW\x80c\xFA\xD0k\x8F\x14a\x02\x9BW__\xFD[\x80c\xE2\x0C\x9Fq\x14a\x02vW\x80c\xE8\x9F\x84\x19\x14a\x02~W__\xFD[\x80c\xB5P\x8A\xA9\x11a\0\xADW\x80c\xB5P\x8A\xA9\x14a\x02NW\x80c\xBAAO\xA6\x14a\x02VW\x80c\xC8\x99\xD2A\x14a\x02nW__\xFD[\x80c\x91j\x17\xC6\x14a\x02>W\x80c\xB5+ X\x14a\x02FW__\xFD[\x80c?r\x86\xF4\x11a\x01\x1CW\x80c]\xF0\xD0v\x11a\x01\x02W\x80c]\xF0\xD0v\x14a\x02\nW\x80cf\xD9\xA9\xA0\x14a\x02\x14W\x80c\x85\"l\x81\x14a\x02)W__\xFD[\x80c?r\x86\xF4\x14a\x01\xE2W\x80cD\xBA\xDB\xB6\x14a\x01\xEAW__\xFD[\x80c\x1E\xD7\x83\x1C\x11a\x01LW\x80c\x1E\xD7\x83\x1C\x14a\x01\xB0W\x80c*\xDE8\x80\x14a\x01\xC5W\x80c>^<#\x14a\x01\xDAW__\xFD[\x80c\x08\x13\x85*\x14a\x01gW\x80c\x1C\r\xA8\x1F\x14a\x01\x90W[__\xFD[a\x01za\x01u6`\x04a \x0BV[a\x02\xAEV[`@Qa\x01\x87\x91\x90a \xC0V[`@Q\x80\x91\x03\x90\xF3[a\x01\xA3a\x01\x9E6`\x04a \x0BV[a\x02\xF9V[`@Qa\x01\x87\x91\x90a!#V[a\x01\xB8a\x03kV[`@Qa\x01\x87\x91\x90a!5V[a\x01\xCDa\x03\xD8V[`@Qa\x01\x87\x91\x90a!\xE7V[a\x01\xB8a\x05!V[a\x01\xB8a\x05\x8CV[a\x01\xFDa\x01\xF86`\x04a \x0BV[a\x05\xF7V[`@Qa\x01\x87\x91\x90a\"kV[a\x02\x12a\x06:V[\0[a\x02\x1Ca\t2V[`@Qa\x01\x87\x91\x90a\"\xA2V[a\x021a\n5V[`@Qa\x01\x87\x91\x90a#sV[a\x02\x1Ca\x0B\0V[a\x02\x12a\x0C\x03V[a\x021a\x0C\xBAV[a\x02^a\r\x85V[`@Q\x90\x15\x15\x81R` \x01a\x01\x87V[a\x02\x12a\x0E\xC2V[a\x01\xB8a\x10\xF6V[a\x02\x12a\x11aV[a\x02\x12a\x13\xC7V[`\x07Ta\x02^\x90`\xFF\x16\x81V[a\x01\xFDa\x02\xA96`\x04a \x0BV[a\x15\xECV[``a\x02\xF1\x84\x84\x84`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01\x7Fhex\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x16/V[\x94\x93PPPPV[``_a\x03\x07\x85\x85\x85a\x02\xAEV[\x90P_[a\x03\x15\x85\x85a#\xB2V[\x81\x10\x15a\x03bW\x82\x82\x82\x81Q\x81\x10a\x03/Wa\x03/a#\xC5V[` \x02` \x01\x01Q`@Q` \x01a\x03H\x92\x91\x90a$\tV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x92P`\x01\x01a\x03\x0BV[PP\x93\x92PPPV[```\x14\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x03\xCEW` \x02\x82\x01\x91\x90_R` _ \x90[\x81Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x03\xA3W[PPPPP\x90P\x90V[```\x1B\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x05\x18W_\x84\x81R` \x80\x82 `@\x80Q\x80\x82\x01\x82R`\x02\x87\x02\x90\x92\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x95\x91\x94\x86\x81\x01\x94\x91\x92\x90\x84\x01[\x82\x82\x10\x15a\x05\x01W\x83\x82\x90_R` _ \x01\x80Ta\x04v\x90a$\x1DV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x04\xA2\x90a$\x1DV[\x80\x15a\x04\xEDW\x80`\x1F\x10a\x04\xC4Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x04\xEDV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x04\xD0W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x04YV[PPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x03\xFBV[PPPP\x90P\x90V[```\x16\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x03\xCEW` \x02\x82\x01\x91\x90_R` _ \x90\x81Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x03\xA3WPPPPP\x90P\x90V[```\x15\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x03\xCEW` \x02\x82\x01\x91\x90_R` _ \x90\x81Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x03\xA3WPPPPP\x90P\x90V[``a\x02\xF1\x84\x84\x84`@Q\x80`@\x01`@R\x80`\t\x81R` \x01\x7Fdigest_le\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x17\x90V[`@\x80Q`\x80\x81\x01\x90\x91R`%\x80T`\xE0\x1B\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x82R`&\x80T_\x93\x92\x91` \x84\x01\x91a\x06\x86\x90a$\x1DV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x06\xB2\x90a$\x1DV[\x80\x15a\x06\xFDW\x80`\x1F\x10a\x06\xD4Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x06\xFDV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x06\xE0W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x02\x82\x01\x80Ta\x07\x16\x90a$\x1DV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x07B\x90a$\x1DV[\x80\x15a\x07\x8DW\x80`\x1F\x10a\x07dWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x07\x8DV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x07pW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x83RPP`\x03\x91\x90\x91\x01T`\xE0\x1B\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16` \x91\x82\x01R`@\x80Q\x80\x82\x01\x82R`\x01\x81R_\x81\x84\x01R\x83\x82\x01R\x80Q\x80\x82\x01\x82R`\x1E\x81R\x7FInvalid output vector provided\0\0\x92\x81\x01\x92\x90\x92RQ\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x91\x92Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x91c\xF2\x8D\xCE\xB3\x91a\x08_\x91`\x04\x01a!#V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x08vW__\xFD[PZ\xF1\x15\x80\x15a\x08\x88W=__>=_\xFD[PP`\"T`\x1CT`#T`@Q\x7FQX\x8B\x84\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x84\x16\x95PcQX\x8B\x84\x94Pa\x08\xEF\x93\x90\x92\x16\x91\x86\x90`)\x90`\x04\x01a%tV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\nW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t.\x91\x90a&eV[PPV[```\x19\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x05\x18W_\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\n\x1DW` \x02\x82\x01\x91\x90_R` _ \x90_\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\t\xCAW\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\tUV[```\x18\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x05\x18W\x83\x82\x90_R` _ \x01\x80Ta\nu\x90a$\x1DV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\n\xA1\x90a$\x1DV[\x80\x15a\n\xECW\x80`\x1F\x10a\n\xC3Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\n\xECV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\n\xCFW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\nXV[```\x1A\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x05\x18W_\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x0B\xEBW` \x02\x82\x01\x91\x90_R` _ \x90_\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x0B\x98W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x0B#V[`\"T`\x1CT`#T`@Q\x7FQX\x8B\x84\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R_\x93s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x93cQX\x8B\x84\x93a\x0Cj\x93\x92\x90\x91\x16\x91`%\x90`)\x90`\x04\x01a'\rV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\x85W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xA9\x91\x90a&eV[\x90Pa\x0C\xB7\x81`$Ta\x18\xDEV[PV[```\x17\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x05\x18W\x83\x82\x90_R` _ \x01\x80Ta\x0C\xFA\x90a$\x1DV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\r&\x90a$\x1DV[\x80\x15a\rqW\x80`\x1F\x10a\rHWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\rqV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\rTW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x0C\xDDV[`\x07T_\x90a\x01\0\x90\x04`\xFF\x16\x15a\r\xA6WP`\x07Ta\x01\0\x90\x04`\xFF\x16\x90V[_sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0E\xBDW`@\x80Qsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-` \x82\x01\x81\x90R\x7Ffailed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x84\x01R\x82Q\x80\x83\x03\x84\x01\x81R``\x83\x01\x90\x93R_\x92\x90\x91a\x0EI\x91\x7Ff\x7F\x9Dp\xCAA\x1Dp\xEA\xD5\r\x8D\\\"\x07\r\xAF\xC3j\xD7_=\xCF^r7\xB2*\xDE\x9A\xEC\xC4\x91`\x80\x01a'SV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x0Ec\x91a'\x86V[_`@Q\x80\x83\x03\x81_\x86Z\xF1\x91PP=\x80_\x81\x14a\x0E\x9CW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x0E\xA1V[``\x91P[P\x91PP\x80\x80` \x01\x90Q\x81\x01\x90a\x0E\xB9\x91\x90a'\x91V[\x91PP[\x91\x90PV[`@\x80Q`\x80\x81\x01\x90\x91R`%\x80T`\xE0\x1B\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x82R`&\x80T_\x93\x92\x91` \x84\x01\x91a\x0F\x0E\x90a$\x1DV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0F:\x90a$\x1DV[\x80\x15a\x0F\x85W\x80`\x1F\x10a\x0F\\Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0F\x85V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0FhW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x02\x82\x01\x80Ta\x0F\x9E\x90a$\x1DV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0F\xCA\x90a$\x1DV[\x80\x15a\x10\x15W\x80`\x1F\x10a\x0F\xECWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x10\x15V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0F\xF8W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x83RPP`\x03\x91\x90\x91\x01T`\xE0\x1B\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16` \x91\x82\x01R|\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x83\x01R`@\x80Q\x80\x82\x01\x82R`\x13\x81R\x7FBad inclusion proof\0\0\0\0\0\0\0\0\0\0\0\0\0\x92\x81\x01\x92\x90\x92RQ\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x91\x92Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x91c\xF2\x8D\xCE\xB3\x91a\x08_\x91`\x04\x01a!#V[```\x13\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x03\xCEW` \x02\x82\x01\x91\x90_R` _ \x90\x81Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x03\xA3WPPPPP\x90P\x90V[_`)`@Q\x80``\x01`@R\x90\x81_\x82\x01\x80Ta\x11~\x90a$\x1DV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x11\xAA\x90a$\x1DV[\x80\x15a\x11\xF5W\x80`\x1F\x10a\x11\xCCWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x11\xF5V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x11\xD8W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x01\x82\x01T\x81R` \x01`\x02\x82\x01\x80Ta\x12\x18\x90a$\x1DV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x12D\x90a$\x1DV[\x80\x15a\x12\x8FW\x80`\x1F\x10a\x12fWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x12\x8FV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x12rW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x90\x92RPP`@\x80Q\x80\x82\x01\x82R`\x01\x81R_` \x80\x83\x01\x91\x90\x91R\x90\x84R\x81Q\x80\x83\x01\x83R`\x16\x81R\x7FBad merkle array proof\0\0\0\0\0\0\0\0\0\0\x91\x81\x01\x91\x90\x91R\x90Q\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x92\x93Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x92c\xF2\x8D\xCE\xB3\x92Pa\x137\x91\x90`\x04\x01a!#V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x13NW__\xFD[PZ\xF1\x15\x80\x15a\x13`W=__>=_\xFD[PP`\"T`\x1CT`#T`@Q\x7FQX\x8B\x84\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x84\x16\x95PcQX\x8B\x84\x94Pa\x08\xEF\x93\x90\x92\x16\x91`%\x90\x87\x90`\x04\x01a'\xB7V[`@\x80Q`\x80\x81\x01\x90\x91R`%\x80T`\xE0\x1B\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x82R`&\x80T_\x93\x92\x91` \x84\x01\x91a\x14\x13\x90a$\x1DV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x14?\x90a$\x1DV[\x80\x15a\x14\x8AW\x80`\x1F\x10a\x14aWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x14\x8AV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x14mW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x02\x82\x01\x80Ta\x14\xA3\x90a$\x1DV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x14\xCF\x90a$\x1DV[\x80\x15a\x15\x1AW\x80`\x1F\x10a\x14\xF1Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x15\x1AV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x14\xFDW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x83RPP`\x03\x91\x90\x91\x01T`\xE0\x1B\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16` \x91\x82\x01R`@\x80Q\x80\x82\x01\x82R`\x01\x81R_\x81\x84\x01R\x83\x83\x01R\x80Q\x80\x82\x01\x82R`\x1D\x81R\x7FInvalid input vector provided\0\0\0\x92\x81\x01\x92\x90\x92RQ\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x91\x92Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x91c\xF2\x8D\xCE\xB3\x91a\x08_\x91`\x04\x01a!#V[``a\x02\xF1\x84\x84\x84`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01\x7Fheight\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x1AFV[``a\x16;\x84\x84a#\xB2V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x16SWa\x16Sa\x1F\x86V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x16\x86W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x16qW\x90P[P\x90P\x83[\x83\x81\x10\x15a\x17\x87Wa\x17Y\x86a\x16\xA0\x83a\x1B\x94V[\x85`@Q` \x01a\x16\xB3\x93\x92\x91\x90a(6V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`\x1D\x80Ta\x16\xCF\x90a$\x1DV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x16\xFB\x90a$\x1DV[\x80\x15a\x17FW\x80`\x1F\x10a\x17\x1DWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x17FV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x17)W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPPa\x1C\xC5\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x82a\x17d\x87\x84a#\xB2V[\x81Q\x81\x10a\x17tWa\x17ta#\xC5V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x16\x8BV[P\x94\x93PPPPV[``a\x17\x9C\x84\x84a#\xB2V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x17\xB4Wa\x17\xB4a\x1F\x86V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x17\xDDW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P\x83[\x83\x81\x10\x15a\x17\x87Wa\x18\xB0\x86a\x17\xF7\x83a\x1B\x94V[\x85`@Q` \x01a\x18\n\x93\x92\x91\x90a(6V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`\x1D\x80Ta\x18&\x90a$\x1DV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x18R\x90a$\x1DV[\x80\x15a\x18\x9DW\x80`\x1F\x10a\x18tWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x18\x9DV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x18\x80W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPPa\x1Dd\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x82a\x18\xBB\x87\x84a#\xB2V[\x81Q\x81\x10a\x18\xCBWa\x18\xCBa#\xC5V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x17\xE2V[\x80\x82\x14a\t.W\x7FA0O\xAC\xD92=u\xB1\x1B\xCD\xD6\t\xCB8\xEF\xFF\xFD\xB0W\x10\xF7\xCA\xF0\xE9\xB1lm\x9Dp\x9FP`@Qa\x19j\x90` \x80\x82R`%\x90\x82\x01R\x7FError: a == b not satisfied [byt`@\x82\x01R\x7Fes32]\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x82\x01R`\x80\x01\x90V[`@Q\x80\x91\x03\x90\xA1`@\x80Q\x81\x81R`\n\x81\x83\x01R\x7F      Left\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x82\x01R` \x81\x01\x84\x90R\x90Q\x7F\xAF\xB7\x95\xC9\xC6\x1EO\xE7F\x8C8o\x92]zT)\xEC\xAD\x9C\x04\x95\xDD\xB8\xD3\x8Di\x06\x14\xD3/\x99\x91\x81\x90\x03`\x80\x01\x90\xA1`@\x80Q\x81\x81R`\n\x81\x83\x01R\x7F     Right\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x82\x01R` \x81\x01\x83\x90R\x90Q\x7F\xAF\xB7\x95\xC9\xC6\x1EO\xE7F\x8C8o\x92]zT)\xEC\xAD\x9C\x04\x95\xDD\xB8\xD3\x8Di\x06\x14\xD3/\x99\x91\x81\x90\x03`\x80\x01\x90\xA1a\t.a\x1D\xF7V[``a\x1AR\x84\x84a#\xB2V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1AjWa\x1Aja\x1F\x86V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1A\x93W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P\x83[\x83\x81\x10\x15a\x17\x87Wa\x1Bf\x86a\x1A\xAD\x83a\x1B\x94V[\x85`@Q` \x01a\x1A\xC0\x93\x92\x91\x90a(6V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`\x1D\x80Ta\x1A\xDC\x90a$\x1DV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1B\x08\x90a$\x1DV[\x80\x15a\x1BSW\x80`\x1F\x10a\x1B*Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1BSV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1B6W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPPa\x1F2\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x82a\x1Bq\x87\x84a#\xB2V[\x81Q\x81\x10a\x1B\x81Wa\x1B\x81a#\xC5V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x1A\x98V[``\x81_\x03a\x1B\xD6WPP`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R\x7F0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x90V[\x81_[\x81\x15a\x1B\xFFW\x80a\x1B\xE9\x81a(\xD3V[\x91Pa\x1B\xF8\x90P`\n\x83a)7V[\x91Pa\x1B\xD9V[_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1C\x19Wa\x1C\x19a\x1F\x86V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x1CCW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P[\x84\x15a\x02\xF1Wa\x1CX`\x01\x83a#\xB2V[\x91Pa\x1Ce`\n\x86a)JV[a\x1Cp\x90`0a)]V[`\xF8\x1B\x81\x83\x81Q\x81\x10a\x1C\x85Wa\x1C\x85a#\xC5V[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81_\x1A\x90SPa\x1C\xBE`\n\x86a)7V[\x94Pa\x1CGV[`@Q\x7F\xFD\x92\x1B\xE8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R``\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\xFD\x92\x1B\xE8\x90a\x1D\x1A\x90\x86\x90\x86\x90`\x04\x01a)pV[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1D4W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1D[\x91\x90\x81\x01\x90a)\x9DV[\x90P[\x92\x91PPV[`@Q\x7F\x17w\xE5\x9D\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R_\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\x17w\xE5\x9D\x90a\x1D\xB8\x90\x86\x90\x86\x90`\x04\x01a)pV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1D\xD3W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D[\x91\x90a&eV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x1F\x04W`@\x80Qsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-` \x82\x01\x81\x90R\x7Ffailed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92\x82\x01\x92\x90\x92R`\x01``\x82\x01R_\x91\x90\x7Fp\xCA\x10\xBB\xD0\xDB\xFD\x90 \xA9\xF4\xB14\x02\xC1l\xB1 p^\r\x1C\n\xEA\xB1\x0F\xA3S\xAEXo\xC4\x90`\x80\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x1E\xA7\x92\x91` \x01a'SV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x1E\xC1\x91a'\x86V[_`@Q\x80\x83\x03\x81_\x86Z\xF1\x91PP=\x80_\x81\x14a\x1E\xFAW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x1E\xFFV[``\x91P[PPPP[`\x07\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\x16a\x01\0\x17\x90UV[`@Q\x7F\xAD\xDD\xE2\xB6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R_\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\xAD\xDD\xE2\xB6\x90a\x1D\xB8\x90\x86\x90\x86\x90`\x04\x01a)pV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1F\xDCWa\x1F\xDCa\x1F\x86V[`@R\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x1F\xFDWa\x1F\xFDa\x1F\x86V[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[___``\x84\x86\x03\x12\x15a \x1DW__\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a 3W__\xFD[\x84\x01`\x1F\x81\x01\x86\x13a CW__\xFD[\x805a Va Q\x82a\x1F\xE4V[a\x1F\xB3V[\x81\x81R\x87` \x83\x85\x01\x01\x11\x15a jW__\xFD[\x81` \x84\x01` \x83\x017_` \x92\x82\x01\x83\x01R\x97\x90\x86\x015\x96P`@\x90\x95\x015\x94\x93PPPPV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a!\x17W`?\x19\x87\x86\x03\x01\x84Ra!\x02\x85\x83Qa \x92V[\x94P` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a \xE6V[P\x92\x96\x95PPPPPPV[` \x81R_a\x1D[` \x83\x01\x84a \x92V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a!\x82W\x83Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a!NV[P\x90\x95\x94PPPPPV[_\x82\x82Q\x80\x85R` \x85\x01\x94P` \x81`\x05\x1B\x83\x01\x01` \x85\x01_[\x83\x81\x10\x15a!\xDBW`\x1F\x19\x85\x84\x03\x01\x88Ra!\xC5\x83\x83Qa \x92V[` \x98\x89\x01\x98\x90\x93P\x91\x90\x91\x01\x90`\x01\x01a!\xA9V[P\x90\x96\x95PPPPPPV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a!\x17W`?\x19\x87\x86\x03\x01\x84R\x81Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16\x86R` \x81\x01Q\x90P`@` \x87\x01Ra\"U`@\x87\x01\x82a!\x8DV[\x95PP` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a\"\rV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a!\x82W\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\"\x84V[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a!\x17W\x86\x85\x03`?\x19\x01\x84R\x81Q\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x86R` \x90\x81\x01Q`@\x82\x88\x01\x81\x90R\x81Q\x90\x88\x01\x81\x90R\x91\x01\x90_\x90``\x88\x01\x90[\x80\x83\x10\x15a#[W\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84Q\x16\x82R` \x82\x01\x91P` \x84\x01\x93P`\x01\x83\x01\x92Pa#\x16V[P\x96PPP` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a\"\xC8V[` \x81R_a\x1D[` \x83\x01\x84a!\x8DV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x1D^Wa\x1D^a#\x85V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_\x81Q\x80` \x84\x01\x85^_\x93\x01\x92\x83RP\x90\x91\x90PV[_a\x02\xF1a$\x17\x83\x86a#\xF2V[\x84a#\xF2V[`\x01\x81\x81\x1C\x90\x82\x16\x80a$1W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a$hW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[P\x91\x90PV[\x80T_\x90`\x01\x81\x81\x1C\x90\x82\x16\x80a$\x86W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a$\xBDW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[\x81\x86R` \x86\x01\x81\x80\x15a$\xD8W`\x01\x81\x14a%\x0CWa%8V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x85\x16\x82R\x83\x15\x15`\x05\x1B\x82\x01\x95Pa%8V[_\x87\x81R` \x90 _[\x85\x81\x10\x15a%2W\x81T\x84\x82\x01R`\x01\x90\x91\x01\x90` \x01a%\x16V[\x83\x01\x96PP[PPPPP\x92\x91PPV[``\x82R_a%U``\x84\x01\x83a$nV[`\x01\x83\x01T` \x85\x01R\x83\x81\x03`@\x85\x01Ra\x02\xF1\x81`\x02\x85\x01a$nV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x81R\x83` \x82\x01R`\x80`@\x82\x01R\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83Q\x16`\x80\x82\x01R_` \x84\x01Q`\x80`\xA0\x84\x01Ra%\xDEa\x01\0\x84\x01\x82a \x92V[\x90P`@\x85\x01Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x84\x83\x03\x01`\xC0\x85\x01Ra&\x19\x82\x82a \x92V[\x91PP\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x86\x01Q\x16`\xE0\x84\x01R\x82\x81\x03``\x84\x01Ra&Z\x81\x85a%CV[\x97\x96PPPPPPPV[_` \x82\x84\x03\x12\x15a&uW__\xFD[PQ\x91\x90PV[\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81T`\xE0\x1B\x16\x82R`\x80` \x83\x01R_a&\xBD`\x80\x84\x01`\x01\x84\x01a$nV[\x83\x81\x03`@\x85\x01Ra&\xD2\x81`\x02\x85\x01a$nV[\x90P\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x03\x84\x01T`\xE0\x1B\x16``\x85\x01R\x80\x91PP\x92\x91PPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x81R\x83` \x82\x01R`\x80`@\x82\x01R_a'A`\x80\x83\x01\x85a&|V[\x82\x81\x03``\x84\x01Ra&Z\x81\x85a%CV[\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x81R_a\x02\xF1`\x04\x83\x01\x84a#\xF2V[_a\x1D[\x82\x84a#\xF2V[_` \x82\x84\x03\x12\x15a'\xA1W__\xFD[\x81Q\x80\x15\x15\x81\x14a'\xB0W__\xFD[\x93\x92PPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x81R\x83` \x82\x01R`\x80`@\x82\x01R_a'\xEB`\x80\x83\x01\x85a&|V[\x82\x81\x03``\x84\x01R\x83Q``\x82Ra(\x06``\x83\x01\x82a \x92V[\x90P` \x85\x01Q` \x83\x01R`@\x85\x01Q\x82\x82\x03`@\x84\x01Ra()\x82\x82a \x92V[\x99\x98PPPPPPPPPV[\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R_a(g`\x01\x83\x01\x86a#\xF2V[\x7F[\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Ra(\x97`\x01\x82\x01\x86a#\xF2V[\x90P\x7F].\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Ra(\xC9`\x02\x82\x01\x85a#\xF2V[\x96\x95PPPPPPV[_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a)\x03Wa)\x03a#\x85V[P`\x01\x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_\x82a)EWa)Ea)\nV[P\x04\x90V[_\x82a)XWa)Xa)\nV[P\x06\x90V[\x80\x82\x01\x80\x82\x11\x15a\x1D^Wa\x1D^a#\x85V[`@\x81R_a)\x82`@\x83\x01\x85a \x92V[\x82\x81\x03` \x84\x01Ra)\x94\x81\x85a \x92V[\x95\x94PPPPPV[_` \x82\x84\x03\x12\x15a)\xADW__\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a)\xC3W__\xFD[\x82\x01`\x1F\x81\x01\x84\x13a)\xD3W__\xFD[\x80Qa)\xE1a Q\x82a\x1F\xE4V[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a)\xF5W__\xFD[\x81` \x84\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV\xFE\xA2dipfsX\"\x12 \x1C\xA2\x9E?\x17\xA4j\xA0\n>\xAB\r\xB4\x10\x1C@Rf \x8D>hV\x8F\xAD72/\x99\xCFGUdsolcC\0\x08\x1C\x003",
    );
    /**Event with signature `log(string)` and selector `0x41304facd9323d75b11bcdd609cb38effffdb05710f7caf0e9b16c6d9d709f50`.
```solidity
event log(string);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log {
        #[allow(missing_docs)]
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log {
            type DataTuple<'a> = (alloy::sol_types::sol_data::String,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log(string)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                65u8,
                48u8,
                79u8,
                172u8,
                217u8,
                50u8,
                61u8,
                117u8,
                177u8,
                27u8,
                205u8,
                214u8,
                9u8,
                203u8,
                56u8,
                239u8,
                255u8,
                253u8,
                176u8,
                87u8,
                16u8,
                247u8,
                202u8,
                240u8,
                233u8,
                177u8,
                108u8,
                109u8,
                157u8,
                112u8,
                159u8,
                80u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { _0: data.0 }
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self._0,
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
        impl alloy_sol_types::private::IntoLogData for log {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_address(address)` and selector `0x7ae74c527414ae135fd97047b12921a5ec3911b804197855d67e25c7b75ee6f3`.
```solidity
event log_address(address);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_address {
        #[allow(missing_docs)]
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log_address {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_address(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                122u8,
                231u8,
                76u8,
                82u8,
                116u8,
                20u8,
                174u8,
                19u8,
                95u8,
                217u8,
                112u8,
                71u8,
                177u8,
                41u8,
                33u8,
                165u8,
                236u8,
                57u8,
                17u8,
                184u8,
                4u8,
                25u8,
                120u8,
                85u8,
                214u8,
                126u8,
                37u8,
                199u8,
                183u8,
                94u8,
                230u8,
                243u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { _0: data.0 }
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
                        &self._0,
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
        impl alloy_sol_types::private::IntoLogData for log_address {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_address> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_address) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_array(uint256[])` and selector `0xfb102865d50addddf69da9b5aa1bced66c80cf869a5c8d0471a467e18ce9cab1`.
```solidity
event log_array(uint256[] val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_array_0 {
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::Vec<
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log_array_0 {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_array(uint256[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                251u8,
                16u8,
                40u8,
                101u8,
                213u8,
                10u8,
                221u8,
                221u8,
                246u8,
                157u8,
                169u8,
                181u8,
                170u8,
                27u8,
                206u8,
                214u8,
                108u8,
                128u8,
                207u8,
                134u8,
                154u8,
                92u8,
                141u8,
                4u8,
                113u8,
                164u8,
                103u8,
                225u8,
                140u8,
                233u8,
                202u8,
                177u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { val: data.0 }
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
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
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
        impl alloy_sol_types::private::IntoLogData for log_array_0 {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_array_0> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_array_0) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_array(int256[])` and selector `0x890a82679b470f2bd82816ed9b161f97d8b967f37fa3647c21d5bf39749e2dd5`.
```solidity
event log_array(int256[] val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_array_1 {
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::I256,
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log_array_1 {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Int<256>>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_array(int256[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                137u8,
                10u8,
                130u8,
                103u8,
                155u8,
                71u8,
                15u8,
                43u8,
                216u8,
                40u8,
                22u8,
                237u8,
                155u8,
                22u8,
                31u8,
                151u8,
                216u8,
                185u8,
                103u8,
                243u8,
                127u8,
                163u8,
                100u8,
                124u8,
                33u8,
                213u8,
                191u8,
                57u8,
                116u8,
                158u8,
                45u8,
                213u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { val: data.0 }
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
                        alloy::sol_types::sol_data::Int<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
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
        impl alloy_sol_types::private::IntoLogData for log_array_1 {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_array_1> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_array_1) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_array(address[])` and selector `0x40e1840f5769073d61bd01372d9b75baa9842d5629a0c99ff103be1178a8e9e2`.
```solidity
event log_array(address[] val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_array_2 {
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
        impl alloy_sol_types::SolEvent for log_array_2 {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_array(address[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                64u8,
                225u8,
                132u8,
                15u8,
                87u8,
                105u8,
                7u8,
                61u8,
                97u8,
                189u8,
                1u8,
                55u8,
                45u8,
                155u8,
                117u8,
                186u8,
                169u8,
                132u8,
                45u8,
                86u8,
                41u8,
                160u8,
                201u8,
                159u8,
                241u8,
                3u8,
                190u8,
                17u8,
                120u8,
                168u8,
                233u8,
                226u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { val: data.0 }
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
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
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
        impl alloy_sol_types::private::IntoLogData for log_array_2 {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_array_2> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_array_2) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_bytes(bytes)` and selector `0x23b62ad0584d24a75f0bf3560391ef5659ec6db1269c56e11aa241d637f19b20`.
```solidity
event log_bytes(bytes);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_bytes {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Bytes,
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
        impl alloy_sol_types::SolEvent for log_bytes {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_bytes(bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                35u8,
                182u8,
                42u8,
                208u8,
                88u8,
                77u8,
                36u8,
                167u8,
                95u8,
                11u8,
                243u8,
                86u8,
                3u8,
                145u8,
                239u8,
                86u8,
                89u8,
                236u8,
                109u8,
                177u8,
                38u8,
                156u8,
                86u8,
                225u8,
                26u8,
                162u8,
                65u8,
                214u8,
                55u8,
                241u8,
                155u8,
                32u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { _0: data.0 }
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
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self._0,
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
        impl alloy_sol_types::private::IntoLogData for log_bytes {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_bytes> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_bytes) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_bytes32(bytes32)` and selector `0xe81699b85113eea1c73e10588b2b035e55893369632173afd43feb192fac64e3`.
```solidity
event log_bytes32(bytes32);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_bytes32 {
        #[allow(missing_docs)]
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log_bytes32 {
            type DataTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_bytes32(bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                232u8,
                22u8,
                153u8,
                184u8,
                81u8,
                19u8,
                238u8,
                161u8,
                199u8,
                62u8,
                16u8,
                88u8,
                139u8,
                43u8,
                3u8,
                94u8,
                85u8,
                137u8,
                51u8,
                105u8,
                99u8,
                33u8,
                115u8,
                175u8,
                212u8,
                63u8,
                235u8,
                25u8,
                47u8,
                172u8,
                100u8,
                227u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { _0: data.0 }
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
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self._0),
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
        impl alloy_sol_types::private::IntoLogData for log_bytes32 {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_bytes32> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_bytes32) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_int(int256)` and selector `0x0eb5d52624c8d28ada9fc55a8c502ed5aa3fbe2fb6e91b71b5f376882b1d2fb8`.
```solidity
event log_int(int256);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_int {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::I256,
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
        impl alloy_sol_types::SolEvent for log_int {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Int<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_int(int256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                14u8,
                181u8,
                213u8,
                38u8,
                36u8,
                200u8,
                210u8,
                138u8,
                218u8,
                159u8,
                197u8,
                90u8,
                140u8,
                80u8,
                46u8,
                213u8,
                170u8,
                63u8,
                190u8,
                47u8,
                182u8,
                233u8,
                27u8,
                113u8,
                181u8,
                243u8,
                118u8,
                136u8,
                43u8,
                29u8,
                47u8,
                184u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { _0: data.0 }
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
                    <alloy::sol_types::sol_data::Int<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._0),
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
        impl alloy_sol_types::private::IntoLogData for log_int {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_int> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_int) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_named_address(string,address)` and selector `0x9c4e8541ca8f0dc1c413f9108f66d82d3cecb1bddbce437a61caa3175c4cc96f`.
```solidity
event log_named_address(string key, address val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_address {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for log_named_address {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Address,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_address(string,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                156u8,
                78u8,
                133u8,
                65u8,
                202u8,
                143u8,
                13u8,
                193u8,
                196u8,
                19u8,
                249u8,
                16u8,
                143u8,
                102u8,
                216u8,
                45u8,
                60u8,
                236u8,
                177u8,
                189u8,
                219u8,
                206u8,
                67u8,
                122u8,
                97u8,
                202u8,
                163u8,
                23u8,
                92u8,
                76u8,
                201u8,
                111u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { key: data.0, val: data.1 }
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.val,
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
        impl alloy_sol_types::private::IntoLogData for log_named_address {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_address> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_address) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_named_array(string,uint256[])` and selector `0x00aaa39c9ffb5f567a4534380c737075702e1f7f14107fc95328e3b56c0325fb`.
```solidity
event log_named_array(string key, uint256[] val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_array_0 {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::Vec<
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log_named_array_0 {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_array(string,uint256[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                0u8,
                170u8,
                163u8,
                156u8,
                159u8,
                251u8,
                95u8,
                86u8,
                122u8,
                69u8,
                52u8,
                56u8,
                12u8,
                115u8,
                112u8,
                117u8,
                112u8,
                46u8,
                31u8,
                127u8,
                20u8,
                16u8,
                127u8,
                201u8,
                83u8,
                40u8,
                227u8,
                181u8,
                108u8,
                3u8,
                37u8,
                251u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { key: data.0, val: data.1 }
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
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
        impl alloy_sol_types::private::IntoLogData for log_named_array_0 {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_array_0> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_array_0) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_named_array(string,int256[])` and selector `0xa73eda09662f46dde729be4611385ff34fe6c44fbbc6f7e17b042b59a3445b57`.
```solidity
event log_named_array(string key, int256[] val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_array_1 {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::I256,
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log_named_array_1 {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Int<256>>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_array(string,int256[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                167u8,
                62u8,
                218u8,
                9u8,
                102u8,
                47u8,
                70u8,
                221u8,
                231u8,
                41u8,
                190u8,
                70u8,
                17u8,
                56u8,
                95u8,
                243u8,
                79u8,
                230u8,
                196u8,
                79u8,
                187u8,
                198u8,
                247u8,
                225u8,
                123u8,
                4u8,
                43u8,
                89u8,
                163u8,
                68u8,
                91u8,
                87u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { key: data.0, val: data.1 }
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Int<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
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
        impl alloy_sol_types::private::IntoLogData for log_named_array_1 {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_array_1> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_array_1) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_named_array(string,address[])` and selector `0x3bcfb2ae2e8d132dd1fce7cf278a9a19756a9fceabe470df3bdabb4bc577d1bd`.
```solidity
event log_named_array(string key, address[] val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_array_2 {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
        impl alloy_sol_types::SolEvent for log_named_array_2 {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_array(string,address[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                59u8,
                207u8,
                178u8,
                174u8,
                46u8,
                141u8,
                19u8,
                45u8,
                209u8,
                252u8,
                231u8,
                207u8,
                39u8,
                138u8,
                154u8,
                25u8,
                117u8,
                106u8,
                159u8,
                206u8,
                171u8,
                228u8,
                112u8,
                223u8,
                59u8,
                218u8,
                187u8,
                75u8,
                197u8,
                119u8,
                209u8,
                189u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { key: data.0, val: data.1 }
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
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
        impl alloy_sol_types::private::IntoLogData for log_named_array_2 {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_array_2> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_array_2) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_named_bytes(string,bytes)` and selector `0xd26e16cad4548705e4c9e2d94f98ee91c289085ee425594fd5635fa2964ccf18`.
```solidity
event log_named_bytes(string key, bytes val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_bytes {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::Bytes,
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
        impl alloy_sol_types::SolEvent for log_named_bytes {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Bytes,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_bytes(string,bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                210u8,
                110u8,
                22u8,
                202u8,
                212u8,
                84u8,
                135u8,
                5u8,
                228u8,
                201u8,
                226u8,
                217u8,
                79u8,
                152u8,
                238u8,
                145u8,
                194u8,
                137u8,
                8u8,
                94u8,
                228u8,
                37u8,
                89u8,
                79u8,
                213u8,
                99u8,
                95u8,
                162u8,
                150u8,
                76u8,
                207u8,
                24u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { key: data.0, val: data.1 }
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.val,
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
        impl alloy_sol_types::private::IntoLogData for log_named_bytes {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_bytes> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_bytes) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_named_bytes32(string,bytes32)` and selector `0xafb795c9c61e4fe7468c386f925d7a5429ecad9c0495ddb8d38d690614d32f99`.
```solidity
event log_named_bytes32(string key, bytes32 val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_bytes32 {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::FixedBytes<32>,
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
        impl alloy_sol_types::SolEvent for log_named_bytes32 {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_bytes32(string,bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                175u8,
                183u8,
                149u8,
                201u8,
                198u8,
                30u8,
                79u8,
                231u8,
                70u8,
                140u8,
                56u8,
                111u8,
                146u8,
                93u8,
                122u8,
                84u8,
                41u8,
                236u8,
                173u8,
                156u8,
                4u8,
                149u8,
                221u8,
                184u8,
                211u8,
                141u8,
                105u8,
                6u8,
                20u8,
                211u8,
                47u8,
                153u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { key: data.0, val: data.1 }
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
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
        impl alloy_sol_types::private::IntoLogData for log_named_bytes32 {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_bytes32> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_bytes32) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_named_decimal_int(string,int256,uint256)` and selector `0x5da6ce9d51151ba10c09a559ef24d520b9dac5c5b8810ae8434e4d0d86411a95`.
```solidity
event log_named_decimal_int(string key, int256 val, uint256 decimals);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_decimal_int {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::primitives::aliases::I256,
        #[allow(missing_docs)]
        pub decimals: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for log_named_decimal_int {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Int<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_decimal_int(string,int256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                93u8,
                166u8,
                206u8,
                157u8,
                81u8,
                21u8,
                27u8,
                161u8,
                12u8,
                9u8,
                165u8,
                89u8,
                239u8,
                36u8,
                213u8,
                32u8,
                185u8,
                218u8,
                197u8,
                197u8,
                184u8,
                129u8,
                10u8,
                232u8,
                67u8,
                78u8,
                77u8,
                13u8,
                134u8,
                65u8,
                26u8,
                149u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    key: data.0,
                    val: data.1,
                    decimals: data.2,
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::Int<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.decimals),
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
        impl alloy_sol_types::private::IntoLogData for log_named_decimal_int {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_decimal_int> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_decimal_int) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_named_decimal_uint(string,uint256,uint256)` and selector `0xeb8ba43ced7537421946bd43e828b8b2b8428927aa8f801c13d934bf11aca57b`.
```solidity
event log_named_decimal_uint(string key, uint256 val, uint256 decimals);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_decimal_uint {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub decimals: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for log_named_decimal_uint {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_decimal_uint(string,uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                235u8,
                139u8,
                164u8,
                60u8,
                237u8,
                117u8,
                55u8,
                66u8,
                25u8,
                70u8,
                189u8,
                67u8,
                232u8,
                40u8,
                184u8,
                178u8,
                184u8,
                66u8,
                137u8,
                39u8,
                170u8,
                143u8,
                128u8,
                28u8,
                19u8,
                217u8,
                52u8,
                191u8,
                17u8,
                172u8,
                165u8,
                123u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    key: data.0,
                    val: data.1,
                    decimals: data.2,
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.decimals),
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
        impl alloy_sol_types::private::IntoLogData for log_named_decimal_uint {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_decimal_uint> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_decimal_uint) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_named_int(string,int256)` and selector `0x2fe632779174374378442a8e978bccfbdcc1d6b2b0d81f7e8eb776ab2286f168`.
```solidity
event log_named_int(string key, int256 val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_int {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::primitives::aliases::I256,
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
        impl alloy_sol_types::SolEvent for log_named_int {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Int<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_int(string,int256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                47u8,
                230u8,
                50u8,
                119u8,
                145u8,
                116u8,
                55u8,
                67u8,
                120u8,
                68u8,
                42u8,
                142u8,
                151u8,
                139u8,
                204u8,
                251u8,
                220u8,
                193u8,
                214u8,
                178u8,
                176u8,
                216u8,
                31u8,
                126u8,
                142u8,
                183u8,
                118u8,
                171u8,
                34u8,
                134u8,
                241u8,
                104u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { key: data.0, val: data.1 }
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::Int<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
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
        impl alloy_sol_types::private::IntoLogData for log_named_int {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_int> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_int) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_named_string(string,string)` and selector `0x280f4446b28a1372417dda658d30b95b2992b12ac9c7f378535f29a97acf3583`.
```solidity
event log_named_string(string key, string val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_string {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::String,
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
        impl alloy_sol_types::SolEvent for log_named_string {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::String,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_string(string,string)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                40u8,
                15u8,
                68u8,
                70u8,
                178u8,
                138u8,
                19u8,
                114u8,
                65u8,
                125u8,
                218u8,
                101u8,
                141u8,
                48u8,
                185u8,
                91u8,
                41u8,
                146u8,
                177u8,
                42u8,
                201u8,
                199u8,
                243u8,
                120u8,
                83u8,
                95u8,
                41u8,
                169u8,
                122u8,
                207u8,
                53u8,
                131u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { key: data.0, val: data.1 }
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.val,
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
        impl alloy_sol_types::private::IntoLogData for log_named_string {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_string> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_string) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_named_uint(string,uint256)` and selector `0xb2de2fbe801a0df6c0cbddfd448ba3c41d48a040ca35c56c8196ef0fcae721a8`.
```solidity
event log_named_uint(string key, uint256 val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_uint {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for log_named_uint {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_uint(string,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                178u8,
                222u8,
                47u8,
                190u8,
                128u8,
                26u8,
                13u8,
                246u8,
                192u8,
                203u8,
                221u8,
                253u8,
                68u8,
                139u8,
                163u8,
                196u8,
                29u8,
                72u8,
                160u8,
                64u8,
                202u8,
                53u8,
                197u8,
                108u8,
                129u8,
                150u8,
                239u8,
                15u8,
                202u8,
                231u8,
                33u8,
                168u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { key: data.0, val: data.1 }
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
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
        impl alloy_sol_types::private::IntoLogData for log_named_uint {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_uint> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_uint) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_string(string)` and selector `0x0b2e13ff20ac7b474198655583edf70dedd2c1dc980e329c4fbb2fc0748b796b`.
```solidity
event log_string(string);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_string {
        #[allow(missing_docs)]
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log_string {
            type DataTuple<'a> = (alloy::sol_types::sol_data::String,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_string(string)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                11u8,
                46u8,
                19u8,
                255u8,
                32u8,
                172u8,
                123u8,
                71u8,
                65u8,
                152u8,
                101u8,
                85u8,
                131u8,
                237u8,
                247u8,
                13u8,
                237u8,
                210u8,
                193u8,
                220u8,
                152u8,
                14u8,
                50u8,
                156u8,
                79u8,
                187u8,
                47u8,
                192u8,
                116u8,
                139u8,
                121u8,
                107u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { _0: data.0 }
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self._0,
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
        impl alloy_sol_types::private::IntoLogData for log_string {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_string> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_string) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_uint(uint256)` and selector `0x2cab9790510fd8bdfbd2115288db33fec66691d476efc5427cfd4c0969301755`.
```solidity
event log_uint(uint256);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_uint {
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log_uint {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_uint(uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                44u8,
                171u8,
                151u8,
                144u8,
                81u8,
                15u8,
                216u8,
                189u8,
                251u8,
                210u8,
                17u8,
                82u8,
                136u8,
                219u8,
                51u8,
                254u8,
                198u8,
                102u8,
                145u8,
                212u8,
                118u8,
                239u8,
                197u8,
                66u8,
                124u8,
                253u8,
                76u8,
                9u8,
                105u8,
                48u8,
                23u8,
                85u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { _0: data.0 }
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
                    > as alloy_sol_types::SolType>::tokenize(&self._0),
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
        impl alloy_sol_types::private::IntoLogData for log_uint {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_uint> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_uint) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `logs(bytes)` and selector `0xe7950ede0394b9f2ce4a5a1bf5a7e1852411f7e6661b4308c913c4bfd11027e4`.
```solidity
event logs(bytes);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct logs {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Bytes,
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
        impl alloy_sol_types::SolEvent for logs {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "logs(bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                231u8,
                149u8,
                14u8,
                222u8,
                3u8,
                148u8,
                185u8,
                242u8,
                206u8,
                74u8,
                90u8,
                27u8,
                245u8,
                167u8,
                225u8,
                133u8,
                36u8,
                17u8,
                247u8,
                230u8,
                102u8,
                27u8,
                67u8,
                8u8,
                201u8,
                19u8,
                196u8,
                191u8,
                209u8,
                16u8,
                39u8,
                228u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { _0: data.0 }
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
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self._0,
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
        impl alloy_sol_types::private::IntoLogData for logs {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&logs> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &logs) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Function with signature `IS_TEST()` and selector `0xfa7626d4`.
```solidity
function IS_TEST() external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct IS_TESTCall {}
    ///Container type for the return parameters of the [`IS_TEST()`](IS_TESTCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct IS_TESTReturn {
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
            impl ::core::convert::From<IS_TESTCall> for UnderlyingRustTuple<'_> {
                fn from(value: IS_TESTCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for IS_TESTCall {
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
            impl ::core::convert::From<IS_TESTReturn> for UnderlyingRustTuple<'_> {
                fn from(value: IS_TESTReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for IS_TESTReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for IS_TESTCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = IS_TESTReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "IS_TEST()";
            const SELECTOR: [u8; 4] = [250u8, 118u8, 38u8, 212u8];
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
    /**Function with signature `excludeArtifacts()` and selector `0xb5508aa9`.
```solidity
function excludeArtifacts() external view returns (string[] memory excludedArtifacts_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct excludeArtifactsCall {}
    ///Container type for the return parameters of the [`excludeArtifacts()`](excludeArtifactsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct excludeArtifactsReturn {
        #[allow(missing_docs)]
        pub excludedArtifacts_: alloy::sol_types::private::Vec<
            alloy::sol_types::private::String,
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
            impl ::core::convert::From<excludeArtifactsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: excludeArtifactsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for excludeArtifactsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::String>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::String>,
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
            impl ::core::convert::From<excludeArtifactsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: excludeArtifactsReturn) -> Self {
                    (value.excludedArtifacts_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for excludeArtifactsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        excludedArtifacts_: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for excludeArtifactsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = excludeArtifactsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::String>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "excludeArtifacts()";
            const SELECTOR: [u8; 4] = [181u8, 80u8, 138u8, 169u8];
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
    /**Function with signature `excludeContracts()` and selector `0xe20c9f71`.
```solidity
function excludeContracts() external view returns (address[] memory excludedContracts_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct excludeContractsCall {}
    ///Container type for the return parameters of the [`excludeContracts()`](excludeContractsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct excludeContractsReturn {
        #[allow(missing_docs)]
        pub excludedContracts_: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
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
            impl ::core::convert::From<excludeContractsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: excludeContractsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for excludeContractsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
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
            impl ::core::convert::From<excludeContractsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: excludeContractsReturn) -> Self {
                    (value.excludedContracts_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for excludeContractsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        excludedContracts_: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for excludeContractsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = excludeContractsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "excludeContracts()";
            const SELECTOR: [u8; 4] = [226u8, 12u8, 159u8, 113u8];
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
    /**Function with signature `excludeSenders()` and selector `0x1ed7831c`.
```solidity
function excludeSenders() external view returns (address[] memory excludedSenders_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct excludeSendersCall {}
    ///Container type for the return parameters of the [`excludeSenders()`](excludeSendersCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct excludeSendersReturn {
        #[allow(missing_docs)]
        pub excludedSenders_: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
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
            impl ::core::convert::From<excludeSendersCall> for UnderlyingRustTuple<'_> {
                fn from(value: excludeSendersCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for excludeSendersCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
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
            impl ::core::convert::From<excludeSendersReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: excludeSendersReturn) -> Self {
                    (value.excludedSenders_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for excludeSendersReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { excludedSenders_: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for excludeSendersCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = excludeSendersReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "excludeSenders()";
            const SELECTOR: [u8; 4] = [30u8, 215u8, 131u8, 28u8];
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
    /**Function with signature `failed()` and selector `0xba414fa6`.
```solidity
function failed() external returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct failedCall {}
    ///Container type for the return parameters of the [`failed()`](failedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct failedReturn {
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
            impl ::core::convert::From<failedCall> for UnderlyingRustTuple<'_> {
                fn from(value: failedCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for failedCall {
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
            impl ::core::convert::From<failedReturn> for UnderlyingRustTuple<'_> {
                fn from(value: failedReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for failedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for failedCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = failedReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "failed()";
            const SELECTOR: [u8; 4] = [186u8, 65u8, 79u8, 166u8];
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
    /**Function with signature `getBlockHeights(string,uint256,uint256)` and selector `0xfad06b8f`.
```solidity
function getBlockHeights(string memory chainName, uint256 from, uint256 to) external view returns (uint256[] memory elements);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getBlockHeightsCall {
        #[allow(missing_docs)]
        pub chainName: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub from: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub to: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`getBlockHeights(string,uint256,uint256)`](getBlockHeightsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getBlockHeightsReturn {
        #[allow(missing_docs)]
        pub elements: alloy::sol_types::private::Vec<
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::String,
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
            impl ::core::convert::From<getBlockHeightsCall> for UnderlyingRustTuple<'_> {
                fn from(value: getBlockHeightsCall) -> Self {
                    (value.chainName, value.from, value.to)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getBlockHeightsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        chainName: tuple.0,
                        from: tuple.1,
                        to: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<getBlockHeightsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getBlockHeightsReturn) -> Self {
                    (value.elements,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getBlockHeightsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { elements: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getBlockHeightsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getBlockHeightsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getBlockHeights(string,uint256,uint256)";
            const SELECTOR: [u8; 4] = [250u8, 208u8, 107u8, 143u8];
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
                        &self.chainName,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.from),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.to),
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
    /**Function with signature `getDigestLes(string,uint256,uint256)` and selector `0x44badbb6`.
```solidity
function getDigestLes(string memory chainName, uint256 from, uint256 to) external view returns (bytes32[] memory elements);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getDigestLesCall {
        #[allow(missing_docs)]
        pub chainName: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub from: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub to: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`getDigestLes(string,uint256,uint256)`](getDigestLesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getDigestLesReturn {
        #[allow(missing_docs)]
        pub elements: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<32>,
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::String,
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
            impl ::core::convert::From<getDigestLesCall> for UnderlyingRustTuple<'_> {
                fn from(value: getDigestLesCall) -> Self {
                    (value.chainName, value.from, value.to)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getDigestLesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        chainName: tuple.0,
                        from: tuple.1,
                        to: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<getDigestLesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getDigestLesReturn) -> Self {
                    (value.elements,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getDigestLesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { elements: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getDigestLesCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getDigestLesReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getDigestLes(string,uint256,uint256)";
            const SELECTOR: [u8; 4] = [68u8, 186u8, 219u8, 182u8];
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
                        &self.chainName,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.from),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.to),
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
    /**Function with signature `getHeaderHexes(string,uint256,uint256)` and selector `0x0813852a`.
```solidity
function getHeaderHexes(string memory chainName, uint256 from, uint256 to) external view returns (bytes[] memory elements);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getHeaderHexesCall {
        #[allow(missing_docs)]
        pub chainName: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub from: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub to: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`getHeaderHexes(string,uint256,uint256)`](getHeaderHexesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getHeaderHexesReturn {
        #[allow(missing_docs)]
        pub elements: alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
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
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::String,
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
            impl ::core::convert::From<getHeaderHexesCall> for UnderlyingRustTuple<'_> {
                fn from(value: getHeaderHexesCall) -> Self {
                    (value.chainName, value.from, value.to)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getHeaderHexesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        chainName: tuple.0,
                        from: tuple.1,
                        to: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Bytes>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
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
            impl ::core::convert::From<getHeaderHexesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getHeaderHexesReturn) -> Self {
                    (value.elements,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getHeaderHexesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { elements: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getHeaderHexesCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getHeaderHexesReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Bytes>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getHeaderHexes(string,uint256,uint256)";
            const SELECTOR: [u8; 4] = [8u8, 19u8, 133u8, 42u8];
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
                        &self.chainName,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.from),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.to),
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
    /**Function with signature `getHeaders(string,uint256,uint256)` and selector `0x1c0da81f`.
```solidity
function getHeaders(string memory chainName, uint256 from, uint256 to) external view returns (bytes memory headers);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getHeadersCall {
        #[allow(missing_docs)]
        pub chainName: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub from: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub to: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`getHeaders(string,uint256,uint256)`](getHeadersCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getHeadersReturn {
        #[allow(missing_docs)]
        pub headers: alloy::sol_types::private::Bytes,
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
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::String,
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
            impl ::core::convert::From<getHeadersCall> for UnderlyingRustTuple<'_> {
                fn from(value: getHeadersCall) -> Self {
                    (value.chainName, value.from, value.to)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getHeadersCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        chainName: tuple.0,
                        from: tuple.1,
                        to: tuple.2,
                    }
                }
            }
        }
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
            impl ::core::convert::From<getHeadersReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getHeadersReturn) -> Self {
                    (value.headers,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getHeadersReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { headers: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getHeadersCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getHeadersReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getHeaders(string,uint256,uint256)";
            const SELECTOR: [u8; 4] = [28u8, 13u8, 168u8, 31u8];
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
                        &self.chainName,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.from),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.to),
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
    /**Function with signature `targetArtifactSelectors()` and selector `0x66d9a9a0`.
```solidity
function targetArtifactSelectors() external view returns (StdInvariant.FuzzSelector[] memory targetedArtifactSelectors_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetArtifactSelectorsCall {}
    ///Container type for the return parameters of the [`targetArtifactSelectors()`](targetArtifactSelectorsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetArtifactSelectorsReturn {
        #[allow(missing_docs)]
        pub targetedArtifactSelectors_: alloy::sol_types::private::Vec<
            <StdInvariant::FuzzSelector as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<targetArtifactSelectorsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: targetArtifactSelectorsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for targetArtifactSelectorsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<StdInvariant::FuzzSelector>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <StdInvariant::FuzzSelector as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<targetArtifactSelectorsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: targetArtifactSelectorsReturn) -> Self {
                    (value.targetedArtifactSelectors_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for targetArtifactSelectorsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        targetedArtifactSelectors_: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for targetArtifactSelectorsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = targetArtifactSelectorsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<StdInvariant::FuzzSelector>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "targetArtifactSelectors()";
            const SELECTOR: [u8; 4] = [102u8, 217u8, 169u8, 160u8];
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
    /**Function with signature `targetArtifacts()` and selector `0x85226c81`.
```solidity
function targetArtifacts() external view returns (string[] memory targetedArtifacts_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetArtifactsCall {}
    ///Container type for the return parameters of the [`targetArtifacts()`](targetArtifactsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetArtifactsReturn {
        #[allow(missing_docs)]
        pub targetedArtifacts_: alloy::sol_types::private::Vec<
            alloy::sol_types::private::String,
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
            impl ::core::convert::From<targetArtifactsCall> for UnderlyingRustTuple<'_> {
                fn from(value: targetArtifactsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for targetArtifactsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::String>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::String>,
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
            impl ::core::convert::From<targetArtifactsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: targetArtifactsReturn) -> Self {
                    (value.targetedArtifacts_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for targetArtifactsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        targetedArtifacts_: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for targetArtifactsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = targetArtifactsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::String>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "targetArtifacts()";
            const SELECTOR: [u8; 4] = [133u8, 34u8, 108u8, 129u8];
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
    /**Function with signature `targetContracts()` and selector `0x3f7286f4`.
```solidity
function targetContracts() external view returns (address[] memory targetedContracts_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetContractsCall {}
    ///Container type for the return parameters of the [`targetContracts()`](targetContractsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetContractsReturn {
        #[allow(missing_docs)]
        pub targetedContracts_: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
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
            impl ::core::convert::From<targetContractsCall> for UnderlyingRustTuple<'_> {
                fn from(value: targetContractsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for targetContractsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
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
            impl ::core::convert::From<targetContractsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: targetContractsReturn) -> Self {
                    (value.targetedContracts_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for targetContractsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        targetedContracts_: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for targetContractsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = targetContractsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "targetContracts()";
            const SELECTOR: [u8; 4] = [63u8, 114u8, 134u8, 244u8];
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
    /**Function with signature `targetInterfaces()` and selector `0x2ade3880`.
```solidity
function targetInterfaces() external view returns (StdInvariant.FuzzInterface[] memory targetedInterfaces_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetInterfacesCall {}
    ///Container type for the return parameters of the [`targetInterfaces()`](targetInterfacesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetInterfacesReturn {
        #[allow(missing_docs)]
        pub targetedInterfaces_: alloy::sol_types::private::Vec<
            <StdInvariant::FuzzInterface as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<targetInterfacesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: targetInterfacesCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for targetInterfacesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<StdInvariant::FuzzInterface>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <StdInvariant::FuzzInterface as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<targetInterfacesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: targetInterfacesReturn) -> Self {
                    (value.targetedInterfaces_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for targetInterfacesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        targetedInterfaces_: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for targetInterfacesCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = targetInterfacesReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<StdInvariant::FuzzInterface>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "targetInterfaces()";
            const SELECTOR: [u8; 4] = [42u8, 222u8, 56u8, 128u8];
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
    /**Function with signature `targetSelectors()` and selector `0x916a17c6`.
```solidity
function targetSelectors() external view returns (StdInvariant.FuzzSelector[] memory targetedSelectors_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetSelectorsCall {}
    ///Container type for the return parameters of the [`targetSelectors()`](targetSelectorsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetSelectorsReturn {
        #[allow(missing_docs)]
        pub targetedSelectors_: alloy::sol_types::private::Vec<
            <StdInvariant::FuzzSelector as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<targetSelectorsCall> for UnderlyingRustTuple<'_> {
                fn from(value: targetSelectorsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for targetSelectorsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<StdInvariant::FuzzSelector>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <StdInvariant::FuzzSelector as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<targetSelectorsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: targetSelectorsReturn) -> Self {
                    (value.targetedSelectors_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for targetSelectorsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        targetedSelectors_: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for targetSelectorsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = targetSelectorsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<StdInvariant::FuzzSelector>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "targetSelectors()";
            const SELECTOR: [u8; 4] = [145u8, 106u8, 23u8, 198u8];
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
    /**Function with signature `targetSenders()` and selector `0x3e5e3c23`.
```solidity
function targetSenders() external view returns (address[] memory targetedSenders_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetSendersCall {}
    ///Container type for the return parameters of the [`targetSenders()`](targetSendersCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetSendersReturn {
        #[allow(missing_docs)]
        pub targetedSenders_: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
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
            impl ::core::convert::From<targetSendersCall> for UnderlyingRustTuple<'_> {
                fn from(value: targetSendersCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for targetSendersCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
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
            impl ::core::convert::From<targetSendersReturn> for UnderlyingRustTuple<'_> {
                fn from(value: targetSendersReturn) -> Self {
                    (value.targetedSenders_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for targetSendersReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { targetedSenders_: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for targetSendersCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = targetSendersReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "targetSenders()";
            const SELECTOR: [u8; 4] = [62u8, 94u8, 60u8, 35u8];
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
    /**Function with signature `testIncorrectInputVectorSupplied()` and selector `0xef3e6812`.
```solidity
function testIncorrectInputVectorSupplied() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testIncorrectInputVectorSuppliedCall {}
    ///Container type for the return parameters of the [`testIncorrectInputVectorSupplied()`](testIncorrectInputVectorSuppliedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testIncorrectInputVectorSuppliedReturn {}
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
            impl ::core::convert::From<testIncorrectInputVectorSuppliedCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testIncorrectInputVectorSuppliedCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testIncorrectInputVectorSuppliedCall {
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
            impl ::core::convert::From<testIncorrectInputVectorSuppliedReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testIncorrectInputVectorSuppliedReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testIncorrectInputVectorSuppliedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testIncorrectInputVectorSuppliedCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testIncorrectInputVectorSuppliedReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testIncorrectInputVectorSupplied()";
            const SELECTOR: [u8; 4] = [239u8, 62u8, 104u8, 18u8];
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
    /**Function with signature `testIncorrectLocktimeSupplied()` and selector `0xc899d241`.
```solidity
function testIncorrectLocktimeSupplied() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testIncorrectLocktimeSuppliedCall {}
    ///Container type for the return parameters of the [`testIncorrectLocktimeSupplied()`](testIncorrectLocktimeSuppliedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testIncorrectLocktimeSuppliedReturn {}
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
            impl ::core::convert::From<testIncorrectLocktimeSuppliedCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testIncorrectLocktimeSuppliedCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testIncorrectLocktimeSuppliedCall {
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
            impl ::core::convert::From<testIncorrectLocktimeSuppliedReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testIncorrectLocktimeSuppliedReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testIncorrectLocktimeSuppliedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testIncorrectLocktimeSuppliedCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testIncorrectLocktimeSuppliedReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testIncorrectLocktimeSupplied()";
            const SELECTOR: [u8; 4] = [200u8, 153u8, 210u8, 65u8];
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
    /**Function with signature `testIncorrectOutputVectorSupplied()` and selector `0x5df0d076`.
```solidity
function testIncorrectOutputVectorSupplied() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testIncorrectOutputVectorSuppliedCall {}
    ///Container type for the return parameters of the [`testIncorrectOutputVectorSupplied()`](testIncorrectOutputVectorSuppliedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testIncorrectOutputVectorSuppliedReturn {}
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
            impl ::core::convert::From<testIncorrectOutputVectorSuppliedCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testIncorrectOutputVectorSuppliedCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testIncorrectOutputVectorSuppliedCall {
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
            impl ::core::convert::From<testIncorrectOutputVectorSuppliedReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testIncorrectOutputVectorSuppliedReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testIncorrectOutputVectorSuppliedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testIncorrectOutputVectorSuppliedCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testIncorrectOutputVectorSuppliedReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testIncorrectOutputVectorSupplied()";
            const SELECTOR: [u8; 4] = [93u8, 240u8, 208u8, 118u8];
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
    /**Function with signature `testIncorrectProofSupplied()` and selector `0xe89f8419`.
```solidity
function testIncorrectProofSupplied() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testIncorrectProofSuppliedCall {}
    ///Container type for the return parameters of the [`testIncorrectProofSupplied()`](testIncorrectProofSuppliedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testIncorrectProofSuppliedReturn {}
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
            impl ::core::convert::From<testIncorrectProofSuppliedCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testIncorrectProofSuppliedCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testIncorrectProofSuppliedCall {
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
            impl ::core::convert::From<testIncorrectProofSuppliedReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testIncorrectProofSuppliedReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testIncorrectProofSuppliedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testIncorrectProofSuppliedCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testIncorrectProofSuppliedReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testIncorrectProofSupplied()";
            const SELECTOR: [u8; 4] = [232u8, 159u8, 132u8, 25u8];
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
    /**Function with signature `testSuccessfullyVerify()` and selector `0xb52b2058`.
```solidity
function testSuccessfullyVerify() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testSuccessfullyVerifyCall {}
    ///Container type for the return parameters of the [`testSuccessfullyVerify()`](testSuccessfullyVerifyCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testSuccessfullyVerifyReturn {}
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
            impl ::core::convert::From<testSuccessfullyVerifyCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testSuccessfullyVerifyCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testSuccessfullyVerifyCall {
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
            impl ::core::convert::From<testSuccessfullyVerifyReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testSuccessfullyVerifyReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testSuccessfullyVerifyReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testSuccessfullyVerifyCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testSuccessfullyVerifyReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testSuccessfullyVerify()";
            const SELECTOR: [u8; 4] = [181u8, 43u8, 32u8, 88u8];
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
    ///Container for all the [`FullRelayWithVerifyThroughBitcoinTxTest`](self) function calls.
    pub enum FullRelayWithVerifyThroughBitcoinTxTestCalls {
        #[allow(missing_docs)]
        IS_TEST(IS_TESTCall),
        #[allow(missing_docs)]
        excludeArtifacts(excludeArtifactsCall),
        #[allow(missing_docs)]
        excludeContracts(excludeContractsCall),
        #[allow(missing_docs)]
        excludeSenders(excludeSendersCall),
        #[allow(missing_docs)]
        failed(failedCall),
        #[allow(missing_docs)]
        getBlockHeights(getBlockHeightsCall),
        #[allow(missing_docs)]
        getDigestLes(getDigestLesCall),
        #[allow(missing_docs)]
        getHeaderHexes(getHeaderHexesCall),
        #[allow(missing_docs)]
        getHeaders(getHeadersCall),
        #[allow(missing_docs)]
        targetArtifactSelectors(targetArtifactSelectorsCall),
        #[allow(missing_docs)]
        targetArtifacts(targetArtifactsCall),
        #[allow(missing_docs)]
        targetContracts(targetContractsCall),
        #[allow(missing_docs)]
        targetInterfaces(targetInterfacesCall),
        #[allow(missing_docs)]
        targetSelectors(targetSelectorsCall),
        #[allow(missing_docs)]
        targetSenders(targetSendersCall),
        #[allow(missing_docs)]
        testIncorrectInputVectorSupplied(testIncorrectInputVectorSuppliedCall),
        #[allow(missing_docs)]
        testIncorrectLocktimeSupplied(testIncorrectLocktimeSuppliedCall),
        #[allow(missing_docs)]
        testIncorrectOutputVectorSupplied(testIncorrectOutputVectorSuppliedCall),
        #[allow(missing_docs)]
        testIncorrectProofSupplied(testIncorrectProofSuppliedCall),
        #[allow(missing_docs)]
        testSuccessfullyVerify(testSuccessfullyVerifyCall),
    }
    #[automatically_derived]
    impl FullRelayWithVerifyThroughBitcoinTxTestCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [8u8, 19u8, 133u8, 42u8],
            [28u8, 13u8, 168u8, 31u8],
            [30u8, 215u8, 131u8, 28u8],
            [42u8, 222u8, 56u8, 128u8],
            [62u8, 94u8, 60u8, 35u8],
            [63u8, 114u8, 134u8, 244u8],
            [68u8, 186u8, 219u8, 182u8],
            [93u8, 240u8, 208u8, 118u8],
            [102u8, 217u8, 169u8, 160u8],
            [133u8, 34u8, 108u8, 129u8],
            [145u8, 106u8, 23u8, 198u8],
            [181u8, 43u8, 32u8, 88u8],
            [181u8, 80u8, 138u8, 169u8],
            [186u8, 65u8, 79u8, 166u8],
            [200u8, 153u8, 210u8, 65u8],
            [226u8, 12u8, 159u8, 113u8],
            [232u8, 159u8, 132u8, 25u8],
            [239u8, 62u8, 104u8, 18u8],
            [250u8, 118u8, 38u8, 212u8],
            [250u8, 208u8, 107u8, 143u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for FullRelayWithVerifyThroughBitcoinTxTestCalls {
        const NAME: &'static str = "FullRelayWithVerifyThroughBitcoinTxTestCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 20usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::IS_TEST(_) => <IS_TESTCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::excludeArtifacts(_) => {
                    <excludeArtifactsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::excludeContracts(_) => {
                    <excludeContractsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::excludeSenders(_) => {
                    <excludeSendersCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::failed(_) => <failedCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::getBlockHeights(_) => {
                    <getBlockHeightsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getDigestLes(_) => {
                    <getDigestLesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getHeaderHexes(_) => {
                    <getHeaderHexesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getHeaders(_) => {
                    <getHeadersCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::targetArtifactSelectors(_) => {
                    <targetArtifactSelectorsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::targetArtifacts(_) => {
                    <targetArtifactsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::targetContracts(_) => {
                    <targetContractsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::targetInterfaces(_) => {
                    <targetInterfacesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::targetSelectors(_) => {
                    <targetSelectorsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::targetSenders(_) => {
                    <targetSendersCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testIncorrectInputVectorSupplied(_) => {
                    <testIncorrectInputVectorSuppliedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testIncorrectLocktimeSupplied(_) => {
                    <testIncorrectLocktimeSuppliedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testIncorrectOutputVectorSupplied(_) => {
                    <testIncorrectOutputVectorSuppliedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testIncorrectProofSupplied(_) => {
                    <testIncorrectProofSuppliedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testSuccessfullyVerify(_) => {
                    <testSuccessfullyVerifyCall as alloy_sol_types::SolCall>::SELECTOR
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
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
                bool,
            ) -> alloy_sol_types::Result<
                FullRelayWithVerifyThroughBitcoinTxTestCalls,
            >] = &[
                {
                    fn getHeaderHexes(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<
                        FullRelayWithVerifyThroughBitcoinTxTestCalls,
                    > {
                        <getHeaderHexesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                FullRelayWithVerifyThroughBitcoinTxTestCalls::getHeaderHexes,
                            )
                    }
                    getHeaderHexes
                },
                {
                    fn getHeaders(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<
                        FullRelayWithVerifyThroughBitcoinTxTestCalls,
                    > {
                        <getHeadersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                FullRelayWithVerifyThroughBitcoinTxTestCalls::getHeaders,
                            )
                    }
                    getHeaders
                },
                {
                    fn excludeSenders(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<
                        FullRelayWithVerifyThroughBitcoinTxTestCalls,
                    > {
                        <excludeSendersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                FullRelayWithVerifyThroughBitcoinTxTestCalls::excludeSenders,
                            )
                    }
                    excludeSenders
                },
                {
                    fn targetInterfaces(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<
                        FullRelayWithVerifyThroughBitcoinTxTestCalls,
                    > {
                        <targetInterfacesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                FullRelayWithVerifyThroughBitcoinTxTestCalls::targetInterfaces,
                            )
                    }
                    targetInterfaces
                },
                {
                    fn targetSenders(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<
                        FullRelayWithVerifyThroughBitcoinTxTestCalls,
                    > {
                        <targetSendersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                FullRelayWithVerifyThroughBitcoinTxTestCalls::targetSenders,
                            )
                    }
                    targetSenders
                },
                {
                    fn targetContracts(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<
                        FullRelayWithVerifyThroughBitcoinTxTestCalls,
                    > {
                        <targetContractsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                FullRelayWithVerifyThroughBitcoinTxTestCalls::targetContracts,
                            )
                    }
                    targetContracts
                },
                {
                    fn getDigestLes(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<
                        FullRelayWithVerifyThroughBitcoinTxTestCalls,
                    > {
                        <getDigestLesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                FullRelayWithVerifyThroughBitcoinTxTestCalls::getDigestLes,
                            )
                    }
                    getDigestLes
                },
                {
                    fn testIncorrectOutputVectorSupplied(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<
                        FullRelayWithVerifyThroughBitcoinTxTestCalls,
                    > {
                        <testIncorrectOutputVectorSuppliedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                FullRelayWithVerifyThroughBitcoinTxTestCalls::testIncorrectOutputVectorSupplied,
                            )
                    }
                    testIncorrectOutputVectorSupplied
                },
                {
                    fn targetArtifactSelectors(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<
                        FullRelayWithVerifyThroughBitcoinTxTestCalls,
                    > {
                        <targetArtifactSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                FullRelayWithVerifyThroughBitcoinTxTestCalls::targetArtifactSelectors,
                            )
                    }
                    targetArtifactSelectors
                },
                {
                    fn targetArtifacts(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<
                        FullRelayWithVerifyThroughBitcoinTxTestCalls,
                    > {
                        <targetArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                FullRelayWithVerifyThroughBitcoinTxTestCalls::targetArtifacts,
                            )
                    }
                    targetArtifacts
                },
                {
                    fn targetSelectors(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<
                        FullRelayWithVerifyThroughBitcoinTxTestCalls,
                    > {
                        <targetSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                FullRelayWithVerifyThroughBitcoinTxTestCalls::targetSelectors,
                            )
                    }
                    targetSelectors
                },
                {
                    fn testSuccessfullyVerify(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<
                        FullRelayWithVerifyThroughBitcoinTxTestCalls,
                    > {
                        <testSuccessfullyVerifyCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                FullRelayWithVerifyThroughBitcoinTxTestCalls::testSuccessfullyVerify,
                            )
                    }
                    testSuccessfullyVerify
                },
                {
                    fn excludeArtifacts(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<
                        FullRelayWithVerifyThroughBitcoinTxTestCalls,
                    > {
                        <excludeArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                FullRelayWithVerifyThroughBitcoinTxTestCalls::excludeArtifacts,
                            )
                    }
                    excludeArtifacts
                },
                {
                    fn failed(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<
                        FullRelayWithVerifyThroughBitcoinTxTestCalls,
                    > {
                        <failedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(FullRelayWithVerifyThroughBitcoinTxTestCalls::failed)
                    }
                    failed
                },
                {
                    fn testIncorrectLocktimeSupplied(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<
                        FullRelayWithVerifyThroughBitcoinTxTestCalls,
                    > {
                        <testIncorrectLocktimeSuppliedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                FullRelayWithVerifyThroughBitcoinTxTestCalls::testIncorrectLocktimeSupplied,
                            )
                    }
                    testIncorrectLocktimeSupplied
                },
                {
                    fn excludeContracts(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<
                        FullRelayWithVerifyThroughBitcoinTxTestCalls,
                    > {
                        <excludeContractsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                FullRelayWithVerifyThroughBitcoinTxTestCalls::excludeContracts,
                            )
                    }
                    excludeContracts
                },
                {
                    fn testIncorrectProofSupplied(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<
                        FullRelayWithVerifyThroughBitcoinTxTestCalls,
                    > {
                        <testIncorrectProofSuppliedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                FullRelayWithVerifyThroughBitcoinTxTestCalls::testIncorrectProofSupplied,
                            )
                    }
                    testIncorrectProofSupplied
                },
                {
                    fn testIncorrectInputVectorSupplied(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<
                        FullRelayWithVerifyThroughBitcoinTxTestCalls,
                    > {
                        <testIncorrectInputVectorSuppliedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                FullRelayWithVerifyThroughBitcoinTxTestCalls::testIncorrectInputVectorSupplied,
                            )
                    }
                    testIncorrectInputVectorSupplied
                },
                {
                    fn IS_TEST(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<
                        FullRelayWithVerifyThroughBitcoinTxTestCalls,
                    > {
                        <IS_TESTCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(FullRelayWithVerifyThroughBitcoinTxTestCalls::IS_TEST)
                    }
                    IS_TEST
                },
                {
                    fn getBlockHeights(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<
                        FullRelayWithVerifyThroughBitcoinTxTestCalls,
                    > {
                        <getBlockHeightsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                FullRelayWithVerifyThroughBitcoinTxTestCalls::getBlockHeights,
                            )
                    }
                    getBlockHeights
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
            DECODE_SHIMS[idx](data, validate)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::IS_TEST(inner) => {
                    <IS_TESTCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::excludeArtifacts(inner) => {
                    <excludeArtifactsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::excludeContracts(inner) => {
                    <excludeContractsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::excludeSenders(inner) => {
                    <excludeSendersCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::failed(inner) => {
                    <failedCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::getBlockHeights(inner) => {
                    <getBlockHeightsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getDigestLes(inner) => {
                    <getDigestLesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getHeaderHexes(inner) => {
                    <getHeaderHexesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getHeaders(inner) => {
                    <getHeadersCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::targetArtifactSelectors(inner) => {
                    <targetArtifactSelectorsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::targetArtifacts(inner) => {
                    <targetArtifactsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::targetContracts(inner) => {
                    <targetContractsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::targetInterfaces(inner) => {
                    <targetInterfacesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::targetSelectors(inner) => {
                    <targetSelectorsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::targetSenders(inner) => {
                    <targetSendersCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testIncorrectInputVectorSupplied(inner) => {
                    <testIncorrectInputVectorSuppliedCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testIncorrectLocktimeSupplied(inner) => {
                    <testIncorrectLocktimeSuppliedCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testIncorrectOutputVectorSupplied(inner) => {
                    <testIncorrectOutputVectorSuppliedCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testIncorrectProofSupplied(inner) => {
                    <testIncorrectProofSuppliedCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testSuccessfullyVerify(inner) => {
                    <testSuccessfullyVerifyCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::IS_TEST(inner) => {
                    <IS_TESTCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::excludeArtifacts(inner) => {
                    <excludeArtifactsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::excludeContracts(inner) => {
                    <excludeContractsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::excludeSenders(inner) => {
                    <excludeSendersCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::failed(inner) => {
                    <failedCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::getBlockHeights(inner) => {
                    <getBlockHeightsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getDigestLes(inner) => {
                    <getDigestLesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getHeaderHexes(inner) => {
                    <getHeaderHexesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getHeaders(inner) => {
                    <getHeadersCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::targetArtifactSelectors(inner) => {
                    <targetArtifactSelectorsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::targetArtifacts(inner) => {
                    <targetArtifactsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::targetContracts(inner) => {
                    <targetContractsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::targetInterfaces(inner) => {
                    <targetInterfacesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::targetSelectors(inner) => {
                    <targetSelectorsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::targetSenders(inner) => {
                    <targetSendersCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testIncorrectInputVectorSupplied(inner) => {
                    <testIncorrectInputVectorSuppliedCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testIncorrectLocktimeSupplied(inner) => {
                    <testIncorrectLocktimeSuppliedCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testIncorrectOutputVectorSupplied(inner) => {
                    <testIncorrectOutputVectorSuppliedCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testIncorrectProofSupplied(inner) => {
                    <testIncorrectProofSuppliedCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testSuccessfullyVerify(inner) => {
                    <testSuccessfullyVerifyCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`FullRelayWithVerifyThroughBitcoinTxTest`](self) events.
    pub enum FullRelayWithVerifyThroughBitcoinTxTestEvents {
        #[allow(missing_docs)]
        log(log),
        #[allow(missing_docs)]
        log_address(log_address),
        #[allow(missing_docs)]
        log_array_0(log_array_0),
        #[allow(missing_docs)]
        log_array_1(log_array_1),
        #[allow(missing_docs)]
        log_array_2(log_array_2),
        #[allow(missing_docs)]
        log_bytes(log_bytes),
        #[allow(missing_docs)]
        log_bytes32(log_bytes32),
        #[allow(missing_docs)]
        log_int(log_int),
        #[allow(missing_docs)]
        log_named_address(log_named_address),
        #[allow(missing_docs)]
        log_named_array_0(log_named_array_0),
        #[allow(missing_docs)]
        log_named_array_1(log_named_array_1),
        #[allow(missing_docs)]
        log_named_array_2(log_named_array_2),
        #[allow(missing_docs)]
        log_named_bytes(log_named_bytes),
        #[allow(missing_docs)]
        log_named_bytes32(log_named_bytes32),
        #[allow(missing_docs)]
        log_named_decimal_int(log_named_decimal_int),
        #[allow(missing_docs)]
        log_named_decimal_uint(log_named_decimal_uint),
        #[allow(missing_docs)]
        log_named_int(log_named_int),
        #[allow(missing_docs)]
        log_named_string(log_named_string),
        #[allow(missing_docs)]
        log_named_uint(log_named_uint),
        #[allow(missing_docs)]
        log_string(log_string),
        #[allow(missing_docs)]
        log_uint(log_uint),
        #[allow(missing_docs)]
        logs(logs),
    }
    #[automatically_derived]
    impl FullRelayWithVerifyThroughBitcoinTxTestEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                0u8,
                170u8,
                163u8,
                156u8,
                159u8,
                251u8,
                95u8,
                86u8,
                122u8,
                69u8,
                52u8,
                56u8,
                12u8,
                115u8,
                112u8,
                117u8,
                112u8,
                46u8,
                31u8,
                127u8,
                20u8,
                16u8,
                127u8,
                201u8,
                83u8,
                40u8,
                227u8,
                181u8,
                108u8,
                3u8,
                37u8,
                251u8,
            ],
            [
                11u8,
                46u8,
                19u8,
                255u8,
                32u8,
                172u8,
                123u8,
                71u8,
                65u8,
                152u8,
                101u8,
                85u8,
                131u8,
                237u8,
                247u8,
                13u8,
                237u8,
                210u8,
                193u8,
                220u8,
                152u8,
                14u8,
                50u8,
                156u8,
                79u8,
                187u8,
                47u8,
                192u8,
                116u8,
                139u8,
                121u8,
                107u8,
            ],
            [
                14u8,
                181u8,
                213u8,
                38u8,
                36u8,
                200u8,
                210u8,
                138u8,
                218u8,
                159u8,
                197u8,
                90u8,
                140u8,
                80u8,
                46u8,
                213u8,
                170u8,
                63u8,
                190u8,
                47u8,
                182u8,
                233u8,
                27u8,
                113u8,
                181u8,
                243u8,
                118u8,
                136u8,
                43u8,
                29u8,
                47u8,
                184u8,
            ],
            [
                35u8,
                182u8,
                42u8,
                208u8,
                88u8,
                77u8,
                36u8,
                167u8,
                95u8,
                11u8,
                243u8,
                86u8,
                3u8,
                145u8,
                239u8,
                86u8,
                89u8,
                236u8,
                109u8,
                177u8,
                38u8,
                156u8,
                86u8,
                225u8,
                26u8,
                162u8,
                65u8,
                214u8,
                55u8,
                241u8,
                155u8,
                32u8,
            ],
            [
                40u8,
                15u8,
                68u8,
                70u8,
                178u8,
                138u8,
                19u8,
                114u8,
                65u8,
                125u8,
                218u8,
                101u8,
                141u8,
                48u8,
                185u8,
                91u8,
                41u8,
                146u8,
                177u8,
                42u8,
                201u8,
                199u8,
                243u8,
                120u8,
                83u8,
                95u8,
                41u8,
                169u8,
                122u8,
                207u8,
                53u8,
                131u8,
            ],
            [
                44u8,
                171u8,
                151u8,
                144u8,
                81u8,
                15u8,
                216u8,
                189u8,
                251u8,
                210u8,
                17u8,
                82u8,
                136u8,
                219u8,
                51u8,
                254u8,
                198u8,
                102u8,
                145u8,
                212u8,
                118u8,
                239u8,
                197u8,
                66u8,
                124u8,
                253u8,
                76u8,
                9u8,
                105u8,
                48u8,
                23u8,
                85u8,
            ],
            [
                47u8,
                230u8,
                50u8,
                119u8,
                145u8,
                116u8,
                55u8,
                67u8,
                120u8,
                68u8,
                42u8,
                142u8,
                151u8,
                139u8,
                204u8,
                251u8,
                220u8,
                193u8,
                214u8,
                178u8,
                176u8,
                216u8,
                31u8,
                126u8,
                142u8,
                183u8,
                118u8,
                171u8,
                34u8,
                134u8,
                241u8,
                104u8,
            ],
            [
                59u8,
                207u8,
                178u8,
                174u8,
                46u8,
                141u8,
                19u8,
                45u8,
                209u8,
                252u8,
                231u8,
                207u8,
                39u8,
                138u8,
                154u8,
                25u8,
                117u8,
                106u8,
                159u8,
                206u8,
                171u8,
                228u8,
                112u8,
                223u8,
                59u8,
                218u8,
                187u8,
                75u8,
                197u8,
                119u8,
                209u8,
                189u8,
            ],
            [
                64u8,
                225u8,
                132u8,
                15u8,
                87u8,
                105u8,
                7u8,
                61u8,
                97u8,
                189u8,
                1u8,
                55u8,
                45u8,
                155u8,
                117u8,
                186u8,
                169u8,
                132u8,
                45u8,
                86u8,
                41u8,
                160u8,
                201u8,
                159u8,
                241u8,
                3u8,
                190u8,
                17u8,
                120u8,
                168u8,
                233u8,
                226u8,
            ],
            [
                65u8,
                48u8,
                79u8,
                172u8,
                217u8,
                50u8,
                61u8,
                117u8,
                177u8,
                27u8,
                205u8,
                214u8,
                9u8,
                203u8,
                56u8,
                239u8,
                255u8,
                253u8,
                176u8,
                87u8,
                16u8,
                247u8,
                202u8,
                240u8,
                233u8,
                177u8,
                108u8,
                109u8,
                157u8,
                112u8,
                159u8,
                80u8,
            ],
            [
                93u8,
                166u8,
                206u8,
                157u8,
                81u8,
                21u8,
                27u8,
                161u8,
                12u8,
                9u8,
                165u8,
                89u8,
                239u8,
                36u8,
                213u8,
                32u8,
                185u8,
                218u8,
                197u8,
                197u8,
                184u8,
                129u8,
                10u8,
                232u8,
                67u8,
                78u8,
                77u8,
                13u8,
                134u8,
                65u8,
                26u8,
                149u8,
            ],
            [
                122u8,
                231u8,
                76u8,
                82u8,
                116u8,
                20u8,
                174u8,
                19u8,
                95u8,
                217u8,
                112u8,
                71u8,
                177u8,
                41u8,
                33u8,
                165u8,
                236u8,
                57u8,
                17u8,
                184u8,
                4u8,
                25u8,
                120u8,
                85u8,
                214u8,
                126u8,
                37u8,
                199u8,
                183u8,
                94u8,
                230u8,
                243u8,
            ],
            [
                137u8,
                10u8,
                130u8,
                103u8,
                155u8,
                71u8,
                15u8,
                43u8,
                216u8,
                40u8,
                22u8,
                237u8,
                155u8,
                22u8,
                31u8,
                151u8,
                216u8,
                185u8,
                103u8,
                243u8,
                127u8,
                163u8,
                100u8,
                124u8,
                33u8,
                213u8,
                191u8,
                57u8,
                116u8,
                158u8,
                45u8,
                213u8,
            ],
            [
                156u8,
                78u8,
                133u8,
                65u8,
                202u8,
                143u8,
                13u8,
                193u8,
                196u8,
                19u8,
                249u8,
                16u8,
                143u8,
                102u8,
                216u8,
                45u8,
                60u8,
                236u8,
                177u8,
                189u8,
                219u8,
                206u8,
                67u8,
                122u8,
                97u8,
                202u8,
                163u8,
                23u8,
                92u8,
                76u8,
                201u8,
                111u8,
            ],
            [
                167u8,
                62u8,
                218u8,
                9u8,
                102u8,
                47u8,
                70u8,
                221u8,
                231u8,
                41u8,
                190u8,
                70u8,
                17u8,
                56u8,
                95u8,
                243u8,
                79u8,
                230u8,
                196u8,
                79u8,
                187u8,
                198u8,
                247u8,
                225u8,
                123u8,
                4u8,
                43u8,
                89u8,
                163u8,
                68u8,
                91u8,
                87u8,
            ],
            [
                175u8,
                183u8,
                149u8,
                201u8,
                198u8,
                30u8,
                79u8,
                231u8,
                70u8,
                140u8,
                56u8,
                111u8,
                146u8,
                93u8,
                122u8,
                84u8,
                41u8,
                236u8,
                173u8,
                156u8,
                4u8,
                149u8,
                221u8,
                184u8,
                211u8,
                141u8,
                105u8,
                6u8,
                20u8,
                211u8,
                47u8,
                153u8,
            ],
            [
                178u8,
                222u8,
                47u8,
                190u8,
                128u8,
                26u8,
                13u8,
                246u8,
                192u8,
                203u8,
                221u8,
                253u8,
                68u8,
                139u8,
                163u8,
                196u8,
                29u8,
                72u8,
                160u8,
                64u8,
                202u8,
                53u8,
                197u8,
                108u8,
                129u8,
                150u8,
                239u8,
                15u8,
                202u8,
                231u8,
                33u8,
                168u8,
            ],
            [
                210u8,
                110u8,
                22u8,
                202u8,
                212u8,
                84u8,
                135u8,
                5u8,
                228u8,
                201u8,
                226u8,
                217u8,
                79u8,
                152u8,
                238u8,
                145u8,
                194u8,
                137u8,
                8u8,
                94u8,
                228u8,
                37u8,
                89u8,
                79u8,
                213u8,
                99u8,
                95u8,
                162u8,
                150u8,
                76u8,
                207u8,
                24u8,
            ],
            [
                231u8,
                149u8,
                14u8,
                222u8,
                3u8,
                148u8,
                185u8,
                242u8,
                206u8,
                74u8,
                90u8,
                27u8,
                245u8,
                167u8,
                225u8,
                133u8,
                36u8,
                17u8,
                247u8,
                230u8,
                102u8,
                27u8,
                67u8,
                8u8,
                201u8,
                19u8,
                196u8,
                191u8,
                209u8,
                16u8,
                39u8,
                228u8,
            ],
            [
                232u8,
                22u8,
                153u8,
                184u8,
                81u8,
                19u8,
                238u8,
                161u8,
                199u8,
                62u8,
                16u8,
                88u8,
                139u8,
                43u8,
                3u8,
                94u8,
                85u8,
                137u8,
                51u8,
                105u8,
                99u8,
                33u8,
                115u8,
                175u8,
                212u8,
                63u8,
                235u8,
                25u8,
                47u8,
                172u8,
                100u8,
                227u8,
            ],
            [
                235u8,
                139u8,
                164u8,
                60u8,
                237u8,
                117u8,
                55u8,
                66u8,
                25u8,
                70u8,
                189u8,
                67u8,
                232u8,
                40u8,
                184u8,
                178u8,
                184u8,
                66u8,
                137u8,
                39u8,
                170u8,
                143u8,
                128u8,
                28u8,
                19u8,
                217u8,
                52u8,
                191u8,
                17u8,
                172u8,
                165u8,
                123u8,
            ],
            [
                251u8,
                16u8,
                40u8,
                101u8,
                213u8,
                10u8,
                221u8,
                221u8,
                246u8,
                157u8,
                169u8,
                181u8,
                170u8,
                27u8,
                206u8,
                214u8,
                108u8,
                128u8,
                207u8,
                134u8,
                154u8,
                92u8,
                141u8,
                4u8,
                113u8,
                164u8,
                103u8,
                225u8,
                140u8,
                233u8,
                202u8,
                177u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface
    for FullRelayWithVerifyThroughBitcoinTxTestEvents {
        const NAME: &'static str = "FullRelayWithVerifyThroughBitcoinTxTestEvents";
        const COUNT: usize = 22usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<log as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log)
                }
                Some(<log_address as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_address as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_address)
                }
                Some(<log_array_0 as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_array_0 as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_array_0)
                }
                Some(<log_array_1 as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_array_1 as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_array_1)
                }
                Some(<log_array_2 as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_array_2 as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_array_2)
                }
                Some(<log_bytes as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_bytes as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_bytes)
                }
                Some(<log_bytes32 as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_bytes32 as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_bytes32)
                }
                Some(<log_int as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_int as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_int)
                }
                Some(
                    <log_named_address as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <log_named_address as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_named_address)
                }
                Some(
                    <log_named_array_0 as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <log_named_array_0 as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_named_array_0)
                }
                Some(
                    <log_named_array_1 as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <log_named_array_1 as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_named_array_1)
                }
                Some(
                    <log_named_array_2 as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <log_named_array_2 as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_named_array_2)
                }
                Some(<log_named_bytes as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_named_bytes as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_named_bytes)
                }
                Some(
                    <log_named_bytes32 as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <log_named_bytes32 as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_named_bytes32)
                }
                Some(
                    <log_named_decimal_int as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <log_named_decimal_int as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_named_decimal_int)
                }
                Some(
                    <log_named_decimal_uint as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <log_named_decimal_uint as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_named_decimal_uint)
                }
                Some(<log_named_int as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_named_int as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_named_int)
                }
                Some(<log_named_string as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_named_string as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_named_string)
                }
                Some(<log_named_uint as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_named_uint as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_named_uint)
                }
                Some(<log_string as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_string as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_string)
                }
                Some(<log_uint as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_uint as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_uint)
                }
                Some(<logs as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <logs as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::logs)
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
    impl alloy_sol_types::private::IntoLogData
    for FullRelayWithVerifyThroughBitcoinTxTestEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::log(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_address(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_array_0(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_array_1(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_array_2(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_bytes(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_bytes32(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_int(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_address(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_array_0(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_array_1(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_array_2(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_bytes(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_bytes32(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_decimal_int(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_decimal_uint(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_int(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_string(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_uint(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_string(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_uint(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::logs(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::log(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_address(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_array_0(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_array_1(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_array_2(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_bytes(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_bytes32(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_int(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_address(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_array_0(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_array_1(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_array_2(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_bytes(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_bytes32(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_decimal_int(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_decimal_uint(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_int(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_string(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_uint(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_string(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_uint(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::logs(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`FullRelayWithVerifyThroughBitcoinTxTest`](self) contract instance.

See the [wrapper's documentation](`FullRelayWithVerifyThroughBitcoinTxTestInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> FullRelayWithVerifyThroughBitcoinTxTestInstance<T, P, N> {
        FullRelayWithVerifyThroughBitcoinTxTestInstance::<
            T,
            P,
            N,
        >::new(address, provider)
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
        Output = alloy_contract::Result<
            FullRelayWithVerifyThroughBitcoinTxTestInstance<T, P, N>,
        >,
    > {
        FullRelayWithVerifyThroughBitcoinTxTestInstance::<T, P, N>::deploy(provider)
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
        FullRelayWithVerifyThroughBitcoinTxTestInstance::<
            T,
            P,
            N,
        >::deploy_builder(provider)
    }
    /**A [`FullRelayWithVerifyThroughBitcoinTxTest`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`FullRelayWithVerifyThroughBitcoinTxTest`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct FullRelayWithVerifyThroughBitcoinTxTestInstance<
        T,
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug
    for FullRelayWithVerifyThroughBitcoinTxTestInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("FullRelayWithVerifyThroughBitcoinTxTestInstance")
                .field(&self.address)
                .finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > FullRelayWithVerifyThroughBitcoinTxTestInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`FullRelayWithVerifyThroughBitcoinTxTest`](self) contract instance.

See the [wrapper's documentation](`FullRelayWithVerifyThroughBitcoinTxTestInstance`) for more details.*/
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
        ) -> alloy_contract::Result<
            FullRelayWithVerifyThroughBitcoinTxTestInstance<T, P, N>,
        > {
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
    impl<
        T,
        P: ::core::clone::Clone,
        N,
    > FullRelayWithVerifyThroughBitcoinTxTestInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(
            self,
        ) -> FullRelayWithVerifyThroughBitcoinTxTestInstance<T, P, N> {
            FullRelayWithVerifyThroughBitcoinTxTestInstance {
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
    > FullRelayWithVerifyThroughBitcoinTxTestInstance<T, P, N> {
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
        ///Creates a new call builder for the [`IS_TEST`] function.
        pub fn IS_TEST(&self) -> alloy_contract::SolCallBuilder<T, &P, IS_TESTCall, N> {
            self.call_builder(&IS_TESTCall {})
        }
        ///Creates a new call builder for the [`excludeArtifacts`] function.
        pub fn excludeArtifacts(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, excludeArtifactsCall, N> {
            self.call_builder(&excludeArtifactsCall {})
        }
        ///Creates a new call builder for the [`excludeContracts`] function.
        pub fn excludeContracts(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, excludeContractsCall, N> {
            self.call_builder(&excludeContractsCall {})
        }
        ///Creates a new call builder for the [`excludeSenders`] function.
        pub fn excludeSenders(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, excludeSendersCall, N> {
            self.call_builder(&excludeSendersCall {})
        }
        ///Creates a new call builder for the [`failed`] function.
        pub fn failed(&self) -> alloy_contract::SolCallBuilder<T, &P, failedCall, N> {
            self.call_builder(&failedCall {})
        }
        ///Creates a new call builder for the [`getBlockHeights`] function.
        pub fn getBlockHeights(
            &self,
            chainName: alloy::sol_types::private::String,
            from: alloy::sol_types::private::primitives::aliases::U256,
            to: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, getBlockHeightsCall, N> {
            self.call_builder(
                &getBlockHeightsCall {
                    chainName,
                    from,
                    to,
                },
            )
        }
        ///Creates a new call builder for the [`getDigestLes`] function.
        pub fn getDigestLes(
            &self,
            chainName: alloy::sol_types::private::String,
            from: alloy::sol_types::private::primitives::aliases::U256,
            to: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, getDigestLesCall, N> {
            self.call_builder(
                &getDigestLesCall {
                    chainName,
                    from,
                    to,
                },
            )
        }
        ///Creates a new call builder for the [`getHeaderHexes`] function.
        pub fn getHeaderHexes(
            &self,
            chainName: alloy::sol_types::private::String,
            from: alloy::sol_types::private::primitives::aliases::U256,
            to: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, getHeaderHexesCall, N> {
            self.call_builder(
                &getHeaderHexesCall {
                    chainName,
                    from,
                    to,
                },
            )
        }
        ///Creates a new call builder for the [`getHeaders`] function.
        pub fn getHeaders(
            &self,
            chainName: alloy::sol_types::private::String,
            from: alloy::sol_types::private::primitives::aliases::U256,
            to: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, getHeadersCall, N> {
            self.call_builder(
                &getHeadersCall {
                    chainName,
                    from,
                    to,
                },
            )
        }
        ///Creates a new call builder for the [`targetArtifactSelectors`] function.
        pub fn targetArtifactSelectors(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, targetArtifactSelectorsCall, N> {
            self.call_builder(&targetArtifactSelectorsCall {})
        }
        ///Creates a new call builder for the [`targetArtifacts`] function.
        pub fn targetArtifacts(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, targetArtifactsCall, N> {
            self.call_builder(&targetArtifactsCall {})
        }
        ///Creates a new call builder for the [`targetContracts`] function.
        pub fn targetContracts(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, targetContractsCall, N> {
            self.call_builder(&targetContractsCall {})
        }
        ///Creates a new call builder for the [`targetInterfaces`] function.
        pub fn targetInterfaces(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, targetInterfacesCall, N> {
            self.call_builder(&targetInterfacesCall {})
        }
        ///Creates a new call builder for the [`targetSelectors`] function.
        pub fn targetSelectors(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, targetSelectorsCall, N> {
            self.call_builder(&targetSelectorsCall {})
        }
        ///Creates a new call builder for the [`targetSenders`] function.
        pub fn targetSenders(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, targetSendersCall, N> {
            self.call_builder(&targetSendersCall {})
        }
        ///Creates a new call builder for the [`testIncorrectInputVectorSupplied`] function.
        pub fn testIncorrectInputVectorSupplied(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            testIncorrectInputVectorSuppliedCall,
            N,
        > {
            self.call_builder(
                &testIncorrectInputVectorSuppliedCall {
                },
            )
        }
        ///Creates a new call builder for the [`testIncorrectLocktimeSupplied`] function.
        pub fn testIncorrectLocktimeSupplied(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            testIncorrectLocktimeSuppliedCall,
            N,
        > {
            self.call_builder(
                &testIncorrectLocktimeSuppliedCall {
                },
            )
        }
        ///Creates a new call builder for the [`testIncorrectOutputVectorSupplied`] function.
        pub fn testIncorrectOutputVectorSupplied(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            testIncorrectOutputVectorSuppliedCall,
            N,
        > {
            self.call_builder(
                &testIncorrectOutputVectorSuppliedCall {
                },
            )
        }
        ///Creates a new call builder for the [`testIncorrectProofSupplied`] function.
        pub fn testIncorrectProofSupplied(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, testIncorrectProofSuppliedCall, N> {
            self.call_builder(&testIncorrectProofSuppliedCall {})
        }
        ///Creates a new call builder for the [`testSuccessfullyVerify`] function.
        pub fn testSuccessfullyVerify(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, testSuccessfullyVerifyCall, N> {
            self.call_builder(&testSuccessfullyVerifyCall {})
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > FullRelayWithVerifyThroughBitcoinTxTestInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`log`] event.
        pub fn log_filter(&self) -> alloy_contract::Event<T, &P, log, N> {
            self.event_filter::<log>()
        }
        ///Creates a new event filter for the [`log_address`] event.
        pub fn log_address_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_address, N> {
            self.event_filter::<log_address>()
        }
        ///Creates a new event filter for the [`log_array_0`] event.
        pub fn log_array_0_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_array_0, N> {
            self.event_filter::<log_array_0>()
        }
        ///Creates a new event filter for the [`log_array_1`] event.
        pub fn log_array_1_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_array_1, N> {
            self.event_filter::<log_array_1>()
        }
        ///Creates a new event filter for the [`log_array_2`] event.
        pub fn log_array_2_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_array_2, N> {
            self.event_filter::<log_array_2>()
        }
        ///Creates a new event filter for the [`log_bytes`] event.
        pub fn log_bytes_filter(&self) -> alloy_contract::Event<T, &P, log_bytes, N> {
            self.event_filter::<log_bytes>()
        }
        ///Creates a new event filter for the [`log_bytes32`] event.
        pub fn log_bytes32_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_bytes32, N> {
            self.event_filter::<log_bytes32>()
        }
        ///Creates a new event filter for the [`log_int`] event.
        pub fn log_int_filter(&self) -> alloy_contract::Event<T, &P, log_int, N> {
            self.event_filter::<log_int>()
        }
        ///Creates a new event filter for the [`log_named_address`] event.
        pub fn log_named_address_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_named_address, N> {
            self.event_filter::<log_named_address>()
        }
        ///Creates a new event filter for the [`log_named_array_0`] event.
        pub fn log_named_array_0_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_named_array_0, N> {
            self.event_filter::<log_named_array_0>()
        }
        ///Creates a new event filter for the [`log_named_array_1`] event.
        pub fn log_named_array_1_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_named_array_1, N> {
            self.event_filter::<log_named_array_1>()
        }
        ///Creates a new event filter for the [`log_named_array_2`] event.
        pub fn log_named_array_2_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_named_array_2, N> {
            self.event_filter::<log_named_array_2>()
        }
        ///Creates a new event filter for the [`log_named_bytes`] event.
        pub fn log_named_bytes_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_named_bytes, N> {
            self.event_filter::<log_named_bytes>()
        }
        ///Creates a new event filter for the [`log_named_bytes32`] event.
        pub fn log_named_bytes32_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_named_bytes32, N> {
            self.event_filter::<log_named_bytes32>()
        }
        ///Creates a new event filter for the [`log_named_decimal_int`] event.
        pub fn log_named_decimal_int_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_named_decimal_int, N> {
            self.event_filter::<log_named_decimal_int>()
        }
        ///Creates a new event filter for the [`log_named_decimal_uint`] event.
        pub fn log_named_decimal_uint_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_named_decimal_uint, N> {
            self.event_filter::<log_named_decimal_uint>()
        }
        ///Creates a new event filter for the [`log_named_int`] event.
        pub fn log_named_int_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_named_int, N> {
            self.event_filter::<log_named_int>()
        }
        ///Creates a new event filter for the [`log_named_string`] event.
        pub fn log_named_string_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_named_string, N> {
            self.event_filter::<log_named_string>()
        }
        ///Creates a new event filter for the [`log_named_uint`] event.
        pub fn log_named_uint_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_named_uint, N> {
            self.event_filter::<log_named_uint>()
        }
        ///Creates a new event filter for the [`log_string`] event.
        pub fn log_string_filter(&self) -> alloy_contract::Event<T, &P, log_string, N> {
            self.event_filter::<log_string>()
        }
        ///Creates a new event filter for the [`log_uint`] event.
        pub fn log_uint_filter(&self) -> alloy_contract::Event<T, &P, log_uint, N> {
            self.event_filter::<log_uint>()
        }
        ///Creates a new event filter for the [`logs`] event.
        pub fn logs_filter(&self) -> alloy_contract::Event<T, &P, logs, N> {
            self.event_filter::<logs>()
        }
    }
}
