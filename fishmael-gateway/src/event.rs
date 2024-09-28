use serde::Deserialize;
use serde_json::Value;
use tokio_tungstenite::tungstenite::protocol::CloseFrame;

use fishmael_model::event::{
    guild_create::GuildCreate,
    hello::Hello,
    identify::Identify,
    ready::Ready,
    Opcode,
    Payload,
};

#[derive(Deserialize)]
pub struct MinimalEvent {
    pub op: Opcode,
    pub d: Value,
    pub s: Option<u64>,
    pub t: Option<String>,
}


#[derive(Debug)]
pub enum Event {
    Heartbeat(Option<u64>),
    Hello(Hello),
    GatewayClose(Option<CloseFrame<'static>>),
    GuildCreate(GuildCreate),
    Identify(Identify),
    Ready(Ready),
}

impl Event {
    pub fn name(&self) -> &str {
        match self {
            Self::Heartbeat(_) => "Heartbeat",
            Self::Hello(_) => "Hello",
            Self::GatewayClose(_) => "GatewayClose",
            Self::GuildCreate(_) => "GuildCreate",
            Self::Identify(_) => "Identify",
            Self::Ready(_) => "Ready",
        }
    }
}

impl From<Payload> for Event {
    fn from(value: Payload) -> Self {
        match value {
            Payload::Heartbeat(v) => Self::Heartbeat(v),
            Payload::Hello(v) => Self::Hello(v),
            Payload::GuildCreate(v) => Self::GuildCreate(v),
            Payload::Identify(v) => Self::Identify(v),
            Payload::Ready(v) => Self::Ready(v), 
        }
    }
}