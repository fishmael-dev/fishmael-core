use serde::{de::Visitor, Deserialize, Serialize};
use serde_aux::field_attributes::deserialize_number_from_string;


#[derive(Debug, Serialize, Deserialize)]
pub struct UnavailableGuild {
    #[serde(deserialize_with="deserialize_number_from_string")]
    pub id: u64,
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
    // #[serde(default)]
    // owner: bool,
    // #[serde(deserialize_with="deserialize_number_from_string")]
    // owner_id: u64,
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
    // #[serde(default, deserialize_with="deserialize_number_from_string")]
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


#[derive(Debug)]
pub struct Id(u64);


impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self, f)
    }
}


impl<'de> Deserialize<'de> for Id {
    
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
    D: serde::Deserializer<'de> 
    {
        struct IdVisitor {}
    
        impl IdVisitor {
            const fn new() -> Self {
                Self {}
            }
        }

        impl<'de> Visitor<'de> for IdVisitor {
            type Value = Id;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a discord id")
            }

            fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<Self::Value, E> {
                println!("WHOA");
                Ok(
                    Id (
                        value.parse()
                            .map_err(|_| E::invalid_value(serde::de::Unexpected::Str(value), &"a u64 string"))?
                    )
                )
            }
        }

        deserializer.deserialize_str(IdVisitor::new())
    }
}


impl Serialize for Id {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_newtype_struct("Id", &self.to_string())
    }
}
