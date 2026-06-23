import pRetry, { AbortError } from "p-retry";

/**
 * Structured context for a failed swap, surfaced by the --json serializer and
 * consumed by downstream tooling (e.g. gateway-bot alerting). Every field is
 * optional — which ones are known depends on where the swap failed.
 */
export interface SwapContext {
  orderId?: string;
  txId?: string;
  txParams?: { to?: string; from?: string; chainId?: number; chainName?: string };
  srcAsset?: { symbol: string; chain: string };
  dstAsset?: { symbol: string; chain: string };
  functionSelector?: string;
  revertData?: string;
}

/** A terminal (non-retryable) swap failure carrying structured {@link SwapContext}. */
export class SwapError extends Error {
  constructor(
    message: string,
    readonly context: SwapContext = {},
  ) {
    super(message);
    this.name = "SwapError";
  }
}

/**
 * Retry `fn` while it throws a transient error; stop immediately on anything else.
 *
 * p-retry's `AbortError` is an internal detail here, never exposed to callers: it
 * always wraps the ORIGINAL Error, so a {@link SwapError}'s `context` survives
 * p-retry's `throw originalError`. Callers throw domain errors (e.g. `SwapError`
 * for terminal, a transient sentinel to keep retrying) and never touch AbortError.
 */
export async function withRetry<T>(
  fn: () => Promise<T>,
  opts: { retries: number; isTransient: (e: unknown) => boolean; minTimeout?: number; maxTimeout?: number },
): Promise<T> {
  return pRetry(
    async () => {
      try {
        return await fn();
      } catch (e) {
        if (opts.isTransient(e)) throw e; // retryable → let p-retry retry
        throw new AbortError(e instanceof Error ? e : new Error(String(e))); // terminal → stop, preserve the Error (+ context)
      }
    },
    { retries: opts.retries, minTimeout: opts.minTimeout, maxTimeout: opts.maxTimeout, factor: 1 },
  );
}
