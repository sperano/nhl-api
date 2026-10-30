//! Path contract for every non-Edge `Client` method: each case drives the
//! public method against a mock server (see `Client::with_origin_override`)
//! and checks the exact path, base path and query it requests. The Edge
//! methods have their own table in `edge_contract_test.rs`.

use std::future::Future;
use std::pin::Pin;

use chrono::NaiveDate;
use mockito::Matcher;

use super::super::{Client, DEFAULT_SEARCH_LIMIT};
use crate::date::{GameDate, Season};
use crate::error::NHLApiError;
use crate::ids::{GameId, PlayerId};
use crate::types::{GameType, WeeklyScheduleResponse};

type MethodFuture<'a> = Pin<Box<dyn Future<Output = Result<(), NHLApiError>> + 'a>>;
type MethodFn = for<'a> fn(&'a Client) -> MethodFuture<'a>;

/// One request a method is expected to send, and the body the mock answers.
struct Request {
    path: Matcher,
    query: Matcher,
    body: &'static str,
}

struct MethodCase {
    name: &'static str,
    requests: Vec<Request>,
    call: MethodFn,
}

const GAME_ID: i64 = 2025020001;
const PLAYER_ID: i64 = 8478402;
const TEAM: &str = "MTL";
const GAME_TYPE: GameType = GameType::RegularSeason;
/// `GAME_TYPE.to_int()`, spelled out so the paths below are literal.
const GAME_TYPE_ID: i32 = 2;
const SEARCH_QUERY: &str = "mcdavid";
/// Path of a request for a `GameDate::today()` default (date varies per run).
const TODAY_DATE_PATTERN: &str = r"\d{4}-\d{2}-\d{2}$";
/// `standingsEnd` of 2024-25 in the recorded season manifest.
const SEASON_2024_STANDINGS_END: &str = "2025-04-17";

const STANDINGS: &str =
    include_str!("../../tests/fixtures/league_standings_for_date/2026-10-21.json");
const SEASON_MANIFEST: &str =
    include_str!("../../tests/fixtures/season_standing_manifest/all.json");
const BOXSCORE: &str = include_str!("../../tests/fixtures/boxscore/fut.json");
const PLAY_BY_PLAY: &str = include_str!("../../tests/fixtures/play_by_play/fut.json");
const LANDING: &str = include_str!("../../tests/fixtures/landing/fut.json");
const SEASON_SERIES: &str = include_str!("../../tests/fixtures/season_series/fut.json");
const GAME_STORY: &str = include_str!("../../tests/fixtures/game_story/fut.json");
const SHIFT_CHART: &str = include_str!("../../tests/fixtures/shift_chart/fut.json");
/// Week starting 2026-10-21 (seven game days).
const WEEKLY_SCHEDULE: &str = include_str!("../../tests/fixtures/weekly_schedule/2026-10-21.json");
const DAILY_SCORES: &str = include_str!("../../tests/fixtures/daily_scores/2026-10-21.json");
const TEAM_WEEK: &str =
    include_str!("../../tests/fixtures/team_weekly_schedule/mtl_2026-10-21.json");
const TEAM_SEASON_SCHEDULE: &str =
    include_str!("../../tests/fixtures/club_schedule_season/mtl_20252026.json");
const PLAYER_LANDING: &str = include_str!("../../tests/fixtures/player_landing/mcdavid.json");
const PLAYER_GAME_LOG: &str =
    include_str!("../../tests/fixtures/player_game_log/mcdavid_20252026_regular.json");
const SEARCH_RESULTS: &str = include_str!("../../tests/fixtures/search_player/mcdavid.json");
const FRANCHISES: &str = include_str!("../../tests/fixtures/franchises/all.json");
const CLUB_STATS: &str = include_str!("../../tests/fixtures/club_stats/mtl_20252026_regular.json");
const CLUB_STATS_SEASON: &str = include_str!("../../tests/fixtures/club_stats_season/mtl.json");
const ROSTER_CURRENT: &str = include_str!("../../tests/fixtures/roster_current/mtl.json");
const ROSTER_SEASON: &str = include_str!("../../tests/fixtures/roster_season/mtl_20102011.json");

fn date() -> GameDate {
    GameDate::Date(NaiveDate::from_ymd_opt(2026, 10, 21).unwrap())
}

fn season() -> Season {
    Season::new(2025)
}

/// Turns a call on `client` into a [`MethodFn`] discarding the response.
macro_rules! call {
    ($client:ident => $call:expr) => {{
        fn call($client: &Client) -> MethodFuture<'_> {
            Box::pin(async move { $call.await.map(|_| ()) })
        }
        call as MethodFn
    }};
}

