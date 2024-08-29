import { ethers, AbiCoder } from "ethers";
import { GatewayQuoteParams, Chain, ChainId, Token, GatewayStrategyContract } from "./types";
import { SYMBOL_LOOKUP, ADDRESS_LOOKUP } from "./tokens";
import { createBitcoinPsbt } from "../wallet";

type EvmAddress = string;

type GatewayQuote = {
    /** @description The gateway address */
    gatewayAddress: EvmAddress;
    /** @description The minimum amount of Bitcoin to send */
    dustThreshold: number;
    /** @description The satoshi output amount */
    satoshis: number;
    /** @description The fee paid in satoshis (includes gas refill) */
    fee: number;
    /** @description The Bitcoin address to send BTC */
    bitcoinAddress: string;
    /** @description The number of confirmations required to confirm the Bitcoin tx */
    txProofDifficultyFactor: number;
    /** @description The optional strategy address */
    strategyAddress?: EvmAddress,
};

/** @dev Internal request type used to call the Gateway API */
type GatewayCreateOrderRequest = {
    gatewayAddress: EvmAddress,
    strategyAddress?: EvmAddress,
    satsToConvertToEth: number,
    userAddress: EvmAddress,
    gatewayExtraData?: string,
    strategyExtraData?: string,
    satoshis: number,
};

type GatewayOrderResponse = {
    /** @description The gateway address */
    gatewayAddress: EvmAddress;
    /** @description The token address */
    tokenAddress: EvmAddress;
    /** @description The Bitcoin txid */
    txid: string;
    /** @description True when the order was executed on BOB */
    status: boolean;
    /** @description When the order was created */
    timestamp: number;
    /** @description The converted satoshi amount */
    tokens: string;
    /** @description The satoshi output amount */
    satoshis: number;
    /** @description The fee paid in satoshis (includes gas refill) */
    fee: number;
    /** @description The number of confirmations required to confirm the Bitcoin tx */
    txProofDifficultyFactor: number;
    /** @description The optional strategy address */
    strategyAddress?: EvmAddress,
    /** @description The gas refill in satoshis */
    satsToConvertToEth: number,
};

/** Order given by the Gateway API once the bitcoin tx is submitted */
type GatewayOrder = Omit<GatewayOrderResponse & {
    /** @description The gas refill in satoshis */
    gasRefill: number,
}, "satsToConvertToEth">;

type GatewayCreateOrderResponse = {
    uuid: string,
    opReturnHash: string,
};

/** @dev The success type on create order */
type GatewayStartOrderResult = GatewayCreateOrderResponse & {
    bitcoinAddress: string,
    satoshis: number;
    psbtBase64?: string;
};

interface GatewayStrategy {
    strategyAddress: string,
    strategyName: string,
    strategyType: "staking" | "lending",
    inputTokenAddress: string,
    outputTokenAddress?: string,
}

type Optional<T, K extends keyof T> = Omit<T, K> & Partial<T>;

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

/**
 * Gateway REST HTTP API client 
 */
export class GatewayApiClient {
    private chain: Chain.BOB | Chain.BOB_SEPOLIA;
    private baseUrl: string;

    /**
     * @constructor
     * @param chainName The chain name.
     */
    constructor(chainName: string) {
        switch (chainName) {
            case "mainnet":
            case Chain.BOB:
                this.chain = Chain.BOB;
                this.baseUrl = MAINNET_GATEWAY_BASE_URL;
                break;
            case "testnet":
            case Chain.BOB_SEPOLIA:
                this.chain = Chain.BOB_SEPOLIA;
                this.baseUrl = TESTNET_GATEWAY_BASE_URL;
                break;
            default:
                throw new Error("Invalid chain");
        }
    }

    /**
     * Returns all chains supported by the SDK.
     * 
     * @returns {string[]} The array of chain names.
     */
    getChains(): string[] {
        return Object.values(Chain);
    }

