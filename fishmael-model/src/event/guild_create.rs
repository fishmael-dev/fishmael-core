use serde::{Deserialize, Serialize};

use crate::guild::{Guild, UnavailableGuild};


#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(untagged)]
pub enum GuildCreate {
    Available(Guild),
    Unavailable(UnavailableGuild),
}
