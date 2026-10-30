use serde::{Deserialize, Serialize};

use crate::date::Season;
use crate::ids::{GameId, PlayerId, TeamId};

use super::super::boxscore::{
    BoxscoreTeam, GameClock, PeriodDescriptor, SpecialEvent, TvBroadcast,
};
use super::super::common::LocalizedString;
use super::super::enums::{
    empty_string_as_none, DefendingSide, GameScheduleState, PeriodType, Position, ZoneCode,
};
use super::super::game_state::GameState;
use super::super::game_type::GameType;
use super::{GameSituation, GameSummary, PlayEventType};

/// Play by play response with all game events
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlayByPlay {
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
    #[serde(rename = "tvBroadcasts", default)]
    pub tv_broadcasts: Vec<TvBroadcast>,
    #[serde(rename = "gameState")]
    pub game_state: GameState,
    #[serde(rename = "gameScheduleState")]
    pub game_schedule_state: GameScheduleState,
    /// All defaults before puck drop, when the API omits it.
    #[serde(rename = "periodDescriptor", default)]
    pub period_descriptor: PeriodDescriptor,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "specialEvent")]
    pub special_event: Option<SpecialEvent>,
    #[serde(rename = "awayTeam")]
    pub away_team: BoxscoreTeam,
    #[serde(rename = "homeTeam")]
    pub home_team: BoxscoreTeam,
    #[serde(rename = "shootoutInUse")]
    pub shootout_in_use: bool,
    #[serde(rename = "otInUse")]
    pub ot_in_use: bool,
    pub clock: GameClock,
    #[serde(rename = "displayPeriod")]
    pub display_period: i32,
    /// `None` for playoff games, whose overtime has no period limit.
    #[serde(rename = "maxPeriods", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_periods: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "gameOutcome")]
    pub game_outcome: Option<GameOutcome>,
    #[serde(default)]
    pub plays: Vec<PlayEvent>,
    #[serde(rename = "rosterSpots", default)]
    pub roster_spots: Vec<RosterSpot>,
    #[serde(rename = "regPeriods", default)]
    pub reg_periods: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<GameSummary>,
}

impl PlayByPlay {
    /// Get the most recent N plays (most recent first)
    pub fn recent_plays(&self, count: usize) -> Vec<&PlayEvent> {
        self.plays.iter().rev().take(count).collect()
    }

    /// Get all goals in the game
    pub fn goals(&self) -> Vec<&PlayEvent> {
        self.plays
            .iter()
            .filter(|p| p.type_desc_key == PlayEventType::Goal)
            .collect()
    }

    /// Get all penalties in the game
    pub fn penalties(&self) -> Vec<&PlayEvent> {
        self.plays
            .iter()
            .filter(|p| p.type_desc_key == PlayEventType::Penalty)
            .collect()
    }

    /// Get all shots (on goal, missed, blocked)
    pub fn shots(&self) -> Vec<&PlayEvent> {
        self.plays
            .iter()
            .filter(|p| p.type_desc_key.is_scoring_chance())
            .collect()
    }

    /// Get plays for a specific period
    pub fn plays_in_period(&self, period: i32) -> Vec<&PlayEvent> {
        self.plays
            .iter()
            .filter(|p| p.period_descriptor.number == period)
            .collect()
    }

    /// Get a player from the roster by ID
    pub fn get_player(&self, player_id: impl Into<PlayerId>) -> Option<&RosterSpot> {
        let player_id = player_id.into();
        self.roster_spots.iter().find(|p| p.player_id == player_id)
    }

    /// Get all players for a team
    pub fn team_roster(&self, team_id: impl Into<TeamId>) -> Vec<&RosterSpot> {
        let team_id = team_id.into();
        self.roster_spots
            .iter()
            .filter(|p| p.team_id == team_id)
            .collect()
    }

    /// Get the current game situation from the latest play
    pub fn current_situation(&self) -> Option<GameSituation> {
        self.plays.last()?.situation()
    }
}

/// Game outcome information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GameOutcome {
    /// `None` when the API returns `""` or omits the field entirely, which
    /// happens for unplayed games (e.g. future season-series entries) —
    /// see `empty_string_as_none`.
    #[serde(
        rename = "lastPeriodType",
        deserialize_with = "empty_string_as_none",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_period_type: Option<PeriodType>,
}

