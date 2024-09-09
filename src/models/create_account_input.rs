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
pub struct CreateAccountInput {
    #[serde(rename = "private_key", skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,
}

impl CreateAccountInput {
    pub fn new() -> CreateAccountInput {
        CreateAccountInput {
            private_key: None,
        }
    }
}


