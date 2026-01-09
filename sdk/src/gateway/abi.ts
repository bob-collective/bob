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

export const offrampCallerV2Abi = [
    {
        type: 'function',
        name: 'createOrderV2',
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
                        name: 'satSolverFeeMax',
                        type: 'uint256',
                        internalType: 'uint256',
                    },
                    {
                        name: 'satAffiliateFee',
                        type: 'uint256',
                        internalType: 'uint256',
                    },
                    {
                        name: 'affiliateFeeRecipient',
                        type: 'address',
                        internalType: 'address',
                    },
                    {
                        name: 'creationDeadline',
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
                    {
                        name: 'owner',
                        type: 'address',
                        internalType: 'address',
                    },
                ],
            },
        ],
        outputs: [],
        stateMutability: 'nonpayable',
    },
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
    {
        type: 'function',
        name: 'refundOrder',
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

export const compoundV2CTokenAbi = parseAbi([
    'function exchangeRateCurrent() external returns (uint256)',
    'function underlying() external view returns (address)',
]);

export const aaveV2AtokenAbi = parseAbi(['function UNDERLYING_ASSET_ADDRESS() external view returns (address)']);
