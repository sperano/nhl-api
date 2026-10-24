use serde::{Deserialize, Serialize};
use std::fmt;

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

/// Type of play event in a game
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PlayEventType {
    #[serde(rename = "game-start")]
    GameStart,
    #[serde(rename = "period-start")]
    PeriodStart,
    #[serde(rename = "period-end")]
    PeriodEnd,
    #[serde(rename = "game-end")]
    GameEnd,
    Faceoff,
    Hit,
    Giveaway,
    Takeaway,
    #[serde(rename = "shot-on-goal")]
    ShotOnGoal,
    #[serde(rename = "missed-shot")]
    MissedShot,
    #[serde(rename = "blocked-shot")]
    BlockedShot,
    Goal,
    Penalty,
    Stoppage,
    #[serde(rename = "delayed-penalty")]
    DelayedPenalty,
    #[serde(rename = "failed-shot-attempt")]
    FailedShotAttempt,
    #[serde(rename = "shootout-complete")]
    ShootoutComplete,
    #[serde(other)]
    Unknown,
}

impl PlayEventType {
    /// Returns true if this event is a scoring chance (shot, goal, missed shot, blocked shot)
    pub fn is_scoring_chance(&self) -> bool {
        matches!(
            self,
            Self::ShotOnGoal | Self::Goal | Self::MissedShot | Self::BlockedShot
        )
    }

    /// Returns true if this event affects the score
    pub fn is_goal(&self) -> bool {
        matches!(self, Self::Goal)
    }

    /// Returns true if this is a period boundary event
    pub fn is_period_boundary(&self) -> bool {
        matches!(
            self,
            Self::PeriodStart | Self::PeriodEnd | Self::GameStart | Self::GameEnd
        )
    }
}

impl fmt::Display for PlayEventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::GameStart => write!(f, "Game Start"),
            Self::PeriodStart => write!(f, "Period Start"),
            Self::PeriodEnd => write!(f, "Period End"),
            Self::GameEnd => write!(f, "Game End"),
            Self::Faceoff => write!(f, "Faceoff"),
            Self::Hit => write!(f, "Hit"),
            Self::Giveaway => write!(f, "Giveaway"),
            Self::Takeaway => write!(f, "Takeaway"),
            Self::ShotOnGoal => write!(f, "Shot on Goal"),
            Self::MissedShot => write!(f, "Missed Shot"),
            Self::BlockedShot => write!(f, "Blocked Shot"),
            Self::Goal => write!(f, "Goal"),
            Self::Penalty => write!(f, "Penalty"),
            Self::Stoppage => write!(f, "Stoppage"),
            Self::DelayedPenalty => write!(f, "Delayed Penalty"),
            Self::FailedShotAttempt => write!(f, "Failed Shot Attempt"),
            Self::ShootoutComplete => write!(f, "Shootout Complete"),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}

/// Parsed game situation from situation code
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GameSituation {
    /// Number of away team skaters on ice (1-6)
    pub away_skaters: u8,
    /// Whether away team has goalie in net
    pub away_goalie_in: bool,
    /// Number of home team skaters on ice (1-6)
    pub home_skaters: u8,
    /// Whether home team has goalie in net
    pub home_goalie_in: bool,
}

/// Minimum valid skater count encoded in a situation code (goalie excluded).
const MIN_SITUATION_SKATERS: u32 = 1;
/// Maximum valid skater count encoded in a situation code (goalie excluded).
const MAX_SITUATION_SKATERS: u32 = 6;

impl GameSituation {
    /// Parse a situation code string (e.g., "1551").
    ///
    /// Returns `None` if the code is not exactly 4 ASCII digits, either
    /// goalie flag is not `0`/`1`, or either skater count falls outside
    /// `MIN_SITUATION_SKATERS..=MAX_SITUATION_SKATERS`. An earlier version
    /// only checked the string length, so codes like `"a55b"` parsed
    /// "successfully" with both goalies read as pulled.
    pub fn from_code(code: &str) -> Option<Self> {
        if code.len() != 4 {
            return None;
        }

        let digits: Vec<u32> = code
            .chars()
            .map(|c| c.to_digit(10))
            .collect::<Option<_>>()?;
        let (away_goalie, away_skaters, home_skaters, home_goalie) =
            (digits[0], digits[1], digits[2], digits[3]);

        if away_goalie > 1 || home_goalie > 1 {
            return None;
        }

        let skater_range = MIN_SITUATION_SKATERS..=MAX_SITUATION_SKATERS;
        if !skater_range.contains(&away_skaters) || !skater_range.contains(&home_skaters) {
            return None;
        }

        Some(Self {
            away_goalie_in: away_goalie == 1,
            away_skaters: away_skaters as u8,
            home_skaters: home_skaters as u8,
            home_goalie_in: home_goalie == 1,
        })
    }

    /// Returns true if this is even strength (5v5, 4v4, or 3v3)
    pub fn is_even_strength(&self) -> bool {
        self.away_skaters == self.home_skaters
    }

    /// Returns true if away team has a power play
    pub fn is_away_power_play(&self) -> bool {
        self.away_goalie_in && self.home_goalie_in && self.away_skaters > self.home_skaters
    }

    /// Returns true if home team has a power play
    pub fn is_home_power_play(&self) -> bool {
        self.away_goalie_in && self.home_goalie_in && self.home_skaters > self.away_skaters
    }

    /// Returns true if either team has pulled their goalie
    pub fn is_empty_net(&self) -> bool {
        !self.away_goalie_in || !self.home_goalie_in
    }

    /// Returns the strength description (e.g., "5v5", "5v4 PP", "6v5 EN")
    pub fn strength_description(&self) -> String {
        let base = format!("{}v{}", self.away_skaters, self.home_skaters);

        if !self.away_goalie_in || !self.home_goalie_in {
            format!("{} EN", base)
        } else if self.away_skaters != self.home_skaters {
            format!("{} PP", base)
        } else {
            base
        }
    }
}

impl fmt::Display for GameSituation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.strength_description())
    }
}

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
    #[serde(rename = "periodDescriptor")]
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
    #[serde(rename = "situationCode")]
    pub situation_code: String,
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
        GameSituation::from_code(&self.situation_code)
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
use super::GameSummary;
