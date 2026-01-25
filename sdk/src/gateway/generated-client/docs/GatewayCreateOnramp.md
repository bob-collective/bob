# GatewayCreateOnramp

Created new Gateway order

## Properties

| Name           | Type   |
| -------------- | ------ |
| `address`      | string |
| `id`           | string |
| `opReturnData` | string |
| `psbt`         | string |

## Example

```typescript
import type { GatewayCreateOnramp } from '';

// TODO: Update the object below with actual values
const example = {
    address: null,
    id: 1,
    opReturnData: null,
    psbt: null,
} satisfies GatewayCreateOnramp;

console.log(example);

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example);
console.log(exampleJSON);

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as GatewayCreateOnramp;
console.log(exampleParsed);
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)
