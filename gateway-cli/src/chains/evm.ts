import { createPublicClient, createWalletClient, http, erc20Abi, isAddressEqual, isHex, formatUnits, type WalletClient, type PublicClient, type Hex, type Chain } from 'viem';
import { privateKeyToAccount } from 'viem/accounts';
import { supportedChainsMapping } from '@gobob/bob-sdk';
import { getNativeToken } from '../util/route-provider.js';
import type { TokenBalance } from './index.js';
import type { EnrichedToken } from '../util/route-provider.js';
import type { BalanceJson } from '../output.js';

/** Resolve RPC URL from env var EVM_RPC_URL_<CHAIN>, or undefined for viem defaults. */
export function resolveRpcUrl(chainName: string): string | undefined {
  return process.env[`EVM_RPC_URL_${chainName.toUpperCase()}`];
}

/** Get the viem Chain object for a gateway chain name, if supported. */
export function getViemChain(chainName: string): Chain | undefined {
  return (supportedChainsMapping as Record<string, Chain>)[chainName];
}

export const NATIVE_GAS_BUFFER = 900_000n;

function getClient(chain: string): PublicClient {
  return createPublicClient({ chain: getViemChain(chain), transport: http(resolveRpcUrl(chain)) }) as PublicClient;
}

// ─── Single-token balance (used by --amount ALL in swap) ────────────────────

export async function getEvmNativeBalance(chain: string, address: string): Promise<TokenBalance> {
  const client = getClient(chain);
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
  const client = getClient(chain);
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

export async function getEvmChainBalances(
  chain: string,
  address: string,
  tokens: EnrichedToken[],
  opts?: { feeToken?: string; feeReserve?: string },
): Promise<BalanceJson[string]> {
  const client = getClient(chain);
  const nt = getNativeToken(chain);

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
      balance: formatUnits(bal, t.decimals),
      allSpendable: formatUnits(allSpendable, t.decimals),
    };
  });

  return {
    address,
    native: {
      symbol: nt.symbol,
      balance: formatUnits(nativeBalance, nt.decimals),
      allSpendable: formatUnits(nativeSpendable, nt.decimals),
    },
    tokens: tokenBals,
  };
}

// ─── Signer ─────────────────────────────────────────────────────────────────

export function deriveEvmAddress(key: string): string {
  const account = privateKeyToAccount((isHex(key) ? key : `0x${key}`) as Hex);
  return account.address;
}

export function resolveEvmSigner(
  key: string,
  chainName: string,
): { address: string; walletClient: WalletClient; publicClient: PublicClient } {
  const rpcUrl = resolveRpcUrl(chainName);
  const viemChain = getViemChain(chainName);
  const account = privateKeyToAccount((isHex(key) ? key : `0x${key}`) as Hex);
  const transport = http(rpcUrl);
  return {
    address: account.address,
    walletClient: createWalletClient({ account, chain: viemChain, transport }),
    publicClient: createPublicClient({ chain: viemChain, transport }),
  };
}