fn request(path: impl Into<Matcher>, body: &'static str) -> Request {
    Request {
        path: path.into(),
        query: Matcher::Any,
        body,
    }
}

fn today_path(prefix: &str) -> Matcher {
    Matcher::Regex(format!("^{prefix}{TODAY_DATE_PATTERN}"))
}

fn case(name: &'static str, request: Request, call: MethodFn) -> MethodCase {
    MethodCase {
        name,
        requests: vec![request],
        call,
    }
}

fn standings_cases() -> Vec<MethodCase> {
    vec![
        case(
            "teams",
            request("/v1/standings/2026-10-21", STANDINGS),
            call!(c => c.teams(Some(date()))),
        ),
        case(
            "teams(None)",
            request("/v1/standings/now", STANDINGS),
            call!(c => c.teams(None)),
        ),
        case(
            "current_league_standings",
            request("/v1/standings/now", STANDINGS),
            call!(c => c.current_league_standings()),
        ),
        case(
            "league_standings_for_date",
            request("/v1/standings/2026-10-21", STANDINGS),
            call!(c => c.league_standings_for_date(&date())),
        ),
        MethodCase {
            name: "league_standings_for_season",
            requests: vec![
                request("/v1/standings-season", SEASON_MANIFEST),
                request(
                    Matcher::Exact(format!("/v1/standings/{SEASON_2024_STANDINGS_END}")),
                    STANDINGS,
                ),
            ],
            call: call!(c => c.league_standings_for_season(Season::new(2024))),
        },
        case(
            "season_standing_manifest",
            request("/v1/standings-season", SEASON_MANIFEST),
            call!(c => c.season_standing_manifest()),
        ),
    ]
}

fn shift_chart_request() -> Request {
    Request {
        query: Matcher::AllOf(vec![
            Matcher::UrlEncoded(
                "cayenneExp".into(),
                "gameId=2025020001 and ((duration != '00:00' and typeCode = 517) or typeCode != 517 )"
                    .into(),
            ),
            Matcher::UrlEncoded("exclude".into(), "eventDetails".into()),
        ]),
        ..request("/stats/rest/en/shiftcharts", SHIFT_CHART)
    }
}

fn game_cases() -> Vec<MethodCase> {
    vec![
        case(
            "boxscore",
            request("/v1/gamecenter/2025020001/boxscore", BOXSCORE),
            call!(c => c.boxscore(GameId::new(GAME_ID))),
        ),
        case(
            "play_by_play",
            request("/v1/gamecenter/2025020001/play-by-play", PLAY_BY_PLAY),
            call!(c => c.play_by_play(GAME_ID)),
        ),
        case(
            "landing",
            request("/v1/gamecenter/2025020001/landing", LANDING),
            call!(c => c.landing(GAME_ID)),
        ),
        case(
            "season_series",
            request("/v1/gamecenter/2025020001/right-rail", SEASON_SERIES),
            call!(c => c.season_series(GAME_ID)),
        ),
        case(
            "game_story",
            request("/v1/wsc/game-story/2025020001", GAME_STORY),
            call!(c => c.game_story(GAME_ID)),
        ),
        case(
            "shift_chart",
            shift_chart_request(),
            call!(c => c.shift_chart(GAME_ID)),
        ),
    ]
}

fn schedule_cases() -> Vec<MethodCase> {
    vec![
        case(
            "daily_schedule",
            request("/v1/schedule/2026-10-21", WEEKLY_SCHEDULE),
            call!(c => c.daily_schedule(Some(date()))),
        ),
        case(
            "daily_schedule(None)",
            request(today_path("/v1/schedule/"), WEEKLY_SCHEDULE),
            call!(c => c.daily_schedule(None)),
        ),
        case(
            "weekly_schedule",
            request("/v1/schedule/2026-10-21", WEEKLY_SCHEDULE),
            call!(c => c.weekly_schedule(Some(date()))),
        ),
        case(
            "weekly_schedule(None)",
            request("/v1/schedule/now", WEEKLY_SCHEDULE),
            call!(c => c.weekly_schedule(None)),
        ),
        case(
            "daily_scores",
            request("/v1/score/2026-10-21", DAILY_SCORES),
            call!(c => c.daily_scores(Some(date()))),
        ),
        case(
            "daily_scores(None)",
            request(today_path("/v1/score/"), DAILY_SCORES),
            call!(c => c.daily_scores(None)),
        ),
        case(
            "team_weekly_schedule",
            request("/v1/club-schedule/MTL/week/2026-10-21", TEAM_WEEK),
            call!(c => c.team_weekly_schedule(TEAM, Some(date()))),
        ),
        case(
            "team_weekly_schedule(None)",
            request(today_path("/v1/club-schedule/MTL/week/"), TEAM_WEEK),
            call!(c => c.team_weekly_schedule(TEAM, None)),
        ),
        case(
            "club_schedule_season",
            request(
                "/v1/club-schedule-season/MTL/20252026",
                TEAM_SEASON_SCHEDULE,
            ),
            call!(c => c.club_schedule_season(TEAM, season())),
        ),
    ]
}

