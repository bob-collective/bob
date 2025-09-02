import { Address, encodeAbiParameters, encodeFunctionData, encodePacked, Hex, padHex, parseAbiParameters } from 'viem';
import { AllWalletClientParams, GatewayApiClient } from './client';
import {
    ExecuteQuoteParams,
    GatewayQuote,
    GatewayTokensInfo,
    GetQuoteParams,
    OfframpExecuteQuoteParams,
    OfframpQuote,
} from './types';
import { bob, bobSepolia } from 'viem/chains';
import oftAbi from './abis/OFT.abi';
import { offrampCaller } from './abi';
import { toHexScriptPubKey } from './utils';
import * as bitcoin from 'bitcoinjs-lib';
import { viemClient } from './utils';
import { layerZeroOftAbi, quoterV2Abi } from './abi';

const WBTC_OFT_ADDRESS: Address = '0x0555E30da8f98308EdB960aa94C0Db47230d2B9c';

type SendParam = {
    dstEid: number;
    to: Hex;
    amountLD: bigint;
    minAmountLD: bigint;
    extraOptions: Hex;
    composeMsg: Hex;
    oftCmd: Hex;
};

export class LayerZeroClient {
    private basePath: string;

    constructor() {
        this.basePath = 'https://metadata.layerzero-api.com/v1/metadata';
    }

    async getSupportedChains(): Promise<Array<string>> {
        const params = new URLSearchParams({
            symbols: 'WBTC',
        });

        const data = await this.getJson<{
            WBTC: [{ deployments: { [chainKey: string]: { address: string } } }];
        }>(`${this.basePath}/experiment/ofts/list?${params.toString()}`);

        return Object.keys(data.WBTC[0].deployments);
    }

    async getEidForChain(chainKey: string) {
        const data = await this.getJson<{
            [chainKey: string]: {
                deployments: [
                    {
                        version: number;
                        eid: string;
                    },
                ];
            };
        }>(`${this.basePath}`);

        return data[chainKey]?.deployments.find((item) => item.version === 2)?.eid || null;
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
        super(chainId, options);
        this.l0Client = new LayerZeroClient();
    }

    async getSupportedChains(): Promise<Array<string>> {
        return this.l0Client.getSupportedChains();
    }

    async getQuote(params: GetQuoteParams): Promise<{
        params: GetQuoteParams;
        onrampQuote?: GatewayQuote & GatewayTokensInfo;
        offrampQuote?: OfframpQuote;
    }> {
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

            // Getting the layer zero fee gas so we know how much we need to swap from the order
            const layerZeroFee = await publicClient.readContract({
                address: params.toToken as Hex,
                abi: layerZeroOftAbi,
                functionName: 'quoteSend',
                args: [sendParam, false],
            });

            const buffer = BigInt(500); // 5% buffer

            // Add buffer to the layer zero fee to account for changes from now until the order is executed
            const layerZeroFeeWithBuffer = (layerZeroFee.nativeFee * (10000n + buffer)) / 10000n; // 5% buffer

            // Getting the amount of tokens we need to swap from the order by using the uniswap quoter
            const quote = await publicClient.readContract({
                address: '0x6Aa54a43d7eEF5b239a18eed3Af4877f46522BCA',
                abi: quoterV2Abi,
                functionName: 'quoteExactOutputSingle',
                args: [
                    {
                        tokenIn: params.toToken as Hex,
                        tokenOut: '0x4200000000000000000000000000000000000006' as Hex,
                        amountOut: layerZeroFeeWithBuffer, // Desired output amount
                        fee: 3000,
                        sqrtPriceLimitX96: BigInt(0),
                    },
                ],
            });
            const tokensToSwapForLayerZeroFees = quote[0];
            // Adding a buffer to the swap amount to account for slippage
            const tokensToSwapForLayerZeroFeesWithBuffer = (tokensToSwapForLayerZeroFees * (10000n + buffer)) / 10000n; // 5% buffer

            // https://github.com/LayerZero-Labs/LayerZero-v2/blob/200cda254120375f40ed0a7e89931afb897b8891/packages/layerzero-v2/evm/oapp/contracts/oft/interfaces/IOFT.sol#L10-L18
            const encodedParameters = encodeAbiParameters(
                parseAbiParameters([
                    '(uint32 dstEid, bytes32 to, uint256 amountLD, uint256 minAmountLD, bytes extraOptions, bytes composeMsg, bytes oftCmd)',
                    'uint256 buffer',
                    'uint256 maxTokensToSwap',
                ]),
                [sendParam, BigInt(buffer), BigInt(tokensToSwapForLayerZeroFeesWithBuffer)]
            );

            // encode bob -> l0 chain calldata
            params.strategyAddress = '0x5Fd9B934c219663C7f4f432f39682be2dC42eDC7';
            params.message = encodedParameters;
            // change to BOB chain for bridging
            params.toChain = bob.id;

            // Handle bitcoin -> l0 chain: need to add calldata
            return super.getQuote(params);
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

            const dstEid = await this.l0Client.getEidForChain(toChain);
            if (!dstEid) {
                throw new Error(`Unsupported destination chain: ${toChain}`);
            }

            const recipient = params.toUserAddress as Address;
            const amount = BigInt(params.amount);

            const receiverAddress = toHexScriptPubKey(params.toUserAddress, bitcoin.networks.bitcoin);

            const sendParam: SendParam = {
                dstEid: parseInt(dstEid, 10),
                to: padHex(recipient, { size: 32 }),
                minAmountLD: amount,
                amountLD: amount,
                oftCmd: '0x',
                extraOptions: '0x', // TODO: add extra options for gas estimation
                composeMsg: encodeFunctionData({
                    abi: offrampCaller,
                    functionName: 'createOrder',
                    args: [
                        {
                            satAmountToLock: BigInt(quote.amountLockInSat),
                            satFeesMax: BigInt(quote.feeBreakdown.overallFeeSats),
                            orderCreationDeadline: BigInt(quote.deadline),
                            outputScript: receiverAddress as Hex,
                            token: quote.token,
                            orderOwner: params.toUserAddress as Address,
                        },
                    ],
                }),
            };

            const sendFees = await publicClient.readContract({
                abi: oftAbi,
                address: WBTC_OFT_ADDRESS, // TODO: may be different for other chains
                functionName: 'quoteSend',
                args: [sendParam, false],
            });

            const { request } = await publicClient.simulateContract({
                account: walletClient.account,
                abi: oftAbi,
                address: WBTC_OFT_ADDRESS,
                functionName: 'send',
                args: [sendParam, sendFees, recipient],
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
