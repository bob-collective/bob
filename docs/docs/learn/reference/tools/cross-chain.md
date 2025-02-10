# Cross-chain

## Chainlink CCIP

BOB has chosen Chainlink’s [Cross-Chain Interoperability Protocol (CCIP)](https://chain.link/cross-chain) as its canonical bridge solution. This means asset issuers looking to expand their token cross-chain can benefit from CCIP. This applies both to assets issued natively on BOB that want to expand to other chains, as well as native tokens on other chains looking to bridge into the BOB ecosystem.

Chainlink CCIP enables developers to build secure cross-chain apps that can transfer tokens, send messages, and initiate actions across blockchains. Through the [Cross-Chain Token (CCT)](https://blog.chain.link/ccip-v-1-5-upgrade/) standard, CCIP enables token developers to integrate new and existing tokens with CCIP in a self-serve manner in minutes. CCTs support self-serve deployments, full control and ownership for developers, enhanced programmability, and zero-slippage transfers. CCIP is built with [defense-in-depth security](https://blog.chain.link/ccip-security-features/) and is powered by Chainlink oracle networks—a proven standard with a track record of securing tens of billions of dollars and enabling over $18 trillion in onchain transaction value.

A selection of bridging apps powered by CCIP:

- [BOB Bridge](https://app.gobob.xyz/bridge)—Native bridging from Ethereum mainnet, including CCT-powered tokens.
- [Transporter](https://www.transporter.io/)—an intuitive bridging app built in association with the Chainlink Foundation.
- [XSwap](https://xswap.link/)—a cross-chain swaps protocol powered by Chainlink CCIP.
- [Interport](https://interport.fi/)—a comprehensive cross-chain hub powered by Chainlink CCIP.

Key CCIP developer tools:

- [CCIP official documentation](https://docs.chain.link/ccip)—start adopting CCIP into your cross-chain application.
- [CCIP Token Manager](https://tokenmanager.chain.link/)—an intuitive front-end web interface for the deployment of new and management of existing CCTs by their developers, including no-code guided deployments and configuration tools.
- [CCIP SDK](https://docs.chain.link/ccip/ccip-javascript-sdk)—a software development kit that streamlines the process of integrating CCIP, allowing developers to use JavaScript to create a token transfer frontend dApp.

If you require technical advice or wish to consult on your project's implementation, please contact a CCIP expert through Chainlink’s [CCIP contact form](https://chain.link/ccip-contact).

Supported Networks

- [BOB Mainnet](https://docs.chain.link/ccip/directory/mainnet/chain/bitcoin-mainnet-bob-1)
- [BOB Sepolia](https://docs.chain.link/ccip/directory/testnet/chain/bitcoin-testnet-sepolia-bob-1)

## Hyperlane

[Hyperlane](https://hyperlane.xyz/) is a permissionless interoperability protocol for cross-chain communication. It enables message passing and asset transfers across different chains without relying on centralized intermediaries or requiring any permissions.

See [Hyperlane's docs](https://docs.hyperlane.xyz/docs/intro) to learn how you can integrated their protocol. Their [GitHub](https://github.com/hyperlane-xyz) and [Discord](https://discord.com/invite/hyperlane) are available as well.

### Supported Networks

**Supported Networks**

- [BOB Mainnet](https://docs.hyperlane.xyz/docs/reference/domains)

## LayerZero

LayerZero is a technology that enables applications to move data across blockchains, uniquely supporting censorship-resistant messages and permissionless development through immutable smart contracts. Projects on BOB can connect to 35+ supported blockchains using LayerZero's contracts.

Visit [LayerZero's documentation](https://docs.layerzero.network/v2) to learn more about their supported chains, data schema, omnichain fungible tokens (OFTs), and other useful guides. Their [GitHub repository](https://github.com/LayerZero-Labs) and [Discord](https://discord-layerzero.netlify.app/discord) are also wonderful resources for getting started.

### Supported Networks

**Supported Networks**

- [BOB Mainnet](https://docs.layerzero.network/v2/developers/evm/technical-reference/deployed-contracts#bob)
- [BOB Testnet](https://docs.layerzero.network/v2/developers/evm/technical-reference/deployed-contracts#bob-testnet)

:::warning BOB Testnet is not BOB Sepolia
For historical reasons, the smart contracts in the link above point to our previous testnet, "BOB Testnet". While it has been replaced by our newer testnet, "BOB Sepolia", the LayerZero contracts remain live on BOB Testnet.
:::
