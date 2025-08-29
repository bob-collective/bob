import { createPublicClient, http, zeroAddress, Chain as ViemChain } from 'viem';
import { GatewayCreateOrderRequest, OfframpOrderStatus, OrderDetails, OrderDetailsRaw, BitcoinSigner } from './types';
import { encodeAbiParameters, parseAbiParameters, keccak256, parseUnits, formatUnits } from 'viem';
import * as bitcoin from 'bitcoinjs-lib';
import * as btc from '@scure/btc-signer';
import { base64 } from '@scure/base';
import { mnemonicToSeedSync } from 'bip39';
import { HDKey } from '@scure/bip32';

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

/**
 * Bitcoin signer implementation from seed phrase using scure-btc-signer
 */
export class ScureBitcoinSigner implements BitcoinSigner {
    private privateKey: Uint8Array;

    constructor(privateKeyHex: string) {
        const cleanPrivateKey = privateKeyHex.startsWith('0x') ? privateKeyHex.slice(2) : privateKeyHex;
        this.privateKey = new Uint8Array(Buffer.from(cleanPrivateKey, 'hex'));
    }

    /**
     * Create a Bitcoin signer from a seed phrase (BIP39 mnemonic)
     * @param seedPhrase The BIP39 mnemonic seed phrase
     * @param derivationPath The derivation path (e.g., "m/84'/0'/0'/0/0")
     * @returns Promise resolving to a new ScureBitcoinSigner instance
     */
    static async fromSeedPhrase(seedPhrase: string, derivationPath: string): Promise<ScureBitcoinSigner> {
        try {
            const seed = mnemonicToSeedSync(seedPhrase);
            const hdkey = HDKey.fromMasterSeed(seed);
            const childKey = hdkey.derive(derivationPath);
            if (!childKey.privateKey) {
                throw new Error('Failed to derive private key from seed phrase');
            }
            return new ScureBitcoinSigner(Buffer.from(childKey.privateKey).toString('hex'));
        } catch (error) {
            throw new Error(`Failed to create signer from seed phrase: ${error}`);
        }
    }

    /**
     * Sign all inputs in a PSBT
     *
     * @param psbtBase64 Base64 encoded PSBT string
     * @returns Promise resolving to the signed transaction hex
     */
    async signAllInputs(psbtBase64: string): Promise<string> {
        try {
            const tx = btc.Transaction.fromPSBT(base64.decode(psbtBase64));

            // Sign all inputs
            for (let i = 0; i < tx.inputsLength; i++) {
                tx.signIdx(this.privateKey, i);
            }

            tx.finalize();

            return tx.hex;
        } catch (error) {
            throw new Error(`Failed to sign PSBT with scure-btc-signer: ${error}`);
        }
    }

    /**
     * Get the P2WPKH address for this signer
     */
    async getP2WPKHAddress(): Promise<string> {
        return btc.getAddress('wpkh', this.privateKey) as string;
    }
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

export function parseBtc(btc: string) {
    return parseUnits(btc, 8);
}

export function formatBtc(btc: bigint) {
    return formatUnits(btc, 8);
}
