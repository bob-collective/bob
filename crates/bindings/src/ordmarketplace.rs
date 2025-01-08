///Module containing a contract's types and functions.
/**

```solidity
library BitcoinTx {
    struct Info { bytes4 version; bytes inputVector; bytes outputVector; bytes4 locktime; }
    struct Proof { bytes merkleProof; uint256 txIndexInBlock; bytes bitcoinHeaders; }
    struct UTXO { bytes32 txHash; uint32 txOutputIndex; uint64 txOutputValue; }
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
    /**```solidity
struct UTXO { bytes32 txHash; uint32 txOutputIndex; uint64 txOutputValue; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UTXO {
        pub txHash: alloy::sol_types::private::FixedBytes<32>,
        pub txOutputIndex: u32,
        pub txOutputValue: u64,
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
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::Uint<32>,
            alloy::sol_types::sol_data::Uint<64>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::FixedBytes<32>,
            u32,
            u64,
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
        impl ::core::convert::From<UTXO> for UnderlyingRustTuple<'_> {
            fn from(value: UTXO) -> Self {
                (value.txHash, value.txOutputIndex, value.txOutputValue)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for UTXO {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    txHash: tuple.0,
                    txOutputIndex: tuple.1,
                    txOutputValue: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for UTXO {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for UTXO {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.txHash),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.txOutputIndex),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.txOutputValue),
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
        impl alloy_sol_types::SolType for UTXO {
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
        impl alloy_sol_types::SolStruct for UTXO {
            const NAME: &'static str = "UTXO";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "UTXO(bytes32 txHash,uint32 txOutputIndex,uint64 txOutputValue)",
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
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.txHash)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.txOutputIndex)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.txOutputValue)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for UTXO {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.txHash,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.txOutputIndex,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.txOutputValue,
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
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.txHash,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.txOutputIndex,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.txOutputValue,
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
    struct UTXO {
        bytes32 txHash;
        uint32 txOutputIndex;
        uint64 txOutputValue;
    }
}

interface OrdMarketplace {
    struct AcceptedOrdinalSellOrder {
        uint256 orderId;
        BitcoinAddress bitcoinAddress;
        address ercToken;
        uint256 ercAmount;
        address requester;
        address acceptor;
        uint256 acceptTime;
    }
    struct BitcoinAddress {
        bytes scriptPubKey;
    }
    struct OrdinalId {
        bytes32 txId;
        uint32 index;
    }
    struct OrdinalSellOrder {
        OrdinalId ordinalID;
        address sellToken;
        uint256 sellAmount;
        BitcoinTx.UTXO utxo;
        address requester;
        bool isOrderAccepted;
    }

    event acceptOrdinalSellOrderEvent(uint256 indexed id, uint256 indexed acceptId, BitcoinAddress bitcoinAddress, address ercToken, uint256 ercAmount);
    event cancelAcceptedOrdinalSellOrderEvent(uint256 id);
    event placeOrdinalSellOrderEvent(uint256 indexed orderId, OrdinalId ordinalID, address sellToken, uint256 sellAmount);
    event proofOrdinalSellOrderEvent(uint256 id);
    event withdrawOrdinalSellOrderEvent(uint256 id);

    constructor(address _relay);

    function REQUEST_EXPIRATION_SECONDS() external view returns (uint256);
    function acceptOrdinalSellOrder(uint256 id, BitcoinAddress memory bitcoinAddress) external returns (uint256);
    function acceptedOrdinalSellOrders(uint256) external view returns (uint256 orderId, BitcoinAddress memory bitcoinAddress, address ercToken, uint256 ercAmount, address requester, address acceptor, uint256 acceptTime);
    function cancelAcceptedOrdinalSellOrder(uint256 id) external;
    function getOpenAcceptedOrdinalSellOrders() external view returns (AcceptedOrdinalSellOrder[] memory, uint256[] memory);
    function getOpenOrdinalSellOrders() external view returns (OrdinalSellOrder[] memory, uint256[] memory);
    function ordinalSellOrders(uint256) external view returns (OrdinalId memory ordinalID, address sellToken, uint256 sellAmount, BitcoinTx.UTXO memory utxo, address requester, bool isOrderAccepted);
    function placeOrdinalSellOrder(OrdinalId memory ordinalID, BitcoinTx.UTXO memory utxo, address sellToken, uint256 sellAmount) external;
    function proofOrdinalSellOrder(uint256 id, BitcoinTx.Info memory transaction, BitcoinTx.Proof memory proof) external;
    function withdrawOrdinalSellOrder(uint256 id) external;
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
    "name": "acceptOrdinalSellOrder",
    "inputs": [
      {
        "name": "id",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "bitcoinAddress",
        "type": "tuple",
        "internalType": "struct OrdMarketplace.BitcoinAddress",
        "components": [
          {
            "name": "scriptPubKey",
            "type": "bytes",
            "internalType": "bytes"
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
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "acceptedOrdinalSellOrders",
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
        "internalType": "struct OrdMarketplace.BitcoinAddress",
        "components": [
          {
            "name": "scriptPubKey",
            "type": "bytes",
            "internalType": "bytes"
          }
        ]
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
        "name": "acceptor",
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
    "name": "cancelAcceptedOrdinalSellOrder",
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
    "name": "getOpenAcceptedOrdinalSellOrders",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "tuple[]",
        "internalType": "struct OrdMarketplace.AcceptedOrdinalSellOrder[]",
        "components": [
          {
            "name": "orderId",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "bitcoinAddress",
            "type": "tuple",
            "internalType": "struct OrdMarketplace.BitcoinAddress",
            "components": [
              {
                "name": "scriptPubKey",
                "type": "bytes",
                "internalType": "bytes"
              }
            ]
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
            "name": "acceptor",
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
    "name": "getOpenOrdinalSellOrders",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "tuple[]",
        "internalType": "struct OrdMarketplace.OrdinalSellOrder[]",
        "components": [
          {
            "name": "ordinalID",
            "type": "tuple",
            "internalType": "struct OrdMarketplace.OrdinalId",
            "components": [
              {
                "name": "txId",
                "type": "bytes32",
                "internalType": "bytes32"
              },
              {
                "name": "index",
                "type": "uint32",
                "internalType": "uint32"
              }
            ]
          },
          {
            "name": "sellToken",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "sellAmount",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "utxo",
            "type": "tuple",
            "internalType": "struct BitcoinTx.UTXO",
            "components": [
              {
                "name": "txHash",
                "type": "bytes32",
                "internalType": "bytes32"
              },
              {
                "name": "txOutputIndex",
                "type": "uint32",
                "internalType": "uint32"
              },
              {
                "name": "txOutputValue",
                "type": "uint64",
                "internalType": "uint64"
              }
            ]
          },
          {
            "name": "requester",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "isOrderAccepted",
            "type": "bool",
            "internalType": "bool"
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
    "name": "ordinalSellOrders",
    "inputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "ordinalID",
        "type": "tuple",
        "internalType": "struct OrdMarketplace.OrdinalId",
        "components": [
          {
            "name": "txId",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "index",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
      },
      {
        "name": "sellToken",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "sellAmount",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "utxo",
        "type": "tuple",
        "internalType": "struct BitcoinTx.UTXO",
        "components": [
          {
            "name": "txHash",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "txOutputIndex",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "txOutputValue",
            "type": "uint64",
            "internalType": "uint64"
          }
        ]
      },
      {
        "name": "requester",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "isOrderAccepted",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "placeOrdinalSellOrder",
    "inputs": [
      {
        "name": "ordinalID",
        "type": "tuple",
        "internalType": "struct OrdMarketplace.OrdinalId",
        "components": [
          {
            "name": "txId",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "index",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
      },
      {
        "name": "utxo",
        "type": "tuple",
        "internalType": "struct BitcoinTx.UTXO",
        "components": [
          {
            "name": "txHash",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "txOutputIndex",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "txOutputValue",
            "type": "uint64",
            "internalType": "uint64"
          }
        ]
      },
      {
        "name": "sellToken",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "sellAmount",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "proofOrdinalSellOrder",
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
    "name": "withdrawOrdinalSellOrder",
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
    "name": "acceptOrdinalSellOrderEvent",
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
        "internalType": "struct OrdMarketplace.BitcoinAddress",
        "components": [
          {
            "name": "scriptPubKey",
            "type": "bytes",
            "internalType": "bytes"
          }
        ]
      },
      {
        "name": "ercToken",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "ercAmount",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "cancelAcceptedOrdinalSellOrderEvent",
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
    "name": "placeOrdinalSellOrderEvent",
    "inputs": [
      {
        "name": "orderId",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "ordinalID",
        "type": "tuple",
        "indexed": false,
        "internalType": "struct OrdMarketplace.OrdinalId",
        "components": [
          {
            "name": "txId",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "index",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
      },
      {
        "name": "sellToken",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "sellAmount",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "proofOrdinalSellOrderEvent",
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
    "name": "withdrawOrdinalSellOrderEvent",
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
pub mod OrdMarketplace {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x6080604052348015600e575f5ffd5b50604051613baa380380613baa833981016040819052602b916050565b600380546001600160a01b0319166001600160a01b038316179055506001600455607b565b5f60208284031215605f575f5ffd5b81516001600160a01b03811681146074575f5ffd5b9392505050565b613b22806100885f395ff3fe608060405234801561000f575f5ffd5b50600436106100b9575f3560e01c80637378715511610072578063db82d5fa11610058578063db82d5fa14610209578063e4ae61dd1461022f578063e8532be314610242575f5ffd5b806373787155146101ed578063d1920ff014610200575f5ffd5b80632b260fa0116100a25780632b260fa0146100fd5780633c49febe146101c25780635c9ddc84146101d8575f5ffd5b8063171abce5146100bd5780632814a1cd146100dc575b5f5ffd5b6100c5610255565b6040516100d3929190612cc7565b60405180910390f35b6100ef6100ea366004612dae565b6104cf565b6040519081526020016100d3565b6101b061010b366004612df8565b5f60208181529181526040908190208151808301835281548152600182015463ffffffff908116828601526002830154600384015485516060810187526004860154815260058601549384169781019790975264010000000090920467ffffffffffffffff169486019490945260069092015490936001600160a01b039384169390919081169074010000000000000000000000000000000000000000900460ff1686565b6040516100d396959493929190612e0f565b6101ca61072a565b6040516100d3929190612ed3565b6101eb6101e6366004612fde565b610982565b005b6101eb6101fb366004612df8565b610c27565b6100ef61546081565b61021c610217366004612df8565b610dbe565b6040516100d3979695949392919061303b565b6101eb61023d366004612df8565b610ea0565b6101eb610250366004613088565b611077565b6060805f805b60025481101561029a575f818152602081905260409020600601546001600160a01b031615610292578161028e81613126565b9250505b60010161025b565b505f8167ffffffffffffffff8111156102b5576102b561313e565b60405190808252806020026020018201604052801561033957816020015b60408051610100810182525f60c0820181815260e0830182905282526020808301829052828401829052835160608082018652838252818301849052948101839052938301939093526080820181905260a082015282525f199092019101816102d35790505b5090505f8267ffffffffffffffff8111156103565761035661313e565b60405190808252806020026020018201604052801561037f578160200160208202803683370190505b5090505f805b6002548110156104c3575f818152602081905260409020600601546001600160a01b0316156104bb575f8181526020818152604091829020825161010081018452815460c08201908152600183015463ffffffff90811660e084015290825260028301546001600160a01b03908116838601526003840154838701528551606080820188526004860154825260058601549384169682019690965264010000000090920467ffffffffffffffff1695820195909552928101929092526006015491821660808201527401000000000000000000000000000000000000000090910460ff16151560a082015284518590849081106104845761048461316b565b6020026020010181905250808383815181106104a2576104a261316b565b6020908102919091010152816104b781613126565b9250505b600101610385565b50919590945092505050565b5f828152602081905260408120600681015474010000000000000000000000000000000000000000900460ff161561054e5760405162461bcd60e51b815260206004820152601660248201527f4f7264657220416c72656164792041636365707465640000000000000000000060448201526064015b60405180910390fd5b60038101546002820154610571916001600160a01b03909116903390309061139e565b600280545f918261058183613126565b9190505590506040518060e00160405280868152602001856105a29061324c565b815260028401546001600160a01b039081166020808401919091526003860154604080850191909152600687015490921660608401523360808401524260a0909301929092525f8481526001808452919020835181559183015180519091830190819061060f9082613343565b505050604082810151600280840180546001600160a01b039384167fffffffffffffffffffffffff0000000000000000000000000000000000000000918216179091556060860151600380870191909155608087015160048701805491861691841691909117905560a0870151600587018054918616919093161790915560c09095015160069485015592860180547fffffffffffffffffffffff00ffffffffffffffffffffffffffffffffffffffff167401000000000000000000000000000000000000000017905591850154928501549051849389937ffe350ff9ccadd1b7c26b5f96dd078d08a877c8f37d506931ecd8f2bdbd51b6f293610718938b9390921691613427565b60405180910390a39150505b92915050565b6060805f805b600254811015610766575f818152600160205260409020600301541561075e578161075a81613126565b9250505b600101610730565b505f8167ffffffffffffffff8111156107815761078161313e565b6040519080825280602002602001820160405280156107ba57816020015b6107a7612bec565b81526020019060019003908161079f5790505b5090505f8267ffffffffffffffff8111156107d7576107d761313e565b604051908082528060200260200182016040528015610800578160200160208202803683370190505b5090505f805b6002548110156104c3575f818152600160205260409020600301541561097a5760015f8281526020019081526020015f206040518060e00160405290815f8201548152602001600182016040518060200160405290815f8201805461086a906132ad565b80601f0160208091040260200160405190810160405280929190818152602001828054610896906132ad565b80156108e15780601f106108b8576101008083540402835291602001916108e1565b820191905f5260205f20905b8154815290600101906020018083116108c457829003601f168201915b50505091909252505050815260028201546001600160a01b03908116602083015260038301546040830152600483015481166060830152600583015416608082015260069091015460a09091015284518590849081106109435761094361316b565b6020026020010181905250808383815181106109615761096161316b565b60209081029190910101528161097681613126565b9250505b600101610806565b6001600160a01b0382166109d85760405162461bcd60e51b815260206004820152601460248201527f496e76616c696420627579696e6720746f6b656e0000000000000000000000006044820152606401610545565b5f8111610a4d5760405162461bcd60e51b815260206004820152602660248201527f427579696e6720616d6f756e742073686f756c6420626520677265617465722060448201527f7468616e203000000000000000000000000000000000000000000000000000006064820152608401610545565b600280545f9182610a5d83613126565b9190505590506040518060c0016040528086803603810190610a7f91906134d1565b81526001600160a01b038516602082015260408101849052606001610aa936879003870187613525565b8152336020808301919091525f604092830181905284815280825282902083518051825582015160018201805463ffffffff92831663ffffffff19909116179055848301516002830180546001600160a01b039283167fffffffffffffffffffffffff0000000000000000000000000000000000000000909116179055858501516003840155606086015180516004850155938401516005840180549587015167ffffffffffffffff16640100000000027fffffffffffffffffffffffffffffffffffffffff000000000000000000000000909616919093161793909317905560808401516006909101805460a090950151151574010000000000000000000000000000000000000000027fffffffffffffffffffffff0000000000000000000000000000000000000000009095169190921617929092179091555181907ffb2d3310e3e79578ac507cdbdb32e52581dbc17be04e5197d3b7c522735fb9e490610c189088908790879061357a565b60405180910390a25050505050565b5f8181526001602052604090206006810154610c4690615460906135b3565b4211610c945760405162461bcd60e51b815260206004820152601360248201527f52657175657374207374696c6c2076616c6964000000000000000000000000006044820152606401610545565b60058101546001600160a01b03163314610cf05760405162461bcd60e51b815260206004820152601760248201527f53656e646572206e6f7420746865206163636570746f720000000000000000006044820152606401610545565b60038101546002820154610d11916001600160a01b03909116903390611455565b5f82815260016020819052604082208281559190820181610d328282612c3b565b5050506002810180547fffffffffffffffffffffffff00000000000000000000000000000000000000009081169091555f60038301819055600483018054831690556005830180549092169091556006909101556040518281527f9c216a4617d6c03dc7cbd9632166f1c5c9ef41f9ee86bf3b83f671c005107704906020015b60405180910390a15050565b600160208181525f92835260409283902080548451928301909452918201805482908290610deb906132ad565b80601f0160208091040260200160405190810160405280929190818152602001828054610e17906132ad565b8015610e625780601f10610e3957610100808354040283529160200191610e62565b820191905f5260205f20905b815481529060010190602001808311610e4557829003601f168201915b50505091909252505050600282015460038301546004840154600585015460069095015493946001600160a01b039384169492939182169291169087565b5f81815260208190526040902060068101546001600160a01b03163314610f095760405162461bcd60e51b815260206004820152601860248201527f53656e646572206e6f74207468652072657175657374657200000000000000006044820152606401610545565b600681015474010000000000000000000000000000000000000000900460ff1615610f9c5760405162461bcd60e51b815260206004820152603060248201527f4f726465722068617320616c7265616479206265656e2061636365707465642c60448201527f2063616e6e6f74207769746864726177000000000000000000000000000000006064820152608401610545565b5f8281526020818152604080832083815560018101805463ffffffff191690556002810180547fffffffffffffffffffffffff00000000000000000000000000000000000000001690556003810184905560048101939093556005830180547fffffffffffffffffffffffffffffffffffffffff000000000000000000000000169055600690920180547fffffffffffffffffffffff00000000000000000000000000000000000000000016905590518381527fb35b3fe4daaf6cc66eb8bd413e9ab54449e766f6d46125cc58f255694a0e847e9101610db2565b5f83815260016020526040902060048101546001600160a01b031633146110e05760405162461bcd60e51b815260206004820152601860248201527f53656e646572206e6f74207468652072657175657374657200000000000000006044820152606401610545565b80545f908152602081905260409081902060035490916001600160a01b039091169063d38c29a190611114908601866135c6565b6040518363ffffffff1660e01b8152600401611131929190613627565b5f604051808303815f87803b158015611148575f5ffd5b505af115801561115a573d5f5f3e3d5ffd5b5050505061118b6004548561116e90613669565b61117786613717565b6003546001600160a01b03169291906114a3565b5061121161119c60208601866135c6565b8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f92019190915250506040805160608101825260048701548152600587015463ffffffff81166020830152640100000000900467ffffffffffffffff1691810191909152915061163e9050565b61121e826001018561183f565b600482015460038301546002840154611245926001600160a01b0391821692911690611455565b81545f908152602081815260408083208381556001808201805463ffffffff191690556002820180547fffffffffffffffffffffffff000000000000000000000000000000000000000016905560038201859055600482018590556005820180547fffffffffffffffffffffffffffffffffffffffff000000000000000000000000169055600690910180547fffffffffffffffffffffff00000000000000000000000000000000000000000016905588845291829052822082815591908201816113108282612c3b565b5050506002810180547fffffffffffffffffffffffff00000000000000000000000000000000000000009081169091555f60038301819055600483018054831690556005830180549092169091556006909101556040518581527fc577309acd7939cc2f01f67f073e1a548224454cdddc79b161db17b5315e9f0c9060200160405180910390a15050505050565b6040516001600160a01b038085166024830152831660448201526064810182905261144f9085907f23b872dd00000000000000000000000000000000000000000000000000000000906084015b60408051601f198184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167fffffffff00000000000000000000000000000000000000000000000000000000909316929092179091526118c6565b50505050565b6040516001600160a01b03831660248201526044810182905261149e9084907fa9059cbb00000000000000000000000000000000000000000000000000000000906064016113eb565b505050565b5f6114b183602001516119aa565b6114fd5760405162461bcd60e51b815260206004820152601d60248201527f496e76616c696420696e70757420766563746f722070726f76696465640000006044820152606401610545565b61150a8360400151611a44565b6115565760405162461bcd60e51b815260206004820152601e60248201527f496e76616c6964206f757470757420766563746f722070726f766964656400006044820152606401610545565b611593835f015184602001518560400151866060015160405160200161157f94939291906137a6565b604051602081830303815290604052611ad1565b90506115b56115a58360400151611af3565b8351602085015184929190611aff565b6116275760405162461bcd60e51b815260206004820152603c60248201527f5478206d65726b6c652070726f6f66206973206e6f742076616c696420666f7260448201527f2070726f76696465642068656164657220616e642074782068617368000000006064820152608401610545565b61163685858460400151611b3a565b949350505050565b5f5f61164984611e8a565b9092509050600182016116c45760405162461bcd60e51b815260206004820152602260248201527f52656164206f76657272756e20647572696e6720566172496e7420706172736960448201527f6e670000000000000000000000000000000000000000000000000000000000006064820152608401610545565b5f806116d18460016135b3565b90505f6116e0865f0151611e9f565b90505f5b848110156117d0575f6116f78985611f4c565b90505f61172d6117078b87611f61565b60d881901c63ff00ff001662ff00ff60e89290921c9190911617601081811b91901c1790565b9050818414801561174d57508063ffffffff16896020015163ffffffff16145b1561175e5750505050505050505050565b6117688a86611f77565b95505f1986036117ba5760405162461bcd60e51b815260206004820152601760248201527f42616420566172496e7420696e207363726970745369670000000000000000006044820152606401610545565b6117c486866135b3565b945050506001016116e4565b5060405162461bcd60e51b815260206004820152602c60248201527f5472616e73616374696f6e20646f6573206e6f74207370656e6420746865207260448201527f65717569726564207574786f00000000000000000000000000000000000000006064820152608401610545565b5f825f01805461184e906132ad565b60405161186092508590602001613815565b60405160208183030381529060405280519060200120905061144f82806040019061188b91906135c6565b8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f92019190915250859250611fbd915050565b5f61191a826040518060400160405280602081526020017f5361666545524332303a206c6f772d6c6576656c2063616c6c206661696c6564815250856001600160a01b031661215b9092919063ffffffff16565b80519091501561149e578080602001905181019061193891906138dc565b61149e5760405162461bcd60e51b815260206004820152602a60248201527f5361666545524332303a204552433230206f7065726174696f6e20646964206e60448201527f6f742073756363656564000000000000000000000000000000000000000000006064820152608401610545565b5f5f5f6119b684611e8a565b90925090508015806119c857505f1982145b156119d657505f9392505050565b5f6119e28360016135b3565b90505f5b82811015611a375785518210611a0157505f95945050505050565b5f611a0c8784611f77565b90505f198103611a2257505f9695505050505050565b611a2c81846135b3565b9250506001016119e6565b5093519093149392505050565b5f5f5f611a5084611e8a565b9092509050801580611a6257505f1982145b15611a7057505f9392505050565b5f611a7c8360016135b3565b90505f5b82811015611a375785518210611a9b57505f95945050505050565b5f611aa68784612169565b90505f198103611abc57505f9695505050505050565b611ac681846135b3565b925050600101611a80565b5f60205f83516020850160025afa5060205f60205f60025afa50505f51919050565b60448101515f90610724565b5f8385148015611b0d575081155b8015611b1857508251155b15611b2557506001611636565b611b31858486856121c9565b95945050505050565b5f836001600160a01b031663113764be6040518163ffffffff1660e01b8152600401602060405180830381865afa158015611b77573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611b9b91906138fb565b90505f846001600160a01b0316632b97be246040518163ffffffff1660e01b8152600401602060405180830381865afa158015611bda573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611bfe91906138fb565b90505f80611c13611c0e8661226e565b612279565b9050838103611c2457839150611ca1565b828103611c3357829150611ca1565b60405162461bcd60e51b815260206004820152602560248201527f4e6f742061742063757272656e74206f722070726576696f757320646966666960448201527f63756c74790000000000000000000000000000000000000000000000000000006064820152608401610545565b5f611cab866122a0565b90505f198103611d235760405162461bcd60e51b815260206004820152602360248201527f496e76616c6964206c656e677468206f6620746865206865616465727320636860448201527f61696e00000000000000000000000000000000000000000000000000000000006064820152608401610545565b7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe8103611d925760405162461bcd60e51b815260206004820152601560248201527f496e76616c6964206865616465727320636861696e00000000000000000000006044820152606401610545565b7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffd8103611e015760405162461bcd60e51b815260206004820152601d60248201527f496e73756666696369656e7420776f726b20696e2061206865616465720000006044820152606401610545565b611e0b8784613912565b811015611e805760405162461bcd60e51b815260206004820152603360248201527f496e73756666696369656e7420616363756d756c61746564206469666669637560448201527f6c747920696e2068656164657220636861696e000000000000000000000000006064820152608401610545565b5050505050505050565b5f5f611e96835f6124c4565b91509150915091565b6040805160208082528183019092525f918291906020820181803683370190505090505f5b6020811015611f4157838160208110611edf57611edf61316b565b1a60f81b826001611ef1846020613929565b611efb9190613929565b81518110611f0b57611f0b61316b565b60200101907effffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff191690815f1a905350600101611ec4565b506020015192915050565b5f611f5a8383016020015190565b9392505050565b5f611f5a611f708360206135b3565b8490611f4c565b5f5f5f611f84858561263b565b909250905060018201611f9c575f1992505050610724565b80611fa88360256135b3565b611fb291906135b3565b611b319060046135b3565b604080516060810182525f8082526020808301829052828401829052835180850190945281845283015290611ff184611e8a565b60208301528082528161200382613126565b9052505f805b82602001518110156121055782515f90612024908890612169565b84519091505f90612036908990612679565b90505f612044600884613929565b86519091505f906120569060086135b3565b8a8101602001839020909150808a03612090576001965083895f0181815161207e919061393c565b67ffffffffffffffff169052506120e0565b5f61209e8c8a5f01516126ef565b90506001600160a01b038116156120bf576001600160a01b03811660208b01525b5f6120cd8d8b5f01516127cf565b905080156120dd5760408b018190525b50505b84885f018181516120f191906135b3565b905250506001909401935061200992505050565b50806121535760405162461bcd60e51b815260206004820181905260248201527f4e6f206f757470757420666f756e6420666f72207363726970745075624b65796044820152606401610545565b505092915050565b606061163684845f856128af565b5f6121758260096135b3565b8351101561218557505f19610724565b5f8061219b856121968660086135b3565b6124c4565b9092509050600182016121b3575f1992505050610724565b806121bf8360096135b3565b611b3191906135b3565b5f602084516121d89190613989565b156121e457505f611636565b83515f036121f357505f611636565b81855f5b86518110156122615761220b600284613989565b60010361222f576122286122228883016020015190565b836129f3565b9150612248565b612245826122408984016020015190565b6129f3565b91505b60019290921c9161225a6020826135b3565b90506121f7565b5090931495945050505050565b5f610724825f6129fe565b5f6107247bffff000000000000000000000000000000000000000000000000000083612a97565b5f605082516122af9190613989565b156122bc57505f19919050565b505f80805b83518110156124bd578015612308576122db848284612aa2565b61230857507ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe9392505050565b5f61231385836129fe565b905061232185836050612acb565b925080612464845f8190506008817eff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff16901b600882901c7eff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff161790506010817dffff0000ffff0000ffff0000ffff0000ffff0000ffff0000ffff0000ffff16901b601082901c7dffff0000ffff0000ffff0000ffff0000ffff0000ffff0000ffff0000ffff161790506020817bffffffff00000000ffffffff00000000ffffffff00000000ffffffff16901b602082901c7bffffffff00000000ffffffff00000000ffffffff00000000ffffffff1617905060408177ffffffffffffffff0000000000000000ffffffffffffffff16901b604082901c77ffffffffffffffff0000000000000000ffffffffffffffff16179050608081901b608082901c179050919050565b111561249457507ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffd949350505050565b61249d81612279565b6124a790856135b3565b93506124b690506050826135b3565b90506122c1565b5050919050565b5f5f5f6124d18585612af0565b90508060ff165f03612504575f8585815181106124f0576124f061316b565b016020015190935060f81c91506126349050565b8361251082600161399c565b60ff1661251d91906135b3565b85511015612532575f195f9250925050612634565b5f8160ff166002036125755761256a61255661254f8760016135b3565b8890611f4c565b62ffff0060e882901c1660f89190911c1790565b61ffff16905061262a565b8160ff1660040361259e5761259161170761254f8760016135b3565b63ffffffff16905061262a565b8160ff1660080361262a5761261d6125ba61254f8760016135b3565b60c01c64ff000000ff600882811c91821665ff000000ff009390911b92831617601090811b67ffffffffffffffff1666ff00ff00ff00ff9290921667ff00ff00ff00ff009093169290921790911c65ffff0000ffff1617602081811c91901b1790565b67ffffffffffffffff1690505b60ff909116925090505b9250929050565b5f806126488360256135b3565b8451101561265b57505f1990505f612634565b5f8061266c866121968760246135b3565b9097909650945050505050565b5f806126858484611f4c565b60c01c90505f611b318264ff000000ff600882811c91821665ff000000ff009390911b92831617601090811b67ffffffffffffffff1666ff00ff00ff00ff9290921667ff00ff00ff00ff009093169290921790911c65ffff0000ffff1617602081811c91901b1790565b5f826126fc8360096135b3565b8151811061270c5761270c61316b565b6020910101517fff00000000000000000000000000000000000000000000000000000000000000167f6a000000000000000000000000000000000000000000000000000000000000001461276157505f610724565b5f8361276e84600a6135b3565b8151811061277e5761277e61316b565b01602001517fff000000000000000000000000000000000000000000000000000000000000008116915060f81c6014036127c8575f6127be84600b6135b3565b8501601401519250505b5092915050565b5f826127dc8360096135b3565b815181106127ec576127ec61316b565b6020910101517fff00000000000000000000000000000000000000000000000000000000000000167f6a000000000000000000000000000000000000000000000000000000000000001461284157505f610724565b5f8361284e84600a6135b3565b8151811061285e5761285e61316b565b016020908101517fff000000000000000000000000000000000000000000000000000000000000008116925060f81c90036127c8575f61289f84600b6135b3565b8501602001519250505092915050565b6060824710156129275760405162461bcd60e51b815260206004820152602660248201527f416464726573733a20696e73756666696369656e742062616c616e636520666f60448201527f722063616c6c00000000000000000000000000000000000000000000000000006064820152608401610545565b6001600160a01b0385163b61297e5760405162461bcd60e51b815260206004820152601d60248201527f416464726573733a2063616c6c20746f206e6f6e2d636f6e74726163740000006044820152606401610545565b5f5f866001600160a01b0316858760405161299991906139b5565b5f6040518083038185875af1925050503d805f81146129d3576040519150601f19603f3d011682016040523d82523d5f602084013e6129d8565b606091505b50915091506129e8828286612b74565b979650505050505050565b5f611f5a8383612bad565b5f80612a15612a0e8460486135b3565b8590611f4c565b60e81c90505f84612a2785604b6135b3565b81518110612a3757612a3761316b565b016020015160f81c90505f612a69835f60108262ffffff16901c8261ff001660108462ffffff16901b17179050919050565b62ffffff1690505f612a7c6003846139c0565b60ff169050612a8d81610100613abc565b6129e89083613912565b5f611f5a8284613ac7565b5f80612aae8585612bd4565b9050828114612ac0575f915050611f5a565b506001949350505050565b5f60205f8385602001870160025afa5060205f60205f60025afa50505f519392505050565b5f828281518110612b0357612b0361316b565b016020015160f81c60ff03612b1a57506008610724565b828281518110612b2c57612b2c61316b565b016020015160f81c60fe03612b4357506004610724565b828281518110612b5557612b5561316b565b016020015160f81c60fd03612b6c57506002610724565b505f92915050565b60608315612b83575081611f5a565b825115612b935782518084602001fd5b8160405162461bcd60e51b81526004016105459190613ada565b5f825f528160205260205f60405f60025afa5060205f60205f60025afa50505f5192915050565b5f611f5a612be38360046135b3565b84016020015190565b6040518060e001604052805f8152602001612c136040518060200160405280606081525090565b81525f6020820181905260408201819052606082018190526080820181905260a09091015290565b508054612c47906132ad565b5f825580601f10612c56575050565b601f0160209004905f5260205f2090810190612c729190612c75565b50565b5b80821115612c89575f8155600101612c76565b5090565b5f8151808452602084019350602083015f5b82811015612cbd578151865260209586019590910190600101612c9f565b5093949350505050565b604080825283519082018190525f9060208501906060840190835b81811015612d90578351612d078482518051825260209081015163ffffffff16910152565b6001600160a01b036020820151166040850152604081015160608501526060810151612d5a60808601828051825260208082015163ffffffff169083015260409081015167ffffffffffffffff16910152565b5060808101516001600160a01b031660e085015260a0015115156101008401526020939093019261012090920191600101612ce2565b50508381036020850152612da48186612c8d565b9695505050505050565b5f5f60408385031215612dbf575f5ffd5b82359150602083013567ffffffffffffffff811115612ddc575f5ffd5b830160208186031215612ded575f5ffd5b809150509250929050565b5f60208284031215612e08575f5ffd5b5035919050565b8651815260208088015163ffffffff169082015261012081016001600160a01b0387166040830152856060830152612e6e60808301868051825260208082015163ffffffff169083015260409081015167ffffffffffffffff16910152565b6001600160a01b03841660e0830152821515610100830152979650505050505050565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b5f8151602084526116366020850182612e91565b5f604082016040835280855180835260608501915060608160051b8601019250602087015f5b82811015612fb2577fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa0878603018452815180518652602081015160e06020880152612f4760e0880182612ebf565b90506001600160a01b036040830151166040880152606082015160608801526001600160a01b0360808301511660808801526001600160a01b0360a08301511660a088015260c082015160c08801528096505050602082019150602084019350600181019050612ef9565b505050508281036020840152611b318185612c8d565b5f60608284031215612fd8575f5ffd5b50919050565b5f5f5f5f84860360e0811215612ff2575f5ffd5b6040811215612fff575f5ffd5b508493506130108660408701612fc8565b925060a08501356001600160a01b038116811461302b575f5ffd5b9396929550929360c00135925050565b87815260e060208201525f61305360e0830189612ebf565b6001600160a01b0397881660408401526060830196909652509285166080840152931660a082015260c0019190915292915050565b5f5f5f6060848603121561309a575f5ffd5b83359250602084013567ffffffffffffffff8111156130b7575f5ffd5b8401608081870312156130c8575f5ffd5b9150604084013567ffffffffffffffff8111156130e3575f5ffd5b6130ef86828701612fc8565b9150509250925092565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f5f198203613137576131376130f9565b5060010190565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b6040516060810167ffffffffffffffff811182821017156131bb576131bb61313e565b60405290565b5f82601f8301126131d0575f5ffd5b813567ffffffffffffffff8111156131ea576131ea61313e565b604051601f8201601f19908116603f0116810167ffffffffffffffff811182821017156132195761321961313e565b604052818152838201602001851015613230575f5ffd5b816020850160208301375f918101602001919091529392505050565b5f6020823603121561325c575f5ffd5b6040516020810167ffffffffffffffff8111828210171561327f5761327f61313e565b604052823567ffffffffffffffff811115613298575f5ffd5b6132a4368286016131c1565b82525092915050565b600181811c908216806132c157607f821691505b602082108103612fd8577f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b601f82111561149e57805f5260205f20601f840160051c8101602085101561331d5750805b601f840160051c820191505b8181101561333c575f8155600101613329565b5050505050565b815167ffffffffffffffff81111561335d5761335d61313e565b6133718161336b84546132ad565b846132f8565b6020601f8211600181146133a3575f831561338c5750848201515b5f19600385901b1c1916600184901b17845561333c565b5f84815260208120601f198516915b828110156133d257878501518255602094850194600190920191016133b2565b50848210156133ef57868401515f19600387901b60f8161c191681555b50505050600190811b01905550565b81835281816020850137505f602082840101525f6020601f19601f840116840101905092915050565b606081525f84357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe186360301811261345d575f5ffd5b850160208101903567ffffffffffffffff811115613479575f5ffd5b803603821315613487575f5ffd5b6020606085015261349c6080850182846133fe565b6001600160a01b0396909616602085015250505060400152919050565b803563ffffffff811681146134cc575f5ffd5b919050565b5f60408284031280156134e2575f5ffd5b506040805190810167ffffffffffffffff811182821017156135065761350661313e565b60405282358152613519602084016134b9565b60208201529392505050565b5f6060828403128015613536575f5ffd5b5061353f613198565b8235815261354f602084016134b9565b6020820152604083013567ffffffffffffffff8116811461356e575f5ffd5b60408201529392505050565b833581526080810163ffffffff613593602087016134b9565b1660208301526001600160a01b0393909316604082015260600152919050565b80820180821115610724576107246130f9565b5f5f83357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe18436030181126135f9575f5ffd5b83018035915067ffffffffffffffff821115613613575f5ffd5b602001915036819003821315612634575f5ffd5b602081525f6116366020830184866133fe565b80357fffffffff00000000000000000000000000000000000000000000000000000000811681146134cc575f5ffd5b5f60808236031215613679575f5ffd5b6040516080810167ffffffffffffffff8111828210171561369c5761369c61313e565b6040526136a88361363a565b8152602083013567ffffffffffffffff8111156136c3575f5ffd5b6136cf368286016131c1565b602083015250604083013567ffffffffffffffff8111156136ee575f5ffd5b6136fa368286016131c1565b60408301525061370c6060840161363a565b606082015292915050565b5f60608236031215613727575f5ffd5b61372f613198565b823567ffffffffffffffff811115613745575f5ffd5b613751368286016131c1565b82525060208381013590820152604083013567ffffffffffffffff811115613777575f5ffd5b613783368286016131c1565b60408301525092915050565b5f81518060208401855e5f93019283525090919050565b7fffffffff00000000000000000000000000000000000000000000000000000000851681525f6137e26137dc600484018761378f565b8561378f565b7fffffffff0000000000000000000000000000000000000000000000000000000093909316835250506004019392505050565b7fff000000000000000000000000000000000000000000000000000000000000008360f81b1681525f5f835461384a816132ad565b600182168015613861576001811461389a576138d0565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00831660018701526001821515830287010193506138d0565b865f5260205f205f5b838110156138c55781546001828a0101526001820191506020810190506138a3565b505060018287010193505b50919695505050505050565b5f602082840312156138ec575f5ffd5b81518015158114611f5a575f5ffd5b5f6020828403121561390b575f5ffd5b5051919050565b8082028115828204841417610724576107246130f9565b81810381811115610724576107246130f9565b67ffffffffffffffff8181168382160190811115610724576107246130f9565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f826139975761399761395c565b500690565b60ff8181168382160190811115610724576107246130f9565b5f611f5a828461378f565b60ff8281168282160390811115610724576107246130f9565b6001815b6001841115613a14578085048111156139f8576139f86130f9565b6001841615613a0657908102905b60019390931c9280026139dd565b935093915050565b5f82613a2a57506001610724565b81613a3657505f610724565b8160018114613a4c5760028114613a5657613a72565b6001915050610724565b60ff841115613a6757613a676130f9565b50506001821b610724565b5060208310610133831016604e8410600b8410161715613a95575081810a610724565b613aa15f1984846139d9565b805f1904821115613ab457613ab46130f9565b029392505050565b5f611f5a8383613a1c565b5f82613ad557613ad561395c565b500490565b602081525f611f5a6020830184612e9156fea26469706673582212206d8898220e0faf50c77d86acd2dcc77d7f7318d32946dda6b1f7a5b8895f451864736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15`\x0EW__\xFD[P`@Qa;\xAA8\x03\x80a;\xAA\x839\x81\x01`@\x81\x90R`+\x91`PV[`\x03\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x17\x90UP`\x01`\x04U`{V[_` \x82\x84\x03\x12\x15`_W__\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14`tW__\xFD[\x93\x92PPPV[a;\"\x80a\0\x88_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0\xB9W_5`\xE0\x1C\x80csxqU\x11a\0rW\x80c\xDB\x82\xD5\xFA\x11a\0XW\x80c\xDB\x82\xD5\xFA\x14a\x02\tW\x80c\xE4\xAEa\xDD\x14a\x02/W\x80c\xE8S+\xE3\x14a\x02BW__\xFD[\x80csxqU\x14a\x01\xEDW\x80c\xD1\x92\x0F\xF0\x14a\x02\0W__\xFD[\x80c+&\x0F\xA0\x11a\0\xA2W\x80c+&\x0F\xA0\x14a\0\xFDW\x80c<I\xFE\xBE\x14a\x01\xC2W\x80c\\\x9D\xDC\x84\x14a\x01\xD8W__\xFD[\x80c\x17\x1A\xBC\xE5\x14a\0\xBDW\x80c(\x14\xA1\xCD\x14a\0\xDCW[__\xFD[a\0\xC5a\x02UV[`@Qa\0\xD3\x92\x91\x90a,\xC7V[`@Q\x80\x91\x03\x90\xF3[a\0\xEFa\0\xEA6`\x04a-\xAEV[a\x04\xCFV[`@Q\x90\x81R` \x01a\0\xD3V[a\x01\xB0a\x01\x0B6`\x04a-\xF8V[_` \x81\x81R\x91\x81R`@\x90\x81\x90 \x81Q\x80\x83\x01\x83R\x81T\x81R`\x01\x82\x01Tc\xFF\xFF\xFF\xFF\x90\x81\x16\x82\x86\x01R`\x02\x83\x01T`\x03\x84\x01T\x85Q``\x81\x01\x87R`\x04\x86\x01T\x81R`\x05\x86\x01T\x93\x84\x16\x97\x81\x01\x97\x90\x97Rd\x01\0\0\0\0\x90\x92\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x94\x86\x01\x94\x90\x94R`\x06\x90\x92\x01T\x90\x93`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x93\x90\x91\x90\x81\x16\x90t\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16\x86V[`@Qa\0\xD3\x96\x95\x94\x93\x92\x91\x90a.\x0FV[a\x01\xCAa\x07*V[`@Qa\0\xD3\x92\x91\x90a.\xD3V[a\x01\xEBa\x01\xE66`\x04a/\xDEV[a\t\x82V[\0[a\x01\xEBa\x01\xFB6`\x04a-\xF8V[a\x0C'V[a\0\xEFaT`\x81V[a\x02\x1Ca\x02\x176`\x04a-\xF8V[a\r\xBEV[`@Qa\0\xD3\x97\x96\x95\x94\x93\x92\x91\x90a0;V[a\x01\xEBa\x02=6`\x04a-\xF8V[a\x0E\xA0V[a\x01\xEBa\x02P6`\x04a0\x88V[a\x10wV[``\x80_\x80[`\x02T\x81\x10\x15a\x02\x9AW_\x81\x81R` \x81\x90R`@\x90 `\x06\x01T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x02\x92W\x81a\x02\x8E\x81a1&V[\x92PP[`\x01\x01a\x02[V[P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x02\xB5Wa\x02\xB5a1>V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x039W\x81` \x01[`@\x80Qa\x01\0\x81\x01\x82R_`\xC0\x82\x01\x81\x81R`\xE0\x83\x01\x82\x90R\x82R` \x80\x83\x01\x82\x90R\x82\x84\x01\x82\x90R\x83Q``\x80\x82\x01\x86R\x83\x82R\x81\x83\x01\x84\x90R\x94\x81\x01\x83\x90R\x93\x83\x01\x93\x90\x93R`\x80\x82\x01\x81\x90R`\xA0\x82\x01R\x82R_\x19\x90\x92\x01\x91\x01\x81a\x02\xD3W\x90P[P\x90P_\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x03VWa\x03Va1>V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x03\x7FW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_\x80[`\x02T\x81\x10\x15a\x04\xC3W_\x81\x81R` \x81\x90R`@\x90 `\x06\x01T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x04\xBBW_\x81\x81R` \x81\x81R`@\x91\x82\x90 \x82Qa\x01\0\x81\x01\x84R\x81T`\xC0\x82\x01\x90\x81R`\x01\x83\x01Tc\xFF\xFF\xFF\xFF\x90\x81\x16`\xE0\x84\x01R\x90\x82R`\x02\x83\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83\x86\x01R`\x03\x84\x01T\x83\x87\x01R\x85Q``\x80\x82\x01\x88R`\x04\x86\x01T\x82R`\x05\x86\x01T\x93\x84\x16\x96\x82\x01\x96\x90\x96Rd\x01\0\0\0\0\x90\x92\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x95\x82\x01\x95\x90\x95R\x92\x81\x01\x92\x90\x92R`\x06\x01T\x91\x82\x16`\x80\x82\x01Rt\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x04`\xFF\x16\x15\x15`\xA0\x82\x01R\x84Q\x85\x90\x84\x90\x81\x10a\x04\x84Wa\x04\x84a1kV[` \x02` \x01\x01\x81\x90RP\x80\x83\x83\x81Q\x81\x10a\x04\xA2Wa\x04\xA2a1kV[` \x90\x81\x02\x91\x90\x91\x01\x01R\x81a\x04\xB7\x81a1&V[\x92PP[`\x01\x01a\x03\x85V[P\x91\x95\x90\x94P\x92PPPV[_\x82\x81R` \x81\x90R`@\x81 `\x06\x81\x01Tt\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16\x15a\x05NW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01R\x7FOrder Already Accepted\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\x03\x81\x01T`\x02\x82\x01Ta\x05q\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x903\x900\x90a\x13\x9EV[`\x02\x80T_\x91\x82a\x05\x81\x83a1&V[\x91\x90PU\x90P`@Q\x80`\xE0\x01`@R\x80\x86\x81R` \x01\x85a\x05\xA2\x90a2LV[\x81R`\x02\x84\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16` \x80\x84\x01\x91\x90\x91R`\x03\x86\x01T`@\x80\x85\x01\x91\x90\x91R`\x06\x87\x01T\x90\x92\x16``\x84\x01R3`\x80\x84\x01RB`\xA0\x90\x93\x01\x92\x90\x92R_\x84\x81R`\x01\x80\x84R\x91\x90 \x83Q\x81U\x91\x83\x01Q\x80Q\x90\x91\x83\x01\x90\x81\x90a\x06\x0F\x90\x82a3CV[PPP`@\x82\x81\x01Q`\x02\x80\x84\x01\x80T`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x82\x16\x17\x90\x91U``\x86\x01Q`\x03\x80\x87\x01\x91\x90\x91U`\x80\x87\x01Q`\x04\x87\x01\x80T\x91\x86\x16\x91\x84\x16\x91\x90\x91\x17\x90U`\xA0\x87\x01Q`\x05\x87\x01\x80T\x91\x86\x16\x91\x90\x93\x16\x17\x90\x91U`\xC0\x90\x95\x01Q`\x06\x94\x85\x01U\x92\x86\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16t\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90U\x91\x85\x01T\x92\x85\x01T\x90Q\x84\x93\x89\x93\x7F\xFE5\x0F\xF9\xCC\xAD\xD1\xB7\xC2k_\x96\xDD\x07\x8D\x08\xA8w\xC8\xF3}Pi1\xEC\xD8\xF2\xBD\xBDQ\xB6\xF2\x93a\x07\x18\x93\x8B\x93\x90\x92\x16\x91a4'V[`@Q\x80\x91\x03\x90\xA3\x91PP[\x92\x91PPV[``\x80_\x80[`\x02T\x81\x10\x15a\x07fW_\x81\x81R`\x01` R`@\x90 `\x03\x01T\x15a\x07^W\x81a\x07Z\x81a1&V[\x92PP[`\x01\x01a\x070V[P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07\x81Wa\x07\x81a1>V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x07\xBAW\x81` \x01[a\x07\xA7a+\xECV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x07\x9FW\x90P[P\x90P_\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07\xD7Wa\x07\xD7a1>V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x08\0W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_\x80[`\x02T\x81\x10\x15a\x04\xC3W_\x81\x81R`\x01` R`@\x90 `\x03\x01T\x15a\tzW`\x01_\x82\x81R` \x01\x90\x81R` \x01_ `@Q\x80`\xE0\x01`@R\x90\x81_\x82\x01T\x81R` \x01`\x01\x82\x01`@Q\x80` \x01`@R\x90\x81_\x82\x01\x80Ta\x08j\x90a2\xADV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x08\x96\x90a2\xADV[\x80\x15a\x08\xE1W\x80`\x1F\x10a\x08\xB8Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x08\xE1V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x08\xC4W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x90\x92RPPP\x81R`\x02\x82\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16` \x83\x01R`\x03\x83\x01T`@\x83\x01R`\x04\x83\x01T\x81\x16``\x83\x01R`\x05\x83\x01T\x16`\x80\x82\x01R`\x06\x90\x91\x01T`\xA0\x90\x91\x01R\x84Q\x85\x90\x84\x90\x81\x10a\tCWa\tCa1kV[` \x02` \x01\x01\x81\x90RP\x80\x83\x83\x81Q\x81\x10a\taWa\taa1kV[` \x90\x81\x02\x91\x90\x91\x01\x01R\x81a\tv\x81a1&V[\x92PP[`\x01\x01a\x08\x06V[`\x01`\x01`\xA0\x1B\x03\x82\x16a\t\xD8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01R\x7FInvalid buying token\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05EV[_\x81\x11a\nMW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FBuying amount should be greater `D\x82\x01R\x7Fthan 0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05EV[`\x02\x80T_\x91\x82a\n]\x83a1&V[\x91\x90PU\x90P`@Q\x80`\xC0\x01`@R\x80\x86\x806\x03\x81\x01\x90a\n\x7F\x91\x90a4\xD1V[\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16` \x82\x01R`@\x81\x01\x84\x90R``\x01a\n\xA96\x87\x90\x03\x87\x01\x87a5%V[\x81R3` \x80\x83\x01\x91\x90\x91R_`@\x92\x83\x01\x81\x90R\x84\x81R\x80\x82R\x82\x90 \x83Q\x80Q\x82U\x82\x01Q`\x01\x82\x01\x80Tc\xFF\xFF\xFF\xFF\x92\x83\x16c\xFF\xFF\xFF\xFF\x19\x90\x91\x16\x17\x90U\x84\x83\x01Q`\x02\x83\x01\x80T`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x17\x90U\x85\x85\x01Q`\x03\x84\x01U``\x86\x01Q\x80Q`\x04\x85\x01U\x93\x84\x01Q`\x05\x84\x01\x80T\x95\x87\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16d\x01\0\0\0\0\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\x90\x96\x16\x91\x90\x93\x16\x17\x93\x90\x93\x17\x90U`\x80\x84\x01Q`\x06\x90\x91\x01\x80T`\xA0\x90\x95\x01Q\x15\x15t\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x95\x16\x91\x90\x92\x16\x17\x92\x90\x92\x17\x90\x91UQ\x81\x90\x7F\xFB-3\x10\xE3\xE7\x95x\xACP|\xDB\xDB2\xE5%\x81\xDB\xC1{\xE0NQ\x97\xD3\xB7\xC5\"s_\xB9\xE4\x90a\x0C\x18\x90\x88\x90\x87\x90\x87\x90a5zV[`@Q\x80\x91\x03\x90\xA2PPPPPV[_\x81\x81R`\x01` R`@\x90 `\x06\x81\x01Ta\x0CF\x90aT`\x90a5\xB3V[B\x11a\x0C\x94W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01R\x7FRequest still valid\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05EV[`\x05\x81\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0C\xF0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FSender not the acceptor\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05EV[`\x03\x81\x01T`\x02\x82\x01Ta\r\x11\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x903\x90a\x14UV[_\x82\x81R`\x01` \x81\x90R`@\x82 \x82\x81U\x91\x90\x82\x01\x81a\r2\x82\x82a,;V[PPP`\x02\x81\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x81\x16\x90\x91U_`\x03\x83\x01\x81\x90U`\x04\x83\x01\x80T\x83\x16\x90U`\x05\x83\x01\x80T\x90\x92\x16\x90\x91U`\x06\x90\x91\x01U`@Q\x82\x81R\x7F\x9C!jF\x17\xD6\xC0=\xC7\xCB\xD9c!f\xF1\xC5\xC9\xEFA\xF9\xEE\x86\xBF;\x83\xF6q\xC0\x05\x10w\x04\x90` \x01[`@Q\x80\x91\x03\x90\xA1PPV[`\x01` \x81\x81R_\x92\x83R`@\x92\x83\x90 \x80T\x84Q\x92\x83\x01\x90\x94R\x91\x82\x01\x80T\x82\x90\x82\x90a\r\xEB\x90a2\xADV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0E\x17\x90a2\xADV[\x80\x15a\x0EbW\x80`\x1F\x10a\x0E9Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0EbV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0EEW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x90\x92RPPP`\x02\x82\x01T`\x03\x83\x01T`\x04\x84\x01T`\x05\x85\x01T`\x06\x90\x95\x01T\x93\x94`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x94\x92\x93\x91\x82\x16\x92\x91\x16\x90\x87V[_\x81\x81R` \x81\x90R`@\x90 `\x06\x81\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0F\tW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FSender not the requester\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05EV[`\x06\x81\x01Tt\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16\x15a\x0F\x9CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`0`$\x82\x01R\x7FOrder has already been accepted,`D\x82\x01R\x7F cannot withdraw\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05EV[_\x82\x81R` \x81\x81R`@\x80\x83 \x83\x81U`\x01\x81\x01\x80Tc\xFF\xFF\xFF\xFF\x19\x16\x90U`\x02\x81\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90U`\x03\x81\x01\x84\x90U`\x04\x81\x01\x93\x90\x93U`\x05\x83\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90U`\x06\x90\x92\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90U\x90Q\x83\x81R\x7F\xB3[?\xE4\xDA\xAFl\xC6n\xB8\xBDA>\x9A\xB5DI\xE7f\xF6\xD4a%\xCCX\xF2UiJ\x0E\x84~\x91\x01a\r\xB2V[_\x83\x81R`\x01` R`@\x90 `\x04\x81\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x10\xE0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FSender not the requester\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05EV[\x80T_\x90\x81R` \x81\x90R`@\x90\x81\x90 `\x03T\x90\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xD3\x8C)\xA1\x90a\x11\x14\x90\x86\x01\x86a5\xC6V[`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x111\x92\x91\x90a6'V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x11HW__\xFD[PZ\xF1\x15\x80\x15a\x11ZW=__>=_\xFD[PPPPa\x11\x8B`\x04T\x85a\x11n\x90a6iV[a\x11w\x86a7\x17V[`\x03T`\x01`\x01`\xA0\x1B\x03\x16\x92\x91\x90a\x14\xA3V[Pa\x12\x11a\x11\x9C` \x86\x01\x86a5\xC6V[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPP`@\x80Q``\x81\x01\x82R`\x04\x87\x01T\x81R`\x05\x87\x01Tc\xFF\xFF\xFF\xFF\x81\x16` \x83\x01Rd\x01\0\0\0\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91\x81\x01\x91\x90\x91R\x91Pa\x16>\x90PV[a\x12\x1E\x82`\x01\x01\x85a\x18?V[`\x04\x82\x01T`\x03\x83\x01T`\x02\x84\x01Ta\x12E\x92`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x92\x91\x16\x90a\x14UV[\x81T_\x90\x81R` \x81\x81R`@\x80\x83 \x83\x81U`\x01\x80\x82\x01\x80Tc\xFF\xFF\xFF\xFF\x19\x16\x90U`\x02\x82\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90U`\x03\x82\x01\x85\x90U`\x04\x82\x01\x85\x90U`\x05\x82\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90U`\x06\x90\x91\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90U\x88\x84R\x91\x82\x90R\x82 \x82\x81U\x91\x90\x82\x01\x81a\x13\x10\x82\x82a,;V[PPP`\x02\x81\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x81\x16\x90\x91U_`\x03\x83\x01\x81\x90U`\x04\x83\x01\x80T\x83\x16\x90U`\x05\x83\x01\x80T\x90\x92\x16\x90\x91U`\x06\x90\x91\x01U`@Q\x85\x81R\x7F\xC5w0\x9A\xCDy9\xCC/\x01\xF6\x7F\x07>\x1AT\x82$EL\xDD\xDCy\xB1a\xDB\x17\xB51^\x9F\x0C\x90` \x01`@Q\x80\x91\x03\x90\xA1PPPPPV[`@Q`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`$\x83\x01R\x83\x16`D\x82\x01R`d\x81\x01\x82\x90Ra\x14O\x90\x85\x90\x7F#\xB8r\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90`\x84\x01[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x93\x16\x92\x90\x92\x17\x90\x91Ra\x18\xC6V[PPPPV[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16`$\x82\x01R`D\x81\x01\x82\x90Ra\x14\x9E\x90\x84\x90\x7F\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90`d\x01a\x13\xEBV[PPPV[_a\x14\xB1\x83` \x01Qa\x19\xAAV[a\x14\xFDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FInvalid input vector provided\0\0\0`D\x82\x01R`d\x01a\x05EV[a\x15\n\x83`@\x01Qa\x1ADV[a\x15VW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FInvalid output vector provided\0\0`D\x82\x01R`d\x01a\x05EV[a\x15\x93\x83_\x01Q\x84` \x01Q\x85`@\x01Q\x86``\x01Q`@Q` \x01a\x15\x7F\x94\x93\x92\x91\x90a7\xA6V[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra\x1A\xD1V[\x90Pa\x15\xB5a\x15\xA5\x83`@\x01Qa\x1A\xF3V[\x83Q` \x85\x01Q\x84\x92\x91\x90a\x1A\xFFV[a\x16'W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`<`$\x82\x01R\x7FTx merkle proof is not valid for`D\x82\x01R\x7F provided header and tx hash\0\0\0\0`d\x82\x01R`\x84\x01a\x05EV[a\x166\x85\x85\x84`@\x01Qa\x1B:V[\x94\x93PPPPV[__a\x16I\x84a\x1E\x8AV[\x90\x92P\x90P`\x01\x82\x01a\x16\xC4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FRead overrun during VarInt parsi`D\x82\x01R\x7Fng\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05EV[_\x80a\x16\xD1\x84`\x01a5\xB3V[\x90P_a\x16\xE0\x86_\x01Qa\x1E\x9FV[\x90P_[\x84\x81\x10\x15a\x17\xD0W_a\x16\xF7\x89\x85a\x1FLV[\x90P_a\x17-a\x17\x07\x8B\x87a\x1FaV[`\xD8\x81\x90\x1Cc\xFF\0\xFF\0\x16b\xFF\0\xFF`\xE8\x92\x90\x92\x1C\x91\x90\x91\x16\x17`\x10\x81\x81\x1B\x91\x90\x1C\x17\x90V[\x90P\x81\x84\x14\x80\x15a\x17MWP\x80c\xFF\xFF\xFF\xFF\x16\x89` \x01Qc\xFF\xFF\xFF\xFF\x16\x14[\x15a\x17^WPPPPPPPPPPV[a\x17h\x8A\x86a\x1FwV[\x95P_\x19\x86\x03a\x17\xBAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FBad VarInt in scriptSig\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05EV[a\x17\xC4\x86\x86a5\xB3V[\x94PPP`\x01\x01a\x16\xE4V[P`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FTransaction does not spend the r`D\x82\x01R\x7Fequired utxo\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05EV[_\x82_\x01\x80Ta\x18N\x90a2\xADV[`@Qa\x18`\x92P\x85\x90` \x01a8\x15V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90Pa\x14O\x82\x80`@\x01\x90a\x18\x8B\x91\x90a5\xC6V[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RP\x85\x92Pa\x1F\xBD\x91PPV[_a\x19\x1A\x82`@Q\x80`@\x01`@R\x80` \x81R` \x01\x7FSafeERC20: low-level call failed\x81RP\x85`\x01`\x01`\xA0\x1B\x03\x16a![\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x80Q\x90\x91P\x15a\x14\x9EW\x80\x80` \x01\x90Q\x81\x01\x90a\x198\x91\x90a8\xDCV[a\x14\x9EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01R\x7Fot succeed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05EV[___a\x19\xB6\x84a\x1E\x8AV[\x90\x92P\x90P\x80\x15\x80a\x19\xC8WP_\x19\x82\x14[\x15a\x19\xD6WP_\x93\x92PPPV[_a\x19\xE2\x83`\x01a5\xB3V[\x90P_[\x82\x81\x10\x15a\x1A7W\x85Q\x82\x10a\x1A\x01WP_\x95\x94PPPPPV[_a\x1A\x0C\x87\x84a\x1FwV[\x90P_\x19\x81\x03a\x1A\"WP_\x96\x95PPPPPPV[a\x1A,\x81\x84a5\xB3V[\x92PP`\x01\x01a\x19\xE6V[P\x93Q\x90\x93\x14\x93\x92PPPV[___a\x1AP\x84a\x1E\x8AV[\x90\x92P\x90P\x80\x15\x80a\x1AbWP_\x19\x82\x14[\x15a\x1ApWP_\x93\x92PPPV[_a\x1A|\x83`\x01a5\xB3V[\x90P_[\x82\x81\x10\x15a\x1A7W\x85Q\x82\x10a\x1A\x9BWP_\x95\x94PPPPPV[_a\x1A\xA6\x87\x84a!iV[\x90P_\x19\x81\x03a\x1A\xBCWP_\x96\x95PPPPPPV[a\x1A\xC6\x81\x84a5\xB3V[\x92PP`\x01\x01a\x1A\x80V[_` _\x83Q` \x85\x01`\x02Z\xFAP` _` _`\x02Z\xFAPP_Q\x91\x90PV[`D\x81\x01Q_\x90a\x07$V[_\x83\x85\x14\x80\x15a\x1B\rWP\x81\x15[\x80\x15a\x1B\x18WP\x82Q\x15[\x15a\x1B%WP`\x01a\x166V[a\x1B1\x85\x84\x86\x85a!\xC9V[\x95\x94PPPPPV[_\x83`\x01`\x01`\xA0\x1B\x03\x16c\x117d\xBE`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1BwW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\x9B\x91\x90a8\xFBV[\x90P_\x84`\x01`\x01`\xA0\x1B\x03\x16c+\x97\xBE$`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B\xDAW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\xFE\x91\x90a8\xFBV[\x90P_\x80a\x1C\x13a\x1C\x0E\x86a\"nV[a\"yV[\x90P\x83\x81\x03a\x1C$W\x83\x91Pa\x1C\xA1V[\x82\x81\x03a\x1C3W\x82\x91Pa\x1C\xA1V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FNot at current or previous diffi`D\x82\x01R\x7Fculty\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05EV[_a\x1C\xAB\x86a\"\xA0V[\x90P_\x19\x81\x03a\x1D#W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FInvalid length of the headers ch`D\x82\x01R\x7Fain\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05EV[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x03a\x1D\x92W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01R\x7FInvalid headers chain\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05EV[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFD\x81\x03a\x1E\x01W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FInsufficient work in a header\0\0\0`D\x82\x01R`d\x01a\x05EV[a\x1E\x0B\x87\x84a9\x12V[\x81\x10\x15a\x1E\x80W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`3`$\x82\x01R\x7FInsufficient accumulated difficu`D\x82\x01R\x7Flty in header chain\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05EV[PPPPPPPPV[__a\x1E\x96\x83_a$\xC4V[\x91P\x91P\x91P\x91V[`@\x80Q` \x80\x82R\x81\x83\x01\x90\x92R_\x91\x82\x91\x90` \x82\x01\x81\x806\x837\x01\x90PP\x90P_[` \x81\x10\x15a\x1FAW\x83\x81` \x81\x10a\x1E\xDFWa\x1E\xDFa1kV[\x1A`\xF8\x1B\x82`\x01a\x1E\xF1\x84` a9)V[a\x1E\xFB\x91\x90a9)V[\x81Q\x81\x10a\x1F\x0BWa\x1F\x0Ba1kV[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81_\x1A\x90SP`\x01\x01a\x1E\xC4V[P` \x01Q\x92\x91PPV[_a\x1FZ\x83\x83\x01` \x01Q\x90V[\x93\x92PPPV[_a\x1FZa\x1Fp\x83` a5\xB3V[\x84\x90a\x1FLV[___a\x1F\x84\x85\x85a&;V[\x90\x92P\x90P`\x01\x82\x01a\x1F\x9CW_\x19\x92PPPa\x07$V[\x80a\x1F\xA8\x83`%a5\xB3V[a\x1F\xB2\x91\x90a5\xB3V[a\x1B1\x90`\x04a5\xB3V[`@\x80Q``\x81\x01\x82R_\x80\x82R` \x80\x83\x01\x82\x90R\x82\x84\x01\x82\x90R\x83Q\x80\x85\x01\x90\x94R\x81\x84R\x83\x01R\x90a\x1F\xF1\x84a\x1E\x8AV[` \x83\x01R\x80\x82R\x81a \x03\x82a1&V[\x90RP_\x80[\x82` \x01Q\x81\x10\x15a!\x05W\x82Q_\x90a $\x90\x88\x90a!iV[\x84Q\x90\x91P_\x90a 6\x90\x89\x90a&yV[\x90P_a D`\x08\x84a9)V[\x86Q\x90\x91P_\x90a V\x90`\x08a5\xB3V[\x8A\x81\x01` \x01\x83\x90 \x90\x91P\x80\x8A\x03a \x90W`\x01\x96P\x83\x89_\x01\x81\x81Qa ~\x91\x90a9<V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90RPa \xE0V[_a \x9E\x8C\x8A_\x01Qa&\xEFV[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a \xBFW`\x01`\x01`\xA0\x1B\x03\x81\x16` \x8B\x01R[_a \xCD\x8D\x8B_\x01Qa'\xCFV[\x90P\x80\x15a \xDDW`@\x8B\x01\x81\x90R[PP[\x84\x88_\x01\x81\x81Qa \xF1\x91\x90a5\xB3V[\x90RPP`\x01\x90\x94\x01\x93Pa \t\x92PPPV[P\x80a!SW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FNo output found for scriptPubKey`D\x82\x01R`d\x01a\x05EV[PP\x92\x91PPV[``a\x166\x84\x84_\x85a(\xAFV[_a!u\x82`\ta5\xB3V[\x83Q\x10\x15a!\x85WP_\x19a\x07$V[_\x80a!\x9B\x85a!\x96\x86`\x08a5\xB3V[a$\xC4V[\x90\x92P\x90P`\x01\x82\x01a!\xB3W_\x19\x92PPPa\x07$V[\x80a!\xBF\x83`\ta5\xB3V[a\x1B1\x91\x90a5\xB3V[_` \x84Qa!\xD8\x91\x90a9\x89V[\x15a!\xE4WP_a\x166V[\x83Q_\x03a!\xF3WP_a\x166V[\x81\x85_[\x86Q\x81\x10\x15a\"aWa\"\x0B`\x02\x84a9\x89V[`\x01\x03a\"/Wa\"(a\"\"\x88\x83\x01` \x01Q\x90V[\x83a)\xF3V[\x91Pa\"HV[a\"E\x82a\"@\x89\x84\x01` \x01Q\x90V[a)\xF3V[\x91P[`\x01\x92\x90\x92\x1C\x91a\"Z` \x82a5\xB3V[\x90Pa!\xF7V[P\x90\x93\x14\x95\x94PPPPPV[_a\x07$\x82_a)\xFEV[_a\x07${\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83a*\x97V[_`P\x82Qa\"\xAF\x91\x90a9\x89V[\x15a\"\xBCWP_\x19\x91\x90PV[P_\x80\x80[\x83Q\x81\x10\x15a$\xBDW\x80\x15a#\x08Wa\"\xDB\x84\x82\x84a*\xA2V[a#\x08WP\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x93\x92PPPV[_a#\x13\x85\x83a)\xFEV[\x90Pa#!\x85\x83`Pa*\xCBV[\x92P\x80a$d\x84_\x81\x90P`\x08\x81~\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\x16\x90\x1B`\x08\x82\x90\x1C~\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\x16\x17\x90P`\x10\x81}\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\x16\x90\x1B`\x10\x82\x90\x1C}\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\x16\x17\x90P` \x81{\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\x16\x90\x1B` \x82\x90\x1C{\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\x16\x17\x90P`@\x81w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x1B`@\x82\x90\x1Cw\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x17\x90P`\x80\x81\x90\x1B`\x80\x82\x90\x1C\x17\x90P\x91\x90PV[\x11\x15a$\x94WP\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFD\x94\x93PPPPV[a$\x9D\x81a\"yV[a$\xA7\x90\x85a5\xB3V[\x93Pa$\xB6\x90P`P\x82a5\xB3V[\x90Pa\"\xC1V[PP\x91\x90PV[___a$\xD1\x85\x85a*\xF0V[\x90P\x80`\xFF\x16_\x03a%\x04W_\x85\x85\x81Q\x81\x10a$\xF0Wa$\xF0a1kV[\x01` \x01Q\x90\x93P`\xF8\x1C\x91Pa&4\x90PV[\x83a%\x10\x82`\x01a9\x9CV[`\xFF\x16a%\x1D\x91\x90a5\xB3V[\x85Q\x10\x15a%2W_\x19_\x92P\x92PPa&4V[_\x81`\xFF\x16`\x02\x03a%uWa%ja%Va%O\x87`\x01a5\xB3V[\x88\x90a\x1FLV[b\xFF\xFF\0`\xE8\x82\x90\x1C\x16`\xF8\x91\x90\x91\x1C\x17\x90V[a\xFF\xFF\x16\x90Pa&*V[\x81`\xFF\x16`\x04\x03a%\x9EWa%\x91a\x17\x07a%O\x87`\x01a5\xB3V[c\xFF\xFF\xFF\xFF\x16\x90Pa&*V[\x81`\xFF\x16`\x08\x03a&*Wa&\x1Da%\xBAa%O\x87`\x01a5\xB3V[`\xC0\x1Cd\xFF\0\0\0\xFF`\x08\x82\x81\x1C\x91\x82\x16e\xFF\0\0\0\xFF\0\x93\x90\x91\x1B\x92\x83\x16\x17`\x10\x90\x81\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16f\xFF\0\xFF\0\xFF\0\xFF\x92\x90\x92\x16g\xFF\0\xFF\0\xFF\0\xFF\0\x90\x93\x16\x92\x90\x92\x17\x90\x91\x1Ce\xFF\xFF\0\0\xFF\xFF\x16\x17` \x81\x81\x1C\x91\x90\x1B\x17\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P[`\xFF\x90\x91\x16\x92P\x90P[\x92P\x92\x90PV[_\x80a&H\x83`%a5\xB3V[\x84Q\x10\x15a&[WP_\x19\x90P_a&4V[_\x80a&l\x86a!\x96\x87`$a5\xB3V[\x90\x97\x90\x96P\x94PPPPPV[_\x80a&\x85\x84\x84a\x1FLV[`\xC0\x1C\x90P_a\x1B1\x82d\xFF\0\0\0\xFF`\x08\x82\x81\x1C\x91\x82\x16e\xFF\0\0\0\xFF\0\x93\x90\x91\x1B\x92\x83\x16\x17`\x10\x90\x81\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16f\xFF\0\xFF\0\xFF\0\xFF\x92\x90\x92\x16g\xFF\0\xFF\0\xFF\0\xFF\0\x90\x93\x16\x92\x90\x92\x17\x90\x91\x1Ce\xFF\xFF\0\0\xFF\xFF\x16\x17` \x81\x81\x1C\x91\x90\x1B\x17\x90V[_\x82a&\xFC\x83`\ta5\xB3V[\x81Q\x81\x10a'\x0CWa'\x0Ca1kV[` \x91\x01\x01Q\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x7Fj\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x14a'aWP_a\x07$V[_\x83a'n\x84`\na5\xB3V[\x81Q\x81\x10a'~Wa'~a1kV[\x01` \x01Q\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x91P`\xF8\x1C`\x14\x03a'\xC8W_a'\xBE\x84`\x0Ba5\xB3V[\x85\x01`\x14\x01Q\x92PP[P\x92\x91PPV[_\x82a'\xDC\x83`\ta5\xB3V[\x81Q\x81\x10a'\xECWa'\xECa1kV[` \x91\x01\x01Q\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x7Fj\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x14a(AWP_a\x07$V[_\x83a(N\x84`\na5\xB3V[\x81Q\x81\x10a(^Wa(^a1kV[\x01` \x90\x81\x01Q\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x92P`\xF8\x1C\x90\x03a'\xC8W_a(\x9F\x84`\x0Ba5\xB3V[\x85\x01` \x01Q\x92PPP\x92\x91PPV[``\x82G\x10\x15a)'W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FAddress: insufficient balance fo`D\x82\x01R\x7Fr call\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05EV[`\x01`\x01`\xA0\x1B\x03\x85\x16;a)~W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x05EV[__\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@Qa)\x99\x91\x90a9\xB5V[_`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80_\x81\x14a)\xD3W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a)\xD8V[``\x91P[P\x91P\x91Pa)\xE8\x82\x82\x86a+tV[\x97\x96PPPPPPPV[_a\x1FZ\x83\x83a+\xADV[_\x80a*\x15a*\x0E\x84`Ha5\xB3V[\x85\x90a\x1FLV[`\xE8\x1C\x90P_\x84a*'\x85`Ka5\xB3V[\x81Q\x81\x10a*7Wa*7a1kV[\x01` \x01Q`\xF8\x1C\x90P_a*i\x83_`\x10\x82b\xFF\xFF\xFF\x16\x90\x1C\x82a\xFF\0\x16`\x10\x84b\xFF\xFF\xFF\x16\x90\x1B\x17\x17\x90P\x91\x90PV[b\xFF\xFF\xFF\x16\x90P_a*|`\x03\x84a9\xC0V[`\xFF\x16\x90Pa*\x8D\x81a\x01\0a:\xBCV[a)\xE8\x90\x83a9\x12V[_a\x1FZ\x82\x84a:\xC7V[_\x80a*\xAE\x85\x85a+\xD4V[\x90P\x82\x81\x14a*\xC0W_\x91PPa\x1FZV[P`\x01\x94\x93PPPPV[_` _\x83\x85` \x01\x87\x01`\x02Z\xFAP` _` _`\x02Z\xFAPP_Q\x93\x92PPPV[_\x82\x82\x81Q\x81\x10a+\x03Wa+\x03a1kV[\x01` \x01Q`\xF8\x1C`\xFF\x03a+\x1AWP`\x08a\x07$V[\x82\x82\x81Q\x81\x10a+,Wa+,a1kV[\x01` \x01Q`\xF8\x1C`\xFE\x03a+CWP`\x04a\x07$V[\x82\x82\x81Q\x81\x10a+UWa+Ua1kV[\x01` \x01Q`\xF8\x1C`\xFD\x03a+lWP`\x02a\x07$V[P_\x92\x91PPV[``\x83\x15a+\x83WP\x81a\x1FZV[\x82Q\x15a+\x93W\x82Q\x80\x84` \x01\xFD[\x81`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05E\x91\x90a:\xDAV[_\x82_R\x81` R` _`@_`\x02Z\xFAP` _` _`\x02Z\xFAPP_Q\x92\x91PPV[_a\x1FZa+\xE3\x83`\x04a5\xB3V[\x84\x01` \x01Q\x90V[`@Q\x80`\xE0\x01`@R\x80_\x81R` \x01a,\x13`@Q\x80` \x01`@R\x80``\x81RP\x90V[\x81R_` \x82\x01\x81\x90R`@\x82\x01\x81\x90R``\x82\x01\x81\x90R`\x80\x82\x01\x81\x90R`\xA0\x90\x91\x01R\x90V[P\x80Ta,G\x90a2\xADV[_\x82U\x80`\x1F\x10a,VWPPV[`\x1F\x01` \x90\x04\x90_R` _ \x90\x81\x01\x90a,r\x91\x90a,uV[PV[[\x80\x82\x11\x15a,\x89W_\x81U`\x01\x01a,vV[P\x90V[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15a,\xBDW\x81Q\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01a,\x9FV[P\x93\x94\x93PPPPV[`@\x80\x82R\x83Q\x90\x82\x01\x81\x90R_\x90` \x85\x01\x90``\x84\x01\x90\x83[\x81\x81\x10\x15a-\x90W\x83Qa-\x07\x84\x82Q\x80Q\x82R` \x90\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x91\x01RV[`\x01`\x01`\xA0\x1B\x03` \x82\x01Q\x16`@\x85\x01R`@\x81\x01Q``\x85\x01R``\x81\x01Qa-Z`\x80\x86\x01\x82\x80Q\x82R` \x80\x82\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x83\x01R`@\x90\x81\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91\x01RV[P`\x80\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\xE0\x85\x01R`\xA0\x01Q\x15\x15a\x01\0\x84\x01R` \x93\x90\x93\x01\x92a\x01 \x90\x92\x01\x91`\x01\x01a,\xE2V[PP\x83\x81\x03` \x85\x01Ra-\xA4\x81\x86a,\x8DV[\x96\x95PPPPPPV[__`@\x83\x85\x03\x12\x15a-\xBFW__\xFD[\x825\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a-\xDCW__\xFD[\x83\x01` \x81\x86\x03\x12\x15a-\xEDW__\xFD[\x80\x91PP\x92P\x92\x90PV[_` \x82\x84\x03\x12\x15a.\x08W__\xFD[P5\x91\x90PV[\x86Q\x81R` \x80\x88\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x82\x01Ra\x01 \x81\x01`\x01`\x01`\xA0\x1B\x03\x87\x16`@\x83\x01R\x85``\x83\x01Ra.n`\x80\x83\x01\x86\x80Q\x82R` \x80\x82\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x83\x01R`@\x90\x81\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91\x01RV[`\x01`\x01`\xA0\x1B\x03\x84\x16`\xE0\x83\x01R\x82\x15\x15a\x01\0\x83\x01R\x97\x96PPPPPPPV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[_\x81Q` \x84Ra\x166` \x85\x01\x82a.\x91V[_`@\x82\x01`@\x83R\x80\x85Q\x80\x83R``\x85\x01\x91P``\x81`\x05\x1B\x86\x01\x01\x92P` \x87\x01_[\x82\x81\x10\x15a/\xB2W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA0\x87\x86\x03\x01\x84R\x81Q\x80Q\x86R` \x81\x01Q`\xE0` \x88\x01Ra/G`\xE0\x88\x01\x82a.\xBFV[\x90P`\x01`\x01`\xA0\x1B\x03`@\x83\x01Q\x16`@\x88\x01R``\x82\x01Q``\x88\x01R`\x01`\x01`\xA0\x1B\x03`\x80\x83\x01Q\x16`\x80\x88\x01R`\x01`\x01`\xA0\x1B\x03`\xA0\x83\x01Q\x16`\xA0\x88\x01R`\xC0\x82\x01Q`\xC0\x88\x01R\x80\x96PPP` \x82\x01\x91P` \x84\x01\x93P`\x01\x81\x01\x90Pa.\xF9V[PPPP\x82\x81\x03` \x84\x01Ra\x1B1\x81\x85a,\x8DV[_``\x82\x84\x03\x12\x15a/\xD8W__\xFD[P\x91\x90PV[____\x84\x86\x03`\xE0\x81\x12\x15a/\xF2W__\xFD[`@\x81\x12\x15a/\xFFW__\xFD[P\x84\x93Pa0\x10\x86`@\x87\x01a/\xC8V[\x92P`\xA0\x85\x015`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a0+W__\xFD[\x93\x96\x92\x95P\x92\x93`\xC0\x015\x92PPV[\x87\x81R`\xE0` \x82\x01R_a0S`\xE0\x83\x01\x89a.\xBFV[`\x01`\x01`\xA0\x1B\x03\x97\x88\x16`@\x84\x01R``\x83\x01\x96\x90\x96RP\x92\x85\x16`\x80\x84\x01R\x93\x16`\xA0\x82\x01R`\xC0\x01\x91\x90\x91R\x92\x91PPV[___``\x84\x86\x03\x12\x15a0\x9AW__\xFD[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a0\xB7W__\xFD[\x84\x01`\x80\x81\x87\x03\x12\x15a0\xC8W__\xFD[\x91P`@\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a0\xE3W__\xFD[a0\xEF\x86\x82\x87\x01a/\xC8V[\x91PP\x92P\x92P\x92V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[__\x19\x82\x03a17Wa17a0\xF9V[P`\x01\x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a1\xBBWa1\xBBa1>V[`@R\x90V[_\x82`\x1F\x83\x01\x12a1\xD0W__\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a1\xEAWa1\xEAa1>V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a2\x19Wa2\x19a1>V[`@R\x81\x81R\x83\x82\x01` \x01\x85\x10\x15a20W__\xFD[\x81` \x85\x01` \x83\x017_\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[_` \x826\x03\x12\x15a2\\W__\xFD[`@Q` \x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a2\x7FWa2\x7Fa1>V[`@R\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a2\x98W__\xFD[a2\xA46\x82\x86\x01a1\xC1V[\x82RP\x92\x91PPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a2\xC1W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a/\xD8W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[`\x1F\x82\x11\x15a\x14\x9EW\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a3\x1DWP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a3<W_\x81U`\x01\x01a3)V[PPPPPV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a3]Wa3]a1>V[a3q\x81a3k\x84Ta2\xADV[\x84a2\xF8V[` `\x1F\x82\x11`\x01\x81\x14a3\xA3W_\x83\x15a3\x8CWP\x84\x82\x01Q[_\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua3<V[_\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a3\xD2W\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a3\xB2V[P\x84\x82\x10\x15a3\xEFW\x86\x84\x01Q_\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[\x81\x83R\x81\x81` \x85\x017P_` \x82\x84\x01\x01R_` `\x1F\x19`\x1F\x84\x01\x16\x84\x01\x01\x90P\x92\x91PPV[``\x81R_\x845\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x866\x03\x01\x81\x12a4]W__\xFD[\x85\x01` \x81\x01\x905g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a4yW__\xFD[\x806\x03\x82\x13\x15a4\x87W__\xFD[` ``\x85\x01Ra4\x9C`\x80\x85\x01\x82\x84a3\xFEV[`\x01`\x01`\xA0\x1B\x03\x96\x90\x96\x16` \x85\x01RPPP`@\x01R\x91\x90PV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a4\xCCW__\xFD[\x91\x90PV[_`@\x82\x84\x03\x12\x80\x15a4\xE2W__\xFD[P`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a5\x06Wa5\x06a1>V[`@R\x825\x81Ra5\x19` \x84\x01a4\xB9V[` \x82\x01R\x93\x92PPPV[_``\x82\x84\x03\x12\x80\x15a56W__\xFD[Pa5?a1\x98V[\x825\x81Ra5O` \x84\x01a4\xB9V[` \x82\x01R`@\x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a5nW__\xFD[`@\x82\x01R\x93\x92PPPV[\x835\x81R`\x80\x81\x01c\xFF\xFF\xFF\xFFa5\x93` \x87\x01a4\xB9V[\x16` \x83\x01R`\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16`@\x82\x01R``\x01R\x91\x90PV[\x80\x82\x01\x80\x82\x11\x15a\x07$Wa\x07$a0\xF9V[__\x835\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x846\x03\x01\x81\x12a5\xF9W__\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a6\x13W__\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a&4W__\xFD[` \x81R_a\x166` \x83\x01\x84\x86a3\xFEV[\x805\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x81\x14a4\xCCW__\xFD[_`\x80\x826\x03\x12\x15a6yW__\xFD[`@Q`\x80\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a6\x9CWa6\x9Ca1>V[`@Ra6\xA8\x83a6:V[\x81R` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a6\xC3W__\xFD[a6\xCF6\x82\x86\x01a1\xC1V[` \x83\x01RP`@\x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a6\xEEW__\xFD[a6\xFA6\x82\x86\x01a1\xC1V[`@\x83\x01RPa7\x0C``\x84\x01a6:V[``\x82\x01R\x92\x91PPV[_``\x826\x03\x12\x15a7'W__\xFD[a7/a1\x98V[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a7EW__\xFD[a7Q6\x82\x86\x01a1\xC1V[\x82RP` \x83\x81\x015\x90\x82\x01R`@\x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a7wW__\xFD[a7\x836\x82\x86\x01a1\xC1V[`@\x83\x01RP\x92\x91PPV[_\x81Q\x80` \x84\x01\x85^_\x93\x01\x92\x83RP\x90\x91\x90PV[\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85\x16\x81R_a7\xE2a7\xDC`\x04\x84\x01\x87a7\x8FV[\x85a7\x8FV[\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x93\x90\x93\x16\x83RPP`\x04\x01\x93\x92PPPV[\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83`\xF8\x1B\x16\x81R__\x83Ta8J\x81a2\xADV[`\x01\x82\x16\x80\x15a8aW`\x01\x81\x14a8\x9AWa8\xD0V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x83\x16`\x01\x87\x01R`\x01\x82\x15\x15\x83\x02\x87\x01\x01\x93Pa8\xD0V[\x86_R` _ _[\x83\x81\x10\x15a8\xC5W\x81T`\x01\x82\x8A\x01\x01R`\x01\x82\x01\x91P` \x81\x01\x90Pa8\xA3V[PP`\x01\x82\x87\x01\x01\x93P[P\x91\x96\x95PPPPPPV[_` \x82\x84\x03\x12\x15a8\xECW__\xFD[\x81Q\x80\x15\x15\x81\x14a\x1FZW__\xFD[_` \x82\x84\x03\x12\x15a9\x0BW__\xFD[PQ\x91\x90PV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x07$Wa\x07$a0\xF9V[\x81\x81\x03\x81\x81\x11\x15a\x07$Wa\x07$a0\xF9V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x07$Wa\x07$a0\xF9V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_\x82a9\x97Wa9\x97a9\\V[P\x06\x90V[`\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x07$Wa\x07$a0\xF9V[_a\x1FZ\x82\x84a7\x8FV[`\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x07$Wa\x07$a0\xF9V[`\x01\x81[`\x01\x84\x11\x15a:\x14W\x80\x85\x04\x81\x11\x15a9\xF8Wa9\xF8a0\xF9V[`\x01\x84\x16\x15a:\x06W\x90\x81\x02\x90[`\x01\x93\x90\x93\x1C\x92\x80\x02a9\xDDV[\x93P\x93\x91PPV[_\x82a:*WP`\x01a\x07$V[\x81a:6WP_a\x07$V[\x81`\x01\x81\x14a:LW`\x02\x81\x14a:VWa:rV[`\x01\x91PPa\x07$V[`\xFF\x84\x11\x15a:gWa:ga0\xF9V[PP`\x01\x82\x1Ba\x07$V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a:\x95WP\x81\x81\na\x07$V[a:\xA1_\x19\x84\x84a9\xD9V[\x80_\x19\x04\x82\x11\x15a:\xB4Wa:\xB4a0\xF9V[\x02\x93\x92PPPV[_a\x1FZ\x83\x83a:\x1CV[_\x82a:\xD5Wa:\xD5a9\\V[P\x04\x90V[` \x81R_a\x1FZ` \x83\x01\x84a.\x91V\xFE\xA2dipfsX\"\x12 m\x88\x98\"\x0E\x0F\xAFP\xC7}\x86\xAC\xD2\xDC\xC7}\x7Fs\x18\xD3)F\xDD\xA6\xB1\xF7\xA5\xB8\x89_E\x18dsolcC\0\x08\x1B\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b50600436106100b9575f3560e01c80637378715511610072578063db82d5fa11610058578063db82d5fa14610209578063e4ae61dd1461022f578063e8532be314610242575f5ffd5b806373787155146101ed578063d1920ff014610200575f5ffd5b80632b260fa0116100a25780632b260fa0146100fd5780633c49febe146101c25780635c9ddc84146101d8575f5ffd5b8063171abce5146100bd5780632814a1cd146100dc575b5f5ffd5b6100c5610255565b6040516100d3929190612cc7565b60405180910390f35b6100ef6100ea366004612dae565b6104cf565b6040519081526020016100d3565b6101b061010b366004612df8565b5f60208181529181526040908190208151808301835281548152600182015463ffffffff908116828601526002830154600384015485516060810187526004860154815260058601549384169781019790975264010000000090920467ffffffffffffffff169486019490945260069092015490936001600160a01b039384169390919081169074010000000000000000000000000000000000000000900460ff1686565b6040516100d396959493929190612e0f565b6101ca61072a565b6040516100d3929190612ed3565b6101eb6101e6366004612fde565b610982565b005b6101eb6101fb366004612df8565b610c27565b6100ef61546081565b61021c610217366004612df8565b610dbe565b6040516100d3979695949392919061303b565b6101eb61023d366004612df8565b610ea0565b6101eb610250366004613088565b611077565b6060805f805b60025481101561029a575f818152602081905260409020600601546001600160a01b031615610292578161028e81613126565b9250505b60010161025b565b505f8167ffffffffffffffff8111156102b5576102b561313e565b60405190808252806020026020018201604052801561033957816020015b60408051610100810182525f60c0820181815260e0830182905282526020808301829052828401829052835160608082018652838252818301849052948101839052938301939093526080820181905260a082015282525f199092019101816102d35790505b5090505f8267ffffffffffffffff8111156103565761035661313e565b60405190808252806020026020018201604052801561037f578160200160208202803683370190505b5090505f805b6002548110156104c3575f818152602081905260409020600601546001600160a01b0316156104bb575f8181526020818152604091829020825161010081018452815460c08201908152600183015463ffffffff90811660e084015290825260028301546001600160a01b03908116838601526003840154838701528551606080820188526004860154825260058601549384169682019690965264010000000090920467ffffffffffffffff1695820195909552928101929092526006015491821660808201527401000000000000000000000000000000000000000090910460ff16151560a082015284518590849081106104845761048461316b565b6020026020010181905250808383815181106104a2576104a261316b565b6020908102919091010152816104b781613126565b9250505b600101610385565b50919590945092505050565b5f828152602081905260408120600681015474010000000000000000000000000000000000000000900460ff161561054e5760405162461bcd60e51b815260206004820152601660248201527f4f7264657220416c72656164792041636365707465640000000000000000000060448201526064015b60405180910390fd5b60038101546002820154610571916001600160a01b03909116903390309061139e565b600280545f918261058183613126565b9190505590506040518060e00160405280868152602001856105a29061324c565b815260028401546001600160a01b039081166020808401919091526003860154604080850191909152600687015490921660608401523360808401524260a0909301929092525f8481526001808452919020835181559183015180519091830190819061060f9082613343565b505050604082810151600280840180546001600160a01b039384167fffffffffffffffffffffffff0000000000000000000000000000000000000000918216179091556060860151600380870191909155608087015160048701805491861691841691909117905560a0870151600587018054918616919093161790915560c09095015160069485015592860180547fffffffffffffffffffffff00ffffffffffffffffffffffffffffffffffffffff167401000000000000000000000000000000000000000017905591850154928501549051849389937ffe350ff9ccadd1b7c26b5f96dd078d08a877c8f37d506931ecd8f2bdbd51b6f293610718938b9390921691613427565b60405180910390a39150505b92915050565b6060805f805b600254811015610766575f818152600160205260409020600301541561075e578161075a81613126565b9250505b600101610730565b505f8167ffffffffffffffff8111156107815761078161313e565b6040519080825280602002602001820160405280156107ba57816020015b6107a7612bec565b81526020019060019003908161079f5790505b5090505f8267ffffffffffffffff8111156107d7576107d761313e565b604051908082528060200260200182016040528015610800578160200160208202803683370190505b5090505f805b6002548110156104c3575f818152600160205260409020600301541561097a5760015f8281526020019081526020015f206040518060e00160405290815f8201548152602001600182016040518060200160405290815f8201805461086a906132ad565b80601f0160208091040260200160405190810160405280929190818152602001828054610896906132ad565b80156108e15780601f106108b8576101008083540402835291602001916108e1565b820191905f5260205f20905b8154815290600101906020018083116108c457829003601f168201915b50505091909252505050815260028201546001600160a01b03908116602083015260038301546040830152600483015481166060830152600583015416608082015260069091015460a09091015284518590849081106109435761094361316b565b6020026020010181905250808383815181106109615761096161316b565b60209081029190910101528161097681613126565b9250505b600101610806565b6001600160a01b0382166109d85760405162461bcd60e51b815260206004820152601460248201527f496e76616c696420627579696e6720746f6b656e0000000000000000000000006044820152606401610545565b5f8111610a4d5760405162461bcd60e51b815260206004820152602660248201527f427579696e6720616d6f756e742073686f756c6420626520677265617465722060448201527f7468616e203000000000000000000000000000000000000000000000000000006064820152608401610545565b600280545f9182610a5d83613126565b9190505590506040518060c0016040528086803603810190610a7f91906134d1565b81526001600160a01b038516602082015260408101849052606001610aa936879003870187613525565b8152336020808301919091525f604092830181905284815280825282902083518051825582015160018201805463ffffffff92831663ffffffff19909116179055848301516002830180546001600160a01b039283167fffffffffffffffffffffffff0000000000000000000000000000000000000000909116179055858501516003840155606086015180516004850155938401516005840180549587015167ffffffffffffffff16640100000000027fffffffffffffffffffffffffffffffffffffffff000000000000000000000000909616919093161793909317905560808401516006909101805460a090950151151574010000000000000000000000000000000000000000027fffffffffffffffffffffff0000000000000000000000000000000000000000009095169190921617929092179091555181907ffb2d3310e3e79578ac507cdbdb32e52581dbc17be04e5197d3b7c522735fb9e490610c189088908790879061357a565b60405180910390a25050505050565b5f8181526001602052604090206006810154610c4690615460906135b3565b4211610c945760405162461bcd60e51b815260206004820152601360248201527f52657175657374207374696c6c2076616c6964000000000000000000000000006044820152606401610545565b60058101546001600160a01b03163314610cf05760405162461bcd60e51b815260206004820152601760248201527f53656e646572206e6f7420746865206163636570746f720000000000000000006044820152606401610545565b60038101546002820154610d11916001600160a01b03909116903390611455565b5f82815260016020819052604082208281559190820181610d328282612c3b565b5050506002810180547fffffffffffffffffffffffff00000000000000000000000000000000000000009081169091555f60038301819055600483018054831690556005830180549092169091556006909101556040518281527f9c216a4617d6c03dc7cbd9632166f1c5c9ef41f9ee86bf3b83f671c005107704906020015b60405180910390a15050565b600160208181525f92835260409283902080548451928301909452918201805482908290610deb906132ad565b80601f0160208091040260200160405190810160405280929190818152602001828054610e17906132ad565b8015610e625780601f10610e3957610100808354040283529160200191610e62565b820191905f5260205f20905b815481529060010190602001808311610e4557829003601f168201915b50505091909252505050600282015460038301546004840154600585015460069095015493946001600160a01b039384169492939182169291169087565b5f81815260208190526040902060068101546001600160a01b03163314610f095760405162461bcd60e51b815260206004820152601860248201527f53656e646572206e6f74207468652072657175657374657200000000000000006044820152606401610545565b600681015474010000000000000000000000000000000000000000900460ff1615610f9c5760405162461bcd60e51b815260206004820152603060248201527f4f726465722068617320616c7265616479206265656e2061636365707465642c60448201527f2063616e6e6f74207769746864726177000000000000000000000000000000006064820152608401610545565b5f8281526020818152604080832083815560018101805463ffffffff191690556002810180547fffffffffffffffffffffffff00000000000000000000000000000000000000001690556003810184905560048101939093556005830180547fffffffffffffffffffffffffffffffffffffffff000000000000000000000000169055600690920180547fffffffffffffffffffffff00000000000000000000000000000000000000000016905590518381527fb35b3fe4daaf6cc66eb8bd413e9ab54449e766f6d46125cc58f255694a0e847e9101610db2565b5f83815260016020526040902060048101546001600160a01b031633146110e05760405162461bcd60e51b815260206004820152601860248201527f53656e646572206e6f74207468652072657175657374657200000000000000006044820152606401610545565b80545f908152602081905260409081902060035490916001600160a01b039091169063d38c29a190611114908601866135c6565b6040518363ffffffff1660e01b8152600401611131929190613627565b5f604051808303815f87803b158015611148575f5ffd5b505af115801561115a573d5f5f3e3d5ffd5b5050505061118b6004548561116e90613669565b61117786613717565b6003546001600160a01b03169291906114a3565b5061121161119c60208601866135c6565b8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f92019190915250506040805160608101825260048701548152600587015463ffffffff81166020830152640100000000900467ffffffffffffffff1691810191909152915061163e9050565b61121e826001018561183f565b600482015460038301546002840154611245926001600160a01b0391821692911690611455565b81545f908152602081815260408083208381556001808201805463ffffffff191690556002820180547fffffffffffffffffffffffff000000000000000000000000000000000000000016905560038201859055600482018590556005820180547fffffffffffffffffffffffffffffffffffffffff000000000000000000000000169055600690910180547fffffffffffffffffffffff00000000000000000000000000000000000000000016905588845291829052822082815591908201816113108282612c3b565b5050506002810180547fffffffffffffffffffffffff00000000000000000000000000000000000000009081169091555f60038301819055600483018054831690556005830180549092169091556006909101556040518581527fc577309acd7939cc2f01f67f073e1a548224454cdddc79b161db17b5315e9f0c9060200160405180910390a15050505050565b6040516001600160a01b038085166024830152831660448201526064810182905261144f9085907f23b872dd00000000000000000000000000000000000000000000000000000000906084015b60408051601f198184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167fffffffff00000000000000000000000000000000000000000000000000000000909316929092179091526118c6565b50505050565b6040516001600160a01b03831660248201526044810182905261149e9084907fa9059cbb00000000000000000000000000000000000000000000000000000000906064016113eb565b505050565b5f6114b183602001516119aa565b6114fd5760405162461bcd60e51b815260206004820152601d60248201527f496e76616c696420696e70757420766563746f722070726f76696465640000006044820152606401610545565b61150a8360400151611a44565b6115565760405162461bcd60e51b815260206004820152601e60248201527f496e76616c6964206f757470757420766563746f722070726f766964656400006044820152606401610545565b611593835f015184602001518560400151866060015160405160200161157f94939291906137a6565b604051602081830303815290604052611ad1565b90506115b56115a58360400151611af3565b8351602085015184929190611aff565b6116275760405162461bcd60e51b815260206004820152603c60248201527f5478206d65726b6c652070726f6f66206973206e6f742076616c696420666f7260448201527f2070726f76696465642068656164657220616e642074782068617368000000006064820152608401610545565b61163685858460400151611b3a565b949350505050565b5f5f61164984611e8a565b9092509050600182016116c45760405162461bcd60e51b815260206004820152602260248201527f52656164206f76657272756e20647572696e6720566172496e7420706172736960448201527f6e670000000000000000000000000000000000000000000000000000000000006064820152608401610545565b5f806116d18460016135b3565b90505f6116e0865f0151611e9f565b90505f5b848110156117d0575f6116f78985611f4c565b90505f61172d6117078b87611f61565b60d881901c63ff00ff001662ff00ff60e89290921c9190911617601081811b91901c1790565b9050818414801561174d57508063ffffffff16896020015163ffffffff16145b1561175e5750505050505050505050565b6117688a86611f77565b95505f1986036117ba5760405162461bcd60e51b815260206004820152601760248201527f42616420566172496e7420696e207363726970745369670000000000000000006044820152606401610545565b6117c486866135b3565b945050506001016116e4565b5060405162461bcd60e51b815260206004820152602c60248201527f5472616e73616374696f6e20646f6573206e6f74207370656e6420746865207260448201527f65717569726564207574786f00000000000000000000000000000000000000006064820152608401610545565b5f825f01805461184e906132ad565b60405161186092508590602001613815565b60405160208183030381529060405280519060200120905061144f82806040019061188b91906135c6565b8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f92019190915250859250611fbd915050565b5f61191a826040518060400160405280602081526020017f5361666545524332303a206c6f772d6c6576656c2063616c6c206661696c6564815250856001600160a01b031661215b9092919063ffffffff16565b80519091501561149e578080602001905181019061193891906138dc565b61149e5760405162461bcd60e51b815260206004820152602a60248201527f5361666545524332303a204552433230206f7065726174696f6e20646964206e60448201527f6f742073756363656564000000000000000000000000000000000000000000006064820152608401610545565b5f5f5f6119b684611e8a565b90925090508015806119c857505f1982145b156119d657505f9392505050565b5f6119e28360016135b3565b90505f5b82811015611a375785518210611a0157505f95945050505050565b5f611a0c8784611f77565b90505f198103611a2257505f9695505050505050565b611a2c81846135b3565b9250506001016119e6565b5093519093149392505050565b5f5f5f611a5084611e8a565b9092509050801580611a6257505f1982145b15611a7057505f9392505050565b5f611a7c8360016135b3565b90505f5b82811015611a375785518210611a9b57505f95945050505050565b5f611aa68784612169565b90505f198103611abc57505f9695505050505050565b611ac681846135b3565b925050600101611a80565b5f60205f83516020850160025afa5060205f60205f60025afa50505f51919050565b60448101515f90610724565b5f8385148015611b0d575081155b8015611b1857508251155b15611b2557506001611636565b611b31858486856121c9565b95945050505050565b5f836001600160a01b031663113764be6040518163ffffffff1660e01b8152600401602060405180830381865afa158015611b77573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611b9b91906138fb565b90505f846001600160a01b0316632b97be246040518163ffffffff1660e01b8152600401602060405180830381865afa158015611bda573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611bfe91906138fb565b90505f80611c13611c0e8661226e565b612279565b9050838103611c2457839150611ca1565b828103611c3357829150611ca1565b60405162461bcd60e51b815260206004820152602560248201527f4e6f742061742063757272656e74206f722070726576696f757320646966666960448201527f63756c74790000000000000000000000000000000000000000000000000000006064820152608401610545565b5f611cab866122a0565b90505f198103611d235760405162461bcd60e51b815260206004820152602360248201527f496e76616c6964206c656e677468206f6620746865206865616465727320636860448201527f61696e00000000000000000000000000000000000000000000000000000000006064820152608401610545565b7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe8103611d925760405162461bcd60e51b815260206004820152601560248201527f496e76616c6964206865616465727320636861696e00000000000000000000006044820152606401610545565b7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffd8103611e015760405162461bcd60e51b815260206004820152601d60248201527f496e73756666696369656e7420776f726b20696e2061206865616465720000006044820152606401610545565b611e0b8784613912565b811015611e805760405162461bcd60e51b815260206004820152603360248201527f496e73756666696369656e7420616363756d756c61746564206469666669637560448201527f6c747920696e2068656164657220636861696e000000000000000000000000006064820152608401610545565b5050505050505050565b5f5f611e96835f6124c4565b91509150915091565b6040805160208082528183019092525f918291906020820181803683370190505090505f5b6020811015611f4157838160208110611edf57611edf61316b565b1a60f81b826001611ef1846020613929565b611efb9190613929565b81518110611f0b57611f0b61316b565b60200101907effffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff191690815f1a905350600101611ec4565b506020015192915050565b5f611f5a8383016020015190565b9392505050565b5f611f5a611f708360206135b3565b8490611f4c565b5f5f5f611f84858561263b565b909250905060018201611f9c575f1992505050610724565b80611fa88360256135b3565b611fb291906135b3565b611b319060046135b3565b604080516060810182525f8082526020808301829052828401829052835180850190945281845283015290611ff184611e8a565b60208301528082528161200382613126565b9052505f805b82602001518110156121055782515f90612024908890612169565b84519091505f90612036908990612679565b90505f612044600884613929565b86519091505f906120569060086135b3565b8a8101602001839020909150808a03612090576001965083895f0181815161207e919061393c565b67ffffffffffffffff169052506120e0565b5f61209e8c8a5f01516126ef565b90506001600160a01b038116156120bf576001600160a01b03811660208b01525b5f6120cd8d8b5f01516127cf565b905080156120dd5760408b018190525b50505b84885f018181516120f191906135b3565b905250506001909401935061200992505050565b50806121535760405162461bcd60e51b815260206004820181905260248201527f4e6f206f757470757420666f756e6420666f72207363726970745075624b65796044820152606401610545565b505092915050565b606061163684845f856128af565b5f6121758260096135b3565b8351101561218557505f19610724565b5f8061219b856121968660086135b3565b6124c4565b9092509050600182016121b3575f1992505050610724565b806121bf8360096135b3565b611b3191906135b3565b5f602084516121d89190613989565b156121e457505f611636565b83515f036121f357505f611636565b81855f5b86518110156122615761220b600284613989565b60010361222f576122286122228883016020015190565b836129f3565b9150612248565b612245826122408984016020015190565b6129f3565b91505b60019290921c9161225a6020826135b3565b90506121f7565b5090931495945050505050565b5f610724825f6129fe565b5f6107247bffff000000000000000000000000000000000000000000000000000083612a97565b5f605082516122af9190613989565b156122bc57505f19919050565b505f80805b83518110156124bd578015612308576122db848284612aa2565b61230857507ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe9392505050565b5f61231385836129fe565b905061232185836050612acb565b925080612464845f8190506008817eff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff16901b600882901c7eff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff161790506010817dffff0000ffff0000ffff0000ffff0000ffff0000ffff0000ffff0000ffff16901b601082901c7dffff0000ffff0000ffff0000ffff0000ffff0000ffff0000ffff0000ffff161790506020817bffffffff00000000ffffffff00000000ffffffff00000000ffffffff16901b602082901c7bffffffff00000000ffffffff00000000ffffffff00000000ffffffff1617905060408177ffffffffffffffff0000000000000000ffffffffffffffff16901b604082901c77ffffffffffffffff0000000000000000ffffffffffffffff16179050608081901b608082901c179050919050565b111561249457507ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffd949350505050565b61249d81612279565b6124a790856135b3565b93506124b690506050826135b3565b90506122c1565b5050919050565b5f5f5f6124d18585612af0565b90508060ff165f03612504575f8585815181106124f0576124f061316b565b016020015190935060f81c91506126349050565b8361251082600161399c565b60ff1661251d91906135b3565b85511015612532575f195f9250925050612634565b5f8160ff166002036125755761256a61255661254f8760016135b3565b8890611f4c565b62ffff0060e882901c1660f89190911c1790565b61ffff16905061262a565b8160ff1660040361259e5761259161170761254f8760016135b3565b63ffffffff16905061262a565b8160ff1660080361262a5761261d6125ba61254f8760016135b3565b60c01c64ff000000ff600882811c91821665ff000000ff009390911b92831617601090811b67ffffffffffffffff1666ff00ff00ff00ff9290921667ff00ff00ff00ff009093169290921790911c65ffff0000ffff1617602081811c91901b1790565b67ffffffffffffffff1690505b60ff909116925090505b9250929050565b5f806126488360256135b3565b8451101561265b57505f1990505f612634565b5f8061266c866121968760246135b3565b9097909650945050505050565b5f806126858484611f4c565b60c01c90505f611b318264ff000000ff600882811c91821665ff000000ff009390911b92831617601090811b67ffffffffffffffff1666ff00ff00ff00ff9290921667ff00ff00ff00ff009093169290921790911c65ffff0000ffff1617602081811c91901b1790565b5f826126fc8360096135b3565b8151811061270c5761270c61316b565b6020910101517fff00000000000000000000000000000000000000000000000000000000000000167f6a000000000000000000000000000000000000000000000000000000000000001461276157505f610724565b5f8361276e84600a6135b3565b8151811061277e5761277e61316b565b01602001517fff000000000000000000000000000000000000000000000000000000000000008116915060f81c6014036127c8575f6127be84600b6135b3565b8501601401519250505b5092915050565b5f826127dc8360096135b3565b815181106127ec576127ec61316b565b6020910101517fff00000000000000000000000000000000000000000000000000000000000000167f6a000000000000000000000000000000000000000000000000000000000000001461284157505f610724565b5f8361284e84600a6135b3565b8151811061285e5761285e61316b565b016020908101517fff000000000000000000000000000000000000000000000000000000000000008116925060f81c90036127c8575f61289f84600b6135b3565b8501602001519250505092915050565b6060824710156129275760405162461bcd60e51b815260206004820152602660248201527f416464726573733a20696e73756666696369656e742062616c616e636520666f60448201527f722063616c6c00000000000000000000000000000000000000000000000000006064820152608401610545565b6001600160a01b0385163b61297e5760405162461bcd60e51b815260206004820152601d60248201527f416464726573733a2063616c6c20746f206e6f6e2d636f6e74726163740000006044820152606401610545565b5f5f866001600160a01b0316858760405161299991906139b5565b5f6040518083038185875af1925050503d805f81146129d3576040519150601f19603f3d011682016040523d82523d5f602084013e6129d8565b606091505b50915091506129e8828286612b74565b979650505050505050565b5f611f5a8383612bad565b5f80612a15612a0e8460486135b3565b8590611f4c565b60e81c90505f84612a2785604b6135b3565b81518110612a3757612a3761316b565b016020015160f81c90505f612a69835f60108262ffffff16901c8261ff001660108462ffffff16901b17179050919050565b62ffffff1690505f612a7c6003846139c0565b60ff169050612a8d81610100613abc565b6129e89083613912565b5f611f5a8284613ac7565b5f80612aae8585612bd4565b9050828114612ac0575f915050611f5a565b506001949350505050565b5f60205f8385602001870160025afa5060205f60205f60025afa50505f519392505050565b5f828281518110612b0357612b0361316b565b016020015160f81c60ff03612b1a57506008610724565b828281518110612b2c57612b2c61316b565b016020015160f81c60fe03612b4357506004610724565b828281518110612b5557612b5561316b565b016020015160f81c60fd03612b6c57506002610724565b505f92915050565b60608315612b83575081611f5a565b825115612b935782518084602001fd5b8160405162461bcd60e51b81526004016105459190613ada565b5f825f528160205260205f60405f60025afa5060205f60205f60025afa50505f5192915050565b5f611f5a612be38360046135b3565b84016020015190565b6040518060e001604052805f8152602001612c136040518060200160405280606081525090565b81525f6020820181905260408201819052606082018190526080820181905260a09091015290565b508054612c47906132ad565b5f825580601f10612c56575050565b601f0160209004905f5260205f2090810190612c729190612c75565b50565b5b80821115612c89575f8155600101612c76565b5090565b5f8151808452602084019350602083015f5b82811015612cbd578151865260209586019590910190600101612c9f565b5093949350505050565b604080825283519082018190525f9060208501906060840190835b81811015612d90578351612d078482518051825260209081015163ffffffff16910152565b6001600160a01b036020820151166040850152604081015160608501526060810151612d5a60808601828051825260208082015163ffffffff169083015260409081015167ffffffffffffffff16910152565b5060808101516001600160a01b031660e085015260a0015115156101008401526020939093019261012090920191600101612ce2565b50508381036020850152612da48186612c8d565b9695505050505050565b5f5f60408385031215612dbf575f5ffd5b82359150602083013567ffffffffffffffff811115612ddc575f5ffd5b830160208186031215612ded575f5ffd5b809150509250929050565b5f60208284031215612e08575f5ffd5b5035919050565b8651815260208088015163ffffffff169082015261012081016001600160a01b0387166040830152856060830152612e6e60808301868051825260208082015163ffffffff169083015260409081015167ffffffffffffffff16910152565b6001600160a01b03841660e0830152821515610100830152979650505050505050565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b5f8151602084526116366020850182612e91565b5f604082016040835280855180835260608501915060608160051b8601019250602087015f5b82811015612fb2577fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa0878603018452815180518652602081015160e06020880152612f4760e0880182612ebf565b90506001600160a01b036040830151166040880152606082015160608801526001600160a01b0360808301511660808801526001600160a01b0360a08301511660a088015260c082015160c08801528096505050602082019150602084019350600181019050612ef9565b505050508281036020840152611b318185612c8d565b5f60608284031215612fd8575f5ffd5b50919050565b5f5f5f5f84860360e0811215612ff2575f5ffd5b6040811215612fff575f5ffd5b508493506130108660408701612fc8565b925060a08501356001600160a01b038116811461302b575f5ffd5b9396929550929360c00135925050565b87815260e060208201525f61305360e0830189612ebf565b6001600160a01b0397881660408401526060830196909652509285166080840152931660a082015260c0019190915292915050565b5f5f5f6060848603121561309a575f5ffd5b83359250602084013567ffffffffffffffff8111156130b7575f5ffd5b8401608081870312156130c8575f5ffd5b9150604084013567ffffffffffffffff8111156130e3575f5ffd5b6130ef86828701612fc8565b9150509250925092565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f5f198203613137576131376130f9565b5060010190565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b6040516060810167ffffffffffffffff811182821017156131bb576131bb61313e565b60405290565b5f82601f8301126131d0575f5ffd5b813567ffffffffffffffff8111156131ea576131ea61313e565b604051601f8201601f19908116603f0116810167ffffffffffffffff811182821017156132195761321961313e565b604052818152838201602001851015613230575f5ffd5b816020850160208301375f918101602001919091529392505050565b5f6020823603121561325c575f5ffd5b6040516020810167ffffffffffffffff8111828210171561327f5761327f61313e565b604052823567ffffffffffffffff811115613298575f5ffd5b6132a4368286016131c1565b82525092915050565b600181811c908216806132c157607f821691505b602082108103612fd8577f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b601f82111561149e57805f5260205f20601f840160051c8101602085101561331d5750805b601f840160051c820191505b8181101561333c575f8155600101613329565b5050505050565b815167ffffffffffffffff81111561335d5761335d61313e565b6133718161336b84546132ad565b846132f8565b6020601f8211600181146133a3575f831561338c5750848201515b5f19600385901b1c1916600184901b17845561333c565b5f84815260208120601f198516915b828110156133d257878501518255602094850194600190920191016133b2565b50848210156133ef57868401515f19600387901b60f8161c191681555b50505050600190811b01905550565b81835281816020850137505f602082840101525f6020601f19601f840116840101905092915050565b606081525f84357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe186360301811261345d575f5ffd5b850160208101903567ffffffffffffffff811115613479575f5ffd5b803603821315613487575f5ffd5b6020606085015261349c6080850182846133fe565b6001600160a01b0396909616602085015250505060400152919050565b803563ffffffff811681146134cc575f5ffd5b919050565b5f60408284031280156134e2575f5ffd5b506040805190810167ffffffffffffffff811182821017156135065761350661313e565b60405282358152613519602084016134b9565b60208201529392505050565b5f6060828403128015613536575f5ffd5b5061353f613198565b8235815261354f602084016134b9565b6020820152604083013567ffffffffffffffff8116811461356e575f5ffd5b60408201529392505050565b833581526080810163ffffffff613593602087016134b9565b1660208301526001600160a01b0393909316604082015260600152919050565b80820180821115610724576107246130f9565b5f5f83357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe18436030181126135f9575f5ffd5b83018035915067ffffffffffffffff821115613613575f5ffd5b602001915036819003821315612634575f5ffd5b602081525f6116366020830184866133fe565b80357fffffffff00000000000000000000000000000000000000000000000000000000811681146134cc575f5ffd5b5f60808236031215613679575f5ffd5b6040516080810167ffffffffffffffff8111828210171561369c5761369c61313e565b6040526136a88361363a565b8152602083013567ffffffffffffffff8111156136c3575f5ffd5b6136cf368286016131c1565b602083015250604083013567ffffffffffffffff8111156136ee575f5ffd5b6136fa368286016131c1565b60408301525061370c6060840161363a565b606082015292915050565b5f60608236031215613727575f5ffd5b61372f613198565b823567ffffffffffffffff811115613745575f5ffd5b613751368286016131c1565b82525060208381013590820152604083013567ffffffffffffffff811115613777575f5ffd5b613783368286016131c1565b60408301525092915050565b5f81518060208401855e5f93019283525090919050565b7fffffffff00000000000000000000000000000000000000000000000000000000851681525f6137e26137dc600484018761378f565b8561378f565b7fffffffff0000000000000000000000000000000000000000000000000000000093909316835250506004019392505050565b7fff000000000000000000000000000000000000000000000000000000000000008360f81b1681525f5f835461384a816132ad565b600182168015613861576001811461389a576138d0565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00831660018701526001821515830287010193506138d0565b865f5260205f205f5b838110156138c55781546001828a0101526001820191506020810190506138a3565b505060018287010193505b50919695505050505050565b5f602082840312156138ec575f5ffd5b81518015158114611f5a575f5ffd5b5f6020828403121561390b575f5ffd5b5051919050565b8082028115828204841417610724576107246130f9565b81810381811115610724576107246130f9565b67ffffffffffffffff8181168382160190811115610724576107246130f9565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f826139975761399761395c565b500690565b60ff8181168382160190811115610724576107246130f9565b5f611f5a828461378f565b60ff8281168282160390811115610724576107246130f9565b6001815b6001841115613a14578085048111156139f8576139f86130f9565b6001841615613a0657908102905b60019390931c9280026139dd565b935093915050565b5f82613a2a57506001610724565b81613a3657505f610724565b8160018114613a4c5760028114613a5657613a72565b6001915050610724565b60ff841115613a6757613a676130f9565b50506001821b610724565b5060208310610133831016604e8410600b8410161715613a95575081810a610724565b613aa15f1984846139d9565b805f1904821115613ab457613ab46130f9565b029392505050565b5f611f5a8383613a1c565b5f82613ad557613ad561395c565b500490565b602081525f611f5a6020830184612e9156fea26469706673582212206d8898220e0faf50c77d86acd2dcc77d7f7318d32946dda6b1f7a5b8895f451864736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0\xB9W_5`\xE0\x1C\x80csxqU\x11a\0rW\x80c\xDB\x82\xD5\xFA\x11a\0XW\x80c\xDB\x82\xD5\xFA\x14a\x02\tW\x80c\xE4\xAEa\xDD\x14a\x02/W\x80c\xE8S+\xE3\x14a\x02BW__\xFD[\x80csxqU\x14a\x01\xEDW\x80c\xD1\x92\x0F\xF0\x14a\x02\0W__\xFD[\x80c+&\x0F\xA0\x11a\0\xA2W\x80c+&\x0F\xA0\x14a\0\xFDW\x80c<I\xFE\xBE\x14a\x01\xC2W\x80c\\\x9D\xDC\x84\x14a\x01\xD8W__\xFD[\x80c\x17\x1A\xBC\xE5\x14a\0\xBDW\x80c(\x14\xA1\xCD\x14a\0\xDCW[__\xFD[a\0\xC5a\x02UV[`@Qa\0\xD3\x92\x91\x90a,\xC7V[`@Q\x80\x91\x03\x90\xF3[a\0\xEFa\0\xEA6`\x04a-\xAEV[a\x04\xCFV[`@Q\x90\x81R` \x01a\0\xD3V[a\x01\xB0a\x01\x0B6`\x04a-\xF8V[_` \x81\x81R\x91\x81R`@\x90\x81\x90 \x81Q\x80\x83\x01\x83R\x81T\x81R`\x01\x82\x01Tc\xFF\xFF\xFF\xFF\x90\x81\x16\x82\x86\x01R`\x02\x83\x01T`\x03\x84\x01T\x85Q``\x81\x01\x87R`\x04\x86\x01T\x81R`\x05\x86\x01T\x93\x84\x16\x97\x81\x01\x97\x90\x97Rd\x01\0\0\0\0\x90\x92\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x94\x86\x01\x94\x90\x94R`\x06\x90\x92\x01T\x90\x93`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x93\x90\x91\x90\x81\x16\x90t\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16\x86V[`@Qa\0\xD3\x96\x95\x94\x93\x92\x91\x90a.\x0FV[a\x01\xCAa\x07*V[`@Qa\0\xD3\x92\x91\x90a.\xD3V[a\x01\xEBa\x01\xE66`\x04a/\xDEV[a\t\x82V[\0[a\x01\xEBa\x01\xFB6`\x04a-\xF8V[a\x0C'V[a\0\xEFaT`\x81V[a\x02\x1Ca\x02\x176`\x04a-\xF8V[a\r\xBEV[`@Qa\0\xD3\x97\x96\x95\x94\x93\x92\x91\x90a0;V[a\x01\xEBa\x02=6`\x04a-\xF8V[a\x0E\xA0V[a\x01\xEBa\x02P6`\x04a0\x88V[a\x10wV[``\x80_\x80[`\x02T\x81\x10\x15a\x02\x9AW_\x81\x81R` \x81\x90R`@\x90 `\x06\x01T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x02\x92W\x81a\x02\x8E\x81a1&V[\x92PP[`\x01\x01a\x02[V[P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x02\xB5Wa\x02\xB5a1>V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x039W\x81` \x01[`@\x80Qa\x01\0\x81\x01\x82R_`\xC0\x82\x01\x81\x81R`\xE0\x83\x01\x82\x90R\x82R` \x80\x83\x01\x82\x90R\x82\x84\x01\x82\x90R\x83Q``\x80\x82\x01\x86R\x83\x82R\x81\x83\x01\x84\x90R\x94\x81\x01\x83\x90R\x93\x83\x01\x93\x90\x93R`\x80\x82\x01\x81\x90R`\xA0\x82\x01R\x82R_\x19\x90\x92\x01\x91\x01\x81a\x02\xD3W\x90P[P\x90P_\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x03VWa\x03Va1>V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x03\x7FW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_\x80[`\x02T\x81\x10\x15a\x04\xC3W_\x81\x81R` \x81\x90R`@\x90 `\x06\x01T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x04\xBBW_\x81\x81R` \x81\x81R`@\x91\x82\x90 \x82Qa\x01\0\x81\x01\x84R\x81T`\xC0\x82\x01\x90\x81R`\x01\x83\x01Tc\xFF\xFF\xFF\xFF\x90\x81\x16`\xE0\x84\x01R\x90\x82R`\x02\x83\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83\x86\x01R`\x03\x84\x01T\x83\x87\x01R\x85Q``\x80\x82\x01\x88R`\x04\x86\x01T\x82R`\x05\x86\x01T\x93\x84\x16\x96\x82\x01\x96\x90\x96Rd\x01\0\0\0\0\x90\x92\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x95\x82\x01\x95\x90\x95R\x92\x81\x01\x92\x90\x92R`\x06\x01T\x91\x82\x16`\x80\x82\x01Rt\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x04`\xFF\x16\x15\x15`\xA0\x82\x01R\x84Q\x85\x90\x84\x90\x81\x10a\x04\x84Wa\x04\x84a1kV[` \x02` \x01\x01\x81\x90RP\x80\x83\x83\x81Q\x81\x10a\x04\xA2Wa\x04\xA2a1kV[` \x90\x81\x02\x91\x90\x91\x01\x01R\x81a\x04\xB7\x81a1&V[\x92PP[`\x01\x01a\x03\x85V[P\x91\x95\x90\x94P\x92PPPV[_\x82\x81R` \x81\x90R`@\x81 `\x06\x81\x01Tt\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16\x15a\x05NW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01R\x7FOrder Already Accepted\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\x03\x81\x01T`\x02\x82\x01Ta\x05q\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x903\x900\x90a\x13\x9EV[`\x02\x80T_\x91\x82a\x05\x81\x83a1&V[\x91\x90PU\x90P`@Q\x80`\xE0\x01`@R\x80\x86\x81R` \x01\x85a\x05\xA2\x90a2LV[\x81R`\x02\x84\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16` \x80\x84\x01\x91\x90\x91R`\x03\x86\x01T`@\x80\x85\x01\x91\x90\x91R`\x06\x87\x01T\x90\x92\x16``\x84\x01R3`\x80\x84\x01RB`\xA0\x90\x93\x01\x92\x90\x92R_\x84\x81R`\x01\x80\x84R\x91\x90 \x83Q\x81U\x91\x83\x01Q\x80Q\x90\x91\x83\x01\x90\x81\x90a\x06\x0F\x90\x82a3CV[PPP`@\x82\x81\x01Q`\x02\x80\x84\x01\x80T`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x82\x16\x17\x90\x91U``\x86\x01Q`\x03\x80\x87\x01\x91\x90\x91U`\x80\x87\x01Q`\x04\x87\x01\x80T\x91\x86\x16\x91\x84\x16\x91\x90\x91\x17\x90U`\xA0\x87\x01Q`\x05\x87\x01\x80T\x91\x86\x16\x91\x90\x93\x16\x17\x90\x91U`\xC0\x90\x95\x01Q`\x06\x94\x85\x01U\x92\x86\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16t\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90U\x91\x85\x01T\x92\x85\x01T\x90Q\x84\x93\x89\x93\x7F\xFE5\x0F\xF9\xCC\xAD\xD1\xB7\xC2k_\x96\xDD\x07\x8D\x08\xA8w\xC8\xF3}Pi1\xEC\xD8\xF2\xBD\xBDQ\xB6\xF2\x93a\x07\x18\x93\x8B\x93\x90\x92\x16\x91a4'V[`@Q\x80\x91\x03\x90\xA3\x91PP[\x92\x91PPV[``\x80_\x80[`\x02T\x81\x10\x15a\x07fW_\x81\x81R`\x01` R`@\x90 `\x03\x01T\x15a\x07^W\x81a\x07Z\x81a1&V[\x92PP[`\x01\x01a\x070V[P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07\x81Wa\x07\x81a1>V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x07\xBAW\x81` \x01[a\x07\xA7a+\xECV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x07\x9FW\x90P[P\x90P_\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07\xD7Wa\x07\xD7a1>V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x08\0W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_\x80[`\x02T\x81\x10\x15a\x04\xC3W_\x81\x81R`\x01` R`@\x90 `\x03\x01T\x15a\tzW`\x01_\x82\x81R` \x01\x90\x81R` \x01_ `@Q\x80`\xE0\x01`@R\x90\x81_\x82\x01T\x81R` \x01`\x01\x82\x01`@Q\x80` \x01`@R\x90\x81_\x82\x01\x80Ta\x08j\x90a2\xADV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x08\x96\x90a2\xADV[\x80\x15a\x08\xE1W\x80`\x1F\x10a\x08\xB8Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x08\xE1V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x08\xC4W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x90\x92RPPP\x81R`\x02\x82\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16` \x83\x01R`\x03\x83\x01T`@\x83\x01R`\x04\x83\x01T\x81\x16``\x83\x01R`\x05\x83\x01T\x16`\x80\x82\x01R`\x06\x90\x91\x01T`\xA0\x90\x91\x01R\x84Q\x85\x90\x84\x90\x81\x10a\tCWa\tCa1kV[` \x02` \x01\x01\x81\x90RP\x80\x83\x83\x81Q\x81\x10a\taWa\taa1kV[` \x90\x81\x02\x91\x90\x91\x01\x01R\x81a\tv\x81a1&V[\x92PP[`\x01\x01a\x08\x06V[`\x01`\x01`\xA0\x1B\x03\x82\x16a\t\xD8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01R\x7FInvalid buying token\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05EV[_\x81\x11a\nMW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FBuying amount should be greater `D\x82\x01R\x7Fthan 0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05EV[`\x02\x80T_\x91\x82a\n]\x83a1&V[\x91\x90PU\x90P`@Q\x80`\xC0\x01`@R\x80\x86\x806\x03\x81\x01\x90a\n\x7F\x91\x90a4\xD1V[\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16` \x82\x01R`@\x81\x01\x84\x90R``\x01a\n\xA96\x87\x90\x03\x87\x01\x87a5%V[\x81R3` \x80\x83\x01\x91\x90\x91R_`@\x92\x83\x01\x81\x90R\x84\x81R\x80\x82R\x82\x90 \x83Q\x80Q\x82U\x82\x01Q`\x01\x82\x01\x80Tc\xFF\xFF\xFF\xFF\x92\x83\x16c\xFF\xFF\xFF\xFF\x19\x90\x91\x16\x17\x90U\x84\x83\x01Q`\x02\x83\x01\x80T`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x17\x90U\x85\x85\x01Q`\x03\x84\x01U``\x86\x01Q\x80Q`\x04\x85\x01U\x93\x84\x01Q`\x05\x84\x01\x80T\x95\x87\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16d\x01\0\0\0\0\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\x90\x96\x16\x91\x90\x93\x16\x17\x93\x90\x93\x17\x90U`\x80\x84\x01Q`\x06\x90\x91\x01\x80T`\xA0\x90\x95\x01Q\x15\x15t\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x95\x16\x91\x90\x92\x16\x17\x92\x90\x92\x17\x90\x91UQ\x81\x90\x7F\xFB-3\x10\xE3\xE7\x95x\xACP|\xDB\xDB2\xE5%\x81\xDB\xC1{\xE0NQ\x97\xD3\xB7\xC5\"s_\xB9\xE4\x90a\x0C\x18\x90\x88\x90\x87\x90\x87\x90a5zV[`@Q\x80\x91\x03\x90\xA2PPPPPV[_\x81\x81R`\x01` R`@\x90 `\x06\x81\x01Ta\x0CF\x90aT`\x90a5\xB3V[B\x11a\x0C\x94W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01R\x7FRequest still valid\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05EV[`\x05\x81\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0C\xF0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FSender not the acceptor\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05EV[`\x03\x81\x01T`\x02\x82\x01Ta\r\x11\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x903\x90a\x14UV[_\x82\x81R`\x01` \x81\x90R`@\x82 \x82\x81U\x91\x90\x82\x01\x81a\r2\x82\x82a,;V[PPP`\x02\x81\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x81\x16\x90\x91U_`\x03\x83\x01\x81\x90U`\x04\x83\x01\x80T\x83\x16\x90U`\x05\x83\x01\x80T\x90\x92\x16\x90\x91U`\x06\x90\x91\x01U`@Q\x82\x81R\x7F\x9C!jF\x17\xD6\xC0=\xC7\xCB\xD9c!f\xF1\xC5\xC9\xEFA\xF9\xEE\x86\xBF;\x83\xF6q\xC0\x05\x10w\x04\x90` \x01[`@Q\x80\x91\x03\x90\xA1PPV[`\x01` \x81\x81R_\x92\x83R`@\x92\x83\x90 \x80T\x84Q\x92\x83\x01\x90\x94R\x91\x82\x01\x80T\x82\x90\x82\x90a\r\xEB\x90a2\xADV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0E\x17\x90a2\xADV[\x80\x15a\x0EbW\x80`\x1F\x10a\x0E9Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0EbV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0EEW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x90\x92RPPP`\x02\x82\x01T`\x03\x83\x01T`\x04\x84\x01T`\x05\x85\x01T`\x06\x90\x95\x01T\x93\x94`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x94\x92\x93\x91\x82\x16\x92\x91\x16\x90\x87V[_\x81\x81R` \x81\x90R`@\x90 `\x06\x81\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0F\tW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FSender not the requester\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05EV[`\x06\x81\x01Tt\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16\x15a\x0F\x9CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`0`$\x82\x01R\x7FOrder has already been accepted,`D\x82\x01R\x7F cannot withdraw\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05EV[_\x82\x81R` \x81\x81R`@\x80\x83 \x83\x81U`\x01\x81\x01\x80Tc\xFF\xFF\xFF\xFF\x19\x16\x90U`\x02\x81\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90U`\x03\x81\x01\x84\x90U`\x04\x81\x01\x93\x90\x93U`\x05\x83\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90U`\x06\x90\x92\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90U\x90Q\x83\x81R\x7F\xB3[?\xE4\xDA\xAFl\xC6n\xB8\xBDA>\x9A\xB5DI\xE7f\xF6\xD4a%\xCCX\xF2UiJ\x0E\x84~\x91\x01a\r\xB2V[_\x83\x81R`\x01` R`@\x90 `\x04\x81\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x10\xE0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FSender not the requester\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05EV[\x80T_\x90\x81R` \x81\x90R`@\x90\x81\x90 `\x03T\x90\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xD3\x8C)\xA1\x90a\x11\x14\x90\x86\x01\x86a5\xC6V[`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x111\x92\x91\x90a6'V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x11HW__\xFD[PZ\xF1\x15\x80\x15a\x11ZW=__>=_\xFD[PPPPa\x11\x8B`\x04T\x85a\x11n\x90a6iV[a\x11w\x86a7\x17V[`\x03T`\x01`\x01`\xA0\x1B\x03\x16\x92\x91\x90a\x14\xA3V[Pa\x12\x11a\x11\x9C` \x86\x01\x86a5\xC6V[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPP`@\x80Q``\x81\x01\x82R`\x04\x87\x01T\x81R`\x05\x87\x01Tc\xFF\xFF\xFF\xFF\x81\x16` \x83\x01Rd\x01\0\0\0\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91\x81\x01\x91\x90\x91R\x91Pa\x16>\x90PV[a\x12\x1E\x82`\x01\x01\x85a\x18?V[`\x04\x82\x01T`\x03\x83\x01T`\x02\x84\x01Ta\x12E\x92`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x92\x91\x16\x90a\x14UV[\x81T_\x90\x81R` \x81\x81R`@\x80\x83 \x83\x81U`\x01\x80\x82\x01\x80Tc\xFF\xFF\xFF\xFF\x19\x16\x90U`\x02\x82\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90U`\x03\x82\x01\x85\x90U`\x04\x82\x01\x85\x90U`\x05\x82\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90U`\x06\x90\x91\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90U\x88\x84R\x91\x82\x90R\x82 \x82\x81U\x91\x90\x82\x01\x81a\x13\x10\x82\x82a,;V[PPP`\x02\x81\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x81\x16\x90\x91U_`\x03\x83\x01\x81\x90U`\x04\x83\x01\x80T\x83\x16\x90U`\x05\x83\x01\x80T\x90\x92\x16\x90\x91U`\x06\x90\x91\x01U`@Q\x85\x81R\x7F\xC5w0\x9A\xCDy9\xCC/\x01\xF6\x7F\x07>\x1AT\x82$EL\xDD\xDCy\xB1a\xDB\x17\xB51^\x9F\x0C\x90` \x01`@Q\x80\x91\x03\x90\xA1PPPPPV[`@Q`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`$\x83\x01R\x83\x16`D\x82\x01R`d\x81\x01\x82\x90Ra\x14O\x90\x85\x90\x7F#\xB8r\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90`\x84\x01[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x93\x16\x92\x90\x92\x17\x90\x91Ra\x18\xC6V[PPPPV[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16`$\x82\x01R`D\x81\x01\x82\x90Ra\x14\x9E\x90\x84\x90\x7F\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90`d\x01a\x13\xEBV[PPPV[_a\x14\xB1\x83` \x01Qa\x19\xAAV[a\x14\xFDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FInvalid input vector provided\0\0\0`D\x82\x01R`d\x01a\x05EV[a\x15\n\x83`@\x01Qa\x1ADV[a\x15VW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FInvalid output vector provided\0\0`D\x82\x01R`d\x01a\x05EV[a\x15\x93\x83_\x01Q\x84` \x01Q\x85`@\x01Q\x86``\x01Q`@Q` \x01a\x15\x7F\x94\x93\x92\x91\x90a7\xA6V[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra\x1A\xD1V[\x90Pa\x15\xB5a\x15\xA5\x83`@\x01Qa\x1A\xF3V[\x83Q` \x85\x01Q\x84\x92\x91\x90a\x1A\xFFV[a\x16'W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`<`$\x82\x01R\x7FTx merkle proof is not valid for`D\x82\x01R\x7F provided header and tx hash\0\0\0\0`d\x82\x01R`\x84\x01a\x05EV[a\x166\x85\x85\x84`@\x01Qa\x1B:V[\x94\x93PPPPV[__a\x16I\x84a\x1E\x8AV[\x90\x92P\x90P`\x01\x82\x01a\x16\xC4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FRead overrun during VarInt parsi`D\x82\x01R\x7Fng\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05EV[_\x80a\x16\xD1\x84`\x01a5\xB3V[\x90P_a\x16\xE0\x86_\x01Qa\x1E\x9FV[\x90P_[\x84\x81\x10\x15a\x17\xD0W_a\x16\xF7\x89\x85a\x1FLV[\x90P_a\x17-a\x17\x07\x8B\x87a\x1FaV[`\xD8\x81\x90\x1Cc\xFF\0\xFF\0\x16b\xFF\0\xFF`\xE8\x92\x90\x92\x1C\x91\x90\x91\x16\x17`\x10\x81\x81\x1B\x91\x90\x1C\x17\x90V[\x90P\x81\x84\x14\x80\x15a\x17MWP\x80c\xFF\xFF\xFF\xFF\x16\x89` \x01Qc\xFF\xFF\xFF\xFF\x16\x14[\x15a\x17^WPPPPPPPPPPV[a\x17h\x8A\x86a\x1FwV[\x95P_\x19\x86\x03a\x17\xBAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FBad VarInt in scriptSig\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05EV[a\x17\xC4\x86\x86a5\xB3V[\x94PPP`\x01\x01a\x16\xE4V[P`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FTransaction does not spend the r`D\x82\x01R\x7Fequired utxo\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05EV[_\x82_\x01\x80Ta\x18N\x90a2\xADV[`@Qa\x18`\x92P\x85\x90` \x01a8\x15V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90Pa\x14O\x82\x80`@\x01\x90a\x18\x8B\x91\x90a5\xC6V[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RP\x85\x92Pa\x1F\xBD\x91PPV[_a\x19\x1A\x82`@Q\x80`@\x01`@R\x80` \x81R` \x01\x7FSafeERC20: low-level call failed\x81RP\x85`\x01`\x01`\xA0\x1B\x03\x16a![\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x80Q\x90\x91P\x15a\x14\x9EW\x80\x80` \x01\x90Q\x81\x01\x90a\x198\x91\x90a8\xDCV[a\x14\x9EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01R\x7Fot succeed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05EV[___a\x19\xB6\x84a\x1E\x8AV[\x90\x92P\x90P\x80\x15\x80a\x19\xC8WP_\x19\x82\x14[\x15a\x19\xD6WP_\x93\x92PPPV[_a\x19\xE2\x83`\x01a5\xB3V[\x90P_[\x82\x81\x10\x15a\x1A7W\x85Q\x82\x10a\x1A\x01WP_\x95\x94PPPPPV[_a\x1A\x0C\x87\x84a\x1FwV[\x90P_\x19\x81\x03a\x1A\"WP_\x96\x95PPPPPPV[a\x1A,\x81\x84a5\xB3V[\x92PP`\x01\x01a\x19\xE6V[P\x93Q\x90\x93\x14\x93\x92PPPV[___a\x1AP\x84a\x1E\x8AV[\x90\x92P\x90P\x80\x15\x80a\x1AbWP_\x19\x82\x14[\x15a\x1ApWP_\x93\x92PPPV[_a\x1A|\x83`\x01a5\xB3V[\x90P_[\x82\x81\x10\x15a\x1A7W\x85Q\x82\x10a\x1A\x9BWP_\x95\x94PPPPPV[_a\x1A\xA6\x87\x84a!iV[\x90P_\x19\x81\x03a\x1A\xBCWP_\x96\x95PPPPPPV[a\x1A\xC6\x81\x84a5\xB3V[\x92PP`\x01\x01a\x1A\x80V[_` _\x83Q` \x85\x01`\x02Z\xFAP` _` _`\x02Z\xFAPP_Q\x91\x90PV[`D\x81\x01Q_\x90a\x07$V[_\x83\x85\x14\x80\x15a\x1B\rWP\x81\x15[\x80\x15a\x1B\x18WP\x82Q\x15[\x15a\x1B%WP`\x01a\x166V[a\x1B1\x85\x84\x86\x85a!\xC9V[\x95\x94PPPPPV[_\x83`\x01`\x01`\xA0\x1B\x03\x16c\x117d\xBE`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1BwW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\x9B\x91\x90a8\xFBV[\x90P_\x84`\x01`\x01`\xA0\x1B\x03\x16c+\x97\xBE$`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B\xDAW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\xFE\x91\x90a8\xFBV[\x90P_\x80a\x1C\x13a\x1C\x0E\x86a\"nV[a\"yV[\x90P\x83\x81\x03a\x1C$W\x83\x91Pa\x1C\xA1V[\x82\x81\x03a\x1C3W\x82\x91Pa\x1C\xA1V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FNot at current or previous diffi`D\x82\x01R\x7Fculty\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05EV[_a\x1C\xAB\x86a\"\xA0V[\x90P_\x19\x81\x03a\x1D#W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FInvalid length of the headers ch`D\x82\x01R\x7Fain\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05EV[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x03a\x1D\x92W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01R\x7FInvalid headers chain\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05EV[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFD\x81\x03a\x1E\x01W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FInsufficient work in a header\0\0\0`D\x82\x01R`d\x01a\x05EV[a\x1E\x0B\x87\x84a9\x12V[\x81\x10\x15a\x1E\x80W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`3`$\x82\x01R\x7FInsufficient accumulated difficu`D\x82\x01R\x7Flty in header chain\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05EV[PPPPPPPPV[__a\x1E\x96\x83_a$\xC4V[\x91P\x91P\x91P\x91V[`@\x80Q` \x80\x82R\x81\x83\x01\x90\x92R_\x91\x82\x91\x90` \x82\x01\x81\x806\x837\x01\x90PP\x90P_[` \x81\x10\x15a\x1FAW\x83\x81` \x81\x10a\x1E\xDFWa\x1E\xDFa1kV[\x1A`\xF8\x1B\x82`\x01a\x1E\xF1\x84` a9)V[a\x1E\xFB\x91\x90a9)V[\x81Q\x81\x10a\x1F\x0BWa\x1F\x0Ba1kV[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81_\x1A\x90SP`\x01\x01a\x1E\xC4V[P` \x01Q\x92\x91PPV[_a\x1FZ\x83\x83\x01` \x01Q\x90V[\x93\x92PPPV[_a\x1FZa\x1Fp\x83` a5\xB3V[\x84\x90a\x1FLV[___a\x1F\x84\x85\x85a&;V[\x90\x92P\x90P`\x01\x82\x01a\x1F\x9CW_\x19\x92PPPa\x07$V[\x80a\x1F\xA8\x83`%a5\xB3V[a\x1F\xB2\x91\x90a5\xB3V[a\x1B1\x90`\x04a5\xB3V[`@\x80Q``\x81\x01\x82R_\x80\x82R` \x80\x83\x01\x82\x90R\x82\x84\x01\x82\x90R\x83Q\x80\x85\x01\x90\x94R\x81\x84R\x83\x01R\x90a\x1F\xF1\x84a\x1E\x8AV[` \x83\x01R\x80\x82R\x81a \x03\x82a1&V[\x90RP_\x80[\x82` \x01Q\x81\x10\x15a!\x05W\x82Q_\x90a $\x90\x88\x90a!iV[\x84Q\x90\x91P_\x90a 6\x90\x89\x90a&yV[\x90P_a D`\x08\x84a9)V[\x86Q\x90\x91P_\x90a V\x90`\x08a5\xB3V[\x8A\x81\x01` \x01\x83\x90 \x90\x91P\x80\x8A\x03a \x90W`\x01\x96P\x83\x89_\x01\x81\x81Qa ~\x91\x90a9<V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90RPa \xE0V[_a \x9E\x8C\x8A_\x01Qa&\xEFV[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a \xBFW`\x01`\x01`\xA0\x1B\x03\x81\x16` \x8B\x01R[_a \xCD\x8D\x8B_\x01Qa'\xCFV[\x90P\x80\x15a \xDDW`@\x8B\x01\x81\x90R[PP[\x84\x88_\x01\x81\x81Qa \xF1\x91\x90a5\xB3V[\x90RPP`\x01\x90\x94\x01\x93Pa \t\x92PPPV[P\x80a!SW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FNo output found for scriptPubKey`D\x82\x01R`d\x01a\x05EV[PP\x92\x91PPV[``a\x166\x84\x84_\x85a(\xAFV[_a!u\x82`\ta5\xB3V[\x83Q\x10\x15a!\x85WP_\x19a\x07$V[_\x80a!\x9B\x85a!\x96\x86`\x08a5\xB3V[a$\xC4V[\x90\x92P\x90P`\x01\x82\x01a!\xB3W_\x19\x92PPPa\x07$V[\x80a!\xBF\x83`\ta5\xB3V[a\x1B1\x91\x90a5\xB3V[_` \x84Qa!\xD8\x91\x90a9\x89V[\x15a!\xE4WP_a\x166V[\x83Q_\x03a!\xF3WP_a\x166V[\x81\x85_[\x86Q\x81\x10\x15a\"aWa\"\x0B`\x02\x84a9\x89V[`\x01\x03a\"/Wa\"(a\"\"\x88\x83\x01` \x01Q\x90V[\x83a)\xF3V[\x91Pa\"HV[a\"E\x82a\"@\x89\x84\x01` \x01Q\x90V[a)\xF3V[\x91P[`\x01\x92\x90\x92\x1C\x91a\"Z` \x82a5\xB3V[\x90Pa!\xF7V[P\x90\x93\x14\x95\x94PPPPPV[_a\x07$\x82_a)\xFEV[_a\x07${\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83a*\x97V[_`P\x82Qa\"\xAF\x91\x90a9\x89V[\x15a\"\xBCWP_\x19\x91\x90PV[P_\x80\x80[\x83Q\x81\x10\x15a$\xBDW\x80\x15a#\x08Wa\"\xDB\x84\x82\x84a*\xA2V[a#\x08WP\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x93\x92PPPV[_a#\x13\x85\x83a)\xFEV[\x90Pa#!\x85\x83`Pa*\xCBV[\x92P\x80a$d\x84_\x81\x90P`\x08\x81~\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\x16\x90\x1B`\x08\x82\x90\x1C~\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\x16\x17\x90P`\x10\x81}\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\x16\x90\x1B`\x10\x82\x90\x1C}\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\x16\x17\x90P` \x81{\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\x16\x90\x1B` \x82\x90\x1C{\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\x16\x17\x90P`@\x81w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x1B`@\x82\x90\x1Cw\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x17\x90P`\x80\x81\x90\x1B`\x80\x82\x90\x1C\x17\x90P\x91\x90PV[\x11\x15a$\x94WP\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFD\x94\x93PPPPV[a$\x9D\x81a\"yV[a$\xA7\x90\x85a5\xB3V[\x93Pa$\xB6\x90P`P\x82a5\xB3V[\x90Pa\"\xC1V[PP\x91\x90PV[___a$\xD1\x85\x85a*\xF0V[\x90P\x80`\xFF\x16_\x03a%\x04W_\x85\x85\x81Q\x81\x10a$\xF0Wa$\xF0a1kV[\x01` \x01Q\x90\x93P`\xF8\x1C\x91Pa&4\x90PV[\x83a%\x10\x82`\x01a9\x9CV[`\xFF\x16a%\x1D\x91\x90a5\xB3V[\x85Q\x10\x15a%2W_\x19_\x92P\x92PPa&4V[_\x81`\xFF\x16`\x02\x03a%uWa%ja%Va%O\x87`\x01a5\xB3V[\x88\x90a\x1FLV[b\xFF\xFF\0`\xE8\x82\x90\x1C\x16`\xF8\x91\x90\x91\x1C\x17\x90V[a\xFF\xFF\x16\x90Pa&*V[\x81`\xFF\x16`\x04\x03a%\x9EWa%\x91a\x17\x07a%O\x87`\x01a5\xB3V[c\xFF\xFF\xFF\xFF\x16\x90Pa&*V[\x81`\xFF\x16`\x08\x03a&*Wa&\x1Da%\xBAa%O\x87`\x01a5\xB3V[`\xC0\x1Cd\xFF\0\0\0\xFF`\x08\x82\x81\x1C\x91\x82\x16e\xFF\0\0\0\xFF\0\x93\x90\x91\x1B\x92\x83\x16\x17`\x10\x90\x81\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16f\xFF\0\xFF\0\xFF\0\xFF\x92\x90\x92\x16g\xFF\0\xFF\0\xFF\0\xFF\0\x90\x93\x16\x92\x90\x92\x17\x90\x91\x1Ce\xFF\xFF\0\0\xFF\xFF\x16\x17` \x81\x81\x1C\x91\x90\x1B\x17\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P[`\xFF\x90\x91\x16\x92P\x90P[\x92P\x92\x90PV[_\x80a&H\x83`%a5\xB3V[\x84Q\x10\x15a&[WP_\x19\x90P_a&4V[_\x80a&l\x86a!\x96\x87`$a5\xB3V[\x90\x97\x90\x96P\x94PPPPPV[_\x80a&\x85\x84\x84a\x1FLV[`\xC0\x1C\x90P_a\x1B1\x82d\xFF\0\0\0\xFF`\x08\x82\x81\x1C\x91\x82\x16e\xFF\0\0\0\xFF\0\x93\x90\x91\x1B\x92\x83\x16\x17`\x10\x90\x81\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16f\xFF\0\xFF\0\xFF\0\xFF\x92\x90\x92\x16g\xFF\0\xFF\0\xFF\0\xFF\0\x90\x93\x16\x92\x90\x92\x17\x90\x91\x1Ce\xFF\xFF\0\0\xFF\xFF\x16\x17` \x81\x81\x1C\x91\x90\x1B\x17\x90V[_\x82a&\xFC\x83`\ta5\xB3V[\x81Q\x81\x10a'\x0CWa'\x0Ca1kV[` \x91\x01\x01Q\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x7Fj\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x14a'aWP_a\x07$V[_\x83a'n\x84`\na5\xB3V[\x81Q\x81\x10a'~Wa'~a1kV[\x01` \x01Q\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x91P`\xF8\x1C`\x14\x03a'\xC8W_a'\xBE\x84`\x0Ba5\xB3V[\x85\x01`\x14\x01Q\x92PP[P\x92\x91PPV[_\x82a'\xDC\x83`\ta5\xB3V[\x81Q\x81\x10a'\xECWa'\xECa1kV[` \x91\x01\x01Q\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x7Fj\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x14a(AWP_a\x07$V[_\x83a(N\x84`\na5\xB3V[\x81Q\x81\x10a(^Wa(^a1kV[\x01` \x90\x81\x01Q\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x92P`\xF8\x1C\x90\x03a'\xC8W_a(\x9F\x84`\x0Ba5\xB3V[\x85\x01` \x01Q\x92PPP\x92\x91PPV[``\x82G\x10\x15a)'W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FAddress: insufficient balance fo`D\x82\x01R\x7Fr call\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05EV[`\x01`\x01`\xA0\x1B\x03\x85\x16;a)~W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x05EV[__\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@Qa)\x99\x91\x90a9\xB5V[_`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80_\x81\x14a)\xD3W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a)\xD8V[``\x91P[P\x91P\x91Pa)\xE8\x82\x82\x86a+tV[\x97\x96PPPPPPPV[_a\x1FZ\x83\x83a+\xADV[_\x80a*\x15a*\x0E\x84`Ha5\xB3V[\x85\x90a\x1FLV[`\xE8\x1C\x90P_\x84a*'\x85`Ka5\xB3V[\x81Q\x81\x10a*7Wa*7a1kV[\x01` \x01Q`\xF8\x1C\x90P_a*i\x83_`\x10\x82b\xFF\xFF\xFF\x16\x90\x1C\x82a\xFF\0\x16`\x10\x84b\xFF\xFF\xFF\x16\x90\x1B\x17\x17\x90P\x91\x90PV[b\xFF\xFF\xFF\x16\x90P_a*|`\x03\x84a9\xC0V[`\xFF\x16\x90Pa*\x8D\x81a\x01\0a:\xBCV[a)\xE8\x90\x83a9\x12V[_a\x1FZ\x82\x84a:\xC7V[_\x80a*\xAE\x85\x85a+\xD4V[\x90P\x82\x81\x14a*\xC0W_\x91PPa\x1FZV[P`\x01\x94\x93PPPPV[_` _\x83\x85` \x01\x87\x01`\x02Z\xFAP` _` _`\x02Z\xFAPP_Q\x93\x92PPPV[_\x82\x82\x81Q\x81\x10a+\x03Wa+\x03a1kV[\x01` \x01Q`\xF8\x1C`\xFF\x03a+\x1AWP`\x08a\x07$V[\x82\x82\x81Q\x81\x10a+,Wa+,a1kV[\x01` \x01Q`\xF8\x1C`\xFE\x03a+CWP`\x04a\x07$V[\x82\x82\x81Q\x81\x10a+UWa+Ua1kV[\x01` \x01Q`\xF8\x1C`\xFD\x03a+lWP`\x02a\x07$V[P_\x92\x91PPV[``\x83\x15a+\x83WP\x81a\x1FZV[\x82Q\x15a+\x93W\x82Q\x80\x84` \x01\xFD[\x81`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05E\x91\x90a:\xDAV[_\x82_R\x81` R` _`@_`\x02Z\xFAP` _` _`\x02Z\xFAPP_Q\x92\x91PPV[_a\x1FZa+\xE3\x83`\x04a5\xB3V[\x84\x01` \x01Q\x90V[`@Q\x80`\xE0\x01`@R\x80_\x81R` \x01a,\x13`@Q\x80` \x01`@R\x80``\x81RP\x90V[\x81R_` \x82\x01\x81\x90R`@\x82\x01\x81\x90R``\x82\x01\x81\x90R`\x80\x82\x01\x81\x90R`\xA0\x90\x91\x01R\x90V[P\x80Ta,G\x90a2\xADV[_\x82U\x80`\x1F\x10a,VWPPV[`\x1F\x01` \x90\x04\x90_R` _ \x90\x81\x01\x90a,r\x91\x90a,uV[PV[[\x80\x82\x11\x15a,\x89W_\x81U`\x01\x01a,vV[P\x90V[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15a,\xBDW\x81Q\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01a,\x9FV[P\x93\x94\x93PPPPV[`@\x80\x82R\x83Q\x90\x82\x01\x81\x90R_\x90` \x85\x01\x90``\x84\x01\x90\x83[\x81\x81\x10\x15a-\x90W\x83Qa-\x07\x84\x82Q\x80Q\x82R` \x90\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x91\x01RV[`\x01`\x01`\xA0\x1B\x03` \x82\x01Q\x16`@\x85\x01R`@\x81\x01Q``\x85\x01R``\x81\x01Qa-Z`\x80\x86\x01\x82\x80Q\x82R` \x80\x82\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x83\x01R`@\x90\x81\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91\x01RV[P`\x80\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\xE0\x85\x01R`\xA0\x01Q\x15\x15a\x01\0\x84\x01R` \x93\x90\x93\x01\x92a\x01 \x90\x92\x01\x91`\x01\x01a,\xE2V[PP\x83\x81\x03` \x85\x01Ra-\xA4\x81\x86a,\x8DV[\x96\x95PPPPPPV[__`@\x83\x85\x03\x12\x15a-\xBFW__\xFD[\x825\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a-\xDCW__\xFD[\x83\x01` \x81\x86\x03\x12\x15a-\xEDW__\xFD[\x80\x91PP\x92P\x92\x90PV[_` \x82\x84\x03\x12\x15a.\x08W__\xFD[P5\x91\x90PV[\x86Q\x81R` \x80\x88\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x82\x01Ra\x01 \x81\x01`\x01`\x01`\xA0\x1B\x03\x87\x16`@\x83\x01R\x85``\x83\x01Ra.n`\x80\x83\x01\x86\x80Q\x82R` \x80\x82\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x83\x01R`@\x90\x81\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91\x01RV[`\x01`\x01`\xA0\x1B\x03\x84\x16`\xE0\x83\x01R\x82\x15\x15a\x01\0\x83\x01R\x97\x96PPPPPPPV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[_\x81Q` \x84Ra\x166` \x85\x01\x82a.\x91V[_`@\x82\x01`@\x83R\x80\x85Q\x80\x83R``\x85\x01\x91P``\x81`\x05\x1B\x86\x01\x01\x92P` \x87\x01_[\x82\x81\x10\x15a/\xB2W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA0\x87\x86\x03\x01\x84R\x81Q\x80Q\x86R` \x81\x01Q`\xE0` \x88\x01Ra/G`\xE0\x88\x01\x82a.\xBFV[\x90P`\x01`\x01`\xA0\x1B\x03`@\x83\x01Q\x16`@\x88\x01R``\x82\x01Q``\x88\x01R`\x01`\x01`\xA0\x1B\x03`\x80\x83\x01Q\x16`\x80\x88\x01R`\x01`\x01`\xA0\x1B\x03`\xA0\x83\x01Q\x16`\xA0\x88\x01R`\xC0\x82\x01Q`\xC0\x88\x01R\x80\x96PPP` \x82\x01\x91P` \x84\x01\x93P`\x01\x81\x01\x90Pa.\xF9V[PPPP\x82\x81\x03` \x84\x01Ra\x1B1\x81\x85a,\x8DV[_``\x82\x84\x03\x12\x15a/\xD8W__\xFD[P\x91\x90PV[____\x84\x86\x03`\xE0\x81\x12\x15a/\xF2W__\xFD[`@\x81\x12\x15a/\xFFW__\xFD[P\x84\x93Pa0\x10\x86`@\x87\x01a/\xC8V[\x92P`\xA0\x85\x015`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a0+W__\xFD[\x93\x96\x92\x95P\x92\x93`\xC0\x015\x92PPV[\x87\x81R`\xE0` \x82\x01R_a0S`\xE0\x83\x01\x89a.\xBFV[`\x01`\x01`\xA0\x1B\x03\x97\x88\x16`@\x84\x01R``\x83\x01\x96\x90\x96RP\x92\x85\x16`\x80\x84\x01R\x93\x16`\xA0\x82\x01R`\xC0\x01\x91\x90\x91R\x92\x91PPV[___``\x84\x86\x03\x12\x15a0\x9AW__\xFD[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a0\xB7W__\xFD[\x84\x01`\x80\x81\x87\x03\x12\x15a0\xC8W__\xFD[\x91P`@\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a0\xE3W__\xFD[a0\xEF\x86\x82\x87\x01a/\xC8V[\x91PP\x92P\x92P\x92V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[__\x19\x82\x03a17Wa17a0\xF9V[P`\x01\x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a1\xBBWa1\xBBa1>V[`@R\x90V[_\x82`\x1F\x83\x01\x12a1\xD0W__\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a1\xEAWa1\xEAa1>V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a2\x19Wa2\x19a1>V[`@R\x81\x81R\x83\x82\x01` \x01\x85\x10\x15a20W__\xFD[\x81` \x85\x01` \x83\x017_\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[_` \x826\x03\x12\x15a2\\W__\xFD[`@Q` \x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a2\x7FWa2\x7Fa1>V[`@R\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a2\x98W__\xFD[a2\xA46\x82\x86\x01a1\xC1V[\x82RP\x92\x91PPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a2\xC1W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a/\xD8W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[`\x1F\x82\x11\x15a\x14\x9EW\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a3\x1DWP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a3<W_\x81U`\x01\x01a3)V[PPPPPV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a3]Wa3]a1>V[a3q\x81a3k\x84Ta2\xADV[\x84a2\xF8V[` `\x1F\x82\x11`\x01\x81\x14a3\xA3W_\x83\x15a3\x8CWP\x84\x82\x01Q[_\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua3<V[_\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a3\xD2W\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a3\xB2V[P\x84\x82\x10\x15a3\xEFW\x86\x84\x01Q_\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[\x81\x83R\x81\x81` \x85\x017P_` \x82\x84\x01\x01R_` `\x1F\x19`\x1F\x84\x01\x16\x84\x01\x01\x90P\x92\x91PPV[``\x81R_\x845\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x866\x03\x01\x81\x12a4]W__\xFD[\x85\x01` \x81\x01\x905g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a4yW__\xFD[\x806\x03\x82\x13\x15a4\x87W__\xFD[` ``\x85\x01Ra4\x9C`\x80\x85\x01\x82\x84a3\xFEV[`\x01`\x01`\xA0\x1B\x03\x96\x90\x96\x16` \x85\x01RPPP`@\x01R\x91\x90PV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a4\xCCW__\xFD[\x91\x90PV[_`@\x82\x84\x03\x12\x80\x15a4\xE2W__\xFD[P`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a5\x06Wa5\x06a1>V[`@R\x825\x81Ra5\x19` \x84\x01a4\xB9V[` \x82\x01R\x93\x92PPPV[_``\x82\x84\x03\x12\x80\x15a56W__\xFD[Pa5?a1\x98V[\x825\x81Ra5O` \x84\x01a4\xB9V[` \x82\x01R`@\x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a5nW__\xFD[`@\x82\x01R\x93\x92PPPV[\x835\x81R`\x80\x81\x01c\xFF\xFF\xFF\xFFa5\x93` \x87\x01a4\xB9V[\x16` \x83\x01R`\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16`@\x82\x01R``\x01R\x91\x90PV[\x80\x82\x01\x80\x82\x11\x15a\x07$Wa\x07$a0\xF9V[__\x835\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x846\x03\x01\x81\x12a5\xF9W__\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a6\x13W__\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a&4W__\xFD[` \x81R_a\x166` \x83\x01\x84\x86a3\xFEV[\x805\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x81\x14a4\xCCW__\xFD[_`\x80\x826\x03\x12\x15a6yW__\xFD[`@Q`\x80\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a6\x9CWa6\x9Ca1>V[`@Ra6\xA8\x83a6:V[\x81R` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a6\xC3W__\xFD[a6\xCF6\x82\x86\x01a1\xC1V[` \x83\x01RP`@\x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a6\xEEW__\xFD[a6\xFA6\x82\x86\x01a1\xC1V[`@\x83\x01RPa7\x0C``\x84\x01a6:V[``\x82\x01R\x92\x91PPV[_``\x826\x03\x12\x15a7'W__\xFD[a7/a1\x98V[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a7EW__\xFD[a7Q6\x82\x86\x01a1\xC1V[\x82RP` \x83\x81\x015\x90\x82\x01R`@\x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a7wW__\xFD[a7\x836\x82\x86\x01a1\xC1V[`@\x83\x01RP\x92\x91PPV[_\x81Q\x80` \x84\x01\x85^_\x93\x01\x92\x83RP\x90\x91\x90PV[\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85\x16\x81R_a7\xE2a7\xDC`\x04\x84\x01\x87a7\x8FV[\x85a7\x8FV[\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x93\x90\x93\x16\x83RPP`\x04\x01\x93\x92PPPV[\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83`\xF8\x1B\x16\x81R__\x83Ta8J\x81a2\xADV[`\x01\x82\x16\x80\x15a8aW`\x01\x81\x14a8\x9AWa8\xD0V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x83\x16`\x01\x87\x01R`\x01\x82\x15\x15\x83\x02\x87\x01\x01\x93Pa8\xD0V[\x86_R` _ _[\x83\x81\x10\x15a8\xC5W\x81T`\x01\x82\x8A\x01\x01R`\x01\x82\x01\x91P` \x81\x01\x90Pa8\xA3V[PP`\x01\x82\x87\x01\x01\x93P[P\x91\x96\x95PPPPPPV[_` \x82\x84\x03\x12\x15a8\xECW__\xFD[\x81Q\x80\x15\x15\x81\x14a\x1FZW__\xFD[_` \x82\x84\x03\x12\x15a9\x0BW__\xFD[PQ\x91\x90PV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x07$Wa\x07$a0\xF9V[\x81\x81\x03\x81\x81\x11\x15a\x07$Wa\x07$a0\xF9V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x07$Wa\x07$a0\xF9V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_\x82a9\x97Wa9\x97a9\\V[P\x06\x90V[`\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x07$Wa\x07$a0\xF9V[_a\x1FZ\x82\x84a7\x8FV[`\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x07$Wa\x07$a0\xF9V[`\x01\x81[`\x01\x84\x11\x15a:\x14W\x80\x85\x04\x81\x11\x15a9\xF8Wa9\xF8a0\xF9V[`\x01\x84\x16\x15a:\x06W\x90\x81\x02\x90[`\x01\x93\x90\x93\x1C\x92\x80\x02a9\xDDV[\x93P\x93\x91PPV[_\x82a:*WP`\x01a\x07$V[\x81a:6WP_a\x07$V[\x81`\x01\x81\x14a:LW`\x02\x81\x14a:VWa:rV[`\x01\x91PPa\x07$V[`\xFF\x84\x11\x15a:gWa:ga0\xF9V[PP`\x01\x82\x1Ba\x07$V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a:\x95WP\x81\x81\na\x07$V[a:\xA1_\x19\x84\x84a9\xD9V[\x80_\x19\x04\x82\x11\x15a:\xB4Wa:\xB4a0\xF9V[\x02\x93\x92PPPV[_a\x1FZ\x83\x83a:\x1CV[_\x82a:\xD5Wa:\xD5a9\\V[P\x04\x90V[` \x81R_a\x1FZ` \x83\x01\x84a.\x91V\xFE\xA2dipfsX\"\x12 m\x88\x98\"\x0E\x0F\xAFP\xC7}\x86\xAC\xD2\xDC\xC7}\x7Fs\x18\xD3)F\xDD\xA6\xB1\xF7\xA5\xB8\x89_E\x18dsolcC\0\x08\x1B\x003",
    );
    /**```solidity
struct AcceptedOrdinalSellOrder { uint256 orderId; BitcoinAddress bitcoinAddress; address ercToken; uint256 ercAmount; address requester; address acceptor; uint256 acceptTime; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AcceptedOrdinalSellOrder {
        pub orderId: alloy::sol_types::private::primitives::aliases::U256,
        pub bitcoinAddress: <BitcoinAddress as alloy::sol_types::SolType>::RustType,
        pub ercToken: alloy::sol_types::private::Address,
        pub ercAmount: alloy::sol_types::private::primitives::aliases::U256,
        pub requester: alloy::sol_types::private::Address,
        pub acceptor: alloy::sol_types::private::Address,
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
        impl ::core::convert::From<AcceptedOrdinalSellOrder>
        for UnderlyingRustTuple<'_> {
            fn from(value: AcceptedOrdinalSellOrder) -> Self {
                (
                    value.orderId,
                    value.bitcoinAddress,
                    value.ercToken,
                    value.ercAmount,
                    value.requester,
                    value.acceptor,
                    value.acceptTime,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for AcceptedOrdinalSellOrder {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    orderId: tuple.0,
                    bitcoinAddress: tuple.1,
                    ercToken: tuple.2,
                    ercAmount: tuple.3,
                    requester: tuple.4,
                    acceptor: tuple.5,
                    acceptTime: tuple.6,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for AcceptedOrdinalSellOrder {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for AcceptedOrdinalSellOrder {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.orderId),
                    <BitcoinAddress as alloy_sol_types::SolType>::tokenize(
                        &self.bitcoinAddress,
                    ),
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
                        &self.acceptor,
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
        impl alloy_sol_types::SolType for AcceptedOrdinalSellOrder {
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
        impl alloy_sol_types::SolStruct for AcceptedOrdinalSellOrder {
            const NAME: &'static str = "AcceptedOrdinalSellOrder";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "AcceptedOrdinalSellOrder(uint256 orderId,BitcoinAddress bitcoinAddress,address ercToken,uint256 ercAmount,address requester,address acceptor,uint256 acceptTime)",
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
                            &self.acceptor,
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
        impl alloy_sol_types::EventTopic for AcceptedOrdinalSellOrder {
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
                        &rust.acceptor,
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
                    &rust.acceptor,
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
struct OrdinalId { bytes32 txId; uint32 index; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OrdinalId {
        pub txId: alloy::sol_types::private::FixedBytes<32>,
        pub index: u32,
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
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::Uint<32>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>, u32);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<OrdinalId> for UnderlyingRustTuple<'_> {
            fn from(value: OrdinalId) -> Self {
                (value.txId, value.index)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OrdinalId {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    txId: tuple.0,
                    index: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for OrdinalId {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for OrdinalId {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.txId),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.index),
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
        impl alloy_sol_types::SolType for OrdinalId {
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
        impl alloy_sol_types::SolStruct for OrdinalId {
            const NAME: &'static str = "OrdinalId";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "OrdinalId(bytes32 txId,uint32 index)",
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
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.txId)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.index)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for OrdinalId {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.txId)
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.index)
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
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.txId,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.index,
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
struct OrdinalSellOrder { OrdinalId ordinalID; address sellToken; uint256 sellAmount; BitcoinTx.UTXO utxo; address requester; bool isOrderAccepted; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OrdinalSellOrder {
        pub ordinalID: <OrdinalId as alloy::sol_types::SolType>::RustType,
        pub sellToken: alloy::sol_types::private::Address,
        pub sellAmount: alloy::sol_types::private::primitives::aliases::U256,
        pub utxo: <BitcoinTx::UTXO as alloy::sol_types::SolType>::RustType,
        pub requester: alloy::sol_types::private::Address,
        pub isOrderAccepted: bool,
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
            OrdinalId,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<256>,
            BitcoinTx::UTXO,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Bool,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            <OrdinalId as alloy::sol_types::SolType>::RustType,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::primitives::aliases::U256,
            <BitcoinTx::UTXO as alloy::sol_types::SolType>::RustType,
            alloy::sol_types::private::Address,
            bool,
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
        impl ::core::convert::From<OrdinalSellOrder> for UnderlyingRustTuple<'_> {
            fn from(value: OrdinalSellOrder) -> Self {
                (
                    value.ordinalID,
                    value.sellToken,
                    value.sellAmount,
                    value.utxo,
                    value.requester,
                    value.isOrderAccepted,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OrdinalSellOrder {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    ordinalID: tuple.0,
                    sellToken: tuple.1,
                    sellAmount: tuple.2,
                    utxo: tuple.3,
                    requester: tuple.4,
                    isOrderAccepted: tuple.5,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for OrdinalSellOrder {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for OrdinalSellOrder {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <OrdinalId as alloy_sol_types::SolType>::tokenize(&self.ordinalID),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.sellToken,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.sellAmount),
                    <BitcoinTx::UTXO as alloy_sol_types::SolType>::tokenize(&self.utxo),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.requester,
                    ),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.isOrderAccepted,
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
        impl alloy_sol_types::SolType for OrdinalSellOrder {
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
        impl alloy_sol_types::SolStruct for OrdinalSellOrder {
            const NAME: &'static str = "OrdinalSellOrder";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "OrdinalSellOrder(OrdinalId ordinalID,address sellToken,uint256 sellAmount,BitcoinTx.UTXO utxo,address requester,bool isOrderAccepted)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(2);
                components
                    .push(<OrdinalId as alloy_sol_types::SolStruct>::eip712_root_type());
                components
                    .extend(
                        <OrdinalId as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
                    .push(
                        <BitcoinTx::UTXO as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <BitcoinTx::UTXO as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <OrdinalId as alloy_sol_types::SolType>::eip712_data_word(
                            &self.ordinalID,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.sellToken,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.sellAmount)
                        .0,
                    <BitcoinTx::UTXO as alloy_sol_types::SolType>::eip712_data_word(
                            &self.utxo,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.requester,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::eip712_data_word(
                            &self.isOrderAccepted,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for OrdinalSellOrder {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <OrdinalId as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.ordinalID,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.sellToken,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.sellAmount,
                    )
                    + <BitcoinTx::UTXO as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.utxo,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.requester,
                    )
                    + <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.isOrderAccepted,
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
                <OrdinalId as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.ordinalID,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.sellToken,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.sellAmount,
                    out,
                );
                <BitcoinTx::UTXO as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.utxo,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.requester,
                    out,
                );
                <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.isOrderAccepted,
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
    /**Event with signature `acceptOrdinalSellOrderEvent(uint256,uint256,(bytes),address,uint256)` and selector `0xfe350ff9ccadd1b7c26b5f96dd078d08a877c8f37d506931ecd8f2bdbd51b6f2`.
```solidity
event acceptOrdinalSellOrderEvent(uint256 indexed id, uint256 indexed acceptId, BitcoinAddress bitcoinAddress, address ercToken, uint256 ercAmount);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct acceptOrdinalSellOrderEvent {
        #[allow(missing_docs)]
        pub id: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub acceptId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub bitcoinAddress: <BitcoinAddress as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub ercToken: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub ercAmount: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for acceptOrdinalSellOrderEvent {
            type DataTuple<'a> = (
                BitcoinAddress,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "acceptOrdinalSellOrderEvent(uint256,uint256,(bytes),address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                254u8,
                53u8,
                15u8,
                249u8,
                204u8,
                173u8,
                209u8,
                183u8,
                194u8,
                107u8,
                95u8,
                150u8,
                221u8,
                7u8,
                141u8,
                8u8,
                168u8,
                119u8,
                200u8,
                243u8,
                125u8,
                80u8,
                105u8,
                49u8,
                236u8,
                216u8,
                242u8,
                189u8,
                189u8,
                81u8,
                182u8,
                242u8,
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
                    ercToken: data.1,
                    ercAmount: data.2,
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.ercToken,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.ercAmount),
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
        impl alloy_sol_types::private::IntoLogData for acceptOrdinalSellOrderEvent {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&acceptOrdinalSellOrderEvent> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &acceptOrdinalSellOrderEvent,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `cancelAcceptedOrdinalSellOrderEvent(uint256)` and selector `0x9c216a4617d6c03dc7cbd9632166f1c5c9ef41f9ee86bf3b83f671c005107704`.
```solidity
event cancelAcceptedOrdinalSellOrderEvent(uint256 id);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct cancelAcceptedOrdinalSellOrderEvent {
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
        impl alloy_sol_types::SolEvent for cancelAcceptedOrdinalSellOrderEvent {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "cancelAcceptedOrdinalSellOrderEvent(uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                156u8,
                33u8,
                106u8,
                70u8,
                23u8,
                214u8,
                192u8,
                61u8,
                199u8,
                203u8,
                217u8,
                99u8,
                33u8,
                102u8,
                241u8,
                197u8,
                201u8,
                239u8,
                65u8,
                249u8,
                238u8,
                134u8,
                191u8,
                59u8,
                131u8,
                246u8,
                113u8,
                192u8,
                5u8,
                16u8,
                119u8,
                4u8,
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
        impl alloy_sol_types::private::IntoLogData
        for cancelAcceptedOrdinalSellOrderEvent {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&cancelAcceptedOrdinalSellOrderEvent>
        for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &cancelAcceptedOrdinalSellOrderEvent,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `placeOrdinalSellOrderEvent(uint256,(bytes32,uint32),address,uint256)` and selector `0xfb2d3310e3e79578ac507cdbdb32e52581dbc17be04e5197d3b7c522735fb9e4`.
```solidity
event placeOrdinalSellOrderEvent(uint256 indexed orderId, OrdinalId ordinalID, address sellToken, uint256 sellAmount);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct placeOrdinalSellOrderEvent {
        #[allow(missing_docs)]
        pub orderId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub ordinalID: <OrdinalId as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub sellToken: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub sellAmount: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for placeOrdinalSellOrderEvent {
            type DataTuple<'a> = (
                OrdinalId,
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
            const SIGNATURE: &'static str = "placeOrdinalSellOrderEvent(uint256,(bytes32,uint32),address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                251u8,
                45u8,
                51u8,
                16u8,
                227u8,
                231u8,
                149u8,
                120u8,
                172u8,
                80u8,
                124u8,
                219u8,
                219u8,
                50u8,
                229u8,
                37u8,
                129u8,
                219u8,
                193u8,
                123u8,
                224u8,
                78u8,
                81u8,
                151u8,
                211u8,
                183u8,
                197u8,
                34u8,
                115u8,
                95u8,
                185u8,
                228u8,
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
                    ordinalID: data.0,
                    sellToken: data.1,
                    sellAmount: data.2,
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
                    <OrdinalId as alloy_sol_types::SolType>::tokenize(&self.ordinalID),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.sellToken,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.sellAmount),
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
        impl alloy_sol_types::private::IntoLogData for placeOrdinalSellOrderEvent {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&placeOrdinalSellOrderEvent> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &placeOrdinalSellOrderEvent,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `proofOrdinalSellOrderEvent(uint256)` and selector `0xc577309acd7939cc2f01f67f073e1a548224454cdddc79b161db17b5315e9f0c`.
```solidity
event proofOrdinalSellOrderEvent(uint256 id);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct proofOrdinalSellOrderEvent {
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
        impl alloy_sol_types::SolEvent for proofOrdinalSellOrderEvent {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "proofOrdinalSellOrderEvent(uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                197u8,
                119u8,
                48u8,
                154u8,
                205u8,
                121u8,
                57u8,
                204u8,
                47u8,
                1u8,
                246u8,
                127u8,
                7u8,
                62u8,
                26u8,
                84u8,
                130u8,
                36u8,
                69u8,
                76u8,
                221u8,
                220u8,
                121u8,
                177u8,
                97u8,
                219u8,
                23u8,
                181u8,
                49u8,
                94u8,
                159u8,
                12u8,
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
        impl alloy_sol_types::private::IntoLogData for proofOrdinalSellOrderEvent {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&proofOrdinalSellOrderEvent> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &proofOrdinalSellOrderEvent,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `withdrawOrdinalSellOrderEvent(uint256)` and selector `0xb35b3fe4daaf6cc66eb8bd413e9ab54449e766f6d46125cc58f255694a0e847e`.
```solidity
event withdrawOrdinalSellOrderEvent(uint256 id);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct withdrawOrdinalSellOrderEvent {
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
        impl alloy_sol_types::SolEvent for withdrawOrdinalSellOrderEvent {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "withdrawOrdinalSellOrderEvent(uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                179u8,
                91u8,
                63u8,
                228u8,
                218u8,
                175u8,
                108u8,
                198u8,
                110u8,
                184u8,
                189u8,
                65u8,
                62u8,
                154u8,
                181u8,
                68u8,
                73u8,
                231u8,
                102u8,
                246u8,
                212u8,
                97u8,
                37u8,
                204u8,
                88u8,
                242u8,
                85u8,
                105u8,
                74u8,
                14u8,
                132u8,
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
        impl alloy_sol_types::private::IntoLogData for withdrawOrdinalSellOrderEvent {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&withdrawOrdinalSellOrderEvent> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &withdrawOrdinalSellOrderEvent,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Constructor`.
```solidity
constructor(address _relay);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        pub _relay: alloy::sol_types::private::Address,
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
                    (value._relay,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _relay: tuple.0 }
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
                        &self._relay,
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
    /**Function with signature `acceptOrdinalSellOrder(uint256,(bytes))` and selector `0x2814a1cd`.
```solidity
function acceptOrdinalSellOrder(uint256 id, BitcoinAddress memory bitcoinAddress) external returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct acceptOrdinalSellOrderCall {
        pub id: alloy::sol_types::private::primitives::aliases::U256,
        pub bitcoinAddress: <BitcoinAddress as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`acceptOrdinalSellOrder(uint256,(bytes))`](acceptOrdinalSellOrderCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct acceptOrdinalSellOrderReturn {
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
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                <BitcoinAddress as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<acceptOrdinalSellOrderCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: acceptOrdinalSellOrderCall) -> Self {
                    (value.id, value.bitcoinAddress)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for acceptOrdinalSellOrderCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        id: tuple.0,
                        bitcoinAddress: tuple.1,
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
            impl ::core::convert::From<acceptOrdinalSellOrderReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: acceptOrdinalSellOrderReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for acceptOrdinalSellOrderReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for acceptOrdinalSellOrderCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                BitcoinAddress,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = acceptOrdinalSellOrderReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "acceptOrdinalSellOrder(uint256,(bytes))";
            const SELECTOR: [u8; 4] = [40u8, 20u8, 161u8, 205u8];
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
    /**Function with signature `acceptedOrdinalSellOrders(uint256)` and selector `0xdb82d5fa`.
```solidity
function acceptedOrdinalSellOrders(uint256) external view returns (uint256 orderId, BitcoinAddress memory bitcoinAddress, address ercToken, uint256 ercAmount, address requester, address acceptor, uint256 acceptTime);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct acceptedOrdinalSellOrdersCall {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`acceptedOrdinalSellOrders(uint256)`](acceptedOrdinalSellOrdersCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct acceptedOrdinalSellOrdersReturn {
        pub orderId: alloy::sol_types::private::primitives::aliases::U256,
        pub bitcoinAddress: <BitcoinAddress as alloy::sol_types::SolType>::RustType,
        pub ercToken: alloy::sol_types::private::Address,
        pub ercAmount: alloy::sol_types::private::primitives::aliases::U256,
        pub requester: alloy::sol_types::private::Address,
        pub acceptor: alloy::sol_types::private::Address,
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
            impl ::core::convert::From<acceptedOrdinalSellOrdersCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: acceptedOrdinalSellOrdersCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for acceptedOrdinalSellOrdersCall {
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
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                <BitcoinAddress as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<acceptedOrdinalSellOrdersReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: acceptedOrdinalSellOrdersReturn) -> Self {
                    (
                        value.orderId,
                        value.bitcoinAddress,
                        value.ercToken,
                        value.ercAmount,
                        value.requester,
                        value.acceptor,
                        value.acceptTime,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for acceptedOrdinalSellOrdersReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        orderId: tuple.0,
                        bitcoinAddress: tuple.1,
                        ercToken: tuple.2,
                        ercAmount: tuple.3,
                        requester: tuple.4,
                        acceptor: tuple.5,
                        acceptTime: tuple.6,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for acceptedOrdinalSellOrdersCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = acceptedOrdinalSellOrdersReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                BitcoinAddress,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "acceptedOrdinalSellOrders(uint256)";
            const SELECTOR: [u8; 4] = [219u8, 130u8, 213u8, 250u8];
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
    /**Function with signature `cancelAcceptedOrdinalSellOrder(uint256)` and selector `0x73787155`.
```solidity
function cancelAcceptedOrdinalSellOrder(uint256 id) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct cancelAcceptedOrdinalSellOrderCall {
        pub id: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`cancelAcceptedOrdinalSellOrder(uint256)`](cancelAcceptedOrdinalSellOrderCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct cancelAcceptedOrdinalSellOrderReturn {}
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
            impl ::core::convert::From<cancelAcceptedOrdinalSellOrderCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: cancelAcceptedOrdinalSellOrderCall) -> Self {
                    (value.id,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for cancelAcceptedOrdinalSellOrderCall {
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
            impl ::core::convert::From<cancelAcceptedOrdinalSellOrderReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: cancelAcceptedOrdinalSellOrderReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for cancelAcceptedOrdinalSellOrderReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for cancelAcceptedOrdinalSellOrderCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = cancelAcceptedOrdinalSellOrderReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "cancelAcceptedOrdinalSellOrder(uint256)";
            const SELECTOR: [u8; 4] = [115u8, 120u8, 113u8, 85u8];
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
    /**Function with signature `getOpenAcceptedOrdinalSellOrders()` and selector `0x3c49febe`.
```solidity
function getOpenAcceptedOrdinalSellOrders() external view returns (AcceptedOrdinalSellOrder[] memory, uint256[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOpenAcceptedOrdinalSellOrdersCall {}
    ///Container type for the return parameters of the [`getOpenAcceptedOrdinalSellOrders()`](getOpenAcceptedOrdinalSellOrdersCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOpenAcceptedOrdinalSellOrdersReturn {
        pub _0: alloy::sol_types::private::Vec<
            <AcceptedOrdinalSellOrder as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getOpenAcceptedOrdinalSellOrdersCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOpenAcceptedOrdinalSellOrdersCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOpenAcceptedOrdinalSellOrdersCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<AcceptedOrdinalSellOrder>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <AcceptedOrdinalSellOrder as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getOpenAcceptedOrdinalSellOrdersReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOpenAcceptedOrdinalSellOrdersReturn) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOpenAcceptedOrdinalSellOrdersReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0, _1: tuple.1 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOpenAcceptedOrdinalSellOrdersCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOpenAcceptedOrdinalSellOrdersReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<AcceptedOrdinalSellOrder>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOpenAcceptedOrdinalSellOrders()";
            const SELECTOR: [u8; 4] = [60u8, 73u8, 254u8, 190u8];
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
    /**Function with signature `getOpenOrdinalSellOrders()` and selector `0x171abce5`.
```solidity
function getOpenOrdinalSellOrders() external view returns (OrdinalSellOrder[] memory, uint256[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOpenOrdinalSellOrdersCall {}
    ///Container type for the return parameters of the [`getOpenOrdinalSellOrders()`](getOpenOrdinalSellOrdersCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOpenOrdinalSellOrdersReturn {
        pub _0: alloy::sol_types::private::Vec<
            <OrdinalSellOrder as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getOpenOrdinalSellOrdersCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOpenOrdinalSellOrdersCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOpenOrdinalSellOrdersCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<OrdinalSellOrder>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <OrdinalSellOrder as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getOpenOrdinalSellOrdersReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOpenOrdinalSellOrdersReturn) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOpenOrdinalSellOrdersReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0, _1: tuple.1 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOpenOrdinalSellOrdersCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOpenOrdinalSellOrdersReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<OrdinalSellOrder>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOpenOrdinalSellOrders()";
            const SELECTOR: [u8; 4] = [23u8, 26u8, 188u8, 229u8];
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
    /**Function with signature `ordinalSellOrders(uint256)` and selector `0x2b260fa0`.
```solidity
function ordinalSellOrders(uint256) external view returns (OrdinalId memory ordinalID, address sellToken, uint256 sellAmount, BitcoinTx.UTXO memory utxo, address requester, bool isOrderAccepted);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ordinalSellOrdersCall {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`ordinalSellOrders(uint256)`](ordinalSellOrdersCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ordinalSellOrdersReturn {
        pub ordinalID: <OrdinalId as alloy::sol_types::SolType>::RustType,
        pub sellToken: alloy::sol_types::private::Address,
        pub sellAmount: alloy::sol_types::private::primitives::aliases::U256,
        pub utxo: <BitcoinTx::UTXO as alloy::sol_types::SolType>::RustType,
        pub requester: alloy::sol_types::private::Address,
        pub isOrderAccepted: bool,
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
            impl ::core::convert::From<ordinalSellOrdersCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: ordinalSellOrdersCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for ordinalSellOrdersCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                OrdinalId,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                BitcoinTx::UTXO,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bool,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <OrdinalId as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                <BitcoinTx::UTXO as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::Address,
                bool,
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
            impl ::core::convert::From<ordinalSellOrdersReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: ordinalSellOrdersReturn) -> Self {
                    (
                        value.ordinalID,
                        value.sellToken,
                        value.sellAmount,
                        value.utxo,
                        value.requester,
                        value.isOrderAccepted,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for ordinalSellOrdersReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        ordinalID: tuple.0,
                        sellToken: tuple.1,
                        sellAmount: tuple.2,
                        utxo: tuple.3,
                        requester: tuple.4,
                        isOrderAccepted: tuple.5,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for ordinalSellOrdersCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = ordinalSellOrdersReturn;
            type ReturnTuple<'a> = (
                OrdinalId,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                BitcoinTx::UTXO,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bool,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ordinalSellOrders(uint256)";
            const SELECTOR: [u8; 4] = [43u8, 38u8, 15u8, 160u8];
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
    /**Function with signature `placeOrdinalSellOrder((bytes32,uint32),(bytes32,uint32,uint64),address,uint256)` and selector `0x5c9ddc84`.
```solidity
function placeOrdinalSellOrder(OrdinalId memory ordinalID, BitcoinTx.UTXO memory utxo, address sellToken, uint256 sellAmount) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct placeOrdinalSellOrderCall {
        pub ordinalID: <OrdinalId as alloy::sol_types::SolType>::RustType,
        pub utxo: <BitcoinTx::UTXO as alloy::sol_types::SolType>::RustType,
        pub sellToken: alloy::sol_types::private::Address,
        pub sellAmount: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`placeOrdinalSellOrder((bytes32,uint32),(bytes32,uint32,uint64),address,uint256)`](placeOrdinalSellOrderCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct placeOrdinalSellOrderReturn {}
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
                OrdinalId,
                BitcoinTx::UTXO,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <OrdinalId as alloy::sol_types::SolType>::RustType,
                <BitcoinTx::UTXO as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<placeOrdinalSellOrderCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: placeOrdinalSellOrderCall) -> Self {
                    (value.ordinalID, value.utxo, value.sellToken, value.sellAmount)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for placeOrdinalSellOrderCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        ordinalID: tuple.0,
                        utxo: tuple.1,
                        sellToken: tuple.2,
                        sellAmount: tuple.3,
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
            impl ::core::convert::From<placeOrdinalSellOrderReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: placeOrdinalSellOrderReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for placeOrdinalSellOrderReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for placeOrdinalSellOrderCall {
            type Parameters<'a> = (
                OrdinalId,
                BitcoinTx::UTXO,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = placeOrdinalSellOrderReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "placeOrdinalSellOrder((bytes32,uint32),(bytes32,uint32,uint64),address,uint256)";
            const SELECTOR: [u8; 4] = [92u8, 157u8, 220u8, 132u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <OrdinalId as alloy_sol_types::SolType>::tokenize(&self.ordinalID),
                    <BitcoinTx::UTXO as alloy_sol_types::SolType>::tokenize(&self.utxo),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.sellToken,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.sellAmount),
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
    /**Function with signature `proofOrdinalSellOrder(uint256,(bytes4,bytes,bytes,bytes4),(bytes,uint256,bytes))` and selector `0xe8532be3`.
```solidity
function proofOrdinalSellOrder(uint256 id, BitcoinTx.Info memory transaction, BitcoinTx.Proof memory proof) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proofOrdinalSellOrderCall {
        pub id: alloy::sol_types::private::primitives::aliases::U256,
        pub transaction: <BitcoinTx::Info as alloy::sol_types::SolType>::RustType,
        pub proof: <BitcoinTx::Proof as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`proofOrdinalSellOrder(uint256,(bytes4,bytes,bytes,bytes4),(bytes,uint256,bytes))`](proofOrdinalSellOrderCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proofOrdinalSellOrderReturn {}
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
            impl ::core::convert::From<proofOrdinalSellOrderCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: proofOrdinalSellOrderCall) -> Self {
                    (value.id, value.transaction, value.proof)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for proofOrdinalSellOrderCall {
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
            impl ::core::convert::From<proofOrdinalSellOrderReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: proofOrdinalSellOrderReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for proofOrdinalSellOrderReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for proofOrdinalSellOrderCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                BitcoinTx::Info,
                BitcoinTx::Proof,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = proofOrdinalSellOrderReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "proofOrdinalSellOrder(uint256,(bytes4,bytes,bytes,bytes4),(bytes,uint256,bytes))";
            const SELECTOR: [u8; 4] = [232u8, 83u8, 43u8, 227u8];
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
    /**Function with signature `withdrawOrdinalSellOrder(uint256)` and selector `0xe4ae61dd`.
```solidity
function withdrawOrdinalSellOrder(uint256 id) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct withdrawOrdinalSellOrderCall {
        pub id: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`withdrawOrdinalSellOrder(uint256)`](withdrawOrdinalSellOrderCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct withdrawOrdinalSellOrderReturn {}
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
            impl ::core::convert::From<withdrawOrdinalSellOrderCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: withdrawOrdinalSellOrderCall) -> Self {
                    (value.id,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for withdrawOrdinalSellOrderCall {
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
            impl ::core::convert::From<withdrawOrdinalSellOrderReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: withdrawOrdinalSellOrderReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for withdrawOrdinalSellOrderReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for withdrawOrdinalSellOrderCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = withdrawOrdinalSellOrderReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "withdrawOrdinalSellOrder(uint256)";
            const SELECTOR: [u8; 4] = [228u8, 174u8, 97u8, 221u8];
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
    ///Container for all the [`OrdMarketplace`](self) function calls.
    pub enum OrdMarketplaceCalls {
        REQUEST_EXPIRATION_SECONDS(REQUEST_EXPIRATION_SECONDSCall),
        acceptOrdinalSellOrder(acceptOrdinalSellOrderCall),
        acceptedOrdinalSellOrders(acceptedOrdinalSellOrdersCall),
        cancelAcceptedOrdinalSellOrder(cancelAcceptedOrdinalSellOrderCall),
        getOpenAcceptedOrdinalSellOrders(getOpenAcceptedOrdinalSellOrdersCall),
        getOpenOrdinalSellOrders(getOpenOrdinalSellOrdersCall),
        ordinalSellOrders(ordinalSellOrdersCall),
        placeOrdinalSellOrder(placeOrdinalSellOrderCall),
        proofOrdinalSellOrder(proofOrdinalSellOrderCall),
        withdrawOrdinalSellOrder(withdrawOrdinalSellOrderCall),
    }
    #[automatically_derived]
    impl OrdMarketplaceCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [23u8, 26u8, 188u8, 229u8],
            [40u8, 20u8, 161u8, 205u8],
            [43u8, 38u8, 15u8, 160u8],
            [60u8, 73u8, 254u8, 190u8],
            [92u8, 157u8, 220u8, 132u8],
            [115u8, 120u8, 113u8, 85u8],
            [209u8, 146u8, 15u8, 240u8],
            [219u8, 130u8, 213u8, 250u8],
            [228u8, 174u8, 97u8, 221u8],
            [232u8, 83u8, 43u8, 227u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for OrdMarketplaceCalls {
        const NAME: &'static str = "OrdMarketplaceCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 10usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::REQUEST_EXPIRATION_SECONDS(_) => {
                    <REQUEST_EXPIRATION_SECONDSCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::acceptOrdinalSellOrder(_) => {
                    <acceptOrdinalSellOrderCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::acceptedOrdinalSellOrders(_) => {
                    <acceptedOrdinalSellOrdersCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::cancelAcceptedOrdinalSellOrder(_) => {
                    <cancelAcceptedOrdinalSellOrderCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOpenAcceptedOrdinalSellOrders(_) => {
                    <getOpenAcceptedOrdinalSellOrdersCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOpenOrdinalSellOrders(_) => {
                    <getOpenOrdinalSellOrdersCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::ordinalSellOrders(_) => {
                    <ordinalSellOrdersCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::placeOrdinalSellOrder(_) => {
                    <placeOrdinalSellOrderCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::proofOrdinalSellOrder(_) => {
                    <proofOrdinalSellOrderCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::withdrawOrdinalSellOrder(_) => {
                    <withdrawOrdinalSellOrderCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<OrdMarketplaceCalls>] = &[
                {
                    fn getOpenOrdinalSellOrders(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OrdMarketplaceCalls> {
                        <getOpenOrdinalSellOrdersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OrdMarketplaceCalls::getOpenOrdinalSellOrders)
                    }
                    getOpenOrdinalSellOrders
                },
                {
                    fn acceptOrdinalSellOrder(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OrdMarketplaceCalls> {
                        <acceptOrdinalSellOrderCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OrdMarketplaceCalls::acceptOrdinalSellOrder)
                    }
                    acceptOrdinalSellOrder
                },
                {
                    fn ordinalSellOrders(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OrdMarketplaceCalls> {
                        <ordinalSellOrdersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OrdMarketplaceCalls::ordinalSellOrders)
                    }
                    ordinalSellOrders
                },
                {
                    fn getOpenAcceptedOrdinalSellOrders(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OrdMarketplaceCalls> {
                        <getOpenAcceptedOrdinalSellOrdersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OrdMarketplaceCalls::getOpenAcceptedOrdinalSellOrders)
                    }
                    getOpenAcceptedOrdinalSellOrders
                },
                {
                    fn placeOrdinalSellOrder(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OrdMarketplaceCalls> {
                        <placeOrdinalSellOrderCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OrdMarketplaceCalls::placeOrdinalSellOrder)
                    }
                    placeOrdinalSellOrder
                },
                {
                    fn cancelAcceptedOrdinalSellOrder(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OrdMarketplaceCalls> {
                        <cancelAcceptedOrdinalSellOrderCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OrdMarketplaceCalls::cancelAcceptedOrdinalSellOrder)
                    }
                    cancelAcceptedOrdinalSellOrder
                },
                {
                    fn REQUEST_EXPIRATION_SECONDS(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OrdMarketplaceCalls> {
                        <REQUEST_EXPIRATION_SECONDSCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OrdMarketplaceCalls::REQUEST_EXPIRATION_SECONDS)
                    }
                    REQUEST_EXPIRATION_SECONDS
                },
                {
                    fn acceptedOrdinalSellOrders(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OrdMarketplaceCalls> {
                        <acceptedOrdinalSellOrdersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OrdMarketplaceCalls::acceptedOrdinalSellOrders)
                    }
                    acceptedOrdinalSellOrders
                },
                {
                    fn withdrawOrdinalSellOrder(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OrdMarketplaceCalls> {
                        <withdrawOrdinalSellOrderCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OrdMarketplaceCalls::withdrawOrdinalSellOrder)
                    }
                    withdrawOrdinalSellOrder
                },
                {
                    fn proofOrdinalSellOrder(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<OrdMarketplaceCalls> {
                        <proofOrdinalSellOrderCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(OrdMarketplaceCalls::proofOrdinalSellOrder)
                    }
                    proofOrdinalSellOrder
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
                Self::acceptOrdinalSellOrder(inner) => {
                    <acceptOrdinalSellOrderCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::acceptedOrdinalSellOrders(inner) => {
                    <acceptedOrdinalSellOrdersCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::cancelAcceptedOrdinalSellOrder(inner) => {
                    <cancelAcceptedOrdinalSellOrderCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOpenAcceptedOrdinalSellOrders(inner) => {
                    <getOpenAcceptedOrdinalSellOrdersCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOpenOrdinalSellOrders(inner) => {
                    <getOpenOrdinalSellOrdersCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ordinalSellOrders(inner) => {
                    <ordinalSellOrdersCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::placeOrdinalSellOrder(inner) => {
                    <placeOrdinalSellOrderCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::proofOrdinalSellOrder(inner) => {
                    <proofOrdinalSellOrderCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::withdrawOrdinalSellOrder(inner) => {
                    <withdrawOrdinalSellOrderCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::acceptOrdinalSellOrder(inner) => {
                    <acceptOrdinalSellOrderCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::acceptedOrdinalSellOrders(inner) => {
                    <acceptedOrdinalSellOrdersCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::cancelAcceptedOrdinalSellOrder(inner) => {
                    <cancelAcceptedOrdinalSellOrderCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getOpenAcceptedOrdinalSellOrders(inner) => {
                    <getOpenAcceptedOrdinalSellOrdersCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getOpenOrdinalSellOrders(inner) => {
                    <getOpenOrdinalSellOrdersCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ordinalSellOrders(inner) => {
                    <ordinalSellOrdersCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::placeOrdinalSellOrder(inner) => {
                    <placeOrdinalSellOrderCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::proofOrdinalSellOrder(inner) => {
                    <proofOrdinalSellOrderCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::withdrawOrdinalSellOrder(inner) => {
                    <withdrawOrdinalSellOrderCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`OrdMarketplace`](self) events.
    pub enum OrdMarketplaceEvents {
        acceptOrdinalSellOrderEvent(acceptOrdinalSellOrderEvent),
        cancelAcceptedOrdinalSellOrderEvent(cancelAcceptedOrdinalSellOrderEvent),
        placeOrdinalSellOrderEvent(placeOrdinalSellOrderEvent),
        proofOrdinalSellOrderEvent(proofOrdinalSellOrderEvent),
        withdrawOrdinalSellOrderEvent(withdrawOrdinalSellOrderEvent),
    }
    #[automatically_derived]
    impl OrdMarketplaceEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                156u8,
                33u8,
                106u8,
                70u8,
                23u8,
                214u8,
                192u8,
                61u8,
                199u8,
                203u8,
                217u8,
                99u8,
                33u8,
                102u8,
                241u8,
                197u8,
                201u8,
                239u8,
                65u8,
                249u8,
                238u8,
                134u8,
                191u8,
                59u8,
                131u8,
                246u8,
                113u8,
                192u8,
                5u8,
                16u8,
                119u8,
                4u8,
            ],
            [
                179u8,
                91u8,
                63u8,
                228u8,
                218u8,
                175u8,
                108u8,
                198u8,
                110u8,
                184u8,
                189u8,
                65u8,
                62u8,
                154u8,
                181u8,
                68u8,
                73u8,
                231u8,
                102u8,
                246u8,
                212u8,
                97u8,
                37u8,
                204u8,
                88u8,
                242u8,
                85u8,
                105u8,
                74u8,
                14u8,
                132u8,
                126u8,
            ],
            [
                197u8,
                119u8,
                48u8,
                154u8,
                205u8,
                121u8,
                57u8,
                204u8,
                47u8,
                1u8,
                246u8,
                127u8,
                7u8,
                62u8,
                26u8,
                84u8,
                130u8,
                36u8,
                69u8,
                76u8,
                221u8,
                220u8,
                121u8,
                177u8,
                97u8,
                219u8,
                23u8,
                181u8,
                49u8,
                94u8,
                159u8,
                12u8,
            ],
            [
                251u8,
                45u8,
                51u8,
                16u8,
                227u8,
                231u8,
                149u8,
                120u8,
                172u8,
                80u8,
                124u8,
                219u8,
                219u8,
                50u8,
                229u8,
                37u8,
                129u8,
                219u8,
                193u8,
                123u8,
                224u8,
                78u8,
                81u8,
                151u8,
                211u8,
                183u8,
                197u8,
                34u8,
                115u8,
                95u8,
                185u8,
                228u8,
            ],
            [
                254u8,
                53u8,
                15u8,
                249u8,
                204u8,
                173u8,
                209u8,
                183u8,
                194u8,
                107u8,
                95u8,
                150u8,
                221u8,
                7u8,
                141u8,
                8u8,
                168u8,
                119u8,
                200u8,
                243u8,
                125u8,
                80u8,
                105u8,
                49u8,
                236u8,
                216u8,
                242u8,
                189u8,
                189u8,
                81u8,
                182u8,
                242u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for OrdMarketplaceEvents {
        const NAME: &'static str = "OrdMarketplaceEvents";
        const COUNT: usize = 5usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(
                    <acceptOrdinalSellOrderEvent as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <acceptOrdinalSellOrderEvent as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::acceptOrdinalSellOrderEvent)
                }
                Some(
                    <cancelAcceptedOrdinalSellOrderEvent as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <cancelAcceptedOrdinalSellOrderEvent as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::cancelAcceptedOrdinalSellOrderEvent)
                }
                Some(
                    <placeOrdinalSellOrderEvent as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <placeOrdinalSellOrderEvent as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::placeOrdinalSellOrderEvent)
                }
                Some(
                    <proofOrdinalSellOrderEvent as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <proofOrdinalSellOrderEvent as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::proofOrdinalSellOrderEvent)
                }
                Some(
                    <withdrawOrdinalSellOrderEvent as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <withdrawOrdinalSellOrderEvent as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::withdrawOrdinalSellOrderEvent)
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
    impl alloy_sol_types::private::IntoLogData for OrdMarketplaceEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::acceptOrdinalSellOrderEvent(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::cancelAcceptedOrdinalSellOrderEvent(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::placeOrdinalSellOrderEvent(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::proofOrdinalSellOrderEvent(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::withdrawOrdinalSellOrderEvent(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::acceptOrdinalSellOrderEvent(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::cancelAcceptedOrdinalSellOrderEvent(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::placeOrdinalSellOrderEvent(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::proofOrdinalSellOrderEvent(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::withdrawOrdinalSellOrderEvent(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`OrdMarketplace`](self) contract instance.

See the [wrapper's documentation](`OrdMarketplaceInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> OrdMarketplaceInstance<T, P, N> {
        OrdMarketplaceInstance::<T, P, N>::new(address, provider)
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
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<OrdMarketplaceInstance<T, P, N>>,
    > {
        OrdMarketplaceInstance::<T, P, N>::deploy(provider, _relay)
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
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        OrdMarketplaceInstance::<T, P, N>::deploy_builder(provider, _relay)
    }
    /**A [`OrdMarketplace`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`OrdMarketplace`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct OrdMarketplaceInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for OrdMarketplaceInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("OrdMarketplaceInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > OrdMarketplaceInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`OrdMarketplace`](self) contract instance.

See the [wrapper's documentation](`OrdMarketplaceInstance`) for more details.*/
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
        ) -> alloy_contract::Result<OrdMarketplaceInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(provider, _relay);
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
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall { _relay },
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
    impl<T, P: ::core::clone::Clone, N> OrdMarketplaceInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> OrdMarketplaceInstance<T, P, N> {
            OrdMarketplaceInstance {
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
    > OrdMarketplaceInstance<T, P, N> {
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
        ///Creates a new call builder for the [`acceptOrdinalSellOrder`] function.
        pub fn acceptOrdinalSellOrder(
            &self,
            id: alloy::sol_types::private::primitives::aliases::U256,
            bitcoinAddress: <BitcoinAddress as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, acceptOrdinalSellOrderCall, N> {
            self.call_builder(
                &acceptOrdinalSellOrderCall {
                    id,
                    bitcoinAddress,
                },
            )
        }
        ///Creates a new call builder for the [`acceptedOrdinalSellOrders`] function.
        pub fn acceptedOrdinalSellOrders(
            &self,
            _0: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, acceptedOrdinalSellOrdersCall, N> {
            self.call_builder(
                &acceptedOrdinalSellOrdersCall {
                    _0,
                },
            )
        }
        ///Creates a new call builder for the [`cancelAcceptedOrdinalSellOrder`] function.
        pub fn cancelAcceptedOrdinalSellOrder(
            &self,
            id: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            cancelAcceptedOrdinalSellOrderCall,
            N,
        > {
            self.call_builder(
                &cancelAcceptedOrdinalSellOrderCall {
                    id,
                },
            )
        }
        ///Creates a new call builder for the [`getOpenAcceptedOrdinalSellOrders`] function.
        pub fn getOpenAcceptedOrdinalSellOrders(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            getOpenAcceptedOrdinalSellOrdersCall,
            N,
        > {
            self.call_builder(
                &getOpenAcceptedOrdinalSellOrdersCall {
                },
            )
        }
        ///Creates a new call builder for the [`getOpenOrdinalSellOrders`] function.
        pub fn getOpenOrdinalSellOrders(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getOpenOrdinalSellOrdersCall, N> {
            self.call_builder(&getOpenOrdinalSellOrdersCall {})
        }
        ///Creates a new call builder for the [`ordinalSellOrders`] function.
        pub fn ordinalSellOrders(
            &self,
            _0: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, ordinalSellOrdersCall, N> {
            self.call_builder(&ordinalSellOrdersCall { _0 })
        }
        ///Creates a new call builder for the [`placeOrdinalSellOrder`] function.
        pub fn placeOrdinalSellOrder(
            &self,
            ordinalID: <OrdinalId as alloy::sol_types::SolType>::RustType,
            utxo: <BitcoinTx::UTXO as alloy::sol_types::SolType>::RustType,
            sellToken: alloy::sol_types::private::Address,
            sellAmount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, placeOrdinalSellOrderCall, N> {
            self.call_builder(
                &placeOrdinalSellOrderCall {
                    ordinalID,
                    utxo,
                    sellToken,
                    sellAmount,
                },
            )
        }
        ///Creates a new call builder for the [`proofOrdinalSellOrder`] function.
        pub fn proofOrdinalSellOrder(
            &self,
            id: alloy::sol_types::private::primitives::aliases::U256,
            transaction: <BitcoinTx::Info as alloy::sol_types::SolType>::RustType,
            proof: <BitcoinTx::Proof as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, proofOrdinalSellOrderCall, N> {
            self.call_builder(
                &proofOrdinalSellOrderCall {
                    id,
                    transaction,
                    proof,
                },
            )
        }
        ///Creates a new call builder for the [`withdrawOrdinalSellOrder`] function.
        pub fn withdrawOrdinalSellOrder(
            &self,
            id: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, withdrawOrdinalSellOrderCall, N> {
            self.call_builder(&withdrawOrdinalSellOrderCall { id })
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > OrdMarketplaceInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`acceptOrdinalSellOrderEvent`] event.
        pub fn acceptOrdinalSellOrderEvent_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, acceptOrdinalSellOrderEvent, N> {
            self.event_filter::<acceptOrdinalSellOrderEvent>()
        }
        ///Creates a new event filter for the [`cancelAcceptedOrdinalSellOrderEvent`] event.
        pub fn cancelAcceptedOrdinalSellOrderEvent_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, cancelAcceptedOrdinalSellOrderEvent, N> {
            self.event_filter::<cancelAcceptedOrdinalSellOrderEvent>()
        }
        ///Creates a new event filter for the [`placeOrdinalSellOrderEvent`] event.
        pub fn placeOrdinalSellOrderEvent_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, placeOrdinalSellOrderEvent, N> {
            self.event_filter::<placeOrdinalSellOrderEvent>()
        }
        ///Creates a new event filter for the [`proofOrdinalSellOrderEvent`] event.
        pub fn proofOrdinalSellOrderEvent_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, proofOrdinalSellOrderEvent, N> {
            self.event_filter::<proofOrdinalSellOrderEvent>()
        }
        ///Creates a new event filter for the [`withdrawOrdinalSellOrderEvent`] event.
        pub fn withdrawOrdinalSellOrderEvent_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, withdrawOrdinalSellOrderEvent, N> {
            self.event_filter::<withdrawOrdinalSellOrderEvent>()
        }
    }
}
