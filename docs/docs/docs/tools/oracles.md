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

[DIA](https://www.diadata.org/) is a cross-chain, trustless oracle network delivering verifiable price feeds for BOB. DIA sources raw trade data directly from primary markets and computes it onchain, ensuring complete transparency and data integrity.

To Query DIA Price Feeds:

Call the `getValue()` function on the oracle contract with the Query Symbol (e.g., "WBTC/USD"). The function returns the asset price with 8 decimal precision and the timestamp of the last update. You can view the full BOB oracle integration guide [here](https://www.diadata.org/docs/guides/chain-specific-guide/bob-build-on-bitcoin).

**Price Feeds on BOB Mainnet**

- WBTC/USD: [0x6Ae049df8FC215aC1A5f5226B220E51301FE7e3D](https://explorer.gobob.xyz/address/0x6Ae049df8FC215aC1A5f5226B220E51301FE7e3D)
- tBTC/USD: [0x7F1AD8fBb80A262cE5987439d2F4B52f1a426f55](https://explorer.gobob.xyz/address/0x7F1AD8fBb80A262cE5987439d2F4B52f1a426f55)
- USDC/USD: [0x13B6052B34c6A9Fe0419E5154826a1CB858f3181](https://explorer.gobob.xyz/address/0x13B6052B34c6A9Fe0419E5154826a1CB858f3181)
- USDT/USD: [0xF67Ce8007810e8e87B3871B104366b105a71bB55](https://explorer.gobob.xyz/address/0xF67Ce8007810e8e87B3871B104366b105a71bB55)
- ETH/USD: [0x27abC874f709fbc7b2af4153e875cf52C701725E](https://explorer.gobob.xyz/address/0x27abC874f709fbc7b2af4153e875cf52C701725E)
- wstETH/USD: [0xa6aEdd027Bc91569617c26EEB6146A2b9148788a](https://explorer.gobob.xyz/address/0xa6aEdd027Bc91569617c26EEB6146A2b9148788a)
- eSOV/USD: [0x3844E091e4058c7B8D96b3eCb676d0B40d1941d2](https://explorer.gobob.xyz/address/0x3844E091e4058c7B8D96b3eCb676d0B40d1941d2)

For assets not currently available or dApps requiring specific configurations, you can request for a custom oracle tailored to your requirements.

→ [Request a Custom Oracle](https://www.diadata.org/docs/guides/how-to-guides/request-a-custom-oracle)

**Supported Networks**

- BOB Mainnet: [0x4d24e7c1cf0ed63bc8c6cb5a795af31fd8127c6b](https://explorer.gobob.xyz/address/0x4d24E7c1cF0ed63bc8c6cB5a795af31FD8127C6B)

## Tellor

[Tellor](https://tellor.io/) is a versatile oracle protocol that can provide any data type permissionlessly, with crypto-economic liveness and security.

To use Tellor data see their [integration guide](https://docs.tellor.io/tellor/getting-data/solidity-integration) and BOB related [contracts](https://docs.tellor.io/tellor/the-basics/contracts-reference#bob).

**Supported Networks**

BOB Mainnet:

- Token: [0x665060707c3Ea3c31b3eaBaD7F409072446E1D50](https://explorer.gobob.xyz/address/0x665060707c3Ea3c31b3eaBaD7F409072446E1D50)
- Oracle: [0x896419Ed2E0dC848a1f7d2814F4e5Df4b9B9bFcc](https://explorer.gobob.xyz/address/0x896419Ed2E0dC848a1f7d2814F4e5Df4b9B9bFcc)
- Governance: [0xC866DB9021fe81856fF6c5B3E3514BF9D1593D81](https://explorer.gobob.xyz/address/0xC866DB9021fe81856fF6c5B3E3514BF9D1593D81)
- Autopay: [0x9EA18BFDB50E9bb4A18F9d3Df7804E398F8fE0dc](https://explorer.gobob.xyz/address/0x9EA18BFDB50E9bb4A18F9d3Df7804E398F8fE0dc)
  BOB Testnet:

- Token/Playground: [0x896419Ed2E0dC848a1f7d2814F4e5Df4b9B9bFcc](https://bob-sepolia.explorer.gobob.xyz/address/0x896419Ed2E0dC848a1f7d2814F4e5Df4b9B9bFcc)
- Oracle: [0xC866DB9021fe81856fF6c5B3E3514BF9D1593D81](https://bob-sepolia.explorer.gobob.xyz/address/0xC866DB9021fe81856fF6c5B3E3514BF9D1593D81)
- Governance: [0x6684E5DdbEe1b97E10847468cB5f4e38f3aB83FE](https://bob-sepolia.explorer.gobob.xyz/address/0x6684E5DdbEe1b97E10847468cB5f4e38f3aB83FE)
- Autopay: [0x89e44099f5E80484dcF48995080481214b9c2D7c](https://bob-sepolia.explorer.gobob.xyz/address/0x89e44099f5E80484dcF48995080481214b9c2D7c)
- QueryDataStorage: [0x9EA18BFDB50E9bb4A18F9d3Df7804E398F8fE0dc](https://bob-sepolia.explorer.gobob.xyz/address/0x9EA18BFDB50E9bb4A18F9d3Df7804E398F8fE0dc)
