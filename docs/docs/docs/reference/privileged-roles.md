---
sidebar_position: 5
---

# Privileged Roles in BOB Mainnet

BOB uses the [OP Stack](https://docs.optimism.io/stack/getting-started) as its foundation and has upgraded to a [hybrid zk rollup](/docs/bob-chain/hybrid-chain) powered by Kailua, which enables validity proofs for dispute resolution and on-demand fast withdrawals.

OP Stack chains still include "privileged" roles that allow certain addresses to carry out specific actions. Read this page to understand these roles, why they exist, and what risks they pose.

For independent reviews of BOB's security and decentralization status, see:
- [L2Beat](https://l2beat.com/scaling/projects/bob) - Comprehensive analysis of L2 scaling solutions
- [Bitcoin Layers](https://www.bitcoinlayers.org/layers/bob) - Bitcoin-focused layer analysis

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

- L2 Proxy Admin is a 4-of-6 [multisig](https://explorer.gobob.xyz/address/0x432c1fe0a868c8eeec2c73f59743f88fb07b561b).

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

The Proposer is a software service that submits proposals about the state of BOB to the `DisputeGameFactory` contract on Ethereum, which spawns a new `KailuaGame` contract for each proposal in which disputes can be resolved if necessary. BOB operates in [Kailua's vanguard mode](https://boundless-xyz.github.io/kailua/parameters.html?highlight=vanguard#vanguard-advantage), where the BOB proposer has priority to submit proposals. If no proposal is made by the BOB proposer within 30 days, then any user can submit their own proposal.

Proposals can be finalized in multiple ways:

- After 4 days if there is no challenge
- Instantly when a challenge is resolved through a validity proof
- Instantly when a proposal is submitted with a validity proof

Proposer addresses are typically "hot wallets" as they must be available to frequently sign and publish new state proposals.

### Risks

- Proposer address is typically a hot wallet.
- Compromised proposer address could propose invalid state proposals.
- Invalid state proposals can be used to execute invalid withdrawals if not challenged.

### Mitigations

- Compromised proposer address can be replaced by the [L1 Proxy Admin](#l1-proxy-admin).
- Invalid state proposals can be challenged by anyone with 0.5 ETH collateral.
- Validity proofs provide mathematical certainty during disputes.

### Address

- **Ethereum**: [`0x7cB1022D30b9860C36b243E7B181A1d46f618C69`](https://etherscan.io/address/0x7cB1022D30b9860C36b243E7B181A1d46f618C69)

## Challenger

### Description

In BOB's Kailua-powered system, anyone can challenge invalid state proposals submitted by the [Proposer](#proposer) role. Challenges require a collateral deposit of 0.5 ETH. When a challenge is initiated, the dispute is resolved through validity proofs that provide mathematical certainty about the correctness of the state transition.

Successful challengers are rewarded, while unsuccessful challengers forfeit their collateral. This permissionless challenging mechanism ensures the security of the hybrid zk rollup system.

### Risks

- Economic barrier (0.5 ETH) may limit the number of potential challengers.
- If no one challenges invalid proposals within the challenge period, invalid withdrawals could be executed.

### Mitigations

- Low collateral requirement (0.5 ETH) makes challenging accessible to many participants.
- Validity proofs provide cryptographic certainty in dispute resolution.
- Anyone can participate in challenging, not limited to specific addresses.
- Economic incentives reward successful challengers.

### Address

- **Anyone** can challenge by depositing 0.5 ETH collateral

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

All other ERC20 contracts on BOB are not upgradable by this proxy.

### Risks

- Compromised ERC20 Contract Upgrade Proxy could upgrade contracts to malicious versions.

### Mitigations

- USDC and wstETH Contract Upgrade Proxy is a 3-of-5 [multisig](https://explorer.gobob.xyz/address/0xAE554F69fEd747006BF48481A6629921c3cD2Ba5).
- tBTC Contract Upgrade Proxy is a 3-of-5 [multisig](https://explorer.gobob.xyz/address/0x694DeC29F197c76eb13d4Cc549cE38A1e06Cd24C).

## BOB Token Admin Multisig

### Description

The BOB token contracts on BOB, Ethereum, and BNB Smart Chain share a single admin multisig at `0x71083c7F1dCa054E3D96c833Eb38500aA6881Db9`. Today it operates as a 2-of-4 multisig with an upgrade plan to move to a 3-of-4 threshold.

This multisig owns each BOB token proxy contract and can:

- Upgrade implementation logic
- Mint or burn supply
- Configure Chainlink CCIP bridge parameters (adding chains, adjusting rate limits, pausing flows)

### Risks

- A compromised multisig could mint unlimited BOB, impose malicious upgrades, or block all transfers/bridging by altering CCIP parameters.

### Mitigations

- Multisig threshold (moving to 3-of-4) distributes key custody across independent signers.
- CCIP rate-limit changes and new chain listings follow internal change-management reviews before execution.

### Address

- **BOB / Ethereum / BSC:** [`0x71083c7F1dCa054E3D96c833Eb38500aA6881Db9`](https://explorer.gobob.xyz/address/0x71083c7F1dCa054E3D96c833Eb38500aA6881Db9)
