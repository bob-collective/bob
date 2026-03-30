import { createPublicClient, createWalletClient, http, erc20Abi, isAddressEqual, isHex, type WalletClient, type PublicClient, type Hex, type Chain } from 'viem';
import { privateKeyToAccount } from 'viem/accounts';
import { supportedChainsMapping, getChainConfig } from '@gobob/bob-sdk';
import tokenlistJson from '@gobob/tokenlist/tokenlist.json';
import { BTC_DECIMALS } from '../config.js';

// ─── Tokenlist index ─────────────────────────────────────────────────────────

/**
 * Build an in-memory index of token metadata from the tokenlist.
 * Groups tokens by address (case-insensitive) and tracks whether metadata
 * is uniform across all chains or varies by chain.
 */
type TokenEntry = { address: string; symbol: string; decimals: number; chainId: number };

interface AddressEntry {
  canonical: TokenEntry;
  uniform: boolean;
  byChainId: Map<number, TokenEntry>;
}

const tokenIndex = new Map<string, AddressEntry>();
for (const t of tokenlistJson.tokens as TokenEntry[]) {
  const key = t.address.toLowerCase();
  const existing = tokenIndex.get(key);
  if (existing) {
    existing.byChainId.set(t.chainId, t);
    if (existing.uniform && (t.symbol !== existing.canonical.symbol || t.decimals !== existing.canonical.decimals)) {
      existing.uniform = false;
    }
  } else {
    tokenIndex.set(key, {
      canonical: t,
      uniform: true,
      byChainId: new Map([[t.chainId, t]]),
    });
  }
}

/** Map of chain names to their numeric chain IDs (e.g., "ethereum" → 1). */
export const CHAIN_IDS: Record<string, number> = Object.fromEntries(
  Object.entries(supportedChainsMapping).map(([name, chain]) => [name, (chain as Chain).id]),
);

/** Resolve token metadata from the tokenlist. For BTC, returns { symbol: "BTC", decimals: 8 }.
 *  Throws on unknown tokens by default (safe for amount calculations).
 *  Pass { throwOnUnknown: false } for display paths where best-effort metadata is acceptable. */
export function getTokenMetadata(address: string, chain: string, opts?: { throwOnUnknown?: boolean }): { symbol: string; decimals: number } {
  if (chain === 'bitcoin' || address === 'BTC') {
    return { symbol: 'BTC', decimals: BTC_DECIMALS };
  }

  // Zero address represents the native token on EVM chains (ETH, BNB, etc.)
  if (isNativeToken(address)) {
    return getNativeToken(chain);
  }

  const entry = tokenIndex.get(address.toLowerCase());
  if (!entry) {
    if (opts?.throwOnUnknown === false) {
      return { symbol: address.slice(0, 10), decimals: 18 };
    }
    throw new Error(`Unknown token ${address} on chain "${chain}" — cannot determine decimals. Use a known token symbol or verify the address.`);
  }

  if (entry.uniform) {
    return { symbol: entry.canonical.symbol, decimals: entry.canonical.decimals };
  }

  const chainId = CHAIN_IDS[chain];
  const token = chainId !== undefined ? entry.byChainId.get(chainId) : undefined;
  if (token) return { symbol: token.symbol, decimals: token.decimals };

  // Token has varying metadata across chains; use canonical (first seen) values
  return { symbol: entry.canonical.symbol, decimals: entry.canonical.decimals };
}

// ─── Native token metadata ───────────────────────────────────────────────────

/**
 * Map of chain names to their native token metadata (symbol and decimals).
 * Built from supportedChainsMapping to ensure consistency with viem chain configs.
 */
const NATIVE_TOKENS: Record<string, { symbol: string; decimals: number }> = Object.fromEntries(
  Object.entries(supportedChainsMapping).map(([name, chain]) => [
    name,
    { symbol: chain.nativeCurrency.symbol, decimals: chain.nativeCurrency.decimals },
  ]),
);

/**
 * Get the native token metadata for a chain (e.g., ETH with 18 decimals for Ethereum).
 * @throws Error if chain is not in supportedChainsMapping
 */
export function getNativeToken(chain: string): { symbol: string; decimals: number } {
  const token = NATIVE_TOKENS[chain];
  if (!token) throw new Error(`unknown chain "${chain}" — cannot determine native token`);
  return token;
}

// ─── RPC client ──────────────────────────────────────────────────────────────

import { resolveRpcUrl } from '../util/rpc-resolver.js';

/**
 * Conservative gas buffer for estimating spendable native token balance.
 * Set to 900,000 gas units to cover complex transactions with safety margin.
 * Simple ETH transfers use ~21,000 gas; this buffer accounts for contract interactions
 * and gas price fluctuations during transaction execution.
 */
export const NATIVE_GAS_BUFFER = 900_000n;

/**
 * Create a public RPC client for the specified chain.
 * Resolves RPC URL from environment variable, chainlist probe, or viem defaults.
 */
async function getClient(chain: string): Promise<PublicClient> {
  const rpcUrl = await resolveRpcUrl(chain);
  return createPublicClient({ chain: getChainConfig(chain), transport: http(rpcUrl) }) as PublicClient;
}

// ─── Shared helper ──────────────────────────────────────────────────────────

/** Zero address constant for comparing against native token (no contract address). */
const ZERO_ADDR = "0x0000000000000000000000000000000000000000" as `0x${string}`;

/**
 * Check if a token address represents the native token.
 * Native tokens have no contract address (represented by zero address or undefined).
 */
function isNativeToken(tokenAddress?: string): boolean {
  return !tokenAddress || isAddressEqual(tokenAddress as `0x${string}`, ZERO_ADDR);
}

/**
 * Deduct a fee reserve from a balance if the token matches the fee token.
 * Used to calculate spendable balance when a token is reserved for gas payments.
 */
