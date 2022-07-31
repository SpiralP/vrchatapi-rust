/*
 * VRChat API Documentation
 *
 *
 * Contact: me@ariesclark.com
 * Generated by: https://openapi-generator.tech
 */

/// AvatarUnityPackageUrlObject : **Deprecation:** `Object` has unknown usage/fields, and is always empty. Use normal `Url` field instead.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AvatarUnityPackageUrlObject {
    #[serde(rename = "unityPackageUrl", skip_serializing_if = "Option::is_none")]
    pub unity_package_url: Option<String>,
}

impl AvatarUnityPackageUrlObject {
    /// **Deprecation:** `Object` has unknown usage/fields, and is always empty. Use normal `Url` field instead.
    pub fn new() -> AvatarUnityPackageUrlObject {
        AvatarUnityPackageUrlObject {
            unity_package_url: None,
        }
    }
}
