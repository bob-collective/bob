---
sidebar_position: 6
title: BOB Token Reference
---

This page aggregates the information most partners need to integrate the BOB token across supported networks. The on-chain contracts themselves remain the canonical source of truth; always verify against the contract addresses linked below before deploying changes.

## Summary

- **Name:** BOB
- **Symbol:** `BOB`
- **Decimals:** `18`
- **Standard:** ERC-20 (OpenZeppelin implementation, no transfer fees or hooks)
- **Bridging:** Chainlink CCIP (lock/unlock on BOB, burn/mint on other chains)
- **Use cases:** Protocol reward token, liquidity incentives, gas subsidies, ecosystem alignment

## Deployments

| Network | Chain ID | Contract | Explorer |
| --- | --- | --- | --- |
| BOB Mainnet | `60808` (`0xED88`) | `0xb0bd54846a92b214c04a63b26ad7dc5e19a60808` | [explorer.gobob.xyz/address/0xb0bd...0808](https://explorer.gobob.xyz/address/0xb0bd54846a92b214c04a63b26ad7dc5e19a60808) |
| Ethereum Mainnet | `1` (`0x1`) | `0xC9746F73cC33a36c2cD55b8aEFD732586946Cedd` | [etherscan.io/address/0xC974...Cedd](https://etherscan.io/address/0xC9746F73cC33a36c2cD55b8aEFD732586946Cedd) |
| BNB Smart Chain | `56` (`0x38`) | `0x52B5fB4B0F6572B8C44d0251Cc224513ac5eB7E7` | [bscscan.com/address/0x52B5...B7E7](https://bscscan.com/address/0x52B5fB4B0F6572B8C44d0251Cc224513ac5eB7E7) |

All contracts are verified on their respective explorers. Always reference the full address (not a token symbol) when configuring integrations.

## Metadata & ABI

- `name() -> "BOB"`
- `symbol() -> "BOB"`
- `decimals() -> 18`
- `totalSupply()` returns 18-decimal units. Supply can change based on governance-controlled issuance.
- Standard ERC-20 events: `Transfer(address indexed from, address indexed to, uint256 value)` and `Approval(address indexed owner, address indexed spender, uint256 value)`.
- No custom callbacks are fired during transfers, so DeFi protocols can treat BOB like a vanilla ERC-20.
- ABI parity across every chain—only the deployment address and chain ID differ.

## Bridging

| Route | Purpose | Notes |
| --- | --- | --- |
| [CCIP Transporter](https://app.transporter.io/?token=BOB) | Canonical bridge for BOB between Ethereum ↔ BOB ↔ BSC and other CCIP-enabled domains | Uses Chainlink CCIP token pools. Provides attested, rate-limited transfers suitable for protocol treasury moves or end users. |

### CCIP Token Flow

- **BOB Mainnet:** CCIP locks/unlocks the existing L2 token when bridging in or out. Total BOB supply on BOB reflects supply across all chains.
- **Ethereum & BNB Smart Chain:** CCIP burns the source token and mints on the destination chain. On Etheruem and BSC, the total token supply is only the supply on that particular chain.

You can audit pool parameters, supply, and admin roles in the [Chainlink Token Manager dashboard](https://tokenmanager.chain.link/dashboard/ethereum-mainnet,0xc9746f73cc33a36c2cd55b8aefd732586946cedd).  

### Integration Guidance

- Prefer CCIP for automated bridging flows, especially when you require deterministic settlement messages or per-transfer metadata.
- When monitoring bridges, watch for CCIP `MessageSent` / `MessageExecuted` events and the ERC-20 `Transfer` events emitted by the destination token.
- There is no secondary BOB-run liquidity bridge for BOB; CHAINLINK CCIP is the sole supported path for token portability today.

## Wallet & UI Integration Checklist

1. **Token metadata:** Symbol `BOB`, decimals `18`
2. **Add-to-wallet:** You can embed the `<AddBobToken />` component from `@site/src/components/AddBobToken` to let users add the token across supported chains.
3. **Logos:** Use the official SVG from the [brand assets](https://www.notion.so/build-on-bitcoin/Brand-Assets-2163a8aad3b48072afe8e134fc04e720/).
4. **Allowances:** Follow standard ERC-20 allowance flow.

## Security & Audits

- Smart contracts have been reviewed and [audit reports](/docs/reference/audits/) are available.
- Monitor [privileged roles](/docs/reference/privileged-roles) for information about controllers that can mint or pause bridge contracts.
