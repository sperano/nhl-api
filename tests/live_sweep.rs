//! Opt-in checks against the live NHL API; never run by plain `cargo test`.
//!
//! - `live_sweep` calls every `Client` method across the matrix in
//!   `tests/common/matrix.rs` and reports which ones fail:
//!   `cargo nextest run --test live_sweep --run-ignored only live_sweep --no-capture`
//! - `capture_fixtures` re-records `tests/fixtures/<method>/<label>.json` from
//!   the live API, only when `NHL_API_CAPTURE_FIXTURES=1` is set.

mod common;

use common::{cases, Case, CurrentGames, Outcome};
use nhl_api::{Client, GameDate, NHLApiError};
use std::error::Error;
use std::fmt::Write as _;
use std::path::PathBuf;
use std::time::Duration;

/// Set to `1` to let `capture_fixtures` overwrite the recorded fixtures.
const CAPTURE_ENV: &str = "NHL_API_CAPTURE_FIXTURES";
/// Per-request timeout while recording, matching the library's default order
/// of magnitude so a stalled connection cannot hang the capture.
const CAPTURE_TIMEOUT: Duration = Duration::from_secs(30);
/// The `Accept` header every `Client` request sends.
const ACCEPT_JSON: &str = "application/json";

#[tokio::test]
#[ignore = "calls the live NHL API"]
async fn live_sweep() {
    let client = Client::new().expect("build client");
    let current = current_games(&client).await;
    let mut report = String::new();
    let mut unexpected = Vec::new();

    for case in cases(&current) {
        let Some(live) = &case.live else {
            writeln!(report, "skip   {} (no such game right now)", case.name).unwrap();
            continue;
        };
        let result = (live.fetch)(&client).await.map_err(|err| err.to_string());
        let outcome = Outcome::classify(&case.name, result);
        let line = describe(&case, &outcome);
        writeln!(report, "{line}").unwrap();
        if outcome.is_unexpected() {
            unexpected.push(line);
        }
    }

    println!("{current:?}\n{report}");
    assert!(
        unexpected.is_empty(),
        "{} unexpected live sweep result(s):\n{}",
        unexpected.len(),
        unexpected.join("\n")
    );
}

#[tokio::test]
#[ignore = "calls the live NHL API and rewrites tests/fixtures"]
async fn capture_fixtures() {
    if std::env::var(CAPTURE_ENV).as_deref() != Ok("1") {
        eprintln!("{CAPTURE_ENV} is not 1; not touching tests/fixtures");
        return;
    }
    let client = Client::new().expect("build client");
    let current = current_games(&client).await;
    let http = reqwest::Client::builder()
        .user_agent(nhl_api::DEFAULT_USER_AGENT)
        .timeout(CAPTURE_TIMEOUT)
        .build()
        .expect("build reqwest client");
    let mut failures = Vec::new();

    for case in cases(&current) {
        let Some(url) = case.live.as_ref().and_then(|live| live.url.as_ref()) else {
            continue;
        };
        match record(&http, &case, url).await {
            Ok(path) => println!("wrote  {}", path.display()),
            Err(err) => failures.push(format!("{}: {err}", case.name)),
        }
    }
    assert!(
        failures.is_empty(),
        "could not record:\n{}",
        failures.join("\n")
    );
}

/// Fetches `url` and writes the body, byte for byte, to the case's fixture.
async fn record(
    http: &reqwest::Client,
    case: &Case,
    url: &reqwest::Url,
) -> Result<PathBuf, Box<dyn Error>> {
    let response = http
        .get(url.clone())
        .header(reqwest::header::ACCEPT, ACCEPT_JSON)
        .send()
        .await?
        .error_for_status()?;
    let body = response.bytes().await?;
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(format!("{}.json", case.name));
    std::fs::create_dir_all(path.parent().expect("fixture path has a directory"))?;
    std::fs::write(&path, &body)?;
    Ok(path)
}

/// Finds a game not started yet and one in progress, if any.
async fn current_games(client: &Client) -> CurrentGames {
    let scheduled = client
        .weekly_schedule(Some(GameDate::Now))
        .await
        .map(|week| {
            week.game_week
                .into_iter()
                .flat_map(|day| day.games)
                .find(|game| game.game_state.is_scheduled())
                .map(|game| game.id)
        });
    let in_progress = client
        .daily_scores(Some(GameDate::Now))
        .await
        .map(|scores| {
            scores
                .games
                .into_iter()
                .find(|game| game.game_state.is_live())
                .map(|game| game.id)
        });
    CurrentGames {
        scheduled: found_or_warn("scheduled game", scheduled),
        in_progress: found_or_warn("game in progress", in_progress),
    }
}

fn found_or_warn<T>(what: &str, lookup: Result<Option<T>, NHLApiError>) -> Option<T> {
    lookup.unwrap_or_else(|err| {
        eprintln!("could not look up a {what}: {err}");
        None
    })
}

fn describe(case: &Case, outcome: &Outcome) -> String {
    match outcome {
        Outcome::Pass => format!("ok     {}", case.name),
        Outcome::KnownFailure => format!("known  {}", case.name),
        Outcome::KnownFailurePassed => {
            format!(
                "FIXED  {} passes: remove it from KNOWN_FAILURES (re-record its fixture first if \
                 the live response changed)",
                case.name
            )
        }
        Outcome::Failure(err) => format!("FAIL   {}: {err}", case.name),
    }
}
