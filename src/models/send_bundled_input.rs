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
pub struct SendBundledInput {
    #[serde(rename = "rpcUrl")]
    pub rpc_url: String,
    #[serde(rename = "bundleId")]
    pub bundle_id: String,
}

impl SendBundledInput {
    pub fn new(rpc_url: String, bundle_id: String) -> SendBundledInput {
        SendBundledInput {
            rpc_url,
            bundle_id,
        }
    }
}


