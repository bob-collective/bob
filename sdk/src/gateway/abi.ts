import { parseAbi } from 'viem';

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

export const offRampCaller = [
    {
        type: 'function',
        name: 'createOffRampRequest',
        inputs: [
            {
                name: '_offRampRequestArgs',
                type: 'tuple',
                internalType: 'struct OffRampRequestArgs',
                components: [
                    {
                        name: 'offRampAddress',
                        type: 'address',
                        internalType: 'contract IGateway',
                    },
                    {
                        name: 'amountLocked',
                        type: 'uint256',
                        internalType: 'uint256',
                    },
                    {
                        name: 'maxFees',
                        type: 'uint256',
                        internalType: 'uint256',
                    },
                    {
                        name: 'user',
                        type: 'address',
                        internalType: 'address',
                    },
                    {
                        name: 'token',
                        type: 'address',
                        internalType: 'contract IERC20Ext',
                    },
                    {
                        name: 'userBtcAddress',
                        type: 'bytes',
                        internalType: 'bytes',
                    },
                ],
            },
        ],
        outputs: [],
        stateMutability: 'nonpayable',
    },
] as const;

export const compoundV2CTokenAbi = parseAbi([
    'function exchangeRateCurrent() external returns (uint256)',
    'function underlying() external view returns (address)',
]);

export const aaveV2AtokenAbi = parseAbi(['function UNDERLYING_ASSET_ADDRESS() external view returns (address)']);
