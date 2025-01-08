///Module containing a contract's types and functions.
/**

```solidity
library IEntryPoint {
    struct AggregatorStakeInfo { address aggregator; IStakeManager.StakeInfo stakeInfo; }
    struct ReturnInfo { uint256 preOpGas; uint256 prefund; bool sigFailed; uint48 validAfter; uint48 validUntil; bytes paymasterContext; }
    struct UserOpsPerAggregator { UserOperation[] userOps; address aggregator; bytes signature; }
}
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
    }
}
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
library IEntryPoint {
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
    struct UserOpsPerAggregator {
        UserOperation[] userOps;
        address aggregator;
        bytes signature;
    }
}

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

interface EntryPoint {
    struct MemoryUserOp {
        address sender;
        uint256 nonce;
        uint256 callGasLimit;
        uint256 verificationGasLimit;
        uint256 preVerificationGas;
        address paymaster;
        uint256 maxFeePerGas;
        uint256 maxPriorityFeePerGas;
    }
    struct UserOpInfo {
        MemoryUserOp mUserOp;
        bytes32 userOpHash;
        uint256 prefund;
        uint256 contextOffset;
        uint256 preOpGas;
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

    error ExecutionResult(uint256 preOpGas, uint256 paid, uint48 validAfter, uint48 validUntil, bool targetSuccess, bytes targetResult);
    error FailedOp(uint256 opIndex, string reason);
    error SenderAddressResult(address sender);
    error SignatureValidationFailed(address aggregator);
    error ValidationResult(IEntryPoint.ReturnInfo returnInfo, IStakeManager.StakeInfo senderInfo, IStakeManager.StakeInfo factoryInfo, IStakeManager.StakeInfo paymasterInfo);
    error ValidationResultWithAggregation(IEntryPoint.ReturnInfo returnInfo, IStakeManager.StakeInfo senderInfo, IStakeManager.StakeInfo factoryInfo, IStakeManager.StakeInfo paymasterInfo, IEntryPoint.AggregatorStakeInfo aggregatorInfo);

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

    receive() external payable;

    function SIG_VALIDATION_FAILED() external view returns (uint256);
    function _validateSenderAndPaymaster(bytes memory initCode, address sender, bytes memory paymasterAndData) external view;
    function addStake(uint32 unstakeDelaySec) external payable;
    function balanceOf(address account) external view returns (uint256);
    function depositTo(address account) external payable;
    function deposits(address) external view returns (uint112 deposit, bool staked, uint112 stake, uint32 unstakeDelaySec, uint48 withdrawTime);
    function getDepositInfo(address account) external view returns (IStakeManager.DepositInfo memory info);
    function getNonce(address sender, uint192 key) external view returns (uint256 nonce);
    function getSenderAddress(bytes memory initCode) external;
    function getUserOpHash(UserOperation memory userOp) external view returns (bytes32);
    function handleAggregatedOps(IEntryPoint.UserOpsPerAggregator[] memory opsPerAggregator, address payable beneficiary) external;
    function handleOps(UserOperation[] memory ops, address payable beneficiary) external;
    function incrementNonce(uint192 key) external;
    function innerHandleOp(bytes memory callData, UserOpInfo memory opInfo, bytes memory context) external returns (uint256 actualGasCost);
    function nonceSequenceNumber(address, uint192) external view returns (uint256);
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
    "type": "receive",
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "SIG_VALIDATION_FAILED",
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
    "name": "_validateSenderAndPaymaster",
    "inputs": [
      {
        "name": "initCode",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "sender",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "paymasterAndData",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "addStake",
    "inputs": [
      {
        "name": "unstakeDelaySec",
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
    "name": "deposits",
    "inputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
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
    ],
    "stateMutability": "view"
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
    "name": "innerHandleOp",
    "inputs": [
      {
        "name": "callData",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "opInfo",
        "type": "tuple",
        "internalType": "struct EntryPoint.UserOpInfo",
        "components": [
          {
            "name": "mUserOp",
            "type": "tuple",
            "internalType": "struct EntryPoint.MemoryUserOp",
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
                "name": "paymaster",
                "type": "address",
                "internalType": "address"
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
              }
            ]
          },
          {
            "name": "userOpHash",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "prefund",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "contextOffset",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "preOpGas",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      },
      {
        "name": "context",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [
      {
        "name": "actualGasCost",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "nonceSequenceNumber",
    "inputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "",
        "type": "uint192",
        "internalType": "uint192"
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
pub mod EntryPoint {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60a0604052604051600e906047565b604051809103905ff0801580156026573d5f5f3e3d5ffd5b506001600160a01b0316608052348015603d575f5ffd5b5060016002556054565b61023e8061455583390190565b6080516144e26100735f395f81816114a7015261323d01526144e25ff3fe60806040526004361061015a575f3560e01c80638f41ec5a116100bb578063bb9fe6bf11610071578063d6383f9411610057578063d6383f94146104d0578063ee219423146104ef578063fc7e286d1461050e575f5ffd5b8063bb9fe6bf1461049d578063c23a5cea146104b1575f5ffd5b80639b249f69116100a15780639b249f691461044c578063a61935311461046b578063b760faf91461048a575f5ffd5b80638f41ec5a14610419578063957122ab1461042d575f5ffd5b8063205c2878116101105780634b1d7cf5116100f65780634b1d7cf5146102655780635287ce121461028457806370a08231146103d5575f5ffd5b8063205c28781461022757806335567e1a14610246575f5ffd5b80631b2e01b8116101405780631b2e01b8146101a05780631d732756146101e95780631fad948c14610208575f5ffd5b80630396cb601461016e5780630bd28e3b14610181575f5ffd5b3661016a57610168336105e6565b005b5f5ffd5b61016861017c3660046134fa565b610653565b34801561018c575f5ffd5b5061016861019b366004613549565b61096a565b3480156101ab575f5ffd5b506101d66101ba366004613581565b600160209081525f928352604080842090915290825290205481565b6040519081526020015b60405180910390f35b3480156101f4575f5ffd5b506101d661020336600461378f565b6109b1565b348015610213575f5ffd5b50610168610222366004613890565b610b31565b348015610232575f5ffd5b506101686102413660046138e3565b610cef565b348015610251575f5ffd5b506101d6610260366004613581565b610e9e565b348015610270575f5ffd5b5061016861027f366004613890565b610f0b565b34801561028f575f5ffd5b5061036461029e36600461390d565b6040805160a0810182525f80825260208201819052918101829052606081018290526080810191909152506001600160a01b03165f9081526020818152604091829020825160a08101845281546dffffffffffffffffffffffffffff80821683526e010000000000000000000000000000820460ff161515948301949094526f0100000000000000000000000000000090049092169282019290925260019091015463ffffffff81166060830152640100000000900465ffffffffffff16608082015290565b6040516101e091905f60a0820190506dffffffffffffffffffffffffffff83511682526020830151151560208301526dffffffffffffffffffffffffffff604084015116604083015263ffffffff606084015116606083015265ffffffffffff608084015116608083015292915050565b3480156103e0575f5ffd5b506101d66103ef36600461390d565b6001600160a01b03165f908152602081905260409020546dffffffffffffffffffffffffffff1690565b348015610424575f5ffd5b506101d6600181565b348015610438575f5ffd5b50610168610447366004613928565b61137b565b348015610457575f5ffd5b506101686104663660046139aa565b611475565b348015610476575f5ffd5b506101d6610485366004613a00565b61155e565b61016861049836600461390d565b6105e6565b3480156104a8575f5ffd5b5061016861159f565b3480156104bc575f5ffd5b506101686104cb36600461390d565b611720565b3480156104db575f5ffd5b506101686104ea366004613a32565b611999565b3480156104fa575f5ffd5b50610168610509366004613a00565b611aa1565b348015610519575f5ffd5b5061059961052836600461390d565b5f60208190529081526040902080546001909101546dffffffffffffffffffffffffffff808316926e010000000000000000000000000000810460ff16926f010000000000000000000000000000009091049091169063ffffffff811690640100000000900465ffffffffffff1685565b604080516dffffffffffffffffffffffffffff96871681529415156020860152929094169183019190915263ffffffff16606082015265ffffffffffff909116608082015260a0016101e0565b6105f08134611da0565b6001600160a01b0381165f8181526020818152604091829020805492516dffffffffffffffffffffffffffff909316835292917f2da466a7b24304f47e87fa2e1e5a81b9831ce54fec19055ce277ca2f39ba42c491015b60405180910390a25050565b335f90815260208190526040902063ffffffff82166106b95760405162461bcd60e51b815260206004820152601a60248201527f6d757374207370656369667920756e7374616b652064656c617900000000000060448201526064015b60405180910390fd5b600181015463ffffffff90811690831610156107175760405162461bcd60e51b815260206004820152601c60248201527f63616e6e6f7420646563726561736520756e7374616b652074696d650000000060448201526064016106b0565b80545f906107499034906f0100000000000000000000000000000090046dffffffffffffffffffffffffffff16613ac0565b90505f811161079a5760405162461bcd60e51b815260206004820152601260248201527f6e6f207374616b6520737065636966696564000000000000000000000000000060448201526064016106b0565b6dffffffffffffffffffffffffffff8111156107f85760405162461bcd60e51b815260206004820152600e60248201527f7374616b65206f766572666c6f7700000000000000000000000000000000000060448201526064016106b0565b6040805160a08101825283546dffffffffffffffffffffffffffff90811682526001602080840182815286841685870190815263ffffffff808b16606088019081525f608089018181523380835296829052908a9020985189549551945189166f01000000000000000000000000000000027fffffff0000000000000000000000000000ffffffffffffffffffffffffffffff9515156e010000000000000000000000000000027fffffffffffffffffffffffffffffffffff0000000000000000000000000000009097169190991617949094179290921695909517865551949092018054925165ffffffffffff16640100000000027fffffffffffffffffffffffffffffffffffffffffffff00000000000000000000909316949093169390931717905590517fa5ae833d0bb1dcd632d98a8b70973e8516812898e19bf27b70071ebc8dc52c019061095d908490879091825263ffffffff16602082015260400190565b60405180910390a2505050565b335f90815260016020908152604080832077ffffffffffffffffffffffffffffffffffffffffffffffff8516845290915281208054916109a983613ad3565b919050555050565b5f5f5a9050333014610a055760405162461bcd60e51b815260206004820152601760248201527f4141393220696e7465726e616c2063616c6c206f6e6c7900000000000000000060448201526064016106b0565b8451604081015160608201518101611388015a1015610a46577fdeaddead000000000000000000000000000000000000000000000000000000005f5260205ffd5b87515f9015610ad4575f610a5f845f01515f8c86611e75565b905080610ad2575f610a72610800611e8b565b805190915015610acc57845f01516001600160a01b03168a602001517f1c4fada7374c0a9ee8841fc38afe82932dc0f8e69012e927f061a8bae611a201876020015184604051610ac3929190613b19565b60405180910390a35b60019250505b505b5f88608001515a8603019050610b235f838b8b8b8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f92019190915250889250611eb6915050565b9a9950505050505050505050565b6002805403610b825760405162461bcd60e51b815260206004820152601f60248201527f5265656e7472616e637947756172643a207265656e7472616e742063616c6c0060448201526064016106b0565b60028055815f8167ffffffffffffffff811115610ba157610ba16135b4565b604051908082528060200260200182016040528015610bda57816020015b610bc761347f565b815260200190600190039081610bbf5790505b5090505f5b82811015610c4f575f828281518110610bfa57610bfa613b31565b602002602001015190505f5f610c34848a8a87818110610c1c57610c1c613b31565b9050602002810190610c2e9190613b5e565b856121dd565b91509150610c448483835f6123c3565b505050600101610bdf565b506040515f907fbb47ee3e183a558b1a2ff0874b079f3fc5478b7454eacf2bfc5af2ff5878f972908290a15f5b83811015610cd757610ccb81888884818110610c9a57610c9a613b31565b9050602002810190610cac9190613b5e565b858481518110610cbe57610cbe613b31565b602002602001015161258b565b90910190600101610c7c565b50610ce284826126d2565b5050600160025550505050565b335f90815260208190526040902080546dffffffffffffffffffffffffffff16821115610d5e5760405162461bcd60e51b815260206004820152601960248201527f576974686472617720616d6f756e7420746f6f206c617267650000000000000060448201526064016106b0565b8054610d7b9083906dffffffffffffffffffffffffffff16613b9a565b81547fffffffffffffffffffffffffffffffffffff0000000000000000000000000000166dffffffffffffffffffffffffffff91909116178155604080516001600160a01b03851681526020810184905233917fd1c19fbcd4551a5edfb66d43d2e337c04837afda3482b42bdf569a8fccdae5fb910160405180910390a25f836001600160a01b0316836040515f6040518083038185875af1925050503d805f8114610e42576040519150601f19603f3d011682016040523d82523d5f602084013e610e47565b606091505b5050905080610e985760405162461bcd60e51b815260206004820152601260248201527f6661696c656420746f207769746864726177000000000000000000000000000060448201526064016106b0565b50505050565b6001600160a01b0382165f90815260016020908152604080832077ffffffffffffffffffffffffffffffffffffffffffffffff8516845290915290819020549082901b7fffffffffffffffffffffffffffffffffffffffffffffffff000000000000000016175b92915050565b6002805403610f5c5760405162461bcd60e51b815260206004820152601f60248201527f5265656e7472616e637947756172643a207265656e7472616e742063616c6c0060448201526064016106b0565b60028055815f805b828110156110e05736868683818110610f7f57610f7f613b31565b9050602002810190610f919190613bad565b9050365f610f9f8380613bdf565b90925090505f610fb5604085016020860161390d565b90505f196001600160a01b038216016110105760405162461bcd60e51b815260206004820152601760248201527f4141393620696e76616c69642061676772656761746f7200000000000000000060448201526064016106b0565b6001600160a01b038116156110c4576001600160a01b03811663e3563a4f848461103d6040890189613c25565b6040518563ffffffff1660e01b815260040161105c9493929190613dc9565b5f6040518083038186803b158015611072575f5ffd5b505afa925050508015611083575060015b6110c4576040517f86a9f7500000000000000000000000000000000000000000000000000000000081526001600160a01b03821660048201526024016106b0565b6110ce8287613ac0565b95505060019093019250610f64915050565b505f8167ffffffffffffffff8111156110fb576110fb6135b4565b60405190808252806020026020018201604052801561113457816020015b61112161347f565b8152602001906001900390816111195790505b506040519091507fbb47ee3e183a558b1a2ff0874b079f3fc5478b7454eacf2bfc5af2ff5878f972905f90a15f805b84811015611234573688888381811061117e5761117e613b31565b90506020028101906111909190613bad565b9050365f61119e8380613bdf565b90925090505f6111b4604085016020860161390d565b9050815f5b81811015611222575f8989815181106111d4576111d4613b31565b602002602001015190505f5f6111f68b898987818110610c1c57610c1c613b31565b91509150611206848383896123c3565b8a61121081613ad3565b9b5050600190930192506111b9915050565b50506001909401935061116392505050565b505f905080805b85811015611337573689898381811061125657611256613b31565b90506020028101906112689190613bad565b905061127a604082016020830161390d565b6001600160a01b03167f575ff3acadd5ab348fe1855e217e0f3678f8d767d7494c9f9fefbee2e17cca4d60405160405180910390a2365f6112bb8380613bdf565b9092509050805f5b8181101561132657611305888585848181106112e1576112e1613b31565b90506020028101906112f39190613b5e565b8b8b81518110610cbe57610cbe613b31565b61130f9088613ac0565b96508761131b81613ad3565b9850506001016112c3565b50506001909301925061123b915050565b506040515f907f575ff3acadd5ab348fe1855e217e0f3678f8d767d7494c9f9fefbee2e17cca4d908290a261136c86826126d2565b50506001600255505050505050565b8315801561139157506001600160a01b0383163b155b156113de5760405162461bcd60e51b815260206004820152601960248201527f41413230206163636f756e74206e6f74206465706c6f7965640000000000000060448201526064016106b0565b60148110611454575f6113f46014828486613e8f565b6113fd91613eb6565b60601c9050803b5f036114525760405162461bcd60e51b815260206004820152601b60248201527f41413330207061796d6173746572206e6f74206465706c6f796564000000000060448201526064016106b0565b505b60405162461bcd60e51b8152602060048201525f60248201526044016106b0565b6040517f570e1a360000000000000000000000000000000000000000000000000000000081525f906001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063570e1a36906114de9086908690600401613f1c565b6020604051808303815f875af11580156114fa573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061151e9190613f2f565b6040517f6ca7b8060000000000000000000000000000000000000000000000000000000081526001600160a01b03821660048201529091506024016106b0565b5f611568826127cc565b6040805160208101929092523090820152466060820152608001604051602081830303815290604052805190602001209050919050565b335f9081526020819052604081206001810154909163ffffffff909116900361160a5760405162461bcd60e51b815260206004820152600a60248201527f6e6f74207374616b65640000000000000000000000000000000000000000000060448201526064016106b0565b80546e010000000000000000000000000000900460ff1661166d5760405162461bcd60e51b815260206004820152601160248201527f616c726561647920756e7374616b696e6700000000000000000000000000000060448201526064016106b0565b60018101545f906116849063ffffffff1642613f4a565b6001830180547fffffffffffffffffffffffffffffffffffffffffffff000000000000ffffffff1664010000000065ffffffffffff84169081029190911790915583547fffffffffffffffffffffffffffffffffff00ffffffffffffffffffffffffffff16845560405190815290915033907ffa9b3c14cc825c412c9ed81b3ba365a5b459439403f18829e572ed53a4180f0a90602001610647565b335f90815260208190526040902080546f0100000000000000000000000000000090046dffffffffffffffffffffffffffff16806117a05760405162461bcd60e51b815260206004820152601460248201527f4e6f207374616b6520746f20776974686472617700000000000000000000000060448201526064016106b0565b6001820154640100000000900465ffffffffffff166118015760405162461bcd60e51b815260206004820152601d60248201527f6d7573742063616c6c20756e6c6f636b5374616b65282920666972737400000060448201526064016106b0565b60018201544264010000000090910465ffffffffffff1611156118665760405162461bcd60e51b815260206004820152601b60248201527f5374616b65207769746864726177616c206973206e6f7420647565000000000060448201526064016106b0565b6001820180547fffffffffffffffffffffffffffffffffffffffffffff0000000000000000000016905581547fffffff0000000000000000000000000000ffffffffffffffffffffffffffffff168255604080516001600160a01b03851681526020810183905233917fb7c918e0e249f999e965cafeb6c664271b3f4317d296461500e71da39f0cbda3910160405180910390a25f836001600160a01b0316826040515f6040518083038185875af1925050503d805f8114611943576040519150601f19603f3d011682016040523d82523d5f602084013e611948565b606091505b5050905080610e985760405162461bcd60e51b815260206004820152601860248201527f6661696c656420746f207769746864726177207374616b65000000000000000060448201526064016106b0565b6119a161347f565b6119aa856127e4565b5f5f6119b75f88856121dd565b915091505f6119c683836128b9565b90506119d0435f52565b5f6119dc5f8a8761258b565b90506119e6435f52565b5f60606001600160a01b038a1615611a5757896001600160a01b03168989604051611a12929190613f68565b5f604051808303815f865af19150503d805f8114611a4b576040519150601f19603f3d011682016040523d82523d5f602084013e611a50565b606091505b5090925090505b8660800151838560200151866040015185856040517f8b7ac9800000000000000000000000000000000000000000000000000000000081526004016106b096959493929190613f77565b611aa961347f565b611ab2826127e4565b5f5f611abf5f85856121dd565b845160a001516040805180820182525f80825260208083018281526001600160a01b03958616835282825284832080546dffffffffffffffffffffffffffff6f01000000000000000000000000000000918290048116875260019283015463ffffffff9081169094528d51518851808a018a5287815280870188815291909a16875286865288872080549390930490911689529101549091169052835180850190945281845283015293955091935090365f611b7e60408a018a613c25565b90925090505f6014821015611b93575f611bad565b611ba060145f8486613e8f565b611ba991613eb6565b60601c5b6040805180820182525f80825260208083018281526001600160a01b03861683529082905292902080546f0100000000000000000000000000000090046dffffffffffffffffffffffffffff1682526001015463ffffffff1690915290915093505050505f611c1c86866128b9565b90505f815f015190505f60016001600160a01b0316826001600160a01b03161490505f6040518060c001604052808b6080015181526020018b6040015181526020018315158152602001856020015165ffffffffffff168152602001856040015165ffffffffffff168152602001611c958c6060015190565b905290506001600160a01b03831615801590611cbb57506001600160a01b038316600114155b15611d66576040805180820182526001600160a01b038516808252825180840184525f80825260208083018281529382528181529085902080546f0100000000000000000000000000000090046dffffffffffffffffffffffffffff1683526001015463ffffffff169092529082015290517ffaecb4e40000000000000000000000000000000000000000000000000000000081526106b0908390899089908c90869060040161401d565b808686896040517fe0cff05f0000000000000000000000000000000000000000000000000000000081526004016106b0949392919061409c565b6001600160a01b0382165f9081526020819052604081208054909190611dd79084906dffffffffffffffffffffffffffff16613ac0565b90506dffffffffffffffffffffffffffff811115611e375760405162461bcd60e51b815260206004820152601060248201527f6465706f736974206f766572666c6f770000000000000000000000000000000060448201526064016106b0565b81547fffffffffffffffffffffffffffffffffffff0000000000000000000000000000166dffffffffffffffffffffffffffff919091161790555050565b5f5f5f845160208601878987f195945050505050565b60603d82811115611e995750815b604051602082018101604052818152815f602083013e9392505050565b5f5f5a85519091505f9081611eca82612983565b60a08301519091506001600160a01b038116611ee957825193506120c4565b8093505f885111156120c457868202955060028a6002811115611f0e57611f0e6140f2565b14611f945760608301516040517fa9a234090000000000000000000000000000000000000000000000000000000081526001600160a01b0383169163a9a2340991611f61908e908d908c9060040161411f565b5f604051808303815f88803b158015611f78575f5ffd5b5087f1158015611f8a573d5f5f3e3d5ffd5b50505050506120c4565b60608301516040517fa9a234090000000000000000000000000000000000000000000000000000000081526001600160a01b0383169163a9a2340991611fe2908e908d908c9060040161411f565b5f604051808303815f88803b158015611ff9575f5ffd5b5087f19350505050801561200b575060015b6120c45761201761417c565b806308c379a003612070575061202b614195565b806120365750612072565b8b81604051602001612048919061426c565b60408051601f1981840301815290829052631101335b60e11b82526106b09291600401613b19565b505b8a604051631101335b60e11b81526004016106b09181526040602082018190526012908201527f4141353020706f73744f70207265766572740000000000000000000000000000606082015260800190565b5a8503870196508187029550858960400151101561212d578a604051631101335b60e11b81526004016106b091815260406020808301829052908201527f414135312070726566756e642062656c6f772061637475616c476173436f7374606082015260800190565b604089015186900361213f8582611da0565b5f808c6002811115612153576121536140f2565b1490508460a001516001600160a01b0316855f01516001600160a01b03168c602001517f49628fd1471006c1482da88028e9ce4dbb080b815c9b0344d39e5a8e6ec1419f8860200151858d8f6040516121c5949392919093845291151560208401526040830152606082015260800190565b60405180910390a45050505050505095945050505050565b5f5f5f5a84519091506121f086826129b2565b6121f98661155e565b6020860152604081015160608201516080830151171760e087013517610100870135176effffffffffffffffffffffffffffff81111561227b5760405162461bcd60e51b815260206004820152601860248201527f41413934206761732076616c756573206f766572666c6f77000000000000000060448201526064016106b0565b5f5f61228684612aa8565b90506122948a8a8a84612af3565b855160208701519199509193506122ab9190612d70565b6123015789604051631101335b60e11b81526004016106b0918152604060208201819052601a908201527f4141323520696e76616c6964206163636f756e74206e6f6e6365000000000000606082015260800190565b612309435f52565b60a08401516060906001600160a01b0316156123315761232c8b8b8b8587612dbd565b975090505b5f5a87039050808b60a001351015612395578b604051631101335b60e11b81526004016106b0918152604060208201819052601e908201527f41413430206f76657220766572696669636174696f6e4761734c696d69740000606082015260800190565b60408a018390528160608b015260c08b01355a8803018a608001818152505050505050505050935093915050565b5f5f6123ce85612ff5565b91509150816001600160a01b0316836001600160a01b03161461243d5785604051631101335b60e11b81526004016106b09181526040602082018190526014908201527f41413234207369676e6174757265206572726f72000000000000000000000000606082015260800190565b80156124955785604051631101335b60e11b81526004016106b09181526040602082018190526017908201527f414132322065787069726564206f72206e6f7420647565000000000000000000606082015260800190565b5f61249f85612ff5565b925090506001600160a01b038116156125045786604051631101335b60e11b81526004016106b09181526040602082018190526014908201527f41413334207369676e6174757265206572726f72000000000000000000000000606082015260800190565b81156125825786604051631101335b60e11b81526004016106b09181526040602082018190526021908201527f41413332207061796d61737465722065787069726564206f72206e6f7420647560608201527f6500000000000000000000000000000000000000000000000000000000000000608082015260a00190565b50505050505050565b5f5f5a90505f61259c846060015190565b905030631d7327566125b16060880188613c25565b87856040518563ffffffff1660e01b81526004016125d2949392919061429d565b6020604051808303815f875af192505050801561260c575060408051601f3d908101601f1916820190925261260991810190614354565b60015b6126c6575f60205f5f3e505f517f215221530000000000000000000000000000000000000000000000000000000081016126925786604051631101335b60e11b81526004016106b0918152604060208201819052600f908201527f41413935206f7574206f66206761730000000000000000000000000000000000606082015260800190565b5f85608001515a6126a39086613b9a565b6126ad9190613ac0565b90506126bd886002888685611eb6565b945050506126c9565b92505b50509392505050565b6001600160a01b0382166127285760405162461bcd60e51b815260206004820152601860248201527f4141393020696e76616c69642062656e6566696369617279000000000000000060448201526064016106b0565b5f826001600160a01b0316826040515f6040518083038185875af1925050503d805f8114612771576040519150601f19603f3d011682016040523d82523d5f602084013e612776565b606091505b50509050806127c75760405162461bcd60e51b815260206004820152601f60248201527f41413931206661696c65642073656e6420746f2062656e65666963696172790060448201526064016106b0565b505050565b5f6127d682613044565b805190602001209050919050565b3063957122ab6127f76040840184613c25565b612804602086018661390d565b612812610120870187613c25565b6040518663ffffffff1660e01b815260040161283295949392919061436b565b5f6040518083038186803b158015612848575f5ffd5b505afa925050508015612859575060015b6128b65761286561417c565b806308c379a0036128ac5750612879614195565b8061288457506128ae565b8051156128a8575f81604051631101335b60e11b81526004016106b0929190613b19565b5050565b505b3d5f5f3e3d5ffd5b50565b604080516060810182525f80825260208201819052918101829052906128de84613114565b90505f6128ea84613114565b82519091506001600160a01b038116612901575080515b602080840151604080860151928501519085015191929165ffffffffffff808316908516101561292f578193505b8065ffffffffffff168365ffffffffffff16111561294b578092505b5050604080516060810182526001600160a01b03909416845265ffffffffffff92831660208501529116908201529250505092915050565b60c081015160e08201515f919080820361299e575092915050565b6129aa82488301613183565b949350505050565b6129bf602083018361390d565b6001600160a01b0316815260208083013590820152608080830135604083015260a0830135606083015260c0808401359183019190915260e0808401359183019190915261010083013590820152365f612a1d610120850185613c25565b90925090508015612a9c576014811015612a795760405162461bcd60e51b815260206004820152601d60248201527f4141393320696e76616c6964207061796d6173746572416e644461746100000060448201526064016106b0565b612a8660145f8385613e8f565b612a8f91613eb6565b60601c60a0840152610e98565b5f60a084015250505050565b60a08101515f9081906001600160a01b0316612ac5576001612ac8565b60035b60ff1690505f8360800151828560600151028560400151010190508360c00151810292505050919050565b5f5f5f5a8551805191925090612b168988612b1160408c018c613c25565b61319a565b60a0820151612b23435f52565b5f6001600160a01b038216612b6f576001600160a01b0383165f908152602081905260409020546dffffffffffffffffffffffffffff16888111612b6957808903612b6b565b5f5b9150505b606084015160208a01516040517f3a871cdd0000000000000000000000000000000000000000000000000000000081526001600160a01b03861692633a871cdd929091612bc2918f9187906004016143a0565b6020604051808303815f8887f193505050508015612bfd575060408051601f3d908101601f19168201909252612bfa91810190614354565b60015b612c8e57612c0961417c565b806308c379a003612c3a5750612c1d614195565b80612c285750612c3c565b8b8160405160200161204891906143c4565b505b8a604051631101335b60e11b81526004016106b09181526040602082018190526016908201527f4141323320726576657274656420286f72204f4f472900000000000000000000606082015260800190565b95506001600160a01b038216612d5d576001600160a01b0383165f90815260208190526040902080546dffffffffffffffffffffffffffff16808a1115612d21578c604051631101335b60e11b81526004016106b09181526040602082018190526017908201527f41413231206469646e2774207061792070726566756e64000000000000000000606082015260800190565b81547fffffffffffffffffffffffffffffffffffff000000000000000000000000000016908a90036dffffffffffffffffffffffffffff161790555b5a85039650505050505094509492505050565b6001600160a01b0382165f90815260016020908152604080832084821c808552925282208054849167ffffffffffffffff8316919085612daf83613ad3565b909155501495945050505050565b825160608181015190915f91848111612e185760405162461bcd60e51b815260206004820152601f60248201527f4141343120746f6f206c6974746c6520766572696669636174696f6e4761730060448201526064016106b0565b60a08201516001600160a01b0381165f90815260208190526040902080548784039291906dffffffffffffffffffffffffffff1689811015612ea6578c604051631101335b60e11b81526004016106b0918152604060208201819052601e908201527f41413331207061796d6173746572206465706f73697420746f6f206c6f770000606082015260800190565b898103825f015f6101000a8154816dffffffffffffffffffffffffffff02191690836dffffffffffffffffffffffffffff160217905550826001600160a01b031663f465c77e858e8e602001518e6040518563ffffffff1660e01b8152600401612f12939291906143a0565b5f604051808303815f8887f193505050508015612f5057506040513d5f823e601f3d908101601f19168201604052612f4d91908101906143f5565b60015b612fe157612f5c61417c565b806308c379a003612f8d5750612f70614195565b80612f7b5750612f8f565b8d81604051602001612048919061447b565b505b8c604051631101335b60e11b81526004016106b09181526040602082018190526016908201527f4141333320726576657274656420286f72204f4f472900000000000000000000606082015260800190565b909e909d509b505050505050505050505050565b5f5f825f0361300857505f928392509050565b5f61301284613114565b9050806040015165ffffffffffff164211806130395750806020015165ffffffffffff1642105b905194909350915050565b6060813560208301355f61306361305e6040870187613c25565b61346d565b90505f61307661305e6060880188613c25565b9050608086013560a087013560c088013560e08901356101008a01355f6130a461305e6101208e018e613c25565b604080516001600160a01b039c909c1660208d01528b81019a909a5260608b019890985250608089019590955260a088019390935260c087019190915260e08601526101008501526101208401526101408084019190915281518084039091018152610160909201905292915050565b604080516060810182525f80825260208201819052918101919091528160a081901c65ffffffffffff81165f0361314e575065ffffffffffff5b604080516060810182526001600160a01b03909316835260d09490941c602083015265ffffffffffff16928101929092525090565b5f8183106131915781613193565b825b9392505050565b8015610e98578251516001600160a01b0381163b156132055784604051631101335b60e11b81526004016106b0918152604060208201819052601f908201527f414131302073656e64657220616c726561647920636f6e737472756374656400606082015260800190565b8351606001516040517f570e1a360000000000000000000000000000000000000000000000000000000081525f916001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169163570e1a3691906132759088908890600401613f1c565b6020604051808303815f8887f1158015613291573d5f5f3e3d5ffd5b50505050506040513d601f19601f820116820180604052508101906132b69190613f2f565b90506001600160a01b0381166133185785604051631101335b60e11b81526004016106b0918152604060208201819052601b908201527f4141313320696e6974436f6465206661696c6564206f72204f4f470000000000606082015260800190565b816001600160a01b0316816001600160a01b0316146133825785604051631101335b60e11b81526004016106b091815260406020808301829052908201527f4141313420696e6974436f6465206d7573742072657475726e2073656e646572606082015260800190565b806001600160a01b03163b5f036133e45785604051631101335b60e11b81526004016106b091815260406020808301829052908201527f4141313520696e6974436f6465206d757374206372656174652073656e646572606082015260800190565b5f6133f26014828688613e8f565b6133fb91613eb6565b60601c9050826001600160a01b031686602001517fd51a9c61267aa6196961883ecf5ff2da6619c37dac0fa92122513fb32c032d2d83895f015160a0015160405161345c9291906001600160a01b0392831681529116602082015260400190565b60405180910390a350505050505050565b5f604051828085833790209392505050565b6040518060a001604052806134dc6040518061010001604052805f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f6001600160a01b031681526020015f81526020015f81525090565b81526020015f81526020015f81526020015f81526020015f81525090565b5f6020828403121561350a575f5ffd5b813563ffffffff81168114613193575f5ffd5b803577ffffffffffffffffffffffffffffffffffffffffffffffff81168114613544575f5ffd5b919050565b5f60208284031215613559575f5ffd5b6131938261351d565b6001600160a01b03811681146128b6575f5ffd5b803561354481613562565b5f5f60408385031215613592575f5ffd5b823561359d81613562565b91506135ab6020840161351d565b90509250929050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b60a0810181811067ffffffffffffffff82111715613601576136016135b4565b60405250565b610100810181811067ffffffffffffffff82111715613601576136016135b4565b601f19601f830116810181811067ffffffffffffffff8211171561364e5761364e6135b4565b6040525050565b5f67ffffffffffffffff82111561366e5761366e6135b4565b50601f01601f191660200190565b5f81830361018081121561368e575f5ffd5b60405161369a816135e1565b8092506101008212156136ab575f5ffd5b60405191506136b982613607565b6136c284613576565b8252602084810135908301526040808501359083015260608085013590830152608080850135908301526136f860a08501613576565b60a083015260c0848101359083015260e0808501359083015290815261010083013560208201526101208301356040820152610140830135606082015261016090920135608090920191909152919050565b5f5f83601f84011261375a575f5ffd5b50813567ffffffffffffffff811115613771575f5ffd5b602083019150836020828501011115613788575f5ffd5b9250929050565b5f5f5f5f6101c085870312156137a3575f5ffd5b843567ffffffffffffffff8111156137b9575f5ffd5b8501601f810187136137c9575f5ffd5b80356137d481613655565b6040516137e18282613628565b8281528960208486010111156137f5575f5ffd5b826020850160208301375f602084830101528097505050505061381b866020870161367c565b92506101a085013567ffffffffffffffff811115613837575f5ffd5b6138438782880161374a565b95989497509550505050565b5f5f83601f84011261385f575f5ffd5b50813567ffffffffffffffff811115613876575f5ffd5b6020830191508360208260051b8501011115613788575f5ffd5b5f5f5f604084860312156138a2575f5ffd5b833567ffffffffffffffff8111156138b8575f5ffd5b6138c48682870161384f565b90945092505060208401356138d881613562565b809150509250925092565b5f5f604083850312156138f4575f5ffd5b82356138ff81613562565b946020939093013593505050565b5f6020828403121561391d575f5ffd5b813561319381613562565b5f5f5f5f5f6060868803121561393c575f5ffd5b853567ffffffffffffffff811115613952575f5ffd5b61395e8882890161374a565b909650945050602086013561397281613562565b9250604086013567ffffffffffffffff81111561398d575f5ffd5b6139998882890161374a565b969995985093965092949392505050565b5f5f602083850312156139bb575f5ffd5b823567ffffffffffffffff8111156139d1575f5ffd5b6139dd8582860161374a565b90969095509350505050565b5f61016082840312156139fa575f5ffd5b50919050565b5f60208284031215613a10575f5ffd5b813567ffffffffffffffff811115613a26575f5ffd5b6129aa848285016139e9565b5f5f5f5f60608587031215613a45575f5ffd5b843567ffffffffffffffff811115613a5b575f5ffd5b613a67878288016139e9565b9450506020850135613a7881613562565b9250604085013567ffffffffffffffff811115613837575f5ffd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b80820180821115610f0557610f05613a93565b5f5f198203613ae457613ae4613a93565b5060010190565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b828152604060208201525f6129aa6040830184613aeb565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f82357ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffea1833603018112613b90575f5ffd5b9190910192915050565b81810381811115610f0557610f05613a93565b5f82357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa1833603018112613b90575f5ffd5b5f5f8335601e19843603018112613bf4575f5ffd5b83018035915067ffffffffffffffff821115613c0e575f5ffd5b6020019150600581901b3603821315613788575f5ffd5b5f5f8335601e19843603018112613c3a575f5ffd5b83018035915067ffffffffffffffff821115613c54575f5ffd5b602001915036819003821315613788575f5ffd5b5f5f8335601e19843603018112613c7d575f5ffd5b830160208101925035905067ffffffffffffffff811115613c9c575f5ffd5b803603821315613788575f5ffd5b81835281816020850137505f602082840101525f6020601f19601f840116840101905092915050565b613ced82613ce083613576565b6001600160a01b03169052565b602081810135908301525f613d056040830183613c68565b6101606040860152613d1c61016086018284613caa565b915050613d2c6060840184613c68565b8583036060870152613d3f838284613caa565b6080868101359088015260a0808701359088015260c0808701359088015260e0808701359088015261010080870135908801529250613d85915050610120840184613c68565b858303610120870152613d99838284613caa565b92505050613dab610140840184613c68565b858303610140870152613dbf838284613caa565b9695505050505050565b604080825281018490525f6060600586901b8301810190830187837ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffea136839003015b89821015613e6d577fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa08786030184528235818112613e47575f5ffd5b613e53868d8301613cd3565b955050602083019250602084019350600182019150613e0b565b505050508281036020840152613e84818587613caa565b979650505050505050565b5f5f85851115613e9d575f5ffd5b83861115613ea9575f5ffd5b5050820193919092039150565b80357fffffffffffffffffffffffffffffffffffffffff0000000000000000000000008116906014841015613f15577fffffffffffffffffffffffffffffffffffffffff000000000000000000000000808560140360031b1b82161691505b5092915050565b602081525f6129aa602083018486613caa565b5f60208284031215613f3f575f5ffd5b815161319381613562565b65ffffffffffff8181168382160190811115610f0557610f05613a93565b818382375f9101908152919050565b86815285602082015265ffffffffffff8516604082015265ffffffffffff84166060820152821515608082015260c060a08201525f613fb960c0830184613aeb565b98975050505050505050565b805182526020810151602083015260408101511515604083015265ffffffffffff606082015116606083015265ffffffffffff60808201511660808301525f60a082015160c060a08501526129aa60c0850182613aeb565b61014081525f614031610140830188613fc5565b905061404a602083018780518252602090810151910152565b845160608301526020948501516080830152835160a08301529284015160c082015281516001600160a01b031660e0820152908301518051610100830152909201516101209092019190915292915050565b60e081525f6140ae60e0830187613fc5565b90506140c7602083018680518252602090810151910152565b8351606083015260208401516080830152825160a0830152602083015160c083015295945050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b5f60038510614155577f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b8482526060602083015261416c6060830185613aeb565b9050826040830152949350505050565b5f60033d11156141925760045f5f3e505f5160e01c5b90565b5f60443d10156141a25790565b6040517ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3d016004823e80513d602482011167ffffffffffffffff821117156141ea57505090565b808201805167ffffffffffffffff811115614206575050505090565b7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3d850101602082840101111561423e575050505090565b61424d60208285010185613628565b509392505050565b5f81518060208401855e5f93019283525090919050565b7f4141353020706f73744f702072657665727465643a200000000000000000000081525f6131936016830184614255565b6101c081525f6142b26101c083018688613caa565b84516001600160a01b038151166020850152602081015160408501526040810151606085015260608101516080850152608081015160a08501526001600160a01b0360a08201511660c085015260c081015160e085015260e08101516101008501525060208501516101208401526040850151610140840152606085015161016084015260808501516101808401528281036101a0840152613e848185613aeb565b5f60208284031215614364575f5ffd5b5051919050565b606081525f61437e606083018789613caa565b6001600160a01b03861660208401528281036040840152613fb9818587613caa565b606081525f6143b26060830186613cd3565b60208301949094525060400152919050565b7f414132332072657665727465643a20000000000000000000000000000000000081525f613193600f830184614255565b5f5f60408385031215614406575f5ffd5b825167ffffffffffffffff81111561441c575f5ffd5b8301601f8101851361442c575f5ffd5b805161443781613655565b6040516144448282613628565b828152876020848601011115614458575f5ffd5b8260208501602083015e5f60209382018401529590910151949694955050505050565b7f414133332072657665727465643a20000000000000000000000000000000000081525f613193600f83018461425556fea2646970667358221220e0c34964218a71842f0d2ad4b72b88b8cf5140c40a0f645e6582e86f75cf58f764736f6c634300081b00336080604052348015600e575f5ffd5b506102228061001c5f395ff3fe608060405234801561000f575f5ffd5b5060043610610029575f3560e01c8063570e1a361461002d575b5f5ffd5b61004061003b3660046100f1565b610069565b60405173ffffffffffffffffffffffffffffffffffffffff909116815260200160405180910390f35b5f80610078601482858761015f565b61008191610186565b60601c90505f610094846014818861015f565b8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f92018290525084519495509360209350849250905082850182875af190505f519350806100e8575f93505b50505092915050565b5f5f60208385031215610102575f5ffd5b823567ffffffffffffffff811115610118575f5ffd5b8301601f81018513610128575f5ffd5b803567ffffffffffffffff81111561013e575f5ffd5b85602082840101111561014f575f5ffd5b6020919091019590945092505050565b5f5f8585111561016d575f5ffd5b83861115610179575f5ffd5b5050820193919092039150565b80357fffffffffffffffffffffffffffffffffffffffff00000000000000000000000081169060148410156101e5577fffffffffffffffffffffffffffffffffffffffff000000000000000000000000808560140360031b1b82161691505b509291505056fea2646970667358221220b5cd58846963fdd3206a7f7a8eb77f890f52b2efc4699d857608509669f6a39c64736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xA0`@R`@Q`\x0E\x90`GV[`@Q\x80\x91\x03\x90_\xF0\x80\x15\x80\x15`&W=__>=_\xFD[P`\x01`\x01`\xA0\x1B\x03\x16`\x80R4\x80\x15`=W__\xFD[P`\x01`\x02U`TV[a\x02>\x80aEU\x839\x01\x90V[`\x80QaD\xE2a\0s_9_\x81\x81a\x14\xA7\x01Ra2=\x01RaD\xE2_\xF3\xFE`\x80`@R`\x046\x10a\x01ZW_5`\xE0\x1C\x80c\x8FA\xECZ\x11a\0\xBBW\x80c\xBB\x9F\xE6\xBF\x11a\0qW\x80c\xD68?\x94\x11a\0WW\x80c\xD68?\x94\x14a\x04\xD0W\x80c\xEE!\x94#\x14a\x04\xEFW\x80c\xFC~(m\x14a\x05\x0EW__\xFD[\x80c\xBB\x9F\xE6\xBF\x14a\x04\x9DW\x80c\xC2:\\\xEA\x14a\x04\xB1W__\xFD[\x80c\x9B$\x9Fi\x11a\0\xA1W\x80c\x9B$\x9Fi\x14a\x04LW\x80c\xA6\x1951\x14a\x04kW\x80c\xB7`\xFA\xF9\x14a\x04\x8AW__\xFD[\x80c\x8FA\xECZ\x14a\x04\x19W\x80c\x95q\"\xAB\x14a\x04-W__\xFD[\x80c \\(x\x11a\x01\x10W\x80cK\x1D|\xF5\x11a\0\xF6W\x80cK\x1D|\xF5\x14a\x02eW\x80cR\x87\xCE\x12\x14a\x02\x84W\x80cp\xA0\x821\x14a\x03\xD5W__\xFD[\x80c \\(x\x14a\x02'W\x80c5V~\x1A\x14a\x02FW__\xFD[\x80c\x1B.\x01\xB8\x11a\x01@W\x80c\x1B.\x01\xB8\x14a\x01\xA0W\x80c\x1Ds'V\x14a\x01\xE9W\x80c\x1F\xAD\x94\x8C\x14a\x02\x08W__\xFD[\x80c\x03\x96\xCB`\x14a\x01nW\x80c\x0B\xD2\x8E;\x14a\x01\x81W__\xFD[6a\x01jWa\x01h3a\x05\xE6V[\0[__\xFD[a\x01ha\x01|6`\x04a4\xFAV[a\x06SV[4\x80\x15a\x01\x8CW__\xFD[Pa\x01ha\x01\x9B6`\x04a5IV[a\tjV[4\x80\x15a\x01\xABW__\xFD[Pa\x01\xD6a\x01\xBA6`\x04a5\x81V[`\x01` \x90\x81R_\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xF4W__\xFD[Pa\x01\xD6a\x02\x036`\x04a7\x8FV[a\t\xB1V[4\x80\x15a\x02\x13W__\xFD[Pa\x01ha\x02\"6`\x04a8\x90V[a\x0B1V[4\x80\x15a\x022W__\xFD[Pa\x01ha\x02A6`\x04a8\xE3V[a\x0C\xEFV[4\x80\x15a\x02QW__\xFD[Pa\x01\xD6a\x02`6`\x04a5\x81V[a\x0E\x9EV[4\x80\x15a\x02pW__\xFD[Pa\x01ha\x02\x7F6`\x04a8\x90V[a\x0F\x0BV[4\x80\x15a\x02\x8FW__\xFD[Pa\x03da\x02\x9E6`\x04a9\rV[`@\x80Q`\xA0\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x91\x90\x91RP`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R` \x81\x81R`@\x91\x82\x90 \x82Q`\xA0\x81\x01\x84R\x81Tm\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x16\x83Rn\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x04`\xFF\x16\x15\x15\x94\x83\x01\x94\x90\x94Ro\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04\x90\x92\x16\x92\x82\x01\x92\x90\x92R`\x01\x90\x91\x01Tc\xFF\xFF\xFF\xFF\x81\x16``\x83\x01Rd\x01\0\0\0\0\x90\x04e\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x82\x01R\x90V[`@Qa\x01\xE0\x91\x90_`\xA0\x82\x01\x90Pm\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83Q\x16\x82R` \x83\x01Q\x15\x15` \x83\x01Rm\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@\x84\x01Q\x16`@\x83\x01Rc\xFF\xFF\xFF\xFF``\x84\x01Q\x16``\x83\x01Re\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x84\x01Q\x16`\x80\x83\x01R\x92\x91PPV[4\x80\x15a\x03\xE0W__\xFD[Pa\x01\xD6a\x03\xEF6`\x04a9\rV[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R` \x81\x90R`@\x90 Tm\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[4\x80\x15a\x04$W__\xFD[Pa\x01\xD6`\x01\x81V[4\x80\x15a\x048W__\xFD[Pa\x01ha\x04G6`\x04a9(V[a\x13{V[4\x80\x15a\x04WW__\xFD[Pa\x01ha\x04f6`\x04a9\xAAV[a\x14uV[4\x80\x15a\x04vW__\xFD[Pa\x01\xD6a\x04\x856`\x04a:\0V[a\x15^V[a\x01ha\x04\x986`\x04a9\rV[a\x05\xE6V[4\x80\x15a\x04\xA8W__\xFD[Pa\x01ha\x15\x9FV[4\x80\x15a\x04\xBCW__\xFD[Pa\x01ha\x04\xCB6`\x04a9\rV[a\x17 V[4\x80\x15a\x04\xDBW__\xFD[Pa\x01ha\x04\xEA6`\x04a:2V[a\x19\x99V[4\x80\x15a\x04\xFAW__\xFD[Pa\x01ha\x05\t6`\x04a:\0V[a\x1A\xA1V[4\x80\x15a\x05\x19W__\xFD[Pa\x05\x99a\x05(6`\x04a9\rV[_` \x81\x90R\x90\x81R`@\x90 \x80T`\x01\x90\x91\x01Tm\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x83\x16\x92n\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x04`\xFF\x16\x92o\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x04\x90\x91\x16\x90c\xFF\xFF\xFF\xFF\x81\x16\x90d\x01\0\0\0\0\x90\x04e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85V[`@\x80Qm\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x96\x87\x16\x81R\x94\x15\x15` \x86\x01R\x92\x90\x94\x16\x91\x83\x01\x91\x90\x91Rc\xFF\xFF\xFF\xFF\x16``\x82\x01Re\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16`\x80\x82\x01R`\xA0\x01a\x01\xE0V[a\x05\xF0\x814a\x1D\xA0V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x81\x81R` \x81\x81R`@\x91\x82\x90 \x80T\x92Qm\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x93\x16\x83R\x92\x91\x7F-\xA4f\xA7\xB2C\x04\xF4~\x87\xFA.\x1EZ\x81\xB9\x83\x1C\xE5O\xEC\x19\x05\\\xE2w\xCA/9\xBAB\xC4\x91\x01[`@Q\x80\x91\x03\x90\xA2PPV[3_\x90\x81R` \x81\x90R`@\x90 c\xFF\xFF\xFF\xFF\x82\x16a\x06\xB9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7Fmust specify unstake delay\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\x01\x81\x01Tc\xFF\xFF\xFF\xFF\x90\x81\x16\x90\x83\x16\x10\x15a\x07\x17W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7Fcannot decrease unstake time\0\0\0\0`D\x82\x01R`d\x01a\x06\xB0V[\x80T_\x90a\x07I\x904\x90o\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04m\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a:\xC0V[\x90P_\x81\x11a\x07\x9AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01R\x7Fno stake specified\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06\xB0V[m\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07\xF8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01R\x7Fstake overflow\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06\xB0V[`@\x80Q`\xA0\x81\x01\x82R\x83Tm\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x82R`\x01` \x80\x84\x01\x82\x81R\x86\x84\x16\x85\x87\x01\x90\x81Rc\xFF\xFF\xFF\xFF\x80\x8B\x16``\x88\x01\x90\x81R_`\x80\x89\x01\x81\x81R3\x80\x83R\x96\x82\x90R\x90\x8A\x90 \x98Q\x89T\x95Q\x94Q\x89\x16o\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x02\x7F\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x95\x15\x15n\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x97\x16\x91\x90\x99\x16\x17\x94\x90\x94\x17\x92\x90\x92\x16\x95\x90\x95\x17\x86UQ\x94\x90\x92\x01\x80T\x92Qe\xFF\xFF\xFF\xFF\xFF\xFF\x16d\x01\0\0\0\0\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\x90\x93\x16\x94\x90\x93\x16\x93\x90\x93\x17\x17\x90U\x90Q\x7F\xA5\xAE\x83=\x0B\xB1\xDC\xD62\xD9\x8A\x8Bp\x97>\x85\x16\x81(\x98\xE1\x9B\xF2{p\x07\x1E\xBC\x8D\xC5,\x01\x90a\t]\x90\x84\x90\x87\x90\x91\x82Rc\xFF\xFF\xFF\xFF\x16` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA2PPPV[3_\x90\x81R`\x01` \x90\x81R`@\x80\x83 w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x84R\x90\x91R\x81 \x80T\x91a\t\xA9\x83a:\xD3V[\x91\x90PUPPV[__Z\x90P30\x14a\n\x05W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FAA92 internal call only\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06\xB0V[\x84Q`@\x81\x01Q``\x82\x01Q\x81\x01a\x13\x88\x01Z\x10\x15a\nFW\x7F\xDE\xAD\xDE\xAD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R` _\xFD[\x87Q_\x90\x15a\n\xD4W_a\n_\x84_\x01Q_\x8C\x86a\x1EuV[\x90P\x80a\n\xD2W_a\nra\x08\0a\x1E\x8BV[\x80Q\x90\x91P\x15a\n\xCCW\x84_\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x8A` \x01Q\x7F\x1CO\xAD\xA77L\n\x9E\xE8\x84\x1F\xC3\x8A\xFE\x82\x93-\xC0\xF8\xE6\x90\x12\xE9'\xF0a\xA8\xBA\xE6\x11\xA2\x01\x87` \x01Q\x84`@Qa\n\xC3\x92\x91\x90a;\x19V[`@Q\x80\x91\x03\x90\xA3[`\x01\x92PP[P[_\x88`\x80\x01QZ\x86\x03\x01\x90Pa\x0B#_\x83\x8B\x8B\x8B\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RP\x88\x92Pa\x1E\xB6\x91PPV[\x9A\x99PPPPPPPPPPV[`\x02\x80T\x03a\x0B\x82W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01a\x06\xB0V[`\x02\x80U\x81_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0B\xA1Wa\x0B\xA1a5\xB4V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0B\xDAW\x81` \x01[a\x0B\xC7a4\x7FV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x0B\xBFW\x90P[P\x90P_[\x82\x81\x10\x15a\x0COW_\x82\x82\x81Q\x81\x10a\x0B\xFAWa\x0B\xFAa;1V[` \x02` \x01\x01Q\x90P__a\x0C4\x84\x8A\x8A\x87\x81\x81\x10a\x0C\x1CWa\x0C\x1Ca;1V[\x90P` \x02\x81\x01\x90a\x0C.\x91\x90a;^V[\x85a!\xDDV[\x91P\x91Pa\x0CD\x84\x83\x83_a#\xC3V[PPP`\x01\x01a\x0B\xDFV[P`@Q_\x90\x7F\xBBG\xEE>\x18:U\x8B\x1A/\xF0\x87K\x07\x9F?\xC5G\x8BtT\xEA\xCF+\xFCZ\xF2\xFFXx\xF9r\x90\x82\x90\xA1_[\x83\x81\x10\x15a\x0C\xD7Wa\x0C\xCB\x81\x88\x88\x84\x81\x81\x10a\x0C\x9AWa\x0C\x9Aa;1V[\x90P` \x02\x81\x01\x90a\x0C\xAC\x91\x90a;^V[\x85\x84\x81Q\x81\x10a\x0C\xBEWa\x0C\xBEa;1V[` \x02` \x01\x01Qa%\x8BV[\x90\x91\x01\x90`\x01\x01a\x0C|V[Pa\x0C\xE2\x84\x82a&\xD2V[PP`\x01`\x02UPPPPV[3_\x90\x81R` \x81\x90R`@\x90 \x80Tm\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82\x11\x15a\r^W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FWithdraw amount too large\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06\xB0V[\x80Ta\r{\x90\x83\x90m\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a;\x9AV[\x81T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16m\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x90\x91\x16\x17\x81U`@\x80Q`\x01`\x01`\xA0\x1B\x03\x85\x16\x81R` \x81\x01\x84\x90R3\x91\x7F\xD1\xC1\x9F\xBC\xD4U\x1A^\xDF\xB6mC\xD2\xE37\xC0H7\xAF\xDA4\x82\xB4+\xDFV\x9A\x8F\xCC\xDA\xE5\xFB\x91\x01`@Q\x80\x91\x03\x90\xA2_\x83`\x01`\x01`\xA0\x1B\x03\x16\x83`@Q_`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80_\x81\x14a\x0EBW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x0EGV[``\x91P[PP\x90P\x80a\x0E\x98W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01R\x7Ffailed to withdraw\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06\xB0V[PPPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x01` \x90\x81R`@\x80\x83 w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x84R\x90\x91R\x90\x81\x90 T\x90\x82\x90\x1B\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x16\x17[\x92\x91PPV[`\x02\x80T\x03a\x0F\\W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01a\x06\xB0V[`\x02\x80U\x81_\x80[\x82\x81\x10\x15a\x10\xE0W6\x86\x86\x83\x81\x81\x10a\x0F\x7FWa\x0F\x7Fa;1V[\x90P` \x02\x81\x01\x90a\x0F\x91\x91\x90a;\xADV[\x90P6_a\x0F\x9F\x83\x80a;\xDFV[\x90\x92P\x90P_a\x0F\xB5`@\x85\x01` \x86\x01a9\rV[\x90P_\x19`\x01`\x01`\xA0\x1B\x03\x82\x16\x01a\x10\x10W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FAA96 invalid aggregator\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06\xB0V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\x10\xC4W`\x01`\x01`\xA0\x1B\x03\x81\x16c\xE3V:O\x84\x84a\x10=`@\x89\x01\x89a<%V[`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x10\\\x94\x93\x92\x91\x90a=\xC9V[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x10rW__\xFD[PZ\xFA\x92PPP\x80\x15a\x10\x83WP`\x01[a\x10\xC4W`@Q\x7F\x86\xA9\xF7P\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x06\xB0V[a\x10\xCE\x82\x87a:\xC0V[\x95PP`\x01\x90\x93\x01\x92Pa\x0Fd\x91PPV[P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x10\xFBWa\x10\xFBa5\xB4V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x114W\x81` \x01[a\x11!a4\x7FV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x11\x19W\x90P[P`@Q\x90\x91P\x7F\xBBG\xEE>\x18:U\x8B\x1A/\xF0\x87K\x07\x9F?\xC5G\x8BtT\xEA\xCF+\xFCZ\xF2\xFFXx\xF9r\x90_\x90\xA1_\x80[\x84\x81\x10\x15a\x124W6\x88\x88\x83\x81\x81\x10a\x11~Wa\x11~a;1V[\x90P` \x02\x81\x01\x90a\x11\x90\x91\x90a;\xADV[\x90P6_a\x11\x9E\x83\x80a;\xDFV[\x90\x92P\x90P_a\x11\xB4`@\x85\x01` \x86\x01a9\rV[\x90P\x81_[\x81\x81\x10\x15a\x12\"W_\x89\x89\x81Q\x81\x10a\x11\xD4Wa\x11\xD4a;1V[` \x02` \x01\x01Q\x90P__a\x11\xF6\x8B\x89\x89\x87\x81\x81\x10a\x0C\x1CWa\x0C\x1Ca;1V[\x91P\x91Pa\x12\x06\x84\x83\x83\x89a#\xC3V[\x8Aa\x12\x10\x81a:\xD3V[\x9BPP`\x01\x90\x93\x01\x92Pa\x11\xB9\x91PPV[PP`\x01\x90\x94\x01\x93Pa\x11c\x92PPPV[P_\x90P\x80\x80[\x85\x81\x10\x15a\x137W6\x89\x89\x83\x81\x81\x10a\x12VWa\x12Va;1V[\x90P` \x02\x81\x01\x90a\x12h\x91\x90a;\xADV[\x90Pa\x12z`@\x82\x01` \x83\x01a9\rV[`\x01`\x01`\xA0\x1B\x03\x16\x7FW_\xF3\xAC\xAD\xD5\xAB4\x8F\xE1\x85^!~\x0F6x\xF8\xD7g\xD7IL\x9F\x9F\xEF\xBE\xE2\xE1|\xCAM`@Q`@Q\x80\x91\x03\x90\xA26_a\x12\xBB\x83\x80a;\xDFV[\x90\x92P\x90P\x80_[\x81\x81\x10\x15a\x13&Wa\x13\x05\x88\x85\x85\x84\x81\x81\x10a\x12\xE1Wa\x12\xE1a;1V[\x90P` \x02\x81\x01\x90a\x12\xF3\x91\x90a;^V[\x8B\x8B\x81Q\x81\x10a\x0C\xBEWa\x0C\xBEa;1V[a\x13\x0F\x90\x88a:\xC0V[\x96P\x87a\x13\x1B\x81a:\xD3V[\x98PP`\x01\x01a\x12\xC3V[PP`\x01\x90\x93\x01\x92Pa\x12;\x91PPV[P`@Q_\x90\x7FW_\xF3\xAC\xAD\xD5\xAB4\x8F\xE1\x85^!~\x0F6x\xF8\xD7g\xD7IL\x9F\x9F\xEF\xBE\xE2\xE1|\xCAM\x90\x82\x90\xA2a\x13l\x86\x82a&\xD2V[PP`\x01`\x02UPPPPPPV[\x83\x15\x80\x15a\x13\x91WP`\x01`\x01`\xA0\x1B\x03\x83\x16;\x15[\x15a\x13\xDEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FAA20 account not deployed\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06\xB0V[`\x14\x81\x10a\x14TW_a\x13\xF4`\x14\x82\x84\x86a>\x8FV[a\x13\xFD\x91a>\xB6V[``\x1C\x90P\x80;_\x03a\x14RW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FAA30 paymaster not deployed\0\0\0\0\0`D\x82\x01R`d\x01a\x06\xB0V[P[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R_`$\x82\x01R`D\x01a\x06\xB0V[`@Q\x7FW\x0E\x1A6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R_\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cW\x0E\x1A6\x90a\x14\xDE\x90\x86\x90\x86\x90`\x04\x01a?\x1CV[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x14\xFAW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\x1E\x91\x90a?/V[`@Q\x7Fl\xA7\xB8\x06\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R\x90\x91P`$\x01a\x06\xB0V[_a\x15h\x82a'\xCCV[`@\x80Q` \x81\x01\x92\x90\x92R0\x90\x82\x01RF``\x82\x01R`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[3_\x90\x81R` \x81\x90R`@\x81 `\x01\x81\x01T\x90\x91c\xFF\xFF\xFF\xFF\x90\x91\x16\x90\x03a\x16\nW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\n`$\x82\x01R\x7Fnot staked\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06\xB0V[\x80Tn\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16a\x16mW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01R\x7Falready unstaking\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06\xB0V[`\x01\x81\x01T_\x90a\x16\x84\x90c\xFF\xFF\xFF\xFF\x16Ba?JV[`\x01\x83\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\xFF\xFF\xFF\xFF\x16d\x01\0\0\0\0e\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x90\x81\x02\x91\x90\x91\x17\x90\x91U\x83T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84U`@Q\x90\x81R\x90\x91P3\x90\x7F\xFA\x9B<\x14\xCC\x82\\A,\x9E\xD8\x1B;\xA3e\xA5\xB4YC\x94\x03\xF1\x88)\xE5r\xEDS\xA4\x18\x0F\n\x90` \x01a\x06GV[3_\x90\x81R` \x81\x90R`@\x90 \x80To\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04m\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x80a\x17\xA0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01R\x7FNo stake to withdraw\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06\xB0V[`\x01\x82\x01Td\x01\0\0\0\0\x90\x04e\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x18\x01W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7Fmust call unlockStake() first\0\0\0`D\x82\x01R`d\x01a\x06\xB0V[`\x01\x82\x01TBd\x01\0\0\0\0\x90\x91\x04e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15a\x18fW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FStake withdrawal is not due\0\0\0\0\0`D\x82\x01R`d\x01a\x06\xB0V[`\x01\x82\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\x16\x90U\x81T\x7F\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82U`@\x80Q`\x01`\x01`\xA0\x1B\x03\x85\x16\x81R` \x81\x01\x83\x90R3\x91\x7F\xB7\xC9\x18\xE0\xE2I\xF9\x99\xE9e\xCA\xFE\xB6\xC6d'\x1B?C\x17\xD2\x96F\x15\0\xE7\x1D\xA3\x9F\x0C\xBD\xA3\x91\x01`@Q\x80\x91\x03\x90\xA2_\x83`\x01`\x01`\xA0\x1B\x03\x16\x82`@Q_`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80_\x81\x14a\x19CW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x19HV[``\x91P[PP\x90P\x80a\x0E\x98W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7Ffailed to withdraw stake\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06\xB0V[a\x19\xA1a4\x7FV[a\x19\xAA\x85a'\xE4V[__a\x19\xB7_\x88\x85a!\xDDV[\x91P\x91P_a\x19\xC6\x83\x83a(\xB9V[\x90Pa\x19\xD0C_RV[_a\x19\xDC_\x8A\x87a%\x8BV[\x90Pa\x19\xE6C_RV[_```\x01`\x01`\xA0\x1B\x03\x8A\x16\x15a\x1AWW\x89`\x01`\x01`\xA0\x1B\x03\x16\x89\x89`@Qa\x1A\x12\x92\x91\x90a?hV[_`@Q\x80\x83\x03\x81_\x86Z\xF1\x91PP=\x80_\x81\x14a\x1AKW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x1APV[``\x91P[P\x90\x92P\x90P[\x86`\x80\x01Q\x83\x85` \x01Q\x86`@\x01Q\x85\x85`@Q\x7F\x8Bz\xC9\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x06\xB0\x96\x95\x94\x93\x92\x91\x90a?wV[a\x1A\xA9a4\x7FV[a\x1A\xB2\x82a'\xE4V[__a\x1A\xBF_\x85\x85a!\xDDV[\x84Q`\xA0\x01Q`@\x80Q\x80\x82\x01\x82R_\x80\x82R` \x80\x83\x01\x82\x81R`\x01`\x01`\xA0\x1B\x03\x95\x86\x16\x83R\x82\x82R\x84\x83 \x80Tm\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFo\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x82\x90\x04\x81\x16\x87R`\x01\x92\x83\x01Tc\xFF\xFF\xFF\xFF\x90\x81\x16\x90\x94R\x8DQQ\x88Q\x80\x8A\x01\x8AR\x87\x81R\x80\x87\x01\x88\x81R\x91\x90\x9A\x16\x87R\x86\x86R\x88\x87 \x80T\x93\x90\x93\x04\x90\x91\x16\x89R\x91\x01T\x90\x91\x16\x90R\x83Q\x80\x85\x01\x90\x94R\x81\x84R\x83\x01R\x93\x95P\x91\x93P\x906_a\x1B~`@\x8A\x01\x8Aa<%V[\x90\x92P\x90P_`\x14\x82\x10\x15a\x1B\x93W_a\x1B\xADV[a\x1B\xA0`\x14_\x84\x86a>\x8FV[a\x1B\xA9\x91a>\xB6V[``\x1C[`@\x80Q\x80\x82\x01\x82R_\x80\x82R` \x80\x83\x01\x82\x81R`\x01`\x01`\xA0\x1B\x03\x86\x16\x83R\x90\x82\x90R\x92\x90 \x80To\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04m\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82R`\x01\x01Tc\xFF\xFF\xFF\xFF\x16\x90\x91R\x90\x91P\x93PPPP_a\x1C\x1C\x86\x86a(\xB9V[\x90P_\x81_\x01Q\x90P_`\x01`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x14\x90P_`@Q\x80`\xC0\x01`@R\x80\x8B`\x80\x01Q\x81R` \x01\x8B`@\x01Q\x81R` \x01\x83\x15\x15\x81R` \x01\x85` \x01Qe\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x85`@\x01Qe\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01a\x1C\x95\x8C``\x01Q\x90V[\x90R\x90P`\x01`\x01`\xA0\x1B\x03\x83\x16\x15\x80\x15\x90a\x1C\xBBWP`\x01`\x01`\xA0\x1B\x03\x83\x16`\x01\x14\x15[\x15a\x1DfW`@\x80Q\x80\x82\x01\x82R`\x01`\x01`\xA0\x1B\x03\x85\x16\x80\x82R\x82Q\x80\x84\x01\x84R_\x80\x82R` \x80\x83\x01\x82\x81R\x93\x82R\x81\x81R\x90\x85\x90 \x80To\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04m\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83R`\x01\x01Tc\xFF\xFF\xFF\xFF\x16\x90\x92R\x90\x82\x01R\x90Q\x7F\xFA\xEC\xB4\xE4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Ra\x06\xB0\x90\x83\x90\x89\x90\x89\x90\x8C\x90\x86\x90`\x04\x01a@\x1DV[\x80\x86\x86\x89`@Q\x7F\xE0\xCF\xF0_\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x06\xB0\x94\x93\x92\x91\x90a@\x9CV[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R` \x81\x90R`@\x81 \x80T\x90\x91\x90a\x1D\xD7\x90\x84\x90m\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a:\xC0V[\x90Pm\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1E7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01R\x7Fdeposit overflow\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06\xB0V[\x81T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16m\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x90\x91\x16\x17\x90UPPV[___\x84Q` \x86\x01\x87\x89\x87\xF1\x95\x94PPPPPV[``=\x82\x81\x11\x15a\x1E\x99WP\x81[`@Q` \x82\x01\x81\x01`@R\x81\x81R\x81_` \x83\x01>\x93\x92PPPV[__Z\x85Q\x90\x91P_\x90\x81a\x1E\xCA\x82a)\x83V[`\xA0\x83\x01Q\x90\x91P`\x01`\x01`\xA0\x1B\x03\x81\x16a\x1E\xE9W\x82Q\x93Pa \xC4V[\x80\x93P_\x88Q\x11\x15a \xC4W\x86\x82\x02\x95P`\x02\x8A`\x02\x81\x11\x15a\x1F\x0EWa\x1F\x0Ea@\xF2V[\x14a\x1F\x94W``\x83\x01Q`@Q\x7F\xA9\xA24\t\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16\x91c\xA9\xA24\t\x91a\x1Fa\x90\x8E\x90\x8D\x90\x8C\x90`\x04\x01aA\x1FV[_`@Q\x80\x83\x03\x81_\x88\x80;\x15\x80\x15a\x1FxW__\xFD[P\x87\xF1\x15\x80\x15a\x1F\x8AW=__>=_\xFD[PPPPPa \xC4V[``\x83\x01Q`@Q\x7F\xA9\xA24\t\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16\x91c\xA9\xA24\t\x91a\x1F\xE2\x90\x8E\x90\x8D\x90\x8C\x90`\x04\x01aA\x1FV[_`@Q\x80\x83\x03\x81_\x88\x80;\x15\x80\x15a\x1F\xF9W__\xFD[P\x87\xF1\x93PPPP\x80\x15a \x0BWP`\x01[a \xC4Wa \x17aA|V[\x80c\x08\xC3y\xA0\x03a pWPa +aA\x95V[\x80a 6WPa rV[\x8B\x81`@Q` \x01a H\x91\x90aBlV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Rc\x11\x013[`\xE1\x1B\x82Ra\x06\xB0\x92\x91`\x04\x01a;\x19V[P[\x8A`@Qc\x11\x013[`\xE1\x1B\x81R`\x04\x01a\x06\xB0\x91\x81R`@` \x82\x01\x81\x90R`\x12\x90\x82\x01R\x7FAA50 postOp revert\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x82\x01R`\x80\x01\x90V[Z\x85\x03\x87\x01\x96P\x81\x87\x02\x95P\x85\x89`@\x01Q\x10\x15a!-W\x8A`@Qc\x11\x013[`\xE1\x1B\x81R`\x04\x01a\x06\xB0\x91\x81R`@` \x80\x83\x01\x82\x90R\x90\x82\x01R\x7FAA51 prefund below actualGasCost``\x82\x01R`\x80\x01\x90V[`@\x89\x01Q\x86\x90\x03a!?\x85\x82a\x1D\xA0V[_\x80\x8C`\x02\x81\x11\x15a!SWa!Sa@\xF2V[\x14\x90P\x84`\xA0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x85_\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x8C` \x01Q\x7FIb\x8F\xD1G\x10\x06\xC1H-\xA8\x80(\xE9\xCEM\xBB\x08\x0B\x81\\\x9B\x03D\xD3\x9EZ\x8En\xC1A\x9F\x88` \x01Q\x85\x8D\x8F`@Qa!\xC5\x94\x93\x92\x91\x90\x93\x84R\x91\x15\x15` \x84\x01R`@\x83\x01R``\x82\x01R`\x80\x01\x90V[`@Q\x80\x91\x03\x90\xA4PPPPPPP\x95\x94PPPPPV[___Z\x84Q\x90\x91Pa!\xF0\x86\x82a)\xB2V[a!\xF9\x86a\x15^V[` \x86\x01R`@\x81\x01Q``\x82\x01Q`\x80\x83\x01Q\x17\x17`\xE0\x87\x015\x17a\x01\0\x87\x015\x17n\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\"{W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FAA94 gas values overflow\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06\xB0V[__a\"\x86\x84a*\xA8V[\x90Pa\"\x94\x8A\x8A\x8A\x84a*\xF3V[\x85Q` \x87\x01Q\x91\x99P\x91\x93Pa\"\xAB\x91\x90a-pV[a#\x01W\x89`@Qc\x11\x013[`\xE1\x1B\x81R`\x04\x01a\x06\xB0\x91\x81R`@` \x82\x01\x81\x90R`\x1A\x90\x82\x01R\x7FAA25 invalid account nonce\0\0\0\0\0\0``\x82\x01R`\x80\x01\x90V[a#\tC_RV[`\xA0\x84\x01Q``\x90`\x01`\x01`\xA0\x1B\x03\x16\x15a#1Wa#,\x8B\x8B\x8B\x85\x87a-\xBDV[\x97P\x90P[_Z\x87\x03\x90P\x80\x8B`\xA0\x015\x10\x15a#\x95W\x8B`@Qc\x11\x013[`\xE1\x1B\x81R`\x04\x01a\x06\xB0\x91\x81R`@` \x82\x01\x81\x90R`\x1E\x90\x82\x01R\x7FAA40 over verificationGasLimit\0\0``\x82\x01R`\x80\x01\x90V[`@\x8A\x01\x83\x90R\x81``\x8B\x01R`\xC0\x8B\x015Z\x88\x03\x01\x8A`\x80\x01\x81\x81RPPPPPPPPP\x93P\x93\x91PPV[__a#\xCE\x85a/\xF5V[\x91P\x91P\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x14a$=W\x85`@Qc\x11\x013[`\xE1\x1B\x81R`\x04\x01a\x06\xB0\x91\x81R`@` \x82\x01\x81\x90R`\x14\x90\x82\x01R\x7FAA24 signature error\0\0\0\0\0\0\0\0\0\0\0\0``\x82\x01R`\x80\x01\x90V[\x80\x15a$\x95W\x85`@Qc\x11\x013[`\xE1\x1B\x81R`\x04\x01a\x06\xB0\x91\x81R`@` \x82\x01\x81\x90R`\x17\x90\x82\x01R\x7FAA22 expired or not due\0\0\0\0\0\0\0\0\0``\x82\x01R`\x80\x01\x90V[_a$\x9F\x85a/\xF5V[\x92P\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a%\x04W\x86`@Qc\x11\x013[`\xE1\x1B\x81R`\x04\x01a\x06\xB0\x91\x81R`@` \x82\x01\x81\x90R`\x14\x90\x82\x01R\x7FAA34 signature error\0\0\0\0\0\0\0\0\0\0\0\0``\x82\x01R`\x80\x01\x90V[\x81\x15a%\x82W\x86`@Qc\x11\x013[`\xE1\x1B\x81R`\x04\x01a\x06\xB0\x91\x81R`@` \x82\x01\x81\x90R`!\x90\x82\x01R\x7FAA32 paymaster expired or not du``\x82\x01R\x7Fe\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x80\x82\x01R`\xA0\x01\x90V[PPPPPPPV[__Z\x90P_a%\x9C\x84``\x01Q\x90V[\x90P0c\x1Ds'Va%\xB1``\x88\x01\x88a<%V[\x87\x85`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a%\xD2\x94\x93\x92\x91\x90aB\x9DV[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x92PPP\x80\x15a&\x0CWP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra&\t\x91\x81\x01\x90aCTV[`\x01[a&\xC6W_` __>P_Q\x7F!R!S\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x01a&\x92W\x86`@Qc\x11\x013[`\xE1\x1B\x81R`\x04\x01a\x06\xB0\x91\x81R`@` \x82\x01\x81\x90R`\x0F\x90\x82\x01R\x7FAA95 out of gas\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x82\x01R`\x80\x01\x90V[_\x85`\x80\x01QZa&\xA3\x90\x86a;\x9AV[a&\xAD\x91\x90a:\xC0V[\x90Pa&\xBD\x88`\x02\x88\x86\x85a\x1E\xB6V[\x94PPPa&\xC9V[\x92P[PP\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16a'(W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FAA90 invalid beneficiary\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06\xB0V[_\x82`\x01`\x01`\xA0\x1B\x03\x16\x82`@Q_`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80_\x81\x14a'qW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a'vV[``\x91P[PP\x90P\x80a'\xC7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FAA91 failed send to beneficiary\0`D\x82\x01R`d\x01a\x06\xB0V[PPPV[_a'\xD6\x82a0DV[\x80Q\x90` \x01 \x90P\x91\x90PV[0c\x95q\"\xABa'\xF7`@\x84\x01\x84a<%V[a(\x04` \x86\x01\x86a9\rV[a(\x12a\x01 \x87\x01\x87a<%V[`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a(2\x95\x94\x93\x92\x91\x90aCkV[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a(HW__\xFD[PZ\xFA\x92PPP\x80\x15a(YWP`\x01[a(\xB6Wa(eaA|V[\x80c\x08\xC3y\xA0\x03a(\xACWPa(yaA\x95V[\x80a(\x84WPa(\xAEV[\x80Q\x15a(\xA8W_\x81`@Qc\x11\x013[`\xE1\x1B\x81R`\x04\x01a\x06\xB0\x92\x91\x90a;\x19V[PPV[P[=__>=_\xFD[PV[`@\x80Q``\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R\x90a(\xDE\x84a1\x14V[\x90P_a(\xEA\x84a1\x14V[\x82Q\x90\x91P`\x01`\x01`\xA0\x1B\x03\x81\x16a)\x01WP\x80Q[` \x80\x84\x01Q`@\x80\x86\x01Q\x92\x85\x01Q\x90\x85\x01Q\x91\x92\x91e\xFF\xFF\xFF\xFF\xFF\xFF\x80\x83\x16\x90\x85\x16\x10\x15a)/W\x81\x93P[\x80e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15a)KW\x80\x92P[PP`@\x80Q``\x81\x01\x82R`\x01`\x01`\xA0\x1B\x03\x90\x94\x16\x84Re\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x16` \x85\x01R\x91\x16\x90\x82\x01R\x92PPP\x92\x91PPV[`\xC0\x81\x01Q`\xE0\x82\x01Q_\x91\x90\x80\x82\x03a)\x9EWP\x92\x91PPV[a)\xAA\x82H\x83\x01a1\x83V[\x94\x93PPPPV[a)\xBF` \x83\x01\x83a9\rV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x80\x83\x015\x90\x82\x01R`\x80\x80\x83\x015`@\x83\x01R`\xA0\x83\x015``\x83\x01R`\xC0\x80\x84\x015\x91\x83\x01\x91\x90\x91R`\xE0\x80\x84\x015\x91\x83\x01\x91\x90\x91Ra\x01\0\x83\x015\x90\x82\x01R6_a*\x1Da\x01 \x85\x01\x85a<%V[\x90\x92P\x90P\x80\x15a*\x9CW`\x14\x81\x10\x15a*yW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAA93 invalid paymasterAndData\0\0\0`D\x82\x01R`d\x01a\x06\xB0V[a*\x86`\x14_\x83\x85a>\x8FV[a*\x8F\x91a>\xB6V[``\x1C`\xA0\x84\x01Ra\x0E\x98V[_`\xA0\x84\x01RPPPPV[`\xA0\x81\x01Q_\x90\x81\x90`\x01`\x01`\xA0\x1B\x03\x16a*\xC5W`\x01a*\xC8V[`\x03[`\xFF\x16\x90P_\x83`\x80\x01Q\x82\x85``\x01Q\x02\x85`@\x01Q\x01\x01\x90P\x83`\xC0\x01Q\x81\x02\x92PPP\x91\x90PV[___Z\x85Q\x80Q\x91\x92P\x90a+\x16\x89\x88a+\x11`@\x8C\x01\x8Ca<%V[a1\x9AV[`\xA0\x82\x01Qa+#C_RV[_`\x01`\x01`\xA0\x1B\x03\x82\x16a+oW`\x01`\x01`\xA0\x1B\x03\x83\x16_\x90\x81R` \x81\x90R`@\x90 Tm\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x88\x81\x11a+iW\x80\x89\x03a+kV[_[\x91PP[``\x84\x01Q` \x8A\x01Q`@Q\x7F:\x87\x1C\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x86\x16\x92c:\x87\x1C\xDD\x92\x90\x91a+\xC2\x91\x8F\x91\x87\x90`\x04\x01aC\xA0V[` `@Q\x80\x83\x03\x81_\x88\x87\xF1\x93PPPP\x80\x15a+\xFDWP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra+\xFA\x91\x81\x01\x90aCTV[`\x01[a,\x8EWa,\taA|V[\x80c\x08\xC3y\xA0\x03a,:WPa,\x1DaA\x95V[\x80a,(WPa,<V[\x8B\x81`@Q` \x01a H\x91\x90aC\xC4V[P[\x8A`@Qc\x11\x013[`\xE1\x1B\x81R`\x04\x01a\x06\xB0\x91\x81R`@` \x82\x01\x81\x90R`\x16\x90\x82\x01R\x7FAA23 reverted (or OOG)\0\0\0\0\0\0\0\0\0\0``\x82\x01R`\x80\x01\x90V[\x95P`\x01`\x01`\xA0\x1B\x03\x82\x16a-]W`\x01`\x01`\xA0\x1B\x03\x83\x16_\x90\x81R` \x81\x90R`@\x90 \x80Tm\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x80\x8A\x11\x15a-!W\x8C`@Qc\x11\x013[`\xE1\x1B\x81R`\x04\x01a\x06\xB0\x91\x81R`@` \x82\x01\x81\x90R`\x17\x90\x82\x01R\x7FAA21 didn't pay prefund\0\0\0\0\0\0\0\0\0``\x82\x01R`\x80\x01\x90V[\x81T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90\x8A\x90\x03m\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x17\x90U[Z\x85\x03\x96PPPPPP\x94P\x94\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x01` \x90\x81R`@\x80\x83 \x84\x82\x1C\x80\x85R\x92R\x82 \x80T\x84\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x91\x90\x85a-\xAF\x83a:\xD3V[\x90\x91UP\x14\x95\x94PPPPPV[\x82Q``\x81\x81\x01Q\x90\x91_\x91\x84\x81\x11a.\x18W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FAA41 too little verificationGas\0`D\x82\x01R`d\x01a\x06\xB0V[`\xA0\x82\x01Q`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R` \x81\x90R`@\x90 \x80T\x87\x84\x03\x92\x91\x90m\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x89\x81\x10\x15a.\xA6W\x8C`@Qc\x11\x013[`\xE1\x1B\x81R`\x04\x01a\x06\xB0\x91\x81R`@` \x82\x01\x81\x90R`\x1E\x90\x82\x01R\x7FAA31 paymaster deposit too low\0\0``\x82\x01R`\x80\x01\x90V[\x89\x81\x03\x82_\x01_a\x01\0\n\x81T\x81m\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83m\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x82`\x01`\x01`\xA0\x1B\x03\x16c\xF4e\xC7~\x85\x8E\x8E` \x01Q\x8E`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a/\x12\x93\x92\x91\x90aC\xA0V[_`@Q\x80\x83\x03\x81_\x88\x87\xF1\x93PPPP\x80\x15a/PWP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra/M\x91\x90\x81\x01\x90aC\xF5V[`\x01[a/\xE1Wa/\\aA|V[\x80c\x08\xC3y\xA0\x03a/\x8DWPa/paA\x95V[\x80a/{WPa/\x8FV[\x8D\x81`@Q` \x01a H\x91\x90aD{V[P[\x8C`@Qc\x11\x013[`\xE1\x1B\x81R`\x04\x01a\x06\xB0\x91\x81R`@` \x82\x01\x81\x90R`\x16\x90\x82\x01R\x7FAA33 reverted (or OOG)\0\0\0\0\0\0\0\0\0\0``\x82\x01R`\x80\x01\x90V[\x90\x9E\x90\x9DP\x9BPPPPPPPPPPPPV[__\x82_\x03a0\x08WP_\x92\x83\x92P\x90PV[_a0\x12\x84a1\x14V[\x90P\x80`@\x01Qe\xFF\xFF\xFF\xFF\xFF\xFF\x16B\x11\x80a09WP\x80` \x01Qe\xFF\xFF\xFF\xFF\xFF\xFF\x16B\x10[\x90Q\x94\x90\x93P\x91PPV[``\x815` \x83\x015_a0ca0^`@\x87\x01\x87a<%V[a4mV[\x90P_a0va0^``\x88\x01\x88a<%V[\x90P`\x80\x86\x015`\xA0\x87\x015`\xC0\x88\x015`\xE0\x89\x015a\x01\0\x8A\x015_a0\xA4a0^a\x01 \x8E\x01\x8Ea<%V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x9C\x90\x9C\x16` \x8D\x01R\x8B\x81\x01\x9A\x90\x9AR``\x8B\x01\x98\x90\x98RP`\x80\x89\x01\x95\x90\x95R`\xA0\x88\x01\x93\x90\x93R`\xC0\x87\x01\x91\x90\x91R`\xE0\x86\x01Ra\x01\0\x85\x01Ra\x01 \x84\x01Ra\x01@\x80\x84\x01\x91\x90\x91R\x81Q\x80\x84\x03\x90\x91\x01\x81Ra\x01`\x90\x92\x01\x90R\x92\x91PPV[`@\x80Q``\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R\x81`\xA0\x81\x90\x1Ce\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16_\x03a1NWPe\xFF\xFF\xFF\xFF\xFF\xFF[`@\x80Q``\x81\x01\x82R`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x83R`\xD0\x94\x90\x94\x1C` \x83\x01Re\xFF\xFF\xFF\xFF\xFF\xFF\x16\x92\x81\x01\x92\x90\x92RP\x90V[_\x81\x83\x10a1\x91W\x81a1\x93V[\x82[\x93\x92PPPV[\x80\x15a\x0E\x98W\x82QQ`\x01`\x01`\xA0\x1B\x03\x81\x16;\x15a2\x05W\x84`@Qc\x11\x013[`\xE1\x1B\x81R`\x04\x01a\x06\xB0\x91\x81R`@` \x82\x01\x81\x90R`\x1F\x90\x82\x01R\x7FAA10 sender already constructed\0``\x82\x01R`\x80\x01\x90V[\x83Q``\x01Q`@Q\x7FW\x0E\x1A6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R_\x91`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91cW\x0E\x1A6\x91\x90a2u\x90\x88\x90\x88\x90`\x04\x01a?\x1CV[` `@Q\x80\x83\x03\x81_\x88\x87\xF1\x15\x80\x15a2\x91W=__>=_\xFD[PPPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a2\xB6\x91\x90a?/V[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a3\x18W\x85`@Qc\x11\x013[`\xE1\x1B\x81R`\x04\x01a\x06\xB0\x91\x81R`@` \x82\x01\x81\x90R`\x1B\x90\x82\x01R\x7FAA13 initCode failed or OOG\0\0\0\0\0``\x82\x01R`\x80\x01\x90V[\x81`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14a3\x82W\x85`@Qc\x11\x013[`\xE1\x1B\x81R`\x04\x01a\x06\xB0\x91\x81R`@` \x80\x83\x01\x82\x90R\x90\x82\x01R\x7FAA14 initCode must return sender``\x82\x01R`\x80\x01\x90V[\x80`\x01`\x01`\xA0\x1B\x03\x16;_\x03a3\xE4W\x85`@Qc\x11\x013[`\xE1\x1B\x81R`\x04\x01a\x06\xB0\x91\x81R`@` \x80\x83\x01\x82\x90R\x90\x82\x01R\x7FAA15 initCode must create sender``\x82\x01R`\x80\x01\x90V[_a3\xF2`\x14\x82\x86\x88a>\x8FV[a3\xFB\x91a>\xB6V[``\x1C\x90P\x82`\x01`\x01`\xA0\x1B\x03\x16\x86` \x01Q\x7F\xD5\x1A\x9Ca&z\xA6\x19ia\x88>\xCF_\xF2\xDAf\x19\xC3}\xAC\x0F\xA9!\"Q?\xB3,\x03--\x83\x89_\x01Q`\xA0\x01Q`@Qa4\\\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x16` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA3PPPPPPPV[_`@Q\x82\x80\x85\x837\x90 \x93\x92PPPV[`@Q\x80`\xA0\x01`@R\x80a4\xDC`@Q\x80a\x01\0\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[_` \x82\x84\x03\x12\x15a5\nW__\xFD[\x815c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a1\x93W__\xFD[\x805w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a5DW__\xFD[\x91\x90PV[_` \x82\x84\x03\x12\x15a5YW__\xFD[a1\x93\x82a5\x1DV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a(\xB6W__\xFD[\x805a5D\x81a5bV[__`@\x83\x85\x03\x12\x15a5\x92W__\xFD[\x825a5\x9D\x81a5bV[\x91Pa5\xAB` \x84\x01a5\x1DV[\x90P\x92P\x92\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[`\xA0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a6\x01Wa6\x01a5\xB4V[`@RPV[a\x01\0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a6\x01Wa6\x01a5\xB4V[`\x1F\x19`\x1F\x83\x01\x16\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a6NWa6Na5\xB4V[`@RPPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a6nWa6na5\xB4V[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[_\x81\x83\x03a\x01\x80\x81\x12\x15a6\x8EW__\xFD[`@Qa6\x9A\x81a5\xE1V[\x80\x92Pa\x01\0\x82\x12\x15a6\xABW__\xFD[`@Q\x91Pa6\xB9\x82a6\x07V[a6\xC2\x84a5vV[\x82R` \x84\x81\x015\x90\x83\x01R`@\x80\x85\x015\x90\x83\x01R``\x80\x85\x015\x90\x83\x01R`\x80\x80\x85\x015\x90\x83\x01Ra6\xF8`\xA0\x85\x01a5vV[`\xA0\x83\x01R`\xC0\x84\x81\x015\x90\x83\x01R`\xE0\x80\x85\x015\x90\x83\x01R\x90\x81Ra\x01\0\x83\x015` \x82\x01Ra\x01 \x83\x015`@\x82\x01Ra\x01@\x83\x015``\x82\x01Ra\x01`\x90\x92\x015`\x80\x90\x92\x01\x91\x90\x91R\x91\x90PV[__\x83`\x1F\x84\x01\x12a7ZW__\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a7qW__\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a7\x88W__\xFD[\x92P\x92\x90PV[____a\x01\xC0\x85\x87\x03\x12\x15a7\xA3W__\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a7\xB9W__\xFD[\x85\x01`\x1F\x81\x01\x87\x13a7\xC9W__\xFD[\x805a7\xD4\x81a6UV[`@Qa7\xE1\x82\x82a6(V[\x82\x81R\x89` \x84\x86\x01\x01\x11\x15a7\xF5W__\xFD[\x82` \x85\x01` \x83\x017_` \x84\x83\x01\x01R\x80\x97PPPPPa8\x1B\x86` \x87\x01a6|V[\x92Pa\x01\xA0\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a87W__\xFD[a8C\x87\x82\x88\x01a7JV[\x95\x98\x94\x97P\x95PPPPV[__\x83`\x1F\x84\x01\x12a8_W__\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a8vW__\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a7\x88W__\xFD[___`@\x84\x86\x03\x12\x15a8\xA2W__\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a8\xB8W__\xFD[a8\xC4\x86\x82\x87\x01a8OV[\x90\x94P\x92PP` \x84\x015a8\xD8\x81a5bV[\x80\x91PP\x92P\x92P\x92V[__`@\x83\x85\x03\x12\x15a8\xF4W__\xFD[\x825a8\xFF\x81a5bV[\x94` \x93\x90\x93\x015\x93PPPV[_` \x82\x84\x03\x12\x15a9\x1DW__\xFD[\x815a1\x93\x81a5bV[_____``\x86\x88\x03\x12\x15a9<W__\xFD[\x855g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a9RW__\xFD[a9^\x88\x82\x89\x01a7JV[\x90\x96P\x94PP` \x86\x015a9r\x81a5bV[\x92P`@\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a9\x8DW__\xFD[a9\x99\x88\x82\x89\x01a7JV[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[__` \x83\x85\x03\x12\x15a9\xBBW__\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a9\xD1W__\xFD[a9\xDD\x85\x82\x86\x01a7JV[\x90\x96\x90\x95P\x93PPPPV[_a\x01`\x82\x84\x03\x12\x15a9\xFAW__\xFD[P\x91\x90PV[_` \x82\x84\x03\x12\x15a:\x10W__\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a:&W__\xFD[a)\xAA\x84\x82\x85\x01a9\xE9V[____``\x85\x87\x03\x12\x15a:EW__\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a:[W__\xFD[a:g\x87\x82\x88\x01a9\xE9V[\x94PP` \x85\x015a:x\x81a5bV[\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a87W__\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\x0F\x05Wa\x0F\x05a:\x93V[__\x19\x82\x03a:\xE4Wa:\xE4a:\x93V[P`\x01\x01\x90V[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[\x82\x81R`@` \x82\x01R_a)\xAA`@\x83\x01\x84a:\xEBV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_\x825\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\xA1\x836\x03\x01\x81\x12a;\x90W__\xFD[\x91\x90\x91\x01\x92\x91PPV[\x81\x81\x03\x81\x81\x11\x15a\x0F\x05Wa\x0F\x05a:\x93V[_\x825\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA1\x836\x03\x01\x81\x12a;\x90W__\xFD[__\x835`\x1E\x19\x846\x03\x01\x81\x12a;\xF4W__\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a<\x0EW__\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15a7\x88W__\xFD[__\x835`\x1E\x19\x846\x03\x01\x81\x12a<:W__\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a<TW__\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a7\x88W__\xFD[__\x835`\x1E\x19\x846\x03\x01\x81\x12a<}W__\xFD[\x83\x01` \x81\x01\x92P5\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a<\x9CW__\xFD[\x806\x03\x82\x13\x15a7\x88W__\xFD[\x81\x83R\x81\x81` \x85\x017P_` \x82\x84\x01\x01R_` `\x1F\x19`\x1F\x84\x01\x16\x84\x01\x01\x90P\x92\x91PPV[a<\xED\x82a<\xE0\x83a5vV[`\x01`\x01`\xA0\x1B\x03\x16\x90RV[` \x81\x81\x015\x90\x83\x01R_a=\x05`@\x83\x01\x83a<hV[a\x01``@\x86\x01Ra=\x1Ca\x01`\x86\x01\x82\x84a<\xAAV[\x91PPa=,``\x84\x01\x84a<hV[\x85\x83\x03``\x87\x01Ra=?\x83\x82\x84a<\xAAV[`\x80\x86\x81\x015\x90\x88\x01R`\xA0\x80\x87\x015\x90\x88\x01R`\xC0\x80\x87\x015\x90\x88\x01R`\xE0\x80\x87\x015\x90\x88\x01Ra\x01\0\x80\x87\x015\x90\x88\x01R\x92Pa=\x85\x91PPa\x01 \x84\x01\x84a<hV[\x85\x83\x03a\x01 \x87\x01Ra=\x99\x83\x82\x84a<\xAAV[\x92PPPa=\xABa\x01@\x84\x01\x84a<hV[\x85\x83\x03a\x01@\x87\x01Ra=\xBF\x83\x82\x84a<\xAAV[\x96\x95PPPPPPV[`@\x80\x82R\x81\x01\x84\x90R_```\x05\x86\x90\x1B\x83\x01\x81\x01\x90\x83\x01\x87\x83\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\xA16\x83\x90\x03\x01[\x89\x82\x10\x15a>mW\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA0\x87\x86\x03\x01\x84R\x825\x81\x81\x12a>GW__\xFD[a>S\x86\x8D\x83\x01a<\xD3V[\x95PP` \x83\x01\x92P` \x84\x01\x93P`\x01\x82\x01\x91Pa>\x0BV[PPPP\x82\x81\x03` \x84\x01Ra>\x84\x81\x85\x87a<\xAAV[\x97\x96PPPPPPPV[__\x85\x85\x11\x15a>\x9DW__\xFD[\x83\x86\x11\x15a>\xA9W__\xFD[PP\x82\x01\x93\x91\x90\x92\x03\x91PV[\x805\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x90`\x14\x84\x10\x15a?\x15W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\x80\x85`\x14\x03`\x03\x1B\x1B\x82\x16\x16\x91P[P\x92\x91PPV[` \x81R_a)\xAA` \x83\x01\x84\x86a<\xAAV[_` \x82\x84\x03\x12\x15a??W__\xFD[\x81Qa1\x93\x81a5bV[e\xFF\xFF\xFF\xFF\xFF\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x0F\x05Wa\x0F\x05a:\x93V[\x81\x83\x827_\x91\x01\x90\x81R\x91\x90PV[\x86\x81R\x85` \x82\x01Re\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16`@\x82\x01Re\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16``\x82\x01R\x82\x15\x15`\x80\x82\x01R`\xC0`\xA0\x82\x01R_a?\xB9`\xC0\x83\x01\x84a:\xEBV[\x98\x97PPPPPPPPV[\x80Q\x82R` \x81\x01Q` \x83\x01R`@\x81\x01Q\x15\x15`@\x83\x01Re\xFF\xFF\xFF\xFF\xFF\xFF``\x82\x01Q\x16``\x83\x01Re\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x82\x01Q\x16`\x80\x83\x01R_`\xA0\x82\x01Q`\xC0`\xA0\x85\x01Ra)\xAA`\xC0\x85\x01\x82a:\xEBV[a\x01@\x81R_a@1a\x01@\x83\x01\x88a?\xC5V[\x90Pa@J` \x83\x01\x87\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[\x84Q``\x83\x01R` \x94\x85\x01Q`\x80\x83\x01R\x83Q`\xA0\x83\x01R\x92\x84\x01Q`\xC0\x82\x01R\x81Q`\x01`\x01`\xA0\x1B\x03\x16`\xE0\x82\x01R\x90\x83\x01Q\x80Qa\x01\0\x83\x01R\x90\x92\x01Qa\x01 \x90\x92\x01\x91\x90\x91R\x92\x91PPV[`\xE0\x81R_a@\xAE`\xE0\x83\x01\x87a?\xC5V[\x90Pa@\xC7` \x83\x01\x86\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[\x83Q``\x83\x01R` \x84\x01Q`\x80\x83\x01R\x82Q`\xA0\x83\x01R` \x83\x01Q`\xC0\x83\x01R\x95\x94PPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[_`\x03\x85\x10aAUW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[\x84\x82R``` \x83\x01RaAl``\x83\x01\x85a:\xEBV[\x90P\x82`@\x83\x01R\x94\x93PPPPV[_`\x03=\x11\x15aA\x92W`\x04__>P_Q`\xE0\x1C[\x90V[_`D=\x10\x15aA\xA2W\x90V[`@Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC=\x01`\x04\x82>\x80Q=`$\x82\x01\x11g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15aA\xEAWPP\x90V[\x80\x82\x01\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aB\x06WPPPP\x90V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC=\x85\x01\x01` \x82\x84\x01\x01\x11\x15aB>WPPPP\x90V[aBM` \x82\x85\x01\x01\x85a6(V[P\x93\x92PPPV[_\x81Q\x80` \x84\x01\x85^_\x93\x01\x92\x83RP\x90\x91\x90PV[\x7FAA50 postOp reverted: \0\0\0\0\0\0\0\0\0\0\x81R_a1\x93`\x16\x83\x01\x84aBUV[a\x01\xC0\x81R_aB\xB2a\x01\xC0\x83\x01\x86\x88a<\xAAV[\x84Q`\x01`\x01`\xA0\x1B\x03\x81Q\x16` \x85\x01R` \x81\x01Q`@\x85\x01R`@\x81\x01Q``\x85\x01R``\x81\x01Q`\x80\x85\x01R`\x80\x81\x01Q`\xA0\x85\x01R`\x01`\x01`\xA0\x1B\x03`\xA0\x82\x01Q\x16`\xC0\x85\x01R`\xC0\x81\x01Q`\xE0\x85\x01R`\xE0\x81\x01Qa\x01\0\x85\x01RP` \x85\x01Qa\x01 \x84\x01R`@\x85\x01Qa\x01@\x84\x01R``\x85\x01Qa\x01`\x84\x01R`\x80\x85\x01Qa\x01\x80\x84\x01R\x82\x81\x03a\x01\xA0\x84\x01Ra>\x84\x81\x85a:\xEBV[_` \x82\x84\x03\x12\x15aCdW__\xFD[PQ\x91\x90PV[``\x81R_aC~``\x83\x01\x87\x89a<\xAAV[`\x01`\x01`\xA0\x1B\x03\x86\x16` \x84\x01R\x82\x81\x03`@\x84\x01Ra?\xB9\x81\x85\x87a<\xAAV[``\x81R_aC\xB2``\x83\x01\x86a<\xD3V[` \x83\x01\x94\x90\x94RP`@\x01R\x91\x90PV[\x7FAA23 reverted: \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R_a1\x93`\x0F\x83\x01\x84aBUV[__`@\x83\x85\x03\x12\x15aD\x06W__\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aD\x1CW__\xFD[\x83\x01`\x1F\x81\x01\x85\x13aD,W__\xFD[\x80QaD7\x81a6UV[`@QaDD\x82\x82a6(V[\x82\x81R\x87` \x84\x86\x01\x01\x11\x15aDXW__\xFD[\x82` \x85\x01` \x83\x01^_` \x93\x82\x01\x84\x01R\x95\x90\x91\x01Q\x94\x96\x94\x95PPPPPV[\x7FAA33 reverted: \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R_a1\x93`\x0F\x83\x01\x84aBUV\xFE\xA2dipfsX\"\x12 \xE0\xC3Id!\x8Aq\x84/\r*\xD4\xB7+\x88\xB8\xCFQ@\xC4\n\x0Fd^e\x82\xE8ou\xCFX\xF7dsolcC\0\x08\x1B\x003`\x80`@R4\x80\x15`\x0EW__\xFD[Pa\x02\"\x80a\0\x1C_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0)W_5`\xE0\x1C\x80cW\x0E\x1A6\x14a\0-W[__\xFD[a\0@a\0;6`\x04a\0\xF1V[a\0iV[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[_\x80a\0x`\x14\x82\x85\x87a\x01_V[a\0\x81\x91a\x01\x86V[``\x1C\x90P_a\0\x94\x84`\x14\x81\x88a\x01_V[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x82\x90RP\x84Q\x94\x95P\x93` \x93P\x84\x92P\x90P\x82\x85\x01\x82\x87Z\xF1\x90P_Q\x93P\x80a\0\xE8W_\x93P[PPP\x92\x91PPV[__` \x83\x85\x03\x12\x15a\x01\x02W__\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x01\x18W__\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x01(W__\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x01>W__\xFD[\x85` \x82\x84\x01\x01\x11\x15a\x01OW__\xFD[` \x91\x90\x91\x01\x95\x90\x94P\x92PPPV[__\x85\x85\x11\x15a\x01mW__\xFD[\x83\x86\x11\x15a\x01yW__\xFD[PP\x82\x01\x93\x91\x90\x92\x03\x91PV[\x805\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x90`\x14\x84\x10\x15a\x01\xE5W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\x80\x85`\x14\x03`\x03\x1B\x1B\x82\x16\x16\x91P[P\x92\x91PPV\xFE\xA2dipfsX\"\x12 \xB5\xCDX\x84ic\xFD\xD3 j\x7Fz\x8E\xB7\x7F\x89\x0FR\xB2\xEF\xC4i\x9D\x85v\x08P\x96i\xF6\xA3\x9CdsolcC\0\x08\x1B\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361061015a575f3560e01c80638f41ec5a116100bb578063bb9fe6bf11610071578063d6383f9411610057578063d6383f94146104d0578063ee219423146104ef578063fc7e286d1461050e575f5ffd5b8063bb9fe6bf1461049d578063c23a5cea146104b1575f5ffd5b80639b249f69116100a15780639b249f691461044c578063a61935311461046b578063b760faf91461048a575f5ffd5b80638f41ec5a14610419578063957122ab1461042d575f5ffd5b8063205c2878116101105780634b1d7cf5116100f65780634b1d7cf5146102655780635287ce121461028457806370a08231146103d5575f5ffd5b8063205c28781461022757806335567e1a14610246575f5ffd5b80631b2e01b8116101405780631b2e01b8146101a05780631d732756146101e95780631fad948c14610208575f5ffd5b80630396cb601461016e5780630bd28e3b14610181575f5ffd5b3661016a57610168336105e6565b005b5f5ffd5b61016861017c3660046134fa565b610653565b34801561018c575f5ffd5b5061016861019b366004613549565b61096a565b3480156101ab575f5ffd5b506101d66101ba366004613581565b600160209081525f928352604080842090915290825290205481565b6040519081526020015b60405180910390f35b3480156101f4575f5ffd5b506101d661020336600461378f565b6109b1565b348015610213575f5ffd5b50610168610222366004613890565b610b31565b348015610232575f5ffd5b506101686102413660046138e3565b610cef565b348015610251575f5ffd5b506101d6610260366004613581565b610e9e565b348015610270575f5ffd5b5061016861027f366004613890565b610f0b565b34801561028f575f5ffd5b5061036461029e36600461390d565b6040805160a0810182525f80825260208201819052918101829052606081018290526080810191909152506001600160a01b03165f9081526020818152604091829020825160a08101845281546dffffffffffffffffffffffffffff80821683526e010000000000000000000000000000820460ff161515948301949094526f0100000000000000000000000000000090049092169282019290925260019091015463ffffffff81166060830152640100000000900465ffffffffffff16608082015290565b6040516101e091905f60a0820190506dffffffffffffffffffffffffffff83511682526020830151151560208301526dffffffffffffffffffffffffffff604084015116604083015263ffffffff606084015116606083015265ffffffffffff608084015116608083015292915050565b3480156103e0575f5ffd5b506101d66103ef36600461390d565b6001600160a01b03165f908152602081905260409020546dffffffffffffffffffffffffffff1690565b348015610424575f5ffd5b506101d6600181565b348015610438575f5ffd5b50610168610447366004613928565b61137b565b348015610457575f5ffd5b506101686104663660046139aa565b611475565b348015610476575f5ffd5b506101d6610485366004613a00565b61155e565b61016861049836600461390d565b6105e6565b3480156104a8575f5ffd5b5061016861159f565b3480156104bc575f5ffd5b506101686104cb36600461390d565b611720565b3480156104db575f5ffd5b506101686104ea366004613a32565b611999565b3480156104fa575f5ffd5b50610168610509366004613a00565b611aa1565b348015610519575f5ffd5b5061059961052836600461390d565b5f60208190529081526040902080546001909101546dffffffffffffffffffffffffffff808316926e010000000000000000000000000000810460ff16926f010000000000000000000000000000009091049091169063ffffffff811690640100000000900465ffffffffffff1685565b604080516dffffffffffffffffffffffffffff96871681529415156020860152929094169183019190915263ffffffff16606082015265ffffffffffff909116608082015260a0016101e0565b6105f08134611da0565b6001600160a01b0381165f8181526020818152604091829020805492516dffffffffffffffffffffffffffff909316835292917f2da466a7b24304f47e87fa2e1e5a81b9831ce54fec19055ce277ca2f39ba42c491015b60405180910390a25050565b335f90815260208190526040902063ffffffff82166106b95760405162461bcd60e51b815260206004820152601a60248201527f6d757374207370656369667920756e7374616b652064656c617900000000000060448201526064015b60405180910390fd5b600181015463ffffffff90811690831610156107175760405162461bcd60e51b815260206004820152601c60248201527f63616e6e6f7420646563726561736520756e7374616b652074696d650000000060448201526064016106b0565b80545f906107499034906f0100000000000000000000000000000090046dffffffffffffffffffffffffffff16613ac0565b90505f811161079a5760405162461bcd60e51b815260206004820152601260248201527f6e6f207374616b6520737065636966696564000000000000000000000000000060448201526064016106b0565b6dffffffffffffffffffffffffffff8111156107f85760405162461bcd60e51b815260206004820152600e60248201527f7374616b65206f766572666c6f7700000000000000000000000000000000000060448201526064016106b0565b6040805160a08101825283546dffffffffffffffffffffffffffff90811682526001602080840182815286841685870190815263ffffffff808b16606088019081525f608089018181523380835296829052908a9020985189549551945189166f01000000000000000000000000000000027fffffff0000000000000000000000000000ffffffffffffffffffffffffffffff9515156e010000000000000000000000000000027fffffffffffffffffffffffffffffffffff0000000000000000000000000000009097169190991617949094179290921695909517865551949092018054925165ffffffffffff16640100000000027fffffffffffffffffffffffffffffffffffffffffffff00000000000000000000909316949093169390931717905590517fa5ae833d0bb1dcd632d98a8b70973e8516812898e19bf27b70071ebc8dc52c019061095d908490879091825263ffffffff16602082015260400190565b60405180910390a2505050565b335f90815260016020908152604080832077ffffffffffffffffffffffffffffffffffffffffffffffff8516845290915281208054916109a983613ad3565b919050555050565b5f5f5a9050333014610a055760405162461bcd60e51b815260206004820152601760248201527f4141393220696e7465726e616c2063616c6c206f6e6c7900000000000000000060448201526064016106b0565b8451604081015160608201518101611388015a1015610a46577fdeaddead000000000000000000000000000000000000000000000000000000005f5260205ffd5b87515f9015610ad4575f610a5f845f01515f8c86611e75565b905080610ad2575f610a72610800611e8b565b805190915015610acc57845f01516001600160a01b03168a602001517f1c4fada7374c0a9ee8841fc38afe82932dc0f8e69012e927f061a8bae611a201876020015184604051610ac3929190613b19565b60405180910390a35b60019250505b505b5f88608001515a8603019050610b235f838b8b8b8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f92019190915250889250611eb6915050565b9a9950505050505050505050565b6002805403610b825760405162461bcd60e51b815260206004820152601f60248201527f5265656e7472616e637947756172643a207265656e7472616e742063616c6c0060448201526064016106b0565b60028055815f8167ffffffffffffffff811115610ba157610ba16135b4565b604051908082528060200260200182016040528015610bda57816020015b610bc761347f565b815260200190600190039081610bbf5790505b5090505f5b82811015610c4f575f828281518110610bfa57610bfa613b31565b602002602001015190505f5f610c34848a8a87818110610c1c57610c1c613b31565b9050602002810190610c2e9190613b5e565b856121dd565b91509150610c448483835f6123c3565b505050600101610bdf565b506040515f907fbb47ee3e183a558b1a2ff0874b079f3fc5478b7454eacf2bfc5af2ff5878f972908290a15f5b83811015610cd757610ccb81888884818110610c9a57610c9a613b31565b9050602002810190610cac9190613b5e565b858481518110610cbe57610cbe613b31565b602002602001015161258b565b90910190600101610c7c565b50610ce284826126d2565b5050600160025550505050565b335f90815260208190526040902080546dffffffffffffffffffffffffffff16821115610d5e5760405162461bcd60e51b815260206004820152601960248201527f576974686472617720616d6f756e7420746f6f206c617267650000000000000060448201526064016106b0565b8054610d7b9083906dffffffffffffffffffffffffffff16613b9a565b81547fffffffffffffffffffffffffffffffffffff0000000000000000000000000000166dffffffffffffffffffffffffffff91909116178155604080516001600160a01b03851681526020810184905233917fd1c19fbcd4551a5edfb66d43d2e337c04837afda3482b42bdf569a8fccdae5fb910160405180910390a25f836001600160a01b0316836040515f6040518083038185875af1925050503d805f8114610e42576040519150601f19603f3d011682016040523d82523d5f602084013e610e47565b606091505b5050905080610e985760405162461bcd60e51b815260206004820152601260248201527f6661696c656420746f207769746864726177000000000000000000000000000060448201526064016106b0565b50505050565b6001600160a01b0382165f90815260016020908152604080832077ffffffffffffffffffffffffffffffffffffffffffffffff8516845290915290819020549082901b7fffffffffffffffffffffffffffffffffffffffffffffffff000000000000000016175b92915050565b6002805403610f5c5760405162461bcd60e51b815260206004820152601f60248201527f5265656e7472616e637947756172643a207265656e7472616e742063616c6c0060448201526064016106b0565b60028055815f805b828110156110e05736868683818110610f7f57610f7f613b31565b9050602002810190610f919190613bad565b9050365f610f9f8380613bdf565b90925090505f610fb5604085016020860161390d565b90505f196001600160a01b038216016110105760405162461bcd60e51b815260206004820152601760248201527f4141393620696e76616c69642061676772656761746f7200000000000000000060448201526064016106b0565b6001600160a01b038116156110c4576001600160a01b03811663e3563a4f848461103d6040890189613c25565b6040518563ffffffff1660e01b815260040161105c9493929190613dc9565b5f6040518083038186803b158015611072575f5ffd5b505afa925050508015611083575060015b6110c4576040517f86a9f7500000000000000000000000000000000000000000000000000000000081526001600160a01b03821660048201526024016106b0565b6110ce8287613ac0565b95505060019093019250610f64915050565b505f8167ffffffffffffffff8111156110fb576110fb6135b4565b60405190808252806020026020018201604052801561113457816020015b61112161347f565b8152602001906001900390816111195790505b506040519091507fbb47ee3e183a558b1a2ff0874b079f3fc5478b7454eacf2bfc5af2ff5878f972905f90a15f805b84811015611234573688888381811061117e5761117e613b31565b90506020028101906111909190613bad565b9050365f61119e8380613bdf565b90925090505f6111b4604085016020860161390d565b9050815f5b81811015611222575f8989815181106111d4576111d4613b31565b602002602001015190505f5f6111f68b898987818110610c1c57610c1c613b31565b91509150611206848383896123c3565b8a61121081613ad3565b9b5050600190930192506111b9915050565b50506001909401935061116392505050565b505f905080805b85811015611337573689898381811061125657611256613b31565b90506020028101906112689190613bad565b905061127a604082016020830161390d565b6001600160a01b03167f575ff3acadd5ab348fe1855e217e0f3678f8d767d7494c9f9fefbee2e17cca4d60405160405180910390a2365f6112bb8380613bdf565b9092509050805f5b8181101561132657611305888585848181106112e1576112e1613b31565b90506020028101906112f39190613b5e565b8b8b81518110610cbe57610cbe613b31565b61130f9088613ac0565b96508761131b81613ad3565b9850506001016112c3565b50506001909301925061123b915050565b506040515f907f575ff3acadd5ab348fe1855e217e0f3678f8d767d7494c9f9fefbee2e17cca4d908290a261136c86826126d2565b50506001600255505050505050565b8315801561139157506001600160a01b0383163b155b156113de5760405162461bcd60e51b815260206004820152601960248201527f41413230206163636f756e74206e6f74206465706c6f7965640000000000000060448201526064016106b0565b60148110611454575f6113f46014828486613e8f565b6113fd91613eb6565b60601c9050803b5f036114525760405162461bcd60e51b815260206004820152601b60248201527f41413330207061796d6173746572206e6f74206465706c6f796564000000000060448201526064016106b0565b505b60405162461bcd60e51b8152602060048201525f60248201526044016106b0565b6040517f570e1a360000000000000000000000000000000000000000000000000000000081525f906001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063570e1a36906114de9086908690600401613f1c565b6020604051808303815f875af11580156114fa573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061151e9190613f2f565b6040517f6ca7b8060000000000000000000000000000000000000000000000000000000081526001600160a01b03821660048201529091506024016106b0565b5f611568826127cc565b6040805160208101929092523090820152466060820152608001604051602081830303815290604052805190602001209050919050565b335f9081526020819052604081206001810154909163ffffffff909116900361160a5760405162461bcd60e51b815260206004820152600a60248201527f6e6f74207374616b65640000000000000000000000000000000000000000000060448201526064016106b0565b80546e010000000000000000000000000000900460ff1661166d5760405162461bcd60e51b815260206004820152601160248201527f616c726561647920756e7374616b696e6700000000000000000000000000000060448201526064016106b0565b60018101545f906116849063ffffffff1642613f4a565b6001830180547fffffffffffffffffffffffffffffffffffffffffffff000000000000ffffffff1664010000000065ffffffffffff84169081029190911790915583547fffffffffffffffffffffffffffffffffff00ffffffffffffffffffffffffffff16845560405190815290915033907ffa9b3c14cc825c412c9ed81b3ba365a5b459439403f18829e572ed53a4180f0a90602001610647565b335f90815260208190526040902080546f0100000000000000000000000000000090046dffffffffffffffffffffffffffff16806117a05760405162461bcd60e51b815260206004820152601460248201527f4e6f207374616b6520746f20776974686472617700000000000000000000000060448201526064016106b0565b6001820154640100000000900465ffffffffffff166118015760405162461bcd60e51b815260206004820152601d60248201527f6d7573742063616c6c20756e6c6f636b5374616b65282920666972737400000060448201526064016106b0565b60018201544264010000000090910465ffffffffffff1611156118665760405162461bcd60e51b815260206004820152601b60248201527f5374616b65207769746864726177616c206973206e6f7420647565000000000060448201526064016106b0565b6001820180547fffffffffffffffffffffffffffffffffffffffffffff0000000000000000000016905581547fffffff0000000000000000000000000000ffffffffffffffffffffffffffffff168255604080516001600160a01b03851681526020810183905233917fb7c918e0e249f999e965cafeb6c664271b3f4317d296461500e71da39f0cbda3910160405180910390a25f836001600160a01b0316826040515f6040518083038185875af1925050503d805f8114611943576040519150601f19603f3d011682016040523d82523d5f602084013e611948565b606091505b5050905080610e985760405162461bcd60e51b815260206004820152601860248201527f6661696c656420746f207769746864726177207374616b65000000000000000060448201526064016106b0565b6119a161347f565b6119aa856127e4565b5f5f6119b75f88856121dd565b915091505f6119c683836128b9565b90506119d0435f52565b5f6119dc5f8a8761258b565b90506119e6435f52565b5f60606001600160a01b038a1615611a5757896001600160a01b03168989604051611a12929190613f68565b5f604051808303815f865af19150503d805f8114611a4b576040519150601f19603f3d011682016040523d82523d5f602084013e611a50565b606091505b5090925090505b8660800151838560200151866040015185856040517f8b7ac9800000000000000000000000000000000000000000000000000000000081526004016106b096959493929190613f77565b611aa961347f565b611ab2826127e4565b5f5f611abf5f85856121dd565b845160a001516040805180820182525f80825260208083018281526001600160a01b03958616835282825284832080546dffffffffffffffffffffffffffff6f01000000000000000000000000000000918290048116875260019283015463ffffffff9081169094528d51518851808a018a5287815280870188815291909a16875286865288872080549390930490911689529101549091169052835180850190945281845283015293955091935090365f611b7e60408a018a613c25565b90925090505f6014821015611b93575f611bad565b611ba060145f8486613e8f565b611ba991613eb6565b60601c5b6040805180820182525f80825260208083018281526001600160a01b03861683529082905292902080546f0100000000000000000000000000000090046dffffffffffffffffffffffffffff1682526001015463ffffffff1690915290915093505050505f611c1c86866128b9565b90505f815f015190505f60016001600160a01b0316826001600160a01b03161490505f6040518060c001604052808b6080015181526020018b6040015181526020018315158152602001856020015165ffffffffffff168152602001856040015165ffffffffffff168152602001611c958c6060015190565b905290506001600160a01b03831615801590611cbb57506001600160a01b038316600114155b15611d66576040805180820182526001600160a01b038516808252825180840184525f80825260208083018281529382528181529085902080546f0100000000000000000000000000000090046dffffffffffffffffffffffffffff1683526001015463ffffffff169092529082015290517ffaecb4e40000000000000000000000000000000000000000000000000000000081526106b0908390899089908c90869060040161401d565b808686896040517fe0cff05f0000000000000000000000000000000000000000000000000000000081526004016106b0949392919061409c565b6001600160a01b0382165f9081526020819052604081208054909190611dd79084906dffffffffffffffffffffffffffff16613ac0565b90506dffffffffffffffffffffffffffff811115611e375760405162461bcd60e51b815260206004820152601060248201527f6465706f736974206f766572666c6f770000000000000000000000000000000060448201526064016106b0565b81547fffffffffffffffffffffffffffffffffffff0000000000000000000000000000166dffffffffffffffffffffffffffff919091161790555050565b5f5f5f845160208601878987f195945050505050565b60603d82811115611e995750815b604051602082018101604052818152815f602083013e9392505050565b5f5f5a85519091505f9081611eca82612983565b60a08301519091506001600160a01b038116611ee957825193506120c4565b8093505f885111156120c457868202955060028a6002811115611f0e57611f0e6140f2565b14611f945760608301516040517fa9a234090000000000000000000000000000000000000000000000000000000081526001600160a01b0383169163a9a2340991611f61908e908d908c9060040161411f565b5f604051808303815f88803b158015611f78575f5ffd5b5087f1158015611f8a573d5f5f3e3d5ffd5b50505050506120c4565b60608301516040517fa9a234090000000000000000000000000000000000000000000000000000000081526001600160a01b0383169163a9a2340991611fe2908e908d908c9060040161411f565b5f604051808303815f88803b158015611ff9575f5ffd5b5087f19350505050801561200b575060015b6120c45761201761417c565b806308c379a003612070575061202b614195565b806120365750612072565b8b81604051602001612048919061426c565b60408051601f1981840301815290829052631101335b60e11b82526106b09291600401613b19565b505b8a604051631101335b60e11b81526004016106b09181526040602082018190526012908201527f4141353020706f73744f70207265766572740000000000000000000000000000606082015260800190565b5a8503870196508187029550858960400151101561212d578a604051631101335b60e11b81526004016106b091815260406020808301829052908201527f414135312070726566756e642062656c6f772061637475616c476173436f7374606082015260800190565b604089015186900361213f8582611da0565b5f808c6002811115612153576121536140f2565b1490508460a001516001600160a01b0316855f01516001600160a01b03168c602001517f49628fd1471006c1482da88028e9ce4dbb080b815c9b0344d39e5a8e6ec1419f8860200151858d8f6040516121c5949392919093845291151560208401526040830152606082015260800190565b60405180910390a45050505050505095945050505050565b5f5f5f5a84519091506121f086826129b2565b6121f98661155e565b6020860152604081015160608201516080830151171760e087013517610100870135176effffffffffffffffffffffffffffff81111561227b5760405162461bcd60e51b815260206004820152601860248201527f41413934206761732076616c756573206f766572666c6f77000000000000000060448201526064016106b0565b5f5f61228684612aa8565b90506122948a8a8a84612af3565b855160208701519199509193506122ab9190612d70565b6123015789604051631101335b60e11b81526004016106b0918152604060208201819052601a908201527f4141323520696e76616c6964206163636f756e74206e6f6e6365000000000000606082015260800190565b612309435f52565b60a08401516060906001600160a01b0316156123315761232c8b8b8b8587612dbd565b975090505b5f5a87039050808b60a001351015612395578b604051631101335b60e11b81526004016106b0918152604060208201819052601e908201527f41413430206f76657220766572696669636174696f6e4761734c696d69740000606082015260800190565b60408a018390528160608b015260c08b01355a8803018a608001818152505050505050505050935093915050565b5f5f6123ce85612ff5565b91509150816001600160a01b0316836001600160a01b03161461243d5785604051631101335b60e11b81526004016106b09181526040602082018190526014908201527f41413234207369676e6174757265206572726f72000000000000000000000000606082015260800190565b80156124955785604051631101335b60e11b81526004016106b09181526040602082018190526017908201527f414132322065787069726564206f72206e6f7420647565000000000000000000606082015260800190565b5f61249f85612ff5565b925090506001600160a01b038116156125045786604051631101335b60e11b81526004016106b09181526040602082018190526014908201527f41413334207369676e6174757265206572726f72000000000000000000000000606082015260800190565b81156125825786604051631101335b60e11b81526004016106b09181526040602082018190526021908201527f41413332207061796d61737465722065787069726564206f72206e6f7420647560608201527f6500000000000000000000000000000000000000000000000000000000000000608082015260a00190565b50505050505050565b5f5f5a90505f61259c846060015190565b905030631d7327566125b16060880188613c25565b87856040518563ffffffff1660e01b81526004016125d2949392919061429d565b6020604051808303815f875af192505050801561260c575060408051601f3d908101601f1916820190925261260991810190614354565b60015b6126c6575f60205f5f3e505f517f215221530000000000000000000000000000000000000000000000000000000081016126925786604051631101335b60e11b81526004016106b0918152604060208201819052600f908201527f41413935206f7574206f66206761730000000000000000000000000000000000606082015260800190565b5f85608001515a6126a39086613b9a565b6126ad9190613ac0565b90506126bd886002888685611eb6565b945050506126c9565b92505b50509392505050565b6001600160a01b0382166127285760405162461bcd60e51b815260206004820152601860248201527f4141393020696e76616c69642062656e6566696369617279000000000000000060448201526064016106b0565b5f826001600160a01b0316826040515f6040518083038185875af1925050503d805f8114612771576040519150601f19603f3d011682016040523d82523d5f602084013e612776565b606091505b50509050806127c75760405162461bcd60e51b815260206004820152601f60248201527f41413931206661696c65642073656e6420746f2062656e65666963696172790060448201526064016106b0565b505050565b5f6127d682613044565b805190602001209050919050565b3063957122ab6127f76040840184613c25565b612804602086018661390d565b612812610120870187613c25565b6040518663ffffffff1660e01b815260040161283295949392919061436b565b5f6040518083038186803b158015612848575f5ffd5b505afa925050508015612859575060015b6128b65761286561417c565b806308c379a0036128ac5750612879614195565b8061288457506128ae565b8051156128a8575f81604051631101335b60e11b81526004016106b0929190613b19565b5050565b505b3d5f5f3e3d5ffd5b50565b604080516060810182525f80825260208201819052918101829052906128de84613114565b90505f6128ea84613114565b82519091506001600160a01b038116612901575080515b602080840151604080860151928501519085015191929165ffffffffffff808316908516101561292f578193505b8065ffffffffffff168365ffffffffffff16111561294b578092505b5050604080516060810182526001600160a01b03909416845265ffffffffffff92831660208501529116908201529250505092915050565b60c081015160e08201515f919080820361299e575092915050565b6129aa82488301613183565b949350505050565b6129bf602083018361390d565b6001600160a01b0316815260208083013590820152608080830135604083015260a0830135606083015260c0808401359183019190915260e0808401359183019190915261010083013590820152365f612a1d610120850185613c25565b90925090508015612a9c576014811015612a795760405162461bcd60e51b815260206004820152601d60248201527f4141393320696e76616c6964207061796d6173746572416e644461746100000060448201526064016106b0565b612a8660145f8385613e8f565b612a8f91613eb6565b60601c60a0840152610e98565b5f60a084015250505050565b60a08101515f9081906001600160a01b0316612ac5576001612ac8565b60035b60ff1690505f8360800151828560600151028560400151010190508360c00151810292505050919050565b5f5f5f5a8551805191925090612b168988612b1160408c018c613c25565b61319a565b60a0820151612b23435f52565b5f6001600160a01b038216612b6f576001600160a01b0383165f908152602081905260409020546dffffffffffffffffffffffffffff16888111612b6957808903612b6b565b5f5b9150505b606084015160208a01516040517f3a871cdd0000000000000000000000000000000000000000000000000000000081526001600160a01b03861692633a871cdd929091612bc2918f9187906004016143a0565b6020604051808303815f8887f193505050508015612bfd575060408051601f3d908101601f19168201909252612bfa91810190614354565b60015b612c8e57612c0961417c565b806308c379a003612c3a5750612c1d614195565b80612c285750612c3c565b8b8160405160200161204891906143c4565b505b8a604051631101335b60e11b81526004016106b09181526040602082018190526016908201527f4141323320726576657274656420286f72204f4f472900000000000000000000606082015260800190565b95506001600160a01b038216612d5d576001600160a01b0383165f90815260208190526040902080546dffffffffffffffffffffffffffff16808a1115612d21578c604051631101335b60e11b81526004016106b09181526040602082018190526017908201527f41413231206469646e2774207061792070726566756e64000000000000000000606082015260800190565b81547fffffffffffffffffffffffffffffffffffff000000000000000000000000000016908a90036dffffffffffffffffffffffffffff161790555b5a85039650505050505094509492505050565b6001600160a01b0382165f90815260016020908152604080832084821c808552925282208054849167ffffffffffffffff8316919085612daf83613ad3565b909155501495945050505050565b825160608181015190915f91848111612e185760405162461bcd60e51b815260206004820152601f60248201527f4141343120746f6f206c6974746c6520766572696669636174696f6e4761730060448201526064016106b0565b60a08201516001600160a01b0381165f90815260208190526040902080548784039291906dffffffffffffffffffffffffffff1689811015612ea6578c604051631101335b60e11b81526004016106b0918152604060208201819052601e908201527f41413331207061796d6173746572206465706f73697420746f6f206c6f770000606082015260800190565b898103825f015f6101000a8154816dffffffffffffffffffffffffffff02191690836dffffffffffffffffffffffffffff160217905550826001600160a01b031663f465c77e858e8e602001518e6040518563ffffffff1660e01b8152600401612f12939291906143a0565b5f604051808303815f8887f193505050508015612f5057506040513d5f823e601f3d908101601f19168201604052612f4d91908101906143f5565b60015b612fe157612f5c61417c565b806308c379a003612f8d5750612f70614195565b80612f7b5750612f8f565b8d81604051602001612048919061447b565b505b8c604051631101335b60e11b81526004016106b09181526040602082018190526016908201527f4141333320726576657274656420286f72204f4f472900000000000000000000606082015260800190565b909e909d509b505050505050505050505050565b5f5f825f0361300857505f928392509050565b5f61301284613114565b9050806040015165ffffffffffff164211806130395750806020015165ffffffffffff1642105b905194909350915050565b6060813560208301355f61306361305e6040870187613c25565b61346d565b90505f61307661305e6060880188613c25565b9050608086013560a087013560c088013560e08901356101008a01355f6130a461305e6101208e018e613c25565b604080516001600160a01b039c909c1660208d01528b81019a909a5260608b019890985250608089019590955260a088019390935260c087019190915260e08601526101008501526101208401526101408084019190915281518084039091018152610160909201905292915050565b604080516060810182525f80825260208201819052918101919091528160a081901c65ffffffffffff81165f0361314e575065ffffffffffff5b604080516060810182526001600160a01b03909316835260d09490941c602083015265ffffffffffff16928101929092525090565b5f8183106131915781613193565b825b9392505050565b8015610e98578251516001600160a01b0381163b156132055784604051631101335b60e11b81526004016106b0918152604060208201819052601f908201527f414131302073656e64657220616c726561647920636f6e737472756374656400606082015260800190565b8351606001516040517f570e1a360000000000000000000000000000000000000000000000000000000081525f916001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169163570e1a3691906132759088908890600401613f1c565b6020604051808303815f8887f1158015613291573d5f5f3e3d5ffd5b50505050506040513d601f19601f820116820180604052508101906132b69190613f2f565b90506001600160a01b0381166133185785604051631101335b60e11b81526004016106b0918152604060208201819052601b908201527f4141313320696e6974436f6465206661696c6564206f72204f4f470000000000606082015260800190565b816001600160a01b0316816001600160a01b0316146133825785604051631101335b60e11b81526004016106b091815260406020808301829052908201527f4141313420696e6974436f6465206d7573742072657475726e2073656e646572606082015260800190565b806001600160a01b03163b5f036133e45785604051631101335b60e11b81526004016106b091815260406020808301829052908201527f4141313520696e6974436f6465206d757374206372656174652073656e646572606082015260800190565b5f6133f26014828688613e8f565b6133fb91613eb6565b60601c9050826001600160a01b031686602001517fd51a9c61267aa6196961883ecf5ff2da6619c37dac0fa92122513fb32c032d2d83895f015160a0015160405161345c9291906001600160a01b0392831681529116602082015260400190565b60405180910390a350505050505050565b5f604051828085833790209392505050565b6040518060a001604052806134dc6040518061010001604052805f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f6001600160a01b031681526020015f81526020015f81525090565b81526020015f81526020015f81526020015f81526020015f81525090565b5f6020828403121561350a575f5ffd5b813563ffffffff81168114613193575f5ffd5b803577ffffffffffffffffffffffffffffffffffffffffffffffff81168114613544575f5ffd5b919050565b5f60208284031215613559575f5ffd5b6131938261351d565b6001600160a01b03811681146128b6575f5ffd5b803561354481613562565b5f5f60408385031215613592575f5ffd5b823561359d81613562565b91506135ab6020840161351d565b90509250929050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b60a0810181811067ffffffffffffffff82111715613601576136016135b4565b60405250565b610100810181811067ffffffffffffffff82111715613601576136016135b4565b601f19601f830116810181811067ffffffffffffffff8211171561364e5761364e6135b4565b6040525050565b5f67ffffffffffffffff82111561366e5761366e6135b4565b50601f01601f191660200190565b5f81830361018081121561368e575f5ffd5b60405161369a816135e1565b8092506101008212156136ab575f5ffd5b60405191506136b982613607565b6136c284613576565b8252602084810135908301526040808501359083015260608085013590830152608080850135908301526136f860a08501613576565b60a083015260c0848101359083015260e0808501359083015290815261010083013560208201526101208301356040820152610140830135606082015261016090920135608090920191909152919050565b5f5f83601f84011261375a575f5ffd5b50813567ffffffffffffffff811115613771575f5ffd5b602083019150836020828501011115613788575f5ffd5b9250929050565b5f5f5f5f6101c085870312156137a3575f5ffd5b843567ffffffffffffffff8111156137b9575f5ffd5b8501601f810187136137c9575f5ffd5b80356137d481613655565b6040516137e18282613628565b8281528960208486010111156137f5575f5ffd5b826020850160208301375f602084830101528097505050505061381b866020870161367c565b92506101a085013567ffffffffffffffff811115613837575f5ffd5b6138438782880161374a565b95989497509550505050565b5f5f83601f84011261385f575f5ffd5b50813567ffffffffffffffff811115613876575f5ffd5b6020830191508360208260051b8501011115613788575f5ffd5b5f5f5f604084860312156138a2575f5ffd5b833567ffffffffffffffff8111156138b8575f5ffd5b6138c48682870161384f565b90945092505060208401356138d881613562565b809150509250925092565b5f5f604083850312156138f4575f5ffd5b82356138ff81613562565b946020939093013593505050565b5f6020828403121561391d575f5ffd5b813561319381613562565b5f5f5f5f5f6060868803121561393c575f5ffd5b853567ffffffffffffffff811115613952575f5ffd5b61395e8882890161374a565b909650945050602086013561397281613562565b9250604086013567ffffffffffffffff81111561398d575f5ffd5b6139998882890161374a565b969995985093965092949392505050565b5f5f602083850312156139bb575f5ffd5b823567ffffffffffffffff8111156139d1575f5ffd5b6139dd8582860161374a565b90969095509350505050565b5f61016082840312156139fa575f5ffd5b50919050565b5f60208284031215613a10575f5ffd5b813567ffffffffffffffff811115613a26575f5ffd5b6129aa848285016139e9565b5f5f5f5f60608587031215613a45575f5ffd5b843567ffffffffffffffff811115613a5b575f5ffd5b613a67878288016139e9565b9450506020850135613a7881613562565b9250604085013567ffffffffffffffff811115613837575f5ffd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b80820180821115610f0557610f05613a93565b5f5f198203613ae457613ae4613a93565b5060010190565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b828152604060208201525f6129aa6040830184613aeb565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f82357ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffea1833603018112613b90575f5ffd5b9190910192915050565b81810381811115610f0557610f05613a93565b5f82357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa1833603018112613b90575f5ffd5b5f5f8335601e19843603018112613bf4575f5ffd5b83018035915067ffffffffffffffff821115613c0e575f5ffd5b6020019150600581901b3603821315613788575f5ffd5b5f5f8335601e19843603018112613c3a575f5ffd5b83018035915067ffffffffffffffff821115613c54575f5ffd5b602001915036819003821315613788575f5ffd5b5f5f8335601e19843603018112613c7d575f5ffd5b830160208101925035905067ffffffffffffffff811115613c9c575f5ffd5b803603821315613788575f5ffd5b81835281816020850137505f602082840101525f6020601f19601f840116840101905092915050565b613ced82613ce083613576565b6001600160a01b03169052565b602081810135908301525f613d056040830183613c68565b6101606040860152613d1c61016086018284613caa565b915050613d2c6060840184613c68565b8583036060870152613d3f838284613caa565b6080868101359088015260a0808701359088015260c0808701359088015260e0808701359088015261010080870135908801529250613d85915050610120840184613c68565b858303610120870152613d99838284613caa565b92505050613dab610140840184613c68565b858303610140870152613dbf838284613caa565b9695505050505050565b604080825281018490525f6060600586901b8301810190830187837ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffea136839003015b89821015613e6d577fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa08786030184528235818112613e47575f5ffd5b613e53868d8301613cd3565b955050602083019250602084019350600182019150613e0b565b505050508281036020840152613e84818587613caa565b979650505050505050565b5f5f85851115613e9d575f5ffd5b83861115613ea9575f5ffd5b5050820193919092039150565b80357fffffffffffffffffffffffffffffffffffffffff0000000000000000000000008116906014841015613f15577fffffffffffffffffffffffffffffffffffffffff000000000000000000000000808560140360031b1b82161691505b5092915050565b602081525f6129aa602083018486613caa565b5f60208284031215613f3f575f5ffd5b815161319381613562565b65ffffffffffff8181168382160190811115610f0557610f05613a93565b818382375f9101908152919050565b86815285602082015265ffffffffffff8516604082015265ffffffffffff84166060820152821515608082015260c060a08201525f613fb960c0830184613aeb565b98975050505050505050565b805182526020810151602083015260408101511515604083015265ffffffffffff606082015116606083015265ffffffffffff60808201511660808301525f60a082015160c060a08501526129aa60c0850182613aeb565b61014081525f614031610140830188613fc5565b905061404a602083018780518252602090810151910152565b845160608301526020948501516080830152835160a08301529284015160c082015281516001600160a01b031660e0820152908301518051610100830152909201516101209092019190915292915050565b60e081525f6140ae60e0830187613fc5565b90506140c7602083018680518252602090810151910152565b8351606083015260208401516080830152825160a0830152602083015160c083015295945050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b5f60038510614155577f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b8482526060602083015261416c6060830185613aeb565b9050826040830152949350505050565b5f60033d11156141925760045f5f3e505f5160e01c5b90565b5f60443d10156141a25790565b6040517ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3d016004823e80513d602482011167ffffffffffffffff821117156141ea57505090565b808201805167ffffffffffffffff811115614206575050505090565b7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3d850101602082840101111561423e575050505090565b61424d60208285010185613628565b509392505050565b5f81518060208401855e5f93019283525090919050565b7f4141353020706f73744f702072657665727465643a200000000000000000000081525f6131936016830184614255565b6101c081525f6142b26101c083018688613caa565b84516001600160a01b038151166020850152602081015160408501526040810151606085015260608101516080850152608081015160a08501526001600160a01b0360a08201511660c085015260c081015160e085015260e08101516101008501525060208501516101208401526040850151610140840152606085015161016084015260808501516101808401528281036101a0840152613e848185613aeb565b5f60208284031215614364575f5ffd5b5051919050565b606081525f61437e606083018789613caa565b6001600160a01b03861660208401528281036040840152613fb9818587613caa565b606081525f6143b26060830186613cd3565b60208301949094525060400152919050565b7f414132332072657665727465643a20000000000000000000000000000000000081525f613193600f830184614255565b5f5f60408385031215614406575f5ffd5b825167ffffffffffffffff81111561441c575f5ffd5b8301601f8101851361442c575f5ffd5b805161443781613655565b6040516144448282613628565b828152876020848601011115614458575f5ffd5b8260208501602083015e5f60209382018401529590910151949694955050505050565b7f414133332072657665727465643a20000000000000000000000000000000000081525f613193600f83018461425556fea2646970667358221220e0c34964218a71842f0d2ad4b72b88b8cf5140c40a0f645e6582e86f75cf58f764736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10a\x01ZW_5`\xE0\x1C\x80c\x8FA\xECZ\x11a\0\xBBW\x80c\xBB\x9F\xE6\xBF\x11a\0qW\x80c\xD68?\x94\x11a\0WW\x80c\xD68?\x94\x14a\x04\xD0W\x80c\xEE!\x94#\x14a\x04\xEFW\x80c\xFC~(m\x14a\x05\x0EW__\xFD[\x80c\xBB\x9F\xE6\xBF\x14a\x04\x9DW\x80c\xC2:\\\xEA\x14a\x04\xB1W__\xFD[\x80c\x9B$\x9Fi\x11a\0\xA1W\x80c\x9B$\x9Fi\x14a\x04LW\x80c\xA6\x1951\x14a\x04kW\x80c\xB7`\xFA\xF9\x14a\x04\x8AW__\xFD[\x80c\x8FA\xECZ\x14a\x04\x19W\x80c\x95q\"\xAB\x14a\x04-W__\xFD[\x80c \\(x\x11a\x01\x10W\x80cK\x1D|\xF5\x11a\0\xF6W\x80cK\x1D|\xF5\x14a\x02eW\x80cR\x87\xCE\x12\x14a\x02\x84W\x80cp\xA0\x821\x14a\x03\xD5W__\xFD[\x80c \\(x\x14a\x02'W\x80c5V~\x1A\x14a\x02FW__\xFD[\x80c\x1B.\x01\xB8\x11a\x01@W\x80c\x1B.\x01\xB8\x14a\x01\xA0W\x80c\x1Ds'V\x14a\x01\xE9W\x80c\x1F\xAD\x94\x8C\x14a\x02\x08W__\xFD[\x80c\x03\x96\xCB`\x14a\x01nW\x80c\x0B\xD2\x8E;\x14a\x01\x81W__\xFD[6a\x01jWa\x01h3a\x05\xE6V[\0[__\xFD[a\x01ha\x01|6`\x04a4\xFAV[a\x06SV[4\x80\x15a\x01\x8CW__\xFD[Pa\x01ha\x01\x9B6`\x04a5IV[a\tjV[4\x80\x15a\x01\xABW__\xFD[Pa\x01\xD6a\x01\xBA6`\x04a5\x81V[`\x01` \x90\x81R_\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xF4W__\xFD[Pa\x01\xD6a\x02\x036`\x04a7\x8FV[a\t\xB1V[4\x80\x15a\x02\x13W__\xFD[Pa\x01ha\x02\"6`\x04a8\x90V[a\x0B1V[4\x80\x15a\x022W__\xFD[Pa\x01ha\x02A6`\x04a8\xE3V[a\x0C\xEFV[4\x80\x15a\x02QW__\xFD[Pa\x01\xD6a\x02`6`\x04a5\x81V[a\x0E\x9EV[4\x80\x15a\x02pW__\xFD[Pa\x01ha\x02\x7F6`\x04a8\x90V[a\x0F\x0BV[4\x80\x15a\x02\x8FW__\xFD[Pa\x03da\x02\x9E6`\x04a9\rV[`@\x80Q`\xA0\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x91\x90\x91RP`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R` \x81\x81R`@\x91\x82\x90 \x82Q`\xA0\x81\x01\x84R\x81Tm\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x16\x83Rn\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x04`\xFF\x16\x15\x15\x94\x83\x01\x94\x90\x94Ro\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04\x90\x92\x16\x92\x82\x01\x92\x90\x92R`\x01\x90\x91\x01Tc\xFF\xFF\xFF\xFF\x81\x16``\x83\x01Rd\x01\0\0\0\0\x90\x04e\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x82\x01R\x90V[`@Qa\x01\xE0\x91\x90_`\xA0\x82\x01\x90Pm\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83Q\x16\x82R` \x83\x01Q\x15\x15` \x83\x01Rm\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@\x84\x01Q\x16`@\x83\x01Rc\xFF\xFF\xFF\xFF``\x84\x01Q\x16``\x83\x01Re\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x84\x01Q\x16`\x80\x83\x01R\x92\x91PPV[4\x80\x15a\x03\xE0W__\xFD[Pa\x01\xD6a\x03\xEF6`\x04a9\rV[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R` \x81\x90R`@\x90 Tm\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[4\x80\x15a\x04$W__\xFD[Pa\x01\xD6`\x01\x81V[4\x80\x15a\x048W__\xFD[Pa\x01ha\x04G6`\x04a9(V[a\x13{V[4\x80\x15a\x04WW__\xFD[Pa\x01ha\x04f6`\x04a9\xAAV[a\x14uV[4\x80\x15a\x04vW__\xFD[Pa\x01\xD6a\x04\x856`\x04a:\0V[a\x15^V[a\x01ha\x04\x986`\x04a9\rV[a\x05\xE6V[4\x80\x15a\x04\xA8W__\xFD[Pa\x01ha\x15\x9FV[4\x80\x15a\x04\xBCW__\xFD[Pa\x01ha\x04\xCB6`\x04a9\rV[a\x17 V[4\x80\x15a\x04\xDBW__\xFD[Pa\x01ha\x04\xEA6`\x04a:2V[a\x19\x99V[4\x80\x15a\x04\xFAW__\xFD[Pa\x01ha\x05\t6`\x04a:\0V[a\x1A\xA1V[4\x80\x15a\x05\x19W__\xFD[Pa\x05\x99a\x05(6`\x04a9\rV[_` \x81\x90R\x90\x81R`@\x90 \x80T`\x01\x90\x91\x01Tm\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x83\x16\x92n\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x04`\xFF\x16\x92o\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x04\x90\x91\x16\x90c\xFF\xFF\xFF\xFF\x81\x16\x90d\x01\0\0\0\0\x90\x04e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85V[`@\x80Qm\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x96\x87\x16\x81R\x94\x15\x15` \x86\x01R\x92\x90\x94\x16\x91\x83\x01\x91\x90\x91Rc\xFF\xFF\xFF\xFF\x16``\x82\x01Re\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16`\x80\x82\x01R`\xA0\x01a\x01\xE0V[a\x05\xF0\x814a\x1D\xA0V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x81\x81R` \x81\x81R`@\x91\x82\x90 \x80T\x92Qm\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x93\x16\x83R\x92\x91\x7F-\xA4f\xA7\xB2C\x04\xF4~\x87\xFA.\x1EZ\x81\xB9\x83\x1C\xE5O\xEC\x19\x05\\\xE2w\xCA/9\xBAB\xC4\x91\x01[`@Q\x80\x91\x03\x90\xA2PPV[3_\x90\x81R` \x81\x90R`@\x90 c\xFF\xFF\xFF\xFF\x82\x16a\x06\xB9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7Fmust specify unstake delay\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\x01\x81\x01Tc\xFF\xFF\xFF\xFF\x90\x81\x16\x90\x83\x16\x10\x15a\x07\x17W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7Fcannot decrease unstake time\0\0\0\0`D\x82\x01R`d\x01a\x06\xB0V[\x80T_\x90a\x07I\x904\x90o\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04m\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a:\xC0V[\x90P_\x81\x11a\x07\x9AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01R\x7Fno stake specified\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06\xB0V[m\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07\xF8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01R\x7Fstake overflow\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06\xB0V[`@\x80Q`\xA0\x81\x01\x82R\x83Tm\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x82R`\x01` \x80\x84\x01\x82\x81R\x86\x84\x16\x85\x87\x01\x90\x81Rc\xFF\xFF\xFF\xFF\x80\x8B\x16``\x88\x01\x90\x81R_`\x80\x89\x01\x81\x81R3\x80\x83R\x96\x82\x90R\x90\x8A\x90 \x98Q\x89T\x95Q\x94Q\x89\x16o\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x02\x7F\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x95\x15\x15n\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x97\x16\x91\x90\x99\x16\x17\x94\x90\x94\x17\x92\x90\x92\x16\x95\x90\x95\x17\x86UQ\x94\x90\x92\x01\x80T\x92Qe\xFF\xFF\xFF\xFF\xFF\xFF\x16d\x01\0\0\0\0\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\x90\x93\x16\x94\x90\x93\x16\x93\x90\x93\x17\x17\x90U\x90Q\x7F\xA5\xAE\x83=\x0B\xB1\xDC\xD62\xD9\x8A\x8Bp\x97>\x85\x16\x81(\x98\xE1\x9B\xF2{p\x07\x1E\xBC\x8D\xC5,\x01\x90a\t]\x90\x84\x90\x87\x90\x91\x82Rc\xFF\xFF\xFF\xFF\x16` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA2PPPV[3_\x90\x81R`\x01` \x90\x81R`@\x80\x83 w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x84R\x90\x91R\x81 \x80T\x91a\t\xA9\x83a:\xD3V[\x91\x90PUPPV[__Z\x90P30\x14a\n\x05W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FAA92 internal call only\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06\xB0V[\x84Q`@\x81\x01Q``\x82\x01Q\x81\x01a\x13\x88\x01Z\x10\x15a\nFW\x7F\xDE\xAD\xDE\xAD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R` _\xFD[\x87Q_\x90\x15a\n\xD4W_a\n_\x84_\x01Q_\x8C\x86a\x1EuV[\x90P\x80a\n\xD2W_a\nra\x08\0a\x1E\x8BV[\x80Q\x90\x91P\x15a\n\xCCW\x84_\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x8A` \x01Q\x7F\x1CO\xAD\xA77L\n\x9E\xE8\x84\x1F\xC3\x8A\xFE\x82\x93-\xC0\xF8\xE6\x90\x12\xE9'\xF0a\xA8\xBA\xE6\x11\xA2\x01\x87` \x01Q\x84`@Qa\n\xC3\x92\x91\x90a;\x19V[`@Q\x80\x91\x03\x90\xA3[`\x01\x92PP[P[_\x88`\x80\x01QZ\x86\x03\x01\x90Pa\x0B#_\x83\x8B\x8B\x8B\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RP\x88\x92Pa\x1E\xB6\x91PPV[\x9A\x99PPPPPPPPPPV[`\x02\x80T\x03a\x0B\x82W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01a\x06\xB0V[`\x02\x80U\x81_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0B\xA1Wa\x0B\xA1a5\xB4V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0B\xDAW\x81` \x01[a\x0B\xC7a4\x7FV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x0B\xBFW\x90P[P\x90P_[\x82\x81\x10\x15a\x0COW_\x82\x82\x81Q\x81\x10a\x0B\xFAWa\x0B\xFAa;1V[` \x02` \x01\x01Q\x90P__a\x0C4\x84\x8A\x8A\x87\x81\x81\x10a\x0C\x1CWa\x0C\x1Ca;1V[\x90P` \x02\x81\x01\x90a\x0C.\x91\x90a;^V[\x85a!\xDDV[\x91P\x91Pa\x0CD\x84\x83\x83_a#\xC3V[PPP`\x01\x01a\x0B\xDFV[P`@Q_\x90\x7F\xBBG\xEE>\x18:U\x8B\x1A/\xF0\x87K\x07\x9F?\xC5G\x8BtT\xEA\xCF+\xFCZ\xF2\xFFXx\xF9r\x90\x82\x90\xA1_[\x83\x81\x10\x15a\x0C\xD7Wa\x0C\xCB\x81\x88\x88\x84\x81\x81\x10a\x0C\x9AWa\x0C\x9Aa;1V[\x90P` \x02\x81\x01\x90a\x0C\xAC\x91\x90a;^V[\x85\x84\x81Q\x81\x10a\x0C\xBEWa\x0C\xBEa;1V[` \x02` \x01\x01Qa%\x8BV[\x90\x91\x01\x90`\x01\x01a\x0C|V[Pa\x0C\xE2\x84\x82a&\xD2V[PP`\x01`\x02UPPPPV[3_\x90\x81R` \x81\x90R`@\x90 \x80Tm\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82\x11\x15a\r^W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FWithdraw amount too large\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06\xB0V[\x80Ta\r{\x90\x83\x90m\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a;\x9AV[\x81T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16m\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x90\x91\x16\x17\x81U`@\x80Q`\x01`\x01`\xA0\x1B\x03\x85\x16\x81R` \x81\x01\x84\x90R3\x91\x7F\xD1\xC1\x9F\xBC\xD4U\x1A^\xDF\xB6mC\xD2\xE37\xC0H7\xAF\xDA4\x82\xB4+\xDFV\x9A\x8F\xCC\xDA\xE5\xFB\x91\x01`@Q\x80\x91\x03\x90\xA2_\x83`\x01`\x01`\xA0\x1B\x03\x16\x83`@Q_`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80_\x81\x14a\x0EBW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x0EGV[``\x91P[PP\x90P\x80a\x0E\x98W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01R\x7Ffailed to withdraw\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06\xB0V[PPPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x01` \x90\x81R`@\x80\x83 w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x84R\x90\x91R\x90\x81\x90 T\x90\x82\x90\x1B\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x16\x17[\x92\x91PPV[`\x02\x80T\x03a\x0F\\W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01a\x06\xB0V[`\x02\x80U\x81_\x80[\x82\x81\x10\x15a\x10\xE0W6\x86\x86\x83\x81\x81\x10a\x0F\x7FWa\x0F\x7Fa;1V[\x90P` \x02\x81\x01\x90a\x0F\x91\x91\x90a;\xADV[\x90P6_a\x0F\x9F\x83\x80a;\xDFV[\x90\x92P\x90P_a\x0F\xB5`@\x85\x01` \x86\x01a9\rV[\x90P_\x19`\x01`\x01`\xA0\x1B\x03\x82\x16\x01a\x10\x10W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FAA96 invalid aggregator\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06\xB0V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\x10\xC4W`\x01`\x01`\xA0\x1B\x03\x81\x16c\xE3V:O\x84\x84a\x10=`@\x89\x01\x89a<%V[`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x10\\\x94\x93\x92\x91\x90a=\xC9V[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x10rW__\xFD[PZ\xFA\x92PPP\x80\x15a\x10\x83WP`\x01[a\x10\xC4W`@Q\x7F\x86\xA9\xF7P\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x06\xB0V[a\x10\xCE\x82\x87a:\xC0V[\x95PP`\x01\x90\x93\x01\x92Pa\x0Fd\x91PPV[P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x10\xFBWa\x10\xFBa5\xB4V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x114W\x81` \x01[a\x11!a4\x7FV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x11\x19W\x90P[P`@Q\x90\x91P\x7F\xBBG\xEE>\x18:U\x8B\x1A/\xF0\x87K\x07\x9F?\xC5G\x8BtT\xEA\xCF+\xFCZ\xF2\xFFXx\xF9r\x90_\x90\xA1_\x80[\x84\x81\x10\x15a\x124W6\x88\x88\x83\x81\x81\x10a\x11~Wa\x11~a;1V[\x90P` \x02\x81\x01\x90a\x11\x90\x91\x90a;\xADV[\x90P6_a\x11\x9E\x83\x80a;\xDFV[\x90\x92P\x90P_a\x11\xB4`@\x85\x01` \x86\x01a9\rV[\x90P\x81_[\x81\x81\x10\x15a\x12\"W_\x89\x89\x81Q\x81\x10a\x11\xD4Wa\x11\xD4a;1V[` \x02` \x01\x01Q\x90P__a\x11\xF6\x8B\x89\x89\x87\x81\x81\x10a\x0C\x1CWa\x0C\x1Ca;1V[\x91P\x91Pa\x12\x06\x84\x83\x83\x89a#\xC3V[\x8Aa\x12\x10\x81a:\xD3V[\x9BPP`\x01\x90\x93\x01\x92Pa\x11\xB9\x91PPV[PP`\x01\x90\x94\x01\x93Pa\x11c\x92PPPV[P_\x90P\x80\x80[\x85\x81\x10\x15a\x137W6\x89\x89\x83\x81\x81\x10a\x12VWa\x12Va;1V[\x90P` \x02\x81\x01\x90a\x12h\x91\x90a;\xADV[\x90Pa\x12z`@\x82\x01` \x83\x01a9\rV[`\x01`\x01`\xA0\x1B\x03\x16\x7FW_\xF3\xAC\xAD\xD5\xAB4\x8F\xE1\x85^!~\x0F6x\xF8\xD7g\xD7IL\x9F\x9F\xEF\xBE\xE2\xE1|\xCAM`@Q`@Q\x80\x91\x03\x90\xA26_a\x12\xBB\x83\x80a;\xDFV[\x90\x92P\x90P\x80_[\x81\x81\x10\x15a\x13&Wa\x13\x05\x88\x85\x85\x84\x81\x81\x10a\x12\xE1Wa\x12\xE1a;1V[\x90P` \x02\x81\x01\x90a\x12\xF3\x91\x90a;^V[\x8B\x8B\x81Q\x81\x10a\x0C\xBEWa\x0C\xBEa;1V[a\x13\x0F\x90\x88a:\xC0V[\x96P\x87a\x13\x1B\x81a:\xD3V[\x98PP`\x01\x01a\x12\xC3V[PP`\x01\x90\x93\x01\x92Pa\x12;\x91PPV[P`@Q_\x90\x7FW_\xF3\xAC\xAD\xD5\xAB4\x8F\xE1\x85^!~\x0F6x\xF8\xD7g\xD7IL\x9F\x9F\xEF\xBE\xE2\xE1|\xCAM\x90\x82\x90\xA2a\x13l\x86\x82a&\xD2V[PP`\x01`\x02UPPPPPPV[\x83\x15\x80\x15a\x13\x91WP`\x01`\x01`\xA0\x1B\x03\x83\x16;\x15[\x15a\x13\xDEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FAA20 account not deployed\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06\xB0V[`\x14\x81\x10a\x14TW_a\x13\xF4`\x14\x82\x84\x86a>\x8FV[a\x13\xFD\x91a>\xB6V[``\x1C\x90P\x80;_\x03a\x14RW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FAA30 paymaster not deployed\0\0\0\0\0`D\x82\x01R`d\x01a\x06\xB0V[P[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R_`$\x82\x01R`D\x01a\x06\xB0V[`@Q\x7FW\x0E\x1A6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R_\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cW\x0E\x1A6\x90a\x14\xDE\x90\x86\x90\x86\x90`\x04\x01a?\x1CV[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x14\xFAW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\x1E\x91\x90a?/V[`@Q\x7Fl\xA7\xB8\x06\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R\x90\x91P`$\x01a\x06\xB0V[_a\x15h\x82a'\xCCV[`@\x80Q` \x81\x01\x92\x90\x92R0\x90\x82\x01RF``\x82\x01R`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[3_\x90\x81R` \x81\x90R`@\x81 `\x01\x81\x01T\x90\x91c\xFF\xFF\xFF\xFF\x90\x91\x16\x90\x03a\x16\nW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\n`$\x82\x01R\x7Fnot staked\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06\xB0V[\x80Tn\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16a\x16mW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01R\x7Falready unstaking\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06\xB0V[`\x01\x81\x01T_\x90a\x16\x84\x90c\xFF\xFF\xFF\xFF\x16Ba?JV[`\x01\x83\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\xFF\xFF\xFF\xFF\x16d\x01\0\0\0\0e\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x90\x81\x02\x91\x90\x91\x17\x90\x91U\x83T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84U`@Q\x90\x81R\x90\x91P3\x90\x7F\xFA\x9B<\x14\xCC\x82\\A,\x9E\xD8\x1B;\xA3e\xA5\xB4YC\x94\x03\xF1\x88)\xE5r\xEDS\xA4\x18\x0F\n\x90` \x01a\x06GV[3_\x90\x81R` \x81\x90R`@\x90 \x80To\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04m\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x80a\x17\xA0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01R\x7FNo stake to withdraw\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06\xB0V[`\x01\x82\x01Td\x01\0\0\0\0\x90\x04e\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x18\x01W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7Fmust call unlockStake() first\0\0\0`D\x82\x01R`d\x01a\x06\xB0V[`\x01\x82\x01TBd\x01\0\0\0\0\x90\x91\x04e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15a\x18fW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FStake withdrawal is not due\0\0\0\0\0`D\x82\x01R`d\x01a\x06\xB0V[`\x01\x82\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\x16\x90U\x81T\x7F\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82U`@\x80Q`\x01`\x01`\xA0\x1B\x03\x85\x16\x81R` \x81\x01\x83\x90R3\x91\x7F\xB7\xC9\x18\xE0\xE2I\xF9\x99\xE9e\xCA\xFE\xB6\xC6d'\x1B?C\x17\xD2\x96F\x15\0\xE7\x1D\xA3\x9F\x0C\xBD\xA3\x91\x01`@Q\x80\x91\x03\x90\xA2_\x83`\x01`\x01`\xA0\x1B\x03\x16\x82`@Q_`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80_\x81\x14a\x19CW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x19HV[``\x91P[PP\x90P\x80a\x0E\x98W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7Ffailed to withdraw stake\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06\xB0V[a\x19\xA1a4\x7FV[a\x19\xAA\x85a'\xE4V[__a\x19\xB7_\x88\x85a!\xDDV[\x91P\x91P_a\x19\xC6\x83\x83a(\xB9V[\x90Pa\x19\xD0C_RV[_a\x19\xDC_\x8A\x87a%\x8BV[\x90Pa\x19\xE6C_RV[_```\x01`\x01`\xA0\x1B\x03\x8A\x16\x15a\x1AWW\x89`\x01`\x01`\xA0\x1B\x03\x16\x89\x89`@Qa\x1A\x12\x92\x91\x90a?hV[_`@Q\x80\x83\x03\x81_\x86Z\xF1\x91PP=\x80_\x81\x14a\x1AKW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x1APV[``\x91P[P\x90\x92P\x90P[\x86`\x80\x01Q\x83\x85` \x01Q\x86`@\x01Q\x85\x85`@Q\x7F\x8Bz\xC9\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x06\xB0\x96\x95\x94\x93\x92\x91\x90a?wV[a\x1A\xA9a4\x7FV[a\x1A\xB2\x82a'\xE4V[__a\x1A\xBF_\x85\x85a!\xDDV[\x84Q`\xA0\x01Q`@\x80Q\x80\x82\x01\x82R_\x80\x82R` \x80\x83\x01\x82\x81R`\x01`\x01`\xA0\x1B\x03\x95\x86\x16\x83R\x82\x82R\x84\x83 \x80Tm\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFo\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x82\x90\x04\x81\x16\x87R`\x01\x92\x83\x01Tc\xFF\xFF\xFF\xFF\x90\x81\x16\x90\x94R\x8DQQ\x88Q\x80\x8A\x01\x8AR\x87\x81R\x80\x87\x01\x88\x81R\x91\x90\x9A\x16\x87R\x86\x86R\x88\x87 \x80T\x93\x90\x93\x04\x90\x91\x16\x89R\x91\x01T\x90\x91\x16\x90R\x83Q\x80\x85\x01\x90\x94R\x81\x84R\x83\x01R\x93\x95P\x91\x93P\x906_a\x1B~`@\x8A\x01\x8Aa<%V[\x90\x92P\x90P_`\x14\x82\x10\x15a\x1B\x93W_a\x1B\xADV[a\x1B\xA0`\x14_\x84\x86a>\x8FV[a\x1B\xA9\x91a>\xB6V[``\x1C[`@\x80Q\x80\x82\x01\x82R_\x80\x82R` \x80\x83\x01\x82\x81R`\x01`\x01`\xA0\x1B\x03\x86\x16\x83R\x90\x82\x90R\x92\x90 \x80To\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04m\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82R`\x01\x01Tc\xFF\xFF\xFF\xFF\x16\x90\x91R\x90\x91P\x93PPPP_a\x1C\x1C\x86\x86a(\xB9V[\x90P_\x81_\x01Q\x90P_`\x01`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x14\x90P_`@Q\x80`\xC0\x01`@R\x80\x8B`\x80\x01Q\x81R` \x01\x8B`@\x01Q\x81R` \x01\x83\x15\x15\x81R` \x01\x85` \x01Qe\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x85`@\x01Qe\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01a\x1C\x95\x8C``\x01Q\x90V[\x90R\x90P`\x01`\x01`\xA0\x1B\x03\x83\x16\x15\x80\x15\x90a\x1C\xBBWP`\x01`\x01`\xA0\x1B\x03\x83\x16`\x01\x14\x15[\x15a\x1DfW`@\x80Q\x80\x82\x01\x82R`\x01`\x01`\xA0\x1B\x03\x85\x16\x80\x82R\x82Q\x80\x84\x01\x84R_\x80\x82R` \x80\x83\x01\x82\x81R\x93\x82R\x81\x81R\x90\x85\x90 \x80To\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04m\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83R`\x01\x01Tc\xFF\xFF\xFF\xFF\x16\x90\x92R\x90\x82\x01R\x90Q\x7F\xFA\xEC\xB4\xE4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Ra\x06\xB0\x90\x83\x90\x89\x90\x89\x90\x8C\x90\x86\x90`\x04\x01a@\x1DV[\x80\x86\x86\x89`@Q\x7F\xE0\xCF\xF0_\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x06\xB0\x94\x93\x92\x91\x90a@\x9CV[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R` \x81\x90R`@\x81 \x80T\x90\x91\x90a\x1D\xD7\x90\x84\x90m\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a:\xC0V[\x90Pm\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1E7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01R\x7Fdeposit overflow\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06\xB0V[\x81T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16m\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x90\x91\x16\x17\x90UPPV[___\x84Q` \x86\x01\x87\x89\x87\xF1\x95\x94PPPPPV[``=\x82\x81\x11\x15a\x1E\x99WP\x81[`@Q` \x82\x01\x81\x01`@R\x81\x81R\x81_` \x83\x01>\x93\x92PPPV[__Z\x85Q\x90\x91P_\x90\x81a\x1E\xCA\x82a)\x83V[`\xA0\x83\x01Q\x90\x91P`\x01`\x01`\xA0\x1B\x03\x81\x16a\x1E\xE9W\x82Q\x93Pa \xC4V[\x80\x93P_\x88Q\x11\x15a \xC4W\x86\x82\x02\x95P`\x02\x8A`\x02\x81\x11\x15a\x1F\x0EWa\x1F\x0Ea@\xF2V[\x14a\x1F\x94W``\x83\x01Q`@Q\x7F\xA9\xA24\t\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16\x91c\xA9\xA24\t\x91a\x1Fa\x90\x8E\x90\x8D\x90\x8C\x90`\x04\x01aA\x1FV[_`@Q\x80\x83\x03\x81_\x88\x80;\x15\x80\x15a\x1FxW__\xFD[P\x87\xF1\x15\x80\x15a\x1F\x8AW=__>=_\xFD[PPPPPa \xC4V[``\x83\x01Q`@Q\x7F\xA9\xA24\t\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16\x91c\xA9\xA24\t\x91a\x1F\xE2\x90\x8E\x90\x8D\x90\x8C\x90`\x04\x01aA\x1FV[_`@Q\x80\x83\x03\x81_\x88\x80;\x15\x80\x15a\x1F\xF9W__\xFD[P\x87\xF1\x93PPPP\x80\x15a \x0BWP`\x01[a \xC4Wa \x17aA|V[\x80c\x08\xC3y\xA0\x03a pWPa +aA\x95V[\x80a 6WPa rV[\x8B\x81`@Q` \x01a H\x91\x90aBlV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Rc\x11\x013[`\xE1\x1B\x82Ra\x06\xB0\x92\x91`\x04\x01a;\x19V[P[\x8A`@Qc\x11\x013[`\xE1\x1B\x81R`\x04\x01a\x06\xB0\x91\x81R`@` \x82\x01\x81\x90R`\x12\x90\x82\x01R\x7FAA50 postOp revert\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x82\x01R`\x80\x01\x90V[Z\x85\x03\x87\x01\x96P\x81\x87\x02\x95P\x85\x89`@\x01Q\x10\x15a!-W\x8A`@Qc\x11\x013[`\xE1\x1B\x81R`\x04\x01a\x06\xB0\x91\x81R`@` \x80\x83\x01\x82\x90R\x90\x82\x01R\x7FAA51 prefund below actualGasCost``\x82\x01R`\x80\x01\x90V[`@\x89\x01Q\x86\x90\x03a!?\x85\x82a\x1D\xA0V[_\x80\x8C`\x02\x81\x11\x15a!SWa!Sa@\xF2V[\x14\x90P\x84`\xA0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x85_\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x8C` \x01Q\x7FIb\x8F\xD1G\x10\x06\xC1H-\xA8\x80(\xE9\xCEM\xBB\x08\x0B\x81\\\x9B\x03D\xD3\x9EZ\x8En\xC1A\x9F\x88` \x01Q\x85\x8D\x8F`@Qa!\xC5\x94\x93\x92\x91\x90\x93\x84R\x91\x15\x15` \x84\x01R`@\x83\x01R``\x82\x01R`\x80\x01\x90V[`@Q\x80\x91\x03\x90\xA4PPPPPPP\x95\x94PPPPPV[___Z\x84Q\x90\x91Pa!\xF0\x86\x82a)\xB2V[a!\xF9\x86a\x15^V[` \x86\x01R`@\x81\x01Q``\x82\x01Q`\x80\x83\x01Q\x17\x17`\xE0\x87\x015\x17a\x01\0\x87\x015\x17n\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\"{W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FAA94 gas values overflow\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06\xB0V[__a\"\x86\x84a*\xA8V[\x90Pa\"\x94\x8A\x8A\x8A\x84a*\xF3V[\x85Q` \x87\x01Q\x91\x99P\x91\x93Pa\"\xAB\x91\x90a-pV[a#\x01W\x89`@Qc\x11\x013[`\xE1\x1B\x81R`\x04\x01a\x06\xB0\x91\x81R`@` \x82\x01\x81\x90R`\x1A\x90\x82\x01R\x7FAA25 invalid account nonce\0\0\0\0\0\0``\x82\x01R`\x80\x01\x90V[a#\tC_RV[`\xA0\x84\x01Q``\x90`\x01`\x01`\xA0\x1B\x03\x16\x15a#1Wa#,\x8B\x8B\x8B\x85\x87a-\xBDV[\x97P\x90P[_Z\x87\x03\x90P\x80\x8B`\xA0\x015\x10\x15a#\x95W\x8B`@Qc\x11\x013[`\xE1\x1B\x81R`\x04\x01a\x06\xB0\x91\x81R`@` \x82\x01\x81\x90R`\x1E\x90\x82\x01R\x7FAA40 over verificationGasLimit\0\0``\x82\x01R`\x80\x01\x90V[`@\x8A\x01\x83\x90R\x81``\x8B\x01R`\xC0\x8B\x015Z\x88\x03\x01\x8A`\x80\x01\x81\x81RPPPPPPPPP\x93P\x93\x91PPV[__a#\xCE\x85a/\xF5V[\x91P\x91P\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x14a$=W\x85`@Qc\x11\x013[`\xE1\x1B\x81R`\x04\x01a\x06\xB0\x91\x81R`@` \x82\x01\x81\x90R`\x14\x90\x82\x01R\x7FAA24 signature error\0\0\0\0\0\0\0\0\0\0\0\0``\x82\x01R`\x80\x01\x90V[\x80\x15a$\x95W\x85`@Qc\x11\x013[`\xE1\x1B\x81R`\x04\x01a\x06\xB0\x91\x81R`@` \x82\x01\x81\x90R`\x17\x90\x82\x01R\x7FAA22 expired or not due\0\0\0\0\0\0\0\0\0``\x82\x01R`\x80\x01\x90V[_a$\x9F\x85a/\xF5V[\x92P\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a%\x04W\x86`@Qc\x11\x013[`\xE1\x1B\x81R`\x04\x01a\x06\xB0\x91\x81R`@` \x82\x01\x81\x90R`\x14\x90\x82\x01R\x7FAA34 signature error\0\0\0\0\0\0\0\0\0\0\0\0``\x82\x01R`\x80\x01\x90V[\x81\x15a%\x82W\x86`@Qc\x11\x013[`\xE1\x1B\x81R`\x04\x01a\x06\xB0\x91\x81R`@` \x82\x01\x81\x90R`!\x90\x82\x01R\x7FAA32 paymaster expired or not du``\x82\x01R\x7Fe\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x80\x82\x01R`\xA0\x01\x90V[PPPPPPPV[__Z\x90P_a%\x9C\x84``\x01Q\x90V[\x90P0c\x1Ds'Va%\xB1``\x88\x01\x88a<%V[\x87\x85`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a%\xD2\x94\x93\x92\x91\x90aB\x9DV[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x92PPP\x80\x15a&\x0CWP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra&\t\x91\x81\x01\x90aCTV[`\x01[a&\xC6W_` __>P_Q\x7F!R!S\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x01a&\x92W\x86`@Qc\x11\x013[`\xE1\x1B\x81R`\x04\x01a\x06\xB0\x91\x81R`@` \x82\x01\x81\x90R`\x0F\x90\x82\x01R\x7FAA95 out of gas\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x82\x01R`\x80\x01\x90V[_\x85`\x80\x01QZa&\xA3\x90\x86a;\x9AV[a&\xAD\x91\x90a:\xC0V[\x90Pa&\xBD\x88`\x02\x88\x86\x85a\x1E\xB6V[\x94PPPa&\xC9V[\x92P[PP\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16a'(W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FAA90 invalid beneficiary\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06\xB0V[_\x82`\x01`\x01`\xA0\x1B\x03\x16\x82`@Q_`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80_\x81\x14a'qW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a'vV[``\x91P[PP\x90P\x80a'\xC7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FAA91 failed send to beneficiary\0`D\x82\x01R`d\x01a\x06\xB0V[PPPV[_a'\xD6\x82a0DV[\x80Q\x90` \x01 \x90P\x91\x90PV[0c\x95q\"\xABa'\xF7`@\x84\x01\x84a<%V[a(\x04` \x86\x01\x86a9\rV[a(\x12a\x01 \x87\x01\x87a<%V[`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a(2\x95\x94\x93\x92\x91\x90aCkV[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a(HW__\xFD[PZ\xFA\x92PPP\x80\x15a(YWP`\x01[a(\xB6Wa(eaA|V[\x80c\x08\xC3y\xA0\x03a(\xACWPa(yaA\x95V[\x80a(\x84WPa(\xAEV[\x80Q\x15a(\xA8W_\x81`@Qc\x11\x013[`\xE1\x1B\x81R`\x04\x01a\x06\xB0\x92\x91\x90a;\x19V[PPV[P[=__>=_\xFD[PV[`@\x80Q``\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R\x90a(\xDE\x84a1\x14V[\x90P_a(\xEA\x84a1\x14V[\x82Q\x90\x91P`\x01`\x01`\xA0\x1B\x03\x81\x16a)\x01WP\x80Q[` \x80\x84\x01Q`@\x80\x86\x01Q\x92\x85\x01Q\x90\x85\x01Q\x91\x92\x91e\xFF\xFF\xFF\xFF\xFF\xFF\x80\x83\x16\x90\x85\x16\x10\x15a)/W\x81\x93P[\x80e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15a)KW\x80\x92P[PP`@\x80Q``\x81\x01\x82R`\x01`\x01`\xA0\x1B\x03\x90\x94\x16\x84Re\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x16` \x85\x01R\x91\x16\x90\x82\x01R\x92PPP\x92\x91PPV[`\xC0\x81\x01Q`\xE0\x82\x01Q_\x91\x90\x80\x82\x03a)\x9EWP\x92\x91PPV[a)\xAA\x82H\x83\x01a1\x83V[\x94\x93PPPPV[a)\xBF` \x83\x01\x83a9\rV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x80\x83\x015\x90\x82\x01R`\x80\x80\x83\x015`@\x83\x01R`\xA0\x83\x015``\x83\x01R`\xC0\x80\x84\x015\x91\x83\x01\x91\x90\x91R`\xE0\x80\x84\x015\x91\x83\x01\x91\x90\x91Ra\x01\0\x83\x015\x90\x82\x01R6_a*\x1Da\x01 \x85\x01\x85a<%V[\x90\x92P\x90P\x80\x15a*\x9CW`\x14\x81\x10\x15a*yW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAA93 invalid paymasterAndData\0\0\0`D\x82\x01R`d\x01a\x06\xB0V[a*\x86`\x14_\x83\x85a>\x8FV[a*\x8F\x91a>\xB6V[``\x1C`\xA0\x84\x01Ra\x0E\x98V[_`\xA0\x84\x01RPPPPV[`\xA0\x81\x01Q_\x90\x81\x90`\x01`\x01`\xA0\x1B\x03\x16a*\xC5W`\x01a*\xC8V[`\x03[`\xFF\x16\x90P_\x83`\x80\x01Q\x82\x85``\x01Q\x02\x85`@\x01Q\x01\x01\x90P\x83`\xC0\x01Q\x81\x02\x92PPP\x91\x90PV[___Z\x85Q\x80Q\x91\x92P\x90a+\x16\x89\x88a+\x11`@\x8C\x01\x8Ca<%V[a1\x9AV[`\xA0\x82\x01Qa+#C_RV[_`\x01`\x01`\xA0\x1B\x03\x82\x16a+oW`\x01`\x01`\xA0\x1B\x03\x83\x16_\x90\x81R` \x81\x90R`@\x90 Tm\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x88\x81\x11a+iW\x80\x89\x03a+kV[_[\x91PP[``\x84\x01Q` \x8A\x01Q`@Q\x7F:\x87\x1C\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x86\x16\x92c:\x87\x1C\xDD\x92\x90\x91a+\xC2\x91\x8F\x91\x87\x90`\x04\x01aC\xA0V[` `@Q\x80\x83\x03\x81_\x88\x87\xF1\x93PPPP\x80\x15a+\xFDWP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra+\xFA\x91\x81\x01\x90aCTV[`\x01[a,\x8EWa,\taA|V[\x80c\x08\xC3y\xA0\x03a,:WPa,\x1DaA\x95V[\x80a,(WPa,<V[\x8B\x81`@Q` \x01a H\x91\x90aC\xC4V[P[\x8A`@Qc\x11\x013[`\xE1\x1B\x81R`\x04\x01a\x06\xB0\x91\x81R`@` \x82\x01\x81\x90R`\x16\x90\x82\x01R\x7FAA23 reverted (or OOG)\0\0\0\0\0\0\0\0\0\0``\x82\x01R`\x80\x01\x90V[\x95P`\x01`\x01`\xA0\x1B\x03\x82\x16a-]W`\x01`\x01`\xA0\x1B\x03\x83\x16_\x90\x81R` \x81\x90R`@\x90 \x80Tm\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x80\x8A\x11\x15a-!W\x8C`@Qc\x11\x013[`\xE1\x1B\x81R`\x04\x01a\x06\xB0\x91\x81R`@` \x82\x01\x81\x90R`\x17\x90\x82\x01R\x7FAA21 didn't pay prefund\0\0\0\0\0\0\0\0\0``\x82\x01R`\x80\x01\x90V[\x81T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90\x8A\x90\x03m\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x17\x90U[Z\x85\x03\x96PPPPPP\x94P\x94\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x01` \x90\x81R`@\x80\x83 \x84\x82\x1C\x80\x85R\x92R\x82 \x80T\x84\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x91\x90\x85a-\xAF\x83a:\xD3V[\x90\x91UP\x14\x95\x94PPPPPV[\x82Q``\x81\x81\x01Q\x90\x91_\x91\x84\x81\x11a.\x18W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FAA41 too little verificationGas\0`D\x82\x01R`d\x01a\x06\xB0V[`\xA0\x82\x01Q`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R` \x81\x90R`@\x90 \x80T\x87\x84\x03\x92\x91\x90m\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x89\x81\x10\x15a.\xA6W\x8C`@Qc\x11\x013[`\xE1\x1B\x81R`\x04\x01a\x06\xB0\x91\x81R`@` \x82\x01\x81\x90R`\x1E\x90\x82\x01R\x7FAA31 paymaster deposit too low\0\0``\x82\x01R`\x80\x01\x90V[\x89\x81\x03\x82_\x01_a\x01\0\n\x81T\x81m\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83m\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x82`\x01`\x01`\xA0\x1B\x03\x16c\xF4e\xC7~\x85\x8E\x8E` \x01Q\x8E`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a/\x12\x93\x92\x91\x90aC\xA0V[_`@Q\x80\x83\x03\x81_\x88\x87\xF1\x93PPPP\x80\x15a/PWP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra/M\x91\x90\x81\x01\x90aC\xF5V[`\x01[a/\xE1Wa/\\aA|V[\x80c\x08\xC3y\xA0\x03a/\x8DWPa/paA\x95V[\x80a/{WPa/\x8FV[\x8D\x81`@Q` \x01a H\x91\x90aD{V[P[\x8C`@Qc\x11\x013[`\xE1\x1B\x81R`\x04\x01a\x06\xB0\x91\x81R`@` \x82\x01\x81\x90R`\x16\x90\x82\x01R\x7FAA33 reverted (or OOG)\0\0\0\0\0\0\0\0\0\0``\x82\x01R`\x80\x01\x90V[\x90\x9E\x90\x9DP\x9BPPPPPPPPPPPPV[__\x82_\x03a0\x08WP_\x92\x83\x92P\x90PV[_a0\x12\x84a1\x14V[\x90P\x80`@\x01Qe\xFF\xFF\xFF\xFF\xFF\xFF\x16B\x11\x80a09WP\x80` \x01Qe\xFF\xFF\xFF\xFF\xFF\xFF\x16B\x10[\x90Q\x94\x90\x93P\x91PPV[``\x815` \x83\x015_a0ca0^`@\x87\x01\x87a<%V[a4mV[\x90P_a0va0^``\x88\x01\x88a<%V[\x90P`\x80\x86\x015`\xA0\x87\x015`\xC0\x88\x015`\xE0\x89\x015a\x01\0\x8A\x015_a0\xA4a0^a\x01 \x8E\x01\x8Ea<%V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x9C\x90\x9C\x16` \x8D\x01R\x8B\x81\x01\x9A\x90\x9AR``\x8B\x01\x98\x90\x98RP`\x80\x89\x01\x95\x90\x95R`\xA0\x88\x01\x93\x90\x93R`\xC0\x87\x01\x91\x90\x91R`\xE0\x86\x01Ra\x01\0\x85\x01Ra\x01 \x84\x01Ra\x01@\x80\x84\x01\x91\x90\x91R\x81Q\x80\x84\x03\x90\x91\x01\x81Ra\x01`\x90\x92\x01\x90R\x92\x91PPV[`@\x80Q``\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R\x81`\xA0\x81\x90\x1Ce\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16_\x03a1NWPe\xFF\xFF\xFF\xFF\xFF\xFF[`@\x80Q``\x81\x01\x82R`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x83R`\xD0\x94\x90\x94\x1C` \x83\x01Re\xFF\xFF\xFF\xFF\xFF\xFF\x16\x92\x81\x01\x92\x90\x92RP\x90V[_\x81\x83\x10a1\x91W\x81a1\x93V[\x82[\x93\x92PPPV[\x80\x15a\x0E\x98W\x82QQ`\x01`\x01`\xA0\x1B\x03\x81\x16;\x15a2\x05W\x84`@Qc\x11\x013[`\xE1\x1B\x81R`\x04\x01a\x06\xB0\x91\x81R`@` \x82\x01\x81\x90R`\x1F\x90\x82\x01R\x7FAA10 sender already constructed\0``\x82\x01R`\x80\x01\x90V[\x83Q``\x01Q`@Q\x7FW\x0E\x1A6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R_\x91`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91cW\x0E\x1A6\x91\x90a2u\x90\x88\x90\x88\x90`\x04\x01a?\x1CV[` `@Q\x80\x83\x03\x81_\x88\x87\xF1\x15\x80\x15a2\x91W=__>=_\xFD[PPPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a2\xB6\x91\x90a?/V[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a3\x18W\x85`@Qc\x11\x013[`\xE1\x1B\x81R`\x04\x01a\x06\xB0\x91\x81R`@` \x82\x01\x81\x90R`\x1B\x90\x82\x01R\x7FAA13 initCode failed or OOG\0\0\0\0\0``\x82\x01R`\x80\x01\x90V[\x81`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14a3\x82W\x85`@Qc\x11\x013[`\xE1\x1B\x81R`\x04\x01a\x06\xB0\x91\x81R`@` \x80\x83\x01\x82\x90R\x90\x82\x01R\x7FAA14 initCode must return sender``\x82\x01R`\x80\x01\x90V[\x80`\x01`\x01`\xA0\x1B\x03\x16;_\x03a3\xE4W\x85`@Qc\x11\x013[`\xE1\x1B\x81R`\x04\x01a\x06\xB0\x91\x81R`@` \x80\x83\x01\x82\x90R\x90\x82\x01R\x7FAA15 initCode must create sender``\x82\x01R`\x80\x01\x90V[_a3\xF2`\x14\x82\x86\x88a>\x8FV[a3\xFB\x91a>\xB6V[``\x1C\x90P\x82`\x01`\x01`\xA0\x1B\x03\x16\x86` \x01Q\x7F\xD5\x1A\x9Ca&z\xA6\x19ia\x88>\xCF_\xF2\xDAf\x19\xC3}\xAC\x0F\xA9!\"Q?\xB3,\x03--\x83\x89_\x01Q`\xA0\x01Q`@Qa4\\\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x16` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA3PPPPPPPV[_`@Q\x82\x80\x85\x837\x90 \x93\x92PPPV[`@Q\x80`\xA0\x01`@R\x80a4\xDC`@Q\x80a\x01\0\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[_` \x82\x84\x03\x12\x15a5\nW__\xFD[\x815c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a1\x93W__\xFD[\x805w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a5DW__\xFD[\x91\x90PV[_` \x82\x84\x03\x12\x15a5YW__\xFD[a1\x93\x82a5\x1DV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a(\xB6W__\xFD[\x805a5D\x81a5bV[__`@\x83\x85\x03\x12\x15a5\x92W__\xFD[\x825a5\x9D\x81a5bV[\x91Pa5\xAB` \x84\x01a5\x1DV[\x90P\x92P\x92\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[`\xA0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a6\x01Wa6\x01a5\xB4V[`@RPV[a\x01\0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a6\x01Wa6\x01a5\xB4V[`\x1F\x19`\x1F\x83\x01\x16\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a6NWa6Na5\xB4V[`@RPPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a6nWa6na5\xB4V[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[_\x81\x83\x03a\x01\x80\x81\x12\x15a6\x8EW__\xFD[`@Qa6\x9A\x81a5\xE1V[\x80\x92Pa\x01\0\x82\x12\x15a6\xABW__\xFD[`@Q\x91Pa6\xB9\x82a6\x07V[a6\xC2\x84a5vV[\x82R` \x84\x81\x015\x90\x83\x01R`@\x80\x85\x015\x90\x83\x01R``\x80\x85\x015\x90\x83\x01R`\x80\x80\x85\x015\x90\x83\x01Ra6\xF8`\xA0\x85\x01a5vV[`\xA0\x83\x01R`\xC0\x84\x81\x015\x90\x83\x01R`\xE0\x80\x85\x015\x90\x83\x01R\x90\x81Ra\x01\0\x83\x015` \x82\x01Ra\x01 \x83\x015`@\x82\x01Ra\x01@\x83\x015``\x82\x01Ra\x01`\x90\x92\x015`\x80\x90\x92\x01\x91\x90\x91R\x91\x90PV[__\x83`\x1F\x84\x01\x12a7ZW__\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a7qW__\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a7\x88W__\xFD[\x92P\x92\x90PV[____a\x01\xC0\x85\x87\x03\x12\x15a7\xA3W__\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a7\xB9W__\xFD[\x85\x01`\x1F\x81\x01\x87\x13a7\xC9W__\xFD[\x805a7\xD4\x81a6UV[`@Qa7\xE1\x82\x82a6(V[\x82\x81R\x89` \x84\x86\x01\x01\x11\x15a7\xF5W__\xFD[\x82` \x85\x01` \x83\x017_` \x84\x83\x01\x01R\x80\x97PPPPPa8\x1B\x86` \x87\x01a6|V[\x92Pa\x01\xA0\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a87W__\xFD[a8C\x87\x82\x88\x01a7JV[\x95\x98\x94\x97P\x95PPPPV[__\x83`\x1F\x84\x01\x12a8_W__\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a8vW__\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a7\x88W__\xFD[___`@\x84\x86\x03\x12\x15a8\xA2W__\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a8\xB8W__\xFD[a8\xC4\x86\x82\x87\x01a8OV[\x90\x94P\x92PP` \x84\x015a8\xD8\x81a5bV[\x80\x91PP\x92P\x92P\x92V[__`@\x83\x85\x03\x12\x15a8\xF4W__\xFD[\x825a8\xFF\x81a5bV[\x94` \x93\x90\x93\x015\x93PPPV[_` \x82\x84\x03\x12\x15a9\x1DW__\xFD[\x815a1\x93\x81a5bV[_____``\x86\x88\x03\x12\x15a9<W__\xFD[\x855g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a9RW__\xFD[a9^\x88\x82\x89\x01a7JV[\x90\x96P\x94PP` \x86\x015a9r\x81a5bV[\x92P`@\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a9\x8DW__\xFD[a9\x99\x88\x82\x89\x01a7JV[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[__` \x83\x85\x03\x12\x15a9\xBBW__\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a9\xD1W__\xFD[a9\xDD\x85\x82\x86\x01a7JV[\x90\x96\x90\x95P\x93PPPPV[_a\x01`\x82\x84\x03\x12\x15a9\xFAW__\xFD[P\x91\x90PV[_` \x82\x84\x03\x12\x15a:\x10W__\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a:&W__\xFD[a)\xAA\x84\x82\x85\x01a9\xE9V[____``\x85\x87\x03\x12\x15a:EW__\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a:[W__\xFD[a:g\x87\x82\x88\x01a9\xE9V[\x94PP` \x85\x015a:x\x81a5bV[\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a87W__\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\x0F\x05Wa\x0F\x05a:\x93V[__\x19\x82\x03a:\xE4Wa:\xE4a:\x93V[P`\x01\x01\x90V[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[\x82\x81R`@` \x82\x01R_a)\xAA`@\x83\x01\x84a:\xEBV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_\x825\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\xA1\x836\x03\x01\x81\x12a;\x90W__\xFD[\x91\x90\x91\x01\x92\x91PPV[\x81\x81\x03\x81\x81\x11\x15a\x0F\x05Wa\x0F\x05a:\x93V[_\x825\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA1\x836\x03\x01\x81\x12a;\x90W__\xFD[__\x835`\x1E\x19\x846\x03\x01\x81\x12a;\xF4W__\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a<\x0EW__\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15a7\x88W__\xFD[__\x835`\x1E\x19\x846\x03\x01\x81\x12a<:W__\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a<TW__\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a7\x88W__\xFD[__\x835`\x1E\x19\x846\x03\x01\x81\x12a<}W__\xFD[\x83\x01` \x81\x01\x92P5\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a<\x9CW__\xFD[\x806\x03\x82\x13\x15a7\x88W__\xFD[\x81\x83R\x81\x81` \x85\x017P_` \x82\x84\x01\x01R_` `\x1F\x19`\x1F\x84\x01\x16\x84\x01\x01\x90P\x92\x91PPV[a<\xED\x82a<\xE0\x83a5vV[`\x01`\x01`\xA0\x1B\x03\x16\x90RV[` \x81\x81\x015\x90\x83\x01R_a=\x05`@\x83\x01\x83a<hV[a\x01``@\x86\x01Ra=\x1Ca\x01`\x86\x01\x82\x84a<\xAAV[\x91PPa=,``\x84\x01\x84a<hV[\x85\x83\x03``\x87\x01Ra=?\x83\x82\x84a<\xAAV[`\x80\x86\x81\x015\x90\x88\x01R`\xA0\x80\x87\x015\x90\x88\x01R`\xC0\x80\x87\x015\x90\x88\x01R`\xE0\x80\x87\x015\x90\x88\x01Ra\x01\0\x80\x87\x015\x90\x88\x01R\x92Pa=\x85\x91PPa\x01 \x84\x01\x84a<hV[\x85\x83\x03a\x01 \x87\x01Ra=\x99\x83\x82\x84a<\xAAV[\x92PPPa=\xABa\x01@\x84\x01\x84a<hV[\x85\x83\x03a\x01@\x87\x01Ra=\xBF\x83\x82\x84a<\xAAV[\x96\x95PPPPPPV[`@\x80\x82R\x81\x01\x84\x90R_```\x05\x86\x90\x1B\x83\x01\x81\x01\x90\x83\x01\x87\x83\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\xA16\x83\x90\x03\x01[\x89\x82\x10\x15a>mW\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA0\x87\x86\x03\x01\x84R\x825\x81\x81\x12a>GW__\xFD[a>S\x86\x8D\x83\x01a<\xD3V[\x95PP` \x83\x01\x92P` \x84\x01\x93P`\x01\x82\x01\x91Pa>\x0BV[PPPP\x82\x81\x03` \x84\x01Ra>\x84\x81\x85\x87a<\xAAV[\x97\x96PPPPPPPV[__\x85\x85\x11\x15a>\x9DW__\xFD[\x83\x86\x11\x15a>\xA9W__\xFD[PP\x82\x01\x93\x91\x90\x92\x03\x91PV[\x805\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x90`\x14\x84\x10\x15a?\x15W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\x80\x85`\x14\x03`\x03\x1B\x1B\x82\x16\x16\x91P[P\x92\x91PPV[` \x81R_a)\xAA` \x83\x01\x84\x86a<\xAAV[_` \x82\x84\x03\x12\x15a??W__\xFD[\x81Qa1\x93\x81a5bV[e\xFF\xFF\xFF\xFF\xFF\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x0F\x05Wa\x0F\x05a:\x93V[\x81\x83\x827_\x91\x01\x90\x81R\x91\x90PV[\x86\x81R\x85` \x82\x01Re\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16`@\x82\x01Re\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16``\x82\x01R\x82\x15\x15`\x80\x82\x01R`\xC0`\xA0\x82\x01R_a?\xB9`\xC0\x83\x01\x84a:\xEBV[\x98\x97PPPPPPPPV[\x80Q\x82R` \x81\x01Q` \x83\x01R`@\x81\x01Q\x15\x15`@\x83\x01Re\xFF\xFF\xFF\xFF\xFF\xFF``\x82\x01Q\x16``\x83\x01Re\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x82\x01Q\x16`\x80\x83\x01R_`\xA0\x82\x01Q`\xC0`\xA0\x85\x01Ra)\xAA`\xC0\x85\x01\x82a:\xEBV[a\x01@\x81R_a@1a\x01@\x83\x01\x88a?\xC5V[\x90Pa@J` \x83\x01\x87\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[\x84Q``\x83\x01R` \x94\x85\x01Q`\x80\x83\x01R\x83Q`\xA0\x83\x01R\x92\x84\x01Q`\xC0\x82\x01R\x81Q`\x01`\x01`\xA0\x1B\x03\x16`\xE0\x82\x01R\x90\x83\x01Q\x80Qa\x01\0\x83\x01R\x90\x92\x01Qa\x01 \x90\x92\x01\x91\x90\x91R\x92\x91PPV[`\xE0\x81R_a@\xAE`\xE0\x83\x01\x87a?\xC5V[\x90Pa@\xC7` \x83\x01\x86\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[\x83Q``\x83\x01R` \x84\x01Q`\x80\x83\x01R\x82Q`\xA0\x83\x01R` \x83\x01Q`\xC0\x83\x01R\x95\x94PPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[_`\x03\x85\x10aAUW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[\x84\x82R``` \x83\x01RaAl``\x83\x01\x85a:\xEBV[\x90P\x82`@\x83\x01R\x94\x93PPPPV[_`\x03=\x11\x15aA\x92W`\x04__>P_Q`\xE0\x1C[\x90V[_`D=\x10\x15aA\xA2W\x90V[`@Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC=\x01`\x04\x82>\x80Q=`$\x82\x01\x11g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15aA\xEAWPP\x90V[\x80\x82\x01\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aB\x06WPPPP\x90V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFC=\x85\x01\x01` \x82\x84\x01\x01\x11\x15aB>WPPPP\x90V[aBM` \x82\x85\x01\x01\x85a6(V[P\x93\x92PPPV[_\x81Q\x80` \x84\x01\x85^_\x93\x01\x92\x83RP\x90\x91\x90PV[\x7FAA50 postOp reverted: \0\0\0\0\0\0\0\0\0\0\x81R_a1\x93`\x16\x83\x01\x84aBUV[a\x01\xC0\x81R_aB\xB2a\x01\xC0\x83\x01\x86\x88a<\xAAV[\x84Q`\x01`\x01`\xA0\x1B\x03\x81Q\x16` \x85\x01R` \x81\x01Q`@\x85\x01R`@\x81\x01Q``\x85\x01R``\x81\x01Q`\x80\x85\x01R`\x80\x81\x01Q`\xA0\x85\x01R`\x01`\x01`\xA0\x1B\x03`\xA0\x82\x01Q\x16`\xC0\x85\x01R`\xC0\x81\x01Q`\xE0\x85\x01R`\xE0\x81\x01Qa\x01\0\x85\x01RP` \x85\x01Qa\x01 \x84\x01R`@\x85\x01Qa\x01@\x84\x01R``\x85\x01Qa\x01`\x84\x01R`\x80\x85\x01Qa\x01\x80\x84\x01R\x82\x81\x03a\x01\xA0\x84\x01Ra>\x84\x81\x85a:\xEBV[_` \x82\x84\x03\x12\x15aCdW__\xFD[PQ\x91\x90PV[``\x81R_aC~``\x83\x01\x87\x89a<\xAAV[`\x01`\x01`\xA0\x1B\x03\x86\x16` \x84\x01R\x82\x81\x03`@\x84\x01Ra?\xB9\x81\x85\x87a<\xAAV[``\x81R_aC\xB2``\x83\x01\x86a<\xD3V[` \x83\x01\x94\x90\x94RP`@\x01R\x91\x90PV[\x7FAA23 reverted: \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R_a1\x93`\x0F\x83\x01\x84aBUV[__`@\x83\x85\x03\x12\x15aD\x06W__\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aD\x1CW__\xFD[\x83\x01`\x1F\x81\x01\x85\x13aD,W__\xFD[\x80QaD7\x81a6UV[`@QaDD\x82\x82a6(V[\x82\x81R\x87` \x84\x86\x01\x01\x11\x15aDXW__\xFD[\x82` \x85\x01` \x83\x01^_` \x93\x82\x01\x84\x01R\x95\x90\x91\x01Q\x94\x96\x94\x95PPPPPV[\x7FAA33 reverted: \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R_a1\x93`\x0F\x83\x01\x84aBUV\xFE\xA2dipfsX\"\x12 \xE0\xC3Id!\x8Aq\x84/\r*\xD4\xB7+\x88\xB8\xCFQ@\xC4\n\x0Fd^e\x82\xE8ou\xCFX\xF7dsolcC\0\x08\x1B\x003",
    );
    /**```solidity
struct MemoryUserOp { address sender; uint256 nonce; uint256 callGasLimit; uint256 verificationGasLimit; uint256 preVerificationGas; address paymaster; uint256 maxFeePerGas; uint256 maxPriorityFeePerGas; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MemoryUserOp {
        pub sender: alloy::sol_types::private::Address,
        pub nonce: alloy::sol_types::private::primitives::aliases::U256,
        pub callGasLimit: alloy::sol_types::private::primitives::aliases::U256,
        pub verificationGasLimit: alloy::sol_types::private::primitives::aliases::U256,
        pub preVerificationGas: alloy::sol_types::private::primitives::aliases::U256,
        pub paymaster: alloy::sol_types::private::Address,
        pub maxFeePerGas: alloy::sol_types::private::primitives::aliases::U256,
        pub maxPriorityFeePerGas: alloy::sol_types::private::primitives::aliases::U256,
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
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::Address,
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
        impl ::core::convert::From<MemoryUserOp> for UnderlyingRustTuple<'_> {
            fn from(value: MemoryUserOp) -> Self {
                (
                    value.sender,
                    value.nonce,
                    value.callGasLimit,
                    value.verificationGasLimit,
                    value.preVerificationGas,
                    value.paymaster,
                    value.maxFeePerGas,
                    value.maxPriorityFeePerGas,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for MemoryUserOp {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    sender: tuple.0,
                    nonce: tuple.1,
                    callGasLimit: tuple.2,
                    verificationGasLimit: tuple.3,
                    preVerificationGas: tuple.4,
                    paymaster: tuple.5,
                    maxFeePerGas: tuple.6,
                    maxPriorityFeePerGas: tuple.7,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for MemoryUserOp {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for MemoryUserOp {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.sender,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.nonce),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.callGasLimit),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.verificationGasLimit),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.preVerificationGas),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.paymaster,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.maxFeePerGas),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.maxPriorityFeePerGas),
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
        impl alloy_sol_types::SolType for MemoryUserOp {
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
        impl alloy_sol_types::SolStruct for MemoryUserOp {
            const NAME: &'static str = "MemoryUserOp";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "MemoryUserOp(address sender,uint256 nonce,uint256 callGasLimit,uint256 verificationGasLimit,uint256 preVerificationGas,address paymaster,uint256 maxFeePerGas,uint256 maxPriorityFeePerGas)",
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.paymaster,
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
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for MemoryUserOp {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.sender,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.nonce)
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
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.paymaster,
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
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.paymaster,
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
struct UserOpInfo { MemoryUserOp mUserOp; bytes32 userOpHash; uint256 prefund; uint256 contextOffset; uint256 preOpGas; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UserOpInfo {
        pub mUserOp: <MemoryUserOp as alloy::sol_types::SolType>::RustType,
        pub userOpHash: alloy::sol_types::private::FixedBytes<32>,
        pub prefund: alloy::sol_types::private::primitives::aliases::U256,
        pub contextOffset: alloy::sol_types::private::primitives::aliases::U256,
        pub preOpGas: alloy::sol_types::private::primitives::aliases::U256,
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
            MemoryUserOp,
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            <MemoryUserOp as alloy::sol_types::SolType>::RustType,
            alloy::sol_types::private::FixedBytes<32>,
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
        impl ::core::convert::From<UserOpInfo> for UnderlyingRustTuple<'_> {
            fn from(value: UserOpInfo) -> Self {
                (
                    value.mUserOp,
                    value.userOpHash,
                    value.prefund,
                    value.contextOffset,
                    value.preOpGas,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for UserOpInfo {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    mUserOp: tuple.0,
                    userOpHash: tuple.1,
                    prefund: tuple.2,
                    contextOffset: tuple.3,
                    preOpGas: tuple.4,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for UserOpInfo {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for UserOpInfo {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <MemoryUserOp as alloy_sol_types::SolType>::tokenize(&self.mUserOp),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.userOpHash),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.prefund),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.contextOffset),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.preOpGas),
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
        impl alloy_sol_types::SolType for UserOpInfo {
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
        impl alloy_sol_types::SolStruct for UserOpInfo {
            const NAME: &'static str = "UserOpInfo";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "UserOpInfo(MemoryUserOp mUserOp,bytes32 userOpHash,uint256 prefund,uint256 contextOffset,uint256 preOpGas)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(1);
                components
                    .push(
                        <MemoryUserOp as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <MemoryUserOp as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <MemoryUserOp as alloy_sol_types::SolType>::eip712_data_word(
                            &self.mUserOp,
                        )
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.userOpHash)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.prefund)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.contextOffset)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.preOpGas)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for UserOpInfo {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <MemoryUserOp as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.mUserOp,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.userOpHash,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.prefund,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.contextOffset,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.preOpGas,
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
                <MemoryUserOp as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.mUserOp,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.userOpHash,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.prefund,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.contextOffset,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.preOpGas,
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
error ValidationResult(IEntryPoint.ReturnInfo returnInfo, IStakeManager.StakeInfo senderInfo, IStakeManager.StakeInfo factoryInfo, IStakeManager.StakeInfo paymasterInfo);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ValidationResult {
        pub returnInfo: <IEntryPoint::ReturnInfo as alloy::sol_types::SolType>::RustType,
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
            IEntryPoint::ReturnInfo,
            IStakeManager::StakeInfo,
            IStakeManager::StakeInfo,
            IStakeManager::StakeInfo,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            <IEntryPoint::ReturnInfo as alloy::sol_types::SolType>::RustType,
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
                    <IEntryPoint::ReturnInfo as alloy_sol_types::SolType>::tokenize(
                        &self.returnInfo,
                    ),
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
error ValidationResultWithAggregation(IEntryPoint.ReturnInfo returnInfo, IStakeManager.StakeInfo senderInfo, IStakeManager.StakeInfo factoryInfo, IStakeManager.StakeInfo paymasterInfo, IEntryPoint.AggregatorStakeInfo aggregatorInfo);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ValidationResultWithAggregation {
        pub returnInfo: <IEntryPoint::ReturnInfo as alloy::sol_types::SolType>::RustType,
        pub senderInfo: <IStakeManager::StakeInfo as alloy::sol_types::SolType>::RustType,
        pub factoryInfo: <IStakeManager::StakeInfo as alloy::sol_types::SolType>::RustType,
        pub paymasterInfo: <IStakeManager::StakeInfo as alloy::sol_types::SolType>::RustType,
        pub aggregatorInfo: <IEntryPoint::AggregatorStakeInfo as alloy::sol_types::SolType>::RustType,
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
            IEntryPoint::ReturnInfo,
            IStakeManager::StakeInfo,
            IStakeManager::StakeInfo,
            IStakeManager::StakeInfo,
            IEntryPoint::AggregatorStakeInfo,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            <IEntryPoint::ReturnInfo as alloy::sol_types::SolType>::RustType,
            <IStakeManager::StakeInfo as alloy::sol_types::SolType>::RustType,
            <IStakeManager::StakeInfo as alloy::sol_types::SolType>::RustType,
            <IStakeManager::StakeInfo as alloy::sol_types::SolType>::RustType,
            <IEntryPoint::AggregatorStakeInfo as alloy::sol_types::SolType>::RustType,
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
                    <IEntryPoint::ReturnInfo as alloy_sol_types::SolType>::tokenize(
                        &self.returnInfo,
                    ),
                    <IStakeManager::StakeInfo as alloy_sol_types::SolType>::tokenize(
                        &self.senderInfo,
                    ),
                    <IStakeManager::StakeInfo as alloy_sol_types::SolType>::tokenize(
                        &self.factoryInfo,
                    ),
                    <IStakeManager::StakeInfo as alloy_sol_types::SolType>::tokenize(
                        &self.paymasterInfo,
                    ),
                    <IEntryPoint::AggregatorStakeInfo as alloy_sol_types::SolType>::tokenize(
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
    /**Function with signature `SIG_VALIDATION_FAILED()` and selector `0x8f41ec5a`.
```solidity
function SIG_VALIDATION_FAILED() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SIG_VALIDATION_FAILEDCall {}
    ///Container type for the return parameters of the [`SIG_VALIDATION_FAILED()`](SIG_VALIDATION_FAILEDCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SIG_VALIDATION_FAILEDReturn {
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
            impl ::core::convert::From<SIG_VALIDATION_FAILEDCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: SIG_VALIDATION_FAILEDCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for SIG_VALIDATION_FAILEDCall {
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
            impl ::core::convert::From<SIG_VALIDATION_FAILEDReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: SIG_VALIDATION_FAILEDReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for SIG_VALIDATION_FAILEDReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for SIG_VALIDATION_FAILEDCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = SIG_VALIDATION_FAILEDReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "SIG_VALIDATION_FAILED()";
            const SELECTOR: [u8; 4] = [143u8, 65u8, 236u8, 90u8];
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
    /**Function with signature `_validateSenderAndPaymaster(bytes,address,bytes)` and selector `0x957122ab`.
```solidity
function _validateSenderAndPaymaster(bytes memory initCode, address sender, bytes memory paymasterAndData) external view;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct _validateSenderAndPaymasterCall {
        pub initCode: alloy::sol_types::private::Bytes,
        pub sender: alloy::sol_types::private::Address,
        pub paymasterAndData: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`_validateSenderAndPaymaster(bytes,address,bytes)`](_validateSenderAndPaymasterCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct _validateSenderAndPaymasterReturn {}
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
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Bytes,
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
            impl ::core::convert::From<_validateSenderAndPaymasterCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: _validateSenderAndPaymasterCall) -> Self {
                    (value.initCode, value.sender, value.paymasterAndData)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for _validateSenderAndPaymasterCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        initCode: tuple.0,
                        sender: tuple.1,
                        paymasterAndData: tuple.2,
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
            impl ::core::convert::From<_validateSenderAndPaymasterReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: _validateSenderAndPaymasterReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for _validateSenderAndPaymasterReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for _validateSenderAndPaymasterCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = _validateSenderAndPaymasterReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "_validateSenderAndPaymaster(bytes,address,bytes)";
            const SELECTOR: [u8; 4] = [149u8, 113u8, 34u8, 171u8];
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.sender,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.paymasterAndData,
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
    /**Function with signature `addStake(uint32)` and selector `0x0396cb60`.
```solidity
function addStake(uint32 unstakeDelaySec) external payable;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addStakeCall {
        pub unstakeDelaySec: u32,
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
                    (value.unstakeDelaySec,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for addStakeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { unstakeDelaySec: tuple.0 }
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
                    > as alloy_sol_types::SolType>::tokenize(&self.unstakeDelaySec),
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
    /**Function with signature `deposits(address)` and selector `0xfc7e286d`.
```solidity
function deposits(address) external view returns (uint112 deposit, bool staked, uint112 stake, uint32 unstakeDelaySec, uint48 withdrawTime);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct depositsCall {
        pub _0: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`deposits(address)`](depositsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct depositsReturn {
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
            impl ::core::convert::From<depositsCall> for UnderlyingRustTuple<'_> {
                fn from(value: depositsCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for depositsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        {
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
            impl ::core::convert::From<depositsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: depositsReturn) -> Self {
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
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for depositsReturn {
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
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for depositsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = depositsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Uint<112>,
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Uint<112>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<48>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "deposits(address)";
            const SELECTOR: [u8; 4] = [252u8, 126u8, 40u8, 109u8];
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
function handleAggregatedOps(IEntryPoint.UserOpsPerAggregator[] memory opsPerAggregator, address beneficiary) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct handleAggregatedOpsCall {
        pub opsPerAggregator: alloy::sol_types::private::Vec<
            <IEntryPoint::UserOpsPerAggregator as alloy::sol_types::SolType>::RustType,
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
                alloy::sol_types::sol_data::Array<IEntryPoint::UserOpsPerAggregator>,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <IEntryPoint::UserOpsPerAggregator as alloy::sol_types::SolType>::RustType,
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
                alloy::sol_types::sol_data::Array<IEntryPoint::UserOpsPerAggregator>,
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
                        IEntryPoint::UserOpsPerAggregator,
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
    /**Function with signature `innerHandleOp(bytes,((address,uint256,uint256,uint256,uint256,address,uint256,uint256),bytes32,uint256,uint256,uint256),bytes)` and selector `0x1d732756`.
```solidity
function innerHandleOp(bytes memory callData, UserOpInfo memory opInfo, bytes memory context) external returns (uint256 actualGasCost);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct innerHandleOpCall {
        pub callData: alloy::sol_types::private::Bytes,
        pub opInfo: <UserOpInfo as alloy::sol_types::SolType>::RustType,
        pub context: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`innerHandleOp(bytes,((address,uint256,uint256,uint256,uint256,address,uint256,uint256),bytes32,uint256,uint256,uint256),bytes)`](innerHandleOpCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct innerHandleOpReturn {
        pub actualGasCost: alloy::sol_types::private::primitives::aliases::U256,
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
                alloy::sol_types::sol_data::Bytes,
                UserOpInfo,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Bytes,
                <UserOpInfo as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<innerHandleOpCall> for UnderlyingRustTuple<'_> {
                fn from(value: innerHandleOpCall) -> Self {
                    (value.callData, value.opInfo, value.context)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for innerHandleOpCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        callData: tuple.0,
                        opInfo: tuple.1,
                        context: tuple.2,
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
            impl ::core::convert::From<innerHandleOpReturn> for UnderlyingRustTuple<'_> {
                fn from(value: innerHandleOpReturn) -> Self {
                    (value.actualGasCost,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for innerHandleOpReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { actualGasCost: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for innerHandleOpCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Bytes,
                UserOpInfo,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = innerHandleOpReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "innerHandleOp(bytes,((address,uint256,uint256,uint256,uint256,address,uint256,uint256),bytes32,uint256,uint256,uint256),bytes)";
            const SELECTOR: [u8; 4] = [29u8, 115u8, 39u8, 86u8];
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
                        &self.callData,
                    ),
                    <UserOpInfo as alloy_sol_types::SolType>::tokenize(&self.opInfo),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.context,
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
    /**Function with signature `nonceSequenceNumber(address,uint192)` and selector `0x1b2e01b8`.
```solidity
function nonceSequenceNumber(address, uint192) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct nonceSequenceNumberCall {
        pub _0: alloy::sol_types::private::Address,
        pub _1: alloy::sol_types::private::primitives::aliases::U192,
    }
    ///Container type for the return parameters of the [`nonceSequenceNumber(address,uint192)`](nonceSequenceNumberCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct nonceSequenceNumberReturn {
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
            impl ::core::convert::From<nonceSequenceNumberCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: nonceSequenceNumberCall) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for nonceSequenceNumberCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0, _1: tuple.1 }
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
            impl ::core::convert::From<nonceSequenceNumberReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: nonceSequenceNumberReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for nonceSequenceNumberReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for nonceSequenceNumberCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<192>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = nonceSequenceNumberReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "nonceSequenceNumber(address,uint192)";
            const SELECTOR: [u8; 4] = [27u8, 46u8, 1u8, 184u8];
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
                    <alloy::sol_types::sol_data::Uint<
                        192,
                    > as alloy_sol_types::SolType>::tokenize(&self._1),
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
    ///Container for all the [`EntryPoint`](self) function calls.
    pub enum EntryPointCalls {
        SIG_VALIDATION_FAILED(SIG_VALIDATION_FAILEDCall),
        _validateSenderAndPaymaster(_validateSenderAndPaymasterCall),
        addStake(addStakeCall),
        balanceOf(balanceOfCall),
        depositTo(depositToCall),
        deposits(depositsCall),
        getDepositInfo(getDepositInfoCall),
        getNonce(getNonceCall),
        getSenderAddress(getSenderAddressCall),
        getUserOpHash(getUserOpHashCall),
        handleAggregatedOps(handleAggregatedOpsCall),
        handleOps(handleOpsCall),
        incrementNonce(incrementNonceCall),
        innerHandleOp(innerHandleOpCall),
        nonceSequenceNumber(nonceSequenceNumberCall),
        simulateHandleOp(simulateHandleOpCall),
        simulateValidation(simulateValidationCall),
        unlockStake(unlockStakeCall),
        withdrawStake(withdrawStakeCall),
        withdrawTo(withdrawToCall),
    }
    #[automatically_derived]
    impl EntryPointCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [3u8, 150u8, 203u8, 96u8],
            [11u8, 210u8, 142u8, 59u8],
            [27u8, 46u8, 1u8, 184u8],
            [29u8, 115u8, 39u8, 86u8],
            [31u8, 173u8, 148u8, 140u8],
            [32u8, 92u8, 40u8, 120u8],
            [53u8, 86u8, 126u8, 26u8],
            [75u8, 29u8, 124u8, 245u8],
            [82u8, 135u8, 206u8, 18u8],
            [112u8, 160u8, 130u8, 49u8],
            [143u8, 65u8, 236u8, 90u8],
            [149u8, 113u8, 34u8, 171u8],
            [155u8, 36u8, 159u8, 105u8],
            [166u8, 25u8, 53u8, 49u8],
            [183u8, 96u8, 250u8, 249u8],
            [187u8, 159u8, 230u8, 191u8],
            [194u8, 58u8, 92u8, 234u8],
            [214u8, 56u8, 63u8, 148u8],
            [238u8, 33u8, 148u8, 35u8],
            [252u8, 126u8, 40u8, 109u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for EntryPointCalls {
        const NAME: &'static str = "EntryPointCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 20usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::SIG_VALIDATION_FAILED(_) => {
                    <SIG_VALIDATION_FAILEDCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::_validateSenderAndPaymaster(_) => {
                    <_validateSenderAndPaymasterCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::addStake(_) => <addStakeCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::balanceOf(_) => {
                    <balanceOfCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::depositTo(_) => {
                    <depositToCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::deposits(_) => <depositsCall as alloy_sol_types::SolCall>::SELECTOR,
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
                Self::innerHandleOp(_) => {
                    <innerHandleOpCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::nonceSequenceNumber(_) => {
                    <nonceSequenceNumberCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<EntryPointCalls>] = &[
                {
                    fn addStake(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EntryPointCalls> {
                        <addStakeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EntryPointCalls::addStake)
                    }
                    addStake
                },
                {
                    fn incrementNonce(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EntryPointCalls> {
                        <incrementNonceCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EntryPointCalls::incrementNonce)
                    }
                    incrementNonce
                },
                {
                    fn nonceSequenceNumber(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EntryPointCalls> {
                        <nonceSequenceNumberCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EntryPointCalls::nonceSequenceNumber)
                    }
                    nonceSequenceNumber
                },
                {
                    fn innerHandleOp(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EntryPointCalls> {
                        <innerHandleOpCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EntryPointCalls::innerHandleOp)
                    }
                    innerHandleOp
                },
                {
                    fn handleOps(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EntryPointCalls> {
                        <handleOpsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EntryPointCalls::handleOps)
                    }
                    handleOps
                },
                {
                    fn withdrawTo(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EntryPointCalls> {
                        <withdrawToCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EntryPointCalls::withdrawTo)
                    }
                    withdrawTo
                },
                {
                    fn getNonce(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EntryPointCalls> {
                        <getNonceCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EntryPointCalls::getNonce)
                    }
                    getNonce
                },
                {
                    fn handleAggregatedOps(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EntryPointCalls> {
                        <handleAggregatedOpsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EntryPointCalls::handleAggregatedOps)
                    }
                    handleAggregatedOps
                },
                {
                    fn getDepositInfo(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EntryPointCalls> {
                        <getDepositInfoCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EntryPointCalls::getDepositInfo)
                    }
                    getDepositInfo
                },
                {
                    fn balanceOf(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EntryPointCalls> {
                        <balanceOfCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EntryPointCalls::balanceOf)
                    }
                    balanceOf
                },
                {
                    fn SIG_VALIDATION_FAILED(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EntryPointCalls> {
                        <SIG_VALIDATION_FAILEDCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EntryPointCalls::SIG_VALIDATION_FAILED)
                    }
                    SIG_VALIDATION_FAILED
                },
                {
                    fn _validateSenderAndPaymaster(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EntryPointCalls> {
                        <_validateSenderAndPaymasterCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EntryPointCalls::_validateSenderAndPaymaster)
                    }
                    _validateSenderAndPaymaster
                },
                {
                    fn getSenderAddress(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EntryPointCalls> {
                        <getSenderAddressCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EntryPointCalls::getSenderAddress)
                    }
                    getSenderAddress
                },
                {
                    fn getUserOpHash(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EntryPointCalls> {
                        <getUserOpHashCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EntryPointCalls::getUserOpHash)
                    }
                    getUserOpHash
                },
                {
                    fn depositTo(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EntryPointCalls> {
                        <depositToCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EntryPointCalls::depositTo)
                    }
                    depositTo
                },
                {
                    fn unlockStake(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EntryPointCalls> {
                        <unlockStakeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EntryPointCalls::unlockStake)
                    }
                    unlockStake
                },
                {
                    fn withdrawStake(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EntryPointCalls> {
                        <withdrawStakeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EntryPointCalls::withdrawStake)
                    }
                    withdrawStake
                },
                {
                    fn simulateHandleOp(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EntryPointCalls> {
                        <simulateHandleOpCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EntryPointCalls::simulateHandleOp)
                    }
                    simulateHandleOp
                },
                {
                    fn simulateValidation(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EntryPointCalls> {
                        <simulateValidationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EntryPointCalls::simulateValidation)
                    }
                    simulateValidation
                },
                {
                    fn deposits(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EntryPointCalls> {
                        <depositsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EntryPointCalls::deposits)
                    }
                    deposits
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
                Self::SIG_VALIDATION_FAILED(inner) => {
                    <SIG_VALIDATION_FAILEDCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::_validateSenderAndPaymaster(inner) => {
                    <_validateSenderAndPaymasterCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::addStake(inner) => {
                    <addStakeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::balanceOf(inner) => {
                    <balanceOfCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::depositTo(inner) => {
                    <depositToCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::deposits(inner) => {
                    <depositsCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
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
                Self::innerHandleOp(inner) => {
                    <innerHandleOpCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::nonceSequenceNumber(inner) => {
                    <nonceSequenceNumberCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::SIG_VALIDATION_FAILED(inner) => {
                    <SIG_VALIDATION_FAILEDCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::_validateSenderAndPaymaster(inner) => {
                    <_validateSenderAndPaymasterCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
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
                Self::deposits(inner) => {
                    <depositsCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::innerHandleOp(inner) => {
                    <innerHandleOpCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::nonceSequenceNumber(inner) => {
                    <nonceSequenceNumberCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
    ///Container for all the [`EntryPoint`](self) custom errors.
    pub enum EntryPointErrors {
        ExecutionResult(ExecutionResult),
        FailedOp(FailedOp),
        SenderAddressResult(SenderAddressResult),
        SignatureValidationFailed(SignatureValidationFailed),
        ValidationResult(ValidationResult),
        ValidationResultWithAggregation(ValidationResultWithAggregation),
    }
    #[automatically_derived]
    impl EntryPointErrors {
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
    impl alloy_sol_types::SolInterface for EntryPointErrors {
        const NAME: &'static str = "EntryPointErrors";
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
            ) -> alloy_sol_types::Result<EntryPointErrors>] = &[
                {
                    fn FailedOp(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EntryPointErrors> {
                        <FailedOp as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EntryPointErrors::FailedOp)
                    }
                    FailedOp
                },
                {
                    fn SenderAddressResult(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EntryPointErrors> {
                        <SenderAddressResult as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EntryPointErrors::SenderAddressResult)
                    }
                    SenderAddressResult
                },
                {
                    fn SignatureValidationFailed(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EntryPointErrors> {
                        <SignatureValidationFailed as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EntryPointErrors::SignatureValidationFailed)
                    }
                    SignatureValidationFailed
                },
                {
                    fn ExecutionResult(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EntryPointErrors> {
                        <ExecutionResult as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EntryPointErrors::ExecutionResult)
                    }
                    ExecutionResult
                },
                {
                    fn ValidationResult(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EntryPointErrors> {
                        <ValidationResult as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EntryPointErrors::ValidationResult)
                    }
                    ValidationResult
                },
                {
                    fn ValidationResultWithAggregation(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EntryPointErrors> {
                        <ValidationResultWithAggregation as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EntryPointErrors::ValidationResultWithAggregation)
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
    ///Container for all the [`EntryPoint`](self) events.
    pub enum EntryPointEvents {
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
    impl EntryPointEvents {
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
    impl alloy_sol_types::SolEventInterface for EntryPointEvents {
        const NAME: &'static str = "EntryPointEvents";
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
    impl alloy_sol_types::private::IntoLogData for EntryPointEvents {
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
    /**Creates a new wrapper around an on-chain [`EntryPoint`](self) contract instance.

See the [wrapper's documentation](`EntryPointInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> EntryPointInstance<T, P, N> {
        EntryPointInstance::<T, P, N>::new(address, provider)
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
        Output = alloy_contract::Result<EntryPointInstance<T, P, N>>,
    > {
        EntryPointInstance::<T, P, N>::deploy(provider)
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
        EntryPointInstance::<T, P, N>::deploy_builder(provider)
    }
    /**A [`EntryPoint`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`EntryPoint`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct EntryPointInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for EntryPointInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("EntryPointInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > EntryPointInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`EntryPoint`](self) contract instance.

See the [wrapper's documentation](`EntryPointInstance`) for more details.*/
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
        ) -> alloy_contract::Result<EntryPointInstance<T, P, N>> {
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
    impl<T, P: ::core::clone::Clone, N> EntryPointInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> EntryPointInstance<T, P, N> {
            EntryPointInstance {
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
    > EntryPointInstance<T, P, N> {
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
        ///Creates a new call builder for the [`SIG_VALIDATION_FAILED`] function.
        pub fn SIG_VALIDATION_FAILED(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, SIG_VALIDATION_FAILEDCall, N> {
            self.call_builder(&SIG_VALIDATION_FAILEDCall {})
        }
        ///Creates a new call builder for the [`_validateSenderAndPaymaster`] function.
        pub fn _validateSenderAndPaymaster(
            &self,
            initCode: alloy::sol_types::private::Bytes,
            sender: alloy::sol_types::private::Address,
            paymasterAndData: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, _validateSenderAndPaymasterCall, N> {
            self.call_builder(
                &_validateSenderAndPaymasterCall {
                    initCode,
                    sender,
                    paymasterAndData,
                },
            )
        }
        ///Creates a new call builder for the [`addStake`] function.
        pub fn addStake(
            &self,
            unstakeDelaySec: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, addStakeCall, N> {
            self.call_builder(&addStakeCall { unstakeDelaySec })
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
        ///Creates a new call builder for the [`deposits`] function.
        pub fn deposits(
            &self,
            _0: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, depositsCall, N> {
            self.call_builder(&depositsCall { _0 })
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
                <IEntryPoint::UserOpsPerAggregator as alloy::sol_types::SolType>::RustType,
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
        ///Creates a new call builder for the [`innerHandleOp`] function.
        pub fn innerHandleOp(
            &self,
            callData: alloy::sol_types::private::Bytes,
            opInfo: <UserOpInfo as alloy::sol_types::SolType>::RustType,
            context: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, innerHandleOpCall, N> {
            self.call_builder(
                &innerHandleOpCall {
                    callData,
                    opInfo,
                    context,
                },
            )
        }
        ///Creates a new call builder for the [`nonceSequenceNumber`] function.
        pub fn nonceSequenceNumber(
            &self,
            _0: alloy::sol_types::private::Address,
            _1: alloy::sol_types::private::primitives::aliases::U192,
        ) -> alloy_contract::SolCallBuilder<T, &P, nonceSequenceNumberCall, N> {
            self.call_builder(&nonceSequenceNumberCall { _0, _1 })
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
    > EntryPointInstance<T, P, N> {
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
