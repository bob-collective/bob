# BTC Swap

This example demonstrates how BOB can be used to build a peer to peer swap application which allows BTC to be exchanged for an ERC20 token without the use of a custodian.

:::note

This example app is a work in progress, and the example application repository is currently set to private.

:::

## Set up local environment

1. Clone [https://github.com/bob-collective/bob](https://github.com/bob-collective/bob)
2. Install [Docker](https://www.docker.com)
3. Run `docker-compose up`
4. Verify that Regtest and Electrum are running by checking for the latest blocks at [http://localhost:3002/blocks](http://localhost:3002/blocks)
5. Add BOB L2 testnet to your EVM wallet and fund it with ETH using [Conduit](https://app.conduit.xyz/published/view/fluffy-bob-7mjgi9pmtg)

An example application can be found at [https://github.com/bob-collective/bob-ui-poc](https://github.com/bob-collective/bob-ui-poc). This has been built using [Viem](https://viem.sh/), [Wagmi](https://wagmi.sh/), React and Typescript, but you can interact with the smart contract using your preferred tools and frameworks.

## Funding your wallet with ERC20 tokens

TODO: Add faucet UI.

Add the contact addresses for supported ERC20 tokens to your Ethereum wallet:

- ZBTC: `0xd6cd079ee8bc26b5000a5e1ea8d434c840e3434b`
- USDT: `0x3c252953224948E441aAfdE7b391685201ccd3bC`

## Basic Protocol

### BTC marketplace smart contract ABI

```
export const BtcMarketplaceAbi = [
  {
    type: 'event',
    name: 'acceptBtcBuyOrderEvent',
    inputs: [
      { type: 'uint256', name: 'orderId', internalType: 'uint256', indexed: true },
      { type: 'uint256', name: 'acceptId', internalType: 'uint256', indexed: true },
      { type: 'uint256', name: 'amountBtc', internalType: 'uint256', indexed: false },
      { type: 'uint256', name: 'ercAmount', internalType: 'uint256', indexed: false },
      { type: 'address', name: 'ercToken', internalType: 'address', indexed: false }
    ],
    anonymous: false
  },
  {
    type: 'event',
    name: 'acceptBtcSellOrderEvent',
    inputs: [
      { type: 'uint256', name: 'id', internalType: 'uint256', indexed: true },
      { type: 'uint256', name: 'acceptId', internalType: 'uint256', indexed: true },
      {
        type: 'tuple',
        name: 'bitcoinAddress',
        internalType: 'struct BtcMarketPlace.BitcoinAddress',
        indexed: false,
        components: [{ type: 'uint256', name: 'bitcoinAddress', internalType: 'uint256' }]
      },
      { type: 'uint256', name: 'amountBtc', internalType: 'uint256', indexed: false },
      { type: 'uint256', name: 'ercAmount', internalType: 'uint256', indexed: false },
      { type: 'address', name: 'ercToken', internalType: 'address', indexed: false }
    ],
    anonymous: false
  },
  {
    type: 'event',
    name: 'cancelAcceptedBtcBuyOrderEvent',
    inputs: [{ type: 'uint256', name: 'id', internalType: 'uint256', indexed: false }],
    anonymous: false
  },
  {
    type: 'event',
    name: 'cancelAcceptedBtcSellOrderEvent',
    inputs: [{ type: 'uint256', name: 'id', internalType: 'uint256', indexed: false }],
    anonymous: false
  },
  {
    type: 'event',
    name: 'placeBtcBuyOrderEvent',
    inputs: [
      { type: 'uint256', name: 'amountBtc', internalType: 'uint256', indexed: false },
      {
        type: 'tuple',
        name: 'bitcoinAddress',
        internalType: 'struct BtcMarketPlace.BitcoinAddress',
        indexed: false,
        components: [{ type: 'uint256', name: 'bitcoinAddress', internalType: 'uint256' }]
      },
      { type: 'address', name: 'sellingToken', internalType: 'address', indexed: false },
      { type: 'uint256', name: 'saleAmount', internalType: 'uint256', indexed: false }
    ],
    anonymous: false
  },
  {
    type: 'event',
    name: 'placeBtcSellOrderEvent',
    inputs: [
      { type: 'uint256', name: 'orderId', internalType: 'uint256', indexed: true },
      { type: 'uint256', name: 'amountBtc', internalType: 'uint256', indexed: false },
      { type: 'address', name: 'buyingToken', internalType: 'address', indexed: false },
      { type: 'uint256', name: 'buyAmount', internalType: 'uint256', indexed: false }
    ],
    anonymous: false
  },
  {
    type: 'event',
    name: 'proofBtcBuyOrderEvent',
    inputs: [
      { type: 'uint256', name: 'id', internalType: 'uint256', indexed: false },
      {
        type: 'tuple',
        name: '_proof',
        internalType: 'struct BtcMarketPlace.TransactionProof',
        indexed: false,
        components: [{ type: 'uint256', name: 'dummy', internalType: 'uint256' }]
      }
    ],
    anonymous: false
  },
  {
    type: 'event',
    name: 'proofBtcSellOrderEvent',
    inputs: [
      { type: 'uint256', name: 'id', internalType: 'uint256', indexed: false },
      {
        type: 'tuple',
        name: '_proof',
        internalType: 'struct BtcMarketPlace.TransactionProof',
        indexed: false,
        components: [{ type: 'uint256', name: 'dummy', internalType: 'uint256' }]
      }
    ],
    anonymous: false
  },
  {
    type: 'event',
    name: 'withdrawBtcBuyOrderEvent',
    inputs: [{ type: 'uint256', name: 'id', internalType: 'uint256', indexed: false }],
    anonymous: false
  },
  {
    type: 'event',
    name: 'withdrawBtcSellOrderEvent',
    inputs: [{ type: 'uint256', name: 'id', internalType: 'uint256', indexed: false }],
    anonymous: false
  },
  {
    type: 'function',
    stateMutability: 'view',
    outputs: [{ type: 'uint256', name: '', internalType: 'uint256' }],
    name: 'REQUEST_EXPIRATION_SECONDS',
    inputs: []
  },
  {
    type: 'function',
    stateMutability: 'nonpayable',
    outputs: [{ type: 'uint256', name: '', internalType: 'uint256' }],
    name: 'acceptBtcBuyOrder',
    inputs: [
      { type: 'uint256', name: 'id', internalType: 'uint256' },
      { type: 'uint256', name: 'amountBtc', internalType: 'uint256' }
    ]
  },
  {
    type: 'function',
    stateMutability: 'nonpayable',
    outputs: [{ type: 'uint256', name: '', internalType: 'uint256' }],
    name: 'acceptBtcSellOrder',
    inputs: [
      { type: 'uint256', name: 'id', internalType: 'uint256' },
      {
        type: 'tuple',
        name: 'bitcoinAddress',
        internalType: 'struct BtcMarketPlace.BitcoinAddress',
        components: [{ type: 'uint256', name: 'bitcoinAddress', internalType: 'uint256' }]
      },
      { type: 'uint256', name: 'amountBtc', internalType: 'uint256' }
    ]
  },
  {
    type: 'function',
    stateMutability: 'view',
    outputs: [
      { type: 'uint256', name: 'orderId', internalType: 'uint256' },
      { type: 'uint256', name: 'amountBtc', internalType: 'uint256' },
      { type: 'address', name: 'ercToken', internalType: 'address' },
      { type: 'uint256', name: 'ercAmount', internalType: 'uint256' },
      { type: 'address', name: 'requester', internalType: 'address' },
      { type: 'address', name: 'accepter', internalType: 'address' },
      { type: 'uint256', name: 'acceptTime', internalType: 'uint256' }
    ],
    name: 'acceptedBtcBuyOrders',
    inputs: [{ type: 'uint256', name: '', internalType: 'uint256' }]
  },
  {
    type: 'function',
    stateMutability: 'view',
    outputs: [
      { type: 'uint256', name: 'orderId', internalType: 'uint256' },
      {
        type: 'tuple',
        name: 'bitcoinAddress',
        internalType: 'struct BtcMarketPlace.BitcoinAddress',
        components: [{ type: 'uint256', name: 'bitcoinAddress', internalType: 'uint256' }]
      },
      { type: 'uint256', name: 'amountBtc', internalType: 'uint256' },
      { type: 'address', name: 'ercToken', internalType: 'address' },
      { type: 'uint256', name: 'ercAmount', internalType: 'uint256' },
      { type: 'address', name: 'requester', internalType: 'address' },
      { type: 'address', name: 'accepter', internalType: 'address' },
      { type: 'uint256', name: 'acceptTime', internalType: 'uint256' }
    ],
    name: 'acceptedBtcSellOrders',
    inputs: [{ type: 'uint256', name: '', internalType: 'uint256' }]
  },
  {
    type: 'function',
    stateMutability: 'view',
    outputs: [
      { type: 'uint256', name: 'amountBtc', internalType: 'uint256' },
      {
        type: 'tuple',
        name: 'bitcoinAddress',
        internalType: 'struct BtcMarketPlace.BitcoinAddress',
        components: [{ type: 'uint256', name: 'bitcoinAddress', internalType: 'uint256' }]
      },
      { type: 'address', name: 'offeringToken', internalType: 'address' },
      { type: 'uint256', name: 'offeringAmount', internalType: 'uint256' },
      { type: 'address', name: 'requester', internalType: 'address' }
    ],
    name: 'btcBuyOrders',
    inputs: [{ type: 'uint256', name: '', internalType: 'uint256' }]
  },
  {
    type: 'function',
    stateMutability: 'view',
    outputs: [
      { type: 'uint256', name: 'amountBtc', internalType: 'uint256' },
      { type: 'address', name: 'askingToken', internalType: 'address' },
      { type: 'uint256', name: 'askingAmount', internalType: 'uint256' },
      { type: 'address', name: 'requester', internalType: 'address' }
    ],
    name: 'btcSellOrders',
    inputs: [{ type: 'uint256', name: '', internalType: 'uint256' }]
  },
  {
    type: 'function',
    stateMutability: 'nonpayable',
    outputs: [],
    name: 'cancelAcceptedBtcBuyOrder',
    inputs: [{ type: 'uint256', name: 'id', internalType: 'uint256' }]
  },
  {
    type: 'function',
    stateMutability: 'nonpayable',
    outputs: [],
    name: 'cancelAcceptedBtcSellOrder',
    inputs: [{ type: 'uint256', name: 'id', internalType: 'uint256' }]
  },
  {
    type: 'function',
    stateMutability: 'view',
    outputs: [
      {
        type: 'tuple[]',
        name: '',
        internalType: 'struct BtcMarketPlace.AcceptedBtcBuyOrder[]',
        components: [
          { type: 'uint256', name: 'orderId', internalType: 'uint256' },
          { type: 'uint256', name: 'amountBtc', internalType: 'uint256' },
          { type: 'address', name: 'ercToken', internalType: 'address' },
          { type: 'uint256', name: 'ercAmount', internalType: 'uint256' },
          { type: 'address', name: 'requester', internalType: 'address' },
          { type: 'address', name: 'accepter', internalType: 'address' },
          { type: 'uint256', name: 'acceptTime', internalType: 'uint256' }
        ]
      },
      { type: 'uint256[]', name: '', internalType: 'uint256[]' }
    ],
    name: 'getOpenAcceptedBtcBuyOrders',
    inputs: []
  },
  {
    type: 'function',
    stateMutability: 'view',
    outputs: [
      {
        type: 'tuple[]',
        name: '',
        internalType: 'struct BtcMarketPlace.AcceptedBtcSellOrder[]',
        components: [
          { type: 'uint256', name: 'orderId', internalType: 'uint256' },
          {
            type: 'tuple',
            name: 'bitcoinAddress',
            internalType: 'struct BtcMarketPlace.BitcoinAddress',
            components: [{ type: 'uint256', name: 'bitcoinAddress', internalType: 'uint256' }]
          },
          { type: 'uint256', name: 'amountBtc', internalType: 'uint256' },
          { type: 'address', name: 'ercToken', internalType: 'address' },
          { type: 'uint256', name: 'ercAmount', internalType: 'uint256' },
          { type: 'address', name: 'requester', internalType: 'address' },
          { type: 'address', name: 'accepter', internalType: 'address' },
          { type: 'uint256', name: 'acceptTime', internalType: 'uint256' }
        ]
      },
      { type: 'uint256[]', name: '', internalType: 'uint256[]' }
    ],
    name: 'getOpenAcceptedBtcSellOrders',
    inputs: []
  },
  {
    type: 'function',
    stateMutability: 'view',
    outputs: [
      {
        type: 'tuple[]',
        name: '',
        internalType: 'struct BtcMarketPlace.BtcBuyOrder[]',
        components: [
          { type: 'uint256', name: 'amountBtc', internalType: 'uint256' },
          {
            type: 'tuple',
            name: 'bitcoinAddress',
            internalType: 'struct BtcMarketPlace.BitcoinAddress',
            components: [{ type: 'uint256', name: 'bitcoinAddress', internalType: 'uint256' }]
          },
          { type: 'address', name: 'offeringToken', internalType: 'address' },
          { type: 'uint256', name: 'offeringAmount', internalType: 'uint256' },
          { type: 'address', name: 'requester', internalType: 'address' }
        ]
      },
      { type: 'uint256[]', name: '', internalType: 'uint256[]' }
    ],
    name: 'getOpenBtcBuyOrders',
    inputs: []
  },
  {
    type: 'function',
    stateMutability: 'view',
    outputs: [
      {
        type: 'tuple[]',
        name: '',
        internalType: 'struct BtcMarketPlace.BtcSellOrder[]',
        components: [
          { type: 'uint256', name: 'amountBtc', internalType: 'uint256' },
          { type: 'address', name: 'askingToken', internalType: 'address' },
          { type: 'uint256', name: 'askingAmount', internalType: 'uint256' },
          { type: 'address', name: 'requester', internalType: 'address' }
        ]
      },
      { type: 'uint256[]', name: '', internalType: 'uint256[]' }
    ],
    name: 'getOpenBtcSellOrders',
    inputs: []
  },
  {
    type: 'function',
    stateMutability: 'nonpayable',
    outputs: [],
    name: 'placeBtcBuyOrder',
    inputs: [
      { type: 'uint256', name: 'amountBtc', internalType: 'uint256' },
      {
        type: 'tuple',
        name: 'bitcoinAddress',
        internalType: 'struct BtcMarketPlace.BitcoinAddress',
        components: [{ type: 'uint256', name: 'bitcoinAddress', internalType: 'uint256' }]
      },
      { type: 'address', name: 'sellingToken', internalType: 'address' },
      { type: 'uint256', name: 'saleAmount', internalType: 'uint256' }
    ]
  },
  {
    type: 'function',
    stateMutability: 'nonpayable',
    outputs: [],
    name: 'placeBtcSellOrder',
    inputs: [
      { type: 'uint256', name: 'amountBtc', internalType: 'uint256' },
      { type: 'address', name: 'buyingToken', internalType: 'address' },
      { type: 'uint256', name: 'buyAmount', internalType: 'uint256' }
    ]
  },
  {
    type: 'function',
    stateMutability: 'nonpayable',
    outputs: [],
    name: 'proofBtcBuyOrder',
    inputs: [
      { type: 'uint256', name: 'id', internalType: 'uint256' },
      {
        type: 'tuple',
        name: '_proof',
        internalType: 'struct BtcMarketPlace.TransactionProof',
        components: [{ type: 'uint256', name: 'dummy', internalType: 'uint256' }]
      }
    ]
  },
  {
    type: 'function',
    stateMutability: 'nonpayable',
    outputs: [],
    name: 'proofBtcSellOrder',
    inputs: [
      { type: 'uint256', name: 'id', internalType: 'uint256' },
      {
        type: 'tuple',
        name: '_proof',
        internalType: 'struct BtcMarketPlace.TransactionProof',
        components: [{ type: 'uint256', name: 'dummy', internalType: 'uint256' }]
      }
    ]
  },
  {
    type: 'function',
    stateMutability: 'nonpayable',
    outputs: [],
    name: 'withdrawBtcBuyOrder',
    inputs: [{ type: 'uint256', name: 'id', internalType: 'uint256' }]
  },
  {
    type: 'function',
    stateMutability: 'nonpayable',
    outputs: [],
    name: 'withdrawBtcSellOrder',
    inputs: [{ type: 'uint256', name: 'id', internalType: 'uint256' }]
  }
] as const;

```

### ERC20 ABI

```
export const ERC20Abi = [
  {
    constant: true,
    inputs: [],
    name: 'name',
    outputs: [
      {
        name: '',
        type: 'string'
      }
    ],
    payable: false,
    stateMutability: 'view',
    type: 'function'
  },
  {
    constant: false,
    inputs: [
      {
        name: '_spender',
        type: 'address'
      },
      {
        name: '_value',
        type: 'uint256'
      }
    ],
    name: 'approve',
    outputs: [
      {
        name: '',
        type: 'bool'
      }
    ],
    payable: false,
    stateMutability: 'nonpayable',
    type: 'function'
  },
  {
    constant: true,
    inputs: [],
    name: 'totalSupply',
    outputs: [
      {
        name: '',
        type: 'uint256'
      }
    ],
    payable: false,
    stateMutability: 'view',
    type: 'function'
  },
  {
    constant: false,
    inputs: [
      {
        name: '_from',
        type: 'address'
      },
      {
        name: '_to',
        type: 'address'
      },
      {
        name: '_value',
        type: 'uint256'
      }
    ],
    name: 'transferFrom',
    outputs: [
      {
        name: '',
        type: 'bool'
      }
    ],
    payable: false,
    stateMutability: 'nonpayable',
    type: 'function'
  },
  {
    constant: true,
    inputs: [],
    name: 'decimals',
    outputs: [
      {
        name: '',
        type: 'uint8'
      }
    ],
    payable: false,
    stateMutability: 'view',
    type: 'function'
  },
  {
    constant: true,
    inputs: [
      {
        name: '_owner',
        type: 'address'
      }
    ],
    name: 'balanceOf',
    outputs: [
      {
        name: 'balance',
        type: 'uint256'
      }
    ],
    payable: false,
    stateMutability: 'view',
    type: 'function'
  },
  {
    constant: true,
    inputs: [],
    name: 'symbol',
    outputs: [
      {
        name: '',
        type: 'string'
      }
    ],
    payable: false,
    stateMutability: 'view',
    type: 'function'
  },
  {
    constant: false,
    inputs: [
      {
        name: '_to',
        type: 'address'
      },
      {
        name: '_value',
        type: 'uint256'
      }
    ],
    name: 'transfer',
    outputs: [
      {
        name: '',
        type: 'bool'
      }
    ],
    payable: false,
    stateMutability: 'nonpayable',
    type: 'function'
  },
  {
    constant: true,
    inputs: [
      {
        name: '_owner',
        type: 'address'
      },
      {
        name: '_spender',
        type: 'address'
      }
    ],
    name: 'allowance',
    outputs: [
      {
        name: '',
        type: 'uint256'
      }
    ],
    payable: false,
    stateMutability: 'view',
    type: 'function'
  },
  {
    payable: true,
    stateMutability: 'payable',
    type: 'fallback'
  },
  {
    anonymous: false,
    inputs: [
      {
        indexed: true,
        name: 'owner',
        type: 'address'
      },
      {
        indexed: true,
        name: 'spender',
        type: 'address'
      },
      {
        indexed: false,
        name: 'value',
        type: 'uint256'
      }
    ],
    name: 'Approval',
    type: 'event'
  },
  {
    anonymous: false,
    inputs: [
      {
        indexed: true,
        name: 'from',
        type: 'address'
      },
      {
        indexed: true,
        name: 'to',
        type: 'address'
      },
      {
        indexed: false,
        name: 'value',
        type: 'uint256'
      }
    ],
    name: 'Transfer',
    type: 'event'
  }
] as const;

```
