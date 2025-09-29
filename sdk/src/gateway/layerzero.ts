import { Address, encodeAbiParameters, encodePacked, Hex, padHex, parseAbiParameters } from 'viem';
import { AllWalletClientParams, GatewayApiClient } from './client';
import { ExecuteQuoteParams, GetQuoteParams, OfframpExecuteQuoteParams } from './types';
import { bob, bobSepolia } from 'viem/chains';
import { toHexScriptPubKey } from './utils';
import * as bitcoin from 'bitcoinjs-lib';
import { viemClient } from './utils';
import { layerZeroOftAbi, quoterV2Abi } from './abi';
import ecc from '@bitcoinerlab/secp256k1';

bitcoin.initEccLib(ecc);

type SendParam = {
    dstEid: number;
    to: Hex;
    amountLD: bigint;
    minAmountLD: bigint;
    extraOptions: Hex;
    composeMsg: Hex;
    oftCmd: Hex;
};

type LayerZeroChainInfo = {
    name: string;
    eid?: string;
    oftAddress: string;
    nativeChainId?: number;
};

type LayerZeroDeploymentsMetadata = {
    [chainKey: string]: {
        deployments?: [
            {
                version: number;
                eid: string;
            },
        ];
        chainKey: string;
        chainDetails?: {
            nativeChainId: number;
        };
    };
};

type LayerZeroTokenDeployments = {
    [chainKey: string]: {
        address: string;
    };
};

interface LayerZeroQuoteParamsExt {
    /** @description Buffer in BPS to account for Bitcoin to BOB finality delay (30 mins+) when using the L0 Strategy */
    l0OriginFinalityBuffer?: number | bigint;
    /** @description Buffer in BPS to account for BOB to destination finality delay (a few minutes) when using the L0 Strategy */
    l0DestinationFinalityBuffer?: number | bigint;
}

interface LayerZeroQuoteExt {
    /** @description The expected amount of wBTC to be swapped to pay for L0 fees */
    estimatedDestinationL0Fee?: bigint;
    /** @description The maximum amount of wBTC that can be swapped to pay for L0 fees */
    maxAllocatedL0DestinationFee?: bigint;
}

export class LayerZeroClient {
    private basePath: string;

    private getChainDeploymentsPromiseCache: Promise<LayerZeroDeploymentsMetadata> | null = null;
    private getWbtcDeploymentsPromiseCache: Promise<LayerZeroTokenDeployments> | null = null;

    constructor() {
        this.basePath = 'https://metadata.layerzero-api.com/v1/metadata';
    }

    private async getChainDeployments() {
        if (!this.getChainDeploymentsPromiseCache) {
            this.getChainDeploymentsPromiseCache = this.getJson<LayerZeroDeploymentsMetadata>(`${this.basePath}`).catch(
                (err) => {
                    // On failure, clear the cache to allow retries on subsequent calls.
                    this.getChainDeploymentsPromiseCache = null;
                    throw err;
                }
            );
        }

        return this.getChainDeploymentsPromiseCache;
    }

    async getEidForChain(chainKey: string) {
        const data = await this.getChainDeployments();
        return data[chainKey]?.deployments?.find((item) => item.version === 2)?.eid || null;
    }

    private async getWbtcDeployments() {
        if (!this.getWbtcDeploymentsPromiseCache) {
            const params = new URLSearchParams({
                symbols: 'WBTC',
            });

            this.getWbtcDeploymentsPromiseCache = this.getJson<{
                WBTC: [{ deployments: LayerZeroTokenDeployments }];
            }>(`${this.basePath}/experiment/ofts/list?${params.toString()}`)
                .then((data) => {
                    return data.WBTC[0].deployments;
                })
                .catch((err) => {
                    // On failure, clear the cache to allow retries on subsequent calls.
                    this.getWbtcDeploymentsPromiseCache = null;
                    throw err;
                });
        }

        return this.getWbtcDeploymentsPromiseCache;
    }

    async getSupportedChains(): Promise<Array<string>> {
        const deployments = await this.getWbtcDeployments();
        return Object.keys(deployments);
    }

    async getOftAddressForChain(chainKey: string): Promise<string | null> {
        const deployments = await this.getWbtcDeployments();
        return deployments[chainKey]?.address || null;
    }

    async getSupportedChainsInfo(): Promise<Array<LayerZeroChainInfo>> {
        const chains = await this.getChainDeployments();
        const chainLookup = Object.fromEntries(
            Object.entries(chains).map(([_, chainData]) => [
                chainData.chainKey,
                {
                    eid: chainData.deployments?.find((item) => item.version === 2)?.eid,
                    nativeChainId: chainData.chainDetails?.nativeChainId,
                },
            ])
        );

        const deployments = await this.getWbtcDeployments();
        return Object.entries(deployments).map(([chainKey, deployment]) => {
            const chainInfo = chainLookup[chainKey];
            return {
                name: chainKey,
                eid: chainInfo?.eid,
                oftAddress: deployment.address,
                nativeChainId: chainInfo?.nativeChainId,
            };
        });
    }

