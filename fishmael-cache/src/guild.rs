use fishmael_model::{
    guild::{Guild, Permissions, SystemChannelFlags},
    snowflake::{ChannelMarker, GuildMarker, Id, RoleMarker, UserMarker}
};
use redis::Cmd;

use crate::Cacheable;
use itertools::Itertools;


pub struct CacheableGuild {
    pub afk_timeout: u32,
    pub application_id: Option<u64>,
    pub approximate_member_count: Option<u32>,
    pub approximate_presence_count: Option<u32>,
    pub banner: Option<String>,
    pub channels: Vec<Id<ChannelMarker>>,
    pub default_message_notifications: u8,  // TODO: implement defaultmessagenotifications struct
    pub description: Option<String>,
    pub discovery_splash: Option<String>,
    // pub emojis: Vec<Emoji>,
    pub explicit_content_filter: u8,
    // pub features: Vec<GuildFeature>,
    // pub guild_scheduled_events: Vec<GuildScheduledEvent>,
    pub icon: Option<String>,
    pub id: Id<GuildMarker>,
    pub joined_at: Option<String>,  // TODO: timestamp struct
    pub large: bool,
    pub max_members: Option<u64>,
    pub max_presences: Option<u64>,
    pub max_stage_video_channel_users: Option<u32>,
    pub max_video_channel_users: Option<u32>,
    pub member_count: Option<u64>,
    pub members: Vec<Id<UserMarker>>,
    pub mfa_level: u8,  // TODO: mfalevel struct
    pub name: String,
    pub nsfw_level: u8,  // TODO: nsfwlevel struct
    pub owner_id: Id<UserMarker>,
    pub owner: Option<bool>,
    pub permissions: Option<Permissions>,
    pub preferred_locale: String,
    pub premium_progress_bar_enabled: bool,
    pub premium_subscription_count: Option<u64>,
    pub premium_tier: u8,  // TODO: premiumtier struct
    // pub presences: Vec<Presence>,
    pub public_updates_channel_id: Option<Id<ChannelMarker>>,
    pub roles: Vec<Id<RoleMarker>>,
    pub rules_channel_id: Option<Id<ChannelMarker>>,
    pub safety_alerts_channel_id: Option<Id<ChannelMarker>>,
    pub splash: Option<String>,
    // pub stage_instances: Vec<StageInstance>,
    // pub stickers: Vec<Sticker>,
    pub system_channel_flags: SystemChannelFlags,
    pub system_channel_id: Option<Id<ChannelMarker>>,
    pub threads: Vec<Id<ChannelMarker>>,
    pub unavailable: bool,
    pub vanity_url_code: Option<String>,
    pub verification_level: u8,  // TODO: verificationlevel struct
    // pub voice_states: Vec<VoiceState>,
    pub widget_channel_id: Option<Id<ChannelMarker>>,
    pub widget_enabled: Option<bool>,
}

impl From<Guild> for CacheableGuild {
    fn from(value: Guild) -> Self {
        Self {
            afk_timeout: value.afk_timeout,
            application_id: value.application_id,
            approximate_member_count: value.approximate_member_count,
            approximate_presence_count: value.approximate_presence_count,
            banner: value.banner,
            channels: value.channels.iter().map(|c| c.id).collect(),
            default_message_notifications: value.default_message_notifications,
            description: value.description.clone(),
            discovery_splash: value.discovery_splash,
            explicit_content_filter: value.explicit_content_filter,
            icon: value.icon.clone(),
            id: value.id,
            joined_at: value.joined_at,
            large: value.large,
            max_members: value.max_members,
            max_presences: value.max_presences,
            max_stage_video_channel_users: value.max_stage_video_channel_users,
            max_video_channel_users: value.max_video_channel_users,
            member_count: value.member_count,
            members: value.members.iter().map(|m| m.user.id).collect(),
            mfa_level: value.mfa_level,
            name: value.name,
            nsfw_level: value.nsfw_level,
            owner: value.owner,
            owner_id: value.owner_id,
            permissions: value.permissions,
            preferred_locale: value.preferred_locale,
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
            vanity_url_code: value.vanity_url_code,
            verification_level: value.verification_level,
            widget_channel_id: value.widget_channel_id,
            widget_enabled: value.widget_enabled,
        }
    }
}


