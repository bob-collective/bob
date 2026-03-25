import { getChainConfig } from "@gobob/bob-sdk";

// ─── Types ──────────────────────────────────────────────────────────────────

interface ChainlistEntry {
  chainId: number;
  rpc: Array<{ url: string; tracking?: string } | string>;
}

// ─── Constants ──────────────────────────────────────────────────────────────

const CHAINLIST_URL = "https://chainlist.org/rpcs.json";
const PROBE_TIMEOUT_MS = 3000;
const MAX_CANDIDATES = 3;

// ─── In-memory memoization (per-process) ────────────────────────────────────

const winners = new Map<number, string>();

// ─── Chainlist fetch ───────────────────────────────────────────────────────

async function fetchChainlist(): Promise<ChainlistEntry[] | null> {
  try {
    const res = await fetch(CHAINLIST_URL, { signal: AbortSignal.timeout(10_000) });
    if (!res.ok) return null;
    const data = await res.json();
    return Array.isArray(data) ? data : null;
  } catch {
    return null;
  }
}

// ─── RPC probing ────────────────────────────────────────────────────────────

function getCandidateUrls(entries: ChainlistEntry[], chainId: number): string[] {
  const chain = entries.find(e => e.chainId === chainId);
  if (!chain) return [];

  return chain.rpc
    .map(r => typeof r === "string" ? r : r.url)
    .filter(url => url.startsWith("https://") && !url.includes("${"))
    .slice(0, MAX_CANDIDATES);
}

async function probeRpc(url: string): Promise<{ url: string; latency: number } | null> {
  const start = Date.now();
  try {
    const res = await fetch(url, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ jsonrpc: "2.0", method: "eth_chainId", params: [], id: 1 }),
      signal: AbortSignal.timeout(PROBE_TIMEOUT_MS),
    });
    if (!res.ok) return null;
    const json = await res.json();
    if (!json.result) return null;
    return { url, latency: Date.now() - start };
  } catch {
    return null;
  }
}

async function raceCandidates(urls: string[]): Promise<string | undefined> {
  if (urls.length === 0) return undefined;

  // Race all probes — first successful response wins
  const result = await Promise.any(
    urls.map(url => probeRpc(url).then(r => {
      if (!r) throw new Error("probe failed");
      return r.url;
    }))
  ).catch(() => undefined);

  return result;
}

// ─── Public API ─────────────────────────────────────────────────────────────

/**
 * Resolve the best RPC URL for a chain by name.
 * Priority: env var > chainlist probe winner > undefined (viem default).
 */
export async function resolveRpcUrl(chainName: string): Promise<string | undefined> {
  // 1. Explicit env var override always wins
  const envUrl = process.env[`EVM_RPC_URL_${chainName.toUpperCase()}`];
  if (envUrl) return envUrl;

  // 2. Resolve chain ID from bob-sdk config
  let chainId: number;
  try {
    chainId = getChainConfig(chainName).id;
  } catch {
    return undefined; // unknown chain — let viem handle it
  }

  // 3. Check in-memory cache (already probed this process)
  if (winners.has(chainId)) return winners.get(chainId);

  // 4. Fetch chainlist data and probe candidates
  const entries = await fetchChainlist();
  if (!entries) return undefined;

  const candidates = getCandidateUrls(entries, chainId);
  const winner = await raceCandidates(candidates);

  if (winner) {
    winners.set(chainId, winner);
  }

  return winner;
}
