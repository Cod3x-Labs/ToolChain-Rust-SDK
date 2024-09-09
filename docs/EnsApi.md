# \EnsApi

All URIs are relative to *https://beta.usemoon.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**resolve**](EnsApi.md#resolve) | **POST** /ens/resolve | 



## resolve

> crate::models::EnsResolveApiResponse resolve(authorization, ens_resolve_input)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**ens_resolve_input** | [**EnsResolveInput**](EnsResolveInput.md) |  | [required] |

### Return type

[**crate::models::EnsResolveApiResponse**](EnsResolveAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

