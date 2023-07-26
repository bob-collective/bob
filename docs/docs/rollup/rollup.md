---
sidebar_position: 1
---

# Rolling Up Bitcoin

The key of Sphere is to allow both EVM compatbility, OP Stack compatibility, and Rust Bitcoin library compatibility at the same time.
On a high level, this is achieved by separating out the six parts of the OP Stack and making adjustements where necessary whiole staying backwards compatible.

The Substrate framework allows great freedom to change the implementation of a chain. By using and modifying existing pallets, Sphere achieves compatbility with EVm, OP Stack, and Bitcoin Rust libraries.

The OP Stack has six layers which Sphere implements as follows.

## Data Availability

OP Stack uses Ethereum as the DA layer.

OPEN QUESTIONS
- Should Sphere use Ethereum as well to stay compatible with the OP Stack Superchain concept or is this not a requirement? If it's not a requirement, it would be ideal to use Bitcoin as a DA layer. However, this might also be costly and having a dedicated DA layer like Celestia could be a better option.

## Sequencing

The OP Stack uses a single, centralized sequencer. 

OPEN QUESTIONS
- Should we stick with a PoA-style sequencer as well? We could use AURA for that.
- Should we keep GRANDPA and BABE consensus style block production and finalization? This will likely require some custom implmenetation on the Settlement layer.

## Execution

OP Stack uses a geth fork for its EVM state and state transition functions. Instead of using op-geth directly, Sphere's execution layer is implemented such that a state transition is applied as follows:

1. Initial state: The initial state is retrieved from an EVM and OP Stack compatible Patricia Merkle trie stored in the DA layer. The Frontier pallet in substrate already implements decoding of EVM-compatible state. Having the initial state in an EVM/OP Stack-compatible trie (i.e., encoding and decoding of the trie is possible via Ethereum defined RPCs) allows wallets, block explorers, and other EVM-compatible tooling to read from this state.
2. State transition: State transitions are applied through a Substrate-based runtime implementation. The key difference to vanilla OP Stack is that instead of relying on the EVM core runtime and EVM smart contracts, the Substrate-based runtime is implemented in WASM and houses three distinct state transition domains. A state transition is achieved by the successful execution of a state transition function on either of the three domains.
    a. Core runtime: The core runtime exposes functions that are implemented in Rust/Substrate and can be upgraded via governance. The are the equivalent of pre-compiles in the Ethereumm world. Sphere exposes several core functions that builders can make use of including a Bitcoin light client, DeFi (AMM and lending), a Bitcoin bridge, transaction fee converters, and many more. Deployment of new core runtime functions is subject to governance vote.
    b. Rust smart contracts: The core runtime has a dedicated space for builders to deploy arbitrary smart contracts written in Rust using an eDSL called ink!. Through an SDK, developers can write smart contracts within macros that can utilize the types from `rust-bitcoin`. Deploying Rust smart contracts is permissionless and deployed contracts are immutable.
    c. EVM smart contracts: The core runtime further has a dedicated space for builders to deploy EVM-compatible contracts that can be written in Solidity or other languages that compile to the EVM. Deploying EVM smart contracts is permissionless and deployed contracts are immutable.
3. Resulting state: Upon successful completion of the state transition function, the state stored in the Patricia Merkle trie is updated as indicated in the deterministic state transition function. The state is then updated and stored in the DA layer.

## Settlement

OP Stack settles on Ethereum and uses fraud proofs. While Sphere is made for Bitcoin, there are distinct reasons where Sphere will initially not roll up against Bitcoin:

1. Bitcoin's consensus only validates BTC as an asset. By rolling up against Ethereum, users can exit Ethereum-native assets (those that are validted by Ethereum consensus). This requires a bridge to Bitcoin, but having a fully collaterlaized Bitcoin bridge offers similar security levels as a *threoretically* possible roll-up to Bitcoin. 
2. Bitcoin lacks the possibility have its consensus validate a roll-up at the moment. While the Sphere project closely follows developments areound adding an `OP_ZKPVERIFY`, it remains quesitonable when such a code will be available.

In the future, Sphere can also expose capabilities to be ZK-compatible where the data structure is rather compatible with Starkware instead of the OP Stack to enable a ZKP roll-up on Bitcoin and ZKP bridges to Ethereum and other EVM networks.

## Bridge

...add details about bridge here.


