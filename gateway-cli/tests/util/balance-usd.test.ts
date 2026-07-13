import { describe, it } from "vitest";
import { strict as assert } from "node:assert";
import { applyUsd } from "../../src/util/balance-usd.js";

describe("applyUsd", () => {
  it("sets priceUsd and usdValue = balance × price", () => {
    const asset: { priceUsd?: number; usdValue?: number } = {};
    applyUsd(asset, "0.002", 62000);
    assert.equal(asset.priceUsd, 62000);
    assert.equal(asset.usdValue, 124); // 0.002 × 62000
  });

  it("is a no-op when the price is undefined (asset left unannotated)", () => {
    const asset: { priceUsd?: number; usdValue?: number } = {};
    applyUsd(asset, "5", undefined);
    assert.equal(asset.priceUsd, undefined);
    assert.equal(asset.usdValue, undefined);
  });

  it("handles a zero balance as a zero value", () => {
    const asset: { priceUsd?: number; usdValue?: number } = {};
    applyUsd(asset, "0", 1);
    assert.equal(asset.usdValue, 0);
  });
});
