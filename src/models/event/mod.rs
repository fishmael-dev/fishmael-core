use serde::{Deserialize, Serialize};

pub mod guild_create;
pub mod hello;
pub mod identify;
pub mod ready;

use guild_create::GuildCreate;
use hello::Hello;
use identify::Identify;
use ready::Ready;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Payload {
    Bool(bool),
    Int(u64),
    OptInt(Option<u64>),
    Identify(Identify),
    Hello(Hello),
    Ready(Ready),
    GuildCreate(GuildCreate),
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
