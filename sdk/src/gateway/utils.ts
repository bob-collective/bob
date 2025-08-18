import * as bitcoin from 'bitcoinjs-lib';
import {
    Address,
    createPublicClient,
    encodeAbiParameters,
    encodeFunctionData,
    erc20Abi,
    http,
    keccak256,
    parseAbiParameters,
    PublicClient,
    Transport,
    Chain as ViemChain,
    zeroAddress,
} from 'viem';
import { GatewayCreateOrderRequest, OfframpOrderStatus, OrderDetails, OrderDetailsRaw } from './types';

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

const LIFI_DIAMOND_ADDRESS = '0x1231DEB6f5749EF6cE6943a275A1D3E7486F4EaE';

const callAbi = [
    { name: 'target', type: 'address', internalType: 'address' },
    { name: 'callData', type: 'bytes', internalType: 'bytes' },
    { name: 'value', type: 'uint256', internalType: 'uint256' },
] as const;

const gatewayOnrampV4Abi = [
    {
        type: 'function',
        name: 'calculateFee',
        inputs: [
            { name: '_amount', type: 'uint256', internalType: 'uint256' },
            {
                name: '_feeRanges',
                type: 'tuple[]',
                internalType: 'struct FeeRange[]',
                components: [
                    { name: 'scaledFeePercent', type: 'uint256', internalType: 'uint256' },
                    { name: 'amountLowerRange', type: 'uint256', internalType: 'uint256' },
                ],
            },
        ],
        outputs: [{ name: 'fee', type: 'uint256', internalType: 'uint256' }],
        stateMutability: 'view',
    },
    {
        type: 'function',
        name: 'getFeeRanges',
        inputs: [],
        outputs: [
            {
                name: '_feeRanges',
                type: 'tuple[]',
                internalType: 'struct FeeRange[]',
                components: [
                    { name: 'scaledFeePercent', type: 'uint256', internalType: 'uint256' },
                    { name: 'amountLowerRange', type: 'uint256', internalType: 'uint256' },
                ],
            },
        ],
        stateMutability: 'view',
    },
    {
        type: 'function',
        name: 'multiplier',
        inputs: [],
        outputs: [{ name: '', type: 'uint256', internalType: 'uint256' }],
        stateMutability: 'view',
    },
] as const;

const gatewayV4Address = zeroAddress;

