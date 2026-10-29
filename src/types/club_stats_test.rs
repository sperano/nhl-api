use super::*;

const CURRENT_FIXTURE: &str =
    include_str!("../../tests/fixtures/club_stats/mtl_20252026_regular.json");
const PLAYOFFS_FIXTURE: &str =
    include_str!("../../tests/fixtures/club_stats/mtl_20242025_playoffs.json");
const EIGHTIES_FIXTURE: &str =
    include_str!("../../tests/fixtures/club_stats/mtl_19801981_regular.json");
const TWENTIES_FIXTURE: &str =
    include_str!("../../tests/fixtures/club_stats/mtl_19251926_regular.json");

fn parse_fixture(json: &str) -> ClubStats {
    serde_json::from_str(json).unwrap()
}

fn skater<'a>(stats: &'a ClubStats, last_name: &str) -> &'a ClubSkaterStats {
    stats
        .skaters
        .iter()
        .find(|s| s.last_name.default == last_name)
        .unwrap()
}

fn goalie<'a>(stats: &'a ClubStats, last_name: &str) -> &'a ClubGoalieStats {
    stats
        .goalies
        .iter()
        .find(|g| g.last_name.default == last_name)
        .unwrap()
}

fn sample_skater() -> ClubSkaterStats {
    ClubSkaterStats {
        player_id: PlayerId::new(8475233),
        headshot: "test.png".to_string(),
        first_name: LocalizedString {
            default: "David".to_string(),
        },
        last_name: LocalizedString {
            default: "Savard".to_string(),
        },
        position: Some(Position::Defense),
        games_played: 75,
        goals: 1,
        assists: 14,
        points: 15,
        plus_minus: Some(-8),
        penalty_minutes: 36,
        power_play_goals: Some(0),
        shorthanded_goals: Some(0),
        game_winning_goals: 0,
        overtime_goals: 0,
        shots: Some(48),
        shooting_pctg: Some(0.020833),
        avg_time_on_ice_per_game: Some(995.36),
        avg_shifts_per_game: Some(19.84),
        faceoff_win_pctg: Some(0.0),
    }
}

fn sample_goalie() -> ClubGoalieStats {
    ClubGoalieStats {
        player_id: PlayerId::new(8478470),
        headshot: "test.png".to_string(),
        first_name: LocalizedString {
            default: "Sam".to_string(),
        },
        last_name: LocalizedString {
            default: "Montembeault".to_string(),
        },
        games_played: 62,
        games_started: 60,
        wins: Some(31),
        losses: Some(24),
        ties: None,
        overtime_losses: Some(7),
        goals_against_average: 2.818349,
        save_percentage: Some(0.901669),
        shots_against: Some(1678),
        saves: Some(1513),
        goals_against: 166,
        shutouts: 4,
        goals: 0,
        assists: 1,
        points: 1,
        penalty_minutes: Some(0),
        time_on_ice: 212039,
    }
}

fn empty_club_stats() -> ClubStats {
    ClubStats {
        season: Season::new(2024),
        game_type: GameType::RegularSeason,
        skaters: vec![],
        goalies: vec![],
    }
}

fn season_game_types(start_year: u16, game_types: Vec<GameType>) -> SeasonGameTypes {
    SeasonGameTypes {
        season: Season::new(start_year),
        game_types,
    }
}

