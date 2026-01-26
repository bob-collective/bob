
# GatewayOrderInfoOneOf1Offramp


## Properties

Name | Type
------------ | -------------
`amount` | string
`bitcoinExplorerUrl` | string
`bobExplorerUrl` | string
`bumpFeeTx` | [TxInfo](TxInfo.md)
`dstInfo` | [ChainTxInfo](ChainTxInfo.md)
`estimatedTimeInSecs` | number
`fees` | string
`offrampRegistryAddress` | string
`orderId` | string
`refundOrderTx` | [TxInfo](TxInfo.md)
`srcInfo` | [ChainTxInfo](ChainTxInfo.md)
`status` | [OrderStatus](OrderStatus.md)
`timestamp` | number

## Example

```typescript
import type { GatewayOrderInfoOneOf1Offramp } from ''

// TODO: Update the object below with actual values
const example = {
  "amount": null,
  "bitcoinExplorerUrl": null,
  "bobExplorerUrl": null,
  "bumpFeeTx": null,
  "dstInfo": null,
  "estimatedTimeInSecs": null,
  "fees": null,
  "offrampRegistryAddress": null,
  "orderId": null,
  "refundOrderTx": null,
  "srcInfo": null,
  "status": null,
  "timestamp": null,
} satisfies GatewayOrderInfoOneOf1Offramp

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as GatewayOrderInfoOneOf1Offramp
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


