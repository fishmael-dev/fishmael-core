use tokio;
use fishmael::{self, models::intents::Intents};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let client = fishmael::Client::new(
        "https://discord.com/api/v10".to_string(),
        "wss://gateway.discord.gg".to_string(),
        "".to_string(),
        Intents::from_bits_truncate(0),
    );

    client.connect().await?;

    Ok(())
}