#[test]
fn test_skater_stats_deserialization() {
    let json = r#"{
        "playerId": 8475233,
        "headshot": "https://assets.nhle.com/mugs/nhl/20242025/MTL/8475233.png",
        "firstName": {
            "default": "David"
        },
        "lastName": {
            "default": "Savard"
        },
        "positionCode": "D",
        "gamesPlayed": 75,
        "goals": 1,
        "assists": 14,
        "points": 15,
        "plusMinus": -8,
        "penaltyMinutes": 36,
        "powerPlayGoals": 0,
        "shorthandedGoals": 0,
        "gameWinningGoals": 0,
        "overtimeGoals": 0,
        "shots": 48,
        "shootingPctg": 0.020833,
        "avgTimeOnIcePerGame": 995.36,
        "avgShiftsPerGame": 19.84,
        "faceoffWinPctg": 0.0
    }"#;

    let stats: ClubSkaterStats = serde_json::from_str(json).unwrap();
    assert_eq!(
        stats,
        ClubSkaterStats {
            headshot: "https://assets.nhle.com/mugs/nhl/20242025/MTL/8475233.png".to_string(),
            ..sample_skater()
        }
    );
}

/// Historical club stats data (some older seasons) return an empty
/// position code.
#[test]
fn test_club_skater_stats_empty_position() {
    let json = r#"{
        "playerId": 8475233,
        "headshot": "https://assets.nhle.com/mugs/nhl/20242025/MTL/8475233.png",
        "firstName": {"default": "David"},
        "lastName": {"default": "Savard"},
        "positionCode": "",
        "gamesPlayed": 75,
        "goals": 1,
        "assists": 14,
        "points": 15,
        "penaltyMinutes": 36,
        "gameWinningGoals": 0,
        "overtimeGoals": 0
    }"#;

    let stats: ClubSkaterStats = serde_json::from_str(json).unwrap();
    assert_eq!(stats.position, None);
}

#[test]
fn test_club_skater_stats_missing_position() {
    let json = r#"{
        "playerId": 8475233,
        "headshot": "https://assets.nhle.com/mugs/nhl/20242025/MTL/8475233.png",
        "firstName": {"default": "David"},
        "lastName": {"default": "Savard"},
        "gamesPlayed": 75,
        "goals": 1,
        "assists": 14,
        "points": 15,
        "penaltyMinutes": 36,
        "gameWinningGoals": 0,
        "overtimeGoals": 0
    }"#;

    let stats: ClubSkaterStats = serde_json::from_str(json).unwrap();
    assert_eq!(stats.position, None);
}

#[test]
fn test_goalie_stats_deserialization() {
    let json = r#"{
        "playerId": 8478470,
        "headshot": "test.png",
        "firstName": {
            "default": "Sam"
        },
        "lastName": {
            "default": "Montembeault"
        },
        "gamesPlayed": 62,
        "gamesStarted": 60,
        "wins": 31,
        "losses": 24,
        "overtimeLosses": 7,
        "goalsAgainstAverage": 2.818349,
        "savePercentage": 0.901669,
        "shotsAgainst": 1678,
        "saves": 1513,
        "goalsAgainst": 166,
        "shutouts": 4,
        "goals": 0,
        "assists": 1,
        "points": 1,
        "penaltyMinutes": 0,
        "timeOnIce": 212039
    }"#;

    let stats: ClubGoalieStats = serde_json::from_str(json).unwrap();
    assert_eq!(stats, sample_goalie());
}

/// `ClubSkaterStats.player_id`/`ClubGoalieStats.player_id` accept
/// numeric-string forms too (1.3).
#[test]
fn test_club_stats_player_id_deserializes_from_numeric_string() {
    let mut skater_json = serde_json::to_value(sample_skater()).unwrap();
    skater_json["playerId"] = "8475233".into();
    let skater: ClubSkaterStats = serde_json::from_value(skater_json).unwrap();
    assert_eq!(skater.player_id, PlayerId::new(8475233));

    let mut goalie_json = serde_json::to_value(sample_goalie()).unwrap();
    goalie_json["playerId"] = "8478470".into();
    let goalie: ClubGoalieStats = serde_json::from_value(goalie_json).unwrap();
    assert_eq!(goalie.player_id, PlayerId::new(8478470));
}

