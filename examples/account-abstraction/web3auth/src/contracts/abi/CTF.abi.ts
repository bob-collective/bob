export const CTFAbi = [
  { type: 'constructor', stateMutability: 'nonpayable', inputs: [] },
  {
    type: 'event',
    name: 'FlagCaptured',
    inputs: [{ type: 'address', name: 'flagHolder', internalType: 'address', indexed: true }],
    anonymous: false
  },
  { type: 'function', stateMutability: 'nonpayable', outputs: [], name: 'captureFlag', inputs: [] },
  {
    type: 'function',
    stateMutability: 'view',
    outputs: [{ type: 'address', name: '', internalType: 'address' }],
    name: 'flagHolder',
    inputs: []
  },
  {
    type: 'function',
    stateMutability: 'nonpayable',
    outputs: [],
    name: 'transferOwnership',
    inputs: [{ type: 'address', name: 'newFlagHolder', internalType: 'address' }]
  }
];
