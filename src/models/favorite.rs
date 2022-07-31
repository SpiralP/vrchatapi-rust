/*
 * VRChat API Documentation
 *
 *
 * Contact: me@ariesclark.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Favorite {
    /// MUST be either AvatarID, UserID or WorldID.
    #[serde(rename = "favoriteId")]
    pub favorite_id: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
    #[serde(rename = "type")]
    pub _type: crate::models::FavoriteType,
}

impl Favorite {
    pub fn new(
        favorite_id: String,
        id: String,
        tags: Vec<String>,
        _type: crate::models::FavoriteType,
    ) -> Favorite {
        Favorite {
            favorite_id,
            id,
            tags,
            _type,
        }
    }
}
