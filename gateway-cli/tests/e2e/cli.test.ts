import { describe, it, expect } from "vitest";
import { execFileSync } from "node:child_process";
import { fileURLToPath } from "node:url";
import { dirname, resolve } from "node:path";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
const CLI = resolve(__dirname, "../../bin/gateway-cli.ts");

function run(...args: string[]): string {
  return execFileSync("npx", ["tsx", CLI, ...args], { encoding: "utf-8" });
}

describe("CLI smoke tests", () => {
  it("--help shows description", () => {
    expect(run("--help")).toContain("Swap between BTC");
  });

  it("--version shows semver", () => {
    expect(run("--version")).toMatch(/\d+\.\d+\.\d+/);
  });

  it("swap --help shows flags (no --dry-run)", () => {
    const output = run("swap", "--help");
    expect(output).toContain("--unsigned");
    expect(output).toContain("--src");
    expect(output).not.toContain("--dry-run");
  });

  it("swap --help shows unified amount flag", () => {
    const output = run("swap", "--help");
    expect(output).toContain("--amount");
    expect(output).not.toContain("--amount-atomic");
    expect(output).not.toContain("--amount-usd");
  });

  it("quote --help shows flags", () => {
    const output = run("quote", "--help");
    expect(output).toContain("--src");
    expect(output).toContain("--amount");
  });

  it("routes --help shows flags", () => {
    const output = run("routes", "--help");
    expect(output).toContain("--src-chain");
  });

  it("balance --help does not show --no-cache", () => {
    const output = run("balance", "--help");
    expect(output).not.toContain("--no-cache");
  });

  it("swap --help shows fee-token and fee-reserve flags", () => {
    const output = run("swap", "--help");
    expect(output).toContain("--fee-token");
    expect(output).toContain("--fee-reserve");
  });

  it("balance --help shows fee-token and fee-reserve flags", () => {
    const output = run("balance", "--help");
    expect(output).toContain("--fee-token");
    expect(output).toContain("--fee-reserve");
  });

  it("quote --help shows fee-token and fee-reserve flags", () => {
    const output = run("quote", "--help");
    expect(output).toContain("--fee-token");
    expect(output).toContain("--fee-reserve");
  });

  it("offramp --help works (hidden alias)", () => {
    const output = run("offramp", "--help");
    expect(output).toContain("--src");
  });
});
