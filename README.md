# Build on Bitcoin

BOB is a Bitcoin-augmented rollup bringing experimentation and freedom of choice to builders to make a real-world impact. BOBs vision is to onboard the next billion users to Bitcoin.

## BOB Token

- **Name:** BOB
- **Symbol:** `BOB`
- **Decimals:** `18`
- **Standard:** ERC-20 (OpenZeppelin implementation, no transfer fees or hooks)
- **Bridging:** Chainlink CCIP (lock/unlock on BOB, burn/mint on other chains)

### Deployments

| Network | Chain ID | Contract |
| --- | --- | --- |
| BOB Mainnet | `60808` (`0xED88`) | [`0xb0bd54846a92b214c04a63b26ad7dc5e19a60808`](https://explorer.gobob.xyz/address/0xb0bd54846a92b214c04a63b26ad7dc5e19a60808) |
| Ethereum Mainnet | `1` (`0x1`) | [`0xC9746F73cC33a36c2cD55b8aEFD732586946Cedd`](https://etherscan.io/address/0xC9746F73cC33a36c2cD55b8aEFD732586946Cedd) |
| BNB Smart Chain (BSC) | `56` (`0x38`) | [`0x52B5fB4B0F6572B8C44d0251Cc224513ac5eB7E7`](https://bscscan.com/address/0x52B5fB4B0F6572B8C44d0251Cc224513ac5eB7E7) |

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

To deploy the relay contract to a local environment for testing use our convenience script [here](https://github.com/bob-collective/bob/blob/master/sdk/scripts/relay-retarget.ts):

```shell
# start local ethereum testnet node
docker-compose up anvil
# run script to deploy the relay contract
cd sdk
npm run deploy-relay -- --init-height=latest --dev --network=mainnet --testnet
npm run update-relay -- --dev --network=mainnet --relay-address=0x<relay-address>

# sepolia
FOUNDRY_PROFILE=optimized npm run deploy-relay -- --network=testnet --testnet --rpc-url=testnet --init-height=latest --proof-length=20 --private-key=0x<exported-privatekey>

# mainnet
FOUNDRY_PROFILE=optimized npm run deploy-relay -- --network=mainnet --rpc-url=mainnet --init-height=latest --proof-length=20 --private-key=0x<exported-privatekey>
FOUNDRY_PROFILE=optimized forge verify-contract --verifier blockscout --verifier-url https://explorer.gobob.xyz/api 0x9fe7ef727da3d79e0308ff43f31ea1d077ee9f41 src/relay/LightRelay.sol:LightRelay
FOUNDRY_PROFILE=optimized npm run update-relay -- --network=mainnet --rpc-url=mainnet --relay-address 0x9fe7ef727da3d79e0308ff43f31ea1d077ee9f41 --private-key=0x<exported-privatekey>
```

The initialization height should be a multiple of 2016 with at least one subsequent retarget (i.e. if using 2016 as the starting height, blocks 4031-4032 must exist).

Proof length is the number of headers needed before and after a retarget to update the relay - tBTC-v2 uses 20 on ETH mainnet.

## Contracts

| Network               | Name           | Address                                                                                                                                 |
| --------------------- | -------------- | --------------------------------------------------------------------------------------------------------------------------------------- |
| BOB Sepolia (Testnet) | TestLightRelay | [0x377d6993b848013991e3d8b3028db74ec6fdf03a](https://bob-sepolia.explorer.gobob.xyz/address/0x377d6993b848013991e3d8b3028db74ec6fdf03a) |
| BOB Sepolia (Signet) | FullRelayWithVerify | [0x604d1442e9534940992435D558807e3BE813a22A](https://bob-sepolia.explorer.gobob.xyz/address/0x604d1442e9534940992435D558807e3BE813a22A)             |
| BOB Mainnet           | LightRelay     | [0x9fe7ef727da3d79e0308ff43f31ea1d077ee9f41](https://explorer.gobob.xyz/address/0x9fe7ef727da3d79e0308ff43f31ea1d077ee9f41)             |

## Security (Light Relay)

Let's review the calculation given in the [Bitcoin Wiki](https://en.bitcoin.it/wiki/Difficulty) to compute the hashrate:

```
hashrate = difficulty * 2**32 / 600 (60 * 10 = 10 minutes)
hashrate = ~157 (GH/s) = (22012.4941572 * 2**32 / 600) / 10**9 (example)
hashrate = ~743 (EH/s) = (103919634711492.2 * 2**32 / 600) / 10**18 (time of writing, Dec 2024)
```

The `LightRelay` requires that the proof is included at the _current_ or _previous_ difficulty so we can assume the attacker has 2016 \* 2 blocks to brute-force a valid chain of `proofLength`.
This is possible since, due to SPV assumptions, we can not verify the transactions referenced by the block header are valid, only that sufficient PoW has accumulated on the chain.

Let's assume the attacker can generate 6 blocks (with some invalid transactions) within two difficulty adjustment period, 2016 \* 2 blocks (four weeks).

```
hashrate * 6/(2016*2)
743 (EH/s) * 6/(2016*2) = 1.105 EH/s (~1,105,654 TH/s)
```

So we need ~0.142% of the current hashrate to build six blocks in two weeks.

If we estimate it would cost $11 per TH/s (excluding electricity and other setup costs) then the total cost of that hashrate would be $12,162,194.
This excludes the opportunity cost from actually mining on Bitcoin mainnet, receiving fees and block rewards.

Therefore, provided the value protected by the relay is less than $12m protocols secured by the relay are "economically safe".
