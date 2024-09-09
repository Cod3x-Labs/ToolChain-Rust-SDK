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
pub struct CreatePaymentIntentInput {
    #[serde(rename = "config", skip_serializing_if = "Option::is_none")]
    pub config: Option<String>,
    #[serde(rename = "metadata")]
    pub metadata: ::std::collections::HashMap<String, String>,
    #[serde(rename = "network", skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    #[serde(rename = "amount")]
    pub amount: f64,
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}

impl CreatePaymentIntentInput {
    pub fn new(metadata: ::std::collections::HashMap<String, String>, amount: f64) -> CreatePaymentIntentInput {
        CreatePaymentIntentInput {
            config: None,
            metadata,
            network: None,
            amount,
            currency: None,
        }
    }
}


