# \BitcoincashApi

All URIs are relative to *https://beta.usemoon.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_bitcoin_cash_account**](BitcoincashApi.md#create_bitcoin_cash_account) | **POST** /bitcoincash | 
[**get_bitcoin_cash_account**](BitcoincashApi.md#get_bitcoin_cash_account) | **GET** /bitcoincash/{accountName} | 
[**list_bitcoin_cash_accounts**](BitcoincashApi.md#list_bitcoin_cash_accounts) | **GET** /bitcoincash | 
[**sign_bitcoin_cash_transaction**](BitcoincashApi.md#sign_bitcoin_cash_transaction) | **POST** /bitcoincash/{accountName}/sign-tx | 



## create_bitcoin_cash_account

> crate::models::AccountApiResponse create_bitcoin_cash_account(authorization, bitcoin_cash_input)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**bitcoin_cash_input** | [**BitcoinCashInput**](BitcoinCashInput.md) |  | [required] |

### Return type

[**crate::models::AccountApiResponse**](AccountAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_bitcoin_cash_account

> crate::models::AccountApiResponse get_bitcoin_cash_account(authorization, account_name)


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


## list_bitcoin_cash_accounts

> crate::models::AccountApiResponse list_bitcoin_cash_accounts(authorization)


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


## sign_bitcoin_cash_transaction

> crate::models::BitcoinCashApiResponse sign_bitcoin_cash_transaction(authorization, account_name, bitcoin_cash_transaction_input)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**account_name** | **String** |  | [required] |
**bitcoin_cash_transaction_input** | [**BitcoinCashTransactionInput**](BitcoinCashTransactionInput.md) |  | [required] |

### Return type

[**crate::models::BitcoinCashApiResponse**](BitcoinCashAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

