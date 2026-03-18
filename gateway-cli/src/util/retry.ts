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
