//! Per-game log entries returned by `/player/{id}/game-log/{season}/{gameType}`.
//!
//! The endpoint returns a different entry shape depending on the player's
//! role: skaters carry `points`/`shots`/`shifts`/power-play counts, goalies
//! carry `decision`/`shotsAgainst`/`goalsAgainst`/`savePctg`/`gamesStarted`.
//! [`GameLog`] wraps both, mirroring `SkaterStats`/`GoalieStats` in the
//! boxscore types.

use crate::ids::GameId;
use crate::types::enums::{empty_string_as_none, GoalieDecision, HomeRoad};
use serde::{de, Deserialize, Deserializer, Serialize};

/// Field present on every goalie entry and on no skater entry; its presence
/// selects the [`GameLog::Goalie`] variant during deserialization.
const GOALIE_DISCRIMINATOR_FIELD: &str = "shotsAgainst";

/// Game log entry for a single game, either a skater's or a goalie's line.
///
/// Deserialization picks the variant from the entry's fields (goalie entries
/// carry `shotsAgainst`) rather than trying each variant in turn, so a
/// malformed entry still reports the precise missing or invalid field.
/// Serializes as the inner entry, without a variant tag.
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(untagged)]
pub enum GameLog {
    Skater(SkaterGameLog),
    Goalie(GoalieGameLog),
}

/// Skater (forward/defense) game log entry
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SkaterGameLog {
    pub game_id: GameId,
    pub game_date: String,
    pub team_abbrev: String,
    pub home_road_flag: HomeRoad,
    pub opponent_abbrev: String,
    pub goals: i32,
    pub assists: i32,
    pub points: i32,
    pub plus_minus: i32,
    pub power_play_goals: i32,
    pub power_play_points: i32,
    pub shots: i32,
    pub shifts: i32,
    pub toi: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_winning_goals: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ot_goals: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pim: Option<i32>,
}

/// Goalie game log entry
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GoalieGameLog {
    pub game_id: GameId,
    pub game_date: String,
    pub team_abbrev: String,
    pub home_road_flag: HomeRoad,
    pub opponent_abbrev: String,
    /// `1` if the goalie started the game, `0` for a relief appearance.
    pub games_started: i32,

    /// `None` when the goalie was not credited with the decision (e.g. a
    /// relief appearance, or a starter pulled before the deciding goal).
    #[serde(deserialize_with = "empty_string_as_none", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decision: Option<GoalieDecision>,

    pub shots_against: i32,
    pub goals_against: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub save_pctg: Option<f64>,

    pub shutouts: i32,
    pub goals: i32,
    pub assists: i32,
    pub toi: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pim: Option<i32>,
}

impl<'de> Deserialize<'de> for GameLog {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = serde_json::Value::deserialize(deserializer)?;
        let entry = if value.get(GOALIE_DISCRIMINATOR_FIELD).is_some() {
            serde_json::from_value(value).map(GameLog::Goalie)
        } else {
            serde_json::from_value(value).map(GameLog::Skater)
        };
        entry.map_err(de::Error::custom)
    }
}

impl GameLog {
    /// The skater entry, or `None` for a goalie entry
    pub fn as_skater(&self) -> Option<&SkaterGameLog> {
        match self {
            GameLog::Skater(entry) => Some(entry),
            GameLog::Goalie(_) => None,
        }
    }

    /// The goalie entry, or `None` for a skater entry
    pub fn as_goalie(&self) -> Option<&GoalieGameLog> {
        match self {
            GameLog::Skater(_) => None,
            GameLog::Goalie(entry) => Some(entry),
        }
    }

    pub fn game_id(&self) -> GameId {
        match self {
            GameLog::Skater(entry) => entry.game_id,
            GameLog::Goalie(entry) => entry.game_id,
        }
    }

    pub fn game_date(&self) -> &str {
        match self {
            GameLog::Skater(entry) => &entry.game_date,
            GameLog::Goalie(entry) => &entry.game_date,
        }
    }

    pub fn team_abbrev(&self) -> &str {
        match self {
            GameLog::Skater(entry) => &entry.team_abbrev,
            GameLog::Goalie(entry) => &entry.team_abbrev,
        }
    }

    pub fn opponent_abbrev(&self) -> &str {
        match self {
            GameLog::Skater(entry) => &entry.opponent_abbrev,
            GameLog::Goalie(entry) => &entry.opponent_abbrev,
        }
    }

    pub fn home_road_flag(&self) -> HomeRoad {
        match self {
            GameLog::Skater(entry) => entry.home_road_flag,
            GameLog::Goalie(entry) => entry.home_road_flag,
        }
    }

    /// Time on ice, `"MM:SS"`
    pub fn toi(&self) -> &str {
        match self {
            GameLog::Skater(entry) => &entry.toi,
            GameLog::Goalie(entry) => &entry.toi,
        }
    }
}

#[cfg(test)]
#[path = "game_log_test.rs"]
mod tests;
