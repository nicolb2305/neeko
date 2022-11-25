pub mod constants;
use std::collections::HashMap;
use reqwest::Method;
use crate::client::Client;
use self::constants::{SummonerDTO, MatchDto, MatchTimelineDto};

fn insert_query<'a, T: ToString>(
    query: &mut HashMap<&'a str, String>, 
    k: &'a str, 
    v: &Option<T>
) -> () {
    match v {
        Some(w) => query.insert(k, w.to_string()),
        None => None
    };
}

impl Client {
    pub async fn get_summoner_by_name(
        &self, 
        summoner_name: &str
    ) -> Result<SummonerDTO, reqwest::Error> {
        let endpoint = format!("/lol/summoner/v4/summoners/by-name/{summoner_name}");
        let req = self.request(Method::GET, endpoint, true, None).await?;
        Ok(req)
    }
    
    pub async fn get_matches(
        &self, 
        puuid: String, 
        start_time: Option<i64>, 
        end_time: Option<i64>,
        queue: Option<i32>,
        type_: Option<String>,
        start: Option<i32>,
        count: Option<i32>
    ) -> Result<Vec<String>, reqwest::Error> {
        let endpoint = format!("/lol/match/v5/matches/by-puuid/{puuid}/ids");

        let mut query = HashMap::new();
        insert_query(&mut query, "startTime", &start_time);
        insert_query(&mut query, "endTime", &end_time);
        insert_query(&mut query, "queue", &queue);
        insert_query(&mut query, "type", &type_);
        insert_query(&mut query, "start", &start);
        insert_query(&mut query, "count", &count);

        let req = self.request(Method::GET, endpoint, false, Some(query)).await?;
        Ok(req)
    }

    pub async fn get_match(
        &self, 
        match_id: String
    ) -> Result<MatchDto, reqwest::Error> {
        let endpoint = format!("/lol/match/v5/matches/{match_id}");
        let req = self.request(Method::GET, endpoint, false, None).await?;
        Ok(req)
    }

    pub async fn get_match_timeline(
        &self,
        match_id: String
    ) -> Result<MatchTimelineDto, reqwest::Error> {
        let endpoint = format!("/lol/match/v5/matches/{match_id}/timeline");
        let req = self.request(Method::GET, endpoint, false, None).await?;
        Ok(req)
    }
}