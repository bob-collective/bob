import { describe, it, expect, vi } from "vitest";
import { handleRoutes } from "../../src/commands/routes.js";

// ─── Mocks ───────────────────────────────────────────────────────────────────

vi.mock("../../src/config.js", () => ({
  BTC_DECIMALS: 8,
  getSdk: vi.fn(),
}));

vi.mock("../../src/util/route-provider.js", () => {
  const routes = [
    { srcChain: "bitcoin", dstChain: "base", srcToken: "BTC", dstToken: "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913" },
    { srcChain: "bitcoin", dstChain: "ethereum", srcToken: "BTC", dstToken: "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48" },
    { srcChain: "base", dstChain: "bitcoin", srcToken: "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913", dstToken: "BTC" },
  ];

  const getTokenMeta = (address: string, chain: string) => {
    if (chain === "bitcoin" || address === "BTC") return { symbol: "BTC", decimals: 8 };
    const TOKENS: Record<string, { symbol: string; decimals: number }> = {
      "0x833589fcd6edb6e08f4c7c32d4f71b54bda02913": { symbol: "USDC", decimals: 6 },
      "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48": { symbol: "USDC", decimals: 6 },
    };
    return TOKENS[address.toLowerCase()] ?? { symbol: address.slice(0, 10), decimals: 18 };
  };

  return {
    getRoutes: vi.fn().mockResolvedValue(routes),
    getUniqueChains: vi.fn((rs: any[]) => [...new Set(rs.flatMap((r: any) => [r.srcChain, r.dstChain]))]),
    getTokenAddressesForChain: vi.fn((chain: string, rs: any[]) => {
      const seen = new Set<string>();
      const addrs: string[] = [];
      for (const r of rs) {
        for (const [c, addr] of [[r.srcChain, r.srcToken], [r.dstChain, r.dstToken]]) {
          if (c === chain && addr !== "BTC" && !seen.has(addr.toLowerCase())) {
            seen.add(addr.toLowerCase());
            addrs.push(addr);
          }
        }
      }
      return addrs;
    }),
    getTokensForChain: vi.fn((chain: string, rs: any[]) => {
      const seen = new Set<string>();
      const tokens: any[] = [];
      for (const r of rs) {
        for (const [c, addr] of [[r.srcChain, r.srcToken], [r.dstChain, r.dstToken]]) {
          if (c === chain && addr !== "BTC" && !seen.has(addr.toLowerCase())) {
            seen.add(addr.toLowerCase());
            const meta = getTokenMeta(addr, chain);
            tokens.push({ address: addr, symbol: meta.symbol, decimals: meta.decimals });
          }
        }
      }
      return tokens;
    }),
  };
});

vi.mock("../../src/util/input-resolver.js", () => ({
  resolveChain: vi.fn((input: string) => {
    const aliases: Record<string, string> = { btc: "bitcoin", eth: "ethereum" };
    return aliases[input.toLowerCase()] ?? input.toLowerCase();
  }),
}));

// ─── Tests ───────────────────────────────────────────────────────────────────

describe("handleRoutes", () => {
  it("returns all routes when no filters provided", async () => {
    const result = await handleRoutes({});

    expect(result.type).toBe("routes");
    if (result.type === "routes") {
      expect(result.data).toHaveLength(3);
    }
  });

  it("lists unique chains when --chains flag is set", async () => {
    const result = await handleRoutes({ chains: true });

    expect(result.type).toBe("chains");
    if (result.type === "chains") {
      const chainNames = result.data.map((c) => c.canonical);
      expect(chainNames).toContain("bitcoin");
      expect(chainNames).toContain("base");
      expect(chainNames).toContain("ethereum");
      // Should be sorted alphabetically
      expect(chainNames).toEqual([...chainNames].sort());
    }
  });

  it("lists tokens for a specific chain when --tokens is set", async () => {
    const result = await handleRoutes({ tokens: "base" });

    expect(result.type).toBe("tokens");
    if (result.type === "tokens") {
      expect(result.data.length).toBeGreaterThan(0);
      const symbols = result.data.map((t) => t.symbol);
      expect(symbols).toContain("USDC");
    }
  });

  it("throws when querying tokens for unknown chain", async () => {
    await expect(handleRoutes({ tokens: "unknownchain" })).rejects.toThrow(
      /no tokens found on chain/,
    );
  });

  it("filters routes by source chain", async () => {
    const result = await handleRoutes({ from: "base" });

    expect(result.type).toBe("routes");
    if (result.type === "routes") {
      expect(result.data).toHaveLength(1);
      expect(result.data[0].srcChain).toBe("base");
    }
  });

  it("filters routes by destination chain", async () => {
    const result = await handleRoutes({ to: "bitcoin" });

    expect(result.type).toBe("routes");
    if (result.type === "routes") {
      expect(result.data).toHaveLength(1);
      expect(result.data[0].dstChain).toBe("bitcoin");
    }
  });

  it("filters routes by both source and destination chain", async () => {
    const result = await handleRoutes({ from: "bitcoin", to: "base" });

    expect(result.type).toBe("routes");
    if (result.type === "routes") {
      expect(result.data).toHaveLength(1);
      expect(result.data[0].srcChain).toBe("bitcoin");
      expect(result.data[0].dstChain).toBe("base");
    }
  });

  it("returns empty routes when no matches for filter", async () => {
    const result = await handleRoutes({ from: "arbitrum" });

    expect(result.type).toBe("routes");
    if (result.type === "routes") {
      expect(result.data).toHaveLength(0);
    }
  });
});
