use bitflags::bitflags;
use serde::{Deserialize, Serialize};

use super::{
    member::Member,
    snowflake::{
        ApplicationMarker,
        ChannelMarker,
        EmojiMarker,
        GuildMarker,
        Id,
        MessageMarker,
        TagMarker,
        UserMarker,
    },
    user::User,
    util::impl_serde_for_flags
};


#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Channel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<Id<ApplicationMarker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applied_tags: Option<Vec<Id<TagMarker>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_tags: Option<Vec<ForumTag>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_auto_archive_duration: Option<AutoArchiveDuration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_forum_layout: Option<ForumLayout>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub default_reaction_emoji: Option<DefaultReaction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_sort_order: Option<ForumSortOrder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_thread_rate_limit_per_user: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<ChannelFlags>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<Id<GuildMarker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    pub id: Id<ChannelMarker>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invitable: Option<bool>,
    #[serde(rename = "type")]
    pub kind: ChannelType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_message_id: Option<Id<MessageMarker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_pin_timestamp: Option<String>,  // TODO: Timestamp struct
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<ThreadMember>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_count: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub newly_created: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsfw: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<Id<UserMarker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<Id<ChannelMarker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_overwrites: Option<Vec<PermissionOverwrite>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_per_user: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipients: Option<Vec<User>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rtc_region: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_metadata: Option<ThreadMetadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_limit: Option<u32>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub video_quality_mode: Option<VideoQualityMode>,
}


bitflags! {
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    pub struct ChannelFlags: u64 {
        const PINNED = 1 << 1;
        const REQUIRE_TAG = 1 << 4;
    }
}

impl_serde_for_flags!(ChannelFlags);


#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(from = "u8", into = "u8")]
pub enum ChannelType {
    GuildText,
    Private,
    GuildVoice,
    Group,
    GuildCategory,
    GuildAnnouncement,
    AnnouncementThread,
    PublicThread,
    PrivateThread,
    GuildStageVoice,
    GuildDirectory,
    GuildForum,
    GuildMedia,
    Unknown(u8),
}

impl From<u8> for ChannelType {
    fn from(value: u8) -> Self {
        match value {
            0 => ChannelType::GuildText,
            1 => ChannelType::Private,
            2 => ChannelType::GuildVoice,
            3 => ChannelType::Group,
            4 => ChannelType::GuildCategory,
            5 => ChannelType::GuildAnnouncement,
            10 => ChannelType::AnnouncementThread,
            11 => ChannelType::PublicThread,
            12 => ChannelType::PrivateThread,
            13 => ChannelType::GuildStageVoice,
            14 => ChannelType::GuildDirectory,
            15 => ChannelType::GuildForum,
            16 => ChannelType::GuildMedia,
            unknown => ChannelType::Unknown(unknown),
        }
    }
}

impl From<ChannelType> for u8 {
    fn from(value: ChannelType) -> Self {
        match value {
            ChannelType::GuildText => 0,
            ChannelType::Private => 1,
            ChannelType::GuildVoice => 2,
            ChannelType::Group => 3,
            ChannelType::GuildCategory => 4,
            ChannelType::GuildAnnouncement => 5,
            ChannelType::AnnouncementThread => 10,
            ChannelType::PublicThread => 11,
            ChannelType::PrivateThread => 12,
            ChannelType::GuildStageVoice => 13,
            ChannelType::GuildDirectory => 14,
            ChannelType::GuildForum => 15,
            ChannelType::GuildMedia => 16,
            ChannelType::Unknown(unknown) => unknown,
        }
    }
}


#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ThreadMember {
    // Values currently unknown and undocumented.
    pub flags: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Id<ChannelMarker>>,
    pub join_timestamp: String,  // TODO: timestamp struct
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<Member>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub presence: Option<Presence>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<Id<UserMarker>>,
}


#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ThreadMetadata {
    pub archived: bool,
    pub auto_archive_duration: AutoArchiveDuration,
    pub archive_timestamp: String,  // TODO: timestamp struct
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_timestamp: Option<String>,  // TODO: timestamp struct
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invitable: Option<bool>,
    #[serde(default)]
    pub locked: bool,
}


#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(from = "u8", into = "u8")]
pub enum ForumLayout {
    GalleryView,
    ListView,
    NotSet,
    Unknown(u8),
}

impl From<u8> for ForumLayout {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::NotSet,
            1 => Self::ListView,
            2 => Self::GalleryView,
            unknown => Self::Unknown(unknown),
        }
    }
}

impl From<ForumLayout> for u8 {
    fn from(value: ForumLayout) -> Self {
        match value {
            ForumLayout::NotSet => 0,
            ForumLayout::ListView => 1,
            ForumLayout::GalleryView => 2,
            ForumLayout::Unknown(unknown) => unknown,
        }
    }
}


#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(from = "u8", into = "u8")]
pub enum ForumSortOrder {
    CreationDate,
    LatestActivity,
    Unknown(u8),
}

impl From<u8> for ForumSortOrder {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::LatestActivity,
            1 => Self::CreationDate,
            unknown => Self::Unknown(unknown),
        }
    }
}

impl From<ForumSortOrder> for u8 {
    fn from(value: ForumSortOrder) -> Self {
        match value {
            ForumSortOrder::LatestActivity => 0,
            ForumSortOrder::CreationDate => 1,
            ForumSortOrder::Unknown(unknown) => unknown,
        }
    }
}


#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ForumTag {
    pub emoji_id: Option<Id<EmojiMarker>>,
    pub emoji_name: Option<String>,
    pub id: Id<TagMarker>,
    pub moderated: bool,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(from = "u16", into = "u16")]
pub enum AutoArchiveDuration {
    Hour,
    Day,
    ThreeDays,
    Week,
    Unknown { value: u16 },
}


impl AutoArchiveDuration {
    // In minutes...
    pub const fn number(self) -> u16 {
        match self {
            Self::Hour => 60,
            Self::Day => 1440,
            Self::ThreeDays => 4320,
            Self::Week => 10080,
            Self::Unknown { value } => value,
        }
    }
}

impl From<u16> for AutoArchiveDuration {
    fn from(value: u16) -> Self {
        match value {
            60 => Self::Hour,
            1440 => Self::Day,
            4320 => Self::ThreeDays,
            10080 => Self::Week,
            value => Self::Unknown { value },
        }
    }
}

impl From<AutoArchiveDuration> for u16 {
    fn from(value: AutoArchiveDuration) -> Self {
        value.number()
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct PermissionOverwrite {
    // pub allow: Permissions,
    // pub deny: Permissions,
    pub id: Id<ChannelMarker>,
    #[serde(rename = "type")]
    pub kind: PermissionOverwriteType,
}


#[derive(Clone, Copy, Debug, Serialize, Eq, Hash, PartialEq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PermissionOverwriteType {
    Member,
    Role,
    Unknown(u8),
}

impl From<u8> for PermissionOverwriteType {
    fn from(value: u8) -> Self {
        match value {
            0 => PermissionOverwriteType::Role,
            1 => PermissionOverwriteType::Member,
            unknown => PermissionOverwriteType::Unknown(unknown),
        }
    }
}

impl From<PermissionOverwriteType> for u8 {
    fn from(value: PermissionOverwriteType) -> Self {
        match value {
            PermissionOverwriteType::Member => 1,
            PermissionOverwriteType::Role => 0,
            PermissionOverwriteType::Unknown(unknown) => unknown,
        }
    }
}
