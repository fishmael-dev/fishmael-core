use futures::{
    stream::{SplitSink, StreamExt},
    SinkExt,
};
use rand::{rngs::StdRng, Rng, SeedableRng};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::{env, sync::Arc, time::Duration};
use tokio::{net::TcpStream, sync::Mutex, task::JoinHandle, time};
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};
use anyhow::{bail, Context, Result};
use serde_aux::field_attributes::deserialize_number_from_string;


#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum GatewayOpcode {
    DISPATCH = 0,
    HEARTBEAT = 1,
    IDENTIFY = 2,
    PRESENCE = 3,
    VOICE_STATE = 4,
    VOICE_PING = 5,
    RESUME = 6,
    RECONNECT = 7,
    REQUEST_MEMBERS = 8,
    INVALIDATE_SESSION = 9,
    HELLO = 10,
    ACK = 11,
    GUILD_SYNC = 12,
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
        user: User,
        session_id: String,
        resume_gateway_url: String,
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct AvatarDecorationData {
    asset: String,
    sku_id: u64,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(deserialize_with="deserialize_number_from_string")]
    id: u64,
    username: String,
    discriminator: String,
    #[serde(default)]
    global_name: Option<String>,
    #[serde(default)]
    avatar: Option<String>,
    #[serde(default)]
    bot: bool,
    #[serde(default)]
    system: bool,
    #[serde(default)]
    mfa_enabled: bool,
    #[serde(default)]
    banner: Option<String>,
    #[serde(default)]
    accent_color: Option<String>,
    #[serde(default)]
    locale: Option<String>,
    #[serde(default)]
    verified: bool,
    #[serde(default)]
    email: Option<String>,
    #[serde(default, deserialize_with="deserialize_number_from_string")]
    flags: u64,
    #[serde(default, deserialize_with="deserialize_number_from_string")]
    premium_type: u64,
    #[serde(default, deserialize_with="deserialize_number_from_string")]
    public_flags: u64,
    #[serde(default)]
    avatar_decoration_data: Option<AvatarDecorationData>,
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
    heartbeat_interval: Option<u64>,
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
            heartbeat_interval: None,
            rng: Arc::new(Mutex::new(StdRng::from_entropy())),
            seq: Arc::new(Mutex::new(None)),
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
                    op: GatewayOpcode::HELLO,
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
                GatewayOpcode::IDENTIFY,
                Payload::Identify {
                    token: self.token.to_string(),
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
                        GatewayEvent::new(GatewayOpcode::HEARTBEAT, Payload::OptInt(*loop_seq.lock().await)),
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
        &self,
        tx: Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>,
        payload: String,
    ) -> Result<()> {
        println!("RECEIVED: {}", payload);

        match serde_json::from_str(&payload) {
            Ok(GatewayEvent {op, d, t: _, s}) => {
                if let Some(s) = s {
                    *self.seq.lock().await = Some(s);
                }

                match (op, d) {
                    (GatewayOpcode::DISPATCH, Some(Payload::Ready { v, user, session_id, resume_gateway_url })) => {
                        // TODO: Store resume url, implement resuming.
                        println!("Ready! We are user {:?} ({})", user.username, user.discriminator);
                    },
                    (GatewayOpcode::ACK, None) => {
                        println!("got ack!");
                    },
                    (GatewayOpcode::HEARTBEAT, _) => {
                        // Immediately restart heartbeat...
                        self.start_heartbeat(tx, false).await?;
                    }
                    _ => todo!(),
                }
            },
            Err(err) => println!("Failed to deserialise: {:?}", err)
        }

        Ok(())
    }
}