    async getChainId(eid: string): Promise<number | null> {
        const chains = await this.getChainDeployments();

        const chainId = Object.values(chains).find((chain) =>
            chain.deployments?.some((deployment) => deployment.eid === eid)
        )?.chainDetails?.nativeChainId;

        if (chainId) {
            return chainId;
        }

        return null;
    }

    private async getJson<T>(url: string): Promise<T> {
        const response = await fetch(url);
        if (!response.ok) {
            throw new Error(response.statusText);
        }
        return (await response.json()) as Promise<T>;
    }
}

function resolveChainName(chain: number | string): string {
    if (typeof chain === 'number') {
        switch (chain) {
            case bob.id:
                return bob.name.toLowerCase();
            case bobSepolia.id:
                return bobSepolia.name.toLowerCase();
            default:
                throw new Error(`Unsupported chain ID: ${chain}`);
        }
    }
    return chain.toLowerCase();
}

// TODO: support bob sepolia
export class LayerZeroGatewayClient extends GatewayApiClient {
    private l0Client: LayerZeroClient;

    constructor(chainId: number, options?: { rpcUrl?: string }) {
        if (chainId !== bob.id) {
            throw new Error('LayerZeroGatewayClient only supports BOB mainnet');
        }
        super(chainId, options);
        this.l0Client = new LayerZeroClient();
    }

    async getSupportedChainsInfo(): Promise<Array<LayerZeroChainInfo>> {
        return this.l0Client.getSupportedChainsInfo();
    }

    /**
     * @deprecated Use getSupportedChainsInfo() instead
     */
    async getSupportedChains(): Promise<Array<string>> {
        return this.l0Client.getSupportedChains();
    }

    /**
     * @deprecated Use getSupportedChainsInfo() instead
     */
    async getEidForChain(chainKey: string): Promise<string | null> {
        return this.l0Client.getEidForChain(chainKey);
    }

    /**
     * Get the chain id for a given LayerZero EID
     * @param eid LayerZero EID
     * @returns chain id for the given eid if found
     */
    async getChainIdForEid(eid: string): Promise<number | null> {
        return this.l0Client.getChainId(eid);
    }

    /**
     * @deprecated Use getSupportedChainsInfo() instead
     */
    async getOftAddressForChain(chainKey: string): Promise<string | null> {
        return this.l0Client.getOftAddressForChain(chainKey);
    }

