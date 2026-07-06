mod client;
mod config;
mod date;
mod error;
#[cfg(feature = "fixtures")]
pub mod fixtures;
mod http_client;
mod ids;
mod types;

// Client
pub use client::Client;

// Config
pub use config::ClientConfig;

// Date and Season
pub use date::{GameDate, Season, SeasonError};

// Error types
pub use error::NHLApiError;

// IDs
pub use ids::{GameId, PlayerId, TeamId};

// Common types
pub use types::{
    Conference, Division, Franchise, FranchisesResponse, LocalizedString, Roster, RosterPlayer,
    Team,
};

// Boxscore types
pub use types::{
    Boxscore, BoxscoreTeam, GameClock, GoalieStats, PeriodDescriptor, PlayerByGameStats,
    SkaterStats, SpecialEvent, TeamGameStats, TeamPlayerStats, TvBroadcast,
};

// Club stats types
pub use types::{ClubGoalieStats, ClubSkaterStats, ClubStats, SeasonGameTypes};

// Game center types
pub use types::{
    AssistSummary, GameMatchup, GameOutcome, GameSituation, GameStory, GameSummary, GoalSummary,
    MatchupTeam, PenaltyPlayer, PenaltySummary, PeriodPenalties, PeriodScoring, PlayByPlay,
    PlayEvent, PlayEventDetails, PlayEventType, RosterSpot, ScratchedPlayer, SeasonSeriesMatchup,
    SeriesGame, SeriesGameInfo, SeriesTeam, SeriesWins, ShiftChart, ShiftEntry, ShootoutAttempt,
    StoryTeam, TeamGameInfo, ThreeStar,
};

// Game state types
pub use types::{GameState, ParseGameStateError};

// Game type
pub use types::GameType;

// Enum types
pub use types::{
    DefendingSide, GameScheduleState, GoalieDecision, Handedness, HomeRoad, PeriodType, Position,
    UnknownEnumValue, ZoneCode,
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

// Edge stats shared types
pub use types::{
    EdgeComparisonDistanceLast10Entry, EdgeComparisonShotLocationDetail,
    EdgeComparisonShotLocationTotal, EdgeComparisonShotSpeedDetails,
    EdgeComparisonSkatingDistanceDetails, EdgeComparisonSkatingSpeedDetails,
    EdgeComparisonZoneStarts, EdgeComparisonZoneTimeDetails, EdgeCountLeagueAvg,
    EdgeCountPercentileStat, EdgeGoaliePlayer, EdgeLeaderShotLocation, EdgeMeasurement,
    EdgeMeasurementWithOverlay, EdgeOverlay, EdgeOverlayPlayer, EdgeOverlayTeam,
    EdgePercentileStat, EdgePercentileStatWithOverlay, EdgeRankStat, EdgeRankStatWithOverlay,
    EdgeSeasonAvailability, EdgeSkaterPlayer, EdgeTeamInfo, EdgeTeamLogo,
};
