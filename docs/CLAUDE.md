# CLAUDE.md - Project Context for BOB Documentation

## Project Overview
This is the documentation repository for BOB (Build on Bitcoin), containing docs about BOB Chain, smart contracts, and developer resources.

## Documentation Standards

### Formatting Guidelines
- Use Docusaurus admonition syntax for notes:
  - `:::info` for informational notes
  - `:::warning` for warnings
  - `:::danger` for critical warnings
  - `:::tip` for helpful tips
- Do NOT use markdown blockquotes (`>`) for notes

### Contract Documentation
- Contract addresses should be sorted alphabetically within their sections
- Keep all contracts under a single hierarchy (no sub-sections for different deployment types)
- Include both implementation and proxy contracts when available
- Format: `- ContractName: [address](explorer-link)`

### Explorers
- BOB Mainnet Explorer: https://explorer.gobob.xyz/
- BOB Sepolia Explorer: https://bob-sepolia.explorer.gobob.xyz/
- Ethereum Mainnet: https://etherscan.io/
- Ethereum Sepolia: https://sepolia.etherscan.io/

## Directory Structure
- Main docs are in `/docs/docs/docs/`

## Testing Commands
If you need to test changes:
```bash
cd docs
yarn build  # Build the documentation
yarn serve  # Serve the built documentation
```

## Important Notes
- This is a Docusaurus-based documentation site
- Always maintain alphabetical ordering when adding new contracts
- Distinguish between L1 (Ethereum) and L2 (BOB) contracts clearly