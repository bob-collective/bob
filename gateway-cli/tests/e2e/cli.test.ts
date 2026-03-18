import { describe, it, expect } from "vitest";
import { execFileSync } from "node:child_process";
import { fileURLToPath } from "node:url";
import { dirname, resolve } from "node:path";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
const CLI = resolve(__dirname, "../../bin/gateway-cli.ts");

function run(...args: string[]): string {
  return execFileSync("npx", ["tsx", CLI, ...args], {
    encoding: "utf-8",
  });
}

describe("CLI smoke tests", () => {
  it("--help", () => {
    expect(run("--help")).toContain(
      "Bridge Bitcoin to any chain"
    );
  });

  it("--version", () => {
    expect(run("--version")).toMatch(/\d+\.\d+\.\d+/);
  });

  it("routes --help", () => {
    const output = run("routes", "--help");
    expect(output).toContain("--src-chain");
    expect(output).toContain("--dst-chain");
  });

  it("swap --help", () => {
    const output = run("swap", "--help");
    expect(output).toContain("--unsigned");
    expect(output).toContain("--auto-confirm");
    expect(output).toContain("--src");
    expect(output).toContain("--dst");
  });

  it("offramp --help", () => {
    const output = run("offramp", "--help");
    expect(output).toContain("--src");
    expect(output).toContain("--dst");
    expect(output).toContain("--recipient");
  });

  it("chains --help", () => {
    const output = run("chains", "--help");
    expect(output).toContain("--json");
  });

  it("quote --help", () => {
    const output = run("quote", "--help");
    expect(output).toContain("--src");
    expect(output).toContain("--dst");
    expect(output).toContain("--amount");
    expect(output).toContain("--recipient");
  });
});
