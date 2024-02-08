# Steps to deploy all required contracts

The steps below use various variables. In the latest deployments, these were set to:

```sh
export PRIVATE_KEY=<private-key-of-0x09Af4E864b84706fbCFE8679BF696e8c0B472201>
export ENTRYPOINT_ADDRESS=0x8b57d6ec08e09078Db50F265729440713E024C6a
export RELAY_ADDRESS=0x7B72bA8c9f3Ba4A94E6d8fA07c822228034d2e61
export ERC_ADDRESS=0x2868d708e442A6a940670d26100036d426F1e16b # wbtc
export ORACLE_ADDRESS=0x9AfBdFF0434acD4F325e3c35b739a62365099BCE # wbtc oracle
export ERC_DECIMALS=8
export FORWARDER_ADDRESS=0xFd0042D3d05c82acb937aC86F23247a2D77785f2
export RPC_URL=https://l2-puff-bob-jznbxtoq7h.t.conduit.xyz
export VERIFIER_URL='https://explorerl2new-puff-bob-jznbxtoq7h.t.conduit.xyz/api?'
export OWNER_ADDRESS='0x09Af4E864b84706fbCFE8679BF696e8c0B472201'
export BITCOIN_PRICE=3761500000000
```

## GSN standard infrastructure

Use the `gsn` tool to deploy the standard gsn infrasturcture. I believe the beta.10 version was used for the most recent deployment.

```sh
npx gsn deploy --network https://l2-puff-bob-jznbxtoq7h.t.conduit.xyz --privateKeyHex $PRIVATE_KEY --testToken --burnAddress 0x09Af4E864b84706fbCFE8679BF696e8c0B472201 --devAddress 0x09Af4E864b84706fbCFE8679BF696e8c0B472201
# Output:
#
# info:    Setting minimum stake of 1 TestWeth on Hub
# info:    Setting minimum stake of 0.000000000000000001 wnTok (0x14d8...446)
# Deployed GSN to network: https://l2-puff-bob-jznbxtoq7h.t.conduit.xyz
# 
#   RelayHub: 0x7B72bA8c9f3Ba4A94E6d8fA07c822228034d2e61
#   RelayRegistrar: 0x6Ff484e7530C4ab20aEa1B19E5b33FE7415dB9Fd
#   StakeManager: 0xE5a27E68bE43A69dfd3A26be7DaE9Feac236C826
#   Penalizer: 0x1C36129916E3EA2ACcD516Ae92C8f91deF7c4146
#   Forwarder: 0xFd0042D3d05c82acb937aC86F23247a2D77785f2
#   TestToken (test only): 0x14d8b98c9f685FB3e13F5BB24B8016BD709A5446
#   Paymaster (Default): 0x0000000000000000000000000000000000000000
```

## Miscelaneous deployments

### Tokens

```sh
# wbtc oracle: 0x9AfBdFF0434acD4F325e3c35b739a62365099BCE
# wbtc: 0x2868d708e442A6a940670d26100036d426F1e16b

export ERC_PRICE=$BITCOIN_PRICE && forge script script/TestingWbtc.sol --rpc-url=$RPC_URL --broadcast --verify --verifier=blockscout --verifier-url=$VERIFIER_URL
```

### Marketplace
```sh
# btc marketplace: 0x0cfd830a59e94b6957609fFd85CcDD742C521F34
# marketplace: 0x69F14d077Fcc88e70F4737a48fE09C0FD32506FB
# dummy relay: 0x077c5ed60fABb260784891786c6573373fDa8A3E
forge script script/Marketplace.sol --rpc-url=$RPC_URL --broadcast --verify --verifier blockscout --verifier-url=$VERIFIER_URL
```

### GSN Paymaster

```sh
# paymaster: 0x25Aa86d188E37A47dd2011535534E53Cf994559d
forge script script/OracleTokenPaymaster.sol --rpc-url=$RPC_URL --broadcast --verify --verifier=blockscout --verifier-url=$VERIFIER_URL
```

# ERC-4337

## Entrypoint

Clone [our fork](https://github.com/bob-collective/account-abstraction/tree/sepolia-bob)  of the account-abstraction repo, set the `PRIVATE_KEY` environment variable, and deploy:

```sh
yarn hardhat deploy --network sepoliaBob
```

The command above will output an address. Specify that address to verify the contract:
```sh
yarn hardhat verify --network sepoliaBob 0x8b57d6ec08e09078Db50F265729440713E024C6a
```


## Deterministic deployer

Clone [our fork](https://github.com/bob-collective/deterministic-deployment-proxy) of deterministic-deployment-proxy. This contract is used by the account abstraction wallets to deploy wallets to deterministic addresses. Edit the `chainIdNum` in the `scripts/compile.ts` file and then run `ts-node scripts/compile.ts`. Check the output as follows:


```sh
❯ cat output/deployment.json
{
	"gasPrice": 100000000000,
	"gasLimit": 100000,
	"signerAddress": "3760847f009a294e07309e80514ac0a7ee194269",
	"transaction": "f8a78085174876e800830186a08080b853604580600e600039806000f350fe7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe03601600081602082378035828234f58015156039578182fd5b8082525050506014600cf3820101a02222222222222222222222222222222222222222222222222222222222222222a02222222222222222222222222222222222222222222222222222222222222222",
	"address": "250f60877f1397002ae79528b218be925b6b4c79"
}
```

Then, in the front-end code, update `DeterministicDeployer.ts` using the values from above. See https://github.com/bob-collective/demo-account-abstraction-transfer/pull/4 for an example.



# Dummy oracle: 0x31b36BB047f6D5e3B49E95c4c99Cce4591e82E3f
# Dummy oracle: 0x6669d0C53fCf30c00F5AbE5a32cFa2EaD2bc2d5a
# Paymaster: 0x777FA19ea9e771018678161ABf2f1E2879D3cA6C
# forge script script/AATokenPaymaster.sol --rpc-url=$RPC_URL --broadcast --verify --verifier blockscout --verifier-url=$VERIFIER_URL