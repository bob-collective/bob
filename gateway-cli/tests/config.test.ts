import { afterEach, describe, expect, test, vi } from "vitest";

const { gatewaySDK } = vi.hoisted(() => ({ gatewaySDK: vi.fn(function () { return { api: {} }; }) }));
vi.mock("@gobob/bob-sdk", () => ({ GatewaySDK: gatewaySDK }));

const previousApiKey = process.env.GATEWAY_API_KEY;
const previousApiUrl = process.env.GATEWAY_API_URL;

afterEach(() => {
  if (previousApiKey === undefined) delete process.env.GATEWAY_API_KEY;
  else process.env.GATEWAY_API_KEY = previousApiKey;
  if (previousApiUrl === undefined) delete process.env.GATEWAY_API_URL;
  else process.env.GATEWAY_API_URL = previousApiUrl;
  gatewaySDK.mockClear();
  vi.resetModules();
});

describe("Gateway SDK configuration", () => {
  test("passes the configured API key and URL to the SDK", async () => {
    process.env.GATEWAY_API_KEY = "a".repeat(32);
    process.env.GATEWAY_API_URL = "https://gateway.example";
    const { getSdk } = await import("../src/config.js");

    getSdk();
    expect(gatewaySDK).toHaveBeenCalledWith({
      basePath: "https://gateway.example",
      apiKey: "a".repeat(32),
    });
  });
});
