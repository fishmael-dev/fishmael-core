use async_trait::async_trait;
use redis::{self, aio::ConnectionLike, Cmd, RedisError};
use twilight_model::gateway::ShardId;

mod hargs;

pub use hargs::ToRedisHArgs;


pub trait RedisKeyProvider {
    fn get_key(&self) -> String;
}

pub trait RedisFieldProvider {
    fn add_fields_to_cmd(self, cmd: &mut Cmd); 
}

#[async_trait]
pub trait Cacheable: RedisKeyProvider + RedisFieldProvider {
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

#[async_trait]
pub trait Streamable: RedisFieldProvider {
    async fn stream<T: ConnectionLike + Send>(
        self,
        con: &mut T,
        stream: &str,
        shard: &ShardId,
        max_len: u64,
    ) -> Result<(), RedisError>
    where
        Self: Sized
    {
        redis::cmd("XADD")
            .arg(format!("{}:{}", stream, shard.number()))  // stream key
            .arg("MAXLEN")
            .arg(max_len)
            .arg("*")
            .args_from(self)
            .exec_async(con)
            .await
    }
}


trait ArgsFrom<T> {
    fn args_from(&mut self, value: T) -> &mut Self;
}

impl<T: RedisFieldProvider> ArgsFrom<T> for Cmd {
    fn args_from(&mut self, value: T) -> &mut Self {
        value.add_fields_to_cmd(self);
        self
    }
}
