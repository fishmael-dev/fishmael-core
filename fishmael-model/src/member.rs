use bitflags::bitflags;
use serde::{Serialize, Deserialize};

use super::{
    snowflake::{Id, RoleMarker},
    user::User, util::impl_serde_for_flags,
};


#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Member {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    pub communication_disabled_until: Option<String>,  // TODO: timestamp struct
    pub deaf: bool,
    pub flags: MemberFlags,
    pub joined_at: Option<String>,  // TODO: timestamp struct
    pub mute: bool,
    pub nick: Option<String>,
    #[serde(default)]
    pub pending: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_since: Option<String>,  // TODO: timestamp struct,
    pub roles: Vec<Id<RoleMarker>>,
    pub user: User,
}


bitflags! {
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    pub struct MemberFlags: u64 {
        const DID_REJOIN = 1 << 0;
        const COMPLETED_ONBOARDING = 1 << 1;
        const BYPASSES_VERIFICATION = 1 << 2;
        const STARTED_ONBOARDING = 1 << 3;
    }
}


impl_serde_for_flags!(MemberFlags);
