pub mod constants;
use crate::endpoints::constants::Error as ApiError;
use self::constants::Region;
use std::{collections::HashMap, error::Error};
use bytes::Bytes;
use serde::de::DeserializeOwned;
use tokio::{sync::Mutex, time::{Instant, Duration}};

// type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
type Result<T> = std::result::Result<T, reqwest::Error>;

struct DataWithTimestamp {
    data: Bytes,
    timestamp: Instant 
}

impl DataWithTimestamp {
    fn new(data: Bytes) -> Self {
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
    store: HashMap<String, DataWithTimestamp>,
    lifetime: Duration
}

impl Cache {
    fn new(lifetime: Duration) -> Self {
        Self {
            store: HashMap::new(),
            lifetime
        }
    }

    async fn get(
        &mut self, 
        url: &String, 
        client: &Mutex<reqwest::Client>, 
        method: reqwest::Method, 
        query: Option<HashMap<&str, String>>
    ) -> std::result::Result<Bytes, Box<dyn Error>>  {
        match self.store.get(url) {
            Some(v) if v.valid(self.lifetime) => Ok(v.data.clone()),
            _ => {
                println!("Cache miss!");

                let req = client
                    .lock()
                    .await
                    .request(method, url);

                let req = match query {
                    Some(m) => req.query(&m),
                    None => req
                };

                let resp = req.send().await?;
                let status = resp.status();

                match status {
                    _ if status.as_u16() <= 300 => {
                        let bytes_data = resp.bytes().await?;
                        self.store.insert(url.clone(), DataWithTimestamp::new(bytes_data.clone()));
                        Ok(bytes_data)
                    },
                    _ => return Err(Box::new(resp.json::<ApiError>().await?.status))
                }
            }
        }
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

        match self.cache.get(&url, &self.client, method, query).await {
            Ok(v) => Ok(serde_json::from_slice(&v)?),
            Err(e) => Err(e)
        }
    }
}