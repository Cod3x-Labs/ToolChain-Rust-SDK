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
pub struct TransactionResponse {
    #[serde(rename = "message")]
    pub message: String,
    #[serde(rename = "tx")]
    pub tx: Box<crate::models::TransactionResponseTx>,
    #[serde(rename = "info")]
    pub info: Box<crate::models::TransactionResponseInfo>,
    #[serde(rename = "chainId")]
    pub chain_id: f64,
    #[serde(rename = "currentBlockNumber")]
    pub current_block_number: f64,
}

impl TransactionResponse {
    pub fn new(message: String, tx: crate::models::TransactionResponseTx, info: crate::models::TransactionResponseInfo, chain_id: f64, current_block_number: f64) -> TransactionResponse {
        TransactionResponse {
            message,
            tx: Box::new(tx),
            info: Box::new(info),
            chain_id,
            current_block_number,
        }
    }
}


