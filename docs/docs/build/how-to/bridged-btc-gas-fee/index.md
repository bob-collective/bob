# Using bridged BTC as Transaction Fees

There are two standard approaches to how the gas fee can be paid in bridged BTC (ERC-20 token):

- [Meta Transactions (ERC-2771)](meta-transactions)
- [Account Abstraction (ERC-4337)](account-abstraction)

## Meta Transactions vs. Account Abstraction

|                        | Meta Transactions                     | Account Abstraction                                                     |
| ---------------------- | ------------------------------------- | ----------------------------------------------------------------------- |
| Smart contract changes | Requires changes to smart contracts.  | No changes to smart contracts needed.                                   |
| UX changes             | Uses standard UX approach.            | New UX approach is needed to distinguish between signer and smart account. |
| Wallets compatibility  | Compatible with standard EOA wallets. | Requires new smart contract wallets.                                    |
| Production readiness               | OpenGSN v3 is in beta.                 | New standard that is currently getting adopted.                             |
