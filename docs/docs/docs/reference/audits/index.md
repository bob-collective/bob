---
sidebar_position: 3
sidebar_label: Audits
---

# Audits

## BOB Mainnet

BOB is based on the OP Stack and relies on the extensive [security reviews](https://github.com/ethereum-optimism/optimism/tree/v1.1.4/technical-documents/security-reviews) conducted for the OP Stack infrastructure by the Optimism Collective.

Our dispute game differs from the standard OP Stack implementation by using [Kailua](https://github.com/risc0/kailua), a hybrid zk rollup framework that enables validity proofs for dispute resolution and on-demand fast withdrawals. The Kailua framework has undergone multiple security audits:

### Veridise

- February 2025: Kailua Security Audit. [Report](veridise-kailua-20250217.pdf)
- May 2025: Kailua Security Audit. [Report](veridise-kailua-20250522.pdf)
- June 2025: Kailua Security Audit. [Report](veridise-kailua-20250616.pdf)

## BOB Gateway

BOB's most novel product is our intent-based Bitcoin bridge, called "[BOB Gateway](/docs/user-hub/onboard-to-bob/bob-gateway/)." The reports below show the results of audits for every major release so far.

### Cure53

- April 2024: BOB Onramp Smart Contract Audit. [Report](BOB-02-WP2-report.pdf)

### Common Prefix

- April 2024: BOB Onramp Smart Contract Audit. [Report](Common-Prefix-Audit-Report-2024.pdf)

### Pashov

- April 2024: BOB Onramp Smart Contract Security Review. [Report](Pashov-Audit-Report-2024-v1.pdf)
- August 2024: BOB Gateway V2 Smart Contract Security Review. [Report](Pashov-Audit-Report-2024-v2.pdf)
- September 2024: BOB Gateway V3 Smart Contract Security Review. [Report](Pashov-Audit-Report-2024-v3.pdf)

## USDC Bridge

We contracted several auditing firms to evaluate BOB's implementation of the USDC bridge from Ethereum mainnet to BOB.

### Cure53

- April 2024: BOB Modified USDC Bridge Library. [Report](BOB-02-WP1-report.pdf)

### Pashov

- April 2024: BOB USDC Bridge Security Review. [Report](BOB-USDCBridge-security-review.pdf)

## FusionLock Contract

BOB's "Fusion Season One" campaign preceded our mainnet launch. Users had the option of depositing their tokens into a `FusionLock.sol` smart contract with the intention of bridging those assets to BOB mainnet once it went live. In preparation for that campaign, we contracted reviews of the FusionLock contract from several auditing firms.

### Ottersec

- March 2024: FusionLock Smart Contract Audit. [Report](FusionLock-Ottersec.pdf)

### Common Prefix

- March 2024: FusionLock Smart Contract Audit. [Report](FusionLock-Common_Prefix.pdf)

### Trail of Bits

- April 2024: FusionLock Smart Contract Security Review. [Report](FusionLock-Trail_of_Bits.pdf)
