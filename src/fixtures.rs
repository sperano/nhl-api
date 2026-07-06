//! Test fixture constructors, gated behind the `fixtures` cargo feature.
//!
//! These return minimum-valid objects that survive a `serde_json` round-trip
//! — useful for downstream consumers (e.g. puckdb) that need a throwaway
//! `Boxscore`/`PlayByPlay`/etc. in their own tests without hand-assembling
//! every required field. Mirrors the Go reference's `fixtures.go`.
//!
//! Several NHL enum types (`GameType`, `GameState`, `GameScheduleState`,
//! `PeriodType`) reject absent/zero values during deserialization, so each
//! constructor seeds those fields to a valid variant; every other field gets
//! the simplest value that still round-trips (empty string/vec, `0`,
//! `false`).

use crate::date::Season;
use crate::ids::{GameId, TeamId};
use crate::types::{
    Boxscore, BoxscoreTeam, GameClock, GameScheduleState, GameState, GameStory, GameType,
    LocalizedString, PeriodDescriptor, PeriodType, PlayByPlay, PlayerByGameStats,
    SeasonSeriesMatchup, SeriesGameInfo, SeriesWins, ShiftChart, StoryTeam, TeamGameInfo,
    TeamPlayerStats,
};

/// Starting year for the placeholder `Season` used by these fixtures. Not
/// tied to any specific NHL year — any valid season works here.
const FIXTURE_SEASON_START_YEAR: u16 = 2024;

fn fixture_period_descriptor() -> PeriodDescriptor {
    PeriodDescriptor {
        number: 0,
        period_type: Some(PeriodType::Regulation),
        max_regulation_periods: 0,
    }
}

fn fixture_boxscore_team() -> BoxscoreTeam {
    BoxscoreTeam {
        id: TeamId::new(0),
        common_name: LocalizedString::default(),
        abbrev: String::new(),
        score: 0,
        sog: 0,
        logo: String::new(),
        dark_logo: String::new(),
        place_name: LocalizedString::default(),
        place_name_with_preposition: LocalizedString::default(),
    }
}

fn fixture_game_clock() -> GameClock {
    GameClock {
        time_remaining: String::new(),
        seconds_remaining: 0,
        running: false,
        in_intermission: false,
    }
}

fn fixture_team_player_stats() -> TeamPlayerStats {
    TeamPlayerStats {
        forwards: Vec::new(),
        defense: Vec::new(),
        goalies: Vec::new(),
    }
}

fn fixture_story_team() -> StoryTeam {
    StoryTeam {
        id: TeamId::new(0),
        name: LocalizedString::default(),
        abbrev: String::new(),
        place_name: LocalizedString::default(),
        score: 0,
        sog: 0,
        logo: String::new(),
    }
}

/// Returns a `Boxscore` with the minimum fields needed for `serde_json::to_string`.
pub fn boxscore() -> Boxscore {
    Boxscore {
        id: GameId::new(0),
        season: Season::new(FIXTURE_SEASON_START_YEAR),
        game_type: GameType::RegularSeason,
        limited_scoring: false,
        game_date: String::new(),
        venue: LocalizedString::default(),
        venue_location: LocalizedString::default(),
        start_time_utc: String::new(),
        eastern_utc_offset: String::new(),
        venue_utc_offset: String::new(),
        tv_broadcasts: Vec::new(),
        game_state: GameState::Final,
        game_schedule_state: GameScheduleState::Ok,
        period_descriptor: fixture_period_descriptor(),
        special_event: None,
        away_team: fixture_boxscore_team(),
        home_team: fixture_boxscore_team(),
        clock: fixture_game_clock(),
        player_by_game_stats: PlayerByGameStats {
            away_team: fixture_team_player_stats(),
            home_team: fixture_team_player_stats(),
        },
    }
}

/// Returns a `PlayByPlay` with the minimum fields needed for `serde_json::to_string`.
pub fn play_by_play() -> PlayByPlay {
    PlayByPlay {
        id: GameId::new(0),
        season: Season::new(FIXTURE_SEASON_START_YEAR),
        game_type: GameType::RegularSeason,
        limited_scoring: false,
        game_date: String::new(),
        venue: LocalizedString::default(),
        venue_location: LocalizedString::default(),
        start_time_utc: String::new(),
        eastern_utc_offset: String::new(),
        venue_utc_offset: String::new(),
        tv_broadcasts: Vec::new(),
        game_state: GameState::Final,
        game_schedule_state: GameScheduleState::Ok,
        period_descriptor: fixture_period_descriptor(),
        special_event: None,
        away_team: fixture_boxscore_team(),
        home_team: fixture_boxscore_team(),
        shootout_in_use: false,
        ot_in_use: false,
        clock: fixture_game_clock(),
        display_period: 0,
        max_periods: 0,
        game_outcome: None,
        plays: Vec::new(),
        roster_spots: Vec::new(),
        reg_periods: 0,
        summary: None,
    }
}

