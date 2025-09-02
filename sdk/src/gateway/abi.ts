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

export const offrampCaller = [
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
                    {
                        name: 'orderOwner',
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

export const compoundV2CTokenAbi = parseAbi([
    'function exchangeRateCurrent() external returns (uint256)',
    'function underlying() external view returns (address)',
]);

export const aaveV2AtokenAbi = parseAbi(['function UNDERLYING_ASSET_ADDRESS() external view returns (address)']);

export const claimDelayAbi = parseAbi(['function CLAIM_DELAY() view returns (uint64)']);

export const layerZeroOftAbi = [
    {
        type: 'function',
        name: 'quoteSend',
        inputs: [
            {
                name: 'sendParam',
                type: 'tuple',
                internalType: 'struct SendParam',
                components: [
                    { name: 'dstEid', type: 'uint32', internalType: 'uint32' },
                    { name: 'to', type: 'bytes32', internalType: 'bytes32' },
                    { name: 'amountLD', type: 'uint256', internalType: 'uint256' },
                    { name: 'minAmountLD', type: 'uint256', internalType: 'uint256' },
                    { name: 'extraOptions', type: 'bytes', internalType: 'bytes' },
                    { name: 'composeMsg', type: 'bytes', internalType: 'bytes' },
                    { name: 'oftCmd', type: 'bytes', internalType: 'bytes' },
                ],
            },
            {
                name: 'isNative',
                type: 'bool',
                internalType: 'bool',
            },
        ],
        outputs: [
            {
                name: '',
                type: 'tuple',
                internalType: 'struct MessagingFee',
                components: [
                    { name: 'nativeFee', type: 'uint256', internalType: 'uint256' },
                    { name: 'lzTokenFee', type: 'uint256', internalType: 'uint256' },
                ],
            },
        ],
        stateMutability: 'view',
    },
    {
        type: 'function',
        name: 'send',
        inputs: [
            {
                name: 'sendParam',
                type: 'tuple',
                internalType: 'struct SendParam',
                components: [
                    { name: 'dstEid', type: 'uint32', internalType: 'uint32' },
                    { name: 'to', type: 'bytes32', internalType: 'bytes32' },
                    { name: 'amountLD', type: 'uint256', internalType: 'uint256' },
                    { name: 'minAmountLD', type: 'uint256', internalType: 'uint256' },
                    { name: 'extraOptions', type: 'bytes', internalType: 'bytes' },
                    { name: 'composeMsg', type: 'bytes', internalType: 'bytes' },
                    { name: 'oftCmd', type: 'bytes', internalType: 'bytes' },
                ],
            },
            {
                name: 'fee',
                type: 'tuple',
                internalType: 'struct MessagingFee',
                components: [
                    { name: 'nativeFee', type: 'uint256', internalType: 'uint256' },
                    { name: 'lzTokenFee', type: 'uint256', internalType: 'uint256' },
                ],
            },
            {
                name: 'refundAddress',
                type: 'address',
                internalType: 'address',
            },
        ],
        outputs: [
            {
                name: 'receipt',
                type: 'tuple',
                internalType: 'struct MessagingReceipt',
                components: [
                    { name: 'guid', type: 'bytes32', internalType: 'bytes32' },
                    { name: 'nonce', type: 'uint64', internalType: 'uint64' },
                    {
                        name: 'fee',
                        type: 'tuple',
                        internalType: 'struct MessagingFee',
                        components: [
                            { name: 'nativeFee', type: 'uint256', internalType: 'uint256' },
                            { name: 'lzTokenFee', type: 'uint256', internalType: 'uint256' },
                        ],
                    },
                ],
            },
            {
                name: 'oftReceipt',
                type: 'tuple',
                internalType: 'struct OFTReceipt',
                components: [
                    { name: 'amountSentLD', type: 'uint256', internalType: 'uint256' },
                    { name: 'amountReceivedLD', type: 'uint256', internalType: 'uint256' },
                ],
            },
        ],
        stateMutability: 'payable',
    },
] as const;

export const quoterV2Abi = [
    {
        type: 'function',
        name: 'quoteExactOutputSingle',
        inputs: [
            {
                name: 'params',
                type: 'tuple',
                internalType: 'struct IQuoterV2.QuoteExactOutputSingleParams',
                components: [
                    { name: 'tokenIn', type: 'address', internalType: 'address' },
                    { name: 'tokenOut', type: 'address', internalType: 'address' },
                    { name: 'amountOut', type: 'uint256', internalType: 'uint256' },
                    { name: 'fee', type: 'uint24', internalType: 'uint24' },
                    { name: 'sqrtPriceLimitX96', type: 'uint160', internalType: 'uint160' },
                ],
            },
        ],
        outputs: [
            { name: 'amountIn', type: 'uint256', internalType: 'uint256' },
            { name: 'sqrtPriceX96After', type: 'uint160', internalType: 'uint160' },
            { name: 'initializedTicksCrossed', type: 'uint32', internalType: 'uint32' },
            { name: 'gasEstimate', type: 'uint256', internalType: 'uint256' },
        ],
        stateMutability: 'nonpayable',
    },
] as const;
