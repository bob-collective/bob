
# GatewayOrderInfo

Gateway order info

## Properties

Name | Type
------------ | -------------
`dstInfo` | [ChainTxInfo](ChainTxInfo.md)
`estimatedTimeInSecs` | number
`srcInfo` | [ChainTxInfo](ChainTxInfo.md)
`status` | [GatewayOrderStatus](GatewayOrderStatus.md)
`timestamp` | number

## Example

```typescript
import type { GatewayOrderInfo } from ''

// TODO: Update the object below with actual values
const example = {
  "dstInfo": null,
  "estimatedTimeInSecs": null,
  "srcInfo": null,
  "status": null,
  "timestamp": null,
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


