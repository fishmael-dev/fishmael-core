use futures::{
    stream::{SplitSink, SplitStream, StreamExt},
    SinkExt,
};
use rand::{rngs::StdRng, Rng, SeedableRng};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::{env, sync::Arc, time::Duration};
use tokio::{net::TcpStream, sync::Mutex, task::JoinHandle, time};
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};
use anyhow::{Context, Result};


#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum GatewayOpcode {
    DISPATCH = 0,
    HEARTBEAT = 1,
    IDENTIFY = 2,
    HELLO = 10,
    ACK = 11,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct IdentifyProperties {
    os: String,
    browser: String,
    device: String,
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Payload {
    Bool(bool),
    Int(u64),
    OptInt(Option<u64>),
    Identify {
        token: String,
        properties: IdentifyProperties,
        intents: u64,
    },
    Hello {
        heartbeat_interval: u64,
    },
    Ready {
        v: u8,
        session_id: String,
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct GatewayEvent {
    op: GatewayOpcode,
    d: Option<Payload>,
    s: Option<u64>,
    t: Option<String>,
}


impl GatewayEvent {
    pub fn new(op: GatewayOpcode, d: Payload) -> Self {
        GatewayEvent {
            op,
            d: Some(d),
            s: None,
            t: None
        }
    }
}


pub struct Client {
    pub api_url: String,
    pub gateway_url: String,
    pub token: String,
    pub intents: u64,
    heartbeat: Arc<Mutex<Option<JoinHandle<()>>>>,
    rng: Arc<Mutex<StdRng>>,
    seq: Arc<Mutex<Option<u64>>>,
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
            rng: Arc::new(Mutex::new(StdRng::from_entropy())),
            seq: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn connect(mut self) -> Result<()>{
        let (ws, _) = connect_async(&self.gateway_url).await?;
        let (tx, rx) = ws.split();

        let tx = Arc::new(Mutex::new(tx));
        let rx = Arc::new(Mutex::new(rx));

        // Start heartbeat...
        self.start_heartbeat(Arc::clone(&tx), Arc::clone(&rx), true).await?;

        // Send Identify...
        Client::send_gateway_event(
            Arc::clone(&tx),
            // TODO: This sucks wtf
            GatewayEvent::new(
                GatewayOpcode::IDENTIFY,
                Payload::Identify {
                    token: self.token.to_string(),
                    properties: IdentifyProperties {
                        os: env::consts::OS.to_string(),
                        browser: "fishmael".to_string(),
                        device: "fishmael".to_string(),
                    },
                    intents: self.intents,
                }
            ),
        ).await?;

        // Listen for incoming events...
        while let Some(msg) = rx.lock()
            .await
            .next()
            .await {
                match msg {
                    Ok(Message::Text(msg)) => {
                        self.process_gateway_event(Arc::clone(&tx), Arc::clone(&rx), msg).await?;
                    },
                    _ => println!("Failed to decode websocket message: {:?}", msg),
                };
            };

        Ok(())
    }

    async fn start_heartbeat(
        &mut self,
        tx: Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>,
        rx: Arc<Mutex<SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>>>,
        wait: bool,
    ) -> Result<()> {
        let mut heartbeat = self.heartbeat.lock().await;
        if heartbeat.is_some() {
            heartbeat.as_mut().unwrap().abort();
        }

        println!("heartbeat gaming");

        if let Some(Ok(Message::Text(msg))) = rx.lock().await.next().await {
            if let Ok(GatewayEvent {
                op: GatewayOpcode::HELLO,
                d: Some(Payload::Hello { heartbeat_interval }),
                ..
            }) = serde_json::from_str(&msg) {
                if wait {
                    // Sleep a random time from 0..heartbeat_interval for the first
                    // heartbeat.
                    time::sleep(Duration::from_millis(
                        self.rng.lock().await.gen_range(0..heartbeat_interval)
                    )).await;
                }

                Client::send_gateway_event(
                    Arc::clone(&tx),
                    GatewayEvent::new(GatewayOpcode::HEARTBEAT, Payload::OptInt(*self.seq.lock().await)),
                ).await?;

                let loop_seq = Arc::clone(&self.seq);
                *heartbeat = Some(tokio::spawn(async move {
                    loop {
                        match Client::send_gateway_event(
                            Arc::clone(&tx),
                            GatewayEvent::new(GatewayOpcode::HEARTBEAT, Payload::OptInt(*loop_seq.lock().await)),
                        ).await {
                            Ok(_) => time::sleep(Duration::from_millis(heartbeat_interval)).await,
                            Err(_) => break,  // TODO: log this
                        };
                    }
                }))
            }
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
        rx: Arc<Mutex<SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>>>,
        payload: String,
    ) -> Result<()> {
        println!("RECEIVED: {}", payload);

        if let Ok(GatewayEvent {op, d, t, s}) = serde_json::from_str(&payload) {
            if let Some(s) = s {
                *self.seq.lock().await = Some(s);
            }

            match (op, d) {
                (GatewayOpcode::DISPATCH, Some(Payload::Ready { v, session_id })) => {
                    println!("Ready! API version {}, session id {}.", v, session_id);
                }
                (GatewayOpcode::ACK, None) => {
                    println!("got ack!");
                },
                (GatewayOpcode::HEARTBEAT, _) => {
                    // Immediately restart heartbeat...
                    self.start_heartbeat(tx, rx, false).await?;
                }
                _ => todo!(),
            }
        }

        Ok(())
    }
}
