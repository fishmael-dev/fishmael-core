use std::{fmt::Debug, ops::Deref};

use anyhow::{bail, Error};
use itertools::Itertools;
use twilight_model::{
    application::interaction::{
        application_command::{CommandDataOption, CommandOptionValue},
        Interaction,
        InteractionData
    },
    gateway::{payload::incoming::InteractionCreate, ShardId},
};

use fishmael_cache_core::{RedisStreamKeyProvider, Streamable};
use fishmael_cache_derive::RedisFieldProvider;


#[derive(RedisFieldProvider, Clone, Debug)]
pub struct StreamableCommandInteraction {
    pub app_permissions: Option<u64>,
    pub application_id: u64,
    pub channel_id: Option<u64>,
    pub command_id: u64,
    pub guild_id: Option<u64>,
    pub guild_locale: Option<String>,
    pub id: u64,
    pub kind: u8,
    pub locale: Option<String>,
    pub options: String,
    // pub resolved_attachments: Vec<String>,
    pub resolved_channels: Vec<u64>,
    pub resolved_messages: Vec<u64>,
    pub resolved_roles: Vec<u64>,
    pub resolved_users: Vec<u64>,
    pub target_id: Option<u64>,
    pub token: String,
    pub user_id: u64,
}

impl RedisStreamKeyProvider for StreamableCommandInteraction {
    fn get_stream_key(&self, shard: &ShardId) -> String {
        return format!("command_interaction:{}", shard.number())
    }
}

impl Streamable for StreamableCommandInteraction {}

impl TryFrom<InteractionCreate> for StreamableCommandInteraction {
    type Error = Error;

    fn try_from(value: InteractionCreate) -> Result<Self, Error> {
        value.deref().clone().try_into()
    }
}

macro_rules! unwrap_resolved {
    ($d:expr, $n:ident) => {
        match &$d {
            Some(res) => {
                res.$n
                    .keys()
                    .map(|k| (*k).into())
                    .collect()
            },
            None => Vec::new(),
        }
    };
}

fn to_json(options: &[CommandDataOption]) -> String {
    format!(
        "{{{}}}",
        options.iter()
            .map(|o| {
                match &o.value {
                    CommandOptionValue::Attachment(a) => format!("\"{}\":\"{}\"", o.name, a),
                    CommandOptionValue::Boolean(b) => format!("\"{}\":\"{}\"", o.name, b),
                    CommandOptionValue::Channel(c) => format!("\"{}\":\"{}\"", o.name, c),
                    CommandOptionValue::Focused(f, _) => format!("\"{}\":\"{}\"", o.name, f),
                    CommandOptionValue::Integer(i) => format!("\"{}\":\"{}\"", o.name, i),
                    CommandOptionValue::Mentionable(m) => format!("\"{}\":\"{}\"", o.name, m),
                    CommandOptionValue::Number(n) => format!("\"{}\":\"{}\"", o.name, n),
                    CommandOptionValue::Role(r) => format!("\"{}\":\"{}\"", o.name, r),
                    CommandOptionValue::String(s) => format!("\"{}\":\"{}\"", o.name, s),
                    CommandOptionValue::User(u) => format!("\"{}\":\"{}\"", o.name, u),
                    _ => panic!("well shit"),
                }
            })
            .join(","),
    )
}

impl TryFrom<Interaction> for StreamableCommandInteraction {
    type Error = Error;

    fn try_from(value: Interaction) -> Result<Self, Error> {        
        match value.data {
            Some(InteractionData::ApplicationCommand(data)) => {
                let user_id = value.user
                    .map_or_else(
                        || value.member.and_then(|m| m.user.map(|u| u.id)),
                        |u| Some(u.id)
                    )
                    .expect("neither user nor member were provided")
                    .into();

                Ok(StreamableCommandInteraction {
                    app_permissions: value.app_permissions.map(|f| f.bits()),
                    application_id: value.application_id.into(),
                    channel_id: value.channel.as_ref().map(|c| c.id.into()),
                    command_id: data.id.into(),
                    guild_id: value.guild_id.map(Into::into),
                    guild_locale: value.guild_locale,
                    id: value.id.into(),
                    kind: value.kind as u8,
                    locale: value.locale,
                    options: to_json(&data.options),
                    // pub resolved_attachments: Vec<String>,
                    resolved_channels: unwrap_resolved!(data.resolved, channels),
                    resolved_messages: unwrap_resolved!(data.resolved, messages),
                    resolved_roles: unwrap_resolved!(data.resolved, roles),
                    resolved_users: unwrap_resolved!(data.resolved, users),
                    target_id: data.target_id.map(Into::into),
                    token: value.token,
                    user_id
                })
            },
            _ => bail!("expected a command interaction."),
        }
    }
}