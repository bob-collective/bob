import {
    createPublicClient,
    http,
    zeroAddress,
    Chain as ViemChain,
    erc20Abi,
    parseAbi,
    decodeAbiParameters,
    concatHex,
    Address,
} from 'viem';
import { GatewayCreateOrderRequest, OfframpOrderStatus, OrderDetails, OrderDetailsRaw } from './types';
import { encodeAbiParameters, encodeFunctionData, parseAbiParameters, keccak256 } from 'viem';
import * as bitcoin from 'bitcoinjs-lib';

/**
 * Should compute the same OP_RETURN hash as the Gateway API and smart contracts.
 * This is used for data integrity checking.
 */
export function calculateOpReturnHash(req: GatewayCreateOrderRequest) {
    return keccak256(
        encodeAbiParameters(
            parseAbiParameters([
                'address gateway',
                'address strategy',
                'uint256 satsToConvertToEth',
                'address recipient',
                'bytes gatewayExtraData',
                'bytes strategyExtraData',
            ]),
            [
                req.gatewayAddress,
                req.strategyAddress || zeroAddress,
                BigInt(req.satsToConvertToEth),
                req.userAddress,
                req.gatewayExtraData || '0x',
                req.strategyExtraData || '0x',
            ]
        )
    );
}

const bridgeDataType = [
    { name: 'transactionId', type: 'bytes32', internalType: 'bytes32' },
    { name: 'bridge', type: 'string', internalType: 'string' },
    { name: 'integrator', type: 'string', internalType: 'string' },
    { name: 'referrer', type: 'address', internalType: 'address' },
    { name: 'sendingAssetId', type: 'address', internalType: 'address' },
    { name: 'receiver', type: 'address', internalType: 'address' },
    { name: 'minAmount', type: 'uint256', internalType: 'uint256' },
    { name: 'destinationChainId', type: 'uint256', internalType: 'uint256' },
    { name: 'hasSourceSwaps', type: 'bool', internalType: 'bool' },
    { name: 'hasDestinationCall', type: 'bool', internalType: 'bool' },
] as const;

const swapDataType = [
    { name: 'callTo', type: 'address', internalType: 'address' },
    {
        name: 'approveTo',
        type: 'address',
        internalType: 'address',
    },
    {
        name: 'sendingAssetId',
        type: 'address',
        internalType: 'address',
    },
    {
        name: 'receivingAssetId',
        type: 'address',
        internalType: 'address',
    },
    {
        name: 'fromAmount',
        type: 'uint256',
        internalType: 'uint256',
    },
    { name: 'callData', type: 'bytes', internalType: 'bytes' },
    {
        name: 'requiresDeposit',
        type: 'bool',
        internalType: 'bool',
    },
] as const;

const relayDataType = [
    {
        name: 'requestId',
        type: 'bytes32',
        internalType: 'bytes32',
    },
    {
        name: 'nonEVMReceiver',
        type: 'bytes32',
        internalType: 'bytes32',
    },
    {
        name: 'receivingAssetId',
        type: 'bytes32',
        internalType: 'bytes32',
    },
    { name: 'signature', type: 'bytes', internalType: 'bytes' },
] as const;

