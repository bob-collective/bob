---
sidebar_position: 3
sidebar_label: Pay Fees with BTC
---

# Pay Fees with BTC

## wBTC, tBTC, and other ERC-20 as Gas Token

On Ethereum rollups, gas fees need to be paid in ETH as the base currency. While a rollup can, in theory, choose to use their gas token, part of the gas fees on the rollup are used on Ethereum to pay for data availability.

In many cases, it is desirable to pay gas fees in other tokens. In particular, on BOB, we want to allow users to pay gas fees in bridged BTC to create a seamless experience for Bitcoin users.

There are two approaches to how gas fees can be paid in bridged BTC (or any other ERC-20 token like USDT or USDC) that can be implemented directly on the rollup without changing the rollup client:

- [Meta Transactions (ERC-2771)](meta-transactions): Let someone else submit a transaction on your behalf.
- [Account Abstraction (ERC-4337)](account-abstraction): Use a smart contract wallet with a paymaster to pay gas fees.

BOB supports both approaches, allowing projects to choose their preferred method.

## Meta Transactions vs. Account Abstraction

Both approaches have their pros and cons. The following table briefly summarizes the differences between the two approaches but we recommend reading the individual pages for more details:

|                        | Meta Transactions                        | Account Abstraction                                                        |
| ---------------------- | ---------------------------------------- | -------------------------------------------------------------------------- |
| Smart contract changes | Requires changes to smart contracts.     | No changes to smart contracts needed.                                      |
| UX changes             | Uses standard crypto-wallet UX approach. | New UX approach is needed to distinguish between signer and smart account. |
| Wallets compatibility  | Compatible with standard EOA wallets.    | Requires new smart contract wallets.                                       |
| Production readiness   | OpenGSN v3 is in beta.                   | New standard that is currently getting adopted.                            |
