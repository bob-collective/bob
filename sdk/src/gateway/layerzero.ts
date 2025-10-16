import ecc from '@bitcoinerlab/secp256k1';
import * as bitcoin from 'bitcoinjs-lib';
import {
    Address,
    encodeAbiParameters,
    encodePacked,
    Hex,
    isAddress,
    isAddressEqual,
    maxUint256,
    padHex,
    parseAbiParameters,
    parseEther,
    StateOverride,
    toHex,
    zeroAddress,
} from 'viem';
import { bob, bobSepolia, mainnet } from 'viem/chains';
import { layerZeroOftAbi, quoterV2Abi } from './abi';
import { AllWalletClientParams, GatewayApiClient } from './client';
import { getTokenAddress, getTokenSlots } from './tokens';
import {
    CrossChainOrder,
    ExecuteQuoteParams,
    GatewayOrder,
    GatewayOrderType,
    GetQuoteParams,
    LayerZeroChainInfo,
    LayerZeroDeploymentsMetadataResponse,
    LayerZeroMessagesWalletResponse,
    LayerZeroQuoteParamsExt,
    LayerZeroSendParam,
    LayerZeroTokenDeploymentsResponse,
} from './types';
import {
    computeAllowanceSlot,
    computeBalanceSlot,
    getChainConfig,
    getCrossChainStatus,
    toHexScriptPubKey,
    viemClient,
} from './utils';

bitcoin.initEccLib(ecc);

export class LayerZeroClient {
    private basePath: string;

    private getChainDeploymentsPromiseCache: Promise<LayerZeroDeploymentsMetadataResponse> | null = null;
    private getWbtcDeploymentsPromiseCache: Promise<LayerZeroTokenDeploymentsResponse> | null = null;

    constructor() {
        this.basePath = 'https://metadata.layerzero-api.com/v1/metadata';
    }

    private async getChainDeployments() {
        if (!this.getChainDeploymentsPromiseCache) {
            this.getChainDeploymentsPromiseCache = this.getJson<LayerZeroDeploymentsMetadataResponse>(
                `${this.basePath}`
            ).catch((err) => {
                // On failure, clear the cache to allow retries on subsequent calls.
                this.getChainDeploymentsPromiseCache = null;
                throw err;
            });
        }

        return this.getChainDeploymentsPromiseCache;
    }

    async getEidForChain(chainKey: string) {
        const data = await this.getChainDeployments();
        const eid = data[chainKey]?.deployments?.find((item) => item.version === 2)?.eid;
        return eid !== undefined && eid !== null ? Number(eid) : null;
    }

