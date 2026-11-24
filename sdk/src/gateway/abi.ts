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

// `function approvalRequired() view returns (bool)`, written out so bundles skip viem's `parseAbi`.
export const oftApprovalRequiredAbi = [
    {
        type: 'function',
        name: 'approvalRequired',
        stateMutability: 'view',
        inputs: [],
        outputs: [{ type: 'bool' }],
    },
] as const;

export const USDTApproveAbi = [
    {
        type: 'function',
        name: 'approve',
        stateMutability: 'nonpayable',
        inputs: [
            { name: 'spender', type: 'address' },
            { name: 'amount', type: 'uint256' },
        ],
        outputs: [],
    },
] as const;
