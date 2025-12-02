use serde::{Deserialize, Serialize};
use std::fmt;

use super::common::LocalizedString;
use super::enums::Position;
use super::game_type::GameType;

/// Skater season statistics for a team
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ClubSkaterStats {
    #[serde(rename = "playerId")]
    pub player_id: i64,
    pub headshot: String,
    #[serde(rename = "firstName")]
    pub first_name: LocalizedString,
    #[serde(rename = "lastName")]
    pub last_name: LocalizedString,
    #[serde(rename = "positionCode")]
    pub position: Position,
    #[serde(rename = "gamesPlayed")]
    pub games_played: i32,
    pub goals: i32,
    pub assists: i32,
    pub points: i32,
    #[serde(rename = "plusMinus")]
    pub plus_minus: i32,
    #[serde(rename = "penaltyMinutes")]
    pub penalty_minutes: i32,
    #[serde(rename = "powerPlayGoals")]
    pub power_play_goals: i32,
    #[serde(rename = "shorthandedGoals")]
    pub shorthanded_goals: i32,
    #[serde(rename = "gameWinningGoals")]
    pub game_winning_goals: i32,
    #[serde(rename = "overtimeGoals")]
    pub overtime_goals: i32,
    pub shots: i32,
    #[serde(rename = "shootingPctg")]
    pub shooting_pctg: f64,
    #[serde(rename = "avgTimeOnIcePerGame")]
    pub avg_time_on_ice_per_game: f64,
    #[serde(rename = "avgShiftsPerGame")]
    pub avg_shifts_per_game: f64,
    #[serde(rename = "faceoffWinPctg")]
    pub faceoff_win_pctg: f64,
}

impl fmt::Display for ClubSkaterStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} - {} GP, {} G, {} A, {} PTS",
            self.first_name.default,
            self.last_name.default,
            self.games_played,
            self.goals,
            self.assists,
            self.points
        )
    }
}

/// Goalie season statistics for a team
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ClubGoalieStats {
    #[serde(rename = "playerId")]
    pub player_id: i64,
    pub headshot: String,
    #[serde(rename = "firstName")]
    pub first_name: LocalizedString,
    #[serde(rename = "lastName")]
    pub last_name: LocalizedString,
    #[serde(rename = "gamesPlayed")]
    pub games_played: i32,
    #[serde(rename = "gamesStarted")]
    pub games_started: i32,
    pub wins: i32,
    pub losses: i32,
    #[serde(rename = "overtimeLosses")]
    pub overtime_losses: i32,
    #[serde(rename = "goalsAgainstAverage")]
    pub goals_against_average: f64,
    #[serde(rename = "savePercentage")]
    pub save_percentage: f64,
    #[serde(rename = "shotsAgainst")]
    pub shots_against: i32,
    pub saves: i32,
    #[serde(rename = "goalsAgainst")]
    pub goals_against: i32,
    pub shutouts: i32,
    pub goals: i32,
    pub assists: i32,
    pub points: i32,
    #[serde(rename = "penaltyMinutes")]
    pub penalty_minutes: i32,
    #[serde(rename = "timeOnIce")]
    pub time_on_ice: i64,
}

impl fmt::Display for ClubGoalieStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} - {} GP, {}-{}-{}, {:.3} GAA, {:.3} SV%",
            self.first_name.default,
            self.last_name.default,
            self.games_played,
            self.wins,
            self.losses,
            self.overtime_losses,
            self.goals_against_average,
            self.save_percentage
        )
    }
}

/// Club statistics response containing skater and goalie stats
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ClubStats {
    #[serde(rename = "season")]
    pub season: String,
    #[serde(rename = "gameType")]
    pub game_type: GameType,
    pub skaters: Vec<ClubSkaterStats>,
    pub goalies: Vec<ClubGoalieStats>,
}

/// Season game type availability for a team
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SeasonGameTypes {
    pub season: i32,
    #[serde(rename = "gameTypes")]
    #[serde(with = "game_types_vec")]
    pub game_types: Vec<GameType>,
}

