/*
 * VRChat API Documentation
 *
 *
 * Contact: me@ariesclark.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InstancePlatforms {
    #[serde(rename = "android")]
    pub android: i32,
    #[serde(rename = "standalonewindows")]
    pub standalonewindows: i32,
}

impl InstancePlatforms {
    pub fn new(android: i32, standalonewindows: i32) -> InstancePlatforms {
        InstancePlatforms {
            android,
            standalonewindows,
        }
    }
}
