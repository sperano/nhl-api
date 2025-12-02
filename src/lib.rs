mod client;
mod config;
mod date;
mod error;
mod http_client;
mod ids;
mod types;

// Client
pub use client::Client;

// Config
pub use config::ClientConfig;

// Date and Season
pub use date::{GameDate, Season};

// Error types
pub use error::NHLApiError;

// IDs
pub use ids::GameId;

// Common types
pub use types::{Conference, Division, Franchise, FranchisesResponse, LocalizedString, Roster, RosterPlayer, Team};

// Boxscore types
pub use types::{
    Boxscore, BoxscoreTeam, GameClock, GoalieStats, PlayerByGameStats, PeriodDescriptor,
    SkaterStats, SpecialEvent, TeamGameStats, TeamPlayerStats, TvBroadcast,
};

// Club stats types
pub use types::{ClubGoalieStats, ClubSkaterStats, ClubStats, SeasonGameTypes};

// Game center types
pub use types::{
    AssistSummary, GameMatchup, GameOutcome, GameStory, GameSummary, GoalSummary, MatchupTeam,
    PenaltyPlayer, PenaltySummary, PeriodPenalties, PeriodScoring, PlayByPlay, PlayEvent,
    PlayEventDetails, RosterSpot, ScratchedPlayer, SeasonSeriesMatchup, SeriesGame,
    SeriesGameInfo, SeriesTeam, SeriesWins, ShiftChart, ShiftEntry, ShootoutAttempt, StoryTeam,
    TeamGameInfo, ThreeStar,
};

// Game state types
pub use types::{GameState, ParseGameStateError};

// Game type
pub use types::GameType;

// Enum types
pub use types::{
    DefendingSide, GameScheduleState, GoalieDecision, Handedness, HomeRoad,
    ParseDefendingSideError, ParseGameScheduleStateError, ParseGoalieDecisionError,
    ParseHandednessError, ParseHomeRoadError, ParsePeriodTypeError, ParsePositionError,
    ParseZoneCodeError, PeriodType, Position, ZoneCode,
};

// Player types
pub use types::{
    Award, AwardSeason, CareerTotals, DraftDetails, FeaturedStats, GameLog, PlayerGameLog,
    PlayerLanding, PlayerSearchResult, PlayerStats, SeasonTotal,
};

// Schedule types
pub use types::{
    DailySchedule, DailyScores, GameDay, GameScore, ScheduleGame, ScheduleTeam,
    TeamScheduleResponse, WeeklyScheduleResponse,
};

// Standings types
pub use types::{SeasonInfo, SeasonsResponse, Standing, StandingsResponse};
