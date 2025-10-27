---
sidebar_position: 11
sidebar_label: Run a Full Node
---

# Run a Full Node

:::warning Upcoming BOB Fusaka Readiness Upgrade – October 14, 2025 (Sepolia), Early December 2025 (Mainnet)
**What's Included in the Upgrade**
This is a **readiness upgrade** to make BOB protocol compatible with Ethereum's Fusaka hardfork on L1. This is NOT Fusaka adoption on L2—that will happen in a future upgrade.

**Important Dates**

- Ethereum Sepolia Fusaka hard fork: Tuesday, October 14th, 2025 07:36:00 UTC (BOB testnet already upgraded)
- Ethereum Mainnet Fusaka hard fork: Expected early December 2025

**Required Actions for Node Operators**

If you or your partners are running external nodes, please ensure the following steps are completed:

- **op-node**: Update to version [v1.14.1](https://github.com/ethereum-optimism/optimism/releases/tag/op-node%2Fv1.14.1)
- **op-geth**: Update to version [v1.101603.1](https://github.com/ethereum-optimism/op-geth/releases/tag/v1.101603.1)
- **L1 Beacon Node**: Ensure your L1 beacon node endpoint can serve all blobs and is configured with the appropriate Fusaka flags before the fork

**Required Actions for Chain Operators**

- **op-batcher**: Update to [v1.16.0](https://github.com/ethereum-optimism/optimism/releases/tag/op-batcher%2Fv1.16.0) with `OP_BATCHER_TXMGR_ENABLE_CELL_PROOFS: true` and restart just after Fusaka activates on L1

- **proxyd**: Update to [v4.19.0](https://github.com/ethereum-optimism/infra/releases/tag/proxyd%2Fv4.19.0) or greater (requires whitelisting `eth_blobBaseFee` RPC)
- **op-challenger**: Update to [v1.6.0](https://github.com/ethereum-optimism/optimism/releases/tag/op-challenger%2Fv1.6.0) if using permissionless fault proofs

**Previous Upgrades**

- **Isthmus**: Active on May 9, 2025, 16:00:01 UTC
- **Granite and Holocene**: Active on BOB mainnet

**More Info**
For full details, please refer to the [Fusaka Upgrade Notice](https://docs.optimism.io/notices/fusaka-notice)
Feel free to reach out with any questions or concerns.
:::

:::info
There is no protocol level incentive to run a BOB full node. If you're interested in accessing the BOB chain, but you don't want to set up your own node, see our [Node Providers](/docs/tools/node-providers) to get RPC access to fully-managed nodes hosted by a third-party provider.

To stay updated on node upgrades and announcements, join our [Telegram channel](https://t.me/+dmxnLC3uDwgyYWQy).
:::

## Requirements

As of May 2025 we recommend you have at least the following hardware configuration to run a node:

- at least 8 GB RAM
- an SSD, preferably NVME drive with at least 100 GB free

Software stack:

- [Docker](https://docs.docker.com/engine/install/)
- [Docker Compose](https://docs.docker.com/compose/install/)

## Configuration

We provide a simple docker-compose configuration to get you started. This guide assumes all data will be stored under `/opt/`.

1. Create all the necessary configuration files in the same directory as the docker-compose.yml file.

```sh title="op-geth.env"
GETH_SNAPSHOT=false
GETH_DISCOVERY_V4=true
GETH_DATADIR=/opt/op-geth/
GETH_ROLLUP_SEQUENCERHTTP=https://rpc.gobob.xyz
GETH_ROLLUP_HISTORICALRPC=https://rpc.gobob.xyz
GETH_SYNCMODE=full
GETH_STATE_SCHEME=path
GETH_OP_NETWORK=bob-mainnet
GETH_DB_ENGINE=pebble
GETH_ROLLUP_DISABLETXPOOLGOSSIP=true
GETH_HTTP=true
GETH_HTTP_API=web3,debug,eth,txpool,net,engine
GETH_WS=true
GETH_WS_API=web3,debug,eth,txpool,net,engine
GETH_METRICS=true
GETH_AUTHRPC_JWTSECRET=/opt/op-geth/geth/jwtsecret.hex
```

Ensure you have an Ethereum L1 full node RPC available and set `OP_NODE_L1_ETH_RPC` & `OP_NODE_L1_BEACON` to the respective RPC endpoints.

```sh title="op-node.env"
OP_NODE_L1_ETH_RPC=.....
OP_NODE_L1_BEACON=......
OP_NODE_L1_RPC_KIND=standard
OP_NODE_L1_TRUST_RPC=true
OP_NODE_LOG_LEVEL=INFO
OP_NODE_P2P_BOOTNODES=enode://09acd29625beb40604b12b1c2194d6d5eb290aee03e0149675201ed717ce226c506671f46fcd440ce6f5e62dc4e059ffe88bcd931f2febcd22520ae7b9d00b5e@34.83.120.192:9222?discport=30301,enode://d25ce99435982b04d60c4b41ba256b84b888626db7bee45a9419382300fbe907359ae5ef250346785bff8d3b9d07cd3e017a27e2ee3cfda3bcbb0ba762ac9674@bootnode.conduit.xyz:0?discport=30301,enode://2d4e7e9d48f4dd4efe9342706dd1b0024681bd4c3300d021f86fc75eab7865d4e0cbec6fbc883f011cfd6a57423e7e2f6e104baad2b744c3cafaec6bc7dc92c1@34.65.43.171:0?discport=30305,enode://9d7a3efefe442351217e73b3a593bcb8efffb55b4807699972145324eab5e6b382152f8d24f6301baebbfb5ecd4127bd3faab2842c04cd432bdf50ba092f6645@34.65.109.126:0?discport=30305
OP_NODE_P2P_STATIC=/ip4/34.83.120.192/tcp/9222/p2p/16Uiu2HAkv5SVdeF4hFqJyCATwT87S3PZmutm8akrgwfcdFeqNxWw
OP_NODE_P2P_SYNC_ONLYREQTOSTATIC=true
OP_NODE_L2_ENGINE_RPC=http://localhost:8551
OP_NODE_L2_ENGINE_KIND=geth
OP_NODE_L2_ENGINE_AUTH=/opt/op-geth/geth/jwtsecret.hex
OP_NODE_NETWORK=bob-mainnet
OP_NODE_SYNCMODE=execution-layer
OP_NODE_ROLLUP_LOAD_PROTOCOL_VERSIONS=true
OP_NODE_RPC_ENABLE_ADMIN=true
OP_NODE_SAFEDB_PATH=/opt/op-node/
OP_NODE_METRICS_ENABLED=true
```

```yml title="docker-compose.yml"
services:
  opgeth:
    image: us-docker.pkg.dev/oplabs-tools-artifacts/images/op-geth:v1.101603.1
    env_file: op-geth.env
    volumes:
      - ./op-geth-data:/opt/op-geth/
    network_mode: host

  opnode:
    image: us-docker.pkg.dev/oplabs-tools-artifacts/images/op-node:v1.14.1
    env_file: op-node.env
    command:
      - op-node
    volumes:
      - ./op-node-data:/opt/op-node/
      - ./op-geth-data/geth:/opt/op-geth/geth:ro
    network_mode: host
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:7300"]
      interval: 30s
      timeout: 10s
      retries: 5
    depends_on:
      opgeth:
        condition: service_started
```

You can finally start the node with `docker-compose up`.

## Rollup Configuration and Genesis

If you want to download the rollup configuration and genesis file, you can access them on the [Conduit's BOB Mainnet hub page](https://app.conduit.xyz/view-network/bob-mainnet-0/overview).

- [Rollup Configuration](https://api.conduit.xyz/file/getOptimismRollupJSON?network=036d1667-e469-424e-9db9-5b09cf4d460d&organization=610ec5c5-8b4c-444a-b2b4-a94c1835defe)
- [Genesis](https://api.conduit.xyz/file/getOptimismGenesisJSON?network=036d1667-e469-424e-9db9-5b09cf4d460d&organization=610ec5c5-8b4c-444a-b2b4-a94c1835defe)
- [Contracts](https://api.conduit.xyz/file/getOptimismContractsJSON?network=036d1667-e469-424e-9db9-5b09cf4d460d&organization=610ec5c5-8b4c-444a-b2b4-a94c1835defe)

## BOB Sepolia (Testnet)

You can also run a full for BOB Sepolia. Configuration information can be found on [Conduit's BOB Sepolia hub page](https://hub.conduit.xyz/bob-sepolia-dm6uw0yhh3).

:::info
We do not provide support for running a full node on the testnet. If you have any issues, please refer to the [Conduit's node documentation](https://docs.conduit.xyz/guides/run-a-node/op-stack-node) for more information.
:::

## Resources

Additional information on how to self-host a node for an OP Stack rollup is available on [Conduit's node documentation](https://docs.conduit.xyz/guides/run-a-node/op-stack-node). Conduit extends [Optimism's guide for running a node with Docker](https://docs.optimism.io/builders/node-operators/tutorials/node-from-docker), the source of the specific hardware and software dependencies above. Full details on the rollup configuration are available in the [Conduit's BOB Mainnet hub page](https://hub.conduit.xyz/bob-mainnet-0).

## External Links

1. [BOB Mainnet rollup configuration](https://hub.conduit.xyz/bob-mainnet-0)
1. [Conduit's node documentation](https://docs.conduit.xyz/guides/run-a-node/op-stack-node)
1. [Optimism's guide for running a node with Docker](https://docs.optimism.io/builders/node-operators/tutorials/node-from-docker)
