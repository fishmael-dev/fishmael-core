use anyhow::{Context, Result};
use error::ReceiveErrorKind;
// use event::MinimalEvent;
use futures::Sink;
use futures_core::Stream;
use rand::{rngs::StdRng, Rng, SeedableRng};
use serde::{de::DeserializeOwned, Deserialize};
use std::{
    env,
    future::Future,
    io::ErrorKind as IoErrorKind,
    mem,
    pin::Pin,
    task::{ready, Context as AsyncContext, Poll},
    time::Duration,
};
use tokio::{
    net::TcpStream,
    time::{self, Instant, Interval, MissedTickBehavior}
};
use tokio_tungstenite::{
    connect_async,
    tungstenite::{
        protocol::frame::coding::CloseCode,
        Error as WebsocketError,
    },
    MaybeTlsStream,
    WebSocketStream,
};
use twilight_model::gateway::{
    event::GatewayEventDeserializer, payload::{
        incoming::{Hello, Ready},
        outgoing::{identify::{IdentifyInfo, IdentifyProperties},
        Heartbeat,
        Identify,
        Resume}
    }, CloseCode as LibraryCloseCode, CloseFrame, OpCode
};

pub use twilight_model::gateway::{
    Intents,
    ShardId,
    event::Event,
};

pub mod close_code;
pub mod deserialize;
pub mod error;
pub mod event;
pub mod message;
pub mod poll_event;

use crate::{
    error::ReceiveError,
    poll_event::PollEvent,
    message::Message,
};


const GATEWAY_URL: &str = "wss://gateway.discord.gg";
const API_VERSION: u8 = 10;


type Connection = WebSocketStream<MaybeTlsStream<TcpStream>>;

struct ConnectionFuture(Pin<Box<dyn Future<Output = Result<Connection, WebsocketError>> + Send>>);


#[derive(Deserialize)]
struct MinimalEvent<T> {
    /// Attached data of the gateway event.
    #[serde(rename = "d")]
    data: T,
}


pub struct Session {
    id: Box<str>,
    sequence: u64,
}


impl Session {
    pub fn new(sequence: u64, session_id: String) -> Self {
        Self {
            id: session_id.into_boxed_str(),
            sequence,
        }
    }

    pub const fn id(&self) -> &str {
        &self.id
    }

    pub const fn sequence(&self) -> u64 {
        self.sequence
    }

    pub(crate) fn set_sequence(&mut self, sequence: u64) -> u64 {
        mem::replace(&mut self.sequence, sequence)
    } 
}


#[derive(Clone, Debug)]
enum CloseInitiator {
    Gateway(Option<u16>),
    Shard(CloseFrame<'static>),
    Transport,
}


#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ShardState {
    Active,
    Disconnected{reconnect_attempts: u8},
    FatallyClosed,
    Identifying,
    /// In the middle of event playback during resuming.
    Resuming,
}


impl ShardState {
    pub fn from_close_code(close_code: u16) -> Self {
        match LibraryCloseCode::try_from(close_code) {
            Ok(close_code) if !close_code.can_reconnect() => Self::FatallyClosed,
            _ => Self::Disconnected { reconnect_attempts: 0 },
        }
    }
}


pub struct Shard {
    connection: Option<Connection>,
    connection_future: Option<ConnectionFuture>,
    heartbeat_interval: Option<Interval>,
    identified: bool,
    intents: Intents,
    pending: Option<Message>,
    resume_gateway_url: Option<String>,
    rng: StdRng,
    session: Option<Session>,
    shard_id: ShardId,
    state: ShardState,
    token: String,
}


impl Shard {

    pub fn new(
        token: String,
        shard_id: ShardId,
        intents: Intents,
    ) -> Self {
        Self {
            connection: None,
            connection_future: None,
            heartbeat_interval: None,
            identified: false,
            intents,
            pending: None,
            resume_gateway_url: None,
            rng: StdRng::from_entropy(),
            session: None,
            shard_id,
            state: ShardState::Disconnected{reconnect_attempts: 0},
            token,
        }
    }

    pub fn id(&self) -> ShardId {
        self.shard_id
    }

    pub fn state(&self) -> ShardState {
        self.state
    }

    fn disconnect(&mut self, initiator: CloseInitiator) {
        self.heartbeat_interval = None;
        self.state = match initiator {
            CloseInitiator::Gateway(Some(close_code)) => ShardState::from_close_code(close_code),
            _ => ShardState::Disconnected{reconnect_attempts: 0},
        };

        if let CloseInitiator::Shard(frame) = initiator {
            // Normal closure so we don't reconnect.
            if matches!(frame.code.into(), CloseCode::Normal | CloseCode::Away) {
                self.resume_gateway_url = None;
                self.session = None;
            }
            self.pending = Some(Message::Close(Some(frame)));
        }
    }