function applyFeeReserve(balance: bigint, tokenAddress: string, feeToken?: string, feeReserve?: string): bigint {
  if (!feeToken || !feeReserve) return balance;
  if (!isAddressEqual(tokenAddress as `0x${string}`, feeToken as `0x${string}`)) return balance;
  const reserved = BigInt(feeReserve);
  return balance > reserved ? balance - reserved : 0n;
}

// ─── EVM balances ───────────────────────────────────────────────────────────

/**
 * Balance data for a single chain.
 * Contains native token balance and/or ERC20 token balances in atomic units.
 * For Bitcoin chains, uses balance/allSpendable fields directly.
 */
export interface ChainBalance {
  address: string;
  /** Bitcoin total balance in satoshis (atomic units). */
  balance?: string;
  /** Bitcoin maximum spendable balance in satoshis (atomic units). */
  allSpendable?: string;
  /** EVM native token balance and spendable amount in atomic units (wei). */
  native?: { symbol: string; decimals: number; balance: string; allSpendable: string };
  /** EVM ERC20 token balances in atomic units. */
  tokens?: Array<{ symbol: string; address: string; decimals: number; balance: string; allSpendable: string }>;
  /** True if RPC call failed for this chain. */
  error?: boolean;
  /** Error message if RPC call failed. */
  errorMessage?: string;
}

/**
 * Fetch EVM native and ERC20 token balances for an address.
 * Deducts estimated gas costs from native spendable balance.
 * Applies fee reserve deduction for tokens used as gas payment.
 *
 * @param chain - Chain name (e.g., "ethereum", "base")
 * @param address - EVM address to query
 * @param tokens - Optional list of ERC20 tokens to query
 * @param opts - Options including fee token/reserve for gas payment calculations
 * @returns Balance data with native and/or token balances in atomic units
 */
export async function getEvmBalances(
  chain: string,
  address: string,
  tokens?: Array<{ address: string; symbol: string; decimals: number }>,
  opts?: { feeToken?: string; feeReserve?: string; includeNative?: boolean },
): Promise<ChainBalance> {
  const client = await getClient(chain);

  // Filter out zero address from tokens — native is handled via includeNative
  const erc20s = (tokens ?? []).filter(t => !isNativeToken(t.address));

  const calls: Promise<any>[] = [];
  const hasNative = opts?.includeNative ?? false;
  const hasTokens = erc20s.length > 0;

  // Build a single Promise.all with only the needed RPC calls
  if (hasNative) {
    calls.push(
      client.getBalance({ address: address as `0x${string}` }),
      client.estimateFeesPerGas(),
    );
  }
  if (hasTokens) {
    calls.push(client.multicall({
      contracts: erc20s.map(t => ({
        address: t.address as `0x${string}`,
        abi: erc20Abi,
        functionName: 'balanceOf' as const,
        args: [address as `0x${string}`],
      })),
    }));
  }

  const results = await Promise.all(calls);

  // Unpack results based on what was requested
  let idx = 0;
  let native: ChainBalance["native"];
  if (hasNative) {
    const nativeBalance = results[idx++] as bigint;
    const feeData = results[idx++] as { maxFeePerGas?: bigint | null; gasPrice?: bigint | null };
    const gasCost = (feeData.maxFeePerGas ?? feeData.gasPrice ?? 0n) * NATIVE_GAS_BUFFER;
    const nativeSpendable = nativeBalance > gasCost ? nativeBalance - gasCost : 0n;
    const nt = getNativeToken(chain);
    native = {
      symbol: nt.symbol,
      decimals: nt.decimals,
      balance: nativeBalance.toString(),
      allSpendable: nativeSpendable.toString(),
    };
  }

  let tokenBals: ChainBalance["tokens"];
  if (hasTokens) {
    const multicallResults = results[idx] as Array<{ status: 'success' | 'failure'; result?: bigint; error?: Error }>;
    tokenBals = erc20s.map((t, i) => {
      const entry = multicallResults[i];
      const bal = entry?.status === 'success' ? (entry.result as bigint) ?? 0n : 0n;
      if (entry?.status === 'failure') {
        console.warn(`Warning: balanceOf call failed for token ${t.symbol} (${t.address}): ${entry.error?.message ?? 'unknown error'}`);
      }
      return {
        symbol: t.symbol,
        address: t.address,
        decimals: t.decimals,
        balance: bal.toString(),
        allSpendable: applyFeeReserve(bal, t.address, opts?.feeToken, opts?.feeReserve).toString(),
      };
    });
  }

  return { address, native, tokens: tokenBals };
}

// ─── Signer ─────────────────────────────────────────────────────────────────

/**
 * Derive an EVM address from a private key.
 * Accepts hex string with or without 0x prefix.
 */
export function deriveEvmAddress(key: string): string {
  const account = privateKeyToAccount((isHex(key) ? key : `0x${key}`) as Hex);
  return account.address;
}

/**
 * Create an EVM signer (wallet client + public client) for a private key and chain.
 * @param key - Private key in hex format (with or without 0x prefix)
 * @param chainName - Chain name for RPC configuration
 * @returns Signer object with address, wallet client, and public client
 */
export async function resolveEvmSigner(
  key: string,
  chainName: string,
): Promise<{ address: string; walletClient: WalletClient; publicClient: PublicClient }> {
  const rpcUrl = await resolveRpcUrl(chainName);
  const viemChain = getChainConfig(chainName);
  const account = privateKeyToAccount((isHex(key) ? key : `0x${key}`) as Hex);
  const transport = http(rpcUrl);
  return {
    address: account.address,
    walletClient: createWalletClient({ account, chain: viemChain, transport }),
    publicClient: createPublicClient({ chain: viemChain, transport }),
  };
}
