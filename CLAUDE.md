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
- Game methods accept `impl Into<GameId>`, player methods `impl Into<PlayerId>`, team-id methods
  `impl Into<TeamId>` — all allowing either the newtype or a plain `i64` call site
- Key methods by category:
  - **Schedule**: `daily_schedule()`, `weekly_schedule()`, `team_weekly_schedule()`, `club_schedule_season()`, `daily_scores()`
  - **Standings**: `current_league_standings()`, `league_standings_for_date()`, `league_standings_for_season()`, `season_standing_manifest()`
  - **Game**: `boxscore()`, `play_by_play()`, `landing()`, `game_story()`, `season_series()`, `shift_chart()`
  - **Player**: `player_landing()`, `player_game_log()`, `search_player()`
  - **Team**: `franchises()`, `roster_current()`, `roster_season()`, `club_stats()`, `club_stats_season()`
  - **Edge stats** (`/v1/edge/...`, 22 methods): per-skater/goalie/team `_detail`, `_speed_detail`,
    `_distance_detail` (skater/team only), `_shot_speed_detail`, `_shot_location_detail`,
    `_zone_time`/`_zone_time_details`, `_comparison`, and a no-id `_landing` leaderboard for each of
    the three domains (`edge_skater_*`, `edge_goalie_*`, `edge_team_*`) — see the endpoint list below
    for exact path slugs

**HttpClient (`http_client.rs`)**
- Wraps `reqwest::Client` with NHL-specific configuration
- `Endpoint` enum defines API base URLs (ApiWebV1, ApiCore, ApiStats)
- `handle_response()` maps HTTP status codes to `NHLApiError` types
- `get_json()` performs GET requests and deserializes responses

**Types (`types/`)**
- Modular organization:
  - `common.rs` - LocalizedString, Team (incl. `place_name`), Conference, Division, Franchise, Roster,
    RosterPlayer (with `full_name()`/`birth_place()`/`height_feet_inches()`/`age()` helpers)
  - `standings.rs` - Standing, StandingsResponse, SeasonInfo, SeasonsResponse
  - `schedule.rs` - ScheduleGame, GameDay, WeeklyScheduleResponse, DailySchedule, DailyScores, TeamScheduleResponse
  - `boxscore.rs` - Boxscore, BoxscoreTeam, SkaterStats, GoalieStats, PeriodDescriptor
  - `game_center.rs` - PlayByPlay, PlayEvent, GameMatchup, GameSummary, GameStory, ShiftChart
  - `game_state.rs` - GameState enum (FUT, PRE, LIVE, CRIT, FINAL, OFF)
  - `game_type.rs` - GameType enum, 15 variants (regular/playoffs/preseason/all-star plus World Cup,
    Olympics, Young Stars, PWHL Showcase, Lockout, Canada Cup, exhibition-overseas, women's all-star,
    Four Nations), with `label()` (snake_case) and `FromStr` (numeric/display-name/label)
  - `player.rs` - PlayerLanding, PlayerGameLog, PlayerSearchResult, CareerTotals, Award
  - `club_stats.rs` - ClubStats (`season: Season`), SeasonGameTypes (`season: Season`),
    ClubSkaterStats, ClubGoalieStats
  - `edge/` - Edge puck/player-tracking stats (`common.rs`, `skater.rs`, `goalie.rs`, `team.rs`,
    ~80 structs). Every Edge struct deserializes from `{}` (all fields default; nullability is
    reserved for genuinely-optional pointers) — see `edge/mod.rs` module docs for the full rule set
    and the field-naming gotchas (`shots` vs `sog`, `savePctgDetails` as an object not an array,
    `shotDifferential` as a single nested object)
  - `enums/` (`mod.rs`, `game_enums.rs`, `player_enums.rs`, `macros.rs`) - Position, PeriodType,
    Handedness, HomeRoad, ZoneCode, DefendingSide, GoalieDecision, GameScheduleState, all generated
    by the `nhl_string_enum!` macro (see Serde Patterns below)
- All types use serde derive macros with field renaming (e.g., `#[serde(rename = "teamName")]`)
- Types handle API evolution with `Option<T>` for fields that may not exist in all API versions
- ID fields use the typed newtypes (`GameId`/`PlayerId`/`TeamId`) rather than raw `i64` throughout
  response structs; season-id fields use `Season` rather than raw `i32`/`String`

**Error Handling (`error.rs`)**
- Custom error types: `NHLApiError` enum with variants for different HTTP status codes
- Specific errors: ResourceNotFound (404), RateLimitExceeded (429), BadRequest (400), Unauthorized (401), ServerError (5xx)
- Non-2xx responses capture up to `MAX_ERROR_BODY_BYTES` (4096, `http_client.rs`) of the response
  body and append it to the error message (`"Request to {url} failed: {snippet}"`)