impl Cacheable for CacheableGuild {
    fn get_key(&self) -> String {
        format!("guild:{}", self.id)
    }

    fn add_fields_to_cmd(&self, cmd: &mut Cmd) -> () {
        cmd.arg("afk_timeout")
            .arg(self.afk_timeout)
            .arg("application_id")
            .arg(self.application_id)
            .arg("approximate_member_count")
            .arg(self.approximate_member_count)
            .arg("approximate_presence_count")
            .arg(self.approximate_presence_count)
            .arg("banner")
            .arg(self.banner.clone())
            .arg("channels")
            .arg(self.channels.iter().map(|id| id.value()).join(","))
            .arg("default_message_notifications")
            .arg(self.default_message_notifications)
            .arg("description")
            .arg(self.description.clone())
            .arg("discovery_splash")
            .arg(self.discovery_splash.clone())
            .arg("explicit_content_filter")
            .arg(self.explicit_content_filter)
            .arg("icon")
            .arg(self.icon.clone())
            .arg("id")
            .arg(self.id.value())
            .arg("joined_at")
            .arg(self.joined_at.clone())
            .arg("large")
            .arg(self.large)
            .arg("max_members")
            .arg(self.max_members)
            .arg("max_presences")
            .arg(self.max_presences)
            .arg("max_stage_video_channel_users")
            .arg(self.max_stage_video_channel_users)
            .arg("max_video_channel_users")
            .arg(self.max_video_channel_users)
            .arg("member_count")
            .arg(self.member_count)
            .arg("members")
            .arg(self.members.iter().map(|id| id.value()).join(","))
            .arg("mfa_level")
            .arg(self.mfa_level)
            .arg("name")
            .arg(self.name.clone())
            .arg("nsfw_level")
            .arg(self.nsfw_level)
            .arg("owner")
            .arg(self.owner)
            .arg("owner_id")
            .arg(self.owner_id.value())
            .arg("permissions")
            .arg(self.permissions.map(|flag| flag.bits()))
            .arg("preferred_locale")
            .arg(self.preferred_locale.clone())
            .arg("premium_progress_bar_enabled")
            .arg(self.premium_progress_bar_enabled)
            .arg("premium_subscription_count")
            .arg(self.premium_subscription_count)
            .arg("premium_tier")
            .arg(self.premium_tier)
            .arg("public_updates_channel_id")
            .arg(self.public_updates_channel_id.map(|id| id.value()))
            .arg("roles")
            .arg(self.roles.iter().map(|id| id.value()).join(","))
            .arg("rules_channel_id")
            .arg(self.rules_channel_id.map(|id| id.value()))
            .arg("safety_alerts_channel_id")
            .arg(self.safety_alerts_channel_id.map(|id| id.value()))
            .arg("splash")
            .arg(self.splash.clone())
            .arg("system_channel_flags")
            .arg(self.system_channel_flags.bits())
            .arg("system_channel_id")
            .arg(self.system_channel_id.map(|id| id.value()))
            .arg("threads")
            .arg(self.threads.iter().map(|id| id.value()).join(","))
            .arg("unavailable")
            .arg(self.unavailable)
            .arg("vanity_url_code")
            .arg(self.vanity_url_code.clone())
            .arg("verification_level")
            .arg(self.verification_level)
            .arg("widget_channel_id")
            .arg(self.widget_channel_id.map(|id| id.value()))
            .arg("widget_enabled")
            .arg(self.widget_enabled);
    }
}
