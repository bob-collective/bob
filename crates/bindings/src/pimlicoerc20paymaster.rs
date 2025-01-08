///Module containing a contract's types and functions.
/**

```solidity
library IPaymaster {
    type PostOpMode is uint8;
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
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct PostOpMode(u8);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<PostOpMode> for u8 {
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
        impl PostOpMode {
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
        impl alloy_sol_types::SolType for PostOpMode {
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
        impl alloy_sol_types::EventTopic for PostOpMode {
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
library IPaymaster {
    type PostOpMode is uint8;
}

interface PimlicoERC20Paymaster {
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

    event ConfigUpdated(uint32 priceMarkup, uint32 updateThreshold);
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    event UserOperationSponsored(address indexed user, uint256 actualTokenNeeded, uint256 actualGasCost);

    constructor(address _token, address _entryPoint, address _tokenOracle, address _nativeAssetOracle, address _owner, uint8 _tokenDecimals);

    function REFUND_POSTOP_COST() external view returns (uint256);
    function addStake(uint32 unstakeDelaySec) external payable;
    function deposit() external payable;
    function entryPoint() external view returns (address);
    function getDeposit() external view returns (uint256);
    function nativeAssetOracle() external view returns (address);
    function owner() external view returns (address);
    function postOp(IPaymaster.PostOpMode mode, bytes memory context, uint256 actualGasCost) external;
    function previousPrice() external view returns (uint192);
    function priceDenominator() external view returns (uint256);
    function priceMarkup() external view returns (uint32);
    function priceUpdateThreshold() external view returns (uint32);
    function renounceOwnership() external;
    function token() external view returns (address);
    function tokenDecimals() external view returns (uint256);
    function tokenOracle() external view returns (address);
    function transferOwnership(address newOwner) external;
    function unlockStake() external;
    function updateConfig(uint32 _priceMarkup, uint32 _updateThreshold) external;
    function updatePrice() external;
    function validatePaymasterUserOp(UserOperation memory userOp, bytes32 userOpHash, uint256 maxCost) external returns (bytes memory context, uint256 validationData);
    function withdrawStake(address payable withdrawAddress) external;
    function withdrawTo(address payable withdrawAddress, uint256 amount) external;
    function withdrawToken(address to, uint256 amount) external;
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [
      {
        "name": "_token",
        "type": "address",
        "internalType": "contract IERC20"
      },
      {
        "name": "_entryPoint",
        "type": "address",
        "internalType": "contract IEntryPoint"
      },
      {
        "name": "_tokenOracle",
        "type": "address",
        "internalType": "contract IOracle"
      },
      {
        "name": "_nativeAssetOracle",
        "type": "address",
        "internalType": "contract IOracle"
      },
      {
        "name": "_owner",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "_tokenDecimals",
        "type": "uint8",
        "internalType": "uint8"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "REFUND_POSTOP_COST",
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
    "name": "deposit",
    "inputs": [],
    "outputs": [],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "entryPoint",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IEntryPoint"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getDeposit",
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
    "name": "nativeAssetOracle",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IOracle"
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
    "name": "postOp",
    "inputs": [
      {
        "name": "mode",
        "type": "uint8",
        "internalType": "enum IPaymaster.PostOpMode"
      },
      {
        "name": "context",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "actualGasCost",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "previousPrice",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint192",
        "internalType": "uint192"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "priceDenominator",
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
    "name": "priceMarkup",
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
    "name": "priceUpdateThreshold",
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
    "name": "renounceOwnership",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "token",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IERC20"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "tokenDecimals",
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
    "name": "tokenOracle",
    "inputs": [],
    "outputs": [
      {
        "name": "",
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
    "name": "unlockStake",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "updateConfig",
    "inputs": [
      {
        "name": "_priceMarkup",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "_updateThreshold",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "updatePrice",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "validatePaymasterUserOp",
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
      },
      {
        "name": "userOpHash",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "maxCost",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "context",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "validationData",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
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
    "name": "withdrawToken",
    "inputs": [
      {
        "name": "to",
        "type": "address",
        "internalType": "address"
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
    "type": "event",
    "name": "ConfigUpdated",
    "inputs": [
      {
        "name": "priceMarkup",
        "type": "uint32",
        "indexed": false,
        "internalType": "uint32"
      },
      {
        "name": "updateThreshold",
        "type": "uint32",
        "indexed": false,
        "internalType": "uint32"
      }
    ],
    "anonymous": false
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
    "name": "UserOperationSponsored",
    "inputs": [
      {
        "name": "user",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "actualTokenNeeded",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "actualGasCost",
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
pub mod PimlicoERC20Paymaster {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x610120604052348015610010575f5ffd5b5060405161243938038061243983398101604081905261002f91610371565b846100393361022f565b6001600160a01b0390811660805286811660a05284811660e052831661010052600180546001600160c01b031665030d4000864760c51b17905561007c8261027e565b61008781600a6104e8565b60c08181525050836001600160a01b031663313ce5676040518163ffffffff1660e01b8152600401602060405180830381865afa1580156100ca573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906100ee91906104fd565b60ff166008146101585760405162461bcd60e51b815260206004820152602a60248201527f50502d4552433230203a20746f6b656e206f7261636c6520646563696d616c73604482015269040daeae6e840c4ca40760b31b60648201526084015b60405180910390fd5b826001600160a01b031663313ce5676040518163ffffffff1660e01b8152600401602060405180830381865afa158015610194573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906101b891906104fd565b60ff166008146102245760405162461bcd60e51b815260206004820152603160248201527f50502d4552433230203a206e6174697665206173736574206f7261636c6520646044820152700cac6d2dac2d8e640daeae6e840c4ca407607b1b606482015260840161014f565b505050505050610516565b5f80546001600160a01b038381166001600160a01b0319831681178455604051919092169283917f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09190a35050565b5f546001600160a01b031633146102d75760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e6572604482015260640161014f565b6001600160a01b03811661033c5760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b606482015260840161014f565b6103458161022f565b50565b6001600160a01b0381168114610345575f5ffd5b805160ff8116811461036c575f5ffd5b919050565b5f5f5f5f5f5f60c08789031215610386575f5ffd5b865161039181610348565b60208801519096506103a281610348565b60408801519095506103b381610348565b60608801519094506103c481610348565b60808801519093506103d581610348565b91506103e360a0880161035c565b90509295509295509295565b634e487b7160e01b5f52601160045260245ffd5b6001815b600184111561043e57808504811115610422576104226103ef565b600184161561043057908102905b60019390931c928002610407565b935093915050565b5f82610454575060016104e2565b8161046057505f6104e2565b816001811461047657600281146104805761049c565b60019150506104e2565b60ff841115610491576104916103ef565b50506001821b6104e2565b5060208310610133831016604e8410600b84101617156104bf575081810a6104e2565b6104cb5f198484610403565b805f19048211156104de576104de6103ef565b0290505b92915050565b5f6104f660ff841683610446565b9392505050565b5f6020828403121561050d575f5ffd5b6104f68261035c565b60805160a05160c05160e05161010051611e766105c35f395f818161048d01528181610984015261123601525f81816102ce01528181610958015261120a01525f8181610222015281816109b0015261128101525f818161050c01528181610b280152818161144c015261179701525f81816103f5015281816105c0015281816106bd01528181610c0c01528181610d0501528181610d8f01528181610e33015261117e0152611e765ff3fe608060405260043610610183575f3560e01c80639e281a98116100d1578063cdcf4b9b1161007c578063f2fde38b11610057578063f2fde38b146104af578063f465c77e146104ce578063fc0c546a146104fb575f5ffd5b8063cdcf4b9b1461045e578063d0e30db014610474578063efb1ad5d1461047c575f5ffd5b8063bb9fe6bf116100ac578063bb9fe6bf14610417578063c23a5cea1461042b578063c399ec881461044a575f5ffd5b80639e281a98146103a6578063a9a23409146103c5578063b0d691fe146103e4575f5ffd5b8063673a7e28116101315780638da5cb5b1161010c5780638da5cb5b1461031c578063914e245a146103385780639dbdb97714610391575f5ffd5b8063673a7e28146102a95780636c5ec25c146102bd578063715018a614610308575f5ffd5b80633b97e856116101615780633b97e856146102115780633c2154bc146102525780633e04619d14610271575f5ffd5b80630396cb6014610187578063205c28781461019c5780633a34c83f146101bb575b5f5ffd5b61019a6101953660046118f7565b61052e565b005b3480156101a7575f5ffd5b5061019a6101b636600461192b565b610625565b3480156101c6575f5ffd5b506001546101f7907c0100000000000000000000000000000000000000000000000000000000900463ffffffff1681565b60405163ffffffff90911681526020015b60405180910390f35b34801561021c575f5ffd5b506102447f000000000000000000000000000000000000000000000000000000000000000081565b604051908152602001610208565b34801561025d575f5ffd5b5061019a61026c366004611955565b6106fe565b34801561027c575f5ffd5b506001546101f7907801000000000000000000000000000000000000000000000000900463ffffffff1681565b3480156102b4575f5ffd5b5061019a610952565b3480156102c8575f5ffd5b506102f07f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b039091168152602001610208565b348015610313575f5ffd5b5061019a610a2c565b348015610327575f5ffd5b505f546001600160a01b03166102f0565b348015610343575f5ffd5b506001546103689077ffffffffffffffffffffffffffffffffffffffffffffffff1681565b60405177ffffffffffffffffffffffffffffffffffffffffffffffff9091168152602001610208565b34801561039c575f5ffd5b50610244619c4081565b3480156103b1575f5ffd5b5061019a6103c036600461192b565b610a90565b3480156103d0575f5ffd5b5061019a6103df366004611986565b610b97565b3480156103ef575f5ffd5b506102f07f000000000000000000000000000000000000000000000000000000000000000081565b348015610422575f5ffd5b5061019a610bb1565b348015610436575f5ffd5b5061019a610445366004611a10565b610c74565b348015610455575f5ffd5b50610244610d5f565b348015610469575f5ffd5b50610244620f424081565b61019a610e05565b348015610487575f5ffd5b506102f07f000000000000000000000000000000000000000000000000000000000000000081565b3480156104ba575f5ffd5b5061019a6104c9366004611a10565b610e7e565b3480156104d9575f5ffd5b506104ed6104e8366004611a2b565b610f5f565b604051610208929190611a7a565b348015610506575f5ffd5b506102f07f000000000000000000000000000000000000000000000000000000000000000081565b5f546001600160a01b0316331461058c5760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e657260448201526064015b60405180910390fd5b6040517f0396cb6000000000000000000000000000000000000000000000000000000000815263ffffffff821660048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690630396cb609034906024015f604051808303818588803b15801561060b575f5ffd5b505af115801561061d573d5f5f3e3d5ffd5b505050505050565b5f546001600160a01b0316331461067e5760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152606401610583565b6040517f205c28780000000000000000000000000000000000000000000000000000000081526001600160a01b038381166004830152602482018390527f0000000000000000000000000000000000000000000000000000000000000000169063205c2878906044015f604051808303815f87803b15801561060b575f5ffd5b5f546001600160a01b031633146107575760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152606401610583565b62124f808263ffffffff1611156107b05760405162461bcd60e51b815260206004820181905260248201527f50502d4552433230203a207072696365206d61726b757020746f6f20686967686044820152606401610583565b620f42408263ffffffff1610156108095760405162461bcd60e51b815260206004820181905260248201527f50502d4552433230203a207072696365206d61726b65757020746f6f206c6f776044820152606401610583565b620f42408163ffffffff1611156108875760405162461bcd60e51b8152602060048201526024808201527f50502d4552433230203a20757064617465207468726573686f6c6420746f6f2060448201527f68696768000000000000000000000000000000000000000000000000000000006064820152608401610583565b6001805477ffffffffffffffffffffffffffffffffffffffffffffffff16780100000000000000000000000000000000000000000000000063ffffffff8581169182027bffffffffffffffffffffffffffffffffffffffffffffffffffffffff16929092177c0100000000000000000000000000000000000000000000000000000000928516928302179092556040805192835260208301919091527ffed7660357162e9e060534e05beba94ac6e3bfb17b1f793bd7350aaed0e9e8c4910160405180910390a15050565b5f61097c7f0000000000000000000000000000000000000000000000000000000000000000610f81565b90505f6109a87f0000000000000000000000000000000000000000000000000000000000000000610f81565b9050816109d57f000000000000000000000000000000000000000000000000000000000000000083611b01565b6109df9190611ba0565b600180547fffffffffffffffff0000000000000000000000000000000000000000000000001677ffffffffffffffffffffffffffffffffffffffffffffffff929092169190911790555050565b5f546001600160a01b03163314610a855760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152606401610583565b610a8e5f61110c565b565b5f546001600160a01b03163314610ae95760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152606401610583565b6040517fa9059cbb0000000000000000000000000000000000000000000000000000000081526001600160a01b038381166004830152602482018390527f0000000000000000000000000000000000000000000000000000000000000000169063a9059cbb906044016020604051808303815f875af1158015610b6e573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610b929190611bef565b505050565b610b9f611173565b610bab848484846111eb565b50505050565b5f546001600160a01b03163314610c0a5760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152606401610583565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663bb9fe6bf6040518163ffffffff1660e01b81526004015f604051808303815f87803b158015610c62575f5ffd5b505af1158015610bab573d5f5f3e3d5ffd5b5f546001600160a01b03163314610ccd5760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152606401610583565b6040517fc23a5cea0000000000000000000000000000000000000000000000000000000081526001600160a01b0382811660048301527f0000000000000000000000000000000000000000000000000000000000000000169063c23a5cea906024015f604051808303815f87803b158015610d46575f5ffd5b505af1158015610d58573d5f5f3e3d5ffd5b5050505050565b6040517f70a082310000000000000000000000000000000000000000000000000000000081523060048201525f907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316906370a0823190602401602060405180830381865afa158015610ddc573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610e009190611c0e565b905090565b6040517fb760faf90000000000000000000000000000000000000000000000000000000081523060048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063b760faf99034906024015f604051808303818588803b158015610d46575f5ffd5b5f546001600160a01b03163314610ed75760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152606401610583565b6001600160a01b038116610f535760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201527f64647265737300000000000000000000000000000000000000000000000000006064820152608401610583565b610f5c8161110c565b50565b60605f610f6a611173565b610f7585858561159b565b91509150935093915050565b5f5f5f5f5f856001600160a01b031663feaf968c6040518163ffffffff1660e01b815260040160a060405180830381865afa158015610fc2573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610fe69190611c3e565b9450945050935093505f831361103e5760405162461bcd60e51b815260206004820152601f60248201527f50502d4552433230203a20436861696e6c696e6b207072696365203c3d2030006044820152606401610583565b61104b6202a30042611c8c565b82101561109a5760405162461bcd60e51b815260206004820152601b60248201527f50502d4552433230203a20496e636f6d706c65746520726f756e6400000000006044820152606401610583565b8369ffffffffffffffffffff168169ffffffffffffffffffff1610156111025760405162461bcd60e51b815260206004820152601660248201527f50502d4552433230203a205374616c65207072696365000000000000000000006044820152606401610583565b5090949350505050565b5f80546001600160a01b038381167fffffffffffffffffffffffff0000000000000000000000000000000000000000831681178455604051919092169283917f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09190a35050565b336001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614610a8e5760405162461bcd60e51b815260206004820152601560248201527f53656e646572206e6f7420456e747279506f696e7400000000000000000000006044820152606401610583565b60028460028111156111ff576111ff611ca5565b14610bab575f61122e7f0000000000000000000000000000000000000000000000000000000000000000610f81565b90505f61125a7f0000000000000000000000000000000000000000000000000000000000000000610f81565b60015490915077ffffffffffffffffffffffffffffffffffffffffffffffff165f836112a67f000000000000000000000000000000000000000000000000000000000000000085611b01565b6112b09190611ba0565b6001549091507c0100000000000000000000000000000000000000000000000000000000900463ffffffff166112e981620f4240611cd2565b83611311620f424077ffffffffffffffffffffffffffffffffffffffffffffffff8616611ce5565b61131b9190611cfc565b1180611363575061132f81620f4240611c8c565b83611357620f424077ffffffffffffffffffffffffffffffffffffffffffffffff8616611ce5565b6113619190611cfc565b105b156113b257600180547fffffffffffffffff0000000000000000000000000000000000000000000000001677ffffffffffffffffffffffffffffffffffffffffffffffff841690811790915592505b5f6113c8620f4240670de0b6b3a7640000611ce5565b60015485907801000000000000000000000000000000000000000000000000900463ffffffff166113fb3a619c40611ce5565b611405908b611cd2565b61140f9190611ce5565b6114199190611ce5565b6114239190611cfc565b90508061143360205f8b8d611d0f565b61143c91611d36565b1115611538576001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001663a9059cbb61147f603460208c8e611d0f565b61148891611d72565b60601c838c8c5f9060209261149f93929190611d0f565b6114a891611d36565b6114b29190611c8c565b6040517fffffffff0000000000000000000000000000000000000000000000000000000060e085901b1681526001600160a01b03909216600483015260248201526044016020604051808303815f875af1158015611512573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906115369190611bef565b505b611546603460208a8c611d0f565b61154f91611d72565b60408051838152602081018a905260609290921c917f472a42a044527b87df02c0ce8e6c00c0057fac40d6c424c93c24b02322eb14b5910160405180910390a250505050505050505050565b6001546060905f9077ffffffffffffffffffffffffffffffffffffffffffffffff1680820361160c5760405162461bcd60e51b815260206004820152601860248201527f50502d4552433230203a207072696365206e6f742073657400000000000000006044820152606401610583565b5f601461161d610120890189611dd8565b611628929150611c8c565b90507effffffffffffffffffffffffffffffffffffffffffffffffffffffffffffdf8116156116995760405162461bcd60e51b815260206004820152601e60248201527f50502d4552433230203a20696e76616c69642064617461206c656e67746800006044820152606401610583565b5f6116af620f4240670de0b6b3a7640000611ce5565b60015484907801000000000000000000000000000000000000000000000000900463ffffffff166116e660e08c0135619c40611ce5565b6116f0908a611cd2565b6116fa9190611ce5565b6117049190611ce5565b61170e9190611cfc565b90508160200361178d57611726610120890189611dd8565b61173591603491601491611d0f565b61173e91611d36565b81111561178d5760405162461bcd60e51b815260206004820181905260248201527f50502d4552433230203a20746f6b656e20616d6f756e7420746f6f20686967686044820152606401610583565b6001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000166323b872dd6117c960208b018b611a10565b6040517fffffffff0000000000000000000000000000000000000000000000000000000060e084901b1681526001600160a01b039091166004820152306024820152604481018490526064016020604051808303815f875af1158015611831573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906118559190611bef565b508061186460208a018a611a10565b6040516020016118a392919091825260601b7fffffffffffffffffffffffffffffffffffffffff00000000000000000000000016602082015260340190565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0818403018152919052985f98509650505050505050565b803563ffffffff811681146118f2575f5ffd5b919050565b5f60208284031215611907575f5ffd5b611910826118df565b9392505050565b6001600160a01b0381168114610f5c575f5ffd5b5f5f6040838503121561193c575f5ffd5b823561194781611917565b946020939093013593505050565b5f5f60408385031215611966575f5ffd5b61196f836118df565b915061197d602084016118df565b90509250929050565b5f5f5f5f60608587031215611999575f5ffd5b8435600381106119a7575f5ffd5b9350602085013567ffffffffffffffff8111156119c2575f5ffd5b8501601f810187136119d2575f5ffd5b803567ffffffffffffffff8111156119e8575f5ffd5b8760208284010111156119f9575f5ffd5b949760209190910196509394604001359392505050565b5f60208284031215611a20575f5ffd5b813561191081611917565b5f5f5f60608486031215611a3d575f5ffd5b833567ffffffffffffffff811115611a53575f5ffd5b84016101608187031215611a65575f5ffd5b95602085013595506040909401359392505050565b604081525f83518060408401528060208601606085015e5f6060828501015260607fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f8301168401019150508260208301529392505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f77ffffffffffffffffffffffffffffffffffffffffffffffff821677ffffffffffffffffffffffffffffffffffffffffffffffff841677ffffffffffffffffffffffffffffffffffffffffffffffff8183021692508183048114821517611b6b57611b6b611ad4565b505092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f77ffffffffffffffffffffffffffffffffffffffffffffffff831680611bc957611bc9611b73565b8077ffffffffffffffffffffffffffffffffffffffffffffffff84160491505092915050565b5f60208284031215611bff575f5ffd5b81518015158114611910575f5ffd5b5f60208284031215611c1e575f5ffd5b5051919050565b805169ffffffffffffffffffff811681146118f2575f5ffd5b5f5f5f5f5f60a08688031215611c52575f5ffd5b611c5b86611c25565b60208701516040880151606089015192975090955093509150611c8060808701611c25565b90509295509295909350565b81810381811115611c9f57611c9f611ad4565b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b80820180821115611c9f57611c9f611ad4565b8082028115828204841417611c9f57611c9f611ad4565b5f82611d0a57611d0a611b73565b500490565b5f5f85851115611d1d575f5ffd5b83861115611d29575f5ffd5b5050820193919092039150565b80356020831015611c9f577fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff602084900360031b1b1692915050565b80357fffffffffffffffffffffffffffffffffffffffff0000000000000000000000008116906014841015611dd1577fffffffffffffffffffffffffffffffffffffffff000000000000000000000000808560140360031b1b82161691505b5092915050565b5f5f83357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe1843603018112611e0b575f5ffd5b83018035915067ffffffffffffffff821115611e25575f5ffd5b602001915036819003821315611e39575f5ffd5b925092905056fea264697066735822122049be2f222c7c7ffbeae7cd1f00ac3e3e27479fce041fcfa22e7b39cace087ce664736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x01 `@R4\x80\x15a\0\x10W__\xFD[P`@Qa$98\x03\x80a$9\x839\x81\x01`@\x81\x90Ra\0/\x91a\x03qV[\x84a\093a\x02/V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\x80R\x86\x81\x16`\xA0R\x84\x81\x16`\xE0R\x83\x16a\x01\0R`\x01\x80T`\x01`\x01`\xC0\x1B\x03\x16e\x03\r@\0\x86G`\xC5\x1B\x17\x90Ua\0|\x82a\x02~V[a\0\x87\x81`\na\x04\xE8V[`\xC0\x81\x81RPP\x83`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\0\xCAW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\0\xEE\x91\x90a\x04\xFDV[`\xFF\x16`\x08\x14a\x01XW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FPP-ERC20 : token oracle decimals`D\x82\x01Ri\x04\r\xAE\xAEn\x84\x0CL\xA4\x07`\xB3\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[\x82`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\x94W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xB8\x91\x90a\x04\xFDV[`\xFF\x16`\x08\x14a\x02$W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`1`$\x82\x01R\x7FPP-ERC20 : native asset oracle d`D\x82\x01Rp\x0C\xACm-\xAC-\x8Ed\r\xAE\xAEn\x84\x0CL\xA4\x07`{\x1B`d\x82\x01R`\x84\x01a\x01OV[PPPPPPa\x05\x16V[_\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x02\xD7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x01OV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x03<W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x01OV[a\x03E\x81a\x02/V[PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x03EW__\xFD[\x80Q`\xFF\x81\x16\x81\x14a\x03lW__\xFD[\x91\x90PV[______`\xC0\x87\x89\x03\x12\x15a\x03\x86W__\xFD[\x86Qa\x03\x91\x81a\x03HV[` \x88\x01Q\x90\x96Pa\x03\xA2\x81a\x03HV[`@\x88\x01Q\x90\x95Pa\x03\xB3\x81a\x03HV[``\x88\x01Q\x90\x94Pa\x03\xC4\x81a\x03HV[`\x80\x88\x01Q\x90\x93Pa\x03\xD5\x81a\x03HV[\x91Pa\x03\xE3`\xA0\x88\x01a\x03\\V[\x90P\x92\x95P\x92\x95P\x92\x95V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[`\x01\x81[`\x01\x84\x11\x15a\x04>W\x80\x85\x04\x81\x11\x15a\x04\"Wa\x04\"a\x03\xEFV[`\x01\x84\x16\x15a\x040W\x90\x81\x02\x90[`\x01\x93\x90\x93\x1C\x92\x80\x02a\x04\x07V[\x93P\x93\x91PPV[_\x82a\x04TWP`\x01a\x04\xE2V[\x81a\x04`WP_a\x04\xE2V[\x81`\x01\x81\x14a\x04vW`\x02\x81\x14a\x04\x80Wa\x04\x9CV[`\x01\x91PPa\x04\xE2V[`\xFF\x84\x11\x15a\x04\x91Wa\x04\x91a\x03\xEFV[PP`\x01\x82\x1Ba\x04\xE2V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a\x04\xBFWP\x81\x81\na\x04\xE2V[a\x04\xCB_\x19\x84\x84a\x04\x03V[\x80_\x19\x04\x82\x11\x15a\x04\xDEWa\x04\xDEa\x03\xEFV[\x02\x90P[\x92\x91PPV[_a\x04\xF6`\xFF\x84\x16\x83a\x04FV[\x93\x92PPPV[_` \x82\x84\x03\x12\x15a\x05\rW__\xFD[a\x04\xF6\x82a\x03\\V[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x01\0Qa\x1Eva\x05\xC3_9_\x81\x81a\x04\x8D\x01R\x81\x81a\t\x84\x01Ra\x126\x01R_\x81\x81a\x02\xCE\x01R\x81\x81a\tX\x01Ra\x12\n\x01R_\x81\x81a\x02\"\x01R\x81\x81a\t\xB0\x01Ra\x12\x81\x01R_\x81\x81a\x05\x0C\x01R\x81\x81a\x0B(\x01R\x81\x81a\x14L\x01Ra\x17\x97\x01R_\x81\x81a\x03\xF5\x01R\x81\x81a\x05\xC0\x01R\x81\x81a\x06\xBD\x01R\x81\x81a\x0C\x0C\x01R\x81\x81a\r\x05\x01R\x81\x81a\r\x8F\x01R\x81\x81a\x0E3\x01Ra\x11~\x01Ra\x1Ev_\xF3\xFE`\x80`@R`\x046\x10a\x01\x83W_5`\xE0\x1C\x80c\x9E(\x1A\x98\x11a\0\xD1W\x80c\xCD\xCFK\x9B\x11a\0|W\x80c\xF2\xFD\xE3\x8B\x11a\0WW\x80c\xF2\xFD\xE3\x8B\x14a\x04\xAFW\x80c\xF4e\xC7~\x14a\x04\xCEW\x80c\xFC\x0CTj\x14a\x04\xFBW__\xFD[\x80c\xCD\xCFK\x9B\x14a\x04^W\x80c\xD0\xE3\r\xB0\x14a\x04tW\x80c\xEF\xB1\xAD]\x14a\x04|W__\xFD[\x80c\xBB\x9F\xE6\xBF\x11a\0\xACW\x80c\xBB\x9F\xE6\xBF\x14a\x04\x17W\x80c\xC2:\\\xEA\x14a\x04+W\x80c\xC3\x99\xEC\x88\x14a\x04JW__\xFD[\x80c\x9E(\x1A\x98\x14a\x03\xA6W\x80c\xA9\xA24\t\x14a\x03\xC5W\x80c\xB0\xD6\x91\xFE\x14a\x03\xE4W__\xFD[\x80cg:~(\x11a\x011W\x80c\x8D\xA5\xCB[\x11a\x01\x0CW\x80c\x8D\xA5\xCB[\x14a\x03\x1CW\x80c\x91N$Z\x14a\x038W\x80c\x9D\xBD\xB9w\x14a\x03\x91W__\xFD[\x80cg:~(\x14a\x02\xA9W\x80cl^\xC2\\\x14a\x02\xBDW\x80cqP\x18\xA6\x14a\x03\x08W__\xFD[\x80c;\x97\xE8V\x11a\x01aW\x80c;\x97\xE8V\x14a\x02\x11W\x80c<!T\xBC\x14a\x02RW\x80c>\x04a\x9D\x14a\x02qW__\xFD[\x80c\x03\x96\xCB`\x14a\x01\x87W\x80c \\(x\x14a\x01\x9CW\x80c:4\xC8?\x14a\x01\xBBW[__\xFD[a\x01\x9Aa\x01\x956`\x04a\x18\xF7V[a\x05.V[\0[4\x80\x15a\x01\xA7W__\xFD[Pa\x01\x9Aa\x01\xB66`\x04a\x19+V[a\x06%V[4\x80\x15a\x01\xC6W__\xFD[P`\x01Ta\x01\xF7\x90|\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\x1CW__\xFD[Pa\x02D\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q\x90\x81R` \x01a\x02\x08V[4\x80\x15a\x02]W__\xFD[Pa\x01\x9Aa\x02l6`\x04a\x19UV[a\x06\xFEV[4\x80\x15a\x02|W__\xFD[P`\x01Ta\x01\xF7\x90x\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[4\x80\x15a\x02\xB4W__\xFD[Pa\x01\x9Aa\tRV[4\x80\x15a\x02\xC8W__\xFD[Pa\x02\xF0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\x08V[4\x80\x15a\x03\x13W__\xFD[Pa\x01\x9Aa\n,V[4\x80\x15a\x03'W__\xFD[P_T`\x01`\x01`\xA0\x1B\x03\x16a\x02\xF0V[4\x80\x15a\x03CW__\xFD[P`\x01Ta\x03h\x90w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qw\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02\x08V[4\x80\x15a\x03\x9CW__\xFD[Pa\x02Da\x9C@\x81V[4\x80\x15a\x03\xB1W__\xFD[Pa\x01\x9Aa\x03\xC06`\x04a\x19+V[a\n\x90V[4\x80\x15a\x03\xD0W__\xFD[Pa\x01\x9Aa\x03\xDF6`\x04a\x19\x86V[a\x0B\x97V[4\x80\x15a\x03\xEFW__\xFD[Pa\x02\xF0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x04\"W__\xFD[Pa\x01\x9Aa\x0B\xB1V[4\x80\x15a\x046W__\xFD[Pa\x01\x9Aa\x04E6`\x04a\x1A\x10V[a\x0CtV[4\x80\x15a\x04UW__\xFD[Pa\x02Da\r_V[4\x80\x15a\x04iW__\xFD[Pa\x02Db\x0FB@\x81V[a\x01\x9Aa\x0E\x05V[4\x80\x15a\x04\x87W__\xFD[Pa\x02\xF0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x04\xBAW__\xFD[Pa\x01\x9Aa\x04\xC96`\x04a\x1A\x10V[a\x0E~V[4\x80\x15a\x04\xD9W__\xFD[Pa\x04\xEDa\x04\xE86`\x04a\x1A+V[a\x0F_V[`@Qa\x02\x08\x92\x91\x90a\x1AzV[4\x80\x15a\x05\x06W__\xFD[Pa\x02\xF0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x05\x8CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`@Q\x7F\x03\x96\xCB`\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rc\xFF\xFF\xFF\xFF\x82\x16`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\x03\x96\xCB`\x904\x90`$\x01_`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\x06\x0BW__\xFD[PZ\xF1\x15\x80\x15a\x06\x1DW=__>=_\xFD[PPPPPPV[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x06~W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x05\x83V[`@Q\x7F \\(x\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R`$\x82\x01\x83\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c \\(x\x90`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x06\x0BW__\xFD[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x07WW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x05\x83V[b\x12O\x80\x82c\xFF\xFF\xFF\xFF\x16\x11\x15a\x07\xB0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FPP-ERC20 : price markup too high`D\x82\x01R`d\x01a\x05\x83V[b\x0FB@\x82c\xFF\xFF\xFF\xFF\x16\x10\x15a\x08\tW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FPP-ERC20 : price markeup too low`D\x82\x01R`d\x01a\x05\x83V[b\x0FB@\x81c\xFF\xFF\xFF\xFF\x16\x11\x15a\x08\x87W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FPP-ERC20 : update threshold too `D\x82\x01R\x7Fhigh\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05\x83V[`\x01\x80Tw\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16x\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0c\xFF\xFF\xFF\xFF\x85\x81\x16\x91\x82\x02{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x92\x90\x92\x17|\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92\x85\x16\x92\x83\x02\x17\x90\x92U`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x7F\xFE\xD7f\x03W\x16.\x9E\x06\x054\xE0[\xEB\xA9J\xC6\xE3\xBF\xB1{\x1Fy;\xD75\n\xAE\xD0\xE9\xE8\xC4\x91\x01`@Q\x80\x91\x03\x90\xA1PPV[_a\t|\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x0F\x81V[\x90P_a\t\xA8\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x0F\x81V[\x90P\x81a\t\xD5\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83a\x1B\x01V[a\t\xDF\x91\x90a\x1B\xA0V[`\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90UPPV[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\n\x85W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x05\x83V[a\n\x8E_a\x11\x0CV[V[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\n\xE9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x05\x83V[`@Q\x7F\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R`$\x82\x01\x83\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x0BnW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\x92\x91\x90a\x1B\xEFV[PPPV[a\x0B\x9Fa\x11sV[a\x0B\xAB\x84\x84\x84\x84a\x11\xEBV[PPPPV[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0C\nW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x05\x83V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xBB\x9F\xE6\xBF`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0CbW__\xFD[PZ\xF1\x15\x80\x15a\x0B\xABW=__>=_\xFD[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0C\xCDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x05\x83V[`@Q\x7F\xC2:\\\xEA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\x04\x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xC2:\\\xEA\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\rFW__\xFD[PZ\xF1\x15\x80\x15a\rXW=__>=_\xFD[PPPPPV[`@Q\x7Fp\xA0\x821\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R0`\x04\x82\x01R_\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\xDCW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\0\x91\x90a\x1C\x0EV[\x90P\x90V[`@Q\x7F\xB7`\xFA\xF9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R0`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xB7`\xFA\xF9\x904\x90`$\x01_`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\rFW__\xFD[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0E\xD7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x05\x83V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x0FSW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05\x83V[a\x0F\\\x81a\x11\x0CV[PV[``_a\x0Fja\x11sV[a\x0Fu\x85\x85\x85a\x15\x9BV[\x91P\x91P\x93P\x93\x91PPV[_____\x85`\x01`\x01`\xA0\x1B\x03\x16c\xFE\xAF\x96\x8C`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\xC2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xE6\x91\x90a\x1C>V[\x94P\x94PP\x93P\x93P_\x83\x13a\x10>W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FPP-ERC20 : Chainlink price <= 0\0`D\x82\x01R`d\x01a\x05\x83V[a\x10Kb\x02\xA3\0Ba\x1C\x8CV[\x82\x10\x15a\x10\x9AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FPP-ERC20 : Incomplete round\0\0\0\0\0`D\x82\x01R`d\x01a\x05\x83V[\x83i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15a\x11\x02W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01R\x7FPP-ERC20 : Stale price\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\x83V[P\x90\x94\x93PPPPV[_\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\n\x8EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01R\x7FSender not EntryPoint\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\x83V[`\x02\x84`\x02\x81\x11\x15a\x11\xFFWa\x11\xFFa\x1C\xA5V[\x14a\x0B\xABW_a\x12.\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x0F\x81V[\x90P_a\x12Z\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x0F\x81V[`\x01T\x90\x91Pw\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x83a\x12\xA6\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85a\x1B\x01V[a\x12\xB0\x91\x90a\x1B\xA0V[`\x01T\x90\x91P|\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04c\xFF\xFF\xFF\xFF\x16a\x12\xE9\x81b\x0FB@a\x1C\xD2V[\x83a\x13\x11b\x0FB@w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16a\x1C\xE5V[a\x13\x1B\x91\x90a\x1C\xFCV[\x11\x80a\x13cWPa\x13/\x81b\x0FB@a\x1C\x8CV[\x83a\x13Wb\x0FB@w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16a\x1C\xE5V[a\x13a\x91\x90a\x1C\xFCV[\x10[\x15a\x13\xB2W`\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x90\x81\x17\x90\x91U\x92P[_a\x13\xC8b\x0FB@g\r\xE0\xB6\xB3\xA7d\0\0a\x1C\xE5V[`\x01T\x85\x90x\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04c\xFF\xFF\xFF\xFF\x16a\x13\xFB:a\x9C@a\x1C\xE5V[a\x14\x05\x90\x8Ba\x1C\xD2V[a\x14\x0F\x91\x90a\x1C\xE5V[a\x14\x19\x91\x90a\x1C\xE5V[a\x14#\x91\x90a\x1C\xFCV[\x90P\x80a\x143` _\x8B\x8Da\x1D\x0FV[a\x14<\x91a\x1D6V[\x11\x15a\x158W`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16c\xA9\x05\x9C\xBBa\x14\x7F`4` \x8C\x8Ea\x1D\x0FV[a\x14\x88\x91a\x1DrV[``\x1C\x83\x8C\x8C_\x90` \x92a\x14\x9F\x93\x92\x91\x90a\x1D\x0FV[a\x14\xA8\x91a\x1D6V[a\x14\xB2\x91\x90a\x1C\x8CV[`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x15\x12W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x156\x91\x90a\x1B\xEFV[P[a\x15F`4` \x8A\x8Ca\x1D\x0FV[a\x15O\x91a\x1DrV[`@\x80Q\x83\x81R` \x81\x01\x8A\x90R``\x92\x90\x92\x1C\x91\x7FG*B\xA0DR{\x87\xDF\x02\xC0\xCE\x8El\0\xC0\x05\x7F\xAC@\xD6\xC4$\xC9<$\xB0#\"\xEB\x14\xB5\x91\x01`@Q\x80\x91\x03\x90\xA2PPPPPPPPPPV[`\x01T``\x90_\x90w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x80\x82\x03a\x16\x0CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FPP-ERC20 : price not set\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\x83V[_`\x14a\x16\x1Da\x01 \x89\x01\x89a\x1D\xD8V[a\x16(\x92\x91Pa\x1C\x8CV[\x90P~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xDF\x81\x16\x15a\x16\x99W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FPP-ERC20 : invalid data length\0\0`D\x82\x01R`d\x01a\x05\x83V[_a\x16\xAFb\x0FB@g\r\xE0\xB6\xB3\xA7d\0\0a\x1C\xE5V[`\x01T\x84\x90x\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04c\xFF\xFF\xFF\xFF\x16a\x16\xE6`\xE0\x8C\x015a\x9C@a\x1C\xE5V[a\x16\xF0\x90\x8Aa\x1C\xD2V[a\x16\xFA\x91\x90a\x1C\xE5V[a\x17\x04\x91\x90a\x1C\xE5V[a\x17\x0E\x91\x90a\x1C\xFCV[\x90P\x81` \x03a\x17\x8DWa\x17&a\x01 \x89\x01\x89a\x1D\xD8V[a\x175\x91`4\x91`\x14\x91a\x1D\x0FV[a\x17>\x91a\x1D6V[\x81\x11\x15a\x17\x8DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FPP-ERC20 : token amount too high`D\x82\x01R`d\x01a\x05\x83V[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16c#\xB8r\xDDa\x17\xC9` \x8B\x01\x8Ba\x1A\x10V[`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x84\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x84\x90R`d\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x181W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18U\x91\x90a\x1B\xEFV[P\x80a\x18d` \x8A\x01\x8Aa\x1A\x10V[`@Q` \x01a\x18\xA3\x92\x91\x90\x91\x82R``\x1B\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\x16` \x82\x01R`4\x01\x90V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R\x98_\x98P\x96PPPPPPPV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x18\xF2W__\xFD[\x91\x90PV[_` \x82\x84\x03\x12\x15a\x19\x07W__\xFD[a\x19\x10\x82a\x18\xDFV[\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0F\\W__\xFD[__`@\x83\x85\x03\x12\x15a\x19<W__\xFD[\x825a\x19G\x81a\x19\x17V[\x94` \x93\x90\x93\x015\x93PPPV[__`@\x83\x85\x03\x12\x15a\x19fW__\xFD[a\x19o\x83a\x18\xDFV[\x91Pa\x19}` \x84\x01a\x18\xDFV[\x90P\x92P\x92\x90PV[____``\x85\x87\x03\x12\x15a\x19\x99W__\xFD[\x845`\x03\x81\x10a\x19\xA7W__\xFD[\x93P` \x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x19\xC2W__\xFD[\x85\x01`\x1F\x81\x01\x87\x13a\x19\xD2W__\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x19\xE8W__\xFD[\x87` \x82\x84\x01\x01\x11\x15a\x19\xF9W__\xFD[\x94\x97` \x91\x90\x91\x01\x96P\x93\x94`@\x015\x93\x92PPPV[_` \x82\x84\x03\x12\x15a\x1A W__\xFD[\x815a\x19\x10\x81a\x19\x17V[___``\x84\x86\x03\x12\x15a\x1A=W__\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1ASW__\xFD[\x84\x01a\x01`\x81\x87\x03\x12\x15a\x1AeW__\xFD[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[`@\x81R_\x83Q\x80`@\x84\x01R\x80` \x86\x01``\x85\x01^_``\x82\x85\x01\x01R``\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x82` \x83\x01R\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x83\x02\x16\x92P\x81\x83\x04\x81\x14\x82\x15\x17a\x1BkWa\x1Bka\x1A\xD4V[PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x80a\x1B\xC9Wa\x1B\xC9a\x1BsV[\x80w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x04\x91PP\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x1B\xFFW__\xFD[\x81Q\x80\x15\x15\x81\x14a\x19\x10W__\xFD[_` \x82\x84\x03\x12\x15a\x1C\x1EW__\xFD[PQ\x91\x90PV[\x80Qi\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x18\xF2W__\xFD[_____`\xA0\x86\x88\x03\x12\x15a\x1CRW__\xFD[a\x1C[\x86a\x1C%V[` \x87\x01Q`@\x88\x01Q``\x89\x01Q\x92\x97P\x90\x95P\x93P\x91Pa\x1C\x80`\x80\x87\x01a\x1C%V[\x90P\x92\x95P\x92\x95\x90\x93PV[\x81\x81\x03\x81\x81\x11\x15a\x1C\x9FWa\x1C\x9Fa\x1A\xD4V[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\x1C\x9FWa\x1C\x9Fa\x1A\xD4V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x1C\x9FWa\x1C\x9Fa\x1A\xD4V[_\x82a\x1D\nWa\x1D\na\x1BsV[P\x04\x90V[__\x85\x85\x11\x15a\x1D\x1DW__\xFD[\x83\x86\x11\x15a\x1D)W__\xFD[PP\x82\x01\x93\x91\x90\x92\x03\x91PV[\x805` \x83\x10\x15a\x1C\x9FW\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x84\x90\x03`\x03\x1B\x1B\x16\x92\x91PPV[\x805\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x90`\x14\x84\x10\x15a\x1D\xD1W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\x80\x85`\x14\x03`\x03\x1B\x1B\x82\x16\x16\x91P[P\x92\x91PPV[__\x835\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x846\x03\x01\x81\x12a\x1E\x0BW__\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x1E%W__\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x1E9W__\xFD[\x92P\x92\x90PV\xFE\xA2dipfsX\"\x12 I\xBE/\",|\x7F\xFB\xEA\xE7\xCD\x1F\0\xAC>>'G\x9F\xCE\x04\x1F\xCF\xA2.{9\xCA\xCE\x08|\xE6dsolcC\0\x08\x1B\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405260043610610183575f3560e01c80639e281a98116100d1578063cdcf4b9b1161007c578063f2fde38b11610057578063f2fde38b146104af578063f465c77e146104ce578063fc0c546a146104fb575f5ffd5b8063cdcf4b9b1461045e578063d0e30db014610474578063efb1ad5d1461047c575f5ffd5b8063bb9fe6bf116100ac578063bb9fe6bf14610417578063c23a5cea1461042b578063c399ec881461044a575f5ffd5b80639e281a98146103a6578063a9a23409146103c5578063b0d691fe146103e4575f5ffd5b8063673a7e28116101315780638da5cb5b1161010c5780638da5cb5b1461031c578063914e245a146103385780639dbdb97714610391575f5ffd5b8063673a7e28146102a95780636c5ec25c146102bd578063715018a614610308575f5ffd5b80633b97e856116101615780633b97e856146102115780633c2154bc146102525780633e04619d14610271575f5ffd5b80630396cb6014610187578063205c28781461019c5780633a34c83f146101bb575b5f5ffd5b61019a6101953660046118f7565b61052e565b005b3480156101a7575f5ffd5b5061019a6101b636600461192b565b610625565b3480156101c6575f5ffd5b506001546101f7907c0100000000000000000000000000000000000000000000000000000000900463ffffffff1681565b60405163ffffffff90911681526020015b60405180910390f35b34801561021c575f5ffd5b506102447f000000000000000000000000000000000000000000000000000000000000000081565b604051908152602001610208565b34801561025d575f5ffd5b5061019a61026c366004611955565b6106fe565b34801561027c575f5ffd5b506001546101f7907801000000000000000000000000000000000000000000000000900463ffffffff1681565b3480156102b4575f5ffd5b5061019a610952565b3480156102c8575f5ffd5b506102f07f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b039091168152602001610208565b348015610313575f5ffd5b5061019a610a2c565b348015610327575f5ffd5b505f546001600160a01b03166102f0565b348015610343575f5ffd5b506001546103689077ffffffffffffffffffffffffffffffffffffffffffffffff1681565b60405177ffffffffffffffffffffffffffffffffffffffffffffffff9091168152602001610208565b34801561039c575f5ffd5b50610244619c4081565b3480156103b1575f5ffd5b5061019a6103c036600461192b565b610a90565b3480156103d0575f5ffd5b5061019a6103df366004611986565b610b97565b3480156103ef575f5ffd5b506102f07f000000000000000000000000000000000000000000000000000000000000000081565b348015610422575f5ffd5b5061019a610bb1565b348015610436575f5ffd5b5061019a610445366004611a10565b610c74565b348015610455575f5ffd5b50610244610d5f565b348015610469575f5ffd5b50610244620f424081565b61019a610e05565b348015610487575f5ffd5b506102f07f000000000000000000000000000000000000000000000000000000000000000081565b3480156104ba575f5ffd5b5061019a6104c9366004611a10565b610e7e565b3480156104d9575f5ffd5b506104ed6104e8366004611a2b565b610f5f565b604051610208929190611a7a565b348015610506575f5ffd5b506102f07f000000000000000000000000000000000000000000000000000000000000000081565b5f546001600160a01b0316331461058c5760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e657260448201526064015b60405180910390fd5b6040517f0396cb6000000000000000000000000000000000000000000000000000000000815263ffffffff821660048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690630396cb609034906024015f604051808303818588803b15801561060b575f5ffd5b505af115801561061d573d5f5f3e3d5ffd5b505050505050565b5f546001600160a01b0316331461067e5760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152606401610583565b6040517f205c28780000000000000000000000000000000000000000000000000000000081526001600160a01b038381166004830152602482018390527f0000000000000000000000000000000000000000000000000000000000000000169063205c2878906044015f604051808303815f87803b15801561060b575f5ffd5b5f546001600160a01b031633146107575760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152606401610583565b62124f808263ffffffff1611156107b05760405162461bcd60e51b815260206004820181905260248201527f50502d4552433230203a207072696365206d61726b757020746f6f20686967686044820152606401610583565b620f42408263ffffffff1610156108095760405162461bcd60e51b815260206004820181905260248201527f50502d4552433230203a207072696365206d61726b65757020746f6f206c6f776044820152606401610583565b620f42408163ffffffff1611156108875760405162461bcd60e51b8152602060048201526024808201527f50502d4552433230203a20757064617465207468726573686f6c6420746f6f2060448201527f68696768000000000000000000000000000000000000000000000000000000006064820152608401610583565b6001805477ffffffffffffffffffffffffffffffffffffffffffffffff16780100000000000000000000000000000000000000000000000063ffffffff8581169182027bffffffffffffffffffffffffffffffffffffffffffffffffffffffff16929092177c0100000000000000000000000000000000000000000000000000000000928516928302179092556040805192835260208301919091527ffed7660357162e9e060534e05beba94ac6e3bfb17b1f793bd7350aaed0e9e8c4910160405180910390a15050565b5f61097c7f0000000000000000000000000000000000000000000000000000000000000000610f81565b90505f6109a87f0000000000000000000000000000000000000000000000000000000000000000610f81565b9050816109d57f000000000000000000000000000000000000000000000000000000000000000083611b01565b6109df9190611ba0565b600180547fffffffffffffffff0000000000000000000000000000000000000000000000001677ffffffffffffffffffffffffffffffffffffffffffffffff929092169190911790555050565b5f546001600160a01b03163314610a855760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152606401610583565b610a8e5f61110c565b565b5f546001600160a01b03163314610ae95760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152606401610583565b6040517fa9059cbb0000000000000000000000000000000000000000000000000000000081526001600160a01b038381166004830152602482018390527f0000000000000000000000000000000000000000000000000000000000000000169063a9059cbb906044016020604051808303815f875af1158015610b6e573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610b929190611bef565b505050565b610b9f611173565b610bab848484846111eb565b50505050565b5f546001600160a01b03163314610c0a5760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152606401610583565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663bb9fe6bf6040518163ffffffff1660e01b81526004015f604051808303815f87803b158015610c62575f5ffd5b505af1158015610bab573d5f5f3e3d5ffd5b5f546001600160a01b03163314610ccd5760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152606401610583565b6040517fc23a5cea0000000000000000000000000000000000000000000000000000000081526001600160a01b0382811660048301527f0000000000000000000000000000000000000000000000000000000000000000169063c23a5cea906024015f604051808303815f87803b158015610d46575f5ffd5b505af1158015610d58573d5f5f3e3d5ffd5b5050505050565b6040517f70a082310000000000000000000000000000000000000000000000000000000081523060048201525f907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316906370a0823190602401602060405180830381865afa158015610ddc573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610e009190611c0e565b905090565b6040517fb760faf90000000000000000000000000000000000000000000000000000000081523060048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063b760faf99034906024015f604051808303818588803b158015610d46575f5ffd5b5f546001600160a01b03163314610ed75760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152606401610583565b6001600160a01b038116610f535760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201527f64647265737300000000000000000000000000000000000000000000000000006064820152608401610583565b610f5c8161110c565b50565b60605f610f6a611173565b610f7585858561159b565b91509150935093915050565b5f5f5f5f5f856001600160a01b031663feaf968c6040518163ffffffff1660e01b815260040160a060405180830381865afa158015610fc2573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610fe69190611c3e565b9450945050935093505f831361103e5760405162461bcd60e51b815260206004820152601f60248201527f50502d4552433230203a20436861696e6c696e6b207072696365203c3d2030006044820152606401610583565b61104b6202a30042611c8c565b82101561109a5760405162461bcd60e51b815260206004820152601b60248201527f50502d4552433230203a20496e636f6d706c65746520726f756e6400000000006044820152606401610583565b8369ffffffffffffffffffff168169ffffffffffffffffffff1610156111025760405162461bcd60e51b815260206004820152601660248201527f50502d4552433230203a205374616c65207072696365000000000000000000006044820152606401610583565b5090949350505050565b5f80546001600160a01b038381167fffffffffffffffffffffffff0000000000000000000000000000000000000000831681178455604051919092169283917f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09190a35050565b336001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614610a8e5760405162461bcd60e51b815260206004820152601560248201527f53656e646572206e6f7420456e747279506f696e7400000000000000000000006044820152606401610583565b60028460028111156111ff576111ff611ca5565b14610bab575f61122e7f0000000000000000000000000000000000000000000000000000000000000000610f81565b90505f61125a7f0000000000000000000000000000000000000000000000000000000000000000610f81565b60015490915077ffffffffffffffffffffffffffffffffffffffffffffffff165f836112a67f000000000000000000000000000000000000000000000000000000000000000085611b01565b6112b09190611ba0565b6001549091507c0100000000000000000000000000000000000000000000000000000000900463ffffffff166112e981620f4240611cd2565b83611311620f424077ffffffffffffffffffffffffffffffffffffffffffffffff8616611ce5565b61131b9190611cfc565b1180611363575061132f81620f4240611c8c565b83611357620f424077ffffffffffffffffffffffffffffffffffffffffffffffff8616611ce5565b6113619190611cfc565b105b156113b257600180547fffffffffffffffff0000000000000000000000000000000000000000000000001677ffffffffffffffffffffffffffffffffffffffffffffffff841690811790915592505b5f6113c8620f4240670de0b6b3a7640000611ce5565b60015485907801000000000000000000000000000000000000000000000000900463ffffffff166113fb3a619c40611ce5565b611405908b611cd2565b61140f9190611ce5565b6114199190611ce5565b6114239190611cfc565b90508061143360205f8b8d611d0f565b61143c91611d36565b1115611538576001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001663a9059cbb61147f603460208c8e611d0f565b61148891611d72565b60601c838c8c5f9060209261149f93929190611d0f565b6114a891611d36565b6114b29190611c8c565b6040517fffffffff0000000000000000000000000000000000000000000000000000000060e085901b1681526001600160a01b03909216600483015260248201526044016020604051808303815f875af1158015611512573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906115369190611bef565b505b611546603460208a8c611d0f565b61154f91611d72565b60408051838152602081018a905260609290921c917f472a42a044527b87df02c0ce8e6c00c0057fac40d6c424c93c24b02322eb14b5910160405180910390a250505050505050505050565b6001546060905f9077ffffffffffffffffffffffffffffffffffffffffffffffff1680820361160c5760405162461bcd60e51b815260206004820152601860248201527f50502d4552433230203a207072696365206e6f742073657400000000000000006044820152606401610583565b5f601461161d610120890189611dd8565b611628929150611c8c565b90507effffffffffffffffffffffffffffffffffffffffffffffffffffffffffffdf8116156116995760405162461bcd60e51b815260206004820152601e60248201527f50502d4552433230203a20696e76616c69642064617461206c656e67746800006044820152606401610583565b5f6116af620f4240670de0b6b3a7640000611ce5565b60015484907801000000000000000000000000000000000000000000000000900463ffffffff166116e660e08c0135619c40611ce5565b6116f0908a611cd2565b6116fa9190611ce5565b6117049190611ce5565b61170e9190611cfc565b90508160200361178d57611726610120890189611dd8565b61173591603491601491611d0f565b61173e91611d36565b81111561178d5760405162461bcd60e51b815260206004820181905260248201527f50502d4552433230203a20746f6b656e20616d6f756e7420746f6f20686967686044820152606401610583565b6001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000166323b872dd6117c960208b018b611a10565b6040517fffffffff0000000000000000000000000000000000000000000000000000000060e084901b1681526001600160a01b039091166004820152306024820152604481018490526064016020604051808303815f875af1158015611831573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906118559190611bef565b508061186460208a018a611a10565b6040516020016118a392919091825260601b7fffffffffffffffffffffffffffffffffffffffff00000000000000000000000016602082015260340190565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0818403018152919052985f98509650505050505050565b803563ffffffff811681146118f2575f5ffd5b919050565b5f60208284031215611907575f5ffd5b611910826118df565b9392505050565b6001600160a01b0381168114610f5c575f5ffd5b5f5f6040838503121561193c575f5ffd5b823561194781611917565b946020939093013593505050565b5f5f60408385031215611966575f5ffd5b61196f836118df565b915061197d602084016118df565b90509250929050565b5f5f5f5f60608587031215611999575f5ffd5b8435600381106119a7575f5ffd5b9350602085013567ffffffffffffffff8111156119c2575f5ffd5b8501601f810187136119d2575f5ffd5b803567ffffffffffffffff8111156119e8575f5ffd5b8760208284010111156119f9575f5ffd5b949760209190910196509394604001359392505050565b5f60208284031215611a20575f5ffd5b813561191081611917565b5f5f5f60608486031215611a3d575f5ffd5b833567ffffffffffffffff811115611a53575f5ffd5b84016101608187031215611a65575f5ffd5b95602085013595506040909401359392505050565b604081525f83518060408401528060208601606085015e5f6060828501015260607fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f8301168401019150508260208301529392505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f77ffffffffffffffffffffffffffffffffffffffffffffffff821677ffffffffffffffffffffffffffffffffffffffffffffffff841677ffffffffffffffffffffffffffffffffffffffffffffffff8183021692508183048114821517611b6b57611b6b611ad4565b505092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f77ffffffffffffffffffffffffffffffffffffffffffffffff831680611bc957611bc9611b73565b8077ffffffffffffffffffffffffffffffffffffffffffffffff84160491505092915050565b5f60208284031215611bff575f5ffd5b81518015158114611910575f5ffd5b5f60208284031215611c1e575f5ffd5b5051919050565b805169ffffffffffffffffffff811681146118f2575f5ffd5b5f5f5f5f5f60a08688031215611c52575f5ffd5b611c5b86611c25565b60208701516040880151606089015192975090955093509150611c8060808701611c25565b90509295509295909350565b81810381811115611c9f57611c9f611ad4565b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b80820180821115611c9f57611c9f611ad4565b8082028115828204841417611c9f57611c9f611ad4565b5f82611d0a57611d0a611b73565b500490565b5f5f85851115611d1d575f5ffd5b83861115611d29575f5ffd5b5050820193919092039150565b80356020831015611c9f577fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff602084900360031b1b1692915050565b80357fffffffffffffffffffffffffffffffffffffffff0000000000000000000000008116906014841015611dd1577fffffffffffffffffffffffffffffffffffffffff000000000000000000000000808560140360031b1b82161691505b5092915050565b5f5f83357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe1843603018112611e0b575f5ffd5b83018035915067ffffffffffffffff821115611e25575f5ffd5b602001915036819003821315611e39575f5ffd5b925092905056fea264697066735822122049be2f222c7c7ffbeae7cd1f00ac3e3e27479fce041fcfa22e7b39cace087ce664736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10a\x01\x83W_5`\xE0\x1C\x80c\x9E(\x1A\x98\x11a\0\xD1W\x80c\xCD\xCFK\x9B\x11a\0|W\x80c\xF2\xFD\xE3\x8B\x11a\0WW\x80c\xF2\xFD\xE3\x8B\x14a\x04\xAFW\x80c\xF4e\xC7~\x14a\x04\xCEW\x80c\xFC\x0CTj\x14a\x04\xFBW__\xFD[\x80c\xCD\xCFK\x9B\x14a\x04^W\x80c\xD0\xE3\r\xB0\x14a\x04tW\x80c\xEF\xB1\xAD]\x14a\x04|W__\xFD[\x80c\xBB\x9F\xE6\xBF\x11a\0\xACW\x80c\xBB\x9F\xE6\xBF\x14a\x04\x17W\x80c\xC2:\\\xEA\x14a\x04+W\x80c\xC3\x99\xEC\x88\x14a\x04JW__\xFD[\x80c\x9E(\x1A\x98\x14a\x03\xA6W\x80c\xA9\xA24\t\x14a\x03\xC5W\x80c\xB0\xD6\x91\xFE\x14a\x03\xE4W__\xFD[\x80cg:~(\x11a\x011W\x80c\x8D\xA5\xCB[\x11a\x01\x0CW\x80c\x8D\xA5\xCB[\x14a\x03\x1CW\x80c\x91N$Z\x14a\x038W\x80c\x9D\xBD\xB9w\x14a\x03\x91W__\xFD[\x80cg:~(\x14a\x02\xA9W\x80cl^\xC2\\\x14a\x02\xBDW\x80cqP\x18\xA6\x14a\x03\x08W__\xFD[\x80c;\x97\xE8V\x11a\x01aW\x80c;\x97\xE8V\x14a\x02\x11W\x80c<!T\xBC\x14a\x02RW\x80c>\x04a\x9D\x14a\x02qW__\xFD[\x80c\x03\x96\xCB`\x14a\x01\x87W\x80c \\(x\x14a\x01\x9CW\x80c:4\xC8?\x14a\x01\xBBW[__\xFD[a\x01\x9Aa\x01\x956`\x04a\x18\xF7V[a\x05.V[\0[4\x80\x15a\x01\xA7W__\xFD[Pa\x01\x9Aa\x01\xB66`\x04a\x19+V[a\x06%V[4\x80\x15a\x01\xC6W__\xFD[P`\x01Ta\x01\xF7\x90|\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\x1CW__\xFD[Pa\x02D\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q\x90\x81R` \x01a\x02\x08V[4\x80\x15a\x02]W__\xFD[Pa\x01\x9Aa\x02l6`\x04a\x19UV[a\x06\xFEV[4\x80\x15a\x02|W__\xFD[P`\x01Ta\x01\xF7\x90x\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[4\x80\x15a\x02\xB4W__\xFD[Pa\x01\x9Aa\tRV[4\x80\x15a\x02\xC8W__\xFD[Pa\x02\xF0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\x08V[4\x80\x15a\x03\x13W__\xFD[Pa\x01\x9Aa\n,V[4\x80\x15a\x03'W__\xFD[P_T`\x01`\x01`\xA0\x1B\x03\x16a\x02\xF0V[4\x80\x15a\x03CW__\xFD[P`\x01Ta\x03h\x90w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qw\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02\x08V[4\x80\x15a\x03\x9CW__\xFD[Pa\x02Da\x9C@\x81V[4\x80\x15a\x03\xB1W__\xFD[Pa\x01\x9Aa\x03\xC06`\x04a\x19+V[a\n\x90V[4\x80\x15a\x03\xD0W__\xFD[Pa\x01\x9Aa\x03\xDF6`\x04a\x19\x86V[a\x0B\x97V[4\x80\x15a\x03\xEFW__\xFD[Pa\x02\xF0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x04\"W__\xFD[Pa\x01\x9Aa\x0B\xB1V[4\x80\x15a\x046W__\xFD[Pa\x01\x9Aa\x04E6`\x04a\x1A\x10V[a\x0CtV[4\x80\x15a\x04UW__\xFD[Pa\x02Da\r_V[4\x80\x15a\x04iW__\xFD[Pa\x02Db\x0FB@\x81V[a\x01\x9Aa\x0E\x05V[4\x80\x15a\x04\x87W__\xFD[Pa\x02\xF0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x04\xBAW__\xFD[Pa\x01\x9Aa\x04\xC96`\x04a\x1A\x10V[a\x0E~V[4\x80\x15a\x04\xD9W__\xFD[Pa\x04\xEDa\x04\xE86`\x04a\x1A+V[a\x0F_V[`@Qa\x02\x08\x92\x91\x90a\x1AzV[4\x80\x15a\x05\x06W__\xFD[Pa\x02\xF0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x05\x8CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`@Q\x7F\x03\x96\xCB`\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rc\xFF\xFF\xFF\xFF\x82\x16`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\x03\x96\xCB`\x904\x90`$\x01_`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\x06\x0BW__\xFD[PZ\xF1\x15\x80\x15a\x06\x1DW=__>=_\xFD[PPPPPPV[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x06~W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x05\x83V[`@Q\x7F \\(x\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R`$\x82\x01\x83\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c \\(x\x90`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x06\x0BW__\xFD[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x07WW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x05\x83V[b\x12O\x80\x82c\xFF\xFF\xFF\xFF\x16\x11\x15a\x07\xB0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FPP-ERC20 : price markup too high`D\x82\x01R`d\x01a\x05\x83V[b\x0FB@\x82c\xFF\xFF\xFF\xFF\x16\x10\x15a\x08\tW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FPP-ERC20 : price markeup too low`D\x82\x01R`d\x01a\x05\x83V[b\x0FB@\x81c\xFF\xFF\xFF\xFF\x16\x11\x15a\x08\x87W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FPP-ERC20 : update threshold too `D\x82\x01R\x7Fhigh\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05\x83V[`\x01\x80Tw\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16x\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0c\xFF\xFF\xFF\xFF\x85\x81\x16\x91\x82\x02{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x92\x90\x92\x17|\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92\x85\x16\x92\x83\x02\x17\x90\x92U`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x7F\xFE\xD7f\x03W\x16.\x9E\x06\x054\xE0[\xEB\xA9J\xC6\xE3\xBF\xB1{\x1Fy;\xD75\n\xAE\xD0\xE9\xE8\xC4\x91\x01`@Q\x80\x91\x03\x90\xA1PPV[_a\t|\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x0F\x81V[\x90P_a\t\xA8\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x0F\x81V[\x90P\x81a\t\xD5\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83a\x1B\x01V[a\t\xDF\x91\x90a\x1B\xA0V[`\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90UPPV[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\n\x85W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x05\x83V[a\n\x8E_a\x11\x0CV[V[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\n\xE9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x05\x83V[`@Q\x7F\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R`$\x82\x01\x83\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x0BnW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\x92\x91\x90a\x1B\xEFV[PPPV[a\x0B\x9Fa\x11sV[a\x0B\xAB\x84\x84\x84\x84a\x11\xEBV[PPPPV[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0C\nW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x05\x83V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xBB\x9F\xE6\xBF`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0CbW__\xFD[PZ\xF1\x15\x80\x15a\x0B\xABW=__>=_\xFD[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0C\xCDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x05\x83V[`@Q\x7F\xC2:\\\xEA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\x04\x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xC2:\\\xEA\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\rFW__\xFD[PZ\xF1\x15\x80\x15a\rXW=__>=_\xFD[PPPPPV[`@Q\x7Fp\xA0\x821\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R0`\x04\x82\x01R_\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\xDCW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\0\x91\x90a\x1C\x0EV[\x90P\x90V[`@Q\x7F\xB7`\xFA\xF9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R0`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xB7`\xFA\xF9\x904\x90`$\x01_`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\rFW__\xFD[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0E\xD7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x05\x83V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x0FSW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05\x83V[a\x0F\\\x81a\x11\x0CV[PV[``_a\x0Fja\x11sV[a\x0Fu\x85\x85\x85a\x15\x9BV[\x91P\x91P\x93P\x93\x91PPV[_____\x85`\x01`\x01`\xA0\x1B\x03\x16c\xFE\xAF\x96\x8C`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\xC2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xE6\x91\x90a\x1C>V[\x94P\x94PP\x93P\x93P_\x83\x13a\x10>W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FPP-ERC20 : Chainlink price <= 0\0`D\x82\x01R`d\x01a\x05\x83V[a\x10Kb\x02\xA3\0Ba\x1C\x8CV[\x82\x10\x15a\x10\x9AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FPP-ERC20 : Incomplete round\0\0\0\0\0`D\x82\x01R`d\x01a\x05\x83V[\x83i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15a\x11\x02W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01R\x7FPP-ERC20 : Stale price\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\x83V[P\x90\x94\x93PPPPV[_\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\n\x8EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01R\x7FSender not EntryPoint\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\x83V[`\x02\x84`\x02\x81\x11\x15a\x11\xFFWa\x11\xFFa\x1C\xA5V[\x14a\x0B\xABW_a\x12.\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x0F\x81V[\x90P_a\x12Z\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x0F\x81V[`\x01T\x90\x91Pw\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x83a\x12\xA6\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85a\x1B\x01V[a\x12\xB0\x91\x90a\x1B\xA0V[`\x01T\x90\x91P|\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04c\xFF\xFF\xFF\xFF\x16a\x12\xE9\x81b\x0FB@a\x1C\xD2V[\x83a\x13\x11b\x0FB@w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16a\x1C\xE5V[a\x13\x1B\x91\x90a\x1C\xFCV[\x11\x80a\x13cWPa\x13/\x81b\x0FB@a\x1C\x8CV[\x83a\x13Wb\x0FB@w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16a\x1C\xE5V[a\x13a\x91\x90a\x1C\xFCV[\x10[\x15a\x13\xB2W`\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x90\x81\x17\x90\x91U\x92P[_a\x13\xC8b\x0FB@g\r\xE0\xB6\xB3\xA7d\0\0a\x1C\xE5V[`\x01T\x85\x90x\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04c\xFF\xFF\xFF\xFF\x16a\x13\xFB:a\x9C@a\x1C\xE5V[a\x14\x05\x90\x8Ba\x1C\xD2V[a\x14\x0F\x91\x90a\x1C\xE5V[a\x14\x19\x91\x90a\x1C\xE5V[a\x14#\x91\x90a\x1C\xFCV[\x90P\x80a\x143` _\x8B\x8Da\x1D\x0FV[a\x14<\x91a\x1D6V[\x11\x15a\x158W`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16c\xA9\x05\x9C\xBBa\x14\x7F`4` \x8C\x8Ea\x1D\x0FV[a\x14\x88\x91a\x1DrV[``\x1C\x83\x8C\x8C_\x90` \x92a\x14\x9F\x93\x92\x91\x90a\x1D\x0FV[a\x14\xA8\x91a\x1D6V[a\x14\xB2\x91\x90a\x1C\x8CV[`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x15\x12W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x156\x91\x90a\x1B\xEFV[P[a\x15F`4` \x8A\x8Ca\x1D\x0FV[a\x15O\x91a\x1DrV[`@\x80Q\x83\x81R` \x81\x01\x8A\x90R``\x92\x90\x92\x1C\x91\x7FG*B\xA0DR{\x87\xDF\x02\xC0\xCE\x8El\0\xC0\x05\x7F\xAC@\xD6\xC4$\xC9<$\xB0#\"\xEB\x14\xB5\x91\x01`@Q\x80\x91\x03\x90\xA2PPPPPPPPPPV[`\x01T``\x90_\x90w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x80\x82\x03a\x16\x0CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FPP-ERC20 : price not set\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\x83V[_`\x14a\x16\x1Da\x01 \x89\x01\x89a\x1D\xD8V[a\x16(\x92\x91Pa\x1C\x8CV[\x90P~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xDF\x81\x16\x15a\x16\x99W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FPP-ERC20 : invalid data length\0\0`D\x82\x01R`d\x01a\x05\x83V[_a\x16\xAFb\x0FB@g\r\xE0\xB6\xB3\xA7d\0\0a\x1C\xE5V[`\x01T\x84\x90x\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04c\xFF\xFF\xFF\xFF\x16a\x16\xE6`\xE0\x8C\x015a\x9C@a\x1C\xE5V[a\x16\xF0\x90\x8Aa\x1C\xD2V[a\x16\xFA\x91\x90a\x1C\xE5V[a\x17\x04\x91\x90a\x1C\xE5V[a\x17\x0E\x91\x90a\x1C\xFCV[\x90P\x81` \x03a\x17\x8DWa\x17&a\x01 \x89\x01\x89a\x1D\xD8V[a\x175\x91`4\x91`\x14\x91a\x1D\x0FV[a\x17>\x91a\x1D6V[\x81\x11\x15a\x17\x8DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FPP-ERC20 : token amount too high`D\x82\x01R`d\x01a\x05\x83V[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16c#\xB8r\xDDa\x17\xC9` \x8B\x01\x8Ba\x1A\x10V[`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x84\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x84\x90R`d\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x181W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18U\x91\x90a\x1B\xEFV[P\x80a\x18d` \x8A\x01\x8Aa\x1A\x10V[`@Q` \x01a\x18\xA3\x92\x91\x90\x91\x82R``\x1B\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\x16` \x82\x01R`4\x01\x90V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R\x98_\x98P\x96PPPPPPPV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x18\xF2W__\xFD[\x91\x90PV[_` \x82\x84\x03\x12\x15a\x19\x07W__\xFD[a\x19\x10\x82a\x18\xDFV[\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0F\\W__\xFD[__`@\x83\x85\x03\x12\x15a\x19<W__\xFD[\x825a\x19G\x81a\x19\x17V[\x94` \x93\x90\x93\x015\x93PPPV[__`@\x83\x85\x03\x12\x15a\x19fW__\xFD[a\x19o\x83a\x18\xDFV[\x91Pa\x19}` \x84\x01a\x18\xDFV[\x90P\x92P\x92\x90PV[____``\x85\x87\x03\x12\x15a\x19\x99W__\xFD[\x845`\x03\x81\x10a\x19\xA7W__\xFD[\x93P` \x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x19\xC2W__\xFD[\x85\x01`\x1F\x81\x01\x87\x13a\x19\xD2W__\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x19\xE8W__\xFD[\x87` \x82\x84\x01\x01\x11\x15a\x19\xF9W__\xFD[\x94\x97` \x91\x90\x91\x01\x96P\x93\x94`@\x015\x93\x92PPPV[_` \x82\x84\x03\x12\x15a\x1A W__\xFD[\x815a\x19\x10\x81a\x19\x17V[___``\x84\x86\x03\x12\x15a\x1A=W__\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1ASW__\xFD[\x84\x01a\x01`\x81\x87\x03\x12\x15a\x1AeW__\xFD[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[`@\x81R_\x83Q\x80`@\x84\x01R\x80` \x86\x01``\x85\x01^_``\x82\x85\x01\x01R``\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x82` \x83\x01R\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x83\x02\x16\x92P\x81\x83\x04\x81\x14\x82\x15\x17a\x1BkWa\x1Bka\x1A\xD4V[PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x80a\x1B\xC9Wa\x1B\xC9a\x1BsV[\x80w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x04\x91PP\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x1B\xFFW__\xFD[\x81Q\x80\x15\x15\x81\x14a\x19\x10W__\xFD[_` \x82\x84\x03\x12\x15a\x1C\x1EW__\xFD[PQ\x91\x90PV[\x80Qi\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x18\xF2W__\xFD[_____`\xA0\x86\x88\x03\x12\x15a\x1CRW__\xFD[a\x1C[\x86a\x1C%V[` \x87\x01Q`@\x88\x01Q``\x89\x01Q\x92\x97P\x90\x95P\x93P\x91Pa\x1C\x80`\x80\x87\x01a\x1C%V[\x90P\x92\x95P\x92\x95\x90\x93PV[\x81\x81\x03\x81\x81\x11\x15a\x1C\x9FWa\x1C\x9Fa\x1A\xD4V[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\x1C\x9FWa\x1C\x9Fa\x1A\xD4V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x1C\x9FWa\x1C\x9Fa\x1A\xD4V[_\x82a\x1D\nWa\x1D\na\x1BsV[P\x04\x90V[__\x85\x85\x11\x15a\x1D\x1DW__\xFD[\x83\x86\x11\x15a\x1D)W__\xFD[PP\x82\x01\x93\x91\x90\x92\x03\x91PV[\x805` \x83\x10\x15a\x1C\x9FW\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x84\x90\x03`\x03\x1B\x1B\x16\x92\x91PPV[\x805\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x90`\x14\x84\x10\x15a\x1D\xD1W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\x80\x85`\x14\x03`\x03\x1B\x1B\x82\x16\x16\x91P[P\x92\x91PPV[__\x835\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x846\x03\x01\x81\x12a\x1E\x0BW__\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x1E%W__\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x1E9W__\xFD[\x92P\x92\x90PV\xFE\xA2dipfsX\"\x12 I\xBE/\",|\x7F\xFB\xEA\xE7\xCD\x1F\0\xAC>>'G\x9F\xCE\x04\x1F\xCF\xA2.{9\xCA\xCE\x08|\xE6dsolcC\0\x08\x1B\x003",
    );
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
    /**Event with signature `ConfigUpdated(uint32,uint32)` and selector `0xfed7660357162e9e060534e05beba94ac6e3bfb17b1f793bd7350aaed0e9e8c4`.
```solidity
event ConfigUpdated(uint32 priceMarkup, uint32 updateThreshold);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ConfigUpdated {
        #[allow(missing_docs)]
        pub priceMarkup: u32,
        #[allow(missing_docs)]
        pub updateThreshold: u32,
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
        impl alloy_sol_types::SolEvent for ConfigUpdated {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "ConfigUpdated(uint32,uint32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                254u8,
                215u8,
                102u8,
                3u8,
                87u8,
                22u8,
                46u8,
                158u8,
                6u8,
                5u8,
                52u8,
                224u8,
                91u8,
                235u8,
                169u8,
                74u8,
                198u8,
                227u8,
                191u8,
                177u8,
                123u8,
                31u8,
                121u8,
                59u8,
                215u8,
                53u8,
                10u8,
                174u8,
                208u8,
                233u8,
                232u8,
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
                    priceMarkup: data.0,
                    updateThreshold: data.1,
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
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.priceMarkup),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.updateThreshold),
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
        impl alloy_sol_types::private::IntoLogData for ConfigUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ConfigUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ConfigUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
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
    /**Event with signature `UserOperationSponsored(address,uint256,uint256)` and selector `0x472a42a044527b87df02c0ce8e6c00c0057fac40d6c424c93c24b02322eb14b5`.
```solidity
event UserOperationSponsored(address indexed user, uint256 actualTokenNeeded, uint256 actualGasCost);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct UserOperationSponsored {
        #[allow(missing_docs)]
        pub user: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub actualTokenNeeded: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for UserOperationSponsored {
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
            const SIGNATURE: &'static str = "UserOperationSponsored(address,uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                71u8,
                42u8,
                66u8,
                160u8,
                68u8,
                82u8,
                123u8,
                135u8,
                223u8,
                2u8,
                192u8,
                206u8,
                142u8,
                108u8,
                0u8,
                192u8,
                5u8,
                127u8,
                172u8,
                64u8,
                214u8,
                196u8,
                36u8,
                201u8,
                60u8,
                36u8,
                176u8,
                35u8,
                34u8,
                235u8,
                20u8,
                181u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    user: topics.1,
                    actualTokenNeeded: data.0,
                    actualGasCost: data.1,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.actualTokenNeeded),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.actualGasCost),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.user.clone())
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
                    &self.user,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for UserOperationSponsored {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&UserOperationSponsored> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &UserOperationSponsored) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Constructor`.
```solidity
constructor(address _token, address _entryPoint, address _tokenOracle, address _nativeAssetOracle, address _owner, uint8 _tokenDecimals);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        pub _token: alloy::sol_types::private::Address,
        pub _entryPoint: alloy::sol_types::private::Address,
        pub _tokenOracle: alloy::sol_types::private::Address,
        pub _nativeAssetOracle: alloy::sol_types::private::Address,
        pub _owner: alloy::sol_types::private::Address,
        pub _tokenDecimals: u8,
    }
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<8>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                u8,
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
                    (
                        value._token,
                        value._entryPoint,
                        value._tokenOracle,
                        value._nativeAssetOracle,
                        value._owner,
                        value._tokenDecimals,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _token: tuple.0,
                        _entryPoint: tuple.1,
                        _tokenOracle: tuple.2,
                        _nativeAssetOracle: tuple.3,
                        _owner: tuple.4,
                        _tokenDecimals: tuple.5,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<8>,
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
                        &self._token,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._entryPoint,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._tokenOracle,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._nativeAssetOracle,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._owner,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self._tokenDecimals),
                )
            }
        }
    };
    /**Function with signature `REFUND_POSTOP_COST()` and selector `0x9dbdb977`.
```solidity
function REFUND_POSTOP_COST() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct REFUND_POSTOP_COSTCall {}
    ///Container type for the return parameters of the [`REFUND_POSTOP_COST()`](REFUND_POSTOP_COSTCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct REFUND_POSTOP_COSTReturn {
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
            impl ::core::convert::From<REFUND_POSTOP_COSTCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: REFUND_POSTOP_COSTCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for REFUND_POSTOP_COSTCall {
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
            impl ::core::convert::From<REFUND_POSTOP_COSTReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: REFUND_POSTOP_COSTReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for REFUND_POSTOP_COSTReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for REFUND_POSTOP_COSTCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = REFUND_POSTOP_COSTReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "REFUND_POSTOP_COST()";
            const SELECTOR: [u8; 4] = [157u8, 189u8, 185u8, 119u8];
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
    /**Function with signature `deposit()` and selector `0xd0e30db0`.
```solidity
function deposit() external payable;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct depositCall {}
    ///Container type for the return parameters of the [`deposit()`](depositCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct depositReturn {}
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
            impl ::core::convert::From<depositCall> for UnderlyingRustTuple<'_> {
                fn from(value: depositCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for depositCall {
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
            impl ::core::convert::From<depositReturn> for UnderlyingRustTuple<'_> {
                fn from(value: depositReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for depositReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for depositCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = depositReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "deposit()";
            const SELECTOR: [u8; 4] = [208u8, 227u8, 13u8, 176u8];
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
    /**Function with signature `entryPoint()` and selector `0xb0d691fe`.
```solidity
function entryPoint() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct entryPointCall {}
    ///Container type for the return parameters of the [`entryPoint()`](entryPointCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct entryPointReturn {
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
            impl ::core::convert::From<entryPointCall> for UnderlyingRustTuple<'_> {
                fn from(value: entryPointCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for entryPointCall {
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
            impl ::core::convert::From<entryPointReturn> for UnderlyingRustTuple<'_> {
                fn from(value: entryPointReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for entryPointReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for entryPointCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = entryPointReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "entryPoint()";
            const SELECTOR: [u8; 4] = [176u8, 214u8, 145u8, 254u8];
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
    /**Function with signature `getDeposit()` and selector `0xc399ec88`.
```solidity
function getDeposit() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getDepositCall {}
    ///Container type for the return parameters of the [`getDeposit()`](getDepositCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getDepositReturn {
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
            impl ::core::convert::From<getDepositCall> for UnderlyingRustTuple<'_> {
                fn from(value: getDepositCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getDepositCall {
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
            impl ::core::convert::From<getDepositReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getDepositReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getDepositReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getDepositCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getDepositReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getDeposit()";
            const SELECTOR: [u8; 4] = [195u8, 153u8, 236u8, 136u8];
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
    /**Function with signature `nativeAssetOracle()` and selector `0xefb1ad5d`.
```solidity
function nativeAssetOracle() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct nativeAssetOracleCall {}
    ///Container type for the return parameters of the [`nativeAssetOracle()`](nativeAssetOracleCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct nativeAssetOracleReturn {
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
            impl ::core::convert::From<nativeAssetOracleCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: nativeAssetOracleCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for nativeAssetOracleCall {
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
            impl ::core::convert::From<nativeAssetOracleReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: nativeAssetOracleReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for nativeAssetOracleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for nativeAssetOracleCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = nativeAssetOracleReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "nativeAssetOracle()";
            const SELECTOR: [u8; 4] = [239u8, 177u8, 173u8, 93u8];
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
    /**Function with signature `postOp(uint8,bytes,uint256)` and selector `0xa9a23409`.
```solidity
function postOp(IPaymaster.PostOpMode mode, bytes memory context, uint256 actualGasCost) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct postOpCall {
        pub mode: <IPaymaster::PostOpMode as alloy::sol_types::SolType>::RustType,
        pub context: alloy::sol_types::private::Bytes,
        pub actualGasCost: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`postOp(uint8,bytes,uint256)`](postOpCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct postOpReturn {}
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
                IPaymaster::PostOpMode,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IPaymaster::PostOpMode as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<postOpCall> for UnderlyingRustTuple<'_> {
                fn from(value: postOpCall) -> Self {
                    (value.mode, value.context, value.actualGasCost)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for postOpCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        mode: tuple.0,
                        context: tuple.1,
                        actualGasCost: tuple.2,
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
            impl ::core::convert::From<postOpReturn> for UnderlyingRustTuple<'_> {
                fn from(value: postOpReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for postOpReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for postOpCall {
            type Parameters<'a> = (
                IPaymaster::PostOpMode,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = postOpReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "postOp(uint8,bytes,uint256)";
            const SELECTOR: [u8; 4] = [169u8, 162u8, 52u8, 9u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <IPaymaster::PostOpMode as alloy_sol_types::SolType>::tokenize(
                        &self.mode,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.context,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.actualGasCost),
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
    /**Function with signature `previousPrice()` and selector `0x914e245a`.
```solidity
function previousPrice() external view returns (uint192);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct previousPriceCall {}
    ///Container type for the return parameters of the [`previousPrice()`](previousPriceCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct previousPriceReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U192,
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
            impl ::core::convert::From<previousPriceCall> for UnderlyingRustTuple<'_> {
                fn from(value: previousPriceCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for previousPriceCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
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
            impl ::core::convert::From<previousPriceReturn> for UnderlyingRustTuple<'_> {
                fn from(value: previousPriceReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for previousPriceReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for previousPriceCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = previousPriceReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<192>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "previousPrice()";
            const SELECTOR: [u8; 4] = [145u8, 78u8, 36u8, 90u8];
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
    /**Function with signature `priceDenominator()` and selector `0xcdcf4b9b`.
```solidity
function priceDenominator() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct priceDenominatorCall {}
    ///Container type for the return parameters of the [`priceDenominator()`](priceDenominatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct priceDenominatorReturn {
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
            impl ::core::convert::From<priceDenominatorCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: priceDenominatorCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for priceDenominatorCall {
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
            impl ::core::convert::From<priceDenominatorReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: priceDenominatorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for priceDenominatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for priceDenominatorCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = priceDenominatorReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "priceDenominator()";
            const SELECTOR: [u8; 4] = [205u8, 207u8, 75u8, 155u8];
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
    /**Function with signature `priceMarkup()` and selector `0x3e04619d`.
```solidity
function priceMarkup() external view returns (uint32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct priceMarkupCall {}
    ///Container type for the return parameters of the [`priceMarkup()`](priceMarkupCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct priceMarkupReturn {
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
            impl ::core::convert::From<priceMarkupCall> for UnderlyingRustTuple<'_> {
                fn from(value: priceMarkupCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for priceMarkupCall {
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
            impl ::core::convert::From<priceMarkupReturn> for UnderlyingRustTuple<'_> {
                fn from(value: priceMarkupReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for priceMarkupReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for priceMarkupCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = priceMarkupReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "priceMarkup()";
            const SELECTOR: [u8; 4] = [62u8, 4u8, 97u8, 157u8];
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
    /**Function with signature `priceUpdateThreshold()` and selector `0x3a34c83f`.
```solidity
function priceUpdateThreshold() external view returns (uint32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct priceUpdateThresholdCall {}
    ///Container type for the return parameters of the [`priceUpdateThreshold()`](priceUpdateThresholdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct priceUpdateThresholdReturn {
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
            impl ::core::convert::From<priceUpdateThresholdCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: priceUpdateThresholdCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for priceUpdateThresholdCall {
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
            impl ::core::convert::From<priceUpdateThresholdReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: priceUpdateThresholdReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for priceUpdateThresholdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for priceUpdateThresholdCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = priceUpdateThresholdReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "priceUpdateThreshold()";
            const SELECTOR: [u8; 4] = [58u8, 52u8, 200u8, 63u8];
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
    /**Function with signature `token()` and selector `0xfc0c546a`.
```solidity
function token() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct tokenCall {}
    ///Container type for the return parameters of the [`token()`](tokenCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct tokenReturn {
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
            impl ::core::convert::From<tokenCall> for UnderlyingRustTuple<'_> {
                fn from(value: tokenCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for tokenCall {
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
            impl ::core::convert::From<tokenReturn> for UnderlyingRustTuple<'_> {
                fn from(value: tokenReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for tokenReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for tokenCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = tokenReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "token()";
            const SELECTOR: [u8; 4] = [252u8, 12u8, 84u8, 106u8];
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
    /**Function with signature `tokenDecimals()` and selector `0x3b97e856`.
```solidity
function tokenDecimals() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct tokenDecimalsCall {}
    ///Container type for the return parameters of the [`tokenDecimals()`](tokenDecimalsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct tokenDecimalsReturn {
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
            impl ::core::convert::From<tokenDecimalsCall> for UnderlyingRustTuple<'_> {
                fn from(value: tokenDecimalsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for tokenDecimalsCall {
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
            impl ::core::convert::From<tokenDecimalsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: tokenDecimalsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for tokenDecimalsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for tokenDecimalsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = tokenDecimalsReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "tokenDecimals()";
            const SELECTOR: [u8; 4] = [59u8, 151u8, 232u8, 86u8];
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
    /**Function with signature `tokenOracle()` and selector `0x6c5ec25c`.
```solidity
function tokenOracle() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct tokenOracleCall {}
    ///Container type for the return parameters of the [`tokenOracle()`](tokenOracleCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct tokenOracleReturn {
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
            impl ::core::convert::From<tokenOracleCall> for UnderlyingRustTuple<'_> {
                fn from(value: tokenOracleCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for tokenOracleCall {
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
            impl ::core::convert::From<tokenOracleReturn> for UnderlyingRustTuple<'_> {
                fn from(value: tokenOracleReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for tokenOracleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for tokenOracleCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = tokenOracleReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "tokenOracle()";
            const SELECTOR: [u8; 4] = [108u8, 94u8, 194u8, 92u8];
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
    /**Function with signature `updateConfig(uint32,uint32)` and selector `0x3c2154bc`.
```solidity
function updateConfig(uint32 _priceMarkup, uint32 _updateThreshold) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateConfigCall {
        pub _priceMarkup: u32,
        pub _updateThreshold: u32,
    }
    ///Container type for the return parameters of the [`updateConfig(uint32,uint32)`](updateConfigCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateConfigReturn {}
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
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u32, u32);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateConfigCall> for UnderlyingRustTuple<'_> {
                fn from(value: updateConfigCall) -> Self {
                    (value._priceMarkup, value._updateThreshold)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateConfigCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _priceMarkup: tuple.0,
                        _updateThreshold: tuple.1,
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
            impl ::core::convert::From<updateConfigReturn> for UnderlyingRustTuple<'_> {
                fn from(value: updateConfigReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateConfigReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateConfigCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateConfigReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateConfig(uint32,uint32)";
            const SELECTOR: [u8; 4] = [60u8, 33u8, 84u8, 188u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self._priceMarkup),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self._updateThreshold),
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
    /**Function with signature `updatePrice()` and selector `0x673a7e28`.
```solidity
function updatePrice() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updatePriceCall {}
    ///Container type for the return parameters of the [`updatePrice()`](updatePriceCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updatePriceReturn {}
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
            impl ::core::convert::From<updatePriceCall> for UnderlyingRustTuple<'_> {
                fn from(value: updatePriceCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updatePriceCall {
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
            impl ::core::convert::From<updatePriceReturn> for UnderlyingRustTuple<'_> {
                fn from(value: updatePriceReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updatePriceReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updatePriceCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = updatePriceReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updatePrice()";
            const SELECTOR: [u8; 4] = [103u8, 58u8, 126u8, 40u8];
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
    /**Function with signature `validatePaymasterUserOp((address,uint256,bytes,bytes,uint256,uint256,uint256,uint256,uint256,bytes,bytes),bytes32,uint256)` and selector `0xf465c77e`.
```solidity
function validatePaymasterUserOp(UserOperation memory userOp, bytes32 userOpHash, uint256 maxCost) external returns (bytes memory context, uint256 validationData);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct validatePaymasterUserOpCall {
        pub userOp: <UserOperation as alloy::sol_types::SolType>::RustType,
        pub userOpHash: alloy::sol_types::private::FixedBytes<32>,
        pub maxCost: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`validatePaymasterUserOp((address,uint256,bytes,bytes,uint256,uint256,uint256,uint256,uint256,bytes,bytes),bytes32,uint256)`](validatePaymasterUserOpCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct validatePaymasterUserOpReturn {
        pub context: alloy::sol_types::private::Bytes,
        pub validationData: alloy::sol_types::private::primitives::aliases::U256,
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
                UserOperation,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <UserOperation as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<validatePaymasterUserOpCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: validatePaymasterUserOpCall) -> Self {
                    (value.userOp, value.userOpHash, value.maxCost)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for validatePaymasterUserOpCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        userOp: tuple.0,
                        userOpHash: tuple.1,
                        maxCost: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<validatePaymasterUserOpReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: validatePaymasterUserOpReturn) -> Self {
                    (value.context, value.validationData)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for validatePaymasterUserOpReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        context: tuple.0,
                        validationData: tuple.1,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for validatePaymasterUserOpCall {
            type Parameters<'a> = (
                UserOperation,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = validatePaymasterUserOpReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "validatePaymasterUserOp((address,uint256,bytes,bytes,uint256,uint256,uint256,uint256,uint256,bytes,bytes),bytes32,uint256)";
            const SELECTOR: [u8; 4] = [244u8, 101u8, 199u8, 126u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <UserOperation as alloy_sol_types::SolType>::tokenize(&self.userOp),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.userOpHash),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.maxCost),
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
function withdrawTo(address withdrawAddress, uint256 amount) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct withdrawToCall {
        pub withdrawAddress: alloy::sol_types::private::Address,
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
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
                    (value.withdrawAddress, value.amount)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for withdrawToCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        withdrawAddress: tuple.0,
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
    /**Function with signature `withdrawToken(address,uint256)` and selector `0x9e281a98`.
```solidity
function withdrawToken(address to, uint256 amount) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct withdrawTokenCall {
        pub to: alloy::sol_types::private::Address,
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`withdrawToken(address,uint256)`](withdrawTokenCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct withdrawTokenReturn {}
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
            impl ::core::convert::From<withdrawTokenCall> for UnderlyingRustTuple<'_> {
                fn from(value: withdrawTokenCall) -> Self {
                    (value.to, value.amount)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for withdrawTokenCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        to: tuple.0,
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
            impl ::core::convert::From<withdrawTokenReturn> for UnderlyingRustTuple<'_> {
                fn from(value: withdrawTokenReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for withdrawTokenReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for withdrawTokenCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = withdrawTokenReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "withdrawToken(address,uint256)";
            const SELECTOR: [u8; 4] = [158u8, 40u8, 26u8, 152u8];
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
                        &self.to,
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
    ///Container for all the [`PimlicoERC20Paymaster`](self) function calls.
    pub enum PimlicoERC20PaymasterCalls {
        REFUND_POSTOP_COST(REFUND_POSTOP_COSTCall),
        addStake(addStakeCall),
        deposit(depositCall),
        entryPoint(entryPointCall),
        getDeposit(getDepositCall),
        nativeAssetOracle(nativeAssetOracleCall),
        owner(ownerCall),
        postOp(postOpCall),
        previousPrice(previousPriceCall),
        priceDenominator(priceDenominatorCall),
        priceMarkup(priceMarkupCall),
        priceUpdateThreshold(priceUpdateThresholdCall),
        renounceOwnership(renounceOwnershipCall),
        token(tokenCall),
        tokenDecimals(tokenDecimalsCall),
        tokenOracle(tokenOracleCall),
        transferOwnership(transferOwnershipCall),
        unlockStake(unlockStakeCall),
        updateConfig(updateConfigCall),
        updatePrice(updatePriceCall),
        validatePaymasterUserOp(validatePaymasterUserOpCall),
        withdrawStake(withdrawStakeCall),
        withdrawTo(withdrawToCall),
        withdrawToken(withdrawTokenCall),
    }
    #[automatically_derived]
    impl PimlicoERC20PaymasterCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [3u8, 150u8, 203u8, 96u8],
            [32u8, 92u8, 40u8, 120u8],
            [58u8, 52u8, 200u8, 63u8],
            [59u8, 151u8, 232u8, 86u8],
            [60u8, 33u8, 84u8, 188u8],
            [62u8, 4u8, 97u8, 157u8],
            [103u8, 58u8, 126u8, 40u8],
            [108u8, 94u8, 194u8, 92u8],
            [113u8, 80u8, 24u8, 166u8],
            [141u8, 165u8, 203u8, 91u8],
            [145u8, 78u8, 36u8, 90u8],
            [157u8, 189u8, 185u8, 119u8],
            [158u8, 40u8, 26u8, 152u8],
            [169u8, 162u8, 52u8, 9u8],
            [176u8, 214u8, 145u8, 254u8],
            [187u8, 159u8, 230u8, 191u8],
            [194u8, 58u8, 92u8, 234u8],
            [195u8, 153u8, 236u8, 136u8],
            [205u8, 207u8, 75u8, 155u8],
            [208u8, 227u8, 13u8, 176u8],
            [239u8, 177u8, 173u8, 93u8],
            [242u8, 253u8, 227u8, 139u8],
            [244u8, 101u8, 199u8, 126u8],
            [252u8, 12u8, 84u8, 106u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for PimlicoERC20PaymasterCalls {
        const NAME: &'static str = "PimlicoERC20PaymasterCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 24usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::REFUND_POSTOP_COST(_) => {
                    <REFUND_POSTOP_COSTCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::addStake(_) => <addStakeCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::deposit(_) => <depositCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::entryPoint(_) => {
                    <entryPointCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getDeposit(_) => {
                    <getDepositCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::nativeAssetOracle(_) => {
                    <nativeAssetOracleCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::owner(_) => <ownerCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::postOp(_) => <postOpCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::previousPrice(_) => {
                    <previousPriceCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::priceDenominator(_) => {
                    <priceDenominatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::priceMarkup(_) => {
                    <priceMarkupCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::priceUpdateThreshold(_) => {
                    <priceUpdateThresholdCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::renounceOwnership(_) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::token(_) => <tokenCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::tokenDecimals(_) => {
                    <tokenDecimalsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::tokenOracle(_) => {
                    <tokenOracleCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::transferOwnership(_) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::unlockStake(_) => {
                    <unlockStakeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::updateConfig(_) => {
                    <updateConfigCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::updatePrice(_) => {
                    <updatePriceCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::validatePaymasterUserOp(_) => {
                    <validatePaymasterUserOpCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::withdrawStake(_) => {
                    <withdrawStakeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::withdrawTo(_) => {
                    <withdrawToCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::withdrawToken(_) => {
                    <withdrawTokenCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<PimlicoERC20PaymasterCalls>] = &[
                {
                    fn addStake(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PimlicoERC20PaymasterCalls> {
                        <addStakeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PimlicoERC20PaymasterCalls::addStake)
                    }
                    addStake
                },
                {
                    fn withdrawTo(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PimlicoERC20PaymasterCalls> {
                        <withdrawToCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PimlicoERC20PaymasterCalls::withdrawTo)
                    }
                    withdrawTo
                },
                {
                    fn priceUpdateThreshold(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PimlicoERC20PaymasterCalls> {
                        <priceUpdateThresholdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PimlicoERC20PaymasterCalls::priceUpdateThreshold)
                    }
                    priceUpdateThreshold
                },
                {
                    fn tokenDecimals(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PimlicoERC20PaymasterCalls> {
                        <tokenDecimalsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PimlicoERC20PaymasterCalls::tokenDecimals)
                    }
                    tokenDecimals
                },
                {
                    fn updateConfig(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PimlicoERC20PaymasterCalls> {
                        <updateConfigCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PimlicoERC20PaymasterCalls::updateConfig)
                    }
                    updateConfig
                },
                {
                    fn priceMarkup(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PimlicoERC20PaymasterCalls> {
                        <priceMarkupCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PimlicoERC20PaymasterCalls::priceMarkup)
                    }
                    priceMarkup
                },
                {
                    fn updatePrice(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PimlicoERC20PaymasterCalls> {
                        <updatePriceCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PimlicoERC20PaymasterCalls::updatePrice)
                    }
                    updatePrice
                },
                {
                    fn tokenOracle(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PimlicoERC20PaymasterCalls> {
                        <tokenOracleCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PimlicoERC20PaymasterCalls::tokenOracle)
                    }
                    tokenOracle
                },
                {
                    fn renounceOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PimlicoERC20PaymasterCalls> {
                        <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PimlicoERC20PaymasterCalls::renounceOwnership)
                    }
                    renounceOwnership
                },
                {
                    fn owner(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PimlicoERC20PaymasterCalls> {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PimlicoERC20PaymasterCalls::owner)
                    }
                    owner
                },
                {
                    fn previousPrice(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PimlicoERC20PaymasterCalls> {
                        <previousPriceCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PimlicoERC20PaymasterCalls::previousPrice)
                    }
                    previousPrice
                },
                {
                    fn REFUND_POSTOP_COST(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PimlicoERC20PaymasterCalls> {
                        <REFUND_POSTOP_COSTCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PimlicoERC20PaymasterCalls::REFUND_POSTOP_COST)
                    }
                    REFUND_POSTOP_COST
                },
                {
                    fn withdrawToken(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PimlicoERC20PaymasterCalls> {
                        <withdrawTokenCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PimlicoERC20PaymasterCalls::withdrawToken)
                    }
                    withdrawToken
                },
                {
                    fn postOp(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PimlicoERC20PaymasterCalls> {
                        <postOpCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PimlicoERC20PaymasterCalls::postOp)
                    }
                    postOp
                },
                {
                    fn entryPoint(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PimlicoERC20PaymasterCalls> {
                        <entryPointCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PimlicoERC20PaymasterCalls::entryPoint)
                    }
                    entryPoint
                },
                {
                    fn unlockStake(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PimlicoERC20PaymasterCalls> {
                        <unlockStakeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PimlicoERC20PaymasterCalls::unlockStake)
                    }
                    unlockStake
                },
                {
                    fn withdrawStake(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PimlicoERC20PaymasterCalls> {
                        <withdrawStakeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PimlicoERC20PaymasterCalls::withdrawStake)
                    }
                    withdrawStake
                },
                {
                    fn getDeposit(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PimlicoERC20PaymasterCalls> {
                        <getDepositCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PimlicoERC20PaymasterCalls::getDeposit)
                    }
                    getDeposit
                },
                {
                    fn priceDenominator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PimlicoERC20PaymasterCalls> {
                        <priceDenominatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PimlicoERC20PaymasterCalls::priceDenominator)
                    }
                    priceDenominator
                },
                {
                    fn deposit(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PimlicoERC20PaymasterCalls> {
                        <depositCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PimlicoERC20PaymasterCalls::deposit)
                    }
                    deposit
                },
                {
                    fn nativeAssetOracle(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PimlicoERC20PaymasterCalls> {
                        <nativeAssetOracleCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PimlicoERC20PaymasterCalls::nativeAssetOracle)
                    }
                    nativeAssetOracle
                },
                {
                    fn transferOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PimlicoERC20PaymasterCalls> {
                        <transferOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PimlicoERC20PaymasterCalls::transferOwnership)
                    }
                    transferOwnership
                },
                {
                    fn validatePaymasterUserOp(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PimlicoERC20PaymasterCalls> {
                        <validatePaymasterUserOpCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PimlicoERC20PaymasterCalls::validatePaymasterUserOp)
                    }
                    validatePaymasterUserOp
                },
                {
                    fn token(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PimlicoERC20PaymasterCalls> {
                        <tokenCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PimlicoERC20PaymasterCalls::token)
                    }
                    token
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
                Self::REFUND_POSTOP_COST(inner) => {
                    <REFUND_POSTOP_COSTCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::addStake(inner) => {
                    <addStakeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::deposit(inner) => {
                    <depositCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::entryPoint(inner) => {
                    <entryPointCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::getDeposit(inner) => {
                    <getDepositCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::nativeAssetOracle(inner) => {
                    <nativeAssetOracleCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::postOp(inner) => {
                    <postOpCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::previousPrice(inner) => {
                    <previousPriceCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::priceDenominator(inner) => {
                    <priceDenominatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::priceMarkup(inner) => {
                    <priceMarkupCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::priceUpdateThreshold(inner) => {
                    <priceUpdateThresholdCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::renounceOwnership(inner) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::token(inner) => {
                    <tokenCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::tokenDecimals(inner) => {
                    <tokenDecimalsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::tokenOracle(inner) => {
                    <tokenOracleCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::transferOwnership(inner) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::unlockStake(inner) => {
                    <unlockStakeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::updateConfig(inner) => {
                    <updateConfigCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::updatePrice(inner) => {
                    <updatePriceCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::validatePaymasterUserOp(inner) => {
                    <validatePaymasterUserOpCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::withdrawToken(inner) => {
                    <withdrawTokenCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::REFUND_POSTOP_COST(inner) => {
                    <REFUND_POSTOP_COSTCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::deposit(inner) => {
                    <depositCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::entryPoint(inner) => {
                    <entryPointCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getDeposit(inner) => {
                    <getDepositCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::nativeAssetOracle(inner) => {
                    <nativeAssetOracleCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::postOp(inner) => {
                    <postOpCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::previousPrice(inner) => {
                    <previousPriceCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::priceDenominator(inner) => {
                    <priceDenominatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::priceMarkup(inner) => {
                    <priceMarkupCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::priceUpdateThreshold(inner) => {
                    <priceUpdateThresholdCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::token(inner) => {
                    <tokenCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::tokenDecimals(inner) => {
                    <tokenDecimalsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::tokenOracle(inner) => {
                    <tokenOracleCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::unlockStake(inner) => {
                    <unlockStakeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::updateConfig(inner) => {
                    <updateConfigCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::updatePrice(inner) => {
                    <updatePriceCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::validatePaymasterUserOp(inner) => {
                    <validatePaymasterUserOpCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::withdrawToken(inner) => {
                    <withdrawTokenCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`PimlicoERC20Paymaster`](self) events.
    pub enum PimlicoERC20PaymasterEvents {
        ConfigUpdated(ConfigUpdated),
        OwnershipTransferred(OwnershipTransferred),
        UserOperationSponsored(UserOperationSponsored),
    }
    #[automatically_derived]
    impl PimlicoERC20PaymasterEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                71u8,
                42u8,
                66u8,
                160u8,
                68u8,
                82u8,
                123u8,
                135u8,
                223u8,
                2u8,
                192u8,
                206u8,
                142u8,
                108u8,
                0u8,
                192u8,
                5u8,
                127u8,
                172u8,
                64u8,
                214u8,
                196u8,
                36u8,
                201u8,
                60u8,
                36u8,
                176u8,
                35u8,
                34u8,
                235u8,
                20u8,
                181u8,
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
            [
                254u8,
                215u8,
                102u8,
                3u8,
                87u8,
                22u8,
                46u8,
                158u8,
                6u8,
                5u8,
                52u8,
                224u8,
                91u8,
                235u8,
                169u8,
                74u8,
                198u8,
                227u8,
                191u8,
                177u8,
                123u8,
                31u8,
                121u8,
                59u8,
                215u8,
                53u8,
                10u8,
                174u8,
                208u8,
                233u8,
                232u8,
                196u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for PimlicoERC20PaymasterEvents {
        const NAME: &'static str = "PimlicoERC20PaymasterEvents";
        const COUNT: usize = 3usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<ConfigUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <ConfigUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::ConfigUpdated)
                }
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
                Some(
                    <UserOperationSponsored as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <UserOperationSponsored as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::UserOperationSponsored)
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
    impl alloy_sol_types::private::IntoLogData for PimlicoERC20PaymasterEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::ConfigUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::UserOperationSponsored(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::ConfigUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::UserOperationSponsored(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`PimlicoERC20Paymaster`](self) contract instance.

See the [wrapper's documentation](`PimlicoERC20PaymasterInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> PimlicoERC20PaymasterInstance<T, P, N> {
        PimlicoERC20PaymasterInstance::<T, P, N>::new(address, provider)
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
        _token: alloy::sol_types::private::Address,
        _entryPoint: alloy::sol_types::private::Address,
        _tokenOracle: alloy::sol_types::private::Address,
        _nativeAssetOracle: alloy::sol_types::private::Address,
        _owner: alloy::sol_types::private::Address,
        _tokenDecimals: u8,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<PimlicoERC20PaymasterInstance<T, P, N>>,
    > {
        PimlicoERC20PaymasterInstance::<
            T,
            P,
            N,
        >::deploy(
            provider,
            _token,
            _entryPoint,
            _tokenOracle,
            _nativeAssetOracle,
            _owner,
            _tokenDecimals,
        )
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
        _token: alloy::sol_types::private::Address,
        _entryPoint: alloy::sol_types::private::Address,
        _tokenOracle: alloy::sol_types::private::Address,
        _nativeAssetOracle: alloy::sol_types::private::Address,
        _owner: alloy::sol_types::private::Address,
        _tokenDecimals: u8,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        PimlicoERC20PaymasterInstance::<
            T,
            P,
            N,
        >::deploy_builder(
            provider,
            _token,
            _entryPoint,
            _tokenOracle,
            _nativeAssetOracle,
            _owner,
            _tokenDecimals,
        )
    }
    /**A [`PimlicoERC20Paymaster`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`PimlicoERC20Paymaster`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct PimlicoERC20PaymasterInstance<
        T,
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for PimlicoERC20PaymasterInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("PimlicoERC20PaymasterInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > PimlicoERC20PaymasterInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`PimlicoERC20Paymaster`](self) contract instance.

See the [wrapper's documentation](`PimlicoERC20PaymasterInstance`) for more details.*/
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
            _token: alloy::sol_types::private::Address,
            _entryPoint: alloy::sol_types::private::Address,
            _tokenOracle: alloy::sol_types::private::Address,
            _nativeAssetOracle: alloy::sol_types::private::Address,
            _owner: alloy::sol_types::private::Address,
            _tokenDecimals: u8,
        ) -> alloy_contract::Result<PimlicoERC20PaymasterInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(
                provider,
                _token,
                _entryPoint,
                _tokenOracle,
                _nativeAssetOracle,
                _owner,
                _tokenDecimals,
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
            _token: alloy::sol_types::private::Address,
            _entryPoint: alloy::sol_types::private::Address,
            _tokenOracle: alloy::sol_types::private::Address,
            _nativeAssetOracle: alloy::sol_types::private::Address,
            _owner: alloy::sol_types::private::Address,
            _tokenDecimals: u8,
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall {
                            _token,
                            _entryPoint,
                            _tokenOracle,
                            _nativeAssetOracle,
                            _owner,
                            _tokenDecimals,
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
    impl<T, P: ::core::clone::Clone, N> PimlicoERC20PaymasterInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> PimlicoERC20PaymasterInstance<T, P, N> {
            PimlicoERC20PaymasterInstance {
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
    > PimlicoERC20PaymasterInstance<T, P, N> {
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
        ///Creates a new call builder for the [`REFUND_POSTOP_COST`] function.
        pub fn REFUND_POSTOP_COST(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, REFUND_POSTOP_COSTCall, N> {
            self.call_builder(&REFUND_POSTOP_COSTCall {})
        }
        ///Creates a new call builder for the [`addStake`] function.
        pub fn addStake(
            &self,
            unstakeDelaySec: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, addStakeCall, N> {
            self.call_builder(&addStakeCall { unstakeDelaySec })
        }
        ///Creates a new call builder for the [`deposit`] function.
        pub fn deposit(&self) -> alloy_contract::SolCallBuilder<T, &P, depositCall, N> {
            self.call_builder(&depositCall {})
        }
        ///Creates a new call builder for the [`entryPoint`] function.
        pub fn entryPoint(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, entryPointCall, N> {
            self.call_builder(&entryPointCall {})
        }
        ///Creates a new call builder for the [`getDeposit`] function.
        pub fn getDeposit(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getDepositCall, N> {
            self.call_builder(&getDepositCall {})
        }
        ///Creates a new call builder for the [`nativeAssetOracle`] function.
        pub fn nativeAssetOracle(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, nativeAssetOracleCall, N> {
            self.call_builder(&nativeAssetOracleCall {})
        }
        ///Creates a new call builder for the [`owner`] function.
        pub fn owner(&self) -> alloy_contract::SolCallBuilder<T, &P, ownerCall, N> {
            self.call_builder(&ownerCall {})
        }
        ///Creates a new call builder for the [`postOp`] function.
        pub fn postOp(
            &self,
            mode: <IPaymaster::PostOpMode as alloy::sol_types::SolType>::RustType,
            context: alloy::sol_types::private::Bytes,
            actualGasCost: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, postOpCall, N> {
            self.call_builder(
                &postOpCall {
                    mode,
                    context,
                    actualGasCost,
                },
            )
        }
        ///Creates a new call builder for the [`previousPrice`] function.
        pub fn previousPrice(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, previousPriceCall, N> {
            self.call_builder(&previousPriceCall {})
        }
        ///Creates a new call builder for the [`priceDenominator`] function.
        pub fn priceDenominator(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, priceDenominatorCall, N> {
            self.call_builder(&priceDenominatorCall {})
        }
        ///Creates a new call builder for the [`priceMarkup`] function.
        pub fn priceMarkup(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, priceMarkupCall, N> {
            self.call_builder(&priceMarkupCall {})
        }
        ///Creates a new call builder for the [`priceUpdateThreshold`] function.
        pub fn priceUpdateThreshold(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, priceUpdateThresholdCall, N> {
            self.call_builder(&priceUpdateThresholdCall {})
        }
        ///Creates a new call builder for the [`renounceOwnership`] function.
        pub fn renounceOwnership(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, renounceOwnershipCall, N> {
            self.call_builder(&renounceOwnershipCall {})
        }
        ///Creates a new call builder for the [`token`] function.
        pub fn token(&self) -> alloy_contract::SolCallBuilder<T, &P, tokenCall, N> {
            self.call_builder(&tokenCall {})
        }
        ///Creates a new call builder for the [`tokenDecimals`] function.
        pub fn tokenDecimals(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, tokenDecimalsCall, N> {
            self.call_builder(&tokenDecimalsCall {})
        }
        ///Creates a new call builder for the [`tokenOracle`] function.
        pub fn tokenOracle(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, tokenOracleCall, N> {
            self.call_builder(&tokenOracleCall {})
        }
        ///Creates a new call builder for the [`transferOwnership`] function.
        pub fn transferOwnership(
            &self,
            newOwner: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, transferOwnershipCall, N> {
            self.call_builder(&transferOwnershipCall { newOwner })
        }
        ///Creates a new call builder for the [`unlockStake`] function.
        pub fn unlockStake(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, unlockStakeCall, N> {
            self.call_builder(&unlockStakeCall {})
        }
        ///Creates a new call builder for the [`updateConfig`] function.
        pub fn updateConfig(
            &self,
            _priceMarkup: u32,
            _updateThreshold: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, updateConfigCall, N> {
            self.call_builder(
                &updateConfigCall {
                    _priceMarkup,
                    _updateThreshold,
                },
            )
        }
        ///Creates a new call builder for the [`updatePrice`] function.
        pub fn updatePrice(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, updatePriceCall, N> {
            self.call_builder(&updatePriceCall {})
        }
        ///Creates a new call builder for the [`validatePaymasterUserOp`] function.
        pub fn validatePaymasterUserOp(
            &self,
            userOp: <UserOperation as alloy::sol_types::SolType>::RustType,
            userOpHash: alloy::sol_types::private::FixedBytes<32>,
            maxCost: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, validatePaymasterUserOpCall, N> {
            self.call_builder(
                &validatePaymasterUserOpCall {
                    userOp,
                    userOpHash,
                    maxCost,
                },
            )
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
            amount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, withdrawToCall, N> {
            self.call_builder(
                &withdrawToCall {
                    withdrawAddress,
                    amount,
                },
            )
        }
        ///Creates a new call builder for the [`withdrawToken`] function.
        pub fn withdrawToken(
            &self,
            to: alloy::sol_types::private::Address,
            amount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, withdrawTokenCall, N> {
            self.call_builder(&withdrawTokenCall { to, amount })
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > PimlicoERC20PaymasterInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`ConfigUpdated`] event.
        pub fn ConfigUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, ConfigUpdated, N> {
            self.event_filter::<ConfigUpdated>()
        }
        ///Creates a new event filter for the [`OwnershipTransferred`] event.
        pub fn OwnershipTransferred_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OwnershipTransferred, N> {
            self.event_filter::<OwnershipTransferred>()
        }
        ///Creates a new event filter for the [`UserOperationSponsored`] event.
        pub fn UserOperationSponsored_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, UserOperationSponsored, N> {
            self.event_filter::<UserOperationSponsored>()
        }
    }
}
