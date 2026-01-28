---
sidebar_position: 3
---

# 1 Click DeFi Actions

## Introduction

BOB Gateway Onramp allows Bitcoin users to interact with DeFi protocols using a single Bitcoin transaction. There are three ways to integrate:

1. **Custom Strategy on BOB Chain** - Implement a strategy smart contract that receives wrapped BTC and executes your protocol logic
2. **Multicall Strategy Integration on BOB Chain** - Use the built-in multicall strategy to interact with existing contracts without deploying new ones
3. **Destination Multicall Strategy on LayerZero Chains** - Execute calls on destination chains after LayerZero cross-chain transfers (LayerZero only)

:::info Gateway Overview
For a detailed explanation of Gateway's architecture and user flow, see the [technical overview](./overview.md).
:::

Which approach should I choose?

- **Custom Strategy**: Choose this if you need complex logic, gas optimization, custom events, or want full control over the execution flow
- **Multicall Strategy**: Choose this if you want to integrate with existing contracts on BOB without deploying new ones, or for simple approve + deposit patterns
- **LayerZero Destination Strategy**: Choose this for cross-chain swaps where you want to execute calls on the destination chain (e.g., deposit into a lending protocol on Base after bridging from Bitcoin).

## Option 1: Custom Strategy Contract

Deploy a smart contract that implements the Gateway strategy interface. This gives you full control over the logic and allows for complex multi-step operations.

### Strategy Interface

Implement this interface in your contract:

```solidity
interface IStrategy {
    function handleGatewayMessage(
        IERC20 tokenSent,    // The wrapped BTC token (e.g., WBTC, tBTC)
        uint256 amountIn,    // Amount of wrapped BTC received
        address recipient,   // User's EVM address for receiving output tokens
        bytes memory message // Optional parameters from the user
    ) external;
}
```

### Complete Strategy Example

Here's a full implementation that converts WBTC to SolvBTC:

```solidity
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";

interface ISolvBTCRouter {
    function deposit(
        address targetToken_,
        address currency_,
        uint256 currencyAmount_,
        uint256 minimumTargetTokenAmount_,
        uint64 expireTime_
    ) external returns (uint256);
}

contract SolvBTCStrategy {
    using SafeERC20 for IERC20;

    ISolvBTCRouter public immutable solvBTCRouter;
    IERC20 public immutable solvBTC;

    constructor(address _solvBTCRouter, address _solvBTC) {
        solvBTCRouter = ISolvBTCRouter(_solvBTCRouter);
        solvBTC = IERC20(_solvBTC);
    }

    function handleGatewayMessage(
        IERC20 tokenSent,
        uint256 amountIn,
        address recipient,
        bytes memory message
    ) external {
        // Transfer wrapped BTC from Gateway
        tokenSent.safeTransferFrom(msg.sender, address(this), amountIn);

        // Approve SolvBTC router
        tokenSent.safeIncreaseAllowance(address(solvBTCRouter), amountIn);

        // Convert to SolvBTC
        uint256 solvBTCAmount = solvBTCRouter.deposit(
            address(solvBTC), 
            address(tokenSent), 
            amountIn, 
            minOutput: 0,
            uint64(block.timestamp + 1)
        );

        // Send SolvBTC to user
        solvBTC.safeTransfer(recipient, solvBTCAmount);
    }
}
```

## Option 2: Using the Multicall Strategy

Instead of deploying a custom strategy, you can use the multicall handler to execute multiple contract calls:

```typescript
import { encodeFunctionData, parseAbi, encodeAbiParameters, parseAbiParameters } from 'viem';

function generateMulticallMessage(userAddress: Address, depositAmount: bigint) {
    // Define the function signatures
    const erc20Abi = parseAbi(['function approve(address spender, uint256 value)']);
    const protocolAbi = parseAbi(['function deposit(address asset, uint256 amount, address onBehalfOf)']);
    
    // Encode the function calls
    const approveCall = encodeFunctionData({
        abi: erc20Abi,
        functionName: 'approve',
        args: [PROTOCOL_ADDRESS, depositAmount],
    });
    
    const depositCall = encodeFunctionData({
        abi: protocolAbi,
        functionName: 'deposit', 
        args: [WBTC_ADDRESS, depositAmount, userAddress],
    });

    // Encode as multicall message
    return encodeAbiParameters(
        parseAbiParameters('((address target, bytes callData, uint256 value)[], address fallbackRecipient)'),
        [
            [
                [
                    { target: WBTC_ADDRESS, callData: approveCall, value: 0n },
                    { target: PROTOCOL_ADDRESS, callData: depositCall, value: 0n },
                ],
                userAddress, // fallback recipient
            ],
        ]
    );
}

// Use in Gateway SDK
const quote = await gatewaySDK.getQuote({
    // ... other parameters
    message: generateMulticallMessage(userAddress, depositAmount),
});
```

## Option 3: Using the LayerZero Destination Strategy

For cross-chain swaps via LayerZero, you can execute calls on the destination chain after tokens arrive. This enables complex DeFi interactions like depositing into lending protocols, staking, or swapping tokens - all in a single cross-chain transaction.

### When to Use Destination Actions

- **Cross-Chain Swaps**: When bridging from Bitcoin to another chain (e.g., Base, Ethereum) and want to execute actions on the destination chain
- **No Contract Deployment**: Similar to multicall, but for destination chains - no need to deploy contracts
- **Multi-Step Operations**: Execute approve + deposit patterns or other sequential calls on the destination chain

### Complete Example: Deposit into Lending Protocol

