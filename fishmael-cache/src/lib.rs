use async_trait::async_trait;
use redis::{self, aio::ConnectionLike, Cmd, RedisError};
use serde::{self, Deserialize, Serialize};

use fishmael_model::{guild::{Guild, Permissions, SystemChannelFlags}, snowflake::{ChannelMarker, GuildMarker, Id, RoleMarker, UserMarker}};


pub struct Cache {
    pub client: redis::Client
}


impl Cache {
    pub fn from_url(url: &str) -> Self {
        Self {
            client: redis::Client::open(url)
                .unwrap_or_else(|_| panic!("Failed to open redis client with url {}", url))
        }
    }
}


#[derive(Deserialize, Serialize)]
pub struct CacheableGuild {
    pub afk_timeout: u32,
    pub application_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approximate_member_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approximate_presence_count: Option<u32>,
    pub banner: Option<String>,
    #[serde(default)]
    pub channels: Vec<Id<ChannelMarker>>,
    pub default_message_notifications: u8,  // TODO: implement defaultmessagenotifications struct
    pub description: Option<String>,
    pub discovery_splash: Option<String>,
    // pub emojis: Vec<Emoji>,
    pub explicit_content_filter: u8,
    // pub features: Vec<GuildFeature>,
    // #[serde(default, skip_serializing_if = "Vec::is_empty")]
    // pub guild_scheduled_events: Vec<GuildScheduledEvent>,
    pub icon: Option<String>,
    pub id: Id<GuildMarker>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub joined_at: Option<String>,  // TODO: timestamp struct
    pub large: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_members: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_presences: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_stage_video_channel_users: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_video_channel_users: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_count: Option<u64>,
    #[serde(default)]
    pub members: Vec<Id<UserMarker>>,
    pub mfa_level: u8,  // TODO: mfalevel struct
    pub name: String,
    pub nsfw_level: u8,  // TODO: nsfwlevel struct
    pub owner_id: Id<UserMarker>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Permissions>,
    pub preferred_locale: String,
    pub premium_progress_bar_enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_subscription_count: Option<u64>,
    #[serde(default)]
    pub premium_tier: u8,  // TODO: premiumtier struct
    // #[serde(default)]
    // pub presences: Vec<Presence>,
    pub public_updates_channel_id: Option<Id<ChannelMarker>>,
    pub roles: Vec<Id<RoleMarker>>,
    pub rules_channel_id: Option<Id<ChannelMarker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_alerts_channel_id: Option<Id<ChannelMarker>>,
    pub splash: Option<String>,
    // #[serde(skip_serializing_if = "Vec::is_empty")]
    // pub stage_instances: Vec<StageInstance>,
    // #[serde(skip_serializing_if = "Vec::is_empty")]
    // pub stickers: Vec<Sticker>,
    pub system_channel_flags: SystemChannelFlags,
    pub system_channel_id: Option<Id<ChannelMarker>>,
    #[serde(default)]
    pub threads: Vec<Id<ChannelMarker>>,
    #[serde(default)]
    pub unavailable: bool,
    pub vanity_url_code: Option<String>,
    pub verification_level: u8,  // TODO: verificationlevel struct
    // #[serde(default)]
    // pub voice_states: Vec<VoiceState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub widget_channel_id: Option<Id<ChannelMarker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub widget_enabled: Option<bool>,
}

impl From<&Guild> for CacheableGuild {
    fn from(value: &Guild) -> Self {
        Self{
            afk_timeout: value.afk_timeout,
            application_id: value.application_id,
            approximate_member_count: value.approximate_member_count,
            approximate_presence_count: value.approximate_presence_count,
            banner: value.banner.clone(),
            channels: value.channels.iter().map(|c| c.id).collect(),
            default_message_notifications: value.default_message_notifications,
            description: value.description.clone(),
            discovery_splash: value.discovery_splash.clone(),
            explicit_content_filter: value.explicit_content_filter,
            icon: value.icon.clone(),
            id: value.id,
            joined_at: value.joined_at.clone(),
            large: value.large,
            max_members: value.max_members,
            max_presences: value.max_presences,
            max_stage_video_channel_users: value.max_stage_video_channel_users,
            max_video_channel_users: value.max_video_channel_users,
            member_count: value.member_count,
            members: value.members.iter().map(|m| m.user.id).collect(),
            mfa_level: value.mfa_level,
            name: value.name.clone(),
            nsfw_level: value.nsfw_level,
            owner: value.owner,
            owner_id: value.owner_id,
            permissions: value.permissions,
            preferred_locale: value.preferred_locale.clone(),
            premium_progress_bar_enabled: value.premium_progress_bar_enabled,
            premium_subscription_count: value.premium_subscription_count,
            premium_tier: value.premium_tier,
            public_updates_channel_id: value.public_updates_channel_id,
            roles: value.roles.iter().map(|r| r.id).collect(),
            rules_channel_id: value.rules_channel_id,
            safety_alerts_channel_id: value.safety_alerts_channel_id,
            splash: value.splash.clone(),
            system_channel_flags: value.system_channel_flags,
            system_channel_id: value.system_channel_id,
            threads: value.threads.iter().map(|t| t.id).collect(),
            unavailable: value.unavailable,
            vanity_url_code: value.vanity_url_code.clone(),
            verification_level: value.verification_level,
            widget_channel_id: value.widget_channel_id,
            widget_enabled: value.widget_enabled,
        }
    }
}


#[async_trait]
pub trait Cacheable: Sized {
    fn get_key(&self) -> String;

    fn add_fields_to_cmd(&self, cmd: &mut Cmd) -> (); 

    async fn store(self, con: &mut (impl ConnectionLike + Send)) -> Result<(), RedisError> {
        let mut cmd = redis::cmd("HSET");
        cmd.arg(self.get_key());
        self.add_fields_to_cmd(&mut cmd);

        cmd.exec_async(con).await
    }
}


pub struct Foo {
    pub bar: String,
    pub baz: u64,
    pub id: u64,
}


impl Cacheable for Foo {
    fn get_key(&self) -> String {
        format!("foo:{}", self.id)
    }

    fn add_fields_to_cmd(&self, cmd: &mut Cmd) -> () {
        cmd.arg("bar")
            .arg(self.bar.clone())
            .arg("baz")
            .arg(self.baz)
            .arg("id")
            .arg(self.id);
    }


}


// #[async_trait]
// pub trait StoreCacheable: ConnectionLike + Send + Sized {
//     async fn set_model(&mut self, model: impl Cacheable + Send) -> Result<(), RedisError>
//     {
//         model.store(self).await
//     }
// }

// impl StoreCacheable for MultiplexedConnection {}
