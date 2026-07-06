# Backport Plan: nhl-api-go improvements → nhl-api (Rust)

Source of truth for the improvements: `~/code/puckdb-project/ws/nhl-api-go` (git history + `REVIEW.md`).
Target: this repo, `nhl_api` 0.7.1 → **0.8.0** (several breaking changes; 0.x semver signals breaking via minor bump).

**Goal:** port the Go version's bug fixes, robustness improvements, and new features (Edge stats,
ClubScheduleSeason) while staying Rust-idiomatic — no blind copying of Go idioms. Every step ships
with unit tests.

**Executor model per step** is tagged `[opus]` (API design, breaking changes, cross-cutting
decisions) or `[sonnet]` (mechanical porting, well-specified fixes, tests, docs).

---

## Ground rules for all steps

- Work on branch `backport-go-improvements` off `main`.
- After every step: `cargo check`, run the step's tests (`cargo test --lib <module>::`), then
  `cargo clippy -- -D warnings` and `cargo fmt`. Use LSP diagnostics after each edit.
- One commit per step, sequential git (no parallel git commands). No co-author lines.
- Test naming convention: `test_{component}_{scenario}`. Deserialization tests use JSON string
  literals matching real API responses (copy fixtures from the Go tests — they were validated
  against the live API).
- The Go repo is read-only reference. When this plan cites `Go file:line`, it refers to
  `~/code/puckdb-project/ws/nhl-api-go/nhl/` unless prefixed otherwise.

## Deliberate deviations from Go (Rust-idiomatic decisions — do NOT "fix" these to match Go)

1. **Keep the `thiserror` multi-variant `NHLApiError` enum.** Go consolidated 6 error types into
   one `APIError` + sentinel values because that is the Go idiom (`errors.Is`). Rust's enum with
   pattern matching is already the idiomatic equivalent. We only backport the *information*
   improvements (error body snippet, URL context in JSON errors).
2. **No Gob support.** Go added `GobEncode/GobDecode` for caching. The Rust equivalent is serde,
   which `GameDate`/`Season`/`GameId` currently *lack entirely* — adding `Serialize`/`Deserialize`
   to them is the backport (steps 1.1–1.3).
3. **Keep `search_player(query, limit: Option<i32>)`.** Go's variadic `limit ...int` is a Go-ism;
   `Option` is the Rust idiom. Only replace the magic `20` with a named constant (step 4.2).
4. **No `context.Context` equivalent.** Rust futures are cancellable by dropping; the client-wide
   timeout stays in `ClientConfig`.
5. **Empty-string enum values become `Option<T>` on the affected fields**, not a Go-style "zero
   value" variant. `None` is the honest model for "the API omitted this" (step 2.3).
6. **Season rollover boundary stays October** (`month < 10` → previous season,
   `src/date.rs:130`). Go uses a July boundary — that was port drift, never a reviewed fix. Only
   the Local→UTC change is backported (step 1.2). Flagging for Eric: if you prefer Go's July
   boundary (aligns with free-agency year flip), say so before Phase 1 lands.
7. **Strict enum parsing stays** (unknown values fail deserialization). The Go review evaluated
   leniency and deliberately kept strict parsing, improving error diagnosability instead. We port
   the diagnosability (shared typed error, step 2.1), not leniency.

---

## Phase 0 — Setup `[sonnet]`

**0.1** Create branch `backport-go-improvements`. Run `/check-all` (build, test, clippy, fmt) to
confirm a green baseline. Record the test count for later comparison.

---

## Phase 1 — Core value types (`date.rs`, `ids.rs`)

### 1.1 `Season` redesign `[opus]` — BREAKING

Current Rust problems (`src/date.rs`):
- Stores only `start_year` (`:78-82`); `from_years` debug-asserts `end == start+1` and discards
  the end year (`:91-98`) — `from_years(2004, 2004)` (World Cup 2004) panics in debug, silently
  becomes 2004–2005 in release.
- `parse()` rejects `"20042004"` (`:118-120`).
- `From<i32>`/`From<i64>` (`:144-158`) do unchecked `id/10000` with silent `u16` truncation and
  no validation.
- No numeric `id()` method; no serde support; `Display` and `to_api_string()` are conflated
  (both `"20232024"`).

Go reference: `date.go:223-303, 305-343, 349-426` (stores both years; `FromYears` accepts
`end == start` or `start+1`, errors otherwise; `ID()`; validated `SeasonFromInt`; dual string
formats; JSON accepts int and both string forms).

Changes:
- Store `start_year: u16` **and** `end_year: u16` (make fields private; add `start_year()` /
  `end_year()` accessors — flag this in the commit message, it breaks `season.start_year` access).
- `new(start_year)` keeps the `start+1` convention. `from_years(start, end) -> Result<Self, ...>`
  accepts `end == start` or `end == start + 1`, errors otherwise (introduce a
  `SeasonError`/`InvalidSeason` error via `thiserror`; no more `debug_assert`).
