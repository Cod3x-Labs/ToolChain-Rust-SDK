# \CosmosApi

All URIs are relative to *https://beta.usemoon.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_account**](CosmosApi.md#create_account) | **POST** /cosmos/accounts | 
[**delete_account**](CosmosApi.md#delete_account) | **DELETE** /cosmos/accounts/{accountName} | 
[**list_accounts**](CosmosApi.md#list_accounts) | **GET** /cosmos/accounts | 
[**read_account**](CosmosApi.md#read_account) | **GET** /cosmos/accounts/{accountName} | 
[**sign_ibc_transfer_transaction**](CosmosApi.md#sign_ibc_transfer_transaction) | **POST** /cosmos/accounts/{accountName}/sign-ibc-transfer | 
[**sign_message**](CosmosApi.md#sign_message) | **POST** /cosmos/accounts/{accountName}/sign-message | 
[**sign_transfer_transaction**](CosmosApi.md#sign_transfer_transaction) | **POST** /cosmos/accounts/{accountName}/sign-transfer | 



## create_account

> crate::models::CosmosApiResponse create_account(authorization, create_account_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**create_account_request** | [**CreateAccountRequest**](CreateAccountRequest.md) |  | [required] |

### Return type

[**crate::models::CosmosApiResponse**](CosmosAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_account

> crate::models::CosmosApiResponse delete_account(authorization, account_name)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**account_name** | **String** |  | [required] |

### Return type

[**crate::models::CosmosApiResponse**](CosmosAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_accounts

> crate::models::CosmosApiResponse list_accounts(authorization)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |

### Return type

[**crate::models::CosmosApiResponse**](CosmosAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_account

> crate::models::CosmosApiResponse read_account(authorization, account_name)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**account_name** | **String** |  | [required] |

### Return type

[**crate::models::CosmosApiResponse**](CosmosAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sign_ibc_transfer_transaction

> crate::models::CosmosApiResponse sign_ibc_transfer_transaction(authorization, account_name, ibc_transfer_transaction_input)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**account_name** | **String** |  | [required] |
**ibc_transfer_transaction_input** | [**IbcTransferTransactionInput**](IbcTransferTransactionInput.md) |  | [required] |

### Return type

[**crate::models::CosmosApiResponse**](CosmosAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sign_message

> crate::models::CosmosApiResponse sign_message(authorization, account_name, message_input)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**account_name** | **String** |  | [required] |
**message_input** | [**MessageInput**](MessageInput.md) |  | [required] |

### Return type

[**crate::models::CosmosApiResponse**](CosmosAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sign_transfer_transaction

> crate::models::CosmosApiResponse sign_transfer_transaction(authorization, account_name, transfer_transaction_input)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**account_name** | **String** |  | [required] |
**transfer_transaction_input** | [**TransferTransactionInput**](TransferTransactionInput.md) |  | [required] |

### Return type

[**crate::models::CosmosApiResponse**](CosmosAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