fn search_player_request() -> Request {
    Request {
        query: Matcher::AllOf(vec![
            Matcher::UrlEncoded("culture".into(), "en-us".into()),
            Matcher::UrlEncoded("q".into(), SEARCH_QUERY.into()),
            Matcher::UrlEncoded("limit".into(), DEFAULT_SEARCH_LIMIT.to_string()),
        ]),
        ..request("/api/v1/search/player", SEARCH_RESULTS)
    }
}

fn player_cases() -> Vec<MethodCase> {
    vec![
        case(
            "player_landing",
            request("/v1/player/8478402/landing", PLAYER_LANDING),
            call!(c => c.player_landing(PlayerId::new(PLAYER_ID))),
        ),
        case(
            "player_game_log",
            request(
                Matcher::Exact(format!(
                    "/v1/player/8478402/game-log/20252026/{GAME_TYPE_ID}"
                )),
                PLAYER_GAME_LOG,
            ),
            call!(c => c.player_game_log(PLAYER_ID, season(), GAME_TYPE)),
        ),
        case(
            "search_player",
            search_player_request(),
            call!(c => c.search_player(SEARCH_QUERY, None)),
        ),
    ]
}

fn team_cases() -> Vec<MethodCase> {
    vec![
        case(
            "franchises",
            request("/stats/rest/en/franchise", FRANCHISES),
            call!(c => c.franchises()),
        ),
        case(
            "club_stats",
            request(
                Matcher::Exact(format!("/v1/club-stats/MTL/20252026/{GAME_TYPE_ID}")),
                CLUB_STATS,
            ),
            call!(c => c.club_stats(TEAM, season(), GAME_TYPE)),
        ),
        case(
            "club_stats_season",
            request("/v1/club-stats-season/MTL", CLUB_STATS_SEASON),
            call!(c => c.club_stats_season(TEAM)),
        ),
        case(
            "roster_current",
            request("/v1/roster/MTL/current", ROSTER_CURRENT),
            call!(c => c.roster_current(TEAM)),
        ),
        case(
            "roster_season",
            request("/v1/roster/MTL/20252026", ROSTER_SEASON),
            call!(c => c.roster_season(TEAM, season())),
        ),
    ]
}

fn method_cases() -> Vec<MethodCase> {
    [
        standings_cases(),
        game_cases(),
        schedule_cases(),
        player_cases(),
        team_cases(),
    ]
    .into_iter()
    .flatten()
    .collect()
}

async fn mock_request(
    server: &mut mockito::Server,
    request: &Request,
    status: usize,
) -> mockito::Mock {
    server
        .mock("GET", request.path.clone())
        .match_query(request.query.clone())
        .with_status(status)
        .with_header("content-type", "application/json")
        .with_body(request.body)
        .create_async()
        .await
}

#[test]
fn test_game_type_id_matches_game_type() {
    assert_eq!(GAME_TYPE.to_int(), GAME_TYPE_ID);
}

const OK: usize = 200;
const NOT_FOUND: usize = 404;

/// Runs `case` against a fresh mock server answering each expected request
/// with `status`; returns why the case failed, if it did.
async fn run_case(
    case: &MethodCase,
    status: usize,
    check: fn(Result<(), NHLApiError>) -> Result<(), String>,
) -> Option<String> {
    let mut server = mockito::Server::new_async().await;
    let mut mocks = Vec::new();
    // On a 404 the method must stop after its first request.
    let expected = if status == OK { case.requests.len() } else { 1 };
    for request in &case.requests[..expected] {
        mocks.push(mock_request(&mut server, request, status).await);
    }

    let client = Client::with_origin_override(server.url());
    if let Err(reason) = check((case.call)(&client).await) {
        return Some(format!("{}: {reason}", case.name));
    }
    for (request, mock) in case.requests.iter().zip(&mocks) {
        if !mock.matched_async().await {
            return Some(format!("{}: {:?} not requested", case.name, request.path));
        }
    }
    None
}

