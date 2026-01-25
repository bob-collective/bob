# GatewayOfframpFeeBreakdown

## Properties

| Name             | Type   |
| ---------------- | ------ |
| `affiliateFee`   | string |
| `fastestFeeRate` | string |
| `inclusionFee`   | string |
| `protocolFee`    | string |
| `solverFee`      | string |

## Example

```typescript
import type { GatewayOfframpFeeBreakdown } from '';

// TODO: Update the object below with actual values
const example = {
    affiliateFee: null,
    fastestFeeRate: null,
    inclusionFee: null,
    protocolFee: null,
    solverFee: null,
} satisfies GatewayOfframpFeeBreakdown;

console.log(example);

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example);
console.log(exampleJSON);

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as GatewayOfframpFeeBreakdown;
console.log(exampleParsed);
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)