    fn parse_event<T: DeserializeOwned>(
        json: &str,
    ) -> Result<MinimalEvent<T>, ReceiveError> {
        serde_json::from_str::<MinimalEvent<T>>(json).map_err(|source| ReceiveError {
            kind: ReceiveErrorKind::Deserializing {
                event: json.to_owned(),
            },
            source: Some(Box::new(source)),
        })
    }

    fn process(&mut self, event: &str) -> Result<()> {
        let (raw_opcode, maybe_sequence, maybe_event_type) =
        GatewayEventDeserializer::from_json(event)
            .ok_or(ReceiveError {
                kind: ReceiveErrorKind::Deserializing {
                    event: event.to_owned(),
                },
                source: Some("missing opcode".into()),
            })?
            .into_parts();


        match OpCode::from(raw_opcode) {
            Some(OpCode::Dispatch) => {
                let event_type = maybe_event_type
                    .context("failed to get event type")?;
                let sequence = maybe_sequence
                    .context("failed to get sequence")?;

                match event_type.as_ref() {
                    "READY" => {
                        let event = Self::parse_event::<Ready>(event)
                            .context("failed to deserialise ready event")?;

                        self.resume_gateway_url = Some(event.data.resume_gateway_url);
                        self.session = Some(Session::new(sequence, event.data.session_id));
                        self.state = ShardState::Active;
                    },
                    "RESUMED" => {
                        self.state = ShardState::Active;
                    }
                    _ => {}
                }

                if let Some(session) = self.session.as_mut() {
                    session.set_sequence(sequence);
                }
            },
            Some(OpCode::Heartbeat) => {
                self.pending = Some(Message::Text(
                    serde_json::to_string(
                        &Heartbeat::new(self.session.as_ref().map(|s| s.sequence)),
                    )
                    .expect("failed to serialise heartbeat")
                ));
            },
            Some(OpCode::HeartbeatAck) => {
                println!("heartbeat ack received.");
                // TODO: track heartbeat responses to check if connection is still alive.
            }
            Some(OpCode::Hello) => {
                let event = Self::parse_event::<Hello>(event)
                    .context("failed to deserialise hello event")?;

                let heartbeat_interval = Duration::from_millis(event.data.heartbeat_interval);
                let jitter = Duration::from_millis(self.rng.gen_range(0..event.data.heartbeat_interval));

                let mut interval = time::interval_at(Instant::now() + jitter, heartbeat_interval);
                interval.set_missed_tick_behavior(MissedTickBehavior::Delay);
                self.heartbeat_interval = Some(interval);

                if let Some(session) = &self.session {
                    self.pending = Some(Message::Text(
                        serde_json::to_string(&Resume::new(
                            session.sequence(),
                            session.id(),
                            self.token.clone(),
                        ))
                        .expect("failed to serialise resume event"),
                    ));
                    self.state = ShardState::Resuming;
                }
            }
            Some(OpCode::Reconnect) => {
                println!("Got reconnect!");
                self.disconnect(CloseInitiator::Shard(CloseFrame::RESUME));
            },
            _ => println!("received an unknown opcode: {}", raw_opcode)
        }

        Ok(())
    }

    fn poll_handle_pending(&mut self, cx: &mut AsyncContext<'_>) -> Poll<Result<(), WebsocketError>> {
        println!("Polling pending event...");

        if self.pending.is_none() {
            println!("Polling pending event... no event pending-- done!");
            return Poll::Ready(Ok(()));
        }

        println!("Polling connection state for sending...");
        if let Err(e) = ready!(Pin::new(self.connection.as_mut().unwrap()).poll_ready(cx)) {
            println!("Polling connection state for sending... failed!");

            self.disconnect(CloseInitiator::Transport);
            self.connection = None;
            return Poll::Ready(Err(e));
        }
        println!("Polling connection state for sending... done!");

        let pending = self.pending.as_mut();

        println!("sending event...");
        if let Some(_message) = &pending {
            // TODO: ratelimiting

            let ws_message = pending.unwrap().clone().into_websocket_msg();
            if let Err(e) = Pin::new(self.connection.as_mut().unwrap()).start_send(ws_message) {
                println!("sending event... failed!");

                self.disconnect(CloseInitiator::Transport);
                self.connection = None;
                return Poll::Ready(Err(e));
            }
        }

        println!("polling completion of sending event...");
        if let Err(e) = ready!(Pin::new(self.connection.as_mut().unwrap()).poll_flush(cx)) {
            println!("polling completion of sending event... failed!");

            self.disconnect(CloseInitiator::Transport);
            self.connection = None;
            return Poll::Ready(Err(e));
        }
        println!("polling completion of sending event... done!");

        self.pending = None;

        Poll::Ready(Ok(()))
    }

    pub fn next_event(&mut self) -> PollEvent<Self> {
        PollEvent::new(self)
    }

}