const lifiCalldata =
    '0x25d374e80000000000000000000000000000000000000000000000000000000000000060000000000000000000000000000000000000000000000000000000000000022000000000000000000000000000000000000000000000000000000000000007e0dfb0b59ca3790449592cbb192f9dbc5599e898a406f8166d0fde1b94d959fe3a000000000000000000000000000000000000000000000000000000000000014000000000000000000000000000000000000000000000000000000000000001800000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000097632b3760460a623e068cc70abf11d5fa99be5f000000000000000000000000000000000000000000000000031725319c721fa5000000000000000000000000000000000000000000000000000000000000210500000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000572656c617900000000000000000000000000000000000000000000000000000000000000000000000000000000s000000000000000000000000000000000000086c6966692d6170690000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000000000000000001e0000000000000000000000000728f23859c718b9e1ca762f051c29ea99119b92d000000000000000000000000728f23859c718b9e1ca762f051c29ea99119b92d00000000000000000000000003c7054bcb39f7b2e5b2c7acb37583e32d70cfa300000000000000000000000003c7054bcb39f7b2e5b2c7acb37583e32d70cfa300000000000000000000000000000000000000000000000000000000000d6d8000000000000000000000000000000000000000000000000000000000000000e000000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000084eedd56e100000000000000000000000003c7054bcb39f7b2e5b2c7acb37583e32d70cfa300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000898000000000000000000000000b9c0de368bece5e76b52545a8e377a4c118f597b000000000000000000000000000000000000000000000000000000000000000000000000000000003fc68470a35072c3a49ac28187c2cc0d4ad1bc570000000000000000000000003fc68470a35072c3a49ac28187c2cc0d4ad1bc5700000000000000000000000003c7054bcb39f7b2e5b2c7acb37583e32d70cfa3000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000d64e800000000000000000000000000000000000000000000000000000000000000e0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002a42646478b00000000000000000000000003c7054bcb39f7b2e5b2c7acb37583e32d70cfa300000000000000000000000000000000000000000000000000000000000d64e8000000000000000000000000eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee000000000000000000000000000000000000000000000000031725319c721fa5000000000000000000000000452cf1b8597e6319cd21abd847312bf17e26d8d100000000000000000000000000000000000000000000000000000000000000c000000000000000000000000000000000000000000000000000000000000001a40203c7054bcb39f7b2e5b2c7acb37583e32d70cfa303a22201e112389471d577f7bc45c03c7c37f70abca1cc93013fc68470a35072c3a49ac28187c2cc0d4ad1bc578000016407fec527abad1aafdb9a3b5a2171800c21a2fe013fc68470a35072c3a49ac28187c2cc0d4ad1bc57ffff08cc2acddbbf2c15e680c4480b449e94d4df53c0ef013fc68470a35072c3a49ac28187c2cc0d4ad1bc570105d032ac25d322df992303dca074ee7392c117b9037459019a3b17a2e2daf54ce80f2f4b2c8440902be715de013fc68470a35072c3a49ac28187c2cc0d4ad1bc57d55a012ece0e4b20ab662a9fc222a391b3ccb9ebf0485d013fc68470a35072c3a49ac28187c2cc0d4ad1bc57ffff0137760492b899398a2b3b86aa9490679cd9d63b2d013fc68470a35072c3a49ac28187c2cc0d4ad1bc5701e75d0fb2c24a55ca1e3f96781a2bcc7bdba058f001ffff0102f57b2e4d310f3cf432fafa8d0d780ee920467c003fc68470a35072c3a49ac28187c2cc0d4ad1bc5701420000000000000000000000000000000000000601ffff0200452cf1b8597e6319cd21abd847312bf17e26d8d10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000b8ac7e62fae2cc1eaac6e5d5431e9e3b1b0636ed6a3c16156e0abb4d94fa1f1300000000000000000000000097632b3760460a623e068cc70abf11d5fa99be5f0000000000000000000000000555e30da8f98308edb960aa94c0db47230d2b9c000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000000000000411f10d11f3d32c9bfa48d159e6ddae1180276b763bb05c68b715d49b972ce81ee599b63b6e55ad2ee48251163f70c7664324cc07cb5b15a193813045f780672f01b00000000000000000000000000000000000000000000000000000000000000';

export async function encodeLiFiParameters(
    publicClient: PublicClient<Transport>,
    tokenAddress: Address,
    swapAmount: bigint
) {
    const [feeRanges, multiplier] = await publicClient.multicall({
        allowFailure: false,
        contracts: [
            {
                abi: gatewayOnrampV4Abi,
                address: gatewayV4Address,
                functionName: 'getFeeRanges',
            },
            {
                abi: gatewayOnrampV4Abi,
                address: gatewayV4Address,
                functionName: 'multiplier',
            },
        ],
    });

    const calculatedFee = await publicClient.readContract({
        abi: gatewayOnrampV4Abi,
        functionName: 'calculateFee',
        address: gatewayV4Address,
        args: [swapAmount, feeRanges],
    });

    const tokenOrderSize = swapAmount * multiplier;

    const tokenLpFee = calculatedFee * multiplier;
    const tokenToSwapToEth = (swapAmount * multiplier) / 10n;
    const tokenForStrategy = tokenOrderSize - tokenLpFee - tokenToSwapToEth;

    const approveCalldata = encodeFunctionData({
        abi: erc20Abi,
        functionName: 'approve',
        args: [LIFI_DIAMOND_ADDRESS, tokenForStrategy],
    });

    const approveCall = encodeAbiParameters(callAbi, [tokenAddress, approveCalldata, 0n]);

    const lifiCall = encodeAbiParameters(callAbi, [LIFI_DIAMOND_ADDRESS, lifiCalldata, 0n]);

    return [approveCall, lifiCall];
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
