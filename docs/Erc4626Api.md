# \Erc4626Api

All URIs are relative to *https://beta.usemoon.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**deposit**](Erc4626Api.md#deposit) | **POST** /erc4626/{address}/deposit | 
[**get_asset**](Erc4626Api.md#get_asset) | **GET** /erc4626/{address}/asset | 
[**get_convert_to_assets**](Erc4626Api.md#get_convert_to_assets) | **GET** /erc4626/{address}/convertToAssets | 
[**get_convert_to_shares**](Erc4626Api.md#get_convert_to_shares) | **GET** /erc4626/{address}/convertToShares | 
[**get_max_deposit**](Erc4626Api.md#get_max_deposit) | **GET** /erc4626/{address}/maxDeposit | 
[**get_max_mint**](Erc4626Api.md#get_max_mint) | **GET** /erc4626/{address}/maxMint | 
[**get_max_redeem**](Erc4626Api.md#get_max_redeem) | **GET** /erc4626/{address}/maxRedeem | 
[**get_max_withdraw**](Erc4626Api.md#get_max_withdraw) | **GET** /erc4626/{address}/maxWithdraw | 
[**get_total_assets**](Erc4626Api.md#get_total_assets) | **GET** /erc4626/{address}/totalAssets | 
[**mint**](Erc4626Api.md#mint) | **POST** /erc4626/{address}/mint | 
[**redeem**](Erc4626Api.md#redeem) | **POST** /erc4626/{address}/redeem | 
[**withdraw**](Erc4626Api.md#withdraw) | **POST** /erc4626/{address}/withdraw | 



## deposit

> crate::models::Erc4626ApiResponse deposit(address, authorization, input_body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** |  | [required] |
**authorization** | **String** |  | [required] |
**input_body** | [**InputBody**](InputBody.md) |  | [required] |

### Return type

[**crate::models::Erc4626ApiResponse**](ERC4626APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_asset

> crate::models::Erc4626ApiResponse get_asset(address, authorization, chain_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** |  | [required] |
**authorization** | **String** |  | [required] |
**chain_id** | **String** |  | [required] |

### Return type

[**crate::models::Erc4626ApiResponse**](ERC4626APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_convert_to_assets

> crate::models::Erc4626ApiResponse get_convert_to_assets(address, authorization, chain_id, shares)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** |  | [required] |
**authorization** | **String** |  | [required] |
**chain_id** | **String** |  | [required] |
**shares** | **String** |  | [required] |

### Return type

[**crate::models::Erc4626ApiResponse**](ERC4626APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_convert_to_shares

> crate::models::Erc4626ApiResponse get_convert_to_shares(address, authorization, chain_id, assets)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** |  | [required] |
**authorization** | **String** |  | [required] |
**chain_id** | **String** |  | [required] |
**assets** | **String** |  | [required] |

### Return type

[**crate::models::Erc4626ApiResponse**](ERC4626APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_max_deposit

> crate::models::Erc4626ApiResponse get_max_deposit(address, authorization, chain_id, receiver)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** |  | [required] |
**authorization** | **String** |  | [required] |
**chain_id** | **String** |  | [required] |
**receiver** | **String** |  | [required] |

### Return type

[**crate::models::Erc4626ApiResponse**](ERC4626APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_max_mint

> crate::models::Erc4626ApiResponse get_max_mint(address, authorization, chain_id, receiver)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** |  | [required] |
**authorization** | **String** |  | [required] |
**chain_id** | **String** |  | [required] |
**receiver** | **String** |  | [required] |

### Return type

[**crate::models::Erc4626ApiResponse**](ERC4626APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_max_redeem

> crate::models::Erc4626ApiResponse get_max_redeem(address, authorization, chain_id, owner)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** |  | [required] |
**authorization** | **String** |  | [required] |
**chain_id** | **String** |  | [required] |
**owner** | **String** |  | [required] |

### Return type

[**crate::models::Erc4626ApiResponse**](ERC4626APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_max_withdraw

> crate::models::Erc4626ApiResponse get_max_withdraw(address, authorization, chain_id, owner)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** |  | [required] |
**authorization** | **String** |  | [required] |
**chain_id** | **String** |  | [required] |
**owner** | **String** |  | [required] |

### Return type

[**crate::models::Erc4626ApiResponse**](ERC4626APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_total_assets

> crate::models::Erc4626ApiResponse get_total_assets(address, authorization, chain_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** |  | [required] |
**authorization** | **String** |  | [required] |
**chain_id** | **String** |  | [required] |

### Return type

[**crate::models::Erc4626ApiResponse**](ERC4626APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mint

> crate::models::Erc4626ApiResponse mint(address, authorization, input_body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** |  | [required] |
**authorization** | **String** |  | [required] |
**input_body** | [**InputBody**](InputBody.md) |  | [required] |

### Return type

[**crate::models::Erc4626ApiResponse**](ERC4626APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## redeem

> crate::models::Erc4626ApiResponse redeem(address, authorization, input_body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** |  | [required] |
**authorization** | **String** |  | [required] |
**input_body** | [**InputBody**](InputBody.md) |  | [required] |

### Return type

[**crate::models::Erc4626ApiResponse**](ERC4626APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## withdraw

> crate::models::Erc4626ApiResponse withdraw(address, authorization, input_body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** |  | [required] |
**authorization** | **String** |  | [required] |
**input_body** | [**InputBody**](InputBody.md) |  | [required] |

### Return type

[**crate::models::Erc4626ApiResponse**](ERC4626APIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

