# \OdosApi

All URIs are relative to *https://beta.usemoon.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**assemble_transaction**](OdosApi.md#assemble_transaction) | **POST** /odos/{accountName}/assemble-transaction | 
[**get_contract_info**](OdosApi.md#get_contract_info) | **GET** /odos/{accountName}/contract-info | 
[**get_current_block**](OdosApi.md#get_current_block) | **GET** /odos/{accountName}/current-block | 
[**get_executor_address**](OdosApi.md#get_executor_address) | **GET** /odos/{accountName}/executor-address | 
[**get_liquidity_sources**](OdosApi.md#get_liquidity_sources) | **GET** /odos/{accountName}/liquidity-sources | 
[**get_quote**](OdosApi.md#get_quote) | **POST** /odos/{accountName}/get-quote | 
[**get_router_address**](OdosApi.md#get_router_address) | **GET** /odos/{accountName}/router-address | 
[**get_supported_chains**](OdosApi.md#get_supported_chains) | **GET** /odos/supported-chains | 
[**get_supported_tokens**](OdosApi.md#get_supported_tokens) | **GET** /odos/{accountName}/supported-tokens | 
[**swap**](OdosApi.md#swap) | **POST** /odos/{accountName}/swap | 



## assemble_transaction

> crate::models::OdosApiResponse assemble_transaction(account_name, authorization, assemble_transaction_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_name** | **String** |  | [required] |
**authorization** | **String** |  | [required] |
**assemble_transaction_request** | [**AssembleTransactionRequest**](AssembleTransactionRequest.md) |  | [required] |

### Return type

[**crate::models::OdosApiResponse**](OdosAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_contract_info

> crate::models::OdosApiResponse get_contract_info(account_name, authorization, version, chain_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_name** | **String** |  | [required] |
**authorization** | **String** |  | [required] |
**version** | **String** |  | [required] |
**chain_id** | **f64** |  | [required] |

### Return type

[**crate::models::OdosApiResponse**](OdosAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_current_block

> crate::models::OdosApiResponse get_current_block(account_name, authorization, chain_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_name** | **String** |  | [required] |
**authorization** | **String** |  | [required] |
**chain_id** | **f64** |  | [required] |

### Return type

[**crate::models::OdosApiResponse**](OdosAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_executor_address

> crate::models::OdosApiResponse get_executor_address(account_name, authorization, version, chain_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_name** | **String** |  | [required] |
**authorization** | **String** |  | [required] |
**version** | **String** |  | [required] |
**chain_id** | **f64** |  | [required] |

### Return type

[**crate::models::OdosApiResponse**](OdosAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_liquidity_sources

> crate::models::OdosApiResponse get_liquidity_sources(account_name, authorization, chain_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_name** | **String** |  | [required] |
**authorization** | **String** |  | [required] |
**chain_id** | **f64** |  | [required] |

### Return type

[**crate::models::OdosApiResponse**](OdosAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_quote

> crate::models::OdosApiResponse get_quote(account_name, authorization, swap_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_name** | **String** |  | [required] |
**authorization** | **String** |  | [required] |
**swap_request** | [**SwapRequest**](SwapRequest.md) |  | [required] |

### Return type

[**crate::models::OdosApiResponse**](OdosAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_router_address

> crate::models::OdosApiResponse get_router_address(account_name, authorization, version, chain_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_name** | **String** |  | [required] |
**authorization** | **String** |  | [required] |
**version** | **String** |  | [required] |
**chain_id** | **f64** |  | [required] |

### Return type

[**crate::models::OdosApiResponse**](OdosAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_supported_chains

> crate::models::OdosApiResponse get_supported_chains(authorization, account_name)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**account_name** | **String** |  | [required] |

### Return type

[**crate::models::OdosApiResponse**](OdosAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_supported_tokens

> crate::models::OdosApiResponse get_supported_tokens(account_name, authorization, chain_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_name** | **String** |  | [required] |
**authorization** | **String** |  | [required] |
**chain_id** | **f64** |  | [required] |

### Return type

[**crate::models::OdosApiResponse**](OdosAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## swap

> crate::models::OdosApiResponse swap(account_name, authorization, swap_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_name** | **String** |  | [required] |
**authorization** | **String** |  | [required] |
**swap_request** | [**SwapRequest**](SwapRequest.md) |  | [required] |

### Return type

[**crate::models::OdosApiResponse**](OdosAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

