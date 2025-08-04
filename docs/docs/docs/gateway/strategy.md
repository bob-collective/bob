---
sidebar_position: 3
---

# On-Chain Actions

## Introduction

BOB Gateway allows Bitcoin users to interact with DeFi protocols using a single Bitcoin transaction. There are two ways to integrate:

1. **Custom Strategy Contract** - Implement a smart contract that receives wrapped BTC and executes your protocol logic
2. **Multicall Integration** - Use the built-in multicall handler to interact with existing contracts without deploying new ones

:::info Gateway Overview
For Gateway's architecture details, see the [overview](./overview.md).
:::

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

## Option 2: Using Multicall

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

## Frequently Asked Questions

### Which approach should I choose - Custom Strategy or Multicall?

- **Custom Strategy**: Choose this if you need complex logic, gas optimization, custom events, or want full control over the execution flow
- **Multicall**: Choose this if you want to integrate with existing contracts without deploying new ones, or for simple approve + deposit patterns

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