    async getQuote(params: GetQuoteParams<LayerZeroQuoteParamsExt>): Promise<ExecuteQuoteParams<LayerZeroQuoteExt>> {
        const fromChain = resolveChainName(params.fromChain);
        const toChain = resolveChainName(params.toChain);

        if (fromChain === 'bitcoin' && toChain === bob.name.toLowerCase()) {
            // Handle bitcoin -> bob: use normal flow
            return super.getQuote(params);
        } else if (fromChain === bob.name.toLowerCase() && toChain === 'bitcoin') {
            // Handle bob -> bitcoin: use normal flow
            return super.getQuote(params);
        } else if (fromChain === 'bitcoin') {
            const dstEid = await this.l0Client.getEidForChain(toChain);
            if (!dstEid) {
                throw new Error(`Unsupported destination chain: ${toChain}`);
            }

            // always use the wBTC token on BOB
            const wbtcOftAddress = await this.l0Client.getOftAddressForChain('bob');
            if (!wbtcOftAddress) {
                throw new Error(`WBTC OFT not found for chain: ${fromChain}`);
            }

            // TODO: Will need to generalize this if we want to support other option sets. Its manual ABI encoding so a little complex.
            const extraOptions = encodePacked(
                ['uint16', 'uint8', 'uint16', 'uint8', 'uint128'],
                [
                    // https://github.com/LayerZero-Labs/LayerZero-v2/blob/200cda254120375f40ed0a7e89931afb897b8891/packages/layerzero-v2/evm/oapp/contracts/oapp/libs/OptionsBuilder.sol#L22
                    3, // TYPE_3
                    // https://github.com/LayerZero-Labs/LayerZero-v2/blob/200cda254120375f40ed0a7e89931afb897b8891/packages/layerzero-v2/evm/messagelib/contracts/libs/ExecutorOptions.sol#L10
                    1, // WORKER_ID
                    17, // 16+1 for optionType
                    1, // OPTION_TYPE_LZRECEIVE
                    BigInt(65000),
                ]
            );

            const sendParam: SendParam = {
                dstEid: parseInt(dstEid!, 10),
                to: padHex(params.toUserAddress as Hex) as Hex,
                amountLD: BigInt(0), // will be added inside the strategy
                minAmountLD: BigInt(0), // will be added inside the strategyzz
                extraOptions: extraOptions,
                composeMsg: '0x' as Hex,
                oftCmd: '0x' as Hex,
            };

            // TODO: expose via params
            const publicClient = viemClient(bob);

            // We need to add buffers to fee calculations to account for gas price changes and slippage while waiting for these finalities.
            // There are two finalities we must consider:
            //  1) Bitcoin Finality on Bob (origin finality).
            //      This is 30 mins plus, therefore a large buffer is needed for values to remain valid over this period.
            //  2) Bob Finality on the destination chain (destination finality).
            //      This is much shorter, not more than a few minutes, therefore a smaller/zero buffer can be used.
            const originFinalityBuffer = params.l0OriginFinalityBuffer
                ? BigInt(params.l0OriginFinalityBuffer)
                : BigInt(10000); // 100% default origin finality buffer
            const destinationFinalityBuffer = params.l0DestinationFinalityBuffer
                ? BigInt(params.l0DestinationFinalityBuffer)
                : BigInt(0); // 0% default destination finality buffer

            // Getting the layer zero fee gas so we know how much we need to swap from the order
            const layerZeroFee = await publicClient.readContract({
                address: wbtcOftAddress as Hex,
                abi: layerZeroOftAbi,
                functionName: 'quoteSend',
                args: [sendParam, false],
            });

            // Estimating the amount of tokens we need to swap from the order by using the uniswap quoter
            const quote = await publicClient.readContract({
                address: '0x6Aa54a43d7eEF5b239a18eed3Af4877f46522BCA',
                abi: quoterV2Abi,
                functionName: 'quoteExactOutputSingle',
                args: [
                    {
                        tokenIn: wbtcOftAddress as Hex,
                        tokenOut: '0x4200000000000000000000000000000000000006' as Hex,
                        amountOut: layerZeroFee.nativeFee, // Desired output amount
                        fee: 3000,
                        sqrtPriceLimitX96: BigInt(0),
                    },
                ],
            });
            const tokensToSwapForLayerZeroFees = quote[0];
            // Adding the origin finality buffer to this to work out a safe maximum amount that will be swapped, if the fees required exceed this, the LayerZero strategy will revert.
            const maxTokensToSwapForLayerZeroFees =
                (tokensToSwapForLayerZeroFees * (10000n + originFinalityBuffer)) / 10000n;

            if (maxTokensToSwapForLayerZeroFees >= BigInt(params.amount)) {
                throw new Error(
                    `The maximum allocated LayerZero swap fee (${maxTokensToSwapForLayerZeroFees}) exceeds the order size (${params.amount}). Please increase the order size or wait for gas fees on the destination chain to decrease.`
                );
            }

            // https://github.com/LayerZero-Labs/LayerZero-v2/blob/200cda254120375f40ed0a7e89931afb897b8891/packages/layerzero-v2/evm/oapp/contracts/oft/interfaces/IOFT.sol#L10-L18
            const encodedParameters = encodeAbiParameters(
                parseAbiParameters([
                    '(uint32 dstEid, bytes32 to, uint256 amountLD, uint256 minAmountLD, bytes extraOptions, bytes composeMsg, bytes oftCmd)',
                    'uint256 buffer',
                    'uint256 maxTokensToSwap',
                ]),
                [sendParam, BigInt(destinationFinalityBuffer), BigInt(maxTokensToSwapForLayerZeroFees)]
            );

            // encode bob -> l0 chain calldata
            params.strategyAddress = '0x5Fd9B934c219663C7f4f432f39682be2dC42eDC7';
            params.message = encodedParameters;
            // change to BOB chain for bridging
            params.toChain = bob.id;

            // Handle bitcoin -> l0 chain: need to add calldata
            const baseQuote = await super.getQuote(params);
            return {
                ...baseQuote,
                estimatedDestinationL0Fee: tokensToSwapForLayerZeroFees,
                maxAllocatedL0DestinationFee: maxTokensToSwapForLayerZeroFees,
                totalFeeSats: baseQuote.totalFeeSats + Number(maxTokensToSwapForLayerZeroFees),
            };
        } else if (toChain === 'bitcoin') {
            params.fromChain = bob.id;
            // Handle l0 -> bitcoin: estimate bob -> bitcoin
            const response = await super.getQuote(params);
            // revert fromChain for handling in executeQuote
            response.params.fromChain = fromChain;
            return response;
        } else {
            throw new Error(`Unsupported chain combination: ${fromChain} -> ${toChain}`);
        }
    }

