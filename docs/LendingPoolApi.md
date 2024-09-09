# \LendingPoolApi

All URIs are relative to *https://beta.usemoon.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**borrow**](LendingPoolApi.md#borrow) | **POST** /lending-pool/{accountName}/borrow | 
[**deposit**](LendingPoolApi.md#deposit) | **POST** /lending-pool/{accountName}/deposit | 
[**flash_loan**](LendingPoolApi.md#flash_loan) | **POST** /lending-pool/{accountName}/flash-loan | 
[**get_addresses_provider**](LendingPoolApi.md#get_addresses_provider) | **GET** /lending-pool/addresses-provider | 
[**get_flash_loan_premium_total**](LendingPoolApi.md#get_flash_loan_premium_total) | **GET** /lending-pool/flash-loan-premium | 
[**get_lending_pool_revision**](LendingPoolApi.md#get_lending_pool_revision) | **GET** /lending-pool/revision | 
[**get_max_number_reserves**](LendingPoolApi.md#get_max_number_reserves) | **GET** /lending-pool/max-reserves | 
[**get_max_stable_rate_borrow_size_percent**](LendingPoolApi.md#get_max_stable_rate_borrow_size_percent) | **GET** /lending-pool/max-stable-rate-borrow-size-percent | 
[**get_reserve_data**](LendingPoolApi.md#get_reserve_data) | **GET** /lending-pool/reserve-data | 
[**get_reserves_list**](LendingPoolApi.md#get_reserves_list) | **GET** /lending-pool/reserves-list | 
[**get_user_account_data**](LendingPoolApi.md#get_user_account_data) | **GET** /lending-pool/user-account-data | 
[**is_paused**](LendingPoolApi.md#is_paused) | **GET** /lending-pool/paused | 
[**liquidation_call**](LendingPoolApi.md#liquidation_call) | **POST** /lending-pool/{accountName}/liquidation-call | 
[**repay**](LendingPoolApi.md#repay) | **POST** /lending-pool/{accountName}/repay | 
[**set_user_use_reserve_as_collateral**](LendingPoolApi.md#set_user_use_reserve_as_collateral) | **POST** /lending-pool/{accountName}/set-user-use-reserve-as-collateral | 
[**swap_borrow_rate_mode**](LendingPoolApi.md#swap_borrow_rate_mode) | **POST** /lending-pool/{accountName}/swap-borrow-rate-mode | 



## borrow

> crate::models::LendingPoolApiResponse borrow(account_name, authorization, input_body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_name** | **String** |  | [required] |
**authorization** | **String** |  | [required] |
**input_body** | [**InputBody**](InputBody.md) |  | [required] |

### Return type

[**crate::models::LendingPoolApiResponse**](LendingPoolAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## deposit

> crate::models::LendingPoolApiResponse deposit(account_name, authorization, input_body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_name** | **String** |  | [required] |
**authorization** | **String** |  | [required] |
**input_body** | [**InputBody**](InputBody.md) |  | [required] |

### Return type

[**crate::models::LendingPoolApiResponse**](LendingPoolAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## flash_loan

> crate::models::LendingPoolApiResponse flash_loan(account_name, authorization, input_body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_name** | **String** |  | [required] |
**authorization** | **String** |  | [required] |
**input_body** | [**InputBody**](InputBody.md) |  | [required] |

### Return type

[**crate::models::LendingPoolApiResponse**](LendingPoolAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_addresses_provider

> crate::models::LendingPoolApiResponse get_addresses_provider(authorization, address, chain_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**address** | **String** |  | [required] |
**chain_id** | **String** |  | [required] |

### Return type

[**crate::models::LendingPoolApiResponse**](LendingPoolAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_flash_loan_premium_total

> crate::models::LendingPoolApiResponse get_flash_loan_premium_total(authorization, address, chain_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**address** | **String** |  | [required] |
**chain_id** | **String** |  | [required] |

### Return type

[**crate::models::LendingPoolApiResponse**](LendingPoolAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lending_pool_revision

> crate::models::LendingPoolApiResponse get_lending_pool_revision(authorization, address, chain_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**address** | **String** |  | [required] |
**chain_id** | **String** |  | [required] |

### Return type

[**crate::models::LendingPoolApiResponse**](LendingPoolAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_max_number_reserves

> crate::models::LendingPoolApiResponse get_max_number_reserves(authorization, address, chain_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**address** | **String** |  | [required] |
**chain_id** | **String** |  | [required] |

### Return type

[**crate::models::LendingPoolApiResponse**](LendingPoolAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_max_stable_rate_borrow_size_percent

> crate::models::LendingPoolApiResponse get_max_stable_rate_borrow_size_percent(authorization, address, chain_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**address** | **String** |  | [required] |
**chain_id** | **String** |  | [required] |

### Return type

[**crate::models::LendingPoolApiResponse**](LendingPoolAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_reserve_data

> crate::models::LendingPoolApiResponse get_reserve_data(authorization, address, chain_id, asset)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**address** | **String** |  | [required] |
**chain_id** | **String** |  | [required] |
**asset** | **String** |  | [required] |

### Return type

[**crate::models::LendingPoolApiResponse**](LendingPoolAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_reserves_list

> crate::models::LendingPoolApiResponse get_reserves_list(authorization, address, chain_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**address** | **String** |  | [required] |
**chain_id** | **String** |  | [required] |

### Return type

[**crate::models::LendingPoolApiResponse**](LendingPoolAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_account_data

> crate::models::LendingPoolApiResponse get_user_account_data(authorization, address, chain_id, user)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**address** | **String** |  | [required] |
**chain_id** | **String** |  | [required] |
**user** | **String** |  | [required] |

### Return type

[**crate::models::LendingPoolApiResponse**](LendingPoolAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## is_paused

> crate::models::LendingPoolApiResponse is_paused(authorization, address, chain_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**address** | **String** |  | [required] |
**chain_id** | **String** |  | [required] |

### Return type

[**crate::models::LendingPoolApiResponse**](LendingPoolAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## liquidation_call

> crate::models::LendingPoolApiResponse liquidation_call(account_name, authorization, input_body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_name** | **String** |  | [required] |
**authorization** | **String** |  | [required] |
**input_body** | [**InputBody**](InputBody.md) |  | [required] |

### Return type

[**crate::models::LendingPoolApiResponse**](LendingPoolAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repay

> crate::models::LendingPoolApiResponse repay(account_name, authorization, input_body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_name** | **String** |  | [required] |
**authorization** | **String** |  | [required] |
**input_body** | [**InputBody**](InputBody.md) |  | [required] |

### Return type

[**crate::models::LendingPoolApiResponse**](LendingPoolAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_user_use_reserve_as_collateral

> crate::models::LendingPoolApiResponse set_user_use_reserve_as_collateral(account_name, authorization, input_body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_name** | **String** |  | [required] |
**authorization** | **String** |  | [required] |
**input_body** | [**InputBody**](InputBody.md) |  | [required] |

### Return type

[**crate::models::LendingPoolApiResponse**](LendingPoolAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## swap_borrow_rate_mode

> crate::models::LendingPoolApiResponse swap_borrow_rate_mode(account_name, authorization, input_body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_name** | **String** |  | [required] |
**authorization** | **String** |  | [required] |
**input_body** | [**InputBody**](InputBody.md) |  | [required] |

### Return type

[**crate::models::LendingPoolApiResponse**](LendingPoolAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

