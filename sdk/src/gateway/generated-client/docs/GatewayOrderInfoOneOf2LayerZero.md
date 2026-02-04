
# GatewayOrderInfoOneOf2LayerZero


## Properties

Name | Type
------------ | -------------
`amount` | [GatewayTokenAmount](GatewayTokenAmount.md)
`dstInfo` | [ChainTxInfo](ChainTxInfo.md)
`fees` | [GatewayTokenAmount](GatewayTokenAmount.md)
`layerzeroExplorerUrl` | string
`srcInfo` | [ChainTxInfo](ChainTxInfo.md)
`status` | [LayerZeroOrderStatus](LayerZeroOrderStatus.md)
`timestamp` | number

## Example

```typescript
import type { GatewayOrderInfoOneOf2LayerZero } from ''

// TODO: Update the object below with actual values
const example = {
  "amount": null,
  "dstInfo": null,
  "fees": null,
  "layerzeroExplorerUrl": null,
  "srcInfo": null,
  "status": null,
  "timestamp": null,
} satisfies GatewayOrderInfoOneOf2LayerZero

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as GatewayOrderInfoOneOf2LayerZero
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


