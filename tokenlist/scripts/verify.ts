#!/usr/bin/env ts-node

import {
  createPublicClient,
  http,
  isAddress,
  getAddress,
  parseAbi,
  Chain,
} from 'viem';
import * as fs from 'fs';
import * as path from 'path';
import { bob, mainnet } from 'viem/chains';
import Ajv from 'ajv';
import addFormats from 'ajv-formats';

const SCHEMA_URL =
  'https://raw.githubusercontent.com/Uniswap/token-lists/refs/heads/main/src/tokenlist.schema.json';
const tokenlistPath = path.join(__dirname, '../token-list-bob.json');

// ERC20 ABI for name, symbol, decimals
const erc20Abi = parseAbi([
  'function name() view returns (string)',
  'function symbol() view returns (string)',
  'function decimals() view returns (uint8)',
]);

interface Token {
  name: string;
  address: string;
  symbol: string;
  decimals: number;
  chainId: number;
  logoURI: string;
  extensions?: {
    // "bridgeInfo"
    [key: string]: {
      // "1"
      [key: string]: {
        // "tokenAddress"
        [key: string]: any;
      };
    };
  };
}

interface ValidationResult {
  address: string;
  symbol: string;
  issues: string[];
  onChainData?: {
    name: string;
    symbol: string;
    decimals: number;
  };
}

function validateTokenAddress(address: string): boolean {
  try {
    // Check if address is valid
    if (!isAddress(address)) {
      return false;
    }

    // Check if address is checksummed
    const checksummed = getAddress(address);
    return address === checksummed;
  } catch {
    return false;
  }
}

async function getTokenInfo(
  chain: Chain,
  address: string,
): Promise<{ name: string; symbol: string; decimals: number } | null> {
  try {
    const client = createPublicClient({
      chain,
      transport: http(),
    });

    // Skip ETH (zero address)
    if (address === '0x0000000000000000000000000000000000000000') {
      return { name: 'Ether', symbol: 'ETH', decimals: 18 };
    }

    const [name, symbol, decimals] = await Promise.all([
      client.readContract({
        address: address as `0x${string}`,
        abi: erc20Abi,
        functionName: 'name',
      }),
      client.readContract({
        address: address as `0x${string}`,
        abi: erc20Abi,
        functionName: 'symbol',
      }),
      client.readContract({
        address: address as `0x${string}`,
        abi: erc20Abi,
        functionName: 'decimals',
      }),
    ]);

    return {
      name: name as string,
      symbol: symbol as string,
      decimals: Number(decimals),
    };
  } catch (error) {
    console.error(`Error fetching token info for ${address}:`, error);
    return null;
  }
}

async function validateToken(token: Token): Promise<ValidationResult> {
  const issues: string[] = [];
  const result: ValidationResult = {
    address: token.address,
    symbol: token.symbol,
    issues,
  };

  // Validate chain ID
  if (token.chainId !== 60808) {
    issues.push(`Invalid chainId: expected 60808, got ${token.chainId}`);
  }

  // Validate address format and checksum
  const isValidAddress = validateTokenAddress(token.address);
  if (!isValidAddress) {
    issues.push(
      `Invalid or non-checksummed address, should be ${getAddress(
        token.address,
      )}`,
    );
    return result; // Can't continue validation without valid address
  }

  // Get on-chain data
  const onChainData = await getTokenInfo(bob, token.address);
  if (!onChainData) {
    issues.push(
      'Unable to fetch on-chain token data - contract may not exist or not be ERC20',
    );
    return result;
  }

  result.onChainData = onChainData;

  // Compare with tokenlist data
  if (token.name !== onChainData.name) {
    issues.push(
      `Name mismatch: tokenlist="${token.name}", onchain="${onChainData.name}"`,
    );
  }

  if (token.symbol !== onChainData.symbol) {
    issues.push(
      `Symbol mismatch: tokenlist="${token.symbol}", onchain="${onChainData.symbol}"`,
    );
  }

  if (token.decimals !== onChainData.decimals) {
    issues.push(
      `Decimals mismatch: tokenlist=${token.decimals}, onchain=${onChainData.decimals}`,
    );
  }

  // Validate logoURI format
  if (
    !token.logoURI.startsWith('http') &&
    !token.logoURI.startsWith('ipfs://')
  ) {
    issues.push('Invalid logoURI: must start with http or ipfs://');
  }

  if (token.extensions && token.extensions['bridge']['1']) {
    const bridge = token.extensions['bridge']['1'];
    const l1TokenAddress = bridge['tokenAddress'];
    if (l1TokenAddress) {
      if (!validateTokenAddress(l1TokenAddress)) {
        issues.push(
          `Invalid L1 token address in bridgeInfo: ${l1TokenAddress}, should be ${getAddress(
            l1TokenAddress,
          )}`,
        );
      }
      const onChainL1Data = await getTokenInfo(mainnet, l1TokenAddress);
      if (!onChainL1Data) {
        issues.push(
          'Unable to fetch on-chain token data - contract may not exist or not be ERC20',
        );
        return result;
      }

      if (token.name !== onChainL1Data.name) {
        issues.push(
          `Name mismatch: tokenlist="${token.name}", onchain="${onChainL1Data.name}"`,
        );
      }

      if (token.symbol !== onChainL1Data.symbol) {
        issues.push(
          `Symbol mismatch: tokenlist="${token.symbol}", onchain="${onChainL1Data.symbol}"`,
        );
      }

      if (token.decimals !== onChainL1Data.decimals) {
        issues.push(
          `Decimals mismatch: tokenlist=${token.decimals}, onchain=${onChainL1Data.decimals}`,
        );
      }
    }
  }

  return result;
}

