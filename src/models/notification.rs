/*
 * VRChat API Documentation
 *
 *
 * Contact: me@ariesclark.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Notification {
    #[serde(rename = "created_at")]
    pub created_at: String,
    /// **NOTICE:** This is not a JSON object, this is a json **encoded** object, meaning you have to json-de-encode to get the NotificationDetail object depending on the NotificationType.
    #[serde(rename = "details")]
    pub details: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "message")]
    pub message: String,
    #[serde(rename = "seen")]
    pub seen: bool,
    /// A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed.
    #[serde(rename = "senderUserId")]
    pub sender_user_id: String,
    #[serde(rename = "senderUsername")]
    pub sender_username: String,
    #[serde(rename = "type")]
    pub _type: crate::models::NotificationType,
}

impl Notification {
    pub fn new(
        created_at: String,
        details: String,
        id: String,
        message: String,
        seen: bool,
        sender_user_id: String,
        sender_username: String,
        _type: crate::models::NotificationType,
    ) -> Notification {
        Notification {
            created_at,
            details,
            id,
            message,
            seen,
            sender_user_id,
            sender_username,
            _type,
        }
    }
}
