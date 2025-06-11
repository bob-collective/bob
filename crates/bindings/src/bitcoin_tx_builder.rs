///Module containing a contract's types and functions.
/**

```solidity
library BitcoinTx {
    struct Info { bytes4 version; bytes inputVector; bytes outputVector; bytes4 locktime; }
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
}

interface BitcoinTxBuilder {
    function build() external view returns (BitcoinTx.Info memory);
    function setOpReturn(bytes memory _data) external returns (address);
    function setScript(bytes memory _script) external returns (address);
    function setValue(uint64 _value) external returns (address);
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "function",
    "name": "build",
    "inputs": [],
    "outputs": [
      {
        "name": "",
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
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "setOpReturn",
    "inputs": [
      {
        "name": "_data",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract BitcoinTxBuilder"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setScript",
    "inputs": [
      {
        "name": "_script",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract BitcoinTxBuilder"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setValue",
    "inputs": [
      {
        "name": "_value",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract BitcoinTxBuilder"
      }
    ],
    "stateMutability": "nonpayable"
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
pub mod BitcoinTxBuilder {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x6080604052348015600e575f5ffd5b506109738061001c5f395ff3fe608060405234801561000f575f5ffd5b506004361061004a575f3560e01c80630d15a34a1461004e5780631c4ed3271461008b5780638e1a55fc146100d6578063fcab5e48146100eb575b5f5ffd5b61006161005c36600461044c565b6100fe565b60405173ffffffffffffffffffffffffffffffffffffffff90911681526020015b60405180910390f35b610061610099366004610500565b600180547fffffffffffffffffffffffffffffffffffffffffffffffff00000000000000001667ffffffffffffffff929092169190911790553090565b6100de610112565b604051610082919061055c565b6100616100f936600461044c565b6103df565b5f8061010a8382610696565b503092915050565b604080516080810182525f808252606060208301819052928201839052918101919091525f80548190610144906105f9565b9050116101b2576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601660248201527f5363726970742063616e6e6f7420626520656d7074790000000000000000000060448201526064015b60405180910390fd5b60015468010000000000000000900460ff16156102445760505f60020180546101da906105f9565b90501115610244576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601860248201527f4f505f52455455524e206461746120746f6f206c61726765000000000000000060448201526064016101a9565b6001545f9065ffff0000ffff67ff00ff00ff00ff00600883811b91821666ff00ff00ff00ff9490911c93841617601090811c9290921665ff000000ff009190911664ff000000ff9390931692909217901b67ffffffffffffffff1617602081811b91901c1760c01b6040516020016102e491907fffffffffffffffff00000000000000000000000000000000000000000000000091909116815260080190565b60408051808303601f19018152919052600180549192509068010000000000000000900460ff161561031e5761031b60018261077e565b90505b6040515f9061033590839085908490602001610840565b60408051808303601f1901815291905260015490915068010000000000000000900460ff16156103af57805f600201805461036f906105f9565b61037b91506002610888565b60028054610388906105f9565b60405161039d9493925060029060200161089b565b60405160208183030381529060405290505b604080516080810182525f8082528251602081810185528282528301529181019290925260608201529392505050565b600180547fffffffffffffffffffffffffffffffffffffffffffffff00ffffffffffffffff16680100000000000000001790555f600261010a8382610696565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b5f6020828403121561045c575f5ffd5b813567ffffffffffffffff811115610472575f5ffd5b8201601f81018413610482575f5ffd5b803567ffffffffffffffff81111561049c5761049c61041f565b604051601f19603f601f19601f8501160116810181811067ffffffffffffffff821117156104cc576104cc61041f565b6040528181528282016020018610156104e3575f5ffd5b816020840160208301375f91810160200191909152949350505050565b5f60208284031215610510575f5ffd5b813567ffffffffffffffff81168114610527575f5ffd5b9392505050565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b602081527fffffffff0000000000000000000000000000000000000000000000000000000082511660208201525f6020830151608060408401526105a360a084018261052e565b90506040840151601f198483030160608501526105c0828261052e565b9150507fffffffff0000000000000000000000000000000000000000000000000000000060608501511660808401528091505092915050565b600181811c9082168061060d57607f821691505b602082108103610644577f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b50919050565b601f82111561069157805f5260205f20601f840160051c8101602085101561066f5750805b601f840160051c820191505b8181101561068e575f815560010161067b565b50505b505050565b815167ffffffffffffffff8111156106b0576106b061041f565b6106c4816106be84546105f9565b8461064a565b6020601f8211600181146106f6575f83156106df5750848201515b5f19600385901b1c1916600184901b17845561068e565b5f84815260208120601f198516915b828110156107255787850151825560209485019460019092019101610705565b508482101561074257868401515f19600387901b60f8161c191681555b50505050600190811b01905550565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b60ff818116838216019081111561079757610797610751565b92915050565b5f81518060208401855e5f93019283525090919050565b5f81546107c0816105f9565b6001821680156107d7576001811461080a57610837565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0083168652811515820286019350610837565b845f5260205f205f5b8381101561082f57815488820152600190910190602001610813565b505081860193505b50505092915050565b7fff000000000000000000000000000000000000000000000000000000000000008460f81b1681525f61087f610879600184018661079d565b846107b4565b95945050505050565b8082018082111561079757610797610751565b5f6108a6828761079d565b5f81527fff000000000000000000000000000000000000000000000000000000000000008660f81b1660088201527f6a0000000000000000000000000000000000000000000000000000000000000060098201527fff000000000000000000000000000000000000000000000000000000000000008560f81b16600a820152610932600b8201856107b4565b97965050505050505056fea264697066735822122070d9c0d40a37f7455f0613eb14d5efee613ea6fe2cd350e4a4b053645d92a65864736f6c634300081c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15`\x0EW__\xFD[Pa\ts\x80a\0\x1C_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0JW_5`\xE0\x1C\x80c\r\x15\xA3J\x14a\0NW\x80c\x1CN\xD3'\x14a\0\x8BW\x80c\x8E\x1AU\xFC\x14a\0\xD6W\x80c\xFC\xAB^H\x14a\0\xEBW[__\xFD[a\0aa\0\\6`\x04a\x04LV[a\0\xFEV[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0aa\0\x996`\x04a\x05\0V[`\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90U0\x90V[a\0\xDEa\x01\x12V[`@Qa\0\x82\x91\x90a\x05\\V[a\0aa\0\xF96`\x04a\x04LV[a\x03\xDFV[_\x80a\x01\n\x83\x82a\x06\x96V[P0\x92\x91PPV[`@\x80Q`\x80\x81\x01\x82R_\x80\x82R``` \x83\x01\x81\x90R\x92\x82\x01\x83\x90R\x91\x81\x01\x91\x90\x91R_\x80T\x81\x90a\x01D\x90a\x05\xF9V[\x90P\x11a\x01\xB2W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x16`$\x82\x01R\x7FScript cannot be empty\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\x01Th\x01\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16\x15a\x02DW`P_`\x02\x01\x80Ta\x01\xDA\x90a\x05\xF9V[\x90P\x11\x15a\x02DW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FOP_RETURN data too large\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x01\xA9V[`\x01T_\x90e\xFF\xFF\0\0\xFF\xFFg\xFF\0\xFF\0\xFF\0\xFF\0`\x08\x83\x81\x1B\x91\x82\x16f\xFF\0\xFF\0\xFF\0\xFF\x94\x90\x91\x1C\x93\x84\x16\x17`\x10\x90\x81\x1C\x92\x90\x92\x16e\xFF\0\0\0\xFF\0\x91\x90\x91\x16d\xFF\0\0\0\xFF\x93\x90\x93\x16\x92\x90\x92\x17\x90\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x17` \x81\x81\x1B\x91\x90\x1C\x17`\xC0\x1B`@Q` \x01a\x02\xE4\x91\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x90\x91\x16\x81R`\x08\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x91\x90R`\x01\x80T\x91\x92P\x90h\x01\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16\x15a\x03\x1EWa\x03\x1B`\x01\x82a\x07~V[\x90P[`@Q_\x90a\x035\x90\x83\x90\x85\x90\x84\x90` \x01a\x08@V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x91\x90R`\x01T\x90\x91Ph\x01\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16\x15a\x03\xAFW\x80_`\x02\x01\x80Ta\x03o\x90a\x05\xF9V[a\x03{\x91P`\x02a\x08\x88V[`\x02\x80Ta\x03\x88\x90a\x05\xF9V[`@Qa\x03\x9D\x94\x93\x92P`\x02\x90` \x01a\x08\x9BV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P[`@\x80Q`\x80\x81\x01\x82R_\x80\x82R\x82Q` \x81\x81\x01\x85R\x82\x82R\x83\x01R\x91\x81\x01\x92\x90\x92R``\x82\x01R\x93\x92PPPV[`\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16h\x01\0\0\0\0\0\0\0\0\x17\x90U_`\x02a\x01\n\x83\x82a\x06\x96V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15a\x04\\W__\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x04rW__\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x04\x82W__\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x04\x9CWa\x04\x9Ca\x04\x1FV[`@Q`\x1F\x19`?`\x1F\x19`\x1F\x85\x01\x16\x01\x16\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x04\xCCWa\x04\xCCa\x04\x1FV[`@R\x81\x81R\x82\x82\x01` \x01\x86\x10\x15a\x04\xE3W__\xFD[\x81` \x84\x01` \x83\x017_\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[_` \x82\x84\x03\x12\x15a\x05\x10W__\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x05'W__\xFD[\x93\x92PPPV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82Q\x16` \x82\x01R_` \x83\x01Q`\x80`@\x84\x01Ra\x05\xA3`\xA0\x84\x01\x82a\x05.V[\x90P`@\x84\x01Q`\x1F\x19\x84\x83\x03\x01``\x85\x01Ra\x05\xC0\x82\x82a\x05.V[\x91PP\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x85\x01Q\x16`\x80\x84\x01R\x80\x91PP\x92\x91PPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x06\rW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x06DW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\x06\x91W\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a\x06oWP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x06\x8EW_\x81U`\x01\x01a\x06{V[PP[PPPV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06\xB0Wa\x06\xB0a\x04\x1FV[a\x06\xC4\x81a\x06\xBE\x84Ta\x05\xF9V[\x84a\x06JV[` `\x1F\x82\x11`\x01\x81\x14a\x06\xF6W_\x83\x15a\x06\xDFWP\x84\x82\x01Q[_\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua\x06\x8EV[_\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a\x07%W\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a\x07\x05V[P\x84\x82\x10\x15a\x07BW\x86\x84\x01Q_\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[`\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x07\x97Wa\x07\x97a\x07QV[\x92\x91PPV[_\x81Q\x80` \x84\x01\x85^_\x93\x01\x92\x83RP\x90\x91\x90PV[_\x81Ta\x07\xC0\x81a\x05\xF9V[`\x01\x82\x16\x80\x15a\x07\xD7W`\x01\x81\x14a\x08\nWa\x087V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x83\x16\x86R\x81\x15\x15\x82\x02\x86\x01\x93Pa\x087V[\x84_R` _ _[\x83\x81\x10\x15a\x08/W\x81T\x88\x82\x01R`\x01\x90\x91\x01\x90` \x01a\x08\x13V[PP\x81\x86\x01\x93P[PPP\x92\x91PPV[\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84`\xF8\x1B\x16\x81R_a\x08\x7Fa\x08y`\x01\x84\x01\x86a\x07\x9DV[\x84a\x07\xB4V[\x95\x94PPPPPV[\x80\x82\x01\x80\x82\x11\x15a\x07\x97Wa\x07\x97a\x07QV[_a\x08\xA6\x82\x87a\x07\x9DV[_\x81R\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86`\xF8\x1B\x16`\x08\x82\x01R\x7Fj\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\t\x82\x01R\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\xF8\x1B\x16`\n\x82\x01Ra\t2`\x0B\x82\x01\x85a\x07\xB4V[\x97\x96PPPPPPPV\xFE\xA2dipfsX\"\x12 p\xD9\xC0\xD4\n7\xF7E_\x06\x13\xEB\x14\xD5\xEF\xEEa>\xA6\xFE,\xD3P\xE4\xA4\xB0Sd]\x92\xA6XdsolcC\0\x08\x1C\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b506004361061004a575f3560e01c80630d15a34a1461004e5780631c4ed3271461008b5780638e1a55fc146100d6578063fcab5e48146100eb575b5f5ffd5b61006161005c36600461044c565b6100fe565b60405173ffffffffffffffffffffffffffffffffffffffff90911681526020015b60405180910390f35b610061610099366004610500565b600180547fffffffffffffffffffffffffffffffffffffffffffffffff00000000000000001667ffffffffffffffff929092169190911790553090565b6100de610112565b604051610082919061055c565b6100616100f936600461044c565b6103df565b5f8061010a8382610696565b503092915050565b604080516080810182525f808252606060208301819052928201839052918101919091525f80548190610144906105f9565b9050116101b2576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601660248201527f5363726970742063616e6e6f7420626520656d7074790000000000000000000060448201526064015b60405180910390fd5b60015468010000000000000000900460ff16156102445760505f60020180546101da906105f9565b90501115610244576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601860248201527f4f505f52455455524e206461746120746f6f206c61726765000000000000000060448201526064016101a9565b6001545f9065ffff0000ffff67ff00ff00ff00ff00600883811b91821666ff00ff00ff00ff9490911c93841617601090811c9290921665ff000000ff009190911664ff000000ff9390931692909217901b67ffffffffffffffff1617602081811b91901c1760c01b6040516020016102e491907fffffffffffffffff00000000000000000000000000000000000000000000000091909116815260080190565b60408051808303601f19018152919052600180549192509068010000000000000000900460ff161561031e5761031b60018261077e565b90505b6040515f9061033590839085908490602001610840565b60408051808303601f1901815291905260015490915068010000000000000000900460ff16156103af57805f600201805461036f906105f9565b61037b91506002610888565b60028054610388906105f9565b60405161039d9493925060029060200161089b565b60405160208183030381529060405290505b604080516080810182525f8082528251602081810185528282528301529181019290925260608201529392505050565b600180547fffffffffffffffffffffffffffffffffffffffffffffff00ffffffffffffffff16680100000000000000001790555f600261010a8382610696565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b5f6020828403121561045c575f5ffd5b813567ffffffffffffffff811115610472575f5ffd5b8201601f81018413610482575f5ffd5b803567ffffffffffffffff81111561049c5761049c61041f565b604051601f19603f601f19601f8501160116810181811067ffffffffffffffff821117156104cc576104cc61041f565b6040528181528282016020018610156104e3575f5ffd5b816020840160208301375f91810160200191909152949350505050565b5f60208284031215610510575f5ffd5b813567ffffffffffffffff81168114610527575f5ffd5b9392505050565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b602081527fffffffff0000000000000000000000000000000000000000000000000000000082511660208201525f6020830151608060408401526105a360a084018261052e565b90506040840151601f198483030160608501526105c0828261052e565b9150507fffffffff0000000000000000000000000000000000000000000000000000000060608501511660808401528091505092915050565b600181811c9082168061060d57607f821691505b602082108103610644577f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b50919050565b601f82111561069157805f5260205f20601f840160051c8101602085101561066f5750805b601f840160051c820191505b8181101561068e575f815560010161067b565b50505b505050565b815167ffffffffffffffff8111156106b0576106b061041f565b6106c4816106be84546105f9565b8461064a565b6020601f8211600181146106f6575f83156106df5750848201515b5f19600385901b1c1916600184901b17845561068e565b5f84815260208120601f198516915b828110156107255787850151825560209485019460019092019101610705565b508482101561074257868401515f19600387901b60f8161c191681555b50505050600190811b01905550565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b60ff818116838216019081111561079757610797610751565b92915050565b5f81518060208401855e5f93019283525090919050565b5f81546107c0816105f9565b6001821680156107d7576001811461080a57610837565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0083168652811515820286019350610837565b845f5260205f205f5b8381101561082f57815488820152600190910190602001610813565b505081860193505b50505092915050565b7fff000000000000000000000000000000000000000000000000000000000000008460f81b1681525f61087f610879600184018661079d565b846107b4565b95945050505050565b8082018082111561079757610797610751565b5f6108a6828761079d565b5f81527fff000000000000000000000000000000000000000000000000000000000000008660f81b1660088201527f6a0000000000000000000000000000000000000000000000000000000000000060098201527fff000000000000000000000000000000000000000000000000000000000000008560f81b16600a820152610932600b8201856107b4565b97965050505050505056fea264697066735822122070d9c0d40a37f7455f0613eb14d5efee613ea6fe2cd350e4a4b053645d92a65864736f6c634300081c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0JW_5`\xE0\x1C\x80c\r\x15\xA3J\x14a\0NW\x80c\x1CN\xD3'\x14a\0\x8BW\x80c\x8E\x1AU\xFC\x14a\0\xD6W\x80c\xFC\xAB^H\x14a\0\xEBW[__\xFD[a\0aa\0\\6`\x04a\x04LV[a\0\xFEV[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0aa\0\x996`\x04a\x05\0V[`\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90U0\x90V[a\0\xDEa\x01\x12V[`@Qa\0\x82\x91\x90a\x05\\V[a\0aa\0\xF96`\x04a\x04LV[a\x03\xDFV[_\x80a\x01\n\x83\x82a\x06\x96V[P0\x92\x91PPV[`@\x80Q`\x80\x81\x01\x82R_\x80\x82R``` \x83\x01\x81\x90R\x92\x82\x01\x83\x90R\x91\x81\x01\x91\x90\x91R_\x80T\x81\x90a\x01D\x90a\x05\xF9V[\x90P\x11a\x01\xB2W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x16`$\x82\x01R\x7FScript cannot be empty\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\x01Th\x01\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16\x15a\x02DW`P_`\x02\x01\x80Ta\x01\xDA\x90a\x05\xF9V[\x90P\x11\x15a\x02DW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FOP_RETURN data too large\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x01\xA9V[`\x01T_\x90e\xFF\xFF\0\0\xFF\xFFg\xFF\0\xFF\0\xFF\0\xFF\0`\x08\x83\x81\x1B\x91\x82\x16f\xFF\0\xFF\0\xFF\0\xFF\x94\x90\x91\x1C\x93\x84\x16\x17`\x10\x90\x81\x1C\x92\x90\x92\x16e\xFF\0\0\0\xFF\0\x91\x90\x91\x16d\xFF\0\0\0\xFF\x93\x90\x93\x16\x92\x90\x92\x17\x90\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x17` \x81\x81\x1B\x91\x90\x1C\x17`\xC0\x1B`@Q` \x01a\x02\xE4\x91\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x90\x91\x16\x81R`\x08\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x91\x90R`\x01\x80T\x91\x92P\x90h\x01\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16\x15a\x03\x1EWa\x03\x1B`\x01\x82a\x07~V[\x90P[`@Q_\x90a\x035\x90\x83\x90\x85\x90\x84\x90` \x01a\x08@V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x91\x90R`\x01T\x90\x91Ph\x01\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16\x15a\x03\xAFW\x80_`\x02\x01\x80Ta\x03o\x90a\x05\xF9V[a\x03{\x91P`\x02a\x08\x88V[`\x02\x80Ta\x03\x88\x90a\x05\xF9V[`@Qa\x03\x9D\x94\x93\x92P`\x02\x90` \x01a\x08\x9BV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P[`@\x80Q`\x80\x81\x01\x82R_\x80\x82R\x82Q` \x81\x81\x01\x85R\x82\x82R\x83\x01R\x91\x81\x01\x92\x90\x92R``\x82\x01R\x93\x92PPPV[`\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16h\x01\0\0\0\0\0\0\0\0\x17\x90U_`\x02a\x01\n\x83\x82a\x06\x96V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15a\x04\\W__\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x04rW__\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x04\x82W__\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x04\x9CWa\x04\x9Ca\x04\x1FV[`@Q`\x1F\x19`?`\x1F\x19`\x1F\x85\x01\x16\x01\x16\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x04\xCCWa\x04\xCCa\x04\x1FV[`@R\x81\x81R\x82\x82\x01` \x01\x86\x10\x15a\x04\xE3W__\xFD[\x81` \x84\x01` \x83\x017_\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[_` \x82\x84\x03\x12\x15a\x05\x10W__\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x05'W__\xFD[\x93\x92PPPV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82Q\x16` \x82\x01R_` \x83\x01Q`\x80`@\x84\x01Ra\x05\xA3`\xA0\x84\x01\x82a\x05.V[\x90P`@\x84\x01Q`\x1F\x19\x84\x83\x03\x01``\x85\x01Ra\x05\xC0\x82\x82a\x05.V[\x91PP\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x85\x01Q\x16`\x80\x84\x01R\x80\x91PP\x92\x91PPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x06\rW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x06DW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\x06\x91W\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a\x06oWP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x06\x8EW_\x81U`\x01\x01a\x06{V[PP[PPPV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06\xB0Wa\x06\xB0a\x04\x1FV[a\x06\xC4\x81a\x06\xBE\x84Ta\x05\xF9V[\x84a\x06JV[` `\x1F\x82\x11`\x01\x81\x14a\x06\xF6W_\x83\x15a\x06\xDFWP\x84\x82\x01Q[_\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua\x06\x8EV[_\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a\x07%W\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a\x07\x05V[P\x84\x82\x10\x15a\x07BW\x86\x84\x01Q_\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[`\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x07\x97Wa\x07\x97a\x07QV[\x92\x91PPV[_\x81Q\x80` \x84\x01\x85^_\x93\x01\x92\x83RP\x90\x91\x90PV[_\x81Ta\x07\xC0\x81a\x05\xF9V[`\x01\x82\x16\x80\x15a\x07\xD7W`\x01\x81\x14a\x08\nWa\x087V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x83\x16\x86R\x81\x15\x15\x82\x02\x86\x01\x93Pa\x087V[\x84_R` _ _[\x83\x81\x10\x15a\x08/W\x81T\x88\x82\x01R`\x01\x90\x91\x01\x90` \x01a\x08\x13V[PP\x81\x86\x01\x93P[PPP\x92\x91PPV[\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84`\xF8\x1B\x16\x81R_a\x08\x7Fa\x08y`\x01\x84\x01\x86a\x07\x9DV[\x84a\x07\xB4V[\x95\x94PPPPPV[\x80\x82\x01\x80\x82\x11\x15a\x07\x97Wa\x07\x97a\x07QV[_a\x08\xA6\x82\x87a\x07\x9DV[_\x81R\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86`\xF8\x1B\x16`\x08\x82\x01R\x7Fj\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\t\x82\x01R\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85`\xF8\x1B\x16`\n\x82\x01Ra\t2`\x0B\x82\x01\x85a\x07\xB4V[\x97\x96PPPPPPPV\xFE\xA2dipfsX\"\x12 p\xD9\xC0\xD4\n7\xF7E_\x06\x13\xEB\x14\xD5\xEF\xEEa>\xA6\xFE,\xD3P\xE4\xA4\xB0Sd]\x92\xA6XdsolcC\0\x08\x1C\x003",
    );
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `build()` and selector `0x8e1a55fc`.
```solidity
function build() external view returns (BitcoinTx.Info memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct buildCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`build()`](buildCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct buildReturn {
        #[allow(missing_docs)]
        pub _0: <BitcoinTx::Info as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<buildCall> for UnderlyingRustTuple<'_> {
                fn from(value: buildCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for buildCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (BitcoinTx::Info,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <BitcoinTx::Info as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<buildReturn> for UnderlyingRustTuple<'_> {
                fn from(value: buildReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for buildReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for buildCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <BitcoinTx::Info as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (BitcoinTx::Info,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "build()";
            const SELECTOR: [u8; 4] = [142u8, 26u8, 85u8, 252u8];
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
                (<BitcoinTx::Info as alloy_sol_types::SolType>::tokenize(ret),)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: buildReturn = r.into();
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
                        let r: buildReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `setOpReturn(bytes)` and selector `0xfcab5e48`.
```solidity
function setOpReturn(bytes memory _data) external returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setOpReturnCall {
        #[allow(missing_docs)]
        pub _data: alloy::sol_types::private::Bytes,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`setOpReturn(bytes)`](setOpReturnCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setOpReturnReturn {
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
            impl ::core::convert::From<setOpReturnCall> for UnderlyingRustTuple<'_> {
                fn from(value: setOpReturnCall) -> Self {
                    (value._data,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setOpReturnCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _data: tuple.0 }
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
            impl ::core::convert::From<setOpReturnReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setOpReturnReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setOpReturnReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setOpReturnCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Bytes,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setOpReturn(bytes)";
            const SELECTOR: [u8; 4] = [252u8, 171u8, 94u8, 72u8];
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
                        &self._data,
                    ),
                )
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
                        let r: setOpReturnReturn = r.into();
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
                        let r: setOpReturnReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `setScript(bytes)` and selector `0x0d15a34a`.
```solidity
function setScript(bytes memory _script) external returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setScriptCall {
        #[allow(missing_docs)]
        pub _script: alloy::sol_types::private::Bytes,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`setScript(bytes)`](setScriptCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setScriptReturn {
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
            impl ::core::convert::From<setScriptCall> for UnderlyingRustTuple<'_> {
                fn from(value: setScriptCall) -> Self {
                    (value._script,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setScriptCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _script: tuple.0 }
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
            impl ::core::convert::From<setScriptReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setScriptReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setScriptReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setScriptCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Bytes,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setScript(bytes)";
            const SELECTOR: [u8; 4] = [13u8, 21u8, 163u8, 74u8];
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
                        &self._script,
                    ),
                )
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
                        let r: setScriptReturn = r.into();
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
                        let r: setScriptReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `setValue(uint64)` and selector `0x1c4ed327`.
```solidity
function setValue(uint64 _value) external returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setValueCall {
        #[allow(missing_docs)]
        pub _value: u64,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`setValue(uint64)`](setValueCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setValueReturn {
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
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u64,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setValueCall> for UnderlyingRustTuple<'_> {
                fn from(value: setValueCall) -> Self {
                    (value._value,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setValueCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _value: tuple.0 }
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
            impl ::core::convert::From<setValueReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setValueReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setValueReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setValueCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setValue(uint64)";
            const SELECTOR: [u8; 4] = [28u8, 78u8, 211u8, 39u8];
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
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self._value),
                )
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
                        let r: setValueReturn = r.into();
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
                        let r: setValueReturn = r.into();
                        r._0
                    })
            }
        }
    };
    ///Container for all the [`BitcoinTxBuilder`](self) function calls.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum BitcoinTxBuilderCalls {
        #[allow(missing_docs)]
        build(buildCall),
        #[allow(missing_docs)]
        setOpReturn(setOpReturnCall),
        #[allow(missing_docs)]
        setScript(setScriptCall),
        #[allow(missing_docs)]
        setValue(setValueCall),
    }
    #[automatically_derived]
    impl BitcoinTxBuilderCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [13u8, 21u8, 163u8, 74u8],
            [28u8, 78u8, 211u8, 39u8],
            [142u8, 26u8, 85u8, 252u8],
            [252u8, 171u8, 94u8, 72u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for BitcoinTxBuilderCalls {
        const NAME: &'static str = "BitcoinTxBuilderCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 4usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::build(_) => <buildCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::setOpReturn(_) => {
                    <setOpReturnCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setScript(_) => {
                    <setScriptCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setValue(_) => <setValueCall as alloy_sol_types::SolCall>::SELECTOR,
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
            ) -> alloy_sol_types::Result<BitcoinTxBuilderCalls>] = &[
                {
                    fn setScript(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BitcoinTxBuilderCalls> {
                        <setScriptCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(BitcoinTxBuilderCalls::setScript)
                    }
                    setScript
                },
                {
                    fn setValue(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BitcoinTxBuilderCalls> {
                        <setValueCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(BitcoinTxBuilderCalls::setValue)
                    }
                    setValue
                },
                {
                    fn build(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BitcoinTxBuilderCalls> {
                        <buildCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(BitcoinTxBuilderCalls::build)
                    }
                    build
                },
                {
                    fn setOpReturn(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BitcoinTxBuilderCalls> {
                        <setOpReturnCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(BitcoinTxBuilderCalls::setOpReturn)
                    }
                    setOpReturn
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
            ) -> alloy_sol_types::Result<BitcoinTxBuilderCalls>] = &[
                {
                    fn setScript(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BitcoinTxBuilderCalls> {
                        <setScriptCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(BitcoinTxBuilderCalls::setScript)
                    }
                    setScript
                },
                {
                    fn setValue(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BitcoinTxBuilderCalls> {
                        <setValueCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(BitcoinTxBuilderCalls::setValue)
                    }
                    setValue
                },
                {
                    fn build(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BitcoinTxBuilderCalls> {
                        <buildCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(BitcoinTxBuilderCalls::build)
                    }
                    build
                },
                {
                    fn setOpReturn(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BitcoinTxBuilderCalls> {
                        <setOpReturnCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(BitcoinTxBuilderCalls::setOpReturn)
                    }
                    setOpReturn
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
                Self::build(inner) => {
                    <buildCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::setOpReturn(inner) => {
                    <setOpReturnCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setScript(inner) => {
                    <setScriptCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::setValue(inner) => {
                    <setValueCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::build(inner) => {
                    <buildCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::setOpReturn(inner) => {
                    <setOpReturnCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setScript(inner) => {
                    <setScriptCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setValue(inner) => {
                    <setValueCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`BitcoinTxBuilder`](self) contract instance.

See the [wrapper's documentation](`BitcoinTxBuilderInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> BitcoinTxBuilderInstance<P, N> {
        BitcoinTxBuilderInstance::<P, N>::new(address, provider)
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
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<BitcoinTxBuilderInstance<P, N>>,
    > {
        BitcoinTxBuilderInstance::<P, N>::deploy(provider)
    }
    /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
    #[inline]
    pub fn deploy_builder<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(provider: P) -> alloy_contract::RawCallBuilder<P, N> {
        BitcoinTxBuilderInstance::<P, N>::deploy_builder(provider)
    }
    /**A [`BitcoinTxBuilder`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`BitcoinTxBuilder`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct BitcoinTxBuilderInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for BitcoinTxBuilderInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("BitcoinTxBuilderInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > BitcoinTxBuilderInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`BitcoinTxBuilder`](self) contract instance.

See the [wrapper's documentation](`BitcoinTxBuilderInstance`) for more details.*/
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
        ) -> alloy_contract::Result<BitcoinTxBuilderInstance<P, N>> {
            let call_builder = Self::deploy_builder(provider);
            let contract_address = call_builder.deploy().await?;
            Ok(Self::new(contract_address, call_builder.provider))
        }
        /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
        #[inline]
        pub fn deploy_builder(provider: P) -> alloy_contract::RawCallBuilder<P, N> {
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
    impl<P: ::core::clone::Clone, N> BitcoinTxBuilderInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> BitcoinTxBuilderInstance<P, N> {
            BitcoinTxBuilderInstance {
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
    > BitcoinTxBuilderInstance<P, N> {
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
        ///Creates a new call builder for the [`build`] function.
        pub fn build(&self) -> alloy_contract::SolCallBuilder<&P, buildCall, N> {
            self.call_builder(&buildCall)
        }
        ///Creates a new call builder for the [`setOpReturn`] function.
        pub fn setOpReturn(
            &self,
            _data: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, setOpReturnCall, N> {
            self.call_builder(&setOpReturnCall { _data })
        }
        ///Creates a new call builder for the [`setScript`] function.
        pub fn setScript(
            &self,
            _script: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, setScriptCall, N> {
            self.call_builder(&setScriptCall { _script })
        }
        ///Creates a new call builder for the [`setValue`] function.
        pub fn setValue(
            &self,
            _value: u64,
        ) -> alloy_contract::SolCallBuilder<&P, setValueCall, N> {
            self.call_builder(&setValueCall { _value })
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > BitcoinTxBuilderInstance<P, N> {
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
