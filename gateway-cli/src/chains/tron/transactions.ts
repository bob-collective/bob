import { TronWeb } from 'tronweb';
import type { Hex } from 'viem';
import { hexToTronAddress, isValidTronAddress, toTronContractAddress } from './addresses.js';
import { runTronRpc } from './rpc.js';
import {
  TRON_APPROVE_FEE_LIMIT,
  TRON_APPROVE_SIGNATURE,
  TRON_SEND_FEE_LIMIT,
  TRON_SEND_SIGNATURE,
  type TronSignedTransaction,
  type TronTransaction,
} from './rpc.js';

export type TronCall = {
  contract: string;
  signature: string;
  options: {
    feeLimit: number;
    callValue?: number;
    rawParameter?: string;
  };
  params: Array<{ type: string; value: string | number }>;
};

function toBase58(tronWeb: TronWeb, address: string): string {
  if (isValidTronAddress(address)) return address;
  if (address.startsWith('0x')) return tronWeb.address.fromHex(`41${address.slice(2)}`);
  return toTronContractAddress(address);
}

function decodeRevertMessage(tronWeb: TronWeb, message: string | undefined): string {
  if (!message) return 'unknown revert';
  try {
    return TronWeb.toUtf8(message);
  } catch {
    return message;
  }
}

/** Constant call: simulate on-node without broadcasting or spending TRX. */
export async function preflightTronCall(
  tronWeb: TronWeb,
  ownerAddress: string,
  call: TronCall,
): Promise<{ energyUsed: number }> {
  let res: {
    result?: { result?: boolean; message?: string };
    energy_used?: number;
  };

  try {
    res = await runTronRpc('preflight', () =>
      tronWeb.transactionBuilder.triggerConstantContract(
        call.contract,
        call.signature,
        {
          feeLimit: call.options.feeLimit,
          callValue: call.options.callValue ?? 0,
          ...(call.options.rawParameter ? { rawParameter: call.options.rawParameter } : {}),
        },
        call.params,
        ownerAddress,
      ),
    );
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error);
    if (/revert/i.test(message)) {
      throw new Error(`Tron preflight reverted (tx would fail): ${message}`);
    }
    throw error;
  }

  if (res?.result?.result !== true) {
    const msg = res?.result?.message
      ? decodeRevertMessage(tronWeb, res.result.message)
      : JSON.stringify(res?.result);
    throw new Error(`Tron preflight reverted (tx would fail): ${msg}`);
  }

  const energyUsed = res.energy_used ?? 0;
  const callValue = call.options.callValue ?? 0;
  const energyPriceSun = Number(process.env.TRON_ENERGY_PRICE ?? 210);
  const trxSun = Number(await runTronRpc('getBalance', () => tronWeb.trx.getBalance(ownerAddress)));
  const need = callValue + energyUsed * energyPriceSun;

  if (trxSun < need) {
    console.warn(
      `Warning: liquid TRX may be too low for this Tron tx ` +
        `(balance ${(trxSun / 1e6).toFixed(6)} TRX, need ~${(need / 1e6).toFixed(6)} TRX incl. energy). ` +
        `Top up TRX or stake for energy before retrying.`,
    );
  }

  return { energyUsed };
}

/** Build a smart-contract tx and ensure fee_limit is present (degraded nodes may omit it). */
export async function buildTronTransaction(
  tronWeb: TronWeb,
  ownerAddress: string,
  call: TronCall,
  tries = 4,
): Promise<TronTransaction> {
  for (let i = 0; i < tries; i++) {
    const { transaction } = await runTronRpc('build', () =>
      tronWeb.transactionBuilder.triggerSmartContract(
        call.contract,
        call.signature,
        {
          feeLimit: call.options.feeLimit,
          callValue: call.options.callValue ?? 0,
          ...(call.options.rawParameter ? { rawParameter: call.options.rawParameter } : {}),
        },
        call.params,
        ownerAddress,
      ),
    );

    const feeLimit = (transaction?.raw_data as { fee_limit?: number } | undefined)?.fee_limit;
    if (transaction?.txID && feeLimit != null && feeLimit > 0) {
      return transaction as unknown as TronTransaction;
    }

    console.warn(`Tron build returned no fee_limit — rebuilding (${i + 1}/${tries})`);
  }

  throw new Error(
    'Tron node kept returning a transaction without fee_limit (would fail OUT_OF_ENERGY). ' +
      'Set TRONGRID_API_KEY or TRON_RPC_URL to a non-rate-limited endpoint and retry.',
  );
}

export function approveCall(
  tronWeb: TronWeb,
  tokenAddress: string,
  spenderAddress: string,
  amount: bigint,
): TronCall {
  return {
    contract: toBase58(tronWeb, tokenAddress),
    signature: TRON_APPROVE_SIGNATURE,
    options: { feeLimit: TRON_APPROVE_FEE_LIMIT },
    params: [
      { type: 'address', value: toBase58(tronWeb, spenderAddress) },
      { type: 'uint256', value: amount.toString() },
    ],
  };
}

export function sendCall(
  tronWeb: TronWeb,
  to: string,
  data: Hex,
  value: bigint,
  feeLimit = TRON_SEND_FEE_LIMIT,
): TronCall {
  return {
    contract: toBase58(tronWeb, to),
    signature: TRON_SEND_SIGNATURE,
    options: {
      feeLimit,
      callValue: Number(value),
      rawParameter: data.replace(/^0x/, '').slice(8),
    },
    params: [],
  };
}

export async function signAndBroadcastTronTransaction(
  tronWeb: TronWeb,
  transaction: TronTransaction,
  privateKey: Hex,
): Promise<string> {
  const signed = (await tronWeb.trx.sign(transaction as never, privateKey)) as TronSignedTransaction;
  const receipt = await runTronRpc('broadcast', () =>
    tronWeb.trx.sendRawTransaction(signed as never),
  );
  if (!receipt.result || !receipt.txid) {
    throw new Error(receipt.message ?? 'Failed to broadcast Tron transaction');
  }
  return receipt.txid;
}

export function createTronWeb(options: {
  rpcUrl: string;
  privateKey?: Hex;
  apiKey?: string;
}): TronWeb {
  return new TronWeb({
    fullHost: options.rpcUrl,
    privateKey: options.privateKey,
    headers: options.apiKey ? { 'TRON-PRO-API-KEY': options.apiKey } : undefined,
  });
}