mod game_types_vec {
    use super::GameType;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(game_types: &[GameType], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq = serializer.serialize_seq(Some(game_types.len()))?;
        for gt in game_types {
            seq.serialize_element(&gt.to_int())?;
        }
        seq.end()
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<GameType>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let ints = Vec::<i32>::deserialize(deserializer)?;
        ints.into_iter()
            .map(|i| {
                GameType::from_int(i)
                    .ok_or_else(|| serde::de::Error::custom(format!("Unknown game type: {}", i)))
            })
            .collect()
    }
}

impl fmt::Display for SeasonGameTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let game_types_str: Vec<String> = self.game_types.iter().map(|gt| gt.to_string()).collect();
        write!(f, "{}: {}", self.season, game_types_str.join(", "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(stats.player_id, 8475233);
        assert_eq!(stats.first_name.default, "David");
        assert_eq!(stats.last_name.default, "Savard");
        assert_eq!(stats.position, Position::Defense);
        assert_eq!(stats.games_played, 75);
        assert_eq!(stats.goals, 1);
        assert_eq!(stats.assists, 14);
        assert_eq!(stats.points, 15);
        assert_eq!(stats.plus_minus, -8);
        assert_eq!(stats.shots, 48);
    }

    #[test]
    fn test_goalie_stats_deserialization() {
        let json = r#"{
            "playerId": 8478470,
            "headshot": "https://assets.nhle.com/mugs/nhl/20242025/MTL/8478470.png",
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
        assert_eq!(stats.player_id, 8478470);
        assert_eq!(stats.first_name.default, "Sam");
        assert_eq!(stats.last_name.default, "Montembeault");
        assert_eq!(stats.games_played, 62);
        assert_eq!(stats.wins, 31);
        assert_eq!(stats.losses, 24);
        assert_eq!(stats.overtime_losses, 7);
        assert_eq!(stats.shutouts, 4);
    }

    #[test]
    fn test_club_stats_deserialization() {
        let json = r#"{
            "season": "20242025",
            "gameType": 2,
            "skaters": [
                {
                    "playerId": 8475233,
                    "headshot": "https://assets.nhle.com/mugs/nhl/20242025/MTL/8475233.png",
                    "firstName": {"default": "David"},
                    "lastName": {"default": "Savard"},
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
                }
            ],
            "goalies": [
                {
                    "playerId": 8478470,
                    "headshot": "https://assets.nhle.com/mugs/nhl/20242025/MTL/8478470.png",
                    "firstName": {"default": "Sam"},
                    "lastName": {"default": "Montembeault"},
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
                }
            ]
        }"#;

        let stats: ClubStats = serde_json::from_str(json).unwrap();
        assert_eq!(stats.season, "20242025");
        assert_eq!(stats.game_type, GameType::RegularSeason);
        assert_eq!(stats.skaters.len(), 1);
        assert_eq!(stats.goalies.len(), 1);
    }

    #[test]
    fn test_season_game_types_deserialization() {
        let json = r#"{
            "season": 20242025,
            "gameTypes": [2, 3]
        }"#;

        let season: SeasonGameTypes = serde_json::from_str(json).unwrap();
        assert_eq!(season.season, 20242025);
        assert_eq!(
            season.game_types,
            vec![GameType::RegularSeason, GameType::Playoffs]
        );
    }

    #[test]
    fn test_season_game_types_display() {
        let season = SeasonGameTypes {
            season: 20242025,
            game_types: vec![GameType::RegularSeason, GameType::Playoffs],
        };
        assert_eq!(format!("{}", season), "20242025: Regular Season, Playoffs");
    }

    #[test]
    fn test_season_game_types_display_regular_only() {
        let season = SeasonGameTypes {
            season: 20232024,
            game_types: vec![GameType::RegularSeason],
        };
        assert_eq!(format!("{}", season), "20232024: Regular Season");
    }

    #[test]
    fn test_skater_stats_display() {
        let stats = ClubSkaterStats {
            player_id: 8475233,
            headshot: "test.png".to_string(),
            first_name: LocalizedString {
                default: "David".to_string(),
            },
            last_name: LocalizedString {
                default: "Savard".to_string(),
            },
            position: Position::Defense,
            games_played: 75,
            goals: 1,
            assists: 14,
            points: 15,
            plus_minus: -8,
            penalty_minutes: 36,
            power_play_goals: 0,
            shorthanded_goals: 0,
            game_winning_goals: 0,
            overtime_goals: 0,
            shots: 48,
            shooting_pctg: 0.020833,
            avg_time_on_ice_per_game: 995.36,
            avg_shifts_per_game: 19.84,
            faceoff_win_pctg: 0.0,
        };

        assert_eq!(
            format!("{}", stats),
            "David Savard - 75 GP, 1 G, 14 A, 15 PTS"
        );
    }

    #[test]
    fn test_goalie_stats_display() {
        let stats = ClubGoalieStats {
            player_id: 8478470,
            headshot: "test.png".to_string(),
            first_name: LocalizedString {
                default: "Sam".to_string(),
            },
            last_name: LocalizedString {
                default: "Montembeault".to_string(),
            },
            games_played: 62,
            games_started: 60,
            wins: 31,
            losses: 24,
            overtime_losses: 7,
            goals_against_average: 2.818349,
            save_percentage: 0.901669,
            shots_against: 1678,
            saves: 1513,
            goals_against: 166,
            shutouts: 4,
            goals: 0,
            assists: 1,
            points: 1,
            penalty_minutes: 0,
            time_on_ice: 212039,
        };

        assert_eq!(
            format!("{}", stats),
            "Sam Montembeault - 62 GP, 31-24-7, 2.818 GAA, 0.902 SV%"
        );
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
        assert_eq!(stats.skaters.len(), 0);
        assert_eq!(stats.goalies.len(), 0);
    }

    #[test]
    fn test_season_game_types_display_with_all_star() {
        let season = SeasonGameTypes {
            season: 20242025,
            game_types: vec![
                GameType::RegularSeason,
                GameType::Playoffs,
                GameType::AllStar,
            ],
        };
        assert_eq!(
            format!("{}", season),
            "20242025: Regular Season, Playoffs, All-Star"
        );
    }

    #[test]
    fn test_season_game_types_display_preseason() {
        let season = SeasonGameTypes {
            season: 20242025,
            game_types: vec![GameType::Preseason],
        };
        assert_eq!(format!("{}", season), "20242025: Preseason");
    }

    #[test]
    fn test_season_game_types_display_all_star() {
        let season = SeasonGameTypes {
            season: 20232024,
            game_types: vec![GameType::AllStar],
        };
        assert_eq!(format!("{}", season), "20232024: All-Star");
    }

    #[test]
    fn test_season_game_types_display_mixed_order() {
        let season = SeasonGameTypes {
            season: 20242025,
            game_types: vec![
                GameType::Preseason,
                GameType::RegularSeason,
                GameType::AllStar,
                GameType::Playoffs,
            ],
        };
        assert_eq!(
            format!("{}", season),
            "20242025: Preseason, Regular Season, All-Star, Playoffs"
        );
    }

    #[test]
    fn test_club_skater_stats_clone() {
        let stats = ClubSkaterStats {
            player_id: 8475233,
            headshot: "test.png".to_string(),
            first_name: LocalizedString {
                default: "David".to_string(),
            },
            last_name: LocalizedString {
                default: "Savard".to_string(),
            },
            position: Position::Defense,
            games_played: 75,
            goals: 1,
            assists: 14,
            points: 15,
            plus_minus: -8,
            penalty_minutes: 36,
            power_play_goals: 0,
            shorthanded_goals: 0,
            game_winning_goals: 0,
            overtime_goals: 0,
            shots: 48,
            shooting_pctg: 0.020833,
            avg_time_on_ice_per_game: 995.36,
            avg_shifts_per_game: 19.84,
            faceoff_win_pctg: 0.0,
        };

        let cloned = stats.clone();
        assert_eq!(stats, cloned);
        assert_eq!(stats.player_id, cloned.player_id);
    }

    #[test]
    fn test_club_skater_stats_debug() {
        let stats = ClubSkaterStats {
            player_id: 8475233,
            headshot: "test.png".to_string(),
            first_name: LocalizedString {
                default: "David".to_string(),
            },
            last_name: LocalizedString {
                default: "Savard".to_string(),
            },
            position: Position::Defense,
            games_played: 75,
            goals: 1,
            assists: 14,
            points: 15,
            plus_minus: -8,
            penalty_minutes: 36,
            power_play_goals: 0,
            shorthanded_goals: 0,
            game_winning_goals: 0,
            overtime_goals: 0,
            shots: 48,
            shooting_pctg: 0.020833,
            avg_time_on_ice_per_game: 995.36,
            avg_shifts_per_game: 19.84,
            faceoff_win_pctg: 0.0,
        };

        let debug_str = format!("{:?}", stats);
        assert!(debug_str.contains("ClubSkaterStats"));
        assert!(debug_str.contains("8475233"));
        assert!(debug_str.contains("David"));
    }

    #[test]
    fn test_club_goalie_stats_clone() {
        let stats = ClubGoalieStats {
            player_id: 8478470,
            headshot: "test.png".to_string(),
            first_name: LocalizedString {
                default: "Sam".to_string(),
            },
            last_name: LocalizedString {
                default: "Montembeault".to_string(),
            },
            games_played: 62,
            games_started: 60,
            wins: 31,
            losses: 24,
            overtime_losses: 7,
            goals_against_average: 2.818349,
            save_percentage: 0.901669,
            shots_against: 1678,
            saves: 1513,
            goals_against: 166,
            shutouts: 4,
            goals: 0,
            assists: 1,
            points: 1,
            penalty_minutes: 0,
            time_on_ice: 212039,
        };

        let cloned = stats.clone();
        assert_eq!(stats, cloned);
        assert_eq!(stats.player_id, cloned.player_id);
    }

    #[test]
    fn test_club_goalie_stats_debug() {
        let stats = ClubGoalieStats {
            player_id: 8478470,
            headshot: "test.png".to_string(),
            first_name: LocalizedString {
                default: "Sam".to_string(),
            },
            last_name: LocalizedString {
                default: "Montembeault".to_string(),
            },
            games_played: 62,
            games_started: 60,
            wins: 31,
            losses: 24,
            overtime_losses: 7,
            goals_against_average: 2.818349,
            save_percentage: 0.901669,
            shots_against: 1678,
            saves: 1513,
            goals_against: 166,
            shutouts: 4,
            goals: 0,
            assists: 1,
            points: 1,
            penalty_minutes: 0,
            time_on_ice: 212039,
        };

        let debug_str = format!("{:?}", stats);
        assert!(debug_str.contains("ClubGoalieStats"));
        assert!(debug_str.contains("8478470"));
        assert!(debug_str.contains("Sam"));
    }

    #[test]
    fn test_club_stats_clone() {
        let json = r#"{
            "season": "20242025",
            "gameType": 2,
            "skaters": [],
            "goalies": []
        }"#;

        let stats: ClubStats = serde_json::from_str(json).unwrap();
        let cloned = stats.clone();
        assert_eq!(stats, cloned);
    }

    #[test]
    fn test_club_stats_debug() {
        let json = r#"{
            "season": "20242025",
            "gameType": 2,
            "skaters": [],
            "goalies": []
        }"#;

        let stats: ClubStats = serde_json::from_str(json).unwrap();
        let debug_str = format!("{:?}", stats);
        assert!(debug_str.contains("ClubStats"));
        assert!(debug_str.contains("20242025"));
    }

    #[test]
    fn test_season_game_types_clone() {
        let season = SeasonGameTypes {
            season: 20242025,
            game_types: vec![GameType::RegularSeason, GameType::Playoffs],
        };

        let cloned = season.clone();
        assert_eq!(season, cloned);
    }

    #[test]
    fn test_season_game_types_debug() {
        let season = SeasonGameTypes {
            season: 20242025,
            game_types: vec![GameType::RegularSeason],
        };

        let debug_str = format!("{:?}", season);
        assert!(debug_str.contains("SeasonGameTypes"));
        assert!(debug_str.contains("20242025"));
    }

    #[test]
    fn test_season_game_types_hash() {
        use std::collections::HashSet;

        let season1 = SeasonGameTypes {
            season: 20242025,
            game_types: vec![GameType::RegularSeason],
        };
        let season2 = SeasonGameTypes {
            season: 20242025,
            game_types: vec![GameType::RegularSeason],
        };
        let season3 = SeasonGameTypes {
            season: 20232024,
            game_types: vec![GameType::RegularSeason],
        };

        let mut set = HashSet::new();
        set.insert(season1.clone());
        set.insert(season2); // Duplicate
        set.insert(season3);

        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_season_game_types_serialization_roundtrip() {
        let season = SeasonGameTypes {
            season: 20242025,
            game_types: vec![GameType::RegularSeason, GameType::Playoffs],
        };

        let serialized = serde_json::to_string(&season).unwrap();
        let deserialized: SeasonGameTypes = serde_json::from_str(&serialized).unwrap();
        assert_eq!(season, deserialized);
    }

    #[test]
    fn test_club_skater_stats_serialization_roundtrip() {
        let stats = ClubSkaterStats {
            player_id: 8475233,
            headshot: "test.png".to_string(),
            first_name: LocalizedString {
                default: "David".to_string(),
            },
            last_name: LocalizedString {
                default: "Savard".to_string(),
            },
            position: Position::Defense,
            games_played: 75,
            goals: 1,
            assists: 14,
            points: 15,
            plus_minus: -8,
            penalty_minutes: 36,
            power_play_goals: 0,
            shorthanded_goals: 0,
            game_winning_goals: 0,
            overtime_goals: 0,
            shots: 48,
            shooting_pctg: 0.020833,
            avg_time_on_ice_per_game: 995.36,
            avg_shifts_per_game: 19.84,
            faceoff_win_pctg: 0.0,
        };

        let serialized = serde_json::to_string(&stats).unwrap();
        let deserialized: ClubSkaterStats = serde_json::from_str(&serialized).unwrap();
        assert_eq!(stats, deserialized);
    }

    #[test]
    fn test_club_goalie_stats_serialization_roundtrip() {
        let stats = ClubGoalieStats {
            player_id: 8478470,
            headshot: "test.png".to_string(),
            first_name: LocalizedString {
                default: "Sam".to_string(),
            },
            last_name: LocalizedString {
                default: "Montembeault".to_string(),
            },
            games_played: 62,
            games_started: 60,
            wins: 31,
            losses: 24,
            overtime_losses: 7,
            goals_against_average: 2.818349,
            save_percentage: 0.901669,
            shots_against: 1678,
            saves: 1513,
            goals_against: 166,
            shutouts: 4,
            goals: 0,
            assists: 1,
            points: 1,
            penalty_minutes: 0,
            time_on_ice: 212039,
        };

        let serialized = serde_json::to_string(&stats).unwrap();
        let deserialized: ClubGoalieStats = serde_json::from_str(&serialized).unwrap();
        assert_eq!(stats, deserialized);
    }

    #[test]
    fn test_season_game_types_unknown_game_type() {
        // Test deserializing an unknown game type (should error)
        let json = r#"{
            "season": 20242025,
            "gameTypes": [2, 99]
        }"#;

        let result: Result<SeasonGameTypes, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_season_game_types_display_empty() {
        let season = SeasonGameTypes {
            season: 20242025,
            game_types: vec![],
        };
        assert_eq!(format!("{}", season), "20242025: ");
    }

    #[test]
    fn test_skater_stats_equality() {
        let stats1 = ClubSkaterStats {
            player_id: 8475233,
            headshot: "test.png".to_string(),
            first_name: LocalizedString {
                default: "David".to_string(),
            },
            last_name: LocalizedString {
                default: "Savard".to_string(),
            },
            position: Position::Defense,
            games_played: 75,
            goals: 1,
            assists: 14,
            points: 15,
            plus_minus: -8,
            penalty_minutes: 36,
            power_play_goals: 0,
            shorthanded_goals: 0,
            game_winning_goals: 0,
            overtime_goals: 0,
            shots: 48,
            shooting_pctg: 0.020833,
            avg_time_on_ice_per_game: 995.36,
            avg_shifts_per_game: 19.84,
            faceoff_win_pctg: 0.0,
        };

        let stats2 = stats1.clone();
        let mut stats3 = stats1.clone();
        stats3.goals = 10;

        assert_eq!(stats1, stats2);
        assert_ne!(stats1, stats3);
    }

    #[test]
    fn test_goalie_stats_equality() {
        let stats1 = ClubGoalieStats {
            player_id: 8478470,
            headshot: "test.png".to_string(),
            first_name: LocalizedString {
                default: "Sam".to_string(),
            },
            last_name: LocalizedString {
                default: "Montembeault".to_string(),
            },
            games_played: 62,
            games_started: 60,
            wins: 31,
            losses: 24,
            overtime_losses: 7,
            goals_against_average: 2.818349,
            save_percentage: 0.901669,
            shots_against: 1678,
            saves: 1513,
            goals_against: 166,
            shutouts: 4,
            goals: 0,
            assists: 1,
            points: 1,
            penalty_minutes: 0,
            time_on_ice: 212039,
        };

        let stats2 = stats1.clone();
        let mut stats3 = stats1.clone();
        stats3.wins = 40;

        assert_eq!(stats1, stats2);
        assert_ne!(stats1, stats3);
    }

    #[test]
    fn test_club_stats_equality() {
        let stats1 = ClubStats {
            season: "20242025".to_string(),
            game_type: GameType::RegularSeason,
            skaters: vec![],
            goalies: vec![],
        };

        let stats2 = stats1.clone();
        let mut stats3 = stats1.clone();
        stats3.game_type = GameType::Playoffs;

        assert_eq!(stats1, stats2);
        assert_ne!(stats1, stats3);
    }

    #[test]
    fn test_season_game_types_equality() {
        let season1 = SeasonGameTypes {
            season: 20242025,
            game_types: vec![GameType::RegularSeason],
        };

        let season2 = SeasonGameTypes {
            season: 20242025,
            game_types: vec![GameType::RegularSeason],
        };

        let season3 = SeasonGameTypes {
            season: 20232024,
            game_types: vec![GameType::RegularSeason],
        };

        assert_eq!(season1, season2);
        assert_ne!(season1, season3);
    }

    #[test]
    fn test_club_stats_serialization_roundtrip() {
        let stats = ClubStats {
            season: "20242025".to_string(),
            game_type: GameType::RegularSeason,
            skaters: vec![],
            goalies: vec![],
        };

        let serialized = serde_json::to_string(&stats).unwrap();
        let deserialized: ClubStats = serde_json::from_str(&serialized).unwrap();
        assert_eq!(stats, deserialized);
    }

    #[test]
    fn test_club_stats_with_all_game_types() {
        // Test Preseason game type
        let json = r#"{
            "season": "20242025",
            "gameType": 1,
            "skaters": [],
            "goalies": []
        }"#;
        let stats: ClubStats = serde_json::from_str(json).unwrap();
        assert_eq!(stats.game_type, GameType::Preseason);

        // Test Playoffs game type
        let json = r#"{
            "season": "20242025",
            "gameType": 3,
            "skaters": [],
            "goalies": []
        }"#;
        let stats: ClubStats = serde_json::from_str(json).unwrap();
        assert_eq!(stats.game_type, GameType::Playoffs);

        // Test AllStar game type
        let json = r#"{
            "season": "20242025",
            "gameType": 4,
            "skaters": [],
            "goalies": []
        }"#;
        let stats: ClubStats = serde_json::from_str(json).unwrap();
        assert_eq!(stats.game_type, GameType::AllStar);
    }
}
