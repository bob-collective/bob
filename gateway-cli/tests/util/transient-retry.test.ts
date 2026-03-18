import { describe, it, expect, vi } from "vitest";
import { isTransientError, TransientError } from "../../src/util/retry.js";

describe("isTransientError", () => {
  it("detects TRM screening errors", () => {
    expect(isTransientError(new Error("TRM screening delay"))).toBe(true);
  });

  it("detects rate limit errors (429)", () => {
    expect(isTransientError(new Error("429 Too Many Requests"))).toBe(true);
  });

  it("detects rate limit errors (text)", () => {
    expect(isTransientError(new Error("rate limit exceeded"))).toBe(true);
  });

  it("detects BTC propagation errors", () => {
    expect(isTransientError(new Error("BTC transaction not yet propagated"))).toBe(true);
  });

  it("detects timeout errors", () => {
    expect(isTransientError(new Error("request timeout"))).toBe(true);
  });

  it("detects ECONNRESET", () => {
    expect(isTransientError(new Error("read ECONNRESET"))).toBe(true);
  });

  it("detects ETIMEDOUT", () => {
    expect(isTransientError(new Error("connect ETIMEDOUT 1.2.3.4:443"))).toBe(true);
  });

  it("returns false for non-transient errors", () => {
    expect(isTransientError(new Error("Insufficient funds"))).toBe(false);
  });

  it("returns false for generic errors", () => {
    expect(isTransientError(new Error("something went wrong"))).toBe(false);
  });

  it("returns false for null", () => {
    expect(isTransientError(null)).toBe(false);
  });

  it("returns false for undefined", () => {
    expect(isTransientError(undefined)).toBe(false);
  });

  it("returns false for non-object values", () => {
    expect(isTransientError("string error")).toBe(false);
    expect(isTransientError(42)).toBe(false);
  });
});

describe("TransientError", () => {
  it("wraps an error with retryable flag", () => {
    const err = new TransientError("TRM screening delay");
    expect(err.retryable).toBe(true);
    expect(err.message).toBe("TRM screening delay");
    expect(err.name).toBe("TransientError");
  });

  it("is an instance of Error", () => {
    const err = new TransientError("test");
    expect(err).toBeInstanceOf(Error);
    expect(err).toBeInstanceOf(TransientError);
  });
});