impl Stream for Shard {
    type Item = Result<Message, ReceiveError>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut AsyncContext<'_>) -> Poll<Option<Self::Item>> {
        let message = loop {
            // Ensure connection...
            println!("loop start; connection={:?}, connection_future={:?}", self.connection.is_none(), self.connection_future.is_none());

            match self.state {
                ShardState::FatallyClosed => {
                    _ = ready!(
                        Pin::new(
                            self.connection
                                .as_mut()
                                .expect("connection should still exist")
                        )
                        .poll_close(cx)
                    );
                    self.connection = None;
                    return Poll::Ready(None);
                },
                ShardState::Disconnected { reconnect_attempts } if self.connection.is_none() => {
                    if self.connection_future.is_none() {
                        println!("setting up connection...");
    
                        let base_url = self.resume_gateway_url
                            .as_deref()
                            .unwrap_or(GATEWAY_URL);
            
                        let gateway_url = format!("{base_url}/?v={API_VERSION}&encoding=json");
        
                        self.connection_future = Some(ConnectionFuture(Box::pin(async move {
                            Ok(connect_async(&gateway_url).await?.0)
                        })));
    
                        println!("setting up connection... done!");
                    }

                    println!("polling connection...");
                    let res = ready!(Pin::new(&mut self.connection_future.as_mut().unwrap().0).poll(cx));
                    println!("polling connection... done!");
    
                    // This code is only reachable after ready! returns a completed poll;
                    // i.e. after a successful connection
                    self.connection_future = None;
        
                    match res {
                        Ok(connection) => {
                            println!("connection established!");
    
                            self.connection = Some(connection);
                            self.state = ShardState::Identifying;
                        }
                        Err(err) => {
                            println!("connection failed!");
    
                            self.resume_gateway_url = None;
                            self.state = ShardState::Disconnected{
                                reconnect_attempts: reconnect_attempts + 1
                            };
                            
                            return Poll::Ready(Some(Err(ReceiveError {
                                kind: ReceiveErrorKind::Reconnect,
                                source: Some(Box::new(err)),
                            })))
                        }
                    }
                },
                _ => {},
            }

            // TODO: implement and handle user closing 

            println!("polling heartbeat...");

            if self.heartbeat_interval
                .as_mut()
                .map_or(false, |interval| interval.poll_tick(cx).is_ready())
            {
                println!("sending heartbeat...");
                // TODO: Handle zombied connection
                self.pending = Some(Message::Text(
                    serde_json::to_string(
                        &Heartbeat::new(self.session.as_ref().map(|s| s.sequence))
                    )
                    .expect("failed to serialise heartbeat")
                ));
                
                println!("polling heartbeat status...");

                if ready!(self.poll_handle_pending(cx)).is_err() {
                    println!("polling heartbeat errored!");

                    return Poll::Ready(Some(Ok(Message::ABNORMAL_CLOSE)));
                }

                println!("sending heartbeat done!...");
            }

            if !self.identified {
                self.pending = Some(Message::Text(
                    serde_json::to_string(
                        &Identify::new(
                            IdentifyInfo {
                                compress: false,
                                intents: self.intents.clone(),
                                large_threshold: 250,
                                presence: None,
                                properties: IdentifyProperties {
                                    browser: "fishmael".to_string(),
                                    device: "fishmael".to_string(),
                                    os: env::consts::OS.to_string(),
                                },
                                shard: Some(self.shard_id),
                                token: self.token.as_mut().to_string(),
                            })
                        )
                        .expect("failed to serialise identify")
                    )
                );

                self.identified = true;

                if ready!(self.poll_handle_pending(cx)).is_err() {
                    return Poll::Ready(Some(Ok(Message::ABNORMAL_CLOSE)));
                }
            }

            // TODO: send user gateway messages

            match ready!(Pin::new(self.connection.as_mut().unwrap()).poll_next(cx)) {
                Some(Ok(message)) => {
                    if let Some(message) = Message::from_websocket_msg(&message) {
                        break message
                    }
                },
                Some(Err(WebsocketError::Io(e)))
                    if e.kind() == IoErrorKind::UnexpectedEof
                    && matches!(self.state, ShardState::Disconnected{..}) =>
                {
                    continue;
                },
                Some(Err(_)) => {
                    self.disconnect(CloseInitiator::Transport);
                    return Poll::Ready(Some(Ok(Message::ABNORMAL_CLOSE)));
                }
                None => {
                    println!("received none when polling connection");
                    _ = ready!(Pin::new(self.connection.as_mut().unwrap()).poll_close(cx));

                    if !matches!(self.state, ShardState::Disconnected{..}) {
                        self.disconnect(CloseInitiator::Transport);
                    }

                    self.connection = None
                }
            }
        };

        match &message {
            Message::Close(frame) => {
                // Response is automatically handled by websocket
                println!("received websocket close message");
                if !matches!(self.state, ShardState::Disconnected{..}) {
                    self.disconnect(
                        CloseInitiator::Gateway(frame.as_ref().map(|f| f.code.into()))
                    );
                } 
            }
            Message::Text(event) => {
                self.process(event).map_err(|e| {
                    ReceiveError {
                        kind: ReceiveErrorKind::Reconnect,
                        source: Some(e.into()),
                    }
                })?;
            },
        }

        Poll::Ready(Some(Ok(message)))
    }
}
