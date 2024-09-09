/*
 * moon-vault-api
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConveyorFinanceControllerResponse {
    #[serde(rename = "input", skip_serializing_if = "Option::is_none")]
    pub input: Option<Box<crate::models::InputBody>>,
    #[serde(rename = "convey", skip_serializing_if = "Option::is_none")]
    pub convey: Option<Box<crate::models::TransactionResponse>>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Box<crate::models::TransactionData>>,
    #[serde(rename = "tx", skip_serializing_if = "Option::is_none")]
    pub tx: Option<Box<crate::models::TransactionResponseTx>>,
    #[serde(rename = "signed", skip_serializing_if = "Option::is_none")]
    pub signed: Option<Box<crate::models::Transaction>>,
    #[serde(rename = "success")]
    pub success: bool,
    #[serde(rename = "message")]
    pub message: String,
}

impl ConveyorFinanceControllerResponse {
    pub fn new(success: bool, message: String) -> ConveyorFinanceControllerResponse {
        ConveyorFinanceControllerResponse {
            input: None,
            convey: None,
            data: None,
            tx: None,
            signed: None,
            success,
            message,
        }
    }
}

