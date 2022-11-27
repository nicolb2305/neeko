pub mod client;
pub mod endpoints;

#[cfg(test)]
mod tests {
    use tokio::time::sleep;
    use super::*;
    use std::{error, env, time::Duration};
    use crate::endpoints::constants::Game;

    type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

    fn create_client(api_key: Option<String>) -> Result<client::Client> {
        let api_key = match api_key {
            Some(v) => v,
            None => env::var("riot_api_key")?
        };
        let region = client::constants::Region::EUW;
        let client = client::Client::new(api_key, region)?;
        Ok(client)
    }

    #[tokio::test]
    #[should_panic(expected = "Invalid api_key")]
    async fn make_request_with_bad_api_key() {
        let client = create_client(Some("".to_string())).expect("Failed to create client.");

        let summoner_name = "Påsan";
        client.get_summoner_by_name(summoner_name).await.expect("Invalid api_key");
    }

    #[tokio::test]
    async fn get_summoner() {
        let client = create_client(None).expect("Failed to create client.");

        let summoner_name = "Påsan";
        let mut summoner = client.get_summoner_by_name(summoner_name).await.expect("Failed to get summoner.");
        for _ in 0..6 {
            sleep(Duration::new(2, 0)).await;
            summoner = client.get_summoner_by_name(summoner_name).await.expect("Failed to get summoner.");
        }

        assert_eq!(summoner_name, summoner.name);
    }

    #[tokio::test]
    async fn get_matches() {
        let client = create_client(None).expect("Failed to create client.");

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
        let client = create_client(None).expect("Failed to create client.");

        let match_id = "EUW1_6151255544".to_string();
        let match_ = client.get_match(match_id.clone()).await.expect("Failed to get match");

        assert_eq!(match_.metadata.match_id, match_id, "Checking that correct match is returned");
    }

    #[tokio::test]
    async fn get_match_timeline() {
        let client = create_client(None).expect("Failed to create client.");

        let match_id = "EUW1_6151255544".to_string();
        let match_timeline = client.get_match_timeline(match_id.clone()).await.expect("Failed to get match");

        assert_eq!(match_timeline.metadata.match_id, match_id, "Checking that correct match is returned");
    }

    // #[tokio::test]
    // async fn get_current_game_info_by_summoner() {
    //     let client = create_client().expect("Failed to create client.");

    //     let match_id = "TUuxK2yGBiAFT5hhOLEkILm2I-v4FK2WjCxpZ2i1vxl9_PxOlsfv5Nvv9Q".to_string();
    //     let match_timeline = client.get_current_game_info_by_summoner(match_id.clone()).await.expect("Failed to get match");
    //     println!("{:#?}", match_timeline);

    //     // assert_eq!(match_timeline.metadata.match_id, match_id, "Checking that correct match is returned");
    // }

    #[tokio::test]
    async fn get_featured_games() {
        let client = create_client(None).expect("Failed to create client.");

        let featured_games = client.get_featured_games().await.expect("Failed to get featured games");

        assert_eq!(featured_games.client_refresh_interval, 300);
    }

    #[tokio::test]
    async fn get_platform_data() {
        let client = create_client(None).expect("Failed to create client.");

        let platform_data = client.get_platform_data().await.expect("Failed to get platform data");

        assert_eq!(platform_data.id, "EUW1");
    }

    #[tokio::test]
    async fn get_account_by_puuid() {
        let developer_api_key = env::var("riot_api_key_developer").expect("Failed to get developer api_key");
        let client = create_client(Some(developer_api_key)).expect("Failed to create client.");

        let puuid = "svh2qpmF4m9b9dJ0iPBSVhJVVs6MFIFyNm8Oo__8yCPZnsKCGG3yGkFPxtuFoa5Rbbqp9KekRwKNZQ";
        let account = client.get_account_by_puuid(puuid.to_string()).await.expect("Failed to get account info");

        assert_eq!(account.puuid, puuid);
    }