- `id(&self) -> i32` returning `start*10000 + end` (name it `id`, not `to_int`; add
  `From<Season> for i32/i64`).
- Replace `From<i32>`/`From<i64>` with `TryFrom<i32>`/`TryFrom<i64>`: range-check
  `10_000_000..=99_999_999`, route through `from_years` validation. BREAKING for callers using
  `.into()`.
- `parse`/`FromStr`: accept `"20232024"`, `"2023-2024"`, and single-year `"20042004"`.
- Split formats: `to_api_string()` stays `"20232024"`; change `Display` to the human
  `"2023-2004"`-style `"2023-2024"` (Go `String()`, `date.go:269-271`). BREAKING for anyone
  printing seasons — call this out in the changelog.
- Add `short_label()` → `"2023-24"` (Go `SeasonInfo.Label()` format, `standings.go:115-119`).
- serde: `Serialize` as integer `20232024`; `Deserialize` from integer OR string (both string
  forms), mirroring Go `date.go:364-395`. Use a custom `Deserialize` impl (untagged visitor).
- Audit all internal users of `Season` (client.rs, types) and update.

Tests: single-year construction/parse/round-trip (2004), COVID season `20202021` (cross-year
played in one calendar year — derived values must be identical to today's), `from_years`
rejection (gap years), `TryFrom` range rejection, serde int/string/`"2023-2024"` forms, `Display`
vs `to_api_string` split, `id()`.

### 1.2 UTC consistency + `GameDate` serde `[sonnet]`

- `GameDate::today()` (`src/date.rs:18`), `GameDate::as_date()` (`:42`), and `Season::current()`
  (`:127`) all use `chrono::Local` → switch to `chrono::Utc::now().date_naive()`. Machine-timezone
  independence; Go reference `date.go:127-130, 349-361`. Keep the October rollover boundary
  (deviation #6). Document the UTC choice on `Season::current()`.
- Add `Serialize`/`Deserialize` to `GameDate`: serialize as its `to_api_string()` form; deserialize
  via the existing `FromStr` (which already rejects month-13 etc. through chrono — the validation
  Go had to add in `date.go:177-199` comes free here; add the regression tests anyway:
  `"2024-13-40"`, `"2024-00-00"` rejected).

Tests: serde round-trip for `Now` and concrete dates, invalid-date rejection, plus a test that
`today()`/`current()` don't panic (avoid asserting wall-clock values — the Go review flagged
midnight-crossing flakes, `REVIEW.md` test-suite section).

### 1.3 Typed IDs: `PlayerId`, `TeamId` + serde `[sonnet]`

Current: only `GameId(i64)` (`src/ids.rs:6`), no serde. Go generates `GameID`, `PlayerID`,
`TeamID` with a uniform helper set (`ids_generated.go`, `id.go:11-38`): int-or-string JSON
deserialize, integer serialize, `FromString`.

Changes in `src/ids.rs`:
- Write a `macro_rules! numeric_id` generating, per ID type: the newtype over `i64`, `new`/`as_i64`
  (const), `From<i64>`, `From<Id> for i64`, `Display`, `FromStr`, `Hash/Ord/Eq/Copy` derives,
  and serde — `Serialize` as integer, `Deserialize` accepting integer or numeric string (mirror
  `unmarshalNumericID`, `id.go:11-29`).
- Instantiate `GameId` (keeping its exact current API — no breakage), `PlayerId`, `TeamId`.
- Skip Go's `Must*` panic constructors — `"123".parse::<PlayerId>().unwrap()` in tests is the
  Rust idiom.

Tests (per type, via the macro so write once parameterized or duplicated): construction, Display,
FromStr valid/invalid, serde from int, serde from string `"8478402"`, serde rejects non-numeric
string, serialize emits integer, ordering/hashing.

**Note:** adoption of these IDs inside response structs is Phase 5 (kept separate — it's a large
mechanical diff).

---

## Phase 2 — Enum infrastructure and variants

### 2.1 String-enum macro + shared unknown-value error `[opus]`

Go replaced 10 hand-written enums with a generator (`internal/enumgen/`) giving a uniform surface:
`FromString` (with aliases), `String`, `Code()`/`Name()`, `IsValid`, JSON marshal/unmarshal with
`AllowEmpty`, and a typed `UnknownEnumValueError{EnumType, Value}` recoverable via `errors.As`.

Rust-idiomatic equivalent — a declarative macro, not build-time codegen:
- New `src/types/enums/macros.rs` with `macro_rules! nhl_string_enum!` supporting, per enum:
  variants with canonical API string + optional parse aliases, optional `code()`/`name()` pairs,
  `Display` (raw code, name, or display-name — the three Go modes, `enumgen/main.go:104-189`),
  `FromStr` returning the shared error, `is_valid` is unnecessary in Rust (parsing is the
  validation), serde `Serialize`/`Deserialize` implemented via the canonical strings + aliases.
- Shared error in `src/error.rs` or `enums/mod.rs`:
  `UnknownEnumValue { enum_name: &'static str, value: String }` (thiserror; message format
  `invalid <enum_name>: "<value>"` like Go `enums.go:22-34`). All enum `FromStr` impls return it;
  serde `Deserialize` surfaces it through `serde::de::Error::custom(err)` (serde can't carry typed
  errors — the message keeps the type+value diagnosability, document that limitation).
- Migrate ONE enum (`PeriodType`, `src/types/enums/game_enums.rs:20-33`) to the macro in this step
  to prove the design; the rest migrate in 2.2.

Existing per-enum `ParseXError` structs (e.g. `ParsePositionError`, `player_enums.rs:14-17`) are
replaced by `UnknownEnumValue` — BREAKING for anyone matching those error types.

Tests: macro-generated enum round-trips (serde + FromStr), alias parsing, unknown value yields
`UnknownEnumValue` with correct `enum_name`/`value` from `FromStr`, and the serde error message
contains both.

### 2.2 Migrate remaining enums + missing variants `[sonnet]`

Migrate the string enums in `src/types/enums/` to the 2.1 macro, preserving each enum's current
canonical serialization exactly (existing tests must keep passing), and add the Go-side variants
Rust is missing:

| Enum | Add | Go ref |
|---|---|---|
| `Position` (`player_enums.rs:20-41`) | `Forward` = `"F"` (generic forward in historical games); include in `is_forward()` (`:67-72`) | `enums_generated.go:21`, commit `ae312bc` |
| `GoalieDecision` (`player_enums.rs:169-181`) | `Tie` = `"T"` (pre-shootout era); keep `"O"` as OvertimeLoss's canonical form, add `"OTL"` parse alias | `defs.go:73-74` |
| `GameScheduleState` (`game_enums.rs:254-270`) | `DontPlay` = `"DONT_PLAY"`, `Tbd` = `"TBD"`, `Completed` = `"COMPLETED"` | `defs.go:129-133` |

`PlayEventType` (`game_center.rs:13-43`) already has `#[serde(other)] Unknown` and parity with Go —
leave it where it is, don't force it into the macro if `serde(other)` doesn't fit.

Tests: each new variant serde round-trip + FromStr; historical-data JSON fixture per addition
(e.g. a roster entry with `"positionCode": "F"`, a game log with `"decision": "T"`).

### 2.3 Empty-string tolerance via `Option<T>` `[sonnet]` — BREAKING

The NHL API returns `""` for several enum fields on historical/unplayed data. Go's `AllowEmpty`
accepts `""` as the zero value (`enums_generated.go:102-117, 200-215, 385-400, 641-656`). Rust
currently **fails the whole deserialization** (confirmed: an unplayed season-series game with
`"periodType": ""` breaks `SeasonSeriesMatchup` entirely).

Rust-idiomatic fix (deviation #5): the affected *fields* become `Option<Enum>` with a shared
helper `fn empty_string_as_none<'de, D, T>(...)` (in `enums/mod.rs`) used via
`#[serde(deserialize_with = "...", default)]` — `""` or missing → `None`. Serialization: skip when
`None` (`skip_serializing_if = "Option::is_none"`).

Fields to change (grep for every use of each enum type to catch all — this list is the known
minimum):
- `PeriodType`: `PeriodDescriptor.period_type` (`boxscore.rs:74-75`), `GameOutcome.last_period_type`
  (`game_center.rs:281-282`). This fixes season-series for unplayed games (Go commit `f4e2f17`).
- `Position`: fields on historical roster data (Go commit `313205c`).
- `Handedness`: `shoots_catches`-style fields (Go commit `83cc32d`).
- `DefendingSide`: `home_team_defending_side`-style fields (Go commits `87701fe`/`4ccc0ff`).

Update any `Display`/helper code touching these fields to handle `None` gracefully (CLAUDE.md
pitfall #2).

Tests: per field — deserialize with `""`, with the field absent, and with a real value; a full
`SeasonSeriesMatchup` fixture containing an unplayed game (copy from Go's season-series tests);
serialize `None` omits the field.

### 2.4 `GameType` expansion `[sonnet]`

Current Rust (`src/types/game_type.rs`): 4 variants, int-only serde, no `FromStr`, no snake_case
label. Go (`game_type.go:14-42, 52-87, 161-196, 210-244`) has 15 variants, `Label()`, string
parsing, and string-tolerant unmarshal.

Changes:
- Add variants (code): `WorldCup` (6), `WorldCup2004` (7), `WorldCupPreTournament` (8),
  `Olympics` (9), `YoungStars` (10), `PwhlShowcase` (12), `LockoutLost` (13), `CanadaCup` (14),
  `ExhibitionOverseas` (18), `WomensAllStar` (19), `FourNations` (20). Extend
  `to_int`/`from_int`/`Display`.
- Add `label()` → snake_case (`"regular_season"`, `"world_cup_2004"`, `"four_nations"`, matching
  Go `Label()` exactly — puckdb uses these as DB enum values, so byte-for-byte parity matters).
- Add `FromStr` accepting: numeric strings, display names, and snake_case labels (Go
  `GameTypeFromString`), returning `UnknownEnumValue` from 2.1.
- `Deserialize`: accept int (primary) or string (numeric/label fallback) like Go
  `game_type.go:210-235`; `Serialize` stays int. Replace the bare
  `serde::de::Error::custom("Unknown game type")` (`game_type.rs:71`) with the
  `UnknownEnumValue`-formatted message.

Tests: every variant int round-trip, label() exact strings for all 15, FromStr from all three
input classes, string-JSON deserialize, unknown int/string rejection message contains value.

---

## Phase 3 — Bug fixes in existing types `[sonnet]` (all steps; independent, land in any order)

### 3.1 Remove bogus power-play aggregation — REAL LOGIC BUG

`src/types/boxscore.rs`: `aggregate_goalie_stats` does
`team_stats.power_play_opportunities += goalie.power_play_goals_against` (`:188`) — it counts PP
goals the goalie *allowed* as the team's *own* PP opportunities. Boxscore player stats contain no
team PP-opportunity data at all, so the value cannot be derived — remove rather than patch (Go
resolution, `REVIEW.md` bug #2):
- Delete `power_play_opportunities` field (`:137`), the derivation line (`:188`), and
  `power_play_percentage()` (`:200-206`).
- Delete/fix the tests asserting the wrong behavior (`:897, :951, :969`).
- Keep the legitimate goalie PIM aggregation and the skater-based `power_play_goals`.

### 3.2 Validate `GameSituation::from_code`

`src/types/game_center.rs:108-121`: only checks `len == 4`; `"a55b"` parses "successfully" with
both goalies treated as pulled; skater counts unchecked. Mirror Go `game_center.go:36-73`:
- All four chars must be ASCII digits; goalie flags must be exactly `0` or `1`; skater counts
  range-checked with named constants `MIN_SITUATION_SKATERS: u32 = 1`,
  `MAX_SITUATION_SKATERS: u32 = 6`. Return `None` otherwise.

Tests: `"a55b"` → None, `"1991"` → None (skaters out of range), `"2551"` → None (goalie flag 2),
`"0331"` OT-ish codes, the five standard codes (`"1551"`, `"1451"`, `"1541"`, `"0651"`, `"1560"`).

### 3.3 `reg_periods` consistency

`PlayByPlay.reg_periods` is `Option<i32>` (`game_center.rs:214-216`) while `GameMatchup`/`GameStory`
use plain `i32` (`:492-493`, `:877-878`). Align all three to `#[serde(default)] i32` (Go made all
plain int, `game_center.go:142`). Test: PlayByPlay JSON without `regPeriods` → 0; with → value.

### 3.4 `shootout`/`three_stars` de-Option

`GameSummary.shootout: Option<Vec<ShootoutAttempt>>` and `three_stars: Option<Vec<ThreeStar>>`
(`game_center.rs:527-531`) → `#[serde(default)] Vec<T>` (empty encodes absent), matching the
sibling `scoring`/`penalties` fields and Go `game_center.go:377-378`. Test: absent fields → empty
vecs.

### 3.5 Place-name reconstruction in `Standing::to_team()`

Rust `Team` (`common.rs:27-37`) has no place-name field; Go added one plus a reconstruction helper
(`standings.go:55-69`): strip the common name from the full name (handles start/end placement,
normalizes whitespace), fall back to the full name; empty/not-found common name → full name.
- Add `place_name: LocalizedString` to `Team` (with `#[serde(default)]` so existing JSON paths
  keep working), a private `fn place_name(full: &str, common: &str) -> String` in `standings.rs`,
  and populate it in `to_team()` (`standings.rs:52-68`). Audit other `Team` constructors
  (`client.rs teams()`) for the new field.

Tests: port Go's 8 `TestPlaceName` cases (start placement "Vegas Golden Knights"/"Golden Knights"
→ "Vegas"; end placement; fallback; empty common name) + `to_team()` assertions.

### 3.6 `ClubStats` season typing

`ClubStats.season: String` (`club_stats.rs:120-121`) → `Season`; `SeasonGameTypes.season: i32`
(`:131`) → `Season`. Depends on 1.1 (Season serde deserializing int and string forms — the API
returns both shapes here, which is exactly why Go typed it, `club_stats.go:85,93`). Tests: fixture
deserialization for both fields.

### 3.7 `Boxscore.game_schedule_state` enum typing

`boxscore.rs:32-33` is `String`; every sibling (`PlayByPlay`/`GameMatchup`/`GameStory`) already
uses the `GameScheduleState` enum. Change to the enum (Go `boxscore.go:17`). Depends on 2.2
(needs the three new variants so real data doesn't break). Test: boxscore fixture with `"OK"` and
one with `"CNCL"`.

### 3.8 `RosterPlayer` helper methods

`common.rs:82-113` has no impl block. Port from Go `common.go:125-177`, Rust-flavored:
- `full_name() -> String`, `birth_place() -> String` (join non-empty city/state/country with
  `", "` — iterator + `filter` + `collect::<Vec<_>>().join(", ")`, matching Go commit `086d499`),
  `height_feet_inches() -> String` (e.g. `6'2"`), `age(on: NaiveDate) -> Option<u32>` — take the
  reference date as a parameter instead of Go's implicit now (purity; callers pass
  `Utc::now().date_naive()`).

Tests: birth_place with all parts, missing state, city-only, none; height conversion; age around
birthday boundaries; full_name.

---

## Phase 4 — HTTP / config / client layer

### 4.1 Error body capture + JSON error context `[sonnet]`

`http_client.rs:86-93` drops the non-2xx response body unread; `error_from_status` (`:51-74`)
builds messages from the path only. Go reads a bounded body slice and appends it
(`client.go:38, 155-166`).
- `handle_response` becomes async: on non-2xx, read up to `MAX_ERROR_BODY_BYTES: usize = 4096`
  of the body (use `response.chunk()` loop or read full `bytes()` and truncate — bounded either
  way), trim, and append non-empty content to the error message:
  `"Request to {url} failed: {snippet}"`. Named constant, no magic number.
- Success path: replace `response.json::<T>()` (`:117`) with `response.text()` +
  `serde_json::from_str`, so deserialize failures can be wrapped with URL context (Go
  `client.go:175` wraps as `"unmarshaling response from {url}: ..."`). Extend
  `NHLApiError::JsonError` to carry the URL (or add a message wrapper variant) — keep the
  `#[from]` conversions where they still make sense.

Tests (mockito): 404 with a JSON error body → message contains the body snippet; body larger than
4096 → truncated; empty body → message unchanged shape; deserialize failure message contains the
request path.

### 4.2 Config surface: user agent, custom client, search-limit constant `[opus]`

Go added `WithUserAgent` (default fallback) and `WithHTTPClient` (escape hatch for
retry/instrumentation middleware) — `config.go:85-92`, applied at `client.go:141-146`. Rust
`ClientConfig` (`config.rs:3-18`) has neither, and `get_json` sends no `User-Agent`/`Accept`.

Design (opus finalizes the exact surface, keeping it minimal):
- Add `user_agent: Option<String>` to `ClientConfig`; default
  `concat!("nhl-api/", env!("CARGO_PKG_VERSION"))` as a named const. Set `User-Agent` and
  `Accept: application/json` on every request via `reqwest::ClientBuilder::default_headers`
  (or per-request — builder preferred).
- Add an escape hatch: `ClientConfig::with_http_client(reqwest::Client)` (field
  `client: Option<reqwest::Client>`); when present, `HttpClient::new` uses it as-is and the
  transport-shaping options (`timeout`, `ssl_verify`, `follow_redirects`) are documented as
  ignored — precedence identical to Go. This is the retry/backoff answer too: callers inject a
  middleware-wrapped client rather than the library baking in a retry policy (Go review's
  conclusion, `REVIEW.md` robustness §3).
- Consider making `ClientConfig` fields non-`pub(crate)` + builder-style methods while here
  (currently constructable only via `Default` from outside — verify and fix the ergonomics).
- `search_player` (`client.rs:273-281`): replace inline `unwrap_or(20)` with
  `const DEFAULT_SEARCH_LIMIT: i32 = 20`.

Tests (mockito): default UA header asserted on a request; custom UA; custom injected client is
actually used (e.g. mock requires a header only the injected client sets); Accept header;
search_player default limit hits `limit=20` query.

---

## Phase 5 — Typed-ID adoption in response structs `[sonnet]` — BREAKING, large mechanical diff

Adopt Phase 1.3 newtypes across response types, matching Go's structs: `id`/`game_id` fields →
`GameId`, `playerId`-renamed fields → `PlayerId`, team id fields → `TeamId`, and season fields
currently `i64`/`i32`/`String` → `Season` where they carry a season id (e.g. `Boxscore.id`,
`Boxscore.season` — `boxscore.rs:11` area; the earlier comparison found *no missing fields*, only
these scalar-vs-newtype deltas, so this is the whole remaining structural gap).

Method: file-by-file (boxscore, game_center, schedule, standings, player, club_stats, common),
compile + test after each file. Client method signatures already take `impl Into<GameId>` — extend
the same pattern to new `PlayerId`/`TeamId` parameters (`player_landing`, `roster_*`, etc.) so
`i64` call sites keep working.

Tests: existing deserialization tests updated in place prove serde compatibility (IDs deserialize
from the same integers); add one string-form test per adopted field family (the NHL API sometimes
returns numeric strings — the 1.3 deserializer handles both).

---

## Phase 6 — New endpoints

### 6.1 `club_schedule_season` `[sonnet]`

Go: `ClubScheduleSeason(ctx, teamAbbr, season)` → `GET club-schedule-season/{abbr}/{season}`
(`client.go:496`), reusing the team-schedule response shape. Port: types (check Go's return type —
likely shares `TeamScheduleResponse` in `schedule.go`), client method
`club_schedule_season(&self, team: &str, season: Season)`, endpoint doc in CLAUDE.md's endpoint
list. Tests: mockito path-contract test + fixture deserialization (copy Go's test fixture).

### 6.2 Edge stats — module design + shared building blocks `[opus]`

New: `src/types/edge/` (`mod.rs`, `common.rs`, `skater.rs`, `goalie.rs`, `team.rs`). ~84 structs,
22 client methods total (none exist in Rust today). Opus designs and lands the shared layer;
sonnet ports the domain files (6.3–6.5).

Shared types (`edge/common.rs`, from Go `edge.go`): `EdgeMeasurement` {imperial, metric: f64},
`EdgeMeasurementWithOverlay`, `EdgePercentileStat` (+WithOverlay), `EdgeCountLeagueAvg`,
`EdgeCountPercentileStat`, `EdgeRankStat` (team stats use rank 1–32, skater/goalie use percentile —
**do not share/unify these**), `EdgeRankStatWithOverlay`, `EdgeOverlay` (+Player/Team),
`EdgeSeasonAvailability`, `EdgeTeamLogo`, `EdgeTeamInfo`, `EdgeSkaterPlayer`, `EdgeGoaliePlayer`,
and the shared `EdgeComparison*` family + `EdgeLeaderShotLocation`. Reuses existing
`LocalizedString`, `PeriodDescriptor`, `GameOutcome` (all exist; `GameOutcome.last_period_type`
becomes `Option<PeriodType>` in 2.3 — fine here).

Critical design rule opus must establish and document in `edge/mod.rs`: **every Edge struct must
deserialize from `{}`** (the path-contract test pattern depends on it, and it matches Go's
zero-valuing). Concretely: scalar fields `#[serde(default)]`, genuinely-nullable fields
`Option<T>`. And the reverse rule from Go commit-history pain: plain scalar counts must NOT be
`Option` or `skip_serializing_if` — a legitimate `0` has to round-trip (Go's omitempty bug,
`REVIEW.md` robustness §6). `Option` is reserved for the nullable-pointer list below.

Known gotchas to encode as comments + tests (all pre-fixed in Go — port the *fixed* shapes):
- **A.** `EdgeGoalieSavePctgDetail.savePctgDetails` is an **object**, not an array:
  `Option<EdgeGoalieSavePctgStatDetail>` = `{gamesAbove900: Option<EdgeGoalieStatEntry>,
  pctgGamesAbove900: Option<EdgeGoalieStatEntry>}` (Go commit `cfedbf1`).
- **B.** `EdgeTeamZoneTimeDetails.shotDifferential` is a single object
  `{shotAttemptDifferential: f64, shotAttemptDifferentialRank: i32, sogDifferential: f64,
  sogDifferentialRank: i32}` — JSON key for the SOG field is `sogDifferential`. Parent's `team` and
  `seasonsWithEdgeStats` are `Option` here (unlike other top-levels). `EdgeTeamZoneTimeByStrength`
  carries optional `offensiveZoneLeagueAvg`/`neutralZoneLeagueAvg`/`defensiveZoneLeagueAvg`
  (Go commit `f3ada28`).
- **C.** Field naming split: detail payloads use JSON key `shots`; comparison + leader payloads
  use `sog` for the same concept. Do not unify — serde-rename per context. Also
  `EdgeComparisonDistanceLast10Entry` carries BOTH `distanceSkated` (skater) and `distance` (team)
  keys plus team-only `homeTeam`/`awayTeam` — all `Option`.
- Landing types: `leaders: HashMap<String, EdgeSkaterLeader>` (keyed by category), with
  mutually-exclusive `Option` stat fields — only one populated per category.

Nullable (`Option`) fields — exact list from the Go port: `EdgeMeasurementWithOverlay.overlay`,
`EdgeRankStat.league_avg`, `EdgeOverlay.game_outcome`, all leader stat fields, all comparison
detail sub-objects, gotcha A/B fields above.

Tests for this step: serde round-trip of each shared type; `{}`-deserializes test for each;
zero-value round-trip test (port `TestEdgeScalarZeroValueRoundTrips`, `edge_test.go:1011`).

### 6.3 Edge skater types + methods `[sonnet]`

Types (`edge/skater.rs`, from Go `edge.go`): `EdgeSkaterDetail`, `EdgeSkaterSpeedDetail`,
`EdgeSkaterDistanceDetail`, `EdgeSkaterShotSpeedDetail`, `EdgeSkaterShotLocationDetail`,
`EdgeSkaterZoneTimeDetail`, `EdgeSkaterComparison`, `EdgeSkaterLanding` + their entry structs.

Client methods (all `Endpoint::ApiWebV1`, params: player `impl Into<PlayerId>`, `Season`
(`to_api_string()`), `GameType` (`to_int()`)):

| Method | Path |
|---|---|
| `edge_skater_detail` | `edge/skater-detail/{p}/{s}/{gt}` |
| `edge_skater_speed_detail` | `edge/skater-skating-speed-detail/{p}/{s}/{gt}` |
| `edge_skater_distance_detail` | `edge/skater-skating-distance-detail/{p}/{s}/{gt}` |
| `edge_skater_shot_speed_detail` | `edge/skater-shot-speed-detail/{p}/{s}/{gt}` |
| `edge_skater_shot_location_detail` | `edge/skater-shot-location-detail/{p}/{s}/{gt}` |
| `edge_skater_zone_time` | `edge/skater-zone-time/{p}/{s}/{gt}` (no `-details` suffix) |
| `edge_skater_comparison` | `edge/skater-comparison/{p}/{s}/{gt}` |
| `edge_skater_landing` | `edge/skater-landing/{s}/{gt}` (no id) |

Note the double-"skating" slugs — they are correct; method names collapse them.
Tests: fixture deserialization per type (copy JSON fixtures from Go `edge_test.go` — they were
validated against the live API, including the `sog`-vs-`shots` fixture fix from `REVIEW.md`).

### 6.4 Edge goalie types + methods `[sonnet]`

Types (`edge/goalie.rs`, from Go `edge_goalie.go`): `EdgeGoalieDetail`, `EdgeGoalie5v5Detail`,
`EdgeGoalieShotLocationDetail`, `EdgeGoalieSavePctgDetail` (gotcha A), `EdgeGoalieComparison`,
`EdgeGoalieLanding` + entries.

| Method | Path |
|---|---|
| `edge_goalie_detail` | `edge/goalie-detail/{g}/{s}/{gt}` |
| `edge_goalie_5v5_detail` | `edge/goalie-5v5-detail/{g}/{s}/{gt}` |
| `edge_goalie_shot_location_detail` | `edge/goalie-shot-location-detail/{g}/{s}/{gt}` |
| `edge_goalie_save_pctg_detail` | `edge/goalie-save-percentage-detail/{g}/{s}/{gt}` (slug spelled out) |
| `edge_goalie_comparison` | `edge/goalie-comparison/{g}/{s}/{gt}` |
| `edge_goalie_landing` | `edge/goalie-landing/{s}/{gt}` |

Tests: fixture per type; the savePctgDetails-object fixture (Go `edge_test.go:617-700`) is
mandatory. Landing leader field tags must be exercised with a real fixture, not `{}` — the Go
review flagged that `{}`-only tests left those tags unverified (`REVIEW.md` test §3).

### 6.5 Edge team types + methods `[sonnet]`

Types (`edge/team.rs`, from Go `edge_team.go`): `EdgeTeamDetail`, `EdgeTeamSpeedDetail`,
`EdgeTeamDistanceDetail`, `EdgeTeamShotSpeedDetail`, `EdgeTeamShotLocationDetail`,
`EdgeTeamZoneTimeDetails` (gotcha B; distinct from the summary embedded in `EdgeTeamDetail`),
`EdgeTeamComparison`, `EdgeTeamLanding` + entries. Team stats are rank-based (`EdgeRankStat`).

| Method | Path |
|---|---|
| `edge_team_detail` | `edge/team-detail/{t}/{s}/{gt}` |
| `edge_team_speed_detail` | `edge/team-skating-speed-detail/{t}/{s}/{gt}` |
| `edge_team_distance_detail` | `edge/team-skating-distance-detail/{t}/{s}/{gt}` |
| `edge_team_shot_speed_detail` | `edge/team-shot-speed-detail/{t}/{s}/{gt}` |
| `edge_team_shot_location_detail` | `edge/team-shot-location-detail/{t}/{s}/{gt}` |
| `edge_team_zone_time_details` | `edge/team-zone-time-details/{t}/{s}/{gt}` (WITH `-details`) |
| `edge_team_comparison` | `edge/team-comparison/{t}/{s}/{gt}` |
| `edge_team_landing` | `edge/team-landing/{s}/{gt}` |

Tests: fixture per type; the shotDifferential fixture (Go `edge_test.go:900-915`) is mandatory.

### 6.6 Edge contract-test tables `[sonnet]`

Port Go's two table tests (`edge_client_methods_test.go`, the pattern the review called
"genuinely good") over all 22 methods using mockito + `Endpoint::Custom`:
1. **Path contract**: for each method, mock returns `200 {}`; assert the requested path exactly
   matches the expected `/edge/{slug}/[{id}/]{season}/{gt}` (fixtures: player `8478402`, team `22`,
   `Season::new(2024)` → `"20242025"`, `GameType::RegularSeason` → `"2"`). Catches slug typos and
   swapped id/season order. Requires the 6.2 `{}`-deserializes rule.
2. **404 propagation**: mock returns 404; assert every method returns
   `NHLApiError::ResourceNotFound`.

Implementation note: a `Vec<(name, expected_path, boxed async closure)>` or a macro generating one
`#[tokio::test]` per method — pick whichever reads better; the requirement is that adding a 23rd
Edge method without a table row is conspicuous.

---

## Phase 7 — Fixtures feature `[sonnet]`

Go ships production fixture constructors returning minimum-valid objects that survive
serialization (`fixtures.go`: `FixtureBoxscore`, `FixturePlayByPlay`, `FixtureGameStory`,
`FixtureShiftChart`, `FixtureSeasonSeriesMatchup`) — consumers (puckdb) use them in their tests.
Rust-idiomatic equivalent: a `fixtures` cargo feature (off by default):
- `Cargo.toml`: `[features] fixtures = []`.
- `src/fixtures.rs` behind `#[cfg(feature = "fixtures")]`: `pub fn boxscore() -> Boxscore`, etc.,
  seeding required enums (`GameType`/`GameState`/`GameScheduleState`/`PeriodType`) to valid values
  so `serde_json::to_string` succeeds.
- The existing `#[cfg(test)]` builders in `schedule.rs` (`:149, :191, :257`) stay as-is.

Tests: with the feature on, each fixture serializes and round-trips (a real round-trip — Go's
version only checked `len != 0`, flagged in review). CI note: run
`cargo test --features fixtures` in the final check.

---

## Phase 8 — Docs + release `[sonnet]`

- Update `CLAUDE.md`: endpoint list (+`club-schedule-season`, +22 Edge endpoints), types list
  (+edge module, ID newtypes, Season semantics), error-handling notes (body snippet,
  `UnknownEnumValue`), the new `fixtures` feature, and the serde-patterns section (the
  `empty_string_as_none` pattern, the Edge `{}`-deserializes rule).
- Update `README.md` if it lists methods.
- `Cargo.toml` version → `0.8.0`. Changelog (or release notes in the PR description) with an
  explicit **Breaking changes** section: `Season` field privacy + `From→TryFrom` + `Display`
  format, per-enum `ParseXError` → `UnknownEnumValue`, `Option`-ified enum fields (2.3), removed
  `power_play_opportunities`/`power_play_percentage`, `reg_periods`/`shootout`/`three_stars` type
  changes, ID newtype adoption, `ClubStats.season` type.
- Final gate: `/check-all` + `cargo test --features fixtures` + `cargo doc --no-deps` clean.
  Compare test count vs the Phase 0 baseline — it must be substantially higher.

---

## Execution order & dependencies

```
0.1 ─→ 1.1 ─→ 1.2 ─→ 1.3 ─→ 2.1 ─→ 2.2 ─→ 2.3 ─→ 2.4
                                             │
      3.1, 3.2, 3.3, 3.4, 3.5, 3.8  (independent; any time after 0.1)
      3.6 (needs 1.1)   3.7 (needs 2.2)
                                             │
      4.1, 4.2 (independent of phases 1–3)   │
                                             ▼
      5 (needs 1.1, 1.3) ─→ 6.1, 6.2 ─→ 6.3, 6.4, 6.5 (parallelizable) ─→ 6.6
                                             │
      7 (any time after 3.x)  ─→ 8 (last)
```

Suggested batching for execution sessions: (A) 0–1, (B) 2, (C) 3 + 4, (D) 5, (E) 6, (F) 7–8.
Commit per step; PR per batch is reasonable.

## Explicitly NOT ported (with reasons)

- Go error-type consolidation (`APIError` + sentinels) — Rust enum already idiomatic.
- `GobEncode/GobDecode` — serde derives are the equivalent (added in Phase 1).
- `context.Context` threading / `DefaultContext` — future-drop cancellation is the Rust model.
- Variadic `SearchPlayer` limit — `Option<i32>` stays.
- `http.DefaultTransport` cloning — reqwest manages its transport; N/A.
- Go's July season-rollover boundary (deviation #6 — October stays unless Eric overrides).
- `AllowUnknown` lenient enum mode — Go review evaluated and rejected it for the actual consumer;
  strict parsing stays on both sides.
- Go's `Must*` panic constructors — `.parse().unwrap()` in tests instead.
- CI drift guard for generated code — no codegen in the Rust design (declarative macro instead).
