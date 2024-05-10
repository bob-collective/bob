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
npm run deploy-relay -- --init-height=latest --dev --network=mainnet --testnet
npm run update-relay -- --dev --network=mainnet --relay-address=0x<relay-address>

# sepolia
FOUNDRY_PROFILE=optimized npm run deploy-relay -- --init-height=latest --network=testnet --proof-length=20 --testnet --rpc-url=testnet --private-key=0x<exported-privatekey>

# mainnet
FOUNDRY_PROFILE=optimized npm run deploy-relay -- --init-height=latest --network=mainnet --proof-length=20 --private-key=0x<exported-privatekey>
```

The initialization height should be a multiple of 2016 with at least one subsequent retarget (i.e. if using 2016 as the starting height, blocks 4031-4032 must exist).

Proof length is the number of headers needed before and after a retarget to update the relay - tBTC-v2 uses 20 on ETH mainnet.

## Contracts

| Network     | Name           | Address                                                                                                                             |
|-------------|----------------|-------------------------------------------------------------------------------------------------------------------------------------|
| BOB Sepolia | TestLightRelay | [0x4c51bc419ead57da0d825afae3090f2f76e5892d](https://testnet-explorer.gobob.xyz/address/0x4c51bc419ead57da0d825afae3090f2f76e5892d) |
| BOB Mainnet | LightRelay     | []() |

## Security

Let's review the calculation given in the [Bitcoin Wiki](https://en.bitcoin.it/wiki/Difficulty) to compute the hashrate: 

```
hashrate = ~157 (GH/s) = (22012.4941572 * 2**32 / 600) / 10**9 (example)
hashrate = ~595 (EH/s) (83148355189239.77 * 2**32 / 600) / 10**18 (current)

time = difficulty * 2**32 / hashrate
85899.34592 seconds = 20000 * 2**32 / 10**9 (~24 hours with 1 GH/s - example)

hashrate = difficulty * 2**32 / time
595 (GH/s) = 83148355189239.77 * 2**32 / 600 / 10**18 (current)
```

The `LightRelay` requires that the proof is included at the *current* or *previous* difficulty so we can assume the attacker has 2016 * 2 blocks to brute-force a valid chain of `proofLength`.
This is possible since due to SPV assumptions we can not verify the transactions references by the block header are valid, only that sufficient PoW has accumulated on the chain.

Let's assume the attacker can generate 6 blocks (with some invalid transactions) within one difficulty adjustment period, 2016 blocks or 2016 * 60 * 10 seconds (two weeks).

```
hashrate = difficulty * 2**32 / time
83148355189239.77 * 2**32 / ((2016 * 60 * 10) / 6) = ~1.77 EH/s (~1771425 TH/s)
```

So we need ~0.297% of the current hashrate to build six blocks.

If we estimate it would cost $11 per TH/s (excluding electricity and other setup costs) then the total cost of that hashrate would be $19,485,675.
This excludes the opportunity cost from actually mining on Bitcoin mainnet, receiving fees and block rewards.

Therefore, provided the value protected by the relay is less than $20m protocols secured by the relay are "economically safe".