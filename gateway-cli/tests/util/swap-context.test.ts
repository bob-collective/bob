import { describe, it, expect } from "vitest";
import { resolveOwnerAddress, resolveRefundAddress } from "../../src/util/swap-context.js";

const EVM = "0xAF91558Ba2B1994530c9cfCcbda5AE9cD2b456bb";
const BTC = "bc1q4xdatls497ea76fmuefu9we4ld4yu2vy8hedne";
const OTHER_EVM = "0x1111111111111111111111111111111111111111";

// `ownerAddress` is the EVM address that controls the order. The gateway rejects anything
// else outright ("Invalid Ethereum address: Expected an EVM address but found a Bitcoin
// address"), so the value must be an EVM address whatever route it arrived by.
//
// The subtlety: the family of `--sender` / `--recipient` cannot be pinned by the CLI schema,
// because it depends on the swap's direction — a Bitcoin `--sender` is CORRECT on a BTC
// onramp (it builds the PSBT). So the check can only live here, where the source chain is
// known. That makes resolveOwnerAddress the sole place the invariant can be enforced, and it
// must verify rather than assume.
describe("resolveOwnerAddress", () => {
  it("uses the recipient on a BTC→EVM onramp", () => {
    expect(resolveOwnerAddress({
      srcFamily: "bitcoin", dstFamily: "evm", recipient: EVM, senderAddress: BTC,
    })).toBe(EVM);
  });

  it("accepts a Bitcoin sender on a BTC onramp — it is the PSBT's input, not the owner", () => {
    // Guards the fix direction: family-refining `--sender` at the schema would break this.
    expect(resolveOwnerAddress({
      srcFamily: "bitcoin", dstFamily: "evm", recipient: EVM, senderAddress: BTC,
    })).toBe(EVM);
  });

  it("uses the sender on an EVM→BTC offramp", () => {
    expect(resolveOwnerAddress({
      srcFamily: "evm", dstFamily: "bitcoin", recipient: BTC, senderAddress: EVM,
    })).toBe(EVM);
  });

  it("prefers an explicit --owner", () => {
    expect(resolveOwnerAddress({
      explicit: EVM, srcFamily: "evm", dstFamily: "bitcoin", recipient: BTC, senderAddress: EVM,
    })).toBe(EVM);
  });

  it("rejects a Bitcoin --sender on an EVM-source swap instead of sending it as the owner", () => {
    // Reachable on the unsigned path, where no key is derived and no sender/key mismatch
    // check runs: the BTC address would otherwise reach the gateway as `ownerAddress`.
    expect(() => resolveOwnerAddress({
      srcFamily: "evm", dstFamily: "bitcoin", recipient: BTC, senderAddress: BTC,
    })).toThrow(/owner must be an EVM address.*--sender/s);
  });

  it("rejects a Bitcoin --recipient on an onramp instead of sending it as the owner", () => {
    expect(() => resolveOwnerAddress({
      srcFamily: "bitcoin", dstFamily: "evm", recipient: BTC,
    })).toThrow(/owner must be an EVM address.*--recipient/s);
  });

  it("fails actionably when no EVM address can be determined at all", () => {
    expect(() => resolveOwnerAddress({
      srcFamily: "evm", dstFamily: "bitcoin", recipient: BTC,
    })).toThrow(/Could not determine the EVM owner address/);
  });
});

// The refund address is the mirror image of the owner: always SOURCE-chain, so Bitcoin on
// an onramp and EVM on an offramp or token swap. The gateway defaults nothing — a quote
// asked for without one comes back `refundAddress: null` even when a sender was sent — and
// answers a family mismatch with a bare `INVALID_REQUEST: Conversion Error` that names
// neither the flag nor the family. Both facts are why the resolution lives here.
describe("resolveRefundAddress", () => {
  it("defaults to the sender — the address the funds left", () => {
    expect(resolveRefundAddress({ srcChain: "ethereum", senderAddress: EVM })).toBe(EVM);
    expect(resolveRefundAddress({ srcChain: "bitcoin", senderAddress: BTC })).toBe(BTC);
  });

  it("prefers an explicit --refund-address over the sender", () => {
    expect(resolveRefundAddress({
      explicit: OTHER_EVM, srcChain: "ethereum", senderAddress: EVM,
    })).toBe(OTHER_EVM);
  });

  it("is undefined when nothing supplied one — the quote is still valid without it", () => {
    expect(resolveRefundAddress({ srcChain: "ethereum" })).toBeUndefined();
  });

  it("takes a Bitcoin address on a BTC-source onramp", () => {
    expect(resolveRefundAddress({ explicit: BTC, srcChain: "bitcoin" })).toBe(BTC);
  });

  it("rejects an EVM --refund-address on a BTC-source onramp", () => {
    expect(() => resolveRefundAddress({ explicit: EVM, srcChain: "bitcoin" }))
      .toThrow(/--refund-address .* is not a valid Bitcoin address/);
  });

  it("rejects a Bitcoin --refund-address on an EVM-source swap", () => {
    expect(() => resolveRefundAddress({ explicit: BTC, srcChain: "ethereum" }))
      .toThrow(/--refund-address .* is not a valid EVM address/);
  });
});
