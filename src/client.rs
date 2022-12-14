pub mod constants;
use crate::endpoints::constants::Error as ApiError;
use self::constants::Region;
use std::{collections::HashMap, error::Error};
use serde::de::DeserializeOwned;

// type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
type Result<T> = std::result::Result<T, reqwest::Error>;

pub struct Client {
    client: reqwest::Client,
    region: Region
}

impl Client {
    pub fn new(api_key: String, region: Region) -> Result<Self> {
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

        let req_builder = self
            .client
            .request(method, url);

        let req_builder = match query {
            Some(m) => req_builder.query(&m),
            None => req_builder
        };

        let resp = req_builder
            .send()
            .await?;

        let status = resp.status();

        match status {
            _ if status.as_u16() <= 300 => Ok(resp.json().await?),
            _ => Err(Box::new(resp.json::<ApiError>().await?.status))
        }
    }
}