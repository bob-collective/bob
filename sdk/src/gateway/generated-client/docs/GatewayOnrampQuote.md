
# GatewayOnrampQuote


## Properties

Name | Type
------------ | -------------
`affiliateAddress` | string
`dstChain` | string
`dstToken` | string
`executionFees` | [GatewayTokenAmount](GatewayTokenAmount.md)
`feeBreakdown` | [GatewayOnrampFeeBreakdown](GatewayOnrampFeeBreakdown.md)
`fees` | [GatewayTokenAmount](GatewayTokenAmount.md)
`gasRefill` | string
`inputAmount` | [GatewayTokenAmount](GatewayTokenAmount.md)
`outputAmount` | [GatewayTokenAmount](GatewayTokenAmount.md)
`recipient` | string
`sender` | string
`slippage` | string
`strategyAddress` | string
`strategyMessage` | string
`token` | string

## Example

```typescript
import type { GatewayOnrampQuote } from ''

// TODO: Update the object below with actual values
const example = {
  "affiliateAddress": null,
  "dstChain": null,
  "dstToken": null,
  "executionFees": null,
  "feeBreakdown": null,
  "fees": null,
  "gasRefill": null,
  "inputAmount": null,
  "outputAmount": null,
  "recipient": null,
  "sender": null,
  "slippage": null,
  "strategyAddress": null,
  "strategyMessage": null,
  "token": null,
} satisfies GatewayOnrampQuote

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as GatewayOnrampQuote
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


