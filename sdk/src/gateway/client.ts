import { ethers, AbiCoder } from "ethers";
import { GatewayQuoteParams } from "./types";
import { TOKENS } from "./tokens";

type EvmAddress = string;

type GatewayQuote = {
    gatewayAddress: EvmAddress;
    dustThreshold: number;
    satoshis: number;
    fee: number;
    gratuity: string;
    bitcoinAddress: string;
    txProofDifficultyFactor: number;
    strategyAddress: EvmAddress | null,
};

type GatewayCreateOrderRequest = {
    gatewayAddress: EvmAddress,
    strategyAddress: EvmAddress | null,
    satsToConvertToEth: number,
    userAddress: EvmAddress,
    gatewayExtraData: string | null,
    strategyExtraData: string | null,
    satoshis: number,
};

type GatewayOrderResponse = {
    gatewayAddress: EvmAddress;
    tokenAddress: EvmAddress;
    txid: string;
    status: boolean;
    timestamp: number;
    tokens: string;
    satoshis: number;
    fee: number;
    txProofDifficultyFactor: number;
};

type GatewayCreateOrderResponse = {
    uuid: string,
    opReturnHash: string,
};

type GatewayStartOrderResult = GatewayCreateOrderResponse & {
    bitcoinAddress: string,
    satoshis: number;
};

/**
 * Base url for the mainnet Gateway API.
 * @default "https://gateway-api-mainnet.gobob.xyz"
 */
export const MAINNET_GATEWAY_BASE_URL = "https://gateway-api-mainnet.gobob.xyz";

/**
 * Base url for the testnet Gateway API.
 * @default "https://gateway-api-testnet.gobob.xyz"
 */
export const TESTNET_GATEWAY_BASE_URL = "https://gateway-api-testnet.gobob.xyz";

export class GatewayApiClient {
    private baseUrl: string;

    constructor(networkOrUrl: string = "mainnet") {
        switch (networkOrUrl) {
            case "mainnet" || "bob":
                this.baseUrl = MAINNET_GATEWAY_BASE_URL;
                break;
            case "testnet" || "bob-sepolia":
                this.baseUrl = TESTNET_GATEWAY_BASE_URL;
                break;
            default:
                this.baseUrl = networkOrUrl;
        }
    }

    async getQuote(params: GatewayQuoteParams): Promise<GatewayQuote> {
        const isMainnet = params.toChain == "bob" || params.toChain == 60808;

        let outputToken = "";
        if (params.toToken.startsWith("0x")) {
            outputToken = params.toToken;
        } else if (params.toToken in TOKENS) {
            outputToken = isMainnet ? TOKENS[params.toToken].bob : TOKENS[params.toToken].bob_sepolia;
        } else {
            throw new Error('Unknown output token');
        }

        const atomicAmount = params.amount;
        const response = await fetch(`${this.baseUrl}/quote/${outputToken}/${atomicAmount || ''}`, {
            headers: {
                'Content-Type': 'application/json',
                Accept: 'application/json'
            }
        });

        return await response.json();
    }

    // TODO: add error handling
    async startOrder(gatewayQuote: GatewayQuote, params: GatewayQuoteParams): Promise<GatewayStartOrderResult> {
        const request: GatewayCreateOrderRequest = {
            gatewayAddress: gatewayQuote.gatewayAddress,
            strategyAddress: gatewayQuote.strategyAddress,
            satsToConvertToEth: params.gasRefill,
            userAddress: params.toUserAddress,
            // TODO: figure out how to get extra data
            gatewayExtraData: null,
            strategyExtraData: null,
            satoshis: gatewayQuote.satoshis,
        };

        const response = await fetch(`${this.baseUrl}/order`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                Accept: 'application/json'
            },
            body: JSON.stringify(request)
        });

        if (!response.ok) {
            throw new Error('Failed to create order');
        }

        const data: GatewayCreateOrderResponse = await response.json();
        // NOTE: could remove this check but good for sanity
        if (data.opReturnHash != calculateOpReturnHash(request)) {
            throw new Error('Invalid OP_RETURN hash');
        }

        return {
            uuid: data.uuid,
            opReturnHash: data.opReturnHash,
            bitcoinAddress: gatewayQuote.bitcoinAddress,
            satoshis: gatewayQuote.satoshis,
        }
    }

    async finalizeOrder(orderUuid: string, bitcoinTx: string) {
        const response = await fetch(`${this.baseUrl}/order/${orderUuid}`, {
            method: 'PATCH',
            headers: {
                'Content-Type': 'application/json',
                Accept: 'application/json'
            },
            body: JSON.stringify({ bitcoinTx: bitcoinTx })
        });

        if (!response.ok) {
            throw new Error('Failed to update order');
        }
    }

    async getOrders(userAddress: EvmAddress): Promise<GatewayOrderResponse[]> {
        const response = await fetch(`${this.baseUrl}/orders/${userAddress}`, {
            method: 'GET',
            headers: {
                'Content-Type': 'application/json',
                Accept: 'application/json'
            }
        });

        return response.json();
    }

    async getTokens(): Promise<EvmAddress[]> {
        const response = await fetch(`${this.baseUrl}/tokens`, {
            method: 'GET',
            headers: {
                'Content-Type': 'application/json',
                Accept: 'application/json'
            }
        });

        return response.json();
    }
}

function calculateOpReturnHash(req: GatewayCreateOrderRequest) {
    const abiCoder = new AbiCoder();
    return ethers.keccak256(abiCoder.encode(
        ["address", "address", "uint256", "address", "bytes", "bytes"],
        [
            req.gatewayAddress,
            req.strategyAddress || ethers.ZeroAddress,
            req.satsToConvertToEth,
            req.userAddress,
            req.gatewayExtraData,
            req.strategyExtraData
        ]
    ))
}