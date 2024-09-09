# \AccountsApi

All URIs are relative to *https://beta.usemoon.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**broadcast_tx**](AccountsApi.md#broadcast_tx) | **POST** /accounts/{accountName}/broadcast-tx | 
[**create_account**](AccountsApi.md#create_account) | **POST** /accounts | 
[**delete_account**](AccountsApi.md#delete_account) | **DELETE** /accounts/{accountName} | 
[**deploy_contract**](AccountsApi.md#deploy_contract) | **POST** /accounts/{accountName}/deploy | 
[**encode_data**](AccountsApi.md#encode_data) | **POST** /accounts/encode-data | 
[**estimate_gas**](AccountsApi.md#estimate_gas) | **POST** /accounts/{accountName}/estimate | 
[**get_account**](AccountsApi.md#get_account) | **GET** /accounts/{accountName} | 
[**get_balance**](AccountsApi.md#get_balance) | **GET** /accounts/{accountName}/balance | 
[**get_nonce**](AccountsApi.md#get_nonce) | **GET** /accounts/{accountName}/nonce | 
[**list_accounts**](AccountsApi.md#list_accounts) | **GET** /accounts | 
[**sign_message**](AccountsApi.md#sign_message) | **POST** /accounts/{accountName}/sign-message | 
[**sign_transaction**](AccountsApi.md#sign_transaction) | **POST** /accounts/{accountName}/sign-transaction | 
[**sign_typed_data**](AccountsApi.md#sign_typed_data) | **POST** /accounts/{accountName}/sign-typed-data | 
[**suggest_gas_price**](AccountsApi.md#suggest_gas_price) | **GET** /accounts/{accountName}/suggest-gas | 
[**transfer_eth**](AccountsApi.md#transfer_eth) | **POST** /accounts/{accountName}/transfer-eth | 



## broadcast_tx

> crate::models::BroadCastRawTransactionApiResponse broadcast_tx(authorization, account_name, broadcast_input)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**account_name** | **String** |  | [required] |
**broadcast_input** | [**BroadcastInput**](BroadcastInput.md) |  | [required] |

### Return type

[**crate::models::BroadCastRawTransactionApiResponse**](BroadCastRawTransactionAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_account

> crate::models::AccountApiResponse create_account(authorization, create_account_input)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**create_account_input** | [**CreateAccountInput**](CreateAccountInput.md) |  | [required] |

### Return type

[**crate::models::AccountApiResponse**](AccountAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_account

> crate::models::AccountApiResponse delete_account(authorization, account_name)


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


## deploy_contract

> crate::models::TransactionApiResponse deploy_contract(authorization, account_name, deploy_input)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**account_name** | **String** |  | [required] |
**deploy_input** | [**DeployInput**](DeployInput.md) |  | [required] |

### Return type

[**crate::models::TransactionApiResponse**](TransactionAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## encode_data

> crate::models::AbiEncodeOutput encode_data(authorization, abi_encode_input)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**abi_encode_input** | [**AbiEncodeInput**](AbiEncodeInput.md) |  | [required] |

### Return type

[**crate::models::AbiEncodeOutput**](AbiEncodeOutput.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## estimate_gas

> crate::models::TransactionApiResponse estimate_gas(account_name, authorization, input_body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_name** | **String** |  | [required] |
**authorization** | **String** |  | [required] |
**input_body** | [**InputBody**](InputBody.md) |  | [required] |

### Return type

[**crate::models::TransactionApiResponse**](TransactionAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_account

> crate::models::AccountApiResponse get_account(authorization, account_name)


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


## get_balance

> crate::models::BalanceApiResponse get_balance(account_name, authorization, chain_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_name** | **String** |  | [required] |
**authorization** | **String** |  | [required] |
**chain_id** | **String** |  | [required] |

### Return type

[**crate::models::BalanceApiResponse**](BalanceAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_nonce

> crate::models::NonceApiResponse get_nonce(account_name, authorization)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_name** | **String** |  | [required] |
**authorization** | **String** |  | [required] |

### Return type

[**crate::models::NonceApiResponse**](NonceAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_accounts

> crate::models::AccountApiResponse list_accounts(authorization)


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


## sign_message

> crate::models::SignMessageApiResponse sign_message(account_name, authorization, sign_message)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_name** | **String** |  | [required] |
**authorization** | **String** |  | [required] |
**sign_message** | [**SignMessage**](SignMessage.md) |  | [required] |

### Return type

[**crate::models::SignMessageApiResponse**](SignMessageAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sign_transaction

> crate::models::TransactionApiResponse sign_transaction(account_name, authorization, input_body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_name** | **String** |  | [required] |
**authorization** | **String** |  | [required] |
**input_body** | [**InputBody**](InputBody.md) |  | [required] |

### Return type

[**crate::models::TransactionApiResponse**](TransactionAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sign_typed_data

> crate::models::SignMessageApiResponse sign_typed_data(account_name, authorization, sign_typed_data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_name** | **String** |  | [required] |
**authorization** | **String** |  | [required] |
**sign_typed_data** | [**SignTypedData**](SignTypedData.md) |  | [required] |

### Return type

[**crate::models::SignMessageApiResponse**](SignMessageAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## suggest_gas_price

> crate::models::TransactionApiResponse suggest_gas_price(account_name, authorization, chain_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_name** | **String** |  | [required] |
**authorization** | **String** |  | [required] |
**chain_id** | **String** |  | [required] |

### Return type

[**crate::models::TransactionApiResponse**](TransactionAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transfer_eth

> crate::models::TransactionApiResponse transfer_eth(account_name, authorization, input_body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_name** | **String** |  | [required] |
**authorization** | **String** |  | [required] |
**input_body** | [**InputBody**](InputBody.md) |  | [required] |

### Return type

[**crate::models::TransactionApiResponse**](TransactionAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

