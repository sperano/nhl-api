use serde::{Deserialize, Serialize};

use super::boxscore::{BoxscoreTeam, GameClock, PeriodDescriptor, SpecialEvent, TvBroadcast};
use super::common::LocalizedString;

/// Play by play response with all game events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayByPlay {
    pub id: i64,
    pub season: i64,
    #[serde(rename = "gameType")]
    pub game_type: i32,
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
    pub game_state: String,
    #[serde(rename = "gameScheduleState")]
    pub game_schedule_state: String,
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
    #[serde(rename = "maxPeriods")]
    pub max_periods: i32,
    #[serde(rename = "gameOutcome")]
    pub game_outcome: GameOutcome,
    #[serde(default)]
    pub plays: Vec<PlayEvent>,
    #[serde(rename = "rosterSpots", default)]
    pub roster_spots: Vec<RosterSpot>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "regPeriods")]
    pub reg_periods: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<GameSummary>,
}

/// Game outcome information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GameOutcome {
    #[serde(rename = "lastPeriodType")]
    pub last_period_type: String,
}

/// Individual play event in the game
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    #[serde(rename = "homeTeamDefendingSide")]
    pub home_team_defending_side: String,
    #[serde(rename = "typeCode")]
    pub type_code: i32,
    #[serde(rename = "typeDescKey")]
    pub type_desc_key: String,
    #[serde(rename = "sortOrder")]
    pub sort_order: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<PlayEventDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "pptReplayUrl")]
    pub ppt_replay_url: Option<String>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "zoneCode")]
    pub zone_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "eventOwnerTeamId")]
    pub event_owner_team_id: Option<i64>,

    // Shot details
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "shotType")]
    pub shot_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "shootingPlayerId")]
    pub shooting_player_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "goalieInNetId")]
    pub goalie_in_net_id: Option<i64>,

    // Goal details
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "scoringPlayerId")]
    pub scoring_player_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "scoringPlayerTotal")]
    pub scoring_player_total: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "assist1PlayerId")]
    pub assist1_player_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "assist1PlayerTotal")]
    pub assist1_player_total: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "assist2PlayerId")]
    pub assist2_player_id: Option<i64>,
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
    pub committed_by_player_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "drawnByPlayerId")]
    pub drawn_by_player_id: Option<i64>,

    // Hit details
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "hittingPlayerId")]
    pub hitting_player_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "hitteePlayerId")]
    pub hittee_player_id: Option<i64>,

    // Faceoff details
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "winningPlayerId")]
    pub winning_player_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "losingPlayerId")]
    pub losing_player_id: Option<i64>,

    // General details
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "playerId")]
    pub player_id: Option<i64>,
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
    pub team_id: i64,
    #[serde(rename = "playerId")]
    pub player_id: i64,
    #[serde(rename = "firstName")]
    pub first_name: LocalizedString,
    #[serde(rename = "lastName")]
    pub last_name: LocalizedString,
    #[serde(rename = "sweaterNumber")]
    pub sweater_number: i32,
    #[serde(rename = "positionCode")]
    pub position_code: String,
    pub headshot: String,
}

/// Game matchup/landing response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameMatchup {
    pub id: i64,
    pub season: i64,
    #[serde(rename = "gameType")]
    pub game_type: i32,
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
    #[serde(rename = "periodDescriptor")]
    pub period_descriptor: PeriodDescriptor,
    #[serde(rename = "tvBroadcasts", default)]
    pub tv_broadcasts: Vec<TvBroadcast>,
    #[serde(rename = "gameState")]
    pub game_state: String,
    #[serde(rename = "gameScheduleState")]
    pub game_schedule_state: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "specialEvent")]
    pub special_event: Option<SpecialEvent>,
    #[serde(rename = "awayTeam")]
    pub away_team: MatchupTeam,
    #[serde(rename = "homeTeam")]
    pub home_team: MatchupTeam,
    #[serde(rename = "shootoutInUse")]
    pub shootout_in_use: bool,
    #[serde(rename = "maxPeriods")]
    pub max_periods: i32,
    #[serde(rename = "regPeriods")]
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
    pub id: i64,
    #[serde(rename = "commonName")]
    pub common_name: LocalizedString,
    pub abbrev: String,
    #[serde(rename = "placeName")]
    pub place_name: LocalizedString,
    #[serde(rename = "placeNameWithPreposition")]
    pub place_name_with_preposition: LocalizedString,
    pub score: i32,
    pub sog: i32,
    pub logo: String,
    #[serde(rename = "darkLogo")]
    pub dark_logo: String,
}

