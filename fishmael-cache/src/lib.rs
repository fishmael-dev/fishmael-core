use anyhow::Context;
use redis::{self, aio::MultiplexedConnection};

pub use fishmael_cache_core::Cacheable;

pub mod guild;
pub mod interaction;

pub struct Cache {
    pub client: redis::Client,
    pub con: MultiplexedConnection,
}


impl Cache {
    pub async fn from_url(url: String) -> anyhow::Result<Self> {
        let client = redis::Client::open(url.clone())
            .context(format!("failed to open redis client with url {}", url))?;
        let con = client.get_multiplexed_async_connection()
            .await
            .context("failed to open redis connection")?;

        // TODO: check if/how this affects deserialising
        //       enabling would be cool as it allows using commands while
        //       subscribed to a redis channel.
        // redis::cmd("HELLO")
        //     .arg("3")
        //     .exec_async(&mut con)
        //     .await
        //     .context("failed to set connection parameters")?;

        Ok(Self{client, con})
    }
}
