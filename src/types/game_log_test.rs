use super::*;
use crate::date::Season;
use crate::types::game_type::GameType;
use crate::types::player::PlayerGameLog;

/// Carey Price, 2007-08 regular season (live API response, trimmed to five
/// entries: two wins, a start pulled without a decision, a relief appearance,
/// and an overtime loss).
const GOALIE_FIXTURE: &str = include_str!("../../tests/fixtures/player_game_log_goalie.json");

/// Connor McDavid, 2023-24 regular season (live API response, trimmed to two
/// entries).
const SKATER_FIXTURE: &str = include_str!("../../tests/fixtures/player_game_log_skater.json");

/// Martin Brodeur, 2003-04 regular season: a pre-shootout tie.
const GOALIE_TIE_ENTRY: &str = r#"{
    "gameId": 2003021107, "teamAbbrev": "NJD", "homeRoadFlag": "H",
    "gameDate": "2004-03-19", "goals": 0, "assists": 0,
    "commonName": {"default": "Devils"}, "opponentCommonName": {"default": "Canadiens"},
    "gamesStarted": 1, "decision": "T", "shotsAgainst": 31, "goalsAgainst": 1,
    "savePctg": 0.9677, "shutouts": 0, "pim": 0, "toi": "65:00", "opponentAbbrev": "MTL"
}"#;

const SKATER_ENTRY: &str = r#"{
    "gameId": 2023020001,
    "gameDate": "2023-10-10",
    "teamAbbrev": "EDM",
    "homeRoadFlag": "H",
    "opponentAbbrev": "VAN",
    "goals": 1,
    "assists": 2,
    "points": 3,
    "plusMinus": 1,
    "powerPlayGoals": 0,
    "powerPlayPoints": 1,
    "shots": 4,
    "shifts": 22,
    "toi": "20:15"
}"#;

fn goalie(entry: &GameLog) -> &GoalieGameLog {
    entry.as_goalie().expect("expected a goalie entry")
}

#[test]
fn test_game_log_skater_deserialization() {
    let game_log: GameLog = serde_json::from_str(SKATER_ENTRY).unwrap();
    let skater = game_log.as_skater().expect("expected a skater entry");
    assert_eq!(skater.game_id, GameId::new(2023020001));
    assert_eq!(skater.goals, 1);
    assert_eq!(skater.points, 3);
    assert!(game_log.as_goalie().is_none());
}

/// `SkaterGameLog.game_id` accepts a numeric-string form too (1.3).
#[test]
fn test_game_log_game_id_deserializes_from_numeric_string() {
    let json = SKATER_ENTRY.replace("2023020001", "\"2023020001\"");
    let game_log: GameLog = serde_json::from_str(&json).unwrap();
    assert_eq!(game_log.game_id(), GameId::new(2023020001));
}

#[test]
fn test_player_game_log_goalie_fixture() {
    let log: PlayerGameLog = serde_json::from_str(GOALIE_FIXTURE).unwrap();
    assert_eq!(log.season, Season::new(2007));
    assert_eq!(log.game_type, GameType::RegularSeason);
    assert_eq!(log.game_log.len(), 5);
    assert!(log.game_log.iter().all(|entry| entry.as_goalie().is_some()));

    let decisions: Vec<_> = log.game_log.iter().map(|e| goalie(e).decision).collect();
    assert_eq!(
        decisions,
        vec![
            Some(GoalieDecision::Win),
            Some(GoalieDecision::Win),
            None,
            None,
            Some(GoalieDecision::OvertimeLoss),
        ]
    );

    let first = goalie(&log.game_log[0]);
    assert_eq!(first.game_id, GameId::new(2007021220));
    assert_eq!(first.home_road_flag, HomeRoad::Home);
    assert_eq!(first.games_started, 1);
    assert_eq!(first.shots_against, 27);
    assert_eq!(first.goals_against, 1);
    assert_eq!(first.save_pctg, Some(0.963));
    assert_eq!(first.shutouts, 0);
    assert_eq!(first.toi, "59:50");
    assert_eq!(first.pim, Some(0));

    let relief = goalie(&log.game_log[3]);
    assert_eq!(relief.games_started, 0);
    assert_eq!(relief.assists, 1);
}