/// Game summary with scoring and penalties
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GameSummary {
    #[serde(default)]
    pub scoring: Vec<PeriodScoring>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shootout: Option<Vec<ShootoutAttempt>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "threeStars")]
    pub three_stars: Option<Vec<ThreeStar>>,
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
    #[serde(rename = "situationCode")]
    pub situation_code: String,
    #[serde(rename = "eventId")]
    pub event_id: i64,
    pub strength: String,
    #[serde(rename = "playerId")]
    pub player_id: i64,
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
    #[serde(rename = "goalsToDate")]
    pub goals_to_date: i32,
    #[serde(rename = "awayScore")]
    pub away_score: i32,
    #[serde(rename = "homeScore")]
    pub home_score: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "leadingTeamAbbrev")]
    pub leading_team_abbrev: Option<LocalizedString>,
    #[serde(rename = "timeInPeriod")]
    pub time_in_period: String,
    #[serde(rename = "shotType")]
    pub shot_type: String,
    #[serde(rename = "goalModifier")]
    pub goal_modifier: String,
    #[serde(default)]
    pub assists: Vec<AssistSummary>,
    #[serde(rename = "homeTeamDefendingSide")]
    pub home_team_defending_side: String,
    #[serde(rename = "isHome")]
    pub is_home: bool,
}

/// Assist summary information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AssistSummary {
    #[serde(rename = "playerId")]
    pub player_id: i64,
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
    #[serde(rename = "playerId")]
    pub player_id: i64,
    #[serde(rename = "firstName")]
    pub first_name: LocalizedString,
    #[serde(rename = "lastName")]
    pub last_name: LocalizedString,
    #[serde(rename = "teamAbbrev")]
    pub team_abbrev: String,
    pub headshot: String,
    #[serde(rename = "shotType")]
    pub shot_type: String,
    pub result: String,
    #[serde(rename = "isHomeTeam")]
    pub is_home_team: bool,
}

/// Three stars selection
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ThreeStar {
    pub star: i32,
    #[serde(rename = "playerId")]
    pub player_id: i64,
    #[serde(rename = "teamAbbrev")]
    pub team_abbrev: String,
    pub headshot: String,
    pub name: LocalizedString,
    #[serde(rename = "sweaterNo")]
    pub sweater_no: i32,
    pub position: String,
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

/// Shift chart data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShiftChart {
    pub data: Vec<ShiftEntry>,
}

/// Individual shift entry for a player
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ShiftEntry {
    pub id: i64,
    #[serde(rename = "detailCode")]
    pub detail_code: i32,
    pub duration: String,
    #[serde(rename = "endTime")]
    pub end_time: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "eventDescription")]
    pub event_description: Option<String>,
    #[serde(rename = "eventNumber")]
    pub event_number: i64,
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "gameId")]
    pub game_id: i64,
    #[serde(rename = "hexValue")]
    pub hex_value: String,
    #[serde(rename = "lastName")]
    pub last_name: String,
    pub period: i32,
    #[serde(rename = "playerId")]
    pub player_id: i64,
    #[serde(rename = "shiftNumber")]
    pub shift_number: i32,
    #[serde(rename = "startTime")]
    pub start_time: String,
    #[serde(rename = "teamAbbrev")]
    pub team_abbrev: String,
    #[serde(rename = "teamId")]
    pub team_id: i64,
    #[serde(rename = "teamName")]
    pub team_name: String,
    #[serde(rename = "typeCode")]
    pub type_code: i32,
}

