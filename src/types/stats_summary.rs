//! Rows of the `summary` reports of the `stats/rest` skater, goalie and team
//! endpoints.
//!
//! The shape of a row depends on the query mode:
//!
//! - per season (the default): `season_id`, and for skaters and goalies
//!   `team_abbrevs`, a comma-separated list such as `"ANA,BUF"`
//! - [`aggregate`](crate::StatsQuery::aggregate) over several seasons: no
//!   `season_id` or `team_abbrevs`, and team rows carry `franchise_id` and
//!   `franchise_name` instead of `team_id` and `team_full_name`
//! - [`by_game`](crate::StatsQuery::by_game): no `season_id` or
//!   `team_abbrevs`; instead `game_id`, `game_date`, `home_road`,
//!   `opponent_team_abbrev` and, for skaters and goalies, `team_abbrev`
//!
//! Fields that depend on the mode or the era are `Option`s, `None` when the
//! API omits them (or sends `null`).

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::date::Season;
use crate::ids::{FranchiseId, GameId, PlayerId, TeamId};

use super::enums::{Handedness, HomeRoad, Position};

/// A skater's `summary` row.
///
/// The API sends `null` for what a season or row did not record: 1925-26 rows
/// have no plus-minus, strength splits, shots, shooting percentage, faceoff
/// percentage or time on ice, and 1975-76 rows no faceoff percentage or time
/// on ice. `shoots_catches` is `None` for some 1975-76 playoff skaters,
/// `shooting_pct` for a skater without a shot and `faceoff_win_pct` for one
/// who took no faceoffs. `time_on_ice_per_game` is in seconds.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkaterSummary {
    pub player_id: PlayerId,
    pub skater_full_name: String,
    pub last_name: String,
    #[serde(rename = "positionCode")]
    pub position: Position,
    pub games_played: i32,
    pub goals: i32,
    pub assists: i32,
    pub points: i32,
    pub penalty_minutes: i32,
    pub game_winning_goals: i32,
    pub ot_goals: i32,
    pub points_per_game: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shoots_catches: Option<Handedness>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub season_id: Option<Season>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub team_abbrevs: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub game_id: Option<GameId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub game_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub home_road: Option<HomeRoad>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub opponent_team_abbrev: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub team_abbrev: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plus_minus: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ev_goals: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ev_points: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pp_goals: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pp_points: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sh_goals: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sh_points: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shots: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shooting_pct: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub faceoff_win_pct: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_on_ice_per_game: Option<f64>,
}

impl fmt::Display for SkaterSummary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} - {} GP, {} G, {} A, {} PTS",
            self.skater_full_name, self.games_played, self.goals, self.assists, self.points
        )
    }
}

/// A goalie's `summary` row.
///
/// `ties` is `None` from 2005-06 on, `ot_losses` before overtime losses were
/// recorded (1997-98), and `save_pct` for a goalie who faced no shots.
/// `time_on_ice` is in seconds.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoalieSummary {
    pub player_id: PlayerId,
    pub goalie_full_name: String,
    pub last_name: String,
    pub shoots_catches: Handedness,
    pub games_played: i32,
    pub games_started: i32,
    pub wins: i32,
    pub losses: i32,
    pub goals_against: i32,
    pub shutouts: i32,
    pub goals: i32,
    pub assists: i32,
    pub points: i32,
    pub penalty_minutes: i32,
    pub time_on_ice: i32,
    pub goals_against_average: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub season_id: Option<Season>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub team_abbrevs: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub game_id: Option<GameId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub game_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub home_road: Option<HomeRoad>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub opponent_team_abbrev: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub team_abbrev: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ties: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ot_losses: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub saves: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shots_against: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub save_pct: Option<f64>,
}

impl fmt::Display for GoalieSummary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} - {} GP, {}-{}, {:.2} GAA",
            self.goalie_full_name,
            self.games_played,
            self.wins,
            self.losses,
            self.goals_against_average
        )?;
        if let Some(save_pct) = self.save_pct {
            write!(f, ", {save_pct:.3} SV%")?;
        }
        Ok(())
    }
}

/// A team's `summary` row.
///
/// `team_id`/`team_full_name` are replaced by `franchise_id`/`franchise_name`
/// in an aggregate query. `ties` is `None` in every season recorded since
/// 2005-06 and in playoffs; 1975-76 playoff rows also have no `ot_losses`,
/// faceoff percentage or special-teams percentages.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamSummary {
    pub games_played: i32,
    pub wins: i32,
    pub losses: i32,
    pub points: i32,
    pub goals_for: i32,
    pub goals_against: i32,
    pub regulation_and_ot_wins: i32,
    pub wins_in_regulation: i32,
    pub wins_in_shootout: i32,
    pub team_shutouts: i32,
    pub point_pct: f64,
    pub goals_for_per_game: f64,
    pub goals_against_per_game: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub team_id: Option<TeamId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub team_full_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub franchise_id: Option<FranchiseId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub franchise_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub season_id: Option<Season>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub game_id: Option<GameId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub game_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub home_road: Option<HomeRoad>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub opponent_team_abbrev: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ties: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ot_losses: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub faceoff_win_pct: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub penalty_kill_net_pct: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub penalty_kill_pct: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub power_play_net_pct: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub power_play_pct: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shots_against_per_game: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shots_for_per_game: Option<f64>,
}

impl TeamSummary {
    /// The team's name, or the franchise's in an aggregate row.
    pub fn name(&self) -> Option<&str> {
        self.team_full_name
            .as_deref()
            .or(self.franchise_name.as_deref())
    }
}

impl fmt::Display for TeamSummary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} - {} GP, {} W, {} L, {} PTS",
            self.name().unwrap_or("Unknown"),
            self.games_played,
            self.wins,
            self.losses,
            self.points
        )
    }
}

#[cfg(test)]
#[path = "stats_summary_test.rs"]
mod tests;
