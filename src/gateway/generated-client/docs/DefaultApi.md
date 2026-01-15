# DefaultApi

All URIs are relative to *http://localhost*

| Method | HTTP request | Description |
|------------- | ------------- | -------------|
| [**getChains**](DefaultApi.md#getchains) | **GET** /api/get-chains | Get all supported chains. |
| [**getOrders**](DefaultApi.md#getorders) | **GET** /api/get-orders/{user_address} | Get all user orders. |
| [**getQuote**](DefaultApi.md#getquote) | **GET** /api/get-quote | Get a gateway quote. |
| [**getReferrals**](DefaultApi.md#getreferrals) | **GET** /api/get-referrals/{user_address} | Get user referral stats. |
| [**getRoutes**](DefaultApi.md#getroutes) | **GET** /api/get-routes | Get all supported routes. |
| [**getTokens**](DefaultApi.md#gettokens) | **GET** /api/get-tokens | Get all supported tokens. |
| [**registerBtcTx**](DefaultApi.md#registerbtctx) | **PATCH** /api/register-btc-tx | Register a Bitcoin tx for an onramp request. |
| [**startOnramp**](DefaultApi.md#startonramp) | **POST** /api/start-onramp | Start a new onramp. |



## getChains

> Array&lt;string&gt; getChains()

Get all supported chains.

### Example

```ts
import {
  Configuration,
  DefaultApi,
} from '';
import type { GetChainsRequest } from '';

async function example() {
  console.log("🚀 Testing  SDK...");
  const api = new DefaultApi();

  try {
    const data = await api.getChains();
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

**Array<string>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Get all supported chains |  -  |

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


## getReferrals

> ReferralInfo getReferrals(userAddress)

Get user referral stats.

### Example

```ts
import {
  Configuration,
  DefaultApi,
} from '';
import type { GetReferralsRequest } from '';

async function example() {
  console.log("🚀 Testing  SDK...");
  const api = new DefaultApi();

  const body = {
    // string | User address
    userAddress: userAddress_example,
  } satisfies GetReferralsRequest;

  try {
    const data = await api.getReferrals(body);
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

[**ReferralInfo**](ReferralInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Get referral stats |  -  |

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


## getTokens

> Array&lt;string&gt; getTokens()

Get all supported tokens.

### Example

```ts
import {
  Configuration,
  DefaultApi,
} from '';
import type { GetTokensRequest } from '';

async function example() {
  console.log("🚀 Testing  SDK...");
  const api = new DefaultApi();

  try {
    const data = await api.getTokens();
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

**Array<string>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Get all supported tokens |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## registerBtcTx

> string registerBtcTx(registerBtcTx)

Register a Bitcoin tx for an onramp request.

Required for the Solver to track and execute an onramp request.

### Example

```ts
import {
  Configuration,
  DefaultApi,
} from '';
import type { RegisterBtcTxRequest } from '';

async function example() {
  console.log("🚀 Testing  SDK...");
  const api = new DefaultApi();

  const body = {
    // RegisterBtcTx
    registerBtcTx: ...,
  } satisfies RegisterBtcTxRequest;

  try {
    const data = await api.registerBtcTx(body);
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
| **registerBtcTx** | [RegisterBtcTx](RegisterBtcTx.md) |  | |

### Return type

**string**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: `application/json`
- **Accept**: `text/plain`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Register successful |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## startOnramp

> GatewayCreateOnramp startOnramp(gatewayOnrampQuote)

Start a new onramp.

Creates a new onramp request and reserves the required liquidity.

### Example

```ts
import {
  Configuration,
  DefaultApi,
} from '';
import type { StartOnrampRequest } from '';

async function example() {
  console.log("🚀 Testing  SDK...");
  const api = new DefaultApi();

  const body = {
    // GatewayOnrampQuote
    gatewayOnrampQuote: ...,
  } satisfies StartOnrampRequest;

  try {
    const data = await api.startOnramp(body);
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
| **gatewayOnrampQuote** | [GatewayOnrampQuote](GatewayOnrampQuote.md) |  | |

### Return type

[**GatewayCreateOnramp**](GatewayCreateOnramp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: `application/json`
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **201** | Started onramp |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)

