import type { ChainBalance } from '../evm.js';
import { loadConfig } from '../../config.js';
import { createTronClients } from './clients.js';
import { deriveTronAddress, toTronContractAddress } from './addresses.js';
import { TRON_DEFAULT_RPC_URL } from './rpc.js';
import { createTronWeb } from './transactions.js';
import { erc20Abi } from 'viem';

/** Conservative TRX reserve (in sun) for energy/bandwidth when estimating spendable TRX. */
export const TRON_FEE_BUFFER_SUN = 50_000_000n;

function getTronRpcUrl(): string {
  return loadConfig().tronRpcUrl ?? TRON_DEFAULT_RPC_URL;
}

export { deriveTronAddress };

export async function resolveTronSigner(privateKey: string): Promise<{
  address: string;
  walletClient: Awaited<ReturnType<typeof createTronClients>>['walletClient'];
  publicClient: Awaited<ReturnType<typeof createTronClients>>['publicClient'];
}> {
  const config = loadConfig();
  const rpcUrl = config.tronRpcUrl ?? TRON_DEFAULT_RPC_URL;
  const { address, walletClient, publicClient } = createTronClients(
    privateKey,
    rpcUrl,
    config.tronGridApiKey,
  );
  return { address, walletClient, publicClient };
}

export async function getTronBalances(
  address: string,
  tokens?: Array<{ address: string; symbol: string; decimals: number }>,
  opts?: { includeNative?: boolean },
): Promise<ChainBalance> {
  const config = loadConfig();
  const rpcUrl = getTronRpcUrl();
  const tronWeb = createTronWeb({ rpcUrl, apiKey: config.tronGridApiKey });
  const hasNative = opts?.includeNative ?? false;
  const erc20s = tokens ?? [];

  let native: ChainBalance['native'];
  if (hasNative) {
    const balanceSun = BigInt(await tronWeb.trx.getBalance(address));
    const spendable = balanceSun > TRON_FEE_BUFFER_SUN ? balanceSun - TRON_FEE_BUFFER_SUN : 0n;
    native = {
      symbol: 'TRX',
      decimals: 6,
      balance: balanceSun.toString(),
      allSpendable: spendable.toString(),
    };
  }

  const tokenBals: ChainBalance['tokens'] = [];
  for (const token of erc20s) {
    try {
      const contractAddress = toTronContractAddress(token.address);
      const contract = tronWeb.contract(erc20Abi as never, contractAddress);
      const bal = await contract.balanceOf(address).call();
      const balance = BigInt(bal.toString());
      tokenBals.push({
        symbol: token.symbol,
        address: token.address,
        decimals: token.decimals,
        balance: balance.toString(),
        allSpendable: balance.toString(),
      });
    } catch (err) {
      console.warn(
        `Warning: balanceOf call failed for token ${token.symbol} (${token.address}): ${
          err instanceof Error ? err.message : String(err)
        }`,
      );
      tokenBals.push({
        symbol: token.symbol,
        address: token.address,
        decimals: token.decimals,
        balance: '0',
        allSpendable: '0',
      });
    }
  }

  return {
    address,
    native,
    tokens: tokenBals.length > 0 ? tokenBals : undefined,
  };
}