#[test]
fn test_club_stats_deserialization() {
    let json = format!(
        r#"{{"season": "20242025", "gameType": 2, "skaters": [{}], "goalies": [{}]}}"#,
        serde_json::to_string(&sample_skater()).unwrap(),
        serde_json::to_string(&sample_goalie()).unwrap(),
    );

    let stats: ClubStats = serde_json::from_str(&json).unwrap();
    assert_eq!(stats.season, Season::new(2024));
    assert_eq!(stats.game_type, GameType::RegularSeason);
    assert_eq!(stats.skaters, vec![sample_skater()]);
    assert_eq!(stats.goalies, vec![sample_goalie()]);
}

/// A current regular season has every optional field.
#[test]
fn test_club_stats_current_season_fixture() {
    let stats = parse_fixture(CURRENT_FIXTURE);
    assert_eq!(stats.season, Season::new(2025));
    assert_eq!(stats.game_type, GameType::RegularSeason);
    assert!(stats.skaters.iter().all(|s| s.plus_minus.is_some()
        && s.avg_time_on_ice_per_game.is_some()
        && s.avg_shifts_per_game.is_some()
        && s.faceoff_win_pctg.is_some()));
    assert!(stats
        .goalies
        .iter()
        .all(|g| g.overtime_losses.is_some() && g.ties.is_none()));
}

/// Playoff goalie rows may omit `penaltyMinutes`.
#[test]
fn test_club_stats_playoffs_goalie_without_penalty_minutes() {
    let stats = parse_fixture(PLAYOFFS_FIXTURE);
    assert_eq!(stats.game_type, GameType::Playoffs);

    let montembeault = goalie(&stats, "Montembeault");
    assert_eq!(montembeault.penalty_minutes, None);
    assert_eq!(montembeault.record(), "0-2-0");
}

/// 1980-81 skaters have no TOI, shift or faceoff data, and goalies record
/// ties instead of overtime losses.
#[test]
fn test_club_stats_1980s_fixture() {
    let stats = parse_fixture(EIGHTIES_FIXTURE);
    assert_eq!(stats.season, Season::new(1980));

    let acton = skater(&stats, "Acton");
    assert!(acton.plus_minus.is_some());
    assert!(acton.shots.is_some());
    assert_eq!(acton.avg_time_on_ice_per_game, None);
    assert_eq!(acton.avg_shifts_per_game, None);
    assert_eq!(acton.faceoff_win_pctg, None);

    let joly = skater(&stats, "Joly");
    assert_eq!(joly.shots, Some(0));
    assert_eq!(joly.shooting_pctg, None);

    let herron = goalie(&stats, "Herron");
    assert_eq!(herron.ties, Some(6));
    assert_eq!(herron.overtime_losses, None);
    assert_eq!(herron.penalty_minutes, None);
    assert_eq!(herron.record(), "6-9-6");
}

/// 1925-26 skaters lack plus-minus, special-teams goals and shots; goalies
/// lack shots and saves, and one without a decision lacks wins and losses.
#[test]
fn test_club_stats_1920s_fixture() {
    let stats = parse_fixture(TWENTIES_FIXTURE);
    assert_eq!(stats.season, Season::new(1925));

    let boucher = skater(&stats, "Boucher");
    assert_eq!(boucher.penalty_minutes, 118);
    assert_eq!(boucher.plus_minus, None);
    assert_eq!(boucher.power_play_goals, None);
    assert_eq!(boucher.shorthanded_goals, None);
    assert_eq!(boucher.shots, None);
    assert_eq!(boucher.shooting_pctg, None);

    let vezina = goalie(&stats, "Vezina");
    assert_eq!(vezina.wins, None);
    assert_eq!(vezina.losses, None);
    assert_eq!(vezina.ties, None);
    assert_eq!(vezina.save_percentage, None);
    assert_eq!(vezina.shots_against, None);
    assert_eq!(vezina.saves, None);
    assert_eq!(vezina.record(), "0-0");
    assert_eq!(vezina.to_string(), "Georges Vezina - 1 GP, 0-0, 0.000 GAA");

    assert_eq!(goalie(&stats, "Rheaume").record(), "10-20-1");
}