- `JsonError` is a struct variant carrying both the request `url` and the source `serde_json::Error`,
  so deserialize-failure messages read `"unmarshaling response from {url}: {source}"`
- Enum deserialization failures (unknown string values) surface as `UnknownEnumValue` — see Serde
  Patterns below — either as a typed error from `FromStr` or, at the serde boundary, as a
  `serde::de::Error::custom` message containing both the enum name and offending value (the
  per-enum `ParseXError` types this replaced are gone)
- Uses `thiserror` for automatic Display/Error trait implementations

**Config (`config.rs`)**
- `ClientConfig` fields are private; construct via `ClientConfig::default()` and chain
  `with_timeout()`, `with_ssl_verify()`, `with_follow_redirects()`, `with_user_agent()`,
  `with_http_client()`
- Every request sends `User-Agent` (default `DEFAULT_USER_AGENT`, i.e.
  `concat!("nhl-api/", env!("CARGO_PKG_VERSION"))`, overridable) and `Accept: application/json`
- `with_http_client(reqwest::Client)` is an escape hatch for retry/instrumentation middleware; when
  set, the other transport options and default headers are ignored — the injected client's
  configuration wins

**Date/Time (`date.rs`)**
- `GameDate` enum: Either `Now` (for current date) or `Date(NaiveDate)`; `Serialize`/`Deserialize` as
  its `to_api_string()` form
- `Season` struct: private `start_year`/`end_year: u16` fields (use the `start_year()`/`end_year()`
  accessors, not direct field access) — supports single-calendar-year seasons (e.g. `20042004`, the
  2004 World Cup) as well as the conventional cross-year form
  - `id() -> i32` returns the `YYYYYYYY` integer; `to_api_string()` stays `"20232024"`; `Display`
    is the human `"2023-2024"` form (these two are now deliberately different — do not conflate them)
  - `short_label()` returns `"2023-24"`
  - `from_years(start, end) -> Result<Self, SeasonError>` accepts `end == start` or
    `end == start + 1`, otherwise errors (no more silent/`debug_assert` truncation)
  - `TryFrom<i32>`/`TryFrom<i64>` replace the old unchecked `From` impls — range-checked and routed
    through `from_years`
  - `parse()`/`FromStr` accept `"20232024"`, `"2023-2024"`, and single-year forms; return
    `SeasonError`, not `()`
  - serde: serializes as an integer; deserializes from an integer or either string form
- `GameDate::today()`, `GameDate::as_date()`, and `Season::current()` use `chrono::Utc::now()`, not
  `Local` — machine-timezone independent. The October season-rollover boundary is unchanged.
- Key methods:
  - `GameDate::today()` - Creates a Date variant with today's date (UTC)
  - `GameDate::to_api_string()` - Returns "now" or "YYYY-MM-DD"
  - `GameDate::add_days(n)` - Returns new GameDate offset by n days
  - `Season::current()` - Returns the current NHL season (UTC-based)

**IDs (`ids.rs`)**
- `GameId`, `PlayerId`, `TeamId` — all generated by the `numeric_id!` macro: newtype wrapper over
  `i64` with `new`/`as_i64`, `From<i64>`/`From<Id> for i64`, `Display`, `FromStr`,
  `Hash`/`Ord`/`Eq`/`Copy`/`Default`, and serde (serializes as an integer; deserializes from either
  an integer or a numeric string)
- Adopted throughout response structs (Phase 5): `id`/`gameId`-style fields use `GameId`,
  `playerId`-style fields use `PlayerId`, team id fields use `TeamId`
- Client methods accept `impl Into<GameId>` / `impl Into<PlayerId>` / `impl Into<TeamId>`, so `i64`
  call sites keep working alongside the typed newtype

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
- `fixtures` cargo feature (off by default): `src/fixtures.rs`, gated behind
  `#[cfg(feature = "fixtures")]`, exposes minimum-valid constructors (`boxscore()`, `play_by_play()`,
  `game_story()`, `shift_chart()`, `season_series_matchup()`) that round-trip through
  `serde_json` — for downstream consumers' own tests, not part of the core API surface. Run
  `cargo test --features fixtures` to exercise it.

### Serde Patterns

