import { ethers, AbiCoder } from "ethers";
import {
    GatewayQuoteParams,
    GatewayQuote,
    Chain,
    ChainId,
    Token,
    GatewayStrategyContract,
    GatewayCreateOrderRequest,
    GatewayCreateOrderResponse,
    GatewayOrder,
    GatewayOrderResponse,
    GatewayStartOrder,
    GatewayStrategy,
    EvmAddress,
    GatewayTokensData,
} from "./types";
import { SYMBOL_LOOKUP, ADDRESS_LOOKUP } from "./tokens";
import { createBitcoinPsbt } from "../wallet";
import { Network } from "bitcoin-address-validation";
import { EsploraClient } from "../esplora";

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
    async getQuote(
        params: Optional<
            GatewayQuoteParams,
            "amount" | "fromChain" | "fromToken" | "fromUserAddress" | "toUserAddress"
        >,
    ): Promise<GatewayQuote & GatewayTokensData> {
        const isMainnet =
            params.toChain === ChainId.BOB ||
            (typeof params.toChain === "string" && params.toChain.toLowerCase() === Chain.BOB);
        const isTestnet =
            params.toChain === ChainId.BOB_SEPOLIA ||
            (typeof params.toChain === "string" && params.toChain.toLowerCase() === Chain.BOB_SEPOLIA);

        const isInvalidNetwork = !isMainnet && !isTestnet;
        const isMismatchMainnet = isMainnet && this.chain !== Chain.BOB;
        const isMismatchTestnet = isTestnet && this.chain !== Chain.BOB_SEPOLIA;

        // TODO: switch URL if `toChain` is different chain?
        if (isInvalidNetwork || isMismatchMainnet || isMismatchTestnet) {
            throw new Error("Invalid output chain");
        }

        let outputTokenAddress = "";
        let strategyAddress: string | undefined;

        const toToken = params.toToken.toLowerCase();
        if (params.strategyAddress?.startsWith("0x")) {
            strategyAddress = params.strategyAddress;
        }

        if (toToken.startsWith("0x")) {
            outputTokenAddress = toToken;
        } else if (isMainnet && this.chain === Chain.BOB && SYMBOL_LOOKUP[ChainId.BOB][toToken]) {
            outputTokenAddress = SYMBOL_LOOKUP[ChainId.BOB][toToken].address;
        } else if (isTestnet && this.chain === Chain.BOB_SEPOLIA && SYMBOL_LOOKUP[ChainId.BOB_SEPOLIA][toToken]) {
            outputTokenAddress = SYMBOL_LOOKUP[ChainId.BOB_SEPOLIA][toToken].address;
        } else {
            throw new Error("Unknown output token");
        }

        var url = new URL(`${this.baseUrl}/quote/${outputTokenAddress}`);
        if (strategyAddress) {
            url.searchParams.append("strategy", `${strategyAddress}`);
        }
        const atomicAmount = params.amount;
        if (atomicAmount) {
            url.searchParams.append("satoshis", `${atomicAmount}`);
        }

        const response = await fetch(url, {
            headers: {
                "Content-Type": "application/json",
                Accept: "application/json",
            },
        });

        const quote: GatewayQuote = await response.json();
        return {
            ...quote,
            fee: quote.fee + (params.gasRefill || 0),
            baseToken: ADDRESS_LOOKUP[quote.baseTokenAddress],
            outputToken: ADDRESS_LOOKUP[outputTokenAddress],
        };
    }

    // TODO: add error handling
    /**
     * Start an order via the Gateway API to reserve liquidity. This is step 1 of 2, see the {@link finalizeOrder} method.
     *
     * @param gatewayQuote The quote given by the {@link getQuote} method.
     * @param params The parameters for the quote, same as before.
     * @returns {Promise<GatewayStartOrder>} The success object.
     */
    async startOrder(
        gatewayQuote: GatewayQuote,
        params: Optional<GatewayQuoteParams, "toToken" | "amount">,
    ): Promise<GatewayStartOrder> {
        if (!params.toUserAddress.startsWith("0x") || !ethers.isAddress(params.toUserAddress)) {
            throw new Error("Invalid user address");
        }

        const abiCoder = new AbiCoder();
        const request: GatewayCreateOrderRequest = {
            gatewayAddress: gatewayQuote.gatewayAddress,
            strategyAddress: gatewayQuote.strategyAddress,
            satsToConvertToEth: params.gasRefill || 0,
            userAddress: params.toUserAddress,
            gatewayExtraData: undefined,
            // TODO: update strategy data
            strategyExtraData: abiCoder.encode(["uint256"], [0]),
            satoshis: gatewayQuote.satoshis,
        };

        const response = await fetch(`${this.baseUrl}/order`, {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
                Accept: "application/json",
            },
            body: JSON.stringify(request),
        });

        if (!response.ok) {
            throw new Error("Failed to create order");
        }

        const data: GatewayCreateOrderResponse = await response.json();
        // NOTE: could remove this check but good for sanity
        if (data.opReturnHash != calculateOpReturnHash(request)) {
            throw new Error("Invalid OP_RETURN hash");
        }

        let psbtBase64: string;
        if (
            params.fromUserAddress &&
            typeof params.fromChain === "string" &&
            params.fromChain.toLowerCase() === Chain.BITCOIN
        ) {
            psbtBase64 = await createBitcoinPsbt(
                params.fromUserAddress,
                gatewayQuote.bitcoinAddress,
                gatewayQuote.satoshis,
                params.fromUserPublicKey,
                data.opReturnHash,
                gatewayQuote.txProofDifficultyFactor,
            );
        }

        return {
            uuid: data.uuid,
            opReturnHash: data.opReturnHash,
            bitcoinAddress: gatewayQuote.bitcoinAddress,
            satoshis: gatewayQuote.satoshis,
            psbtBase64,
        };
    }

    /**
     * Finalize an order via the Gateway API by providing the Bitcoin transaction. The tx will
     * be validated for correctness and forwarded to the mempool so there is no need to separately
     * broadcast the transaction. This is step 2 of 2, see the {@link startOrder} method.
     *
     * @param uuid The id given by the {@link startOrder} method.
     * @param bitcoinTxOrId The hex encoded Bitcoin transaction or txid.
     * @returns {Promise<string>} The Bitcoin txid.
     */
    async finalizeOrder(uuid: string, bitcoinTxOrId: string): Promise<string> {
        bitcoinTxOrId = stripHexPrefix(bitcoinTxOrId);

        let bitcoinTxHex: string;
        if (bitcoinTxOrId.length === 64) {
            const esploraClient = new EsploraClient(this.chain === Chain.BOB ? Network.mainnet : Network.testnet);
            bitcoinTxHex = await esploraClient.getTransactionHex(bitcoinTxOrId);
        } else {
            bitcoinTxHex = bitcoinTxOrId;
        }

        const response = await fetch(`${this.baseUrl}/order/${uuid}`, {
            method: "PATCH",
            headers: {
                "Content-Type": "application/json",
                Accept: "application/json",
            },
            body: JSON.stringify({ bitcoinTx: bitcoinTxHex }),
        });

        if (!response.ok) {
            throw new Error("Failed to update order");
        }

        return await response.json();
    }

    /**
     * Returns all pending and completed orders for this account.
     *
     * @param userAddress The user's EVM address.
     * @returns {Promise<GatewayOrder[]>} The array of account orders.
     */
    async getOrders(userAddress: EvmAddress): Promise<(GatewayOrder & Optional<GatewayTokensData, "outputToken">)[]> {
        const response = await this.fetchGet(`${this.baseUrl}/orders/${userAddress}`);
        const orders: GatewayOrderResponse[] = await response.json();
        return orders.map((order) => {
            return {
                gasRefill: order.satsToConvertToEth,
                ...order,
                baseToken: ADDRESS_LOOKUP[order.baseTokenAddress],
                outputToken: ADDRESS_LOOKUP[order.outputTokenAddress]
            };
        });
    }

    /**
     * Returns all strategies supported by the Gateway API.
     *
     * @returns {Promise<GatewayStrategyContract[]>} The array of strategies.
     */
    async getStrategies(): Promise<GatewayStrategyContract[]> {
        const response = await this.fetchGet(`${this.baseUrl}/strategies`);

        const chainName = (this.chain === Chain.BOB ? Chain.BOB : Chain.BOB_SEPOLIA).toString();
        const chainId = this.chain === Chain.BOB ? ChainId.BOB : ChainId.BOB_SEPOLIA;

        const strategies: GatewayStrategy[] = await response.json();
        return strategies.map((strategy) => {
            const strategySlug = slugify(strategy.strategyName);
            const inputToken = ADDRESS_LOOKUP[strategy.inputTokenAddress];
            const outputToken = strategy.outputTokenAddress ? ADDRESS_LOOKUP[strategy.outputTokenAddress] : undefined;
            return {
                id: strategySlug,
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
                    slug: strategySlug,
                    name: strategy.strategyName,
                    logo: strategy.projectLogo || outputToken?.logoURI || "",
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
            };
        });
    }

    /**
     * Returns all tokens supported by the Gateway API.
     *
     * @param includeStrategies Also include output tokens via strategies (e.g. staking or lending).
     * @returns {Promise<EvmAddress[]>} The array of token addresses.
     */
    async getTokenAddresses(includeStrategies: boolean = true): Promise<EvmAddress[]> {
        const response = await this.fetchGet(`${this.baseUrl}/tokens?includeStrategies=${includeStrategies}`);
        return response.json();
    }

    /**
     * Same as {@link getTokenAddresses} but with additional info.
     *
     * @param includeStrategies Also include output tokens via strategies (e.g. staking or lending).
     * @returns {Promise<Token[]>} The array of tokens.
     */
    async getTokens(includeStrategies: boolean = true): Promise<Token[]> {
        // https://github.com/ethereum-optimism/ecosystem/blob/c6faa01455f9e846f31c0343a0be4c03cbeb2a6d/packages/op-app/src/hooks/useOPTokens.ts#L10
        const tokens = await this.getTokenAddresses(includeStrategies);
        return tokens.map((token) => ADDRESS_LOOKUP[token]).filter((token) => token !== undefined);
    }

    private async fetchGet(url: string) {
        return await fetch(url, {
            method: "GET",
            headers: {
                "Content-Type": "application/json",
                Accept: "application/json",
            },
        });
    }
}

/**
 * Should compute the same OP_RETURN hash as the Gateway API and smart contracts.
 * This is used for data integrity checking.
 */
function calculateOpReturnHash(req: GatewayCreateOrderRequest) {
    const abiCoder = new AbiCoder();
    return ethers.keccak256(
        abiCoder.encode(
            ["address", "address", "uint256", "address", "bytes", "bytes"],
            [
                req.gatewayAddress,
                req.strategyAddress || ethers.ZeroAddress,
                req.satsToConvertToEth,
                req.userAddress,
                req.gatewayExtraData || "0x",
                req.strategyExtraData || "0x",
            ],
        ),
    );
}

function isHexPrefixed(str: string): boolean {
    return str.slice(0, 2) === "0x";
}

function stripHexPrefix(str: string): string {
    return isHexPrefixed(str) ? str.slice(2) : str;
}

function slugify(str: string): string {
    return str
        .toLowerCase()
        .replace(/ /g, "-")
        .replace(/[^\w-]+/g, "");
}
