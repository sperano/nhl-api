//! NHL Edge stats types.
//!
//! The Edge endpoints (`/v1/edge/...`) expose puck- and player-tracking
//! statistics for skaters, goalies, and teams. This module holds the shared
//! building blocks in [`common`]; the domain-specific response types live in
//! the `skater`, `goalie`, and `team` submodules.
//!
//! # Design rules (read before adding or porting any Edge type)
//!
//! These two rules are load-bearing. The endpoint path-contract tests mock a
//! `200 {}` body, and puckdb caches these structs, so both directions matter.
//!
//! ## Rule 1 — every Edge struct must deserialize from `{}`
//!
//! An empty JSON object must produce the type's [`Default`] value. Concretely,
//! every struct here:
//!
//! - derives [`Default`] and carries `#[serde(default, rename_all =
//!   "camelCase")]` at the container level, so any missing field falls back to
//!   its default rather than failing deserialization;
//! - uses plain scalar fields (`i32`, `f64`, `String`, `bool`, `Vec<T>`, and
//!   nested Edge structs that themselves satisfy this rule) for values the API
//!   always sends;
//! - uses `Option<T>` **only** for the genuinely-nullable fields listed below.
//!
//! ## Rule 2 — plain scalar counts must round-trip a legitimate `0`
//!
//! A count that is legitimately `0` has to serialize back out as `0`, not be
//! dropped (this was a real omitempty bug in the Go port). Therefore plain
//! scalar counts are **never** `Option` and **never** carry
//! `skip_serializing_if`. `Option` (with `skip_serializing_if =
//! "Option::is_none"`) is reserved for the nullable-pointer fields:
//!
//! - [`common::EdgeMeasurementWithOverlay::overlay`] (and the `overlay` on the
//!   other `*WithOverlay` stats);
//! - [`common::EdgeRankStat::league_avg`];
//! - [`common::EdgeOverlay::game_outcome`];
//! - every leader stat field ([`common::EdgeLeaderShotLocation`] fields);
//! - every comparison detail sub-object (the `Option<Edge*Details>` /
//!   `Option<EdgeMeasurement*>` fields on the `EdgeComparison*` types);
//! - the gotcha A/B nullable fields in the goalie/team domain modules.
//!
//! # Field-naming gotchas (do not "clean up" by unifying)
//!
//! - **Gotcha C — `shots` vs `sog`.** Skater/goalie *detail* payloads name the
//!   shots-on-goal field `shots`; *comparison* and *leader* payloads name the
//!   same concept `sog`. These are serde-renamed per context and must stay
//!   distinct. [`common::EdgeComparisonDistanceLast10Entry`] additionally
//!   carries both a skater `distanceSkated` key and a team `distance` key (plus
//!   team-only `homeTeam`/`awayTeam`), all `Option`, so one struct decodes both
//!   shapes.
//! - **Rank vs percentile.** Team stats rank 1–32 ([`common::EdgeRankStat`]);
//!   skater/goalie stats use percentiles ([`common::EdgeCountPercentileStat`]).
//!   They are deliberately separate types.
//!
//! Domain modules (`skater`/`goalie`/`team`) follow the same rules; see gotchas
//! A (`savePctgDetails` is an object, not an array) and B
//! (`shotDifferential` single object; nullable top-level `team` /
//! `seasonsWithEdgeStats`) documented alongside the affected types there.

pub mod common;
pub mod goalie;
pub mod skater;
