import { bob } from 'viem/chains';
import { AllWalletClientParams } from './client';
import { ExecuteQuoteParams, GetQuoteParams } from './types/quote';
import { resolveChainId, getChainConfig } from './utils/common';
import { GatewayOrderType } from './types/order';
import { LayerZeroGatewayClient } from './layerzero';
import {
    CrossChainSwapQuoteParamsExt,
    ActionsParams,
    EVMToEVMWithSwapsExecuteQuoteParams,
    OnrampWithSwapsExecuteQuoteParams,
    OfframpWithSwapsExecuteQuoteParams,
} from './types';
import { SwapsClient } from './swaps';
import { getTokenAddress } from './tokens';
import { isAddress } from 'viem';

export class CrossChainSwapGatewayClient extends LayerZeroGatewayClient {
    private swapsClient: SwapsClient;

    // TODO: remove constructor, set the config from `getQuote`
    constructor(options?: { rpcUrl?: string }) {
        super(options);
        this.swapsClient = new SwapsClient();
    }

    async getQuote(params: GetQuoteParams<CrossChainSwapQuoteParamsExt>): Promise<ExecuteQuoteParams> {
        const fromChain = typeof params.fromChain === 'number' ? resolveChainId(params.fromChain) : params.fromChain;
        const toChain = typeof params.toChain === 'number' ? resolveChainId(params.toChain) : params.toChain;

        if (fromChain === 'bitcoin' && toChain === bob.name.toLowerCase()) {
            // Handle bitcoin -> wbtc on bob: use normal flow
            return super.getQuote(params);
        } else if (fromChain === bob.name.toLowerCase() && toChain === 'bitcoin') {
            // Handle wbtc on bob -> bitcoin: use normal flow
            return super.getQuote(params);
        } else if (fromChain === 'bitcoin' && toChain !== 'bitcoin') {
            // Handle cross chain swap (with onramp)
            if (params.toToken && (await this.isChainAndTokenSupportedByLayerZero(toChain, params.toToken))) {
                // If toChain and toToken are supported by layerzero, use LayerZero flow
                return super.getQuote(params);
            }
            // Otherwise use Swaps flow
            return this.getSwapsQuote(params, GatewayOrderType.OnrampWithSwaps);
        } else if (fromChain !== 'bitcoin' && toChain === 'bitcoin') {
            // Handle cross chain swap (with offramp)
            if (params.fromToken && (await this.isChainAndTokenSupportedByLayerZero(fromChain, params.fromToken))) {
                // If fromChain and fromToken are supported by layerzero, use LayerZero flow
                return super.getQuote(params);
            }
            // Otherwise use Swaps flow
            return this.getSwapsQuote(params, GatewayOrderType.OfframpWithSwaps);
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
            // Otherwise use Swaps flow
            return this.getSwapsQuote(params, GatewayOrderType.EVMToEVMWithSwaps);
        }
    }

    private async getSwapsQuote(
        params: GetQuoteParams<CrossChainSwapQuoteParamsExt>,
        orderType:
            | GatewayOrderType.EVMToEVMWithSwaps
            | GatewayOrderType.OnrampWithSwaps
            | GatewayOrderType.OfframpWithSwaps
    ): Promise<
        EVMToEVMWithSwapsExecuteQuoteParams | OnrampWithSwapsExecuteQuoteParams | OfframpWithSwapsExecuteQuoteParams
    > {
        // Resolve chain IDs
        const fromChainId =
            typeof params.fromChain === 'number' ? params.fromChain : getChainConfig(params.fromChain).id;
        const toChainId = typeof params.toChain === 'number' ? params.toChain : getChainConfig(params.toChain).id;

        // Resolve token addresses
        if (!params.fromToken) {
            throw new Error('fromToken is required for Swaps API');
        }
        if (!params.toToken) {
            throw new Error('toToken is required for Swaps API');
        }

        const srcToken = isAddress(params.fromToken)
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
            sender: params.fromUserAddress as `0x${string}`,
            srcChainId: fromChainId,
            srcToken: srcToken,
            dstChainId: toChainId,
            dstToken: dstToken,
            slippage: slippage,
            amount: amount,
            swapDirection: 'exact-amount-in',
            recipient: params.toUserAddress as `0x${string}`,
        };

        // Call Swaps API
        const actionResponse = await this.swapsClient.getAction(actionParams);

        // Construct SwapsExecuteQuoteParams
        // Note: finalOutputSats and finalFeeSats are Bitcoin-related fields
        // For Swaps, we'll set them to 0 or calculate from the response if needed
        return {
            type: orderType,
            finalOutputSats: 0, // TODO: Calculate from actionResponse if needed
            finalFeeSats: 0, // TODO: Calculate from actionResponse if needed
            params: params,
            data: {
                actionResponse,
                actionParams,
                baseToken: undefined, // TODO: Resolve from token info if needed
                outputToken: undefined, // TODO: Resolve from token info if needed
            },
        };
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
                return super.executeQuote({ quote, walletClient, publicClient, btcSigner });
            }
            default: {
                return super.executeQuote({ quote, walletClient, publicClient, btcSigner });
            }
        }
    }
}
