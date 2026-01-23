
# RegisterBtcTx

Gateway order to execute

## Properties

Name | Type
------------ | -------------
`bitcoinTx` | string
`id` | string

## Example

```typescript
import type { RegisterBtcTx } from ''

// TODO: Update the object below with actual values
const example = {
  "bitcoinTx": null,
  "id": 1,
} satisfies RegisterBtcTx

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as RegisterBtcTx
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


