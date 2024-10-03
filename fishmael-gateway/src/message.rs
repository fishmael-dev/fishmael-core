use std::borrow::Cow;
use tokio_tungstenite::tungstenite::{
    protocol::frame::{
        coding::CloseCode,
        CloseFrame as WebsocketCloseFrame,
    },
    Message as WebsocketMessage,
};
use twilight_model::gateway::CloseFrame;


#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Message {
    Close(Option<CloseFrame<'static>>),
    Text(String),
}

impl Message {
    pub(crate) const ABNORMAL_CLOSE: Self = Self::Close(Some(CloseFrame::new(1006, "")));

    pub const fn is_close(&self) -> bool {
        matches!(self, Self::Close(_))
    }

    pub const fn is_text(&self) -> bool {
        matches!(self, Self::Text(_))
    }

    pub(crate) fn from_websocket_msg(msg: &WebsocketMessage) -> Option<Self> {
        if msg.is_close() {
            let (code, reason) = match msg {
                WebsocketMessage::Close(Some(frame)) => (frame.code, frame.reason.clone()),
                _ => unreachable!(),
            };

            let frame = (code == CloseCode::Status).then(|| CloseFrame {
                code: code.into(),
                reason: Cow::Owned(reason.to_string()),
            });

            Some(Self::Close(frame))
        } else if msg.is_text() {
            Some(Self::Text(msg.to_text().unwrap().to_owned()))
        } else {
            None
        }
    }

    pub(crate) fn into_websocket_msg(self) -> WebsocketMessage {
        match self {
            Self::Close(frame) => WebsocketMessage::Close(
                frame.map(|f| WebsocketCloseFrame{
                    code: CloseCode::try_from(f.code).unwrap(),
                    reason: f.reason,
                })
            ),
            Self::Text(string) => WebsocketMessage::text(string),
        }
    }
}
