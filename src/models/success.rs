/*
 * VRChat API Documentation
 *
 *
 * Contact: me@ariesclark.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Success {
    #[serde(rename = "success", skip_serializing_if = "Option::is_none")]
    pub success: Option<Box<crate::models::Response>>,
}

impl Success {
    pub fn new() -> Success {
        Success { success: None }
    }
}
