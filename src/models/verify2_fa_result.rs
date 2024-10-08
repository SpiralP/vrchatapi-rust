/*
 * VRChat API Documentation
 *
 *
 * Contact: me@ariesclark.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Verify2FaResult {
    #[serde(rename = "verified")]
    pub verified: bool,
}

impl Verify2FaResult {
    pub fn new(verified: bool) -> Verify2FaResult {
        Verify2FaResult { verified }
    }
}
