use tokio;
use anyhow::{Context, Result};
use dotenv::dotenv;
use fishmael_model::{event::identify::ShardId, intents::Intents};
use fishmael_gateway::Shard;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().context("Failed to find dotenv")?;
    let token = std::env::var("TOKEN").context("Failed to load token from .env")?;
    
    let mut shard = Shard::new(
        token,
        ShardId::new(0, 1),
        Intents::GUILDS,
    );

    while let Some(item) = shard.next_event().await {
        match item {
            Ok(event) => println!("RECEIVED {:?}", event),
            v => {dbg!(v)?;},
        }
    }

    Ok(())
}
