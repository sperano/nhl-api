//! Game-center responses of older games, recorded from the live API, which
//! omit fields modern games always send.

use super::*;

const PLAY_BY_PLAY_2005: &str =
    include_str!("../../../tests/fixtures/play_by_play/regular_20052006.json");
const PLAY_BY_PLAY_2010: &str =
    include_str!("../../../tests/fixtures/play_by_play/regular_20102011.json");
const LANDING_1995: &str = include_str!("../../../tests/fixtures/landing/regular_19951996.json");
const SEASON_SERIES_1995: &str =
    include_str!("../../../tests/fixtures/season_series/regular_19951996.json");
const GAME_STORY_1925: &str =
    include_str!("../../../tests/fixtures/game_story/regular_19251926.json");

#[test]
fn test_play_by_play_without_situation_codes() {
    let pbp: PlayByPlay = serde_json::from_str(PLAY_BY_PLAY_2005).unwrap();
    assert!(!pbp.plays.is_empty());
    assert!(pbp.plays.iter().all(|play| play.situation_code.is_none()));
    assert!(pbp.plays.iter().all(|play| play.situation().is_none()));
}

#[test]
fn test_play_by_play_period_start_without_situation_code() {
    let pbp: PlayByPlay = serde_json::from_str(PLAY_BY_PLAY_2010).unwrap();
    let period_start = &pbp.plays[0];
    assert_eq!(period_start.type_desc_key, PlayEventType::PeriodStart);
    assert_eq!(period_start.situation(), None);
    assert!(pbp.plays.iter().any(|play| play.situation().is_some()));
}

#[test]
fn test_landing_goals_without_shot_type_or_situation() {
    let landing: GameMatchup = serde_json::from_str(LANDING_1995).unwrap();
    let summary = landing.summary.expect("final game has a summary");
    let goal = &summary.scoring[0].goals[0];
    assert_eq!(goal.name.default, "D. Ciccarelli");
    assert_eq!(goal.shot_type, None);
    assert_eq!(goal.situation_code, None);
}

#[test]
fn test_season_series_officials_without_sweater_numbers() {
    let series: SeasonSeriesMatchup = serde_json::from_str(SEASON_SERIES_1995).unwrap();
    let referee = &series.game_info.referees[0];
    assert_eq!(referee.full_name.default, "Paul Stewart");
    assert_eq!(referee.sweater_number, None);
}

#[test]
fn test_game_story_without_shot_counts() {
    let story: GameStory = serde_json::from_str(GAME_STORY_1925).unwrap();
    assert_eq!(story.away_team.abbrev, "PIR");
    assert_eq!(story.away_team.sog, None);
}
