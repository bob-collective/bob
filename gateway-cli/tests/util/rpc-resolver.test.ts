import { describe, it, expect, beforeEach, afterEach } from 'vitest';
import { resolveRpcUrl } from '../../src/util/rpc-resolver';

describe('resolveRpcUrl', () => {
  const originalEnv = process.env;

  beforeEach(() => { process.env = { ...originalEnv }; });
  afterEach(() => { process.env = originalEnv; });

  it('resolves per-chain env var first', () => {
    process.env.EVM_RPC_URL_BOB = 'https://custom-bob-rpc.com';
    process.env.EVM_RPC_URL = 'https://default-rpc.com';
    expect(resolveRpcUrl('bob', {})).toBe('https://custom-bob-rpc.com');
  });

  it('falls back to config.toml rpc section', () => {
    const tomlRpc = { bob: 'https://toml-bob-rpc.com' };
    expect(resolveRpcUrl('bob', tomlRpc)).toBe('https://toml-bob-rpc.com');
  });

  it('falls back to EVM_RPC_URL', () => {
    process.env.EVM_RPC_URL = 'https://default-rpc.com';
    expect(resolveRpcUrl('bob', {})).toBe('https://default-rpc.com');
  });

  it('falls back to built-in public RPCs', () => {
    const url = resolveRpcUrl('bob', {});
    expect(url).toBe('https://rpc.gobob.xyz');
  });

  it('throws for unknown chain with no config', () => {
    expect(() => resolveRpcUrl('unknown-chain', {})).toThrow('No RPC URL configured');
  });
});