const lifiAbi = [
    {
        name: '_bridgeData',
        type: 'tuple',
        internalType: 'struct ILiFi.BridgeData',
        components: [
            { name: 'transactionId', type: 'bytes32', internalType: 'bytes32' },
            { name: 'bridge', type: 'string', internalType: 'string' },
            { name: 'integrator', type: 'string', internalType: 'string' },
            { name: 'referrer', type: 'address', internalType: 'address' },
            { name: 'sendingAssetId', type: 'address', internalType: 'address' },
            { name: 'receiver', type: 'address', internalType: 'address' },
            { name: 'minAmount', type: 'uint256', internalType: 'uint256' },
            { name: 'destinationChainId', type: 'uint256', internalType: 'uint256' },
            { name: 'hasSourceSwaps', type: 'bool', internalType: 'bool' },
            { name: 'hasDestinationCall', type: 'bool', internalType: 'bool' },
        ],
    },
    {
        name: '_swapData',
        type: 'tuple[]',
        internalType: 'struct LibSwap.SwapData[]',
        components: [
            { name: 'callTo', type: 'address', internalType: 'address' },
            { name: 'approveTo', type: 'address', internalType: 'address' },
            { name: 'sendingAssetId', type: 'address', internalType: 'address' },
            { name: 'receivingAssetId', type: 'address', internalType: 'address' },
            { name: 'fromAmount', type: 'uint256', internalType: 'uint256' },
            { name: 'callData', type: 'bytes', internalType: 'bytes' },
            { name: 'requiresDeposit', type: 'bool', internalType: 'bool' },
        ],
    },
    {
        name: '_relayData',
        type: 'tuple',
        internalType: 'struct RelayFacet.RelayData',
        components: [
            { name: 'requestId', type: 'bytes32', internalType: 'bytes32' },
            { name: 'nonEVMReceiver', type: 'bytes32', internalType: 'bytes32' },
            { name: 'receivingAssetId', type: 'bytes32', internalType: 'bytes32' },
            { name: 'signature', type: 'bytes', internalType: 'bytes' },
        ],
    },
] as const;

const LIFI_DIAMOND_ADDRESS = '0x1231DEB6f5749EF6cE6943a275A1D3E7486F4EaE';

export function encodeLiFiParameters(token: Address, swapAmount: bigint, minBridgeAmount: bigint) {
    const multicallAbi = parseAbi([
        'function makeCallWithInjection(address target, bytes callData, uint256 value, (address token, uint256 offset)[] replacements)',
    ]);

    const balanceReplacementForApproval = {
        token,
        offset: 36n,
    };

    // approve with injection
    const approveCalldata = encodeFunctionData({
        abi: erc20Abi,
        functionName: 'approve',
        args: [LIFI_DIAMOND_ADDRESS, 0n],
    });

    // Encode the makeCallWithInjection call
    const approveWithInjectionCalldata = encodeFunctionData({
        abi: multicallAbi,
        functionName: 'makeCallWithInjection',
        args: [LIFI_DIAMOND_ADDRESS, approveCalldata, BigInt(0), [balanceReplacementForApproval]],
    });

    const lifiTxCalldata =
        '0x25d374e80000000000000000000000000000000000000000000000000000000000000060000000000000000000000000000000000000000000000000000000000000022000000000000000000000000000000000000000000000000000000000000004000d35a30ed0b0ca7323f3fedd8cde75743d1433c9289e45ef01d49887a081290e0000000000000000000000000000000000000000000000000000000000000140000000000000000000000000000000000000000000000000000000000000018000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002260fac5e5542a773aa44fbcfedf7c193bc2c59900000000000000000000000097632b3760460a623e068cc70abf11d5fa99be5f00000000000000000000000000000000000000000000000000000000000f387c000000000000000000000000000000000000000000000000000000000000210500000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000572656c617900000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000086c6966692d61706900000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000020000000000000000000000000bd6c7b0d2f68c2b7805d88388319cfb6ecb50ea9000000000000000000000000bd6c7b0d2f68c2b7805d88388319cfb6ecb50ea90000000000000000000000002260fac5e5542a773aa44fbcfedf7c193bc2c5990000000000000000000000002260fac5e5542a773aa44fbcfedf7c193bc2c59900000000000000000000000000000000000000000000000000000000000f424000000000000000000000000000000000000000000000000000000000000000e000000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000084eedd56e10000000000000000000000002260fac5e5542a773aa44fbcfedf7c193bc2c599000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000009c4000000000000000000000000b9c0de368bece5e76b52545a8e377a4c118f597b000000000000000000000000000000000000000000000000000000005366919d2996ddfde07c8cd7fd2f6f0c1bb78364c488607f4b15f51379d98ba800000000000000000000000097632b3760460a623e068cc70abf11d5fa99be5f0000000000000000000000000555e30da8f98308edb960aa94c0db47230d2b9c0000000000000000000000000000000000000000000000000000000000000080000000000000000000000000000000000000000000000000000000000000004199fd4bf7634f69030b6e43dc2f9a201c4c7d69b09c2d789cb8f73341b0ccd7f56774c167a3dbf09a3615f84fb44f7f72330e13609455e3b7b148bc20787c75c91c00000000000000000000000000000000000000000000000000000000000000';

    const [bridgeData, swapData, relayData] = decodeAbiParameters(
        [
            { type: 'tuple', components: bridgeDataType },
            { type: 'tuple[]', components: swapDataType },
            { type: 'tuple', components: relayDataType },
        ],
        `0x${lifiTxCalldata.slice(10)}`
    );

    swapData[0].fromAmount = swapAmount;
    bridgeData.minAmount = minBridgeAmount;

    const lifiData = encodeFunctionData({
        abi: lifiAbi,
        functionName: 'swapAndStartBridgeTokensViaRelay',
        args: [bridgeData, swapData, relayData],
    });

    const balanceReplacementForLifiTx = {
        token,
        offset: 740n, // 23*32+4
    };

    const liFiWithInjectionCalldata = encodeFunctionData({
        abi: multicallAbi,
        functionName: 'makeCallWithInjection',
        args: [LIFI_DIAMOND_ADDRESS, lifiData, BigInt(0), [balanceReplacementForLifiTx]],
    });

    const calldata = concatHex([approveWithInjectionCalldata, liFiWithInjectionCalldata]);

    return calldata;
}

