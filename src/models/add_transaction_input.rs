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
pub struct AddTransactionInput {
    #[serde(rename = "rpcUrl")]
    pub rpc_url: String,
    #[serde(rename = "transaction")]
    pub transaction: Box<crate::models::Transaction>,
}

impl AddTransactionInput {
    pub fn new(rpc_url: String, transaction: crate::models::Transaction) -> AddTransactionInput {
        AddTransactionInput {
            rpc_url,
            transaction: Box::new(transaction),
        }
    }
}


