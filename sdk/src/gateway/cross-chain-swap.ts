import * as bitcoin from 'bitcoinjs-lib';
import {
    ContractFunctionExecutionError,
    encodeAbiParameters,
    encodeFunctionData,
    erc20Abi,
    isAddress,
    maxUint256,
    parseAbiParameters,
    type Address,
    type Chain,
    type Hex,
    type PublicClient,
    type Transport,
    type WalletClient,
} from 'viem';
import { bob } from 'viem/chains';
import { offrampCallerV2, quoterV2Abi } from './abi';
import { AllWalletClientParams, GatewayApiClient } from './client';
import { LayerZeroGatewayClient } from './layerzero';
import { SwapsClient } from './swaps';
import { getTokenAddress } from './tokens';
import {
    ActionsParams,
    CrossChainSwapQuoteParamsExt,
    OfframpWithSwapsExecuteQuoteParams,
    OnrampWithSwapsExecuteQuoteParams,
} from './types';
import { GatewayOrder, GatewayOrderType } from './types/order';
import { ExecuteQuoteParams, GetQuoteParams } from './types/quote';
import { getChainConfig, resolveChainId, toHexScriptPubKey, viemClient } from './utils/common';

export const BOB_WBTC = '0x0555E30da8f98308EdB960aa94C0Db47230d2B9c';

export const MULTICALL_STRATEGY = '0x702405a5F314D0fDC2af516DF1e263f0Ce474E27';

export const OFFRAMP_REGISTRY = '0x2bbFDaeA28604f1d40C6E2deC5fC08fa8A120472';

export class CrossChainSwapGatewayClient extends LayerZeroGatewayClient {
    private swapsClient: SwapsClient;

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
        tokenAddress: Address = BOB_WBTC
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

