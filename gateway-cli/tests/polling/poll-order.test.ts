import { describe, it, expect, vi } from "vitest";
import { pollOrder, PollTimeoutError } from "../../src/polling/poll-order.js";
import type { GatewayOrderInfo, OrderStatus } from "../../src/api/types.js";

function makeOrder(status: OrderStatus): GatewayOrderInfo {
  return {
    orderId: "order-1",
    status,
    inputAmount: "100000",
    outputAmount: "95000",
    srcChain: "bitcoin",
    dstChain: "bob",
    createdAt: "2026-03-12T10:00:00Z",
  };
}

function mockClient(responses: GatewayOrderInfo[]) {
  let callIndex = 0;
  return {
    getOrder: vi.fn(async (_id: string) => {
      const resp = responses[callIndex];
      if (callIndex < responses.length - 1) callIndex++;
      return resp;
    }),
  };
}

describe("pollOrder", () => {
  it("returns immediately when order is already terminal (success)", async () => {
    const client = mockClient([makeOrder("success")]);
    const result = await pollOrder(client, "order-1", {
      intervalMs: 10,
      timeoutMs: 1000,
    });
    expect(result.status).toBe("success");
    expect(client.getOrder).toHaveBeenCalledTimes(1);
  });

  it("returns immediately when order status is strategy_skipped", async () => {
    const client = mockClient([makeOrder("strategy_skipped")]);
    const result = await pollOrder(client, "order-1", {
      intervalMs: 10,
      timeoutMs: 1000,
    });
    expect(result.status).toBe("strategy_skipped");
    expect(client.getOrder).toHaveBeenCalledTimes(1);
  });

  it("returns immediately when order status is strategy_failed", async () => {
    const client = mockClient([makeOrder("strategy_failed")]);
    const result = await pollOrder(client, "order-1", {
      intervalMs: 10,
      timeoutMs: 1000,
    });
    expect(result.status).toBe("strategy_failed");
    expect(client.getOrder).toHaveBeenCalledTimes(1);
  });

  it("throws when order is refunded (terminal failure)", async () => {
    const client = mockClient([makeOrder("refunded")]);
    await expect(
      pollOrder(client, "order-1", { intervalMs: 10, timeoutMs: 1000 }),
    ).rejects.toThrow("failed with status");
  });

  it("throws when order reaches terminal failed state", async () => {
    const failedStatus: OrderStatus = { failed: { refundTx: undefined } };
    const client = mockClient([makeOrder(failedStatus)]);
    await expect(
      pollOrder(client, "order-1", { intervalMs: 10, timeoutMs: 1000 }),
    ).rejects.toThrow("failed with status");
  });

  it("polls multiple times until terminal state", async () => {
    const inProgressStatus: OrderStatus = {
      inProgress: { bumpFeeTx: undefined, refundTx: undefined },
    };
    const client = mockClient([
      makeOrder(inProgressStatus),
      makeOrder(inProgressStatus),
      makeOrder("success"),
    ]);
    const result = await pollOrder(client, "order-1", {
      intervalMs: 10,
      timeoutMs: 5000,
    });
    expect(result.status).toBe("success");
    expect(client.getOrder).toHaveBeenCalledTimes(3);
  });

  it("calls onWaiting callback while polling", async () => {
    const inProgressStatus: OrderStatus = {
      inProgress: { bumpFeeTx: undefined, refundTx: undefined },
    };
    const onWaiting = vi.fn();
    const client = mockClient([
      makeOrder(inProgressStatus),
      makeOrder("success"),
    ]);
    await pollOrder(client, "order-1", {
      intervalMs: 10,
      timeoutMs: 5000,
      callbacks: { onWaiting },
    });
    expect(onWaiting).toHaveBeenCalled();
  });

  it("throws PollTimeoutError when deadline is exceeded", async () => {
    const inProgressStatus: OrderStatus = {
      inProgress: { bumpFeeTx: undefined, refundTx: undefined },
    };
    const client = mockClient([makeOrder(inProgressStatus)]);

    await expect(
      pollOrder(client, "order-1", {
        intervalMs: 10,
        timeoutMs: 50,
      }),
    ).rejects.toThrow(PollTimeoutError);
  });

  it("PollTimeoutError contains orderId and timeout", async () => {
    const inProgressStatus: OrderStatus = {
      inProgress: { bumpFeeTx: undefined, refundTx: undefined },
    };
    const client = mockClient([makeOrder(inProgressStatus)]);

    try {
      await pollOrder(client, "order-1", {
        intervalMs: 10,
        timeoutMs: 50,
      });
      expect.fail("should have thrown");
    } catch (e) {
      expect(e).toBeInstanceOf(PollTimeoutError);
      const err = e as PollTimeoutError;
      expect(err.orderId).toBe("order-1");
      expect(err.timeoutMs).toBe(50);
    }
  });
});
