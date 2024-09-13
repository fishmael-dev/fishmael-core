use futures::{
    stream::{SplitSink, StreamExt},
    SinkExt,
};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, json, Value};
use std::{collections::HashMap, sync::Arc, time::Duration, env};
use tokio::{net::TcpStream, sync::Mutex, task, time};
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};
use anyhow::{bail, Context, Result};


#[derive(Debug, Serialize, Deserialize)]
pub enum GatewayOpcode {
    IDENTIFY = 2,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct IdentifyProperties {
    os: String,
    browser: String,
    device: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub enum GatewayEventPayload {
    Identify {
        token: String,
        properties: IdentifyProperties,
        intents: u64,
    } 
}


#[derive(Debug, Serialize, Deserialize)]
pub struct GatewayEvent {
    op: GatewayOpcode,
    d: GatewayEventPayload,
    s: Option<u64>,
    t: Option<String>,
}


impl GatewayEvent {
    pub fn new(op: GatewayOpcode, d: GatewayEventPayload) -> Self {
        GatewayEvent {
            op,
            d,
            s: Option::None,
            t: Option::None,
        }
    }
}


struct Client {
    pub api_url: String,
    pub gateway_url: String,
    pub token: String,
    pub intents: u64,
}


impl Client {
    async fn connect(self) -> Result<()>{
        let (ws, _) = connect_async(self.gateway_url).await?;
        let (tx, mut rx) = ws.split();

        let tx = Arc::new(Mutex::new(tx));

        Client::send_gateway_event(
            Arc::clone(&tx),
            GatewayEvent::new(
                GatewayOpcode::IDENTIFY,
                GatewayEventPayload::Identify {
                    token: self.token,
                    properties: IdentifyProperties {
                        os: env::consts::OS.to_string(),
                        browser: "fishmael".to_string(),
                        device: "fishmael".to_string(),
                    },
                    intents: self.intents,
                }
            ),
        ).await?;

        while let Some(msg) = rx.next().await {
            match msg {
                Ok(Message::Text(msg)) => Client::process_gateway_event(Arc::clone(&tx), msg).await?,
                _ => bail!("Failed to decode websocket message."),
            };
        };

        Ok(())
    }

    async fn send_gateway_event(
        tx: Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>,
        event: GatewayEvent,
    ) -> Result<()> {
        tx.lock()
            .await
            .send(Message::Text(json!(event).to_string()))
            .await
            // We won't have access to event.t for events with a nonzero opcode,
            // So we'll have to make-do with the enum/opcode.
            .context(format!("Failed to send {:?} event", event.op))?;

        Ok(())
    }

    async fn process_gateway_event(
        tx: Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>,
        payload: String,
    ) -> Result<()> {
        todo!()
    }
}