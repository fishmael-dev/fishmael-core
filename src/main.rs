use tokio;
use fishmael;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let client = fishmael::Client::new(
        "https://discord.com/api/v10".to_string(),
        "wss://gateway.discord.gg".to_string(),
        "".to_string(),
        0,
    );

    client.connect().await?;

    Ok(())
}
