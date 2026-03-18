import { describe, it, expect } from "vitest";
import { formatOutput } from "../../src/output/formatter.js";

describe("formatOutput", () => {
  describe("json mode", () => {
    it("returns JSON with 2-space indent for objects", () => {
      const data = { orderId: "abc-123", status: "success" };
      const result = formatOutput(data, "json");
      expect(result).toBe(JSON.stringify(data, null, 2));
    });

    it("roundtrips: parsing output returns original data", () => {
      const data = { nested: { a: 1, b: [2, 3] }, flag: true };
      const result = formatOutput(data, "json");
      expect(JSON.parse(result)).toEqual(data);
    });

    it("formats arrays as JSON", () => {
      const data = [{ id: 1 }, { id: 2 }];
      const result = formatOutput(data, "json");
      expect(JSON.parse(result)).toEqual(data);
    });

    it("formats primitive values as JSON", () => {
      expect(formatOutput("hello", "json")).toBe('"hello"');
      expect(formatOutput(42, "json")).toBe("42");
      expect(formatOutput(null, "json")).toBe("null");
    });
  });

  describe("human mode", () => {
    it("formats object as key: value lines with camelCase to Title Case", () => {
      const data = { orderId: "abc-123", srcChain: "bitcoin" };
      const result = formatOutput(data, "human");
      expect(result).toContain("Order Id: abc-123");
      expect(result).toContain("Src Chain: bitcoin");
    });

    it("formats nested objects inline as JSON", () => {
      const data = { status: { inProgress: { bumpFeeTx: null } } };
      const result = formatOutput(data, "human");
      expect(result).toContain("Status:");
      expect(result).toContain("inProgress");
    });

    it("formats arrays as numbered items", () => {
      const data = [
        { orderId: "a", status: "success" },
        { orderId: "b", status: "refunded" },
      ];
      const result = formatOutput(data, "human");
      expect(result).toContain("[1]");
      expect(result).toContain("[2]");
      expect(result).toContain("Order Id: a");
      expect(result).toContain("Order Id: b");
    });

    it("handles non-object values", () => {
      expect(formatOutput("hello", "human")).toBe("hello");
      expect(formatOutput(42, "human")).toBe("42");
    });
  });
});
