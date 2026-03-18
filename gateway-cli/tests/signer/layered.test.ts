// src/signer/layered.test.ts
import { describe, test, expect, vi } from "vitest";
import { resolveBtcSigner } from "../../src/signer/btc.js";
import { resolveEvmSigner } from "../../src/signer/evm.js";

describe("resolveBtcSigner", () => {
  test("--private-key flag takes priority over env var", async () => {
    const signer = await resolveBtcSigner({
      privateKey: "wif-from-flag",
      envPrivateKey: "wif-from-env",
      externalSignerCmd: undefined,
      unsigned: false,
    });
    expect(signer.type).toBe("private-key");
    expect((signer as any).key).toBe("wif-from-flag");
  });

  test("env var used when flag not set", async () => {
    const signer = await resolveBtcSigner({
      privateKey: undefined,
      envPrivateKey: "wif-from-env",
      externalSignerCmd: undefined,
      unsigned: false,
    });
    expect(signer.type).toBe("private-key");
    expect((signer as any).key).toBe("wif-from-env");
  });

  test("external signer used when no key configured", async () => {
    const signer = await resolveBtcSigner({
      privateKey: undefined,
      envPrivateKey: undefined,
      externalSignerCmd: "/path/to/signer",
      unsigned: false,
    });
    expect(signer.type).toBe("external");
  });

  test("unsigned mode returned when --unsigned set", async () => {
    const signer = await resolveBtcSigner({
      privateKey: undefined,
      envPrivateKey: undefined,
      externalSignerCmd: undefined,
      unsigned: true,
    });
    expect(signer.type).toBe("unsigned");
  });

  test("throws with helpful message when nothing configured", async () => {
    await expect(
      resolveBtcSigner({ privateKey: undefined, envPrivateKey: undefined, externalSignerCmd: undefined, unsigned: false })
    ).rejects.toThrow("no signer configured for Bitcoin");
  });
});
