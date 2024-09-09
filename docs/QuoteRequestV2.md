# QuoteRequestV2

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**chain_id** | **f64** |  | 
**input_tokens** | [**Vec<crate::models::TokenAmount>**](TokenAmount.md) |  | 
**output_tokens** | [**Vec<crate::models::TokenProportion>**](TokenProportion.md) |  | 
**gas_price** | Option<**f64**> |  | [optional]
**user_addr** | Option<**String**> |  | [optional]
**slippage_limit_percent** | Option<**f64**> |  | [optional]
**source_blacklist** | Option<**Vec<String>**> |  | [optional]
**source_whitelist** | Option<**Vec<String>**> |  | [optional]
**pool_blacklist** | Option<**Vec<String>**> |  | [optional]
**path_viz** | Option<**bool**> |  | [optional]
**path_viz_image** | Option<**bool**> |  | [optional]
**path_viz_image_config** | Option<[**crate::models::PathVizImageConfig**](PathVizImageConfig.md)> |  | [optional]
**disable_rfqs** | Option<**bool**> |  | [optional]
**referral_code** | Option<**f64**> |  | [optional]
**compact** | Option<**bool**> |  | [optional]
**like_asset** | Option<**bool**> |  | [optional]
**simple** | Option<**bool**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


