const PUBLIC_RPCS: Record<string, string> = {
  bob: 'https://rpc.gobob.xyz',
  ethereum: 'https://eth.llamarpc.com',
  base: 'https://mainnet.base.org',
  arbitrum: 'https://arb1.arbitrum.io/rpc',
  optimism: 'https://mainnet.optimism.io',
  bsc: 'https://bsc-dataseed.binance.org',
};

export function resolveRpcUrl(
  chainName: string,
  tomlRpc: Record<string, string>,
): string {
  // 1. Per-chain env var
  const envKey = `EVM_RPC_URL_${chainName.toUpperCase()}`;
  const perChainEnv = process.env[envKey];
  if (perChainEnv) return perChainEnv;

  // 2. TOML config
  const tomlUrl = tomlRpc[chainName];
  if (tomlUrl) return tomlUrl;

  // 3. Fallback env var
  const defaultEnv = process.env.EVM_RPC_URL;
  if (defaultEnv) return defaultEnv;

  // 4. Built-in public RPCs
  const publicRpc = PUBLIC_RPCS[chainName];
  if (publicRpc) return publicRpc;

  throw new Error(`No RPC URL configured for chain "${chainName}". Set EVM_RPC_URL_${chainName.toUpperCase()} or add [rpc] ${chainName} = "..." to config.toml`);
}
