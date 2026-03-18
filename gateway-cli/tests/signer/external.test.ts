import { describe, it, expect } from "vitest";
import { ExternalSigner, SignerError } from "../../src/signer/btc.js";

describe("ExternalSigner", () => {
  it("pipes input through stdin and returns stdout", async () => {
    const signer = new ExternalSigner("cat");
    const result = await signer.sign("hello world");
    expect(result).toBe("hello world");
  });

  it("trims trailing whitespace from stdout", async () => {
    // echo appends a newline; use printf-like behavior via cat which preserves input
    // We'll rely on the trim in sign() by passing input with trailing newline
    const signer = new ExternalSigner("cat");
    const result = await signer.sign("hello\n");
    expect(result).toBe("hello");
  });

  it("throws SignerError on non-zero exit code", async () => {
    const signer = new ExternalSigner("false");
    await expect(signer.sign("anything")).rejects.toThrow(SignerError);
    await expect(signer.sign("anything")).rejects.toMatchObject({
      name: "SignerError",
      command: "false",
    });
  });

  it("throws SignerError when command is not found", async () => {
    const signer = new ExternalSigner("nonexistent-command-xyz-12345");
    await expect(signer.sign("anything")).rejects.toThrow(SignerError);
  });

  it("includes stderr in error message", async () => {
    // sh -c 'echo err >&2; exit 1' writes to stderr and exits non-zero
    const signer = new ExternalSigner("sh -c 'echo err >&2; exit 1'");
    try {
      await signer.sign("input");
      expect.fail("should have thrown");
    } catch (e) {
      expect(e).toBeInstanceOf(SignerError);
      const err = e as SignerError;
      expect(err.stderr).toContain("err");
    }
  });
});
