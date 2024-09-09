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
pub struct TokenInfo {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "decimals")]
    pub decimals: f64,
    #[serde(rename = "logoURI", skip_serializing_if = "Option::is_none")]
    pub logo_uri: Option<String>,
}

impl TokenInfo {
    pub fn new(address: String, symbol: String, name: String, decimals: f64) -> TokenInfo {
        TokenInfo {
            address,
            symbol,
            name,
            decimals,
            logo_uri: None,
        }
    }
}