    private async getWbtcDeployments() {
        if (!this.getWbtcDeploymentsPromiseCache) {
            const params = new URLSearchParams({
                symbols: 'WBTC',
            });

            this.getWbtcDeploymentsPromiseCache = this.getJson<{
                WBTC: [{ deployments: LayerZeroTokenDeploymentsResponse }];
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
            Object.entries(chains).map(([, chainData]) => [
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

    async getChainId(eid: number): Promise<number | null> {
        const chains = await this.getChainDeployments();

        const chainId = Object.values(chains).find((chain) =>
            chain.deployments?.some((deployment) => deployment.eid === eid.toString())
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
            case mainnet.id:
                return 'mainnet';
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
    async getEidForChain(chainKey: string): Promise<number | null> {
        return this.l0Client.getEidForChain(chainKey);
    }

    /**
     * Get the chain id for a given LayerZero EID
     * @param eid LayerZero EID
     * @returns chain id for the given eid if found
     */
    async getChainIdForEid(eid: number): Promise<number | null> {
        return this.l0Client.getChainId(eid);
    }

    /**
     * @deprecated Use getSupportedChainsInfo() instead
     */
    async getOftAddressForChain(chainKey: string): Promise<string | null> {
        return this.l0Client.getOftAddressForChain(chainKey);
    }

    async getQuote(params: GetQuoteParams<LayerZeroQuoteParamsExt>): Promise<ExecuteQuoteParams> {
        const fromChain = resolveChainName(params.fromChain);
        const toChain = resolveChainName(params.toChain);

        if (fromChain === 'bitcoin' && toChain === bob.name.toLowerCase()) {
            // Handle bitcoin -> bob: use normal flow
            return super.getQuote(params);
        } else if (fromChain === bob.name.toLowerCase() && toChain === 'bitcoin') {
            // Handle bob -> bitcoin: use normal flow
            return super.getQuote(params);
        } else if (fromChain === 'bitcoin' && toChain !== 'bitcoin') {
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

            const sendParam: LayerZeroSendParam = {
                dstEid,
                to: padHex(params.toUserAddress as Hex),
                amountLD: BigInt(0), // will be added inside the strategy
                minAmountLD: BigInt(0), // will be added inside the strategy
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
                finalOutputSats: baseQuote.finalOutputSats - Number(maxTokensToSwapForLayerZeroFees),
                finalFeeSats: baseQuote.finalFeeSats + Number(maxTokensToSwapForLayerZeroFees),
            };
        } else if (fromChain !== 'bitcoin' && toChain === 'bitcoin') {
            const dstEid = await this.l0Client.getEidForChain(fromChain);

            if (!dstEid) {
                throw new Error(`Unsupported source chain: ${fromChain}`);
            }

            params.fromChain = bob.id;
            params.l0ChainId = await this.l0Client.getChainId(dstEid);

            // Handle l0 -> bitcoin: estimate bob -> bitcoin
            const response = await super.getQuote(params);
            // revert fromChain for handling in executeQuote
            response.params.fromChain = fromChain;
            return response;
        } else if (fromChain !== 'bitcoin' && toChain !== 'bitcoin') {
            const dstEid = await this.l0Client.getEidForChain(toChain);
            if (!dstEid) {
                throw new Error(`Destination EID not found for chain: ${toChain}`);
            }
            const srcEid = await this.l0Client.getEidForChain(fromChain);
            if (!srcEid) {
                throw new Error(`Source EID not found for chain: ${fromChain}`);
            }

            const wbtcOftAddress = await this.l0Client.getOftAddressForChain(fromChain);
            if (!wbtcOftAddress) {
                throw new Error(`WBTC OFT not found for chain: ${fromChain}`);
            }

            // TODO: expose via params
            const chain = getChainConfig(params.fromChain);
            const publicClient = viemClient(chain);

            const sendParam: LayerZeroSendParam = {
                dstEid,
                to: padHex(params.toUserAddress as Hex),
                amountLD: BigInt(params.amount),
                minAmountLD: BigInt(params.amount),
                extraOptions: '0x',
                composeMsg: '0x',
                oftCmd: '0x',
            };

            const sendFees = await publicClient.readContract({
                abi: layerZeroOftAbi,
                address: wbtcOftAddress as Hex,
                functionName: 'quoteSend',
                args: [sendParam, false],
            });

            const gasFee = await this.getL0CreateOrderGasCost(params, sendParam, sendFees, fromChain);

            return {
                type: GatewayOrderType.CrossChainSwap,
                finalOutputSats: Number(params.amount),
                finalFeeSats: 0, // LayerZero sends don't have Bitcoin fees
                params,
                data: {
                    oftAddress: wbtcOftAddress as Hex,
                    destinationEid: dstEid,
                    sourceEid: srcEid,
                    feeBreakdown: {
                        nativeFee: sendFees.nativeFee,
                        lzTokenFee: sendFees.lzTokenFee,
                        gasFee: gasFee,
                    },
                },
            };
        }

        throw new Error(`Unsupported chain combination: ${fromChain} -> ${toChain}`);
    }

    async executeQuote({
        quote,
        walletClient,
        publicClient,
        btcSigner,
    }: { quote: ExecuteQuoteParams } & AllWalletClientParams): Promise<string> {
        switch (quote.type) {
            case GatewayOrderType.Onramp: {
                return super.executeQuote({ quote, walletClient, publicClient, btcSigner });
            }
            case GatewayOrderType.Offramp: {
                const fromChain = resolveChainName(quote.params.fromChain);

                if (fromChain === bob.name.toLowerCase()) {
                    // Handle bob -> bitcoin, normal flow
                    return super.executeQuote({ quote, walletClient, publicClient, btcSigner });
                }

                const { data, params } = quote;

                const layerZeroClient = new LayerZeroClient();

                // The recipient address of the layer zero send, this contract will create the offramp order
                const offrampComposer = '0xd455e08a6ecfac74e1a159fd3853ef14e6b99c7f';
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

                const sendParam: LayerZeroSendParam = {
                    dstEid,
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
                                satAmountToLock: BigInt(data.amountLockInSat),
                                satFeesMax: BigInt(data.feeBreakdown.overallFeeSats),
                                creationDeadline: BigInt(data.deadline),
                                outputScript: receiverAddress as Hex,
                                token: data.token,
                                owner: params.fromUserAddress as Address,
                            },
                        ]
                    ),
                };

                // we're quoting on the origin chain, so public client must be configured correctly
                const maybeFromChainId = quote.params.fromChain;
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
                    args: [sendParam, sendFees, params.fromUserAddress as Address],
                    value: sendFees.nativeFee,
                });

                const txHash = await walletClient.writeContract(request);
                await publicClient.waitForTransactionReceipt({ hash: txHash });

                return txHash;
            }
            case GatewayOrderType.CrossChainSwap: {
                const { data, params } = quote;
                const { oftAddress, destinationEid } = data;

                const sendParam: LayerZeroSendParam = {
                    dstEid: destinationEid,
                    to: padHex(params.toUserAddress as Hex) as Hex,
                    amountLD: BigInt(params.amount),
                    minAmountLD: BigInt(params.amount),
                    extraOptions: '0x',
                    composeMsg: '0x',
                    oftCmd: '0x',
                };

                const sendFees = {
                    nativeFee: data.feeBreakdown.nativeFee,
                    lzTokenFee: data.feeBreakdown.lzTokenFee,
                };

                const { request } = await publicClient.simulateContract({
                    account: walletClient.account,
                    abi: layerZeroOftAbi,
                    address: oftAddress,
                    functionName: 'send',
                    args: [sendParam, sendFees, params.fromUserAddress as Address],
                    value: sendFees.nativeFee,
                });

                const txHash = await walletClient.writeContract(request);
                await publicClient.waitForTransactionReceipt({ hash: txHash });

                return txHash;
            }
            default:
                // eslint-disable-next-line @typescript-eslint/no-explicit-any
                throw new Error(`Unsupported quote type: ${(quote as any).type}`);
        }
    }

    /**
     * Estimates the gas cost for creating an l0 order on-chain.
     *
     * This uses a state override to simulate max token allowance and balance for the user,
     * ensuring the gas estimate accounts for sufficient funds and approvals.
     *
     * @param params - Quote parameters containing user address and chain info
     * @param sendParams - LayerZero send params
     * @param sendFees - LayerZero `quoteSend` method results
     * @returns Promise resolving to the estimated gas cost in wei (as bigint)
     */
    async getL0CreateOrderGasCost(
        params: GetQuoteParams<LayerZeroQuoteParamsExt>,
        sendParams: LayerZeroSendParam,
        sendFees: {
            nativeFee: bigint;
            lzTokenFee: bigint;
        },
        fromChain: string
    ): Promise<bigint> {
        const chain = getChainConfig(params.l0ChainId ?? params.fromChain);
        const publicClient = viemClient(chain);

        if (
            !params.fromUserAddress ||
            (isAddress(params.fromUserAddress) && isAddressEqual(params.fromUserAddress, zeroAddress))
        ) {
            params.fromUserAddress = '0x1111111111111111111111111111111111111111';
            params.toUserAddress = params.fromUserAddress;

            sendParams.to = (params.fromUserAddress as Address) || zeroAddress;
        }

        const [feeValues, gasPrice, wbtcOftAddress] = await Promise.all([
            publicClient.estimateFeesPerGas(),
            publicClient.getGasPrice(),
            this.l0Client.getOftAddressForChain(fromChain),
        ]);

        const wbtcMainnetAddress = getTokenAddress(mainnet.id, 'wbtc');

        const fee = feeValues.maxFeePerGas ?? gasPrice;

        if (!wbtcOftAddress) {
            throw new Error(`WBTC OFT not found for chain: ${chain.id}`);
        }

        const user = params.fromUserAddress;

        const overrides: StateOverride = [];

        if (chain.id === mainnet.id) {
            // WBTC mainnet
            const wbtcMainnetSlots = getTokenSlots(wbtcMainnetAddress, 'ethereum');
            const allowanceSlot = computeAllowanceSlot(
                user as Address,
                wbtcMainnetAddress as Address,
                wbtcMainnetSlots.allowanceSlot
            );
            const balanceSlot = computeBalanceSlot(user as Address, wbtcMainnetSlots.balanceSlot);

            overrides.push({
                address: wbtcMainnetAddress,
                stateDiff: [
                    {
                        slot: allowanceSlot,
                        value: toHex(maxUint256),
                    },
                    {
                        slot: balanceSlot,
                        value: toHex(maxUint256),
                    },
                ],
            });
        } else {
            // WBTC OFT
            const wbtcOftSlots = getTokenSlots(wbtcOftAddress as Address, fromChain);

            const oftBalanceSlot = computeBalanceSlot(user as Address, wbtcOftSlots.balanceSlot);

            overrides.push({
                address: wbtcOftAddress as Address,
                stateDiff: [
                    {
                        slot: oftBalanceSlot,
                        value: toHex(maxUint256),
                    },
                ],
            });
        }

        const createOrderGasEstimate = await publicClient.estimateContractGas({
            address: wbtcOftAddress as Address,
            abi: layerZeroOftAbi,
            functionName: 'send',
            args: [sendParams, sendFees, user as Address],
            value: sendFees.nativeFee,
            account: user as Address,
            stateOverride: [
                ...overrides,
                // Ether balance override
                {
                    address: user as Address,
                    balance: parseEther('1'), // set 1 ETH for the user
                },
            ],
        });

        return createOrderGasEstimate * fee;
    }

    async getCrossChainSwapOrders(_userAddress: Address): Promise<CrossChainOrder[]> {
        const url = new URL(`https://scan.layerzero-api.com/v1/messages/wallet/${_userAddress}`);

        const response = await super.safeFetch(url.toString(), undefined, 'Failed to fetch LayerZero send orders');

        if (!response.ok) {
            const errorData = await response.json().catch(() => null);
            throw new Error(errorData?.message || 'Failed to fetch LayerZero send orders');
        }

        const json: LayerZeroMessagesWalletResponse = await response.json();

        const items = json.data.filter((item) => item.destination.lzCompose.status === 'N/A');

        return items.map((item): CrossChainOrder => {
            const { payload, blockTimestamp, txHash: sourceTxHash } = item.source.tx;
            const { txHash: destinationTxHash } = item.destination.tx;

            let amount = 0n;

            if (payload && typeof payload === 'string') {
                // LayerZero payload format: the order size is encoded in the last 8 bytes (16 hex chars)
                const hex = payload.startsWith('0x') ? payload.slice(2) : payload;
                // Validate minimum expected payload length
                if (hex.length >= 16 && hex.length % 2 === 0) {
                    const last16 = hex.slice(-16);
                    try {
                        amount = BigInt('0x' + last16);
                    } catch {
                        console.warn('Failed to parse order size from LayerZero payload');
                        amount = 0n;
                    }
                }
            }

            return {
                amount,
                timestamp: blockTimestamp,
                status: getCrossChainStatus(item),
                source: {
                    eid: item.pathway.srcEid,
                    txHash: sourceTxHash,
                    token: item.pathway.sender.address as Address,
                },
                destination: {
                    eid: item.pathway.dstEid,
                    txHash: destinationTxHash,
                    token: item.pathway.receiver.address as Address,
                },
            };
        });
    }

    /**
     * Retrieves all orders (onramp, offramp, and crosschain swaps) for a specific user address.
     *
     * @param userAddress The user's EVM address
     * @returns Promise resolving to array of typed orders
     */
    async getOrders(userAddress: Address): Promise<Array<GatewayOrder>> {
        const [orders, crossChainSwapOrders] = await Promise.all([
            super.getOrders(userAddress),
            this.getCrossChainSwapOrders(userAddress),
        ]);

        return [
            ...orders,
            ...crossChainSwapOrders.map((order) => ({ type: GatewayOrderType.CrossChainSwap as const, order })),
        ];
    }
}