/// `ClubStats.season` is typed `Season`, which accepts the API's string
/// form (e.g. `"20242025"`) as well as the plain-integer form.
#[test]
fn test_club_stats_season_deserializes_from_string_and_int_forms() {
    let from_string: ClubStats = serde_json::from_str(
        r#"{"season": "20242025", "gameType": 2, "skaters": [], "goalies": []}"#,
    )
    .unwrap();
    assert_eq!(from_string.season, Season::new(2024));
    assert_eq!(from_string.season.to_api_string(), "20242025");

    let from_int: ClubStats = serde_json::from_str(
        r#"{"season": 20242025, "gameType": 2, "skaters": [], "goalies": []}"#,
    )
    .unwrap();
    assert_eq!(from_int.season, Season::new(2024));
}

#[test]
fn test_season_game_types_deserialization() {
    let json = r#"{
        "season": 20242025,
        "gameTypes": [2, 3]
    }"#;

    let season: SeasonGameTypes = serde_json::from_str(json).unwrap();
    assert_eq!(season.season, Season::new(2024));
    assert_eq!(
        season.game_types,
        vec![GameType::RegularSeason, GameType::Playoffs]
    );
}

/// The API's `club-stats-season` endpoint returns `season` as a plain
/// integer, but `Season`'s string forms must also deserialize here since
/// the type is shared across endpoints with differing shapes (1.1).
#[test]
fn test_season_game_types_season_deserializes_from_string_form() {
    let json = r#"{"season": "20242025", "gameTypes": [2]}"#;
    let season: SeasonGameTypes = serde_json::from_str(json).unwrap();
    assert_eq!(season.season, Season::new(2024));
}

#[test]
fn test_season_game_types_display() {
    let season = season_game_types(2024, vec![GameType::RegularSeason, GameType::Playoffs]);
    assert_eq!(format!("{}", season), "20242025: Regular Season, Playoffs");
}

#[test]
fn test_season_game_types_display_regular_only() {
    let season = season_game_types(2023, vec![GameType::RegularSeason]);
    assert_eq!(format!("{}", season), "20232024: Regular Season");
}

#[test]
fn test_skater_stats_display() {
    assert_eq!(
        format!("{}", sample_skater()),
        "David Savard - 75 GP, 1 G, 14 A, 15 PTS"
    );
}

#[test]
fn test_goalie_stats_display() {
    assert_eq!(
        format!("{}", sample_goalie()),
        "Sam Montembeault - 62 GP, 31-24-7, 2.818 GAA, 0.902 SV%"
    );
}

#[test]
fn test_goalie_record_with_ties_and_overtime_losses() {
    let goalie = ClubGoalieStats {
        ties: Some(5),
        ..sample_goalie()
    };
    assert_eq!(goalie.record(), "31-24-5-7");
}

#[test]
fn test_club_stats_empty_lists() {
    let json = r#"{
        "season": "20242025",
        "gameType": 2,
        "skaters": [],
        "goalies": []
    }"#;

    let stats: ClubStats = serde_json::from_str(json).unwrap();
    assert_eq!(stats, empty_club_stats());
}

#[test]
fn test_season_game_types_display_with_all_star() {
    let season = season_game_types(
        2024,
        vec![
            GameType::RegularSeason,
            GameType::Playoffs,
            GameType::AllStar,
        ],
    );
    assert_eq!(
        format!("{}", season),
        "20242025: Regular Season, Playoffs, All-Star"
    );
}

#[test]
fn test_season_game_types_display_preseason() {
    let season = season_game_types(2024, vec![GameType::Preseason]);
    assert_eq!(format!("{}", season), "20242025: Preseason");
}

#[test]
fn test_season_game_types_display_all_star() {
    let season = season_game_types(2023, vec![GameType::AllStar]);
    assert_eq!(format!("{}", season), "20232024: All-Star");
}

