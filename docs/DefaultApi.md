# \DefaultApi

All URIs are relative to *https://beta.usemoon.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**fetch_token_details**](DefaultApi.md#fetch_token_details) | **GET** /lifi/token | 
[**fetch_tokens**](DefaultApi.md#fetch_tokens) | **GET** /lifi/tokens | 
[**get_all_possible_connections**](DefaultApi.md#get_all_possible_connections) | **GET** /lifi/allPossibleConnections | 
[**get_chains**](DefaultApi.md#get_chains) | **GET** /lifi/chains | 
[**get_connections**](DefaultApi.md#get_connections) | **GET** /lifi/connections | 
[**get_gas_price**](DefaultApi.md#get_gas_price) | **GET** /thorswap/gasPrice | 
[**get_message**](DefaultApi.md#get_message) | **GET** /ping | 
[**get_quote**](DefaultApi.md#get_quote) | **GET** /lifi/quote | 
[**get_quote_0**](DefaultApi.md#get_quote_0) | **GET** /thorswap/quote | 
[**get_supported_chains**](DefaultApi.md#get_supported_chains) | **GET** /thorswap/chains | 
[**get_supported_providers**](DefaultApi.md#get_supported_providers) | **GET** /thorswap/providers | 
[**get_tools**](DefaultApi.md#get_tools) | **GET** /lifi/tools | 
[**getstatus**](DefaultApi.md#getstatus) | **GET** /lifi/status | 
[**post_quote**](DefaultApi.md#post_quote) | **POST** /lifi/{accountName}/quote | 
[**post_quote_0**](DefaultApi.md#post_quote_0) | **POST** /thorswap/{accountName}/quote | 



## fetch_token_details

> crate::models::ApiResponseTokenDetails fetch_token_details(chain, token)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chain** | **String** |  | [required] |
**token** | **String** |  | [required] |

### Return type

[**crate::models::ApiResponseTokenDetails**](ApiResponse_TokenDetails_.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_tokens

> crate::models::ApiResponseTokensResponse fetch_tokens()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ApiResponseTokensResponse**](ApiResponse_TokensResponse_.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_possible_connections

> crate::models::ApiResponseTokenInfoByChainId get_all_possible_connections(to_chain, to_token)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**to_chain** | **String** |  | [required] |
**to_token** | **String** |  | [required] |

### Return type

[**crate::models::ApiResponseTokenInfoByChainId**](ApiResponse_TokenInfoByChainId_.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_chains

> crate::models::ApiResponseChainsResponse get_chains(optional_chain_types)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**optional_chain_types** | Option<**String**> |  |  |

### Return type

[**crate::models::ApiResponseChainsResponse**](ApiResponse_ChainsResponse_.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_connections

> crate::models::ApiResponseConnectionsResponse get_connections(from_chain, to_chain, from_token, to_token, chain_types)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**from_chain** | **String** |  | [required] |
**to_chain** | **String** |  | [required] |
**from_token** | **String** |  | [required] |
**to_token** | **String** |  | [required] |
**chain_types** | Option<**String**> |  |  |

### Return type

[**crate::models::ApiResponseConnectionsResponse**](ApiResponse_ConnectionsResponse_.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_gas_price

> crate::models::ApiResponseGasPrice get_gas_price(chain_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chain_id** | **String** |  | [required] |

### Return type

[**crate::models::ApiResponseGasPrice**](ApiResponse_GasPrice_.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_message

> crate::models::PingResponse get_message()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::PingResponse**](PingResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_quote

> crate::models::ApiResponseQuote get_quote(from_chain, to_chain, from_token, to_token, from_amount, from_address, to_address, order, slippage, integrator, fee, referrer, allow_bridges, allow_exchanges, deny_bridges, deny_exchanges, prefer_bridges, prefer_exchanges)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**from_chain** | **String** |  | [required] |
**to_chain** | **String** |  | [required] |
**from_token** | **String** |  | [required] |
**to_token** | **String** |  | [required] |
**from_amount** | **String** |  | [required] |
**from_address** | **String** |  | [required] |
**to_address** | Option<**String**> |  |  |
**order** | Option<**String**> |  |  |
**slippage** | Option<**f64**> |  |  |
**integrator** | Option<**String**> |  |  |
**fee** | Option<**f64**> |  |  |
**referrer** | Option<**String**> |  |  |
**allow_bridges** | Option<[**Vec<String>**](String.md)> |  |  |
**allow_exchanges** | Option<[**Vec<String>**](String.md)> |  |  |
**deny_bridges** | Option<[**Vec<String>**](String.md)> |  |  |
**deny_exchanges** | Option<[**Vec<String>**](String.md)> |  |  |
**prefer_bridges** | Option<[**Vec<String>**](String.md)> |  |  |
**prefer_exchanges** | Option<[**Vec<String>**](String.md)> |  |  |

### Return type

[**crate::models::ApiResponseQuote**](ApiResponse_Quote_.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_quote_0

> crate::models::ApiResponseQuote get_quote_0(sell_asset, buy_asset, sell_amount, sender_address, recipient_address, slippage, limit, providers, preferred_provider, affiliate_address, affiliate_basis_points, allow_smart_contract_recipient)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sell_asset** | **String** |  | [required] |
**buy_asset** | **String** |  | [required] |
**sell_amount** | **f64** |  | [required] |
**sender_address** | **String** |  | [required] |
**recipient_address** | **String** |  | [required] |
**slippage** | Option<**f64**> |  |  |
**limit** | Option<**f64**> |  |  |
**providers** | Option<[**Vec<String>**](String.md)> |  |  |
**preferred_provider** | Option<**String**> |  |  |
**affiliate_address** | Option<**String**> |  |  |
**affiliate_basis_points** | Option<**f64**> |  |  |
**allow_smart_contract_recipient** | Option<**bool**> |  |  |

### Return type

[**crate::models::ApiResponseQuote**](ApiResponse_Quote_.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_supported_chains

> crate::models::ApiResponseChainMap get_supported_chains()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ApiResponseChainMap**](ApiResponse_ChainMap_.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_supported_providers

> crate::models::ApiResponseStringArray get_supported_providers()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ApiResponseStringArray**](ApiResponse_string-Array_.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tools

> crate::models::ApiResponseToolsResponse get_tools(chains)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chains** | Option<**String**> |  |  |

### Return type

[**crate::models::ApiResponseToolsResponse**](ApiResponse_ToolsResponse_.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## getstatus

> crate::models::ApiResponseStatusResponse getstatus(tx_hash)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tx_hash** | **String** |  | [required] |

### Return type

[**crate::models::ApiResponseStatusResponse**](ApiResponse_StatusResponse_.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_quote

> crate::models::ApiResponsePostQuote post_quote(account_name, authorization, from_chain, to_chain, from_token, to_token, from_amount, from_address, to_address, order, slippage, integrator, fee, referrer, allow_bridges, allow_exchanges, deny_bridges, deny_exchanges, prefer_bridges, prefer_exchanges)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_name** | **String** |  | [required] |
**authorization** | **String** |  | [required] |
**from_chain** | **String** |  | [required] |
**to_chain** | **String** |  | [required] |
**from_token** | **String** |  | [required] |
**to_token** | **String** |  | [required] |
**from_amount** | **String** |  | [required] |
**from_address** | **String** |  | [required] |
**to_address** | Option<**String**> |  |  |
**order** | Option<**String**> |  |  |
**slippage** | Option<**f64**> |  |  |
**integrator** | Option<**String**> |  |  |
**fee** | Option<**f64**> |  |  |
**referrer** | Option<**String**> |  |  |
**allow_bridges** | Option<[**Vec<String>**](String.md)> |  |  |
**allow_exchanges** | Option<[**Vec<String>**](String.md)> |  |  |
**deny_bridges** | Option<[**Vec<String>**](String.md)> |  |  |
**deny_exchanges** | Option<[**Vec<String>**](String.md)> |  |  |
**prefer_bridges** | Option<[**Vec<String>**](String.md)> |  |  |
**prefer_exchanges** | Option<[**Vec<String>**](String.md)> |  |  |

### Return type

[**crate::models::ApiResponsePostQuote**](ApiResponse_PostQuote_.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_quote_0

> crate::models::ApiResponseQuote post_quote_0(sell_asset, buy_asset, sell_amount, sender_address, recipient_address, slippage, limit, providers, preferred_provider, affiliate_address, affiliate_basis_points, allow_smart_contract_recipient)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sell_asset** | **String** |  | [required] |
**buy_asset** | **String** |  | [required] |
**sell_amount** | **f64** |  | [required] |
**sender_address** | **String** |  | [required] |
**recipient_address** | **String** |  | [required] |
**slippage** | Option<**f64**> |  |  |
**limit** | Option<**f64**> |  |  |
**providers** | Option<[**Vec<String>**](String.md)> |  |  |
**preferred_provider** | Option<**String**> |  |  |
**affiliate_address** | Option<**String**> |  |  |
**affiliate_basis_points** | Option<**f64**> |  |  |
**allow_smart_contract_recipient** | Option<**bool**> |  |  |

### Return type

[**crate::models::ApiResponseQuote**](ApiResponse_Quote_.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

