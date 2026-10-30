use serde::{Deserialize, Serialize};

use crate::date::Season;
use crate::ids::{GameId, TeamId};

use super::super::boxscore::TvBroadcast;
use super::super::common::LocalizedString;
use super::super::enums::GameScheduleState;
use super::super::game_state::GameState;
use super::super::game_type::GameType;
use super::GameSummary;

/// Game story
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GameStory {
    pub id: GameId,
    pub season: Season,
    #[serde(rename = "gameType")]
    pub game_type: GameType,
    #[serde(rename = "limitedScoring")]
    pub limited_scoring: bool,
    #[serde(rename = "gameDate")]
    pub game_date: String,
    pub venue: LocalizedString,
    #[serde(rename = "venueLocation")]
    pub venue_location: LocalizedString,
    #[serde(rename = "startTimeUTC")]
    pub start_time_utc: String,
    #[serde(rename = "easternUTCOffset")]
    pub eastern_utc_offset: String,
    #[serde(rename = "venueUTCOffset")]
    pub venue_utc_offset: String,
    #[serde(rename = "venueTimezone")]
    pub venue_timezone: String,
    #[serde(rename = "tvBroadcasts", default)]
    pub tv_broadcasts: Vec<TvBroadcast>,
    #[serde(rename = "gameState")]
    pub game_state: GameState,
    #[serde(rename = "gameScheduleState")]
    pub game_schedule_state: GameScheduleState,
    #[serde(rename = "awayTeam")]
    pub away_team: StoryTeam,
    #[serde(rename = "homeTeam")]
    pub home_team: StoryTeam,
    #[serde(rename = "shootoutInUse")]
    pub shootout_in_use: bool,
    /// `None` for playoff games, whose overtime has no period limit.
    #[serde(rename = "maxPeriods", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_periods: Option<i32>,
    #[serde(rename = "regPeriods", default)]
    pub reg_periods: i32,
    #[serde(rename = "otInUse")]
    pub ot_in_use: bool,
    #[serde(rename = "tiesInUse")]
    pub ties_in_use: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<GameSummary>,
}

/// Team information in game story
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StoryTeam {
    pub id: TeamId,
    pub name: LocalizedString,
    pub abbrev: String,
    #[serde(rename = "placeName")]
    pub place_name: LocalizedString,
    /// `None` before puck drop, when the API omits it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score: Option<i32>,
    /// `None` before puck drop, when the API omits it, and for early games
    /// without shot counts (e.g. 1925-26).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sog: Option<i32>,
    pub logo: String,
}
