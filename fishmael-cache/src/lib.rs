use redis;


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
