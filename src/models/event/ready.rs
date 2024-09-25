use serde::{Deserialize, Serialize};

use crate::models::{guild::UnavailableGuild, user::User};


#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Ready {
    // TODO: application
    pub guilds: Vec<UnavailableGuild>,
    pub resume_gateway_url: String,
    pub session_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard: Option<(u32, u32)>,
    pub user: User,
    #[serde(rename = "v")]
    pub version: u64,
}
