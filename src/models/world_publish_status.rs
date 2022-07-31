/*
 * VRChat API Documentation
 *
 *
 * Contact: me@ariesclark.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorldPublishStatus {
    #[serde(rename = "canPubilsh")]
    pub can_pubilsh: bool,
}

impl WorldPublishStatus {
    pub fn new(can_pubilsh: bool) -> WorldPublishStatus {
        WorldPublishStatus { can_pubilsh }
    }
}
