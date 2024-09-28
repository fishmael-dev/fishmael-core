use std::fmt::{Display, Formatter, Result as FmtResult};


#[derive(Debug)]
pub enum ReceiveErrorKind {
    Deserializing{
        event: String,
    },
    Reconnect,
}


#[derive(Debug)]
pub struct ReceiveError {
    pub(crate) kind: ReceiveErrorKind,
    pub(crate) source: Option<Box<dyn std::error::Error + Send + Sync>>,
}


impl Display for ReceiveError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.kind {
            ReceiveErrorKind::Deserializing { event } => {
                f.write_str("failed to deserialize event: ")?;
                f.write_str(event)
            },
            ReceiveErrorKind::Reconnect => f.write_str("failed to reconnect")
        }
    }
}

impl std::error::Error for ReceiveError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source
            .as_ref()
            .map(|source| &**source as &(dyn std::error::Error + 'static))
    }
}
