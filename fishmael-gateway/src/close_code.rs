use tokio_tungstenite::tungstenite::protocol::{frame::coding::CloseCode, CloseFrame};
use std::fmt::{Display, Formatter, Result as FmtResult};
use anyhow::bail;


// https://discord.com/developers/docs/topics/opcodes-and-status-codes#gateway-gateway-close-event-codes
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum LibraryCloseCode {
    UnknownError,
    UnknownOpcode,
    DecodeError,
    NotAuthenticated,
    AuthenticationFailed,
    AlreadyAuthenticated,
    InvalidSequence,
    RateLimited,
    SessionTimedOut,
    InvalidShard,
    ShardingRequired,
    InvalidApiVersion,
    InvalidIntents,
    DisallowedIntents,
}

impl LibraryCloseCode {
    pub const RESUME: Self = Self::UnknownError;

    pub const fn can_reconnect(self) -> bool {
        matches!(
            self,
            Self::UnknownError
                | Self::UnknownOpcode
                | Self::DecodeError
                | Self::NotAuthenticated
                | Self::AlreadyAuthenticated
                | Self::InvalidSequence
                | Self::RateLimited
                | Self::SessionTimedOut
        )
    }

    pub fn into_frame<'t>(self) -> CloseFrame<'t> {
        CloseFrame { code: self.into(), reason: std::borrow::Cow::Owned("".to_string()) }
    }
}

impl Display for LibraryCloseCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(match self {
            LibraryCloseCode::UnknownError => "Unknown Error",
            LibraryCloseCode::UnknownOpcode => "Unknown Opcode",
            LibraryCloseCode::DecodeError => "Decode Error",
            LibraryCloseCode::NotAuthenticated => "Not Authenticated",
            LibraryCloseCode::AuthenticationFailed => "Authentication Failed",
            LibraryCloseCode::AlreadyAuthenticated => "Already Authenticated",
            LibraryCloseCode::InvalidSequence => "Invalid Sequence",
            LibraryCloseCode::RateLimited => "Rate Limited",
            LibraryCloseCode::SessionTimedOut => "Session Timed Out",
            LibraryCloseCode::InvalidShard => "Invalid Shard",
            LibraryCloseCode::ShardingRequired => "Sharding Required",
            LibraryCloseCode::InvalidApiVersion => "Invalid Api Version",
            LibraryCloseCode::InvalidIntents => "Invalid Intents",
            LibraryCloseCode::DisallowedIntents => "Disallowed Intents",
        })
    }
}

impl TryFrom<u16> for LibraryCloseCode {
    type Error = anyhow::Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        let close_code = match value {
            4000 => LibraryCloseCode::UnknownError,
            4001 => LibraryCloseCode::UnknownOpcode,
            4002 => LibraryCloseCode::DecodeError,
            4003 => LibraryCloseCode::NotAuthenticated,
            4004 => LibraryCloseCode::AuthenticationFailed,
            4005 => LibraryCloseCode::AlreadyAuthenticated,
            4007 => LibraryCloseCode::InvalidSequence,
            4008 => LibraryCloseCode::RateLimited,
            4009 => LibraryCloseCode::SessionTimedOut,
            4010 => LibraryCloseCode::InvalidShard,
            4011 => LibraryCloseCode::ShardingRequired,
            4012 => LibraryCloseCode::InvalidApiVersion,
            4013 => LibraryCloseCode::InvalidIntents,
            4014 => LibraryCloseCode::DisallowedIntents,
            _ => bail!("{} is not a valid gateway close code", value),
        };

        Ok(close_code)
    }
}

impl From<LibraryCloseCode> for u16 {
    fn from(value: LibraryCloseCode) -> Self {
        match value {
            LibraryCloseCode::UnknownError => 4000,
            LibraryCloseCode::UnknownOpcode => 4001,
            LibraryCloseCode::DecodeError => 4002,
            LibraryCloseCode::NotAuthenticated => 4003,
            LibraryCloseCode::AuthenticationFailed => 4004,
            LibraryCloseCode::AlreadyAuthenticated => 4005,
            LibraryCloseCode::InvalidSequence => 4007,
            LibraryCloseCode::RateLimited => 4008,
            LibraryCloseCode::SessionTimedOut => 4009,
            LibraryCloseCode::InvalidShard => 4010,
            LibraryCloseCode::ShardingRequired => 4011,
            LibraryCloseCode::InvalidApiVersion => 4012,
            LibraryCloseCode::InvalidIntents => 4013,
            LibraryCloseCode::DisallowedIntents => 4014,
        }
    }
}

impl From<LibraryCloseCode> for CloseCode {
    fn from(value: LibraryCloseCode) -> Self {
        CloseCode::Library(value.into())
    }
}