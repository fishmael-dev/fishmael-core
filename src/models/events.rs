use serde::{Deserialize, Serialize};

use crate::models::users::User;


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
