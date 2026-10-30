use serde::{Deserialize, Serialize};

use crate::date::Season;
use crate::ids::{GameId, PlayerId, TeamId};

use super::super::boxscore::PeriodDescriptor;
use super::super::common::LocalizedString;
use super::super::enums::GameScheduleState;
use super::super::game_state::GameState;
use super::super::game_type::GameType;
use super::GameOutcome;

/// Season series matchup
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SeasonSeriesMatchup {
    #[serde(rename = "seasonSeries")]
    pub season_series: Vec<SeriesGame>,
    #[serde(rename = "seasonSeriesWins")]
    pub season_series_wins: SeriesWins,
    #[serde(rename = "gameInfo")]
    pub game_info: SeriesGameInfo,
}

/// Individual game in the season series
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SeriesGame {
    pub id: GameId,
    pub season: Season,
    #[serde(rename = "gameType")]
    pub game_type: GameType,
    #[serde(rename = "gameDate")]
    pub game_date: String,
    #[serde(rename = "startTimeUTC")]
    pub start_time_utc: String,
    #[serde(rename = "easternUTCOffset")]
    pub eastern_utc_offset: String,
    #[serde(rename = "venueUTCOffset")]
    pub venue_utc_offset: String,
    #[serde(rename = "gameState")]
    pub game_state: GameState,
    #[serde(rename = "gameScheduleState")]
    pub game_schedule_state: GameScheduleState,
    #[serde(rename = "awayTeam")]
    pub away_team: SeriesTeam,
    #[serde(rename = "homeTeam")]
    pub home_team: SeriesTeam,
    /// All defaults for a game not played yet, when the API omits it.
    #[serde(rename = "periodDescriptor", default)]
    pub period_descriptor: PeriodDescriptor,
    #[serde(rename = "gameCenterLink")]
    pub game_center_link: String,
    #[serde(rename = "gameOutcome")]
    pub game_outcome: GameOutcome,
}

/// Team information in season series
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SeriesTeam {
    pub id: TeamId,
    pub abbrev: String,
    pub logo: String,
    /// `None` before puck drop, when the API omits it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score: Option<i32>,
}

/// Season series win counts
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SeriesWins {
    #[serde(rename = "awayTeamWins")]
    pub away_team_wins: i32,
    #[serde(rename = "homeTeamWins")]
    pub home_team_wins: i32,
}

/// Game information including officials and scratches
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SeriesGameInfo {
    #[serde(default)]
    pub referees: Vec<GameOfficial>,
    #[serde(default)]
    pub linesmen: Vec<GameOfficial>,
    #[serde(rename = "awayTeam")]
    pub away_team: TeamGameInfo,
    #[serde(rename = "homeTeam")]
    pub home_team: TeamGameInfo,
}

/// On-ice official (referee or linesman) listed in a season-series game.
///
/// The right-rail endpoint describes each official as an object with a
/// localized `fullName` and a `sweaterNumber`, not as a bare localized
/// string.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GameOfficial {
    #[serde(rename = "fullName")]
    pub full_name: LocalizedString,
    /// `None` for officials in older games (2000-01 and earlier).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "sweaterNumber")]
    pub sweater_number: Option<i32>,
}

/// Team-specific game information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TeamGameInfo {
    #[serde(rename = "headCoach")]
    pub head_coach: LocalizedString,
    #[serde(default)]
    pub scratches: Vec<ScratchedPlayer>,
}

/// Scratched player information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScratchedPlayer {
    pub id: PlayerId,
    #[serde(rename = "firstName")]
    pub first_name: LocalizedString,
    #[serde(rename = "lastName")]
    pub last_name: LocalizedString,
}
