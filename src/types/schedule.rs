use serde::{Deserialize, Serialize};
use std::fmt;

use crate::ids::{GameId, TeamId};

use super::common::LocalizedString;
use super::enums::GameScheduleState;
use super::game_state::GameState;
use super::game_type::GameType;

/// Schedule game information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScheduleGame {
    pub id: GameId,
    #[serde(rename = "gameType")]
    pub game_type: GameType,
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
    /// Whether the game is still on the schedule (`OK`) or was postponed,
    /// cancelled, etc. A cancelled game keeps `gameState: FUT`.
    #[serde(rename = "gameScheduleState")]
    pub game_schedule_state: GameScheduleState,
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
    pub id: TeamId,
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
    /// `None` for the newest week on record.
    #[serde(
        rename = "nextStartDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub next_start_date: Option<String>,
    /// `None` for the first week on record (December 1917).
    #[serde(
        rename = "previousStartDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub previous_start_date: Option<String>,
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
    pub id: GameId,
    #[serde(rename = "gameType")]
    pub game_type: GameType,
    #[serde(rename = "gameState")]
    pub game_state: GameState,
    /// See [`ScheduleGame::game_schedule_state`].
    #[serde(rename = "gameScheduleState")]
    pub game_schedule_state: GameScheduleState,
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
#[path = "schedule_test.rs"]
mod tests;
