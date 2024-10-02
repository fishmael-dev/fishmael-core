use async_trait::async_trait;
use redis::{self, aio::ConnectionLike, Cmd, RedisError};

mod hargs;

pub use hargs::{HArgConsumer, HArgProvider};


pub trait KeyProvider {
    fn get_key(&self) -> String;
}

#[async_trait]
pub trait Cacheable: KeyProvider {
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