# \OnramperApi

All URIs are relative to *https://beta.usemoon.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**on_ramper_checkout**](OnramperApi.md#on_ramper_checkout) | **POST** /onramper/fund/${accountName} | 
[**on_ramper_get_quotes_buy**](OnramperApi.md#on_ramper_get_quotes_buy) | **GET** /onramper/quotes/buy | 
[**on_ramper_get_quotes_sell**](OnramperApi.md#on_ramper_get_quotes_sell) | **GET** /onramper/quotes/sell | 
[**on_ramper_get_supported_assets**](OnramperApi.md#on_ramper_get_supported_assets) | **GET** /onramper/assets | 
[**on_ramper_get_supported_currencies**](OnramperApi.md#on_ramper_get_supported_currencies) | **GET** /onramper/currencies | 
[**on_ramper_get_supported_defaults_all**](OnramperApi.md#on_ramper_get_supported_defaults_all) | **GET** /onramper/defaults | 
[**on_ramper_get_supported_on_ramps_all**](OnramperApi.md#on_ramper_get_supported_on_ramps_all) | **GET** /onramper/onramps | 
[**on_ramper_get_supported_payment_types**](OnramperApi.md#on_ramper_get_supported_payment_types) | **GET** /onramper/payment-types | 
[**on_ramper_get_supported_payment_types_fiat**](OnramperApi.md#on_ramper_get_supported_payment_types_fiat) | **GET** /onramper/payment-types/fiat | 



## on_ramper_checkout

> serde_json::Value on_ramper_checkout(authorization, account_name, transaction_input)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**account_name** | **String** |  | [required] |
**transaction_input** | [**TransactionInput**](TransactionInput.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## on_ramper_get_quotes_buy

> Vec<crate::models::Quote> on_ramper_get_quotes_buy(authorization, fiat, crypto, amount, payment_method, uuid, client_name, country)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**fiat** | **String** |  | [required] |
**crypto** | **String** |  | [required] |
**amount** | **f64** |  | [required] |
**payment_method** | Option<**String**> |  |  |[default to creditcard]
**uuid** | Option<**String**> |  |  |[default to ]
**client_name** | Option<**String**> |  |  |[default to ]
**country** | Option<**String**> |  |  |[default to ]

### Return type

[**Vec<crate::models::Quote>**](Quote.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## on_ramper_get_quotes_sell

> Vec<crate::models::SellQuote> on_ramper_get_quotes_sell(authorization, fiat, crypto, amount, payment_method, uuid, client_name, country)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**fiat** | **String** |  | [required] |
**crypto** | **String** |  | [required] |
**amount** | **f64** |  | [required] |
**payment_method** | Option<**String**> |  |  |[default to creditcard]
**uuid** | Option<**String**> |  |  |[default to ]
**client_name** | Option<**String**> |  |  |[default to ]
**country** | Option<**String**> |  |  |[default to ]

### Return type

[**Vec<crate::models::SellQuote>**](SellQuote.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## on_ramper_get_supported_assets

> crate::models::SupportedAssetResponse on_ramper_get_supported_assets(authorization, source, country)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**source** | **String** |  | [required] |
**country** | **String** |  | [required] |

### Return type

[**crate::models::SupportedAssetResponse**](SupportedAssetResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## on_ramper_get_supported_currencies

> crate::models::SupportedCurrenciesResponse on_ramper_get_supported_currencies(authorization, r#type)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**r#type** | **String** |  | [required] |

### Return type

[**crate::models::SupportedCurrenciesResponse**](SupportedCurrenciesResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## on_ramper_get_supported_defaults_all

> crate::models::SupportedDefaultResponse on_ramper_get_supported_defaults_all(authorization, country, r#type)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**country** | **String** |  | [required] |
**r#type** | **String** |  | [required] |

### Return type

[**crate::models::SupportedDefaultResponse**](SupportedDefaultResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## on_ramper_get_supported_on_ramps_all

> crate::models::GetSupportedOnRampsResponse on_ramper_get_supported_on_ramps_all(authorization)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |

### Return type

[**crate::models::GetSupportedOnRampsResponse**](GetSupportedOnRampsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## on_ramper_get_supported_payment_types

> crate::models::SupportedPaymentTypesCurrencyResponse on_ramper_get_supported_payment_types(authorization, fiat, country, r#type)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**fiat** | **String** |  | [required] |
**country** | **String** |  | [required] |
**r#type** | **String** |  | [required] |

### Return type

[**crate::models::SupportedPaymentTypesCurrencyResponse**](SupportedPaymentTypesCurrencyResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## on_ramper_get_supported_payment_types_fiat

> crate::models::SupportedPaymentTypesCurrencyResponse on_ramper_get_supported_payment_types_fiat(authorization, fiat, country)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**fiat** | **String** |  | [required] |
**country** | **String** |  | [required] |

### Return type

[**crate::models::SupportedPaymentTypesCurrencyResponse**](SupportedPaymentTypesCurrencyResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

