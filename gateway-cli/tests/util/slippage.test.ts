import { describe, it, expect, vi } from "vitest";
import type { GatewayQuoteV4, GetQuoteParams } from "@gobob/bob-sdk";
import { checkSlippage, quotedSlippageBps } from "../../src/util/slippage.js";

// The gateway resolves slippage per route when the request omits it, and echoes an
// explicit value back otherwise. These fixtures stand in for both answers.
const onramp = (slippage: string) => ({ onramp: { slippage } }) as unknown as GatewayQuoteV4;
const offramp = (slippage: number) => ({ offramp: { slippage } }) as unknown as GatewayQuoteV4;

const params: GetQuoteParams = {
  fromChain: "bitcoin",
  toChain: "base",
  fromToken: "BTC",
  toToken: "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913",
  toUserAddress: "0x1111111111111111111111111111111111111111",
  amount: "5000000",
};

/** A gateway that recommends `recommendedBps` and otherwise honours what it is asked for. */
function gateway(recommendedBps: number) {
  return vi.fn(async (p: GetQuoteParams) =>
    onramp(String(p.maxSlippage ?? recommendedBps)),
  );
}

describe("quotedSlippageBps", () => {
  it("reads the onramp's string slippage as a number", () => {
    expect(quotedSlippageBps(onramp("175"))).toBe(175);
  });

  it("reads the offramp's numeric slippage", () => {
    expect(quotedSlippageBps(offramp(250))).toBe(250);
  });

  // The SDK types the onramp value as a bare string, and `Number("")` is 0: a blank or
  // malformed value must read as "not reported", never as zero tolerance.
  it.each(["", "  ", "abc", "1.5", "-5", "1e3"])("treats onramp slippage %j as missing, not as a number", (raw) => {
    expect(quotedSlippageBps(onramp(raw))).toBeUndefined();
  });

  it.each([NaN, -1, 1.5])("treats numeric slippage %s as missing", (raw) => {
    expect(quotedSlippageBps(offramp(raw))).toBeUndefined();
  });

  it("accepts an explicit zero", () => {
    expect(quotedSlippageBps(onramp("0"))).toBe(0);
    expect(quotedSlippageBps(offramp(0))).toBe(0);
  });
});

describe("checkSlippage", () => {
  it("does not probe when no --slippage was given: the quote already IS the recommendation", async () => {
    const getQuote = gateway(175);

    expect(await checkSlippage(getQuote, params)).toBeUndefined();
    expect(getQuote).not.toHaveBeenCalled();
  });

  it("warns when the requested slippage is below the gateway's recommendation", async () => {
    // A tolerance below the route's recommendation is accepted at quote time but is
    // likely to fail at settlement, refunding the order instead of filling it.
    const getQuote = gateway(175);

    const warning = await checkSlippage(getQuote, { ...params, maxSlippage: 50 });

    expect(warning).toMatchObject({ requestedBps: 50, recommendedBps: 175 });
    expect(warning?.message).toMatch(/50 bps is below the gateway's recommended 175 bps/);
    // The probe must OMIT slippage — sending one would only get it echoed back.
    expect(getQuote).toHaveBeenCalledOnce();
    expect(getQuote.mock.calls[0][0].maxSlippage).toBeUndefined();
    expect(getQuote.mock.calls[0][0]).toMatchObject({ ...params, maxSlippage: undefined });
  });

  it("treats an explicit 0 as a limit to check, not as 'no slippage given'", async () => {
    const warning = await checkSlippage(gateway(175), { ...params, maxSlippage: 0 });

    expect(warning).toMatchObject({ requestedBps: 0, recommendedBps: 175 });
  });

  it.each([175, 300])("does not warn when the requested %s bps meets the 175 bps recommendation", async (bps) => {
    expect(await checkSlippage(gateway(175), { ...params, maxSlippage: bps })).toBeUndefined();
  });

  it("does not warn when the recommendation comes back blank — it would read as 0 bps", async () => {
    const getQuote = vi.fn(async () => onramp(""));

    expect(await checkSlippage(getQuote, { ...params, maxSlippage: 50 })).toBeUndefined();
  });

  it("does not fail when the recommendation probe fails — it only ever decides a warning", async () => {
    const getQuote = vi.fn().mockRejectedValue(new Error("503 Service Unavailable"));

    expect(await checkSlippage(getQuote, { ...params, maxSlippage: 50 })).toBeUndefined();
  });
});