/// Individual play event in the game
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlayEvent {
    #[serde(rename = "eventId")]
    pub event_id: i64,
    #[serde(rename = "periodDescriptor")]
    pub period_descriptor: PeriodDescriptor,
    #[serde(rename = "timeInPeriod")]
    pub time_in_period: String,
    #[serde(rename = "timeRemaining")]
    pub time_remaining: String,
    /// `None` for every event of older games (2008-09 and earlier) and for
    /// some events of later ones (period starts and ends).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "situationCode")]
    pub situation_code: Option<String>,
    /// `None` for historical games that lack defending-side data.
    #[serde(
        rename = "homeTeamDefendingSide",
        deserialize_with = "empty_string_as_none",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_team_defending_side: Option<DefendingSide>,
    #[serde(rename = "typeCode")]
    pub type_code: i32,
    #[serde(rename = "typeDescKey")]
    pub type_desc_key: PlayEventType,
    #[serde(rename = "sortOrder")]
    pub sort_order: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<PlayEventDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "pptReplayUrl")]
    pub ppt_replay_url: Option<String>,
}

impl PlayEvent {
    /// Parse the situation code into a GameSituation
    pub fn situation(&self) -> Option<GameSituation> {
        GameSituation::from_code(self.situation_code.as_deref()?)
    }
}

/// Details for a play event (varies by event type)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlayEventDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "xCoord")]
    pub x_coord: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "yCoord")]
    pub y_coord: Option<i32>,
    /// `None` when the API returns `""` or omits the field entirely, which
    /// happens for some play events — see `empty_string_as_none`.
    #[serde(
        rename = "zoneCode",
        deserialize_with = "empty_string_as_none",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_code: Option<ZoneCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "eventOwnerTeamId")]
    pub event_owner_team_id: Option<TeamId>,

    // Shot details
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "shotType")]
    pub shot_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "shootingPlayerId")]
    pub shooting_player_id: Option<PlayerId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "goalieInNetId")]
    pub goalie_in_net_id: Option<PlayerId>,

    // Blocked shot details
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "blockingPlayerId")]
    pub blocking_player_id: Option<PlayerId>,

    // Goal details
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "scoringPlayerId")]
    pub scoring_player_id: Option<PlayerId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "scoringPlayerTotal")]
    pub scoring_player_total: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "assist1PlayerId")]
    pub assist1_player_id: Option<PlayerId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "assist1PlayerTotal")]
    pub assist1_player_total: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "assist2PlayerId")]
    pub assist2_player_id: Option<PlayerId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "assist2PlayerTotal")]
    pub assist2_player_total: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "awayScore")]
    pub away_score: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "homeScore")]
    pub home_score: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "highlightClip")]
    pub highlight_clip: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "highlightClipSharingUrl")]
    pub highlight_clip_sharing_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "discreteClip")]
    pub discrete_clip: Option<i64>,

    // Penalty details
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "typeCode")]
    pub type_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "descKey")]
    pub desc_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "committedByPlayerId")]
    pub committed_by_player_id: Option<PlayerId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "drawnByPlayerId")]
    pub drawn_by_player_id: Option<PlayerId>,

    // Hit details
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "hittingPlayerId")]
    pub hitting_player_id: Option<PlayerId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "hitteePlayerId")]
    pub hittee_player_id: Option<PlayerId>,

    // Faceoff details
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "winningPlayerId")]
    pub winning_player_id: Option<PlayerId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "losingPlayerId")]
    pub losing_player_id: Option<PlayerId>,

    // General details
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "playerId")]
    pub player_id: Option<PlayerId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "awaySOG")]
    pub away_sog: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "homeSOG")]
    pub home_sog: Option<i32>,
}

/// Roster spot with player information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RosterSpot {
    #[serde(rename = "teamId")]
    pub team_id: TeamId,
    #[serde(rename = "playerId")]
    pub player_id: PlayerId,
    #[serde(rename = "firstName")]
    pub first_name: LocalizedString,
    #[serde(rename = "lastName")]
    pub last_name: LocalizedString,
    #[serde(rename = "sweaterNumber")]
    pub sweater_number: i32,
    /// `None` for historical roster data where the API returns an empty
    /// position code.
    #[serde(
        rename = "positionCode",
        deserialize_with = "empty_string_as_none",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<Position>,
    pub headshot: String,
}
