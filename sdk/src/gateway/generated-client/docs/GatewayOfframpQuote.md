
# GatewayOfframpQuote


## Properties

Name | Type
------------ | -------------
`affiliateAddress` | string
`feeBreakdown` | [GatewayOfframpFeeBreakdown](GatewayOfframpFeeBreakdown.md)
`fees` | [GatewayTokenAmount](GatewayTokenAmount.md)
`inputAmount` | [GatewayTokenAmount](GatewayTokenAmount.md)
`outputAmount` | [GatewayTokenAmount](GatewayTokenAmount.md)
`recipient` | string
`sender` | string
`slippage` | number
`srcChain` | string
`tokenAddress` | string
`txTo` | string

## Example

```typescript
import type { GatewayOfframpQuote } from ''

// TODO: Update the object below with actual values
const example = {
  "affiliateAddress": null,
  "feeBreakdown": null,
  "fees": null,
  "inputAmount": null,
  "outputAmount": null,
  "recipient": null,
  "sender": null,
  "slippage": null,
  "srcChain": null,
  "tokenAddress": null,
  "txTo": null,
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


