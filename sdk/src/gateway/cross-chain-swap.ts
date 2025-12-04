import { bob } from 'viem/chains';
import { AllWalletClientParams } from './client';
import { ExecuteQuoteParams, GetQuoteParams } from './types/quote';
import { resolveChainId, getChainConfig } from './utils/common';
import { GatewayOrderType } from './types/order';
import { LayerZeroGatewayClient } from './layerzero';
import { GatewayApiClient } from './client';
import { CrossChainSwapQuoteParamsExt, OnrampWithSwapsExecuteQuoteParams, ActionsParams } from './types';
import { SwapsClient } from './swaps';
import { getTokenAddress } from './tokens';
import {
    isAddress,
    encodeAbiParameters,
    parseAbiParameters,
    encodeFunctionData,
    erc20Abi,
    type Address,
    type Hex,
} from 'viem';

export const MULTICALL_STRATEGY = '0x702405a5F314D0fDC2af516DF1e263f0Ce474E27';

export class CrossChainSwapGatewayClient extends LayerZeroGatewayClient {
    private swapsClient: SwapsClient;

    // TODO: remove constructor, set the config from `getQuote`
    constructor(options?: { rpcUrl?: string }) {
        super(options);
        this.swapsClient = new SwapsClient();
    }

    /**
     * Encodes the calldata for MulticallStrategy with approve and swap calls
     * @param swapTo The swap contract address
     * @param swapCalldata The swap transaction calldata
     * @param amountToApprove The amount to approve for the swap contract
     * @param tokenAddress The token address to approve (defaults to WBTC on BOB)
     * @returns Encoded calls array for MulticallStrategy.handleGatewayMessage
     */
    private encodeMulticallCalls(
        swapTo: Address,
        swapCalldata: Hex,
        amountToApprove: bigint,
        tokenAddress: Address = '0x0555E30da8f98308EdB960aa94C0Db47230d2B9c' as Address
    ): Hex {
        // Encode ERC20 approve call: approve(swapTo, amountToApprove)
        const approveCalldata = encodeFunctionData({
            abi: erc20Abi,
            functionName: 'approve',
            args: [swapTo, amountToApprove],
        });

        // Encode the calls array for MulticallStrategy
        // First call: approve the swap contract to spend the token
        // Second call: execute the swap
        // The Call struct is: struct Call { address target; bytes callData; uint256 value; }
        // MulticallStrategy.handleGatewayMessage expects: abi.decode(message, (Call[]))
        // So we encode as: (address,bytes,uint256)[]
        return encodeAbiParameters(parseAbiParameters(['(address,bytes,uint256)[]']), [
            [
                [tokenAddress, approveCalldata, 0n] as readonly [Address, Hex, bigint],
                [swapTo, swapCalldata, 0n] as readonly [Address, Hex, bigint],
            ],
        ]);
    }

    async getQuote(params: GetQuoteParams<CrossChainSwapQuoteParamsExt>): Promise<ExecuteQuoteParams> {
        const fromChain = typeof params.fromChain === 'number' ? resolveChainId(params.fromChain) : params.fromChain;
        const toChain = typeof params.toChain === 'number' ? resolveChainId(params.toChain) : params.toChain;

        if (
            fromChain === 'bitcoin' &&
            toChain === bob.name.toLowerCase() &&
            params.toToken === '0x0555E30da8f98308EdB960aa94C0Db47230d2B9c'
        ) {
            // Handle bitcoin -> wbtc on bob: use normal flow
            return super.getQuote(params);
        } else if (fromChain === bob.name.toLowerCase() && toChain === 'bitcoin') {
            // Handle wbtc on bob -> bitcoin: use normal flow
            return super.getQuote(params);
        } else if (fromChain === 'bitcoin' && toChain !== 'bitcoin') {
            // Handle cross chain swap (with onramp)
            // if (params.toToken && (await this.isChainAndTokenSupportedByLayerZero(toChain, params.toToken))) {
            //     // If toChain and toToken are supported by layerzero, use LayerZero flow
            //     return super.getQuote(params);
            // }

            // Otherwise use Swaps flow
            return this.getSwapsOnrampQuote(params);
        } else if (fromChain !== 'bitcoin' && toChain === 'bitcoin') {
            // Handle cross chain swap (with offramp)
            if (params.fromToken && (await this.isChainAndTokenSupportedByLayerZero(fromChain, params.fromToken))) {
                // If fromChain and fromToken are supported by layerzero, use LayerZero flow
                return super.getQuote(params);
            }
            // OfframpWithSwaps is not yet implemented
            throw new Error('OfframpWithSwaps is not yet implemented');
        } else {
            // Handle cross chain swap (evm to evm)
            if (
                params.fromToken &&
                params.toToken &&
                (await this.isChainAndTokenSupportedByLayerZero(fromChain, params.fromToken)) &&
                (await this.isChainAndTokenSupportedByLayerZero(toChain, params.toToken))
            ) {
                // If fromChain, fromToken, toChain, toToken are supported by layerzero, use LayerZero flow
                return super.getQuote(params);
            }
            // EVMToEVMWithSwaps is not yet implemented
            throw new Error('EVMToEVMWithSwaps is not yet implemented');
        }
    }

