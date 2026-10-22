//! The request matrix shared by the contract tests (`tests/contract.rs`, which
//! deserialize recorded fixtures and need no network) and the opt-in live
//! sweep (`tests/live_sweep.rs`, which calls every `Client` method against the
//! real API and can re-record the fixtures).
//!
//! A [`Case`] is one `Client` method called with one set of arguments, named
//! `<method>/<label>`. Its recorded response, if any, lives in
//! `tests/fixtures/<method>/<label>.json`.

// Each test binary uses a different part of this module.
#![allow(dead_code)]

#[macro_use]
mod macros;
pub mod known_dropped;
pub mod known_failures;
mod matrix;

use nhl_api::{Client, GameId, NHLApiError};
use reqwest::Url;
use serde::de::DeserializeOwned;
use std::collections::BTreeSet;
use std::future::Future;
use std::pin::Pin;

pub use known_failures::{known_failure, KnownFailure};
pub use matrix::cases;

/// Bytes of fixture shown on each side of a deserialization error
/// (`serde_json` reports columns in bytes).
const ERROR_CONTEXT_BYTES: usize = 80;

pub type FetchFuture<'a> = Pin<Box<dyn Future<Output = Result<(), NHLApiError>> + 'a>>;

/// Calls one `Client` method and discards the deserialized value.
pub type Fetch = Box<dyn for<'a> Fn(&'a Client) -> FetchFuture<'a>>;

/// Paths of the fields a response has that its type ignores, such as
/// `hardestShots.[].shotSpeed`: `[]` stands for every element of an array,
/// and `Option`s and newtypes add no segment. A key containing `.` reads
/// like a deeper path; the API sends none.
pub type DroppedFields = BTreeSet<String>;

/// Deserializes a recorded response as the type the `Client` method returns,
/// reporting the fields that type dropped, except inside values a custom
/// `Deserialize` buffers through `serde_json::Value` (see `known_dropped`).
pub type Parse = fn(&str) -> Result<DroppedFields, serde_json::Error>;

/// How to run a case against the live API.
pub struct Live {
    /// The one URL the `Client` method requests, used to record its raw
    /// response; `None` for methods that make several requests.
    pub url: Option<Url>,
    pub fetch: Fetch,
}

pub struct Case {
    /// `<method>/<label>`, also the fixture path under `tests/fixtures/`.
    pub name: String,
    /// The recorded response, when one has been captured.
    pub fixture: Option<&'static str>,
    pub parse: Parse,
    /// `None` when the case cannot run right now (no game is in progress, or
    /// none is scheduled in the coming week).
    pub live: Option<Live>,
}

impl Case {
    pub fn new<T: DeserializeOwned>(
        name: impl Into<String>,
        fixture: Option<&'static str>,
        live: Option<Live>,
    ) -> Self {
        Self {
            name: name.into(),
            fixture,
            parse: parse_as::<T>,
            live,
        }
    }
}

/// The game-center responses recorded for one game; see `game_fixtures!`.
pub struct GameFixtures {
    pub boxscore: &'static str,
    pub play_by_play: &'static str,
    pub landing: &'static str,
    pub season_series: &'static str,
    pub game_story: &'static str,
    pub shift_chart: &'static str,
}

/// Games whose id is only known at run time.
#[derive(Debug, Default, Clone, Copy)]
pub struct CurrentGames {
    /// A game not started yet (`FUT`/`PRE`).
    pub scheduled: Option<GameId>,
    /// A game in progress (`LIVE`/`CRIT`).
    pub in_progress: Option<GameId>,
}

/// How a case's result compares with [`KNOWN_FAILURES`].
#[derive(Debug, PartialEq, Eq)]
pub enum Outcome {
    Pass,
    /// Failed, and is listed in [`KNOWN_FAILURES`] with this error.
    KnownFailure,
    /// Listed in [`KNOWN_FAILURES`] but passed: remove the entry.
    KnownFailurePassed,
    /// Failed with an error not listed in [`KNOWN_FAILURES`].
    Failure(String),
}

impl Outcome {
    pub fn classify(case: &str, result: Result<(), String>) -> Self {
        Self::classify_with(result, known_failure(case))
    }

    /// [`Self::classify`] against an explicit [`KNOWN_FAILURES`] entry.
    pub fn classify_with(result: Result<(), String>, known: Option<&KnownFailure>) -> Self {
        match (result, known) {
            (Ok(()), None) => Self::Pass,
            (Ok(()), Some(_)) => Self::KnownFailurePassed,
            (Err(err), Some(known)) if err.contains(known.error) => Self::KnownFailure,
            (Err(err), Some(known)) => Self::Failure(format!(
                "{err}\n  (listed in KNOWN_FAILURES as failing with `{}`)",
                known.error
            )),
            (Err(err), None) => Self::Failure(err),
        }
    }

    /// Whether this outcome needs a code or `KNOWN_FAILURES` change.
    pub fn is_unexpected(&self) -> bool {
        matches!(self, Self::KnownFailurePassed | Self::Failure(_))
    }
}

/// Boxes a closure calling a `Client` method; see the `call!` macro.
pub fn fetch(f: impl for<'a> Fn(&'a Client) -> FetchFuture<'a> + 'static) -> Fetch {
    Box::new(f)
}

pub fn parse_as<T: DeserializeOwned>(json: &str) -> Result<DroppedFields, serde_json::Error> {
    let mut dropped = DroppedFields::new();
    let mut deserializer = serde_json::Deserializer::from_str(json);
    serde_ignored::deserialize::<_, _, T>(&mut deserializer, |path| {
        dropped.insert(field_path(&path));
    })?;
    deserializer.end()?;
    Ok(dropped)
}

/// `path` in the [`DroppedFields`] notation.
fn field_path(path: &serde_ignored::Path) -> String {
    use serde_ignored::Path;
    let (parent, segment) = match path {
        Path::Root => return String::new(),
        Path::Seq { parent, .. } => (parent, "[]"),
        Path::Map { parent, key } => (parent, key.as_str()),
        Path::Some { parent }
        | Path::NewtypeStruct { parent }
        | Path::NewtypeVariant { parent } => return field_path(parent),
    };
    match field_path(parent) {
        prefix if prefix.is_empty() => segment.to_string(),
        prefix => format!("{prefix}.{segment}"),
    }
}

/// Formats `err` with the fixture text around the position it names, since
/// recorded responses are a single line of compact JSON.
pub fn describe_parse_error(json: &str, err: &serde_json::Error) -> String {
    let line = json.lines().nth(err.line().saturating_sub(1)).unwrap_or("");
    let column = err.column().min(line.len());
    let start = floor_char_boundary(line, column.saturating_sub(ERROR_CONTEXT_BYTES));
    let end = floor_char_boundary(line, (column + ERROR_CONTEXT_BYTES).min(line.len()));
    format!("{err}\n  near: …{}…", &line[start..end])
}

/// `str::floor_char_boundary` is only stable from Rust 1.91 (MSRV is 1.82).
fn floor_char_boundary(s: &str, mut index: usize) -> usize {
    while !s.is_char_boundary(index) {
        index -= 1;
    }
    index
}
