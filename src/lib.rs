pub mod client;
pub mod endpoints;

#[cfg(test)]
mod tests {
    use super::*;
    use std::{error, env};

    type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

    fn create_client() -> Result<client::Client> {
        let api_key = env::var("riot_api_key")?;
        let region = client::constants::Region::EUW;
        let client = client::Client::new(api_key, region)?;
        Ok(client)
    }

    #[tokio::test]
    #[should_panic(expected = "Invalid api_key")]
    async fn make_request_with_bad_api_key() {
        let api_key = "".to_string();
        let region = client::constants::Region::EUW;
        let client = client::Client::new(api_key, region).expect("Failed to create client");

        let summoner_name = "Påsan";
        client.get_summoner_by_name(summoner_name).await.expect("Invalid api_key");
    }

    #[tokio::test]
    async fn get_summoner() {
        let client = create_client().expect("Failed to create client.");

        let summoner_name = "Påsan";
        let summoner = client.get_summoner_by_name(summoner_name).await.expect("Failed to get summoner.");

        assert_eq!(summoner_name, summoner.name);
    }

    #[tokio::test]
    async fn get_matches() {
        let client = create_client().expect("Failed to create client.");

        let puuid = "nfyUVw1yBg9vqTFWtUR3DDVnmR7r2oCc2EuNABo49fRGMXKFzSptSbGNbnhsljxQG-SfEVjTSMpNOQ".to_string();
        let matches = client.get_matches(puuid, None, None, None, None, None, Some(5)).await.expect("Failed to get matches");

        assert_eq!(matches.len(), 5, "Checking that correct number of games is returned");

        let mut first_match_split = matches[0].split("_");
        let region = first_match_split.next().expect("Failed to split matchId");
        let match_id = first_match_split.next().expect("Failed to split matchId");
        let match_id_correct = match_id.parse::<i64>();

        assert_eq!(region, "EUW1", "Checking that first matchId starts with 'EUW1'");
        assert!(match_id_correct.is_ok(), "Checking that matchId ends with an integer");
    }

    #[tokio::test]
    async fn get_match() {
        let client = create_client().expect("Failed to create client.");

        let match_id = "EUW1_6151255544".to_string();
        let match_ = client.get_match(match_id.clone()).await.expect("Failed to get match");

        assert_eq!(match_.metadata.match_id, match_id, "Checking that correct match is returned");
    }

    #[tokio::test]
    async fn get_match_timeline() {
        let client = create_client().expect("Failed to create client.");

        let match_id = "EUW1_6151255544".to_string();
        let match_timeline = client.get_match_timeline(match_id.clone()).await.expect("Failed to get match");

        assert_eq!(match_timeline.metadata.match_id, match_id, "Checking that correct match is returned");
    }
}
