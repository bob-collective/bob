---
sidebar_position: 3
---

# Strategies

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

### One-Intent Example: Staking WBTC into xSolvBTC

At this point in the P2P swap process, the Gateway relayer has already verified that the user sent BTC to the LP. Having trustlessly verified this, the relayer sets about manipulating the LP's wrapped BTC to accomplish the user's intent.

[In this example](https://github.com/bob-collective/bob/blob/master/contracts/src/gateway/strategy/SolvStrategy.sol#L110) the relayer deposits WBTC to mint SolvBTC, which is then deposited to mint xSolvBTC, the LST that receives yield from Babylon.

```solidity title="SolvStrategy.sol"
// Mint SolvBTC with the LP's WBTC
uint256 shareValueBTC = solvBTCRouter.createSubscription(solvBTCPoolId, amountIn);

// Mint xSolvBTC with the SolvBTC created in the previous LP's WBTC
solvBTC.safeIncreaseAllowance(address(solvLSTRouter), shareValueBTC);
uint256 shareValueLST = solvLSTRouter.createSubscription(solvLSTPoolId, shareValueBTC);
require(shareValueLST >= args.amountOutMin, "Insufficient output amount");

// Transfer the xSolvBTC token to the user to complete the process
solvLST.safeTransfer(recipient, shareValueLST);
```

### Multi-Intent Example: Stake-and-Lend

As we saw above, you are not limited to one smart contract call. You may wish to make several functions calls sequentially to accomplish a more sophisticated goal. This allows you to compose several DeFi protocols, such as staking, restaking, or lending.

A user with a higher risk tolerance may want to seek even more yield by depositing their BTC LST in a lending protocol. To extend the staking example above, let's look at [a snippet from AvalonStrategy.sol](https://github.com/bob-collective/bob/blob/master/contracts/src/gateway/strategy/AvalonStrategy.sol#L66) that replaces the final `safeTransfer` function with an additional step to deposit the xSolvBTC into the Avalon lending protocol.

```solidity title="AvalonStrategy.sol"
// tokenSent is solvLST, which identifies the correct Avalon lending pool
// amountIn is the balanceOf xSolvBTC from the previous step
// recipient is the user's EVM address, so their wallet controls the output tokens
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

At the moment, there is only one relayer for BOB Gateway. In addition to its other functionality, this relayer pays gas on the user's behalf so that the user doesn't need to make a transaction on BOB to complete the bridging process. Since a malicious actor could create a strategy designed to take advantage of the relayer (e.g. spend all of the funds available for gas fees), the process of adding new strategies to Gateway as well as decentralizing the relayer role are restricted by the BOB team until Gateway is upgraded as described [on this page](/learn/introduction/gateway/).

### Example End-to-End Test: Staking xSolvBTC

In addition to the strategy contract described above, you also need to create an end-to-end integration test using Foundry’s ability to simulate the transactions on a live fork of BOB mainnet. That’s why the e2e test files follow a `[ProtocolName]StrategyForked.sol` naming convention, such as [SolvStrategyForked.sol](https://github.com/bob-collective/bob/blob/master/contracts/test/gateway/e2e-strategy-tests/SolvStrategyForked.sol#L40).

Returning to the [xSolvBTC staking strategy](#one-intent-example-staking-wbtc-into-xsolvbtc) from the beginning:

```solidity title="SolvStrategyForked.sol"
function testSolvLSTStrategy() public {
    IERC20 solvBTC = IERC20(0x541FD749419CA806a8bc7da8ac23D346f2dF8B77);
    IERC20 solvBTCBBN = IERC20(0xCC0966D8418d412c599A6421b760a847eB169A8c);
    SolvLSTStrategy strategy = new SolvLSTStrategy(
        ISolvBTCRouter(0x49b072158564Db36304518FFa37B1cFc13916A90),
        ISolvBTCRouter(0xbA46FcC16B464D9787314167bDD9f1Ce28405bA1),
        0x5664520240a46b4b3e9655c20cc3f9e08496a9b746a478e476ae3e04d6c8fc31,
        0x6899a7e13b655fa367208cb27c6eaa2410370d1565dc1f5f11853a1e8cbef033,
        solvBTC,
        solvBTCBBN
    );

    vm.startPrank(Constants.DUMMY_SENDER);
    token.approve(address(strategy), 1 * 1e8);

    strategy.handleGatewayMessageWithSlippageArgs(token, 1e8, Constants.DUMMY_RECEIVER, StrategySlippageArgs(0));
    vm.stopPrank();

    assertEq(solvBTCBBN.balanceOf(Constants.DUMMY_RECEIVER), 1 ether);
}
```

After forking and installing the repo, you can run this test with the following command in your shell:

```shell
BOB_PROD_PUBLIC_RPC_URL=https://rpc.gobob.xyz/ forge test --match-contract SolvStrategyForked -vv
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
