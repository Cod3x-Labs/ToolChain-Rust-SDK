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
pub struct BaseApiResponse {
    #[serde(rename = "success")]
    pub success: bool,
    #[serde(rename = "message")]
    pub message: String,
    #[serde(rename = "body", skip_serializing_if = "Option::is_none")]
    pub body: Option<Box<crate::models::InputBody>>,
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
}

impl BaseApiResponse {
    pub fn new(success: bool, message: String) -> BaseApiResponse {
        BaseApiResponse {
            success,
            message,
            body: None,
            address: None,
        }
    }
}

