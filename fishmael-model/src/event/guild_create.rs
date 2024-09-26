use serde::{Deserialize, Serialize};

use crate::{channel::Channel, member::Member};


#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct GuildCreate {
    #[serde(default)]
    pub channels: Vec<Channel>,
    // TODO: guild_scheduled_events,
    pub joined_at: String,  // TODO: ISO8601 timestamp
    pub large: bool,
    #[serde(default)]
    pub member_count: u64,
    // TODO: voice_states,
    #[serde(default)]
    pub members: Vec<Member>,
    // TODO: presences,
    // TODO: soundboard_sounds
    // TODO: stage_instances,
    #[serde(default)]
    pub threads: Vec<Channel>,
    pub unavailable: bool,
}