    #[tokio::test]
    async fn get_account_by_riot_id() {
        let developer_api_key = env::var("riot_api_key_developer").expect("Failed to get developer api_key");
        let client = create_client(Some(developer_api_key)).expect("Failed to create client.");

        let tag_line = "Neeko".to_string();
        let game_name = "Påsan".to_string();
        let account = client.get_account_by_riot_id(game_name.clone(), tag_line.clone()).await.expect("Failed to get account info");

        assert_eq!(account.tag_line, Some(tag_line));
        assert_eq!(account.game_name, Some(game_name));
    }

    #[tokio::test]
    async fn get_active_shard() {
        let developer_api_key = env::var("riot_api_key_developer").expect("Failed to get developer api_key");
        let client = create_client(Some(developer_api_key)).expect("Failed to create client.");

        let game = Game::LOR;
        let puuid = "1UtbqBdSx_-HfxzSKGq_lmgF-J6_LNFhkaoTBV0abdzt4EDth9qju30M61mLNQ9g2AX2pX4DRvQJTA".to_string();
        let active_shard = client.get_active_shard(game.clone(), puuid.clone()).await.expect("Failed to get active shard");

        assert_eq!(active_shard.game, game);
        assert_eq!(active_shard.puuid, puuid);
    }

    #[tokio::test]
    async fn get_champion_masteries() {
        let client = create_client(None).expect("Failed to create client.");

        let encrypted_summoner_id = "-_Qcp4WDs8X7X_E62lgulgzHRHpNZ4vjk0TAYzMV9zCcWzY".to_string();
        let champion_masteries = client
            .get_champion_masteries(encrypted_summoner_id.clone())
            .await
            .expect("Failed to get champion masteries");

        assert_eq!(champion_masteries[0].summoner_id, encrypted_summoner_id);
    }

    #[tokio::test]
    async fn get_champion_mastery_by_champion_id() {
        let client = create_client(None).expect("Failed to create client.");

        let encrypted_summoner_id = "-_Qcp4WDs8X7X_E62lgulgzHRHpNZ4vjk0TAYzMV9zCcWzY".to_string();
        let champion_id = 518;
        let champion_mastery = client
            .get_champion_mastery_by_champion_id(encrypted_summoner_id.clone(), champion_id)
            .await
            .expect("Failed to get champion mastery");

        assert_eq!(champion_mastery.summoner_id, encrypted_summoner_id);
        assert_eq!(champion_mastery.champion_id, champion_id);
    }

    #[tokio::test]
    async fn get_top_champion_masteries() {
        let client = create_client(None).expect("Failed to create client.");

        let encrypted_summoner_id = "-_Qcp4WDs8X7X_E62lgulgzHRHpNZ4vjk0TAYzMV9zCcWzY".to_string();
        let count = 5;
        let champion_masteries = client
            .get_top_champion_masteries(encrypted_summoner_id.clone(), Some(count))
            .await
            .expect("Failed to get top champion masteries");

        assert_eq!(champion_masteries.len(), count as usize);
        assert_eq!(champion_masteries[0].summoner_id, encrypted_summoner_id);
    }

    #[tokio::test]
    async fn get_champion_mastery_score() {
        let client = create_client(None).expect("Failed to create client.");

        let encrypted_summoner_id = "-_Qcp4WDs8X7X_E62lgulgzHRHpNZ4vjk0TAYzMV9zCcWzY".to_string();
        let champion_mastery_score = client
            .get_champion_mastery_score(encrypted_summoner_id)
            .await
            .expect("Failed to get champion mastery score");

        assert_ne!(champion_mastery_score, 0);
    }

    #[tokio::test]
    async fn get_champion_rotation() {
        let client = create_client(None).expect("Failed to create client.");

        let champion_rotation = client
            .get_champion_rotation()
            .await
            .expect("Failed to get champion rotation");

        assert_eq!(champion_rotation.max_new_player_level, 10);
    }
}
