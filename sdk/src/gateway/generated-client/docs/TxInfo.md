# TxInfo

## Properties

| Name    | Type   |
| ------- | ------ |
| `data`  | string |
| `to`    | string |
| `value` | string |

## Example

```typescript
import type { TxInfo } from '';

// TODO: Update the object below with actual values
const example = {
    data: null,
    to: null,
    value: null,
} satisfies TxInfo;

console.log(example);

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example);
console.log(exampleJSON);

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as TxInfo;
console.log(exampleParsed);
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)
