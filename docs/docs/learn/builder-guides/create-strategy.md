---
sidebar_position: 3
---

# Create a New Strategy for BOB Gateway

## Introduction

BOB's Hybrid L2 vision is to unite the Bitcoin and Ethereum ecosystems. An important part of that goal is to improve the security and UX of native BTC holders looking to participate in DeFi.

BOB Gateway makes it possible for them to deposit into your smart contract with only one Bitcoin transaction.

You can find all current strategies and their tests in the BOB SDK GitHub repo.

### Brief Recap of BOB Gateway

Gateway is a trust-minimized P2P swap between BTC on Bitcoin mainnet and wrapped BTC on BOB. The process may include optional smart contract calls to execute the user's "intent". In the language of account abstraction, this is also referred to as "intent-based bridging" by executing "user operations".

In our language, BOB Gateway is the relayer that monitors orders, pairs users with liquidity providers, and trustlessly executes a "strategy" to accomplish the user's intent. Some intents are one step, such as staking or lending. Other intents combine several actions, like staking-and-lending or staking-and-restaking.

No matter how sophisticated the strategy, everything is handled on the user's behalf. Their UX is simply to fill out an order form and sign one Bitcoin transaction.

## How to Write a Strategy

We’ll build up to a complex, multi-step strategy by starting with a simple one that we later extend.

### One-step Strategy example: staking uniBTC

At this point in the P2P swap process the Gateway relayer has already verified that the user sent BTC to the LP. Having trustlessly verified this, the relayer sets about manipulating the LP's wrapped BTC to accomplish the user's intent.

In this example the relayer calls Bedrock's uniBTC vault to deposit WBTC as collateral and mint uniBTC tokens.

```typescript
  // Call the uniBTC vault's mint function with the LP's WBTC
  vault.mint(address(tokensent), amountin);
  ierc20 unibtc = ierc20(vault.unibtc());
  uint256 unibtcamount = unibtc.balanceof(address(this));
  // Transfer the token to the user to complete the process
  uniBTC.safeTransfer(recipient, uniBTCAmount);
```

### Multi-step Strategy Example: stake-and-lend

You are not limited to one smart contract call. You may wish to make several functions calls sequentially to accomplish a more sophisticated goal.

A user with a higher risk tolerance may want to seek even more yield by depositing their BTC LST in a lending protocol. To extend the staking example above, let's look at a snippet that picks up just after the `mint` step.

From AvalonStrategy.sol:

```typescript AvalonStrategy.sol
// Deposit uniBTC from the previous step into the Avalon lending protocol
// The recipient is the user's EVM address, so their wallet controls the output tokens
pool.supply(address(tokenSent), amountIn, recipient, 0);
```

## How to Add Your Strategy to Gateway

A new Gateway strategy requires three files:

<!--TODO: Add links to GitHub folders or Contract pages-->

1. Strategy
2. Unit test
3. End-to-end test

Make a PR to our repo with your strategy and we’ll merge it after review.

:::tip we’d love to support you
You're welcome to reach out to our Developer Relations Engineer, @DerrekWonders, on [Discord](https://discord.gg/gobob) with any questions.
:::

At the moment there is only one relayer for BOB Gateway. In addition to its other functionality, this relayer pays gas on the user's behalf so that the user doesn't need to make a transaction on BOB to complete the bridging process. Since a malicious actor could create a strategy designed to take advantage of the relayer (e.g. spend all of the funds available for gas fees), the process of adding new strategies to Gateway as well as decentralizing the relayer role are restricted by the BOB team until Gateway is upgraded as described [on this page](/learn/introduction/gateway/).

### Example Unit Test: uniBTC

In addition to the strategy contract described above, you also need to create unit tests for each important function call.

Returning to the single-step uniBTC staking strategy from the beginning:

```solidity BedrockStrategy.sol
function testDepositTokenIntoVault() public {
    IBedrockVault vault = new DummyBedrockVaultImplementation(uniBtcToken, true);
    uniBtcToken.sudoMint(address(vault), 1 ether);

    BedrockStrategy bedrockStrategy = new BedrockStrategy(vault);

    // Approve strategy to spend a token on behalf of this contract
    wrappedBTC.increaseAllowance(address(bedrockStrategy), 1 ether);

    vm.expectEmit();
    emit TokenOutput(address(uniBtcToken), 1 ether);
    bedrockStrategy.handleGatewayMessageWithSlippageArgs(
        wrappedBTC, 1 ether, vm.addr(1), StrategySlippageArgs(1 ether)
    );
}
```

<!--TODO: link to file or Contract-->

See the BedrockStrategy.sol testing file for imports, constants, set up, and other tests.

### Example End-to-end Test: uniBTC

Continuing with our example, let’s look at the end-to-end integration test using Foundry’s ability to simulate the transactions on a live fork of BOB mainnet. That’s why the e2e test files follow a [ProtocolName]StrategyForked.sol naming convention.

```solidity BedrockStrategyForked.sol
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

After forking and installing the repo, you can run this test with the following command.

```bash
BOB_PROD_PUBLIC_RPC_URL=https://rpc.gobob.xyz/ forge test --match-contract BedrockStrategyForked -vv
```

<!--TODO: link to file or Contract-->

Constants.sol has useful constants for your test functions, such as WBTC/tBTC contract addresses and dummy sender/receiver addresses.

## FAQs

Q: How do I present my strategy as an option to my users?
A: See the Gateway SDK page for a step-by-step integration guide for your app’s frontend.

Q: Do I need to add Bitcoin wallet support?
A: Yes. Gateway’s UX is designed around the goal of accomplishing the user’s intent after signing only one Bitcoin transaction (and no EVM transactions).

Don’t worry, you don’t need to implement any Bitcoin transaction logic. After confirming the user’s order, the SDK returns a partially signed Bitcoin transaction (PSBT). You present the PSBT for a signature to the user, then send the signed transaction back to the SDK. Gateway will broadcast the transaction to the Bitcoin network for you. Learn more on the frontend integration page

Options for adding Bitcoin wallet support to your frontend:

<!--TODO: add links-->

- sats-wagmi
- Dynamic.xyz

Q: What happens if the strategy call fails?
A: Gateway automatically falls back to sending the wrapped BTC to the user’s EVM address.
