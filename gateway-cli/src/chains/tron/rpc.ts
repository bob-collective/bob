import { hexToString, isAddress, isHex, type Address, type Hex } from 'viem';
import { loadConfig } from '../../config.js';
import { hexToTronAddress, isValidTronAddress, toTronContractAddress } from './addresses.js';

/** Minimum spacing between Tron node RPC calls (public nodes rate-limit bursts). */
const DEFAULT_GAP_MS = 1_000;

let gate: Promise<void> = Promise.resolve();
let lastAt = 0;

function sleep(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

function isTransientRpcError(error: unknown): boolean {
  const message = error instanceof Error ? error.message : String(error);
  const status = (error as { response?: { status?: number } })?.response?.status;
  if (status === 429 || (status != null && status >= 500)) return true;
  return /\b429\b/.test(message) || /\b5\d{2}\b/.test(message);
}

async function postTronRpc<TResponse>(
  rpcUrl: string,
  path: string,
  body: object,
): Promise<TResponse> {
  const apiKey = loadConfig().tronGridApiKey;
  const response = await fetch(new URL(path, rpcUrl), {
    body: JSON.stringify(body),
    headers: {
      'Content-Type': 'application/json',
      ...(apiKey ? { 'TRON-PRO-API-KEY': apiKey } : {}),
    },
    method: 'POST',
  });

  if (!response.ok) {
    throw new Error(`Tron RPC ${path} failed: ${response.status} ${response.statusText}`);
  }

  return (await response.json()) as TResponse;
}

/**
 * Serialize Tron RPC calls, space them apart, and retry transient 429/5xx failures.
 * Re-broadcasting the same signed tx is idempotent on Tron (same txID).
 */
export async function runTronRpc<T>(label: string, fn: () => Promise<T>, tries = 6): Promise<T> {
  const gapMs = Number(process.env.TRON_RPC_GAP_MS ?? DEFAULT_GAP_MS);

  const run = gate.then(async () => {
    for (let i = 0; i < tries; i++) {
      const wait = gapMs - (Date.now() - lastAt);
      if (wait > 0) await sleep(wait);
      try {
        return await fn();
      } catch (error) {
        if (!isTransientRpcError(error) || i === tries - 1) throw error;
        const backoff = 1_000 * 2 ** i;
        console.warn(`${label}: transient RPC error — backing off ${backoff}ms (${i + 1}/${tries})`);
        await sleep(backoff);
      } finally {
        lastAt = Date.now();
      }
    }
    throw new Error(`${label}: exhausted RPC retries`);
  });

  gate = run.then(
    () => undefined,
    () => undefined,
  );
  return run;
}

export const TRON_DEFAULT_RPC_URL = 'https://tron.api.pocket.network';

export const TRON_SEND_SIGNATURE =
  'send((uint32,bytes32,uint256,uint256,bytes,bytes,bytes),(uint256,uint256),address)';
export const TRON_APPROVE_SIGNATURE = 'approve(address,uint256)';

export const TRON_SEND_FEE_LIMIT = 1_500_000_000;
export const TRON_APPROVE_FEE_LIMIT = 200_000_000;

export type TronTransaction = {
  visible: boolean;
  txID: string;
  raw_data: Record<string, unknown>;
  raw_data_hex: string;
  [key: string]: unknown;
};

export type TronSignedTransaction = TronTransaction & {
  signature: string[];
};

type TriggerSmartContractResponse = {
  Error?: string;
  result?: { result?: boolean; message?: string };
  transaction?: TronTransaction;
};

type BroadcastTransactionResponse = {
  result?: boolean;
  txid?: string;
  message?: string;
};

function toSun(value: bigint | undefined): number {
  return Number(value ?? 0n);
}

function getRawParameter(data: Hex | undefined): string {
  return data?.slice(10) ?? '';
}

function decodeTronMessage(message: string | undefined): string | undefined {
  if (!message) return undefined;
  const hexMessage = message.startsWith('0x') ? message : `0x${message}`;
  return isHex(hexMessage) ? hexToString(hexMessage) : message;
}

function getFunctionSignature(functionName: string): string {
  return functionName === 'approve' ? TRON_APPROVE_SIGNATURE : TRON_SEND_SIGNATURE;
}

export async function triggerSmartContract(
  rpcUrl: string,
  {
    data,
    feeLimit,
    from,
    functionName,
    to,
    value,
  }: {
    from: string;
    to: Address | string;
    data?: Hex;
    value?: bigint;
    functionName: string;
    feeLimit: number;
  },
): Promise<TronTransaction> {
  const contractAddress = isAddress(to, { strict: false })
    ? hexToTronAddress(to)
    : isValidTronAddress(to)
      ? to
      : toTronContractAddress(to);

  const result = await runTronRpc('triggersmartcontract', () =>
    postTronRpc<TriggerSmartContractResponse>(rpcUrl, 'wallet/triggersmartcontract', {
    call_value: toSun(value),
    contract_address: contractAddress,
    fee_limit: feeLimit,
    function_selector: getFunctionSignature(functionName),
    owner_address: from,
    parameter: getRawParameter(data),
    visible: true,
    }),
  );

  if (result.Error) throw new Error(result.Error);
  if (result.result?.message) throw new Error(decodeTronMessage(result.result.message) ?? result.result.message);
  if (!result.result?.result || !result.transaction?.txID) {
    throw new Error('Failed to build Tron transaction');
  }

  return result.transaction;
}

export async function broadcastTransaction(
  rpcUrl: string,
  transaction: TronSignedTransaction,
): Promise<string> {
  const result = await runTronRpc('broadcasttransaction', () =>
    postTronRpc<BroadcastTransactionResponse>(
      rpcUrl,
      'wallet/broadcasttransaction',
      transaction,
    ),
  );

  if (!result.result) throw new Error(result.message ?? 'Failed to broadcast Tron transaction');
  if (!result.txid) throw new Error('Tron transaction hash missing from broadcast response');
  return result.txid;
}

export async function getTransactionReceipt(
  rpcUrl: string,
  hash: string,
  retryCount: number,
): Promise<{ status: 'success' | 'reverted' }> {
  const txId = hash.startsWith('0x') ? hash.slice(2) : hash;

  for (let attempt = 0; attempt <= retryCount; attempt++) {
    const info = await runTronRpc('gettransactioninfobyid', () =>
      postTronRpc<{
        id?: string;
        receipt?: { result?: string };
      }>(rpcUrl, 'wallet/gettransactioninfobyid', { value: txId }),
    );

    if (info.id) {
      return { status: info.receipt?.result === 'SUCCESS' ? 'success' : 'reverted' };
    }

    if (attempt < retryCount) {
      await new Promise((resolve) => setTimeout(resolve, 3_000));
    }
  }

  throw new Error(`Timed out waiting for Tron transaction ${txId}`);
}
