use serde::de::DeserializeSeed;
use twilight_model::gateway::event::{GatewayEvent, GatewayEventDeserializer};

use crate::error::{ReceiveError, ReceiveErrorKind};

pub fn deserialize(event: String) -> Result<Option<GatewayEvent>, ReceiveError> {
    let Some(gateway_deserializer) = GatewayEventDeserializer::from_json(&event) else {
        return Err(ReceiveError {
            kind: ReceiveErrorKind::Deserializing { event },
            source: None,
        });
    };

    let mut json_deserializer = serde_json::Deserializer::from_str(&event);

    gateway_deserializer
        .deserialize(&mut json_deserializer)
        .map(Some)
        .map_err(|source| ReceiveError {
            kind: ReceiveErrorKind::Deserializing { event },
            source: Some(Box::new(source)),
    })
}
