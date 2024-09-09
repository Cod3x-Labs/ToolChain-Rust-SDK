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
pub struct SolanaSignTransactionInput {
    #[serde(rename = "unsigned_tx")]
    pub unsigned_tx: String,
    #[serde(rename = "network", skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
}

impl SolanaSignTransactionInput {
    pub fn new(unsigned_tx: String) -> SolanaSignTransactionInput {
        SolanaSignTransactionInput {
            unsigned_tx,
            network: None,
        }
    }
}


