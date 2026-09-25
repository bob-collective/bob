import { afterEach, expect, test, vi } from "vitest";

const previousApiKey = process.env.GATEWAY_API_KEY;
const previousApiUrl = process.env.GATEWAY_API_URL;

afterEach(() => {
  if (previousApiKey === undefined) delete process.env.GATEWAY_API_KEY;
  else process.env.GATEWAY_API_KEY = previousApiKey;
  if (previousApiUrl === undefined) delete process.env.GATEWAY_API_URL;
  else process.env.GATEWAY_API_URL = previousApiUrl;
  vi.unstubAllGlobals();
  vi.resetModules();
});

test("the installed SDK sends the configured Bearer token", async () => {
  const key = "a".repeat(32);
  process.env.GATEWAY_API_KEY = ` ${key}\n`;
  process.env.GATEWAY_API_URL = "https://gateway.example";
  const fetchMock = vi.fn(async () => new Response("[]", { status: 200 }));
  vi.stubGlobal("fetch", fetchMock);

  const { getApi } = await import("../src/config.js");
  await getApi().getRoutesV4();

  expect(fetchMock).toHaveBeenCalledOnce();
  const [url, init] = fetchMock.mock.calls[0] as unknown as [string, RequestInit];
  expect(url).toBe("https://gateway.example/v4/get-routes");
  expect(new Headers(init.headers).get("Authorization")).toBe(`Bearer ${key}`);
});
