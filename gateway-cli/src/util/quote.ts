import type {
  GatewayQuoteV3,
  GatewayOnrampQuoteV2,
  GatewayOfframpQuoteV3,
  GatewayTokenSwapQuoteV2,
} from "@gobob/bob-sdk";
import { getChainFamily } from "../chains/index.js";

/**
 * V3 equivalent of the SDK's `getInnerQuote`. Picks the active variant
 * (onramp / offramp / tokenSwap) from a `GatewayQuoteV3` discriminated union.
 *
 * The SDK's exported `getInnerQuote` is typed against a different quote union
 * (onramp / offramp / layerZero), so we reproduce the unwrap here against the
 * V3 types actually returned by `sdk.getQuote`. Onramp and tokenSwap still
 * carry V2 inner shapes; offramp moved to the V3 inner shape.
 */
export type InnerQuoteV3 = GatewayOnrampQuoteV2 | GatewayOfframpQuoteV3 | GatewayTokenSwapQuoteV2;

export function getInnerQuoteV3(quote: GatewayQuoteV3): InnerQuoteV3 {
  if ("onramp" in quote) return quote.onramp;
  if ("offramp" in quote) return quote.offramp;
  if ("tokenSwap" in quote) return quote.tokenSwap;
  throw new Error("Unknown quote variant");
}

/**
 * Resolve the `ownerAddress` required by the V3 API: the EVM-side address that
 * controls the order. It is the recipient on an onramp (BTC→EVM) and the sender
 * on an offramp/tokenSwap (EVM→*).
 *
 * When the source is EVM the recipient is never a valid owner: on an offramp it
 * is a Bitcoin address, which the API rejects with
 * `INVALID_REQUEST: Invalid Ethereum address: Expected an EVM address but found
 * a Bitcoin address`. Fail with an actionable error instead of substituting it.
 */
export function resolveOwnerAddress(opts: {
  explicit?: string;
  srcChain: string;
  senderAddress?: string;
  recipient: string;
}): string {
  if (opts.explicit) return opts.explicit;
  if (getChainFamily(opts.srcChain) === "bitcoin") return opts.recipient;
  if (opts.senderAddress) return opts.senderAddress;
  throw new Error(
    `Could not determine the EVM owner address for this ${opts.srcChain} swap.\n` +
    `  The order is controlled by the EVM sender, so it cannot fall back to the recipient.\n` +
    `  Set EVM_PRIVATE_KEY, or pass --sender <evm-address> or --owner <evm-address>.`,
  );
}
