const MEMPOOL_BASE = process.env.MEMPOOL_API_URL ?? "https://mempool.space/api";

export async function fetchFeeRate(): Promise<number> {
  try {
    const res = await fetch(`${MEMPOOL_BASE}/v1/fees/recommended`, {
      signal: AbortSignal.timeout(5000),
    });
    if (!res.ok) throw new Error(`HTTP ${res.status}`);
    const data = (await res.json()) as { fastestFee: number };
    return data.fastestFee;
  } catch (err) {
    throw new Error(`could not fetch fee rate from mempool.space: ${err instanceof Error ? err.message : err}`);
  }
}

export async function findPendingMempoolTx(btcAddress: string): Promise<string | null> {
  try {
    const res = await fetch(`${MEMPOOL_BASE}/address/${encodeURIComponent(btcAddress)}/txs/mempool`, {
      signal: AbortSignal.timeout(5000),
    });
    if (!res.ok) return null;
    const txs = (await res.json()) as Array<{ txid: string; status: { confirmed: boolean } }>;
    const pending = txs.find((tx) => !tx.status.confirmed);
    return pending?.txid ?? null;
  } catch {
    return null;
  }
}
