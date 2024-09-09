# \Erc20Api

All URIs are relative to *https://beta.usemoon.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**approve**](Erc20Api.md#approve) | **POST** /erc20/{address}/approve | 
[**get_allowance**](Erc20Api.md#get_allowance) | **GET** /erc20/{address}/allowance | 
[**get_balance_of**](Erc20Api.md#get_balance_of) | **GET** /erc20/{address}/balanceOf | 
[**get_decimals**](Erc20Api.md#get_decimals) | **GET** /erc20/{address}/decimals | 
[**get_name**](Erc20Api.md#get_name) | **GET** /erc20/{address}/name | 
[**get_symbol**](Erc20Api.md#get_symbol) | **GET** /erc20/{address}/symbol | 
[**get_total_supply**](Erc20Api.md#get_total_supply) | **GET** /erc20/{address}/totalSupply | 
[**transfer**](Erc20Api.md#transfer) | **POST** /erc20/{address}/transfer | 
[**transfer_from**](Erc20Api.md#transfer_from) | **POST** /erc20/{address}/transferFrom | 



## approve

> crate::models::Erc20ApiResponse approve(address, authorization, input_body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** |  | [required] |
**authorization** | **String** |  | [required] |
**input_body** | [**InputBody**](InputBody.md) |  | [required] |

### Return type

[**crate::models::Erc20ApiResponse**](ERC20APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_allowance

> crate::models::Erc20ApiResponse get_allowance(address, authorization, chain_id, owner, spender)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** |  | [required] |
**authorization** | **String** |  | [required] |
**chain_id** | **String** |  | [required] |
**owner** | **String** |  | [required] |
**spender** | **String** |  | [required] |

### Return type

[**crate::models::Erc20ApiResponse**](ERC20APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_balance_of

> crate::models::Erc20ApiResponse get_balance_of(address, authorization, chain_id, account)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** |  | [required] |
**authorization** | **String** |  | [required] |
**chain_id** | **String** |  | [required] |
**account** | **String** |  | [required] |

### Return type

[**crate::models::Erc20ApiResponse**](ERC20APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_decimals

> crate::models::Erc20ApiResponse get_decimals(address, authorization, chain_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** |  | [required] |
**authorization** | **String** |  | [required] |
**chain_id** | **String** |  | [required] |

### Return type

[**crate::models::Erc20ApiResponse**](ERC20APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_name

> crate::models::Erc20ApiResponse get_name(address, authorization, chain_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** |  | [required] |
**authorization** | **String** |  | [required] |
**chain_id** | **String** |  | [required] |

### Return type

[**crate::models::Erc20ApiResponse**](ERC20APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_symbol

> crate::models::Erc20ApiResponse get_symbol(address, authorization, chain_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** |  | [required] |
**authorization** | **String** |  | [required] |
**chain_id** | **String** |  | [required] |

### Return type

[**crate::models::Erc20ApiResponse**](ERC20APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_total_supply

> crate::models::Erc20ApiResponse get_total_supply(address, authorization, chain_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** |  | [required] |
**authorization** | **String** |  | [required] |
**chain_id** | **String** |  | [required] |

### Return type

[**crate::models::Erc20ApiResponse**](ERC20APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transfer

> crate::models::Erc20ApiResponse transfer(address, authorization, input_body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** |  | [required] |
**authorization** | **String** |  | [required] |
**input_body** | [**InputBody**](InputBody.md) |  | [required] |

### Return type

[**crate::models::Erc20ApiResponse**](ERC20APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transfer_from

> crate::models::Erc20ApiResponse transfer_from(address, authorization, input_body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** |  | [required] |
**authorization** | **String** |  | [required] |
**input_body** | [**InputBody**](InputBody.md) |  | [required] |

### Return type

[**crate::models::Erc20ApiResponse**](ERC20APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

