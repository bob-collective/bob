
# GatewayOrderStatus


## Properties

Name | Type
------------ | -------------
`inProgress` | [GatewayOrderStatusOneOfInProgress](GatewayOrderStatusOneOfInProgress.md)
`failed` | [GatewayOrderStatusOneOf1Failed](GatewayOrderStatusOneOf1Failed.md)

## Example

```typescript
import type { GatewayOrderStatus } from ''

// TODO: Update the object below with actual values
const example = {
  "inProgress": null,
  "failed": null,
} satisfies GatewayOrderStatus

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as GatewayOrderStatus
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


