import { describe, it } from "vitest";
import { strict as assert } from "node:assert";
import { annotateBalancesUsd } from "../../src/util/balance-usd.js";
import type { BalanceJson } from "../../src/output.js";

describe("annotateBalancesUsd", () => {
  it("annotates the top-level bitcoin balance with priceUsd and usdValue", () => {
    const input: BalanceJson = {
      bitcoin: { address: "bc1qxyz", balance: "0.01", allSpendable: "0.009" },
    };
    const out = annotateBalancesUsd(input, { BTC: 60000 });
    assert.equal(out.bitcoin.priceUsd, 60000);
    assert.equal(out.bitcoin.usdValue, 600); // 0.01 * 60000
  });

  it("annotates native and each token by symbol", () => {
    const input: BalanceJson = {
      bsc: {
        address: "0xabc",
        native: { symbol: "BNB", balance: "0.5" },
        tokens: [
          { symbol: "BTCB", address: "0x7130", balance: "0.002" },
          { symbol: "USDT", address: "0x5539", balance: "40" },
        ],
      },
    };
    const out = annotateBalancesUsd(input, { BNB: 600, BTCB: 62000, USDT: 1 });
    assert.equal(out.bsc.native!.priceUsd, 600);
    assert.equal(out.bsc.native!.usdValue, 300); // 0.5 * 600
    assert.equal(out.bsc.tokens![0].usdValue, 124); // 0.002 * 62000
    assert.equal(out.bsc.tokens![1].usdValue, 40); // 40 * 1
  });

  it("leaves an asset unannotated when no price is available", () => {
    const input: BalanceJson = {
      bsc: {
        address: "0xabc",
        tokens: [{ symbol: "MYSTERY", address: "0xdead", balance: "5" }],
      },
    };
    const out = annotateBalancesUsd(input, { BTC: 60000 });
    assert.equal(out.bsc.tokens![0].priceUsd, undefined);
    assert.equal(out.bsc.tokens![0].usdValue, undefined);
  });

  it("does not mutate the input object", () => {
    const input: BalanceJson = {
      bitcoin: { address: "bc1qxyz", balance: "0.01" },
    };
    annotateBalancesUsd(input, { BTC: 60000 });
    assert.equal((input.bitcoin as { priceUsd?: number }).priceUsd, undefined);
  });

  it("skips chains that errored", () => {
    const input: BalanceJson = {
      base: { address: "0xabc", error: true },
    };
    const out = annotateBalancesUsd(input, { BTC: 60000 });
    assert.deepEqual(out.base, { address: "0xabc", error: true });
  });
});
