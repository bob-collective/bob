# Data Indexers

## Goldsky

[Goldsky](https://goldsky.com/) is a data indexer for web3 builders, offering high-performance subgraph hosting and realtime data replication pipelines.

Their [subgraphs](https://goldsky.com/products/subgraphs) are fully spec-compliant with every subgraph on The Graph’s hosted and decentralized networks. Smart APIs automatically create a subgraph from any smart contract ABI, so you don’t have to.

[Mirror](https://goldsky.com/products/mirror) streams live crypto data to your backend database, giving you more control & flexibility than traditional APIs.

**Supported Networks**

- BOB Mainnet
- BOB Sepolia (Testnet)

## SQD

[SQD](https://sqd.dev/) is a decentralized hyper-scalable data platform optimized for providing efficient, permissionless access to large volumes of data. It currently serves historical on-chain data, including event logs, transaction receipts, traces, and per-transaction state diffs. SQD offers a powerful toolkit for creating custom data extraction and processing pipelines, achieving an indexing speed of up to 150k blocks per second.

To get started, visit the [documentation](https://docs.sqd.dev/) or see [EVM examples](https://github.com/subsquid-labs/squid-evm-examples) of what you can build with SQD.

**Supported Networks**

- BOB Mainnet

## Sim IDX

[Sim IDX](https://sim.dune.com/) is a blockchain indexing framework centered around a Solidity listener contract that runs in an instrumented EVM. Developers define their data requirements using Solidity in a listener contract, specifying triggers for on-chain events or function calls. The framework then handles the entire indexing pipeline, from efficient historical backfilling to real-time data ingestion. The resulting data is stored in a dedicated PostgreSQL instance, with a schema derived directly from the events defined in the listener contract.

The Sim IDX framework provides a complete development environment, including a CLI for project management, a managed database, and a serverless API layer using Cloudflare Workers, Hono, and Drizzle.

To get started, visit the [developer docs](https://docs.sim.dune.com/idx).

**Supported Networks**

- BOB Mainnet
- Ethereum
- Base
- Unichain
- World
- Zora
- Ink
- Soneium
- Mode
- Sepolia
- Base Sepolia