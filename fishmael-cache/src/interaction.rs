use std::ops::Deref;

use fishmael_cache_core::{RedisKeyProvider, Streamable};
use fishmael_cache_derive::RedisFieldProvider;
use twilight_model::{application::interaction::Interaction, gateway::payload::incoming::InteractionCreate};


#[derive(RedisFieldProvider, Clone, Debug)]
pub struct StreamableInteraction {
    pub app_permissions: Option<u64>,
    pub application_id: u64,
    pub channel: Option<u64>,
    // pub data: Option<InteractionData>,
    pub guild_id: Option<u64>,
    pub guild_locale: Option<String>,
    pub id: u64,
    pub kind: u8,
    pub locale: Option<String>,
    pub message: Option<u64>,
    pub token: String,
    pub user: Option<u64>,
}

impl RedisKeyProvider for StreamableInteraction {
    fn get_key(&self) -> String {
        format!("interaction:{}", self.id)
    }
}

impl Streamable for StreamableInteraction {}

impl From<InteractionCreate> for StreamableInteraction {
    fn from(value: InteractionCreate) -> Self {
        Into::into(value.deref().clone())
    }
}

impl From<Interaction> for StreamableInteraction {
    fn from(value: Interaction) -> Self {
        StreamableInteraction {
            app_permissions: value.app_permissions.map(|f| f.bits()),
            application_id: value.application_id.into(),
            channel: value.channel.as_ref().map(|c| c.id.into()),
            guild_id: value.guild_id.map(Into::into),
            guild_locale: value.guild_locale,
            id: value.id.into(),
            kind: *&value.kind as u8,
            locale: value.locale,
            message: value.message.map(|m| m.id.into()),
            token: value.token,
            user: value.user.map(|u| u.id.into()),
        }
    }
}