export const entryPointAbi = [
  {
    type: 'error',
    name: 'ExecutionResult',
    inputs: [
      { type: 'uint256', name: 'preOpGas', internalType: 'uint256' },
      { type: 'uint256', name: 'paid', internalType: 'uint256' },
      { type: 'uint48', name: 'validAfter', internalType: 'uint48' },
      { type: 'uint48', name: 'validUntil', internalType: 'uint48' },
      { type: 'bool', name: 'targetSuccess', internalType: 'bool' },
      { type: 'bytes', name: 'targetResult', internalType: 'bytes' }
    ]
  },
  {
    type: 'error',
    name: 'FailedOp',
    inputs: [
      { type: 'uint256', name: 'opIndex', internalType: 'uint256' },
      { type: 'string', name: 'reason', internalType: 'string' }
    ]
  },
  {
    type: 'error',
    name: 'SenderAddressResult',
    inputs: [{ type: 'address', name: 'sender', internalType: 'address' }]
  },
  {
    type: 'error',
    name: 'SignatureValidationFailed',
    inputs: [{ type: 'address', name: 'aggregator', internalType: 'address' }]
  },
  {
    type: 'error',
    name: 'ValidationResult',
    inputs: [
      {
        type: 'tuple',
        name: 'returnInfo',
        internalType: 'struct IEntryPoint.ReturnInfo',
        components: [
          { type: 'uint256', name: 'preOpGas', internalType: 'uint256' },
          { type: 'uint256', name: 'prefund', internalType: 'uint256' },
          { type: 'bool', name: 'sigFailed', internalType: 'bool' },
          { type: 'uint48', name: 'validAfter', internalType: 'uint48' },
          { type: 'uint48', name: 'validUntil', internalType: 'uint48' },
          { type: 'bytes', name: 'paymasterContext', internalType: 'bytes' }
        ]
      },
      {
        type: 'tuple',
        name: 'senderInfo',
        internalType: 'struct IStakeManager.StakeInfo',
        components: [
          { type: 'uint256', name: 'stake', internalType: 'uint256' },
          { type: 'uint256', name: 'unstakeDelaySec', internalType: 'uint256' }
        ]
      },
      {
        type: 'tuple',
        name: 'factoryInfo',
        internalType: 'struct IStakeManager.StakeInfo',
        components: [
          { type: 'uint256', name: 'stake', internalType: 'uint256' },
          { type: 'uint256', name: 'unstakeDelaySec', internalType: 'uint256' }
        ]
      },
      {
        type: 'tuple',
        name: 'paymasterInfo',
        internalType: 'struct IStakeManager.StakeInfo',
        components: [
          { type: 'uint256', name: 'stake', internalType: 'uint256' },
          { type: 'uint256', name: 'unstakeDelaySec', internalType: 'uint256' }
        ]
      }
    ]
  },
  {
    type: 'error',
    name: 'ValidationResultWithAggregation',
    inputs: [
      {
        type: 'tuple',
        name: 'returnInfo',
        internalType: 'struct IEntryPoint.ReturnInfo',
        components: [
          { type: 'uint256', name: 'preOpGas', internalType: 'uint256' },
          { type: 'uint256', name: 'prefund', internalType: 'uint256' },
          { type: 'bool', name: 'sigFailed', internalType: 'bool' },
          { type: 'uint48', name: 'validAfter', internalType: 'uint48' },
          { type: 'uint48', name: 'validUntil', internalType: 'uint48' },
          { type: 'bytes', name: 'paymasterContext', internalType: 'bytes' }
        ]
      },
      {
        type: 'tuple',
        name: 'senderInfo',
        internalType: 'struct IStakeManager.StakeInfo',
        components: [
          { type: 'uint256', name: 'stake', internalType: 'uint256' },
          { type: 'uint256', name: 'unstakeDelaySec', internalType: 'uint256' }
        ]
      },
      {
        type: 'tuple',
        name: 'factoryInfo',
        internalType: 'struct IStakeManager.StakeInfo',
        components: [
          { type: 'uint256', name: 'stake', internalType: 'uint256' },
          { type: 'uint256', name: 'unstakeDelaySec', internalType: 'uint256' }
        ]
      },
      {
        type: 'tuple',
        name: 'paymasterInfo',
        internalType: 'struct IStakeManager.StakeInfo',
        components: [
          { type: 'uint256', name: 'stake', internalType: 'uint256' },
          { type: 'uint256', name: 'unstakeDelaySec', internalType: 'uint256' }
        ]
      },
      {
        type: 'tuple',
        name: 'aggregatorInfo',
        internalType: 'struct IEntryPoint.AggregatorStakeInfo',
        components: [
          { type: 'address', name: 'aggregator', internalType: 'address' },
          {
            type: 'tuple',
            name: 'stakeInfo',
            internalType: 'struct IStakeManager.StakeInfo',
            components: [
              { type: 'uint256', name: 'stake', internalType: 'uint256' },
              { type: 'uint256', name: 'unstakeDelaySec', internalType: 'uint256' }
            ]
          }
        ]
      }
    ]
  },
  {
    type: 'event',
    name: 'AccountDeployed',
    inputs: [
      { type: 'bytes32', name: 'userOpHash', internalType: 'bytes32', indexed: true },
      { type: 'address', name: 'sender', internalType: 'address', indexed: true },
      { type: 'address', name: 'factory', internalType: 'address', indexed: false },
      { type: 'address', name: 'paymaster', internalType: 'address', indexed: false }
    ],
    anonymous: false
  },
  { type: 'event', name: 'BeforeExecution', inputs: [], anonymous: false },
  {
    type: 'event',
    name: 'Deposited',
    inputs: [
      { type: 'address', name: 'account', internalType: 'address', indexed: true },
      { type: 'uint256', name: 'totalDeposit', internalType: 'uint256', indexed: false }
    ],
    anonymous: false
  },
  {
    type: 'event',
    name: 'SignatureAggregatorChanged',
    inputs: [{ type: 'address', name: 'aggregator', internalType: 'address', indexed: true }],
    anonymous: false
  },
  {
    type: 'event',
    name: 'StakeLocked',
    inputs: [
      { type: 'address', name: 'account', internalType: 'address', indexed: true },
      { type: 'uint256', name: 'totalStaked', internalType: 'uint256', indexed: false },
      { type: 'uint256', name: 'unstakeDelaySec', internalType: 'uint256', indexed: false }
    ],
    anonymous: false
  },
  {
    type: 'event',
    name: 'StakeUnlocked',
    inputs: [
      { type: 'address', name: 'account', internalType: 'address', indexed: true },
      { type: 'uint256', name: 'withdrawTime', internalType: 'uint256', indexed: false }
    ],
    anonymous: false
  },
  {
    type: 'event',
    name: 'StakeWithdrawn',
    inputs: [
      { type: 'address', name: 'account', internalType: 'address', indexed: true },
      { type: 'address', name: 'withdrawAddress', internalType: 'address', indexed: false },
      { type: 'uint256', name: 'amount', internalType: 'uint256', indexed: false }
    ],
    anonymous: false
  },
  {
    type: 'event',
    name: 'UserOperationEvent',
    inputs: [
      { type: 'bytes32', name: 'userOpHash', internalType: 'bytes32', indexed: true },
      { type: 'address', name: 'sender', internalType: 'address', indexed: true },
      { type: 'address', name: 'paymaster', internalType: 'address', indexed: true },
      { type: 'uint256', name: 'nonce', internalType: 'uint256', indexed: false },
      { type: 'bool', name: 'success', internalType: 'bool', indexed: false },
      { type: 'uint256', name: 'actualGasCost', internalType: 'uint256', indexed: false },
      { type: 'uint256', name: 'actualGasUsed', internalType: 'uint256', indexed: false }
    ],
    anonymous: false
  },
  {
    type: 'event',
    name: 'UserOperationRevertReason',
    inputs: [
      { type: 'bytes32', name: 'userOpHash', internalType: 'bytes32', indexed: true },
      { type: 'address', name: 'sender', internalType: 'address', indexed: true },
      { type: 'uint256', name: 'nonce', internalType: 'uint256', indexed: false },
      { type: 'bytes', name: 'revertReason', internalType: 'bytes', indexed: false }
    ],
    anonymous: false
  },
  {
    type: 'event',
    name: 'Withdrawn',
    inputs: [
      { type: 'address', name: 'account', internalType: 'address', indexed: true },
      { type: 'address', name: 'withdrawAddress', internalType: 'address', indexed: false },
      { type: 'uint256', name: 'amount', internalType: 'uint256', indexed: false }
    ],
    anonymous: false
  },
  {
    type: 'function',
    stateMutability: 'view',
    outputs: [{ type: 'uint256', name: '', internalType: 'uint256' }],
    name: 'SIG_VALIDATION_FAILED',
    inputs: []
  },
  {
    type: 'function',
    stateMutability: 'view',
    outputs: [],
    name: '_validateSenderAndPaymaster',
    inputs: [
      { type: 'bytes', name: 'initCode', internalType: 'bytes' },
      { type: 'address', name: 'sender', internalType: 'address' },
      { type: 'bytes', name: 'paymasterAndData', internalType: 'bytes' }
    ]
  },
  {
    type: 'function',
    stateMutability: 'payable',
    outputs: [],
    name: 'addStake',
    inputs: [{ type: 'uint32', name: 'unstakeDelaySec', internalType: 'uint32' }]
  },
  {
    type: 'function',
    stateMutability: 'view',
    outputs: [{ type: 'uint256', name: '', internalType: 'uint256' }],
    name: 'balanceOf',
    inputs: [{ type: 'address', name: 'account', internalType: 'address' }]
  },
  {
    type: 'function',
    stateMutability: 'payable',
    outputs: [],
    name: 'depositTo',
    inputs: [{ type: 'address', name: 'account', internalType: 'address' }]
  },
  {
    type: 'function',
    stateMutability: 'view',
    outputs: [
      { type: 'uint112', name: 'deposit', internalType: 'uint112' },
      { type: 'bool', name: 'staked', internalType: 'bool' },
      { type: 'uint112', name: 'stake', internalType: 'uint112' },
      { type: 'uint32', name: 'unstakeDelaySec', internalType: 'uint32' },
      { type: 'uint48', name: 'withdrawTime', internalType: 'uint48' }
    ],
    name: 'deposits',
    inputs: [{ type: 'address', name: '', internalType: 'address' }]
  },
  {
    type: 'function',
    stateMutability: 'view',
    outputs: [
      {
        type: 'tuple',
        name: 'info',
        internalType: 'struct IStakeManager.DepositInfo',
        components: [
          { type: 'uint112', name: 'deposit', internalType: 'uint112' },
          { type: 'bool', name: 'staked', internalType: 'bool' },
          { type: 'uint112', name: 'stake', internalType: 'uint112' },
          { type: 'uint32', name: 'unstakeDelaySec', internalType: 'uint32' },
          { type: 'uint48', name: 'withdrawTime', internalType: 'uint48' }
        ]
      }
    ],
    name: 'getDepositInfo',
    inputs: [{ type: 'address', name: 'account', internalType: 'address' }]
  },
  {
    type: 'function',
    stateMutability: 'view',
    outputs: [{ type: 'uint256', name: 'nonce', internalType: 'uint256' }],
    name: 'getNonce',
    inputs: [
      { type: 'address', name: 'sender', internalType: 'address' },
      { type: 'uint192', name: 'key', internalType: 'uint192' }
    ]
  },
  {
    type: 'function',
    stateMutability: 'nonpayable',
    outputs: [],
    name: 'getSenderAddress',
    inputs: [{ type: 'bytes', name: 'initCode', internalType: 'bytes' }]
  },
  {
    type: 'function',
    stateMutability: 'view',
    outputs: [{ type: 'bytes32', name: '', internalType: 'bytes32' }],
    name: 'getUserOpHash',
    inputs: [
      {
        type: 'tuple',
        name: 'userOp',
        internalType: 'struct UserOperation',
        components: [
          { type: 'address', name: 'sender', internalType: 'address' },
          { type: 'uint256', name: 'nonce', internalType: 'uint256' },
          { type: 'bytes', name: 'initCode', internalType: 'bytes' },
          { type: 'bytes', name: 'callData', internalType: 'bytes' },
          { type: 'uint256', name: 'callGasLimit', internalType: 'uint256' },
          { type: 'uint256', name: 'verificationGasLimit', internalType: 'uint256' },
          { type: 'uint256', name: 'preVerificationGas', internalType: 'uint256' },
          { type: 'uint256', name: 'maxFeePerGas', internalType: 'uint256' },
          { type: 'uint256', name: 'maxPriorityFeePerGas', internalType: 'uint256' },
          { type: 'bytes', name: 'paymasterAndData', internalType: 'bytes' },
          { type: 'bytes', name: 'signature', internalType: 'bytes' }
        ]
      }
    ]
  },
  {
    type: 'function',
    stateMutability: 'nonpayable',
    outputs: [],
    name: 'handleAggregatedOps',
    inputs: [
      {
        type: 'tuple[]',
        name: 'opsPerAggregator',
        internalType: 'struct IEntryPoint.UserOpsPerAggregator[]',
        components: [
          {
            type: 'tuple[]',
            name: 'userOps',
            internalType: 'struct UserOperation[]',
            components: [
              { type: 'address', name: 'sender', internalType: 'address' },
              { type: 'uint256', name: 'nonce', internalType: 'uint256' },
              { type: 'bytes', name: 'initCode', internalType: 'bytes' },
              { type: 'bytes', name: 'callData', internalType: 'bytes' },
              { type: 'uint256', name: 'callGasLimit', internalType: 'uint256' },
              { type: 'uint256', name: 'verificationGasLimit', internalType: 'uint256' },
              { type: 'uint256', name: 'preVerificationGas', internalType: 'uint256' },
              { type: 'uint256', name: 'maxFeePerGas', internalType: 'uint256' },
              { type: 'uint256', name: 'maxPriorityFeePerGas', internalType: 'uint256' },
              { type: 'bytes', name: 'paymasterAndData', internalType: 'bytes' },
              { type: 'bytes', name: 'signature', internalType: 'bytes' }
            ]
          },
          { type: 'address', name: 'aggregator', internalType: 'contract IAggregator' },
          { type: 'bytes', name: 'signature', internalType: 'bytes' }
        ]
      },
      { type: 'address', name: 'beneficiary', internalType: 'address payable' }
    ]
  },
  {
    type: 'function',
    stateMutability: 'nonpayable',
    outputs: [],
    name: 'handleOps',
    inputs: [
      {
        type: 'tuple[]',
        name: 'ops',
        internalType: 'struct UserOperation[]',
        components: [
          { type: 'address', name: 'sender', internalType: 'address' },
          { type: 'uint256', name: 'nonce', internalType: 'uint256' },
          { type: 'bytes', name: 'initCode', internalType: 'bytes' },
          { type: 'bytes', name: 'callData', internalType: 'bytes' },
          { type: 'uint256', name: 'callGasLimit', internalType: 'uint256' },
          { type: 'uint256', name: 'verificationGasLimit', internalType: 'uint256' },
          { type: 'uint256', name: 'preVerificationGas', internalType: 'uint256' },
          { type: 'uint256', name: 'maxFeePerGas', internalType: 'uint256' },
          { type: 'uint256', name: 'maxPriorityFeePerGas', internalType: 'uint256' },
          { type: 'bytes', name: 'paymasterAndData', internalType: 'bytes' },
          { type: 'bytes', name: 'signature', internalType: 'bytes' }
        ]
      },
      { type: 'address', name: 'beneficiary', internalType: 'address payable' }
    ]
  },
  {
    type: 'function',
    stateMutability: 'nonpayable',
    outputs: [],
    name: 'incrementNonce',
    inputs: [{ type: 'uint192', name: 'key', internalType: 'uint192' }]
  },
  {
    type: 'function',
    stateMutability: 'nonpayable',
    outputs: [{ type: 'uint256', name: 'actualGasCost', internalType: 'uint256' }],
    name: 'innerHandleOp',
    inputs: [
      { type: 'bytes', name: 'callData', internalType: 'bytes' },
      {
        type: 'tuple',
        name: 'opInfo',
        internalType: 'struct EntryPoint.UserOpInfo',
        components: [
          {
            type: 'tuple',
            name: 'mUserOp',
            internalType: 'struct EntryPoint.MemoryUserOp',
            components: [
              { type: 'address', name: 'sender', internalType: 'address' },
              { type: 'uint256', name: 'nonce', internalType: 'uint256' },
              { type: 'uint256', name: 'callGasLimit', internalType: 'uint256' },
              { type: 'uint256', name: 'verificationGasLimit', internalType: 'uint256' },
              { type: 'uint256', name: 'preVerificationGas', internalType: 'uint256' },
              { type: 'address', name: 'paymaster', internalType: 'address' },
              { type: 'uint256', name: 'maxFeePerGas', internalType: 'uint256' },
              { type: 'uint256', name: 'maxPriorityFeePerGas', internalType: 'uint256' }
            ]
          },
          { type: 'bytes32', name: 'userOpHash', internalType: 'bytes32' },
          { type: 'uint256', name: 'prefund', internalType: 'uint256' },
          { type: 'uint256', name: 'contextOffset', internalType: 'uint256' },
          { type: 'uint256', name: 'preOpGas', internalType: 'uint256' }
        ]
      },
      { type: 'bytes', name: 'context', internalType: 'bytes' }
    ]
  },
  {
    type: 'function',
    stateMutability: 'view',
    outputs: [{ type: 'uint256', name: '', internalType: 'uint256' }],
    name: 'nonceSequenceNumber',
    inputs: [
      { type: 'address', name: '', internalType: 'address' },
      { type: 'uint192', name: '', internalType: 'uint192' }
    ]
  },
  {
    type: 'function',
    stateMutability: 'nonpayable',
    outputs: [],
    name: 'simulateHandleOp',
    inputs: [
      {
        type: 'tuple',
        name: 'op',
        internalType: 'struct UserOperation',
        components: [
          { type: 'address', name: 'sender', internalType: 'address' },
          { type: 'uint256', name: 'nonce', internalType: 'uint256' },
          { type: 'bytes', name: 'initCode', internalType: 'bytes' },
          { type: 'bytes', name: 'callData', internalType: 'bytes' },
          { type: 'uint256', name: 'callGasLimit', internalType: 'uint256' },
          { type: 'uint256', name: 'verificationGasLimit', internalType: 'uint256' },
          { type: 'uint256', name: 'preVerificationGas', internalType: 'uint256' },
          { type: 'uint256', name: 'maxFeePerGas', internalType: 'uint256' },
          { type: 'uint256', name: 'maxPriorityFeePerGas', internalType: 'uint256' },
          { type: 'bytes', name: 'paymasterAndData', internalType: 'bytes' },
          { type: 'bytes', name: 'signature', internalType: 'bytes' }
        ]
      },
      { type: 'address', name: 'target', internalType: 'address' },
      { type: 'bytes', name: 'targetCallData', internalType: 'bytes' }
    ]
  },
  {
    type: 'function',
    stateMutability: 'nonpayable',
    outputs: [],
    name: 'simulateValidation',
    inputs: [
      {
        type: 'tuple',
        name: 'userOp',
        internalType: 'struct UserOperation',
        components: [
          { type: 'address', name: 'sender', internalType: 'address' },
          { type: 'uint256', name: 'nonce', internalType: 'uint256' },
          { type: 'bytes', name: 'initCode', internalType: 'bytes' },
          { type: 'bytes', name: 'callData', internalType: 'bytes' },
          { type: 'uint256', name: 'callGasLimit', internalType: 'uint256' },
          { type: 'uint256', name: 'verificationGasLimit', internalType: 'uint256' },
          { type: 'uint256', name: 'preVerificationGas', internalType: 'uint256' },
          { type: 'uint256', name: 'maxFeePerGas', internalType: 'uint256' },
          { type: 'uint256', name: 'maxPriorityFeePerGas', internalType: 'uint256' },
          { type: 'bytes', name: 'paymasterAndData', internalType: 'bytes' },
          { type: 'bytes', name: 'signature', internalType: 'bytes' }
        ]
      }
    ]
  },
  { type: 'function', stateMutability: 'nonpayable', outputs: [], name: 'unlockStake', inputs: [] },
  {
    type: 'function',
    stateMutability: 'nonpayable',
    outputs: [],
    name: 'withdrawStake',
    inputs: [{ type: 'address', name: 'withdrawAddress', internalType: 'address payable' }]
  },
  {
    type: 'function',
    stateMutability: 'nonpayable',
    outputs: [],
    name: 'withdrawTo',
    inputs: [
      { type: 'address', name: 'withdrawAddress', internalType: 'address payable' },
      { type: 'uint256', name: 'withdrawAmount', internalType: 'uint256' }
    ]
  },
  { type: 'receive', stateMutability: 'payable' }
];
