# \SolanaApi

All URIs are relative to *https://beta.usemoon.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_solana_account**](SolanaApi.md#create_solana_account) | **POST** /solana | 
[**get_solana_account**](SolanaApi.md#get_solana_account) | **GET** /solana/{accountName} | 
[**list_solana_accounts**](SolanaApi.md#list_solana_accounts) | **GET** /solana | 
[**multi_sign_solana_transaction**](SolanaApi.md#multi_sign_solana_transaction) | **POST** /solana/{accountName}/multi-sign-tx | 
[**sign_solana_transaction**](SolanaApi.md#sign_solana_transaction) | **POST** /solana/{accountName}/sign-tx | 
[**transfer_solana_transaction**](SolanaApi.md#transfer_solana_transaction) | **POST** /solana/{accountName}/transfer | 
[**transfer_tokens_sign_solana_transaction**](SolanaApi.md#transfer_tokens_sign_solana_transaction) | **POST** /solana/{accountName}/transfer-tokens | 



## create_solana_account

> crate::models::AccountApiResponse create_solana_account(authorization, solana_input)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**solana_input** | [**SolanaInput**](SolanaInput.md) |  | [required] |

### Return type

[**crate::models::AccountApiResponse**](AccountAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_solana_account

> crate::models::AccountApiResponse get_solana_account(authorization, account_name)


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


## list_solana_accounts

> crate::models::AccountApiResponse list_solana_accounts(authorization)


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


## multi_sign_solana_transaction

> crate::models::SolanaApiResponse multi_sign_solana_transaction(authorization, account_name, solana_sign_transaction_input)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**account_name** | **String** |  | [required] |
**solana_sign_transaction_input** | [**SolanaSignTransactionInput**](SolanaSignTransactionInput.md) |  | [required] |

### Return type

[**crate::models::SolanaApiResponse**](SolanaAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sign_solana_transaction

> crate::models::SolanaApiResponse sign_solana_transaction(authorization, account_name, solana_sign_transaction_input)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**account_name** | **String** |  | [required] |
**solana_sign_transaction_input** | [**SolanaSignTransactionInput**](SolanaSignTransactionInput.md) |  | [required] |

### Return type

[**crate::models::SolanaApiResponse**](SolanaAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transfer_solana_transaction

> crate::models::SolanaApiResponse transfer_solana_transaction(authorization, account_name, solana_transaction_input)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**account_name** | **String** |  | [required] |
**solana_transaction_input** | [**SolanaTransactionInput**](SolanaTransactionInput.md) |  | [required] |

### Return type

[**crate::models::SolanaApiResponse**](SolanaAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transfer_tokens_sign_solana_transaction

> crate::models::SolanaApiResponse transfer_tokens_sign_solana_transaction(authorization, account_name, solana_transaction_input)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**account_name** | **String** |  | [required] |
**solana_transaction_input** | [**SolanaTransactionInput**](SolanaTransactionInput.md) |  | [required] |

### Return type

[**crate::models::SolanaApiResponse**](SolanaAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

