use serde::{Deserialize, Serialize};

pub mod guild_create;
pub mod hello;
pub mod identify;
pub mod ready;

use guild_create::GuildCreate;
use hello::Hello;
use identify::Identify;
use ready::Ready;


#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(from = "u8", into = "u8")]
pub enum Opcode {
    Dispatch,
    Heartbeat,
    Identify,
    Presence,
    VoiceState,
    VoicePing,
    Resume,
    Reconnect,
    RequestMembers,
    InvalidateSession,
    Hello,
    ACK,
    GuildSync,
    Unknown(u8),
}

impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Dispatch,
            1 => Self::Heartbeat,
            2 => Self::Identify,
            3 => Self::Presence,
            4 => Self::VoiceState,
            5 => Self::VoicePing,
            6 => Self::Resume,
            7 => Self::Reconnect,
            8 => Self::RequestMembers,
            9 => Self::InvalidateSession,
            10 => Self::Hello,
            11 => Self::ACK,
            12 => Self::GuildSync,
            unknown => Self::Unknown(unknown),
        }
    }
}

impl From<Opcode> for u8 {
    fn from(value: Opcode) -> Self {
        match value {
            Opcode::Dispatch => 0,
            Opcode::Heartbeat => 1,
            Opcode::Identify => 2,
            Opcode::Presence => 3,
            Opcode::VoiceState => 4,
            Opcode::VoicePing => 5,
            Opcode::Resume => 6,
            Opcode::Reconnect => 7,
            Opcode::RequestMembers => 8,
            Opcode::InvalidateSession => 9,
            Opcode::Hello => 10,
            Opcode::ACK => 11,
            Opcode::GuildSync => 12,
            Opcode::Unknown(unknown) => unknown,
        }
    }
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Payload {
    Heartbeat(Option<u64>),
    Identify(Identify),
    Hello(Hello),
    Ready(Ready),
    GuildCreate(GuildCreate),
}


#[derive(Debug, Serialize, Deserialize)]
pub struct GatewayEvent {
    pub op: Opcode,
    pub d: Option<Payload>,
    pub s: Option<u64>,
    pub t: Option<String>,
}


impl GatewayEvent {
    pub fn new(op: Opcode, d: Payload) -> Self {
        GatewayEvent {
            op,
            d: Some(d),
            s: None,
            t: None
        }
    }
}
