use nhl_api::{Client, PlayEventType};

#[tokio::test]
async fn test_play_by_play_real_game() {
    let client = Client::new().unwrap();

    // Use a known completed game
    let pbp = client.play_by_play(2024020444).await.unwrap();

    // Verify basic structure
    assert!(!pbp.plays.is_empty());
    assert!(!pbp.roster_spots.is_empty());

    // Verify goals exist and have proper details
    let goals = pbp.goals();
    assert!(!goals.is_empty());

    for goal in goals {
        assert_eq!(goal.type_desc_key, PlayEventType::Goal);
        let details = goal.details.as_ref().unwrap();
        assert!(details.scoring_player_id.is_some());
    }

    // Test situation parsing
    for play in &pbp.plays {
        if let Some(situation) = play.situation() {
            assert!(situation.away_skaters >= 3 && situation.away_skaters <= 6);
            assert!(situation.home_skaters >= 3 && situation.home_skaters <= 6);
        }
    }
}

#[tokio::test]
async fn test_recent_plays() {
    let client = Client::new().unwrap();
    let pbp = client.play_by_play(2024020444).await.unwrap();

    let recent = pbp.recent_plays(5);
    assert_eq!(recent.len(), 5);

    // Most recent should be game-end
    assert_eq!(recent[0].type_desc_key, PlayEventType::GameEnd);
}

#[tokio::test]
async fn test_play_by_play_helper_methods() {
    let client = Client::new().unwrap();
    let pbp = client.play_by_play(2024020444).await.unwrap();

    // Test penalties helper
    let penalties = pbp.penalties();
    for penalty in &penalties {
        assert_eq!(penalty.type_desc_key, PlayEventType::Penalty);
    }

    // Test shots helper (includes goals, shots on goal, missed shots, blocked shots)
    let shots = pbp.shots();
    for shot in &shots {
        assert!(shot.type_desc_key.is_scoring_chance());
    }

    // Test plays_in_period
    let period_1_plays = pbp.plays_in_period(1);
    assert!(!period_1_plays.is_empty());
    for play in &period_1_plays {
        assert_eq!(play.period_descriptor.number, 1);
    }

    // Test get_player with a known player from goals
    if let Some(goal) = pbp.goals().first() {
        if let Some(details) = &goal.details {
            if let Some(scorer_id) = details.scoring_player_id {
                let player = pbp.get_player(scorer_id);
                assert!(player.is_some());
            }
        }
    }

    // Test team_roster
    let home_roster = pbp.team_roster(pbp.home_team.id);
    let away_roster = pbp.team_roster(pbp.away_team.id);
    assert!(!home_roster.is_empty());
    assert!(!away_roster.is_empty());

    // Test current_situation
    let situation = pbp.current_situation();
    assert!(situation.is_some());
}
