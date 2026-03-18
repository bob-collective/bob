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

// Tokens to skip validation for — keyed by "chainId:address" (address lowercase).
// Add entries here when on-chain data is known to differ from tokenlist for intentional reasons.
const SKIP_VALIDATION = new Set<string>([
  // WBTCOFT Adapter on Ethereum — not a standard ERC20, it's a LayerZero bridge adapter
  `1:0x0555e30da8f98308edb960aa94c0db47230d2b9c`,
  // WBTCOFT on OP Mainnet — contract not fetchable
  `10:0xc3f854b2970f8727d28527ece33176fac67fef48`,
  // BABY on BOB Sepolia — testnet address points to wrong contract ("ubbn")
  `808813:0x5e159518b8303a1f4ec9f9b10f077c89795db178`,
]);

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
  skipped?: boolean;
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
  const skippedResults: ValidationResult[] = [];

  for (const token of chainTokens) {
    if (SKIP_VALIDATION.has(`${chainId}:${token.address.toLowerCase()}`)) {
      skippedResults.push({ address: token.address, symbol: token.symbol, chainId, issues: [], skipped: true });
      continue;
    }
    if (token.address !== zeroAddress && !validateAddress(token.address)) {
      let suggestion: string;
      try {
        suggestion = getAddress(token.address);
      } catch {
        suggestion = '(unable to compute checksum — address is malformed)';
      }
      earlyFails.push({
        address: token.address,
        symbol: token.symbol,
        chainId,
        issues: [`Invalid or non-checksummed address, should be ${suggestion}`],
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

    if (!nameRes || !symbolRes || !decimalsRes) {
      issues.push('Incomplete multicall response — expected 3 results per token');
      return { address: token.address, symbol: token.symbol, chainId, issues };
    }

    if (nameRes.status === 'failure' || symbolRes.status === 'failure' || decimalsRes.status === 'failure') {
      issues.push('Unable to fetch on-chain data — contract may not exist or not be ERC20');
      return { address: token.address, symbol: token.symbol, chainId, issues };
    }

    const onChainName = nameRes.result as string;
    const onChainSymbol = symbolRes.result as string;
    const onChainDecimals = Number(decimalsRes.result);

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

  return [...skippedResults, ...earlyFails, ...contractResults, ...nativeResults];
}

async function validateSchema() {
  const ajv = new Ajv({ allErrors: true, verbose: true });
  addFormats(ajv);
  const schemaResponse = await fetch(TOKENLIST_SCHEMA_URL);
  if (!schemaResponse.ok) {
    throw new Error(`Failed to fetch schema: ${schemaResponse.status} ${schemaResponse.statusText}`);
  }
  const schema = await schemaResponse.json() as any;
  const validator = ajv.compile(schema);
  const data = JSON.parse(fs.readFileSync(tokenlistPath, 'utf8'));
  const valid = validator(data);
  if (valid) {
    console.log('Schema: valid\n');
    return;
  }
  throw validator.errors
    ? validator.errors.map((e) => { delete e.data; return e; })
    : new Error('Schema validation failed with no error details');
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

  // Print results per chain and accumulate totals
  const allResults: ValidationResult[] = [];
  let totalIssues = 0;

  // Treat unknown chains as errors — tokens cannot be verified without a configured client
  for (const chainId of byChain.keys()) {
    if (!chainById.has(chainId)) {
      const unknownTokens = byChain.get(chainId)!;
      console.error(`\n❌ Unknown chainId ${chainId} — ${unknownTokens.length} token(s) cannot be verified`);
      for (const token of unknownTokens) {
        allResults.push({ address: token.address, symbol: token.symbol, chainId, issues: [`Unknown chainId ${chainId} — not in SUPPORTED_CHAINS`] });
        totalIssues++;
      }
    }
  }

  // Validate all chains in parallel — each chain is one multicall round-trip
  const chainEntries = [...byChain.entries()].filter(([chainId]) => chainById.has(chainId));

  const settled = await Promise.allSettled(
    chainEntries.map(([chainId, chainTokens]) =>
      validateChainTokens(chainId, chainTokens, clients.get(chainId)!),
    ),
  );

  for (let i = 0; i < chainEntries.length; i++) {
    const [chainId, chainTokens] = chainEntries[i];
    const chain = chainById.get(chainId)!;
    const outcome = settled[i];

    console.log(`\n🔗 Chain: ${chain.name} (${chainId}) — ${chainTokens.length} tokens`);

    if (outcome.status === 'rejected') {
      console.log(`  ❌ Chain validation failed: ${outcome.reason instanceof Error ? outcome.reason.message : String(outcome.reason)}`);
      for (const token of chainTokens) {
        allResults.push({ address: token.address, symbol: token.symbol, chainId, issues: ['Chain validation failed'] });
      }
      totalIssues += chainTokens.length;
      continue;
    }

    const results = outcome.value;
    for (const result of results) {
      allResults.push(result);
      if (result.skipped) {
        console.log(`  ⏭️  ${result.symbol} (${result.address}) — skipped`);
      } else if (result.issues.length > 0) {
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

    // Duplicate symbol check within the chain
    const symbols = chainTokens.map((t) => t.symbol);
    const dupeSymbols = symbols.filter((s, idx) => symbols.indexOf(s) !== idx);
    if (dupeSymbols.length > 0) {
      console.log(`  ⚠️  Duplicate symbols: ${[...new Set(dupeSymbols)].join(', ')}`);
    }
  }

  // Summary
  const skippedCount = allResults.filter((r) => r.skipped).length;
  const tokensWithIssues = allResults.filter((r) => !r.skipped && r.issues.length > 0).length;
  console.log('\n📊 Summary:');
  console.log(`   Chains checked:      ${chainEntries.length}`);
  console.log(`   Total tokens:        ${tokens.length}`);
  console.log(`   Valid tokens:        ${allResults.length - tokensWithIssues - skippedCount}`);
  console.log(`   Skipped tokens:      ${skippedCount}`);
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

main().catch((err) => {
  console.error('Fatal verification error:', err);
  process.exit(1);
});
