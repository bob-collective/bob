
# GatewayOnrampFeeBreakdown


## Properties

Name | Type
------------ | -------------
`affiliateFee` | [GatewayTokenAmount](GatewayTokenAmount.md)
`executionFee` | [GatewayTokenAmount](GatewayTokenAmount.md)
`layerzeroFee` | [GatewayTokenAmount](GatewayTokenAmount.md)
`protocolFee` | [GatewayTokenAmount](GatewayTokenAmount.md)
`solverFee` | [GatewayTokenAmount](GatewayTokenAmount.md)

## Example

```typescript
import type { GatewayOnrampFeeBreakdown } from ''

// TODO: Update the object below with actual values
const example = {
  "affiliateFee": null,
  "executionFee": null,
  "layerzeroFee": null,
  "protocolFee": null,
  "solverFee": null,
} satisfies GatewayOnrampFeeBreakdown

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as GatewayOnrampFeeBreakdown
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


