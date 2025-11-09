use serde::{Deserialize, Serialize};
use std::fmt;

use super::common::LocalizedString;

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
    pub position_code: String,
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
    pub game_type: i32,
    pub skaters: Vec<ClubSkaterStats>,
    pub goalies: Vec<ClubGoalieStats>,
}

/// Season game type availability for a team
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SeasonGameTypes {
    pub season: i32,
    #[serde(rename = "gameTypes")]
    pub game_types: Vec<i32>,
}

impl fmt::Display for SeasonGameTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let game_types_str: Vec<String> = self
            .game_types
            .iter()
            .map(|gt| match gt {
                2 => "Regular Season".to_string(),
                3 => "Playoffs".to_string(),
                _ => format!("Game Type {}", gt),
            })
            .collect();
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
        assert_eq!(stats.position_code, "D");
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
        assert_eq!(stats.game_type, 2);
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
        assert_eq!(season.game_types, vec![2, 3]);
    }

    #[test]
    fn test_season_game_types_display() {
        let season = SeasonGameTypes {
            season: 20242025,
            game_types: vec![2, 3],
        };
        assert_eq!(format!("{}", season), "20242025: Regular Season, Playoffs");
    }

    #[test]
    fn test_season_game_types_display_regular_only() {
        let season = SeasonGameTypes {
            season: 20232024,
            game_types: vec![2],
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
            position_code: "D".to_string(),
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
    fn test_season_game_types_display_with_unknown_type() {
        // Test the _ => format!("Game Type {}", gt) branch on line 142
        let season = SeasonGameTypes {
            season: 20242025,
            game_types: vec![2, 3, 4],
        };
        assert_eq!(
            format!("{}", season),
            "20242025: Regular Season, Playoffs, Game Type 4"
        );
    }

    #[test]
    fn test_season_game_types_display_only_unknown_types() {
        // Test with only unknown game types
        let season = SeasonGameTypes {
            season: 20232024,
            game_types: vec![1, 5],
        };
        assert_eq!(format!("{}", season), "20232024: Game Type 1, Game Type 5");
    }

    #[test]
    fn test_season_game_types_display_preseason() {
        // Test with game type 1 (typically preseason)
        let season = SeasonGameTypes {
            season: 20242025,
            game_types: vec![1],
        };
        assert_eq!(format!("{}", season), "20242025: Game Type 1");
    }

    #[test]
    fn test_season_game_types_display_all_star() {
        // Test with game type 4 (typically all-star game)
        let season = SeasonGameTypes {
            season: 20232024,
            game_types: vec![4],
        };
        assert_eq!(format!("{}", season), "20232024: Game Type 4");
    }

    #[test]
    fn test_season_game_types_display_mixed_order() {
        // Test with mixed known and unknown types in different order
        let season = SeasonGameTypes {
            season: 20242025,
            game_types: vec![1, 2, 4, 3],
        };
        assert_eq!(
            format!("{}", season),
            "20242025: Game Type 1, Regular Season, Game Type 4, Playoffs"
        );
    }
}
