use serde::{Deserialize, Serialize};

use crate::models::intents::Intents;


#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct IdentifyProperties {
    pub browser: String,
    pub device: String,
    pub os: String,
}


#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Identify {
    pub compress: bool,
    pub intents: Intents,
    pub large_threshold: u64,
    // TODO: presence
    pub properties: IdentifyProperties,
    pub shard: Option<ShardId>,
    pub token: String,
}


#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(from = "[u32; 2]", into = "[u32; 2]")]
pub struct ShardId {
    number: u32,
    total: u32,
}


impl ShardId {
    pub const fn new(number: u32, total: u32) -> Self {
        assert!(total > 0, "total must be at least 1");
        assert!(number < total, "number must be less than total");
        Self {number, total}
    }

    pub const fn number(self) -> u32 {
        self.number
    } 

    pub const fn total(self) -> u32 {
        self.total
    } 
}


impl From<[u32; 2]> for ShardId {
    fn from([number, total]: [u32; 2]) -> Self {
        Self::new(number, total)
    }
}


impl From<ShardId> for [u32; 2] {
    fn from(id: ShardId) -> Self {
        [id.number(), id.total()]
    }
}
