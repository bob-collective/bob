import { describe, test, expect, vi, beforeEach } from "vitest";
import { setJsonMode, isJsonMode, progress, warn, setVerboseMode, verbose } from "../../src/util/progress.js";

describe("progress", () => {
  beforeEach(() => {
    setJsonMode(false);
    setVerboseMode(false);
  });

  test("isJsonMode returns current state", () => {
    expect(isJsonMode()).toBe(false);
    setJsonMode(true);
    expect(isJsonMode()).toBe(true);
  });

  test("progress writes to stderr when not in json mode", () => {
    const spy = vi.spyOn(process.stderr, "write").mockImplementation(() => true);
    progress("test message");
    expect(spy).toHaveBeenCalledWith("test message\n");
    spy.mockRestore();
  });

  test("progress is suppressed in json mode", () => {
    setJsonMode(true);
    const spy = vi.spyOn(process.stderr, "write").mockImplementation(() => true);
    progress("test message");
    expect(spy).not.toHaveBeenCalled();
    spy.mockRestore();
  });

  test("warn writes Warning prefix to stderr", () => {
    const spy = vi.spyOn(process.stderr, "write").mockImplementation(() => true);
    warn("something happened");
    expect(spy).toHaveBeenCalledWith("Warning: something happened\n");
    spy.mockRestore();
  });

  test("verbose writes only when verbose mode enabled", () => {
    const spy = vi.spyOn(process.stderr, "write").mockImplementation(() => true);
    verbose("debug info");
    expect(spy).not.toHaveBeenCalled();
    setVerboseMode(true);
    verbose("debug info");
    expect(spy).toHaveBeenCalledWith("[verbose] debug info\n");
    spy.mockRestore();
  });
});
