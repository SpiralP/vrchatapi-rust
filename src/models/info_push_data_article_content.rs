/*
 * VRChat API Documentation
 *
 *
 * Contact: me@ariesclark.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InfoPushDataArticleContent {
    #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "imageUrl", skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(rename = "onPressed", skip_serializing_if = "Option::is_none")]
    pub on_pressed: Option<Box<crate::models::InfoPushDataClickable>>,
}

impl InfoPushDataArticleContent {
    pub fn new() -> InfoPushDataArticleContent {
        InfoPushDataArticleContent {
            text: None,
            image_url: None,
            on_pressed: None,
        }
    }
}
