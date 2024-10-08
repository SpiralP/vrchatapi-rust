/*
 * VRChat API Documentation
 *
 *
 * Contact: me@ariesclark.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileVersion {
    #[serde(rename = "created_at")]
    pub created_at: String,
    /// Usually only present if `true`
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    #[serde(rename = "delta", skip_serializing_if = "Option::is_none")]
    pub delta: Option<Box<crate::models::FileData>>,
    #[serde(rename = "file", skip_serializing_if = "Option::is_none")]
    pub file: Option<Box<crate::models::FileData>>,
    #[serde(rename = "signature", skip_serializing_if = "Option::is_none")]
    pub signature: Option<Box<crate::models::FileData>>,
    #[serde(rename = "status")]
    pub status: crate::models::FileStatus,
    /// Incremental version counter, can only be increased.
    #[serde(rename = "version")]
    pub version: i32,
}

impl FileVersion {
    pub fn new(created_at: String, status: crate::models::FileStatus, version: i32) -> FileVersion {
        FileVersion {
            created_at,
            deleted: None,
            delta: None,
            file: None,
            signature: None,
            status,
            version,
        }
    }
}
