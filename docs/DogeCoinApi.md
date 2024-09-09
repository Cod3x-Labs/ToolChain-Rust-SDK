# \DogeCoinApi

All URIs are relative to *https://beta.usemoon.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_doge_coin_account**](DogeCoinApi.md#create_doge_coin_account) | **POST** /dogecoin | 
[**get_doge_coin_account**](DogeCoinApi.md#get_doge_coin_account) | **GET** /dogecoin/{accountName} | 
[**list_doge_coin_accounts**](DogeCoinApi.md#list_doge_coin_accounts) | **GET** /dogecoin | 
[**sign_doge_coin_transaction**](DogeCoinApi.md#sign_doge_coin_transaction) | **POST** /dogecoin/{accountName}/sign-tx | 



## create_doge_coin_account

> crate::models::AccountApiResponse create_doge_coin_account(authorization, doge_coin_input)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**doge_coin_input** | [**DogeCoinInput**](DogeCoinInput.md) |  | [required] |

### Return type

[**crate::models::AccountApiResponse**](AccountAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_doge_coin_account

> crate::models::AccountApiResponse get_doge_coin_account(authorization, account_name)


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


## list_doge_coin_accounts

> crate::models::AccountApiResponse list_doge_coin_accounts(authorization)


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


## sign_doge_coin_transaction

> crate::models::DogeCoinApiResponse sign_doge_coin_transaction(authorization, account_name, doge_coin_transaction_input)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**account_name** | **String** |  | [required] |
**doge_coin_transaction_input** | [**DogeCoinTransactionInput**](DogeCoinTransactionInput.md) |  | [required] |

### Return type

[**crate::models::DogeCoinApiResponse**](DogeCoinAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