Here's a full example that bridges BTC to Base and deposits wBTC into a lending protocol:

```typescript
import { LayerZeroGatewayClient, parseBtc } from '@gobob/bob-sdk';
import { encodeFunctionData, erc20Abi, parseAbi, maxUint256 } from 'viem';

const client = new LayerZeroGatewayClient();

const wbtcAddress = '0x0555E30da8f98308EdB960aa94C0Db47230d2B9c';
const lendingPoolAddress = '0x...'; // Your lending protocol address
const userAddress = '0x...'; // User's address on destination chain

// Step 1: Approve lending pool to spend wBTC
const approveCallData = encodeFunctionData({
    abi: erc20Abi,
    functionName: 'approve',
    args: [lendingPoolAddress, maxUint256],
});

// Step 2: Deposit wBTC into lending pool
const lendingAbi = parseAbi([
    'function deposit(address asset, uint256 amount, address onBehalfOf, uint16 referralCode)',
]);

// Get quote first to determine the amount that will arrive
const initialQuote = await client.getQuote({
    fromChain: 'bitcoin',
    fromToken: 'BTC',
    toChain: 'base',
    toToken: wbtcAddress,
    fromUserAddress: 'bc1q...',
    toUserAddress: userAddress,
    amount: parseBtc("0.1"),
});

const depositAmount = BigInt(initialQuote.finalOutputSats);

const depositCallData = encodeFunctionData({
    abi: lendingAbi,
    functionName: 'deposit',
    args: [wbtcAddress, depositAmount, userAddress, 0],
});

// Get final quote with destination calls
const quote = await client.getQuote({
    fromChain: 'bitcoin',
    fromToken: 'BTC',
    toChain: 'base',
    toToken: wbtcAddress,
    fromUserAddress: 'bc1q...',
    toUserAddress: userAddress,
    amount: parseBtc("0.1"),
    destinationCalls: {
        calls: [
            {
                target: wbtcAddress,
                callData: approveCallData,
                value: BigInt(0),
            },
            {
                target: lendingPoolAddress,
                callData: depositCallData,
                value: BigInt(0),
            },
        ],
        leftoverRecipient: userAddress, // Receive any leftover tokens
        gasLimit: 500000, // Higher gas limit for multiple calls
    },
});
```

### Simple Example: Transfer Tokens

For a simpler use case, transfer tokens to a recipient on the destination chain:

```typescript
import { LayerZeroGatewayClient, parseBtc } from '@gobob/bob-sdk';
import { encodeFunctionData, erc20Abi } from 'viem';

const client = new LayerZeroGatewayClient();

// Get quote to determine output amount
const initialQuote = await client.getQuote({
    fromChain: 'bitcoin',
    fromToken: 'BTC',
    toChain: 'base',
    toToken: '0x0555E30da8f98308EdB960aa94C0Db47230d2B9c',
    fromUserAddress: 'bc1q...',
    toUserAddress: '0x...',
    amount: parseBtc("0.1"),
});

const transferAmount = BigInt(initialQuote.finalOutputSats);
const recipientAddress = '0xEf7Ff7Fb24797656DF41616e807AB4016AE9dCD5';

// Encode transfer call
const transferCallData = encodeFunctionData({
    abi: erc20Abi,
    functionName: 'transfer',
    args: [recipientAddress, transferAmount],
});

// Get quote with destination call
const quote = await client.getQuote({
    fromChain: 'bitcoin',
    fromToken: 'BTC',
    toChain: 'base',
    toToken: '0x0555E30da8f98308EdB960aa94C0Db47230d2B9c',
    fromUserAddress: 'bc1q...',
    toUserAddress: '0x...',
    amount: parseBtc("0.1"),
    destinationCalls: {
        calls: [
            {
                target: '0x0555E30da8f98308EdB960aa94C0Db47230d2B9c',
                callData: transferCallData,
                value: BigInt(0),
            },
        ],
        leftoverRecipient: recipientAddress,
    },
});
```

## Frequently Asked Questions

### Who pays the gas fees?

The off-chain relayer estimates gas costs upfront and takes them as a cut from the transaction. Users don't need to hold ETH on BOB to use Gateway.

### What tokens can my strategy receive?

Your strategy can receive any wrapped BTC token supported by Gateway, including WBTC, tBTC, and other Bitcoin derivatives. Check the `tokenSent` parameter to handle different input tokens.

### How do I handle slippage protection?

For custom strategies, decode slippage parameters from the `message` field:
```solidity
uint256 minOutput = abi.decode(message, (uint256));
require(outputAmount >= minOutput, "Insufficient output");
```

### What happens if my strategy fails?

Gateway automatically falls back to sending the wrapped BTC directly to the user's EVM address. Always test your strategy thoroughly to avoid fallbacks.

### Do I need to add Bitcoin wallet support to my frontend?

Yes, your frontend needs to integrate Bitcoin wallet support to allow users to sign Bitcoin transactions. Check the [wallet guide](./wallets.md) for implementation details.

### Can I chain multiple protocols in one strategy?

Yes! Custom strategies can interact with multiple protocols in sequence. For example: stake WBTC → get staked BTC → deposit in lending protocol → send receipt tokens to user.

### How do I test my strategy?

Use Foundry to fork BOB mainnet and simulate Gateway calls to your strategy. Test with different input amounts and edge cases.

### Are there any security considerations?

- Always validate input parameters from the `message` field
- Use SafeERC20 for token transfers
- Consider reentrancy protection if interacting with external protocols
- Ensure proper access controls if your strategy has admin functions
