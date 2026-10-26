use super::*;
use crate::ids::PlayerId;

#[test]
fn test_game_summary_missing_shootout_and_three_stars() {
    let json = r#"{
            "scoring": [],
            "penalties": []
        }"#;

    let summary: GameSummary = serde_json::from_str(json).unwrap();
    assert!(summary.shootout.is_empty());
    assert!(summary.three_stars.is_empty());
}

#[test]
fn test_game_summary_with_shootout_and_three_stars() {
    let json = r#"{
            "scoring": [],
            "shootout": [{
                "sequence": 1,
                "playerId": 8478402,
                "teamAbbrev": {"default": "EDM"},
                "firstName": {"default": "Connor"},
                "lastName": {"default": "McDavid"},
                "shotType": "wrist",
                "result": "goal",
                "headshot": "https://assets.nhle.com/mugs/nhl/20242025/EDM/8478402.png",
                "gameWinner": true
            }],
            "threeStars": [{
                "star": 1,
                "playerId": 8478402,
                "teamAbbrev": "EDM",
                "headshot": "https://assets.nhle.com/mugs/nhl/20242025/EDM/8478402.png",
                "name": "C. McDavid",
                "sweaterNo": 97
            }],
            "penalties": []
        }"#;

    let summary: GameSummary = serde_json::from_str(json).unwrap();
    assert_eq!(summary.shootout.len(), 1);
    assert!(summary.shootout[0].game_winner);
    assert_eq!(summary.three_stars.len(), 1);
    assert_eq!(summary.three_stars[0].player_id, PlayerId::new(8478402));
}
