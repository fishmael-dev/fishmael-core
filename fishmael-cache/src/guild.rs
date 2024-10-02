use fishmael_cache_core::KeyProvider;
use fishmael_cache_derive::Cacheable;
use fishmael_model::{
    guild::{Guild, PartialGuild},
    snowflake::{ApplicationMarker, ChannelMarker, GuildMarker, Id, RoleMarker, UserMarker}
};

#[derive(Cacheable, Clone, Debug)]
pub struct CacheableGuild {
    pub afk_timeout: u32,
    pub application_id: Option<Id<ApplicationMarker>>,
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
    pub large: Option<bool>,
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
    pub permissions: Option<u64>,
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
    pub system_channel_flags: u64,
    pub system_channel_id: Option<Id<ChannelMarker>>,
    pub threads: Vec<Id<ChannelMarker>>,
    pub unavailable: bool,
    pub vanity_url_code: Option<String>,
    pub verification_level: u8,  // TODO: verificationlevel struct
    // pub voice_states: Vec<VoiceState>,
    pub widget_channel_id: Option<Id<ChannelMarker>>,
    pub widget_enabled: Option<bool>,
}

impl KeyProvider for CacheableGuild {
    fn get_key(&self) -> String {
        format!("guild:{}", self.id)
    }
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
            large: Some(value.large),
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
            permissions: value.permissions.map(|flag| flag.bits()),
            preferred_locale: value.preferred_locale,
            premium_progress_bar_enabled: value.premium_progress_bar_enabled,
            premium_subscription_count: value.premium_subscription_count,
            premium_tier: value.premium_tier,
            public_updates_channel_id: value.public_updates_channel_id,
            roles: value.roles.iter().map(|r| r.id).collect(),
            rules_channel_id: value.rules_channel_id,
            safety_alerts_channel_id: value.safety_alerts_channel_id,
            splash: value.splash.clone(),
            system_channel_flags: value.system_channel_flags.bits(),
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


impl From<PartialGuild> for CacheableGuild {
    fn from(value: PartialGuild) -> Self {
        Self {
            afk_timeout: value.afk_timeout,
            application_id: value.application_id,
            approximate_member_count: None,
            approximate_presence_count: None,
            banner: value.banner,
            channels: Vec::new(),
            default_message_notifications: value.default_message_notifications,
            description: value.description.clone(),
            discovery_splash: value.discovery_splash,
            explicit_content_filter: value.explicit_content_filter,
            icon: value.icon.clone(),
            id: value.id,
            joined_at: None,
            large: None,
            max_members: value.max_members,
            max_presences: value.max_presences,
            max_stage_video_channel_users: None,
            max_video_channel_users: None,
            member_count: value.member_count,
            members: Vec::new(),
            mfa_level: value.mfa_level,
            name: value.name,
            nsfw_level: value.nsfw_level,
            owner: value.owner,
            owner_id: value.owner_id,
            permissions: value.permissions.map(|flag| flag.bits()),
            preferred_locale: value.preferred_locale,
            premium_progress_bar_enabled: value.premium_progress_bar_enabled,
            premium_subscription_count: value.premium_subscription_count,
            premium_tier: value.premium_tier,
            public_updates_channel_id: value.public_updates_channel_id,
            roles: value.roles.iter().map(|r| r.id).collect(),
            rules_channel_id: value.rules_channel_id,
            safety_alerts_channel_id: None,
            splash: value.splash.clone(),
            system_channel_flags: value.system_channel_flags.bits(),
            system_channel_id: value.system_channel_id,
            threads: Vec::new(),
            unavailable: false,
            vanity_url_code: value.vanity_url_code,
            verification_level: value.verification_level,
            widget_channel_id: value.widget_channel_id,
            widget_enabled: value.widget_enabled,
        }
    }
}
