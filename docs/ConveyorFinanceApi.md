# \ConveyorFinanceApi

All URIs are relative to *https://beta.usemoon.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**swap**](ConveyorFinanceApi.md#swap) | **POST** /conveyorfinance/{name}/swap | 



## swap

> crate::models::ConveyorFinanceControllerResponse swap(authorization, name, token_swap_params)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**name** | **String** |  | [required] |
**token_swap_params** | [**TokenSwapParams**](TokenSwapParams.md) |  | [required] |

### Return type

[**crate::models::ConveyorFinanceControllerResponse**](ConveyorFinanceControllerResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

