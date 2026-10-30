use nhl_api::{Client, GameLog, GameType, Season};

/// Carey Price, the goalie whose game logs used to fail with
/// `missing field 'points'`.
const CAREY_PRICE: i64 = 8471679;
const CONNOR_MCDAVID: i64 = 8478402;

#[tokio::test]
#[ignore = "calls the live NHL API"]
async fn test_goalie_game_log_regular_season() {
    let client = Client::new().unwrap();
    let log = client
        .player_game_log(CAREY_PRICE, Season::new(2019), GameType::RegularSeason)
        .await
        .unwrap();

    assert!(!log.game_log.is_empty());
    assert!(log
        .game_log
        .iter()
        .all(|entry| matches!(entry, GameLog::Goalie(_))));
}

#[tokio::test]
#[ignore = "calls the live NHL API"]
async fn test_goalie_game_log_playoffs() {
    let client = Client::new().unwrap();
    let log = client
        .player_game_log(CAREY_PRICE, Season::new(2010), GameType::Playoffs)
        .await
        .unwrap();

    assert!(!log.game_log.is_empty());
    for entry in &log.game_log {
        let goalie = entry.as_goalie().expect("expected a goalie entry");
        assert!(goalie.goals_against <= goalie.shots_against);
    }
}

#[tokio::test]
#[ignore = "calls the live NHL API"]
async fn test_skater_game_log_regular_season() {
    let client = Client::new().unwrap();
    let log = client
        .player_game_log(CONNOR_MCDAVID, Season::new(2023), GameType::RegularSeason)
        .await
        .unwrap();

    assert!(!log.game_log.is_empty());
    for entry in &log.game_log {
        let skater = entry.as_skater().expect("expected a skater entry");
        assert_eq!(skater.points, skater.goals + skater.assists);
    }
}
