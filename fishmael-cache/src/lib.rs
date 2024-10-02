use anyhow::Context;
use async_trait::async_trait;
use redis::{self, aio::{ConnectionLike, MultiplexedConnection}, Cmd, RedisError};

pub mod cmd;
pub mod guild;


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

#[async_trait]
pub trait Cacheable {
    fn get_key(&self) -> String;

    fn add_fields_to_cmd(self, cmd: &mut Cmd) -> (); 

    async fn store<T: ConnectionLike + Send>(self, con: &mut T) -> Result<(), RedisError>
    where
        Self: Sized
    {
        redis::cmd("HSET")
            .arg(&self.get_key())
            .args_from(self)
            .exec_async(con)
            .await
    }
}

trait ArgsFrom<T> {
    fn args_from(&mut self, value: T) -> &mut Self;
}

impl<T: Cacheable> ArgsFrom<T> for Cmd {
    fn args_from(&mut self, value: T) -> &mut Self {
        value.add_fields_to_cmd(self);
        self
    }
}
