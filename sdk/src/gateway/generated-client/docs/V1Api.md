# V1Api

All URIs are relative to *https://gateway-api-mainnet.gobob.xyz*

| Method | HTTP request | Description |
|------------- | ------------- | -------------|
| [**createOrder**](V1Api.md#createorder) | **POST** /v1/create-order | Create a new gateway order. |
| [**getOrder**](V1Api.md#getorder) | **GET** /v1/get-order/{order_id} | Get all orders for a specific ID. |
| [**getOrders**](V1Api.md#getorders) | **GET** /v1/get-orders/{user_address} | Get all user orders. |
| [**getQuote**](V1Api.md#getquote) | **GET** /v1/get-quote | Get a gateway quote. |
| [**getRoutes**](V1Api.md#getroutes) | **GET** /v1/get-routes | Get all supported routes. |
| [**registerTx**](V1Api.md#registertx) | **PATCH** /v1/register-tx | Register a tx for a request. |



## createOrder

> GatewayCreateOrder createOrder(gatewayQuote)

Create a new gateway order.

Creates a new request, reserves the required liquidity.

### Example

```ts
import {
  Configuration,
  V1Api,
} from '';
import type { CreateOrderRequest } from '';

async function example() {
  console.log("🚀 Testing  SDK...");
  const api = new V1Api();

  const body = {
    // GatewayQuote
    gatewayQuote: ...,
  } satisfies CreateOrderRequest;

  try {
    const data = await api.createOrder(body);
    console.log(data);
  } catch (error) {
    console.error(error);
  }
}

// Run the test
example().catch(console.error);
```

### Parameters


| Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **gatewayQuote** | [GatewayQuote](GatewayQuote.md) |  | |

### Return type

[**GatewayCreateOrder**](GatewayCreateOrder.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: `application/json`
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **201** | Created order |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getOrder

> GatewayOrderInfo getOrder(orderId)

Get all orders for a specific ID.

### Example

```ts
import {
  Configuration,
  V1Api,
} from '';
import type { GetOrderRequest } from '';

async function example() {
  console.log("🚀 Testing  SDK...");
  const api = new V1Api();

  const body = {
    // string | Order id as returned by create-order
    orderId: orderId_example,
  } satisfies GetOrderRequest;

  try {
    const data = await api.getOrder(body);
    console.log(data);
  } catch (error) {
    console.error(error);
  }
}

// Run the test
example().catch(console.error);
```

### Parameters


| Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **orderId** | `string` | Order id as returned by create-order | [Defaults to `undefined`] |

### Return type

[**GatewayOrderInfo**](GatewayOrderInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Get order by id |  -  |
| **404** | Order not found |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getOrders

> Array&lt;GatewayOrderInfo&gt; getOrders(userAddress)

Get all user orders.

### Example

```ts
import {
  Configuration,
  V1Api,
} from '';
import type { GetOrdersRequest } from '';

async function example() {
  console.log("🚀 Testing  SDK...");
  const api = new V1Api();

  const body = {
    // string | User address
    userAddress: userAddress_example,
  } satisfies GetOrdersRequest;

  try {
    const data = await api.getOrders(body);
    console.log(data);
  } catch (error) {
    console.error(error);
  }
}

// Run the test
example().catch(console.error);
```

### Parameters


| Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **userAddress** | `string` | User address | [Defaults to `undefined`] |

### Return type

[**Array&lt;GatewayOrderInfo&gt;**](GatewayOrderInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Get all orders |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getQuote

> GatewayQuote getQuote(srcChain, dstChain, recipient, srcToken, dstToken, amount, slippage, sender, gasRefill, strategyTarget, strategyMessage, affiliateId)

Get a gateway quote.

Checks the available liquidity and provides a quote.

### Example

```ts
import {
  Configuration,
  V1Api,
} from '';
import type { GetQuoteRequest } from '';

async function example() {
  console.log("🚀 Testing  SDK...");
  const api = new V1Api();

  const body = {
    // string | Source chain
    srcChain: bitcoin,
    // string | Destination chain
    dstChain: bob,
    // string | Address receiving the funds
    recipient: 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266,
    // string | Source token address
    srcToken: 0x0000000000000000000000000000000000000000,
    // string | Destination token address
    dstToken: 0x0555E30da8f98308EdB960aa94C0Db47230d2B9c,
    // string | Amount to swap (in smallest unit, e.g., sats or wei)
    amount: 100000,
    // string | Slippage tolerance (in basis points, e.g., 300 = 3%)
    slippage: 300,
    // string | Address sending the funds (optional)
    sender: bc1qyhc4uslh46axl553pq3mjclrt7dcgmlzxv0ktx,
    // string | Optional gas refill amount (in smallest unit, e.g., wei) (optional)
    gasRefill: gasRefill_example,
    // string (optional)
    strategyTarget: strategyTarget_example,
    // string (optional)
    strategyMessage: strategyMessage_example,
    // string | Optional affiliate ID (optional)
    affiliateId: affiliateId_example,
  } satisfies GetQuoteRequest;

  try {
    const data = await api.getQuote(body);
    console.log(data);
  } catch (error) {
    console.error(error);
  }
}

// Run the test
example().catch(console.error);
```

### Parameters


| Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **srcChain** | `string` | Source chain | [Defaults to `undefined`] |
| **dstChain** | `string` | Destination chain | [Defaults to `undefined`] |
| **recipient** | `string` | Address receiving the funds | [Defaults to `undefined`] |
| **srcToken** | `string` | Source token address | [Defaults to `undefined`] |
| **dstToken** | `string` | Destination token address | [Defaults to `undefined`] |
| **amount** | `string` | Amount to swap (in smallest unit, e.g., sats or wei) | [Defaults to `undefined`] |
| **slippage** | `string` | Slippage tolerance (in basis points, e.g., 300 &#x3D; 3%) | [Defaults to `undefined`] |
| **sender** | `string` | Address sending the funds | [Optional] [Defaults to `undefined`] |
| **gasRefill** | `string` | Optional gas refill amount (in smallest unit, e.g., wei) | [Optional] [Defaults to `undefined`] |
| **strategyTarget** | `string` |  | [Optional] [Defaults to `undefined`] |
| **strategyMessage** | `string` |  | [Optional] [Defaults to `undefined`] |
| **affiliateId** | `string` | Optional affiliate ID | [Optional] [Defaults to `undefined`] |

### Return type

[**GatewayQuote**](GatewayQuote.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Get a quote |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getRoutes

> Array&lt;RouteInfo&gt; getRoutes()

Get all supported routes.

### Example

```ts
import {
  Configuration,
  V1Api,
} from '';
import type { GetRoutesRequest } from '';

async function example() {
  console.log("🚀 Testing  SDK...");
  const api = new V1Api();

  try {
    const data = await api.getRoutes();
    console.log(data);
  } catch (error) {
    console.error(error);
  }
}

// Run the test
example().catch(console.error);
```

### Parameters

This endpoint does not need any parameter.

### Return type

[**Array&lt;RouteInfo&gt;**](RouteInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Get all supported routes |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## registerTx

> RegisterTxSuccess registerTx(registerTx)

Register a tx for a request.

Required for the Solver to track and execute some requests.

### Example

```ts
import {
  Configuration,
  V1Api,
} from '';
import type { RegisterTxRequest } from '';

async function example() {
  console.log("🚀 Testing  SDK...");
  const api = new V1Api();

  const body = {
    // RegisterTx
    registerTx: ...,
  } satisfies RegisterTxRequest;

  try {
    const data = await api.registerTx(body);
    console.log(data);
  } catch (error) {
    console.error(error);
  }
}

// Run the test
example().catch(console.error);
```

### Parameters


| Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **registerTx** | [RegisterTx](RegisterTx.md) |  | |

### Return type

[**RegisterTxSuccess**](RegisterTxSuccess.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: `application/json`
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Register successful |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)

