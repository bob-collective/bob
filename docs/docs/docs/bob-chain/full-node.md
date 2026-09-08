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

As of September 2026 we recommend you have at least the following hardware configuration to run a node:

- at least 8 GB RAM
- an SSD, preferably NVME drive with at least 100 GB free

Software stack:

- [Docker](https://docs.docker.com/engine/install/)
- [Docker Compose](https://docs.docker.com/compose/install/)

:::warning
BOB Mainnet activates the **Karst hard fork** (OP Contracts v7.0.0) at **Wed Sep 23 2026 16:00:01 UTC** (timestamp `1790179201`). The fork is consensus-breaking: every node must run `op-node` **v1.19.3** or later and a Karst-capable `op-reth` (**v2.3.0** or later), with the Karst override enabled, **before** the activation time. `op-geth` has reached end of support and cannot be used for Karst — an `op-reth` execution client is required.

The upgrade also includes EIPs that affect gas pricing and may be relevant to application and smart contract developers — see the [breaking changes in Optimism Upgrade 19](https://docs.optimism.io/notices/archive/upgrade-19#breaking-changes).
:::

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

The built-in `bob` chain spec in `op-reth` does not yet include the Karst hardfork timestamp. You must download the genesis file from Conduit and pass it to op-reth via `--chain`.

```sh
curl -o /opt/op-reth/genesis.json \
  https://api.conduit.xyz/file/v1/optimism/genesis/bob-mainnet-0
```

Verify the genesis file includes the Jovian and Karst fork times:

```sh
jq '.config | {jovianTime, karstTime}' /opt/op-reth/genesis.json
# Expected:
# {
#   "jovianTime": 1773325801,
#   "karstTime": 1790179201
# }
```

### 4. Configure a trusted execution-layer peer for op-reth

op-node v1.19.1 removed the consensus-layer request/response sync client. If your node stops or falls behind, it catches up through the execution layer: op-reth fetches the missing range from a trusted EL peer. Conduit publishes one for BOB Mainnet:

```sh
curl -sf https://api.conduit.xyz/public/network/elPeers/bob-mainnet-0
```

Write the returned enode to `/opt/op-reth/reth.toml`:

```toml title="/opt/op-reth/reth.toml"
[peers]
trusted_nodes = [
  "enode://5186f8355abafe7bd900259ffe4c913c1d5e94faa53410018aab26fae911849dc87a9604dd0d849255adae1bc554d51958a122bb6631a8954540b61c33319522@35.252.247.209:30303",
]
```

Without this peer, a node that falls behind can only re-derive the gap from L1, which is slow. The peer retains a rolling window of recent blocks — it closes gaps, it does not bootstrap a node from genesis, so start fresh nodes from a snapshot (available from Conduit on request).

### 5. Create the op-node environment file

Ensure you have an Ethereum L1 full node RPC available and set `OP_NODE_L1_ETH_RPC` and `OP_NODE_L1_BEACON` to the respective RPC endpoints.

`OP_NODE_P2P_STATIC` is the node's only consensus-layer connectivity: external nodes do not use P2P discovery, so the bootnode settings found in older guides are obsolete as of op-node v1.19.1. Fetch the current static peer from Conduit's API and update it if it changes:

```sh
curl -sf https://api.conduit.xyz/public/network/staticPeers/bob-mainnet-0
```

```sh title="op-node.env"
OP_NODE_L1_ETH_RPC=.....
OP_NODE_L1_BEACON=......
OP_NODE_L1_TRUST_RPC=true
OP_NODE_LOG_LEVEL=WARN
OP_NODE_P2P_STATIC=/ip4/35.252.247.209/tcp/9222/p2p/16Uiu2HAm1cWcrbn5RGL1tnQojsJp9z1VRDqPtJy26m26qUBr5Kx8
OP_NODE_L2_ENGINE_RPC=http://localhost:9551
OP_NODE_L2_ENGINE_KIND=reth
OP_NODE_L2_ENGINE_AUTH=/reth/jwt.hex
OP_NODE_NETWORK=bob-mainnet
OP_NODE_SYNCMODE=execution-layer
OP_NODE_OVERRIDE_JOVIAN=1773325801
OP_NODE_OVERRIDE_KARST=1790179201
OP_NODE_ROLLUP_LOAD_PROTOCOL_VERSIONS=true
OP_NODE_RPC_ENABLE_ADMIN=true
OP_NODE_SAFEDB_PATH=/data
OP_NODE_METRICS_ENABLED=true
OP_NODE_METRICS_ADDR=127.0.0.1
```

### 6. Create the Docker Compose file

```yml title="docker-compose.yml"
services:
  op-reth:
    image: us-docker.pkg.dev/oplabs-tools-artifacts/images/op-reth:v2.4.2
    command:
      - node
      - --chain=/data/genesis.json
      - --config=/data/reth.toml
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
    image: us-docker.pkg.dev/oplabs-tools-artifacts/images/op-node:v1.19.5
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

### 7. Start the node

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

Expected sync time from scratch is several hours depending on hardware and network. If you would rather not sync from genesis, Conduit can provide a recent op-reth snapshot on request.

## Rollup Configuration and Genesis

The genesis and rollup configuration files for BOB Mainnet are available from Conduit:

- [Genesis](https://api.conduit.xyz/file/v1/optimism/genesis/bob-mainnet-0)
- [Rollup Configuration](https://api.conduit.xyz/file/v1/optimism/rollup/bob-mainnet-0)
- [Contracts](https://api.conduit.xyz/file/getOptimismContractsJSON?network=036d1667-e469-424e-9db9-5b09cf4d460d&organization=610ec5c5-8b4c-444a-b2b4-a94c1835defe)
- [Static CL peer](https://api.conduit.xyz/public/network/staticPeers/bob-mainnet-0)
- [EL peer](https://api.conduit.xyz/public/network/elPeers/bob-mainnet-0)

:::warning
The published `contracts.json` is republished after the Karst activation in a new format aligned with `op-contracts/v7.0.0` and `OPCMv2`. If you rely on the currently published copy, treat its addresses as potentially stale, and re-download the file after activation — updating any hardcoded addresses and parsing logic that assumes the old structure.
:::

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
