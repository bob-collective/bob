import { createPublicClient, createWalletClient, http, erc20Abi, isAddressEqual, isHex, type WalletClient, type PublicClient, type Hex, type Chain } from 'viem';
import { privateKeyToAccount } from 'viem/accounts';
import { supportedChainsMapping } from '@gobob/bob-sdk';
import { getNativeToken } from '../util/route-provider.js';

/** Resolve RPC URL from env var EVM_RPC_URL_<CHAIN>, or undefined for viem defaults. */
export function resolveRpcUrl(chainName: string): string | undefined {
  return process.env[`EVM_RPC_URL_${chainName.toUpperCase()}`];
}

/** Get the viem Chain object for a gateway chain name, if supported. */
export function getViemChain(chainName: string): Chain | undefined {
  return (supportedChainsMapping as Record<string, Chain>)[chainName];
}
import type { TokenBalance } from './index.js';

export const NATIVE_GAS_BUFFER = 900_000n;

export function isNativeToken(chain: string, symbol: string): boolean {
  const nt = getNativeToken(chain);
  return symbol.toUpperCase() === nt.symbol.toUpperCase();
}

export async function getEvmNativeBalance(
  chain: string,
  address: string,
): Promise<TokenBalance> {
  const client = createPublicClient({ chain: getViemChain(chain), transport: http(resolveRpcUrl(chain)) });
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
  decimals: number,
  feeToken?: string,
  feeReserve?: string,
): Promise<TokenBalance> {
  const client = createPublicClient({ chain: getViemChain(chain), transport: http(resolveRpcUrl(chain)) });
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
