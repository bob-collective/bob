---
sidebar_position: 3
---

# Create a New Strategy for BOB Gateway

## Introduction

BOB's Hybrid L2 vision is to unite the Bitcoin and Ethereum ecosystems. An important part of that goal is to improve the security and UX of native BTC holders looking to participate in DeFi.

BOB Gateway makes it possible for them to deposit into your smart contract with only one Bitcoin transaction.

Our open-source repositories contain [all current strategies](https://github.com/bob-collective/bob/tree/master/contracts/src/gateway/strategy) and [their tests](https://github.com/bob-collective/bob/tree/master/contracts/test/gateway/e2e-strategy-tests).

### Brief Recap of BOB Gateway

Gateway is a trust-minimized P2P swap between BTC on Bitcoin mainnet and wrapped BTC on BOB. The process may include optional smart contract calls to execute the user's "intent". In the language of account abstraction, this is also referred to as "intent-based bridging" by executing "user operations".

In our language, BOB Gateway is the relayer that monitors orders, pairs users with liquidity providers, and trustlessly executes a "strategy" to accomplish the user's intent. Some intents are one step, such as staking or lending. Other intents combine several actions, like staking-and-lending or staking-and-restaking.

No matter how sophisticated the strategy, everything is handled on the user's behalf. Their UX is simply to fill out an order form and sign one Bitcoin transaction.

## How to Write a Strategy

We’ll build up to a complex, multi-step strategy by starting with a simple one that we later extend.

### One-Step Strategy Example: Staking uniBTC

At this point in the P2P swap process the Gateway relayer has already verified that the user sent BTC to the LP. Having trustlessly verified this, the relayer sets about manipulating the LP's wrapped BTC to accomplish the user's intent.

[In this example](https://github.com/bob-collective/bob/blob/master/contracts/src/gateway/strategy/BedrockStrategy.sol#L48) the relayer calls Bedrock's uniBTC vault to deposit WBTC as collateral and mint uniBTC tokens.

```typescript title="BedrockStrategy.sol"
  // Call the uniBTC vault's mint function with the LP's WBTC
  vault.mint(address(tokensent), amountin);
  ierc20 unibtc = ierc20(vault.unibtc());
  uint256 unibtcamount = unibtc.balanceof(address(this));
  // Transfer the token to the user to complete the process
  uniBTC.safeTransfer(recipient, uniBTCAmount);
```

### Multi-Step Strategy Example: Stake-and-Lend

You are not limited to one smart contract call. You may wish to make several functions calls sequentially to accomplish a more sophisticated goal.

A user with a higher risk tolerance may want to seek even more yield by depositing their BTC LST in a lending protocol. To extend the staking example above, [let's look at a snippet](https://github.com/bob-collective/bob/blob/master/contracts/src/gateway/strategy/AvalonStrategy.sol#L66) that picks up just after the `mint` step.

```typescript title="AvalonStrategy.sol"
// Deposit uniBTC from the previous step into the Avalon lending protocol
// The recipient is the user's EVM address, so their wallet controls the output tokens
pool.supply(address(tokenSent), amountIn, recipient, 0);
```

## How to Add Your Strategy to Gateway

A new Gateway strategy requires two files:

1. [Strategy](https://github.com/bob-collective/bob/tree/master/contracts/src/gateway/strategy)
1. [End-to-end test](https://github.com/bob-collective/bob/tree/master/contracts/test/gateway/e2e-strategy-tests)

Make a PR to our repo with your strategy and we’ll merge it after review.

:::tip we’d love to support you
You're welcome to reach out to our Developer Relations Engineer, @DerrekWonders, on [Discord](https://discord.gg/gobob) with any questions.
:::

At the moment there is only one relayer for BOB Gateway. In addition to its other functionality, this relayer pays gas on the user's behalf so that the user doesn't need to make a transaction on BOB to complete the bridging process. Since a malicious actor could create a strategy designed to take advantage of the relayer (e.g. spend all of the funds available for gas fees), the process of adding new strategies to Gateway as well as decentralizing the relayer role are restricted by the BOB team until Gateway is upgraded as described [on this page](/learn/introduction/gateway/).

### Example End-to-End Test: Staking uniBTC

In addition to the strategy contract described above, you also need to create an end-to-end integration test using Foundry’s ability to simulate the transactions on a live fork of BOB mainnet. That’s why the e2e test files follow a `[ProtocolName]StrategyForked.sol` naming convention, such as [BedrockStrategyForked.sol](https://github.com/bob-collective/bob/blob/master/contracts/test/gateway/e2e-strategy-tests/BedrockStrategyForked.sol#L21).

Returning to the single-step uniBTC staking strategy from the beginning:

```solidity title="BedrockStrategyForked.sol"
function testBedrockStrategy() public {
    IBedrockVault vault = IBedrockVault(0x2ac98DB41Cbd3172CB7B8FD8A8Ab3b91cFe45dCf);
    BedrockStrategy strategy = new BedrockStrategy(vault);

    vm.startPrank(Constants.DUMMY_SENDER);
    token.approve(address(strategy), 1e8);
    strategy.handleGatewayMessageWithSlippageArgs(
        token,
        1e8, // Amount: 1 WBTC
        Constants.DUMMY_RECEIVER,
        StrategySlippageArgs(0) // No slippage allowed
    );
    vm.stopPrank();

    IERC20 uniBTC = IERC20(vault.uniBTC());
    assertEq(uniBTC.balanceOf(Constants.DUMMY_RECEIVER), 1e8, "User uniBTC balance mismatch");
}
```

After forking and installing the repo, you can run this test with the following command in your shell:

```shell
BOB_PROD_PUBLIC_RPC_URL=https://rpc.gobob.xyz/ forge test --match-contract BedrockStrategyForked -vv
```

[Constants.sol](https://github.com/bob-collective/bob/blob/master/contracts/test/gateway/e2e-strategy-tests/Constants.sol) has useful constants for your test functions, such as WBTC/tBTC contract addresses and dummy sender/receiver addresses.

## Frequently Asked Questions

### How do I present my strategy as an option to my users?

See the Gateway SDK page for a step-by-step integration guide for your app's frontend.

### Do I need to add Bitcoin wallet support?

Yes. Gateway's UX is designed around the goal of accomplishing the user's intent after signing only one Bitcoin transaction (and no EVM transactions).

Don't worry, you don't need to implement any Bitcoin transaction logic. After confirming the user's order, the SDK returns a partially signed Bitcoin transaction (PSBT). You present the PSBT for a signature to the user, then send the signed transaction back to the SDK. Gateway will broadcast the transaction to the Bitcoin network for you. Learn more on the frontend integration page.

Options for adding Bitcoin wallet support to your frontend:

- [sats-wagmi](/learn/builder-guides/sats-wagmi/)
- [Dynamic.xyz](https://www.dynamic.xyz/)

### What happens if the strategy call fails?

Gateway automatically falls back to sending the wrapped BTC to the user's EVM address.
