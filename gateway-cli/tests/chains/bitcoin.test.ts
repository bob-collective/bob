import { describe, it, expect, vi, beforeEach } from "vitest";

// ─── Mock fixtures ──────────────────────────────────────────────────────────

const mockGetBalance = vi.fn();
const mockGetMaxSpendable = vi.fn();
const mockSignAllInputs = vi.fn();
const mockGetP2WPKHAddress = vi.fn();

vi.mock("@gobob/bob-sdk", () => ({
  EsploraClient: vi.fn(() => ({
    getBalance: mockGetBalance,
  })),
  ScureBitcoinSigner: {
    fromKey: vi.fn(() => ({
      signAllInputs: mockSignAllInputs,
      getP2WPKHAddress: mockGetP2WPKHAddress,
    })),
  },
}));

vi.mock("../../src/config.js", () => ({
  getSdk: vi.fn(() => ({
    getMaxSpendable: mockGetMaxSpendable,
  })),
}));

// ─── Import after mocks ─────────────────────────────────────────────────────

import { getBtcBalance } from "../../src/chains/bitcoin.js";
import { getSdk } from "../../src/config.js";

// ─── Tests ──────────────────────────────────────────────────────────────────

describe("getBtcBalance", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("returns total and allSpendable with correct values", async () => {
    mockGetBalance.mockResolvedValue({ confirmed: 500000, unconfirmed: 10000, total: 510000 });
    mockGetMaxSpendable.mockResolvedValue({ amount: { amount: "490000" } });

    const sdk = getSdk();
    const result = await getBtcBalance("bc1qtest", sdk);

    expect(result.total).toBe("510000"); // confirmed + unconfirmed
    expect(result.allSpendable).toBe("490000");
  });

  it("returns zero values when balance is zero", async () => {
    mockGetBalance.mockResolvedValue({ confirmed: 0, unconfirmed: 0, total: 0 });
    mockGetMaxSpendable.mockResolvedValue({ amount: { amount: "0" } });

    const sdk = getSdk();
    const result = await getBtcBalance("bc1qtest", sdk);

    expect(result.total).toBe("0");
    expect(result.allSpendable).toBe("0");
  });

  it("handles large balances correctly", async () => {
    mockGetBalance.mockResolvedValue({ confirmed: 2100000000000000, unconfirmed: 0, total: 2100000000000000 });
    mockGetMaxSpendable.mockResolvedValue({ amount: { amount: "2099999999000000" } });

    const sdk = getSdk();
    const result = await getBtcBalance("bc1qtest", sdk);

    expect(result.total).toBe("2100000000000000");
    expect(result.allSpendable).toBe("2099999999000000");
  });
});

