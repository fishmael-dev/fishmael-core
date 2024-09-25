use bitflags::bitflags;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

use super::{
    snowflake::{Id, UserMarker},
    util::impl_serde_for_flags,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct AvatarDecorationData {
    pub asset: String,
    pub sku_id: u64,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub accent_color: Option<u32>,
    pub avatar: Option<String>,
    pub avatar_decoration: Option<String>,
    pub avatar_decoration_data: Option<AvatarDecorationData>,
    pub banner: Option<String>,
    #[serde(default)]
    pub bot: bool,
    pub discriminator: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<UserFlags>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_name: Option<String>,
    pub id: Id<UserMarker>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa_enabled: Option<bool>,
    #[serde(rename = "username")]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_type: Option<PremiumType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_flags: Option<UserFlags>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified: Option<bool>,
}


// https://discord.com/developers/docs/resources/user#user-object-user-flags
bitflags! {
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
    pub struct UserFlags: u64 {
        const STAFF = 1;
        const PARTNER = 1 << 1;
        const HYPESQUAD = 1 << 2;
        const BUG_HUNTER_LEVEL_1 = 1 << 3;
        const HYPESQUAD_ONLINE_HOUSE_1 = 1 << 6;
        const HYPESQUAD_ONLINE_HOUSE_2 = 1 << 7;
        const HYPESQUAD_ONLINE_HOUSE_3 = 1 << 8;
        const PREMIUM_EARLY_SUPPORTER = 1 << 9;
        const TEAM_PSEUDO_USER = 1 << 10;
        const BUG_HUNTER_LEVEL_2 = 1 << 14;
        const VERIFIED_BOT = 1 << 16;
        const VERIFIED_DEVELOPER = 1 << 17;
        const MODERATOR_PROGRAMS_ALUMNI = 1 << 18;
        const BOT_HTTP_INTERACTIONS = 1 << 19;
        const ACTIVE_DEVELOPER = 1 << 22;
    } 
}

impl_serde_for_flags!(UserFlags);


#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(untagged, from="u8", into="u8")]
pub enum PremiumType {
    None,
    NitroClassic,
    Nitro,
    NitroBasic,
    Unknown(u8),
}


impl From<u8> for PremiumType {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::None,
            1 => Self::NitroClassic,
            2 => Self::Nitro,
            3 => Self::NitroBasic,
            unknown => Self::Unknown(unknown),
        }
    }
}

impl From<PremiumType> for u8 {
    fn from(value: PremiumType) -> Self {
        match value {
            PremiumType::None => 0,
            PremiumType::NitroClassic => 1,
            PremiumType::Nitro => 2,
            PremiumType::NitroBasic => 2,
            PremiumType::Unknown(unknown) => unknown,
        }
    }
}
