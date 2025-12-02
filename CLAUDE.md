# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust client library for the NHL API. It provides type-safe access to NHL statistics, scores, schedules, and standings data through the official NHL API endpoints.

## Build Commands

```bash
# Build the library
cargo build

# Run all tests
cargo test

# Run tests for a specific module
cargo test --lib date::
cargo test --lib ids::
cargo test --lib http_client::
cargo test --lib standings::
cargo test --lib schedule::

# Run a single test
cargo test --lib test_name

# Check for compilation errors without building
cargo check

# Format code
cargo fmt

# Run linter
cargo clippy
```

## Architecture

### Layered Design

The codebase follows a clean layered architecture:

1. **Client Layer** (`client.rs`): High-level API methods that users interact with
2. **HTTP Layer** (`http_client.rs`): Handles HTTP requests, response handling, and error mapping
3. **Types Layer** (`types/`): Serde-based data structures matching NHL API responses
4. **Supporting Modules**: Shared utilities (config, error, date, ids, enums)

### Key Components

**Client (`client.rs`)**
- Main entry point for library users
- All methods are async and return `Result<T>`
- Game methods accept `impl Into<GameId>` allowing both `i64` and `GameId`
- Key methods by category:
  - **Schedule**: `daily_schedule()`, `weekly_schedule()`, `team_weekly_schedule()`, `daily_scores()`
  - **Standings**: `current_league_standings()`, `league_standings_for_date()`, `league_standings_for_season()`, `seasons()`
  - **Game**: `boxscore()`, `play_by_play()`, `game_landing()`, `game_story()`, `season_series()`, `shift_chart()`
  - **Player**: `player_landing()`, `player_game_log()`, `search_player()`
  - **Team**: `franchises()`, `roster_current()`, `roster_season()`, `club_stats()`, `club_stats_season()`

**HttpClient (`http_client.rs`)**
- Wraps `reqwest::Client` with NHL-specific configuration
- `Endpoint` enum defines API base URLs (ApiWebV1, ApiCore, ApiStats)
- `handle_response()` maps HTTP status codes to `NHLApiError` types
- `get_json()` performs GET requests and deserializes responses

**Types (`types/`)**
- Modular organization:
  - `common.rs` - LocalizedString, Team, Conference, Division, Franchise, Roster, RosterPlayer
  - `standings.rs` - Standing, StandingsResponse, SeasonInfo, SeasonsResponse
  - `schedule.rs` - ScheduleGame, GameDay, WeeklyScheduleResponse, DailySchedule, DailyScores, TeamScheduleResponse
  - `boxscore.rs` - Boxscore, BoxscoreTeam, SkaterStats, GoalieStats, PeriodDescriptor
  - `game_center.rs` - PlayByPlay, PlayEvent, GameMatchup, GameSummary, GameStory, ShiftChart
  - `game_state.rs` - GameState enum (FUT, PRE, LIVE, CRIT, FINAL, OFF)
  - `game_type.rs` - GameType enum (Preseason, RegularSeason, Playoffs, AllStar)
  - `player.rs` - PlayerLanding, PlayerGameLog, PlayerSearchResult, CareerTotals, Award
  - `club_stats.rs` - ClubStats, ClubSkaterStats, ClubGoalieStats
  - `enums.rs` - Position, PeriodType, Handedness, HomeRoad, ZoneCode, DefendingSide, GoalieDecision, GameScheduleState
- All types use serde derive macros with field renaming (e.g., `#[serde(rename = "teamName")]`)
- Types handle API evolution with `Option<T>` for fields that may not exist in all API versions

**Error Handling (`error.rs`)**
- Custom error types: `NHLApiError` enum with variants for different HTTP status codes
- Specific errors: ResourceNotFound (404), RateLimitExceeded (429), BadRequest (400), Unauthorized (401), ServerError (5xx)
- Uses `thiserror` for automatic Display/Error trait implementations

**Date/Time (`date.rs`)**
- `GameDate` enum: Either `Now` (for current date) or `Date(NaiveDate)`
- `Season` struct: Represents NHL seasons (e.g., 20232024)
- Implements FromStr for parsing, Display for API string formatting
- Key methods:
  - `GameDate::today()` - Creates a Date variant with today's date
  - `GameDate::to_api_string()` - Returns "now" or "YYYY-MM-DD"
  - `GameDate::add_days(n)` - Returns new GameDate offset by n days
  - `Season::current()` - Returns the current NHL season

