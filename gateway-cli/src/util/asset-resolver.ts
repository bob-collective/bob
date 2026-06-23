import { isAddress, createPublicClient, http, erc20Abi, zeroAddress } from "viem";
import { getChainConfig } from "@gobob/bob-sdk";
import tokenlistJson from "@gobob/tokenlist/tokenlist.json";
import type { ResolvedAsset } from "./input-resolver.js";
import { resolveChain } from "./input-resolver.js";
import { CHAIN_IDS, getNativeToken } from "../chains/evm.js";
import { resolveRpcUrl } from "./rpc-resolver.js";
import { BTC_DECIMALS } from "../config.js";

/** Token metadata index keyed by `${chainId}:SYMBOL` and `${chainId}:address` (lowercase). */
type Meta = { address: string; symbol: string; decimals: number };
let _index: Map<string, Meta> | null = null;
function tokenlistIndex(): Map<string, Meta> {
  if (_index) return _index;
  const idx = new Map<string, Meta>();
  for (const t of tokenlistJson.tokens as Array<{ address: string; symbol: string; decimals: number; chainId: number }>) {
    const meta: Meta = { address: t.address, symbol: t.symbol, decimals: t.decimals };
    idx.set(`${t.chainId}:${t.symbol.toUpperCase()}`, meta);
    idx.set(`${t.chainId}:${t.address.toLowerCase()}`, meta);
  }
  _index = idx;
  return idx;
}

export interface AssetResolverDeps {
  /** Resolve a token by chain + (uppercase symbol | lowercase address). Returns undefined if unknown. */
  lookupToken(chain: string, key: string): Meta | undefined;
  /** Read symbol/decimals on-chain for an arbitrary ERC20 address. */
  readOnChainToken(chain: string, address: string): Promise<{ symbol: string; decimals: number }>;
}

function defaultLookupToken(chain: string, key: string): Meta | undefined {
  const chainId = CHAIN_IDS[chain];
  if (chainId === undefined) return undefined;
  return tokenlistIndex().get(`${chainId}:${key}`);
}

async function defaultReadOnChainToken(chain: string, address: string): Promise<{ symbol: string; decimals: number }> {
  const client = createPublicClient({ chain: getChainConfig(chain), transport: http(await resolveRpcUrl(chain)) });
  const addr = address as `0x${string}`;
  try {
    const [decimals, symbol] = await Promise.all([
      client.readContract({ address: addr, abi: erc20Abi, functionName: "decimals" }),
      client.readContract({ address: addr, abi: erc20Abi, functionName: "symbol" }),
    ]);
    return { symbol, decimals };
  } catch (err) {
    throw new Error(`could not read ERC20 metadata for ${address} on "${chain}" — is it a valid token contract? ${err instanceof Error ? err.message : String(err)}`);
  }
}

/** Resolve a `send` asset string into a ResolvedAsset. Route-independent (unlike input-resolver's parseAssetChain). */
export async function resolveSendAsset(raw: string, deps?: Partial<AssetResolverDeps>): Promise<ResolvedAsset> {
  const lookupToken = deps?.lookupToken ?? defaultLookupToken;
  const readOnChainToken = deps?.readOnChainToken ?? defaultReadOnChainToken;

  const colon = raw.indexOf(":");
  const assetPart = colon === -1 ? raw : raw.slice(0, colon);
  const chainPart = colon === -1 ? undefined : raw.slice(colon + 1);

  // BTC with no chain, or explicitly BTC:bitcoin (incl. the "btc" alias)
  if (assetPart.toUpperCase() === "BTC" && (!chainPart || resolveChain(chainPart) === "bitcoin")) {
    return { chain: "bitcoin", address: zeroAddress, symbol: "BTC", decimals: BTC_DECIMALS };
  }
  if (!chainPart) {
    throw new Error(`chain required for token "${assetPart}".\n  Specify a chain, e.g. ${assetPart}:base`);
  }
  const chain = resolveChain(chainPart);

  // Native token by symbol (ETH, BNB, ...)
  try {
    const native = getNativeToken(chain);
    if (assetPart.toUpperCase() === native.symbol.toUpperCase()) {
      return { chain, address: zeroAddress, symbol: native.symbol, decimals: native.decimals };
    }
  } catch {
    // unknown chain for native lookup — fall through to token resolution / errors below
  }

  if (isAddress(assetPart, { strict: false })) {
    if (chain === "bitcoin") {
      throw new Error(`"${chain}" does not support EVM token addresses — only BTC can be sent on bitcoin.`);
    }
    const hit = lookupToken(chain, assetPart.toLowerCase());
    if (hit) return { chain, address: assetPart, symbol: hit.symbol, decimals: hit.decimals };
    const onchain = await readOnChainToken(chain, assetPart);
    return { chain, address: assetPart, symbol: onchain.symbol, decimals: onchain.decimals };
  }

  const hit = lookupToken(chain, assetPart.toUpperCase());
  if (!hit) {
    throw new Error(`unknown token "${assetPart}" on chain "${chain}".\n  Use a known symbol, a raw 0x address, or run 'gateway-cli routes --tokens ${chain}'.`);
  }
  return { chain, address: hit.address, symbol: hit.symbol, decimals: hit.decimals };
}
