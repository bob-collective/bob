import * as bitcoin from 'bitcoinjs-lib';
import {
    Address,
    createPublicClient,
    encodeAbiParameters,
    formatUnits,
    Hex,
    http,
    keccak256,
    parseAbiParameters,
    parseUnits,
    Chain as ViemChain,
    zeroAddress,
} from 'viem';
import { avalanche, base, berachain, bob, bsc, mainnet, optimism, sei, soneium, sonic, unichain } from 'viem/chains';
import {
    GatewayCreateOrderRequest,
    OnrampFeeBreakdown,
    OnrampFeeBreakdownRaw,
    OrderDetails,
    OrderDetailsRaw,
} from '../types';

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
            satsToSwapToEth: order.data.satsToSwapToEth,
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
            satsToSwapToEth: order.data.satsToSwapToEth,
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

export function convertOnrampFeeBreakdown(order: OnrampFeeBreakdownRaw): OnrampFeeBreakdown {
    return {
        overallFeeSats: order.overallFeeSats,
        protocolFeeSats: order.protocolFeeSats,
        affiliateFeeSats: order.affiliateFeeSats,
        executionFeeWei: BigInt(order.executionFeeWei),
        l1DataFeeWei: BigInt(order.l1DataFeeWei),
    };
}

export function parseBtc(btc: string) {
    return parseUnits(btc, 8);
}

export function formatBtc(btc: bigint) {
    return formatUnits(btc, 8);
}

const supportedChainsMapping = {
    bob,
    ethereum: mainnet,
    sonic,
    bsc,
    unichain,
    bera: berachain,
    sei,
    avalanche,
    base,
    soneium,
    optimism,
} as const;

const chainIdToChainConfigMapping = Object.values(supportedChainsMapping).reduce(
    (acc, chain) => {
        acc[chain.id] = chain;
        return acc;
    },
    {} as Record<ViemChain['id'], ViemChain>
);

function getChainIdByName(chainName: string) {
    const chain = supportedChainsMapping[chainName.toLowerCase()];
    if (!chain) {
        throw new Error(`Chain id for "${chainName}" not found. Allowed values ${Object.keys(supportedChainsMapping)}`);
    }
    return chain.id;
}

function getChainConfigById(chainId: number) {
    const config = chainIdToChainConfigMapping[chainId];
    if (!config) {
        throw new Error(
            `Chain id for "${chainId}" not found. Allowed values ${Object.values(supportedChainsMapping).map((chain) => chain.id)}`
        );
    }

    return config;
}

export function getChainConfig(fromChain: string | number) {
    if (typeof fromChain === 'string') {
        const chainId = getChainIdByName(fromChain);
        return getChainConfigById(chainId);
    }

    return getChainConfigById(fromChain);
}

// Compute the final ERC20 allowance storage slot
export function computeAllowanceSlot(owner: Address, spender: Address, tokenAllowanceSlot: bigint): Hex {
    const innerSlot = keccak256(
        encodeAbiParameters(
            [
                { name: 'owner', type: 'address' },
                { name: 'slot', type: 'uint256' },
            ],
            [owner, tokenAllowanceSlot]
        )
    );

    const allowanceSlot = keccak256(
        encodeAbiParameters(
            [
                { name: 'spender', type: 'address' },
                { name: 'innerSlot', type: 'bytes32' },
            ],
            [spender, innerSlot]
        )
    );
    return allowanceSlot;
}

// Compute the final ERC20 balance storage slot for a user
export function computeBalanceSlot(user: Address, balancesMappingSlot: bigint): Hex {
    // Compute the storage slot for the user's balance
    const balanceSlot = keccak256(
        encodeAbiParameters(
            [
                { name: 'owner', type: 'address' },
                { name: 'slot', type: 'uint256' },
            ],
            [user, balancesMappingSlot]
        )
    );

    return balanceSlot;
}

export function safeBigInt(value: string): bigint {
    try {
        return BigInt(value);
    } catch {
        throw new Error(`Invalid BigInt for value "${value}"`);
    }
}

export function safeNumber(value: string): number {
    const n = Number(value);
    if (Number.isNaN(n)) {
        throw new Error(`Invalid number for value  "${value}"`);
    }
    return n;
}
