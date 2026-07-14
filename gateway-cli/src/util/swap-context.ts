import { isAddress } from "viem";
import type { GetQuoteParams, RouteInfo } from "@gobob/bob-sdk";
import type { ChainFamily } from "../chains/index.js";
import { deriveAddress, getChainFamily, resolvePrivateKey, resolveRecipient, validateAddressFamily } from "../chains/index.js";
import { buildTokenIndex, parseAssetChain, resolveAmount, type ResolvedAsset } from "./input-resolver.js";

// ─── Types ───────────────────────────────────────────────────────────────────

/** The gateway order variant implied by the source/destination chain families. */
export type SwapVariant = "onramp" | "offramp" | "tokenSwap";

/** Options common to `quote` and `swap` — everything the context is resolved from. */
export interface SwapContextOptions {
  src: string;
  dst: string;
  amount: string;
  recipient?: string;
  sender?: string;
  owner?: string;
  slippage?: number;
  feeToken?: string;
  feeReserve?: string;
  privateKey?: string;
}

/**
 * What the *caller* independently needs out of the context, beyond what the swap
 * itself implies. Stated as dependencies rather than as per-command booleans, so
 * "is the key load-bearing here?" is answered in exactly one place ({@link
 * resolveSwapContext}) instead of being re-derived by each command.
 */
export interface SwapContextNeeds {
  /**
   * The caller will sign and broadcast the source transaction, so the private key
   * itself is load-bearing (`swap` without `--unsigned`).
   */
  signing: boolean;
  /**
   * The caller needs the source-side sender address for its own reasons, whoever
   * owns the order: `swap` sends it as `fromUserAddress`, checks its pending nonce,
   * and builds the PSBT from it. `quote` does not — so a `quote` that needs no key
   * for any *other* reason must never touch one.
   */
  senderAddress: boolean;
}

/**
 * Everything `quote` and `swap` need about a swap, resolved once.
 *
 * Both commands previously built this pipeline by hand — routes → source family →
 * key → sender → inputs → recipient → owner → quote params — and every divergence
 * between the two copies was a bug. There is now one pipeline, and both consume it.
 */
export interface SwapContext {
  srcAsset: ResolvedAsset;
  dstAsset: ResolvedAsset;
  srcFamily: ChainFamily;
  dstFamily: ChainFamily;
  /** Order variant implied by the chain families. */
  variant: SwapVariant;
  /** The EVM chain involved in the swap (the destination on an onramp, else the source). */
  evmChain: string;
  /** Source-side sender. Undefined only when nothing on this path needed it. */
  senderAddress?: string;
  /** Destination-side recipient — a Bitcoin address on an offramp. */
  recipient: string;
  /**
   * The EVM-side address that controls the order. Required by the V3 API; never a
   * Bitcoin address. See {@link resolveOwnerAddress}.
   */
  ownerAddress: string;
  /** Source amount in atomic units. */
  atomicUnits: string;
  /** Human-readable rendering of the source amount. */
  display: string;
  slippageBps: number;
  /** The signing key, resolved only when {@link SwapContextNeeds.signing} asked for it. */
  key?: string;
  /** Quote parameters, identical for `quote` and `swap` by construction. */
  quoteParams: GetQuoteParams;
}

// ─── Owner address ───────────────────────────────────────────────────────────

/**
 * The ONE definition of `ownerAddress`: the EVM-side address that controls the order.
 *
 * It can never be a Bitcoin address — and this function *enforces* that rather than
 * assuming it. Which flag supplies the value depends on the direction (the recipient on
 * a BTC-source onramp, the sender on an EVM-source swap), and neither `--recipient` nor
 * `--sender` can be pinned to a family by the CLI schema: on a BTC onramp a Bitcoin
 * `--sender` is correct (it builds the PSBT). So the family is only knowable here, where
 * the source chain is known — which makes this the only place the invariant can hold.
 *
 * The recipient is NOT a fallback for an EVM-source swap: on an offramp it is a
 * Bitcoin address, and the API rejects the quote outright with
 * `INVALID_REQUEST: Invalid Ethereum address: Expected an EVM address but found a
 * Bitcoin address`. `ownerAddress` is also mandatory, so omitting it is not an
 * option either — an undeterminable owner is a hard, actionable error.
 */
export function resolveOwnerAddress(opts: {
  explicit?: string;
  srcFamily: ChainFamily;
  dstFamily: ChainFamily;
  senderAddress?: string;
  recipient: string;
}): string {
  const picked =
    opts.explicit ? { owner: opts.explicit, from: "--owner" } :
    opts.srcFamily === "bitcoin" && opts.dstFamily === "evm" ? { owner: opts.recipient, from: "--recipient" } :
    opts.srcFamily === "evm" && opts.senderAddress ? { owner: opts.senderAddress, from: "--sender" } :
    undefined;

  if (!picked) {
    throw new Error(
      `Could not determine the EVM owner address for this swap.\n` +
      `  The order is controlled by an EVM address — on an EVM-source swap the sender, and it\n` +
      `  cannot fall back to the recipient (on an offramp the recipient is a Bitcoin address).\n` +
      `  Set EVM_PRIVATE_KEY, or pass --sender <evm-address> or --owner <evm-address>.`,
    );
  }

  // Verify, don't assume. The value reaching here can come straight from a user-supplied
  // flag that no schema could have family-checked, so the guarantee this function makes is
  // only real if it is checked. Otherwise a Bitcoin `--sender`/`--recipient` sails through
  // into `ownerAddress` and the gateway rejects it remotely instead of us failing locally.
  if (!isAddress(picked.owner, { strict: false })) {
    throw new Error(
      `The order's owner must be an EVM address, but ${picked.from} supplied "${picked.owner}".\n` +
      `  The order is controlled by an EVM address, whatever the direction of the swap.\n` +
      `  Pass an EVM address, or set --owner <evm-address> explicitly.`,
    );
  }
  return picked.owner;
}

