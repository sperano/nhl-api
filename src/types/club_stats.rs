use serde::{Deserialize, Serialize};
use std::fmt;

use crate::date::Season;
use crate::ids::PlayerId;

use super::common::LocalizedString;
use super::enums::{empty_string_as_none, Position};
use super::game_type::GameType;

/// Skater season statistics for a team.
///
/// The `Option` fields are `None` when the API omits them, which depends on
/// the era: 1925-26 rows have no plus-minus, special-teams goals or shots,
/// 1980-81 rows no time on ice, shifts or faceoffs, and `shootingPctg` is
/// missing for a skater without a shot.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClubSkaterStats {
    pub player_id: PlayerId,
    pub headshot: String,
    pub first_name: LocalizedString,
    pub last_name: LocalizedString,
    /// `None` for historical data where the API returns an empty position code.
    #[serde(
        rename = "positionCode",
        deserialize_with = "empty_string_as_none",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<Position>,
    pub games_played: i32,
    pub goals: i32,
    pub assists: i32,
    pub points: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plus_minus: Option<i32>,
    pub penalty_minutes: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub power_play_goals: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shorthanded_goals: Option<i32>,
    pub game_winning_goals: i32,
    pub overtime_goals: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shots: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shooting_pctg: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avg_time_on_ice_per_game: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avg_shifts_per_game: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub faceoff_win_pctg: Option<f64>,
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

/// Goalie season statistics for a team.
///
/// The `Option` fields are `None` when the API omits them: `ties` and
/// `overtimeLosses` only come for seasons that had them, 1925-26 rows have no
/// shots or saves, a goalie never credited with a decision has no
/// `wins`/`losses`, and some rows (every 2024-25 Montreal playoff goalie) have
/// no `penaltyMinutes`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClubGoalieStats {
    pub player_id: PlayerId,
    pub headshot: String,
    pub first_name: LocalizedString,
    pub last_name: LocalizedString,
    pub games_played: i32,
    pub games_started: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wins: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub losses: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ties: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overtime_losses: Option<i32>,
    pub goals_against_average: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub save_percentage: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shots_against: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub saves: Option<i32>,
    pub goals_against: i32,
    pub shutouts: i32,
    pub goals: i32,
    pub assists: i32,
    pub points: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub penalty_minutes: Option<i32>,
    pub time_on_ice: i64,
}

impl ClubGoalieStats {
    /// The record as `W-L`, followed by `-T` and `-OTL` when the API sends
    /// them (`31-24-7` today, `20-4-3` in 1980-81). Missing wins or losses
    /// count as zero.
    pub fn record(&self) -> String {
        use fmt::Write as _;

        let mut record = format!(
            "{}-{}",
            self.wins.unwrap_or_default(),
            self.losses.unwrap_or_default()
        );
        for extra in [self.ties, self.overtime_losses].into_iter().flatten() {
            // Writing to a `String` cannot fail.
            let _ = write!(record, "-{extra}");
        }
        record
    }
}

impl fmt::Display for ClubGoalieStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} - {} GP, {}, {:.3} GAA",
            self.first_name.default,
            self.last_name.default,
            self.games_played,
            self.record(),
            self.goals_against_average
        )?;
        if let Some(save_percentage) = self.save_percentage {
            write!(f, ", {save_percentage:.3} SV%")?;
        }
        Ok(())
    }
}

/// Club statistics response containing skater and goalie stats
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClubStats {
    pub season: Season,
    pub game_type: GameType,
    pub skaters: Vec<ClubSkaterStats>,
    pub goalies: Vec<ClubGoalieStats>,
}

/// Season game type availability for a team
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct SeasonGameTypes {
    pub season: Season,
    pub game_types: Vec<GameType>,
}

impl fmt::Display for SeasonGameTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let game_types_str: Vec<String> = self.game_types.iter().map(|gt| gt.to_string()).collect();
        write!(
            f,
            "{}: {}",
            self.season.to_api_string(),
            game_types_str.join(", ")
        )
    }
}

#[cfg(test)]
#[path = "club_stats_test.rs"]
mod tests;
