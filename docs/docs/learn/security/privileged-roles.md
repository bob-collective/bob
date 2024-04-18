---
sidebar_position: 5
---

# Privileged Roles in BOB Mainnet

BOB uses the [OP Stack](https://docs.optimism.io/stack/getting-started), just like Optimism Mainnet and Base. As a result, BOB is on the same [Pragmatic Path to Decentralization](https://medium.com/ethereum-optimism/our-pragmatic-path-to-decentralization-cb5805ca43c1).

For now, OP Stack chains still include a "privileged" roles that allow certain addresses to carry out specific actions. Read this page to understand these roles, why they exist, and what risks they pose.

## L1 Proxy Admin

The L1 Proxy Admin is an address that can be used to upgrade most BOB system contracts.

### Risks

- Compromised L1 Proxy Admin could upgrade contracts to malicious versions.
- Compromised L1 Proxy Admin could remove or lock ETH or tokens in the Standard Bridge.
- Compromised L1 Proxy Admin could fail to mitigate a risk as described on this page.

### Mitigations

- L1 Proxy Admin owner is a 4-of-6 [multisig](https://etherscan.io/address/0xC91482A96e9c2A104d9298D1980eCCf8C4dc764E#readProxyContract).

### Address

- **Ethereum**: [`0x0d9f416260598313Be6FDf6B010f2FbC34957Cd0`](https://etherscan.io/address/0x0d9f416260598313Be6FDf6B010f2FbC34957Cd0)

## L2 Proxy Admin

The L2 Proxy Admin is an address that can be used to upgrade most BOB system contracts on L2.

### Risks

- Compromised L2 Proxy Admin could upgrade contracts to malicious versions.
- Compromised L2 Proxy Admin could remove or lock ETH or tokens in the Standard Bridge.
- Compromised L2 Proxy Admin could fail to mitigate a risk as described on this page.

### Mitigations

- L2 Proxy Admin is a [hardware wallet](https://explorer.gobob.xyz/address/0xefCf0c8faFB425997870f845e26fC6cA6EE6dD5C).

### Address

- **BOB**: [`0xaCdBaAC6707c7e28ac1A15007f22Aac1188910d7`](https://explorer.gobob.xyz/address/0xaCdBaAC6707c7e28ac1A15007f22Aac1188910d7?tab=read_proxy)

## System Config Owner

The System Config Owner is an address that can be used to change the values within the [`SystemConfig`](https://github.com/ethereum-optimism/optimism/blob/62c7f3b05a70027b30054d4c8974f44000606fb7/packages/contracts-bedrock/contracts/L1/SystemConfig.sol) contract on Ethereum.

### Risks

- Compromised System Config Owner could cause a temporary network outage.
- Compromised System Config Owner could cause users to be overcharged for transactions.

### Mitigations

- System Config Owner is a 4-of-6 [multisig](https://etherscan.io/address/0xC91482A96e9c2A104d9298D1980eCCf8C4dc764E#readProxyContract).
- System Config Owner can be replaced by the [L1 Proxy Admin](#l1-proxy-admin).

### Address

- **Ethereum**: [0xaa0a1efd35d6578ea6b5704dbc2c40b36a55b590](https://etherscan.io/address/0xaa0a1efd35d6578ea6b5704dbc2c40b36a55b590#code)

## Batcher

### Description

The Batcher is a software service that submits batches of transactions to Ethereum on behalf of the current BOB Sequencer.
BOB nodes will look for transactions from this address to find new batches of L2 transactions to process.

### Risks

- Batcher address is typically a hot wallet.
- Compromised batcher address can cause L2 reorgs or sequencer outages.

### Mitigations

- Compromised batcher address cannot publish invalid transactions.
- Compromised batcher address can be replaced by the [L1 Proxy Admin](#l1-proxy-admin).

### Address

- **Ethereum**: [`0x08f9f14ff43e112b18c96f0986f28cb1878f1d11`](https://etherscan.io/address/0x08f9f14ff43e112b18c96f0986f28cb1878f1d11)

## Proposer

### Description

The Proposer is a software service that submits proposals about the state of BOB to the `L2OutputOracle` contract on Ethereum.
Proposals submitted to the `L2OutputOracle` contract can be used to execute withdrawal transactions on Ethereum after 7 days.
Proposer addresses are typically "hot wallets" as they must be available to frequently sign and publish new state proposals.

### Risks

- Proposer address is typically a hot wallet.
- Compromised proposer address could propose invalid state proposals.
- Invalid state proposals can be used to execute invalid withdrawals after 7 days.

### Mitigations

- Compromised proposer address can be replaced by the [L1 Proxy Admin](#l1-proxy-admin).
- Invalid state proposals can be challenged by the [Challenger](#challenger) within 7 days.

### Address

- **Ethereum**: [`0x7cB1022D30b9860C36b243E7B181A1d46f618C69`](https://etherscan.io/address/0x7cB1022D30b9860C36b243E7B181A1d46f618C69)

## Challenger

### Description

The Challenger is an address that can be used to challenge invalid state proposals submitted by the [Proposer](#proposer) role.

### Risks

- Compromised challenger could invalidate valid state proposals.
- Compromised challenger could fail to challenge invalid state proposals.

### Mitigations

- Compromised challenger address can be replaced by the [L1 Proxy Admin](#l1-proxy-admin).
- Challenges can be executed by replaced challenger address.
- Challenger is a 4-of-6 [multisig](https://etherscan.io/address/0xC91482A96e9c2A104d9298D1980eCCf8C4dc764E#readProxyContract).

### Address

- **Ethereum**: [`0xC91482A96e9c2A104d9298D1980eCCf8C4dc764E`](https://etherscan.io/address/0xC91482A96e9c2A104d9298D1980eCCf8C4dc764E#readProxyContract)

## Guardian

### Description

The Guardian is an address that can be used to pause withdrawals from BOB.
This is a backup safety mechanism that allows for a temporary halt in the event of a security concern.
The Guardian role cannot pause specific withdrawals and can only pause all withdrawals.

### Risks

- Compromised guardian could pause withdrawals indefinitely.

### Mitigations

- Compromised guardian address can be replaced by the [L1 Proxy Admin](#l1-proxy-admin).
- Withdrawals can be unpaused by replaced guardian address.
- Guardian is a 4-of-6 [multisig](https://etherscan.io/address/0xC91482A96e9c2A104d9298D1980eCCf8C4dc764E#readProxyContract).

### Address

- **Ethereum**: [`0xC91482A96e9c2A104d9298D1980eCCf8C4dc764E`](https://etherscan.io/address/0xC91482A96e9c2A104d9298D1980eCCf8C4dc764E#readProxyContract)

## ERC20 Contract Upgrade Proxy

### Description

The ERC20 Contract Upgrade Proxy is an address that can be used to upgrade four ERC20 contracts on BOB to new versions: USDC, tBTC, wstETH, and STONE. This is a temporary measure:

- USDC: The ERC20 is upgradable to allow Circle to take over the contract to enable native minting and redeeming on BOB.
- tBTC: The ERC20 is upgradable to allow Threshold governance to take over the contract to enable native minting and redeeming on BOB.
- wstETH: The ERC20 is upgradable to allow Lido governance to take over the contract to control the contract from the Ethereum side.
- STONE: The ERC20 is upgradable to allow the Stone team to take over the contract and replace the OP standard bridge contracts with LayerZero.

All other ERC20 contracts on BOB are not upgradable by this proxy.

### Risks

- Compromised ERC20 Contract Upgrade Proxy could upgrade contracts to malicious versions.

### Mitigations

- USDC, wstETH, STONE Contract Upgrade Proxy is a 3-of-5 [multisig](https://explorer.gobob.xyz/address/0xAE554F69fEd747006BF48481A6629921c3cD2Ba5).
- tBTC Contract Upgrade Proxy is a 3-of-5 [multisig](https://explorer.gobob.xyz/address/0x694DeC29F197c76eb13d4Cc549cE38A1e06Cd24C).