    async executeQuote({
        quote: executeQuoteParams,
        walletClient,
        publicClient,
        btcSigner,
    }: { quote: ExecuteQuoteParams } & AllWalletClientParams): Promise<string> {
        const isOfframpQuoteParams = (args: ExecuteQuoteParams): args is OfframpExecuteQuoteParams => {
            return args.params.toChain?.toString().toLowerCase() === 'bitcoin';
        };

        const fromChain = resolveChainName(executeQuoteParams.params.fromChain);
        const toChain = resolveChainName(executeQuoteParams.params.toChain);

        // Handle bitcoin -> bob / l0 chain, normal flow with additional calldata
        if (fromChain === 'bitcoin') {
            return super.executeQuote({ quote: executeQuoteParams, walletClient, publicClient, btcSigner });
        } else if (fromChain === bob.name.toLowerCase() && toChain === 'bitcoin') {
            // Handle bob -> bitcoin, normal flow
            return super.executeQuote({ quote: executeQuoteParams, walletClient, publicClient, btcSigner });
        } else if (isOfframpQuoteParams(executeQuoteParams)) {
            const { offrampQuote, params } = executeQuoteParams;
            const quote = offrampQuote!;

            const layerZeroClient = new LayerZeroClient();

            // The recipient address of the layer zero send, this contract will create the offramp order
            const offrampComposer = '0xc05AA3D7BD9c61B8b94EaCC937d1F542c3E5b94a';
            const receiverAddress = toHexScriptPubKey(params.toUserAddress, bitcoin.networks.bitcoin);

            const dstEid = await layerZeroClient.getEidForChain('bob');
            if (!dstEid) {
                throw new Error(`Destination EID not found for chain: ${fromChain}`);
            }

            const wbtcOftAddress = await layerZeroClient.getOftAddressForChain(fromChain);
            if (!wbtcOftAddress) {
                throw new Error(`WBTC OFT not found for chain: ${fromChain}`);
            }

            const extraOptions = encodePacked(
                ['uint16', 'uint8', 'uint16', 'uint8', 'uint128', 'uint8', 'uint16', 'uint8', 'uint16', 'uint128'],
                [
                    // https://github.com/LayerZero-Labs/LayerZero-v2/blob/200cda254120375f40ed0a7e89931afb897b8891/packages/layerzero-v2/evm/oapp/contracts/oapp/libs/OptionsBuilder.sol#L22
                    3, // TYPE_3
                    // https://github.com/LayerZero-Labs/LayerZero-v2/blob/200cda254120375f40ed0a7e89931afb897b8891/packages/layerzero-v2/evm/messagelib/contracts/libs/ExecutorOptions.sol#L10
                    1, // WORKER_ID
                    17, // 16+1 option length
                    1, // OPTION_TYPE_LZRECEIVE
                    BigInt(100000),
                    1, // WORKER_ID
                    19, // 18+1 option length
                    3, // OPTION_TYPE_LZCOMPOSE
                    0, // index for compose function
                    BigInt(300000), // gas limit for compose function
                ]
            );

            const sendParam: SendParam = {
                dstEid: parseInt(dstEid!, 10),
                to: padHex(offrampComposer, { size: 32 }),
                minAmountLD: BigInt(params.amount),
                amountLD: BigInt(params.amount),
                oftCmd: '0x',
                extraOptions: extraOptions,
                composeMsg: encodeAbiParameters(
                    parseAbiParameters([
                        '(uint256 satAmountToLock, uint256 satFeesMax, uint256 creationDeadline, bytes outputScript, address token, address owner)',
                    ]),
                    [
                        {
                            satAmountToLock: BigInt(quote.amountLockInSat),
                            satFeesMax: BigInt(quote.feeBreakdown.overallFeeSats),
                            creationDeadline: BigInt(quote.deadline),
                            outputScript: receiverAddress as Hex,
                            token: quote.token,
                            owner: params.fromUserAddress as Address,
                        },
                    ]
                ),
            };

            // we're quoting on the origin chain, so public client must be configured correctly
            const maybeFromChainId = executeQuoteParams.params.fromChain;
            if (typeof maybeFromChainId === 'number' && publicClient.chain?.id !== maybeFromChainId) {
                // avoid matching on name since L0 and viem may have different naming conventions
                throw new Error(`Public client must be origin chain`);
            }

            const sendFees = await publicClient.readContract({
                abi: layerZeroOftAbi,
                address: wbtcOftAddress as Hex,
                functionName: 'quoteSend',
                args: [sendParam, false],
            });

            const { request } = await publicClient.simulateContract({
                account: walletClient.account,
                abi: layerZeroOftAbi,
                address: wbtcOftAddress as Hex,
                functionName: 'send',
                args: [sendParam, sendFees, offrampComposer],
                value: sendFees.nativeFee,
            });

            const txHash = await walletClient.writeContract(request);
            await publicClient.waitForTransactionReceipt({ hash: txHash });

            return txHash;
        } else {
            throw new Error(`Unsupported chain combination: ${fromChain} -> ${toChain}`);
        }
    }
}
