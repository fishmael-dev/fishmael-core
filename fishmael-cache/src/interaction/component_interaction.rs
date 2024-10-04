use std::{fmt::Debug, ops::Deref};

use anyhow::{bail, Error};
use fishmael_cache_core::Streamable;
use fishmael_cache_derive::RedisFieldProvider;
use twilight_model::{
    application::interaction::{Interaction, InteractionData},
    gateway::payload::incoming::InteractionCreate,
};


#[derive(RedisFieldProvider, Clone, Debug)]
pub struct StreamableComponentInteraction {
    pub app_permissions: Option<u64>,
    pub application_id: u64,
    pub channel_id: Option<u64>,
    pub component_type: u8,
    pub custom_id: String,
    pub guild_id: Option<u64>,
    pub guild_locale: Option<String>,
    pub id: u64,
    pub kind: u8,
    pub locale: Option<String>,
    pub message_id: Option<u64>,
    pub token: String,
    pub user_id: Option<u64>,
    pub values: Vec<String>,
}

impl Streamable for StreamableComponentInteraction {}

impl TryFrom<InteractionCreate> for StreamableComponentInteraction {
    type Error = Error;

    fn try_from(value: InteractionCreate) -> Result<Self, Error> {
        value.deref().clone().try_into()
    }
}

impl TryFrom<Interaction> for StreamableComponentInteraction {
    type Error = Error;

    fn try_from(value: Interaction) -> Result<Self, Error> {
        match value.data {
            Some(InteractionData::MessageComponent(data)) => {
                Ok(StreamableComponentInteraction {
                    app_permissions: value.app_permissions.map(|f| f.bits()),
                    application_id: value.application_id.into(),
                    channel_id: value.channel.as_ref().map(|c| c.id.into()),
                    component_type: data.component_type.into(),
                    custom_id: data.custom_id,
                    guild_id: value.guild_id.map(Into::into),
                    guild_locale: value.guild_locale,
                    id: value.id.into(),
                    kind: value.kind as u8,
                    locale: value.locale,
                    message_id: value.message.map(|m| m.id.into()),
                    token: value.token,
                    user_id: value.user.map(|u| u.id.into()),
                    values: data.values,
                })
            },
            _ => bail!("expected a command interaction."),
        }
    }
}
