use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Resume {
    pub seq: u64,
    pub session_id: String,
    pub token: String,
}
