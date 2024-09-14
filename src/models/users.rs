use serde::{Deserialize, Serialize};
use serde_aux::field_attributes::deserialize_number_from_string;


#[derive(Debug, Serialize, Deserialize)]
pub struct AvatarDecorationData {
    pub asset: String,
    pub sku_id: u64,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(deserialize_with="deserialize_number_from_string")]
    pub id: u64,
    pub username: String,
    pub discriminator: String,
    #[serde(default)]
    pub global_name: Option<String>,
    #[serde(default)]
    pub avatar: Option<String>,
    #[serde(default)]
    pub bot: bool,
    #[serde(default)]
    pub system: bool,
    #[serde(default)]
    pub mfa_enabled: bool,
    #[serde(default)]
    pub banner: Option<String>,
    #[serde(default)]
    pub accent_color: Option<String>,
    #[serde(default)]
    pub locale: Option<String>,
    #[serde(default)]
    pub verified: bool,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default, deserialize_with="deserialize_number_from_string")]
    pub flags: u64,
    #[serde(default, deserialize_with="deserialize_number_from_string")]
    pub premium_type: u64,
    #[serde(default, deserialize_with="deserialize_number_from_string")]
    pub public_flags: u64,
    #[serde(default)]
    pub avatar_decoration_data: Option<AvatarDecorationData>,
}
