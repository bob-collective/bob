import { describe, it } from "vitest";
import { strict as assert } from "node:assert";
import { SwapError, withRetry } from "../src/errors.js";

describe("withRetry", () => {
  it("retries transient errors then resolves", async () => {
    let n = 0;
    const result = await withRetry(
      async () => {
        n++;
        if (n < 3) throw new Error("pending");
        return "ok";
      },
      { retries: 5, isTransient: (e) => e instanceof Error && e.message === "pending", minTimeout: 1, maxTimeout: 1 },
    );
    assert.equal(result, "ok");
    assert.equal(n, 3);
  });

  it("stops on a terminal SwapError and preserves its context", async () => {
    const err = await withRetry(
      async () => {
        throw new SwapError("Order x failed", { orderId: "x", txId: "0xabc" });
      },
      { retries: 5, isTransient: () => false, minTimeout: 1, maxTimeout: 1 },
    ).catch((e) => e);
    assert.ok(err instanceof SwapError, "expected a SwapError to propagate");
    assert.equal(err.context.orderId, "x");
    assert.equal(err.context.txId, "0xabc");
  });

  it("does not retry a terminal error (single attempt)", async () => {
    let n = 0;
    await withRetry(
      async () => {
        n++;
        throw new SwapError("nope");
      },
      { retries: 5, isTransient: () => false, minTimeout: 1, maxTimeout: 1 },
    ).catch(() => {});
    assert.equal(n, 1);
  });
});