// ─── Context ─────────────────────────────────────────────────────────────────

/**
 * Resolve a swap end to end: assets, chain families, sender, recipient, owner and
 * amount, plus the quote parameters built from them.
 *
 * The private key is touched only where it is load-bearing. A key nothing needs is
 * never read, so a malformed `EVM_PRIVATE_KEY` cannot break a quote that does not
 * need one (a BTC→EVM quote, or any quote given an explicit `--owner`/`--sender`).
 */
export async function resolveSwapContext(
  opts: SwapContextOptions,
  routes: RouteInfo[],
  config: { bitcoinPrivateKey?: string; evmPrivateKey?: string; slippageBps: number },
  needs: SwapContextNeeds,
): Promise<SwapContext> {
  // The source is resolved ONLY here, through the same resolver the rest of the CLI
  // uses. Re-parsing `opts.src` with an ad-hoc string heuristic is what previously
  // made `--src Btc` an EVM chain and made a bare `--src USDT` derive a key before
  // the asset was even validated.
  const tokenIndex = buildTokenIndex(routes);
  const srcAsset = parseAssetChain(opts.src, routes, tokenIndex);
  const dstAsset = parseAssetChain(opts.dst, routes, tokenIndex);

  const srcFamily = getChainFamily(srcAsset.chain);
  const dstFamily = getChainFamily(dstAsset.chain);
  const variant: SwapVariant = srcFamily === "bitcoin" ? "onramp" : dstFamily === "bitcoin" ? "offramp" : "tokenSwap";
  const evmChain = srcFamily === "bitcoin" ? dstAsset.chain : srcAsset.chain;

  // ── Who needs the sender, and when ─────────────────────────────────────────
  // These are the only places the sender is load-bearing. Expressed once, as
  // dependencies, rather than as hand-written booleans duplicated per command.
  const ownerNeedsSender = srcFamily === "evm" && !opts.owner;
  const balanceNeedsSender = opts.amount.trim().toUpperCase() === "ALL";
  const wantsSender = needs.senderAddress || ownerNeedsSender || balanceNeedsSender;

  // ...and therefore the only conditions under which the KEY is load-bearing: to
  // sign, or to produce a sender address nobody supplied. Anything else must not
  // touch it — reading a key that is not needed turns a malformed key into a
  // failure of an operation that never required it.
  const keyIsLoadBearing = needs.signing || (wantsSender && !opts.sender);
  const key = keyIsLoadBearing ? resolvePrivateKey(srcFamily, opts.privateKey, config) : undefined;

  if (needs.signing && !key) {
    const isBtc = srcFamily === "bitcoin";
    throw new Error(
      `no signer configured for ${isBtc ? "Bitcoin" : "EVM"}.\n` +
      `  Set ${isBtc ? "BITCOIN_PRIVATE_KEY" : "EVM_PRIVATE_KEY"} or pass --private-key.\n` +
      `  Use --unsigned to output the ${isBtc ? "PSBT" : "unsigned transaction"} without signing.`,
    );
  }

  let derivedAddress: string | undefined;
  if (key && wantsSender) {
    try {
      derivedAddress = await deriveAddress(srcFamily, key);
    } catch (err) {
      const envVar = srcFamily === "bitcoin" ? "BITCOIN_PRIVATE_KEY" : "EVM_PRIVATE_KEY";
      throw new Error(
        `Failed to derive sender address from ${envVar}: ${err instanceof Error ? err.message : String(err)}\n` +
        `  Use --sender <address> to provide the address directly.`,
      );
    }
  }

  const senderAddress = opts.sender ?? derivedAddress;

  // A --sender that disagrees with the signing key would create the order for one
  // address and sign it with another.
  if (opts.sender && derivedAddress && opts.sender.toLowerCase() !== derivedAddress.toLowerCase()) {
    throw new Error(
      `--sender ${opts.sender} does not match the signing key (${derivedAddress}).\n` +
      `  The order would be created for one address but signed by another.\n` +
      `  Remove --sender to use the key-derived address, or use --unsigned to skip signing.`,
    );
  }

  const { atomicUnits, display } = await resolveAmount(srcAsset, opts.amount, {
    senderAddress, feeToken: opts.feeToken, feeReserve: opts.feeReserve,
  });

  const recipient = await resolveRecipient(dstAsset.chain, opts.recipient, config);
  // An explicit --recipient is taken verbatim, so check it belongs to the destination's
  // family. Without this a Bitcoin --recipient on an EVM destination reaches the gateway as
  // `toUserAddress` (and, on an onramp, as `ownerAddress`) and comes back as a remote 400
  // instead of a local error naming the flag. `send` already validates this way.
  validateAddressFamily(dstAsset.chain, recipient, "--recipient");

  const ownerAddress = resolveOwnerAddress({
    explicit: opts.owner, srcFamily, dstFamily, senderAddress, recipient,
  });

  const slippageBps = opts.slippage ?? config.slippageBps;

  return {
    srcAsset, dstAsset, srcFamily, dstFamily, variant, evmChain,
    senderAddress, recipient, ownerAddress, atomicUnits, display, slippageBps, key,
    quoteParams: {
      fromChain: srcAsset.chain,
      toChain: dstAsset.chain,
      fromToken: srcAsset.address,
      toToken: dstAsset.address,
      toUserAddress: recipient,
      fromUserAddress: senderAddress,
      ownerAddress,
      amount: atomicUnits,
      maxSlippage: slippageBps,
    },
  };
}
