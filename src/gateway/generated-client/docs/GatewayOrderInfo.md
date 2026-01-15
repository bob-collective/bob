
# GatewayOrderInfo

Gateway order info

## Properties

Name | Type
------------ | -------------
`onramp` | [GatewayOrderInfoOneOfOnramp](GatewayOrderInfoOneOfOnramp.md)
`offramp` | [GatewayOrderInfoOneOf1Offramp](GatewayOrderInfoOneOf1Offramp.md)
`layerZero` | [GatewayOrderInfoOneOf2LayerZero](GatewayOrderInfoOneOf2LayerZero.md)

## Example

```typescript
import type { GatewayOrderInfo } from ''

// TODO: Update the object below with actual values
const example = {
  "onramp": null,
  "offramp": null,
  "layerZero": null,
} satisfies GatewayOrderInfo

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as GatewayOrderInfo
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


