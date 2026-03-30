import { getChainConfig } from "@gobob/bob-sdk";

// ─── Types ──────────────────────────────────────────────────────────────────

/** Chainlist.org RPC entry structure. */
interface ChainlistEntry {
  chainId: number;
  rpc: Array<{ url: string; tracking?: string } | string>;
}

// ─── Constants ──────────────────────────────────────────────────────────────

/** URL to fetch chain RPC data from chainlist.org. */
const CHAINLIST_URL = "https://chainlist.org/rpcs.json";
/** Timeout for individual RPC probe requests (3 seconds). */
const PROBE_TIMEOUT_MS = 3000;
/** Maximum number of candidate RPC URLs to probe per chain. */
const MAX_CANDIDATES = 3;

// ─── In-memory memoization (per-process) ────────────────────────────────────

/** Cache of successfully probed RPC URLs by chain ID. */
const winners = new Map<number, string>();

// ─── Chainlist fetch ───────────────────────────────────────────────────────

/**
 * Fetch chain RPC data from chainlist.org.
 * Returns null on any error (network, timeout, parse failure).
 */
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

/**
 * Extract candidate RPC URLs from chainlist data for a specific chain.
 * Filters to HTTPS-only URLs without template variables.
 * @param entries - Full chainlist data
 * @param chainId - Target chain ID to find
 * @returns Up to MAX_CANDIDATES RPC URLs
 */
function getCandidateUrls(entries: ChainlistEntry[], chainId: number): string[] {
  const chain = entries.find(e => e.chainId === chainId);
  if (!chain) return [];

  return chain.rpc
    .map(r => typeof r === "string" ? r : r.url)
    .filter(url => url.startsWith("https://") && !url.includes("${"))
    .slice(0, MAX_CANDIDATES);
}

/**
 * Probe an RPC URL to verify it's working and returns the expected chain ID.
 * @param url - RPC URL to test
 * @param expectedChainId - Expected chain ID to validate response
 * @returns URL and latency if successful, null otherwise
 */
async function probeRpc(url: string, expectedChainId: number): Promise<{ url: string; latency: number } | null> {
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
    // Validate the returned chain ID matches what we expect
    if (parseInt(json.result, 16) !== expectedChainId) return null;
    return { url, latency: Date.now() - start };
  } catch {
    return null;
  }
}

/**
 * Race multiple RPC probes in parallel, returning the first successful URL.
 * Uses Promise.any to fail fast when any probe succeeds.
 * @param urls - Candidate RPC URLs to probe
 * @param chainId - Expected chain ID for validation
 * @returns First successful URL, or undefined if all fail
 */
async function raceCandidates(urls: string[], chainId: number): Promise<string | undefined> {
  if (urls.length === 0) return undefined;

  // Race all probes — first successful response wins
  const result = await Promise.any(
    urls.map(url => probeRpc(url, chainId).then(r => {
      if (!r) throw new Error("probe failed");
      return r.url;
    }))
  ).catch(() => undefined);

  return result;
}

// ─── Public API ─────────────────────────────────────────────────────────────

/**
 * Resolve the best RPC URL for a chain by name.
 * 
 * Resolution priority:
 * 1. Environment variable (EVM_RPC_URL_<CHAIN>)
 * 2. In-memory cache of previously probed URLs
 * 3. Chainlist.org probe (fastest responding HTTPS endpoint)
 * 4. undefined (falls back to viem defaults)
 * 
 * @param chainName - Chain name (e.g., "ethereum", "base")
 * @returns RPC URL or undefined if unavailable
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
  const winner = await raceCandidates(candidates, chainId);

  if (winner) {
    winners.set(chainId, winner);
  }

  return winner;
}
