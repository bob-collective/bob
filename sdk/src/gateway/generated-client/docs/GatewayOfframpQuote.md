
# GatewayOfframpQuote


## Properties

Name | Type
------------ | -------------
`feeBreakdown` | [GatewayOfframpFeeBreakdown](GatewayOfframpFeeBreakdown.md)
`fees` | [GatewayTokenAmount](GatewayTokenAmount.md)
`inputAmount` | [GatewayTokenAmount](GatewayTokenAmount.md)
`outputAmount` | [GatewayTokenAmount](GatewayTokenAmount.md)
`srcChain` | string
`tokenAddress` | string
`tx` | [TxInfo](TxInfo.md)

## Example

```typescript
import type { GatewayOfframpQuote } from ''

// TODO: Update the object below with actual values
const example = {
  "feeBreakdown": null,
  "fees": null,
  "inputAmount": null,
  "outputAmount": null,
  "srcChain": null,
  "tokenAddress": null,
  "tx": null,
} satisfies GatewayOfframpQuote

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as GatewayOfframpQuote
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


