import ecc from '@bitcoinerlab/secp256k1';
import * as bitcoin from 'bitcoinjs-lib';
import {
    Address,
    CallExecutionError,
    ContractFunctionExecutionError,
    encodeAbiParameters,
    encodePacked,
    erc20Abi,
    getAddress,
    Hex,
    InsufficientFundsError,
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
import { berachain, bob, bsc, mainnet, optimism, sei, soneium } from 'viem/chains';
import { layerZeroOftAbi, quoterV2Abi } from './abi';
import { AllWalletClientParams, GatewayApiClient } from './client';
import { getTokenAddress, getTokenSlots } from './tokens';
import {
    EVMToEVMWithLayerZeroOrder,
    EVMToEVMWithLayerZeroQuote,
    ExecuteQuoteParams,
    GatewayOrder,
    GatewayOrderType,
    GetQuoteParams,
    LayerZeroChainInfo,
    LayerZeroDeploymentsMetadataResponse,
    LayerZeroMessagesWalletResponse,
    LayerZeroSendParam,
    LayerZeroTokenDeploymentsResponse,
    CrossChainSwapQuoteParamsExt,
} from './types';
import { computeAllowanceSlot, computeBalanceSlot, getChainConfig, toHexScriptPubKey, viemClient } from './utils';
import { supportedChainsMapping, resolveChainId, resolveChainName } from './utils/common';
import { getEVMToEVMWithLayerZeroStatus } from './utils/layerzero';

bitcoin.initEccLib(ecc);

/**
 * L0 recipient that forwards messages to create offramp orders on-chain.
 */
export const OFFRAMP_COMPOSER = '0x7E6E65FaeB4ef08557928F78d71fa089a409299F';

/**
 * Strategy contract that handles layerzero cross chain swaps after the onramp is completed
 */
export const LAYERZERO_STRATEGY = '0x4572ce66cB33255B60a15e3c6cb2ef9c65A30ebC';

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

    // Resolve viem and layerzero chain names
    resolveViemChainName(chainKey: string): string {
        const chainNameMapping: Record<string, string> = {
            [bsc.name.toLowerCase()]: 'bsc',
            [optimism.name.toLowerCase()]: 'optimism',
            [sei.name.toLowerCase()]: 'sei',
            [soneium.name.toLowerCase()]: 'soneium',
            [berachain.name.toLowerCase()]: 'bera',
        };
        return chainNameMapping[chainKey.toLowerCase()] || chainKey;
    }

    async getEidForChain(chainKey: string) {
        const data = await this.getChainDeployments();
        const resolvedChainName = this.resolveViemChainName(chainKey);
        const eid = data[resolvedChainName]?.deployments?.find((item) => item.version === 2)?.eid;
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
        const resolvedChainName = this.resolveViemChainName(chainKey);
        return deployments[resolvedChainName]?.address || null;
    }

    async getSupportedChainsInfo(): Promise<Array<LayerZeroChainInfo>> {
        const [chains, deployments] = await Promise.all([this.getChainDeployments(), this.getWbtcDeployments()]);

        const supportedLayerZeroChainKeys = new Set<string>();
        const layerZeroKeyToViemName: Record<string, string> = {};

        Object.values(supportedChainsMapping).forEach((chainConfig) => {
            const viemChainName = chainConfig.name.toLowerCase();
            const layerZeroChainKey = this.resolveViemChainName(viemChainName);
            supportedLayerZeroChainKeys.add(layerZeroChainKey);
            layerZeroKeyToViemName[layerZeroChainKey] = viemChainName;
        });

        // Filter layerzero chains that are in supportedChainsMapping
        return Object.entries(chains)
            .filter(([layerZeroChainKey]) => supportedLayerZeroChainKeys.has(layerZeroChainKey))
            .filter(([layerZeroChainKey]) => deployments[layerZeroChainKey]?.address)
            .map(([layerZeroChainKey, chainData]) => {
                const viemChainName = layerZeroKeyToViemName[layerZeroChainKey];
                const deployment = deployments[layerZeroChainKey];

                return {
                    name: viemChainName,
                    eid: chainData.deployments?.find((item) => item.version === 2)?.eid,
                    oftAddress: deployment?.address,
                    nativeChainId: chainData.chainDetails?.nativeChainId,
                };
            });
    }

    async isChainAndTokenSupportedByLayerZero(chainKey: string, token: string): Promise<boolean> {
        const supportedChains = await this.getSupportedChainsInfo();

        // Find the chain info matching the chainKey (case-insensitive)
        const chainInfo = supportedChains.find((chain) => chain.name.toLowerCase() === chainKey.toLowerCase());

        if (!chainInfo) {
            return false;
        }

        // Token can either be the wbtc string, or the specific OFT address for wbtc on the chain
        const isWbtcToken = token.toLowerCase() === 'wbtc';
        const isOftAddress =
            isAddress(token) && isAddress(chainInfo.oftAddress)
                ? isAddressEqual(getAddress(token), getAddress(chainInfo.oftAddress))
                : false;

        return isWbtcToken || isOftAddress;
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

    // Where possible, the destination composer address should be a Create3 Singleton deployment so the address should be the same on all chains.
    async getDestinationComposerAddress(chainKey: string): Promise<Hex | null> {
        const allowed = ['bob', 'base', 'ethereum', 'sonic'];
        if (allowed.includes(chainKey.toLowerCase())) {
            return '0x814347a131B08679087F9A4842d493B1e788ea7a' as Hex;
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

// TODO: support bob sepolia
export class LayerZeroGatewayClient extends GatewayApiClient {
    private l0Client: LayerZeroClient;

    // TODO: remove constructor, set the config from `getQuote`
    constructor(options?: { rpcUrl?: string }) {
        super(bob.id, options);
        this.l0Client = new LayerZeroClient();
    }

    async getSupportedChainsInfo(): Promise<Array<LayerZeroChainInfo>> {
        return this.l0Client.getSupportedChainsInfo();
    }

    async isChainAndTokenSupportedByLayerZero(chainKey: string, token: string): Promise<boolean> {
        return this.l0Client.isChainAndTokenSupportedByLayerZero(chainKey, token);
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

    async getQuote(params: GetQuoteParams<CrossChainSwapQuoteParamsExt>): Promise<ExecuteQuoteParams> {
        const fromChain = typeof params.fromChain === 'number' ? resolveChainId(params.fromChain) : params.fromChain;
        const toChain = typeof params.toChain === 'number' ? resolveChainId(params.toChain) : params.toChain;

        if (fromChain === 'bitcoin' && toChain === bob.name.toLowerCase()) {
            // Handle bitcoin -> bob: use normal flow
            return super.getQuote(params);
        } else if (fromChain === bob.name.toLowerCase() && toChain === 'bitcoin') {
            // Handle bob -> bitcoin: use normal flow
            return super.getQuote(params);
        } else if (fromChain === 'bitcoin' && toChain !== 'bitcoin') {
            // Handle bitcoin -> l0 chain
            const dstEid = await this.l0Client.getEidForChain(toChain);
            if (!dstEid) {
                throw new Error(`Unsupported destination chain: ${toChain}`);
            }

            // always use the wBTC token on BOB
            const wbtcOftAddress = await this.l0Client.getOftAddressForChain('bob');
            if (!wbtcOftAddress) {
                throw new Error(`WBTC OFT not found for chain: ${fromChain}`);
            }

            let sendParam: LayerZeroSendParam;
            if (!params.destinationCalls && (params.message?.length === 0 || !params.message)) {
                // No destination calls, so we encode options for a standard L0 send with no compose message.
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

                sendParam = {
                    dstEid,
                    to: padHex(params.toUserAddress as Hex),
                    amountLD: BigInt(0), // will be added inside the strategy
                    minAmountLD: BigInt(0), // will be added inside the strategy
                    extraOptions: extraOptions,
                    composeMsg: '0x' as Hex,
                    oftCmd: '0x' as Hex,
                };
            } else {
                // There is a destination message, so we encode options to handle the compose message.

                // revert if both destinationCalls and message are provided
                if (params.destinationCalls && params.message) {
                    throw new Error('Both destinationCalls and message cannot be provided. Please provide only one.');
                }

                const destinationComposer = await this.l0Client.getDestinationComposerAddress(toChain);
                if (!destinationComposer) {
                    throw new Error(`Custom actions not supported for chain: ${toChain}`);
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
                        BigInt(params.destinationCalls?.gasLimit || 300000), // gas limit for compose function, default to 300k if not provided
                    ]
                );

                // encode calldata for the destination composer if destinationCalls are provided, otherwise use params.message
                const composeMsg = params.destinationCalls
                    ? encodeAbiParameters(parseAbiParameters(['address', '(address,bytes,uint256)[]']), [
                          params.destinationCalls.leftoverRecipient,
                          params.destinationCalls.calls.map((call): readonly [Address, Hex, bigint] => [
                              call.target,
                              call.callData,
                              call.value,
                          ]),
                      ])
                    : (params.message as Hex);

                sendParam = {
                    dstEid,
                    to: padHex(destinationComposer, { size: 32 }),
                    amountLD: BigInt(0), // will be added inside the strategy
                    minAmountLD: BigInt(0), // will be added inside the strategy
                    extraOptions: extraOptions,
                    composeMsg: composeMsg,
                    oftCmd: '0x' as Hex,
                };
            }

            // TODO: expose via params
            const publicClient = viemClient(bob);

            // We need to add buffers to fee calculations to account for gas price changes and slippage while waiting for these finalities.
            // There are two finalities we must consider:
            //  1) Bitcoin Finality on Bob (origin finality).
            //      This is 30 mins plus, therefore a large buffer is needed for values to remain valid over this period.
            //  2) Bob Finality on the destination chain (destination finality).
            //      This is much shorter, not more than a few minutes, therefore a smaller/zero buffer can be used.
            const originFinalityBuffer = params.originFinalityBuffer
                ? BigInt(params.originFinalityBuffer)
                : BigInt(10000); // 100% default origin finality buffer
            const destinationFinalityBuffer = params.destinationFinalityBuffer
                ? BigInt(params.destinationFinalityBuffer)
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
            params.strategyAddress = LAYERZERO_STRATEGY;
            params.message = encodedParameters;
            // change to BOB chain for bridging
            params.toChain = bob.id;

            // Handle bitcoin -> l0 chain: need to add calldata
            const baseQuote = await super.getQuote(params);
            // change the type to OnrampWithLayerZero
            baseQuote.type = GatewayOrderType.OnrampWithLayerZero;
            return {
                ...baseQuote,
                finalOutputSats: baseQuote.finalOutputSats - Number(tokensToSwapForLayerZeroFees),
                finalFeeSats: baseQuote.finalFeeSats + Number(tokensToSwapForLayerZeroFees),
            };
        } else if (fromChain !== 'bitcoin' && toChain === 'bitcoin') {
            const dstEid = await this.l0Client.getEidForChain(fromChain);

            if (!dstEid) {
                throw new Error(`Unsupported source chain: ${fromChain}`);
            }

            params.fromChain = bob.id;
            params.destinationChainId = await this.l0Client.getChainId(dstEid);

            // Handle l0 -> bitcoin: estimate bob -> bitcoin
            const response = await super.getQuote(params);
            // revert fromChain for handling in executeQuote
            response.params.fromChain = fromChain;
            // change the type to OfframpWithLayerZero
            response.type = GatewayOrderType.OfframpWithLayerZero;
            return response;
        } else if (fromChain !== 'bitcoin' && toChain !== 'bitcoin') {
            // Handle l0 -> l0 chain
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

            let sendParam: LayerZeroSendParam;
            if (!params.destinationCalls && (params.message?.length === 0 || !params.message)) {
                // No destination message, standard L0 send
                sendParam = {
                    dstEid,
                    to: padHex(params.toUserAddress as Hex),
                    amountLD: BigInt(params.amount),
                    minAmountLD: BigInt(params.amount),
                    extraOptions: '0x',
                    composeMsg: '0x',
                    oftCmd: '0x',
                };
            } else {
                // There is a destination message, so we encode options to handle the compose message.
                const destinationComposer = await this.l0Client.getDestinationComposerAddress(toChain);
                if (!destinationComposer) {
                    throw new Error(`Destination composer not found for chain: ${toChain}`);
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
                        BigInt(params.destinationCalls?.gasLimit || 300000), // gas limit for compose function, default to 300k if not provided
                    ]
                );

                // Encode destinationCalls if provided, otherwise use params.message
                const composeMsg = params.destinationCalls
                    ? encodeAbiParameters(parseAbiParameters(['address', '(address,bytes,uint256)[]']), [
                          params.destinationCalls.leftoverRecipient,
                          params.destinationCalls.calls.map((call): readonly [Address, Hex, bigint] => [
                              call.target,
                              call.callData,
                              call.value,
                          ]),
                      ])
                    : (params.message as Hex);

                sendParam = {
                    dstEid,
                    to: padHex(destinationComposer, { size: 32 }),
                    amountLD: BigInt(params.amount),
                    minAmountLD: BigInt(params.amount),
                    extraOptions: extraOptions,
                    composeMsg: composeMsg,
                    oftCmd: '0x' as Hex,
                };
            }

            const sendFees = await publicClient.readContract({
                abi: layerZeroOftAbi,
                address: wbtcOftAddress as Hex,
                functionName: 'quoteSend',
                args: [sendParam, false],
            });

            // const gasFee = await this.getL0CreateOrderGasCost(params, sendParam, sendFees, fromChain);

            return {
                type: GatewayOrderType.EVMToEVMWithLayerZero,
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
                        // gasFee,
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
                    to: padHex(OFFRAMP_COMPOSER, { size: 32 }),
                    minAmountLD: BigInt(params.amount),
                    amountLD: BigInt(params.amount),
                    oftCmd: '0x',
                    extraOptions: extraOptions,
                    composeMsg: encodeAbiParameters(
                        parseAbiParameters([
                            '(uint256 satAmountToLock, uint256 satSolverFeeMax, uint256 satAffiliateFee, address affiliateFeeRecipient, uint256 creationDeadline, bytes outputScript, address token, address owner)',
                        ]),
                        [
                            {
                                satAmountToLock: BigInt(data.amountLockInSat),
                                satSolverFeeMax: BigInt(data.feeBreakdown.overallFeeSats),
                                satAffiliateFee: BigInt(data.feeBreakdown.affiliateFeeSats),
                                affiliateFeeRecipient: data.affiliateFeeRecipient as Address,
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

                try {
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
                } catch (error) {
                    if (error instanceof ContractFunctionExecutionError) {
                        if (error.cause instanceof CallExecutionError) {
                            if (error.cause.cause instanceof InsufficientFundsError) {
                                // https://github.com/wevm/viem/blob/3aa882692d2c4af3f5e9cc152099e07cde28e551/src/actions/public/simulateContract.test.ts#L711
                                // throw new error
                                throw new Error(
                                    'Insufficient native funds for source and destination gas fees, please add more native funds to your account'
                                );
                            }
                        }
                    }

                    throw error;
                }
            }
            case GatewayOrderType.EVMToEVMWithLayerZero: {
                const { data, params } = quote;
                const { oftAddress, destinationEid } = data as EVMToEVMWithLayerZeroQuote;

                const toChain = resolveChainName(params.toChain);

                let sendParam: LayerZeroSendParam;
                if (!params.destinationCalls && (params.message?.length === 0 || !params.message)) {
                    // No destination message, standard L0 send
                    sendParam = {
                        dstEid: destinationEid,
                        to: padHex(params.toUserAddress as Hex),
                        amountLD: BigInt(params.amount),
                        minAmountLD: BigInt(params.amount),
                        extraOptions: '0x',
                        composeMsg: '0x',
                        oftCmd: '0x',
                    };
                } else {
                    // There is a destination message, so we encode options to handle the compose message.
                    const destinationComposer = await this.l0Client.getDestinationComposerAddress(toChain);
                    if (!destinationComposer) {
                        throw new Error(`Destination composer not found for chain: ${toChain}`);
                    }

                    const extraOptions = encodePacked(
                        [
                            'uint16',
                            'uint8',
                            'uint16',
                            'uint8',
                            'uint128',
                            'uint8',
                            'uint16',
                            'uint8',
                            'uint16',
                            'uint128',
                        ],
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
                            BigInt(params.destinationCalls?.gasLimit || 300000), // gas limit for compose function, default to 300k if not provided
                        ]
                    );

                    // Encode destinationCalls if provided, otherwise use params.message
                    const composeMsg = params.destinationCalls
                        ? encodeAbiParameters(parseAbiParameters(['address', '(address,bytes,uint256)[]']), [
                              params.destinationCalls.leftoverRecipient,
                              params.destinationCalls.calls.map((call): readonly [Address, Hex, bigint] => [
                                  call.target,
                                  call.callData,
                                  call.value,
                              ]),
                          ])
                        : (params.message as Hex);

                    sendParam = {
                        dstEid: destinationEid,
                        to: padHex(destinationComposer, { size: 32 }),
                        amountLD: BigInt(params.amount),
                        minAmountLD: BigInt(params.amount),
                        extraOptions: extraOptions,
                        composeMsg: composeMsg,
                        oftCmd: '0x' as Hex,
                    };
                }

                const sendFees = {
                    nativeFee: data.feeBreakdown.nativeFee,
                    lzTokenFee: data.feeBreakdown.lzTokenFee,
                };

                try {
                    const approvalRequired = await publicClient.readContract({
                        account: walletClient.account,
                        address: oftAddress,
                        abi: layerZeroOftAbi,
                        functionName: 'approvalRequired',
                    });

                    if (approvalRequired) {
                        const allowance = await publicClient.readContract({
                            account: walletClient.account,
                            address: oftAddress,
                            abi: erc20Abi,
                            functionName: 'allowance',
                            args: [params.fromUserAddress as Address, oftAddress],
                        });

                        if (allowance < sendParam.amountLD) {
                            const { request } = await publicClient.simulateContract({
                                account: walletClient.account,
                                address: oftAddress,
                                abi: erc20Abi,
                                functionName: 'approve',
                                args: [oftAddress, maxUint256],
                            });

                            const txHash = await walletClient.writeContract(request);

                            await publicClient.waitForTransactionReceipt({ hash: txHash });
                        }
                    }

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
                } catch (error) {
                    if (error instanceof ContractFunctionExecutionError) {
                        // https://github.com/wevm/viem/blob/3aa882692d2c4af3f5e9cc152099e07cde28e551/src/actions/public/simulateContract.test.ts#L711
                        // throw new error
                        throw new Error(
                            'Insufficient native funds for source and destination gas fees, please add more native funds to your account'
                        );
                    }

                    throw error;
                }
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
        params: GetQuoteParams<CrossChainSwapQuoteParamsExt>,
        sendParams: LayerZeroSendParam,
        sendFees: {
            nativeFee: bigint;
            lzTokenFee: bigint;
        },
        fromChain: string
    ): Promise<bigint> {
        const chain = getChainConfig(params.destinationChainId ?? params.fromChain);
        const publicClient = viemClient(chain);

        if (
            !params.fromUserAddress ||
            (isAddress(params.fromUserAddress) && isAddressEqual(params.fromUserAddress, zeroAddress))
        ) {
            params.fromUserAddress = '0x1111111111111111111111111111111111111111';
            params.toUserAddress = params.fromUserAddress;

            sendParams.to = padHex((params.fromUserAddress as Address) || zeroAddress, { size: 32 });
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
            const wbtcMainnetSlots = getTokenSlots(wbtcMainnetAddress);
            const allowanceSlot = computeAllowanceSlot(
                user as Address,
                wbtcOftAddress as Address,
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
            const wbtcOftSlots = getTokenSlots(wbtcOftAddress as Address);

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

    /**
     * Fetches evm-to-evm swap orders initiated by a given wallet.
     *
     * @param _userAddress - Wallet address the message originated from.
     * @returns Array of normalized evm-to-evm orders.
     */
    async getEVMToEVMWithLayerZeroOrders(_userAddress: Address): Promise<EVMToEVMWithLayerZeroOrder[]> {
        const url = new URL(`https://scan.layerzero-api.com/v1/messages/wallet/${_userAddress}`);

        const response = await super.safeFetch(url.toString(), undefined, 'Failed to fetch LayerZero send orders');

        if (!response.ok) {
            if (response.status === 404) {
                // Note: The API returns an error instead of an empty JSON if the address is not found
                console.warn('LayerZero send orders not found. Status:', response.status, response.statusText);
                return [];
            }

            // For all other errors: throw
            const errorData = await response.json().catch(() => null);
            throw new Error(errorData?.message || 'Failed to fetch LayerZero send orders');
        }

        const json: LayerZeroMessagesWalletResponse = await response.json();

        const items = json.data.filter((item) => item.destination.lzCompose.status === 'N/A');

        return Promise.all(
            items.map(async (item): Promise<EVMToEVMWithLayerZeroOrder> => {
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

                const [fromChain, toChain] = (await Promise.all([
                    this.getChainIdForEid(item.pathway.srcEid),
                    this.getChainIdForEid(item.pathway.dstEid),
                ])) as [number, number];

                return {
                    amount,
                    timestamp: blockTimestamp,
                    status: getEVMToEVMWithLayerZeroStatus(item),
                    source: {
                        chainId: fromChain,
                        txHash: sourceTxHash,
                        token: item.pathway.sender.address as Address,
                    },
                    destination: {
                        chainId: toChain,
                        txHash: destinationTxHash,
                        token: item.pathway.receiver.address as Address,
                    },
                };
            })
        );
    }

    /**
     * Retrieves all orders (onramp, offramp, and evm-to-evm swaps) for a specific user address.
     *
     * @param userAddress The user's EVM address
     * @returns Promise resolving to array of typed orders
     */
    async getOrders(userAddress: Address): Promise<Array<GatewayOrder>> {
        const [orders, evmToEVMWithLayerZeroOrders] = await Promise.all([
            super.getOrders(userAddress),
            this.getEVMToEVMWithLayerZeroOrders(userAddress),
        ]);

        return [
            ...orders,
            ...evmToEVMWithLayerZeroOrders.map((order) => ({
                type: GatewayOrderType.EVMToEVMWithLayerZero as const,
                order,
            })),
        ];
    }
}
