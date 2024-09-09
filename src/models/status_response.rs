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
pub struct StatusResponse {
    #[serde(rename = "sending")]
    pub sending: Box<crate::models::TransactionStatus>,
    #[serde(rename = "receiving")]
    pub receiving: Box<crate::models::TransactionStatus>,
    #[serde(rename = "tool")]
    pub tool: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "substatus")]
    pub substatus: String,
}

impl StatusResponse {
    pub fn new(sending: crate::models::TransactionStatus, receiving: crate::models::TransactionStatus, tool: String, status: String, substatus: String) -> StatusResponse {
        StatusResponse {
            sending: Box::new(sending),
            receiving: Box::new(receiving),
            tool,
            status,
            substatus,
        }
    }
}

