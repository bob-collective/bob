import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import { GatewayApiClient } from "../../src/api/client.js";
import {
  mockRoutes,
  mockQuote,
  mockOrder,
  mockCreateOrderResponse,
  mockMaxSpendable,
} from "../fixtures/api-responses.js";
import type { RegisterTxResponse } from "../../src/api/types.js";

const BASE_URL = "https://gateway-api-mainnet.gobob.xyz";

function mockFetchResponse(data: unknown, status = 200) {
  return vi.fn().mockResolvedValue({
    ok: status >= 200 && status < 300,
    status,
    json: () => Promise.resolve(data),
    text: () => Promise.resolve(JSON.stringify(data)),
  });
}

describe("GatewayApiClient", () => {
  let client: GatewayApiClient;
  let originalFetch: typeof globalThis.fetch;

  beforeEach(() => {
    client = new GatewayApiClient(BASE_URL);
    originalFetch = globalThis.fetch;
  });

  afterEach(() => {
    globalThis.fetch = originalFetch;
  });

  it("getRoutes fetches routes", async () => {
    globalThis.fetch = mockFetchResponse(mockRoutes);

    const routes = await client.getRoutes();

    expect(routes).toEqual(mockRoutes);
    expect(globalThis.fetch).toHaveBeenCalledWith(
      `${BASE_URL}/v1/get-routes`,
      { method: "GET" }
    );
  });

  it("getQuote builds query string and fetches quote", async () => {
    globalThis.fetch = mockFetchResponse(mockQuote);

    const quote = await client.getQuote({
      srcChain: "bitcoin",
      dstChain: "base",
      recipient: "0xRecipient",
      srcToken: "BTC",
      dstToken: "USDC",
      amount: "5000000",
      slippage: 300,
    });

    expect(quote).toEqual(mockQuote);
    const calledUrl = (globalThis.fetch as ReturnType<typeof vi.fn>).mock
      .calls[0][0] as string;
    expect(calledUrl).toContain("/v1/get-quote?");
    expect(calledUrl).toContain("srcChain=bitcoin");
    expect(calledUrl).toContain("dstChain=base");
    expect(calledUrl).toContain("amount=5000000");
    expect(calledUrl).toContain("slippage=300");
  });

  it("getQuote omits undefined optional params", async () => {
    globalThis.fetch = mockFetchResponse(mockQuote);

    await client.getQuote({
      srcChain: "bitcoin",
      dstChain: "base",
      recipient: "0xRecipient",
      srcToken: "BTC",
      dstToken: "USDC",
      amount: "5000000",
      slippage: 300,
    });

    const calledUrl = (globalThis.fetch as ReturnType<typeof vi.fn>).mock
      .calls[0][0] as string;
    expect(calledUrl).not.toContain("gasRefill");
    expect(calledUrl).not.toContain("affiliateId");
    expect(calledUrl).not.toContain("sender");
  });

  it("createOrder posts quote and returns order response", async () => {
    globalThis.fetch = mockFetchResponse(mockCreateOrderResponse);

    const response = await client.createOrder(mockQuote);

    expect(response).toEqual(mockCreateOrderResponse);
    expect(globalThis.fetch).toHaveBeenCalledWith(
      `${BASE_URL}/v1/create-order`,
      {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(mockQuote),
      }
    );
  });

  it("registerTx sends PATCH request", async () => {
    const mockResponse: RegisterTxResponse = { success: true };
    globalThis.fetch = mockFetchResponse(mockResponse);

    const result = await client.registerTx({
      orderId: "order-456",
      bitcoinTxid: "abc123",
    });

    expect(result).toEqual(mockResponse);
    expect(globalThis.fetch).toHaveBeenCalledWith(
      `${BASE_URL}/v1/register-tx`,
      {
        method: "PATCH",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          orderId: "order-456",
          bitcoinTxid: "abc123",
        }),
      }
    );
  });

  it("getOrder fetches single order by id", async () => {
    globalThis.fetch = mockFetchResponse(mockOrder);

    const order = await client.getOrder("order-456");

    expect(order).toEqual(mockOrder);
    expect(globalThis.fetch).toHaveBeenCalledWith(
      `${BASE_URL}/v1/get-order/order-456`,
      { method: "GET" }
    );
  });

  it("getOrders fetches orders for a user address", async () => {
    const orders = [mockOrder];
    globalThis.fetch = mockFetchResponse(orders);

    const result = await client.getOrders("0xUserAddress");

    expect(result).toEqual(orders);
    expect(globalThis.fetch).toHaveBeenCalledWith(
      `${BASE_URL}/v1/get-orders/0xUserAddress`,
      { method: "GET" }
    );
  });

  it("getMaxSpendable fetches max spendable amount", async () => {
    globalThis.fetch = mockFetchResponse(mockMaxSpendable);

    const result = await client.getMaxSpendable("bc1qexample");

    expect(result).toEqual(mockMaxSpendable);
    expect(globalThis.fetch).toHaveBeenCalledWith(
      `${BASE_URL}/v1/get-max-spendable/bc1qexample`,
      { method: "GET" }
    );
  });

  it("throws on non-ok response", async () => {
    globalThis.fetch = mockFetchResponse(
      "Internal Server Error",
      500
    );

    await expect(client.getRoutes()).rejects.toThrow(
      /Gateway API error \(500\)/
    );
  });
});
