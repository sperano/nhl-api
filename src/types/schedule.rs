use serde::{Deserialize, Serialize};
use std::fmt;

use super::common::LocalizedString;
use super::game_state::GameState;

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
        use std::borrow::Cow;

        let format_score = |score: Option<i32>| -> Cow<'static, str> {
            match score {
                Some(s) => Cow::Owned(s.to_string()),
                None => Cow::Borrowed("-"),
            }
        };

        write!(
            f,
            "{} {} @ {} {} [{}]",
            self.away_team.abbrev,
            format_score(self.away_team.score),
            self.home_team.abbrev,
            format_score(self.home_team.score),
            self.game_state
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Builder for creating test ScheduleTeam instances
    struct TeamBuilder {
        id: i64,
        abbrev: String,
        place_name: Option<LocalizedString>,
        logo: String,
        score: Option<i32>,
    }

    impl TeamBuilder {
        fn new(abbrev: &str) -> Self {
            Self {
                id: 1,
                abbrev: abbrev.to_string(),
                place_name: None,
                logo: format!("https://assets.nhle.com/logos/nhl/svg/{}_light.svg", abbrev),
                score: None,
            }
        }

        fn id(mut self, id: i64) -> Self {
            self.id = id;
            self
        }

        #[allow(dead_code)]
        fn score(mut self, score: i32) -> Self {
            self.score = Some(score);
            self
        }

        fn build(self) -> ScheduleTeam {
            ScheduleTeam {
                id: self.id,
                abbrev: self.abbrev,
                place_name: self.place_name,
                logo: self.logo,
                score: self.score,
            }
        }
    }

    /// Builder for creating test ScheduleGame instances
    struct ScheduleGameBuilder {
        id: i64,
        game_type: i32,
        game_date: Option<String>,
        start_time_utc: String,
        away_team: ScheduleTeam,
        home_team: ScheduleTeam,
        game_state: GameState,
    }

    impl ScheduleGameBuilder {
        fn new(away_abbrev: &str, home_abbrev: &str) -> Self {
            Self {
                id: 2023020001,
                game_type: 2,
                game_date: None,
                start_time_utc: "23:00:00Z".to_string(),
                away_team: TeamBuilder::new(away_abbrev).id(7).build(),
                home_team: TeamBuilder::new(home_abbrev).id(10).build(),
                game_state: GameState::Future,
            }
        }

        #[allow(dead_code)]
        fn id(mut self, id: i64) -> Self {
            self.id = id;
            self
        }

        fn game_date(mut self, date: &str) -> Self {
            self.game_date = Some(date.to_string());
            self
        }

        #[allow(dead_code)]
        fn game_state(mut self, state: GameState) -> Self {
            self.game_state = state;
            self
        }

        #[allow(dead_code)]
        fn away_score(mut self, score: i32) -> Self {
            self.away_team.score = Some(score);
            self
        }

        #[allow(dead_code)]
        fn home_score(mut self, score: i32) -> Self {
            self.home_team.score = Some(score);
            self
        }

        fn build(self) -> ScheduleGame {
            ScheduleGame {
                id: self.id,
                game_type: self.game_type,
                game_date: self.game_date,
                start_time_utc: self.start_time_utc,
                away_team: self.away_team,
                home_team: self.home_team,
                game_state: self.game_state,
            }
        }
    }

    /// Builder for creating test GameScore instances
    struct GameScoreBuilder {
        id: i64,
        game_type: i32,
        game_state: GameState,
        away_team: ScheduleTeam,
        home_team: ScheduleTeam,
    }

    impl GameScoreBuilder {
        fn new(away_abbrev: &str, home_abbrev: &str) -> Self {
            Self {
                id: 2023020001,
                game_type: 2,
                game_state: GameState::Future,
                away_team: TeamBuilder::new(away_abbrev).id(7).build(),
                home_team: TeamBuilder::new(home_abbrev).id(10).build(),
            }
        }

        #[allow(dead_code)]
        fn id(mut self, id: i64) -> Self {
            self.id = id;
            self
        }

        fn game_state(mut self, state: GameState) -> Self {
            self.game_state = state;
            self
        }

        fn away_score(mut self, score: i32) -> Self {
            self.away_team.score = Some(score);
            self
        }

        fn home_score(mut self, score: i32) -> Self {
            self.home_team.score = Some(score);
            self
        }

        fn build(self) -> GameScore {
            GameScore {
                id: self.id,
                game_type: self.game_type,
                game_state: self.game_state,
                away_team: self.away_team,
                home_team: self.home_team,
            }
        }
    }

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
        let game = ScheduleGameBuilder::new("BUF", "TOR")
            .game_date("2023-10-10")
            .build();

        assert_eq!(game.to_string(), "BUF @ TOR on 2023-10-10 [FUT]");
    }

    #[test]
    fn test_schedule_game_display_without_date() {
        let game = ScheduleGameBuilder::new("BUF", "TOR").build();

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
        let game = GameScoreBuilder::new("BUF", "TOR")
            .away_score(3)
            .home_score(2)
            .game_state(GameState::Final)
            .build();

        assert_eq!(game.to_string(), "BUF 3 @ TOR 2 [FINAL]");
    }

    #[test]
    fn test_game_score_display_with_no_scores() {
        // Test the None => Cow::Borrowed("-") branch
        let game = GameScoreBuilder::new("BUF", "TOR").build();

        assert_eq!(game.to_string(), "BUF - @ TOR - [FUT]");
    }

    #[test]
    fn test_game_score_display_with_partial_score() {
        // Test mixed Some/None scores (one team has score, other doesn't)
        let game = GameScoreBuilder::new("BUF", "TOR")
            .away_score(1)
            .game_state(GameState::Live)
            .build();

        assert_eq!(game.to_string(), "BUF 1 @ TOR - [LIVE]");
    }

    #[test]
    fn test_game_score_display_with_zero_scores() {
        // Test that zero scores display as "0" not "-"
        let game = GameScoreBuilder::new("BUF", "TOR")
            .away_score(0)
            .home_score(0)
            .game_state(GameState::Live)
            .build();

        assert_eq!(game.to_string(), "BUF 0 @ TOR 0 [LIVE]");
    }
}