**IDs (`ids.rs`)**
- `GameId` newtype wrapper around i64
- Implements Hash, Ord, FromStr, Display for convenient use in collections and APIs
- Implements `From<i64>` enabling `impl Into<GameId>` pattern in client methods

## Important Implementation Details

### API Data Variations

The NHL API returns different data structures based on context:

**Schedule Endpoint**: The `/schedule/{date}` endpoint returns a weekly schedule where:
- The response is a `WeeklyScheduleResponse` with a `gameWeek` array
- Individual game objects do NOT include `gameDate` field (date is at parent `GameDay` level)
- The `ScheduleGame.game_date` field is `Option<String>` to handle this

**Standings Endpoint**: Historical standings data (pre-1975) differs from modern data:
- Old data lacks `conferenceAbbrev` and `conferenceName` fields
- The `Standing` struct uses `Option<String>` for conference fields
- `Standing::to_team()` provides defaults ("UNK"/"Unknown") when conference is None

### Testing Strategy

- Every module has comprehensive unit tests in `#[cfg(test)] mod tests`
- Tests cover: happy paths, edge cases, error conditions, deserialization with missing fields
- Deserialization tests use JSON string literals matching actual API responses
- Test naming convention: `test_{component}_{scenario}` (e.g., `test_game_date_from_str_invalid`)

### Serde Patterns

All API response types follow these patterns:
- Use `#[serde(rename_all = "camelCase")]` at struct level for automatic field renaming
- Use `#[serde(rename = "fieldName")]` for individual fields when needed
- Use `#[serde(skip_serializing_if = "Option::is_none")]` for optional fields
- Use `#[serde(default)]` for fields that should default when missing
- Structs derive `Debug, Clone, Serialize, Deserialize, PartialEq`

### Common Pitfalls

1. **Optional Fields**: When adding new types, check if fields exist in ALL API responses. Historical data and different endpoints may omit fields.

2. **Display Implementations**: Many types implement `fmt::Display` for user-friendly output. When fields are optional, handle None gracefully.

3. **Date Formatting**: Always use `GameDate::to_api_string()` for API calls, not direct string formatting.

4. **Async**: All client methods are async. Tests don't need tokio runtime for non-async unit tests.

## API Endpoint Structure

The NHL API has four base URLs defined in `http_client.rs`:

**ApiWebV1** (`https://api-web.nhle.com/v1/`):
- `GET /standings/{date}` - Standings for a date ("now" or "YYYY-MM-DD")
- `GET /standings-season` - Season manifest with date ranges
- `GET /schedule/{date}` - Week schedule starting from date
- `GET /score/{date}` - Daily scores for a date
- `GET /gamecenter/{gameId}/boxscore` - Boxscore for specific game
- `GET /gamecenter/{gameId}/play-by-play` - Play-by-play data for specific game
- `GET /gamecenter/{gameId}/landing` - Game landing data (lighter than play-by-play)
- `GET /gamecenter/{gameId}/right-rail` - Season series matchup data
- `GET /wsc/game-story/{gameId}` - Game story narrative
- `GET /player/{playerId}/landing` - Player profile and stats
- `GET /player/{playerId}/game-log/{season}/{gameType}` - Player game log
- `GET /roster/{team}/current` - Current team roster
- `GET /roster/{team}/{season}` - Team roster for a season
- `GET /club-stats/{team}/{season}/{gameType}` - Club statistics
- `GET /club-stats-season/{team}` - Available seasons for club stats
- `GET /club-schedule/{team}/week/{date}` - Team weekly schedule

**SearchV1** (`https://search.d3.nhle.com/api/v1/`):
- `GET /search/player?culture=en-us&q={query}&limit={limit}` - Player search

**ApiCore** (`https://api.nhle.com/`):
- Currently unused in codebase

**ApiStats** (`https://api.nhle.com/stats/rest/`):
- `GET /en/franchise` - All NHL franchises
- `GET /en/shiftcharts?cayenneExp=gameId={id}` - Shift chart data for a game

## Requirements

- Rust 1.65 or later
- Use the tracing library for logging/debugging HTTP requests
- Use tracing at debug level to log http requests and responses for troubleshooting

## Reference Implementation

The Python NHL API client (https://github.com/coreyjs/nhl-api-py) serves as a reference for endpoint coverage and behavior.