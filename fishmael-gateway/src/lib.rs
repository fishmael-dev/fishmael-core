use anyhow::{bail, Context, Result};
use futures::{
    stream::{SplitSink, StreamExt},
    SinkExt,
};
use rand::{rngs::StdRng, Rng, SeedableRng};
use serde::Deserialize;
use serde_json::Value;
use std::{borrow::Borrow, env, mem, sync::Arc, time::Duration};
use tokio::{net::TcpStream, sync::Mutex, task::JoinHandle, time::{self, Instant, Interval, MissedTickBehavior}};
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};

use fishmael_model::{
    event::{
        hello::Hello, identify::{Identify, IdentifyProperties, ShardId}, ready::Ready, GatewayEvent, Opcode, Payload
    },
    intents::Intents,
};


const GATEWAY_URL: &str = "wss://gateway.discord.gg";


type Connection = WebSocketStream<MaybeTlsStream<TcpStream>>;


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
    pub token: String,
    pub intents: Intents,
    heartbeat: Arc<Mutex<Option<JoinHandle<()>>>>,
    heartbeat_interval: Option<Interval>,
    rng: StdRng,
    seq: Arc<Mutex<Option<u64>>>,
    session: Option<Session>,
    shard_id: ShardId,
    resume_gateway_url: Option<String>,
}


impl Shard {

    pub fn new(
        token: String,
        shard_id: ShardId,
        intents: Intents,
    ) -> Self {
        Self {
            heartbeat: Arc::new(Mutex::new(None)),
            heartbeat_interval: None,
            intents,
            resume_gateway_url: None,
            rng: StdRng::from_entropy(),
            session: None,
            seq: Arc::new(Mutex::new(None)),
            shard_id,
            token,
        }
    }

    pub async fn connect(&mut self) -> Result<()>{
        let (ws, _) = connect_async(GATEWAY_URL).await?;
        let (tx, mut rx) = ws.split();
        let tx = Arc::new(Mutex::new(tx));

        // Wait for Hello and start heartbeat...
        // if let Some(Ok(Message::Text(msg))) = rx.next().await {
        //     match serde_json::from_str(&msg) {
        //         Ok(GatewayEvent {
        //             op: Opcode::Hello,
        //             d: Some(Payload::Hello(hello)),
        //             ..
        //         }) => {
        //             self.heartbeat_interval = Some(hello.heartbeat_interval);
        //             // self.start_heartbeat(Arc::clone(&tx), true).await?;
        //         },
        //         _ => bail!("Did not receive Hello event!"),
        //     }
        // } else {
        //     bail!("Did not receive Hello event!");
        // }

        // Send Identify...
        Self::send_gateway_event(
            Arc::clone(&tx),
            GatewayEvent::new(
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
            ),
        ).await?;

        // Listen for incoming events...
        while let Some(msg) = rx.next().await {
            match msg {
                Ok(Message::Text(msg)) => {
                    self.process_gateway_event(Arc::clone(&tx), msg).await?;
                },
                _ => println!("Failed to decode websocket message: {:?}", msg),
            };
        };

        Ok(())
    }

    // async fn start_heartbeat(
    //     &mut self,
    //     tx: Arc<Mutex<SplitSink<Connection, Message>>>,
    //     wait: bool,
    // ) -> Result<()> {
    //     let mut heartbeat = self.heartbeat.lock().await;
    //     if heartbeat.is_some() {
    //         heartbeat.as_mut().unwrap().abort();
    //     }

    //     if let Some(heartbeat_interval) = self.heartbeat_interval {
    //         let initial_delay = self.rng.lock().await.gen_range(0..heartbeat_interval);
    
    //         println!("Staring heartbeat with interval {} [ms].", heartbeat_interval);

    //         // All of this runs in the background.
    //         *heartbeat = Some(tokio::spawn(async move {
    //             if wait {
    //                 // Sleep a random time from 0..heartbeat_interval for the first
    //                 // heartbeat.
    //                 println!("Waiting {} [ms] until initial heartbeat.", initial_delay);
    //                 time::sleep(Duration::from_millis(initial_delay)).await;
    //             }
   
    //             loop {
    //                 let sequence = self.session.as_mut()
    //                     .unwrap()
    //                     .sequence()
    //                     .clone()
    //                     .to_owned();

    //                 match Self::send_gateway_event(
    //                     Arc::clone(&tx),
    //                     GatewayEvent::new(Opcode::Heartbeat, Payload::OptInt(Some(sequence))),
    //                 ).await {
    //                     Ok(_) => time::sleep(Duration::from_millis(heartbeat_interval)).await,
    //                     Err(_) => break,  // TODO: log this
    //                 };
    //             }
    //         }));
    //     } else {
    //         bail!("Heartbeat interval was not set before starting heartbeat!");
    //     }

    //     Ok(())
    // }

    async fn send_gateway_event(
        tx: Arc<Mutex<SplitSink<Connection, Message>>>,
        event: GatewayEvent,
    ) -> Result<()> {
        let json = serde_json::json!(event).to_string();
        println!("{}", json);
        tx.lock()
            .await
            .send(Message::Text(json))
            .await
            // We won't have access to event.t for events with a nonzero opcode,
            // So we'll have to make-do with the enum/opcode.
            .context(format!("Failed to send {:?} event", event.op))?;

        Ok(())
    }

    async fn process_gateway_event(
        &mut self,
        tx: Arc<Mutex<SplitSink<Connection, Message>>>,
        payload: String,
    ) -> Result<()> {
        println!("RECEIVED: {}", payload);

        match serde_json::from_str(&payload) {
            Ok(GatewayEvent {op, d, t: _, s}) => {
                if let Some(s) = s {
                    if let Some(ref mut session) = &mut self.session {
                        session.set_sequence(s);
                    };
                }

                match (op, d) {
                    (Opcode::Dispatch, Some(Payload::Ready(ready))) => {
                        println!("Ready received.");
                        // TODO: Store resume url, implement resuming.
                        let sequence = s.context("missing sequence in ready")?;

                        self.session = Some(Session::new(sequence, ready.session_id));
                        self.resume_gateway_url = Some(ready.resume_gateway_url);

                        let id = &ready.guilds.iter().next().unwrap().id;
                        println!("Ready! We are user {:?} ({})", ready.user.name, ready.user.discriminator);
                        println!("Found guild with id {} (created at {})", &id, &id.timestamp())
                    },
                    (Opcode::ACK, None) => {
                        println!("Heartbeat ACK received.");
                    },
                    (Opcode::Heartbeat, _) => {
                        println!("Heartbeat received.");
                        // Immediately restart heartbeat...
                        // self.start_heartbeat(tx, false).await?;
                    }
                    _ => println!("Unknown event: {}", payload),
                }
            },
            Err(err) => println!("Failed to deserialise: {:?}", err)
        }

        Ok(())
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
}
