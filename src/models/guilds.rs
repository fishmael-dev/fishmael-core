use serde::{Deserialize, Serialize};

use crate::models::snowflake::Id;

#[derive(Debug, Serialize, Deserialize)]
pub struct UnavailableGuild {
    pub id: Id,
    pub unavailable: bool,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Guild {
    id: Id,
    name: String,
    icon: Option<String>,
    // TODO?: icon_hash???
    splash: Option<String>,
    discovery_splash: Option<String>,
    #[serde(default)]
    owner: bool,
    owner_id: Id,
    // #[serde(default, deserialize_with="deserialize_number_from_string")]
    // permissions: u64,  // String in API docs.
    // #[serde(deserialize_with="deserialize_number_from_string")]
    // afk_channel_id: Option<u64>,
    // afk_timeout: u32,
    // #[serde(default)]
    // widget_enabled: bool,
    // #[serde(default, deserialize_with="deserialize_number_from_string")]
    // widget_channel_id: Option<u64>,
    // verification_level: u8,
    // default_message_notifications: u8,
    // explicit_content_filter: u8,
    // // TODO: roles
    // // TODO: emojis
    // // TODO: features
    // mfa_level: u8,
    // #[serde(deserialize_with="deserialize_number_from_string")]
    // application_id: Option<u64>,
    // #[serde(deserialize_with="deserialize_number_from_string")]
    // system_channel_id: Option<u64>,
    // system_channel_flags: u8,
    // #[serde(deserialize_with="deserialize_number_from_string")]
    // rules_channel_id: Option<u64>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // max_presences: Option<u64>,
    // #[serde(default, deserialize_with="deserialize_number_from_string")]
    // max_members: u64,
    // vanity_url_code: Option<String>,
    // description: Option<String>,
    // banner: Option<String>,
    // premium_tier: u8,
    // #[serde(default)]
    // premium_subscription_count: u32,
    // preferred_locale: String,
    // #[serde(default, deserialize_with="deserialize_number_from_string")]
    // public_updates_channel_id: u64,
    // #[serde(default)]
    // max_video_channel_users: u32,
    // #[serde(default)]
    // max_stage_video_channel_users: u32,
    // approximate_member_count: u32,
    // approximate_presence_count: u32,
    // // TODO: welcome_screen,
    // nsfw_level: u8,
    // // TODO: stickers,
    // premium_progress_bar_enabled: bool,
    // #[serde(deserialize_with="deserialize_number_from_string")]
    // safety_alerts_channel_id: Option<u64>,
}
