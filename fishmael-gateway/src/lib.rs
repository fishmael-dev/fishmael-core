use anyhow::{Context, Result};
use futures::Sink;
use futures_core::Stream;
use rand::{rngs::StdRng, Rng, SeedableRng};
use serde::Deserialize;
use serde_json::Value;
use std::{
    borrow::Cow,
    env,
    fmt::{Display, Formatter, Result as FmtResult},
    future::Future,
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
        protocol::{frame::coding::CloseCode, CloseFrame},
        Error as WebsocketError, Message,
    },
    MaybeTlsStream,
    WebSocketStream,
};

use fishmael_model::{
    event::{
        guild_create::GuildCreate,
        hello::Hello,
        identify::{Identify, IdentifyProperties, ShardId},
        ready::Ready,
        GatewayEvent,
        Opcode,
        Payload,
    },
    intents::Intents,
};


const GATEWAY_URL: &str = "wss://gateway.discord.gg";
const API_VERSION: u8 = 10;


type Connection = WebSocketStream<MaybeTlsStream<TcpStream>>;


struct ConnectionFuture(Pin<Box<dyn Future<Output = Result<Connection, WebsocketError>> + Send>>);


pub struct Session {
    id: Box<str>,
    sequence: u64,
}


#[derive(Deserialize)]
pub struct Thing {
    op: Opcode,
    d: Value,
    s: Option<u64>,
    t: Option<String>,
}


#[derive(Debug)]
pub enum Event {
    Heartbeat(Option<u64>),
    Hello(Hello),
    GatewayClose(Option<CloseFrame<'static>>),
    GuildCreate(GuildCreate),
    Identify(Identify),
    Ready(Ready),
}


impl From<Payload> for Event {
    fn from(value: Payload) -> Self {
        match value {
            Payload::Heartbeat(v) => Self::Heartbeat(v),
            Payload::Hello(v) => Self::Hello(v),
            Payload::GuildCreate(v) => Self::GuildCreate(v),
            Payload::Identify(v) => Self::Identify(v),
            Payload::Ready(v) => Self::Ready(v), 
        }
    }
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
            token,
        }
    }

    fn process(&mut self, event: &str) -> Result<()> {
        let Thing{op, d: event_data, s: maybe_sequence, t: maybe_type} =
            serde_json::from_str(&event)
            .context("failed to extract data from event")?;

        match op {
            Opcode::Dispatch => {
                let event_type = maybe_type
                    .context("failed to get event type")?;
                let sequence = maybe_sequence
                    .context("failed to get sequence")?;

                match event_type.as_ref() {
                    "READY" => {
                        let event = serde_json::from_value::<Ready>(event_data)
                            .context("failed to deserialise ready event")?;

                        self.resume_gateway_url = Some(event.resume_gateway_url);
                        self.session = Some(Session::new(sequence, event.session_id));
                    },
                    "RESUMED" => {
                        // TODO: implement resuming
                    }
                    _ => {}
                }

                if let Some(session) = self.session.as_mut() {
                    session.set_sequence(sequence);
                }
            },
            Opcode::Heartbeat => {
                // TODO: send heartbeat
            },
            Opcode::ACK => {
                println!("ACK received.");
                // TODO: track heartbeat responses to check if connection is still alive.
            }
            Opcode::Hello => {
                let event = serde_json::from_value::<Hello>(event_data)
                    .context("failed to deserialise hello event")?;
                let heartbeat_interval = Duration::from_millis(event.heartbeat_interval);
                let jitter = Duration::from_millis(self.rng.gen_range(0..event.heartbeat_interval));

                let mut interval = time::interval_at(Instant::now() + jitter, heartbeat_interval);
                interval.set_missed_tick_behavior(MissedTickBehavior::Delay);
                self.heartbeat_interval = Some(interval);

                // TODO: implement reconnection
            }
            _ => todo!()
        }

        Ok(())
    }

    fn disconnect(&mut self) {
        self.heartbeat_interval = None;
        self.pending = Some(Message::Close(Some(CloseFrame {
            code: CloseCode::Normal,
            reason: Cow::Owned("Cya".to_string())
        })));
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

            // TODO: break out disconnect logic.
            // TODO: handle different disconnect types (i.e. reconnect logic).
            self.disconnect();
            self.connection = None;
            return Poll::Ready(Err(e));
        }
        println!("Polling connection state for sending... done!");

        let pending = self.pending.as_mut();

        println!("sending event...");
        if let Some(message) = pending {
            // TODO: ratelimiting
            if let Err(e) = Pin::new(self.connection.as_mut().unwrap()).start_send(message.clone()) {
                println!("sending event... failed!");

                self.disconnect();
                self.connection = None;
                return Poll::Ready(Err(e));
            }
        }

        println!("polling completion of sending event...");
        if let Err(e) = ready!(Pin::new(self.connection.as_mut().unwrap()).poll_flush(cx)) {
            println!("polling completion of sending event... failed!");

            self.disconnect();
            self.connection = None;
            return Poll::Ready(Err(e));
        }
        println!("polling completion of sending event... done!");

        self.pending = None;

        Poll::Ready(Ok(()))
    }

    pub fn next_event(&mut self) -> PollEvent<Self> {
        PollEvent {stream: self}
    }

}

