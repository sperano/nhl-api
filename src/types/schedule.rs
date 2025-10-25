use serde::{Deserialize, Serialize};
use std::fmt;

use super::common::LocalizedString;
use crate::game_state::GameState;

/// Schedule game information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScheduleGame {
    pub id: i64,
    #[serde(rename = "gameType")]
    pub game_type: i32,
    #[serde(rename = "gameDate", skip_serializing_if = "Option::is_none")]
    pub game_date: Option<String>,
    #[serde(rename = "startTimeUTC")]
    pub start_time_utc: String,
    #[serde(rename = "awayTeam")]
    pub away_team: ScheduleTeam,
    #[serde(rename = "homeTeam")]
    pub home_team: ScheduleTeam,
    #[serde(rename = "gameState")]
    pub game_state: GameState,
}

impl fmt::Display for ScheduleGame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref date) = self.game_date {
            write!(
                f,
                "{} @ {} on {} [{}]",
                self.away_team.abbrev, self.home_team.abbrev, date, self.game_state
            )
        } else {
            write!(
                f,
                "{} @ {} [{}]",
                self.away_team.abbrev, self.home_team.abbrev, self.game_state
            )
        }
    }
}

/// Team information in schedule
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScheduleTeam {
    pub id: i64,
    pub abbrev: String,
    #[serde(rename = "placeName")]
    pub place_name: Option<LocalizedString>,
    pub logo: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<i32>,
}

/// Daily schedule response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DailySchedule {
    #[serde(rename = "nextStartDate")]
    pub next_start_date: Option<String>,
    #[serde(rename = "previousStartDate")]
    pub previous_start_date: Option<String>,
    pub date: String,
    #[serde(default)]
    pub games: Vec<ScheduleGame>,
    #[serde(rename = "numberOfGames", default)]
    pub number_of_games: usize,
}

/// Weekly schedule response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WeeklyScheduleResponse {
    #[serde(rename = "nextStartDate")]
    pub next_start_date: String,
    #[serde(rename = "previousStartDate")]
    pub previous_start_date: String,
    #[serde(rename = "gameWeek")]
    pub game_week: Vec<GameDay>,
}

/// A day of games
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GameDay {
    pub date: String,
    pub games: Vec<ScheduleGame>,
}

/// Team schedule response (monthly/weekly)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TeamScheduleResponse {
    pub games: Vec<ScheduleGame>,
}

/// Game scores for a day
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DailyScores {
    #[serde(rename = "prevDate")]
    pub prev_date: String,
    #[serde(rename = "currentDate")]
    pub current_date: String,
    #[serde(rename = "nextDate")]
    pub next_date: String,
    #[serde(default)]
    pub games: Vec<GameScore>,
}

/// Individual game score
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GameScore {
    pub id: i64,
    #[serde(rename = "gameType")]
    pub game_type: i32,
    #[serde(rename = "gameState")]
    pub game_state: GameState,
    #[serde(rename = "awayTeam")]
    pub away_team: ScheduleTeam,
    #[serde(rename = "homeTeam")]
    pub home_team: ScheduleTeam,
}

