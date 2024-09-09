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
pub struct SupportedPaymentTypesMessage {
    #[serde(rename = "googlepay")]
    pub googlepay: Box<crate::models::PaymentType>,
    #[serde(rename = "applepay")]
    pub applepay: Box<crate::models::PaymentType>,
    #[serde(rename = "creditcard")]
    pub creditcard: Box<crate::models::PaymentType>,
}

impl SupportedPaymentTypesMessage {
    pub fn new(googlepay: crate::models::PaymentType, applepay: crate::models::PaymentType, creditcard: crate::models::PaymentType) -> SupportedPaymentTypesMessage {
        SupportedPaymentTypesMessage {
            googlepay: Box::new(googlepay),
            applepay: Box::new(applepay),
            creditcard: Box::new(creditcard),
        }
    }
}

