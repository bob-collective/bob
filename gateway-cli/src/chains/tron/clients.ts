import {
  type Abi,
  type Address,
  type Hash,
  type Hex,
  type PublicClient,
  type Transport,
  type WalletClient,
  encodeFunctionData,
  erc20Abi,
  type Chain as ViemChain,
  type Account,
} from 'viem';
import { privateKeyToAccount } from 'viem/accounts';
import { normalizePrivateKey, toTronContractAddress } from './addresses.js';
import { getTransactionReceipt } from './rpc.js';
import {
  approveCall,
  buildTronTransaction,
  createTronWeb,
  preflightTronCall,
  sendCall,
  signAndBroadcastTronTransaction,
} from './transactions.js';

type WriteContractRequest = {
  abi: Abi;
  address: Address;
  args?: readonly unknown[];
  functionName: string;
  value?: bigint;
};

export function createTronClients(
  privateKey: string,
  rpcUrl: string,
  apiKey?: string,
): { address: string; walletClient: WalletClient<Transport, ViemChain, Account>; publicClient: PublicClient<Transport> } {
  const hexKey = normalizePrivateKey(privateKey);
  const viemAccount = privateKeyToAccount(hexKey);
  const tronWeb = createTronWeb({ rpcUrl, privateKey: hexKey, apiKey });
  const base58Address = tronWeb.defaultAddress.base58 as string;

  if (!base58Address) {
    throw new Error('Failed to derive Tron address from private key');
  }

  async function broadcastPrepared(call: ReturnType<typeof approveCall>): Promise<Hash> {
    await preflightTronCall(tronWeb, base58Address, call);
    const transaction = await buildTronTransaction(tronWeb, base58Address, call);
    const txid = await signAndBroadcastTronTransaction(tronWeb, transaction, hexKey);
    return `0x${txid}` as Hash;
  }

  const publicClient = {
    async readContract({
      address,
      abi,
      functionName,
      args,
    }: {
      address: Address;
      abi: Abi;
      functionName: string;
      args?: readonly unknown[];
    }): Promise<unknown> {
      const contractAddress = toTronContractAddress(address);
      const contract = tronWeb.contract(abi as never, contractAddress);
      const method = (contract as Record<string, { (...a: unknown[]): Promise<unknown> }>)[functionName];
      if (!method) throw new Error(`Contract method ${functionName} not found`);
      const result = args?.length ? await method(...args) : await method();
      if (typeof result === 'bigint') return result;
      if (typeof result === 'number') return BigInt(result);
      if (typeof result === 'string' && /^\d+$/.test(result)) return BigInt(result);
      if (typeof result === 'boolean') return result;
      return result;
    },

    async simulateContract({
      address,
      abi,
      functionName,
      args,
    }: {
      account: Address | Account;
      address: Address;
      abi: Abi;
      functionName: string;
      args: readonly unknown[];
    }): Promise<{ request: WriteContractRequest }> {
      const [spender, amount] = args as [Address, bigint];
      const call = approveCall(tronWeb, address, spender, amount);
      await preflightTronCall(tronWeb, base58Address, call);
      return {
        request: {
          abi,
          address,
          functionName,
          args,
          value: 0n,
          // carry encoded data for logging/debug; writeContract re-encodes from abi/args
        },
      };
    },

    async waitForTransactionReceipt({
      hash,
      retryCount,
    }: {
      hash: Hash;
      retryCount?: number;
    }): Promise<{ status: 'success' | 'reverted' }> {
      return getTransactionReceipt(rpcUrl, hash, retryCount ?? 8);
    },
  } as unknown as PublicClient<Transport>;

  const walletClient = {
    account: {
      ...viemAccount,
      address: base58Address as Address,
      type: 'local' as const,
    },

    async writeContract(request: WriteContractRequest): Promise<Hash> {
      const [spender, amount] = (request.args ?? []) as [Address, bigint];
      const call = approveCall(tronWeb, request.address, spender, amount);
      return broadcastPrepared(call);
    },

    async sendTransaction({
      data,
      to,
      value,
    }: {
      account: Address | Account;
      to: Address;
      data: Hex;
      value?: bigint;
    }): Promise<Hash> {
      const call = sendCall(tronWeb, to, data, value ?? 0n);
      return broadcastPrepared(call);
    },
  } as unknown as WalletClient<Transport, ViemChain, Account>;

  return { address: base58Address, walletClient, publicClient };
}