impl Stream for Shard {
    type Item = Result<Message, ReceiveError>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut AsyncContext<'_>) -> Poll<Option<Self::Item>> {
        let message = loop {
            // Ensure connection...
            println!("loop start; connection={:?}, connection_future={:?}", self.connection.is_none(), self.connection_future.is_none());

            {
                if self.connection.is_none() {
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
                        }
                        Err(err) => {
                            println!("connection failed!");
    
                            self.resume_gateway_url = None;
                            
                            return Poll::Ready(Some(Err(ReceiveError {
                                kind: ReceiveErrorKind::Reconnect,
                                source: Some(Box::new(err)),
                            })))
                        }
                    }
                }
    
            }

            println!("polling heartbeat...");

            if self.heartbeat_interval
                .as_mut()
                .map_or(false, |interval| interval.poll_tick(cx).is_ready())
            {
                println!("sending heartbeat...");
                // TODO: Handle zombied connection
                self.pending = Some(Message::Text(
                    serde_json::to_string(
                        &GatewayEvent::new(
                            Opcode::Heartbeat,
                            Payload::Heartbeat(self.session.as_ref().map(|s| s.sequence))),
                    )
                    .expect("failed to serialise heartbeat")

                ));
                
                println!("polling heartbeat status...");

                if ready!(self.poll_handle_pending(cx)).is_err() {
                    println!("polling heartbeat errored!");

                    return Poll::Ready(Some(Ok(Message::Close(Some(CloseFrame {
                        code: CloseCode::Abnormal,
                        reason: Cow::Owned("".to_string()),
                    })))));
                }

                println!("sending heartbeat done!...");
            }

            if !self.identified {
                self.pending = Some(Message::Text(
                    serde_json::to_string(
                        &GatewayEvent::new(
                            Opcode::Identify,
                            Payload::Identify(Identify {
                                compress: false,
                                intents: self.intents.clone(),
                                large_threshold: 250,
                                properties: IdentifyProperties {
                                    browser: "fishmael".to_string(),
                                    device: "fishmael".to_string(),
                                    os: env::consts::OS.to_string(),
                                },
                                shard: Some(self.shard_id),
                                token: self.token.as_mut().to_string(),
                            })
                        )
                    )
                    .expect("failed to serialise identify")
                ));

                self.identified = true;

                if ready!(self.poll_handle_pending(cx)).is_err() {
                    return Poll::Ready(Some(Ok(Message::Close(Some(CloseFrame {
                        code: CloseCode::Abnormal,
                        reason: Cow::Owned("".to_string()),
                    })))));
                }
            }

            // TODO: send user gateway messages

            match ready!(Pin::new(self.connection.as_mut().unwrap()).poll_next(cx)) {
                Some(Ok(message)) => break message,
                Some(Err(_)) => {
                    self.disconnect();
                    return Poll::Ready(Some(Ok(Message::Close(Some(CloseFrame {
                        code: CloseCode::Abnormal,
                        reason: Cow::Owned("".to_string()),
                    })))));
                }
                None => {
                    println!("received none when polling connection");
                    _ = ready!(Pin::new(self.connection.as_mut().unwrap()).poll_close(cx));
                    self.connection = None
                }
            }
        };

        match &message {
            Message::Text(event) => {
                self.process(event).map_err(|e| {
                    ReceiveError {
                        kind: ReceiveErrorKind::Reconnect,
                        source: Some(e.into()),
                    }
                })?;
            },
            _ => todo!(), 
        }

        Poll::Ready(Some(Ok(message)))
    }
}


#[derive(Debug)]
pub enum ReceiveErrorKind {
    Deseralizing{
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
            ReceiveErrorKind::Deseralizing { event } => {
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


pub struct PollEvent<'a, St: ?Sized> {
    stream: &'a mut St,
}


impl<'a, St: ?Sized + Stream<Item = Result<Message, ReceiveError>> + Unpin> Future for PollEvent<'a, St> {
    type Output = Option<Result<Event, ReceiveError>>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut AsyncContext<'_>) -> Poll<Self::Output> {
        let try_from_message = |message| match message {
            Message::Text(json) => {
                match serde_json::from_str::<Payload>(&json) {
                    Ok(payload) => Ok(Into::<Event>::into(payload)),
                    Err(e) => Err(ReceiveError{
                        kind: ReceiveErrorKind::Deseralizing { event: json },
                        source: Some(Box::new(e))
                    }),
                }
            },
            Message::Close(frame) => Ok(Event::GatewayClose(frame)),
            v => unreachable!("unhandled message in deserializing: {:?}", v),
        };
        
        loop {
            match ready!(Pin::new(&mut self.stream).poll_next(cx)) {
                Some(item) => {
                    if let Ok(event) = item.and_then(try_from_message) {
                        return Poll::Ready(Some(Ok(event)));
                    }
                }
                None => return Poll::Ready(None),
            }
        }
    }
}