#[test]
fn test_season_game_types_display_mixed_order() {
    let season = season_game_types(
        2024,
        vec![
            GameType::Preseason,
            GameType::RegularSeason,
            GameType::AllStar,
            GameType::Playoffs,
        ],
    );
    assert_eq!(
        format!("{}", season),
        "20242025: Preseason, Regular Season, All-Star, Playoffs"
    );
}

#[test]
fn test_club_skater_stats_clone() {
    let stats = sample_skater();
    let cloned = stats.clone();
    assert_eq!(stats, cloned);
    assert_eq!(stats.player_id, cloned.player_id);
}

#[test]
fn test_club_skater_stats_debug() {
    let debug_str = format!("{:?}", sample_skater());
    assert!(debug_str.contains("ClubSkaterStats"));
    assert!(debug_str.contains("8475233"));
    assert!(debug_str.contains("David"));
}

#[test]
fn test_club_goalie_stats_clone() {
    let stats = sample_goalie();
    let cloned = stats.clone();
    assert_eq!(stats, cloned);
    assert_eq!(stats.player_id, cloned.player_id);
}

#[test]
fn test_club_goalie_stats_debug() {
    let debug_str = format!("{:?}", sample_goalie());
    assert!(debug_str.contains("ClubGoalieStats"));
    assert!(debug_str.contains("8478470"));
    assert!(debug_str.contains("Sam"));
}

#[test]
fn test_club_stats_clone() {
    let stats = empty_club_stats();
    let cloned = stats.clone();
    assert_eq!(stats, cloned);
}

#[test]
fn test_club_stats_debug() {
    let debug_str = format!("{:?}", empty_club_stats());
    assert!(debug_str.contains("ClubStats"));
    assert!(debug_str.contains("2024"));
    assert!(debug_str.contains("2025"));
}

#[test]
fn test_season_game_types_clone() {
    let season = season_game_types(2024, vec![GameType::RegularSeason, GameType::Playoffs]);
    let cloned = season.clone();
    assert_eq!(season, cloned);
}

#[test]
fn test_season_game_types_debug() {
    let season = season_game_types(2024, vec![GameType::RegularSeason]);
    let debug_str = format!("{:?}", season);
    assert!(debug_str.contains("SeasonGameTypes"));
    assert!(debug_str.contains("2024"));
    assert!(debug_str.contains("2025"));
}

#[test]
fn test_season_game_types_hash() {
    use std::collections::HashSet;

    let season1 = season_game_types(2024, vec![GameType::RegularSeason]);
    let season2 = season_game_types(2024, vec![GameType::RegularSeason]);
    let season3 = season_game_types(2023, vec![GameType::RegularSeason]);

    let mut set = HashSet::new();
    set.insert(season1);
    set.insert(season2); // Duplicate
    set.insert(season3);

    assert_eq!(set.len(), 2);
}

