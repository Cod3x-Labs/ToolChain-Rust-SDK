# \UniSwapApi

All URIs are relative to *https://beta.usemoon.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_liquidity**](UniSwapApi.md#add_liquidity) | **POST** /uniswap/{name}/add-liquidity | 
[**remove_liquidity**](UniSwapApi.md#remove_liquidity) | **POST** /uniswap/{name}/remove-liquidity | 
[**swap_exact_eth_for_tokens**](UniSwapApi.md#swap_exact_eth_for_tokens) | **POST** /uniswap/{name}/swap-exact-eth-for-tokens | 
[**swap_exact_tokens_for_tokens**](UniSwapApi.md#swap_exact_tokens_for_tokens) | **POST** /uniswap/{name}/swap-exact-tokens-for-tokens | 



## add_liquidity

> crate::models::TransactionApiResponse add_liquidity(authorization, name, uniswap_input)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**name** | **String** |  | [required] |
**uniswap_input** | [**UniswapInput**](UniswapInput.md) |  | [required] |

### Return type

[**crate::models::TransactionApiResponse**](TransactionAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_liquidity

> crate::models::TransactionApiResponse remove_liquidity(authorization, name, uniswap_input)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**name** | **String** |  | [required] |
**uniswap_input** | [**UniswapInput**](UniswapInput.md) |  | [required] |

### Return type

[**crate::models::TransactionApiResponse**](TransactionAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## swap_exact_eth_for_tokens

> crate::models::TransactionApiResponse swap_exact_eth_for_tokens(authorization, name, uniswap_input)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**name** | **String** |  | [required] |
**uniswap_input** | [**UniswapInput**](UniswapInput.md) |  | [required] |

### Return type

[**crate::models::TransactionApiResponse**](TransactionAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## swap_exact_tokens_for_tokens

> crate::models::TransactionApiResponse swap_exact_tokens_for_tokens(authorization, name, uniswap_input)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**name** | **String** |  | [required] |
**uniswap_input** | [**UniswapInput**](UniswapInput.md) |  | [required] |

### Return type

[**crate::models::TransactionApiResponse**](TransactionAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