    /**
     * Get a quote from the Gateway API for swapping or staking BTC.
     * 
     * @param params The parameters for the quote.
     */
    async getQuote(params: Optional<GatewayQuoteParams, "amount" | "fromChain" | "fromToken" | "fromUserAddress" | "toUserAddress">): Promise<GatewayQuote> {
        const isMainnet = params.toChain === ChainId.BOB || typeof params.toChain === "string" && params.toChain.toLowerCase() === Chain.BOB;
        const isTestnet = params.toChain === ChainId.BOB_SEPOLIA || typeof params.toChain === "string" && params.toChain.toLowerCase() === Chain.BOB_SEPOLIA;

        const isInvalidNetwork = (!isMainnet && !isTestnet);
        const isMismatchMainnet = (isMainnet && this.chain !== Chain.BOB);
        const isMismatchTestnet = (isTestnet && this.chain !== Chain.BOB_SEPOLIA);

        // TODO: switch URL if `toChain` is different chain
        if (isInvalidNetwork || isMismatchMainnet || isMismatchTestnet) {
            throw new Error('Invalid output chain');
        }

        const toToken = params.toToken.toLowerCase();
        let outputToken = "";
        if (toToken.startsWith("0x")) {
            // NOTE: this can also be the strategy address
            outputToken = toToken;
        } else if (isMainnet && this.chain === Chain.BOB && SYMBOL_LOOKUP[ChainId.BOB][toToken]) {
            outputToken = SYMBOL_LOOKUP[ChainId.BOB][toToken].address;
        } else if (isTestnet && this.chain === Chain.BOB_SEPOLIA && SYMBOL_LOOKUP[ChainId.BOB_SEPOLIA][toToken]) {
            outputToken = SYMBOL_LOOKUP[ChainId.BOB_SEPOLIA][toToken].address;
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
    /**
     * Start an order via the Gateway API to reserve liquidity. This is step 1 of 2, see the {@link finalizeOrder} method.
     * 
     * @param gatewayQuote The quote given by the {@link getQuote} method.
     * @param params The parameters for the quote, same as before.
     * @returns {Promise<GatewayStartOrderResult>} The success object.
     */
    async startOrder(gatewayQuote: GatewayQuote, params: GatewayQuoteParams): Promise<GatewayStartOrderResult> {
        const request: GatewayCreateOrderRequest = {
            gatewayAddress: gatewayQuote.gatewayAddress,
            strategyAddress: gatewayQuote.strategyAddress,
            satsToConvertToEth: params.gasRefill || 0,
            userAddress: params.toUserAddress,
            // TODO: figure out how to get extra data
            gatewayExtraData: undefined,
            strategyExtraData: undefined,
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

        let psbtBase64: string;
        if (params.fromUserAddress && typeof params.fromChain === "string" && params.fromChain.toLowerCase() === Chain.BITCOIN) {
            psbtBase64 = await createBitcoinPsbt(
                params.fromUserAddress,
                gatewayQuote.bitcoinAddress,
                gatewayQuote.satoshis,
                params.fromUserPublicKey,
                data.opReturnHash,
                gatewayQuote.txProofDifficultyFactor
            );
        }

        return {
            uuid: data.uuid,
            opReturnHash: data.opReturnHash,
            bitcoinAddress: gatewayQuote.bitcoinAddress,
            satoshis: gatewayQuote.satoshis,
            psbtBase64,
        }
    }

    /**
     * Finalize an order via the Gateway API by providing the Bitcoin transaction. The tx will
     * be validated for correctness and forwarded to the mempool so there is no need to separately
     * broadcast the transaction. This is step 2 of 2, see the {@link startOrder} method.
     * 
     * @param uuid The id given by the {@link startOrder} method.
     * @param bitcoinTxHex The hex encoded Bitcoin transaction.
     */
    async finalizeOrder(uuid: string, bitcoinTxHex: string) {
        const response = await fetch(`${this.baseUrl}/order/${uuid}`, {
            method: 'PATCH',
            headers: {
                'Content-Type': 'application/json',
                Accept: 'application/json'
            },
            body: JSON.stringify({ bitcoinTx: bitcoinTxHex })
        });

        if (!response.ok) {
            throw new Error('Failed to update order');
        }
    }

    /**
     * Returns all pending and completed orders for this account.
     * 
     * @param userAddress The user's EVM address.
     * @returns {Promise<GatewayOrder[]>} The array of account orders.
     */
    async getOrders(userAddress: EvmAddress): Promise<GatewayOrder[]> {
        const response = await fetch(`${this.baseUrl}/orders/${userAddress}`, {
            method: 'GET',
            headers: {
                'Content-Type': 'application/json',
                Accept: 'application/json'
            }
        });

        const orders: GatewayOrderResponse[] = await response.json();
        return orders.map(order => { return { gasRefill: order.satsToConvertToEth, ...order } });
    }

    /**
     * Returns all strategies supported by the Gateway API.
     * 
     * @returns {Promise<GatewayStrategyContract[]>} The array of strategies.
     */
    async getStrategies(): Promise<GatewayStrategyContract[]> {
        const response = await fetch(`${this.baseUrl}/strategies`, {
            method: 'GET',
            headers: {
                'Content-Type': 'application/json',
                Accept: 'application/json'
            }
        });

        const chainName = (this.chain === Chain.BOB ? Chain.BOB : Chain.BOB_SEPOLIA).toString();
        const chainId = this.chain === Chain.BOB ? ChainId.BOB : ChainId.BOB_SEPOLIA;

        const strategies: GatewayStrategy[] = await response.json();
        return strategies.map(strategy => {
            const inputToken = ADDRESS_LOOKUP[strategy.inputTokenAddress];
            const outputToken = strategy.outputTokenAddress
                ? ADDRESS_LOOKUP[strategy.outputTokenAddress]
                : undefined;
            return {
                id: "",
                type: "deposit",
                address: strategy.strategyAddress,
                method: "",
                chain: {
                    id: "", // TODO
                    chainId: chainId,
                    slug: chainName,
                    name: chainName,
                    logo: "", // TODO
                    type: "evm",
                    singleChainSwap: true,
                    singleChainStaking: true,
                },
                integration: {
                    type: strategy.strategyType,
                    slug: strategy.strategyName,
                    name: strategy.strategyName,
                    logo: "", // TODO
                    monetization: false,
                },
                inputToken: {
                    symbol: inputToken.symbol,
                    address: inputToken.address,
                    logo: inputToken.logoURI,
                    decimals: inputToken.decimals,
                    chain: chainName,
                },
                outputToken: outputToken
                    ? {
                        symbol: outputToken.symbol,
                        address: outputToken.address,
                        logo: outputToken.logoURI,
                        decimals: outputToken.decimals,
                        chain: chainName,
                    }
                    : null,
            }
        });
    }

    /**
     * Returns all tokens (and strategy tokens) supported by the Gateway API.
     * 
     * @returns {Promise<EvmAddress[]>} The array of token addresses.
     */
    async getTokenAddresses(): Promise<EvmAddress[]> {
        const response = await fetch(`${this.baseUrl}/tokens`, {
            method: 'GET',
            headers: {
                'Content-Type': 'application/json',
                Accept: 'application/json'
            }
        });

        return response.json();
    }

    /**
     * Same as {@link getTokenAddresses} but with additional info.
     * 
     * @returns {Promise<Token[]>} The array of tokens.
     */
    async getTokens(): Promise<Token[]> {
        // https://github.com/ethereum-optimism/ecosystem/blob/c6faa01455f9e846f31c0343a0be4c03cbeb2a6d/packages/op-app/src/hooks/useOPTokens.ts#L10
        const tokens = await this.getTokenAddresses();
        return tokens
            .map(token => ADDRESS_LOOKUP[token])
            .filter(token => token !== undefined);
    }
}

/**
 * Should compute the same OP_RETURN hash as the Gateway API and smart contracts.
 * This is used for data integrity checking.
 */
function calculateOpReturnHash(req: GatewayCreateOrderRequest) {
    const abiCoder = new AbiCoder();
    return ethers.keccak256(abiCoder.encode(
        ["address", "address", "uint256", "address", "bytes", "bytes"],
        [
            req.gatewayAddress,
            req.strategyAddress || ethers.ZeroAddress,
            req.satsToConvertToEth,
            req.userAddress,
            req.gatewayExtraData || "0x",
            req.strategyExtraData || "0x"
        ]
    ))
}