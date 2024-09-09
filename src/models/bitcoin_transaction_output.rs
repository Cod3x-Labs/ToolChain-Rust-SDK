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
pub struct BitcoinTransactionOutput {
    #[serde(rename = "signedTx", skip_serializing_if = "Option::is_none")]
    pub signed_tx: Option<String>,
    #[serde(rename = "transaction_hash", skip_serializing_if = "Option::is_none")]
    pub transaction_hash: Option<String>,
}

impl BitcoinTransactionOutput {
    pub fn new() -> BitcoinTransactionOutput {
        BitcoinTransactionOutput {
            signed_tx: None,
            transaction_hash: None,
        }
    }
}


