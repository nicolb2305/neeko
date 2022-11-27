pub mod constants;
use crate::endpoints::constants::Error as ApiError;
use self::constants::Region;
use std::{collections::HashMap, error::Error, time::Duration};
use governor::{RateLimiter, Quota, state::{NotKeyed, InMemoryState}, clock::QuantaClock};
use serde::de::DeserializeOwned;

// type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
type Result<T> = std::result::Result<T, reqwest::Error>;

pub struct Client {
    client: reqwest::Client,
    region: Region,
    rate_limiters: HashMap<String, Vec<RateLimiter<NotKeyed, InMemoryState, QuantaClock>>>
}

async fn wait_rate_limit(client: &mut Client, endpoint: String, create_if_not_exist: bool, limits: Option<String>) -> () {
    if !create_if_not_exist {
        match client.rate_limiters.get(&endpoint) {
            Some(r) => {
                for x in r.into_iter() {
                    x.until_ready().await;
                }
                ()
            },
            None => ()
        };
    } else {
        match client.rate_limiters.get(&endpoint) {
            None => {
                let limits = limits.expect("limits must be defined if create_if_not_exist");
                // let test = limits
                //     .split(',')
                //     .map(| x | 
                //                 x
                //                     .split(':')
                //                     .map(| y | y.parse::<u64>().unwrap())
                //                     .reduce(| y, z | y*1_000_000 / z)
                //                     .unwrap()
                //                 )
                //     .collect::<Vec<_>>();
                // println!("{:#?}", test);
                let limits = limits
                    .split(',')
                    .map(| x | RateLimiter::direct(
                        Quota::with_period(
                            Duration::from_millis(
                                x
                                    .split(':')
                                    .map(| y | y.parse::<u64>().unwrap())
                                    .reduce(| y, z | z*1_000 / y)
                                    .unwrap()
                            )
                        ).unwrap()
                    ))
                    .collect();
                client.rate_limiters.insert(endpoint.clone(), limits);
                let r = client.rate_limiters.get(&endpoint).unwrap();
                for x in r.into_iter() {
                    x.until_ready().await;
                }
                ()
            },
            Some(_) => ()
        };
    }
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
            region,
            rate_limiters: HashMap::new()
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
        wait_rate_limit(self, "global".to_string(), false, None).await;
        wait_rate_limit(self, endpoint.clone(), false, None).await;

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

        wait_rate_limit(self, "global".to_string(), true, Some(resp.headers().get("X-App-Rate-Limit").unwrap().to_str().unwrap().to_string())).await;
        wait_rate_limit(self, endpoint, true, Some(resp.headers().get("X-Method-Rate-Limit").unwrap().to_str().unwrap().to_string())).await;

        let status = resp.status();

        match status {
            _ if status.as_u16() <= 300 => Ok(resp.json().await?),
            _ => Err(Box::new(resp.json::<ApiError>().await?.status))
        }
    }
}