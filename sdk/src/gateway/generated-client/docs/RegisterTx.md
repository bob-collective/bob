
# RegisterTx

Gateway order to execute

## Properties

Name | Type
------------ | -------------
`onramp` | [RegisterTxOneOfOnramp](RegisterTxOneOfOnramp.md)
`offramp` | [RegisterTxOneOf1Offramp](RegisterTxOneOf1Offramp.md)
`layerZero` | [RegisterTxOneOf1Offramp](RegisterTxOneOf1Offramp.md)

## Example

```typescript
import type { RegisterTx } from ''

// TODO: Update the object below with actual values
const example = {
  "onramp": null,
  "offramp": null,
  "layerZero": null,
} satisfies RegisterTx

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as RegisterTx
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


