import { describe, test, expect } from "vitest";
import { decryptKeystore } from "../../src/signer/evm.js";

describe("decryptKeystore", () => {
  test("throws on missing keystore file", async () => {
    await expect(decryptKeystore("/nonexistent/path.json", "password"))
      .rejects.toThrow("keystore file not found");
  });

  test("throws on wrong password", async () => {
    const ks = JSON.stringify({
      address: "88a5c2d9919e46f883eb62f7b8dd9d0cc45bc290",
      crypto: {
        cipher: "aes-128-ctr",
        ciphertext: "cb09bee28a6c84a4e1668b3a506e3e21c09ae6a86a4c5671de26d3b9aa95e08e",
        cipherparams: { iv: "6bc8a5b5476d9e8c8d0e9e49a38e23b3" },
        kdf: "pbkdf2",
        kdfparams: { c: 1024, dklen: 32, prf: "hmac-sha256", salt: "ae3cd4e7013836a3df6bd7241b12db061dbe2c6785853cce422d148a624ce0bd" },
        mac: "517ead924a9d0dc3124507e3393d175ce3ff7c1e96529c6c555ce9e51205e9b2",
      },
      id: "3198bc9c-6672-5ab3-d995-4942343ae5b6",
      version: 3,
    });

    const { writeFileSync, unlinkSync } = await import("node:fs");
    const { join } = await import("node:path");
    const { tmpdir } = await import("node:os");
    const path = join(tmpdir(), "test-keystore.json");
    writeFileSync(path, ks);

    await expect(decryptKeystore(path, "wrongpassword"))
      .rejects.toThrow("invalid keystore password");

    unlinkSync(path);
  });
});
