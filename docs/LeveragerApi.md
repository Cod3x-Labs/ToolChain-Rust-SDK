# \LeveragerApi

All URIs are relative to *https://beta.usemoon.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**deleverage_erc20**](LeveragerApi.md#deleverage_erc20) | **POST** /leverager/{accountName}/deleverage-erc20 | 
[**deleverage_native**](LeveragerApi.md#deleverage_native) | **POST** /leverager/{accountName}/deleverage-native | 
[**execute_operation**](LeveragerApi.md#execute_operation) | **POST** /leverager/{accountName}/execute-operation | 
[**get_addresses_provider**](LeveragerApi.md#get_addresses_provider) | **GET** /leverager/addresses-provider | 
[**get_default_admin_role**](LeveragerApi.md#get_default_admin_role) | **GET** /leverager/default-admin-role | 
[**get_lending_pool**](LeveragerApi.md#get_lending_pool) | **GET** /leverager/lending-pool | 
[**get_min_hf**](LeveragerApi.md#get_min_hf) | **GET** /leverager/min-hf | 
[**get_role_admin**](LeveragerApi.md#get_role_admin) | **GET** /leverager/role-admin | 
[**get_weth**](LeveragerApi.md#get_weth) | **GET** /leverager/weth | 
[**grant_role**](LeveragerApi.md#grant_role) | **POST** /leverager/{accountName}/grant-role | 
[**has_role**](LeveragerApi.md#has_role) | **GET** /leverager/has-role | 
[**is_paused**](LeveragerApi.md#is_paused) | **GET** /leverager/paused | 
[**leverage_erc20**](LeveragerApi.md#leverage_erc20) | **POST** /leverager/{accountName}/leverage-erc20 | 
[**leverage_native**](LeveragerApi.md#leverage_native) | **POST** /leverager/{accountName}/leverage-native | 
[**pause**](LeveragerApi.md#pause) | **POST** /leverager/{accountName}/pause | 
[**renounce_role**](LeveragerApi.md#renounce_role) | **POST** /leverager/{accountName}/renounce-role | 
[**revoke_role**](LeveragerApi.md#revoke_role) | **POST** /leverager/{accountName}/revoke-role | 
[**supports_interface**](LeveragerApi.md#supports_interface) | **GET** /leverager/supports-interface | 
[**unpause**](LeveragerApi.md#unpause) | **POST** /leverager/{accountName}/unpause | 



## deleverage_erc20

> crate::models::LeveragerApiResponse deleverage_erc20(account_name, authorization, input_body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_name** | **String** |  | [required] |
**authorization** | **String** |  | [required] |
**input_body** | [**InputBody**](InputBody.md) |  | [required] |

### Return type

[**crate::models::LeveragerApiResponse**](LeveragerAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## deleverage_native

> crate::models::LeveragerApiResponse deleverage_native(account_name, authorization, input_body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_name** | **String** |  | [required] |
**authorization** | **String** |  | [required] |
**input_body** | [**InputBody**](InputBody.md) |  | [required] |

### Return type

[**crate::models::LeveragerApiResponse**](LeveragerAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## execute_operation

> crate::models::LeveragerApiResponse execute_operation(account_name, authorization, input_body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_name** | **String** |  | [required] |
**authorization** | **String** |  | [required] |
**input_body** | [**InputBody**](InputBody.md) |  | [required] |

### Return type

[**crate::models::LeveragerApiResponse**](LeveragerAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_addresses_provider

> crate::models::LeveragerApiResponse get_addresses_provider(authorization, address, chain_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**address** | **String** |  | [required] |
**chain_id** | **String** |  | [required] |

### Return type

[**crate::models::LeveragerApiResponse**](LeveragerAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_default_admin_role

> crate::models::LeveragerApiResponse get_default_admin_role(authorization, address, chain_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**address** | **String** |  | [required] |
**chain_id** | **String** |  | [required] |

### Return type

[**crate::models::LeveragerApiResponse**](LeveragerAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lending_pool

> crate::models::LeveragerApiResponse get_lending_pool(authorization, address, chain_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**address** | **String** |  | [required] |
**chain_id** | **String** |  | [required] |

### Return type

[**crate::models::LeveragerApiResponse**](LeveragerAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_min_hf

> crate::models::LeveragerApiResponse get_min_hf(authorization, address, chain_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**address** | **String** |  | [required] |
**chain_id** | **String** |  | [required] |

### Return type

[**crate::models::LeveragerApiResponse**](LeveragerAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_role_admin

> crate::models::LeveragerApiResponse get_role_admin(authorization, address, chain_id, role)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**address** | **String** |  | [required] |
**chain_id** | **String** |  | [required] |
**role** | **String** |  | [required] |

### Return type

[**crate::models::LeveragerApiResponse**](LeveragerAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_weth

> crate::models::LeveragerApiResponse get_weth(authorization, address, chain_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**address** | **String** |  | [required] |
**chain_id** | **String** |  | [required] |

### Return type

[**crate::models::LeveragerApiResponse**](LeveragerAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## grant_role

> crate::models::LeveragerApiResponse grant_role(account_name, authorization, input_body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_name** | **String** |  | [required] |
**authorization** | **String** |  | [required] |
**input_body** | [**InputBody**](InputBody.md) |  | [required] |

### Return type

[**crate::models::LeveragerApiResponse**](LeveragerAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## has_role

> crate::models::LeveragerApiResponse has_role(authorization, address, chain_id, role, account)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**address** | **String** |  | [required] |
**chain_id** | **String** |  | [required] |
**role** | **String** |  | [required] |
**account** | **String** |  | [required] |

### Return type

[**crate::models::LeveragerApiResponse**](LeveragerAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## is_paused

> crate::models::LeveragerApiResponse is_paused(authorization, address, chain_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**address** | **String** |  | [required] |
**chain_id** | **String** |  | [required] |

### Return type

[**crate::models::LeveragerApiResponse**](LeveragerAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## leverage_erc20

> crate::models::LeveragerApiResponse leverage_erc20(account_name, authorization, input_body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_name** | **String** |  | [required] |
**authorization** | **String** |  | [required] |
**input_body** | [**InputBody**](InputBody.md) |  | [required] |

### Return type

[**crate::models::LeveragerApiResponse**](LeveragerAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## leverage_native

> crate::models::LeveragerApiResponse leverage_native(account_name, authorization, input_body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_name** | **String** |  | [required] |
**authorization** | **String** |  | [required] |
**input_body** | [**InputBody**](InputBody.md) |  | [required] |

### Return type

[**crate::models::LeveragerApiResponse**](LeveragerAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pause

> crate::models::LeveragerApiResponse pause(account_name, authorization, input_body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_name** | **String** |  | [required] |
**authorization** | **String** |  | [required] |
**input_body** | [**InputBody**](InputBody.md) |  | [required] |

### Return type

[**crate::models::LeveragerApiResponse**](LeveragerAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## renounce_role

> crate::models::LeveragerApiResponse renounce_role(account_name, authorization, input_body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_name** | **String** |  | [required] |
**authorization** | **String** |  | [required] |
**input_body** | [**InputBody**](InputBody.md) |  | [required] |

### Return type

[**crate::models::LeveragerApiResponse**](LeveragerAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## revoke_role

> crate::models::LeveragerApiResponse revoke_role(account_name, authorization, input_body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_name** | **String** |  | [required] |
**authorization** | **String** |  | [required] |
**input_body** | [**InputBody**](InputBody.md) |  | [required] |

### Return type

[**crate::models::LeveragerApiResponse**](LeveragerAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## supports_interface

> crate::models::LeveragerApiResponse supports_interface(authorization, address, chain_id, interface_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**address** | **String** |  | [required] |
**chain_id** | **String** |  | [required] |
**interface_id** | **String** |  | [required] |

### Return type

[**crate::models::LeveragerApiResponse**](LeveragerAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unpause

> crate::models::LeveragerApiResponse unpause(account_name, authorization, input_body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_name** | **String** |  | [required] |
**authorization** | **String** |  | [required] |
**input_body** | [**InputBody**](InputBody.md) |  | [required] |

### Return type

[**crate::models::LeveragerApiResponse**](LeveragerAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

