import * as btc from "@scure/btc-signer";
import { spawn } from "node:child_process";

// ─── Error types ─────────────────────────────────────────────────────────────

export class SignerError extends Error {
  constructor(
    public readonly command: string,
    public readonly stderr: string,
    message?: string,
  ) {
    super(message || `Signer command "${command}" failed: ${stderr}`);
    this.name = "SignerError";
  }
}

// ─── Types ────────────────────────────────────────────────────────────────────

export type BtcSignerSpec =
  | { type: "private-key"; key: string }
  | { type: "external"; command: string }
  | { type: "unsigned" };

export interface BtcSignerOptions {
  privateKey?: string;
  envPrivateKey?: string;
  externalSignerCmd?: string;
  unsigned: boolean;
}

// ─── Private key signing ─────────────────────────────────────────────────────

export async function signPsbtWithPrivateKey(
  privateKey: string,
  psbtBase64: string,
): Promise<string> {
  let keyBytes: Uint8Array;
  try {
    // Try WIF first; fall back to hex
    try {
      keyBytes = btc.WIF().decode(privateKey);
    } catch {
      const hex = privateKey.startsWith("0x") ? privateKey.slice(2) : privateKey;
      const buf = Buffer.from(hex, "hex");
      if (buf.length !== 32) {
        throw new Error(`hex private key must be 32 bytes (64 hex chars), got ${buf.length}`);
      }
      keyBytes = buf;
    }
  } catch (err) {
    throw new Error(`invalid private key: ${err instanceof Error ? err.message : err}`);
  }

  let psbtBytes: Uint8Array;
  try {
    psbtBytes = Buffer.from(psbtBase64, "base64");
  } catch (err) {
    throw new Error(`invalid PSBT base64: ${err instanceof Error ? err.message : err}`);
  }

  const tx = btc.Transaction.fromPSBT(psbtBytes);
  tx.sign(keyBytes);
  tx.finalize();
  return Buffer.from(tx.extract()).toString("hex");
}

// ─── External signer ─────────────────────────────────────────────────────────

export class ExternalSigner {
  constructor(private command: string) {}

  async sign(input: string): Promise<string> {
    return new Promise((resolve, reject) => {
      const parts = this.command.split(/\s+/);
      const cmd = parts[0];
      const args = parts.slice(1);
      const child = spawn(cmd, args, { stdio: ["pipe", "pipe", "pipe"] });

      let stdout = "";
      let stderr = "";

      child.stdout.on("data", (data: Buffer) => {
        stdout += data.toString();
      });
      child.stderr.on("data", (data: Buffer) => {
        stderr += data.toString();
      });
      child.on("error", (err: Error) => {
        reject(new SignerError(this.command, stderr || err.message));
      });
      child.on("close", (code: number | null) => {
        if (code !== 0) reject(new SignerError(this.command, stderr));
        else resolve(stdout.trim());
      });

      child.stdin.write(input);
      child.stdin.end();
    });
  }
}

// ─── Signer resolver ─────────────────────────────────────────────────────────

export async function resolveBtcSigner(opts: BtcSignerOptions): Promise<BtcSignerSpec> {
  if (opts.unsigned) return { type: "unsigned" };
  const key = opts.privateKey ?? opts.envPrivateKey;
  if (key) return { type: "private-key", key };
  if (opts.externalSignerCmd) return { type: "external", command: opts.externalSignerCmd };
  throw new Error(
    "no signer configured for Bitcoin.\n" +
    "  Set BITCOIN_PRIVATE_KEY, BITCOIN_SIGNER, or pass --private-key.\n" +
    "  Use --unsigned to output the PSBT without signing.",
  );
}

// ─── Sign via resolved spec ───────────────────────────────────────────────────

export async function signBtcWithSpec(
  spec: BtcSignerSpec,
  psbtBase64: string,
): Promise<string> {
  if (spec.type === "private-key") {
    return signPsbtWithPrivateKey(spec.key, psbtBase64);
  }
  if (spec.type === "external") {
    const ext = new ExternalSigner(spec.command);
    return ext.sign(psbtBase64);
  }
  throw new Error("signBtcWithSpec called with unsigned spec");
}
