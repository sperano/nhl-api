# Changelog

All notable changes to this project are documented in this file.

## Unreleased

### Breaking changes

- Game-center responses for scheduled (`FUT`) games now deserialize. Before puck drop the API
  omits team `score`/`sog`, the boxscore `clock` and `playerByGameStats`, and the top-level
  `periodDescriptor`, so `boxscore()`, `play_by_play()`, `landing()`, `season_series()` and
  `game_story()` previously failed with `missing field 'score'` (or `'sog'`); `season_series()`
  also failed for any game whose series still had unplayed games. `score` and `sog` on
  `BoxscoreTeam`, `MatchupTeam` and `StoryTeam`, and `score` on `SeriesTeam`, are now
  `Option<i32>` (`None` when absent, omitted when serialized); `Boxscore.clock` is now
  `Option<GameClock>`. `Boxscore.player_by_game_stats` (`PlayerByGameStats` and
  `TeamPlayerStats` now derive `Default`) and the `period_descriptor` of `Boxscore`,
  `PlayByPlay`, `GameMatchup` and `SeriesGame` default when absent.
- `Standing.division_abbrev` and `Standing.division_name` are now `Option<String>` because
  pre-division seasons previously made standings deserialization fail with
  `missing field 'divisionAbbrev'`; `Standing::to_team()` falls back to `"UNK"`/`"Unknown"`
  when either field is absent. Serialization omits `divisionAbbrev` and `divisionName` when they
  are absent. The new public `Standing.ties` field defaults to zero when absent, is included in
  `Standing::games_played()`, and is always serialized.
- `SeasonTotal.games_played` is now `Option<i32>` (`src/types/player.rs`). Some `seasonTotals`
  rows (minor-league and tournament seasons) carry only team and league, with no stats, so
  `player_landing()` failed for players such as Wayne Gretzky and Bobby Orr with
  `missing field 'gamesPlayed'`. Those rows now read `None`; a listed player who did not play
  still reads `Some(0)`. Serialization now omits `gamesPlayed` when it is `None`, instead of
  always emitting it.
- **`GameLog` is now an enum** (`src/types/game_log.rs`): `GameLog::Skater(SkaterGameLog)` or
  `GameLog::Goalie(GoalieGameLog)`. `player_game_log()` previously failed for every goalie with
  `missing field 'points'`, because the old `GameLog` struct required skater-only fields. The old
  struct's fields now live on `SkaterGameLog`; match on the variant, or use `as_skater()`/
  `as_goalie()`, to reach them. `game_id()`, `game_date()`, `team_abbrev()`, `opponent_abbrev()`,
  `home_road_flag()`, and `toi()` work on either variant. Serialization emits the inner entry
  unchanged (no variant tag).
- `ShiftEntry.duration` is now `Option<String>` and `ShiftEntry.event_number` is now
  `Option<i64>` (`src/types/game_center.rs`). The live API sends `null` for both on goal/penalty
  rows and, for older games, on ordinary shift rows too; `shift_chart()` previously failed
  deserialization entirely for every game that hit one of these rows.
- `SeriesGameInfo.referees` and `SeriesGameInfo.linesmen` are now `Vec<GameOfficial>` (was
  `Vec<LocalizedString>`). The right-rail response describes each official as
  `{ fullName, sweaterNumber }`, not a bare localized string, so `season_series()` previously
  failed with `missing field 'default'`. `GameOfficial` exposes `full_name` and
  `sweater_number`.
- `max_periods` on `PlayByPlay`, `GameMatchup` and `GameStory` is now `Option<i32>`
  (`src/types/game_center.rs`). Playoff responses have no top-level `maxPeriods` (playoff
  overtime is unlimited), so `play_by_play()`, `landing()` and `game_story()` previously failed
  for every playoff game with `missing field 'maxPeriods'`. It is `None` for those games; the
  regulation length stays available as `reg_periods` and `period_descriptor.max_regulation_periods`.

