/** Well-known TRC-20 tokens on Tron (gateway routes). */
const TRON_KNOWN_TOKENS: Record<string, { symbol: string; decimals: number }> = {
  TR7NHqjeKQxGTCi8q8ZY4pL8otSzgjLj6t: { symbol: 'USDT', decimals: 6 },
};

const TRON_ZERO_ADDRESS = '0x0000000000000000000000000000000000000000';

export function getTronTokenMetadata(
  address: string,
  opts?: { throwOnUnknown?: boolean },
): { symbol: string; decimals: number } {
  if (address === TRON_ZERO_ADDRESS || address.toUpperCase() === 'TRX') {
    return { symbol: 'TRX', decimals: 6 };
  }

  const known = TRON_KNOWN_TOKENS[address];
  if (known) return known;

  if (opts?.throwOnUnknown === false) {
    return { symbol: address.slice(0, 10), decimals: 6 };
  }

  throw new Error(
    `Unknown token ${address} on chain "tron" — cannot determine decimals. Use a known token symbol or verify the address.`,
  );
}
