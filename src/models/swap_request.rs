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
pub struct SwapRequest {
    #[serde(rename = "dryrun", skip_serializing_if = "Option::is_none")]
    pub dryrun: Option<bool>,
    #[serde(rename = "to", skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    #[serde(rename = "input", skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "nonce", skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
    #[serde(rename = "gas", skip_serializing_if = "Option::is_none")]
    pub gas: Option<String>,
    #[serde(rename = "gasPrice", skip_serializing_if = "Option::is_none")]
    pub gas_price: Option<f64>,
    #[serde(rename = "chain_id", skip_serializing_if = "Option::is_none")]
    pub chain_id: Option<String>,
    #[serde(rename = "encoding", skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,
    #[serde(rename = "EOA", skip_serializing_if = "Option::is_none")]
    pub eoa: Option<bool>,
    #[serde(rename = "contract_address", skip_serializing_if = "Option::is_none")]
    pub contract_address: Option<String>,
    #[serde(rename = "token_id", skip_serializing_if = "Option::is_none")]
    pub token_id: Option<String>,
    #[serde(rename = "token_ids", skip_serializing_if = "Option::is_none")]
    pub token_ids: Option<String>,
    #[serde(rename = "approved", skip_serializing_if = "Option::is_none")]
    pub approved: Option<bool>,
    #[serde(rename = "broadcast", skip_serializing_if = "Option::is_none")]
    pub broadcast: Option<bool>,
    #[serde(rename = "alwaysIncrementNonce", skip_serializing_if = "Option::is_none")]
    pub always_increment_nonce: Option<bool>,
    #[serde(rename = "asset", skip_serializing_if = "Option::is_none")]
    pub asset: Option<String>,
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(rename = "interestRateMode", skip_serializing_if = "Option::is_none")]
    pub interest_rate_mode: Option<f64>,
    #[serde(rename = "referralCode", skip_serializing_if = "Option::is_none")]
    pub referral_code: Option<f64>,
    #[serde(rename = "onBehalfOf", skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<String>,
    #[serde(rename = "receiverAddress", skip_serializing_if = "Option::is_none")]
    pub receiver_address: Option<String>,
    #[serde(rename = "assets", skip_serializing_if = "Option::is_none")]
    pub assets: Option<Vec<String>>,
    #[serde(rename = "amounts", skip_serializing_if = "Option::is_none")]
    pub amounts: Option<Vec<String>>,
    #[serde(rename = "modes", skip_serializing_if = "Option::is_none")]
    pub modes: Option<Vec<f64>>,
    #[serde(rename = "params", skip_serializing_if = "Option::is_none")]
    pub params: Option<String>,
    #[serde(rename = "rateMode", skip_serializing_if = "Option::is_none")]
    pub rate_mode: Option<f64>,
    #[serde(rename = "useAsCollateral", skip_serializing_if = "Option::is_none")]
    pub use_as_collateral: Option<bool>,
    #[serde(rename = "collateralAsset", skip_serializing_if = "Option::is_none")]
    pub collateral_asset: Option<String>,
    #[serde(rename = "debtAsset", skip_serializing_if = "Option::is_none")]
    pub debt_asset: Option<String>,
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(rename = "debtToCover", skip_serializing_if = "Option::is_none")]
    pub debt_to_cover: Option<String>,
    #[serde(rename = "receiveAToken", skip_serializing_if = "Option::is_none")]
    pub receive_a_token: Option<bool>,
    #[serde(rename = "inputTokens")]
    pub input_tokens: Vec<crate::models::TokenAmount>,
    #[serde(rename = "outputTokens")]
    pub output_tokens: Vec<crate::models::TokenProportion>,
    #[serde(rename = "slippageLimitPercent", skip_serializing_if = "Option::is_none")]
    pub slippage_limit_percent: Option<f64>,
    #[serde(rename = "minHealthFactor", skip_serializing_if = "Option::is_none")]
    pub min_health_factor: Option<Box<crate::models::InputBodyMinHealthFactor>>,
    #[serde(rename = "premiums", skip_serializing_if = "Option::is_none")]
    pub premiums: Option<Box<crate::models::InputBodyPremiums>>,
    #[serde(rename = "initiator", skip_serializing_if = "Option::is_none")]
    pub initiator: Option<String>,
    #[serde(rename = "role", skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(rename = "account", skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    #[serde(rename = "initialDeposit", skip_serializing_if = "Option::is_none")]
    pub initial_deposit: Option<Box<crate::models::InputBodyMinHealthFactor>>,
    #[serde(rename = "borrowAmount", skip_serializing_if = "Option::is_none")]
    pub borrow_amount: Option<Box<crate::models::InputBodyMinHealthFactor>>,
    #[serde(rename = "chainId")]
    pub chain_id: f64,
    #[serde(rename = "userAddr", skip_serializing_if = "Option::is_none")]
    pub user_addr: Option<String>,
    #[serde(rename = "sourceBlacklist", skip_serializing_if = "Option::is_none")]
    pub source_blacklist: Option<Vec<String>>,
    #[serde(rename = "sourceWhitelist", skip_serializing_if = "Option::is_none")]
    pub source_whitelist: Option<Vec<String>>,
    #[serde(rename = "poolBlacklist", skip_serializing_if = "Option::is_none")]
    pub pool_blacklist: Option<Vec<String>>,
    #[serde(rename = "pathViz", skip_serializing_if = "Option::is_none")]
    pub path_viz: Option<bool>,
    #[serde(rename = "pathVizImage", skip_serializing_if = "Option::is_none")]
    pub path_viz_image: Option<bool>,
    #[serde(rename = "pathVizImageConfig", skip_serializing_if = "Option::is_none")]
    pub path_viz_image_config: Option<Box<crate::models::PathVizImageConfig>>,
    #[serde(rename = "disableRFQs", skip_serializing_if = "Option::is_none")]
    pub disable_rfqs: Option<bool>,
    #[serde(rename = "compact", skip_serializing_if = "Option::is_none")]
    pub compact: Option<bool>,
    #[serde(rename = "likeAsset", skip_serializing_if = "Option::is_none")]
    pub like_asset: Option<bool>,
    #[serde(rename = "simple", skip_serializing_if = "Option::is_none")]
    pub simple: Option<bool>,
}