async function validateSchema() {
  const ajv = new Ajv({ allErrors: true, verbose: true });
  addFormats(ajv);
  const schemaResponse = await fetch(SCHEMA_URL);
  const schema = await schemaResponse.json();
  const validator = ajv.compile(schema);
  const data = JSON.parse(fs.readFileSync(tokenlistPath, 'utf8'));
  const valid = validator(data);
  if (valid) {
    console.log('Valid List.');
    return valid;
  }
  if (validator.errors) {
    throw validator.errors.map((error) => {
      delete error.data;
      return error;
    });
  }
}

async function main() {
  console.log('🔍 Verifying BOB tokenlist...\n');

  // Read tokenlist
  if (!fs.existsSync(tokenlistPath)) {
    console.error('❌ tokenlist.json not found');
    process.exit(1);
  }

  await validateSchema();

  const { tokens }: { tokens: Token[] } = JSON.parse(
    fs.readFileSync(tokenlistPath, 'utf8'),
  );
  console.log(`📋 Found ${tokens.length} tokens to validate\n`);

  let totalIssues = 0;
  const results: ValidationResult[] = [];

  // Validate each token
  for (let i = 0; i < tokens.length; i++) {
    const token = tokens[i];
    console.log(
      `[${i + 1}/${tokens.length}] Validating ${token.symbol} (${
        token.address
      })...`,
    );

    const result = await validateToken(token);
    results.push(result);

    if (result.issues.length > 0) {
      console.log(`  ❌ ${result.issues.length} issue(s):`);
      result.issues.forEach((issue) => console.log(`     • ${issue}`));
      totalIssues += result.issues.length;
    } else {
      console.log(`  ✅ Valid`);
    }

    // Small delay to avoid rate limiting
    if (i < tokens.length - 1) {
      await new Promise((resolve) => setTimeout(resolve, 100));
    }
  }

  // Summary
  console.log('\n📊 Validation Summary:');
  console.log(`   Total tokens: ${tokens.length}`);
  console.log(
    `   Valid tokens: ${results.filter((r) => r.issues.length === 0).length}`,
  );
  console.log(
    `   Tokens with issues: ${
      results.filter((r) => r.issues.length > 0).length
    }`,
  );
  console.log(`   Total issues: ${totalIssues}`);

  // Check for duplicate addresses
  const addresses = tokens.map((t) => t.address.toLowerCase());
  const duplicates = addresses.filter(
    (addr, index) => addresses.indexOf(addr) !== index,
  );
  if (duplicates.length > 0) {
    console.log(
      `\n⚠️  Duplicate addresses found: ${[...new Set(duplicates)].join(', ')}`,
    );
  }

  // Check for duplicate symbols
  const symbols = tokens.map((t) => t.symbol);
  const duplicateSymbols = symbols.filter(
    (symbol, index) => symbols.indexOf(symbol) !== index,
  );
  if (duplicateSymbols.length > 0) {
    console.log(
      `\n⚠️  Duplicate symbols found: ${[...new Set(duplicateSymbols)].join(
        ', ',
      )}`,
    );
  }

  if (totalIssues === 0 && duplicates.length === 0) {
    console.log('\n🎉 All tokens are valid!');
    process.exit(0);
  } else {
    console.log('\n❌ Validation failed with issues');
    process.exit(1);
  }
}

main().catch(console.error);