/// Season series matchup
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub id: i64,
    pub season: i64,
    #[serde(rename = "gameType")]
    pub game_type: i32,
    #[serde(rename = "gameDate")]
    pub game_date: String,
    #[serde(rename = "startTimeUTC")]
    pub start_time_utc: String,
    #[serde(rename = "easternUTCOffset")]
    pub eastern_utc_offset: String,
    #[serde(rename = "venueUTCOffset")]
    pub venue_utc_offset: String,
    #[serde(rename = "gameState")]
    pub game_state: String,
    #[serde(rename = "gameScheduleState")]
    pub game_schedule_state: String,
    #[serde(rename = "awayTeam")]
    pub away_team: SeriesTeam,
    #[serde(rename = "homeTeam")]
    pub home_team: SeriesTeam,
    #[serde(rename = "periodDescriptor")]
    pub period_descriptor: PeriodDescriptor,
    #[serde(rename = "gameCenterLink")]
    pub game_center_link: String,
    #[serde(rename = "gameOutcome")]
    pub game_outcome: GameOutcome,
}

/// Team information in season series
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SeriesTeam {
    pub id: i64,
    pub abbrev: String,
    pub logo: String,
    pub score: i32,
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
    pub referees: Vec<LocalizedString>,
    pub linesmen: Vec<LocalizedString>,
    #[serde(rename = "awayTeam")]
    pub away_team: TeamGameInfo,
    #[serde(rename = "homeTeam")]
    pub home_team: TeamGameInfo,
}

/// Team-specific game information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TeamGameInfo {
    #[serde(rename = "headCoach")]
    pub head_coach: LocalizedString,
    pub scratches: Vec<ScratchedPlayer>,
}

/// Scratched player information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScratchedPlayer {
    pub id: i64,
    #[serde(rename = "firstName")]
    pub first_name: LocalizedString,
    #[serde(rename = "lastName")]
    pub last_name: LocalizedString,
}

