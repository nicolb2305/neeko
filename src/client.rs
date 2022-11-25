pub mod constants;
use self::constants::Region;
use std::collections::HashMap;
use serde::de::DeserializeOwned;

// type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct Client {
    client: reqwest::Client,
    region: Region
}

impl Client {
    pub fn new(api_key: String, region: Region) -> Result<Self, reqwest::Error> {
        let mut headers = reqwest::header::HeaderMap::new();

        headers.insert("X-Riot-Token", api_key.parse().unwrap());

        let client = reqwest::ClientBuilder::new()
            .default_headers(headers)
            .build()?;

        Ok(Client {
            client,
            region
        })
    }

    pub async fn request<T>(
        &self, 
        method: reqwest::Method, 
        endpoint: String, 
        short_region: bool, 
        query: Option<HashMap<&str, String>>
    ) -> Result<T, reqwest::Error> 
    where
        T: DeserializeOwned
    {
        let region = if short_region {
            self.region.to_short_region()
        } else {
            self.region.to_long_region()
        };

        let url = format!("https://{region}.api.riotgames.com{endpoint}");

        let req_builder = self
            .client
            .request(method, url);

        let req_builder = match query {
            Some(m) => req_builder.query(&m),
            None => req_builder
        };

        let req = req_builder
            .send()
            .await?
            .json()
            .await?;

        Ok(req)
    }
}