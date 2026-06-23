import { describe, it, expect } from "vitest";
import { validateRecipient } from "../../src/chains/index.js";

const EVM = "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913";
const BTC = "bc1qar0srrr7xfkvy5l643lydnw9re59gtzzwf5mdq";

describe("validateRecipient", () => {
  it("accepts an EVM address for an EVM chain", () => {
    expect(() => validateRecipient("base", EVM)).not.toThrow();
  });
  it("accepts a BTC address for bitcoin", () => {
    expect(() => validateRecipient("bitcoin", BTC)).not.toThrow();
  });
  it("rejects a BTC address for an EVM chain", () => {
    expect(() => validateRecipient("base", BTC)).toThrow(/valid EVM address/i);
  });
  it("rejects an EVM address for bitcoin", () => {
    expect(() => validateRecipient("bitcoin", EVM)).toThrow(/valid Bitcoin address/i);
  });
});
