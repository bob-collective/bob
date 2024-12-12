# Oracles

## Acurast

[Acurast](https://acurast.com/) is a zero-trust application platform with universal interoperability.

Acurast provides [Chainlink-compatible Price Feeds](https://docs.acurast.com/integrations/evm/#chainlink-compatible-price-feeds) on BOB. See their guide on how to integrate with Acurast and the deployed price feeds.

## API3

The [API3 Market](https://market.api3.org/bob) provides access to 200+ price feeds on [BOB Mainnet](https://market.api3.org/bob) and [Testnet](https://market.api3.org/bob-sepolia-testnet). The price feeds operate as a native push oracle and can be activated instantly via the Market UI.

The price feeds are delivered by an aggregate of [first-party oracles](https://old-docs.api3.org/airnode/v0.10/#designed-for-first-party-oracles) using signed data and support [OEV recapture](https://docs.api3.org/oev-searchers/in-depth/oev-searching.html).

Unlike traditional data feeds, reading [API3 price feeds](https://docs.api3.org/guides/dapis/) enables dApps to auction off the right to update the price feeds to searcher bots which facilitates more efficient liquidation processes for users and LPs of DeFi money markets. The OEV recaptured is returned to the dApp.

Check out these guides on how to:
- [Use dAPIs on the Market](https://docs.api3.org/guides/dapis/subscribing-to-dapis/)
- [Read a dAPI](https://docs.api3.org/guides/dapis/read-a-dapi/)

<!-- TODO: Add addresses after API3 deploys -->

## RedStone

[RedStone](https://redstone.finance/) is a modular Oracle providing Pull and Push model on BOB.

1. You can use multiple Push (RedStone Classic) feeds including BTC, ETH, STONE, USDT, USDC, wstETH following the [addresses in the docs](https://docs.redstone.finance/docs/smart-contract-devs/price-feeds).
2. Integrate versatile Pull (RedStone Core) feeds with low-latency and innovative design of attaching signed data packages in transaction call-data by [following these steps](https://docs.redstone.finance/docs/smart-contract-devs/get-started/redstone-core).

**Supported Networks**

- BOB Mainnet: [0x2d484E029b8Ae5cA6335DAe11fC726B232bE87D1](https://explorer.gobob.xyz/address/0x2d484E029b8Ae5cA6335DAe11fC726B232bE87D1) STONE, USDC, USDT, BTC, ETH, wstETH multi-price feed

## DIA

[DIA](https://www.diadata.org/) provides [price feeds](https://www.diadata.org/app/price/) for 20,000+ assets on BOB.

See [this guide](https://docs.diadata.org/introduction/intro-to-dia-oracles/access-the-oracle) to learn how to use DIA price feeds. Learn more about DIA’s [data sourcing](https://docs.diadata.org/introduction/dia-technical-structure/data-sourcing) and [data computation](https://docs.diadata.org/introduction/dia-technical-structure/data-computation) architecture.

Here is an example of how to access a price value on DIA oracles:

1. Access your custom oracle smart contract on BOB.
2. Call `getValue(pair_name)` with `pair_name` being the full pair name such as `BTC/USD`. You can use the "Read" section on the explorer to execute this call.
3. The response of the call contains two values:

- The current asset price in USD with a fix-comma notation of 8 decimals.
- The UNIX timestamp of the last oracle update.

DIA has [oracle integration samples](https://docs.diadata.org/products/token-price-feeds/access-the-oracle) in Solidity and Vyper. For assistance, connect with the DIA team directly on [Discord](https://discord.gg/ZvGjVY5uvs) or [Telegram](https://t.me/diadata_org).

**Assets (BOB Mainnet)**

- WBTC/USD: [0x6Ae049df8FC215aC1A5f5226B220E51301FE7e3D](https://explorer.gobob.xyz/address/0x6Ae049df8FC215aC1A5f5226B220E51301FE7e3D)
- tBTC/USD: [0x7F1AD8fBb80A262cE5987439d2F4B52f1a426f55](https://explorer.gobob.xyz/address/0x7F1AD8fBb80A262cE5987439d2F4B52f1a426f55)
- USDC/USD: [0x13B6052B34c6A9Fe0419E5154826a1CB858f3181](https://explorer.gobob.xyz/address/0x13B6052B34c6A9Fe0419E5154826a1CB858f3181)
- USDT/USD: [0xF67Ce8007810e8e87B3871B104366b105a71bB55](https://explorer.gobob.xyz/address/0xF67Ce8007810e8e87B3871B104366b105a71bB55)
- ETH/USD: [0x27abC874f709fbc7b2af4153e875cf52C701725E](https://explorer.gobob.xyz/address/0x27abC874f709fbc7b2af4153e875cf52C701725E)
- wstETH/USD: [0xa6aEdd027Bc91569617c26EEB6146A2b9148788a](https://explorer.gobob.xyz/address/0xa6aEdd027Bc91569617c26EEB6146A2b9148788a)
- eSOV/USD: [0x3844E091e4058c7B8D96b3eCb676d0B40d1941d2](https://explorer.gobob.xyz/address/0x3844E091e4058c7B8D96b3eCb676d0B40d1941d2)

**Supported Networks**

- BOB Mainnet: [0x4d24e7c1cf0ed63bc8c6cb5a795af31fd8127c6b](https://explorer.gobob.xyz/address/0x4d24E7c1cF0ed63bc8c6cB5a795af31FD8127C6B)

## Tellor

[Tellor](https://tellor.io/) is a versatile oracle protocol that can provide any data type permissionlessly, with crypto-economic liveness and security.

To use Tellor data see their [integration guide](https://docs.tellor.io/tellor/getting-data/solidity-integration) and BOB related [contracts](https://docs.tellor.io/tellor/the-basics/contracts-reference#bob).

**Supported Networks**

- BOB Mainnet: [0x896419Ed2E0dC848a1f7d2814F4e5Df4b9B9bFcc](https://explorer.gobob.xyz/address/0x896419Ed2E0dC848a1f7d2814F4e5Df4b9B9bFcc)
