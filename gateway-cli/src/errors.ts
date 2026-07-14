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
 * Handle given to a retryable operation so it can declare that it has crossed a
 * *point of no return* — an irreversible side effect (a wallet signature, a
 * broadcast) that makes re-running the whole operation unsafe, no matter what
 * error follows.
 */
export interface RetryGuard {
  /**
   * Latch. From this call onward the current operation will NOT be retried,
   * however transient the subsequent error looks.
   *
   * Safe to call repeatedly as the operation progresses: the latch itself is
   * monotonic (once armed it never disarms), and the LAST reason wins, so the
   * error reports the *furthest* irreversible step reached — not the first.
   * Reporting the first would understate how far the operation got (e.g. naming
   * an ERC20 `approve` when the funds tx had in fact already been broadcast),
   * which is precisely the misreading that leads someone to re-run and double-send.
   */
  pointOfNoReturn(reason: string): void;
}

/**
 * Thrown by {@link withRetry} when an operation failed *after* it declared a
 * {@link RetryGuard.pointOfNoReturn}. Its existence tells the caller "this failed
 * with side effects already committed" — which is a different, louder outcome
 * than a clean failure, and must never be papered over as a plain retry exhaustion.
 */
export class PointOfNoReturnError extends Error {
  constructor(
    /** The failure that actually occurred, after the irreversible step. */
    readonly originalError: Error,
    /** Which irreversible step had been reached when it failed. */
    readonly reason: string,
  ) {
    super(originalError.message);
    this.name = "PointOfNoReturnError";
  }
}

/**
 * Retry `fn` while it throws a transient error; stop immediately on anything else.
 *
 * `fn` receives a {@link RetryGuard}. Once `fn` trips the guard, the attempt is
 * never retried — the guard **dominates `isTransient`**, because no amount of
 * "this error looks transient" can make it safe to re-run an operation that has
 * already broadcast a transaction. The latch is created, and consulted, *inside*
 * this function: a caller can arm it but cannot forget to wire it into the retry
 * decision, which is what makes "retried after an irreversible step" an
 * unrepresentable state rather than merely an unlikely one.
 *
 * p-retry's `AbortError` is an internal detail here, never exposed to callers: it
 * always wraps the ORIGINAL Error, so a {@link SwapError}'s `context` survives
 * p-retry's `throw originalError`. Callers throw domain errors (e.g. `SwapError`
 * for terminal, a transient sentinel to keep retrying) and never touch AbortError.
 */
export async function withRetry<T>(
  fn: (guard: RetryGuard) => Promise<T>,
  opts: { retries: number; isTransient: (e: unknown) => boolean; minTimeout?: number; maxTimeout?: number },
): Promise<T> {
  return pRetry(
    async () => {
      let crossed: string | undefined;
      const guard: RetryGuard = { pointOfNoReturn: (reason) => { crossed = reason; } };
      try {
        return await fn(guard);
      } catch (e) {
        const err = e instanceof Error ? e : new Error(String(e));
        // Checked BEFORE isTransient, deliberately: past the point of no return the
        // only safe move is to stop and tell the operator what was already done.
        if (crossed !== undefined) throw new AbortError(new PointOfNoReturnError(err, crossed));
        if (opts.isTransient(e)) throw e; // retryable → let p-retry retry
        throw new AbortError(err); // terminal → stop, preserve the Error (+ context)
      }
    },
    { retries: opts.retries, minTimeout: opts.minTimeout, maxTimeout: opts.maxTimeout, factor: 1 },
  );
}
