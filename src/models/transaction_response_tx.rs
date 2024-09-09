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
pub struct TransactionResponseTx {
    #[serde(rename = "data")]
    pub data: String,
    #[serde(rename = "value")]
    pub value: String,
    #[serde(rename = "nonce")]
    pub nonce: String,
    #[serde(rename = "gas")]
    pub gas: String,
    #[serde(rename = "to")]
    pub to: String,
    #[serde(rename = "from")]
    pub from: String,
}

impl TransactionResponseTx {
    pub fn new(data: String, value: String, nonce: String, gas: String, to: String, from: String) -> TransactionResponseTx {
        TransactionResponseTx {
            data,
            value,
            nonce,
            gas,
            to,
            from,
        }
    }
}


