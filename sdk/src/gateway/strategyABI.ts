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
