pub mod constants;
use crate::endpoints::constants::Error as ApiError;
use self::constants::Region;
use std::{collections::HashMap, error::Error};
use bytes::Bytes;
use serde::de::DeserializeOwned;
use tokio::{sync::Mutex, time::{Instant, Duration}};

// type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
type Result<T> = std::result::Result<T, reqwest::Error>;

struct DataWithTimestamp<T> {
    data: T,
    timestamp: Instant 
}

impl<T> DataWithTimestamp<T> {
    fn new(data: T) -> Self {
        Self {
            data,
            timestamp: Instant::now()
        }
    }

    fn valid(&self, lifetime: Duration) -> bool {
        Instant::now() - self.timestamp < lifetime
    }
}

struct Cache {
    store: HashMap<String, DataWithTimestamp<Bytes>>,
    lifetime: Duration
}

impl Cache {
    fn new(lifetime: Duration) -> Self {
        Self {
            store: HashMap::new(),
            lifetime
        }
    }

    fn get(&self, key: &String) -> Option<Bytes> {
        let timed_data = self.store.get(key)?;
        match timed_data.valid(self.lifetime) {
            true => Some(timed_data.data.clone()),
            false => None
        }
    }

    fn insert_get(&mut self, key: String, value: Bytes) -> Bytes {
        self.store.insert(key.clone(), DataWithTimestamp::new(value));
        self.store.get(&key).unwrap().data.clone()
    }
}

pub struct Client {
    client: Mutex<reqwest::Client>,
    region: Region,
    cache: Cache
}

impl Client {
    pub fn new(api_key: String, region: Region, cache_lifetime: Option<u64>) -> Result<Self> {
        let mut headers = reqwest::header::HeaderMap::new();

        headers.insert("X-Riot-Token", api_key.parse().unwrap());

        let client = reqwest::ClientBuilder::new()
            .default_headers(headers)
            .build()?;

        Ok(Client {
            client: Mutex::new(client),
            region,
            cache: Cache::new(Duration::from_secs(cache_lifetime.unwrap_or(60)))
        })
    }

    pub async fn request<T>(
        &mut self, 
        method: reqwest::Method, 
        endpoint: String, 
        short_region: bool, 
        query: Option<HashMap<&str, String>>
    ) -> std::result::Result<T, Box<dyn Error>> 
    where
        T: DeserializeOwned
    {
        let region = if short_region {
            self.region.to_short_region()
        } else {
            self.region.to_long_region()
        };

        let url = format!("https://{region}.api.riotgames.com{endpoint}");

        let cache_data = self.cache.get(&url);

        let resp = match cache_data {
            Some(r) => r,
            None => {
                println!("Cache miss!");
                let req_builder = self
                    .client
                    .lock()
                    .await
                    .request(method, &url);
    
                let req_builder = match query {
                    Some(m) => req_builder.query(&m),
                    None => req_builder
                };
    
                let resp = req_builder
                    .send()
                    .await?;
    
                let status = resp.status();
    
                // self.cache.insert_get(url, resp.bytes().await?).await
                match status {
                    _ if status.as_u16() <= 300 => self.cache.insert_get(url, resp.bytes().await?),
                    _ => return Err(Box::new(resp.json::<ApiError>().await?.status))
                }
            }
        };

        Ok(serde_json::from_slice(&resp)?)
    }
}