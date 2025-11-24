---
sidebar_position: 3
---

# Monetization

Gateway supports optional affiliate fees, allowing integrators to earn revenue every time users execute Bitcoin onramp or offramp flows through their app.
For context on how `getQuote` works, see the **[Integration Guide](./integration.md)**.

## How It Works

When requesting a quote, you can include two optional parameters:

```ts
/** @description Affiliate-related fee */
affiliateFeeSats?: bigint;

/** @description The address of the affiliate who will receive the affiliate fee */
affiliateFeeRecipient?: Address;
```

## Adding Affiliate Fees to [`getQuote`](./integration#get-a-quote)
You can pass the monetization fields directly into your getQuote call:

```ts
const quote = await gatewaySDK.getQuote({
  fromChain: 'bitcoin',
  fromToken: 'BTC',
  fromUserAddress: 'bc1qafk4yhqvj4wep57m62dgrmutldusqde8adh20d',
  toChain: 'bob',
  toUserAddress: '0x2D2E86236a5bC1c8a5e5499C517E17Fb88Dbc18c',
  toToken: 'wBTC',
  amount: parseBtc("0.1"),
  // Example: 1000 sats fee  
  affiliateFeeSats: 1000n,
  // Affiliate / partner EVM address  
  affiliateFeeRecipient: '0xDeaDDEaDDeAdDeAdDEAdDEaddeAddEAdDEAd0001',    
});
```

After obtaining the quote, follow the **[Integration Guide](./integration.md)** to  [execute the quote](./integration#execute-the-quote) and complete the flow.