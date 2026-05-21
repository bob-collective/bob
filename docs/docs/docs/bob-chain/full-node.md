---
sidebar_position: 11
sidebar_label: Run a Full Node
---

# Run a Full Node

:::info
There is no protocol level incentive to run a BOB full node. If you're interested in accessing the BOB chain, but you don't want to set up your own node, see our [Node Providers](/docs/tools/node-providers) to get RPC access to fully-managed nodes hosted by a third-party provider.

To stay updated on node upgrades and announcements, join our [Telegram channel](https://t.me/bobupgradechannel).
:::

## Requirements

As of April 2026 we recommend you have at least the following hardware configuration to run a node:

- at least 8 GB RAM
- an SSD, preferably NVME drive with at least 100 GB free

Software stack:

- [Docker](https://docs.docker.com/engine/install/)
- [Docker Compose](https://docs.docker.com/compose/install/)

## Configuration

We provide a simple Docker Compose configuration to get you started. This guide assumes all data will be stored under `/opt/`.

### 1. Create data directories

```sh
mkdir -p /opt/op-reth /opt/op-node
```

### 2. Generate a JWT secret

The JWT secret authenticates the connection between `op-node` and `op-reth`.

```sh
openssl rand -hex 32 > /opt/op-reth/jwt.hex
```

### 3. Download the BOB genesis file

The built-in `bob` chain spec in `op-reth` does not yet include the Jovian hardfork timestamp. You must download the genesis file from Conduit and pass it to op-reth via `--chain`.

```sh
curl -o /opt/op-reth/genesis.json \
  https://api.conduit.xyz/file/v1/optimism/genesis/bob-mainnet-0
```

Verify the genesis file includes the Jovian fork time:

```sh
jq '.config.jovianTime' /opt/op-reth/genesis.json
# Expected: 1773325801
```

### 4. Create the op-node environment file

Ensure you have an Ethereum L1 full node RPC available and set `OP_NODE_L1_ETH_RPC` and `OP_NODE_L1_BEACON` to the respective RPC endpoints.

```sh title="op-node.env"
OP_NODE_L1_ETH_RPC=.....
OP_NODE_L1_BEACON=......
OP_NODE_L1_TRUST_RPC=true
OP_NODE_LOG_LEVEL=WARN
OP_NODE_P2P_BOOTNODES=enode://09acd29625beb40604b12b1c2194d6d5eb290aee03e0149675201ed717ce226c506671f46fcd440ce6f5e62dc4e059ffe88bcd931f2febcd22520ae7b9d00b5e@34.83.120.192:9222?discport=30301,enode://d25ce99435982b04d60c4b41ba256b84b888626db7bee45a9419382300fbe907359ae5ef250346785bff8d3b9d07cd3e017a27e2ee3cfda3bcbb0ba762ac9674@bootnode.conduit.xyz:0?discport=30301,enode://2d4e7e9d48f4dd4efe9342706dd1b0024681bd4c3300d021f86fc75eab7865d4e0cbec6fbc883f011cfd6a57423e7e2f6e104baad2b744c3cafaec6bc7dc92c1@34.65.43.171:0?discport=30305,enode://9d7a3efefe442351217e73b3a593bcb8efffb55b4807699972145324eab5e6b382152f8d24f6301baebbfb5ecd4127bd3faab2842c04cd432bdf50ba092f6645@34.65.109.126:0?discport=30305
OP_NODE_P2P_STATIC=/ip4/34.83.120.192/tcp/9222/p2p/16Uiu2HAkv5SVdeF4hFqJyCATwT87S3PZmutm8akrgwfcdFeqNxWw
OP_NODE_P2P_SYNC_ONLYREQTOSTATIC=true
OP_NODE_L2_ENGINE_RPC=http://localhost:9551
OP_NODE_L2_ENGINE_KIND=reth
OP_NODE_L2_ENGINE_AUTH=/reth/jwt.hex
OP_NODE_NETWORK=bob-mainnet
OP_NODE_SYNCMODE=execution-layer
OP_NODE_SYNCMODE_REQ_RESP=true
OP_NODE_OVERRIDE_JOVIAN=1773325801
OP_NODE_ROLLUP_LOAD_PROTOCOL_VERSIONS=true
OP_NODE_RPC_ENABLE_ADMIN=true
OP_NODE_SAFEDB_PATH=/data
OP_NODE_METRICS_ENABLED=true
OP_NODE_METRICS_ADDR=127.0.0.1
```

### 5. Create the Docker Compose file

```yml title="docker-compose.yml"
services:
  op-reth:
    image: us-docker.pkg.dev/oplabs-tools-artifacts/images/op-reth:v2.1.0-rc.1
    command:
      - node
      - --chain=/data/genesis.json
      - --full
      - --storage.v2
      - --datadir=/data
      - --rollup.sequencer-http=https://rpc-bob-mainnet-0.t.conduit.xyz
      - --rollup.historicalrpc=https://rpc-bob-mainnet-0.t.conduit.xyz
      - --rollup.disable-tx-pool-gossip
      - --http
      - --http.api=web3,debug,eth,txpool,net
      - --ws
      - --ws.api=web3,debug,eth,txpool,net
      - --authrpc.port=9551
      - --authrpc.jwtsecret=/data/jwt.hex
      - --metrics=127.0.0.1:9001
    volumes:
      - /opt/op-reth:/data
    network_mode: host
    restart: unless-stopped

  op-node:
    image: us-docker.pkg.dev/oplabs-tools-artifacts/images/op-node:v1.16.12
    command:
      - op-node
    env_file: op-node.env
    volumes:
      - /opt/op-node:/data
      - /opt/op-reth:/reth
    network_mode: host
    restart: unless-stopped
    depends_on:
      op-reth:
        condition: service_started
```

### 6. Start the node

```sh
docker compose up -d
```

## Verifying Sync Progress

Sync proceeds in pipeline stages. You can monitor progress with:

```sh
curl -s -X POST http://localhost:8545 \
  -H 'content-type: application/json' \
  -d '{"jsonrpc":"2.0","method":"eth_syncing","params":[],"id":1}' | jq .
```

While syncing, `eth_syncing` returns a status object with per-stage block checkpoints (Headers → Bodies → Execution). Once all stages are complete, it returns `false` and `eth_blockNumber` will reflect the live chain head.

Expected sync time from scratch is several hours depending on hardware and network.

## Rollup Configuration and Genesis

The genesis and rollup configuration files for BOB Mainnet are available from Conduit:

- [Genesis](https://api.conduit.xyz/file/v1/optimism/genesis/bob-mainnet-0)
- [Rollup Configuration](https://api.conduit.xyz/file/v1/optimism/rollup/bob-mainnet-0)
- [Contracts](https://api.conduit.xyz/file/getOptimismContractsJSON?network=036d1667-e469-424e-9db9-5b09cf4d460d&organization=610ec5c5-8b4c-444a-b2b4-a94c1835defe)

## BOB Sepolia (Testnet)

You can also run a full node for BOB Sepolia. Configuration information can be found on [Conduit's BOB Sepolia hub page](https://hub.conduit.xyz/bob-sepolia-dm6uw0yhh3).

:::info
We do not provide support for running a full node on the testnet. If you have any issues, please refer to the [Conduit's node documentation](https://docs.conduit.xyz/guides/run-a-node/op-stack-node) for more information.
:::

## Resources

Additional information on how to self-host a node for an OP Stack rollup is available on [Conduit's node documentation](https://docs.conduit.xyz/guides/run-a-node/op-stack-node). Full details on the rollup configuration are available on the [Conduit BOB Mainnet hub page](https://hub.conduit.xyz/bob-mainnet-0).

## External Links

1. [BOB Mainnet rollup configuration](https://hub.conduit.xyz/bob-mainnet-0)
1. [Conduit's node documentation](https://docs.conduit.xyz/guides/run-a-node/op-stack-node)
1. [Optimism's guide for running a node with Docker](https://docs.optimism.io/builders/node-operators/tutorials/node-from-docker)