    private async getSwapsOnrampQuote(
        params: GetQuoteParams<CrossChainSwapQuoteParamsExt>
    ): Promise<OnrampWithSwapsExecuteQuoteParams> {
        // Resolve chain IDs
        // If fromChain is bitcoin, use bob as the source chain for Swaps API
        const fromChainResolved =
            typeof params.fromChain === 'number' ? resolveChainId(params.fromChain) : params.fromChain;
        const fromChainId =
            fromChainResolved === 'bitcoin'
                ? bob.id
                : typeof params.fromChain === 'number'
                  ? params.fromChain
                  : getChainConfig(params.fromChain).id;
        const toChainId = typeof params.toChain === 'number' ? params.toChain : getChainConfig(params.toChain).id;

        // Resolve token addresses
        if (!params.fromToken) {
            throw new Error('fromToken is required for Swaps API');
        }
        if (!params.toToken) {
            throw new Error('toToken is required for Swaps API');
        }

        // If fromToken is bitcoin, use WBTC on BOB
        const WBTC_ON_BOB = '0x0555E30da8f98308EdB960aa94C0Db47230d2B9c' as const;
        const srcToken =
            params.fromToken.toLowerCase() === 'bitcoin'
                ? (WBTC_ON_BOB as `0x${string}`)
                : isAddress(params.fromToken)
                  ? params.fromToken
                  : getTokenAddress(fromChainId, params.fromToken);
        const dstToken = isAddress(params.toToken) ? params.toToken : getTokenAddress(toChainId, params.toToken);

        // Validate required addresses
        if (!params.fromUserAddress) {
            throw new Error('fromUserAddress is required for Swaps API');
        }
        if (!params.toUserAddress) {
            throw new Error('toUserAddress is required for Swaps API');
        }

        // Convert amount to string
        const amount =
            typeof params.amount === 'bigint'
                ? params.amount.toString()
                : typeof params.amount === 'number'
                  ? params.amount.toString()
                  : params.amount;

        // Convert maxSlippage (0.01-0.03) to slippage (0-10000)
        // maxSlippage is a percentage (e.g., 0.03 = 3%)
        // slippage needs to be in basis points (0-10000, where 10000 = 100%)
        const slippage = params.maxSlippage
            ? Math.round(params.maxSlippage * 10000) // Convert percentage to basis points
            : 300; // Default 3% = 300 basis points

        // Construct ActionsParams
        const actionParams: ActionsParams = {
            actionType: 'swap-action',
            sender: MULTICALL_STRATEGY as Address,
            srcChainId: fromChainId,
            srcToken: srcToken,
            dstChainId: toChainId,
            dstToken: dstToken,
            slippage: slippage,
            amount: amount,
            swapDirection: 'exact-amount-in',
            recipient: params.toUserAddress as Address,
        };

        // Call Swaps API
        const actionResponse = await this.swapsClient.getAction(actionParams);

        const swapTo = actionResponse.tx.to;
        const swapCalldata = actionResponse.tx.data;

        // Encode the calls array for MulticallStrategy
        const encodedCalls = this.encodeMulticallCalls(
            swapTo as Address,
            swapCalldata as Hex,
            BigInt(amount),
            WBTC_ON_BOB as Address
        );

        // Set up params for onramp quote (similar to layerzero.ts line 427-433)
        params.strategyAddress = MULTICALL_STRATEGY as Address;
        params.message = encodedCalls;

        // For onramp flows (bitcoin -> other chain), change toChain to bob.id for the quote
        // The actual destination chain is handled by the swap transaction
        params.toChain = 'bob';
        params.toToken = '0x0555E30da8f98308EdB960aa94C0Db47230d2B9c';

        // Get the actual onramp quote from GatewayApiClient (skip LayerZeroGatewayClient)
        const baseQuote = await GatewayApiClient.prototype.getQuote.call(this, params);

        // Now refetch the Swap calldata using the finalOutputSats as the amount
        const actionParams2: ActionsParams = {
            ...actionParams,
            amount: baseQuote.finalOutputSats.toString(),
        };

        const actionResponse2 = await this.swapsClient.getAction(actionParams2);

        // Now encode the final swap calls array for MulticallStrategy
        const encodedCalls2 = this.encodeMulticallCalls(
            actionResponse2.tx.to as Address,
            actionResponse2.tx.data as Hex,
            BigInt(baseQuote.finalOutputSats),
            WBTC_ON_BOB as Address
        );

        // Construct SwapsExecuteQuoteParams
        return {
            params: {
                ...baseQuote.params,
                message: encodedCalls2,
            },
            type: GatewayOrderType.OnrampWithSwaps,
            finalOutputSats: baseQuote.finalOutputSats,
            finalFeeSats: baseQuote.finalFeeSats,
            data: baseQuote.data,
        };
    }

    async executeQuote({
        quote,
        walletClient,
        publicClient,
        btcSigner,
    }: { quote: ExecuteQuoteParams } & AllWalletClientParams): Promise<string> {
        switch (quote.type) {
            case GatewayOrderType.OnrampWithSwaps: {
                // Cast quote type to Onramp
                const onrampQuote = {
                    ...quote,
                    type: GatewayOrderType.Onramp,
                };
                return GatewayApiClient.prototype.executeQuote.call(this, {
                    quote: onrampQuote,
                    walletClient,
                    publicClient,
                    btcSigner,
                });
            }
            case GatewayOrderType.Offramp: {
                return GatewayApiClient.prototype.executeQuote.call(this, {
                    quote,
                    walletClient,
                    publicClient,
                    btcSigner,
                });
            }
            default: {
                return GatewayApiClient.prototype.executeQuote.call(this, {
                    quote,
                    walletClient,
                    publicClient,
                    btcSigner,
                });
            }
        }
    }
}
