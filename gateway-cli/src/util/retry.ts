export interface RetryOptions {
  retries: number;
  backoffMs: number[];
}

const DEFAULT_RETRY: RetryOptions = {
  retries: 3,
  backoffMs: [1000, 4000, 16000],
};

export async function retryWithBackoff<T>(
  fn: () => Promise<T>,
  opts: RetryOptions = DEFAULT_RETRY,
  onRetry?: (attempt: number) => void,
): Promise<{ value: T } | { error: unknown }> {
  let lastError: unknown;
  for (let i = 0; i < opts.retries; i++) {
    try {
      return { value: await fn() };
    } catch (err) {
      lastError = err;
      onRetry?.(i + 1);
      await new Promise((r) => setTimeout(r, opts.backoffMs[i] ?? 1000));
    }
  }
  // Final attempt
  try {
    return { value: await fn() };
  } catch (err) {
    lastError = err;
  }
  return { error: lastError };
}

// ─── Transient error detection ──────────────────────────────────────────────

const TRANSIENT_PATTERNS = [
  /TRM screening/i,
  /429/,
  /Too Many Requests/i,
  /rate limit/i,
  /not yet propagated/i,
  /BTC propagation/i,
  /timeout/i,
  /ECONNRESET/,
  /ETIMEDOUT/,
];

export function isTransientError(error: unknown): boolean {
  if (!error || typeof error !== "object") return false;
  const msg = (error as Error).message || "";
  return TRANSIENT_PATTERNS.some((pattern) => pattern.test(msg));
}

export class TransientError extends Error {
  readonly retryable = true;
  constructor(message: string) {
    super(message);
    this.name = "TransientError";
  }
}
