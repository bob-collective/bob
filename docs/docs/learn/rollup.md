---
sidebar_position: 2
sidebar_label: BOP
---

# Bitcoin Optimistic Rollup

The key of BOP is to unify the compatibility of the EVM, OP Stack, and Bitcoin rust libraries. On a high level, this is achieved by separating out the six parts of the OP Stack and making adjustments where necessary while staying backward compatible.

The Substrate framework allows great freedom to change the implementation of a chain. By using and modifying existing pallets, BOP achieves compatibility with EVM, OP Stack, and Bitcoin Rust libraries.

BOP has the following stack:

1. Governance: on-chain governance is used to upgrade and change the BOP stack.
2. Settlement layer: the settlement layer is the L1 from which BOP receives its finality and consensus security. BOPa uses Ethereum for settlement with the long-term vision to also rollup against Bitcoin once ZKPVERIFY or similar OP codes are available.
3. Execution layer: The execution layer is implemented in substrate and exposes three domains for state transition functions:
    1. Core runtime: implemented in Rust and substrate as a governance-controlled upgradeable runtime that houses a BTC light client, DeFi functions, governance, and more.
    2. Rust smart contracts: implemented in Rust and the ink eDSL as a way for Bitcoin smart contract engineers to leverage the power of `rust-bitcoin` and other libraries to implement immutable smart contracts interacting with the core runtime and the EVM.
    3. EVM smart contracts: implemented through the frontier pallet, a runtime to execute EVM smart contracts that can interact with the core runtime and the Rust smart contracts to support EVM wallets and tooling (Safe, Fireblocks, Etherscan, …)
4. Derivation layer:
5. Sequencing layer:
6. Data availability layer:
7. Bridge layer:
8. Multi-chain connector layer:
9. DeFi layer:
10. Programmability layer:

## Data Availability

OP Stack uses Ethereum as the DA layer.

OPEN QUESTIONS
- Should BOP use Ethereum as well to stay compatible with the OP Stack Superchain concept or is this not a requirement? If it's not a requirement, it would be ideal to use Bitcoin as a DA layer. However, this might also be costly and having a dedicated DA layer like Celestia could be a better option.

## Sequencing

The OP Stack uses a single, centralized sequencer. 

OPEN QUESTIONS
- Should we stick with a PoA-style sequencer as well? We could use AURA for that.
- Should we keep GRANDPA and BABE consensus style block production and finalization? This will likely require some custom implmenetation on the Settlement layer.

## Execution

OP Stack uses a geth fork for its EVM state and state transition functions. Instead of using op-geth directly, BOP's execution layer is implemented such that a state transition is applied as follows:

1. Initial state: The initial state is retrieved from an EVM and OP Stack compatible Patricia Merkle trie stored in the DA layer. The Frontier pallet in substrate already implements decoding of EVM-compatible state. Having the initial state in an EVM/OP Stack-compatible trie (i.e., encoding and decoding of the trie is possible via Ethereum defined RPCs) allows wallets, block explorers, and other EVM-compatible tooling to read from this state.
2. State transition: State transitions are applied through a Substrate-based runtime implementation. The key difference to vanilla OP Stack is that instead of relying on the EVM core runtime and EVM smart contracts, the Substrate-based runtime is implemented in WASM and houses three distinct state transition domains. A state transition is achieved by the successful execution of a state transition function on either of the three domains.
    a. Core runtime: The core runtime exposes functions that are implemented in Rust/Substrate and can be upgraded via governance. The are the equivalent of pre-compiles in the Ethereumm world. BOP exposes several core functions that builders can make use of including a Bitcoin light client, DeFi (AMM and lending), a Bitcoin bridge, transaction fee converters, and many more. Deployment of new core runtime functions is subject to governance vote.
    b. Rust smart contracts: The core runtime has a dedicated space for builders to deploy arbitrary smart contracts written in Rust using an eDSL called ink!. Through an SDK, developers can write smart contracts within macros that can utilize the types from `rust-bitcoin`. Deploying Rust smart contracts is permissionless and deployed contracts are immutable.
    c. EVM smart contracts: The core runtime further has a dedicated space for builders to deploy EVM-compatible contracts that can be written in Solidity or other languages that compile to the EVM. Deploying EVM smart contracts is permissionless and deployed contracts are immutable.
3. Resulting state: Upon successful completion of the state transition function, the state stored in the Patricia Merkle trie is updated as indicated in the deterministic state transition function. The state is then updated and stored in the DA layer.

## Settlement

OP Stack settles on Ethereum and uses fraud proofs. While BOP is made for Bitcoin, there are distinct reasons where BOP will initially not roll up against Bitcoin:

1. Bitcoin's consensus only validates BTC as an asset. By rolling up against Ethereum, users can exit Ethereum-native assets (those that are validted by Ethereum consensus). This requires a bridge to Bitcoin, but having a fully collaterlaized Bitcoin bridge offers similar security levels as a *threoretically* possible roll-up to Bitcoin. 
2. Bitcoin lacks the possibility have its consensus validate a roll-up at the moment. While the BOP project closely follows developments areound adding an `OP_ZKPVERIFY`, it remains quesitonable when such a code will be available.

In the future, BOP can also expose capabilities to be ZK-compatible where the data structure is rather compatible with Starkware instead of the OP Stack to enable a ZKP roll-up on Bitcoin and ZKP bridges to Ethereum and other EVM networks.

## Bridge

...add details about bridge here.