All API response types follow these patterns:
- Use `#[serde(rename_all = "camelCase")]` at struct level for automatic field renaming
- Use `#[serde(rename = "fieldName")]` for individual fields when needed
- Use `#[serde(skip_serializing_if = "Option::is_none")]` for optional fields
- Use `#[serde(default)]` for fields that should default when missing
- Structs derive `Debug, Clone, Serialize, Deserialize, PartialEq`
- **String enums**: generated by the `nhl_string_enum!` macro (`src/types/enums/macros.rs`) rather
  than hand-written — gives every enum a canonical code, optional name, `Display` (code/name/
  display-name mode), `FromStr` with parse aliases, and serde routed through `FromStr`. Unknown
  values return the shared `UnknownEnumValue` error, not a per-enum `ParseXError`.
- **Empty-string enum tolerance**: fields where the API sends `""` for historical/unplayed data
  (e.g. `PeriodDescriptor.period_type`, `GameOutcome.last_period_type`) are `Option<Enum>` with
  `#[serde(deserialize_with = "empty_string_as_none", default)]` plus
  `#[serde(skip_serializing_if = "Option::is_none")]` — `""` or a missing field become `None`; any
  other value still goes through the enum's `FromStr` so genuinely unknown values keep failing
  loudly. The helper lives in `src/types/enums/mod.rs`.
- **Edge `{}`-deserializes rule**: every struct in `src/types/edge/` must deserialize from an empty
  JSON object (`#[serde(default, rename_all = "camelCase")]` + `Default` derive + plain scalar
  fields), with `Option<T>` reserved for genuinely-nullable fields — plain scalar counts are never
  `Option`/`skip_serializing_if` (a legitimate `0` must round-trip). See `src/types/edge/mod.rs` for
  the full rule set and naming gotchas.
- **ID/season macros**: `numeric_id!` (`src/ids.rs`) generates `GameId`/`PlayerId`/`TeamId`; `Season`
  (`src/date.rs`) has a hand-written serde impl accepting int or either string form — see the
  Date/Time and IDs sections above.

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
- `GET /club-schedule-season/{team}/{season}` - Team schedule for a full season

Edge stats (player/puck tracking), all under `/edge/...`, params `{p}`=`PlayerId`, `{t}`=`TeamId`,
`{s}`=`Season::to_api_string()`, `{gt}`=`GameType::to_int()`. Skater and goalie stats are
percentile-based; team stats are rank-based (1-32).

- `GET /edge/skater-detail/{p}/{s}/{gt}` - `edge_skater_detail`
- `GET /edge/skater-skating-speed-detail/{p}/{s}/{gt}` - `edge_skater_speed_detail`
- `GET /edge/skater-skating-distance-detail/{p}/{s}/{gt}` - `edge_skater_distance_detail`
- `GET /edge/skater-shot-speed-detail/{p}/{s}/{gt}` - `edge_skater_shot_speed_detail`
- `GET /edge/skater-shot-location-detail/{p}/{s}/{gt}` - `edge_skater_shot_location_detail`
- `GET /edge/skater-zone-time/{p}/{s}/{gt}` - `edge_skater_zone_time` (no `-detail(s)` suffix)
- `GET /edge/skater-comparison/{p}/{s}/{gt}` - `edge_skater_comparison`
- `GET /edge/skater-landing/{s}/{gt}` - `edge_skater_landing` (league-wide leaderboard, no id)
- `GET /edge/goalie-detail/{p}/{s}/{gt}` - `edge_goalie_detail`
- `GET /edge/goalie-5v5-detail/{p}/{s}/{gt}` - `edge_goalie_5v5_detail`
- `GET /edge/goalie-shot-location-detail/{p}/{s}/{gt}` - `edge_goalie_shot_location_detail`
- `GET /edge/goalie-save-percentage-detail/{p}/{s}/{gt}` - `edge_goalie_save_pctg_detail` (slug
  spelled out, not abbreviated)
- `GET /edge/goalie-comparison/{p}/{s}/{gt}` - `edge_goalie_comparison`
- `GET /edge/goalie-landing/{s}/{gt}` - `edge_goalie_landing`
- `GET /edge/team-detail/{t}/{s}/{gt}` - `edge_team_detail`
- `GET /edge/team-skating-speed-detail/{t}/{s}/{gt}` - `edge_team_speed_detail`
- `GET /edge/team-skating-distance-detail/{t}/{s}/{gt}` - `edge_team_distance_detail`
- `GET /edge/team-shot-speed-detail/{t}/{s}/{gt}` - `edge_team_shot_speed_detail`
- `GET /edge/team-shot-location-detail/{t}/{s}/{gt}` - `edge_team_shot_location_detail`
- `GET /edge/team-zone-time-details/{t}/{s}/{gt}` - `edge_team_zone_time_details` (WITH `-details`,
  unlike the skater equivalent)
- `GET /edge/team-comparison/{t}/{s}/{gt}` - `edge_team_comparison`
- `GET /edge/team-landing/{s}/{gt}` - `edge_team_landing`

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