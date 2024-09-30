use async_trait::async_trait;
use redis::{self, aio::ConnectionLike, Cmd, RedisError};

pub mod guild;


pub struct Cache {
    pub client: redis::Client
}


impl Cache {
    pub fn from_url(url: &str) -> Self {
        Self {
            client: redis::Client::open(url)
                .unwrap_or_else(|_| panic!("Failed to open redis client with url {}", url))
        }
    }
}

#[async_trait]
pub trait Cacheable {
    fn get_key(&self) -> String;

    fn add_fields_to_cmd(&self, cmd: &mut Cmd) -> (); 

    async fn store(&self, con: &mut (impl ConnectionLike + Send)) -> Result<(), RedisError> {
        let mut cmd = redis::cmd("HSET");
        cmd.arg(self.get_key());
        self.add_fields_to_cmd(&mut cmd);

        cmd.exec_async(con).await
    }
}
