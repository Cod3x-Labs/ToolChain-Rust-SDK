# \UserOpApi

All URIs are relative to *https://beta.usemoon.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_transaction**](UserOpApi.md#add_transaction) | **POST** /userop/{accountName}/add-transaction | 
[**bundle_transactions**](UserOpApi.md#bundle_transactions) | **POST** /userop/{accountName}/bundle-transactions | 
[**send_bundled_user_operations**](UserOpApi.md#send_bundled_user_operations) | **POST** /userop/{accountName}/send-bundled | 
[**send_user_operation**](UserOpApi.md#send_user_operation) | **POST** /userop/{accountName}/send | 



## add_transaction

> crate::models::BaseApiResponse add_transaction(account_name, authorization, add_transaction_input)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_name** | **String** |  | [required] |
**authorization** | **String** |  | [required] |
**add_transaction_input** | [**AddTransactionInput**](AddTransactionInput.md) |  | [required] |

### Return type

[**crate::models::BaseApiResponse**](BaseAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bundle_transactions

> crate::models::BaseApiResponse bundle_transactions(account_name, authorization, bundle_transactions_input)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_name** | **String** |  | [required] |
**authorization** | **String** |  | [required] |
**bundle_transactions_input** | [**BundleTransactionsInput**](BundleTransactionsInput.md) |  | [required] |

### Return type

[**crate::models::BaseApiResponse**](BaseAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_bundled_user_operations

> crate::models::TransactionApiResponse send_bundled_user_operations(account_name, authorization, send_bundled_input)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_name** | **String** |  | [required] |
**authorization** | **String** |  | [required] |
**send_bundled_input** | [**SendBundledInput**](SendBundledInput.md) |  | [required] |

### Return type

[**crate::models::TransactionApiResponse**](TransactionAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_user_operation

> crate::models::TransactionApiResponse send_user_operation(account_name, authorization, send_user_op_input)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_name** | **String** |  | [required] |
**authorization** | **String** |  | [required] |
**send_user_op_input** | [**SendUserOpInput**](SendUserOpInput.md) |  | [required] |

### Return type

[**crate::models::TransactionApiResponse**](TransactionAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

