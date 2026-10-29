use serde::{Deserialize, Deserializer, Serialize};

use crate::date::Season;
use crate::ids::{GameId, PlayerId, TeamId};

use super::super::boxscore::{GameClock, PeriodDescriptor, SpecialEvent, TvBroadcast};
use super::super::common::LocalizedString;
use super::super::enums::{empty_string_as_none, DefendingSide, GameScheduleState, Position};
use super::super::game_state::GameState;
use super::super::game_type::GameType;

/// Game matchup/landing response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GameMatchup {
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
    /// All defaults before puck drop, when the API omits it.
    #[serde(rename = "periodDescriptor", default)]
    pub period_descriptor: PeriodDescriptor,
    #[serde(rename = "tvBroadcasts", default)]
    pub tv_broadcasts: Vec<TvBroadcast>,
    #[serde(rename = "gameState")]
    pub game_state: GameState,
    #[serde(rename = "gameScheduleState")]
    pub game_schedule_state: GameScheduleState,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "specialEvent")]
    pub special_event: Option<SpecialEvent>,
    #[serde(rename = "awayTeam")]
    pub away_team: MatchupTeam,
    #[serde(rename = "homeTeam")]
    pub home_team: MatchupTeam,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clock: Option<GameClock>,
}

/// Team information in game matchup
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MatchupTeam {
    pub id: TeamId,
    #[serde(rename = "commonName")]
    pub common_name: LocalizedString,
    pub abbrev: String,
    #[serde(rename = "placeName")]
    pub place_name: LocalizedString,
    #[serde(rename = "placeNameWithPreposition")]
    pub place_name_with_preposition: LocalizedString,
    /// `None` before puck drop, when the API omits it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score: Option<i32>,
    /// `None` before puck drop, when the API omits it, and for early games
    /// without shot counts (e.g. 1925-26).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sog: Option<i32>,
    pub logo: String,
    #[serde(rename = "darkLogo")]
    pub dark_logo: String,
}

/// Game summary with scoring and penalties
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GameSummary {
    #[serde(default)]
    pub scoring: Vec<PeriodScoring>,
    #[serde(default)]
    pub shootout: Vec<ShootoutAttempt>,
    #[serde(rename = "threeStars", default)]
    pub three_stars: Vec<ThreeStar>,
    #[serde(default)]
    pub penalties: Vec<PeriodPenalties>,
}

/// Scoring summary for a period
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PeriodScoring {
    #[serde(rename = "periodDescriptor")]
    pub period_descriptor: PeriodDescriptor,
    pub goals: Vec<GoalSummary>,
}

/// Goal summary information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GoalSummary {
    /// `None` for goals in older games (2008-09 and earlier).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "situationCode")]
    pub situation_code: Option<String>,
    #[serde(rename = "eventId")]
    pub event_id: i64,
    pub strength: String,
    #[serde(rename = "playerId")]
    pub player_id: PlayerId,
    #[serde(rename = "firstName")]
    pub first_name: LocalizedString,
    #[serde(rename = "lastName")]
    pub last_name: LocalizedString,
    pub name: LocalizedString,
    #[serde(rename = "teamAbbrev")]
    pub team_abbrev: LocalizedString,
    pub headshot: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "highlightClipSharingUrl")]
    pub highlight_clip_sharing_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "highlightClip")]
    pub highlight_clip: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "discreteClip")]
    pub discrete_clip: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "goalsToDate")]
    pub goals_to_date: Option<i32>,
    #[serde(rename = "awayScore")]
    pub away_score: i32,
    #[serde(rename = "homeScore")]
    pub home_score: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "leadingTeamAbbrev")]
    pub leading_team_abbrev: Option<LocalizedString>,
    #[serde(rename = "timeInPeriod")]
    pub time_in_period: String,
    /// `None` for goals in the 1990s and earlier.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "shotType")]
    pub shot_type: Option<String>,
    #[serde(rename = "goalModifier")]
    pub goal_modifier: String,
    #[serde(default)]
    pub assists: Vec<AssistSummary>,
    /// `None` for historical games that lack defending-side data.
    #[serde(
        rename = "homeTeamDefendingSide",
        deserialize_with = "empty_string_as_none",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_team_defending_side: Option<DefendingSide>,
    #[serde(rename = "isHome")]
    pub is_home: bool,
}

