use bitflags::bitflags;
use serde::{Deserialize, Serialize};

use super::{
    guild::Permissions,
    snowflake::{Id, IntegrationMarker, RoleMarker, SkuMarker, UserMarker},
    util::impl_serde_for_flags,
};


#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Role {
    pub color: u32,
    pub hoist: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    pub id: Id<RoleMarker>,
    pub managed: bool,
    pub mentionable: bool,
    pub name: String,
    pub permissions: Permissions,
    pub position: i64,
    pub flags: RoleFlags,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<RoleTags>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unicode_emoji: Option<String>,
}


bitflags! {
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    pub struct RoleFlags: u64 {
        const IN_PROMPT = 1 << 0;
    }
}

impl_serde_for_flags!(RoleFlags);


#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct RoleTags {
    #[serde(
        default,
        skip_serializing_if = "crate::util::is_false",
        with = "crate::util::null_bool",
    )]
    pub available_for_purchase: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<Id<UserMarker>>,
    #[serde(
        default,
        skip_serializing_if = "crate::util::is_false",
        with = "crate::util::null_bool",
    )]
    pub guild_connections: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_id: Option<Id<IntegrationMarker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_listing_id: Option<Id<SkuMarker>>,
    #[serde(
        default,
        skip_serializing_if = "crate::util::is_false",
        with = "crate::util::null_bool",
    )]
    pub premium_subscriber: bool,
}
