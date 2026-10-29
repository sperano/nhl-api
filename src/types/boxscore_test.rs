//! Boxscores of older games, recorded from the live API, which omit fields
//! modern games always send.

use super::*;

const BOXSCORE_1925: &str = include_str!("../../tests/fixtures/boxscore/regular_19251926.json");
const BOXSCORE_1995: &str = include_str!("../../tests/fixtures/boxscore/regular_19951996.json");
const BOXSCORE_2010: &str = include_str!("../../tests/fixtures/boxscore/regular_20102011.json");

fn parse(json: &str) -> Boxscore {
    serde_json::from_str(json).expect("recorded boxscore deserializes")
}

#[test]
fn test_boxscore_special_event_without_light_logo() {
    let event = parse(BOXSCORE_2010)
        .special_event
        .expect("2010-11 opener is a special event");
    assert_eq!(event.name.default, "Face Off");
    assert_eq!(event.light_logo_url, None);
}

#[test]
fn test_boxscore_skaters_without_toi_before_1997() {
    let boxscore = parse(BOXSCORE_1995);
    let away = &boxscore.player_by_game_stats.away_team;
    assert!(!away.forwards.is_empty());
    assert!(away
        .forwards
        .iter()
        .chain(&away.defense)
        .all(|skater| skater.toi.is_none()));
}

#[test]
fn test_boxscore_backup_goalie_without_stat_line() {
    let boxscore = parse(BOXSCORE_1995);
    let goalies = &boxscore.player_by_game_stats.away_team.goalies;
    let starter = goalies.iter().find(|g| g.starter == Some(true)).unwrap();
    assert_eq!(starter.shots_against, Some(23));
    assert_eq!(starter.toi.as_deref(), Some("58:53"));

    let backup = goalies.iter().find(|g| g.starter == Some(false)).unwrap();
    assert_eq!(backup.name.default, "C. Osgood");
    assert_eq!(backup.shots_against, None);
    assert_eq!(backup.saves, None);
    assert_eq!(backup.goals_against, None);
    assert_eq!(backup.even_strength_shots_against, None);
    assert_eq!(backup.toi, None);
}

#[test]
fn test_boxscore_without_shot_counts_1925() {
    let boxscore = parse(BOXSCORE_1925);
    assert_eq!(boxscore.away_team.sog, None);
    let goalie = &boxscore.player_by_game_stats.away_team.goalies[0];
    assert_eq!(goalie.goals_against, Some(1));
    assert_eq!(goalie.shots_against, None);
    assert_eq!(goalie.saves, None);
}

#[test]
fn test_boxscore_missing_fields_are_not_serialized() {
    let boxscore = parse(BOXSCORE_1995);
    let json = serde_json::to_value(&boxscore).unwrap();
    let backup = &json["playerByGameStats"]["awayTeam"]["goalies"][1];
    assert!(backup.get("shotsAgainst").is_none());
    assert!(json["playerByGameStats"]["awayTeam"]["forwards"][0]
        .get("toi")
        .is_none());
    assert_eq!(serde_json::from_value::<Boxscore>(json).unwrap(), boxscore);
}