async fn assert_all_cases(status: usize, check: fn(Result<(), NHLApiError>) -> Result<(), String>) {
    let mut failures = Vec::new();
    for case in method_cases() {
        failures.extend(run_case(&case, status, check).await);
    }
    assert!(
        failures.is_empty(),
        "failing cases:\n{}",
        failures.join("\n")
    );
}

/// Every method sends exactly its documented requests (an unmatched request
/// gets mockito's 501, failing the call) and deserializes the recorded body.
#[tokio::test]
async fn test_all_client_methods_path_contract() {
    assert_all_cases(OK, |result| result.map_err(|err| format!("{err:?}"))).await;
}

/// Every method propagates a 404 on its first request as `ResourceNotFound`.
#[tokio::test]
async fn test_all_client_methods_404_propagates_error() {
    assert_all_cases(NOT_FOUND, |result| match result {
        Err(NHLApiError::ResourceNotFound { .. }) => Ok(()),
        other => Err(format!("expected ResourceNotFound, got {other:?}")),
    })
    .await;
}

// ===== Client-side logic =====

async fn serve(server: &mut mockito::Server, path: &str, body: &'static str) -> mockito::Mock {
    mock_request(server, &request(path, body), OK).await
}

#[tokio::test]
async fn test_daily_schedule_extracts_requested_day_from_week() {
    let mut server = mockito::Server::new_async().await;
    let mock = serve(&mut server, "/v1/schedule/2026-10-22", WEEKLY_SCHEDULE).await;
    let week: WeeklyScheduleResponse = serde_json::from_str(WEEKLY_SCHEDULE).unwrap();
    let expected_day = week
        .game_week
        .iter()
        .find(|day| day.date == "2026-10-22")
        .unwrap();

    let client = Client::with_origin_override(server.url());
    let date = GameDate::Date(NaiveDate::from_ymd_opt(2026, 10, 22).unwrap());
    let schedule = client.daily_schedule(Some(date)).await.unwrap();

    mock.assert_async().await;
    assert_eq!(schedule.date, "2026-10-22");
    assert_eq!(schedule.games, expected_day.games);
    assert_eq!(schedule.number_of_games, expected_day.games.len());
    assert_eq!(schedule.next_start_date, week.next_start_date);
    assert_eq!(schedule.previous_start_date, week.previous_start_date);
}

#[tokio::test]
async fn test_daily_schedule_day_missing_from_week_has_no_games() {
    let mut server = mockito::Server::new_async().await;
    let mock = serve(&mut server, "/v1/schedule/2026-10-28", WEEKLY_SCHEDULE).await;

    let client = Client::with_origin_override(server.url());
    let date = GameDate::Date(NaiveDate::from_ymd_opt(2026, 10, 28).unwrap());
    let schedule = client.daily_schedule(Some(date)).await.unwrap();

    mock.assert_async().await;
    assert_eq!(schedule.date, "2026-10-28");
    assert!(schedule.games.is_empty());
    assert_eq!(schedule.number_of_games, 0);
}

#[tokio::test]
async fn test_player_game_log_back_fills_player_id() {
    // The response has no player id; any requested id must come back.
    const OTHER_PLAYER_ID: i64 = 8471675;
    let mut server = mockito::Server::new_async().await;
    let path = format!("/v1/player/{OTHER_PLAYER_ID}/game-log/20252026/{GAME_TYPE_ID}");
    let mock = serve(&mut server, &path, PLAYER_GAME_LOG).await;

    let client = Client::with_origin_override(server.url());
    let log = client
        .player_game_log(OTHER_PLAYER_ID, season(), GAME_TYPE)
        .await
        .unwrap();

    mock.assert_async().await;
    assert!(!PLAYER_GAME_LOG.contains("\"playerId\""));
    assert_eq!(log.player_id, PlayerId::new(OTHER_PLAYER_ID));
}

#[tokio::test]
async fn test_league_standings_for_season_unknown_season_skips_standings_request() {
    let mut server = mockito::Server::new_async().await;
    let manifest = serve(&mut server, "/v1/standings-season", SEASON_MANIFEST).await;
    let standings = server
        .mock("GET", Matcher::Regex("^/v1/standings/".into()))
        .expect(0)
        .create_async()
        .await;

    let client = Client::with_origin_override(server.url());
    let result = client.league_standings_for_season(Season::new(1900)).await;

    match result {
        Err(NHLApiError::Other(message)) => assert!(message.contains("19001901"), "{message}"),
        other => panic!("expected Other for an unknown season, got {other:?}"),
    }
    manifest.assert_async().await;
    standings.assert_async().await;
}
