
# GatewayOrderInfoOneOfOnramp


## Properties

Name | Type
------------ | -------------
`amount` | string
`bitcoinExplorerUrl` | string
`bobExplorerUrl` | string
`dstInfo` | [ChainTxInfo](ChainTxInfo.md)
`estimatedTimeInSecs` | number
`fees` | string
`layerzeroExplorerUrl` | string
`orderId` | string
`srcInfo` | [ChainTxInfo](ChainTxInfo.md)
`status` | [OnrampStatus](OnrampStatus.md)
`timestamp` | number

## Example

```typescript
import type { GatewayOrderInfoOneOfOnramp } from ''

// TODO: Update the object below with actual values
const example = {
  "amount": null,
  "bitcoinExplorerUrl": null,
  "bobExplorerUrl": null,
  "dstInfo": null,
  "estimatedTimeInSecs": null,
  "fees": null,
  "layerzeroExplorerUrl": null,
  "orderId": null,
  "srcInfo": null,
  "status": null,
  "timestamp": null,
} satisfies GatewayOrderInfoOneOfOnramp

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as GatewayOrderInfoOneOfOnramp
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