/// Game story
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameStory {
    pub id: i64,
    pub season: i64,
    #[serde(rename = "gameType")]
    pub game_type: i32,
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
    pub game_state: String,
    #[serde(rename = "gameScheduleState")]
    pub game_schedule_state: String,
    #[serde(rename = "awayTeam")]
    pub away_team: StoryTeam,
    #[serde(rename = "homeTeam")]
    pub home_team: StoryTeam,
    #[serde(rename = "shootoutInUse")]
    pub shootout_in_use: bool,
    #[serde(rename = "maxPeriods")]
    pub max_periods: i32,
    #[serde(rename = "regPeriods")]
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
    pub id: i64,
    pub name: LocalizedString,
    pub abbrev: String,
    #[serde(rename = "placeName")]
    pub place_name: LocalizedString,
    pub score: i32,
    pub sog: i32,
    pub logo: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_play_event_goal_deserialization() {
        let json = r#"{
            "eventId": 274,
            "periodDescriptor": {
                "number": 1,
                "periodType": "REG",
                "maxRegulationPeriods": 3
            },
            "timeInPeriod": "08:39",
            "timeRemaining": "11:21",
            "situationCode": "1551",
            "homeTeamDefendingSide": "right",
            "typeCode": 505,
            "typeDescKey": "goal",
            "sortOrder": 146,
            "details": {
                "xCoord": 71,
                "yCoord": -12,
                "zoneCode": "O",
                "shotType": "snap",
                "scoringPlayerId": 8476474,
                "scoringPlayerTotal": 1,
                "assist1PlayerId": 8480192,
                "assist1PlayerTotal": 1,
                "eventOwnerTeamId": 1,
                "goalieInNetId": 8480045,
                "awayScore": 1,
                "homeScore": 0,
                "highlightClip": 6362848229112,
                "discreteClip": 6362846260112
            }
        }"#;

        let event: PlayEvent = serde_json::from_str(json).unwrap();
        assert_eq!(event.event_id, 274);
        assert_eq!(event.type_desc_key, "goal");
        assert_eq!(event.time_in_period, "08:39");

        let details = event.details.unwrap();
        assert_eq!(details.scoring_player_id, Some(8476474));
        assert_eq!(details.scoring_player_total, Some(1));
        assert_eq!(details.assist1_player_id, Some(8480192));
        assert_eq!(details.away_score, Some(1));
        assert_eq!(details.home_score, Some(0));
        assert_eq!(details.shot_type, Some("snap".to_string()));
    }

    #[test]
    fn test_play_event_penalty_deserialization() {
        let json = r#"{
            "eventId": 135,
            "periodDescriptor": {
                "number": 1,
                "periodType": "REG",
                "maxRegulationPeriods": 3
            },
            "timeInPeriod": "01:37",
            "timeRemaining": "18:23",
            "situationCode": "1560",
            "homeTeamDefendingSide": "right",
            "typeCode": 509,
            "typeDescKey": "penalty",
            "sortOrder": 45,
            "details": {
                "xCoord": 1,
                "yCoord": -37,
                "zoneCode": "N",
                "typeCode": "MIN",
                "descKey": "slashing",
                "duration": 2,
                "committedByPlayerId": 8475287,
                "drawnByPlayerId": 8479420,
                "eventOwnerTeamId": 1
            }
        }"#;

        let event: PlayEvent = serde_json::from_str(json).unwrap();
        assert_eq!(event.event_id, 135);
        assert_eq!(event.type_desc_key, "penalty");

        let details = event.details.unwrap();
        assert_eq!(details.type_code, Some("MIN".to_string()));
        assert_eq!(details.desc_key, Some("slashing".to_string()));
        assert_eq!(details.duration, Some(2));
        assert_eq!(details.committed_by_player_id, Some(8475287));
        assert_eq!(details.drawn_by_player_id, Some(8479420));
    }

    #[test]
    fn test_play_event_shot_deserialization() {
        let json = r#"{
            "eventId": 103,
            "periodDescriptor": {
                "number": 1,
                "periodType": "REG",
                "maxRegulationPeriods": 3
            },
            "timeInPeriod": "00:08",
            "timeRemaining": "19:52",
            "situationCode": "1551",
            "homeTeamDefendingSide": "right",
            "typeCode": 506,
            "typeDescKey": "shot-on-goal",
            "sortOrder": 13,
            "details": {
                "xCoord": 56,
                "yCoord": -39,
                "zoneCode": "O",
                "shotType": "wrist",
                "shootingPlayerId": 8483495,
                "goalieInNetId": 8480045,
                "eventOwnerTeamId": 1,
                "awaySOG": 1,
                "homeSOG": 0
            }
        }"#;

        let event: PlayEvent = serde_json::from_str(json).unwrap();
        assert_eq!(event.event_id, 103);
        assert_eq!(event.type_desc_key, "shot-on-goal");

        let details = event.details.unwrap();
        assert_eq!(details.shot_type, Some("wrist".to_string()));
        assert_eq!(details.shooting_player_id, Some(8483495));
        assert_eq!(details.goalie_in_net_id, Some(8480045));
        assert_eq!(details.away_sog, Some(1));
        assert_eq!(details.home_sog, Some(0));
    }

    #[test]
    fn test_play_event_faceoff_deserialization() {
        let json = r#"{
            "eventId": 151,
            "periodDescriptor": {
                "number": 1,
                "periodType": "REG",
                "maxRegulationPeriods": 3
            },
            "timeInPeriod": "00:00",
            "timeRemaining": "20:00",
            "situationCode": "1551",
            "homeTeamDefendingSide": "right",
            "typeCode": 502,
            "typeDescKey": "faceoff",
            "sortOrder": 11,
            "details": {
                "eventOwnerTeamId": 1,
                "losingPlayerId": 8478043,
                "winningPlayerId": 8480002,
                "xCoord": 0,
                "yCoord": 0,
                "zoneCode": "N"
            }
        }"#;

        let event: PlayEvent = serde_json::from_str(json).unwrap();
        assert_eq!(event.event_id, 151);
        assert_eq!(event.type_desc_key, "faceoff");

        let details = event.details.unwrap();
        assert_eq!(details.winning_player_id, Some(8480002));
        assert_eq!(details.losing_player_id, Some(8478043));
        assert_eq!(details.zone_code, Some("N".to_string()));
    }

    #[test]
    fn test_roster_spot_deserialization() {
        let json = r#"{
            "teamId": 1,
            "playerId": 8474593,
            "firstName": {"default": "Jacob"},
            "lastName": {"default": "Markstrom"},
            "sweaterNumber": 25,
            "positionCode": "G",
            "headshot": "https://assets.nhle.com/mugs/nhl/20242025/NJD/8474593.png"
        }"#;

        let roster_spot: RosterSpot = serde_json::from_str(json).unwrap();
        assert_eq!(roster_spot.team_id, 1);
        assert_eq!(roster_spot.player_id, 8474593);
        assert_eq!(roster_spot.first_name.default, "Jacob");
        assert_eq!(roster_spot.last_name.default, "Markstrom");
        assert_eq!(roster_spot.sweater_number, 25);
        assert_eq!(roster_spot.position_code, "G");
    }

    #[test]
    fn test_game_outcome_deserialization() {
        let json = r#"{"lastPeriodType": "REG"}"#;
        let outcome: GameOutcome = serde_json::from_str(json).unwrap();
        assert_eq!(outcome.last_period_type, "REG");
    }

    #[test]
    fn test_shift_entry_deserialization() {
        let json = r##"{
            "id": 14376602,
            "detailCode": 0,
            "duration": "17:15",
            "endTime": "17:15",
            "eventDescription": null,
            "eventNumber": 101,
            "firstName": "Jacob",
            "gameId": 2024020001,
            "hexValue": "#C8102E",
            "lastName": "Markstrom",
            "period": 1,
            "playerId": 8474593,
            "shiftNumber": 1,
            "startTime": "00:00",
            "teamAbbrev": "NJD",
            "teamId": 1,
            "teamName": "New Jersey Devils",
            "typeCode": 517
        }"##;

        let shift: ShiftEntry = serde_json::from_str(json).unwrap();
        assert_eq!(shift.id, 14376602);
        assert_eq!(shift.detail_code, 0);
        assert_eq!(shift.duration, "17:15");
        assert_eq!(shift.end_time, "17:15");
        assert_eq!(shift.event_description, None);
        assert_eq!(shift.event_number, 101);
        assert_eq!(shift.first_name, "Jacob");
        assert_eq!(shift.game_id, 2024020001);
        assert_eq!(shift.hex_value, "#C8102E");
        assert_eq!(shift.last_name, "Markstrom");
        assert_eq!(shift.period, 1);
        assert_eq!(shift.player_id, 8474593);
        assert_eq!(shift.shift_number, 1);
        assert_eq!(shift.start_time, "00:00");
        assert_eq!(shift.team_abbrev, "NJD");
        assert_eq!(shift.team_id, 1);
        assert_eq!(shift.team_name, "New Jersey Devils");
        assert_eq!(shift.type_code, 517);
    }

    #[test]
    fn test_shift_chart_deserialization() {
        let json = r##"{
            "data": [
                {
                    "id": 14376602,
                    "detailCode": 0,
                    "duration": "17:15",
                    "endTime": "17:15",
                    "eventDescription": null,
                    "eventNumber": 101,
                    "firstName": "Jacob",
                    "gameId": 2024020001,
                    "hexValue": "#C8102E",
                    "lastName": "Markstrom",
                    "period": 1,
                    "playerId": 8474593,
                    "shiftNumber": 1,
                    "startTime": "00:00",
                    "teamAbbrev": "NJD",
                    "teamId": 1,
                    "teamName": "New Jersey Devils",
                    "typeCode": 517
                }
            ]
        }"##;

        let chart: ShiftChart = serde_json::from_str(json).unwrap();
        assert_eq!(chart.data.len(), 1);
        assert_eq!(chart.data[0].player_id, 8474593);
        assert_eq!(chart.data[0].first_name, "Jacob");
        assert_eq!(chart.data[0].last_name, "Markstrom");
    }
}
