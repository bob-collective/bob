#!/usr/bin/env ts-node

import {
  createPublicClient,
  http,
  isAddress,
  getAddress,
  erc20Abi,
  Chain,
  zeroAddress,
  PublicClient,
} from 'viem';
import * as fs from 'fs';
import * as path from 'path';
import Ajv from 'ajv';
import addFormats from 'ajv-formats';
import { SUPPORTED_CHAINS, TOKENLIST_SCHEMA_URL } from '../config';

const tokenlistPath = path.join(__dirname, '../tokenlist.json');

// Build chainId → Chain lookup from config
const chainById = new Map<number, Chain>(
  SUPPORTED_CHAINS.map((chain) => [chain.id, chain]),
);

// Cache for token info to avoid refetching
const tokenCache = new Map<string, { name: string; symbol: string; decimals: number }>();

// One persistent client per chain — reused across all tokens on that chain
// Using any type because viem client types are incompatible across different chains
const clients = new Map<number, any>(
  SUPPORTED_CHAINS.map((chain) => [chain.id, createPublicClient({ chain, transport: http(chain.rpcUrls.default.http[0]) })]),
);

interface Token {
  name: string;
  address: string;
  symbol: string;
  decimals: number;
  chainId: number;
  logoURI: string;
}

interface ValidationResult {
  address: string;
  symbol: string;
  chainId: number;
  issues: string[];
}

function validateAddress(address: string): boolean {
  try {
    return isAddress(address) && address === getAddress(address);
  } catch {
    return false;
  }
}

// Validate all tokens on one chain in a single multicall round-trip
async function validateChainTokens(
  chainId: number,
  chainTokens: Token[],
  client: PublicClient,
): Promise<ValidationResult[]> {
  // Split out invalid addresses early — no point including them in the RPC call
  const toFetch: Token[] = [];
  const earlyFails: ValidationResult[] = [];

  for (const token of chainTokens) {
    if (token.address !== zeroAddress && !validateAddress(token.address)) {
      earlyFails.push({
        address: token.address,
        symbol: token.symbol,
        chainId,
        issues: [`Invalid or non-checksummed address, should be ${getAddress(token.address)}`],
      });
    } else {
      toFetch.push(token);
    }
  }

  const contractTokens = toFetch.filter((t) => t.address !== zeroAddress);
  const nativeTokens = toFetch.filter((t) => t.address === zeroAddress);

  // Single multicall for all contract tokens on this chain — one RPC round-trip regardless of count
  const calls = contractTokens.flatMap((t) => [
    { address: t.address as `0x${string}`, abi: erc20Abi, functionName: 'name' as const },
    { address: t.address as `0x${string}`, abi: erc20Abi, functionName: 'symbol' as const },
    { address: t.address as `0x${string}`, abi: erc20Abi, functionName: 'decimals' as const },
  ]);

  let multicallResults: any[];
  try {
    multicallResults = calls.length > 0
      ? await client.multicall({ contracts: calls, allowFailure: true })
      : [];
  } catch (err) {
    // RPC unreachable or multicall3 not deployed — mark all tokens as failed
    return contractTokens.map((token) => ({
      address: token.address,
      symbol: token.symbol,
      chainId,
      issues: [`Multicall failed: ${err instanceof Error ? err.message : String(err)}`],
    }));
  }

  const contractResults: ValidationResult[] = contractTokens.map((token, i) => {
    const issues: string[] = [];

    if (!token.logoURI.startsWith('http') && !token.logoURI.startsWith('ipfs://')) {
      issues.push('Invalid logoURI: must start with http or ipfs://');
    }

    const nameRes = multicallResults[i * 3];
    const symbolRes = multicallResults[i * 3 + 1];
    const decimalsRes = multicallResults[i * 3 + 2];

    if (nameRes?.status === 'failure' || symbolRes?.status === 'failure' || decimalsRes?.status === 'failure') {
      issues.push('Unable to fetch on-chain data — contract may not exist or not be ERC20');
      return { address: token.address, symbol: token.symbol, chainId, issues };
    }

    const onChainName = nameRes?.result as string;
    const onChainSymbol = symbolRes?.result as string;
    const onChainDecimals = Number(decimalsRes?.result);

    if (token.name !== onChainName) {
      issues.push(`Name mismatch: tokenlist="${token.name}", onchain="${onChainName}"`);
    }
    if (token.symbol !== onChainSymbol) {
      issues.push(`Symbol mismatch: tokenlist="${token.symbol}", onchain="${onChainSymbol}"`);
    }
    if (token.decimals !== onChainDecimals) {
      issues.push(`Decimals mismatch: tokenlist=${token.decimals}, onchain=${onChainDecimals}`);
    }

    return { address: token.address, symbol: token.symbol, chainId, issues };
  });

  const nativeResults: ValidationResult[] = nativeTokens.map((token) => {
    const issues: string[] = [];
    if (!token.logoURI.startsWith('http') && !token.logoURI.startsWith('ipfs://')) {
      issues.push('Invalid logoURI: must start with http or ipfs://');
    }
    return { address: token.address, symbol: token.symbol, chainId, issues };
  });

  return [...earlyFails, ...contractResults, ...nativeResults];
}

