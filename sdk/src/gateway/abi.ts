export const strategyCaller = [
    {
        type: 'function',
        name: 'handleGatewayMessageWithSlippageArgs',
        inputs: [
            {
                name: 'tokenSent',
                type: 'address',
                internalType: 'contract IERC20',
            },
            {
                name: 'amountIn',
                type: 'uint256',
                internalType: 'uint256',
            },
            {
                name: 'recipient',
                type: 'address',
                internalType: 'address',
            },
            {
                name: 'args',
                type: 'tuple',
                internalType: 'struct StrategySlippageArgs',
                components: [
                    {
                        name: 'amountOutMin',
                        type: 'uint256',
                        internalType: 'uint256',
                    },
                ],
            },
        ],
        outputs: [],
        stateMutability: 'nonpayable',
    },
] as const;

export const offrampCreateOrderCaller = [
    {
        type: 'function',
        name: 'createOrder',
        inputs: [
            {
                name: 'offrampOrderArgs',
                type: 'tuple',
                internalType: 'struct OfframpOrderArgs',
                components: [
                    {
                        name: 'satAmountToLock',
                        type: 'uint256',
                        internalType: 'uint256',
                    },
                    {
                        name: 'satFeesMax',
                        type: 'uint256',
                        internalType: 'uint256',
                    },
                    {
                        name: 'orderCreationDeadline',
                        type: 'uint256',
                        internalType: 'uint256',
                    },
                    {
                        name: 'outputScript',
                        type: 'bytes',
                        internalType: 'bytes',
                    },
                    {
                        name: 'token',
                        type: 'address',
                        internalType: 'address',
                    },
                ],
            },
        ],
        outputs: [],
        stateMutability: 'nonpayable',
    },
] as const;

export const offrampBumpFeeCaller = [
    {
        type: 'function',
        name: 'bumpFeeOfExistingOrder',
        inputs: [
            {
                name: 'orderId',
                type: 'uint256',
                internalType: 'uint256',
            },
            {
                name: 'newFeeSat',
                type: 'uint256',
                internalType: 'uint256',
            },
        ],
        outputs: [],
        stateMutability: 'nonpayable',
    },
] as const;

export const offrampUnlockFundsCaller = [
    {
        type: 'function',
        name: 'unlockFunds',
        inputs: [
            {
                name: 'orderId',
                type: 'uint256',
                internalType: 'uint256',
            },
            {
                name: 'receiver',
                type: 'address',
                internalType: 'address',
            },
        ],
        outputs: [],
        stateMutability: 'nonpayable',
    },
] as const;

export const offrampGetOrderCaller = [
    {
        type: 'function',
        name: 'getOfframpOrder',
        inputs: [
            {
                name: 'orderId',
                type: 'uint256',
                internalType: 'uint256',
            },
        ],
        outputs: [
            {
                name: '',
                type: 'tuple',
                internalType: 'struct OfframpOrderDetails',
                components: [
                    { name: 'satAmountLocked', type: 'uint256', internalType: 'uint256' },
                    { name: 'satFeesMax', type: 'uint256', internalType: 'uint256' },
                    { name: 'sender', type: 'address', internalType: 'address' },
                    { name: 'receiver', type: 'address', internalType: 'address' },
                    { name: 'outputScript', type: 'bytes', internalType: 'bytes' },
                    { name: 'status', type: 'uint8', internalType: 'enum OfframpOrderStatus' },
                    { name: 'timestamp', type: 'uint256', internalType: 'uint256' },
                    { name: 'token', type: 'address', internalType: 'address' },
                    { name: 'owner', type: 'address', internalType: 'address' },
                ],
            },
        ],
        stateMutability: 'view',
    },
] as const;