/// Returns a `GameStory` with the minimum fields needed for `serde_json::to_string`.
pub fn game_story() -> GameStory {
    GameStory {
        id: GameId::new(0),
        season: Season::new(FIXTURE_SEASON_START_YEAR),
        game_type: GameType::RegularSeason,
        limited_scoring: false,
        game_date: String::new(),
        venue: LocalizedString::default(),
        venue_location: LocalizedString::default(),
        start_time_utc: String::new(),
        eastern_utc_offset: String::new(),
        venue_utc_offset: String::new(),
        venue_timezone: String::new(),
        tv_broadcasts: Vec::new(),
        game_state: GameState::Final,
        game_schedule_state: GameScheduleState::Ok,
        away_team: fixture_story_team(),
        home_team: fixture_story_team(),
        shootout_in_use: false,
        max_periods: 0,
        reg_periods: 0,
        ot_in_use: false,
        ties_in_use: false,
        summary: None,
    }
}

/// Returns a `ShiftChart`. It has no strict enum fields, so an empty `data`
/// vec is already valid; provided for API consistency with the other
/// fixtures.
pub fn shift_chart() -> ShiftChart {
    ShiftChart { data: Vec::new() }
}

/// Returns a `SeasonSeriesMatchup`. It has no top-level strict enum fields
/// (enums only appear in the nested `SeriesGame` entries), so an empty
/// series list is already valid.
pub fn season_series_matchup() -> SeasonSeriesMatchup {
    SeasonSeriesMatchup {
        season_series: Vec::new(),
        season_series_wins: SeriesWins {
            away_team_wins: 0,
            home_team_wins: 0,
        },
        game_info: SeriesGameInfo {
            referees: Vec::new(),
            linesmen: Vec::new(),
            away_team: TeamGameInfo {
                head_coach: LocalizedString::default(),
                scratches: Vec::new(),
            },
            home_team: TeamGameInfo {
                head_coach: LocalizedString::default(),
                scratches: Vec::new(),
            },
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Fixtures must survive a *real* round-trip (serialize then deserialize
    /// then compare), not just produce non-empty JSON — a weak `len != 0`
    /// check was flagged in the Go review as insufficient.
    #[test]
    fn test_boxscore_round_trip() {
        let fixture = boxscore();
        let json = serde_json::to_string(&fixture).expect("fixture must serialize");
        let round_tripped: Boxscore =
            serde_json::from_str(&json).expect("fixture JSON must deserialize");
        assert_eq!(fixture, round_tripped);
    }

    #[test]
    fn test_play_by_play_round_trip() {
        let fixture = play_by_play();
        let json = serde_json::to_string(&fixture).expect("fixture must serialize");
        let round_tripped: PlayByPlay =
            serde_json::from_str(&json).expect("fixture JSON must deserialize");
        assert_eq!(fixture, round_tripped);
    }

    #[test]
    fn test_game_story_round_trip() {
        let fixture = game_story();
        let json = serde_json::to_string(&fixture).expect("fixture must serialize");
        let round_tripped: GameStory =
            serde_json::from_str(&json).expect("fixture JSON must deserialize");
        assert_eq!(fixture, round_tripped);
    }

    #[test]
    fn test_shift_chart_round_trip() {
        let fixture = shift_chart();
        let json = serde_json::to_string(&fixture).expect("fixture must serialize");
        let round_tripped: ShiftChart =
            serde_json::from_str(&json).expect("fixture JSON must deserialize");
        assert_eq!(fixture, round_tripped);
    }

    #[test]
    fn test_season_series_matchup_round_trip() {
        let fixture = season_series_matchup();
        let json = serde_json::to_string(&fixture).expect("fixture must serialize");
        let round_tripped: SeasonSeriesMatchup =
            serde_json::from_str(&json).expect("fixture JSON must deserialize");
        assert_eq!(fixture, round_tripped);
    }
}
