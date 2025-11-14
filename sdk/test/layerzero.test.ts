import { assert, describe, expect, it, vi } from 'vitest';
import { LayerZeroClient, LayerZeroGatewayClient } from '../src/gateway/layerzero';
import { createPublicClient, createWalletClient, http, PublicClient, Transport, zeroAddress } from 'viem';
import {
    bob,
    mainnet,
    base,
    berachain,
    bsc,
    unichain,
    avalanche,
    sonic,
    soneium,
    telos,
    swellchain,
    optimism,
    sei,
} from 'viem/chains';
import { BitcoinSigner, LayerZeroMessageWallet } from '../src/gateway/types';
import * as btc from '@scure/btc-signer';
import { base64 } from '@scure/base';
import { mnemonicToSeedSync } from 'bip39';
import { HDKey } from '@scure/bip32';
import { Hex } from 'viem';
import { privateKeyToAccount } from 'viem/accounts';
import { getCrossChainStatus } from '../src/gateway/utils/layerzero';
import { supportedChainsMapping } from '../src/gateway/utils/common';

describe('LayerZero Tests', () => {
    it.skip('should get chains', async () => {
        const client = new LayerZeroClient();
        const chainsInfo = await client.getSupportedChainsInfo();

        const chainNames = chainsInfo.map((chain) => chain.name);
        assert.containSubset(chainNames, [
            'ethereum',
            'bob',
            'base',
            'bera',
            'unichain',
            'avalanche',
            'sonic',
            'aptos',
            'bsc',
            'soneium',
            'telos',
            'swell',
            'optimism',
            'sei',
        ]);

        const findChain = (name: string) => chainsInfo.find((chain) => chain.name === name);

        assert.equal(findChain('ethereum')?.eid, '30101');
        assert.equal(findChain('bob')?.eid, '30279');
        assert.equal(findChain('base')?.eid, '30184');
        assert.equal(findChain('bera')?.eid, '30362');
        assert.equal(findChain('unichain')?.eid, '30320');
        assert.equal(findChain('avalanche')?.eid, '30106');
        assert.equal(findChain('sonic')?.eid, '30332');
        assert.equal(findChain('aptos')?.eid, '30108');
        assert.equal(findChain('bsc')?.eid, '30102');
        assert.equal(findChain('soneium')?.eid, '30340');
        assert.equal(findChain('telos')?.eid, '30199');
        assert.equal(findChain('swell')?.eid, '30335');
        assert.equal(findChain('optimism')?.eid, '30111');
        assert.equal(findChain('sei')?.eid, '30280');

        assert.equal(findChain('ethereum')?.oftAddress, '0x0555e30da8f98308edb960aa94c0db47230d2b9c');
        assert.equal(findChain('bob')?.oftAddress, '0x0555e30da8f98308edb960aa94c0db47230d2b9c');
        assert.equal(findChain('base')?.oftAddress, '0x0555e30da8f98308edb960aa94c0db47230d2b9c');
        assert.equal(findChain('bera')?.oftAddress, '0x0555e30da8f98308edb960aa94c0db47230d2b9c');
        assert.equal(findChain('unichain')?.oftAddress, '0x0555e30da8f98308edb960aa94c0db47230d2b9c');
        assert.equal(findChain('avalanche')?.oftAddress, '0x0555e30da8f98308edb960aa94c0db47230d2b9c');
        assert.equal(findChain('sonic')?.oftAddress, '0x0555e30da8f98308edb960aa94c0db47230d2b9c');
        assert.equal(
            findChain('aptos')?.oftAddress,
            '0xef3a1c7d6aa1a531336433e48d7b1d9b46c5bedc69f3db291df4e39bef4708e2'
        );
        assert.equal(findChain('bsc')?.oftAddress, '0x0555e30da8f98308edb960aa94c0db47230d2b9c');
        assert.equal(findChain('soneium')?.oftAddress, '0x0555e30da8f98308edb960aa94c0db47230d2b9c');
        assert.equal(findChain('telos')?.oftAddress, '0x0555e30da8f98308edb960aa94c0db47230d2b9c');
        assert.equal(findChain('swell')?.oftAddress, '0x0555e30da8f98308edb960aa94c0db47230d2b9c');
        assert.equal(findChain('optimism')?.oftAddress, '0xc3f854b2970f8727d28527ece33176fac67fef48');
        assert.equal(findChain('sei')?.oftAddress, '0x0555e30da8f98308edb960aa94c0db47230d2b9c');
    }, 120000);

    it.skip('should get an onramp quote and execute it', async () => {
        const client = new LayerZeroGatewayClient();

        const quote = await client.getQuote({
            fromChain: 'bitcoin',
            fromToken: 'bitcoin',
            toChain: 'ethereum',
            toToken: '0x0555E30da8f98308EdB960aa94C0Db47230d2B9c',
            fromUserAddress: 'bc1q6tgkjx4pgc5qda52fsgeuvjrhml5nuawwplejq',
            toUserAddress: '0x2A7f5295ac6e24b6D2ca78d82E3cbf01dDA52745',
            amount: 4000,
            l0OriginFinalityBuffer: 10000,
        });

        console.log('quote', quote);

        const publicClient = createPublicClient({
            chain: bob,
            transport: http(),
        });

        const walletClient = createWalletClient({
            chain: bob,
            transport: http(),
            account: zeroAddress,
        });

        const btcSignerFromSeed = await ScureBitcoinSigner.fromSeedPhrase(process.env.MNEMONIC!, "m/84'/0'/0'/0/0");
        console.log('P2WPKH Address: ', await btcSignerFromSeed.getP2WPKHAddress());

        const txHash = await client.executeQuote({
            quote,
            walletClient,
            publicClient: publicClient as PublicClient<Transport>,
            btcSigner: btcSignerFromSeed,
        });

        console.log(txHash);
    }, 120000);

    it.skip('should get an offramp quote and execute it', async () => {
        const client = new LayerZeroGatewayClient();
        const layerZeroClient = new LayerZeroClient();

        const quote = await client.getQuote({
            fromChain: 'base',
            fromToken: (await layerZeroClient.getOftAddressForChain('base')) as string, // WBTC on base
            toChain: 'bitcoin',
            toToken: 'bitcoin',
            fromUserAddress: '0x2A7f5295ac6e24b6D2ca78d82E3cbf01dDA52745', // EVM address (sender)
            toUserAddress: 'bc1q6tgkjx4pgc5qda52fsgeuvjrhml5nuawwplejq', // Bitcoin address (recipient)
            amount: 17000,
        });

        console.log('quote', quote);

        // Verify we got an offramp quote
        assert.ok(quote.data, 'Should have offramp quote');
        assert.ok(quote.params, 'Should have quote params');

        const publicClient = createPublicClient({
            chain: base,
            transport: http(),
        });

        const walletClient = createWalletClient({
            chain: base,
            transport: http(),
            account: privateKeyToAccount(process.env.PRIVATE_KEY as Hex),
        });

        const btcSignerFromSeed = await ScureBitcoinSigner.fromSeedPhrase(process.env.MNEMONIC!, "m/84'/0'/0'/0/0");
        console.log('P2WPKH Address: ', await btcSignerFromSeed.getP2WPKHAddress());

        const txHash = await client.executeQuote({
            quote,
            walletClient,
            publicClient: publicClient as PublicClient<Transport>,
            btcSigner: btcSignerFromSeed,
        });

        console.log(txHash);
    }, 120000);

    it.skip('should be able to get a layerzero send quote for every chain', async () => {
        const client = new LayerZeroGatewayClient();

        const l0ChainsInfo = await client.getSupportedChainsInfo();
        const toChain = l0ChainsInfo.find((chain) => chain.name === 'bob')!;

        for (const fromChain of l0ChainsInfo) {
            if (fromChain.name !== toChain.name) {
                const quote = await client.getQuote({
                    fromChain: fromChain.nativeChainId as number,
                    toChain: toChain.nativeChainId as number,
                    fromToken: fromChain.oftAddress as string,
                    toToken: toChain.oftAddress as string,
                    fromUserAddress: '0xEf7Ff7Fb24797656DF41616e807AB4016AE9dCD5',
                    toUserAddress: '0xEf7Ff7Fb24797656DF41616e807AB4016AE9dCD5',
                    amount: 100,
                });
            }
        }
    }, 120000);

    it.skip('should get a layerzero send quote and execute it', async () => {
        const client = new LayerZeroGatewayClient();

        const quote = await client.getQuote({
            fromChain: bsc.id,
            fromToken: (await client.getSupportedChainsInfo()).find((chain) => chain.name.toLowerCase() === bsc.name.toLowerCase())
                ?.oftAddress as string,
            toChain: base.id,
            toToken: (await client.getSupportedChainsInfo()).find((chain) => chain.name.toLowerCase() === base.name.toLowerCase())
                ?.oftAddress as string,
            fromUserAddress: '0xEf7Ff7Fb24797656DF41616e807AB4016AE9dCD5',
            toUserAddress: '0xEf7Ff7Fb24797656DF41616e807AB4016AE9dCD5',
            amount: 100,
        });

        console.log('quote', quote);

        const publicClient = createPublicClient({
            chain: bsc,
            transport: http(),
        });

        const walletClient = createWalletClient({
            chain: bsc,
            transport: http(),
            account: privateKeyToAccount(process.env.PRIVATE_KEY as Hex),
        });

        const txHash = await client.executeQuote({
            quote,
            walletClient,
            publicClient: publicClient as PublicClient<Transport>,
        });

        console.log(txHash);
    }, 120000);

    it('should get a layerzero send quote with a destination message and execute it', async () => {
        const client = new LayerZeroGatewayClient();

        const quote = await client.getQuote({
            fromChain: bsc.id,
            fromToken: (await client.getSupportedChainsInfo()).find((chain) => chain.name.toLowerCase() === bsc.name.toLowerCase())
                ?.oftAddress as string,
            toChain: base.id,
            toToken: (await client.getSupportedChainsInfo()).find((chain) => chain.name.toLowerCase() === base.name.toLowerCase())
                ?.oftAddress as string,
            fromUserAddress: '0xEf7Ff7Fb24797656DF41616e807AB4016AE9dCD5',
            toUserAddress: '0xEf7Ff7Fb24797656DF41616e807AB4016AE9dCD5',
            amount: 100,
            message: '0x0000000000000000000000006813eb9362372eef6200f3b1dbc3f819671cba690000000000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000200000000000000000000000000555e30da8f98308edb960aa94c0db47230d2b9c000000000000000000000000000000000000000000000000000000000000006000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000044a9059cbb0000000000000000000000002b5ad5c4795c026514f8317c7a215e218dccd6cf000000000000000000000000000000000000000000000000000000000000006400000000000000000000000000000000000000000000000000000000',
            destinationGasLimit: 100000,
        });

        console.log('quote', quote);

        const publicClient = createPublicClient({
            chain: bsc,
            transport: http(),
        });

        const walletClient = createWalletClient({
            chain: bsc,
            transport: http(),
            account: privateKeyToAccount(process.env.PRIVATE_KEY as Hex),
        });

        const txHash = await client.executeQuote({
            quote,
            walletClient,
            publicClient: publicClient as PublicClient<Transport>,
        });

        console.log(txHash);
    }, 120000);

    it('should get chain id for eid', async () => {
        const client = new LayerZeroClient();

        const optimismEid = 30111;

        assert.equal(await client.getChainId(optimismEid), optimism.id);
    }, 120000);

    it.skip('should return onramp, offramp and cross-chain orders', async () => {
        const gatewaySDK = new LayerZeroGatewayClient();

        const getOrdersSpy = vi.spyOn(gatewaySDK, 'getOrders').mockImplementationOnce(() => Promise.resolve([]));
        const getCrossChainOrdersSpy = vi
            .spyOn(gatewaySDK, 'getCrossChainSwapOrders')
            .mockImplementationOnce(() => Promise.resolve([]));

        const result = await gatewaySDK.getOrders(zeroAddress);

        expect(result).toEqual([]);
        expect(getOrdersSpy).toHaveBeenCalledOnce();
        expect(getCrossChainOrdersSpy).toHaveBeenCalledOnce();
    });

    describe('getCrossChainStatus', () => {
        const createMockMessage = (sourceStatus: string, destinationStatus: string): LayerZeroMessageWallet => ({
            pathway: {
                srcEid: 40184,
                dstEid: 30111,
                sender: {
                    address: '0x123',
                    id: 'sender-1',
                    name: 'Sender',
                    chain: 'bob',
                },
                receiver: {
                    address: '0x456',
                    id: 'receiver-1',
                    name: 'Receiver',
                    chain: 'optimism',
                },
                id: 'pathway-1',
                nonce: 1,
            },
            source: {
                status: sourceStatus,
                tx: {
                    txHash: '0xsource',
                    blockHash: '0xblock',
                    blockNumber: '12345',
                    blockTimestamp: 1700000000,
                    from: '0xfrom',
                    payload: '0x',
                    readinessTimestamp: 1700000100,
                },
            },
            destination: {
                status: destinationStatus,
                tx: {
                    txHash: '0xdest',
                    blockHash: '0xdestblock',
                    blockNumber: '67890',
                    blockTimestamp: 1700000200,
                    from: '0xdestfrom',
                    payload: '0x',
                    readinessTimestamp: 1700000300,
                },
                lzCompose: {
                    status: 'N/A',
                },
            },
        });

        it('should return source-pending when source is WAITING', () => {
            const message = createMockMessage('WAITING', 'WAITING');
            expect(getCrossChainStatus(message)).toBe('source-pending');
        });

        it('should return source-failed when source is SIMULATION_REVERTED', () => {
            const message = createMockMessage('SIMULATION_REVERTED', 'WAITING');
            expect(getCrossChainStatus(message)).toBe('source-failed');
        });

        it('should return destination-pending when source succeeded and destination is waiting', () => {
            const message = createMockMessage('SUCCEEDED', 'WAITING');
            expect(getCrossChainStatus(message)).toBe('destination-pending');
        });

        it('should return destination-confirmed when both source and destination succeeded', () => {
            const message = createMockMessage('SUCCEEDED', 'SUCCEEDED');
            expect(getCrossChainStatus(message)).toBe('destination-confirmed');
        });

        it('should return destination-failed when source succeeded but destination failed', () => {
            const message = createMockMessage('SUCCEEDED', 'SIMULATION_REVERTED');
            expect(getCrossChainStatus(message)).toBe('destination-failed');
        });

        it('should return unknown for unrecognized source status', () => {
            const message = createMockMessage('UNKNOWN_STATUS', 'WAITING');
            expect(getCrossChainStatus(message)).toBe('unknown');
        });

        it('should return unknown when source succeeded but destination has unknown status', () => {
            const message = createMockMessage('SUCCEEDED', 'UNKNOWN_STATUS');
            expect(getCrossChainStatus(message)).toBe('unknown');
        });
    });

    describe('getL0CreateOrderGasCost', () => {
        it('should estimate gas cost for order creation', async () => {
            // Arrange
            const l0Client = {
                getOftAddressForChain: async () => '0x0555E30da8f98308EdB960aa94C0Db47230d2B9c',
            };
            const publicClient = {
                estimateFeesPerGas: async () => ({ maxFeePerGas: BigInt(100) }),
                getGasPrice: async () => BigInt(100),
                estimateContractGas: async () => BigInt(21000),
            };
            const client = new TestLayerZeroGatewayClient(bob.id, l0Client, publicClient);

            // Mock params
            const params = {
                fromChain: 'bob',
                l0ChainId: bob.id,
                fromUserAddress: '0x1111111111111111111111111111111111111111',
                toUserAddress: '0x2222222222222222222222222222222222222222',
                amount: 1000,
                toChain: 'optimism',
                fromToken: 'wbtc',
                toToken: 'wbtc',
            };
            const sendParams = {
                dstEid: 30111,
                to: '0x2222222222222222222222222222222222222222', // valid hex string
                amountLD: BigInt(1000),
                minAmountLD: BigInt(1000),
                extraOptions: '0x',
                composeMsg: '0x',
                oftCmd: '0x',
            };
            const sendFees = {
                nativeFee: BigInt(100000),
                lzTokenFee: BigInt(50000),
            };
            const fromChain = 'bob';

            // Act
            const result = await client.getL0CreateOrderGasCost(params, sendParams as any, sendFees, fromChain);

            // Assert
            expect(result).toBe(BigInt(21000 * 100));
        });

        it('should throw error if WBTC OFT address is missing', async () => {
            const l0Client = {
                getOftAddressForChain: async () => null,
            };
            const publicClient = {
                estimateFeesPerGas: async () => ({ maxFeePerGas: BigInt(100) }),
                getGasPrice: async () => BigInt(100),
                estimateContractGas: async () => BigInt(21000),
            };
            const client = new TestLayerZeroGatewayClient(bob.id, l0Client, publicClient);
            const params = {
                fromChain: 'bob',
                l0ChainId: bob.id,
                fromUserAddress: '0x1111111111111111111111111111111111111111',
                toUserAddress: '0x2222222222222222222222222222222222222222',
                amount: 1000,
                toChain: 'optimism',
                fromToken: 'wbtc',
                toToken: 'wbtc',
            };
            const sendParams = {
                dstEid: 30111,
                to: '0x2222222222222222222222222222222222222222', // valid hex string
                amountLD: BigInt(1000),
                minAmountLD: BigInt(1000),
                extraOptions: '0x',
                composeMsg: '0x',
                oftCmd: '0x',
            };
            const sendFees = {
                nativeFee: BigInt(100000),
                lzTokenFee: BigInt(50000),
            };
            const fromChain = 'bob';

            await expect(
                client.getL0CreateOrderGasCost(params, sendParams as any, sendFees, fromChain)
            ).rejects.toThrow('WBTC OFT not found for chain: 40184');
        });
    });
});

/**
 * Bitcoin signer implementation from seed phrase using scure-btc-signer
 */
class ScureBitcoinSigner implements BitcoinSigner {
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

class TestLayerZeroGatewayClient extends (await import('../src/gateway/layerzero')).LayerZeroGatewayClient {
    _publicClient: any;
    constructor(chainId: number, l0Client: any, publicClient: any) {
        super(chainId);
        // @ts-ignore
        this.l0Client = l0Client;
        this._publicClient = publicClient;
    }
    // @ts-ignore
    get publicClient() {
        return this._publicClient;
    }
}