#[test]
fn test_season_game_types_serialization_roundtrip() {
    let season = season_game_types(2024, vec![GameType::RegularSeason, GameType::Playoffs]);
    let serialized = serde_json::to_string(&season).unwrap();
    assert_eq!(serialized, r#"{"season":20242025,"gameTypes":[2,3]}"#);
    let deserialized: SeasonGameTypes = serde_json::from_str(&serialized).unwrap();
    assert_eq!(season, deserialized);
}

#[test]
fn test_club_skater_stats_serialization_roundtrip() {
    let stats = sample_skater();
    let serialized = serde_json::to_string(&stats).unwrap();
    let deserialized: ClubSkaterStats = serde_json::from_str(&serialized).unwrap();
    assert_eq!(stats, deserialized);
}

/// Absent optional fields stay absent on serialization and read back as
/// `None`.
#[test]
fn test_club_skater_stats_serialization_omits_missing_fields() {
    let stats = ClubSkaterStats {
        plus_minus: None,
        avg_time_on_ice_per_game: None,
        ..sample_skater()
    };
    let value = serde_json::to_value(&stats).unwrap();
    assert!(value.get("plusMinus").is_none());
    assert!(value.get("avgTimeOnIcePerGame").is_none());
    assert_eq!(value["avgShiftsPerGame"], 19.84);

    let deserialized: ClubSkaterStats = serde_json::from_value(value).unwrap();
    assert_eq!(stats, deserialized);
}

#[test]
fn test_club_goalie_stats_serialization_roundtrip() {
    let stats = sample_goalie();
    let serialized = serde_json::to_string(&stats).unwrap();
    let deserialized: ClubGoalieStats = serde_json::from_str(&serialized).unwrap();
    assert_eq!(stats, deserialized);
}

#[test]
fn test_club_goalie_stats_serialization_omits_missing_fields() {
    let stats = ClubGoalieStats {
        ties: None,
        penalty_minutes: None,
        ..sample_goalie()
    };
    let value = serde_json::to_value(&stats).unwrap();
    assert!(value.get("ties").is_none());
    assert!(value.get("penaltyMinutes").is_none());
    assert_eq!(value["overtimeLosses"], 7);

    let deserialized: ClubGoalieStats = serde_json::from_value(value).unwrap();
    assert_eq!(stats, deserialized);
}

/// An unknown game type fails with the shared `UnknownEnumValue` message.
#[test]
fn test_season_game_types_unknown_game_type() {
    let json = r#"{
        "season": 20242025,
        "gameTypes": [2, 99]
    }"#;

    let err = serde_json::from_str::<SeasonGameTypes>(json).unwrap_err();
    assert!(
        err.to_string().contains(r#"invalid game type: "99""#),
        "{err}"
    );
}

#[test]
fn test_season_game_types_display_empty() {
    let season = season_game_types(2024, vec![]);
    assert_eq!(format!("{}", season), "20242025: ");
}

#[test]
fn test_skater_stats_equality() {
    let stats1 = sample_skater();
    let stats2 = stats1.clone();
    let mut stats3 = stats1.clone();
    stats3.goals = 10;

    assert_eq!(stats1, stats2);
    assert_ne!(stats1, stats3);
}

#[test]
fn test_goalie_stats_equality() {
    let stats1 = sample_goalie();
    let stats2 = stats1.clone();
    let mut stats3 = stats1.clone();
    stats3.wins = Some(40);

    assert_eq!(stats1, stats2);
    assert_ne!(stats1, stats3);
}

#[test]
fn test_club_stats_equality() {
    let stats1 = empty_club_stats();
    let stats2 = stats1.clone();
    let mut stats3 = stats1.clone();
    stats3.game_type = GameType::Playoffs;

    assert_eq!(stats1, stats2);
    assert_ne!(stats1, stats3);
}

#[test]
fn test_season_game_types_equality() {
    let season1 = season_game_types(2024, vec![GameType::RegularSeason]);
    let season2 = season_game_types(2024, vec![GameType::RegularSeason]);
    let season3 = season_game_types(2023, vec![GameType::RegularSeason]);

    assert_eq!(season1, season2);
    assert_ne!(season1, season3);
}

#[test]
fn test_club_stats_serialization_roundtrip() {
    let stats = empty_club_stats();
    let serialized = serde_json::to_string(&stats).unwrap();
    let deserialized: ClubStats = serde_json::from_str(&serialized).unwrap();
    assert_eq!(stats, deserialized);
}

#[test]
fn test_club_stats_with_all_game_types() {
    for (code, expected) in [
        (1, GameType::Preseason),
        (3, GameType::Playoffs),
        (4, GameType::AllStar),
    ] {
        let json = format!(
            r#"{{"season": "20242025", "gameType": {code}, "skaters": [], "goalies": []}}"#
        );
        let stats: ClubStats = serde_json::from_str(&json).unwrap();
        assert_eq!(stats.game_type, expected);
    }
}