impl fmt::Display for GameScore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let away_score = self.away_team.score.map(|s| s.to_string()).unwrap_or_else(|| "-".to_string());
        let home_score = self.home_team.score.map(|s| s.to_string()).unwrap_or_else(|| "-".to_string());
        write!(
            f,
            "{} {} @ {} {} [{}]",
            self.away_team.abbrev, away_score, self.home_team.abbrev, home_score, self.game_state
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_daily_schedule_with_no_games() {
        let schedule = DailySchedule {
            next_start_date: Some("2024-10-20".to_string()),
            previous_start_date: Some("2024-10-18".to_string()),
            date: "2024-10-19".to_string(),
            games: vec![],
            number_of_games: 0,
        };

        assert_eq!(schedule.games.len(), 0);
        assert_eq!(schedule.number_of_games, 0);
    }

    #[test]
    fn test_daily_scores_deserialization() {
        let json = r#"{
            "prevDate": "2024-10-18",
            "currentDate": "2024-10-19",
            "nextDate": "2024-10-20",
            "games": []
        }"#;

        let scores: DailyScores = serde_json::from_str(json).unwrap();
        assert_eq!(scores.current_date, "2024-10-19");
        assert_eq!(scores.games.len(), 0);
    }

    #[test]
    fn test_schedule_game_display() {
        let game = ScheduleGame {
            id: 2023020001,
            game_type: 2,
            game_date: Some("2023-10-10".to_string()),
            start_time_utc: "23:00:00Z".to_string(),
            away_team: ScheduleTeam {
                id: 7,
                abbrev: "BUF".to_string(),
                place_name: None,
                logo: "https://assets.nhle.com/logos/nhl/svg/BUF_light.svg".to_string(),
                score: None,
            },
            home_team: ScheduleTeam {
                id: 10,
                abbrev: "TOR".to_string(),
                place_name: None,
                logo: "https://assets.nhle.com/logos/nhl/svg/TOR_light.svg".to_string(),
                score: None,
            },
            game_state: "FUT".to_string(),
        };

        assert_eq!(game.to_string(), "BUF @ TOR on 2023-10-10 [FUT]");
    }

    #[test]
    fn test_schedule_game_display_without_date() {
        let game = ScheduleGame {
            id: 2023020001,
            game_type: 2,
            game_date: None,
            start_time_utc: "23:00:00Z".to_string(),
            away_team: ScheduleTeam {
                id: 7,
                abbrev: "BUF".to_string(),
                place_name: None,
                logo: "https://assets.nhle.com/logos/nhl/svg/BUF_light.svg".to_string(),
                score: None,
            },
            home_team: ScheduleTeam {
                id: 10,
                abbrev: "TOR".to_string(),
                place_name: None,
                logo: "https://assets.nhle.com/logos/nhl/svg/TOR_light.svg".to_string(),
                score: None,
            },
            game_state: "FUT".to_string(),
        };

        assert_eq!(game.to_string(), "BUF @ TOR [FUT]");
    }

    #[test]
    fn test_schedule_game_deserialization_without_game_date() {
        let json = r#"{
            "id": 2024020001,
            "gameType": 2,
            "startTimeUTC": "23:00:00Z",
            "awayTeam": {
                "id": 7,
                "abbrev": "BUF",
                "logo": "https://assets.nhle.com/logos/nhl/svg/BUF_light.svg"
            },
            "homeTeam": {
                "id": 10,
                "abbrev": "TOR",
                "logo": "https://assets.nhle.com/logos/nhl/svg/TOR_light.svg"
            },
            "gameState": "FUT"
        }"#;

        let game: ScheduleGame = serde_json::from_str(json).unwrap();
        assert_eq!(game.id, 2024020001);
        assert_eq!(game.game_date, None);
        assert_eq!(game.away_team.abbrev, "BUF");
        assert_eq!(game.home_team.abbrev, "TOR");
    }

    #[test]
    fn test_game_score_display() {
        let game = GameScore {
            id: 2023020001,
            game_type: 2,
            game_state: "FINAL".to_string(),
            away_team: ScheduleTeam {
                id: 7,
                abbrev: "BUF".to_string(),
                place_name: None,
                logo: "https://assets.nhle.com/logos/nhl/svg/BUF_light.svg".to_string(),
                score: Some(3),
            },
            home_team: ScheduleTeam {
                id: 10,
                abbrev: "TOR".to_string(),
                place_name: None,
                logo: "https://assets.nhle.com/logos/nhl/svg/TOR_light.svg".to_string(),
                score: Some(2),
            },
        };

        assert_eq!(game.to_string(), "BUF 3 @ TOR 2 [FINAL]");
    }
}
