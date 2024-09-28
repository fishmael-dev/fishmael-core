use bitflags::bitflags;
use serde::{Deserialize, Serialize};

use super::{
    channel::Channel,
    member::Member,
    role::Role,
    snowflake::{ChannelMarker, GuildMarker, Id, UserMarker},
    util::impl_serde_for_flags,
};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct UnavailableGuild {
    pub id: Id<GuildMarker>,
    pub unavailable: bool,
}


#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Guild {
    pub afk_channel_id: Option<u64>,
    pub afk_timeout: u32,
    pub application_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approximate_member_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approximate_presence_count: Option<u32>,
    pub banner: Option<String>,
    #[serde(default)]
    pub channels: Vec<Channel>,
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
    max_members: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_presences: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_stage_video_channel_users: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_video_channel_users: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_count: Option<u64>,
    #[serde(default)]
    pub members: Vec<Member>,
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
    pub roles: Vec<Role>,
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
    pub threads: Vec<Channel>,
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


bitflags! {
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    pub struct Permissions: u64 {
        const CREATE_INVITE = 1;
        const KICK_MEMBERS = 1 << 1;
        const BAN_MEMBERS = 1 << 2;
        const ADMINISTRATOR = 1 << 3;
        const MANAGE_CHANNELS = 1 << 4;
        const MANAGE_GUILD = 1 << 5;
        const ADD_REACTIONS = 1 << 6;
        const VIEW_AUDIT_LOG = 1 << 7;
        const PRIORITY_SPEAKER = 1 << 8;
        const STREAM = 1 << 9;
        const VIEW_CHANNEL = 1 << 10;
        const SEND_MESSAGES = 1 << 11;
        const SEND_TTS_MESSAGES = 1 << 12;
        const MANAGE_MESSAGES = 1 << 13;
        const EMBED_LINKS = 1 << 14;
        const ATTACH_FILES = 1 << 15;
        const READ_MESSAGE_HISTORY = 1 << 16;
        const MENTION_EVERYONE = 1 << 17;
        const USE_EXTERNAL_EMOJIS = 1 << 18;
        const VIEW_GUILD_INSIGHTS = 1 << 19;
        const CONNECT = 1 << 20;
        const SPEAK = 1 << 21;
        const MUTE_MEMBERS = 1 << 22;
        const DEAFEN_MEMBERS = 1 << 23;
        const MOVE_MEMBERS = 1 << 24;
        const USE_VAD = 1 << 25;
        const CHANGE_NICKNAME = 1 << 26;
        const MANAGE_NICKNAMES = 1 << 27;
        const MANAGE_ROLES = 1 << 28;
        const MANAGE_WEBHOOKS = 1 << 29;
        const MANAGE_EMOJIS_AND_STICKERS = 1 << 30;
        const MANAGE_GUILD_EXPRESSIONS = 1 << 30;
        const USE_SLASH_COMMANDS = 1 << 31;
        const REQUEST_TO_SPEAK = 1 << 32;
        const MANAGE_EVENTS = 1 << 33;
        const MANAGE_THREADS = 1 << 34;
        const CREATE_PUBLIC_THREADS = 1 << 35;
        const CREATE_PRIVATE_THREADS = 1 << 36;
        const USE_EXTERNAL_STICKERS = 1 << 37;
        const SEND_MESSAGES_IN_THREADS = 1 << 38;
        const USE_EMBEDDED_ACTIVITIES = 1 << 39;
        const MODERATE_MEMBERS = 1 << 40;
        const VIEW_CREATOR_MONETIZATION_ANALYTICS = 1 << 41;
        const USE_SOUNDBOARD = 1 << 42;
        const USE_EXTERNAL_SOUNDS = 1 << 45;
        const SEND_VOICE_MESSAGES = 1 << 46;
        const SEND_POLLS = 1 << 49;
        const USE_EXTERNAL_APPS = 1 << 50;
    }
}


bitflags! {
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    pub struct SystemChannelFlags: u64 {
        const SUPPRESS_JOIN_NOTIFICATIONS = 1;
        const SUPPRESS_PREMIUM_SUBSCRIPTIONS = 1 << 1;
        const SUPPRESS_GUILD_REMINDER_NOTIFICATIONS = 1 << 2;
        const SUPPRESS_JOIN_NOTIFICATION_REPLIES = 1 << 3;
        const SUPPRESS_ROLE_SUBSCRIPTION_PURCHASE_NOTIFICATIONS = 1 << 4;
        const SUPPRESS_ROLE_SUBSCRIPTION_PURCHASE_NOTIFICATION_REPLIES = 1 << 5;
    }
}

impl_serde_for_flags!(Permissions, SystemChannelFlags);
