///Module containing a contract's types and functions.
/**

```solidity
library StdInvariant {
    struct FuzzArtifactSelector { string artifact; bytes4[] selectors; }
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct FuzzArtifactSelector { string artifact; bytes4[] selectors; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct FuzzArtifactSelector {
        #[allow(missing_docs)]
        pub artifact: alloy::sol_types::private::String,
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
            alloy::sol_types::sol_data::String,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::FixedBytes<4>>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::String,
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
        impl ::core::convert::From<FuzzArtifactSelector> for UnderlyingRustTuple<'_> {
            fn from(value: FuzzArtifactSelector) -> Self {
                (value.artifact, value.selectors)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for FuzzArtifactSelector {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    artifact: tuple.0,
                    selectors: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for FuzzArtifactSelector {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for FuzzArtifactSelector {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.artifact,
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
        impl alloy_sol_types::SolType for FuzzArtifactSelector {
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
        impl alloy_sol_types::SolStruct for FuzzArtifactSelector {
            const NAME: &'static str = "FuzzArtifactSelector";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "FuzzArtifactSelector(string artifact,bytes4[] selectors)",
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::eip712_data_word(
                            &self.artifact,
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
        impl alloy_sol_types::EventTopic for FuzzArtifactSelector {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.artifact,
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
                <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.artifact,
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
    struct FuzzArtifactSelector {
        string artifact;
        bytes4[] selectors;
    }
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
    function excludeSelectors() external view returns (StdInvariant.FuzzSelector[] memory excludedSelectors_);
    function excludeSenders() external view returns (address[] memory excludedSenders_);
    function failed() external view returns (bool);
    function getBlockHeights(string memory chainName, uint256 from, uint256 to) external view returns (uint256[] memory elements);
    function getDigestLes(string memory chainName, uint256 from, uint256 to) external view returns (bytes32[] memory elements);
    function getHeaderHexes(string memory chainName, uint256 from, uint256 to) external view returns (bytes[] memory elements);
    function getHeaders(string memory chainName, uint256 from, uint256 to) external view returns (bytes memory headers);
    function targetArtifactSelectors() external view returns (StdInvariant.FuzzArtifactSelector[] memory targetedArtifactSelectors_);
    function targetArtifacts() external view returns (string[] memory targetedArtifacts_);
    function targetContracts() external view returns (address[] memory targetedContracts_);
    function targetInterfaces() external view returns (StdInvariant.FuzzInterface[] memory targetedInterfaces_);
    function targetSelectors() external view returns (StdInvariant.FuzzSelector[] memory targetedSelectors_);
    function targetSenders() external view returns (address[] memory targetedSenders_);
    function testGCDDoesntConfirmHeader() external;
    function testIncorrectInputVectorSupplied() external;
    function testIncorrectLocktimeSupplied() external;
    function testIncorrectOutputVectorSupplied() external;
    function testIncorrectProofSupplied() external;
    function testInsufficientConfirmations() external;
    function testInvalidHeaderSupplied() external;
    function testInvalidMerkleProofSupplied() external;
    function testSuccessfullyVerify() external view;
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
    "name": "excludeSelectors",
    "inputs": [],
    "outputs": [
      {
        "name": "excludedSelectors_",
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
    "stateMutability": "view"
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
        "internalType": "struct StdInvariant.FuzzArtifactSelector[]",
        "components": [
          {
            "name": "artifact",
            "type": "string",
            "internalType": "string"
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
    "name": "testGCDDoesntConfirmHeader",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
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
    "name": "testInsufficientConfirmations",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testInvalidHeaderSupplied",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testInvalidMerkleProofSupplied",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testSuccessfullyVerify",
    "inputs": [],
    "outputs": [],
    "stateMutability": "view"
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
    ///0x600c8054600160ff199182168117909255601f8054909116909117905561010060405260506080818152906175b160a03960219061003d9082610981565b50604051806101600160405280610140815260200161768c61014091396022906100679082610981565b505f51602061766c5f395f51905f5260235561011960245560405161008b906108cf565b604051809103905ff0801580156100a4573d5f5f3e3d5ffd5b5060255f6101000a8154816001600160a01b0302191690836001600160a01b0316021790555060016026555f51602061766c5f395f51905f526027556040518060800160405280600160f81b6001600160e01b03191681526020016040518060600160405280602a81526020016177cc602a913981526020016040518060800160405280604b8152602001617621604b913981525f60209182015281516028805463ffffffff191660e09290921c919091178155908201516029906101699082610981565b506040820151600282019061017e9082610981565b506060820151816003015f6101000a81548163ffffffff021916908360e01c021790555050506040518060a00160405280602280546101bc906108fd565b80601f01602080910402602001604051908101604052809291908181526020018280546101e8906108fd565b80156102335780601f1061020a57610100808354040283529160200191610233565b820191905f5260205f20905b81548152906001019060200180831161021657829003601f168201915b50505050508152602001610119815260200160218054610252906108fd565b80601f016020809104026020016040519081016040528092919081815260200182805461027e906108fd565b80156102c95780601f106102a0576101008083540402835291602001916102c9565b820191905f5260205f20905b8154815290600101906020018083116102ac57829003601f168201915b505050505081526020017f77b98a5e6643973bba49dda18a75140306d2d8694b66f2dcb3561ad5aff0b0c7815260200160405180610160016040528061014081526020016177f6610140913990528051602c9081906103289082610981565b5060208201516001820155604082015160028201906103479082610981565b5060608201516003820155608082015160048201906103669082610981565b505050348015610374575f5ffd5b506040518060400160405280600c81526020016b3432b0b232b939973539b7b760a11b8152506040518060400160405280600c81526020016b05ccecadccae6d2e65cd0caf60a31b8152506040518060400160405280600f81526020016e0b99d95b995cda5ccb9a195a59da1d608a1b815250604051806040016040528060128152602001712e67656e657369732e6469676573745f6c6560701b8152505f5f5160206176015f395f51905f526001600160a01b031663d930a0e66040518163ffffffff1660e01b81526004015f60405180830381865afa15801561045b573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526104829190810190610ab0565b90505f8186604051602001610498929190610b13565b60408051601f19818403018152908290526360f9bb1160e01b825291505f5160206176015f395f51905f52906360f9bb11906104d8908490600401610b85565b5f60405180830381865afa1580156104f2573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526105199190810190610ab0565b6020906105269082610981565b506105bd8560208054610538906108fd565b80601f0160208091040260200160405190810160405280929190818152602001828054610564906108fd565b80156105af5780601f10610586576101008083540402835291602001916105af565b820191905f5260205f20905b81548152906001019060200180831161059257829003601f168201915b5093949350506107ab915050565b61065385602080546105ce906108fd565b80601f01602080910402602001604051908101604052809291908181526020018280546105fa906108fd565b80156106455780601f1061061c57610100808354040283529160200191610645565b820191905f5260205f20905b81548152906001019060200180831161062857829003601f168201915b509394935050610828915050565b6106e98560208054610664906108fd565b80601f0160208091040260200160405190810160405280929190818152602001828054610690906108fd565b80156106db5780601f106106b2576101008083540402835291602001916106db565b820191905f5260205f20905b8154815290600101906020018083116106be57829003601f168201915b50939493505061089b915050565b6040516106f5906108dc565b61070193929190610b97565b604051809103905ff08015801561071a573d5f5f3e3d5ffd5b50601f8054610100600160a81b0319166101006001600160a01b039384168102919091179182905560405163f58db06f60e01b815260016004820181905260248201529104909116965063f58db06f9550604401935061077992505050565b5f604051808303815f87803b158015610790575f5ffd5b505af11580156107a2573d5f5f3e3d5ffd5b50505050610bf6565b604051631fb2437d60e31b81526060905f5160206176015f395f51905f529063fd921be8906107e09086908690600401610bbb565b5f60405180830381865afa1580156107fa573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526108219190810190610ab0565b9392505050565b6040516356eef15b60e11b81525f905f5160206176015f395f51905f529063addde2b69061085c9086908690600401610bbb565b602060405180830381865afa158015610877573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906108219190610bdf565b604051631777e59d60e01b81525f905f5160206176015f395f51905f5290631777e59d9061085c9086908690600401610bbb565b610ff980613c7d83390190565b61293b80614c7683390190565b634e487b7160e01b5f52604160045260245ffd5b600181811c9082168061091157607f821691505b60208210810361092f57634e487b7160e01b5f52602260045260245ffd5b50919050565b601f82111561097c57805f5260205f20601f840160051c8101602085101561095a5750805b601f840160051c820191505b81811015610979575f8155600101610966565b50505b505050565b81516001600160401b0381111561099a5761099a6108e9565b6109ae816109a884546108fd565b84610935565b6020601f8211600181146109e0575f83156109c95750848201515b5f19600385901b1c1916600184901b178455610979565b5f84815260208120601f198516915b82811015610a0f57878501518255602094850194600190920191016109ef565b5084821015610a2c57868401515f19600387901b60f8161c191681555b50505050600190811b01905550565b5f806001600160401b03841115610a5457610a546108e9565b50604051601f19601f85018116603f011681018181106001600160401b0382111715610a8257610a826108e9565b604052838152905080828401851015610a99575f5ffd5b8383602083015e5f60208583010152509392505050565b5f60208284031215610ac0575f5ffd5b81516001600160401b03811115610ad5575f5ffd5b8201601f81018413610ae5575f5ffd5b610af484825160208401610a3b565b949350505050565b5f81518060208401855e5f93019283525090919050565b5f610b1e8285610afc565b7f2f746573742f66756c6c52656c61792f74657374446174612f000000000000008152610b4e6019820185610afc565b95945050505050565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b602081525f6108216020830184610b57565b606081525f610ba96060830186610b57565b60208301949094525060400152919050565b604081525f610bcd6040830185610b57565b8281036020840152610b4e8185610b57565b5f60208284031215610bef575f5ffd5b5051919050565b61307a80610c035f395ff3fe608060405234801561000f575f5ffd5b506004361061019a575f3560e01c8063916a17c6116100e8578063e20c9f7111610093578063ef3e68121161006e578063ef3e6812146102e2578063f11d5cbc146102ea578063fa7626d4146102f2578063fad06b8f146102ff575f5ffd5b8063e20c9f71146102d2578063e89f8419146102da578063ed5a9c49146102da575f5ffd5b8063b5508aa9116100c3578063b5508aa9146102aa578063ba414fa6146102b2578063c899d241146102ca575f5ffd5b8063916a17c614610285578063b0464fdc1461029a578063b52b2058146102a2575f5ffd5b80633f7286f411610148578063632ad53411610123578063632ad5341461025357806366d9a9a01461025b57806385226c8114610270575f5ffd5b80633f7286f41461022357806344badbb61461022b5780635df0d0761461024b575f5ffd5b80632ade3880116101785780632ade3880146101fc57806339425f8f146102115780633e5e3c231461021b575f5ffd5b80630813852a1461019e5780631c0da81f146101c75780631ed7831c146101e7575b5f5ffd5b6101b16101ac366004612544565b610312565b6040516101be91906125f9565b60405180910390f35b6101da6101d5366004612544565b61035d565b6040516101be919061265c565b6101ef6103cf565b6040516101be919061266e565b61020461043c565b6040516101be9190612720565b610219610585565b005b6101ef6106e1565b6101ef61074c565b61023e610239366004612544565b6107b7565b6040516101be91906127a4565b6102196107fa565b610219610ab5565b610263610dbf565b6040516101be9190612837565b610278610f38565b6040516101be91906128b5565b61028d611003565b6040516101be91906128c7565b61028d611106565b610219611209565b6102786112c6565b6102ba611391565b60405190151581526020016101be565b610219611461565b6101ef611665565b6102196116d0565b610219611940565b610219611b65565b601f546102ba9060ff1681565b61023e61030d366004612544565b611d45565b60606103558484846040518060400160405280600381526020017f6865780000000000000000000000000000000000000000000000000000000000815250611d88565b949350505050565b60605f61036b858585610312565b90505f5b6103798585612978565b8110156103c657828282815181106103935761039361298b565b60200260200101516040516020016103ac9291906129cf565b60408051601f19818403018152919052925060010161036f565b50509392505050565b6060601680548060200260200160405190810160405280929190818152602001828054801561043257602002820191905f5260205f20905b815473ffffffffffffffffffffffffffffffffffffffff168152600190910190602001808311610407575b5050505050905090565b6060601e805480602002602001604051908101604052809291908181526020015f905b8282101561057c575f848152602080822060408051808201825260028702909201805473ffffffffffffffffffffffffffffffffffffffff168352600181018054835181870281018701909452808452939591948681019491929084015b82821015610565578382905f5260205f200180546104da906129e3565b80601f0160208091040260200160405190810160405280929190818152602001828054610506906129e3565b80156105515780601f1061052857610100808354040283529160200191610551565b820191905f5260205f20905b81548152906001019060200180831161053457829003601f168201915b5050505050815260200190600101906104bd565b50505050815250508152602001906001019061045f565b50505050905090565b604080518082018252601a81527f496e73756666696369656e7420636f6e6669726d6174696f6e73000000000000602082015290517ff28dceb3000000000000000000000000000000000000000000000000000000008152600991737109709ecfa91a80626ff3989d68f67f5b1dd12d9163f28dceb3916106089160040161265c565b5f604051808303815f87803b15801561061f575f5ffd5b505af1158015610631573d5f5f3e3d5ffd5b5050602554601f546040517f2d79db4600000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff9283169450632d79db46935061069e92610100909204909116908590602890602c90600401612bec565b602060405180830381865afa1580156106b9573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906106dd9190612c3d565b5050565b6060601880548060200260200160405190810160405280929190818152602001828054801561043257602002820191905f5260205f2090815473ffffffffffffffffffffffffffffffffffffffff168152600190910190602001808311610407575050505050905090565b6060601780548060200260200160405190810160405280929190818152602001828054801561043257602002820191905f5260205f2090815473ffffffffffffffffffffffffffffffffffffffff168152600190910190602001808311610407575050505050905090565b60606103558484846040518060400160405280600981526020017f6469676573745f6c650000000000000000000000000000000000000000000000815250611ee9565b60408051608081019091526028805460e01b7fffffffff00000000000000000000000000000000000000000000000000000000168252602980545f9392916020840191610846906129e3565b80601f0160208091040260200160405190810160405280929190818152602001828054610872906129e3565b80156108bd5780601f10610894576101008083540402835291602001916108bd565b820191905f5260205f20905b8154815290600101906020018083116108a057829003601f168201915b505050505081526020016002820180546108d6906129e3565b80601f0160208091040260200160405190810160405280929190818152602001828054610902906129e3565b801561094d5780601f106109245761010080835404028352916020019161094d565b820191905f5260205f20905b81548152906001019060200180831161093057829003601f168201915b50505091835250506003919091015460e01b7fffffffff0000000000000000000000000000000000000000000000000000000016602091820152604080518082018252600181525f818401528382015280518082018252601e81527f496e76616c6964206f757470757420766563746f722070726f7669646564000092810192909252517ff28dceb3000000000000000000000000000000000000000000000000000000008152919250737109709ecfa91a80626ff3989d68f67f5b1dd12d9163f28dceb391610a1f9160040161265c565b5f604051808303815f87803b158015610a36575f5ffd5b505af1158015610a48573d5f5f3e3d5ffd5b5050602554601f546026546040517f2d79db4600000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff9384169550632d79db46945061069e93610100909304909216918690602c90600401612c54565b5f602c6040518060a00160405290815f82018054610ad2906129e3565b80601f0160208091040260200160405190810160405280929190818152602001828054610afe906129e3565b8015610b495780601f10610b2057610100808354040283529160200191610b49565b820191905f5260205f20905b815481529060010190602001808311610b2c57829003601f168201915b5050505050815260200160018201548152602001600282018054610b6c906129e3565b80601f0160208091040260200160405190810160405280929190818152602001828054610b98906129e3565b8015610be35780601f10610bba57610100808354040283529160200191610be3565b820191905f5260205f20905b815481529060010190602001808311610bc657829003601f168201915b5050505050815260200160038201548152602001600482018054610c06906129e3565b80601f0160208091040260200160405190810160405280929190818152602001828054610c32906129e3565b8015610c7d5780601f10610c5457610100808354040283529160200191610c7d565b820191905f5260205f20905b815481529060010190602001808311610c6057829003601f168201915b50505050508152505090506040518060800160405280604f8152602001612ff6604f913960408083019190915280518082018252601081527f4261642068656164657220626c6f636b00000000000000000000000000000000602082015290517ff28dceb3000000000000000000000000000000000000000000000000000000008152737109709ecfa91a80626ff3989d68f67f5b1dd12d9163f28dceb391610d29919060040161265c565b5f604051808303815f87803b158015610d40575f5ffd5b505af1158015610d52573d5f5f3e3d5ffd5b5050602554601f546026546040517f2d79db4600000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff9384169550632d79db46945061069e93610100909304909216916028908790600401612d3a565b6060601b805480602002602001604051908101604052809291908181526020015f905b8282101561057c578382905f5260205f2090600202016040518060400160405290815f82018054610e12906129e3565b80601f0160208091040260200160405190810160405280929190818152602001828054610e3e906129e3565b8015610e895780601f10610e6057610100808354040283529160200191610e89565b820191905f5260205f20905b815481529060010190602001808311610e6c57829003601f168201915b5050505050815260200160018201805480602002602001604051908101604052809291908181526020018280548015610f2057602002820191905f5260205f20905f905b82829054906101000a900460e01b7bffffffffffffffffffffffffffffffffffffffffffffffffffffffff191681526020019060040190602082600301049283019260010382029150808411610ecd5790505b50505050508152505081526020019060010190610de2565b6060601a805480602002602001604051908101604052809291908181526020015f905b8282101561057c578382905f5260205f20018054610f78906129e3565b80601f0160208091040260200160405190810160405280929190818152602001828054610fa4906129e3565b8015610fef5780601f10610fc657610100808354040283529160200191610fef565b820191905f5260205f20905b815481529060010190602001808311610fd257829003601f168201915b505050505081526020019060010190610f5b565b6060601d805480602002602001604051908101604052809291908181526020015f905b8282101561057c575f84815260209081902060408051808201825260028602909201805473ffffffffffffffffffffffffffffffffffffffff1683526001810180548351818702810187019094528084529394919385830193928301828280156110ee57602002820191905f5260205f20905f905b82829054906101000a900460e01b7bffffffffffffffffffffffffffffffffffffffffffffffffffffffff19168152602001906004019060208260030104928301926001038202915080841161109b5790505b50505050508152505081526020019060010190611026565b6060601c805480602002602001604051908101604052809291908181526020015f905b8282101561057c575f84815260209081902060408051808201825260028602909201805473ffffffffffffffffffffffffffffffffffffffff1683526001810180548351818702810187019094528084529394919385830193928301828280156111f157602002820191905f5260205f20905f905b82829054906101000a900460e01b7bffffffffffffffffffffffffffffffffffffffffffffffffffffffff19168152602001906004019060208260030104928301926001038202915080841161119e5790505b50505050508152505081526020019060010190611129565b602554601f546026546040517f2d79db460000000000000000000000000000000000000000000000000000000081525f9373ffffffffffffffffffffffffffffffffffffffff90811693632d79db4693611276936101009092049092169190602890602c90600401612bec565b602060405180830381865afa158015611291573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906112b59190612c3d565b90506112c381602754612037565b50565b60606019805480602002602001604051908101604052809291908181526020015f905b8282101561057c578382905f5260205f20018054611306906129e3565b80601f0160208091040260200160405190810160405280929190818152602001828054611332906129e3565b801561137d5780601f106113545761010080835404028352916020019161137d565b820191905f5260205f20905b81548152906001019060200180831161136057829003601f168201915b5050505050815260200190600101906112e9565b6008545f9060ff16156113a8575060085460ff1690565b6040517f667f9d70000000000000000000000000000000000000000000000000000000008152737109709ecfa91a80626ff3989d68f67f5b1dd12d600482018190527f6661696c6564000000000000000000000000000000000000000000000000000060248301525f9163667f9d7090604401602060405180830381865afa158015611436573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061145a9190612c3d565b1415905090565b60408051608081019091526028805460e01b7fffffffff00000000000000000000000000000000000000000000000000000000168252602980545f93929160208401916114ad906129e3565b80601f01602080910402602001604051908101604052809291908181526020018280546114d9906129e3565b80156115245780601f106114fb57610100808354040283529160200191611524565b820191905f5260205f20905b81548152906001019060200180831161150757829003601f168201915b5050505050815260200160028201805461153d906129e3565b80601f0160208091040260200160405190810160405280929190818152602001828054611569906129e3565b80156115b45780601f1061158b576101008083540402835291602001916115b4565b820191905f5260205f20905b81548152906001019060200180831161159757829003601f168201915b50505091835250506003919091015460e01b7fffffffff00000000000000000000000000000000000000000000000000000000166020918201527c0100000000000000000000000000000000000000000000000000000000606083810191909152604080519182019052603c808252929350737109709ecfa91a80626ff3989d68f67f5b1dd12d9263f28dceb392612fba908301396040518263ffffffff1660e01b8152600401610a1f919061265c565b6060601580548060200260200160405190810160405280929190818152602001828054801561043257602002820191905f5260205f2090815473ffffffffffffffffffffffffffffffffffffffff168152600190910190602001808311610407575050505050905090565b5f602c6040518060a00160405290815f820180546116ed906129e3565b80601f0160208091040260200160405190810160405280929190818152602001828054611719906129e3565b80156117645780601f1061173b57610100808354040283529160200191611764565b820191905f5260205f20905b81548152906001019060200180831161174757829003601f168201915b5050505050815260200160018201548152602001600282018054611787906129e3565b80601f01602080910402602001604051908101604052809291908181526020018280546117b3906129e3565b80156117fe5780601f106117d5576101008083540402835291602001916117fe565b820191905f5260205f20905b8154815290600101906020018083116117e157829003601f168201915b5050505050815260200160038201548152602001600482018054611821906129e3565b80601f016020809104026020016040519081016040528092919081815260200182805461184d906129e3565b80156118985780601f1061186f57610100808354040283529160200191611898565b820191905f5260205f20905b81548152906001019060200180831161187b57829003601f168201915b505050919092525050604080518082018252600181525f60208083019190915290845281518083018352601681527f426164206d65726b6c652061727261792070726f6f66000000000000000000009181019190915290517ff28dceb3000000000000000000000000000000000000000000000000000000008152929350737109709ecfa91a80626ff3989d68f67f5b1dd12d9263f28dceb39250610d29919060040161265c565b60408051608081019091526028805460e01b7fffffffff00000000000000000000000000000000000000000000000000000000168252602980545f939291602084019161198c906129e3565b80601f01602080910402602001604051908101604052809291908181526020018280546119b8906129e3565b8015611a035780601f106119da57610100808354040283529160200191611a03565b820191905f5260205f20905b8154815290600101906020018083116119e657829003601f168201915b50505050508152602001600282018054611a1c906129e3565b80601f0160208091040260200160405190810160405280929190818152602001828054611a48906129e3565b8015611a935780601f10611a6a57610100808354040283529160200191611a93565b820191905f5260205f20905b815481529060010190602001808311611a7657829003601f168201915b50505091835250506003919091015460e01b7fffffffff0000000000000000000000000000000000000000000000000000000016602091820152604080518082018252600181525f818401528383015280518082018252601d81527f496e76616c696420696e70757420766563746f722070726f766964656400000092810192909252517ff28dceb3000000000000000000000000000000000000000000000000000000008152919250737109709ecfa91a80626ff3989d68f67f5b1dd12d9163f28dceb391610a1f9160040161265c565b601f546040517ff58db06f0000000000000000000000000000000000000000000000000000000081525f60048201819052602482015261010090910473ffffffffffffffffffffffffffffffffffffffff169063f58db06f906044015f604051808303815f87803b158015611bd8575f5ffd5b505af1158015611bea573d5f5f3e3d5ffd5b5050604080518082018252601b81527f47434420646f6573206e6f7420636f6e6669726d206865616465720000000000602082015290517ff28dceb3000000000000000000000000000000000000000000000000000000008152737109709ecfa91a80626ff3989d68f67f5b1dd12d935063f28dceb39250611c6f919060040161265c565b5f604051808303815f87803b158015611c86575f5ffd5b505af1158015611c98573d5f5f3e3d5ffd5b5050602554601f546026546040517f2d79db4600000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff9384169550632d79db469450611d069361010090930490921691602890602c90600401612bec565b602060405180830381865afa158015611d21573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906112c39190612c3d565b60606103558484846040518060400160405280600681526020017f68656967687400000000000000000000000000000000000000000000000000008152506120ba565b6060611d948484612978565b67ffffffffffffffff811115611dac57611dac6124bf565b604051908082528060200260200182016040528015611ddf57816020015b6060815260200190600190039081611dca5790505b509050835b83811015611ee057611eb286611df983612208565b85604051602001611e0c93929190612ddd565b60405160208183030381529060405260208054611e28906129e3565b80601f0160208091040260200160405190810160405280929190818152602001828054611e54906129e3565b8015611e9f5780601f10611e7657610100808354040283529160200191611e9f565b820191905f5260205f20905b815481529060010190602001808311611e8257829003601f168201915b505050505061233990919063ffffffff16565b82611ebd8784612978565b81518110611ecd57611ecd61298b565b6020908102919091010152600101611de4565b50949350505050565b6060611ef58484612978565b67ffffffffffffffff811115611f0d57611f0d6124bf565b604051908082528060200260200182016040528015611f36578160200160208202803683370190505b509050835b83811015611ee05761200986611f5083612208565b85604051602001611f6393929190612ddd565b60405160208183030381529060405260208054611f7f906129e3565b80601f0160208091040260200160405190810160405280929190818152602001828054611fab906129e3565b8015611ff65780601f10611fcd57610100808354040283529160200191611ff6565b820191905f5260205f20905b815481529060010190602001808311611fd957829003601f168201915b50505050506123d890919063ffffffff16565b826120148784612978565b815181106120245761202461298b565b6020908102919091010152600101611f3b565b6040517f7c84c69b0000000000000000000000000000000000000000000000000000000081526004810183905260248101829052737109709ecfa91a80626ff3989d68f67f5b1dd12d90637c84c69b906044015f6040518083038186803b1580156120a0575f5ffd5b505afa1580156120b2573d5f5f3e3d5ffd5b505050505050565b60606120c68484612978565b67ffffffffffffffff8111156120de576120de6124bf565b604051908082528060200260200182016040528015612107578160200160208202803683370190505b509050835b83811015611ee0576121da8661212183612208565b8560405160200161213493929190612ddd565b60405160208183030381529060405260208054612150906129e3565b80601f016020809104026020016040519081016040528092919081815260200182805461217c906129e3565b80156121c75780601f1061219e576101008083540402835291602001916121c7565b820191905f5260205f20905b8154815290600101906020018083116121aa57829003601f168201915b505050505061246b90919063ffffffff16565b826121e58784612978565b815181106121f5576121f561298b565b602090810291909101015260010161210c565b6060815f0361224a57505060408051808201909152600181527f3000000000000000000000000000000000000000000000000000000000000000602082015290565b815f5b8115612273578061225d81612e7a565b915061226c9050600a83612ede565b915061224d565b5f8167ffffffffffffffff81111561228d5761228d6124bf565b6040519080825280601f01601f1916602001820160405280156122b7576020820181803683370190505b5090505b8415610355576122cc600183612978565b91506122d9600a86612ef1565b6122e4906030612f04565b60f81b8183815181106122f9576122f961298b565b60200101907effffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff191690815f1a905350612332600a86612ede565b94506122bb565b6040517ffd921be8000000000000000000000000000000000000000000000000000000008152606090737109709ecfa91a80626ff3989d68f67f5b1dd12d9063fd921be89061238e9086908690600401612f17565b5f60405180830381865afa1580156123a8573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526123cf9190810190612f44565b90505b92915050565b6040517f1777e59d0000000000000000000000000000000000000000000000000000000081525f90737109709ecfa91a80626ff3989d68f67f5b1dd12d90631777e59d9061242c9086908690600401612f17565b602060405180830381865afa158015612447573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906123cf9190612c3d565b6040517faddde2b60000000000000000000000000000000000000000000000000000000081525f90737109709ecfa91a80626ff3989d68f67f5b1dd12d9063addde2b69061242c9086908690600401612f17565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b604051601f8201601f1916810167ffffffffffffffff81118282101715612515576125156124bf565b604052919050565b5f67ffffffffffffffff821115612536576125366124bf565b50601f01601f191660200190565b5f5f5f60608486031215612556575f5ffd5b833567ffffffffffffffff81111561256c575f5ffd5b8401601f8101861361257c575f5ffd5b803561258f61258a8261251d565b6124ec565b8181528760208385010111156125a3575f5ffd5b816020840160208301375f602092820183015297908601359650604090950135949350505050565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b8281101561265057603f1987860301845261263b8583516125cb565b9450602093840193919091019060010161261f565b50929695505050505050565b602081525f6123cf60208301846125cb565b602080825282518282018190525f918401906040840190835b818110156126bb57835173ffffffffffffffffffffffffffffffffffffffff16835260209384019390920191600101612687565b509095945050505050565b5f82825180855260208501945060208160051b830101602085015f5b8381101561271457601f198584030188526126fe8383516125cb565b60209889019890935091909101906001016126e2565b50909695505050505050565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b8281101561265057603f19878603018452815173ffffffffffffffffffffffffffffffffffffffff8151168652602081015190506040602087015261278e60408701826126c6565b9550506020938401939190910190600101612746565b602080825282518282018190525f918401906040840190835b818110156126bb5783518352602093840193909201916001016127bd565b5f8151808452602084019350602083015f5b8281101561282d5781517fffffffff00000000000000000000000000000000000000000000000000000000168652602095860195909101906001016127ed565b5093949350505050565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b8281101561265057603f19878603018452815180516040875261288360408801826125cb565b905060208201519150868103602088015261289e81836127db565b96505050602093840193919091019060010161285d565b602081525f6123cf60208301846126c6565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b8281101561265057603f19878603018452815173ffffffffffffffffffffffffffffffffffffffff8151168652602081015190506040602087015261293560408701826127db565b95505060209384019391909101906001016128ed565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b818103818111156123d2576123d261294b565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f81518060208401855e5f93019283525090919050565b5f6103556129dd83866129b8565b846129b8565b600181811c908216806129f757607f821691505b602082108103612a2e577f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b50919050565b80545f90600181811c90821680612a4c57607f821691505b602082108103612a83577f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b81865260208601818015612a9e5760018114612ad257612afe565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff008516825283151560051b82019550612afe565b5f878152602090205f5b85811015612af857815484820152600190910190602001612adc565b83019650505b505050505092915050565b7fffffffff00000000000000000000000000000000000000000000000000000000815460e01b168252608060208301525f612b4a6080840160018401612a34565b8381036040850152612b5f8160028501612a34565b90507fffffffff00000000000000000000000000000000000000000000000000000000600384015460e01b1660608501528091505092915050565b60a082525f612bac60a0840183612a34565b600183015460208501528381036040850152612bcb8160028501612a34565b90506003830154606085015283810360808501526103558160048501612a34565b73ffffffffffffffffffffffffffffffffffffffff85168152836020820152608060408201525f612c206080830185612b09565b8281036060840152612c328185612b9a565b979650505050505050565b5f60208284031215612c4d575f5ffd5b5051919050565b73ffffffffffffffffffffffffffffffffffffffff85168152836020820152608060408201527fffffffff0000000000000000000000000000000000000000000000000000000083511660808201525f6020840151608060a0840152612cbe6101008401826125cb565b905060408501517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff808483030160c0850152612cf982826125cb565b9150507fffffffff0000000000000000000000000000000000000000000000000000000060608601511660e08401528281036060840152612c328185612b9a565b73ffffffffffffffffffffffffffffffffffffffff85168152836020820152608060408201525f612d6e6080830185612b09565b8281036060840152835160a08252612d8960a08301826125cb565b90506020850151602083015260408501518282036040840152612dac82826125cb565b9150506060850151606083015260808501518282036080840152612dd082826125cb565b9998505050505050505050565b7f2e0000000000000000000000000000000000000000000000000000000000000081525f612e0e60018301866129b8565b7f5b000000000000000000000000000000000000000000000000000000000000008152612e3e60018201866129b8565b90507f5d2e0000000000000000000000000000000000000000000000000000000000008152612e7060028201856129b8565b9695505050505050565b5f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8203612eaa57612eaa61294b565b5060010190565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f82612eec57612eec612eb1565b500490565b5f82612eff57612eff612eb1565b500690565b808201808211156123d2576123d261294b565b604081525f612f2960408301856125cb565b8281036020840152612f3b81856125cb565b95945050505050565b5f60208284031215612f54575f5ffd5b815167ffffffffffffffff811115612f6a575f5ffd5b8201601f81018413612f7a575f5ffd5b8051612f8861258a8261251d565b818152856020838501011115612f9c575f5ffd5b8160208401602083015e5f9181016020019190915294935050505056fe5478206d65726b6c652070726f6f66206973206e6f742076616c696420666f722070726f76696465642068656164657220616e6420747820686173680000002073bd2184edd9c4fc76642ea6754ee40136970efc10c4190000000000000000000296ef123ea96da5cf695f22bf7d94be87d49db1ad7ac371ac43c4da4161c8c216349c5ba11928170d3878a2646970667358221220fdef3650b697a7d9ffba1a56b59715ef8fc9a34baf19fb7d99de5751ffca908464736f6c634300081c00336080604052348015600e575f5ffd5b50610fdd8061001c5f395ff3fe608060405234801561000f575f5ffd5b5060043610610029575f3560e01c80632d79db461461002d575b5f5ffd5b61004061003b366004610d28565b610052565b60405190815260200160405180910390f35b5f61005f8585858561006a565b90505b949350505050565b5f6100748361008f565b9050610080818361018a565b610062858584604001516103f4565b5f61009d82602001516104db565b6100ee5760405162461bcd60e51b815260206004820152601d60248201527f496e76616c696420696e70757420766563746f722070726f766964656400000060448201526064015b60405180910390fd5b6100fb8260400151610575565b6101475760405162461bcd60e51b815260206004820152601e60248201527f496e76616c6964206f757470757420766563746f722070726f7669646564000060448201526064016100e5565b610184825f01518360200151846040015185606001516040516020016101709493929190610e58565b604051602081830303815290604052610602565b92915050565b805161019590610624565b6101e15760405162461bcd60e51b815260206004820152601660248201527f426164206d65726b6c652061727261792070726f6f660000000000000000000060448201526064016100e5565b6080810151518151511461025d5760405162461bcd60e51b815260206004820152602f60248201527f5478206e6f74206f6e2073616d65206c6576656c206f66206d65726b6c65207460448201527f72656520617320636f696e62617365000000000000000000000000000000000060648201526084016100e5565b5f61026b826040015161063a565b825160208401519192506102829185918491610646565b6102f45760405162461bcd60e51b815260206004820152603c60248201527f5478206d65726b6c652070726f6f66206973206e6f742076616c696420666f7260448201527f2070726f76696465642068656164657220616e6420747820686173680000000060648201526084016100e5565b5f6002836060015160405160200161030e91815260200190565b60408051601f198184030181529082905261032891610ec7565b602060405180830381855afa158015610343573d5f5f3e3d5ffd5b5050506040513d601f19601f820116820180604052508101906103669190610ed2565b608084015190915061037c90829084905f610646565b6103ee5760405162461bcd60e51b815260206004820152603f60248201527f436f696e62617365206d65726b6c652070726f6f66206973206e6f742076616c60448201527f696420666f722070726f76696465642068656164657220616e6420686173680060648201526084016100e5565b50505050565b80516050146104455760405162461bcd60e51b815260206004820152601060248201527f4261642068656164657220626c6f636b0000000000000000000000000000000060448201526064016100e5565b5f61044f82610678565b6040517fe471e72c0000000000000000000000000000000000000000000000000000000081526004810182905260ff8516602482015290915073ffffffffffffffffffffffffffffffffffffffff85169063e471e72c906044015f6040518083038186803b1580156104bf575f5ffd5b505afa1580156104d1573d5f5f3e3d5ffd5b5050505050505050565b5f5f5f6104e784610732565b90925090508015806104f957505f1982145b1561050757505f9392505050565b5f610513836001610f16565b90505f5b82811015610568578551821061053257505f95945050505050565b5f61053d8784610747565b90505f19810361055357505f9695505050505050565b61055d8184610f16565b925050600101610517565b5093519093149392505050565b5f5f5f61058184610732565b909250905080158061059357505f1982145b156105a157505f9392505050565b5f6105ad836001610f16565b90505f5b8281101561056857855182106105cc57505f95945050505050565b5f6105d78784610796565b90505f1981036105ed57505f9695505050505050565b6105f78184610f16565b9250506001016105b1565b5f60205f83516020850160025afa5060205f60205f60025afa50505f51919050565b5f602082516106339190610f29565b1592915050565b60448101515f90610184565b5f8385148015610654575081155b801561065f57508251155b1561066c57506001610062565b61005f858486856107f6565b5f6002808360405161068a9190610ec7565b602060405180830381855afa1580156106a5573d5f5f3e3d5ffd5b5050506040513d601f19601f820116820180604052508101906106c89190610ed2565b6040516020016106da91815260200190565b60408051601f19818403018152908290526106f491610ec7565b602060405180830381855afa15801561070f573d5f5f3e3d5ffd5b5050506040513d601f19601f820116820180604052508101906101849190610ed2565b5f5f61073e835f61089b565b91509150915091565b5f5f5f6107548585610a38565b90925090506001820161076c575f1992505050610184565b80610778836025610f16565b6107829190610f16565b61078d906004610f16565b95945050505050565b5f6107a2826009610f16565b835110156107b257505f19610184565b5f806107c8856107c3866008610f16565b61089b565b9092509050600182016107e0575f1992505050610184565b806107ec836009610f16565b61078d9190610f16565b5f602084516108059190610f29565b1561081157505f610062565b83515f0361082057505f610062565b81855f5b865181101561088e57610838600284610f29565b60010361085c5761085561084f8883016020015190565b83610a76565b9150610875565b6108728261086d8984016020015190565b610a76565b91505b60019290921c91610887602082610f16565b9050610824565b5090931495945050505050565b5f5f5f6108a88585610a88565b90508060ff165f036108db575f8585815181106108c7576108c7610f61565b016020015190935060f81c9150610a319050565b836108e7826001610f8e565b60ff166108f49190610f16565b85511015610909575f195f9250925050610a31565b5f8160ff1660020361094c5761094161092d610926876001610f16565b8890610b0c565b62ffff0060e882901c1660f89190911c1790565b61ffff169050610a27565b8160ff1660040361099b5761098e610968610926876001610f16565b60d881901c63ff00ff001662ff00ff60e89290921c9190911617601081811b91901c1790565b63ffffffff169050610a27565b8160ff16600803610a2757610a1a6109b7610926876001610f16565b60c01c64ff000000ff600882811c91821665ff000000ff009390911b92831617601090811b67ffffffffffffffff1666ff00ff00ff00ff9290921667ff00ff00ff00ff009093169290921790911c65ffff0000ffff1617602081811c91901b1790565b67ffffffffffffffff1690505b60ff909116925090505b9250929050565b5f80610a45836025610f16565b84511015610a5857505f1990505f610a31565b5f80610a69866107c3876024610f16565b9097909650945050505050565b5f610a818383610b1a565b9392505050565b5f828281518110610a9b57610a9b610f61565b016020015160f81c60ff03610ab257506008610184565b828281518110610ac457610ac4610f61565b016020015160f81c60fe03610adb57506004610184565b828281518110610aed57610aed610f61565b016020015160f81c60fd03610b0457506002610184565b505f92915050565b5f610a818383016020015190565b5f825f528160205260205f60405f60025afa5060205f60205f60025afa50505f5192915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b60405160a0810167ffffffffffffffff81118282101715610b9157610b91610b41565b60405290565b6040516080810167ffffffffffffffff81118282101715610b9157610b91610b41565b80357fffffffff0000000000000000000000000000000000000000000000000000000081168114610be9575f5ffd5b919050565b5f82601f830112610bfd575f5ffd5b813567ffffffffffffffff811115610c1757610c17610b41565b604051601f8201601f19908116603f0116810167ffffffffffffffff81118282101715610c4657610c46610b41565b604052818152838201602001851015610c5d575f5ffd5b816020850160208301375f918101602001919091529392505050565b5f60a08284031215610c89575f5ffd5b610c91610b6e565b9050813567ffffffffffffffff811115610ca9575f5ffd5b610cb584828501610bee565b82525060208281013590820152604082013567ffffffffffffffff811115610cdb575f5ffd5b610ce784828501610bee565b60408301525060608281013590820152608082013567ffffffffffffffff811115610d10575f5ffd5b610d1c84828501610bee565b60808301525092915050565b5f5f5f5f60808587031215610d3b575f5ffd5b843573ffffffffffffffffffffffffffffffffffffffff81168114610d5e575f5ffd5b935060208501359250604085013567ffffffffffffffff811115610d80575f5ffd5b850160808188031215610d91575f5ffd5b610d99610b97565b610da282610bba565b8152602082013567ffffffffffffffff811115610dbd575f5ffd5b610dc989828501610bee565b602083015250604082013567ffffffffffffffff811115610de8575f5ffd5b610df489828501610bee565b604083015250610e0660608301610bba565b60608201528093505050606085013567ffffffffffffffff811115610e29575f5ffd5b610e3587828801610c79565b91505092959194509250565b5f81518060208401855e5f93019283525090919050565b7fffffffff00000000000000000000000000000000000000000000000000000000851681525f610e94610e8e6004840187610e41565b85610e41565b7fffffffff0000000000000000000000000000000000000000000000000000000093909316835250506004019392505050565b5f610a818284610e41565b5f60208284031215610ee2575f5ffd5b5051919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b8082018082111561018457610184610ee9565b5f82610f5c577f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b500690565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b60ff818116838216019081111561018457610184610ee956fea264697066735822122001344713d072b6959d7d3c2dc4eafc4128009f0b4a79162950b41d76ede7b76464736f6c634300081c0033608060405234801561000f575f5ffd5b5060405161293b38038061293b83398101604081905261002e9161032b565b82828282828261003f835160501490565b6100845760405162461bcd60e51b81526020600482015260116024820152704261642067656e6573697320626c6f636b60781b60448201526064015b60405180910390fd5b5f61008e84610166565b905062ffffff8216156101095760405162461bcd60e51b815260206004820152603d60248201527f506572696f64207374617274206861736820646f6573206e6f7420686176652060448201527f776f726b2e2048696e743a2077726f6e672062797465206f726465723f000000606482015260840161007b565b5f818155600182905560028290558181526004602052604090208390556101326107e0846103fe565b61013c9084610425565b5f8381526004602052604090205561015384610226565b600555506105bd98505050505050505050565b5f600280836040516101789190610438565b602060405180830381855afa158015610193573d5f5f3e3d5ffd5b5050506040513d601f19601f820116820180604052508101906101b6919061044e565b6040516020016101c891815260200190565b60408051601f19818403018152908290526101e291610438565b602060405180830381855afa1580156101fd573d5f5f3e3d5ffd5b5050506040513d601f19601f82011682018060405250810190610220919061044e565b92915050565b5f61022061023383610238565b610243565b5f6102208282610253565b5f61022061ffff60d01b836102f7565b5f8061026a610263846048610465565b8590610309565b60e81c90505f8461027c85604b610465565b8151811061028c5761028c610478565b016020015160f81c90505f6102be835f60108262ffffff16901c8261ff001660108462ffffff16901b17179050919050565b62ffffff1690505f6102d160038461048c565b60ff1690506102e281610100610588565b6102ec9083610593565b979650505050505050565b5f61030282846105aa565b9392505050565b5f6103028383016020015190565b634e487b7160e01b5f52604160045260245ffd5b5f5f5f6060848603121561033d575f5ffd5b83516001600160401b03811115610352575f5ffd5b8401601f81018613610362575f5ffd5b80516001600160401b0381111561037b5761037b610317565b604051601f8201601f19908116603f011681016001600160401b03811182821017156103a9576103a9610317565b6040528181528282016020018810156103c0575f5ffd5b8160208401602083015e5f6020928201830152908601516040909601519097959650949350505050565b634e487b7160e01b5f52601260045260245ffd5b5f8261040c5761040c6103ea565b500690565b634e487b7160e01b5f52601160045260245ffd5b8181038181111561022057610220610411565b5f82518060208501845e5f920191825250919050565b5f6020828403121561045e575f5ffd5b5051919050565b8082018082111561022057610220610411565b634e487b7160e01b5f52603260045260245ffd5b60ff828116828216039081111561022057610220610411565b6001815b60018411156104e0578085048111156104c4576104c4610411565b60018416156104d257908102905b60019390931c9280026104a9565b935093915050565b5f826104f657506001610220565b8161050257505f610220565b816001811461051857600281146105225761053e565b6001915050610220565b60ff84111561053357610533610411565b50506001821b610220565b5060208310610133831016604e8410600b8410161715610561575081810a610220565b61056d5f1984846104a5565b805f190482111561058057610580610411565b029392505050565b5f61030283836104e8565b808202811582820484141761022057610220610411565b5f826105b8576105b86103ea565b500490565b612371806105ca5f395ff3fe608060405234801561000f575f5ffd5b5060043610610115575f3560e01c806370d53c18116100ad578063b985621a1161007d578063e3d8d8d811610063578063e3d8d8d814610222578063e471e72c14610229578063f58db06f1461023c575f5ffd5b8063b985621a14610207578063c58242cd1461021a575f5ffd5b806370d53c18146101b157806374c3a3a9146101ce5780637fa637fc146101e1578063b25b9b00146101f4575f5ffd5b80632e4f161a116100e85780632e4f161a1461015557806330017b3b1461017857806360b5c3901461018b57806365da41b91461019e575f5ffd5b806305d09a7014610119578063113764be1461012e5780631910d973146101455780632b97be241461014d575b5f5ffd5b61012c610127366004611d7b565b6102a8565b005b6005545b6040519081526020015b60405180910390f35b600154610132565b600654610132565b610168610163366004611e0c565b6104e1565b604051901515815260200161013c565b610132610186366004611e3b565b6104f9565b610132610199366004611e5b565b61050d565b6101686101ac366004611e72565b610517565b6101b9600481565b60405163ffffffff909116815260200161013c565b6101686101dc366004611ede565b6106c3565b6101686101ef366004611f5f565b610838565b610132610202366004611ffe565b610a17565b610168610215366004612077565b610a94565b600254610132565b5f54610132565b61012c6102373660046120a0565b610aaa565b61012c61024a3660046120d9565b600780547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000169215157fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00ff169290921761010091151591909102179055565b6102e687878080601f0160208091040260200160405190810160405280939291908181526020018383808284375f92019190915250610b6892505050565b6103375760405162461bcd60e51b815260206004820152601060248201527f4261642068656164657220626c6f636b0000000000000000000000000000000060448201526064015b60405180910390fd5b61037585858080601f0160208091040260200160405190810160405280939291908181526020018383808284375f92019190915250610b6f92505050565b6103c15760405162461bcd60e51b815260206004820152601660248201527f426164206d65726b6c652061727261792070726f6f6600000000000000000000604482015260640161032e565b6104408361040389898080601f0160208091040260200160405190810160405280939291908181526020018383808284375f92019190915250610b8592505050565b87878080601f0160208091040260200160405190810160405280939291908181526020018383808284375f92019190915250889250610b91915050565b61048c5760405162461bcd60e51b815260206004820152601360248201527f42616420696e636c7573696f6e2070726f6f6600000000000000000000000000604482015260640161032e565b5f6104cb88888080601f0160208091040260200160405190810160405280939291908181526020018383808284375f92019190915250610bc392505050565b90506104d78183610aaa565b5050505050505050565b5f6104ee85858585610c9b565b90505b949350505050565b5f6105048383610d35565b90505b92915050565b5f61050782610da7565b5f61055683838080601f0160208091040260200160405190810160405280939291908181526020018383808284375f92019190915250610e5592505050565b6105c85760405162461bcd60e51b815260206004820152602b60248201527f486561646572206172726179206c656e677468206d757374206265206469766960448201527f7369626c65206279203830000000000000000000000000000000000000000000606482015260840161032e565b61060685858080601f0160208091040260200160405190810160405280939291908181526020018383808284375f92019190915250610b6892505050565b6106525760405162461bcd60e51b815260206004820152601760248201527f416e63686f72206d757374206265203830206279746573000000000000000000604482015260640161032e565b6104ee85858080601f0160208091040260200160405190810160405280939291908181526020018383808284375f9201919091525050604080516020601f890181900481028201810190925287815292508791508690819084018382808284375f9201829052509250610e64915050565b5f61070284848080601f0160208091040260200160405190810160405280939291908181526020018383808284375f92019190915250610b6892505050565b8015610747575061074786868080601f0160208091040260200160405190810160405280939291908181526020018383808284375f92019190915250610b6892505050565b6107b95760405162461bcd60e51b815260206004820152602e60248201527f42616420617267732e20436865636b2068656164657220616e6420617272617960448201527f2062797465206c656e677468732e000000000000000000000000000000000000606482015260840161032e565b61082d8787878080601f0160208091040260200160405190810160405280939291908181526020018383808284375f9201919091525050604080516020601f8b0181900481028201810190925289815292508991508890819084018382808284375f92019190915250889250611251915050565b979650505050505050565b5f61087787878080601f0160208091040260200160405190810160405280939291908181526020018383808284375f92019190915250610b6892505050565b80156108bc57506108bc85858080601f0160208091040260200160405190810160405280939291908181526020018383808284375f92019190915250610b6892505050565b8015610901575061090183838080601f0160208091040260200160405190810160405280939291908181526020018383808284375f92019190915250610e5592505050565b6109735760405162461bcd60e51b815260206004820152602e60248201527f42616420617267732e20436865636b2068656164657220616e6420617272617960448201527f2062797465206c656e677468732e000000000000000000000000000000000000606482015260840161032e565b61082d87878080601f0160208091040260200160405190810160405280939291908181526020018383808284375f9201919091525050604080516020601f8b0181900481028201810190925289815292508991508890819084018382808284375f9201919091525050604080516020601f8a0181900481028201810190925288815292508891508790819084018382808284375f920191909152506114ee92505050565b5f610a8a8686868080601f0160208091040260200160405190810160405280939291908181526020018383808284375f9201919091525050604080516020601f8a0181900481028201810190925288815292508891508790819084018382808284375f9201919091525061178092505050565b9695505050505050565b5f610aa0848484611911565b90505b9392505050565b5f610ab460025490565b9050610ac38382610800611911565b610b0f5760405162461bcd60e51b815260206004820152601b60248201527f47434420646f6573206e6f7420636f6e6669726d206865616465720000000000604482015260640161032e565b60ff821660081015610b635760405162461bcd60e51b815260206004820152601a60248201527f496e73756666696369656e7420636f6e6669726d6174696f6e73000000000000604482015260640161032e565b505050565b5160501490565b5f60208251610b7e919061212e565b1592915050565b60448101515f90610507565b5f8385148015610b9f575081155b8015610baa57508251155b15610bb7575060016104f1565b6104ee85848685611941565b5f60028083604051610bd59190612141565b602060405180830381855afa158015610bf0573d5f5f3e3d5ffd5b5050506040513d601f19601f82011682018060405250810190610c139190612157565b604051602001610c2591815260200190565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe081840301815290829052610c5d91612141565b602060405180830381855afa158015610c78573d5f5f3e3d5ffd5b5050506040513d601f19601f820116820180604052508101906105079190612157565b5f8385148015610caa57508285145b15610cb7575060016104f1565b838381815f5b86811015610cff57898314610cde575f838152600360205260409020549294505b898214610cf7575f828152600360205260409020549193505b600101610cbd565b50828403610d13575f9450505050506104f1565b808214610d26575f9450505050506104f1565b50600198975050505050505050565b5f82815b83811015610d59575f918252600360205260409091205490600101610d39565b50806105045760405162461bcd60e51b815260206004820152601060248201527f556e6b6e6f776e20616e636573746f7200000000000000000000000000000000604482015260640161032e565b5f8082815b610db86004600161219b565b63ffffffff16811015610e0c575f828152600460205260408120549350839003610df1575f918252600360205260409091205490610e04565b610dfb81846121b7565b95945050505050565b600101610dac565b5060405162461bcd60e51b815260206004820152600d60248201527f556e6b6e6f776e20626c6f636b00000000000000000000000000000000000000604482015260640161032e565b5f60508251610b7e919061212e565b5f5f610e6f85610bc3565b90505f610e7b82610da7565b90505f610e87866119e6565b90508480610e9c575080610e9a886119e6565b145b610f0d5760405162461bcd60e51b8152602060048201526024808201527f556e6578706563746564207265746172676574206f6e2065787465726e616c2060448201527f63616c6c00000000000000000000000000000000000000000000000000000000606482015260840161032e565b85515f908190815b8181101561120e57610f286050826121ca565b610f339060016121b7565b610f3d90876121b7565b9350610f4b8a8260506119f1565b5f8181526003602052604090205490935061112157846110a1845f8190506008817eff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff16901b600882901c7eff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff161790506010817dffff0000ffff0000ffff0000ffff0000ffff0000ffff0000ffff0000ffff16901b601082901c7dffff0000ffff0000ffff0000ffff0000ffff0000ffff0000ffff0000ffff161790506020817bffffffff00000000ffffffff00000000ffffffff00000000ffffffff16901b602082901c7bffffffff00000000ffffffff00000000ffffffff00000000ffffffff1617905060408177ffffffffffffffff0000000000000000ffffffffffffffff16901b604082901c77ffffffffffffffff0000000000000000ffffffffffffffff16179050608081901b608082901c179050919050565b11156110ef5760405162461bcd60e51b815260206004820152601b60248201527f48656164657220776f726b20697320696e73756666696369656e740000000000604482015260640161032e565b5f83815260036020526040902087905561110a60048561212e565b5f03611121575f8381526004602052604090208490555b8461112c8b83611a16565b146111795760405162461bcd60e51b815260206004820152601b60248201527f546172676574206368616e67656420756e65787065637465646c790000000000604482015260640161032e565b866111848b83611aaf565b146111f75760405162461bcd60e51b815260206004820152602660248201527f4865616465727320646f206e6f7420666f726d206120636f6e73697374656e7460448201527f20636861696e0000000000000000000000000000000000000000000000000000606482015260840161032e565b82965060508161120791906121b7565b9050610f15565b50816112198b610bc3565b6040517ff90e4f1d9cd0dd55e339411cbc9b152482307c3a23ed64715e4a2858f641a3f5905f90a35060019998505050505050505050565b5f6107e08211156112ca5760405162461bcd60e51b815260206004820152603360248201527f526571756573746564206c696d69742069732067726561746572207468616e2060448201527f3120646966666963756c747920706572696f6400000000000000000000000000606482015260840161032e565b5f6112d484610bc3565b90505f6112e086610bc3565b905060015481146113335760405162461bcd60e51b815260206004820181905260248201527f50617373656420696e2062657374206973206e6f742062657374206b6e6f776e604482015260640161032e565b5f8281526003602052604090205461138d5760405162461bcd60e51b815260206004820152601360248201527f4e6577206265737420697320756e6b6e6f776e00000000000000000000000000604482015260640161032e565b61139b876001548487610c9b565b61140d5760405162461bcd60e51b815260206004820152602960248201527f416e636573746f72206d75737420626520686561766965737420636f6d6d6f6e60448201527f20616e636573746f720000000000000000000000000000000000000000000000606482015260840161032e565b81611419888888611780565b1461148c5760405162461bcd60e51b815260206004820152603360248201527f4e65772062657374206861736820646f6573206e6f742068617665206d6f726560448201527f20776f726b207468616e2070726576696f757300000000000000000000000000606482015260840161032e565b600182905560028790555f6114a086611ac7565b905060055481146114b15760058190555b8783837f3cc13de64df0f0239626235c51a2da251bbc8c85664ecce39263da3ee03f606c60405160405180910390a4506001979650505050505050565b5f5f6115016114fc86610bc3565b610da7565b90505f6115106114fc86610bc3565b905061151e6107e08261212e565b6107df146115945760405162461bcd60e51b815260206004820152603d60248201527f4d7573742070726f7669646520746865206c61737420686561646572206f662060448201527f74686520636c6f73696e6720646966666963756c747920706572696f64000000606482015260840161032e565b6115a0826107df6121b7565b81146116145760405162461bcd60e51b815260206004820152602860248201527f4d7573742070726f766964652065786163746c79203120646966666963756c7460448201527f7920706572696f64000000000000000000000000000000000000000000000000606482015260840161032e565b61161d85611ac7565b61162687611ac7565b146116995760405162461bcd60e51b815260206004820152602760248201527f506572696f642068656164657220646966666963756c7469657320646f206e6f60448201527f74206d6174636800000000000000000000000000000000000000000000000000606482015260840161032e565b5f6116a3856119e6565b90505f6116d56116b2896119e6565b6116bb8a611ad9565b63ffffffff166116ca8a611ad9565b63ffffffff16611b0c565b905081818316146117285760405162461bcd60e51b815260206004820152601960248201527f496e76616c69642072657461726765742070726f766964656400000000000000604482015260640161032e565b5f61173289611ac7565b9050806006541415801561175c57506107e061174f600154610da7565b61175991906121dd565b84115b156117675760068190555b61177388886001610e64565b9998505050505050505050565b5f5f61178b85610da7565b90505f61179a6114fc86610bc3565b90505f6117a96114fc86610bc3565b90508282101580156117bb5750828110155b61182d5760405162461bcd60e51b815260206004820152603060248201527f412064657363656e64616e74206865696768742069732062656c6f772074686560448201527f20616e636573746f722068656967687400000000000000000000000000000000606482015260840161032e565b5f61183a6107e08561212e565b611846856107e06121b7565b61185091906121dd565b90508083108183108115826118625750805b1561187d5761187089610bc3565b9650505050505050610aa3565b818015611888575080155b156118965761187088610bc3565b8180156118a05750805b156118c457838510156118bb576118b688610bc3565b611870565b61187089610bc3565b6118cd88611ac7565b6118d96107e08661212e565b6118e391906121f0565b6118ec8a611ac7565b6118f86107e08861212e565b61190291906121f0565b10156118bb5761187088610bc3565b6007545f9060ff161561192f5750600754610100900460ff16610aa3565b61193a848484611b94565b9050610aa3565b5f60208451611950919061212e565b1561195c57505f6104f1565b83515f0361196b57505f6104f1565b81855f5b86518110156119d95761198360028461212e565b6001036119a7576119a061199a8883016020015190565b83611bd5565b91506119c0565b6119bd826119b88984016020015190565b611bd5565b91505b60019290921c916119d26020826121b7565b905061196f565b5090931495945050505050565b5f610507825f611a16565b5f60205f8385602001870160025afa5060205f60205f60025afa50505f519392505050565b5f80611a2d611a268460486121b7565b8590611be0565b60e81c90505f84611a3f85604b6121b7565b81518110611a4f57611a4f612207565b016020015160f81c90505f611a81835f60108262ffffff16901c8261ff001660108462ffffff16901b17179050919050565b62ffffff1690505f611a94600384612234565b60ff169050611aa581610100612330565b61082d90836121f0565b5f610504611abe8360046121b7565b84016020015190565b5f610507611ad4836119e6565b611bee565b5f610507611ae683611c15565b60d881901c63ff00ff001662ff00ff60e89290921c9190911617601081811b91901c1790565b5f80611b188385611c21565b9050611b28621275006004611c7c565b811015611b4057611b3d621275006004611c7c565b90505b611b4e621275006004611c87565b811115611b6657611b63621275006004611c87565b90505b5f611b7e82611b788862010000611c7c565b90611c87565b9050610a8a62010000611b788362127500611c7c565b5f82815b83811015611bca57858203611bb257600192505050610aa3565b5f918252600360205260409091205490600101611b98565b505f95945050505050565b5f6105048383611cfa565b5f6105048383016020015190565b5f6105077bffff000000000000000000000000000000000000000000000000000083611c7c565b5f610507826044611be0565b5f82821115611c725760405162461bcd60e51b815260206004820152601d60248201527f556e646572666c6f7720647572696e67207375627472616374696f6e2e000000604482015260640161032e565b61050482846121dd565b5f61050482846121ca565b5f825f03611c9657505f610507565b611ca082846121f0565b905081611cad84836121ca565b146105075760405162461bcd60e51b815260206004820152601f60248201527f4f766572666c6f7720647572696e67206d756c7469706c69636174696f6e2e00604482015260640161032e565b5f825f528160205260205f60405f60025afa5060205f60205f60025afa50505f5192915050565b5f5f83601f840112611d31575f5ffd5b50813567ffffffffffffffff811115611d48575f5ffd5b602083019150836020828501011115611d5f575f5ffd5b9250929050565b803560ff81168114611d76575f5ffd5b919050565b5f5f5f5f5f5f5f60a0888a031215611d91575f5ffd5b873567ffffffffffffffff811115611da7575f5ffd5b611db38a828b01611d21565b909850965050602088013567ffffffffffffffff811115611dd2575f5ffd5b611dde8a828b01611d21565b9096509450506040880135925060608801359150611dfe60808901611d66565b905092959891949750929550565b5f5f5f5f60808587031215611e1f575f5ffd5b5050823594602084013594506040840135936060013592509050565b5f5f60408385031215611e4c575f5ffd5b50508035926020909101359150565b5f60208284031215611e6b575f5ffd5b5035919050565b5f5f5f5f60408587031215611e85575f5ffd5b843567ffffffffffffffff811115611e9b575f5ffd5b611ea787828801611d21565b909550935050602085013567ffffffffffffffff811115611ec6575f5ffd5b611ed287828801611d21565b95989497509550505050565b5f5f5f5f5f5f60808789031215611ef3575f5ffd5b86359550602087013567ffffffffffffffff811115611f10575f5ffd5b611f1c89828a01611d21565b909650945050604087013567ffffffffffffffff811115611f3b575f5ffd5b611f4789828a01611d21565b979a9699509497949695606090950135949350505050565b5f5f5f5f5f5f60608789031215611f74575f5ffd5b863567ffffffffffffffff811115611f8a575f5ffd5b611f9689828a01611d21565b909750955050602087013567ffffffffffffffff811115611fb5575f5ffd5b611fc189828a01611d21565b909550935050604087013567ffffffffffffffff811115611fe0575f5ffd5b611fec89828a01611d21565b979a9699509497509295939492505050565b5f5f5f5f5f60608688031215612012575f5ffd5b85359450602086013567ffffffffffffffff81111561202f575f5ffd5b61203b88828901611d21565b909550935050604086013567ffffffffffffffff81111561205a575f5ffd5b61206688828901611d21565b969995985093965092949392505050565b5f5f5f60608486031215612089575f5ffd5b505081359360208301359350604090920135919050565b5f5f604083850312156120b1575f5ffd5b823591506120c160208401611d66565b90509250929050565b80358015158114611d76575f5ffd5b5f5f604083850312156120ea575f5ffd5b6120f3836120ca565b91506120c1602084016120ca565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f8261213c5761213c612101565b500690565b5f82518060208501845e5f920191825250919050565b5f60208284031215612167575f5ffd5b5051919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b63ffffffff81811683821601908111156105075761050761216e565b808201808211156105075761050761216e565b5f826121d8576121d8612101565b500490565b818103818111156105075761050761216e565b80820281158282048414176105075761050761216e565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b60ff82811682821603908111156105075761050761216e565b6001815b60018411156122885780850481111561226c5761226c61216e565b600184161561227a57908102905b60019390931c928002612251565b935093915050565b5f8261229e57506001610507565b816122aa57505f610507565b81600181146122c057600281146122ca576122e6565b6001915050610507565b60ff8411156122db576122db61216e565b50506001821b610507565b5060208310610133831016604e8410600b8410161715612309575081810a610507565b6123155f19848461224d565b805f19048211156123285761232861216e565b029392505050565b5f610504838361229056fea26469706673582212201142af7e12173b7a99dd453dfc892e01c9c1e5b63659b60c61d3e9d80122f9eb64736f6c634300081c00330000002073bd2184edd9c4fc76642ea6754ee40136970efc10c4190000000000000000000296ef123ea96da5cf695f22bf7d94be87d49db1ad7ac371ac43c4da4161c8c216349c5ba11928170d38782b0000000000000000000000007109709ecfa91a80626ff3989d68f67f5b1dd12d024897070000000000220020a4333e5612ab1a1043b25755c89b16d55184a42f81799e623e6bc39db8539c180000000000000000166a14edb1b5c2f39af0fec151732585b1049b0789521148e5a1a0e616d8fd92b4ef228c424e0c816799a256c6a90892195ccfc53300d6e35a0d6de94b656694589964a252957e4673a9fb1d2f8b4a92e3f0a7bb654fddb94e5a1e6d7f7f499fd1be5dd30a73bf5584bf137da5fdd77cc21aeb95b9e35788894be019284bd4fbed6dd6118ac2cb6d26bc4be4e423f55a3a48f2874d8d02a65d9c87d07de21d4dfe7b0a9f4a23cc9a58373e9e6931fefdb5afade5df54c91104048df1ee999240617984e18b6f931e2373673d0195b8c6987d7ff7650d5ce53bcec46e13ab4f2da1146a7fc621ee672f62bc22742486392d75e55e67b09960c3386a0b49e75f1723d6ab28ac9a2028a0c72866e2111d79d4817b88e17c821937847768d92837bae3832bb8e5a4ab4434b97e00a6c10182f211f592409068d6f5652400d9a3d1cc150a7fb692e874cc42d76bdafc842f2fe0f835a7c24d2d60c109b187d64571efbaa8047be85821f8e67e0e85f2f5894bc63d00c2ed9d64011746bd867400f3494b8f44c24b83e1aa58c4f0ff25b4a61cffeffd4bc0f9ba300000000000ffffffffdc20dadef477faab2852f2f8ae0c826aa7e05c4de0d36f0e63630429554884c371da5974b6f34fa2c3536738f031b49f34e0c9d084d7280f26212e39007ebe9ea0870c312745b58128a00a6557851e987ece02294d156f0020336e158928e8964292642c6c4dc469f34b7bacf2d8c42115bab6afc9067f2ed30e8749729b63e0889e203ee58e355903c1e71f78c008df6c3597b2cc66d0b8aae1a4a33caa775498e531cfb6af58e87db99e0f536dd226d18f43e3864148ba5b7faca5c775f10bc810c602e1af2195a34577976921ce009a4ddc0a07f605c96b0f5fcf580831ebbe01a31fa29bde884609d286dccfa5ba8e558ce3125bd4c3a19e888cf26852286202d2a7d302c75e0ff5ca8fe7299fb0d9d1132bf2c56c2e3b73df799286193d60c109b187d64571efbaa8047be85821f8e67e0e85f2f5894bc63d00c2ed9d64
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x0C\x80T`\x01`\xFF\x19\x91\x82\x16\x81\x17\x90\x92U`\x1F\x80T\x90\x91\x16\x90\x91\x17\x90Ua\x01\0`@R`P`\x80\x81\x81R\x90au\xB1`\xA09`!\x90a\0=\x90\x82a\t\x81V[P`@Q\x80a\x01`\x01`@R\x80a\x01@\x81R` \x01av\x8Ca\x01@\x919`\"\x90a\0g\x90\x82a\t\x81V[P_Q` avl_9_Q\x90_R`#Ua\x01\x19`$U`@Qa\0\x8B\x90a\x08\xCFV[`@Q\x80\x91\x03\x90_\xF0\x80\x15\x80\x15a\0\xA4W=__>=_\xFD[P`%_a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP`\x01`&U_Q` avl_9_Q\x90_R`'U`@Q\x80`\x80\x01`@R\x80`\x01`\xF8\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01`@Q\x80``\x01`@R\x80`*\x81R` \x01aw\xCC`*\x919\x81R` \x01`@Q\x80`\x80\x01`@R\x80`K\x81R` \x01av!`K\x919\x81R_` \x91\x82\x01R\x81Q`(\x80Tc\xFF\xFF\xFF\xFF\x19\x16`\xE0\x92\x90\x92\x1C\x91\x90\x91\x17\x81U\x90\x82\x01Q`)\x90a\x01i\x90\x82a\t\x81V[P`@\x82\x01Q`\x02\x82\x01\x90a\x01~\x90\x82a\t\x81V[P``\x82\x01Q\x81`\x03\x01_a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83`\xE0\x1C\x02\x17\x90UPPP`@Q\x80`\xA0\x01`@R\x80`\"\x80Ta\x01\xBC\x90a\x08\xFDV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x01\xE8\x90a\x08\xFDV[\x80\x15a\x023W\x80`\x1F\x10a\x02\nWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x023V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x02\x16W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01a\x01\x19\x81R` \x01`!\x80Ta\x02R\x90a\x08\xFDV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x02~\x90a\x08\xFDV[\x80\x15a\x02\xC9W\x80`\x1F\x10a\x02\xA0Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x02\xC9V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x02\xACW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x7Fw\xB9\x8A^fC\x97;\xBAI\xDD\xA1\x8Au\x14\x03\x06\xD2\xD8iKf\xF2\xDC\xB3V\x1A\xD5\xAF\xF0\xB0\xC7\x81R` \x01`@Q\x80a\x01`\x01`@R\x80a\x01@\x81R` \x01aw\xF6a\x01@\x919\x90R\x80Q`,\x90\x81\x90a\x03(\x90\x82a\t\x81V[P` \x82\x01Q`\x01\x82\x01U`@\x82\x01Q`\x02\x82\x01\x90a\x03G\x90\x82a\t\x81V[P``\x82\x01Q`\x03\x82\x01U`\x80\x82\x01Q`\x04\x82\x01\x90a\x03f\x90\x82a\t\x81V[PPP4\x80\x15a\x03tW__\xFD[P`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01k42\xB0\xB22\xB99\x9759\xB7\xB7`\xA1\x1B\x81RP`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01k\x05\xCC\xEC\xAD\xCC\xAEm.e\xCD\x0C\xAF`\xA3\x1B\x81RP`@Q\x80`@\x01`@R\x80`\x0F\x81R` \x01n\x0B\x99\xD9[\x99\\\xDA\\\xCB\x9A\x19ZY\xDA\x1D`\x8A\x1B\x81RP`@Q\x80`@\x01`@R\x80`\x12\x81R` \x01q.genesis.digest_le`p\x1B\x81RP__Q` av\x01_9_Q\x90_R`\x01`\x01`\xA0\x1B\x03\x16c\xD90\xA0\xE6`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04[W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x04\x82\x91\x90\x81\x01\x90a\n\xB0V[\x90P_\x81\x86`@Q` \x01a\x04\x98\x92\x91\x90a\x0B\x13V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Rc`\xF9\xBB\x11`\xE0\x1B\x82R\x91P_Q` av\x01_9_Q\x90_R\x90c`\xF9\xBB\x11\x90a\x04\xD8\x90\x84\x90`\x04\x01a\x0B\x85V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xF2W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x05\x19\x91\x90\x81\x01\x90a\n\xB0V[` \x90a\x05&\x90\x82a\t\x81V[Pa\x05\xBD\x85` \x80Ta\x058\x90a\x08\xFDV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x05d\x90a\x08\xFDV[\x80\x15a\x05\xAFW\x80`\x1F\x10a\x05\x86Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x05\xAFV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x05\x92W\x82\x90\x03`\x1F\x16\x82\x01\x91[P\x93\x94\x93PPa\x07\xAB\x91PPV[a\x06S\x85` \x80Ta\x05\xCE\x90a\x08\xFDV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x05\xFA\x90a\x08\xFDV[\x80\x15a\x06EW\x80`\x1F\x10a\x06\x1CWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x06EV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x06(W\x82\x90\x03`\x1F\x16\x82\x01\x91[P\x93\x94\x93PPa\x08(\x91PPV[a\x06\xE9\x85` \x80Ta\x06d\x90a\x08\xFDV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x06\x90\x90a\x08\xFDV[\x80\x15a\x06\xDBW\x80`\x1F\x10a\x06\xB2Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x06\xDBV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x06\xBEW\x82\x90\x03`\x1F\x16\x82\x01\x91[P\x93\x94\x93PPa\x08\x9B\x91PPV[`@Qa\x06\xF5\x90a\x08\xDCV[a\x07\x01\x93\x92\x91\x90a\x0B\x97V[`@Q\x80\x91\x03\x90_\xF0\x80\x15\x80\x15a\x07\x1AW=__>=_\xFD[P`\x1F\x80Ta\x01\0`\x01`\xA8\x1B\x03\x19\x16a\x01\0`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81\x02\x91\x90\x91\x17\x91\x82\x90U`@Qc\xF5\x8D\xB0o`\xE0\x1B\x81R`\x01`\x04\x82\x01\x81\x90R`$\x82\x01R\x91\x04\x90\x91\x16\x96Pc\xF5\x8D\xB0o\x95P`D\x01\x93Pa\x07y\x92PPPV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x07\x90W__\xFD[PZ\xF1\x15\x80\x15a\x07\xA2W=__>=_\xFD[PPPPa\x0B\xF6V[`@Qc\x1F\xB2C}`\xE3\x1B\x81R``\x90_Q` av\x01_9_Q\x90_R\x90c\xFD\x92\x1B\xE8\x90a\x07\xE0\x90\x86\x90\x86\x90`\x04\x01a\x0B\xBBV[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\xFAW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x08!\x91\x90\x81\x01\x90a\n\xB0V[\x93\x92PPPV[`@QcV\xEE\xF1[`\xE1\x1B\x81R_\x90_Q` av\x01_9_Q\x90_R\x90c\xAD\xDD\xE2\xB6\x90a\x08\\\x90\x86\x90\x86\x90`\x04\x01a\x0B\xBBV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08wW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08!\x91\x90a\x0B\xDFV[`@Qc\x17w\xE5\x9D`\xE0\x1B\x81R_\x90_Q` av\x01_9_Q\x90_R\x90c\x17w\xE5\x9D\x90a\x08\\\x90\x86\x90\x86\x90`\x04\x01a\x0B\xBBV[a\x0F\xF9\x80a<}\x839\x01\x90V[a);\x80aLv\x839\x01\x90V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`\x01\x81\x81\x1C\x90\x82\x16\x80a\t\x11W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\t/WcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\t|W\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a\tZWP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\tyW_\x81U`\x01\x01a\tfV[PP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\t\x9AWa\t\x9Aa\x08\xE9V[a\t\xAE\x81a\t\xA8\x84Ta\x08\xFDV[\x84a\t5V[` `\x1F\x82\x11`\x01\x81\x14a\t\xE0W_\x83\x15a\t\xC9WP\x84\x82\x01Q[_\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua\tyV[_\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a\n\x0FW\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a\t\xEFV[P\x84\x82\x10\x15a\n,W\x86\x84\x01Q_\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[_\x80`\x01`\x01`@\x1B\x03\x84\x11\x15a\nTWa\nTa\x08\xE9V[P`@Q`\x1F\x19`\x1F\x85\x01\x81\x16`?\x01\x16\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a\n\x82Wa\n\x82a\x08\xE9V[`@R\x83\x81R\x90P\x80\x82\x84\x01\x85\x10\x15a\n\x99W__\xFD[\x83\x83` \x83\x01^_` \x85\x83\x01\x01RP\x93\x92PPPV[_` \x82\x84\x03\x12\x15a\n\xC0W__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\n\xD5W__\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\n\xE5W__\xFD[a\n\xF4\x84\x82Q` \x84\x01a\n;V[\x94\x93PPPPV[_\x81Q\x80` \x84\x01\x85^_\x93\x01\x92\x83RP\x90\x91\x90PV[_a\x0B\x1E\x82\x85a\n\xFCV[\x7F/test/fullRelay/testData/\0\0\0\0\0\0\0\x81Ra\x0BN`\x19\x82\x01\x85a\n\xFCV[\x95\x94PPPPPV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R_a\x08!` \x83\x01\x84a\x0BWV[``\x81R_a\x0B\xA9``\x83\x01\x86a\x0BWV[` \x83\x01\x94\x90\x94RP`@\x01R\x91\x90PV[`@\x81R_a\x0B\xCD`@\x83\x01\x85a\x0BWV[\x82\x81\x03` \x84\x01Ra\x0BN\x81\x85a\x0BWV[_` \x82\x84\x03\x12\x15a\x0B\xEFW__\xFD[PQ\x91\x90PV[a0z\x80a\x0C\x03_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x01\x9AW_5`\xE0\x1C\x80c\x91j\x17\xC6\x11a\0\xE8W\x80c\xE2\x0C\x9Fq\x11a\0\x93W\x80c\xEF>h\x12\x11a\0nW\x80c\xEF>h\x12\x14a\x02\xE2W\x80c\xF1\x1D\\\xBC\x14a\x02\xEAW\x80c\xFAv&\xD4\x14a\x02\xF2W\x80c\xFA\xD0k\x8F\x14a\x02\xFFW__\xFD[\x80c\xE2\x0C\x9Fq\x14a\x02\xD2W\x80c\xE8\x9F\x84\x19\x14a\x02\xDAW\x80c\xEDZ\x9CI\x14a\x02\xDAW__\xFD[\x80c\xB5P\x8A\xA9\x11a\0\xC3W\x80c\xB5P\x8A\xA9\x14a\x02\xAAW\x80c\xBAAO\xA6\x14a\x02\xB2W\x80c\xC8\x99\xD2A\x14a\x02\xCAW__\xFD[\x80c\x91j\x17\xC6\x14a\x02\x85W\x80c\xB0FO\xDC\x14a\x02\x9AW\x80c\xB5+ X\x14a\x02\xA2W__\xFD[\x80c?r\x86\xF4\x11a\x01HW\x80cc*\xD54\x11a\x01#W\x80cc*\xD54\x14a\x02SW\x80cf\xD9\xA9\xA0\x14a\x02[W\x80c\x85\"l\x81\x14a\x02pW__\xFD[\x80c?r\x86\xF4\x14a\x02#W\x80cD\xBA\xDB\xB6\x14a\x02+W\x80c]\xF0\xD0v\x14a\x02KW__\xFD[\x80c*\xDE8\x80\x11a\x01xW\x80c*\xDE8\x80\x14a\x01\xFCW\x80c9B_\x8F\x14a\x02\x11W\x80c>^<#\x14a\x02\x1BW__\xFD[\x80c\x08\x13\x85*\x14a\x01\x9EW\x80c\x1C\r\xA8\x1F\x14a\x01\xC7W\x80c\x1E\xD7\x83\x1C\x14a\x01\xE7W[__\xFD[a\x01\xB1a\x01\xAC6`\x04a%DV[a\x03\x12V[`@Qa\x01\xBE\x91\x90a%\xF9V[`@Q\x80\x91\x03\x90\xF3[a\x01\xDAa\x01\xD56`\x04a%DV[a\x03]V[`@Qa\x01\xBE\x91\x90a&\\V[a\x01\xEFa\x03\xCFV[`@Qa\x01\xBE\x91\x90a&nV[a\x02\x04a\x04<V[`@Qa\x01\xBE\x91\x90a' V[a\x02\x19a\x05\x85V[\0[a\x01\xEFa\x06\xE1V[a\x01\xEFa\x07LV[a\x02>a\x0296`\x04a%DV[a\x07\xB7V[`@Qa\x01\xBE\x91\x90a'\xA4V[a\x02\x19a\x07\xFAV[a\x02\x19a\n\xB5V[a\x02ca\r\xBFV[`@Qa\x01\xBE\x91\x90a(7V[a\x02xa\x0F8V[`@Qa\x01\xBE\x91\x90a(\xB5V[a\x02\x8Da\x10\x03V[`@Qa\x01\xBE\x91\x90a(\xC7V[a\x02\x8Da\x11\x06V[a\x02\x19a\x12\tV[a\x02xa\x12\xC6V[a\x02\xBAa\x13\x91V[`@Q\x90\x15\x15\x81R` \x01a\x01\xBEV[a\x02\x19a\x14aV[a\x01\xEFa\x16eV[a\x02\x19a\x16\xD0V[a\x02\x19a\x19@V[a\x02\x19a\x1BeV[`\x1FTa\x02\xBA\x90`\xFF\x16\x81V[a\x02>a\x03\r6`\x04a%DV[a\x1DEV[``a\x03U\x84\x84\x84`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01\x7Fhex\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x1D\x88V[\x94\x93PPPPV[``_a\x03k\x85\x85\x85a\x03\x12V[\x90P_[a\x03y\x85\x85a)xV[\x81\x10\x15a\x03\xC6W\x82\x82\x82\x81Q\x81\x10a\x03\x93Wa\x03\x93a)\x8BV[` \x02` \x01\x01Q`@Q` \x01a\x03\xAC\x92\x91\x90a)\xCFV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x92P`\x01\x01a\x03oV[PP\x93\x92PPPV[```\x16\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x042W` \x02\x82\x01\x91\x90_R` _ \x90[\x81Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x04\x07W[PPPPP\x90P\x90V[```\x1E\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x05|W_\x84\x81R` \x80\x82 `@\x80Q\x80\x82\x01\x82R`\x02\x87\x02\x90\x92\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x95\x91\x94\x86\x81\x01\x94\x91\x92\x90\x84\x01[\x82\x82\x10\x15a\x05eW\x83\x82\x90_R` _ \x01\x80Ta\x04\xDA\x90a)\xE3V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x05\x06\x90a)\xE3V[\x80\x15a\x05QW\x80`\x1F\x10a\x05(Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x05QV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x054W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x04\xBDV[PPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x04_V[PPPP\x90P\x90V[`@\x80Q\x80\x82\x01\x82R`\x1A\x81R\x7FInsufficient confirmations\0\0\0\0\0\0` \x82\x01R\x90Q\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\t\x91sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x91c\xF2\x8D\xCE\xB3\x91a\x06\x08\x91`\x04\x01a&\\V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x06\x1FW__\xFD[PZ\xF1\x15\x80\x15a\x061W=__>=_\xFD[PP`%T`\x1FT`@Q\x7F-y\xDBF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x16\x94Pc-y\xDBF\x93Pa\x06\x9E\x92a\x01\0\x90\x92\x04\x90\x91\x16\x90\x85\x90`(\x90`,\x90`\x04\x01a+\xECV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\xB9W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xDD\x91\x90a,=V[PPV[```\x18\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x042W` \x02\x82\x01\x91\x90_R` _ \x90\x81Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x04\x07WPPPPP\x90P\x90V[```\x17\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x042W` \x02\x82\x01\x91\x90_R` _ \x90\x81Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x04\x07WPPPPP\x90P\x90V[``a\x03U\x84\x84\x84`@Q\x80`@\x01`@R\x80`\t\x81R` \x01\x7Fdigest_le\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x1E\xE9V[`@\x80Q`\x80\x81\x01\x90\x91R`(\x80T`\xE0\x1B\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x82R`)\x80T_\x93\x92\x91` \x84\x01\x91a\x08F\x90a)\xE3V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x08r\x90a)\xE3V[\x80\x15a\x08\xBDW\x80`\x1F\x10a\x08\x94Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x08\xBDV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x08\xA0W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x02\x82\x01\x80Ta\x08\xD6\x90a)\xE3V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\t\x02\x90a)\xE3V[\x80\x15a\tMW\x80`\x1F\x10a\t$Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\tMV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\t0W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x83RPP`\x03\x91\x90\x91\x01T`\xE0\x1B\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16` \x91\x82\x01R`@\x80Q\x80\x82\x01\x82R`\x01\x81R_\x81\x84\x01R\x83\x82\x01R\x80Q\x80\x82\x01\x82R`\x1E\x81R\x7FInvalid output vector provided\0\0\x92\x81\x01\x92\x90\x92RQ\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x91\x92Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x91c\xF2\x8D\xCE\xB3\x91a\n\x1F\x91`\x04\x01a&\\V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\n6W__\xFD[PZ\xF1\x15\x80\x15a\nHW=__>=_\xFD[PP`%T`\x1FT`&T`@Q\x7F-y\xDBF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x84\x16\x95Pc-y\xDBF\x94Pa\x06\x9E\x93a\x01\0\x90\x93\x04\x90\x92\x16\x91\x86\x90`,\x90`\x04\x01a,TV[_`,`@Q\x80`\xA0\x01`@R\x90\x81_\x82\x01\x80Ta\n\xD2\x90a)\xE3V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\n\xFE\x90a)\xE3V[\x80\x15a\x0BIW\x80`\x1F\x10a\x0B Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0BIV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0B,W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x01\x82\x01T\x81R` \x01`\x02\x82\x01\x80Ta\x0Bl\x90a)\xE3V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0B\x98\x90a)\xE3V[\x80\x15a\x0B\xE3W\x80`\x1F\x10a\x0B\xBAWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0B\xE3V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0B\xC6W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x03\x82\x01T\x81R` \x01`\x04\x82\x01\x80Ta\x0C\x06\x90a)\xE3V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0C2\x90a)\xE3V[\x80\x15a\x0C}W\x80`\x1F\x10a\x0CTWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0C}V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0C`W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P`@Q\x80`\x80\x01`@R\x80`O\x81R` \x01a/\xF6`O\x919`@\x80\x83\x01\x91\x90\x91R\x80Q\x80\x82\x01\x82R`\x10\x81R\x7FBad header block\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x90Q\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x91c\xF2\x8D\xCE\xB3\x91a\r)\x91\x90`\x04\x01a&\\V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\r@W__\xFD[PZ\xF1\x15\x80\x15a\rRW=__>=_\xFD[PP`%T`\x1FT`&T`@Q\x7F-y\xDBF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x84\x16\x95Pc-y\xDBF\x94Pa\x06\x9E\x93a\x01\0\x90\x93\x04\x90\x92\x16\x91`(\x90\x87\x90`\x04\x01a-:V[```\x1B\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x05|W\x83\x82\x90_R` _ \x90`\x02\x02\x01`@Q\x80`@\x01`@R\x90\x81_\x82\x01\x80Ta\x0E\x12\x90a)\xE3V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0E>\x90a)\xE3V[\x80\x15a\x0E\x89W\x80`\x1F\x10a\x0E`Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0E\x89V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0ElW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x01\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x0F W` \x02\x82\x01\x91\x90_R` _ \x90_\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x0E\xCDW\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\r\xE2V[```\x1A\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x05|W\x83\x82\x90_R` _ \x01\x80Ta\x0Fx\x90a)\xE3V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0F\xA4\x90a)\xE3V[\x80\x15a\x0F\xEFW\x80`\x1F\x10a\x0F\xC6Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0F\xEFV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0F\xD2W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x0F[V[```\x1D\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x05|W_\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x10\xEEW` \x02\x82\x01\x91\x90_R` _ \x90_\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x10\x9BW\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x10&V[```\x1C\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x05|W_\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x11\xF1W` \x02\x82\x01\x91\x90_R` _ \x90_\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x11\x9EW\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x11)V[`%T`\x1FT`&T`@Q\x7F-y\xDBF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R_\x93s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x93c-y\xDBF\x93a\x12v\x93a\x01\0\x90\x92\x04\x90\x92\x16\x91\x90`(\x90`,\x90`\x04\x01a+\xECV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12\x91W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\xB5\x91\x90a,=V[\x90Pa\x12\xC3\x81`'Ta 7V[PV[```\x19\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x05|W\x83\x82\x90_R` _ \x01\x80Ta\x13\x06\x90a)\xE3V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x132\x90a)\xE3V[\x80\x15a\x13}W\x80`\x1F\x10a\x13TWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x13}V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x13`W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x12\xE9V[`\x08T_\x90`\xFF\x16\x15a\x13\xA8WP`\x08T`\xFF\x16\x90V[`@Q\x7Ff\x7F\x9Dp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-`\x04\x82\x01\x81\x90R\x7Ffailed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`$\x83\x01R_\x91cf\x7F\x9Dp\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x146W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14Z\x91\x90a,=V[\x14\x15\x90P\x90V[`@\x80Q`\x80\x81\x01\x90\x91R`(\x80T`\xE0\x1B\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x82R`)\x80T_\x93\x92\x91` \x84\x01\x91a\x14\xAD\x90a)\xE3V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x14\xD9\x90a)\xE3V[\x80\x15a\x15$W\x80`\x1F\x10a\x14\xFBWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x15$V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x15\x07W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x02\x82\x01\x80Ta\x15=\x90a)\xE3V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x15i\x90a)\xE3V[\x80\x15a\x15\xB4W\x80`\x1F\x10a\x15\x8BWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x15\xB4V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x15\x97W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x83RPP`\x03\x91\x90\x91\x01T`\xE0\x1B\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16` \x91\x82\x01R|\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x83\x81\x01\x91\x90\x91R`@\x80Q\x91\x82\x01\x90R`<\x80\x82R\x92\x93Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x92c\xF2\x8D\xCE\xB3\x92a/\xBA\x90\x83\x019`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\n\x1F\x91\x90a&\\V[```\x15\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x042W` \x02\x82\x01\x91\x90_R` _ \x90\x81Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x04\x07WPPPPP\x90P\x90V[_`,`@Q\x80`\xA0\x01`@R\x90\x81_\x82\x01\x80Ta\x16\xED\x90a)\xE3V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x17\x19\x90a)\xE3V[\x80\x15a\x17dW\x80`\x1F\x10a\x17;Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x17dV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x17GW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x01\x82\x01T\x81R` \x01`\x02\x82\x01\x80Ta\x17\x87\x90a)\xE3V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x17\xB3\x90a)\xE3V[\x80\x15a\x17\xFEW\x80`\x1F\x10a\x17\xD5Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x17\xFEV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x17\xE1W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x03\x82\x01T\x81R` \x01`\x04\x82\x01\x80Ta\x18!\x90a)\xE3V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x18M\x90a)\xE3V[\x80\x15a\x18\x98W\x80`\x1F\x10a\x18oWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x18\x98V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x18{W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x90\x92RPP`@\x80Q\x80\x82\x01\x82R`\x01\x81R_` \x80\x83\x01\x91\x90\x91R\x90\x84R\x81Q\x80\x83\x01\x83R`\x16\x81R\x7FBad merkle array proof\0\0\0\0\0\0\0\0\0\0\x91\x81\x01\x91\x90\x91R\x90Q\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x92\x93Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x92c\xF2\x8D\xCE\xB3\x92Pa\r)\x91\x90`\x04\x01a&\\V[`@\x80Q`\x80\x81\x01\x90\x91R`(\x80T`\xE0\x1B\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x82R`)\x80T_\x93\x92\x91` \x84\x01\x91a\x19\x8C\x90a)\xE3V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x19\xB8\x90a)\xE3V[\x80\x15a\x1A\x03W\x80`\x1F\x10a\x19\xDAWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1A\x03V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x19\xE6W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x02\x82\x01\x80Ta\x1A\x1C\x90a)\xE3V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1AH\x90a)\xE3V[\x80\x15a\x1A\x93W\x80`\x1F\x10a\x1AjWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1A\x93V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1AvW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x83RPP`\x03\x91\x90\x91\x01T`\xE0\x1B\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16` \x91\x82\x01R`@\x80Q\x80\x82\x01\x82R`\x01\x81R_\x81\x84\x01R\x83\x83\x01R\x80Q\x80\x82\x01\x82R`\x1D\x81R\x7FInvalid input vector provided\0\0\0\x92\x81\x01\x92\x90\x92RQ\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x91\x92Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x91c\xF2\x8D\xCE\xB3\x91a\n\x1F\x91`\x04\x01a&\\V[`\x1FT`@Q\x7F\xF5\x8D\xB0o\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R_`\x04\x82\x01\x81\x90R`$\x82\x01Ra\x01\0\x90\x91\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90c\xF5\x8D\xB0o\x90`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1B\xD8W__\xFD[PZ\xF1\x15\x80\x15a\x1B\xEAW=__>=_\xFD[PP`@\x80Q\x80\x82\x01\x82R`\x1B\x81R\x7FGCD does not confirm header\0\0\0\0\0` \x82\x01R\x90Q\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x93Pc\xF2\x8D\xCE\xB3\x92Pa\x1Co\x91\x90`\x04\x01a&\\V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1C\x86W__\xFD[PZ\xF1\x15\x80\x15a\x1C\x98W=__>=_\xFD[PP`%T`\x1FT`&T`@Q\x7F-y\xDBF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x84\x16\x95Pc-y\xDBF\x94Pa\x1D\x06\x93a\x01\0\x90\x93\x04\x90\x92\x16\x91`(\x90`,\x90`\x04\x01a+\xECV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1D!W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\xC3\x91\x90a,=V[``a\x03U\x84\x84\x84`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01\x7Fheight\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa \xBAV[``a\x1D\x94\x84\x84a)xV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1D\xACWa\x1D\xACa$\xBFV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1D\xDFW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x1D\xCAW\x90P[P\x90P\x83[\x83\x81\x10\x15a\x1E\xE0Wa\x1E\xB2\x86a\x1D\xF9\x83a\"\x08V[\x85`@Q` \x01a\x1E\x0C\x93\x92\x91\x90a-\xDDV[`@Q` \x81\x83\x03\x03\x81R\x90`@R` \x80Ta\x1E(\x90a)\xE3V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1ET\x90a)\xE3V[\x80\x15a\x1E\x9FW\x80`\x1F\x10a\x1EvWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1E\x9FV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1E\x82W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPPa#9\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x82a\x1E\xBD\x87\x84a)xV[\x81Q\x81\x10a\x1E\xCDWa\x1E\xCDa)\x8BV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x1D\xE4V[P\x94\x93PPPPV[``a\x1E\xF5\x84\x84a)xV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1F\rWa\x1F\ra$\xBFV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1F6W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P\x83[\x83\x81\x10\x15a\x1E\xE0Wa \t\x86a\x1FP\x83a\"\x08V[\x85`@Q` \x01a\x1Fc\x93\x92\x91\x90a-\xDDV[`@Q` \x81\x83\x03\x03\x81R\x90`@R` \x80Ta\x1F\x7F\x90a)\xE3V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1F\xAB\x90a)\xE3V[\x80\x15a\x1F\xF6W\x80`\x1F\x10a\x1F\xCDWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1F\xF6V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1F\xD9W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPPa#\xD8\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x82a \x14\x87\x84a)xV[\x81Q\x81\x10a $Wa $a)\x8BV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x1F;V[`@Q\x7F|\x84\xC6\x9B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x82\x90Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c|\x84\xC6\x9B\x90`D\x01_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a \xA0W__\xFD[PZ\xFA\x15\x80\x15a \xB2W=__>=_\xFD[PPPPPPV[``a \xC6\x84\x84a)xV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a \xDEWa \xDEa$\xBFV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a!\x07W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P\x83[\x83\x81\x10\x15a\x1E\xE0Wa!\xDA\x86a!!\x83a\"\x08V[\x85`@Q` \x01a!4\x93\x92\x91\x90a-\xDDV[`@Q` \x81\x83\x03\x03\x81R\x90`@R` \x80Ta!P\x90a)\xE3V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta!|\x90a)\xE3V[\x80\x15a!\xC7W\x80`\x1F\x10a!\x9EWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a!\xC7V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a!\xAAW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPPa$k\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x82a!\xE5\x87\x84a)xV[\x81Q\x81\x10a!\xF5Wa!\xF5a)\x8BV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a!\x0CV[``\x81_\x03a\"JWPP`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R\x7F0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x90V[\x81_[\x81\x15a\"sW\x80a\"]\x81a.zV[\x91Pa\"l\x90P`\n\x83a.\xDEV[\x91Pa\"MV[_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\"\x8DWa\"\x8Da$\xBFV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\"\xB7W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P[\x84\x15a\x03UWa\"\xCC`\x01\x83a)xV[\x91Pa\"\xD9`\n\x86a.\xF1V[a\"\xE4\x90`0a/\x04V[`\xF8\x1B\x81\x83\x81Q\x81\x10a\"\xF9Wa\"\xF9a)\x8BV[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81_\x1A\x90SPa#2`\n\x86a.\xDEV[\x94Pa\"\xBBV[`@Q\x7F\xFD\x92\x1B\xE8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R``\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\xFD\x92\x1B\xE8\x90a#\x8E\x90\x86\x90\x86\x90`\x04\x01a/\x17V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a#\xA8W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra#\xCF\x91\x90\x81\x01\x90a/DV[\x90P[\x92\x91PPV[`@Q\x7F\x17w\xE5\x9D\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R_\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\x17w\xE5\x9D\x90a$,\x90\x86\x90\x86\x90`\x04\x01a/\x17V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a$GW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a#\xCF\x91\x90a,=V[`@Q\x7F\xAD\xDD\xE2\xB6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R_\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\xAD\xDD\xE2\xB6\x90a$,\x90\x86\x90\x86\x90`\x04\x01a/\x17V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a%\x15Wa%\x15a$\xBFV[`@R\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a%6Wa%6a$\xBFV[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[___``\x84\x86\x03\x12\x15a%VW__\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a%lW__\xFD[\x84\x01`\x1F\x81\x01\x86\x13a%|W__\xFD[\x805a%\x8Fa%\x8A\x82a%\x1DV[a$\xECV[\x81\x81R\x87` \x83\x85\x01\x01\x11\x15a%\xA3W__\xFD[\x81` \x84\x01` \x83\x017_` \x92\x82\x01\x83\x01R\x97\x90\x86\x015\x96P`@\x90\x95\x015\x94\x93PPPPV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a&PW`?\x19\x87\x86\x03\x01\x84Ra&;\x85\x83Qa%\xCBV[\x94P` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a&\x1FV[P\x92\x96\x95PPPPPPV[` \x81R_a#\xCF` \x83\x01\x84a%\xCBV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a&\xBBW\x83Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a&\x87V[P\x90\x95\x94PPPPPV[_\x82\x82Q\x80\x85R` \x85\x01\x94P` \x81`\x05\x1B\x83\x01\x01` \x85\x01_[\x83\x81\x10\x15a'\x14W`\x1F\x19\x85\x84\x03\x01\x88Ra&\xFE\x83\x83Qa%\xCBV[` \x98\x89\x01\x98\x90\x93P\x91\x90\x91\x01\x90`\x01\x01a&\xE2V[P\x90\x96\x95PPPPPPV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a&PW`?\x19\x87\x86\x03\x01\x84R\x81Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16\x86R` \x81\x01Q\x90P`@` \x87\x01Ra'\x8E`@\x87\x01\x82a&\xC6V[\x95PP` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a'FV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a&\xBBW\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a'\xBDV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15a(-W\x81Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01a'\xEDV[P\x93\x94\x93PPPPV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a&PW`?\x19\x87\x86\x03\x01\x84R\x81Q\x80Q`@\x87Ra(\x83`@\x88\x01\x82a%\xCBV[\x90P` \x82\x01Q\x91P\x86\x81\x03` \x88\x01Ra(\x9E\x81\x83a'\xDBV[\x96PPP` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a(]V[` \x81R_a#\xCF` \x83\x01\x84a&\xC6V[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a&PW`?\x19\x87\x86\x03\x01\x84R\x81Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16\x86R` \x81\x01Q\x90P`@` \x87\x01Ra)5`@\x87\x01\x82a'\xDBV[\x95PP` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a(\xEDV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a#\xD2Wa#\xD2a)KV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_\x81Q\x80` \x84\x01\x85^_\x93\x01\x92\x83RP\x90\x91\x90PV[_a\x03Ua)\xDD\x83\x86a)\xB8V[\x84a)\xB8V[`\x01\x81\x81\x1C\x90\x82\x16\x80a)\xF7W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a*.W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[P\x91\x90PV[\x80T_\x90`\x01\x81\x81\x1C\x90\x82\x16\x80a*LW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a*\x83W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[\x81\x86R` \x86\x01\x81\x80\x15a*\x9EW`\x01\x81\x14a*\xD2Wa*\xFEV[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x85\x16\x82R\x83\x15\x15`\x05\x1B\x82\x01\x95Pa*\xFEV[_\x87\x81R` \x90 _[\x85\x81\x10\x15a*\xF8W\x81T\x84\x82\x01R`\x01\x90\x91\x01\x90` \x01a*\xDCV[\x83\x01\x96PP[PPPPP\x92\x91PPV[\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81T`\xE0\x1B\x16\x82R`\x80` \x83\x01R_a+J`\x80\x84\x01`\x01\x84\x01a*4V[\x83\x81\x03`@\x85\x01Ra+_\x81`\x02\x85\x01a*4V[\x90P\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x03\x84\x01T`\xE0\x1B\x16``\x85\x01R\x80\x91PP\x92\x91PPV[`\xA0\x82R_a+\xAC`\xA0\x84\x01\x83a*4V[`\x01\x83\x01T` \x85\x01R\x83\x81\x03`@\x85\x01Ra+\xCB\x81`\x02\x85\x01a*4V[\x90P`\x03\x83\x01T``\x85\x01R\x83\x81\x03`\x80\x85\x01Ra\x03U\x81`\x04\x85\x01a*4V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x81R\x83` \x82\x01R`\x80`@\x82\x01R_a, `\x80\x83\x01\x85a+\tV[\x82\x81\x03``\x84\x01Ra,2\x81\x85a+\x9AV[\x97\x96PPPPPPPV[_` \x82\x84\x03\x12\x15a,MW__\xFD[PQ\x91\x90PV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x81R\x83` \x82\x01R`\x80`@\x82\x01R\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83Q\x16`\x80\x82\x01R_` \x84\x01Q`\x80`\xA0\x84\x01Ra,\xBEa\x01\0\x84\x01\x82a%\xCBV[\x90P`@\x85\x01Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x84\x83\x03\x01`\xC0\x85\x01Ra,\xF9\x82\x82a%\xCBV[\x91PP\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x86\x01Q\x16`\xE0\x84\x01R\x82\x81\x03``\x84\x01Ra,2\x81\x85a+\x9AV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x81R\x83` \x82\x01R`\x80`@\x82\x01R_a-n`\x80\x83\x01\x85a+\tV[\x82\x81\x03``\x84\x01R\x83Q`\xA0\x82Ra-\x89`\xA0\x83\x01\x82a%\xCBV[\x90P` \x85\x01Q` \x83\x01R`@\x85\x01Q\x82\x82\x03`@\x84\x01Ra-\xAC\x82\x82a%\xCBV[\x91PP``\x85\x01Q``\x83\x01R`\x80\x85\x01Q\x82\x82\x03`\x80\x84\x01Ra-\xD0\x82\x82a%\xCBV[\x99\x98PPPPPPPPPV[\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R_a.\x0E`\x01\x83\x01\x86a)\xB8V[\x7F[\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Ra.>`\x01\x82\x01\x86a)\xB8V[\x90P\x7F].\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Ra.p`\x02\x82\x01\x85a)\xB8V[\x96\x95PPPPPPV[_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a.\xAAWa.\xAAa)KV[P`\x01\x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_\x82a.\xECWa.\xECa.\xB1V[P\x04\x90V[_\x82a.\xFFWa.\xFFa.\xB1V[P\x06\x90V[\x80\x82\x01\x80\x82\x11\x15a#\xD2Wa#\xD2a)KV[`@\x81R_a/)`@\x83\x01\x85a%\xCBV[\x82\x81\x03` \x84\x01Ra/;\x81\x85a%\xCBV[\x95\x94PPPPPV[_` \x82\x84\x03\x12\x15a/TW__\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a/jW__\xFD[\x82\x01`\x1F\x81\x01\x84\x13a/zW__\xFD[\x80Qa/\x88a%\x8A\x82a%\x1DV[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a/\x9CW__\xFD[\x81` \x84\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV\xFETx merkle proof is not valid for provided header and tx hash\0\0\0 s\xBD!\x84\xED\xD9\xC4\xFCvd.\xA6uN\xE4\x016\x97\x0E\xFC\x10\xC4\x19\0\0\0\0\0\0\0\0\0\x02\x96\xEF\x12>\xA9m\xA5\xCFi_\"\xBF}\x94\xBE\x87\xD4\x9D\xB1\xADz\xC3q\xACC\xC4\xDAAa\xC8\xC2\x164\x9C[\xA1\x19(\x17\r8x\xA2dipfsX\"\x12 \xFD\xEF6P\xB6\x97\xA7\xD9\xFF\xBA\x1AV\xB5\x97\x15\xEF\x8F\xC9\xA3K\xAF\x19\xFB}\x99\xDEWQ\xFF\xCA\x90\x84dsolcC\0\x08\x1C\x003`\x80`@R4\x80\x15`\x0EW__\xFD[Pa\x0F\xDD\x80a\0\x1C_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0)W_5`\xE0\x1C\x80c-y\xDBF\x14a\0-W[__\xFD[a\0@a\0;6`\x04a\r(V[a\0RV[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[_a\0_\x85\x85\x85\x85a\0jV[\x90P[\x94\x93PPPPV[_a\0t\x83a\0\x8FV[\x90Pa\0\x80\x81\x83a\x01\x8AV[a\0b\x85\x85\x84`@\x01Qa\x03\xF4V[_a\0\x9D\x82` \x01Qa\x04\xDBV[a\0\xEEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FInvalid input vector provided\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[a\0\xFB\x82`@\x01Qa\x05uV[a\x01GW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FInvalid output vector provided\0\0`D\x82\x01R`d\x01a\0\xE5V[a\x01\x84\x82_\x01Q\x83` \x01Q\x84`@\x01Q\x85``\x01Q`@Q` \x01a\x01p\x94\x93\x92\x91\x90a\x0EXV[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra\x06\x02V[\x92\x91PPV[\x80Qa\x01\x95\x90a\x06$V[a\x01\xE1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01R\x7FBad merkle array proof\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\0\xE5V[`\x80\x81\x01QQ\x81QQ\x14a\x02]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FTx not on same level of merkle t`D\x82\x01R\x7Free as coinbase\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\0\xE5V[_a\x02k\x82`@\x01Qa\x06:V[\x82Q` \x84\x01Q\x91\x92Pa\x02\x82\x91\x85\x91\x84\x91a\x06FV[a\x02\xF4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`<`$\x82\x01R\x7FTx merkle proof is not valid for`D\x82\x01R\x7F provided header and tx hash\0\0\0\0`d\x82\x01R`\x84\x01a\0\xE5V[_`\x02\x83``\x01Q`@Q` \x01a\x03\x0E\x91\x81R` \x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x03(\x91a\x0E\xC7V[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\x03CW=__>=_\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03f\x91\x90a\x0E\xD2V[`\x80\x84\x01Q\x90\x91Pa\x03|\x90\x82\x90\x84\x90_a\x06FV[a\x03\xEEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`?`$\x82\x01R\x7FCoinbase merkle proof is not val`D\x82\x01R\x7Fid for provided header and hash\0`d\x82\x01R`\x84\x01a\0\xE5V[PPPPV[\x80Q`P\x14a\x04EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01R\x7FBad header block\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\0\xE5V[_a\x04O\x82a\x06xV[`@Q\x7F\xE4q\xE7,\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x82\x90R`\xFF\x85\x16`$\x82\x01R\x90\x91Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x90c\xE4q\xE7,\x90`D\x01_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x04\xBFW__\xFD[PZ\xFA\x15\x80\x15a\x04\xD1W=__>=_\xFD[PPPPPPPPV[___a\x04\xE7\x84a\x072V[\x90\x92P\x90P\x80\x15\x80a\x04\xF9WP_\x19\x82\x14[\x15a\x05\x07WP_\x93\x92PPPV[_a\x05\x13\x83`\x01a\x0F\x16V[\x90P_[\x82\x81\x10\x15a\x05hW\x85Q\x82\x10a\x052WP_\x95\x94PPPPPV[_a\x05=\x87\x84a\x07GV[\x90P_\x19\x81\x03a\x05SWP_\x96\x95PPPPPPV[a\x05]\x81\x84a\x0F\x16V[\x92PP`\x01\x01a\x05\x17V[P\x93Q\x90\x93\x14\x93\x92PPPV[___a\x05\x81\x84a\x072V[\x90\x92P\x90P\x80\x15\x80a\x05\x93WP_\x19\x82\x14[\x15a\x05\xA1WP_\x93\x92PPPV[_a\x05\xAD\x83`\x01a\x0F\x16V[\x90P_[\x82\x81\x10\x15a\x05hW\x85Q\x82\x10a\x05\xCCWP_\x95\x94PPPPPV[_a\x05\xD7\x87\x84a\x07\x96V[\x90P_\x19\x81\x03a\x05\xEDWP_\x96\x95PPPPPPV[a\x05\xF7\x81\x84a\x0F\x16V[\x92PP`\x01\x01a\x05\xB1V[_` _\x83Q` \x85\x01`\x02Z\xFAP` _` _`\x02Z\xFAPP_Q\x91\x90PV[_` \x82Qa\x063\x91\x90a\x0F)V[\x15\x92\x91PPV[`D\x81\x01Q_\x90a\x01\x84V[_\x83\x85\x14\x80\x15a\x06TWP\x81\x15[\x80\x15a\x06_WP\x82Q\x15[\x15a\x06lWP`\x01a\0bV[a\0_\x85\x84\x86\x85a\x07\xF6V[_`\x02\x80\x83`@Qa\x06\x8A\x91\x90a\x0E\xC7V[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\x06\xA5W=__>=_\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xC8\x91\x90a\x0E\xD2V[`@Q` \x01a\x06\xDA\x91\x81R` \x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x06\xF4\x91a\x0E\xC7V[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\x07\x0FW=__>=_\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\x84\x91\x90a\x0E\xD2V[__a\x07>\x83_a\x08\x9BV[\x91P\x91P\x91P\x91V[___a\x07T\x85\x85a\n8V[\x90\x92P\x90P`\x01\x82\x01a\x07lW_\x19\x92PPPa\x01\x84V[\x80a\x07x\x83`%a\x0F\x16V[a\x07\x82\x91\x90a\x0F\x16V[a\x07\x8D\x90`\x04a\x0F\x16V[\x95\x94PPPPPV[_a\x07\xA2\x82`\ta\x0F\x16V[\x83Q\x10\x15a\x07\xB2WP_\x19a\x01\x84V[_\x80a\x07\xC8\x85a\x07\xC3\x86`\x08a\x0F\x16V[a\x08\x9BV[\x90\x92P\x90P`\x01\x82\x01a\x07\xE0W_\x19\x92PPPa\x01\x84V[\x80a\x07\xEC\x83`\ta\x0F\x16V[a\x07\x8D\x91\x90a\x0F\x16V[_` \x84Qa\x08\x05\x91\x90a\x0F)V[\x15a\x08\x11WP_a\0bV[\x83Q_\x03a\x08 WP_a\0bV[\x81\x85_[\x86Q\x81\x10\x15a\x08\x8EWa\x088`\x02\x84a\x0F)V[`\x01\x03a\x08\\Wa\x08Ua\x08O\x88\x83\x01` \x01Q\x90V[\x83a\nvV[\x91Pa\x08uV[a\x08r\x82a\x08m\x89\x84\x01` \x01Q\x90V[a\nvV[\x91P[`\x01\x92\x90\x92\x1C\x91a\x08\x87` \x82a\x0F\x16V[\x90Pa\x08$V[P\x90\x93\x14\x95\x94PPPPPV[___a\x08\xA8\x85\x85a\n\x88V[\x90P\x80`\xFF\x16_\x03a\x08\xDBW_\x85\x85\x81Q\x81\x10a\x08\xC7Wa\x08\xC7a\x0FaV[\x01` \x01Q\x90\x93P`\xF8\x1C\x91Pa\n1\x90PV[\x83a\x08\xE7\x82`\x01a\x0F\x8EV[`\xFF\x16a\x08\xF4\x91\x90a\x0F\x16V[\x85Q\x10\x15a\t\tW_\x19_\x92P\x92PPa\n1V[_\x81`\xFF\x16`\x02\x03a\tLWa\tAa\t-a\t&\x87`\x01a\x0F\x16V[\x88\x90a\x0B\x0CV[b\xFF\xFF\0`\xE8\x82\x90\x1C\x16`\xF8\x91\x90\x91\x1C\x17\x90V[a\xFF\xFF\x16\x90Pa\n'V[\x81`\xFF\x16`\x04\x03a\t\x9BWa\t\x8Ea\tha\t&\x87`\x01a\x0F\x16V[`\xD8\x81\x90\x1Cc\xFF\0\xFF\0\x16b\xFF\0\xFF`\xE8\x92\x90\x92\x1C\x91\x90\x91\x16\x17`\x10\x81\x81\x1B\x91\x90\x1C\x17\x90V[c\xFF\xFF\xFF\xFF\x16\x90Pa\n'V[\x81`\xFF\x16`\x08\x03a\n'Wa\n\x1Aa\t\xB7a\t&\x87`\x01a\x0F\x16V[`\xC0\x1Cd\xFF\0\0\0\xFF`\x08\x82\x81\x1C\x91\x82\x16e\xFF\0\0\0\xFF\0\x93\x90\x91\x1B\x92\x83\x16\x17`\x10\x90\x81\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16f\xFF\0\xFF\0\xFF\0\xFF\x92\x90\x92\x16g\xFF\0\xFF\0\xFF\0\xFF\0\x90\x93\x16\x92\x90\x92\x17\x90\x91\x1Ce\xFF\xFF\0\0\xFF\xFF\x16\x17` \x81\x81\x1C\x91\x90\x1B\x17\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P[`\xFF\x90\x91\x16\x92P\x90P[\x92P\x92\x90PV[_\x80a\nE\x83`%a\x0F\x16V[\x84Q\x10\x15a\nXWP_\x19\x90P_a\n1V[_\x80a\ni\x86a\x07\xC3\x87`$a\x0F\x16V[\x90\x97\x90\x96P\x94PPPPPV[_a\n\x81\x83\x83a\x0B\x1AV[\x93\x92PPPV[_\x82\x82\x81Q\x81\x10a\n\x9BWa\n\x9Ba\x0FaV[\x01` \x01Q`\xF8\x1C`\xFF\x03a\n\xB2WP`\x08a\x01\x84V[\x82\x82\x81Q\x81\x10a\n\xC4Wa\n\xC4a\x0FaV[\x01` \x01Q`\xF8\x1C`\xFE\x03a\n\xDBWP`\x04a\x01\x84V[\x82\x82\x81Q\x81\x10a\n\xEDWa\n\xEDa\x0FaV[\x01` \x01Q`\xF8\x1C`\xFD\x03a\x0B\x04WP`\x02a\x01\x84V[P_\x92\x91PPV[_a\n\x81\x83\x83\x01` \x01Q\x90V[_\x82_R\x81` R` _`@_`\x02Z\xFAP` _` _`\x02Z\xFAPP_Q\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0B\x91Wa\x0B\x91a\x0BAV[`@R\x90V[`@Q`\x80\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0B\x91Wa\x0B\x91a\x0BAV[\x805\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x81\x14a\x0B\xE9W__\xFD[\x91\x90PV[_\x82`\x1F\x83\x01\x12a\x0B\xFDW__\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0C\x17Wa\x0C\x17a\x0BAV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0CFWa\x0CFa\x0BAV[`@R\x81\x81R\x83\x82\x01` \x01\x85\x10\x15a\x0C]W__\xFD[\x81` \x85\x01` \x83\x017_\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[_`\xA0\x82\x84\x03\x12\x15a\x0C\x89W__\xFD[a\x0C\x91a\x0BnV[\x90P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0C\xA9W__\xFD[a\x0C\xB5\x84\x82\x85\x01a\x0B\xEEV[\x82RP` \x82\x81\x015\x90\x82\x01R`@\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0C\xDBW__\xFD[a\x0C\xE7\x84\x82\x85\x01a\x0B\xEEV[`@\x83\x01RP``\x82\x81\x015\x90\x82\x01R`\x80\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\x10W__\xFD[a\r\x1C\x84\x82\x85\x01a\x0B\xEEV[`\x80\x83\x01RP\x92\x91PPV[____`\x80\x85\x87\x03\x12\x15a\r;W__\xFD[\x845s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\r^W__\xFD[\x93P` \x85\x015\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\x80W__\xFD[\x85\x01`\x80\x81\x88\x03\x12\x15a\r\x91W__\xFD[a\r\x99a\x0B\x97V[a\r\xA2\x82a\x0B\xBAV[\x81R` \x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\xBDW__\xFD[a\r\xC9\x89\x82\x85\x01a\x0B\xEEV[` \x83\x01RP`@\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\xE8W__\xFD[a\r\xF4\x89\x82\x85\x01a\x0B\xEEV[`@\x83\x01RPa\x0E\x06``\x83\x01a\x0B\xBAV[``\x82\x01R\x80\x93PPP``\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0E)W__\xFD[a\x0E5\x87\x82\x88\x01a\x0CyV[\x91PP\x92\x95\x91\x94P\x92PV[_\x81Q\x80` \x84\x01\x85^_\x93\x01\x92\x83RP\x90\x91\x90PV[\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85\x16\x81R_a\x0E\x94a\x0E\x8E`\x04\x84\x01\x87a\x0EAV[\x85a\x0EAV[\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x93\x90\x93\x16\x83RPP`\x04\x01\x93\x92PPPV[_a\n\x81\x82\x84a\x0EAV[_` \x82\x84\x03\x12\x15a\x0E\xE2W__\xFD[PQ\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\x01\x84Wa\x01\x84a\x0E\xE9V[_\x82a\x0F\\W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[P\x06\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[`\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x01\x84Wa\x01\x84a\x0E\xE9V\xFE\xA2dipfsX\"\x12 \x014G\x13\xD0r\xB6\x95\x9D}<-\xC4\xEA\xFCA(\0\x9F\x0BJy\x16)P\xB4\x1Dv\xED\xE7\xB7ddsolcC\0\x08\x1C\x003`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`@Qa);8\x03\x80a);\x839\x81\x01`@\x81\x90Ra\0.\x91a\x03+V[\x82\x82\x82\x82\x82\x82a\0?\x83Q`P\x14\x90V[a\0\x84W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01RpBad genesis block`x\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[_a\0\x8E\x84a\x01fV[\x90Pb\xFF\xFF\xFF\x82\x16\x15a\x01\tW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FPeriod start hash does not have `D\x82\x01R\x7Fwork. Hint: wrong byte order?\0\0\0`d\x82\x01R`\x84\x01a\0{V[_\x81\x81U`\x01\x82\x90U`\x02\x82\x90U\x81\x81R`\x04` R`@\x90 \x83\x90Ua\x012a\x07\xE0\x84a\x03\xFEV[a\x01<\x90\x84a\x04%V[_\x83\x81R`\x04` R`@\x90 Ua\x01S\x84a\x02&V[`\x05UPa\x05\xBD\x98PPPPPPPPPV[_`\x02\x80\x83`@Qa\x01x\x91\x90a\x048V[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\x01\x93W=__>=_\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xB6\x91\x90a\x04NV[`@Q` \x01a\x01\xC8\x91\x81R` \x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x01\xE2\x91a\x048V[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\x01\xFDW=__>=_\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02 \x91\x90a\x04NV[\x92\x91PPV[_a\x02 a\x023\x83a\x028V[a\x02CV[_a\x02 \x82\x82a\x02SV[_a\x02 a\xFF\xFF`\xD0\x1B\x83a\x02\xF7V[_\x80a\x02ja\x02c\x84`Ha\x04eV[\x85\x90a\x03\tV[`\xE8\x1C\x90P_\x84a\x02|\x85`Ka\x04eV[\x81Q\x81\x10a\x02\x8CWa\x02\x8Ca\x04xV[\x01` \x01Q`\xF8\x1C\x90P_a\x02\xBE\x83_`\x10\x82b\xFF\xFF\xFF\x16\x90\x1C\x82a\xFF\0\x16`\x10\x84b\xFF\xFF\xFF\x16\x90\x1B\x17\x17\x90P\x91\x90PV[b\xFF\xFF\xFF\x16\x90P_a\x02\xD1`\x03\x84a\x04\x8CV[`\xFF\x16\x90Pa\x02\xE2\x81a\x01\0a\x05\x88V[a\x02\xEC\x90\x83a\x05\x93V[\x97\x96PPPPPPPV[_a\x03\x02\x82\x84a\x05\xAAV[\x93\x92PPPV[_a\x03\x02\x83\x83\x01` \x01Q\x90V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[___``\x84\x86\x03\x12\x15a\x03=W__\xFD[\x83Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x03RW__\xFD[\x84\x01`\x1F\x81\x01\x86\x13a\x03bW__\xFD[\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x03{Wa\x03{a\x03\x17V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x03\xA9Wa\x03\xA9a\x03\x17V[`@R\x81\x81R\x82\x82\x01` \x01\x88\x10\x15a\x03\xC0W__\xFD[\x81` \x84\x01` \x83\x01^_` \x92\x82\x01\x83\x01R\x90\x86\x01Q`@\x90\x96\x01Q\x90\x97\x95\x96P\x94\x93PPPPV[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[_\x82a\x04\x0CWa\x04\x0Ca\x03\xEAV[P\x06\x90V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x02 Wa\x02 a\x04\x11V[_\x82Q\x80` \x85\x01\x84^_\x92\x01\x91\x82RP\x91\x90PV[_` \x82\x84\x03\x12\x15a\x04^W__\xFD[PQ\x91\x90PV[\x80\x82\x01\x80\x82\x11\x15a\x02 Wa\x02 a\x04\x11V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[`\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x02 Wa\x02 a\x04\x11V[`\x01\x81[`\x01\x84\x11\x15a\x04\xE0W\x80\x85\x04\x81\x11\x15a\x04\xC4Wa\x04\xC4a\x04\x11V[`\x01\x84\x16\x15a\x04\xD2W\x90\x81\x02\x90[`\x01\x93\x90\x93\x1C\x92\x80\x02a\x04\xA9V[\x93P\x93\x91PPV[_\x82a\x04\xF6WP`\x01a\x02 V[\x81a\x05\x02WP_a\x02 V[\x81`\x01\x81\x14a\x05\x18W`\x02\x81\x14a\x05\"Wa\x05>V[`\x01\x91PPa\x02 V[`\xFF\x84\x11\x15a\x053Wa\x053a\x04\x11V[PP`\x01\x82\x1Ba\x02 V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a\x05aWP\x81\x81\na\x02 V[a\x05m_\x19\x84\x84a\x04\xA5V[\x80_\x19\x04\x82\x11\x15a\x05\x80Wa\x05\x80a\x04\x11V[\x02\x93\x92PPPV[_a\x03\x02\x83\x83a\x04\xE8V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x02 Wa\x02 a\x04\x11V[_\x82a\x05\xB8Wa\x05\xB8a\x03\xEAV[P\x04\x90V[a#q\x80a\x05\xCA_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x01\x15W_5`\xE0\x1C\x80cp\xD5<\x18\x11a\0\xADW\x80c\xB9\x85b\x1A\x11a\0}W\x80c\xE3\xD8\xD8\xD8\x11a\0cW\x80c\xE3\xD8\xD8\xD8\x14a\x02\"W\x80c\xE4q\xE7,\x14a\x02)W\x80c\xF5\x8D\xB0o\x14a\x02<W__\xFD[\x80c\xB9\x85b\x1A\x14a\x02\x07W\x80c\xC5\x82B\xCD\x14a\x02\x1AW__\xFD[\x80cp\xD5<\x18\x14a\x01\xB1W\x80ct\xC3\xA3\xA9\x14a\x01\xCEW\x80c\x7F\xA67\xFC\x14a\x01\xE1W\x80c\xB2[\x9B\0\x14a\x01\xF4W__\xFD[\x80c.O\x16\x1A\x11a\0\xE8W\x80c.O\x16\x1A\x14a\x01UW\x80c0\x01{;\x14a\x01xW\x80c`\xB5\xC3\x90\x14a\x01\x8BW\x80ce\xDAA\xB9\x14a\x01\x9EW__\xFD[\x80c\x05\xD0\x9Ap\x14a\x01\x19W\x80c\x117d\xBE\x14a\x01.W\x80c\x19\x10\xD9s\x14a\x01EW\x80c+\x97\xBE$\x14a\x01MW[__\xFD[a\x01,a\x01'6`\x04a\x1D{V[a\x02\xA8V[\0[`\x05T[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[`\x01Ta\x012V[`\x06Ta\x012V[a\x01ha\x01c6`\x04a\x1E\x0CV[a\x04\xE1V[`@Q\x90\x15\x15\x81R` \x01a\x01<V[a\x012a\x01\x866`\x04a\x1E;V[a\x04\xF9V[a\x012a\x01\x996`\x04a\x1E[V[a\x05\rV[a\x01ha\x01\xAC6`\x04a\x1ErV[a\x05\x17V[a\x01\xB9`\x04\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01<V[a\x01ha\x01\xDC6`\x04a\x1E\xDEV[a\x06\xC3V[a\x01ha\x01\xEF6`\x04a\x1F_V[a\x088V[a\x012a\x02\x026`\x04a\x1F\xFEV[a\n\x17V[a\x01ha\x02\x156`\x04a wV[a\n\x94V[`\x02Ta\x012V[_Ta\x012V[a\x01,a\x0276`\x04a \xA0V[a\n\xAAV[a\x01,a\x02J6`\x04a \xD9V[`\x07\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\x16\x92\x15\x15\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\x16\x92\x90\x92\x17a\x01\0\x91\x15\x15\x91\x90\x91\x02\x17\x90UV[a\x02\xE6\x87\x87\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPa\x0Bh\x92PPPV[a\x037W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01R\x7FBad header block\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[a\x03u\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPa\x0Bo\x92PPPV[a\x03\xC1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01R\x7FBad merkle array proof\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03.V[a\x04@\x83a\x04\x03\x89\x89\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPa\x0B\x85\x92PPPV[\x87\x87\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RP\x88\x92Pa\x0B\x91\x91PPV[a\x04\x8CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01R\x7FBad inclusion proof\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03.V[_a\x04\xCB\x88\x88\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPa\x0B\xC3\x92PPPV[\x90Pa\x04\xD7\x81\x83a\n\xAAV[PPPPPPPPV[_a\x04\xEE\x85\x85\x85\x85a\x0C\x9BV[\x90P[\x94\x93PPPPV[_a\x05\x04\x83\x83a\r5V[\x90P[\x92\x91PPV[_a\x05\x07\x82a\r\xA7V[_a\x05V\x83\x83\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPa\x0EU\x92PPPV[a\x05\xC8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FHeader array length must be divi`D\x82\x01R\x7Fsible by 80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x03.V[a\x06\x06\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPa\x0Bh\x92PPPV[a\x06RW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FAnchor must be 80 bytes\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03.V[a\x04\xEE\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPP`@\x80Q` `\x1F\x89\x01\x81\x90\x04\x81\x02\x82\x01\x81\x01\x90\x92R\x87\x81R\x92P\x87\x91P\x86\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847_\x92\x01\x82\x90RP\x92Pa\x0Ed\x91PPV[_a\x07\x02\x84\x84\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPa\x0Bh\x92PPPV[\x80\x15a\x07GWPa\x07G\x86\x86\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPa\x0Bh\x92PPPV[a\x07\xB9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FBad args. Check header and array`D\x82\x01R\x7F byte lengths.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x03.V[a\x08-\x87\x87\x87\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPP`@\x80Q` `\x1F\x8B\x01\x81\x90\x04\x81\x02\x82\x01\x81\x01\x90\x92R\x89\x81R\x92P\x89\x91P\x88\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847_\x92\x01\x91\x90\x91RP\x88\x92Pa\x12Q\x91PPV[\x97\x96PPPPPPPV[_a\x08w\x87\x87\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPa\x0Bh\x92PPPV[\x80\x15a\x08\xBCWPa\x08\xBC\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPa\x0Bh\x92PPPV[\x80\x15a\t\x01WPa\t\x01\x83\x83\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPa\x0EU\x92PPPV[a\tsW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FBad args. Check header and array`D\x82\x01R\x7F byte lengths.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x03.V[a\x08-\x87\x87\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPP`@\x80Q` `\x1F\x8B\x01\x81\x90\x04\x81\x02\x82\x01\x81\x01\x90\x92R\x89\x81R\x92P\x89\x91P\x88\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847_\x92\x01\x91\x90\x91RPP`@\x80Q` `\x1F\x8A\x01\x81\x90\x04\x81\x02\x82\x01\x81\x01\x90\x92R\x88\x81R\x92P\x88\x91P\x87\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847_\x92\x01\x91\x90\x91RPa\x14\xEE\x92PPPV[_a\n\x8A\x86\x86\x86\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPP`@\x80Q` `\x1F\x8A\x01\x81\x90\x04\x81\x02\x82\x01\x81\x01\x90\x92R\x88\x81R\x92P\x88\x91P\x87\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847_\x92\x01\x91\x90\x91RPa\x17\x80\x92PPPV[\x96\x95PPPPPPV[_a\n\xA0\x84\x84\x84a\x19\x11V[\x90P[\x93\x92PPPV[_a\n\xB4`\x02T\x90V[\x90Pa\n\xC3\x83\x82a\x08\0a\x19\x11V[a\x0B\x0FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FGCD does not confirm header\0\0\0\0\0`D\x82\x01R`d\x01a\x03.V[`\xFF\x82\x16`\x08\x10\x15a\x0BcW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FInsufficient confirmations\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03.V[PPPV[Q`P\x14\x90V[_` \x82Qa\x0B~\x91\x90a!.V[\x15\x92\x91PPV[`D\x81\x01Q_\x90a\x05\x07V[_\x83\x85\x14\x80\x15a\x0B\x9FWP\x81\x15[\x80\x15a\x0B\xAAWP\x82Q\x15[\x15a\x0B\xB7WP`\x01a\x04\xF1V[a\x04\xEE\x85\x84\x86\x85a\x19AV[_`\x02\x80\x83`@Qa\x0B\xD5\x91\x90a!AV[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\x0B\xF0W=__>=_\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\x13\x91\x90a!WV[`@Q` \x01a\x0C%\x91\x81R` \x01\x90V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x0C]\x91a!AV[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\x0CxW=__>=_\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x07\x91\x90a!WV[_\x83\x85\x14\x80\x15a\x0C\xAAWP\x82\x85\x14[\x15a\x0C\xB7WP`\x01a\x04\xF1V[\x83\x83\x81\x81_[\x86\x81\x10\x15a\x0C\xFFW\x89\x83\x14a\x0C\xDEW_\x83\x81R`\x03` R`@\x90 T\x92\x94P[\x89\x82\x14a\x0C\xF7W_\x82\x81R`\x03` R`@\x90 T\x91\x93P[`\x01\x01a\x0C\xBDV[P\x82\x84\x03a\r\x13W_\x94PPPPPa\x04\xF1V[\x80\x82\x14a\r&W_\x94PPPPPa\x04\xF1V[P`\x01\x98\x97PPPPPPPPV[_\x82\x81[\x83\x81\x10\x15a\rYW_\x91\x82R`\x03` R`@\x90\x91 T\x90`\x01\x01a\r9V[P\x80a\x05\x04W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01R\x7FUnknown ancestor\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03.V[_\x80\x82\x81[a\r\xB8`\x04`\x01a!\x9BV[c\xFF\xFF\xFF\xFF\x16\x81\x10\x15a\x0E\x0CW_\x82\x81R`\x04` R`@\x81 T\x93P\x83\x90\x03a\r\xF1W_\x91\x82R`\x03` R`@\x90\x91 T\x90a\x0E\x04V[a\r\xFB\x81\x84a!\xB7V[\x95\x94PPPPPV[`\x01\x01a\r\xACV[P`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01R\x7FUnknown block\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03.V[_`P\x82Qa\x0B~\x91\x90a!.V[__a\x0Eo\x85a\x0B\xC3V[\x90P_a\x0E{\x82a\r\xA7V[\x90P_a\x0E\x87\x86a\x19\xE6V[\x90P\x84\x80a\x0E\x9CWP\x80a\x0E\x9A\x88a\x19\xE6V[\x14[a\x0F\rW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FUnexpected retarget on external `D\x82\x01R\x7Fcall\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x03.V[\x85Q_\x90\x81\x90\x81[\x81\x81\x10\x15a\x12\x0EWa\x0F(`P\x82a!\xCAV[a\x0F3\x90`\x01a!\xB7V[a\x0F=\x90\x87a!\xB7V[\x93Pa\x0FK\x8A\x82`Pa\x19\xF1V[_\x81\x81R`\x03` R`@\x90 T\x90\x93Pa\x11!W\x84a\x10\xA1\x84_\x81\x90P`\x08\x81~\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\x16\x90\x1B`\x08\x82\x90\x1C~\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\x16\x17\x90P`\x10\x81}\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\x16\x90\x1B`\x10\x82\x90\x1C}\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\x16\x17\x90P` \x81{\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\x16\x90\x1B` \x82\x90\x1C{\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\x16\x17\x90P`@\x81w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x1B`@\x82\x90\x1Cw\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x17\x90P`\x80\x81\x90\x1B`\x80\x82\x90\x1C\x17\x90P\x91\x90PV[\x11\x15a\x10\xEFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FHeader work is insufficient\0\0\0\0\0`D\x82\x01R`d\x01a\x03.V[_\x83\x81R`\x03` R`@\x90 \x87\x90Ua\x11\n`\x04\x85a!.V[_\x03a\x11!W_\x83\x81R`\x04` R`@\x90 \x84\x90U[\x84a\x11,\x8B\x83a\x1A\x16V[\x14a\x11yW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FTarget changed unexpectedly\0\0\0\0\0`D\x82\x01R`d\x01a\x03.V[\x86a\x11\x84\x8B\x83a\x1A\xAFV[\x14a\x11\xF7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FHeaders do not form a consistent`D\x82\x01R\x7F chain\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x03.V[\x82\x96P`P\x81a\x12\x07\x91\x90a!\xB7V[\x90Pa\x0F\x15V[P\x81a\x12\x19\x8Ba\x0B\xC3V[`@Q\x7F\xF9\x0EO\x1D\x9C\xD0\xDDU\xE39A\x1C\xBC\x9B\x15$\x820|:#\xEDdq^J(X\xF6A\xA3\xF5\x90_\x90\xA3P`\x01\x99\x98PPPPPPPPPV[_a\x07\xE0\x82\x11\x15a\x12\xCAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`3`$\x82\x01R\x7FRequested limit is greater than `D\x82\x01R\x7F1 difficulty period\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x03.V[_a\x12\xD4\x84a\x0B\xC3V[\x90P_a\x12\xE0\x86a\x0B\xC3V[\x90P`\x01T\x81\x14a\x133W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FPassed in best is not best known`D\x82\x01R`d\x01a\x03.V[_\x82\x81R`\x03` R`@\x90 Ta\x13\x8DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01R\x7FNew best is unknown\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03.V[a\x13\x9B\x87`\x01T\x84\x87a\x0C\x9BV[a\x14\rW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`)`$\x82\x01R\x7FAncestor must be heaviest common`D\x82\x01R\x7F ancestor\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x03.V[\x81a\x14\x19\x88\x88\x88a\x17\x80V[\x14a\x14\x8CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`3`$\x82\x01R\x7FNew best hash does not have more`D\x82\x01R\x7F work than previous\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x03.V[`\x01\x82\x90U`\x02\x87\x90U_a\x14\xA0\x86a\x1A\xC7V[\x90P`\x05T\x81\x14a\x14\xB1W`\x05\x81\x90U[\x87\x83\x83\x7F<\xC1=\xE6M\xF0\xF0#\x96&#\\Q\xA2\xDA%\x1B\xBC\x8C\x85fN\xCC\xE3\x92c\xDA>\xE0?`l`@Q`@Q\x80\x91\x03\x90\xA4P`\x01\x97\x96PPPPPPPV[__a\x15\x01a\x14\xFC\x86a\x0B\xC3V[a\r\xA7V[\x90P_a\x15\x10a\x14\xFC\x86a\x0B\xC3V[\x90Pa\x15\x1Ea\x07\xE0\x82a!.V[a\x07\xDF\x14a\x15\x94W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FMust provide the last header of `D\x82\x01R\x7Fthe closing difficulty period\0\0\0`d\x82\x01R`\x84\x01a\x03.V[a\x15\xA0\x82a\x07\xDFa!\xB7V[\x81\x14a\x16\x14W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`(`$\x82\x01R\x7FMust provide exactly 1 difficult`D\x82\x01R\x7Fy period\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x03.V[a\x16\x1D\x85a\x1A\xC7V[a\x16&\x87a\x1A\xC7V[\x14a\x16\x99W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FPeriod header difficulties do no`D\x82\x01R\x7Ft match\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x03.V[_a\x16\xA3\x85a\x19\xE6V[\x90P_a\x16\xD5a\x16\xB2\x89a\x19\xE6V[a\x16\xBB\x8Aa\x1A\xD9V[c\xFF\xFF\xFF\xFF\x16a\x16\xCA\x8Aa\x1A\xD9V[c\xFF\xFF\xFF\xFF\x16a\x1B\x0CV[\x90P\x81\x81\x83\x16\x14a\x17(W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FInvalid retarget provided\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03.V[_a\x172\x89a\x1A\xC7V[\x90P\x80`\x06T\x14\x15\x80\x15a\x17\\WPa\x07\xE0a\x17O`\x01Ta\r\xA7V[a\x17Y\x91\x90a!\xDDV[\x84\x11[\x15a\x17gW`\x06\x81\x90U[a\x17s\x88\x88`\x01a\x0EdV[\x99\x98PPPPPPPPPV[__a\x17\x8B\x85a\r\xA7V[\x90P_a\x17\x9Aa\x14\xFC\x86a\x0B\xC3V[\x90P_a\x17\xA9a\x14\xFC\x86a\x0B\xC3V[\x90P\x82\x82\x10\x15\x80\x15a\x17\xBBWP\x82\x81\x10\x15[a\x18-W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`0`$\x82\x01R\x7FA descendant height is below the`D\x82\x01R\x7F ancestor height\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x03.V[_a\x18:a\x07\xE0\x85a!.V[a\x18F\x85a\x07\xE0a!\xB7V[a\x18P\x91\x90a!\xDDV[\x90P\x80\x83\x10\x81\x83\x10\x81\x15\x82a\x18bWP\x80[\x15a\x18}Wa\x18p\x89a\x0B\xC3V[\x96PPPPPPPa\n\xA3V[\x81\x80\x15a\x18\x88WP\x80\x15[\x15a\x18\x96Wa\x18p\x88a\x0B\xC3V[\x81\x80\x15a\x18\xA0WP\x80[\x15a\x18\xC4W\x83\x85\x10\x15a\x18\xBBWa\x18\xB6\x88a\x0B\xC3V[a\x18pV[a\x18p\x89a\x0B\xC3V[a\x18\xCD\x88a\x1A\xC7V[a\x18\xD9a\x07\xE0\x86a!.V[a\x18\xE3\x91\x90a!\xF0V[a\x18\xEC\x8Aa\x1A\xC7V[a\x18\xF8a\x07\xE0\x88a!.V[a\x19\x02\x91\x90a!\xF0V[\x10\x15a\x18\xBBWa\x18p\x88a\x0B\xC3V[`\x07T_\x90`\xFF\x16\x15a\x19/WP`\x07Ta\x01\0\x90\x04`\xFF\x16a\n\xA3V[a\x19:\x84\x84\x84a\x1B\x94V[\x90Pa\n\xA3V[_` \x84Qa\x19P\x91\x90a!.V[\x15a\x19\\WP_a\x04\xF1V[\x83Q_\x03a\x19kWP_a\x04\xF1V[\x81\x85_[\x86Q\x81\x10\x15a\x19\xD9Wa\x19\x83`\x02\x84a!.V[`\x01\x03a\x19\xA7Wa\x19\xA0a\x19\x9A\x88\x83\x01` \x01Q\x90V[\x83a\x1B\xD5V[\x91Pa\x19\xC0V[a\x19\xBD\x82a\x19\xB8\x89\x84\x01` \x01Q\x90V[a\x1B\xD5V[\x91P[`\x01\x92\x90\x92\x1C\x91a\x19\xD2` \x82a!\xB7V[\x90Pa\x19oV[P\x90\x93\x14\x95\x94PPPPPV[_a\x05\x07\x82_a\x1A\x16V[_` _\x83\x85` \x01\x87\x01`\x02Z\xFAP` _` _`\x02Z\xFAPP_Q\x93\x92PPPV[_\x80a\x1A-a\x1A&\x84`Ha!\xB7V[\x85\x90a\x1B\xE0V[`\xE8\x1C\x90P_\x84a\x1A?\x85`Ka!\xB7V[\x81Q\x81\x10a\x1AOWa\x1AOa\"\x07V[\x01` \x01Q`\xF8\x1C\x90P_a\x1A\x81\x83_`\x10\x82b\xFF\xFF\xFF\x16\x90\x1C\x82a\xFF\0\x16`\x10\x84b\xFF\xFF\xFF\x16\x90\x1B\x17\x17\x90P\x91\x90PV[b\xFF\xFF\xFF\x16\x90P_a\x1A\x94`\x03\x84a\"4V[`\xFF\x16\x90Pa\x1A\xA5\x81a\x01\0a#0V[a\x08-\x90\x83a!\xF0V[_a\x05\x04a\x1A\xBE\x83`\x04a!\xB7V[\x84\x01` \x01Q\x90V[_a\x05\x07a\x1A\xD4\x83a\x19\xE6V[a\x1B\xEEV[_a\x05\x07a\x1A\xE6\x83a\x1C\x15V[`\xD8\x81\x90\x1Cc\xFF\0\xFF\0\x16b\xFF\0\xFF`\xE8\x92\x90\x92\x1C\x91\x90\x91\x16\x17`\x10\x81\x81\x1B\x91\x90\x1C\x17\x90V[_\x80a\x1B\x18\x83\x85a\x1C!V[\x90Pa\x1B(b\x12u\0`\x04a\x1C|V[\x81\x10\x15a\x1B@Wa\x1B=b\x12u\0`\x04a\x1C|V[\x90P[a\x1BNb\x12u\0`\x04a\x1C\x87V[\x81\x11\x15a\x1BfWa\x1Bcb\x12u\0`\x04a\x1C\x87V[\x90P[_a\x1B~\x82a\x1Bx\x88b\x01\0\0a\x1C|V[\x90a\x1C\x87V[\x90Pa\n\x8Ab\x01\0\0a\x1Bx\x83b\x12u\0a\x1C|V[_\x82\x81[\x83\x81\x10\x15a\x1B\xCAW\x85\x82\x03a\x1B\xB2W`\x01\x92PPPa\n\xA3V[_\x91\x82R`\x03` R`@\x90\x91 T\x90`\x01\x01a\x1B\x98V[P_\x95\x94PPPPPV[_a\x05\x04\x83\x83a\x1C\xFAV[_a\x05\x04\x83\x83\x01` \x01Q\x90V[_a\x05\x07{\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83a\x1C|V[_a\x05\x07\x82`Da\x1B\xE0V[_\x82\x82\x11\x15a\x1CrW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FUnderflow during subtraction.\0\0\0`D\x82\x01R`d\x01a\x03.V[a\x05\x04\x82\x84a!\xDDV[_a\x05\x04\x82\x84a!\xCAV[_\x82_\x03a\x1C\x96WP_a\x05\x07V[a\x1C\xA0\x82\x84a!\xF0V[\x90P\x81a\x1C\xAD\x84\x83a!\xCAV[\x14a\x05\x07W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FOverflow during multiplication.\0`D\x82\x01R`d\x01a\x03.V[_\x82_R\x81` R` _`@_`\x02Z\xFAP` _` _`\x02Z\xFAPP_Q\x92\x91PPV[__\x83`\x1F\x84\x01\x12a\x1D1W__\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1DHW__\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x1D_W__\xFD[\x92P\x92\x90PV[\x805`\xFF\x81\x16\x81\x14a\x1DvW__\xFD[\x91\x90PV[_______`\xA0\x88\x8A\x03\x12\x15a\x1D\x91W__\xFD[\x875g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1D\xA7W__\xFD[a\x1D\xB3\x8A\x82\x8B\x01a\x1D!V[\x90\x98P\x96PP` \x88\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1D\xD2W__\xFD[a\x1D\xDE\x8A\x82\x8B\x01a\x1D!V[\x90\x96P\x94PP`@\x88\x015\x92P``\x88\x015\x91Pa\x1D\xFE`\x80\x89\x01a\x1DfV[\x90P\x92\x95\x98\x91\x94\x97P\x92\x95PV[____`\x80\x85\x87\x03\x12\x15a\x1E\x1FW__\xFD[PP\x825\x94` \x84\x015\x94P`@\x84\x015\x93``\x015\x92P\x90PV[__`@\x83\x85\x03\x12\x15a\x1ELW__\xFD[PP\x805\x92` \x90\x91\x015\x91PV[_` \x82\x84\x03\x12\x15a\x1EkW__\xFD[P5\x91\x90PV[____`@\x85\x87\x03\x12\x15a\x1E\x85W__\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1E\x9BW__\xFD[a\x1E\xA7\x87\x82\x88\x01a\x1D!V[\x90\x95P\x93PP` \x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1E\xC6W__\xFD[a\x1E\xD2\x87\x82\x88\x01a\x1D!V[\x95\x98\x94\x97P\x95PPPPV[______`\x80\x87\x89\x03\x12\x15a\x1E\xF3W__\xFD[\x865\x95P` \x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1F\x10W__\xFD[a\x1F\x1C\x89\x82\x8A\x01a\x1D!V[\x90\x96P\x94PP`@\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1F;W__\xFD[a\x1FG\x89\x82\x8A\x01a\x1D!V[\x97\x9A\x96\x99P\x94\x97\x94\x96\x95``\x90\x95\x015\x94\x93PPPPV[______``\x87\x89\x03\x12\x15a\x1FtW__\xFD[\x865g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1F\x8AW__\xFD[a\x1F\x96\x89\x82\x8A\x01a\x1D!V[\x90\x97P\x95PP` \x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1F\xB5W__\xFD[a\x1F\xC1\x89\x82\x8A\x01a\x1D!V[\x90\x95P\x93PP`@\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1F\xE0W__\xFD[a\x1F\xEC\x89\x82\x8A\x01a\x1D!V[\x97\x9A\x96\x99P\x94\x97P\x92\x95\x93\x94\x92PPPV[_____``\x86\x88\x03\x12\x15a \x12W__\xFD[\x855\x94P` \x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a /W__\xFD[a ;\x88\x82\x89\x01a\x1D!V[\x90\x95P\x93PP`@\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a ZW__\xFD[a f\x88\x82\x89\x01a\x1D!V[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[___``\x84\x86\x03\x12\x15a \x89W__\xFD[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[__`@\x83\x85\x03\x12\x15a \xB1W__\xFD[\x825\x91Pa \xC1` \x84\x01a\x1DfV[\x90P\x92P\x92\x90PV[\x805\x80\x15\x15\x81\x14a\x1DvW__\xFD[__`@\x83\x85\x03\x12\x15a \xEAW__\xFD[a \xF3\x83a \xCAV[\x91Pa \xC1` \x84\x01a \xCAV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_\x82a!<Wa!<a!\x01V[P\x06\x90V[_\x82Q\x80` \x85\x01\x84^_\x92\x01\x91\x82RP\x91\x90PV[_` \x82\x84\x03\x12\x15a!gW__\xFD[PQ\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[c\xFF\xFF\xFF\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x05\x07Wa\x05\x07a!nV[\x80\x82\x01\x80\x82\x11\x15a\x05\x07Wa\x05\x07a!nV[_\x82a!\xD8Wa!\xD8a!\x01V[P\x04\x90V[\x81\x81\x03\x81\x81\x11\x15a\x05\x07Wa\x05\x07a!nV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x05\x07Wa\x05\x07a!nV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[`\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x05\x07Wa\x05\x07a!nV[`\x01\x81[`\x01\x84\x11\x15a\"\x88W\x80\x85\x04\x81\x11\x15a\"lWa\"la!nV[`\x01\x84\x16\x15a\"zW\x90\x81\x02\x90[`\x01\x93\x90\x93\x1C\x92\x80\x02a\"QV[\x93P\x93\x91PPV[_\x82a\"\x9EWP`\x01a\x05\x07V[\x81a\"\xAAWP_a\x05\x07V[\x81`\x01\x81\x14a\"\xC0W`\x02\x81\x14a\"\xCAWa\"\xE6V[`\x01\x91PPa\x05\x07V[`\xFF\x84\x11\x15a\"\xDBWa\"\xDBa!nV[PP`\x01\x82\x1Ba\x05\x07V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a#\tWP\x81\x81\na\x05\x07V[a#\x15_\x19\x84\x84a\"MV[\x80_\x19\x04\x82\x11\x15a#(Wa#(a!nV[\x02\x93\x92PPPV[_a\x05\x04\x83\x83a\"\x90V\xFE\xA2dipfsX\"\x12 \x11B\xAF~\x12\x17;z\x99\xDDE=\xFC\x89.\x01\xC9\xC1\xE5\xB66Y\xB6\x0Ca\xD3\xE9\xD8\x01\"\xF9\xEBdsolcC\0\x08\x1C\x003\0\0\0 s\xBD!\x84\xED\xD9\xC4\xFCvd.\xA6uN\xE4\x016\x97\x0E\xFC\x10\xC4\x19\0\0\0\0\0\0\0\0\0\x02\x96\xEF\x12>\xA9m\xA5\xCFi_\"\xBF}\x94\xBE\x87\xD4\x9D\xB1\xADz\xC3q\xACC\xC4\xDAAa\xC8\xC2\x164\x9C[\xA1\x19(\x17\r8x+\0\0\0\0\0\0\0\0\0\0\0\0q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x02H\x97\x07\0\0\0\0\0\"\0 \xA43>V\x12\xAB\x1A\x10C\xB2WU\xC8\x9B\x16\xD5Q\x84\xA4/\x81y\x9Eb>k\xC3\x9D\xB8S\x9C\x18\0\0\0\0\0\0\0\0\x16j\x14\xED\xB1\xB5\xC2\xF3\x9A\xF0\xFE\xC1Qs%\x85\xB1\x04\x9B\x07\x89R\x11H\xE5\xA1\xA0\xE6\x16\xD8\xFD\x92\xB4\xEF\"\x8CBN\x0C\x81g\x99\xA2V\xC6\xA9\x08\x92\x19\\\xCF\xC53\0\xD6\xE3Z\rm\xE9Kef\x94X\x99d\xA2R\x95~Fs\xA9\xFB\x1D/\x8BJ\x92\xE3\xF0\xA7\xBBeO\xDD\xB9NZ\x1Em\x7F\x7FI\x9F\xD1\xBE]\xD3\ns\xBFU\x84\xBF\x13}\xA5\xFD\xD7|\xC2\x1A\xEB\x95\xB9\xE3W\x88\x89K\xE0\x19(K\xD4\xFB\xEDm\xD6\x11\x8A\xC2\xCBm&\xBCK\xE4\xE4#\xF5Z:H\xF2\x87M\x8D\x02\xA6]\x9C\x87\xD0}\xE2\x1DM\xFE{\n\x9FJ#\xCC\x9AX7>\x9Ei1\xFE\xFD\xB5\xAF\xAD\xE5\xDFT\xC9\x11\x04\x04\x8D\xF1\xEE\x99\x92@ay\x84\xE1\x8Bo\x93\x1E#sg=\x01\x95\xB8\xC6\x98}\x7F\xF7e\r\\\xE5;\xCE\xC4n\x13\xABO-\xA1\x14j\x7F\xC6!\xEEg/b\xBC\"t$\x869-u\xE5^g\xB0\x99`\xC38j\x0BI\xE7_\x17#\xD6\xAB(\xAC\x9A (\xA0\xC7(f\xE2\x11\x1Dy\xD4\x81{\x88\xE1|\x82\x197\x84wh\xD9(7\xBA\xE3\x83+\xB8\xE5\xA4\xABD4\xB9~\0\xA6\xC1\x01\x82\xF2\x11\xF5\x92@\x90h\xD6\xF5e$\0\xD9\xA3\xD1\xCC\x15\n\x7F\xB6\x92\xE8t\xCCB\xD7k\xDA\xFC\x84//\xE0\xF85\xA7\xC2M-`\xC1\t\xB1\x87\xD6Eq\xEF\xBA\xA8\x04{\xE8X!\xF8\xE6~\x0E\x85\xF2\xF5\x89K\xC6=\0\xC2\xED\x9Dd\x01\x17F\xBD\x86t\0\xF3IK\x8FD\xC2K\x83\xE1\xAAX\xC4\xF0\xFF%\xB4\xA6\x1C\xFF\xEF\xFDK\xC0\xF9\xBA0\0\0\0\0\0\xFF\xFF\xFF\xFF\xDC \xDA\xDE\xF4w\xFA\xAB(R\xF2\xF8\xAE\x0C\x82j\xA7\xE0\\M\xE0\xD3o\x0Ecc\x04)UH\x84\xC3q\xDAYt\xB6\xF3O\xA2\xC3Sg8\xF01\xB4\x9F4\xE0\xC9\xD0\x84\xD7(\x0F&!.9\0~\xBE\x9E\xA0\x87\x0C1'E\xB5\x81(\xA0\neW\x85\x1E\x98~\xCE\x02)M\x15o\0 3n\x15\x89(\xE8\x96B\x92d,lM\xC4i\xF3K{\xAC\xF2\xD8\xC4!\x15\xBA\xB6\xAF\xC9\x06\x7F.\xD3\x0E\x87Ir\x9Bc\xE0\x88\x9E >\xE5\x8E5Y\x03\xC1\xE7\x1Fx\xC0\x08\xDFl5\x97\xB2\xCCf\xD0\xB8\xAA\xE1\xA4\xA3<\xAAwT\x98\xE51\xCF\xB6\xAFX\xE8}\xB9\x9E\x0FSm\xD2&\xD1\x8FC\xE3\x86AH\xBA[\x7F\xAC\xA5\xC7u\xF1\x0B\xC8\x10\xC6\x02\xE1\xAF!\x95\xA3Ew\x97i!\xCE\0\x9AM\xDC\n\x07\xF6\x05\xC9k\x0F_\xCFX\x081\xEB\xBE\x01\xA3\x1F\xA2\x9B\xDE\x88F\t\xD2\x86\xDC\xCF\xA5\xBA\x8EU\x8C\xE3\x12[\xD4\xC3\xA1\x9E\x88\x8C\xF2hR(b\x02\xD2\xA7\xD3\x02\xC7^\x0F\xF5\xCA\x8F\xE7)\x9F\xB0\xD9\xD1\x13+\xF2\xC5l.;s\xDFy\x92\x86\x19=`\xC1\t\xB1\x87\xD6Eq\xEF\xBA\xA8\x04{\xE8X!\xF8\xE6~\x0E\x85\xF2\xF5\x89K\xC6=\0\xC2\xED\x9Dd",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b506004361061019a575f3560e01c8063916a17c6116100e8578063e20c9f7111610093578063ef3e68121161006e578063ef3e6812146102e2578063f11d5cbc146102ea578063fa7626d4146102f2578063fad06b8f146102ff575f5ffd5b8063e20c9f71146102d2578063e89f8419146102da578063ed5a9c49146102da575f5ffd5b8063b5508aa9116100c3578063b5508aa9146102aa578063ba414fa6146102b2578063c899d241146102ca575f5ffd5b8063916a17c614610285578063b0464fdc1461029a578063b52b2058146102a2575f5ffd5b80633f7286f411610148578063632ad53411610123578063632ad5341461025357806366d9a9a01461025b57806385226c8114610270575f5ffd5b80633f7286f41461022357806344badbb61461022b5780635df0d0761461024b575f5ffd5b80632ade3880116101785780632ade3880146101fc57806339425f8f146102115780633e5e3c231461021b575f5ffd5b80630813852a1461019e5780631c0da81f146101c75780631ed7831c146101e7575b5f5ffd5b6101b16101ac366004612544565b610312565b6040516101be91906125f9565b60405180910390f35b6101da6101d5366004612544565b61035d565b6040516101be919061265c565b6101ef6103cf565b6040516101be919061266e565b61020461043c565b6040516101be9190612720565b610219610585565b005b6101ef6106e1565b6101ef61074c565b61023e610239366004612544565b6107b7565b6040516101be91906127a4565b6102196107fa565b610219610ab5565b610263610dbf565b6040516101be9190612837565b610278610f38565b6040516101be91906128b5565b61028d611003565b6040516101be91906128c7565b61028d611106565b610219611209565b6102786112c6565b6102ba611391565b60405190151581526020016101be565b610219611461565b6101ef611665565b6102196116d0565b610219611940565b610219611b65565b601f546102ba9060ff1681565b61023e61030d366004612544565b611d45565b60606103558484846040518060400160405280600381526020017f6865780000000000000000000000000000000000000000000000000000000000815250611d88565b949350505050565b60605f61036b858585610312565b90505f5b6103798585612978565b8110156103c657828282815181106103935761039361298b565b60200260200101516040516020016103ac9291906129cf565b60408051601f19818403018152919052925060010161036f565b50509392505050565b6060601680548060200260200160405190810160405280929190818152602001828054801561043257602002820191905f5260205f20905b815473ffffffffffffffffffffffffffffffffffffffff168152600190910190602001808311610407575b5050505050905090565b6060601e805480602002602001604051908101604052809291908181526020015f905b8282101561057c575f848152602080822060408051808201825260028702909201805473ffffffffffffffffffffffffffffffffffffffff168352600181018054835181870281018701909452808452939591948681019491929084015b82821015610565578382905f5260205f200180546104da906129e3565b80601f0160208091040260200160405190810160405280929190818152602001828054610506906129e3565b80156105515780601f1061052857610100808354040283529160200191610551565b820191905f5260205f20905b81548152906001019060200180831161053457829003601f168201915b5050505050815260200190600101906104bd565b50505050815250508152602001906001019061045f565b50505050905090565b604080518082018252601a81527f496e73756666696369656e7420636f6e6669726d6174696f6e73000000000000602082015290517ff28dceb3000000000000000000000000000000000000000000000000000000008152600991737109709ecfa91a80626ff3989d68f67f5b1dd12d9163f28dceb3916106089160040161265c565b5f604051808303815f87803b15801561061f575f5ffd5b505af1158015610631573d5f5f3e3d5ffd5b5050602554601f546040517f2d79db4600000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff9283169450632d79db46935061069e92610100909204909116908590602890602c90600401612bec565b602060405180830381865afa1580156106b9573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906106dd9190612c3d565b5050565b6060601880548060200260200160405190810160405280929190818152602001828054801561043257602002820191905f5260205f2090815473ffffffffffffffffffffffffffffffffffffffff168152600190910190602001808311610407575050505050905090565b6060601780548060200260200160405190810160405280929190818152602001828054801561043257602002820191905f5260205f2090815473ffffffffffffffffffffffffffffffffffffffff168152600190910190602001808311610407575050505050905090565b60606103558484846040518060400160405280600981526020017f6469676573745f6c650000000000000000000000000000000000000000000000815250611ee9565b60408051608081019091526028805460e01b7fffffffff00000000000000000000000000000000000000000000000000000000168252602980545f9392916020840191610846906129e3565b80601f0160208091040260200160405190810160405280929190818152602001828054610872906129e3565b80156108bd5780601f10610894576101008083540402835291602001916108bd565b820191905f5260205f20905b8154815290600101906020018083116108a057829003601f168201915b505050505081526020016002820180546108d6906129e3565b80601f0160208091040260200160405190810160405280929190818152602001828054610902906129e3565b801561094d5780601f106109245761010080835404028352916020019161094d565b820191905f5260205f20905b81548152906001019060200180831161093057829003601f168201915b50505091835250506003919091015460e01b7fffffffff0000000000000000000000000000000000000000000000000000000016602091820152604080518082018252600181525f818401528382015280518082018252601e81527f496e76616c6964206f757470757420766563746f722070726f7669646564000092810192909252517ff28dceb3000000000000000000000000000000000000000000000000000000008152919250737109709ecfa91a80626ff3989d68f67f5b1dd12d9163f28dceb391610a1f9160040161265c565b5f604051808303815f87803b158015610a36575f5ffd5b505af1158015610a48573d5f5f3e3d5ffd5b5050602554601f546026546040517f2d79db4600000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff9384169550632d79db46945061069e93610100909304909216918690602c90600401612c54565b5f602c6040518060a00160405290815f82018054610ad2906129e3565b80601f0160208091040260200160405190810160405280929190818152602001828054610afe906129e3565b8015610b495780601f10610b2057610100808354040283529160200191610b49565b820191905f5260205f20905b815481529060010190602001808311610b2c57829003601f168201915b5050505050815260200160018201548152602001600282018054610b6c906129e3565b80601f0160208091040260200160405190810160405280929190818152602001828054610b98906129e3565b8015610be35780601f10610bba57610100808354040283529160200191610be3565b820191905f5260205f20905b815481529060010190602001808311610bc657829003601f168201915b5050505050815260200160038201548152602001600482018054610c06906129e3565b80601f0160208091040260200160405190810160405280929190818152602001828054610c32906129e3565b8015610c7d5780601f10610c5457610100808354040283529160200191610c7d565b820191905f5260205f20905b815481529060010190602001808311610c6057829003601f168201915b50505050508152505090506040518060800160405280604f8152602001612ff6604f913960408083019190915280518082018252601081527f4261642068656164657220626c6f636b00000000000000000000000000000000602082015290517ff28dceb3000000000000000000000000000000000000000000000000000000008152737109709ecfa91a80626ff3989d68f67f5b1dd12d9163f28dceb391610d29919060040161265c565b5f604051808303815f87803b158015610d40575f5ffd5b505af1158015610d52573d5f5f3e3d5ffd5b5050602554601f546026546040517f2d79db4600000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff9384169550632d79db46945061069e93610100909304909216916028908790600401612d3a565b6060601b805480602002602001604051908101604052809291908181526020015f905b8282101561057c578382905f5260205f2090600202016040518060400160405290815f82018054610e12906129e3565b80601f0160208091040260200160405190810160405280929190818152602001828054610e3e906129e3565b8015610e895780601f10610e6057610100808354040283529160200191610e89565b820191905f5260205f20905b815481529060010190602001808311610e6c57829003601f168201915b5050505050815260200160018201805480602002602001604051908101604052809291908181526020018280548015610f2057602002820191905f5260205f20905f905b82829054906101000a900460e01b7bffffffffffffffffffffffffffffffffffffffffffffffffffffffff191681526020019060040190602082600301049283019260010382029150808411610ecd5790505b50505050508152505081526020019060010190610de2565b6060601a805480602002602001604051908101604052809291908181526020015f905b8282101561057c578382905f5260205f20018054610f78906129e3565b80601f0160208091040260200160405190810160405280929190818152602001828054610fa4906129e3565b8015610fef5780601f10610fc657610100808354040283529160200191610fef565b820191905f5260205f20905b815481529060010190602001808311610fd257829003601f168201915b505050505081526020019060010190610f5b565b6060601d805480602002602001604051908101604052809291908181526020015f905b8282101561057c575f84815260209081902060408051808201825260028602909201805473ffffffffffffffffffffffffffffffffffffffff1683526001810180548351818702810187019094528084529394919385830193928301828280156110ee57602002820191905f5260205f20905f905b82829054906101000a900460e01b7bffffffffffffffffffffffffffffffffffffffffffffffffffffffff19168152602001906004019060208260030104928301926001038202915080841161109b5790505b50505050508152505081526020019060010190611026565b6060601c805480602002602001604051908101604052809291908181526020015f905b8282101561057c575f84815260209081902060408051808201825260028602909201805473ffffffffffffffffffffffffffffffffffffffff1683526001810180548351818702810187019094528084529394919385830193928301828280156111f157602002820191905f5260205f20905f905b82829054906101000a900460e01b7bffffffffffffffffffffffffffffffffffffffffffffffffffffffff19168152602001906004019060208260030104928301926001038202915080841161119e5790505b50505050508152505081526020019060010190611129565b602554601f546026546040517f2d79db460000000000000000000000000000000000000000000000000000000081525f9373ffffffffffffffffffffffffffffffffffffffff90811693632d79db4693611276936101009092049092169190602890602c90600401612bec565b602060405180830381865afa158015611291573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906112b59190612c3d565b90506112c381602754612037565b50565b60606019805480602002602001604051908101604052809291908181526020015f905b8282101561057c578382905f5260205f20018054611306906129e3565b80601f0160208091040260200160405190810160405280929190818152602001828054611332906129e3565b801561137d5780601f106113545761010080835404028352916020019161137d565b820191905f5260205f20905b81548152906001019060200180831161136057829003601f168201915b5050505050815260200190600101906112e9565b6008545f9060ff16156113a8575060085460ff1690565b6040517f667f9d70000000000000000000000000000000000000000000000000000000008152737109709ecfa91a80626ff3989d68f67f5b1dd12d600482018190527f6661696c6564000000000000000000000000000000000000000000000000000060248301525f9163667f9d7090604401602060405180830381865afa158015611436573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061145a9190612c3d565b1415905090565b60408051608081019091526028805460e01b7fffffffff00000000000000000000000000000000000000000000000000000000168252602980545f93929160208401916114ad906129e3565b80601f01602080910402602001604051908101604052809291908181526020018280546114d9906129e3565b80156115245780601f106114fb57610100808354040283529160200191611524565b820191905f5260205f20905b81548152906001019060200180831161150757829003601f168201915b5050505050815260200160028201805461153d906129e3565b80601f0160208091040260200160405190810160405280929190818152602001828054611569906129e3565b80156115b45780601f1061158b576101008083540402835291602001916115b4565b820191905f5260205f20905b81548152906001019060200180831161159757829003601f168201915b50505091835250506003919091015460e01b7fffffffff00000000000000000000000000000000000000000000000000000000166020918201527c0100000000000000000000000000000000000000000000000000000000606083810191909152604080519182019052603c808252929350737109709ecfa91a80626ff3989d68f67f5b1dd12d9263f28dceb392612fba908301396040518263ffffffff1660e01b8152600401610a1f919061265c565b6060601580548060200260200160405190810160405280929190818152602001828054801561043257602002820191905f5260205f2090815473ffffffffffffffffffffffffffffffffffffffff168152600190910190602001808311610407575050505050905090565b5f602c6040518060a00160405290815f820180546116ed906129e3565b80601f0160208091040260200160405190810160405280929190818152602001828054611719906129e3565b80156117645780601f1061173b57610100808354040283529160200191611764565b820191905f5260205f20905b81548152906001019060200180831161174757829003601f168201915b5050505050815260200160018201548152602001600282018054611787906129e3565b80601f01602080910402602001604051908101604052809291908181526020018280546117b3906129e3565b80156117fe5780601f106117d5576101008083540402835291602001916117fe565b820191905f5260205f20905b8154815290600101906020018083116117e157829003601f168201915b5050505050815260200160038201548152602001600482018054611821906129e3565b80601f016020809104026020016040519081016040528092919081815260200182805461184d906129e3565b80156118985780601f1061186f57610100808354040283529160200191611898565b820191905f5260205f20905b81548152906001019060200180831161187b57829003601f168201915b505050919092525050604080518082018252600181525f60208083019190915290845281518083018352601681527f426164206d65726b6c652061727261792070726f6f66000000000000000000009181019190915290517ff28dceb3000000000000000000000000000000000000000000000000000000008152929350737109709ecfa91a80626ff3989d68f67f5b1dd12d9263f28dceb39250610d29919060040161265c565b60408051608081019091526028805460e01b7fffffffff00000000000000000000000000000000000000000000000000000000168252602980545f939291602084019161198c906129e3565b80601f01602080910402602001604051908101604052809291908181526020018280546119b8906129e3565b8015611a035780601f106119da57610100808354040283529160200191611a03565b820191905f5260205f20905b8154815290600101906020018083116119e657829003601f168201915b50505050508152602001600282018054611a1c906129e3565b80601f0160208091040260200160405190810160405280929190818152602001828054611a48906129e3565b8015611a935780601f10611a6a57610100808354040283529160200191611a93565b820191905f5260205f20905b815481529060010190602001808311611a7657829003601f168201915b50505091835250506003919091015460e01b7fffffffff0000000000000000000000000000000000000000000000000000000016602091820152604080518082018252600181525f818401528383015280518082018252601d81527f496e76616c696420696e70757420766563746f722070726f766964656400000092810192909252517ff28dceb3000000000000000000000000000000000000000000000000000000008152919250737109709ecfa91a80626ff3989d68f67f5b1dd12d9163f28dceb391610a1f9160040161265c565b601f546040517ff58db06f0000000000000000000000000000000000000000000000000000000081525f60048201819052602482015261010090910473ffffffffffffffffffffffffffffffffffffffff169063f58db06f906044015f604051808303815f87803b158015611bd8575f5ffd5b505af1158015611bea573d5f5f3e3d5ffd5b5050604080518082018252601b81527f47434420646f6573206e6f7420636f6e6669726d206865616465720000000000602082015290517ff28dceb3000000000000000000000000000000000000000000000000000000008152737109709ecfa91a80626ff3989d68f67f5b1dd12d935063f28dceb39250611c6f919060040161265c565b5f604051808303815f87803b158015611c86575f5ffd5b505af1158015611c98573d5f5f3e3d5ffd5b5050602554601f546026546040517f2d79db4600000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff9384169550632d79db469450611d069361010090930490921691602890602c90600401612bec565b602060405180830381865afa158015611d21573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906112c39190612c3d565b60606103558484846040518060400160405280600681526020017f68656967687400000000000000000000000000000000000000000000000000008152506120ba565b6060611d948484612978565b67ffffffffffffffff811115611dac57611dac6124bf565b604051908082528060200260200182016040528015611ddf57816020015b6060815260200190600190039081611dca5790505b509050835b83811015611ee057611eb286611df983612208565b85604051602001611e0c93929190612ddd565b60405160208183030381529060405260208054611e28906129e3565b80601f0160208091040260200160405190810160405280929190818152602001828054611e54906129e3565b8015611e9f5780601f10611e7657610100808354040283529160200191611e9f565b820191905f5260205f20905b815481529060010190602001808311611e8257829003601f168201915b505050505061233990919063ffffffff16565b82611ebd8784612978565b81518110611ecd57611ecd61298b565b6020908102919091010152600101611de4565b50949350505050565b6060611ef58484612978565b67ffffffffffffffff811115611f0d57611f0d6124bf565b604051908082528060200260200182016040528015611f36578160200160208202803683370190505b509050835b83811015611ee05761200986611f5083612208565b85604051602001611f6393929190612ddd565b60405160208183030381529060405260208054611f7f906129e3565b80601f0160208091040260200160405190810160405280929190818152602001828054611fab906129e3565b8015611ff65780601f10611fcd57610100808354040283529160200191611ff6565b820191905f5260205f20905b815481529060010190602001808311611fd957829003601f168201915b50505050506123d890919063ffffffff16565b826120148784612978565b815181106120245761202461298b565b6020908102919091010152600101611f3b565b6040517f7c84c69b0000000000000000000000000000000000000000000000000000000081526004810183905260248101829052737109709ecfa91a80626ff3989d68f67f5b1dd12d90637c84c69b906044015f6040518083038186803b1580156120a0575f5ffd5b505afa1580156120b2573d5f5f3e3d5ffd5b505050505050565b60606120c68484612978565b67ffffffffffffffff8111156120de576120de6124bf565b604051908082528060200260200182016040528015612107578160200160208202803683370190505b509050835b83811015611ee0576121da8661212183612208565b8560405160200161213493929190612ddd565b60405160208183030381529060405260208054612150906129e3565b80601f016020809104026020016040519081016040528092919081815260200182805461217c906129e3565b80156121c75780601f1061219e576101008083540402835291602001916121c7565b820191905f5260205f20905b8154815290600101906020018083116121aa57829003601f168201915b505050505061246b90919063ffffffff16565b826121e58784612978565b815181106121f5576121f561298b565b602090810291909101015260010161210c565b6060815f0361224a57505060408051808201909152600181527f3000000000000000000000000000000000000000000000000000000000000000602082015290565b815f5b8115612273578061225d81612e7a565b915061226c9050600a83612ede565b915061224d565b5f8167ffffffffffffffff81111561228d5761228d6124bf565b6040519080825280601f01601f1916602001820160405280156122b7576020820181803683370190505b5090505b8415610355576122cc600183612978565b91506122d9600a86612ef1565b6122e4906030612f04565b60f81b8183815181106122f9576122f961298b565b60200101907effffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff191690815f1a905350612332600a86612ede565b94506122bb565b6040517ffd921be8000000000000000000000000000000000000000000000000000000008152606090737109709ecfa91a80626ff3989d68f67f5b1dd12d9063fd921be89061238e9086908690600401612f17565b5f60405180830381865afa1580156123a8573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526123cf9190810190612f44565b90505b92915050565b6040517f1777e59d0000000000000000000000000000000000000000000000000000000081525f90737109709ecfa91a80626ff3989d68f67f5b1dd12d90631777e59d9061242c9086908690600401612f17565b602060405180830381865afa158015612447573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906123cf9190612c3d565b6040517faddde2b60000000000000000000000000000000000000000000000000000000081525f90737109709ecfa91a80626ff3989d68f67f5b1dd12d9063addde2b69061242c9086908690600401612f17565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b604051601f8201601f1916810167ffffffffffffffff81118282101715612515576125156124bf565b604052919050565b5f67ffffffffffffffff821115612536576125366124bf565b50601f01601f191660200190565b5f5f5f60608486031215612556575f5ffd5b833567ffffffffffffffff81111561256c575f5ffd5b8401601f8101861361257c575f5ffd5b803561258f61258a8261251d565b6124ec565b8181528760208385010111156125a3575f5ffd5b816020840160208301375f602092820183015297908601359650604090950135949350505050565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b8281101561265057603f1987860301845261263b8583516125cb565b9450602093840193919091019060010161261f565b50929695505050505050565b602081525f6123cf60208301846125cb565b602080825282518282018190525f918401906040840190835b818110156126bb57835173ffffffffffffffffffffffffffffffffffffffff16835260209384019390920191600101612687565b509095945050505050565b5f82825180855260208501945060208160051b830101602085015f5b8381101561271457601f198584030188526126fe8383516125cb565b60209889019890935091909101906001016126e2565b50909695505050505050565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b8281101561265057603f19878603018452815173ffffffffffffffffffffffffffffffffffffffff8151168652602081015190506040602087015261278e60408701826126c6565b9550506020938401939190910190600101612746565b602080825282518282018190525f918401906040840190835b818110156126bb5783518352602093840193909201916001016127bd565b5f8151808452602084019350602083015f5b8281101561282d5781517fffffffff00000000000000000000000000000000000000000000000000000000168652602095860195909101906001016127ed565b5093949350505050565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b8281101561265057603f19878603018452815180516040875261288360408801826125cb565b905060208201519150868103602088015261289e81836127db565b96505050602093840193919091019060010161285d565b602081525f6123cf60208301846126c6565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b8281101561265057603f19878603018452815173ffffffffffffffffffffffffffffffffffffffff8151168652602081015190506040602087015261293560408701826127db565b95505060209384019391909101906001016128ed565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b818103818111156123d2576123d261294b565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f81518060208401855e5f93019283525090919050565b5f6103556129dd83866129b8565b846129b8565b600181811c908216806129f757607f821691505b602082108103612a2e577f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b50919050565b80545f90600181811c90821680612a4c57607f821691505b602082108103612a83577f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b81865260208601818015612a9e5760018114612ad257612afe565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff008516825283151560051b82019550612afe565b5f878152602090205f5b85811015612af857815484820152600190910190602001612adc565b83019650505b505050505092915050565b7fffffffff00000000000000000000000000000000000000000000000000000000815460e01b168252608060208301525f612b4a6080840160018401612a34565b8381036040850152612b5f8160028501612a34565b90507fffffffff00000000000000000000000000000000000000000000000000000000600384015460e01b1660608501528091505092915050565b60a082525f612bac60a0840183612a34565b600183015460208501528381036040850152612bcb8160028501612a34565b90506003830154606085015283810360808501526103558160048501612a34565b73ffffffffffffffffffffffffffffffffffffffff85168152836020820152608060408201525f612c206080830185612b09565b8281036060840152612c328185612b9a565b979650505050505050565b5f60208284031215612c4d575f5ffd5b5051919050565b73ffffffffffffffffffffffffffffffffffffffff85168152836020820152608060408201527fffffffff0000000000000000000000000000000000000000000000000000000083511660808201525f6020840151608060a0840152612cbe6101008401826125cb565b905060408501517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff808483030160c0850152612cf982826125cb565b9150507fffffffff0000000000000000000000000000000000000000000000000000000060608601511660e08401528281036060840152612c328185612b9a565b73ffffffffffffffffffffffffffffffffffffffff85168152836020820152608060408201525f612d6e6080830185612b09565b8281036060840152835160a08252612d8960a08301826125cb565b90506020850151602083015260408501518282036040840152612dac82826125cb565b9150506060850151606083015260808501518282036080840152612dd082826125cb565b9998505050505050505050565b7f2e0000000000000000000000000000000000000000000000000000000000000081525f612e0e60018301866129b8565b7f5b000000000000000000000000000000000000000000000000000000000000008152612e3e60018201866129b8565b90507f5d2e0000000000000000000000000000000000000000000000000000000000008152612e7060028201856129b8565b9695505050505050565b5f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8203612eaa57612eaa61294b565b5060010190565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f82612eec57612eec612eb1565b500490565b5f82612eff57612eff612eb1565b500690565b808201808211156123d2576123d261294b565b604081525f612f2960408301856125cb565b8281036020840152612f3b81856125cb565b95945050505050565b5f60208284031215612f54575f5ffd5b815167ffffffffffffffff811115612f6a575f5ffd5b8201601f81018413612f7a575f5ffd5b8051612f8861258a8261251d565b818152856020838501011115612f9c575f5ffd5b8160208401602083015e5f9181016020019190915294935050505056fe5478206d65726b6c652070726f6f66206973206e6f742076616c696420666f722070726f76696465642068656164657220616e6420747820686173680000002073bd2184edd9c4fc76642ea6754ee40136970efc10c4190000000000000000000296ef123ea96da5cf695f22bf7d94be87d49db1ad7ac371ac43c4da4161c8c216349c5ba11928170d3878a2646970667358221220fdef3650b697a7d9ffba1a56b59715ef8fc9a34baf19fb7d99de5751ffca908464736f6c634300081c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x01\x9AW_5`\xE0\x1C\x80c\x91j\x17\xC6\x11a\0\xE8W\x80c\xE2\x0C\x9Fq\x11a\0\x93W\x80c\xEF>h\x12\x11a\0nW\x80c\xEF>h\x12\x14a\x02\xE2W\x80c\xF1\x1D\\\xBC\x14a\x02\xEAW\x80c\xFAv&\xD4\x14a\x02\xF2W\x80c\xFA\xD0k\x8F\x14a\x02\xFFW__\xFD[\x80c\xE2\x0C\x9Fq\x14a\x02\xD2W\x80c\xE8\x9F\x84\x19\x14a\x02\xDAW\x80c\xEDZ\x9CI\x14a\x02\xDAW__\xFD[\x80c\xB5P\x8A\xA9\x11a\0\xC3W\x80c\xB5P\x8A\xA9\x14a\x02\xAAW\x80c\xBAAO\xA6\x14a\x02\xB2W\x80c\xC8\x99\xD2A\x14a\x02\xCAW__\xFD[\x80c\x91j\x17\xC6\x14a\x02\x85W\x80c\xB0FO\xDC\x14a\x02\x9AW\x80c\xB5+ X\x14a\x02\xA2W__\xFD[\x80c?r\x86\xF4\x11a\x01HW\x80cc*\xD54\x11a\x01#W\x80cc*\xD54\x14a\x02SW\x80cf\xD9\xA9\xA0\x14a\x02[W\x80c\x85\"l\x81\x14a\x02pW__\xFD[\x80c?r\x86\xF4\x14a\x02#W\x80cD\xBA\xDB\xB6\x14a\x02+W\x80c]\xF0\xD0v\x14a\x02KW__\xFD[\x80c*\xDE8\x80\x11a\x01xW\x80c*\xDE8\x80\x14a\x01\xFCW\x80c9B_\x8F\x14a\x02\x11W\x80c>^<#\x14a\x02\x1BW__\xFD[\x80c\x08\x13\x85*\x14a\x01\x9EW\x80c\x1C\r\xA8\x1F\x14a\x01\xC7W\x80c\x1E\xD7\x83\x1C\x14a\x01\xE7W[__\xFD[a\x01\xB1a\x01\xAC6`\x04a%DV[a\x03\x12V[`@Qa\x01\xBE\x91\x90a%\xF9V[`@Q\x80\x91\x03\x90\xF3[a\x01\xDAa\x01\xD56`\x04a%DV[a\x03]V[`@Qa\x01\xBE\x91\x90a&\\V[a\x01\xEFa\x03\xCFV[`@Qa\x01\xBE\x91\x90a&nV[a\x02\x04a\x04<V[`@Qa\x01\xBE\x91\x90a' V[a\x02\x19a\x05\x85V[\0[a\x01\xEFa\x06\xE1V[a\x01\xEFa\x07LV[a\x02>a\x0296`\x04a%DV[a\x07\xB7V[`@Qa\x01\xBE\x91\x90a'\xA4V[a\x02\x19a\x07\xFAV[a\x02\x19a\n\xB5V[a\x02ca\r\xBFV[`@Qa\x01\xBE\x91\x90a(7V[a\x02xa\x0F8V[`@Qa\x01\xBE\x91\x90a(\xB5V[a\x02\x8Da\x10\x03V[`@Qa\x01\xBE\x91\x90a(\xC7V[a\x02\x8Da\x11\x06V[a\x02\x19a\x12\tV[a\x02xa\x12\xC6V[a\x02\xBAa\x13\x91V[`@Q\x90\x15\x15\x81R` \x01a\x01\xBEV[a\x02\x19a\x14aV[a\x01\xEFa\x16eV[a\x02\x19a\x16\xD0V[a\x02\x19a\x19@V[a\x02\x19a\x1BeV[`\x1FTa\x02\xBA\x90`\xFF\x16\x81V[a\x02>a\x03\r6`\x04a%DV[a\x1DEV[``a\x03U\x84\x84\x84`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01\x7Fhex\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x1D\x88V[\x94\x93PPPPV[``_a\x03k\x85\x85\x85a\x03\x12V[\x90P_[a\x03y\x85\x85a)xV[\x81\x10\x15a\x03\xC6W\x82\x82\x82\x81Q\x81\x10a\x03\x93Wa\x03\x93a)\x8BV[` \x02` \x01\x01Q`@Q` \x01a\x03\xAC\x92\x91\x90a)\xCFV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x92P`\x01\x01a\x03oV[PP\x93\x92PPPV[```\x16\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x042W` \x02\x82\x01\x91\x90_R` _ \x90[\x81Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x04\x07W[PPPPP\x90P\x90V[```\x1E\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x05|W_\x84\x81R` \x80\x82 `@\x80Q\x80\x82\x01\x82R`\x02\x87\x02\x90\x92\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x95\x91\x94\x86\x81\x01\x94\x91\x92\x90\x84\x01[\x82\x82\x10\x15a\x05eW\x83\x82\x90_R` _ \x01\x80Ta\x04\xDA\x90a)\xE3V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x05\x06\x90a)\xE3V[\x80\x15a\x05QW\x80`\x1F\x10a\x05(Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x05QV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x054W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x04\xBDV[PPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x04_V[PPPP\x90P\x90V[`@\x80Q\x80\x82\x01\x82R`\x1A\x81R\x7FInsufficient confirmations\0\0\0\0\0\0` \x82\x01R\x90Q\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\t\x91sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x91c\xF2\x8D\xCE\xB3\x91a\x06\x08\x91`\x04\x01a&\\V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x06\x1FW__\xFD[PZ\xF1\x15\x80\x15a\x061W=__>=_\xFD[PP`%T`\x1FT`@Q\x7F-y\xDBF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x16\x94Pc-y\xDBF\x93Pa\x06\x9E\x92a\x01\0\x90\x92\x04\x90\x91\x16\x90\x85\x90`(\x90`,\x90`\x04\x01a+\xECV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\xB9W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xDD\x91\x90a,=V[PPV[```\x18\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x042W` \x02\x82\x01\x91\x90_R` _ \x90\x81Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x04\x07WPPPPP\x90P\x90V[```\x17\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x042W` \x02\x82\x01\x91\x90_R` _ \x90\x81Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x04\x07WPPPPP\x90P\x90V[``a\x03U\x84\x84\x84`@Q\x80`@\x01`@R\x80`\t\x81R` \x01\x7Fdigest_le\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x1E\xE9V[`@\x80Q`\x80\x81\x01\x90\x91R`(\x80T`\xE0\x1B\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x82R`)\x80T_\x93\x92\x91` \x84\x01\x91a\x08F\x90a)\xE3V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x08r\x90a)\xE3V[\x80\x15a\x08\xBDW\x80`\x1F\x10a\x08\x94Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x08\xBDV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x08\xA0W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x02\x82\x01\x80Ta\x08\xD6\x90a)\xE3V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\t\x02\x90a)\xE3V[\x80\x15a\tMW\x80`\x1F\x10a\t$Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\tMV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\t0W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x83RPP`\x03\x91\x90\x91\x01T`\xE0\x1B\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16` \x91\x82\x01R`@\x80Q\x80\x82\x01\x82R`\x01\x81R_\x81\x84\x01R\x83\x82\x01R\x80Q\x80\x82\x01\x82R`\x1E\x81R\x7FInvalid output vector provided\0\0\x92\x81\x01\x92\x90\x92RQ\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x91\x92Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x91c\xF2\x8D\xCE\xB3\x91a\n\x1F\x91`\x04\x01a&\\V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\n6W__\xFD[PZ\xF1\x15\x80\x15a\nHW=__>=_\xFD[PP`%T`\x1FT`&T`@Q\x7F-y\xDBF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x84\x16\x95Pc-y\xDBF\x94Pa\x06\x9E\x93a\x01\0\x90\x93\x04\x90\x92\x16\x91\x86\x90`,\x90`\x04\x01a,TV[_`,`@Q\x80`\xA0\x01`@R\x90\x81_\x82\x01\x80Ta\n\xD2\x90a)\xE3V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\n\xFE\x90a)\xE3V[\x80\x15a\x0BIW\x80`\x1F\x10a\x0B Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0BIV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0B,W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x01\x82\x01T\x81R` \x01`\x02\x82\x01\x80Ta\x0Bl\x90a)\xE3V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0B\x98\x90a)\xE3V[\x80\x15a\x0B\xE3W\x80`\x1F\x10a\x0B\xBAWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0B\xE3V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0B\xC6W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x03\x82\x01T\x81R` \x01`\x04\x82\x01\x80Ta\x0C\x06\x90a)\xE3V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0C2\x90a)\xE3V[\x80\x15a\x0C}W\x80`\x1F\x10a\x0CTWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0C}V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0C`W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P`@Q\x80`\x80\x01`@R\x80`O\x81R` \x01a/\xF6`O\x919`@\x80\x83\x01\x91\x90\x91R\x80Q\x80\x82\x01\x82R`\x10\x81R\x7FBad header block\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x90Q\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x91c\xF2\x8D\xCE\xB3\x91a\r)\x91\x90`\x04\x01a&\\V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\r@W__\xFD[PZ\xF1\x15\x80\x15a\rRW=__>=_\xFD[PP`%T`\x1FT`&T`@Q\x7F-y\xDBF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x84\x16\x95Pc-y\xDBF\x94Pa\x06\x9E\x93a\x01\0\x90\x93\x04\x90\x92\x16\x91`(\x90\x87\x90`\x04\x01a-:V[```\x1B\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x05|W\x83\x82\x90_R` _ \x90`\x02\x02\x01`@Q\x80`@\x01`@R\x90\x81_\x82\x01\x80Ta\x0E\x12\x90a)\xE3V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0E>\x90a)\xE3V[\x80\x15a\x0E\x89W\x80`\x1F\x10a\x0E`Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0E\x89V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0ElW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x01\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x0F W` \x02\x82\x01\x91\x90_R` _ \x90_\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x0E\xCDW\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\r\xE2V[```\x1A\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x05|W\x83\x82\x90_R` _ \x01\x80Ta\x0Fx\x90a)\xE3V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0F\xA4\x90a)\xE3V[\x80\x15a\x0F\xEFW\x80`\x1F\x10a\x0F\xC6Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0F\xEFV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0F\xD2W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x0F[V[```\x1D\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x05|W_\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x10\xEEW` \x02\x82\x01\x91\x90_R` _ \x90_\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x10\x9BW\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x10&V[```\x1C\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x05|W_\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x11\xF1W` \x02\x82\x01\x91\x90_R` _ \x90_\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x11\x9EW\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x11)V[`%T`\x1FT`&T`@Q\x7F-y\xDBF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R_\x93s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x93c-y\xDBF\x93a\x12v\x93a\x01\0\x90\x92\x04\x90\x92\x16\x91\x90`(\x90`,\x90`\x04\x01a+\xECV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12\x91W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\xB5\x91\x90a,=V[\x90Pa\x12\xC3\x81`'Ta 7V[PV[```\x19\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x05|W\x83\x82\x90_R` _ \x01\x80Ta\x13\x06\x90a)\xE3V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x132\x90a)\xE3V[\x80\x15a\x13}W\x80`\x1F\x10a\x13TWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x13}V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x13`W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x12\xE9V[`\x08T_\x90`\xFF\x16\x15a\x13\xA8WP`\x08T`\xFF\x16\x90V[`@Q\x7Ff\x7F\x9Dp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-`\x04\x82\x01\x81\x90R\x7Ffailed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`$\x83\x01R_\x91cf\x7F\x9Dp\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x146W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14Z\x91\x90a,=V[\x14\x15\x90P\x90V[`@\x80Q`\x80\x81\x01\x90\x91R`(\x80T`\xE0\x1B\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x82R`)\x80T_\x93\x92\x91` \x84\x01\x91a\x14\xAD\x90a)\xE3V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x14\xD9\x90a)\xE3V[\x80\x15a\x15$W\x80`\x1F\x10a\x14\xFBWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x15$V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x15\x07W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x02\x82\x01\x80Ta\x15=\x90a)\xE3V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x15i\x90a)\xE3V[\x80\x15a\x15\xB4W\x80`\x1F\x10a\x15\x8BWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x15\xB4V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x15\x97W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x83RPP`\x03\x91\x90\x91\x01T`\xE0\x1B\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16` \x91\x82\x01R|\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x83\x81\x01\x91\x90\x91R`@\x80Q\x91\x82\x01\x90R`<\x80\x82R\x92\x93Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x92c\xF2\x8D\xCE\xB3\x92a/\xBA\x90\x83\x019`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\n\x1F\x91\x90a&\\V[```\x15\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x042W` \x02\x82\x01\x91\x90_R` _ \x90\x81Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x04\x07WPPPPP\x90P\x90V[_`,`@Q\x80`\xA0\x01`@R\x90\x81_\x82\x01\x80Ta\x16\xED\x90a)\xE3V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x17\x19\x90a)\xE3V[\x80\x15a\x17dW\x80`\x1F\x10a\x17;Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x17dV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x17GW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x01\x82\x01T\x81R` \x01`\x02\x82\x01\x80Ta\x17\x87\x90a)\xE3V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x17\xB3\x90a)\xE3V[\x80\x15a\x17\xFEW\x80`\x1F\x10a\x17\xD5Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x17\xFEV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x17\xE1W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x03\x82\x01T\x81R` \x01`\x04\x82\x01\x80Ta\x18!\x90a)\xE3V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x18M\x90a)\xE3V[\x80\x15a\x18\x98W\x80`\x1F\x10a\x18oWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x18\x98V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x18{W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x90\x92RPP`@\x80Q\x80\x82\x01\x82R`\x01\x81R_` \x80\x83\x01\x91\x90\x91R\x90\x84R\x81Q\x80\x83\x01\x83R`\x16\x81R\x7FBad merkle array proof\0\0\0\0\0\0\0\0\0\0\x91\x81\x01\x91\x90\x91R\x90Q\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x92\x93Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x92c\xF2\x8D\xCE\xB3\x92Pa\r)\x91\x90`\x04\x01a&\\V[`@\x80Q`\x80\x81\x01\x90\x91R`(\x80T`\xE0\x1B\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x82R`)\x80T_\x93\x92\x91` \x84\x01\x91a\x19\x8C\x90a)\xE3V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x19\xB8\x90a)\xE3V[\x80\x15a\x1A\x03W\x80`\x1F\x10a\x19\xDAWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1A\x03V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x19\xE6W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x02\x82\x01\x80Ta\x1A\x1C\x90a)\xE3V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1AH\x90a)\xE3V[\x80\x15a\x1A\x93W\x80`\x1F\x10a\x1AjWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1A\x93V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1AvW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x83RPP`\x03\x91\x90\x91\x01T`\xE0\x1B\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16` \x91\x82\x01R`@\x80Q\x80\x82\x01\x82R`\x01\x81R_\x81\x84\x01R\x83\x83\x01R\x80Q\x80\x82\x01\x82R`\x1D\x81R\x7FInvalid input vector provided\0\0\0\x92\x81\x01\x92\x90\x92RQ\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x91\x92Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x91c\xF2\x8D\xCE\xB3\x91a\n\x1F\x91`\x04\x01a&\\V[`\x1FT`@Q\x7F\xF5\x8D\xB0o\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R_`\x04\x82\x01\x81\x90R`$\x82\x01Ra\x01\0\x90\x91\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90c\xF5\x8D\xB0o\x90`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1B\xD8W__\xFD[PZ\xF1\x15\x80\x15a\x1B\xEAW=__>=_\xFD[PP`@\x80Q\x80\x82\x01\x82R`\x1B\x81R\x7FGCD does not confirm header\0\0\0\0\0` \x82\x01R\x90Q\x7F\xF2\x8D\xCE\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x93Pc\xF2\x8D\xCE\xB3\x92Pa\x1Co\x91\x90`\x04\x01a&\\V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1C\x86W__\xFD[PZ\xF1\x15\x80\x15a\x1C\x98W=__>=_\xFD[PP`%T`\x1FT`&T`@Q\x7F-y\xDBF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x84\x16\x95Pc-y\xDBF\x94Pa\x1D\x06\x93a\x01\0\x90\x93\x04\x90\x92\x16\x91`(\x90`,\x90`\x04\x01a+\xECV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1D!W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\xC3\x91\x90a,=V[``a\x03U\x84\x84\x84`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01\x7Fheight\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa \xBAV[``a\x1D\x94\x84\x84a)xV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1D\xACWa\x1D\xACa$\xBFV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1D\xDFW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x1D\xCAW\x90P[P\x90P\x83[\x83\x81\x10\x15a\x1E\xE0Wa\x1E\xB2\x86a\x1D\xF9\x83a\"\x08V[\x85`@Q` \x01a\x1E\x0C\x93\x92\x91\x90a-\xDDV[`@Q` \x81\x83\x03\x03\x81R\x90`@R` \x80Ta\x1E(\x90a)\xE3V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1ET\x90a)\xE3V[\x80\x15a\x1E\x9FW\x80`\x1F\x10a\x1EvWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1E\x9FV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1E\x82W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPPa#9\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x82a\x1E\xBD\x87\x84a)xV[\x81Q\x81\x10a\x1E\xCDWa\x1E\xCDa)\x8BV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x1D\xE4V[P\x94\x93PPPPV[``a\x1E\xF5\x84\x84a)xV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1F\rWa\x1F\ra$\xBFV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1F6W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P\x83[\x83\x81\x10\x15a\x1E\xE0Wa \t\x86a\x1FP\x83a\"\x08V[\x85`@Q` \x01a\x1Fc\x93\x92\x91\x90a-\xDDV[`@Q` \x81\x83\x03\x03\x81R\x90`@R` \x80Ta\x1F\x7F\x90a)\xE3V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1F\xAB\x90a)\xE3V[\x80\x15a\x1F\xF6W\x80`\x1F\x10a\x1F\xCDWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1F\xF6V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1F\xD9W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPPa#\xD8\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x82a \x14\x87\x84a)xV[\x81Q\x81\x10a $Wa $a)\x8BV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x1F;V[`@Q\x7F|\x84\xC6\x9B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x82\x90Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c|\x84\xC6\x9B\x90`D\x01_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a \xA0W__\xFD[PZ\xFA\x15\x80\x15a \xB2W=__>=_\xFD[PPPPPPV[``a \xC6\x84\x84a)xV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a \xDEWa \xDEa$\xBFV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a!\x07W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P\x83[\x83\x81\x10\x15a\x1E\xE0Wa!\xDA\x86a!!\x83a\"\x08V[\x85`@Q` \x01a!4\x93\x92\x91\x90a-\xDDV[`@Q` \x81\x83\x03\x03\x81R\x90`@R` \x80Ta!P\x90a)\xE3V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta!|\x90a)\xE3V[\x80\x15a!\xC7W\x80`\x1F\x10a!\x9EWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a!\xC7V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a!\xAAW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPPa$k\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x82a!\xE5\x87\x84a)xV[\x81Q\x81\x10a!\xF5Wa!\xF5a)\x8BV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a!\x0CV[``\x81_\x03a\"JWPP`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R\x7F0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x90V[\x81_[\x81\x15a\"sW\x80a\"]\x81a.zV[\x91Pa\"l\x90P`\n\x83a.\xDEV[\x91Pa\"MV[_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\"\x8DWa\"\x8Da$\xBFV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\"\xB7W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P[\x84\x15a\x03UWa\"\xCC`\x01\x83a)xV[\x91Pa\"\xD9`\n\x86a.\xF1V[a\"\xE4\x90`0a/\x04V[`\xF8\x1B\x81\x83\x81Q\x81\x10a\"\xF9Wa\"\xF9a)\x8BV[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81_\x1A\x90SPa#2`\n\x86a.\xDEV[\x94Pa\"\xBBV[`@Q\x7F\xFD\x92\x1B\xE8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R``\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\xFD\x92\x1B\xE8\x90a#\x8E\x90\x86\x90\x86\x90`\x04\x01a/\x17V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a#\xA8W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra#\xCF\x91\x90\x81\x01\x90a/DV[\x90P[\x92\x91PPV[`@Q\x7F\x17w\xE5\x9D\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R_\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\x17w\xE5\x9D\x90a$,\x90\x86\x90\x86\x90`\x04\x01a/\x17V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a$GW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a#\xCF\x91\x90a,=V[`@Q\x7F\xAD\xDD\xE2\xB6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R_\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\xAD\xDD\xE2\xB6\x90a$,\x90\x86\x90\x86\x90`\x04\x01a/\x17V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a%\x15Wa%\x15a$\xBFV[`@R\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a%6Wa%6a$\xBFV[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[___``\x84\x86\x03\x12\x15a%VW__\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a%lW__\xFD[\x84\x01`\x1F\x81\x01\x86\x13a%|W__\xFD[\x805a%\x8Fa%\x8A\x82a%\x1DV[a$\xECV[\x81\x81R\x87` \x83\x85\x01\x01\x11\x15a%\xA3W__\xFD[\x81` \x84\x01` \x83\x017_` \x92\x82\x01\x83\x01R\x97\x90\x86\x015\x96P`@\x90\x95\x015\x94\x93PPPPV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a&PW`?\x19\x87\x86\x03\x01\x84Ra&;\x85\x83Qa%\xCBV[\x94P` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a&\x1FV[P\x92\x96\x95PPPPPPV[` \x81R_a#\xCF` \x83\x01\x84a%\xCBV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a&\xBBW\x83Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a&\x87V[P\x90\x95\x94PPPPPV[_\x82\x82Q\x80\x85R` \x85\x01\x94P` \x81`\x05\x1B\x83\x01\x01` \x85\x01_[\x83\x81\x10\x15a'\x14W`\x1F\x19\x85\x84\x03\x01\x88Ra&\xFE\x83\x83Qa%\xCBV[` \x98\x89\x01\x98\x90\x93P\x91\x90\x91\x01\x90`\x01\x01a&\xE2V[P\x90\x96\x95PPPPPPV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a&PW`?\x19\x87\x86\x03\x01\x84R\x81Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16\x86R` \x81\x01Q\x90P`@` \x87\x01Ra'\x8E`@\x87\x01\x82a&\xC6V[\x95PP` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a'FV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a&\xBBW\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a'\xBDV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15a(-W\x81Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01a'\xEDV[P\x93\x94\x93PPPPV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a&PW`?\x19\x87\x86\x03\x01\x84R\x81Q\x80Q`@\x87Ra(\x83`@\x88\x01\x82a%\xCBV[\x90P` \x82\x01Q\x91P\x86\x81\x03` \x88\x01Ra(\x9E\x81\x83a'\xDBV[\x96PPP` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a(]V[` \x81R_a#\xCF` \x83\x01\x84a&\xC6V[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a&PW`?\x19\x87\x86\x03\x01\x84R\x81Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16\x86R` \x81\x01Q\x90P`@` \x87\x01Ra)5`@\x87\x01\x82a'\xDBV[\x95PP` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a(\xEDV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a#\xD2Wa#\xD2a)KV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_\x81Q\x80` \x84\x01\x85^_\x93\x01\x92\x83RP\x90\x91\x90PV[_a\x03Ua)\xDD\x83\x86a)\xB8V[\x84a)\xB8V[`\x01\x81\x81\x1C\x90\x82\x16\x80a)\xF7W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a*.W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[P\x91\x90PV[\x80T_\x90`\x01\x81\x81\x1C\x90\x82\x16\x80a*LW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a*\x83W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[\x81\x86R` \x86\x01\x81\x80\x15a*\x9EW`\x01\x81\x14a*\xD2Wa*\xFEV[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x85\x16\x82R\x83\x15\x15`\x05\x1B\x82\x01\x95Pa*\xFEV[_\x87\x81R` \x90 _[\x85\x81\x10\x15a*\xF8W\x81T\x84\x82\x01R`\x01\x90\x91\x01\x90` \x01a*\xDCV[\x83\x01\x96PP[PPPPP\x92\x91PPV[\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81T`\xE0\x1B\x16\x82R`\x80` \x83\x01R_a+J`\x80\x84\x01`\x01\x84\x01a*4V[\x83\x81\x03`@\x85\x01Ra+_\x81`\x02\x85\x01a*4V[\x90P\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x03\x84\x01T`\xE0\x1B\x16``\x85\x01R\x80\x91PP\x92\x91PPV[`\xA0\x82R_a+\xAC`\xA0\x84\x01\x83a*4V[`\x01\x83\x01T` \x85\x01R\x83\x81\x03`@\x85\x01Ra+\xCB\x81`\x02\x85\x01a*4V[\x90P`\x03\x83\x01T``\x85\x01R\x83\x81\x03`\x80\x85\x01Ra\x03U\x81`\x04\x85\x01a*4V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x81R\x83` \x82\x01R`\x80`@\x82\x01R_a, `\x80\x83\x01\x85a+\tV[\x82\x81\x03``\x84\x01Ra,2\x81\x85a+\x9AV[\x97\x96PPPPPPPV[_` \x82\x84\x03\x12\x15a,MW__\xFD[PQ\x91\x90PV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x81R\x83` \x82\x01R`\x80`@\x82\x01R\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83Q\x16`\x80\x82\x01R_` \x84\x01Q`\x80`\xA0\x84\x01Ra,\xBEa\x01\0\x84\x01\x82a%\xCBV[\x90P`@\x85\x01Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x84\x83\x03\x01`\xC0\x85\x01Ra,\xF9\x82\x82a%\xCBV[\x91PP\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x86\x01Q\x16`\xE0\x84\x01R\x82\x81\x03``\x84\x01Ra,2\x81\x85a+\x9AV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x81R\x83` \x82\x01R`\x80`@\x82\x01R_a-n`\x80\x83\x01\x85a+\tV[\x82\x81\x03``\x84\x01R\x83Q`\xA0\x82Ra-\x89`\xA0\x83\x01\x82a%\xCBV[\x90P` \x85\x01Q` \x83\x01R`@\x85\x01Q\x82\x82\x03`@\x84\x01Ra-\xAC\x82\x82a%\xCBV[\x91PP``\x85\x01Q``\x83\x01R`\x80\x85\x01Q\x82\x82\x03`\x80\x84\x01Ra-\xD0\x82\x82a%\xCBV[\x99\x98PPPPPPPPPV[\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R_a.\x0E`\x01\x83\x01\x86a)\xB8V[\x7F[\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Ra.>`\x01\x82\x01\x86a)\xB8V[\x90P\x7F].\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Ra.p`\x02\x82\x01\x85a)\xB8V[\x96\x95PPPPPPV[_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a.\xAAWa.\xAAa)KV[P`\x01\x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_\x82a.\xECWa.\xECa.\xB1V[P\x04\x90V[_\x82a.\xFFWa.\xFFa.\xB1V[P\x06\x90V[\x80\x82\x01\x80\x82\x11\x15a#\xD2Wa#\xD2a)KV[`@\x81R_a/)`@\x83\x01\x85a%\xCBV[\x82\x81\x03` \x84\x01Ra/;\x81\x85a%\xCBV[\x95\x94PPPPPV[_` \x82\x84\x03\x12\x15a/TW__\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a/jW__\xFD[\x82\x01`\x1F\x81\x01\x84\x13a/zW__\xFD[\x80Qa/\x88a%\x8A\x82a%\x1DV[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a/\x9CW__\xFD[\x81` \x84\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV\xFETx merkle proof is not valid for provided header and tx hash\0\0\0 s\xBD!\x84\xED\xD9\xC4\xFCvd.\xA6uN\xE4\x016\x97\x0E\xFC\x10\xC4\x19\0\0\0\0\0\0\0\0\0\x02\x96\xEF\x12>\xA9m\xA5\xCFi_\"\xBF}\x94\xBE\x87\xD4\x9D\xB1\xADz\xC3q\xACC\xC4\xDAAa\xC8\xC2\x164\x9C[\xA1\x19(\x17\r8x\xA2dipfsX\"\x12 \xFD\xEF6P\xB6\x97\xA7\xD9\xFF\xBA\x1AV\xB5\x97\x15\xEF\x8F\xC9\xA3K\xAF\x19\xFB}\x99\xDEWQ\xFF\xCA\x90\x84dsolcC\0\x08\x1C\x003",
    );
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                65u8, 48u8, 79u8, 172u8, 217u8, 50u8, 61u8, 117u8, 177u8, 27u8, 205u8,
                214u8, 9u8, 203u8, 56u8, 239u8, 255u8, 253u8, 176u8, 87u8, 16u8, 247u8,
                202u8, 240u8, 233u8, 177u8, 108u8, 109u8, 157u8, 112u8, 159u8, 80u8,
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                122u8, 231u8, 76u8, 82u8, 116u8, 20u8, 174u8, 19u8, 95u8, 217u8, 112u8,
                71u8, 177u8, 41u8, 33u8, 165u8, 236u8, 57u8, 17u8, 184u8, 4u8, 25u8,
                120u8, 85u8, 214u8, 126u8, 37u8, 199u8, 183u8, 94u8, 230u8, 243u8,
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                251u8, 16u8, 40u8, 101u8, 213u8, 10u8, 221u8, 221u8, 246u8, 157u8, 169u8,
                181u8, 170u8, 27u8, 206u8, 214u8, 108u8, 128u8, 207u8, 134u8, 154u8,
                92u8, 141u8, 4u8, 113u8, 164u8, 103u8, 225u8, 140u8, 233u8, 202u8, 177u8,
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                137u8, 10u8, 130u8, 103u8, 155u8, 71u8, 15u8, 43u8, 216u8, 40u8, 22u8,
                237u8, 155u8, 22u8, 31u8, 151u8, 216u8, 185u8, 103u8, 243u8, 127u8,
                163u8, 100u8, 124u8, 33u8, 213u8, 191u8, 57u8, 116u8, 158u8, 45u8, 213u8,
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                64u8, 225u8, 132u8, 15u8, 87u8, 105u8, 7u8, 61u8, 97u8, 189u8, 1u8, 55u8,
                45u8, 155u8, 117u8, 186u8, 169u8, 132u8, 45u8, 86u8, 41u8, 160u8, 201u8,
                159u8, 241u8, 3u8, 190u8, 17u8, 120u8, 168u8, 233u8, 226u8,
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                35u8, 182u8, 42u8, 208u8, 88u8, 77u8, 36u8, 167u8, 95u8, 11u8, 243u8,
                86u8, 3u8, 145u8, 239u8, 86u8, 89u8, 236u8, 109u8, 177u8, 38u8, 156u8,
                86u8, 225u8, 26u8, 162u8, 65u8, 214u8, 55u8, 241u8, 155u8, 32u8,
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                232u8, 22u8, 153u8, 184u8, 81u8, 19u8, 238u8, 161u8, 199u8, 62u8, 16u8,
                88u8, 139u8, 43u8, 3u8, 94u8, 85u8, 137u8, 51u8, 105u8, 99u8, 33u8,
                115u8, 175u8, 212u8, 63u8, 235u8, 25u8, 47u8, 172u8, 100u8, 227u8,
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                14u8, 181u8, 213u8, 38u8, 36u8, 200u8, 210u8, 138u8, 218u8, 159u8, 197u8,
                90u8, 140u8, 80u8, 46u8, 213u8, 170u8, 63u8, 190u8, 47u8, 182u8, 233u8,
                27u8, 113u8, 181u8, 243u8, 118u8, 136u8, 43u8, 29u8, 47u8, 184u8,
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                156u8, 78u8, 133u8, 65u8, 202u8, 143u8, 13u8, 193u8, 196u8, 19u8, 249u8,
                16u8, 143u8, 102u8, 216u8, 45u8, 60u8, 236u8, 177u8, 189u8, 219u8, 206u8,
                67u8, 122u8, 97u8, 202u8, 163u8, 23u8, 92u8, 76u8, 201u8, 111u8,
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                0u8, 170u8, 163u8, 156u8, 159u8, 251u8, 95u8, 86u8, 122u8, 69u8, 52u8,
                56u8, 12u8, 115u8, 112u8, 117u8, 112u8, 46u8, 31u8, 127u8, 20u8, 16u8,
                127u8, 201u8, 83u8, 40u8, 227u8, 181u8, 108u8, 3u8, 37u8, 251u8,
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                167u8, 62u8, 218u8, 9u8, 102u8, 47u8, 70u8, 221u8, 231u8, 41u8, 190u8,
                70u8, 17u8, 56u8, 95u8, 243u8, 79u8, 230u8, 196u8, 79u8, 187u8, 198u8,
                247u8, 225u8, 123u8, 4u8, 43u8, 89u8, 163u8, 68u8, 91u8, 87u8,
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                59u8, 207u8, 178u8, 174u8, 46u8, 141u8, 19u8, 45u8, 209u8, 252u8, 231u8,
                207u8, 39u8, 138u8, 154u8, 25u8, 117u8, 106u8, 159u8, 206u8, 171u8,
                228u8, 112u8, 223u8, 59u8, 218u8, 187u8, 75u8, 197u8, 119u8, 209u8, 189u8,
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                210u8, 110u8, 22u8, 202u8, 212u8, 84u8, 135u8, 5u8, 228u8, 201u8, 226u8,
                217u8, 79u8, 152u8, 238u8, 145u8, 194u8, 137u8, 8u8, 94u8, 228u8, 37u8,
                89u8, 79u8, 213u8, 99u8, 95u8, 162u8, 150u8, 76u8, 207u8, 24u8,
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                175u8, 183u8, 149u8, 201u8, 198u8, 30u8, 79u8, 231u8, 70u8, 140u8, 56u8,
                111u8, 146u8, 93u8, 122u8, 84u8, 41u8, 236u8, 173u8, 156u8, 4u8, 149u8,
                221u8, 184u8, 211u8, 141u8, 105u8, 6u8, 20u8, 211u8, 47u8, 153u8,
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                93u8, 166u8, 206u8, 157u8, 81u8, 21u8, 27u8, 161u8, 12u8, 9u8, 165u8,
                89u8, 239u8, 36u8, 213u8, 32u8, 185u8, 218u8, 197u8, 197u8, 184u8, 129u8,
                10u8, 232u8, 67u8, 78u8, 77u8, 13u8, 134u8, 65u8, 26u8, 149u8,
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                235u8, 139u8, 164u8, 60u8, 237u8, 117u8, 55u8, 66u8, 25u8, 70u8, 189u8,
                67u8, 232u8, 40u8, 184u8, 178u8, 184u8, 66u8, 137u8, 39u8, 170u8, 143u8,
                128u8, 28u8, 19u8, 217u8, 52u8, 191u8, 17u8, 172u8, 165u8, 123u8,
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                47u8, 230u8, 50u8, 119u8, 145u8, 116u8, 55u8, 67u8, 120u8, 68u8, 42u8,
                142u8, 151u8, 139u8, 204u8, 251u8, 220u8, 193u8, 214u8, 178u8, 176u8,
                216u8, 31u8, 126u8, 142u8, 183u8, 118u8, 171u8, 34u8, 134u8, 241u8, 104u8,
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                40u8, 15u8, 68u8, 70u8, 178u8, 138u8, 19u8, 114u8, 65u8, 125u8, 218u8,
                101u8, 141u8, 48u8, 185u8, 91u8, 41u8, 146u8, 177u8, 42u8, 201u8, 199u8,
                243u8, 120u8, 83u8, 95u8, 41u8, 169u8, 122u8, 207u8, 53u8, 131u8,
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                178u8, 222u8, 47u8, 190u8, 128u8, 26u8, 13u8, 246u8, 192u8, 203u8, 221u8,
                253u8, 68u8, 139u8, 163u8, 196u8, 29u8, 72u8, 160u8, 64u8, 202u8, 53u8,
                197u8, 108u8, 129u8, 150u8, 239u8, 15u8, 202u8, 231u8, 33u8, 168u8,
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                11u8, 46u8, 19u8, 255u8, 32u8, 172u8, 123u8, 71u8, 65u8, 152u8, 101u8,
                85u8, 131u8, 237u8, 247u8, 13u8, 237u8, 210u8, 193u8, 220u8, 152u8, 14u8,
                50u8, 156u8, 79u8, 187u8, 47u8, 192u8, 116u8, 139u8, 121u8, 107u8,
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                44u8, 171u8, 151u8, 144u8, 81u8, 15u8, 216u8, 189u8, 251u8, 210u8, 17u8,
                82u8, 136u8, 219u8, 51u8, 254u8, 198u8, 102u8, 145u8, 212u8, 118u8,
                239u8, 197u8, 66u8, 124u8, 253u8, 76u8, 9u8, 105u8, 48u8, 23u8, 85u8,
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
                231u8, 149u8, 14u8, 222u8, 3u8, 148u8, 185u8, 242u8, 206u8, 74u8, 90u8,
                27u8, 245u8, 167u8, 225u8, 133u8, 36u8, 17u8, 247u8, 230u8, 102u8, 27u8,
                67u8, 8u8, 201u8, 19u8, 196u8, 191u8, 209u8, 16u8, 39u8, 228u8,
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `IS_TEST()` and selector `0xfa7626d4`.
```solidity
function IS_TEST() external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct IS_TESTCall {}
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `excludeArtifacts()` and selector `0xb5508aa9`.
```solidity
function excludeArtifacts() external view returns (string[] memory excludedArtifacts_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct excludeArtifactsCall {}
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `excludeContracts()` and selector `0xe20c9f71`.
```solidity
function excludeContracts() external view returns (address[] memory excludedContracts_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct excludeContractsCall {}
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `excludeSelectors()` and selector `0xb0464fdc`.
```solidity
function excludeSelectors() external view returns (StdInvariant.FuzzSelector[] memory excludedSelectors_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct excludeSelectorsCall {}
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`excludeSelectors()`](excludeSelectorsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct excludeSelectorsReturn {
        #[allow(missing_docs)]
        pub excludedSelectors_: alloy::sol_types::private::Vec<
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
            impl ::core::convert::From<excludeSelectorsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: excludeSelectorsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for excludeSelectorsCall {
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
            impl ::core::convert::From<excludeSelectorsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: excludeSelectorsReturn) -> Self {
                    (value.excludedSelectors_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for excludeSelectorsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        excludedSelectors_: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for excludeSelectorsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = excludeSelectorsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<StdInvariant::FuzzSelector>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "excludeSelectors()";
            const SELECTOR: [u8; 4] = [176u8, 70u8, 79u8, 220u8];
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `excludeSenders()` and selector `0x1ed7831c`.
```solidity
function excludeSenders() external view returns (address[] memory excludedSenders_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct excludeSendersCall {}
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `failed()` and selector `0xba414fa6`.
```solidity
function failed() external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct failedCall {}
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `targetArtifactSelectors()` and selector `0x66d9a9a0`.
```solidity
function targetArtifactSelectors() external view returns (StdInvariant.FuzzArtifactSelector[] memory targetedArtifactSelectors_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetArtifactSelectorsCall {}
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`targetArtifactSelectors()`](targetArtifactSelectorsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetArtifactSelectorsReturn {
        #[allow(missing_docs)]
        pub targetedArtifactSelectors_: alloy::sol_types::private::Vec<
            <StdInvariant::FuzzArtifactSelector as alloy::sol_types::SolType>::RustType,
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
                alloy::sol_types::sol_data::Array<StdInvariant::FuzzArtifactSelector>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <StdInvariant::FuzzArtifactSelector as alloy::sol_types::SolType>::RustType,
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
                alloy::sol_types::sol_data::Array<StdInvariant::FuzzArtifactSelector>,
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `targetArtifacts()` and selector `0x85226c81`.
```solidity
function targetArtifacts() external view returns (string[] memory targetedArtifacts_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetArtifactsCall {}
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `targetContracts()` and selector `0x3f7286f4`.
```solidity
function targetContracts() external view returns (address[] memory targetedContracts_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetContractsCall {}
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `targetInterfaces()` and selector `0x2ade3880`.
```solidity
function targetInterfaces() external view returns (StdInvariant.FuzzInterface[] memory targetedInterfaces_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetInterfacesCall {}
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `targetSelectors()` and selector `0x916a17c6`.
```solidity
function targetSelectors() external view returns (StdInvariant.FuzzSelector[] memory targetedSelectors_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetSelectorsCall {}
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `targetSenders()` and selector `0x3e5e3c23`.
```solidity
function targetSenders() external view returns (address[] memory targetedSenders_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetSendersCall {}
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `testGCDDoesntConfirmHeader()` and selector `0xf11d5cbc`.
```solidity
function testGCDDoesntConfirmHeader() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testGCDDoesntConfirmHeaderCall {}
    ///Container type for the return parameters of the [`testGCDDoesntConfirmHeader()`](testGCDDoesntConfirmHeaderCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testGCDDoesntConfirmHeaderReturn {}
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
            impl ::core::convert::From<testGCDDoesntConfirmHeaderCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testGCDDoesntConfirmHeaderCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testGCDDoesntConfirmHeaderCall {
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
            impl ::core::convert::From<testGCDDoesntConfirmHeaderReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testGCDDoesntConfirmHeaderReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testGCDDoesntConfirmHeaderReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testGCDDoesntConfirmHeaderCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testGCDDoesntConfirmHeaderReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testGCDDoesntConfirmHeader()";
            const SELECTOR: [u8; 4] = [241u8, 29u8, 92u8, 188u8];
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `testInsufficientConfirmations()` and selector `0x39425f8f`.
```solidity
function testInsufficientConfirmations() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testInsufficientConfirmationsCall {}
    ///Container type for the return parameters of the [`testInsufficientConfirmations()`](testInsufficientConfirmationsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testInsufficientConfirmationsReturn {}
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
            impl ::core::convert::From<testInsufficientConfirmationsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testInsufficientConfirmationsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testInsufficientConfirmationsCall {
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
            impl ::core::convert::From<testInsufficientConfirmationsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testInsufficientConfirmationsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testInsufficientConfirmationsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testInsufficientConfirmationsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testInsufficientConfirmationsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testInsufficientConfirmations()";
            const SELECTOR: [u8; 4] = [57u8, 66u8, 95u8, 143u8];
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `testInvalidHeaderSupplied()` and selector `0x632ad534`.
```solidity
function testInvalidHeaderSupplied() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testInvalidHeaderSuppliedCall {}
    ///Container type for the return parameters of the [`testInvalidHeaderSupplied()`](testInvalidHeaderSuppliedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testInvalidHeaderSuppliedReturn {}
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
            impl ::core::convert::From<testInvalidHeaderSuppliedCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testInvalidHeaderSuppliedCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testInvalidHeaderSuppliedCall {
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
            impl ::core::convert::From<testInvalidHeaderSuppliedReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testInvalidHeaderSuppliedReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testInvalidHeaderSuppliedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testInvalidHeaderSuppliedCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testInvalidHeaderSuppliedReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testInvalidHeaderSupplied()";
            const SELECTOR: [u8; 4] = [99u8, 42u8, 213u8, 52u8];
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `testInvalidMerkleProofSupplied()` and selector `0xed5a9c49`.
```solidity
function testInvalidMerkleProofSupplied() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testInvalidMerkleProofSuppliedCall {}
    ///Container type for the return parameters of the [`testInvalidMerkleProofSupplied()`](testInvalidMerkleProofSuppliedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testInvalidMerkleProofSuppliedReturn {}
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
            impl ::core::convert::From<testInvalidMerkleProofSuppliedCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testInvalidMerkleProofSuppliedCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testInvalidMerkleProofSuppliedCall {
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
            impl ::core::convert::From<testInvalidMerkleProofSuppliedReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testInvalidMerkleProofSuppliedReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testInvalidMerkleProofSuppliedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testInvalidMerkleProofSuppliedCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testInvalidMerkleProofSuppliedReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testInvalidMerkleProofSupplied()";
            const SELECTOR: [u8; 4] = [237u8, 90u8, 156u8, 73u8];
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `testSuccessfullyVerify()` and selector `0xb52b2058`.
```solidity
function testSuccessfullyVerify() external view;
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
    #[derive()]
    pub enum FullRelayWithVerifyThroughBitcoinTxTestCalls {
        #[allow(missing_docs)]
        IS_TEST(IS_TESTCall),
        #[allow(missing_docs)]
        excludeArtifacts(excludeArtifactsCall),
        #[allow(missing_docs)]
        excludeContracts(excludeContractsCall),
        #[allow(missing_docs)]
        excludeSelectors(excludeSelectorsCall),
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
        testGCDDoesntConfirmHeader(testGCDDoesntConfirmHeaderCall),
        #[allow(missing_docs)]
        testIncorrectInputVectorSupplied(testIncorrectInputVectorSuppliedCall),
        #[allow(missing_docs)]
        testIncorrectLocktimeSupplied(testIncorrectLocktimeSuppliedCall),
        #[allow(missing_docs)]
        testIncorrectOutputVectorSupplied(testIncorrectOutputVectorSuppliedCall),
        #[allow(missing_docs)]
        testIncorrectProofSupplied(testIncorrectProofSuppliedCall),
        #[allow(missing_docs)]
        testInsufficientConfirmations(testInsufficientConfirmationsCall),
        #[allow(missing_docs)]
        testInvalidHeaderSupplied(testInvalidHeaderSuppliedCall),
        #[allow(missing_docs)]
        testInvalidMerkleProofSupplied(testInvalidMerkleProofSuppliedCall),
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
            [57u8, 66u8, 95u8, 143u8],
            [62u8, 94u8, 60u8, 35u8],
            [63u8, 114u8, 134u8, 244u8],
            [68u8, 186u8, 219u8, 182u8],
            [93u8, 240u8, 208u8, 118u8],
            [99u8, 42u8, 213u8, 52u8],
            [102u8, 217u8, 169u8, 160u8],
            [133u8, 34u8, 108u8, 129u8],
            [145u8, 106u8, 23u8, 198u8],
            [176u8, 70u8, 79u8, 220u8],
            [181u8, 43u8, 32u8, 88u8],
            [181u8, 80u8, 138u8, 169u8],
            [186u8, 65u8, 79u8, 166u8],
            [200u8, 153u8, 210u8, 65u8],
            [226u8, 12u8, 159u8, 113u8],
            [232u8, 159u8, 132u8, 25u8],
            [237u8, 90u8, 156u8, 73u8],
            [239u8, 62u8, 104u8, 18u8],
            [241u8, 29u8, 92u8, 188u8],
            [250u8, 118u8, 38u8, 212u8],
            [250u8, 208u8, 107u8, 143u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for FullRelayWithVerifyThroughBitcoinTxTestCalls {
        const NAME: &'static str = "FullRelayWithVerifyThroughBitcoinTxTestCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 25usize;
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
                Self::excludeSelectors(_) => {
                    <excludeSelectorsCall as alloy_sol_types::SolCall>::SELECTOR
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
                Self::testGCDDoesntConfirmHeader(_) => {
                    <testGCDDoesntConfirmHeaderCall as alloy_sol_types::SolCall>::SELECTOR
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
                Self::testInsufficientConfirmations(_) => {
                    <testInsufficientConfirmationsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testInvalidHeaderSupplied(_) => {
                    <testInvalidHeaderSuppliedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testInvalidMerkleProofSupplied(_) => {
                    <testInvalidMerkleProofSuppliedCall as alloy_sol_types::SolCall>::SELECTOR
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
                    fn testInsufficientConfirmations(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<
                        FullRelayWithVerifyThroughBitcoinTxTestCalls,
                    > {
                        <testInsufficientConfirmationsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                FullRelayWithVerifyThroughBitcoinTxTestCalls::testInsufficientConfirmations,
                            )
                    }
                    testInsufficientConfirmations
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
                    fn testInvalidHeaderSupplied(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<
                        FullRelayWithVerifyThroughBitcoinTxTestCalls,
                    > {
                        <testInvalidHeaderSuppliedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                FullRelayWithVerifyThroughBitcoinTxTestCalls::testInvalidHeaderSupplied,
                            )
                    }
                    testInvalidHeaderSupplied
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
                    fn excludeSelectors(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<
                        FullRelayWithVerifyThroughBitcoinTxTestCalls,
                    > {
                        <excludeSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                FullRelayWithVerifyThroughBitcoinTxTestCalls::excludeSelectors,
                            )
                    }
                    excludeSelectors
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
                    fn testInvalidMerkleProofSupplied(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<
                        FullRelayWithVerifyThroughBitcoinTxTestCalls,
                    > {
                        <testInvalidMerkleProofSuppliedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                FullRelayWithVerifyThroughBitcoinTxTestCalls::testInvalidMerkleProofSupplied,
                            )
                    }
                    testInvalidMerkleProofSupplied
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
                    fn testGCDDoesntConfirmHeader(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<
                        FullRelayWithVerifyThroughBitcoinTxTestCalls,
                    > {
                        <testGCDDoesntConfirmHeaderCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                FullRelayWithVerifyThroughBitcoinTxTestCalls::testGCDDoesntConfirmHeader,
                            )
                    }
                    testGCDDoesntConfirmHeader
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
                Self::excludeSelectors(inner) => {
                    <excludeSelectorsCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::testGCDDoesntConfirmHeader(inner) => {
                    <testGCDDoesntConfirmHeaderCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::testInsufficientConfirmations(inner) => {
                    <testInsufficientConfirmationsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testInvalidHeaderSupplied(inner) => {
                    <testInvalidHeaderSuppliedCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testInvalidMerkleProofSupplied(inner) => {
                    <testInvalidMerkleProofSuppliedCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::excludeSelectors(inner) => {
                    <excludeSelectorsCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::testGCDDoesntConfirmHeader(inner) => {
                    <testGCDDoesntConfirmHeaderCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::testInsufficientConfirmations(inner) => {
                    <testInsufficientConfirmationsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testInvalidHeaderSupplied(inner) => {
                    <testInvalidHeaderSuppliedCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testInvalidMerkleProofSupplied(inner) => {
                    <testInvalidMerkleProofSuppliedCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
    #[derive()]
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
                0u8, 170u8, 163u8, 156u8, 159u8, 251u8, 95u8, 86u8, 122u8, 69u8, 52u8,
                56u8, 12u8, 115u8, 112u8, 117u8, 112u8, 46u8, 31u8, 127u8, 20u8, 16u8,
                127u8, 201u8, 83u8, 40u8, 227u8, 181u8, 108u8, 3u8, 37u8, 251u8,
            ],
            [
                11u8, 46u8, 19u8, 255u8, 32u8, 172u8, 123u8, 71u8, 65u8, 152u8, 101u8,
                85u8, 131u8, 237u8, 247u8, 13u8, 237u8, 210u8, 193u8, 220u8, 152u8, 14u8,
                50u8, 156u8, 79u8, 187u8, 47u8, 192u8, 116u8, 139u8, 121u8, 107u8,
            ],
            [
                14u8, 181u8, 213u8, 38u8, 36u8, 200u8, 210u8, 138u8, 218u8, 159u8, 197u8,
                90u8, 140u8, 80u8, 46u8, 213u8, 170u8, 63u8, 190u8, 47u8, 182u8, 233u8,
                27u8, 113u8, 181u8, 243u8, 118u8, 136u8, 43u8, 29u8, 47u8, 184u8,
            ],
            [
                35u8, 182u8, 42u8, 208u8, 88u8, 77u8, 36u8, 167u8, 95u8, 11u8, 243u8,
                86u8, 3u8, 145u8, 239u8, 86u8, 89u8, 236u8, 109u8, 177u8, 38u8, 156u8,
                86u8, 225u8, 26u8, 162u8, 65u8, 214u8, 55u8, 241u8, 155u8, 32u8,
            ],
            [
                40u8, 15u8, 68u8, 70u8, 178u8, 138u8, 19u8, 114u8, 65u8, 125u8, 218u8,
                101u8, 141u8, 48u8, 185u8, 91u8, 41u8, 146u8, 177u8, 42u8, 201u8, 199u8,
                243u8, 120u8, 83u8, 95u8, 41u8, 169u8, 122u8, 207u8, 53u8, 131u8,
            ],
            [
                44u8, 171u8, 151u8, 144u8, 81u8, 15u8, 216u8, 189u8, 251u8, 210u8, 17u8,
                82u8, 136u8, 219u8, 51u8, 254u8, 198u8, 102u8, 145u8, 212u8, 118u8,
                239u8, 197u8, 66u8, 124u8, 253u8, 76u8, 9u8, 105u8, 48u8, 23u8, 85u8,
            ],
            [
                47u8, 230u8, 50u8, 119u8, 145u8, 116u8, 55u8, 67u8, 120u8, 68u8, 42u8,
                142u8, 151u8, 139u8, 204u8, 251u8, 220u8, 193u8, 214u8, 178u8, 176u8,
                216u8, 31u8, 126u8, 142u8, 183u8, 118u8, 171u8, 34u8, 134u8, 241u8, 104u8,
            ],
            [
                59u8, 207u8, 178u8, 174u8, 46u8, 141u8, 19u8, 45u8, 209u8, 252u8, 231u8,
                207u8, 39u8, 138u8, 154u8, 25u8, 117u8, 106u8, 159u8, 206u8, 171u8,
                228u8, 112u8, 223u8, 59u8, 218u8, 187u8, 75u8, 197u8, 119u8, 209u8, 189u8,
            ],
            [
                64u8, 225u8, 132u8, 15u8, 87u8, 105u8, 7u8, 61u8, 97u8, 189u8, 1u8, 55u8,
                45u8, 155u8, 117u8, 186u8, 169u8, 132u8, 45u8, 86u8, 41u8, 160u8, 201u8,
                159u8, 241u8, 3u8, 190u8, 17u8, 120u8, 168u8, 233u8, 226u8,
            ],
            [
                65u8, 48u8, 79u8, 172u8, 217u8, 50u8, 61u8, 117u8, 177u8, 27u8, 205u8,
                214u8, 9u8, 203u8, 56u8, 239u8, 255u8, 253u8, 176u8, 87u8, 16u8, 247u8,
                202u8, 240u8, 233u8, 177u8, 108u8, 109u8, 157u8, 112u8, 159u8, 80u8,
            ],
            [
                93u8, 166u8, 206u8, 157u8, 81u8, 21u8, 27u8, 161u8, 12u8, 9u8, 165u8,
                89u8, 239u8, 36u8, 213u8, 32u8, 185u8, 218u8, 197u8, 197u8, 184u8, 129u8,
                10u8, 232u8, 67u8, 78u8, 77u8, 13u8, 134u8, 65u8, 26u8, 149u8,
            ],
            [
                122u8, 231u8, 76u8, 82u8, 116u8, 20u8, 174u8, 19u8, 95u8, 217u8, 112u8,
                71u8, 177u8, 41u8, 33u8, 165u8, 236u8, 57u8, 17u8, 184u8, 4u8, 25u8,
                120u8, 85u8, 214u8, 126u8, 37u8, 199u8, 183u8, 94u8, 230u8, 243u8,
            ],
            [
                137u8, 10u8, 130u8, 103u8, 155u8, 71u8, 15u8, 43u8, 216u8, 40u8, 22u8,
                237u8, 155u8, 22u8, 31u8, 151u8, 216u8, 185u8, 103u8, 243u8, 127u8,
                163u8, 100u8, 124u8, 33u8, 213u8, 191u8, 57u8, 116u8, 158u8, 45u8, 213u8,
            ],
            [
                156u8, 78u8, 133u8, 65u8, 202u8, 143u8, 13u8, 193u8, 196u8, 19u8, 249u8,
                16u8, 143u8, 102u8, 216u8, 45u8, 60u8, 236u8, 177u8, 189u8, 219u8, 206u8,
                67u8, 122u8, 97u8, 202u8, 163u8, 23u8, 92u8, 76u8, 201u8, 111u8,
            ],
            [
                167u8, 62u8, 218u8, 9u8, 102u8, 47u8, 70u8, 221u8, 231u8, 41u8, 190u8,
                70u8, 17u8, 56u8, 95u8, 243u8, 79u8, 230u8, 196u8, 79u8, 187u8, 198u8,
                247u8, 225u8, 123u8, 4u8, 43u8, 89u8, 163u8, 68u8, 91u8, 87u8,
            ],
            [
                175u8, 183u8, 149u8, 201u8, 198u8, 30u8, 79u8, 231u8, 70u8, 140u8, 56u8,
                111u8, 146u8, 93u8, 122u8, 84u8, 41u8, 236u8, 173u8, 156u8, 4u8, 149u8,
                221u8, 184u8, 211u8, 141u8, 105u8, 6u8, 20u8, 211u8, 47u8, 153u8,
            ],
            [
                178u8, 222u8, 47u8, 190u8, 128u8, 26u8, 13u8, 246u8, 192u8, 203u8, 221u8,
                253u8, 68u8, 139u8, 163u8, 196u8, 29u8, 72u8, 160u8, 64u8, 202u8, 53u8,
                197u8, 108u8, 129u8, 150u8, 239u8, 15u8, 202u8, 231u8, 33u8, 168u8,
            ],
            [
                210u8, 110u8, 22u8, 202u8, 212u8, 84u8, 135u8, 5u8, 228u8, 201u8, 226u8,
                217u8, 79u8, 152u8, 238u8, 145u8, 194u8, 137u8, 8u8, 94u8, 228u8, 37u8,
                89u8, 79u8, 213u8, 99u8, 95u8, 162u8, 150u8, 76u8, 207u8, 24u8,
            ],
            [
                231u8, 149u8, 14u8, 222u8, 3u8, 148u8, 185u8, 242u8, 206u8, 74u8, 90u8,
                27u8, 245u8, 167u8, 225u8, 133u8, 36u8, 17u8, 247u8, 230u8, 102u8, 27u8,
                67u8, 8u8, 201u8, 19u8, 196u8, 191u8, 209u8, 16u8, 39u8, 228u8,
            ],
            [
                232u8, 22u8, 153u8, 184u8, 81u8, 19u8, 238u8, 161u8, 199u8, 62u8, 16u8,
                88u8, 139u8, 43u8, 3u8, 94u8, 85u8, 137u8, 51u8, 105u8, 99u8, 33u8,
                115u8, 175u8, 212u8, 63u8, 235u8, 25u8, 47u8, 172u8, 100u8, 227u8,
            ],
            [
                235u8, 139u8, 164u8, 60u8, 237u8, 117u8, 55u8, 66u8, 25u8, 70u8, 189u8,
                67u8, 232u8, 40u8, 184u8, 178u8, 184u8, 66u8, 137u8, 39u8, 170u8, 143u8,
                128u8, 28u8, 19u8, 217u8, 52u8, 191u8, 17u8, 172u8, 165u8, 123u8,
            ],
            [
                251u8, 16u8, 40u8, 101u8, 213u8, 10u8, 221u8, 221u8, 246u8, 157u8, 169u8,
                181u8, 170u8, 27u8, 206u8, 214u8, 108u8, 128u8, 207u8, 134u8, 154u8,
                92u8, 141u8, 4u8, 113u8, 164u8, 103u8, 225u8, 140u8, 233u8, 202u8, 177u8,
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
        ///Creates a new call builder for the [`excludeSelectors`] function.
        pub fn excludeSelectors(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, excludeSelectorsCall, N> {
            self.call_builder(&excludeSelectorsCall {})
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
        ///Creates a new call builder for the [`testGCDDoesntConfirmHeader`] function.
        pub fn testGCDDoesntConfirmHeader(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, testGCDDoesntConfirmHeaderCall, N> {
            self.call_builder(&testGCDDoesntConfirmHeaderCall {})
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
        ///Creates a new call builder for the [`testInsufficientConfirmations`] function.
        pub fn testInsufficientConfirmations(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            testInsufficientConfirmationsCall,
            N,
        > {
            self.call_builder(
                &testInsufficientConfirmationsCall {
                },
            )
        }
        ///Creates a new call builder for the [`testInvalidHeaderSupplied`] function.
        pub fn testInvalidHeaderSupplied(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, testInvalidHeaderSuppliedCall, N> {
            self.call_builder(&testInvalidHeaderSuppliedCall {})
        }
        ///Creates a new call builder for the [`testInvalidMerkleProofSupplied`] function.
        pub fn testInvalidMerkleProofSupplied(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            testInvalidMerkleProofSuppliedCall,
            N,
        > {
            self.call_builder(
                &testInvalidMerkleProofSuppliedCall {
                },
            )
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
