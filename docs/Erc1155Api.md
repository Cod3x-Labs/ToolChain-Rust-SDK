# \Erc1155Api

All URIs are relative to *https://beta.usemoon.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**balance_of**](Erc1155Api.md#balance_of) | **POST** /erc1155/{name}/balance-of | 
[**balance_of_batch**](Erc1155Api.md#balance_of_batch) | **POST** /erc1155/{name}/balance-of-batch | 
[**is_approved_for_all**](Erc1155Api.md#is_approved_for_all) | **POST** /erc1155/{name}/is-approved-for-all | 
[**safe_batch_transfer_from**](Erc1155Api.md#safe_batch_transfer_from) | **POST** /erc1155/{name}/safe-batch-transfer-from | 
[**safe_transfer_from**](Erc1155Api.md#safe_transfer_from) | **POST** /erc1155/{name}/safe-transfer-from | 
[**set_approval_for_all**](Erc1155Api.md#set_approval_for_all) | **POST** /erc1155/{name}/set-approval-for-all | 



## balance_of

> crate::models::TransactionApiResponse balance_of(name, authorization, erc1155_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**authorization** | **String** |  | [required] |
**erc1155_request** | [**Erc1155Request**](Erc1155Request.md) |  | [required] |

### Return type

[**crate::models::TransactionApiResponse**](TransactionAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## balance_of_batch

> crate::models::TransactionApiResponse balance_of_batch(name, authorization, erc1155_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**authorization** | **String** |  | [required] |
**erc1155_request** | [**Erc1155Request**](Erc1155Request.md) |  | [required] |

### Return type

[**crate::models::TransactionApiResponse**](TransactionAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## is_approved_for_all

> crate::models::TransactionApiResponse is_approved_for_all(name, authorization, erc1155_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**authorization** | **String** |  | [required] |
**erc1155_request** | [**Erc1155Request**](Erc1155Request.md) |  | [required] |

### Return type

[**crate::models::TransactionApiResponse**](TransactionAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## safe_batch_transfer_from

> crate::models::TransactionApiResponse safe_batch_transfer_from(name, authorization, erc1155_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**authorization** | **String** |  | [required] |
**erc1155_request** | [**Erc1155Request**](Erc1155Request.md) |  | [required] |

### Return type

[**crate::models::TransactionApiResponse**](TransactionAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## safe_transfer_from

> crate::models::TransactionApiResponse safe_transfer_from(name, authorization, erc1155_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**authorization** | **String** |  | [required] |
**erc1155_request** | [**Erc1155Request**](Erc1155Request.md) |  | [required] |

### Return type

[**crate::models::TransactionApiResponse**](TransactionAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_approval_for_all

> crate::models::TransactionApiResponse set_approval_for_all(name, authorization, erc1155_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**authorization** | **String** |  | [required] |
**erc1155_request** | [**Erc1155Request**](Erc1155Request.md) |  | [required] |

### Return type

[**crate::models::TransactionApiResponse**](TransactionAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

