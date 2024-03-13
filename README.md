# Build on Bitcoin

BOB is a Bitcoin-augmented rollup bringing experimentation and freedom of choice to builders to make a real-world impact. BOBs vision is to onboard the next billion users to Bitcoin.

## Learn more

- [Website](https://www.gobob.xyz/)
- [Docs](https://docs.gobob.xyz/)

## Contribution

BOB is an open-source project. We welcome contributions of all sorts. There are many ways to help, from reporting issues, contributing code, and helping us improve our community.

## Getting Started

We use foundry extensively for maintaining and testing this contract suite:

### Build

```shell
forge build

# production
FOUNDRY_PROFILE=optimized forge build
```

### Test

```shell
forge test
```

### Format

```shell
forge fmt
```

## Local Testnet

To deploy the relay contract to a local environment for testing use our convenience script [here](https://github.com/bob-collective/bob/blob/master/sdk/scripts/init-bridge.ts):

```shell
# start local ethereum testnet node
docker-compose up anvil
# run script to deploy the relay contract
cd sdk
yarn run deploy-relay --init-height 2016 --private-key dev-0 --dev --network testnet
```

The initialization height should be a multiple of 2016 with at least one subsequent retarget (i.e. if using 2016 as the starting height, blocks 4031-4032 must exist).