---
sidebar_position: 11
sidebar_label: Run a Full Node
---

# Run a Full Node



:::warning Upcoming BOB Isthmus Hardfork – May 9, 2025, 16:00:01 UTC
**What's Included in the Upgrade**
- Compatibility with Ethereum's Pectra features  
- Improved scalability and maintainability across all OP Stack chains

**Required Actions**
If you or your partners are running external nodes, please ensure the following steps are completed before the fork time:
- op-node: Update to version [v1.13.2](https://github.com/ethereum-optimism/optimism/releases/tag/op-node%2Fv1.13.2)
- op-geth: Update to version [v1.101503.4](https://github.com/ethereum-optimism/op-geth/releases/tag/v1.101503.4)
- If you **do not manage fork timestamps via --network**, add the following flag:  `--override.isthmus=1746806401`

**Granite and/or Holocene Forks**
Granite and Holocene are active on BOB mainnet.
- Set on op-geth and op-node the flags `--override.holocene=1736445601` and `--override.granite=1736272801`

**More Info**
For full details, please refer to the [Upgrade Notice](https://docs.optimism.io/notices/upgrade-15)
Feel free to reach out with any questions or concerns.
:::

:::info
There is no protocol level incentive to run a BOB full node. If you're interested in accessing the BOB chain, but you don't want to set up your own node, see our [Node Providers](/learn/reference/tools/node-providers) to get RPC access to fully-managed nodes hosted by a third-party provider.

To stay updated on node upgrades and announcements, join our [Telegram channel](https://t.me/bobnodeupgrades).
:::

## Requirements

As of August 2024 we recommend you have at least the following hardware configuration to run a node:

- at least 8 GB RAM
- an SSD, preferably NVME drive with at least 100 GB free

Software stack:

- [Docker](https://docs.docker.com/engine/install/)
- [Docker Compose](https://docs.docker.com/compose/install/)

## Configuration

We provide a simple docker-compose configuration to get you started. This guide assumes all data will be stored under `/opt/`.

1. Retrieve BOB mainnet chain configuration.

```json title="genesis.json"
wget -O /opt/genesis.json "https://api.conduit.xyz/file/getOptimismGenesisJSON?network=036d1667-e469-424e-9db9-5b09cf4d460d&organization=610ec5c5-8b4c-444a-b2b4-a94c1835defe"
```

```json title="rollup.json"
{
  "genesis": {
    "l1": {
      "hash": "0x218132178d65c4bc490aadd93c31535326043fe1fe8fea2d87f26c1da83d45c2",
      "number": 19634321
    },
    "l2": {
      "hash": "0x8ed4903b7f9c3f7bb7a09374d63ae9c9852cd9aab1784b433c41dbeb47b4dba2",
      "number": 0
    },
    "l2_time": 1712861987,
    "system_config": {
      "batcherAddr": "0x08f9f14ff43e112b18c96f0986f28cb1878f1d11",
      "overhead": "0x00000000000000000000000000000000000000000000000000000000000000bc",
      "scalar": "0x00000000000000000000000000000000000000000000000000000000000a6fe0",
      "gasLimit": 30000000
    }
  },
  "block_time": 2,
  "max_sequencer_drift": 600,
  "seq_window_size": 3600,
  "channel_timeout": 300,
  "l1_chain_id": 1,
  "l2_chain_id": 60808,
  "regolith_time": 0,
  "canyon_time": 0,
  "delta_time": 0,
  "ecotone_time": 0,
  "batch_inbox_address": "0x3a75346f81302aac0333fb5dcdd407e12a6cfa83",
  "deposit_contract_address": "0x8adee124447435fe03e3cd24df3f4cae32e65a3e",
  "l1_system_config_address": "0xacb886b75d76d1c8d9248cfddfa09b70c71c5393",
  "protocol_versions_address": "0x0000000000000000000000000000000000000000"
}
```

2. Create all the necessary configuration files in the same directory as the docker-compose.yml file.

```sh title="op-geth.env"
GETH_SNAPSHOT=false
GETH_DISCOVERY_V4=true
GETH_DATADIR=/opt/op-geth/
GETH_ROLLUP_SEQUENCERHTTP=https://rpc.gobob.xyz
GETH_ROLLUP_HISTORICALRPC=https://rpc.gobob.xyz
GETH_SYNCMODE=full
GETH_STATE_SCHEME=path
GETH_OVERRIDE_FJORD=1720627201
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
OP_NODE_ROLLUP_CONFIG=/opt/rollup.json
OP_NODE_OVERRIDE_FJORD=1720627201
OP_NODE_SYNCMODE=execution-layer
OP_NODE_ROLLUP_LOAD_PROTOCOL_VERSIONS=true
OP_NODE_RPC_ENABLE_ADMIN=true
OP_NODE_SAFEDB_PATH=/opt/op-node/
OP_NODE_METRICS_ENABLED=true
```

3. Run opgeth-init to initialise the op-geth data directory

```sh
docker run -t \
  --env-file ./op-geth.env \
  -v ./genesis.json:/opt/genesis.json:ro \
  -v ./op-geth-data:/opt/op-geth/ \
  us-docker.pkg.dev/oplabs-tools-artifacts/images/op-geth:v1.101411.1 \
  init --datadir=/opt/op-geth/ --state.scheme=path /opt/genesis.json
```

```yml title="docker-compose.yml"
services:
  opgeth:
    image: us-docker.pkg.dev/oplabs-tools-artifacts/images/op-geth:v1.101411.1
    env_file: op-geth.env
    volumes:
      - ./op-geth-data:/opt/op-geth/
    network_mode: host

  opnode:
    image: us-docker.pkg.dev/oplabs-tools-artifacts/images/op-node:v1.10.0
    env_file: op-node.env
    command:
      - op-node
    volumes:
      - ./op-node-data:/opt/op-node/
      - ./rollup.json:/opt/rollup.json:ro
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

## Resources

Additional information on how to self-host a node for an OP Stack rollup is available on [Conduit's node documentation](https://docs.conduit.xyz/guides/run-a-node/op-stack-node). Conduit extends [Optimism's guide for running a node with Docker](https://docs.optimism.io/builders/node-operators/tutorials/node-from-docker), the source of the specific hardware and software dependencies above.

## External Links

1. [Conduit's node documentation](https://docs.conduit.xyz/guides/run-a-node/op-stack-node)
1. [Optimism's guide for running a node with Docker](https://docs.optimism.io/builders/node-operators/tutorials/node-from-docker)
