# \PaymentApi

All URIs are relative to *https://beta.usemoon.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_payment_intent_config**](PaymentApi.md#create_payment_intent_config) | **POST** /payment/config | 
[**delete_payment_intent_config**](PaymentApi.md#delete_payment_intent_config) | **DELETE** /payment/config/{id} | 
[**get_all_payment_intent_configs**](PaymentApi.md#get_all_payment_intent_configs) | **GET** /payment/config | 
[**get_one_payment_intent_configs**](PaymentApi.md#get_one_payment_intent_configs) | **GET** /payment/config/{id} | 
[**moralis_webhook**](PaymentApi.md#moralis_webhook) | **POST** /payment/webhook/{id} | 
[**payment_create_payment_intent**](PaymentApi.md#payment_create_payment_intent) | **POST** /payment | 
[**payment_delete_payment_intent**](PaymentApi.md#payment_delete_payment_intent) | **DELETE** /payment/{id} | 
[**payment_get_all_payment_intents**](PaymentApi.md#payment_get_all_payment_intents) | **GET** /payment | 
[**payment_get_available_chains**](PaymentApi.md#payment_get_available_chains) | **GET** /payment/chains | 
[**payment_get_payment_intent**](PaymentApi.md#payment_get_payment_intent) | **GET** /payment/{id} | 
[**payment_update_payment_intent**](PaymentApi.md#payment_update_payment_intent) | **PUT** /payment/{id} | 
[**tatum_webhook**](PaymentApi.md#tatum_webhook) | **POST** /payment/webhook/tatum/{id} | 
[**update_payment_intent_config**](PaymentApi.md#update_payment_intent_config) | **PUT** /payment/config/{id} | 



## create_payment_intent_config

> serde_json::Value create_payment_intent_config(authorization, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**body** | Option<**serde_json::Value**> |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_payment_intent_config

> crate::models::PaymentIntentResponse delete_payment_intent_config(authorization, id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**id** | **String** |  | [required] |

### Return type

[**crate::models::PaymentIntentResponse**](PaymentIntentResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_payment_intent_configs

> Vec<crate::models::PaymentIntentResponse> get_all_payment_intent_configs(authorization)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |

### Return type

[**Vec<crate::models::PaymentIntentResponse>**](PaymentIntentResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_one_payment_intent_configs

> crate::models::PaymentIntentResponse get_one_payment_intent_configs(authorization, id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**id** | **String** |  | [required] |

### Return type

[**crate::models::PaymentIntentResponse**](PaymentIntentResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## moralis_webhook

> serde_json::Value moralis_webhook(id, i_webhook)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**i_webhook** | [**IWebhook**](IWebhook.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payment_create_payment_intent

> crate::models::PaymentIntentResponse payment_create_payment_intent(authorization, create_payment_intent_input)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**create_payment_intent_input** | [**CreatePaymentIntentInput**](CreatePaymentIntentInput.md) |  | [required] |

### Return type

[**crate::models::PaymentIntentResponse**](PaymentIntentResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payment_delete_payment_intent

> crate::models::PaymentIntentResponse payment_delete_payment_intent(authorization, id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**id** | **String** |  | [required] |

### Return type

[**crate::models::PaymentIntentResponse**](PaymentIntentResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payment_get_all_payment_intents

> Vec<crate::models::PaymentIntentResponse> payment_get_all_payment_intents(authorization)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |

### Return type

[**Vec<crate::models::PaymentIntentResponse>**](PaymentIntentResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payment_get_available_chains

> Vec<String> payment_get_available_chains()


### Parameters

This endpoint does not need any parameter.

### Return type

**Vec<String>**

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payment_get_payment_intent

> crate::models::PaymentIntentResponse payment_get_payment_intent(authorization, id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**id** | **String** |  | [required] |

### Return type

[**crate::models::PaymentIntentResponse**](PaymentIntentResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payment_update_payment_intent

> crate::models::PaymentIntentResponse payment_update_payment_intent(authorization, id, create_payment_intent_input)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**id** | **String** |  | [required] |
**create_payment_intent_input** | [**CreatePaymentIntentInput**](CreatePaymentIntentInput.md) |  | [required] |

### Return type

[**crate::models::PaymentIntentResponse**](PaymentIntentResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tatum_webhook

> serde_json::Value tatum_webhook(id, tatum_transaction_event)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**tatum_transaction_event** | [**TatumTransactionEvent**](TatumTransactionEvent.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_payment_intent_config

> crate::models::PaymentIntentResponse update_payment_intent_config(authorization, id, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**id** | **String** |  | [required] |
**body** | Option<**serde_json::Value**> |  | [required] |

### Return type

[**crate::models::PaymentIntentResponse**](PaymentIntentResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

