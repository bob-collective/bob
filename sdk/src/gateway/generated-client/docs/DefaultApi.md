# DefaultApi

All URIs are relative to *http://localhost*

| Method | HTTP request | Description |
|------------- | ------------- | -------------|
| [**createOrder**](DefaultApi.md#createorder) | **POST** /api/create-order | Create a new gateway order. |
| [**getOrder**](DefaultApi.md#getorder) | **GET** /api/get-order/{order_id} | Get all user orders. |
| [**getOrders**](DefaultApi.md#getorders) | **GET** /api/get-orders/{user_address} | Get all user orders. |
| [**getQuote**](DefaultApi.md#getquote) | **GET** /api/get-quote | Get a gateway quote. |
| [**getRoutes**](DefaultApi.md#getroutes) | **GET** /api/get-routes | Get all supported routes. |
| [**registerTx**](DefaultApi.md#registertx) | **PATCH** /api/register-tx | Register a tx for a request. |



## createOrder

> GatewayCreateOrder createOrder(gatewayQuote)

Create a new gateway order.

Creates a new request, reserves the required liquidity.

### Example

```ts
import {
  Configuration,
  DefaultApi,
} from '';
import type { CreateOrderRequest } from '';

async function example() {
  console.log("🚀 Testing  SDK...");
  const api = new DefaultApi();

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

Get all user orders.

### Example

```ts
import {
  Configuration,
  DefaultApi,
} from '';
import type { GetOrderRequest } from '';

async function example() {
  console.log("🚀 Testing  SDK...");
  const api = new DefaultApi();

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
  DefaultApi,
} from '';
import type { GetOrdersRequest } from '';

async function example() {
  console.log("🚀 Testing  SDK...");
  const api = new DefaultApi();

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

> GatewayQuote getQuote(srcChain, dstChain, sender, recipient, srcToken, dstToken, amount, slippage, gasRefill, strategyTarget, strategyMessage, affiliateId)

Get a gateway quote.

Checks the available liquidity and provides a quote.

### Example

```ts
import {
  Configuration,
  DefaultApi,
} from '';
import type { GetQuoteRequest } from '';

async function example() {
  console.log("🚀 Testing  SDK...");
  const api = new DefaultApi();

  const body = {
    // string | Source chain
    srcChain: bitcoin,
    // string | Destination chain
    dstChain: bob,
    // string
    sender: bc1qyhc4uslh46axl553pq3mjclrt7dcgmlzxv0ktx,
    // string
    recipient: 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266,
    // string
    srcToken: 0x0000000000000000000000000000000000000000,
    // string
    dstToken: 0x0555E30da8f98308EdB960aa94C0Db47230d2B9c,
    // string
    amount: 100000,
    // string
    slippage: 300,
    // string (optional)
    gasRefill: gasRefill_example,
    // string (optional)
    strategyTarget: strategyTarget_example,
    // string (optional)
    strategyMessage: strategyMessage_example,
    // string (optional)
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
| **sender** | `string` |  | [Defaults to `undefined`] |
| **recipient** | `string` |  | [Defaults to `undefined`] |
| **srcToken** | `string` |  | [Defaults to `undefined`] |
| **dstToken** | `string` |  | [Defaults to `undefined`] |
| **amount** | `string` |  | [Defaults to `undefined`] |
| **slippage** | `string` |  | [Defaults to `undefined`] |
| **gasRefill** | `string` |  | [Optional] [Defaults to `undefined`] |
| **strategyTarget** | `string` |  | [Optional] [Defaults to `undefined`] |
| **strategyMessage** | `string` |  | [Optional] [Defaults to `undefined`] |
| **affiliateId** | `string` |  | [Optional] [Defaults to `undefined`] |

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
  DefaultApi,
} from '';
import type { GetRoutesRequest } from '';

async function example() {
  console.log("🚀 Testing  SDK...");
  const api = new DefaultApi();

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
  DefaultApi,
} from '';
import type { RegisterTxRequest } from '';

async function example() {
  console.log("🚀 Testing  SDK...");
  const api = new DefaultApi();

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

