pub mod constants;
use std::collections::HashMap;
use reqwest::Method;
use crate::client::Client;
use self::constants::{SummonerDTO, MatchDto, MatchTimelineDto, CurrentGameInfo, FeaturedGames, PlatformDataDto, AccountDto, ActiveShardDto, ChampionMasteryDto, ChampionInfo, Game};

type Result<T> = std::result::Result<T, reqwest::Error>;

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
    //Summoner V4
    pub async fn get_summoner_by_account_id(
        &self, 
        encrypted_account_id: &str
    ) -> Result<SummonerDTO> {
        let endpoint = format!("/lol/summoner/v4/summoners/by-account/{encrypted_account_id}");
        let req = self.request(Method::GET, endpoint, true, None).await?;
        Ok(req)
    }

    pub async fn get_summoner_by_name(
        &self, 
        summoner_name: &str
    ) -> Result<SummonerDTO> {
        let endpoint = format!("/lol/summoner/v4/summoners/by-name/{summoner_name}");
        let req = self.request(Method::GET, endpoint, true, None).await?;
        Ok(req)
    }
    
    pub async fn get_summoner_by_puuid(
        &self, 
        encrypted_puuid: &str
    ) -> Result<SummonerDTO> {
        let endpoint = format!("/lol/summoner/v4/summoners/by-puuid/{encrypted_puuid}");
        let req = self.request(Method::GET, endpoint, true, None).await?;
        Ok(req)
    }
    
    pub async fn get_summoner_by_summoner_id(
        &self, 
        encrypted_summoner_id: &str
    ) -> Result<SummonerDTO> {
        let endpoint = format!("/lol/summoner/v4/summoners/{encrypted_summoner_id}");
        let req = self.request(Method::GET, endpoint, true, None).await?;
        Ok(req)
    }

    // Match V5
    pub async fn get_matches(
        &self, 
        puuid: String, 
        start_time: Option<i64>, 
        end_time: Option<i64>,
        queue: Option<i32>,
        type_: Option<String>,
        start: Option<i32>,
        count: Option<i32>
    ) -> Result<Vec<String>> {
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
    ) -> Result<MatchDto> {
        let endpoint = format!("/lol/match/v5/matches/{match_id}");
        let req = self.request(Method::GET, endpoint, false, None).await?;
        Ok(req)
    }

    pub async fn get_match_timeline(
        &self,
        match_id: String
    ) -> Result<MatchTimelineDto> {
        let endpoint = format!("/lol/match/v5/matches/{match_id}/timeline");
        let req = self.request(Method::GET, endpoint, false, None).await?;
        Ok(req)
    }

    // Spectator V4
    pub async fn get_current_game_info_by_summoner(
        &self,
        encrypted_summoner_id: String
    ) -> Result<CurrentGameInfo> {
        let endpoint = format!("/lol/spectator/v4/active-games/by-summoner/{encrypted_summoner_id}");
        let req = self.request(Method::GET, endpoint, true, None).await?;
        Ok(req)
    }

    pub async fn get_featured_games(&self) -> Result<FeaturedGames> {
        let endpoint = "/lol/spectator/v4/featured-games".to_string();
        let req = self.request(Method::GET, endpoint, true, None).await?;
        Ok(req)
    }

    // LoL Status V4
    pub async fn get_platform_data(&self) -> Result<PlatformDataDto> {
        let endpoint = "/lol/status/v4/platform-data".to_string();
        let req = self.request(Method::GET, endpoint, true, None).await?;
        Ok(req)
    }

    // Account V1
    pub async fn get_account_by_puuid(
        &self,
        puuid: String
    ) -> Result<AccountDto> {
        let endpoint = format!("/riot/account/v1/accounts/by-puuid/{puuid}");
        let req = self.request(Method::GET, endpoint, false, None).await?;
        Ok(req)
    }
    
    pub async fn get_account_by_riot_id(
        &self,
        game_name: String,
        tag_line: String
    ) -> Result<AccountDto> {
        let endpoint = format!("/riot/account/v1/accounts/by-riot-id/{game_name}/{tag_line}");
        let req = self.request(Method::GET, endpoint, false, None).await?;
        Ok(req)
    }

    pub async fn get_active_shard(
        &self,
        game: Game,
        puuid: String
    ) -> Result<ActiveShardDto> {
        let game = match game {
            Game::LOR => "lor",
            Game::VAL => "val"
        };
        let endpoint = format!("/riot/account/v1/active-shards/by-game/{game}/by-puuid/{puuid}");
        let req = self.request(Method::GET, endpoint, false, None).await?;
        Ok(req)
    }

    // Champion Mastery V4
    pub async fn get_champion_masteries(
        &self,
        encrypted_summoner_id: String,
    ) -> Result<Vec<ChampionMasteryDto>> {
        let endpoint = format!("/lol/champion-mastery/v4/champion-masteries/by-summoner/{encrypted_summoner_id}");
        let req = self.request(Method::GET, endpoint, true, None).await?;
        Ok(req)
    }
    
    pub async fn get_champion_mastery_by_champion_id(
        &self,
        encrypted_summoner_id: String,
        champion_id: i64
    ) -> Result<ChampionMasteryDto> {
        let endpoint = format!("/lol/champion-mastery/v4/champion-masteries/by-summoner/{encrypted_summoner_id}/by-champion/{champion_id}");
        let req = self.request(Method::GET, endpoint, true, None).await?;
        Ok(req)
    }
    
    pub async fn get_top_champion_masteries(
        &self,
        encrypted_summoner_id: String,
        count: Option<i32>
    ) -> Result<Vec<ChampionMasteryDto>> {
        let endpoint = format!("/lol/champion-mastery/v4/champion-masteries/by-summoner/{encrypted_summoner_id}/top");

        let mut query = HashMap::new();
        insert_query(&mut query, "count", &count);

        let req = self.request(Method::GET, endpoint, true, Some(query)).await?;
        Ok(req)
    }
    
    pub async fn get_champion_mastery_score(
        &self,
        encrypted_summoner_id: String
    ) -> Result<i32> {
        let endpoint = format!("/lol/champion-mastery/v4/scores/by-summoner/{encrypted_summoner_id}");

        let req = self.request(Method::GET, endpoint, true, None).await?;
        Ok(req)
    }
    
    // Champion V3
    pub async fn get_champion_rotation(
        &self
    ) -> Result<ChampionInfo> {
        let endpoint = "/lol/platform/v3/champion-rotations".to_string();

        let req = self.request(Method::GET, endpoint, true, None).await?;
        Ok(req)
    }
}