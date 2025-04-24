---
title: Onboarding
sidebar_position: 10
---

# Onboarding Solutions

## thirdweb 
 
 [**Universal Bridge**](https://thirdweb.com/connect/universal-bridge) is a comprehensive Web3 payment solution to help you monetize any app or game.  
 
 With Universal Bridge, your users can **onramp, bridge, and swap** on any EVM chain — with any EVM token or fiat — thanks to its **automatic cross-chain routing**.  
 
 Plus, developers can **earn from day one** using the **fee-sharing mechanism** and **easy implementation**.
 
 **Supported Networks**
 - [BOB Mainnet](https://thirdweb.com/bridge?chainId=60808)
 
 ### Universal Bridge Features
 
 - Let users pay for assets in any EVM token on BOB right in your app  
 - Automatic cross-chain routing for seamless transactions  
 - Earn from day one with fee-sharing mechanism
 - Access ready-made UI component for easy implementation  
 
 [Learn more in the full documentation on thirdweb’s Universal Bridge](https://portal.thirdweb.com/connect/pay/overview)
 
 ### Get Started with thirdweb Universal Bridge
 
 [How to get started with Universal Bridge](https://portal.thirdweb.com/connect/pay/get-started)
 
 ### See Universal Bridge in Action
 Want to see how Universal Bridge works? Check it out under the hood
 
 ```jsx
 import { createThirdwebClient } from "thirdweb";
 import { PayEmbed } from "thirdweb/react";
 
 import { createWallet } from "thirdweb/wallets";
 import { defineChain } from "thirdweb";
 
 const client = createThirdwebClient({
   clientId: "....",
 });
 
 function Example() {
   return (
     <PayEmbed
       client={client}
       payOptions={{
         mode: "fund_wallet",
         prefillBuy: {
           chain: defineChain(60808),
           amount: "0.01",
         },
       }}
     />
   );
 }
 
 ```
 [**See how the Buy Crypto flow works in the playground**](https://playground.thirdweb.com/connect/pay)
 

## Banxa

[Banxa](https://banxa.com) is a payment service provider that allows users to buy and sell cryptocurrencies using fiat currencies. Banxa provides an easy-to-integrate solution for applications looking to offer fiat on-ramps to their users.

You can access Banxa's checkout system for BOB via this link:
[https://checkout.banxa.com/?coinType=ETH&fiatType=EUR&blockchain=BOB](https://checkout.banxa.com/?coinType=ETH&fiatType=EUR&blockchain=BOB)

Banxa supports multiple cryptocurrencies and blockchain networks. For BOB specifically, the following tokens are supported:
- ETH (Ethereum)
- USDC (USD Coin)

For a complete list of all supported cryptocurrencies and blockchains, visit [Banxa's documentation](https://docs.banxa.com/docs/available-cryptocurrencies-and-blockchains).

To integrate Banxa into your BOB application, follow the [integration tutorial](https://docs.banxa.com/docs/tutorial). The process typically involves setting up a partner account, implementing the API or iframe checkout solution, configuring webhooks for order status updates, and testing before going live 
