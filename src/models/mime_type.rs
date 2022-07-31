/*
 * VRChat API Documentation
 *
 *
 * Contact: me@ariesclark.com
 * Generated by: https://openapi-generator.tech
 */

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MimeType {
    #[serde(rename = "image/jpeg")]
    ImageJpeg,
    #[serde(rename = "image/jpg")]
    ImageJpg,
    #[serde(rename = "image/png")]
    ImagePng,
    #[serde(rename = "image/webp")]
    ImageWebp,
    #[serde(rename = "image/gif")]
    ImageGif,
    #[serde(rename = "image/bmp")]
    ImageBmp,
    #[serde(rename = "image/svg＋xml")]
    ImageSvgxml,
    #[serde(rename = "image/tiff")]
    ImageTiff,
    #[serde(rename = "application/x-avatar")]
    ApplicationXAvatar,
    #[serde(rename = "application/x-world")]
    ApplicationXWorld,
    #[serde(rename = "application/gzip")]
    ApplicationGzip,
    #[serde(rename = "application/x-rsync-signature")]
    ApplicationXRsyncSignature,
    #[serde(rename = "application/x-rsync-delta")]
    ApplicationXRsyncDelta,
    #[serde(rename = "application/octet-stream")]
    ApplicationOctetStream,
}

impl ToString for MimeType {
    fn to_string(&self) -> String {
        match self {
            Self::ImageJpeg => String::from("image/jpeg"),
            Self::ImageJpg => String::from("image/jpg"),
            Self::ImagePng => String::from("image/png"),
            Self::ImageWebp => String::from("image/webp"),
            Self::ImageGif => String::from("image/gif"),
            Self::ImageBmp => String::from("image/bmp"),
            Self::ImageSvgxml => String::from("image/svg＋xml"),
            Self::ImageTiff => String::from("image/tiff"),
            Self::ApplicationXAvatar => String::from("application/x-avatar"),
            Self::ApplicationXWorld => String::from("application/x-world"),
            Self::ApplicationGzip => String::from("application/gzip"),
            Self::ApplicationXRsyncSignature => String::from("application/x-rsync-signature"),
            Self::ApplicationXRsyncDelta => String::from("application/x-rsync-delta"),
            Self::ApplicationOctetStream => String::from("application/octet-stream"),
        }
    }
}
