
# GatewayCreateOrder

Created new Gateway order

## Properties

Name | Type
------------ | -------------
`onramp` | [GatewayCreateOrderOneOfOnramp](GatewayCreateOrderOneOfOnramp.md)
`offramp` | [GatewayCreateOrderOneOf1Offramp](GatewayCreateOrderOneOf1Offramp.md)
`layerZero` | [GatewayCreateOrderOneOf1Offramp](GatewayCreateOrderOneOf1Offramp.md)

## Example

```typescript
import type { GatewayCreateOrder } from ''

// TODO: Update the object below with actual values
const example = {
  "onramp": null,
  "offramp": null,
  "layerZero": null,
} satisfies GatewayCreateOrder

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as GatewayCreateOrder
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


