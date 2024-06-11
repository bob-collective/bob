---
sidebar_position: 5
sidebar_label: Run a Full Node
---

# How to run a self-hosted full node for BOB

:::info
There is no protocol level incentive to run an BOB full node. If you’re interested in accessing the BOB chain, but you don’t want to set up your own node, see our [Node Providers](../tools/node-providers) to get RPC access to fully-managed nodes hosted by a third-party provider.
:::

## Requirements
As of June 2024 we recommend you have at least the following hardware configuration to run a node:

- at least 8 GB RAM
- an SSD, preferably NVME drive with at least 100 GB free

Software stack:
- Python 3
- Docker compose


## Putting it all together

1. Clone the [Conduit Nodes](https://github.com/conduitxyz/node) Github repository
```
git clone https://github.com/conduitxyz/node.git
```

2. Create `.env` file containing all necessary environment variables. You will need to set RPC URLs to an Ethereum L1 execution client (geth, erigon) and L1 consensus client (lighthouse, nimbus, prysm)

Example:
```
CONDUIT_NETWORK=bob-mainnet-0
# Replace with your preferred L1 (Ethereum) execution node RPC URL:
OP_NODE_L1_ETH_RPC=...
# Replace with your preferred L1 (Ethereum) consensus node RPC URL:
OP_NODE_L1_BEACON=...
```

3. Download the required chain configuration

```
./download-config.py bob-mainnet-0
```

4. Start the node

```
docker compose up --build
```

:::tip
Use `docker compose logs` to retrieve the logs for op-node and op-geth
:::