export function toHexScriptPubKey(userAddress: string, network: bitcoin.Network): string {
    const address = bitcoin.address.toOutputScript(userAddress, network);
    const buffer = Buffer.concat([Buffer.from([address.length]), address]);
    return '0x' + buffer.toString('hex'); // Convert buffer to hex string
}

export function isHexPrefixed(str: string): boolean {
    return str.slice(0, 2) === '0x';
}

export function stripHexPrefix(str: string): string {
    return isHexPrefixed(str) ? str.slice(2) : str;
}

export function slugify(str: string): string {
    return str
        .toLowerCase()
        .replace(/ /g, '-')
        .replace(/[^\w-]+/g, '');
}

const STATUSES = ['Active', 'Accepted', 'Processed', 'Refunded'] as const;

export function parseOrderStatus(value: number): OfframpOrderStatus {
    const status = STATUSES[value];
    if (status) return status;
    throw new Error(`Invalid order status: ${value}`);
}

export function viemClient(chain: ViemChain) {
    return createPublicClient({ chain, transport: http() });
}

function parseU256(value: string): bigint {
    return BigInt(value);
}

export function convertOrderDetailsRawToOrderDetails(order: OrderDetailsRaw): OrderDetails {
    return {
        version: order.version,
        data: {
            ethAmountToReceive: parseU256(order.data.ethAmountToReceive),
            maxSatsToSwapToEth: order.data.maxSatsToSwapToEth,
            ethTransferGasLimit: parseU256(order.data.ethTransferGasLimit),
            strategyGasLimit: parseU256(order.data.strategyGasLimit),
            totalUserGasLimit: parseU256(order.data.totalUserGasLimit),
            userGasPriceLimit: parseU256(order.data.userGasPriceLimit),
            l1DataFee: parseU256(order.data.l1DataFee),
            extraSatsFee: order.data.extraSatsFee !== null ? parseU256(order.data.extraSatsFee) : null,
            extraSatsFeeRecipient: order.data.extraSatsFeeRecipient,
        },
    };
}
export function convertOrderDetailsToRaw(order: OrderDetails): OrderDetailsRaw {
    return {
        version: order.version,
        data: {
            ethAmountToReceive: order.data.ethAmountToReceive.toString(), // bigint to string
            maxSatsToSwapToEth: order.data.maxSatsToSwapToEth,
            ethTransferGasLimit: order.data.ethTransferGasLimit.toString(),
            strategyGasLimit: order.data.strategyGasLimit.toString(),
            totalUserGasLimit: order.data.totalUserGasLimit.toString(),
            userGasPriceLimit: order.data.userGasPriceLimit.toString(),
            l1DataFee: order.data.l1DataFee.toString(),
            extraSatsFee: order.data.extraSatsFee !== null ? order.data.extraSatsFee.toString() : null,
            extraSatsFeeRecipient: order.data.extraSatsFeeRecipient,
        },
    };
}
