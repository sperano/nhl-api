//! A scheduled (`FUT`) game, recorded before puck drop: no scores, shots,
//! clock, period or player stats yet.

use super::{GameMatchup, GameStory, PlayByPlay, SeasonSeriesMatchup};
use crate::ids::GameId;
use crate::types::{Boxscore, GameState, PeriodDescriptor, PlayerByGameStats};

const BOXSCORE_FIXTURE: &str = include_str!("../../../tests/fixtures/boxscore/fut.json");
const PLAY_BY_PLAY_FIXTURE: &str = include_str!("../../../tests/fixtures/play_by_play/fut.json");
const LANDING_FIXTURE: &str = include_str!("../../../tests/fixtures/landing/fut.json");
const SEASON_SERIES_FIXTURE: &str = include_str!("../../../tests/fixtures/season_series/fut.json");
const GAME_STORY_FIXTURE: &str = include_str!("../../../tests/fixtures/game_story/fut.json");
const GAME_ID: i64 = 2026020154;

#[test]
fn test_pre_game_boxscore_has_no_score_clock_or_stats() {
    let boxscore: Boxscore = serde_json::from_str(BOXSCORE_FIXTURE).unwrap();

    assert_eq!(boxscore.id, GameId::new(GAME_ID));
    assert_eq!(boxscore.game_state, GameState::Future);
    for team in [&boxscore.away_team, &boxscore.home_team] {
        assert_eq!((team.score, team.sog), (None, None));
    }
    assert_eq!(boxscore.clock, None);
    assert_eq!(boxscore.period_descriptor, PeriodDescriptor::default());
    assert_eq!(boxscore.player_by_game_stats, PlayerByGameStats::default());
}

/// Play-by-play sends a `0` score but no shots before puck drop.
#[test]
fn test_pre_game_play_by_play_has_score_but_no_shots() {
    let pbp: PlayByPlay = serde_json::from_str(PLAY_BY_PLAY_FIXTURE).unwrap();

    assert_eq!(pbp.id, GameId::new(GAME_ID));
    assert_eq!(pbp.game_state, GameState::Future);
    for team in [&pbp.away_team, &pbp.home_team] {
        assert_eq!((team.score, team.sog), (Some(0), None));
    }
    assert_eq!(pbp.period_descriptor, PeriodDescriptor::default());
    assert!(pbp.plays.is_empty());
}

#[test]
fn test_pre_game_landing_has_no_score() {
    let landing: GameMatchup = serde_json::from_str(LANDING_FIXTURE).unwrap();

    assert_eq!(landing.id, GameId::new(GAME_ID));
    assert_eq!(landing.game_state, GameState::Future);
    for team in [&landing.away_team, &landing.home_team] {
        assert_eq!((team.score, team.sog), (None, None));
    }
    assert_eq!(landing.clock, None);
    assert_eq!(landing.summary, None);
}

#[test]
fn test_pre_game_season_series_lists_unplayed_games() {
    let series: SeasonSeriesMatchup = serde_json::from_str(SEASON_SERIES_FIXTURE).unwrap();

    assert!(!series.season_series.is_empty());
    for game in &series.season_series {
        assert_eq!(game.game_state, GameState::Future);
        assert_eq!((game.away_team.score, game.home_team.score), (None, None));
        assert_eq!(game.period_descriptor, PeriodDescriptor::default());
    }
}

#[test]
fn test_pre_game_game_story_has_no_score() {
    let story: GameStory = serde_json::from_str(GAME_STORY_FIXTURE).unwrap();

    assert_eq!(story.id, GameId::new(GAME_ID));
    assert_eq!(story.game_state, GameState::Future);
    for team in [&story.away_team, &story.home_team] {
        assert_eq!((team.score, team.sog), (None, None));
    }
}

/// Absent values stay absent when serialized, so a round trip is lossless.
#[test]
fn test_pre_game_boxscore_round_trips_without_absent_fields() {
    let boxscore: Boxscore = serde_json::from_str(BOXSCORE_FIXTURE).unwrap();
    let value = serde_json::to_value(&boxscore).unwrap();

    assert!(value.get("clock").is_none());
    assert!(value["awayTeam"].get("score").is_none());
    assert!(value["homeTeam"].get("sog").is_none());
    let round_trip: Boxscore = serde_json::from_value(value).unwrap();
    assert_eq!(round_trip, boxscore);
}
