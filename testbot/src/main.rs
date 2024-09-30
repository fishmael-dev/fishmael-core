use tokio;
use anyhow::{Context, Result};
use dotenv::dotenv;
use fishmael_model::{event::{guild_create::GuildCreate, identify::ShardId}, intents::Intents};
use fishmael_gateway::{event::Event, Shard};
use fishmael_cache::{guild::CacheableGuild, Cacheable};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().context("Failed to find dotenv")?;
    let token = std::env::var("TOKEN").context("Failed to load token from .env")?;
    let redis_url = std::env::var("REDIS_URL").context("Failed to load redis url from .env")?;

    let mut shard = Shard::new(
        token,
        ShardId::new(0, 1),
        Intents::GUILDS,
    );

    let r = redis::Client::open(redis_url)?;
    let mut con = r.get_multiplexed_async_connection().await?;

    while let Some(item) = shard.next_event().await {
        if let Ok(event) = item {
            println!("RECEIVED EVENT: {}", event.name());
            match event {
                Event::GuildCreate(GuildCreate::Unavailable(g)) => {println!("Unavailable Guild: ??? (id: {})", g.id)},
                Event::GuildCreate(GuildCreate::Available(g)) => {
                    let cg: CacheableGuild = g.into();
                    cg.store(&mut con).await?;

                    println!("Available Guild: {} (id: {})", cg.id, cg.name);
                    break
                },
                _ => println!("Unhandled!"),
            }
        }
    }

    Ok(())
}
