/*
 * VRChat API Documentation
 *
 *
 * Contact: me@ariesclark.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InfoPushData {
    #[serde(rename = "contentList", skip_serializing_if = "Option::is_none")]
    pub content_list: Option<Box<crate::models::DynamicContentRow>>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "imageUrl", skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "onPressed", skip_serializing_if = "Option::is_none")]
    pub on_pressed: Option<Box<crate::models::InfoPushDataClickable>>,
    #[serde(rename = "template", skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "article", skip_serializing_if = "Option::is_none")]
    pub article: Option<Box<crate::models::InfoPushDataArticle>>,
}

impl InfoPushData {
    pub fn new() -> InfoPushData {
        InfoPushData {
            content_list: None,
            description: None,
            image_url: None,
            name: None,
            on_pressed: None,
            template: None,
            version: None,
            article: None,
        }
    }
}
