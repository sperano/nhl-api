use crate::date::Season;
use crate::ids::{PlayerId, TeamId};
use crate::types::common::LocalizedString;
use crate::types::enums::{empty_string_as_none, Handedness, Position};
use crate::types::game_log::GameLog;
use crate::types::game_type::GameType;
use serde::{Deserialize, Serialize};

/// Player landing page data - comprehensive player profile
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PlayerLanding {
    pub player_id: PlayerId,
    pub is_active: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_team_id: Option<TeamId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_team_abbrev: Option<String>,

    pub first_name: LocalizedString,
    pub last_name: LocalizedString,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sweater_number: Option<i32>,

    /// `None` when the API returns an empty position code.
    #[serde(deserialize_with = "empty_string_as_none", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<Position>,
    pub headshot: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hero_image: Option<String>,

    pub height_in_inches: i32,
    pub weight_in_pounds: i32,
    pub birth_date: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_city: Option<LocalizedString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_state_province: Option<LocalizedString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_country: Option<String>,

    /// `None` for players with missing handedness data from the API.
    #[serde(deserialize_with = "empty_string_as_none", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shoots_catches: Option<Handedness>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub draft_details: Option<DraftDetails>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub player_slug: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub featured_stats: Option<FeaturedStats>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub career_totals: Option<CareerTotals>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub season_totals: Option<Vec<SeasonTotal>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub awards: Option<Vec<Award>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_five_games: Option<Vec<GameLog>>,
}

/// Draft details for a player
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DraftDetails {
    pub year: i32,
    pub team_abbrev: String,
    pub round: i32,
    pub pick_in_round: i32,
    pub overall_pick: i32,
}

/// Featured stats shown prominently on player page
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FeaturedStats {
    pub season: Season,
    pub regular_season: PlayerStats,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub playoffs: Option<PlayerStats>,
}

/// Career totals for regular season and playoffs
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CareerTotals {
    pub regular_season: PlayerStats,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub playoffs: Option<PlayerStats>,
}

/// Player statistics (skater or goalie)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PlayerStats {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub games_played: Option<i32>,

    // Skater stats
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goals: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub assists: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub points: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub plus_minus: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pim: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub power_play_goals: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub power_play_points: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_handed_goals: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_handed_points: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub shots: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub shooting_pctg: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub faceoff_win_pctg: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_toi: Option<String>,

    // Goalie stats
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wins: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub losses: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ot_losses: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub shutouts: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub goals_against_avg: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub save_pctg: Option<f64>,
}

/// Season-by-season statistics
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SeasonTotal {
    pub season: Season,
    #[serde(rename = "gameTypeId")]
    pub game_type: GameType,
    pub league_abbrev: String,
    pub team_name: LocalizedString,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_common_name: Option<LocalizedString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence: Option<i32>,

    /// `None` on rows that carry only the team and league, with no stats
    /// (some minor-league and tournament seasons); `Some(0)` when the player
    /// was listed but did not play.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub games_played: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub goals: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub assists: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub points: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub plus_minus: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pim: Option<i32>,
}

/// Award won by player
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Award {
    pub trophy: LocalizedString,
    pub seasons: Vec<AwardSeason>,
}

/// Season when award was won
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AwardSeason {
    pub season_id: Season,
}

/// Player game log response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PlayerGameLog {
    /// The player ID is not in the API response, we track it ourselves
    #[serde(skip)]
    pub player_id: PlayerId,

    #[serde(rename = "seasonId")]
    pub season: Season,

    #[serde(rename = "gameTypeId")]
    pub game_type: GameType,

    pub game_log: Vec<GameLog>,
}

/// Player search result
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PlayerSearchResult {
    #[serde(rename = "playerId")]
    pub player_id: PlayerId,
    pub name: String,
    /// `None` when the API returns an empty position code.
    #[serde(
        rename = "positionCode",
        deserialize_with = "empty_string_as_none",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<Position>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<TeamId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_abbrev: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sweater_number: Option<i32>,

    pub active: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_city: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_state_province: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_country: Option<String>,
}

#[cfg(test)]
#[path = "player_test.rs"]
mod tests;
