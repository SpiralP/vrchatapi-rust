/*
 * VRChat API Documentation
 *
 *
 * Contact: me@ariesclark.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Transaction {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "status")]
    pub status: crate::models::TransactionStatus,
    #[serde(rename = "subscription")]
    pub subscription: Box<crate::models::Subscription>,
    #[serde(rename = "sandbox")]
    pub sandbox: bool,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "steam", skip_serializing_if = "Option::is_none")]
    pub steam: Option<Box<crate::models::TransactionSteamInfo>>,
    #[serde(rename = "agreement", skip_serializing_if = "Option::is_none")]
    pub agreement: Option<Box<crate::models::TransactionAgreement>>,
    #[serde(rename = "error")]
    pub error: String,
}

impl Transaction {
    pub fn new(
        id: String,
        status: crate::models::TransactionStatus,
        subscription: crate::models::Subscription,
        sandbox: bool,
        created_at: String,
        updated_at: String,
        error: String,
    ) -> Transaction {
        Transaction {
            id,
            status,
            subscription: Box::new(subscription),
            sandbox,
            created_at,
            updated_at,
            steam: None,
            agreement: None,
            error,
        }
    }
}
