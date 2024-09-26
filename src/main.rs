use tokio;
use fishmael::{self, models::intents::Intents};
use anyhow::{Context, Result};
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().context("Failed to find dotenv")?;
    let token = std::env::var("TOKEN").context("Failed to load token from .env")?;
    
    let client = fishmael::Client::new(
        "https://discord.com/api/v10".to_string(),
        "wss://gateway.discord.gg".to_string(),
        token,
        Intents::GUILDS,
    );

    client.connect().await?;

    Ok(())
}