async function validateSchema() {
  const ajv = new Ajv({ allErrors: true, verbose: true });
  addFormats(ajv);
  const schemaResponse = await fetch(TOKENLIST_SCHEMA_URL);
  const schema = await schemaResponse.json() as any;
  const validator = ajv.compile(schema);
  const data = JSON.parse(fs.readFileSync(tokenlistPath, 'utf8'));
  const valid = validator(data);
  if (valid) {
    console.log('Schema: valid\n');
    return;
  }
  if (validator.errors) {
    throw validator.errors.map((e) => { delete e.data; return e; });
  }
}

async function main() {
  console.log('🔍 Verifying all tokens across all chains...\n');

  if (!fs.existsSync(tokenlistPath)) {
    console.error('❌ tokenlist.json not found');
    process.exit(1);
  }

  await validateSchema();

  const { tokens }: { tokens: Token[] } = JSON.parse(
    fs.readFileSync(tokenlistPath, 'utf8'),
  );
  console.log(`📋 Found ${tokens.length} tokens across all chains\n`);

  // Group by chainId
  const byChain = new Map<number, Token[]>();
  for (const token of tokens) {
    const list = byChain.get(token.chainId) ?? [];
    list.push(token);
    byChain.set(token.chainId, list);
  }

  // Warn about unknown chains
  for (const chainId of byChain.keys()) {
    if (!chainById.has(chainId)) {
      console.warn(`⚠️  Unknown chainId ${chainId} — tokens on this chain will be skipped`);
    }
  }

  // Validate all chains in parallel — each chain is one multicall round-trip
  const chainEntries = [...byChain.entries()].filter(([chainId]) => chainById.has(chainId));

  const settled = await Promise.allSettled(
    chainEntries.map(([chainId, chainTokens]) =>
      validateChainTokens(chainId, chainTokens, clients.get(chainId)!),
    ),
  );

  // Print results per chain and accumulate totals
  const allResults: ValidationResult[] = [];
  let totalIssues = 0;

  for (let i = 0; i < chainEntries.length; i++) {
    const [chainId, chainTokens] = chainEntries[i];
    const chain = chainById.get(chainId)!;
    const outcome = settled[i];

    console.log(`\n🔗 Chain: ${chain.name} (${chainId}) — ${chainTokens.length} tokens`);

    if (outcome.status === 'rejected') {
      console.log(`  ❌ Chain validation failed: ${outcome.reason instanceof Error ? outcome.reason.message : String(outcome.reason)}`);
      totalIssues += chainTokens.length;
      continue;
    }

    const results = outcome.value;
    for (const result of results) {
      allResults.push(result);
      if (result.issues.length > 0) {
        console.log(`  ❌ ${result.symbol} (${result.address})`);
        result.issues.forEach((issue) => console.log(`     • ${issue}`));
        totalIssues += result.issues.length;
      } else {
        console.log(`  ✅ ${result.symbol} (${result.address})`);
      }
    }

    // Duplicate address check within the chain
    const addrs = chainTokens.map((t) => t.address.toLowerCase());
    const dupes = addrs.filter((a, idx) => addrs.indexOf(a) !== idx);
    if (dupes.length > 0) {
      console.log(`  ⚠️  Duplicate addresses: ${[...new Set(dupes)].join(', ')}`);
    }
  }

  // Summary
  const tokensWithIssues = allResults.filter((r) => r.issues.length > 0).length;
  console.log('\n📊 Summary:');
  console.log(`   Chains checked:      ${chainEntries.length}`);
  console.log(`   Total tokens:        ${tokens.length}`);
  console.log(`   Valid tokens:        ${allResults.length - tokensWithIssues}`);
  console.log(`   Tokens with issues:  ${tokensWithIssues}`);
  console.log(`   Total issues:        ${totalIssues}`);

  if (totalIssues === 0) {
    console.log('\n🎉 All tokens valid!');
    process.exit(0);
  } else {
    console.log('\n❌ Validation failed with issues');
    process.exit(1);
  }
}

main().catch(console.error);
