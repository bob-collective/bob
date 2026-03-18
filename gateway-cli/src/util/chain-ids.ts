const ALIASES: Record<string, string> = {
  btc: "bitcoin", eth: "ethereum", mainnet: "ethereum",
  arb: "arbitrum", arb1: "arbitrum", "arbitrum-one": "arbitrum",
  bas: "base", opt: "optimism", oeth: "optimism",
  pol: "polygon", bnb: "bsc", avax: "avalanche",
};

export function resolveChain(input: string, knownChains?: string[]): string {
  const lower = input.toLowerCase();
  const resolved = ALIASES[lower] ?? lower;
  if (knownChains && !knownChains.includes(resolved)) {
    throw new Error(`unknown chain "${input}". Known chains: ${knownChains.join(", ")}`);
  }
  return resolved;
}