#[test]
fn test_player_game_log_skater_fixture() {
    let log: PlayerGameLog = serde_json::from_str(SKATER_FIXTURE).unwrap();
    assert_eq!(log.season, Season::new(2023));
    assert_eq!(log.game_log.len(), 2);

    let first = log.game_log[0]
        .as_skater()
        .expect("expected a skater entry");
    assert_eq!(first.game_id, GameId::new(2023021306));
    assert_eq!(first.plus_minus, -2);
    assert_eq!(first.shots, 2);
    assert_eq!(first.shifts, 19);
    assert_eq!(first.game_winning_goals, Some(0));
}

#[test]
fn test_game_log_goalie_tie_decision() {
    let game_log: GameLog = serde_json::from_str(GOALIE_TIE_ENTRY).unwrap();
    assert_eq!(goalie(&game_log).decision, Some(GoalieDecision::Tie));
}

#[test]
fn test_game_log_goalie_empty_decision_is_none() {
    let json = GOALIE_TIE_ENTRY.replace(r#""decision": "T""#, r#""decision": """#);
    let game_log: GameLog = serde_json::from_str(&json).unwrap();
    assert_eq!(goalie(&game_log).decision, None);
}

#[test]
fn test_game_log_goalie_unknown_decision_fails() {
    let json = GOALIE_TIE_ENTRY.replace(r#""decision": "T""#, r#""decision": "X""#);
    let err = serde_json::from_str::<GameLog>(&json).unwrap_err();
    assert!(err.to_string().contains("goalie decision"), "{err}");
}

/// The variant is chosen up front, so a malformed entry names the offending
/// field instead of a generic "did not match any variant" error.
#[test]
fn test_game_log_missing_field_names_the_field() {
    let goalie_json = GOALIE_TIE_ENTRY.replace(r#""goalsAgainst": 1,"#, "");
    let err = serde_json::from_str::<GameLog>(&goalie_json).unwrap_err();
    assert!(err.to_string().contains("goalsAgainst"), "{err}");

    let skater_json = SKATER_ENTRY.replace(r#""points": 3,"#, "");
    let err = serde_json::from_str::<GameLog>(&skater_json).unwrap_err();
    assert!(err.to_string().contains("points"), "{err}");
}

#[test]
fn test_game_log_round_trip_keeps_variant() {
    let log: PlayerGameLog = serde_json::from_str(GOALIE_FIXTURE).unwrap();
    for entry in log.game_log {
        let serialized = serde_json::to_string(&entry).unwrap();
        assert!(!serialized.contains("Goalie"), "untagged: {serialized}");
        let back: GameLog = serde_json::from_str(&serialized).unwrap();
        assert_eq!(back, entry);
    }

    let skater: GameLog = serde_json::from_str(SKATER_ENTRY).unwrap();
    let back: GameLog = serde_json::from_str(&serde_json::to_string(&skater).unwrap()).unwrap();
    assert_eq!(back, skater);
}

#[test]
fn test_game_log_goalie_serialize_omits_none_decision() {
    let log: PlayerGameLog = serde_json::from_str(GOALIE_FIXTURE).unwrap();
    let serialized = serde_json::to_string(&log.game_log[2]).unwrap();
    assert!(!serialized.contains("decision"), "{serialized}");
}

#[test]
fn test_game_log_common_accessors() {
    let skater: GameLog = serde_json::from_str(SKATER_ENTRY).unwrap();
    let goalie: GameLog = serde_json::from_str(GOALIE_TIE_ENTRY).unwrap();

    assert_eq!(skater.game_id(), GameId::new(2023020001));
    assert_eq!(skater.game_date(), "2023-10-10");
    assert_eq!(skater.team_abbrev(), "EDM");
    assert_eq!(skater.opponent_abbrev(), "VAN");
    assert_eq!(skater.home_road_flag(), HomeRoad::Home);
    assert_eq!(skater.toi(), "20:15");

    assert_eq!(goalie.game_id(), GameId::new(2003021107));
    assert_eq!(goalie.game_date(), "2004-03-19");
    assert_eq!(goalie.team_abbrev(), "NJD");
    assert_eq!(goalie.opponent_abbrev(), "MTL");
    assert_eq!(goalie.home_road_flag(), HomeRoad::Home);
    assert_eq!(goalie.toi(), "65:00");
}