        if (fromChain === 'bitcoin' && toChain === bob.name.toLowerCase() && params.toToken === BOB_WBTC) {
            // Handle bitcoin -> wbtc on bob: use normal flow
            return GatewayApiClient.prototype.getQuote.call(this, params);
        } else if (fromChain === bob.name.toLowerCase() && toChain === 'bitcoin' && params.fromToken === BOB_WBTC) {
            // Handle wbtc on bob -> bitcoin: use normal flow
            return GatewayApiClient.prototype.getQuote.call(this, params);
        } else if (fromChain === 'bitcoin' && toChain !== 'bitcoin') {
            // Handle cross chain swap (with onramp)
            if (params.toToken && (await this.isChainAndTokenSupportedByLayerZero(toChain, params.toToken))) {
                // If toChain and toToken are supported by layerzero, use LayerZero flow
                return super.getQuote(params);
            }

            // Otherwise use Swaps flow
            return this.getSwapsOnrampQuote(params);
        } else if (fromChain !== 'bitcoin' && toChain === 'bitcoin') {
            // Handle cross chain swap (with offramp)
            if (params.fromToken && (await this.isChainAndTokenSupportedByLayerZero(fromChain, params.fromToken))) {
                // If fromChain and fromToken are supported by layerzero, use LayerZero flow
                return super.getQuote(params);
            }

            // Otherwise use Swaps flow
            return this.getSwapsOfframpQuote(params);
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

    async executeQuote({
        quote,
        walletClient,
        publicClient,
        btcSigner,
    }: { quote: ExecuteQuoteParams } & AllWalletClientParams): Promise<string> {
        switch (quote.type) {
            case GatewayOrderType.OnrampWithLayerZero: {
                // Cast quote type to Onramp
                const onrampQuote = {
                    ...quote,
                    type: GatewayOrderType.Onramp as const,
                };
                return super.executeQuote({
                    quote: onrampQuote,
                    walletClient,
                    publicClient,
                    btcSigner,
                });
            }
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
            case GatewayOrderType.OfframpWithLayerZero: {
                // Cast quote type to Offramp
                const onrampQuote = {
                    ...quote,
                    type: GatewayOrderType.Offramp as const,
                };
                return super.executeQuote({
                    quote: onrampQuote,
                    walletClient,
                    publicClient,
                    btcSigner,
                });
            }
            case GatewayOrderType.OfframpWithSwaps: {
                return this.executeOfframpSwapsQuote({
                    quote,
                    walletClient,
                    publicClient,
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

    /**
     * Retrieves all orders (onramp, offramp, and evm-to-evm swaps) for a specific user address.
     *
     * @param {TransactionParams} params query params for the swaps api
     * @returns Promise resolving to array of typed orders
     */
    async getOrders(userAddress: Address): Promise<Array<GatewayOrder>> {
        const [swapOrders, gatewayOrders] = await Promise.all([
            this.swapsClient.getTransactions(userAddress),
            super.getOrders(userAddress),
        ]);

        const orders = swapOrders.txs.map<GatewayOrder>((order) => ({
            type: GatewayOrderType.EVMToEVMWithSwaps as const,
            order: {
                amount: BigInt(order.actionResponse.amountOut.amount),
                timestamp: new Date(order.srcTx.timestamp).getTime(),
                status: order.status,
                source: {
                    chainId: order.srcTx.chainId,
                    txHash: order.srcTxHash,
                    token: order.srcTx.paymentToken.address,
                },
                destination: {
                    chainId: order.dstTx.chainId,
                    txHash: order.dstTxHash,
                    token: order.dstTx.paymentToken.address,
                },
            },
        }));

        return [...gatewayOrders, ...orders];
    }

    private async getSwapsOnrampQuote(
        params: GetQuoteParams<CrossChainSwapQuoteParamsExt>
    ): Promise<OnrampWithSwapsExecuteQuoteParams> {
        // Resolve chain IDs
        // If fromChain is bitcoin, use bob as the source chain for Swaps API
        const fromChainResolved =
            typeof params.fromChain === 'number' ? resolveChainId(params.fromChain) : params.fromChain;
        const fromChainId = bob.id;
        const toChainId = typeof params.toChain === 'number' ? params.toChain : getChainConfig(params.toChain).id;

        // Resolve token addresses
        if (!params.fromToken) {
            throw new Error('fromToken is required for Swaps API');
        }
        if (!params.toToken) {
            throw new Error('toToken is required for Swaps API');
        }

        const dstToken = isAddress(params.toToken) ? params.toToken : getTokenAddress(toChainId, params.toToken);

        // Validate required addresses
        // if (!params.fromUserAddress) {
        //     // params.fromUserAddress = '0x1111111111111111111111111111111111111111';
        //     throw new Error('fromUserAddress is required for Swaps API');
        // }
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
            srcChainId: bob.id,
            srcToken: BOB_WBTC,
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
        const swapValue = BigInt(actionResponse.tx.value);

        let maxTokensToSwap = BigInt(0);
        let finalAmount = BigInt(amount);

        if (swapValue > 0) {
            // figure out the amount to swap
            const publicClient = viemClient(bob);
            const quote = await publicClient.readContract({
                address: '0x6Aa54a43d7eEF5b239a18eed3Af4877f46522BCA',
                abi: quoterV2Abi,
                functionName: 'quoteExactOutputSingle',
                args: [
                    {
                        tokenIn: BOB_WBTC as Hex,
                        tokenOut: '0x4200000000000000000000000000000000000006' as Hex,
                        amountOut: swapValue, // Desired output amount
                        fee: 3000,
                        sqrtPriceLimitX96: BigInt(0),
                    },
                ],
            });
            const tokensToSwapForSwapFees = quote[0];
            const maxTokensToSwapForSwapFees = (tokensToSwapForSwapFees * (10000n + BigInt(300))) / 10000n; // 3%

            maxTokensToSwap = maxTokensToSwapForSwapFees;
            finalAmount -= maxTokensToSwap;

            const encodedParameters = encodeAbiParameters(
                parseAbiParameters([
                    'uint256 value',
                    'uint256 maxTokensToSwap',
                    'address target',
                    'bytes memory callData',
                ]),
                [swapValue, maxTokensToSwap, swapTo, swapCalldata]
            );

            params.strategyAddress = '0x20A68781116EBdC3b2C92040eFdf6fcc72ad1BF6' as Address;
            params.message = encodedParameters;
        } else {
            // Encode the calls array for MulticallStrategy
            const encodedCalls = this.encodeMulticallCalls(
                swapTo as Address,
                swapCalldata as Hex,
                finalAmount,
                BOB_WBTC
            );

            // Set up params for onramp quote (similar to layerzero.ts line 427-433)
            params.strategyAddress = MULTICALL_STRATEGY as Address;
            params.message = encodedCalls;
        }

        // For onramp flows (bitcoin -> other chain), change toChain to bob.id for the quote
        // The actual destination chain is handled by the swap transaction
        params.toChain = bob.id;
        params.toToken = BOB_WBTC;

        if (swapValue > 0) {
            // Now refetch the Swap calldata using the swapped amount subtracted
            const actionResponse2 = await this.swapsClient.getAction({
                ...actionParams,
                amount: finalAmount.toString(), // subtract the swap fee
            });

            // re-encode the Gateway calldata with the new tx
            params.message = encodeAbiParameters(
                parseAbiParameters([
                    'uint256 value',
                    'uint256 maxTokensToSwap',
                    'address target',
                    'bytes memory callData',
                ]),
                [swapValue, maxTokensToSwap, actionResponse2.tx.to, actionResponse2.tx.data]
            );
        }

        // Get the actual onramp quote from GatewayApiClient (skip LayerZeroGatewayClient)
        const baseQuote: Awaited<ReturnType<typeof GatewayApiClient.prototype.getQuote>> =
            await GatewayApiClient.prototype.getQuote.call(this, params);
        finalAmount = BigInt(baseQuote.finalOutputSats) - maxTokensToSwap;

        // Now refetch the Swap calldata using the finalOutputSats as the amount
        const actionResponse3 = await this.swapsClient.getAction({
            ...actionParams,
            amount: finalAmount.toString(),
        });

        // Now encode the final swap call
        if (BigInt(actionResponse3.tx.value) > 0) {
            params.message = encodeAbiParameters(
                parseAbiParameters([
                    'uint256 value',
                    'uint256 maxTokensToSwap',
                    'address target',
                    'bytes memory callData',
                ]),
                [swapValue, maxTokensToSwap, actionResponse3.tx.to, actionResponse3.tx.data]
            );
        } else {
            params.message = this.encodeMulticallCalls(
                actionResponse3.tx.to as Address,
                actionResponse3.tx.data as Hex,
                BigInt(baseQuote.finalOutputSats),
                BOB_WBTC
            );
        }

        // Construct SwapsExecuteQuoteParams
        return {
            params: {
                ...baseQuote.params,
                message: params.message,
            },
            type: GatewayOrderType.OnrampWithSwaps,
            finalOutputSats: Number(actionResponse3.amountOut.amount),
            finalFeeSats: baseQuote.finalFeeSats,
            data: baseQuote.data,
        } as OnrampWithSwapsExecuteQuoteParams;
    }

    private async getSwapsOfframpQuote(
        params: GetQuoteParams<CrossChainSwapQuoteParamsExt>
    ): Promise<OfframpWithSwapsExecuteQuoteParams> {
        // Resolve chain IDs
        const fromChainId =
            typeof params.fromChain === 'number' ? params.fromChain : getChainConfig(params.fromChain).id;

        // Resolve token addresses
        if (!params.fromToken) {
            throw new Error('fromToken is required for Swaps API');
        }
        if (!params.toUserAddress) {
            params.toUserAddress = 'bc1p0lnvm7x4naxn7v9mlyjxyg8asf7fqqzcj92tamlzr255q5csheqsc5y5r3';
            params.fromUserPublicKey = '03a76385336e0a8e1538f4f7f7dbb36a82913acf2c8fc0bfc673d3e13599d5210c';
        }

        const srcToken = isAddress(params.fromToken)
            ? params.fromToken
            : getTokenAddress(fromChainId, params.fromToken);

        // Validate required addresses
        if (!params.fromUserAddress) {
            params.fromUserAddress = '0x1111111111111111111111111111111111111111';
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

        // First, call Swaps API with swap-action to determine how much WBTC will be received on BOB
        const swapActionParams: ActionsParams = {
            actionType: 'swap-action',
            sender: params.fromUserAddress as Address,
            srcChainId: fromChainId,
            srcToken: srcToken,
            dstChainId: bob.id,
            dstToken: BOB_WBTC,
            slippage: slippage,
            amount: amount,
            swapDirection: 'exact-amount-in',
            recipient: params.fromUserAddress as Address, // can be any address, we wont actually execute this swap
        };

        const swapActionResponse = await this.swapsClient.getAction(swapActionParams);

        // Get the WBTC amount that will be received on BOB (amountOut)
        const wbtcAmountReceived = swapActionResponse.amountOut.amount;

        // Set up params for offramp quote using the WBTC amount received
        const offrampParams = {
            ...params,
            fromChain: bob.id,
            toChain: 'bitcoin',
            fromToken: BOB_WBTC,
            amount: wbtcAmountReceived, // Use the WBTC amount from the swap
        };

        // Get the bob -> bitcoin offramp quote from GatewayApiClient
        const baseQuote: Awaited<ReturnType<typeof GatewayApiClient.prototype.getQuote>> =
            await GatewayApiClient.prototype.getQuote.call(this, offrampParams);

        if (baseQuote.type !== GatewayOrderType.Offramp) {
            throw new Error('Expected offramp quote but got different type');
        }

        const offrampQuote = baseQuote.data;

        // Convert Bitcoin address to output script
        // Use mainnet by default, can be made configurable if needed
        const bitcoinNetwork = bitcoin.networks.bitcoin;
        const outputScript = toHexScriptPubKey(params.toUserAddress, bitcoinNetwork) as Hex;

        // Encode createOrderV2 call
        const createOrderCalldata = encodeFunctionData({
            abi: offrampCallerV2,
            functionName: 'createOrderV2',
            args: [
                {
                    satAmountToLock: BigInt(offrampQuote.amountLockInSat),
                    satSolverFeeMax: BigInt(offrampQuote.feeBreakdown.overallFeeSats),
                    satAffiliateFee: BigInt(offrampQuote.feeBreakdown.affiliateFeeSats),
                    affiliateFeeRecipient: offrampQuote.affiliateFeeRecipient,
                    creationDeadline: BigInt(offrampQuote.deadline),
                    outputScript: outputScript,
                    token: offrampQuote.token,
                    owner: params.fromUserAddress as Address,
                },
            ],
        });

        // Construct ActionsParams for evm-calldata-tx
        // Use the WBTC amount received from the swap for erc20Amount (amount that will be locked in the order)
        const actionParams: ActionsParams = {
            actionType: 'evm-calldata-tx',
            sender: params.fromUserAddress as Address,
            srcChainId: fromChainId,
            srcToken: srcToken,
            dstChainId: bob.id,
            dstToken: BOB_WBTC,
            slippage: 0, // Not used for evm-calldata-tx
            amount: amount, // Source amount for the swap
            to: OFFRAMP_REGISTRY as Address,
            data: createOrderCalldata,
            value: '0',
            erc20Amount: wbtcAmountReceived, // WBTC amount that will be locked in the offramp order
            erc20Spender: OFFRAMP_REGISTRY as Address,
        };

        // Call Swaps API
        const actionResponse = await this.swapsClient.getAction(actionParams);

        return {
            params,
            type: GatewayOrderType.OfframpWithSwaps,
            finalOutputSats: baseQuote.finalOutputSats,
            finalFeeSats: baseQuote.finalFeeSats,
            data: {
                ...offrampQuote,
                tx: actionResponse.tx,
                amountInMax: actionResponse.amountInMax,
            },
        };
    }

    /**
     * Executes an offramp swaps quote by simulating and executing the transaction from Swaps API
     * @param quote The offramp swaps quote containing the transaction to execute
     * @param walletClient The wallet client for signing and sending transactions
     * @param publicClient The public client for simulating transactions and reading chain data
     * @returns Promise resolving to the transaction hash
     */
    private async executeOfframpSwapsQuote({
        quote,
        walletClient,
        publicClient,
    }: {
        quote: OfframpWithSwapsExecuteQuoteParams;
        walletClient: WalletClient<Transport, Chain>;
        publicClient: PublicClient<Transport>;
    }): Promise<string> {
        // Extract transaction data from quote
        const swapsTx = quote.data.tx;

        if (!swapsTx || !swapsTx.to || !swapsTx.data) {
            throw new Error('Transaction data (tx.to, tx.data) not found in quote');
        }

        const { to, data, value, chainId } = swapsTx;

        // Verify the transaction is for the correct chain
        const currentChainId = await publicClient.getChainId();

        if (chainId !== currentChainId) {
            throw new Error(`Transaction chainId (${chainId}) does not match current chain (${currentChainId})`);
        }

        if (!walletClient.account) {
            throw new Error('Wallet client account is required');
        }

        if (!quote.data.amountInMax.isNative) {
            const allowance = await publicClient.readContract({
                account: walletClient.account,
                address: quote.data.amountInMax.address,
                abi: erc20Abi,
                functionName: 'allowance',
                args: [quote.params.fromUserAddress as Address, quote.data.tx.to],
            });

            const amountIn = BigInt(quote.data.amountInMax.amount);
            if (allowance < amountIn) {
                const { request } = await publicClient.simulateContract({
                    account: walletClient.account,
                    address: quote.data.amountInMax.address,
                    abi: erc20Abi,
                    functionName: 'approve',
                    args: [quote.data.tx.to, maxUint256],
                });

                const txHash = await walletClient.writeContract(request);
                await publicClient.waitForTransactionReceipt({ hash: txHash });
            }
        }

        // Estimate gas to check if transaction will succeed
        try {
            await publicClient.estimateGas({
                account: walletClient.account,
                to,
                data,
                value: BigInt(value || '0'),
            });
        } catch (error) {
            throw new Error(`Transaction estimation failed: ${error instanceof Error ? error.message : String(error)}`);
        }

        // Execute the transaction
        const transactionHash = await walletClient.sendTransaction({
            account: walletClient.account,
            to,
            data,
            value: BigInt(value || '0'),
        });

        await publicClient.waitForTransactionReceipt({ hash: transactionHash });

        return transactionHash;
    }
}
