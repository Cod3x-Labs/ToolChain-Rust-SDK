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
pub struct TokenProportion {
    #[serde(rename = "tokenAddress")]
    pub token_address: String,
    #[serde(rename = "proportion")]
    pub proportion: f64,
}

impl TokenProportion {
    pub fn new(token_address: String, proportion: f64) -> TokenProportion {
        TokenProportion {
            token_address,
            proportion,
        }
    }
}


