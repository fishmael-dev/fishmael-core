use futures::{
    stream::{SplitSink, StreamExt},
    SinkExt,
};
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::{env, sync::Arc, time::Duration};
use tokio::{net::TcpStream, sync::Mutex, task::JoinHandle, time};
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};
use anyhow::{bail, Context, Result};

// use crate::models::events::*;
pub mod models;
use crate::models::events::*;

// Opcodes
const DISPATCH: u8 = 0;
const HEARTBEAT: u8 = 1;
const IDENTIFY: u8 = 2;
const PRESENCE: u8 = 3;
const VOICE_STATE: u8 = 4;
const VOICE_PING: u8 = 5;
const RESUME: u8 = 6;
const RECONNECT: u8 = 7;
const REQUEST_MEMBERS: u8 = 8;
const INVALIDATE_SESSION: u8 = 9;
const HELLO: u8 = 10;
const ACK: u8 = 11;
const GUILD_SYNC: u8 = 12;


pub struct Client {
    pub api_url: String,
    pub gateway_url: String,
    pub token: String,
    pub intents: u64,
    heartbeat: Arc<Mutex<Option<JoinHandle<()>>>>,
    heartbeat_interval: Option<u64>,
    rng: Arc<Mutex<StdRng>>,
    seq: Arc<Mutex<Option<u64>>>,
    session_id: Option<String>,
    resume_gateway_url: Option<String>,
}


impl Client {

    pub fn new(
        api_url: String,
        gateway_url: String,
        token: String,
        intents: u64,
    ) -> Self {
        Client {
            api_url,
            gateway_url,
            token,
            intents,
            heartbeat: Arc::new(Mutex::new(None)),
            heartbeat_interval: None,
            rng: Arc::new(Mutex::new(StdRng::from_entropy())),
            seq: Arc::new(Mutex::new(None)),
            session_id: None,
            resume_gateway_url: None,
        }
    }

    pub async fn connect(mut self) -> Result<()>{
        let (ws, _) = connect_async(&self.gateway_url).await?;
        let (tx, mut rx) = ws.split();
        let tx = Arc::new(Mutex::new(tx));

        // Wait for Hello and start heartbeat...
        if let Some(Ok(Message::Text(msg))) = rx.next().await {
            match serde_json::from_str(&msg) {
                Ok(GatewayEvent {
                    op: HELLO,
                    d: Some(Payload::Hello { heartbeat_interval }),
                    ..
                }) => {
                    self.heartbeat_interval = Some(heartbeat_interval);
                    self.start_heartbeat(Arc::clone(&tx), true).await?;
                },
                _ => bail!("Did not receive Hello event!"),
            }
        } else {
            bail!("Did not receive Hello event!");
        }

        // Send Identify...
        Client::send_gateway_event(
            Arc::clone(&tx),
            GatewayEvent::new(
                IDENTIFY,
                Payload::Identify {
                    token: self.token.clone(),
                    properties: IdentifyProperties {
                        os: env::consts::OS.to_string(),
                        browser: "fishmael".to_string(),
                        device: "fishmael".to_string(),
                    },
                    intents: self.intents.clone(),
                }
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

    async fn start_heartbeat(
        &self,
        tx: Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>,
        wait: bool,
    ) -> Result<()> {
        let mut heartbeat = self.heartbeat.lock().await;
        if heartbeat.is_some() {
            heartbeat.as_mut().unwrap().abort();
        }

        if let Some(heartbeat_interval) = self.heartbeat_interval {
            let loop_seq = Arc::clone(&self.seq);
            let initial_delay = self.rng.lock().await.gen_range(0..heartbeat_interval);
    
            println!("Staring heartbeat with interval {} [ms].", heartbeat_interval);

            // All of this runs in the background.
            *heartbeat = Some(tokio::spawn(async move {
                if wait {
                    // Sleep a random time from 0..heartbeat_interval for the first
                    // heartbeat.
                    println!("Waiting {} [ms] until initial heartbeat.", initial_delay);
                    time::sleep(Duration::from_millis(initial_delay)).await;
                }
   
                loop {
                    match Client::send_gateway_event(
                        Arc::clone(&tx),
                        GatewayEvent::new(HEARTBEAT, Payload::OptInt(*loop_seq.lock().await)),
                    ).await {
                        Ok(_) => time::sleep(Duration::from_millis(heartbeat_interval)).await,
                        Err(_) => break,  // TODO: log this
                    };
                }
            }));
        } else {
            bail!("Heartbeat interval was not set before starting heartbeat!");
        }

        Ok(())
    }

    async fn send_gateway_event(
        tx: Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>,
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
        tx: Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>,
        payload: String,
    ) -> Result<()> {
        println!("RECEIVED: {}", payload);

        match serde_json::from_str(&payload) {
            Ok(GatewayEvent {op, d, t, s}) => {
                if let Some(s) = s {
                    *self.seq.lock().await = Some(s);
                }

                match (op, d) {
                    (DISPATCH, Some(Payload::Ready { v, user, session_id, resume_gateway_url, guilds, shard })) => {
                        // TODO: Store resume url, implement resuming.
                        self.session_id = Some(session_id);
                        self.resume_gateway_url = Some(resume_gateway_url);

                        let id = &guilds.iter().next().unwrap().id;
                        println!("Ready! We are user {:?} ({})", user.username, user.discriminator);
                        println!("Found guild with id {} (created at {})", &id, &id.timestamp())
                    },
                    (ACK, None) => {
                        println!("Heartbeat ACK received.");
                    },
                    (HEARTBEAT, _) => {
                        // Immediately restart heartbeat...
                        self.start_heartbeat(tx, false).await?;
                    }
                    _ => println!("Unknown event: {}", payload),
                }
            },
            Err(err) => println!("Failed to deserialise: {:?}", err)
        }

        Ok(())
    }
}
