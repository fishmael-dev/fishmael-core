use tokio;
use anyhow::{Context, Result};
use dotenv::dotenv;

use fishmael_gateway::{Intents, Shard, ShardId};
use fishmael_cache::{
    guild::CacheableGuild,
    Cache,
    Cacheable
};
use twilight_model::gateway::event::Event;


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

    let mut cache = Cache::from_url(redis_url).await?;

    while let Some(item) = shard.next_event().await {
        if let Ok(event) = item {
            println!("RECEIVED EVENT: {:?}", event.kind());
            match event {
                Event::GuildCreate(g) => {
                    let cg: CacheableGuild = g.0.into();
                    cg.clone().store(&mut cache.con).await?;

                    println!("GuildCreate: {} (id: {})", cg.id, cg.name);
                },
                Event::GuildUpdate(g) => {
                    let cg: CacheableGuild = g.0.into();
                    cg.clone().store(&mut cache.con).await?;

                    println!("GuildUpdate: {} (id: {})", cg.id, cg.name);
                }
                _ => println!("Unhandled!"),
            }
        }
    }

    Ok(())
}
