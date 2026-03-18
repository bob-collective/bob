import { describe, it, expect, vi, beforeEach } from "vitest";
import { handleRoutes } from "../../src/commands/routes.js";
import { mockRoutes } from "../fixtures/api-responses.js";

const mockGetRoutes = vi.fn();
vi.mock("../../src/api/client.js", () => ({
  GatewayApiClient: vi.fn().mockImplementation(function () {
    return { getRoutes: mockGetRoutes };
  }),
}));

describe("handleRoutes", () => {
  beforeEach(() => {
    mockGetRoutes.mockReset();
  });

  it("returns all routes when no filters", async () => {
    mockGetRoutes.mockResolvedValueOnce(mockRoutes);
    const result = await handleRoutes({
      apiUrl: "https://example.com",
      json: true,
    });
    expect(JSON.parse(result)).toHaveLength(1);
  });

  it("filters by --from", async () => {
    const routes = [
      ...mockRoutes,
      {
        srcChain: "bob",
        dstChain: "bitcoin",
        srcToken: {
          address: "0x1",
          symbol: "USDC",
          decimals: 6,
          chain: "bob",
        },
        dstToken: {
          address: "0x0",
          symbol: "BTC",
          decimals: 8,
          chain: "bitcoin",
        },
      },
    ];
    mockGetRoutes.mockResolvedValueOnce(routes);
    const result = await handleRoutes({
      apiUrl: "https://example.com",
      json: true,
      from: "bitcoin",
    });
    const parsed = JSON.parse(result);
    expect(parsed).toHaveLength(1);
    expect(parsed[0].srcChain).toBe("bitcoin");
  });
});
