// src/util/chain-ids.test.ts
import { describe, test, expect } from "vitest";
import { resolveChain } from "../../src/util/chain-ids.js";

describe("resolveChain", () => {
  test("canonical names pass through unchanged", () => {
    expect(resolveChain("ethereum")).toBe("ethereum");
    expect(resolveChain("bitcoin")).toBe("bitcoin");
    expect(resolveChain("bob")).toBe("bob");
  });

  test("aliases resolve to canonical", () => {
    expect(resolveChain("eth")).toBe("ethereum");
    expect(resolveChain("mainnet")).toBe("ethereum");
    expect(resolveChain("arb")).toBe("arbitrum");
    expect(resolveChain("arb1")).toBe("arbitrum");
    expect(resolveChain("arbitrum-one")).toBe("arbitrum");
    expect(resolveChain("bas")).toBe("base");
    expect(resolveChain("opt")).toBe("optimism");
    expect(resolveChain("oeth")).toBe("optimism");
    expect(resolveChain("pol")).toBe("polygon");
    expect(resolveChain("btc")).toBe("bitcoin");
    expect(resolveChain("bnb")).toBe("bsc");
    expect(resolveChain("avax")).toBe("avalanche");
  });

  test("case-insensitive", () => {
    expect(resolveChain("ETH")).toBe("ethereum");
    expect(resolveChain("Ethereum")).toBe("ethereum");
  });

  test("unknown chain passes through when no knownChains provided", () => {
    expect(resolveChain("eth-mainnet")).toBe("eth-mainnet");
  });

  test("throws with knownChains when chain not found", () => {
    expect(() => resolveChain("eth-mainnet", ["ethereum", "bitcoin"])).toThrow(
      'unknown chain "eth-mainnet"'
    );
  });
});
