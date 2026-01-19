# GatewayQuote

Gateway quote.

## Properties

| Name        | Type                                              |
| ----------- | ------------------------------------------------- |
| `onramp`    | [GatewayOnrampQuote](GatewayOnrampQuote.md)       |
| `offramp`   | [GatewayOfframpQuote](GatewayOfframpQuote.md)     |
| `layerZero` | [GatewayLayerZeroQuote](GatewayLayerZeroQuote.md) |

## Example

```typescript
import type { GatewayQuote } from '';

// TODO: Update the object below with actual values
const example = {
    onramp: null,
    offramp: null,
    layerZero: null,
} satisfies GatewayQuote;

console.log(example);

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example);
console.log(exampleJSON);

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as GatewayQuote;
console.log(exampleParsed);
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)
