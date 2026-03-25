import { createPublicClient, createWalletClient, http, erc20Abi, isAddressEqual, isHex, type WalletClient, type PublicClient, type Hex, type Chain } from 'viem';
import { privateKeyToAccount } from 'viem/accounts';
import { supportedChainsMapping, getChainConfig } from '@gobob/bob-sdk';
import tokenlistJson from '@gobob/tokenlist/tokenlist.json';
import type { TokenBalance } from './index.js';
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
const CHAIN_IDS: Record<string, number> = Object.fromEntries(
  Object.entries(supportedChainsMapping).map(([name, chain]) => [name, (chain as Chain).id]),
);

/** Resolve token metadata from the tokenlist. For BTC, returns { symbol: "BTC", decimals: 8 }. */
export function getTokenMetadata(address: string, chain: string): { symbol: string; decimals: number } {
  if (chain === 'bitcoin' || address === 'BTC') {
    return { symbol: 'BTC', decimals: BTC_DECIMALS };
  }

  const entry = tokenIndex.get(address.toLowerCase());
  if (!entry) {
    return { symbol: address.slice(0, 10), decimals: 18 }; // fallback
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

/** Resolve RPC URL from env var EVM_RPC_URL_<CHAIN>, or undefined for viem defaults. */
import { resolveRpcUrl } from '../util/rpc-resolver.js';

export { resolveRpcUrl };

export const NATIVE_GAS_BUFFER = 900_000n;

async function getClient(chain: string): Promise<PublicClient> {
  const rpcUrl = await resolveRpcUrl(chain);
  return createPublicClient({ chain: getChainConfig(chain), transport: http(rpcUrl) }) as PublicClient;
}

// ─── Single-token balance (used by --amount ALL in swap) ────────────────────

export async function getEvmNativeBalance(chain: string, address: string): Promise<TokenBalance> {
  const client = await getClient(chain);
  const [balance, feeData] = await Promise.all([
    client.getBalance({ address: address as `0x${string}` }),
    client.estimateFeesPerGas(),
  ]);
  const gasCost = (feeData.maxFeePerGas ?? feeData.gasPrice ?? 0n) * NATIVE_GAS_BUFFER;
  const available = balance > gasCost ? balance - gasCost : 0n;
  return { total: balance.toString(), allSpendable: available.toString() };
}

export async function getEvmTokenBalance(
  chain: string,
  address: string,
  tokenAddress: string,
  feeToken?: string,
  feeReserve?: string,
): Promise<TokenBalance> {
  const client = await getClient(chain);
  const balance = await client.readContract({
    address: tokenAddress as `0x${string}`,
    abi: erc20Abi,
    functionName: 'balanceOf',
    args: [address as `0x${string}`],
  });
  const total = balance.toString();
  let allSpendable = total;
  if (feeToken && feeReserve && isAddressEqual(tokenAddress as `0x${string}`, feeToken as `0x${string}`)) {
    const reserved = BigInt(feeReserve);
    const available = balance > reserved ? balance - reserved : 0n;
    allSpendable = available.toString();
  }
  return { total, allSpendable };
}

// ─── All balances for a chain (used by balance command) ─────────────────────

export interface ChainBalanceRaw {
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

export async function getEvmChainBalancesRaw(
  chain: string,
  address: string,
  tokens: Array<{ address: string; symbol: string; decimals: number }>,
  opts?: { feeToken?: string; feeReserve?: string },
): Promise<ChainBalanceRaw> {
  const client = await getClient(chain);
  const nt = getNativeToken(chain);

  // Filter out the zero address — native ETH is already queried separately
  const ZERO_ADDR = "0x0000000000000000000000000000000000000000";
  tokens = tokens.filter(t => !isAddressEqual(t.address as `0x${string}`, ZERO_ADDR as `0x${string}`));

  // Single RPC batch: native balance + fee estimation + all token balances
  const [nativeBalance, feeData, ...tokenResults] = await Promise.all([
    client.getBalance({ address: address as `0x${string}` }),
    client.estimateFeesPerGas(),
    ...(tokens.length > 0
      ? [client.multicall({
          contracts: tokens.map(t => ({
            address: t.address as `0x${string}`,
            abi: erc20Abi,
            functionName: 'balanceOf' as const,
            args: [address as `0x${string}`],
          })),
        })]
      : []),
  ]);

  const gasCost = (feeData.maxFeePerGas ?? feeData.gasPrice ?? 0n) * NATIVE_GAS_BUFFER;
  const nativeSpendable = nativeBalance > gasCost ? nativeBalance - gasCost : 0n;

  const multicallResults = tokenResults[0] as Array<{ result?: bigint }> | undefined;
  const tokenBals = tokens.map((t, i) => {
    const bal = (multicallResults?.[i]?.result as bigint) ?? 0n;
    let allSpendable = bal;
    if (opts?.feeToken && opts?.feeReserve && isAddressEqual(t.address as `0x${string}`, opts.feeToken as `0x${string}`)) {
      const reserved = BigInt(opts.feeReserve);
      allSpendable = bal > reserved ? bal - reserved : 0n;
    }
    return {
      symbol: t.symbol,
      address: t.address,
      decimals: t.decimals,
      balance: bal.toString(),
      allSpendable: allSpendable.toString(),
    };
  });

  return {
    address,
    native: {
      symbol: nt.symbol,
      decimals: nt.decimals,
      balance: nativeBalance.toString(),
      allSpendable: nativeSpendable.toString(),
    },
    tokens: tokenBals,
  };
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
