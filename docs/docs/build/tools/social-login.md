# Social Login

## Arcana Wallet

The [Arcana Network](https://arcana.network) SDKs enable web3 developers to use [social login](https://docs.arcana.network/concepts/social-login) to quickly onboard new users.

Authenticated users can instantly access the embedded, non-custodial [Arcana wallet](https://docs.arcana.network/concepts/anwallet/) from within the app to sign blockchain transactions.

Developers can leverage the [built-in gasless feature](https://docs.arcana.network/quick-start/gasless-standalone-quick-start) to sponsor gas fees for allowlisted blockchain transactions. The gasless transactions can be enabled for Arcana Wallet and third-party, browser-based wallets.

Developers need to follow these steps for integration:

1. **Register the app** with the [Arcana Developer Dashboard](https://dashboard.arcana.network/), and copy the unique client identifier for the app.
2. **Configure Auth SDK usage** via the dashboard, specify social login options, wallet user experience settings, gasless transaction settings, etc.
3. **Install the Arcana Auth SDK and use the client identifier to integrate the app**. Initialize the `AuthProvider`, then add a single line code to onboard users by calling the `connect()` method and enabling the plug-and-play login UI. After a successful user login, _Arcana wallet will automatically display within the app context_, enabling the user to sign blockchain transactions instantly.

:::note

Arcana wallet supports [JSON/RPC calls and web3 wallet operations](https://docs.arcana.network/web-sdk/auth-usage-guide#arcana-wallet-operations).

:::

## References

- [Social login Providers](https://docs.arcana.network/state-of-the-arcana-auth#app-login)
- [Blockchain networks](https://docs.arcana.network/state-of-the-arcana-auth#supported-blockchains)
- [Browsers](https://docs.arcana.network/state-of-the-arcana-auth#supported-browsers)
- [Supported App Types](https://docs.arcana.network/sdk-installation)
- [SDK Reference Guides](https://docs.arcana.network/dev-resources/sdk-ref/)
