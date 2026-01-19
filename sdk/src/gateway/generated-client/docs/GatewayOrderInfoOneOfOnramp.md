# GatewayOrderInfoOneOfOnramp

## Properties

| Name                   | Type                            |
| ---------------------- | ------------------------------- |
| `amount`               | string                          |
| `bitcoinExplorerUrl`   | string                          |
| `bobExplorerUrl`       | string                          |
| `destinationChain`     | [ChainDetails](ChainDetails.md) |
| `done`                 | boolean                         |
| `estimatedTimeInSecs`  | number                          |
| `feeInSats`            | string                          |
| `layerzeroExplorerUrl` | string                          |
| `orderId`              | string                          |
| `sourceChain`          | [ChainDetails](ChainDetails.md) |
| `status`               | [OnrampStatus](OnrampStatus.md) |
| `strategyAddress`      | string                          |
| `timestamp`            | number                          |

## Example

```typescript
import type { GatewayOrderInfoOneOfOnramp } from '';

// TODO: Update the object below with actual values
const example = {
    amount: null,
    bitcoinExplorerUrl: null,
    bobExplorerUrl: null,
    destinationChain: null,
    done: null,
    estimatedTimeInSecs: null,
    feeInSats: null,
    layerzeroExplorerUrl: null,
    orderId: null,
    sourceChain: null,
    status: null,
    strategyAddress: null,
    timestamp: null,
} satisfies GatewayOrderInfoOneOfOnramp;

console.log(example);

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example);
console.log(exampleJSON);

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as GatewayOrderInfoOneOfOnramp;
console.log(exampleParsed);
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)
