use tokio;
use anyhow::{Context, Result};
use dotenv::dotenv;

use fishmael_gateway::{Intents, Shard, ShardId};
use fishmael_cache::{
    guild::CacheableGuild,
    interaction::{StreamableCommandInteraction, StreamableComponentInteraction},
    Cache,
    Cacheable,
    Streamable,
};
use twilight_model::{
    application::interaction::InteractionData,
    gateway::event::Event
};


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
                Event::InteractionCreate(i) => {
                    match i.0.data {
                        Some(InteractionData::ApplicationCommand(_)) => {
                            let ci = TryInto::<StreamableCommandInteraction>::try_into(i.0)?;
                            ci.stream(
                                &mut cache.con,
                                &shard.id(),
                                100,
                            ).await?;
                        },
                        Some(InteractionData::MessageComponent(_)) => {
                            let ci = TryInto::<StreamableComponentInteraction>::try_into(i.0)?;
                            ci.stream(
                                &mut cache.con,
                                &shard.id(),
                                100,
                            ).await?;
                        },
                        Some(InteractionData::ModalSubmit(_)) => unimplemented!(),
                        Some(_) | None => {},
                    };

                    println!("InteractionCreate!!!");
                }
                _ => println!("Unhandled!"),
            }
        }
    }

    Ok(())
}