impl SwapRequest {
    pub fn new(input_tokens: Vec<crate::models::TokenAmount>, output_tokens: Vec<crate::models::TokenProportion>, chain_id: f64) -> SwapRequest {
        SwapRequest {
            dryrun: None,
            to: None,
            data: None,
            input: None,
            value: None,
            nonce: None,
            gas: None,
            gas_price: None,
            chain_id: None,
            encoding: None,
            eoa: None,
            contract_address: None,
            token_id: None,
            token_ids: None,
            approved: None,
            broadcast: None,
            always_increment_nonce: None,
            asset: None,
            amount: None,
            interest_rate_mode: None,
            referral_code: None,
            on_behalf_of: None,
            receiver_address: None,
            assets: None,
            amounts: None,
            modes: None,
            params: None,
            rate_mode: None,
            use_as_collateral: None,
            collateral_asset: None,
            debt_asset: None,
            user: None,
            debt_to_cover: None,
            receive_a_token: None,
            input_tokens,
            output_tokens,
            slippage_limit_percent: None,
            min_health_factor: None,
            premiums: None,
            initiator: None,
            role: None,
            account: None,
            initial_deposit: None,
            borrow_amount: None,
            chain_id,
            user_addr: None,
            source_blacklist: None,
            source_whitelist: None,
            pool_blacklist: None,
            path_viz: None,
            path_viz_image: None,
            path_viz_image_config: None,
            disable_rfqs: None,
            compact: None,
            like_asset: None,
            simple: None,
        }
    }
}


