import { GatewayApiClient } from './client';
import { bob } from 'viem/chains';
import { SwingSDK } from '@swing.xyz/sdk';
import { Hex, zeroAddress } from 'viem';

interface SwingQuoteParams {
    fromChain: string;
    toChain: string;
    fromTokenAddress: Hex;
    toTokenAddress: Hex;
    fromTokenSymbol: string;
    toTokenSymbol: string;
    tokenAmount: string;
}

export class SwingClient {
    private swingSDK: SwingSDK;

    constructor() {
        this.swingSDK = new SwingSDK({
            projectId: 'bob-swap',
        });
    }

    async getQuote(params: SwingQuoteParams) {
        const quoteResponse = await this.swingSDK.crossChainAPI.GET('/v0/transfer/quote', {
            params: {
                query: {
                    fromChain: params.fromChain,
                    tokenSymbol: params.fromTokenSymbol,
                    fromTokenAddress: params.fromTokenAddress,
                    fromUserAddress: zeroAddress,
                    toChain: params.toChain,
                    toTokenSymbol: params.toTokenSymbol,
                    toTokenAddress: params.toTokenAddress,
                    toUserAddress: zeroAddress,
                    tokenAmount: params.tokenAmount,
                },
            },
        });

        if (!quoteResponse.response.ok) {
            throw new Error(quoteResponse.response.statusText);
        } else if (!quoteResponse.data) {
            throw new Error('No quote data');
        } else if (quoteResponse.data.routes.length === 0) {
            throw new Error('No routes found');
        }

        const transferRoute = quoteResponse.data.routes[0];

        const sendResponse = await this.swingSDK.crossChainAPI.POST('/v0/transfer/send', {
            body: {
                fromChain: params.fromChain,
                fromTokenAddress: params.fromTokenAddress,
                fromUserAddress: zeroAddress,
                tokenSymbol: params.fromTokenSymbol,
                toTokenAddress: params.toTokenAddress,
                toChain: params.toChain,
                toTokenAmount: transferRoute.quote.amount,
                toTokenSymbol: params.toTokenSymbol,
                toUserAddress: zeroAddress,
                tokenAmount: params.tokenAmount,
                route: transferRoute.route,
                skipValidation: 'true',
            },
        });

        return sendResponse.data;
    }
}

// TODO: support bob sepolia
export class SwingGatewayClient extends GatewayApiClient {
    private swingClient: SwingClient;

    constructor(chainId: number, options?: { rpcUrl?: string }) {
        if (chainId !== bob.id) {
            throw new Error('SwingGatewayClient only supports BOB mainnet');
        }
        super(chainId, options);
        this.swingClient = new SwingClient();
    }
}
