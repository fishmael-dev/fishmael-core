use serde::{Deserialize, Serialize};

use crate::models::{
    guilds::UnavailableGuild,
    users::User,
};


#[derive(Debug, Serialize, Deserialize)]
pub struct IdentifyProperties {
    pub os: String,
    pub browser: String,
    pub device: String,
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Payload {
    Bool(bool),
    Int(u64),
    OptInt(Option<u64>),
    Identify {
        token: String,
        properties: IdentifyProperties,
        intents: u64,
    },
    Hello {
        heartbeat_interval: u64,
    },
    Ready {
        v: u8,
        user: User,
        session_id: String,
        resume_gateway_url: String,
        guilds: Vec<UnavailableGuild>,
        #[serde(default)]
        shard: (u32, u32),
        // TODO: application
    },
    GuildCreate {
        joined_at: String,  // TODO: ISO8601 timestamp
        large: bool,
        #[serde(default)]
        unavailable: bool,
        member_count: u64,
        // TODO: voice_states,
        // TODO: members,
        // TODO: channels,
        // TODO: threads,
        // TODO: presences,
        // TODO: stage_instances,
        // TODO: guild_scheduled_events,
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct GatewayEvent {
    pub op: u8,
    pub d: Option<Payload>,
    pub s: Option<u64>,
    pub t: Option<String>,
}


impl GatewayEvent {
    pub fn new(op: u8, d: Payload) -> Self {
        GatewayEvent {
            op,
            d: Some(d),
            s: None,
            t: None
        }
    }
}
