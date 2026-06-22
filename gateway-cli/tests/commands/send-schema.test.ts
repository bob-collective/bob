import { describe, it, expect } from "vitest";
import { sendSchema } from "../../src/schemas.js";

describe("sendSchema", () => {
  it("parses a minimal valid send with defaults", () => {
    const out = sendSchema.parse({ asset: "BTC", amount: "0.01BTC", to: "bc1qxyz" });
    expect(out.wait).toBe(true);
    expect(out.unsigned).toBe(false);
    expect(out.json).toBe(false);
  });
  it("coerces btcFeeRate to a positive integer", () => {
    const out = sendSchema.parse({ asset: "BTC", amount: "1000", to: "bc1qxyz", btcFeeRate: "5" });
    expect(out.btcFeeRate).toBe(5);
  });
  it("rejects a non-positive btcFeeRate", () => {
    expect(() => sendSchema.parse({ asset: "BTC", amount: "1000", to: "bc1qxyz", btcFeeRate: "0" })).toThrow();
  });
});