- **`TeamGameStats` faceoffs**: `faceoff_wins`/`faceoff_total` are replaced by
  `faceoffs: Option<FaceoffTotals>`, and `faceoff_percentage()` now returns `Option<f64>`.
  `from_team_player_stats()` used to estimate the totals from centers' shift counts, which gave
  wrong numbers (shifts are not faceoffs, and wingers take faceoffs too). The boxscore carries no
  faceoff counts, so `from_team_player_stats()` now leaves `faceoffs` as `None`; attach real
  totals with `with_faceoffs(play_by_play.faceoff_totals().home)` (or `.away`).
- `NHLApiError` is now `#[non_exhaustive]`: new variants (and new status-code mappings) can be
  added without that being a breaking change, so an exhaustive `match` on it without a wildcard
  arm no longer compiles.
- A `403` response is now `NHLApiError::Forbidden` instead of `ApiError`; code that matched
  `ApiError` to detect a `403` (by status code or message text) now needs to also match
  `Forbidden`.

### Added

- `PlayByPlay::faceoff_totals()` returns `GameFaceoffs { away, home }`, each a
  `FaceoffTotals { wins, total }` counted from the play-by-play `faceoff` events (the winner is the
  event's `eventOwnerTeamId`). For game 2024020444 it matches the right-rail `faceoffWins` category
  (13/35 away, 22/35 home).
- `GoalieGameLog`: `games_started`, `decision: Option<GoalieDecision>` (`None` when the goalie
  was not credited with the decision), `shots_against`, `goals_against`, `save_pctg`,
  `shutouts`, `goals`, `assists`, `toi`, `pim`.
- `NHLApiError::Forbidden` for a `403` response (previously fell into `ApiError` with an
  "Unexpected error" message). `NHLApiError::status_code() -> Option<u16>` returns the HTTP status
  code for every status-carrying variant, `None` for `RequestError`/`JsonError`/`Other`. Every
  variant now has rustdoc explaining when it occurs.

### Fixed

- `LocalizedString.default` is now `#[serde(default)]`, so a localized object that omits the
  `default` key deserializes to an empty string instead of failing the whole response.
- `SeriesGameInfo.referees`/`.linesmen` and `TeamGameInfo.scratches` now default to an empty vec
  when the API omits them.
- `landing()` failed with `invalid type: map, expected a string` for every completed game whose
  response lists its three stars: the landing response sends `threeStars[].name` as a localized
  object, while the game story sends a plain string. `ThreeStar.name` (still a `String`) now
  accepts both and keeps the `default` text.
- Status-code error messages (`NHLApiError::ResourceNotFound` and friends) now read
  `Request to {full_url} failed`, with the scheme and host, instead of the bare relative resource
  path (`get_json` was passing `resource` instead of the already-built `full_url` to
  `handle_response`).

### Testing

- `cargo test` no longer needs network access: the live-API tests in
  `tests/play_by_play_integration.rs` and `tests/player_game_log_integration.rs` are `#[ignore]`d.
- `tests/contract.rs` deserializes 98 responses recorded from the live API
  (`tests/fixtures/<method>/<label>.json`): scheduled and final games, regular season and
  playoffs, 1925/1975/2010/current games, skaters and goalies, empty schedule weeks, and every
  Edge endpoint. 34 of them fail today; `tests/common/known_failures.rs` pins each error so the
  suite turns red when one is fixed or changes. `tests/common/known_dropped.rs` likewise pins the
  fields that 16 responses silently drop although they deserialize (most Edge per-game speeds
  and distances, `PlayerLanding`'s `last5Games`).
- `tests/live_sweep.rs` (opt-in) calls every `Client` method against the same matrix and reports
  which fail; it also re-records the fixtures (`NHL_API_CAPTURE_FIXTURES=1`).
- Deterministic tests for the `PlayByPlay` helpers and for `GameMatchup`/`MatchupTeam` on recorded
  responses.

## 0.8.1

### Changed

- Removed the internal backport planning document from the repository and the
  published crate package. No code changes.

## 0.8.0

A large backport of robustness improvements, bug fixes, and new features
(Edge stats, `club_schedule_season`) developed against the Go port of this
client. Several changes are breaking; see below.

### Breaking changes

- **`Season` redesign** (`src/date.rs`):
  - `start_year`/`end_year` are now private; use the `start_year()`/`end_year()` accessors.
  - `From<i32>`/`From<i64>` are replaced by `TryFrom<i32>`/`TryFrom<i64>`, which validate the
    `YYYYYYYY` range and the start/end year relationship instead of silently truncating.
  - `Display` now renders the human `"2023-2024"` form. `to_api_string()` is unchanged
    (`"20232024"`) — the two are deliberately no longer the same string.
  - `parse()`/`FromStr` return `Result<Self, SeasonError>` instead of panicking or silently
    accepting invalid input; `SeasonError` is a new `thiserror` enum.
  - `from_years(start, end)` now returns `Result` instead of `debug_assert`-ing `end == start + 1`;
    it also accepts `end == start` (single-calendar-year seasons, e.g. the 2004 World Cup's
    `20042004`).
- **Enum parse errors unified**: the per-enum `ParseXError` types (e.g. `ParsePositionError`) are
  removed. All string-enum `FromStr` implementations now return the shared
  `types::enums::UnknownEnumValue { enum_name, value }`.
- **Enum fields become `Option<T>`** to tolerate the API's empty-string values on historical/
  unplayed data, instead of failing the whole deserialization:
  - `PeriodDescriptor.period_type: Option<PeriodType>`
  - `GameOutcome.last_period_type: Option<PeriodType>`
  - Position/Handedness/DefendingSide fields across roster, player, boxscore, and game-center types
- **`TeamGameStats.power_play_opportunities` and `power_play_percentage()` removed** — the field
  was computed from goalie-allowed power-play goals, which is not a valid source for the team's own
  power-play-opportunity count; boxscore data contains no field the value can be legitimately
  derived from.
- **`PlayByPlay.reg_periods`**: `Option<i32>` → `#[serde(default)] i32`, matching the plain-`i32`
  sibling fields on `GameMatchup`/`GameStory`. Missing field now decodes as `0` instead of `None`.
- **`GameSummary.shootout` / `GameSummary.three_stars`**: `Option<Vec<T>>` → `#[serde(default)]
  Vec<T>`, matching the sibling `scoring`/`penalties` fields. Absent field now decodes as an empty
  vec instead of `None`.
- **Typed-ID adoption**: `GameId`/`PlayerId`/`TeamId`/`Season` now appear in place of raw
  `i64`/`i32`/`String` across response structs (boxscore, game_center, schedule, standings, player,
  club_stats, common) and as client method parameters (`impl Into<PlayerId>` /
  `impl Into<TeamId>`, mirroring the existing `impl Into<GameId>` pattern). `i64` call sites keep
  working through the `Into` conversions, but code that pattern-matches on field types or calls
  `i64`-only methods on them will need updating.
- **`ClubStats.season`**: `String` → `Season`. **`SeasonGameTypes.season`**: `i32` → `Season`.
- **`Boxscore.game_schedule_state`**: `String` → `GameScheduleState` enum.
- **`NHLApiError::JsonError`** is now a struct variant (`{ url, source }`) instead of a plain
  `#[from] serde_json::Error` tuple variant, so it can carry the request URL. Code matching on the
  old tuple shape needs updating.

### Added

- `GameDate` and `Season` now implement `Serialize`/`Deserialize`; `Season` accepts an integer or
  either string form (`"20232024"` or `"2023-2024"`) on deserialize.
- `Season::id()`, `Season::short_label()` (`"2023-24"`), `Season::from_years()`.
- `GameDate::today()`, `GameDate::as_date()` (internal), and `Season::current()` now use UTC
  (`chrono::Utc::now()`) instead of the local system timezone. The October season-rollover
  boundary is unchanged.
- New `PlayerId` and `TeamId` newtypes alongside `GameId`, generated by a shared `numeric_id!`
  macro (`src/ids.rs`): serialize as an integer, deserialize from an integer or numeric string.
- `nhl_string_enum!` declarative macro (`src/types/enums/macros.rs`) generating a uniform
  code/name/`Display`/`FromStr`/serde surface for string-backed enums, with parse aliases and the
  shared `UnknownEnumValue` error.
- New enum variants: `Position::Forward` (`"F"`), `GoalieDecision::Tie` (`"T"`) plus an `"OTL"`
  parse alias for `OvertimeLoss`, and `GameScheduleState::DontPlay`/`Tbd`/`Completed`.
- `GameType` expanded from 4 to 15 variants (World Cup, World Cup 2004, World Cup Pre-Tournament,
  Olympics, Young Stars, PWHL Showcase, Lockout Lost, Canada Cup, Exhibition Overseas, Women's
  All-Star, Four Nations), plus `label()` (stable snake_case string) and `FromStr` (accepts
  numeric, display-name, or label form).
- `GameSituation::from_code` now validates all four characters are ASCII digits, goalie flags are
  `0`/`1`, and skater counts fall in `MIN_SITUATION_SKATERS..=MAX_SITUATION_SKATERS`, returning
  `None` for malformed codes instead of accepting them.
- `Team.place_name`, reconstructed from the full/common name pair in `Standing::to_team()`.
- `RosterPlayer` helper methods: `full_name()`, `birth_place()`, `height_feet_inches()`,
  `age(on: NaiveDate)`.
- Error responses now capture up to `MAX_ERROR_BODY_BYTES` (4096) of the response body and append
  it to the error message; JSON deserialization failures include the request URL.
- `ClientConfig` gains `with_user_agent()` (default `nhl-api/{version}`) and
  `with_http_client(reqwest::Client)` (escape hatch for retry/instrumentation middleware); every
  request now sends `User-Agent` and `Accept: application/json`. `ClientConfig` fields are now
  private, built via `default()` + chainable `with_*` methods.
- `DEFAULT_SEARCH_LIMIT` named constant backing `search_player`'s default limit (previously an
  inline magic `20`).
- `club_schedule_season(team, season)` client method (`GET club-schedule-season/{team}/{season}`).
- Full Edge stats support: `src/types/edge/` (~80 structs: shared measurement/rank/percentile/
  comparison types plus skater, goalie, and team domain types) and 22 new client methods
  (`edge_skater_*`, `edge_goalie_*`, `edge_team_*`) covering detail, speed, distance, shot-speed,
  shot-location, zone-time, comparison, and league-wide landing endpoints for each domain.
- `fixtures` cargo feature (off by default): `nhl_api::fixtures` exposes minimum-valid,
  round-trip-safe constructors (`boxscore()`, `play_by_play()`, `game_story()`, `shift_chart()`,
  `season_series_matchup()`) for downstream consumers' own tests.

### Fixed

- Season-series matchup data for unplayed games (empty-string `periodType`) no longer fails
  deserialization entirely.
- `GameSituation::from_code` no longer accepts malformed 4-character codes (non-digit characters,
  out-of-range goalie flags, or skater counts outside the valid range).
- The Edge skater/goalie/team domain types (`EdgeSkaterDetail`, `EdgeGoalieDetail`,
  `EdgeTeamDetail`, and ~55 related structs) are now re-exported from the crate root. They were
  previously only reachable through the private `types` module, so callers could not name the
  return type of any of the 22 `edge_*` client methods.
- `PlayEventDetails.zone_code` now tolerates an empty-string `zoneCode` (resolving to `None`)
  instead of failing deserialization.
