---
sidebar_position: 2
sidebar_label: Swap Tokens for Bitcoin
title: Swap tokens for native Bitcoin
description: Swap USDT, USDC, wrapped Bitcoin or tokenised gold for native BTC delivered to any Bitcoin address.
---

# Swap tokens for native Bitcoin

You hold a token on an EVM chain and want native Bitcoin. Gateway swaps it and sends BTC to any Bitcoin address you nominate.

1. **Connect your wallet**

   Connect any of the 500+ supported EVM wallets.

   A Bitcoin wallet connection is **optional** here. Connect one and Gateway fills in your receiving address automatically. If you'd rather not, paste the Bitcoin address you want the BTC sent to — every standard address type is accepted (Legacy, SegWit, Native SegWit and Taproot).

   It doesn't have to be a browser wallet. A hardware wallet address works just as well, as does a centralised exchange deposit address.

   <video src="/user-hub/tutorial/connect-wallet.mp4" controls muted loop playsInline />

2. **Set up the swap**

   Pick the token you want to swap and the chain it's on. The token picker shows exactly what's live on the route you've selected.

   <video src="/user-hub/tutorial/evm-to-btc-setup.mp4" controls muted loop playsInline />

3. **Review and swap**

   Enter the amount and confirm the recipient Bitcoin address. Check the quote — output amount, fees and estimated time are all shown before you commit.

   Then sign in your wallet. This takes two separate signatures: one to approve the token, one for the swap itself.

   <video src="/user-hub/tutorial/evm-to-btc-review.mp4" controls muted loop playsInline />

4. **Track your order**

   Open the wallet modal at the top right to follow progress and find your Order ID.

   For a detailed view, look the Order ID up in the [Gateway Explorer](https://gateway-explorer.gobob.xyz/) — it shows live status, amounts and transaction hashes on both sides, and the link is shareable.

   <video src="/user-hub/tutorial/evm-to-btc-track.mp4" controls muted loop playsInline />

5. **Receive your BTC**

   Once the swap settles, the BTC lands at your address. Most orders complete in around 10 minutes, though this varies with Bitcoin block times.

   ![Completed swap showing USDT0 on Arbitrum sent and BTC received](/user-hub/tutorial/evm-to-btc-received.png)

## Next steps

- **[Swap native Bitcoin for tokens](../btc-to-evm/)** — the reverse direction, with or without a Bitcoin wallet.
- **[Refunds](https://docs.gobob.xyz/gateway/refunds)** — how Gateway handles swaps that don't complete.
