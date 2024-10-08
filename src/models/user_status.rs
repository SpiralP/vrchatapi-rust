/*
 * VRChat API Documentation
 *
 *
 * Contact: me@ariesclark.com
 * Generated by: https://openapi-generator.tech
 */

/// UserStatus : Defines the User's current status, for example \"ask me\", \"join me\" or \"offline. This status is a combined indicator of their online activity and privacy preference.

/// Defines the User's current status, for example \"ask me\", \"join me\" or \"offline. This status is a combined indicator of their online activity and privacy preference.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum UserStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "join me")]
    JoinMe,
    #[serde(rename = "ask me")]
    AskMe,
    #[serde(rename = "busy")]
    Busy,
    #[serde(rename = "offline")]
    Offline,
}

impl ToString for UserStatus {
    fn to_string(&self) -> String {
        match self {
            Self::Active => String::from("active"),
            Self::JoinMe => String::from("join me"),
            Self::AskMe => String::from("ask me"),
            Self::Busy => String::from("busy"),
            Self::Offline => String::from("offline"),
        }
    }
}
