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
pub struct AbiOutput {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "components", skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<crate::models::AbiOutput>>,
    #[serde(rename = "internalType", skip_serializing_if = "Option::is_none")]
    pub internal_type: Option<String>,
}

impl AbiOutput {
    pub fn new(name: String, r#type: String) -> AbiOutput {
        AbiOutput {
            name,
            r#type,
            components: None,
            internal_type: None,
        }
    }
}


