# \BitcoinApi

All URIs are relative to *https://beta.usemoon.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_bitcoin_account**](BitcoinApi.md#create_bitcoin_account) | **POST** /bitcoin | 
[**get_bitcoin_account**](BitcoinApi.md#get_bitcoin_account) | **GET** /bitcoin/{accountName} | 
[**list_bitcoin_accounts**](BitcoinApi.md#list_bitcoin_accounts) | **GET** /bitcoin | 
[**sign_bitcoin_transaction**](BitcoinApi.md#sign_bitcoin_transaction) | **POST** /bitcoin/{accountName}/sign-tx | 



## create_bitcoin_account

> crate::models::AccountApiResponse create_bitcoin_account(authorization, bitcoin_input)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**bitcoin_input** | [**BitcoinInput**](BitcoinInput.md) |  | [required] |

### Return type

[**crate::models::AccountApiResponse**](AccountAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_bitcoin_account

> crate::models::AccountApiResponse get_bitcoin_account(authorization, account_name)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**account_name** | **String** |  | [required] |

### Return type

[**crate::models::AccountApiResponse**](AccountAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_bitcoin_accounts

> crate::models::AccountApiResponse list_bitcoin_accounts(authorization)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |

### Return type

[**crate::models::AccountApiResponse**](AccountAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sign_bitcoin_transaction

> crate::models::BitcoinApiResponse sign_bitcoin_transaction(authorization, account_name, bitcoin_transaction_input)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**account_name** | **String** |  | [required] |
**bitcoin_transaction_input** | [**BitcoinTransactionInput**](BitcoinTransactionInput.md) |  | [required] |

### Return type

[**crate::models::BitcoinApiResponse**](BitcoinAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

