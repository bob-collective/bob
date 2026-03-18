import type { GatewayOrderInfo, OrderStatus } from "../api/types.js";

export class PollTimeoutError extends Error {
  constructor(public readonly orderId: string, public readonly timeoutMs: number) {
    super(`Polling timed out after ${timeoutMs}ms for order "${orderId}"`);
    this.name = "PollTimeoutError";
  }
}

export interface PollCallbacks {
  onWaiting?: (elapsedMs: number) => void;
  onConfirmation?: (n: number, total: number) => void;
}

export interface PollOptions {
  intervalMs: number;
  timeoutMs: number;
  callbacks?: PollCallbacks;
}

interface OrderClient {
  getOrder(id: string): Promise<GatewayOrderInfo>;
}

function isTerminalSuccess(status: OrderStatus): boolean {
  return (
    status === "success" ||
    status === "strategy_skipped" ||
    status === "strategy_failed"
  );
}

function isTerminalFailure(status: OrderStatus): boolean {
  if (status === "refunded") return true;
  if (typeof status === "object" && status !== null && "failed" in status) return true;
  return false;
}

function sleep(ms: number): Promise<void> {
  return new Promise((r) => setTimeout(r, ms));
}

export async function pollOrder(
  client: OrderClient,
  orderId: string,
  opts: PollOptions,
): Promise<GatewayOrderInfo> {
  const start = Date.now();
  const deadline = start + opts.timeoutMs;

  while (true) {
    if (Date.now() >= deadline) throw new PollTimeoutError(orderId, opts.timeoutMs);

    opts.callbacks?.onWaiting?.(Date.now() - start);
    const order = await client.getOrder(orderId);

    if (isTerminalSuccess(order.status)) return order;
    if (isTerminalFailure(order.status)) {
      throw new Error(`Order ${orderId} failed with status: ${JSON.stringify(order.status)}`);
    }

    await sleep(opts.intervalMs);
  }
}
