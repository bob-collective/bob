import { bob } from 'viem/chains';
import { AllWalletClientParams } from './client';
import { ExecuteQuoteParams, GetQuoteParams } from './types/quote';
import { resolveChainId } from './utils/common';
import { GatewayOrderType } from './types/order';
import { LayerZeroGatewayClient } from './layerzero';
import { LayerZeroQuoteParamsExt } from './types/layerzero';

export class CrossChainSwapGatewayClient extends LayerZeroGatewayClient {
    // TODO: remove constructor, set the config from `getQuote`
    constructor(options?: { rpcUrl?: string }) {
        super(options);
    }

    async getQuote(params: GetQuoteParams<LayerZeroQuoteParamsExt>): Promise<ExecuteQuoteParams> {
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
            // Otherwise use Swaps flow (not implemented yet)
            throw new Error(`Unsupported chain combination: ${fromChain} -> ${toChain}`);
        } else if (fromChain !== 'bitcoin' && toChain === 'bitcoin') {
            // Handle cross chain swap (with offramp)
            if (params.fromToken && (await this.isChainAndTokenSupportedByLayerZero(fromChain, params.fromToken))) {
                // If fromChain and fromToken are supported by layerzero, use LayerZero flow
                return super.getQuote(params);
            }
            // Otherwise use Swaps flow (not implemented yet)
            throw new Error(`Unsupported chain combination: ${fromChain} -> ${toChain}`);
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
            // Otherwise use Swaps flow (not implemented yet)
            throw new Error(`Unsupported chain combination: ${fromChain} -> ${toChain}`);
        }
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