/// Assist summary information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AssistSummary {
    #[serde(rename = "playerId")]
    pub player_id: PlayerId,
    #[serde(rename = "firstName")]
    pub first_name: LocalizedString,
    #[serde(rename = "lastName")]
    pub last_name: LocalizedString,
    pub name: LocalizedString,
    #[serde(rename = "assistsToDate")]
    pub assists_to_date: i32,
    #[serde(rename = "sweaterNumber")]
    pub sweater_number: i32,
}

/// Shootout attempt information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ShootoutAttempt {
    pub sequence: i32,
    #[serde(rename = "playerId")]
    pub player_id: PlayerId,
    #[serde(rename = "teamAbbrev")]
    pub team_abbrev: LocalizedString,
    #[serde(rename = "firstName")]
    pub first_name: LocalizedString,
    #[serde(rename = "lastName")]
    pub last_name: LocalizedString,
    #[serde(rename = "shotType")]
    pub shot_type: String,
    pub result: String,
    pub headshot: String,
    #[serde(rename = "gameWinner")]
    pub game_winner: bool,
}

/// Three stars selection
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ThreeStar {
    pub star: i32,
    #[serde(rename = "playerId")]
    pub player_id: PlayerId,
    #[serde(rename = "teamAbbrev")]
    pub team_abbrev: String,
    pub headshot: String,
    /// The `default` text when the API sends a [`LocalizedString`].
    #[serde(deserialize_with = "plain_or_localized_name")]
    pub name: String,
    #[serde(rename = "sweaterNo")]
    pub sweater_no: i32,
    /// `None` for historical data where the API returns an empty position code.
    #[serde(deserialize_with = "empty_string_as_none", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<Position>,
    // Skater stats
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goals: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assists: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub points: Option<i32>,
    // Goalie stats
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "goalsAgainstAverage")]
    pub goals_against_average: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "savePctg")]
    pub save_pctg: Option<f64>,
}

/// `threeStars[].name` is a plain string in the game story but a
/// [`LocalizedString`] in the landing response; both become its `default` text.
fn plain_or_localized_name<'de, D: Deserializer<'de>>(deserializer: D) -> Result<String, D::Error> {
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Name {
        Plain(String),
        Localized(LocalizedString),
    }

    Ok(match Name::deserialize(deserializer)? {
        Name::Plain(name) => name,
        Name::Localized(name) => name.default,
    })
}

/// Penalty summary for a period
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PeriodPenalties {
    #[serde(rename = "periodDescriptor")]
    pub period_descriptor: PeriodDescriptor,
    pub penalties: Vec<PenaltySummary>,
}

/// Penalty summary information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PenaltySummary {
    #[serde(rename = "timeInPeriod")]
    pub time_in_period: String,
    #[serde(rename = "type")]
    pub penalty_type: String,
    pub duration: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "committedByPlayer")]
    pub committed_by_player: Option<PenaltyPlayer>,
    #[serde(rename = "teamAbbrev")]
    pub team_abbrev: LocalizedString,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "drawnBy")]
    pub drawn_by: Option<PenaltyPlayer>,
    #[serde(rename = "descKey")]
    pub desc_key: String,
    // Bench penalty specific field
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "servedBy")]
    pub served_by: Option<LocalizedString>,
    // Optional fields from play-by-play endpoint
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "eventId")]
    pub event_id: Option<i64>,
}

/// Player information in penalty summary
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PenaltyPlayer {
    #[serde(rename = "firstName")]
    pub first_name: LocalizedString,
    #[serde(rename = "lastName")]
    pub last_name: LocalizedString,
    #[serde(rename = "sweaterNumber")]
    pub sweater_number: i32,
}
