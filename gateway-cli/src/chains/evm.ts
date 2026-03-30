import { createPublicClient, createWalletClient, http, erc20Abi, isAddressEqual, isHex, type WalletClient, type PublicClient, type Hex, type Chain } from 'viem';
import { privateKeyToAccount } from 'viem/accounts';
import { supportedChainsMapping, getChainConfig } from '@gobob/bob-sdk';
import tokenlistJson from '@gobob/tokenlist/tokenlist.json';
import { BTC_DECIMALS } from '../config.js';

// ─── Tokenlist index ─────────────────────────────────────────────────────────

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

// Chain name → chain ID mapping from supportedChainsMapping
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

  // Ambiguous — fall back to canonical
  return { symbol: entry.canonical.symbol, decimals: entry.canonical.decimals };
}

// ─── Native token metadata ───────────────────────────────────────────────────

const NATIVE_TOKENS: Record<string, { symbol: string; decimals: number }> = Object.fromEntries(
  Object.entries(supportedChainsMapping).map(([name, chain]) => [
    name,
    { symbol: chain.nativeCurrency.symbol, decimals: chain.nativeCurrency.decimals },
  ]),
);

export function getNativeToken(chain: string): { symbol: string; decimals: number } {
  const token = NATIVE_TOKENS[chain];
  if (!token) throw new Error(`unknown chain "${chain}" — cannot determine native token`);
  return token;
}

// ─── RPC client ──────────────────────────────────────────────────────────────

import { resolveRpcUrl } from '../util/rpc-resolver.js';

export const NATIVE_GAS_BUFFER = 900_000n;

async function getClient(chain: string): Promise<PublicClient> {
  const rpcUrl = await resolveRpcUrl(chain);
  return createPublicClient({ chain: getChainConfig(chain), transport: http(rpcUrl) }) as PublicClient;
}

// ─── Shared helper ──────────────────────────────────────────────────────────

const ZERO_ADDR = "0x0000000000000000000000000000000000000000" as `0x${string}`;

function isNativeToken(tokenAddress?: string): boolean {
  return !tokenAddress || isAddressEqual(tokenAddress as `0x${string}`, ZERO_ADDR);
}

function applyFeeReserve(balance: bigint, tokenAddress: string, feeToken?: string, feeReserve?: string): bigint {
  if (!feeToken || !feeReserve) return balance;
  if (!isAddressEqual(tokenAddress as `0x${string}`, feeToken as `0x${string}`)) return balance;
  const reserved = BigInt(feeReserve);
  return balance > reserved ? balance - reserved : 0n;
}

// ─── EVM balances ───────────────────────────────────────────────────────────

export interface ChainBalance {
  address: string;
  // BTC
  balance?: string;       // atomic
  allSpendable?: string;  // atomic
  // EVM native
  native?: { symbol: string; decimals: number; balance: string; allSpendable: string };  // atomic
  // EVM tokens
  tokens?: Array<{ symbol: string; address: string; decimals: number; balance: string; allSpendable: string }>;  // atomic
  error?: boolean;
}

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
    const multicallResults = results[idx] as Array<{ result?: bigint }>;
    tokenBals = erc20s.map((t, i) => {
      const bal = (multicallResults[i]?.result as bigint) ?? 0n;
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

export function deriveEvmAddress(key: string): string {
  const account = privateKeyToAccount((isHex(key) ? key : `0x${key}`) as Hex);
  return account.address;
}

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
