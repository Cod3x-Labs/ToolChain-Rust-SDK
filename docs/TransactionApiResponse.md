# TransactionApiResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**success** | **bool** |  | 
**message** | **String** |  | 
**body** | Option<[**crate::models::InputBody**](InputBody.md)> |  | [optional]
**address** | Option<**String**> |  | [optional]
**transaction_hash** | Option<[**serde_json::Value**](.md)> |  | [optional]
**signed_tx** | Option<[**serde_json::Value**](.md)> |  | [optional]
**data** | Option<[**crate::models::Transaction**](Transaction.md)> |  | [optional]
**broadcasted** | Option<[**crate::models::BroadCastRawTransactionResponse**](BroadCastRawTransactionResponse.md)> |  | [optional]
**transaction** | Option<[**serde_json::Value**](.md)> |  | [optional]
**function** | Option<**String**> |  | [optional]
**params** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


