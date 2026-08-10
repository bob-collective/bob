/**
 * Sleep that resolves early — never rejects — when `signal` aborts.
 *
 * A wait must not outlive the budget that bounds the operation it is part of, and the
 * way to express that is to hang the sleep off the same signal that bounds everything
 * else. The alternative, clamping the delay against a computed remaining window, is
 * clock arithmetic — and clock arithmetic is what a hand-maintained deadline turns into
 * a negative delay when two of its reads disagree.
 */
export function sleep(ms: number, signal: AbortSignal): Promise<void> {
  return new Promise<void>(resolve => {
    if (signal.aborted) return resolve();
    const done = () => {
      clearTimeout(timer);
      signal.removeEventListener("abort", done);
      resolve();
    };
    const timer = setTimeout(done, ms);
    signal.addEventListener("abort", done, { once: true });
  });
}
