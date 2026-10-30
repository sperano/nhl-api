//! Path contract for the `stats/rest` methods: each case drives the public
//! method against a mock server (see `Client::with_origin_override`) and
//! checks the exact path, base path and query it requests, then the
//! client-side logic around it (validation, default sorts).

use std::future::Future;
use std::pin::Pin;

use mockito::Matcher;

use super::super::Client;
use crate::date::Season;
use crate::error::NHLApiError;
use crate::ids::{FranchiseId, GameId};
use crate::stats_query::{CayenneExpr, SortDirection, StatsQuery, StatsQueryError};
use crate::types::{GameType, SkaterSummary};

type MethodFuture<'a> = Pin<Box<dyn Future<Output = Result<(), NHLApiError>> + 'a>>;
type MethodFn = for<'a> fn(&'a Client) -> MethodFuture<'a>;

struct StatsCase {
    name: &'static str,
    path: &'static str,
    query: Matcher,
    body: &'static str,
    call: MethodFn,
}

const OK: usize = 200;
const NOT_FOUND: usize = 404;
const SEASON_FILTER: &str = "seasonId=20242025 and gameTypeId=2";
const SKATER_DEFAULT_SORT: &str = r#"[{"property":"points","direction":"DESC"},{"property":"gamesPlayed","direction":"ASC"},{"property":"playerId","direction":"ASC"}]"#;
const GOALIE_DEFAULT_SORT: &str = r#"[{"property":"wins","direction":"DESC"},{"property":"gamesPlayed","direction":"ASC"},{"property":"playerId","direction":"ASC"}]"#;
const TEAM_DEFAULT_SORT: &str = r#"[{"property":"points","direction":"DESC"},{"property":"wins","direction":"DESC"},{"property":"teamId","direction":"ASC"}]"#;
const TEAM_AGGREGATE_DEFAULT_SORT: &str = r#"[{"property":"points","direction":"DESC"},{"property":"wins","direction":"DESC"},{"property":"franchiseId","direction":"ASC"}]"#;
const PAGE_SIZE: u32 = 10;

const GLOSSARY: &str = include_str!("../../tests/fixtures/glossary/all.json");
const CONFIG: &str = include_str!("../../tests/fixtures/stats_config/all.json");
const COUNTRIES: &str = include_str!("../../tests/fixtures/countries/all.json");
const SEASON_RULES: &str = include_str!("../../tests/fixtures/season_rules/all.json");
const DRAFT_ROUNDS: &str = include_str!("../../tests/fixtures/draft_rounds/all.json");
const SKATER_SUMMARY: &str =
    include_str!("../../tests/fixtures/skater_summary/20242025_regular.json");
const GOALIE_SUMMARY: &str =
    include_str!("../../tests/fixtures/goalie_summary/20242025_regular.json");
const TEAM_SUMMARY: &str = include_str!("../../tests/fixtures/team_summary/20242025_regular.json");
const TEAM_AGGREGATE_SUMMARY: &str =
    include_str!("../../tests/fixtures/team_summary/20202021-20232024_aggregate.json");

macro_rules! call {
    ($client:ident => $call:expr) => {{
        fn call($client: &Client) -> MethodFuture<'_> {
            Box::pin(async move { $call.await.map(|_| ()) })
        }
        call as MethodFn
    }};
}

fn season_query() -> StatsQuery {
    StatsQuery::new()
        .season(Season::new(2024))
        .game_type(GameType::RegularSeason)
        .limit(PAGE_SIZE)
}

/// Matches the query string `pairs` exactly (names and decoded values).
fn query_of(pairs: &[(&str, &str)]) -> Matcher {
    Matcher::AllOf(
        pairs
            .iter()
            .map(|(name, value)| Matcher::UrlEncoded((*name).into(), (*value).into()))
            .collect(),
    )
}

/// The query string of [`season_query`] sorted with `sort`.
fn summary_query(sort: &'static str) -> Matcher {
    query_of(&[
        ("cayenneExp", SEASON_FILTER),
        ("isAggregate", "false"),
        ("isGame", "false"),
        ("start", "0"),
        ("limit", "10"),
        ("sort", sort),
    ])
}

fn stats_cases() -> Vec<StatsCase> {
    let no_query = || Matcher::Missing;
    vec![
        StatsCase {
            name: "glossary",
            path: "/stats/rest/en/glossary",
            query: no_query(),
            body: GLOSSARY,
            call: call!(c => c.glossary()),
        },
        StatsCase {
            name: "stats_config",
            path: "/stats/rest/en/config",
            query: no_query(),
            body: CONFIG,
            call: call!(c => c.stats_config()),
        },
        StatsCase {
            name: "countries",
            path: "/stats/rest/en/country",
            query: no_query(),
            body: COUNTRIES,
            call: call!(c => c.countries()),
        },
        StatsCase {
            name: "season_rules",
            path: "/stats/rest/en/season",
            query: no_query(),
            body: SEASON_RULES,
            call: call!(c => c.season_rules()),
        },
        StatsCase {
            name: "draft_rounds",
            path: "/stats/rest/en/draft",
            query: no_query(),
            body: DRAFT_ROUNDS,
            call: call!(c => c.draft_rounds()),
        },
        StatsCase {
            name: "skater_summary",
            path: "/stats/rest/en/skater/summary",
            query: summary_query(SKATER_DEFAULT_SORT),
            body: SKATER_SUMMARY,
            call: call!(c => c.skater_summary(&season_query())),
        },
        StatsCase {
            name: "goalie_summary",
            path: "/stats/rest/en/goalie/summary",
            query: summary_query(GOALIE_DEFAULT_SORT),
            body: GOALIE_SUMMARY,
            call: call!(c => c.goalie_summary(&season_query())),
        },
        StatsCase {
            name: "team_summary",
            path: "/stats/rest/en/team/summary",
            query: summary_query(TEAM_DEFAULT_SORT),
            body: TEAM_SUMMARY,
            call: call!(c => c.team_summary(&season_query())),
        },
        StatsCase {
            name: "skater_report",
            path: "/stats/rest/en/skater/realtime",
            query: query_of(&[("cayenneExp", SEASON_FILTER), ("limit", "10")]),
            body: SKATER_SUMMARY,
            call: call!(c => c.skater_report::<SkaterSummary>("realtime", &season_query())),
        },
        StatsCase {
            name: "goalie_report",
            path: "/stats/rest/en/goalie/advanced",
            query: query_of(&[("cayenneExp", SEASON_FILTER), ("limit", "10")]),
            body: GOALIE_SUMMARY,
            call: call!(c => c.goalie_report::<serde_json::Value>("advanced", &season_query())),
        },
        StatsCase {
            name: "team_report",
            path: "/stats/rest/en/team/faceoffwins",
            query: query_of(&[("cayenneExp", SEASON_FILTER), ("limit", "10")]),
            body: TEAM_SUMMARY,
            call: call!(c => c.team_report::<serde_json::Value>("faceoffwins", &season_query())),
        },
    ]
}

async fn mock_case(server: &mut mockito::Server, case: &StatsCase, status: usize) -> mockito::Mock {
    server
        .mock("GET", case.path)
        .match_query(case.query.clone())
        .with_status(status)
        .with_header("content-type", "application/json")
        .with_body(case.body)
        .create_async()
        .await
}

/// Every method requests its documented path and query and deserializes the
/// recorded body.
#[tokio::test]
async fn test_stats_methods_path_contract() {
    let mut failures = Vec::new();
    for case in stats_cases() {
        let mut server = mockito::Server::new_async().await;
        let mock = mock_case(&mut server, &case, OK).await;
        let client = Client::with_origin_override(server.url());

        if let Err(err) = (case.call)(&client).await {
            failures.push(format!("{}: {err:?}", case.name));
        } else if !mock.matched_async().await {
            failures.push(format!("{}: {} not requested", case.name, case.path));
        }
    }
    assert!(
        failures.is_empty(),
        "failing cases:\n{}",
        failures.join("\n")
    );
}

/// Every method propagates a 404 as `ResourceNotFound`.
#[tokio::test]
async fn test_stats_methods_404_propagates_error() {
    let mut failures = Vec::new();
    for case in stats_cases() {
        let mut server = mockito::Server::new_async().await;
        mock_case(&mut server, &case, NOT_FOUND).await;
        let client = Client::with_origin_override(server.url());

        match (case.call)(&client).await {
            Err(NHLApiError::ResourceNotFound { .. }) => {}
            other => failures.push(format!(
                "{}: expected ResourceNotFound, got {other:?}",
                case.name
            )),
        }
    }
    assert!(
        failures.is_empty(),
        "failing cases:\n{}",
        failures.join("\n")
    );
}

// ===== Validation =====

/// A mock server that fails the test if it gets any request.
struct SilentServer {
    _server: mockito::ServerGuard,
    any_request: mockito::Mock,
    client: Client,
}

impl SilentServer {
    async fn start() -> Self {
        let mut server = mockito::Server::new_async().await;
        let any_request = server
            .mock("GET", Matcher::Regex(".*".into()))
            .expect(0)
            .create_async()
            .await;
        let client = Client::with_origin_override(server.url());
        Self {
            _server: server,
            any_request,
            client,
        }
    }
}

#[tokio::test]
async fn test_skater_report_invalid_report_name_skips_request() {
    let silent = SilentServer::start().await;

    for report in ["", "../config", "summary/x", "a b", "1abc"] {
        let result = silent
            .client
            .skater_report::<serde_json::Value>(report, &StatsQuery::new())
            .await;

        match result {
            Err(NHLApiError::InvalidStatsQuery(StatsQueryError::InvalidReport(name))) => {
                assert_eq!(name, report)
            }
            other => panic!("expected InvalidReport for {report:?}, got {other:?}"),
        }
    }
    silent.any_request.assert_async().await;
}

#[tokio::test]
async fn test_goalie_and_team_report_invalid_report_name_skip_request() {
    let silent = SilentServer::start().await;
    let query = StatsQuery::new();

    let goalie = silent
        .client
        .goalie_report::<serde_json::Value>("a/b", &query);
    let team = silent
        .client
        .team_report::<serde_json::Value>("a/b", &query);

    for result in [goalie.await, team.await] {
        assert!(matches!(
            result,
            Err(NHLApiError::InvalidStatsQuery(
                StatsQueryError::InvalidReport(_)
            ))
        ));
    }
    silent.any_request.assert_async().await;
}

#[tokio::test]
async fn test_skater_summary_invalid_field_skips_request() {
    let silent = SilentServer::start().await;
    let query = StatsQuery::new().filter(CayenneExpr::field("points or 1=1 or x").eq(1));

    let err = silent.client.skater_summary(&query).await.unwrap_err();

    match &err {
        NHLApiError::InvalidStatsQuery(StatsQueryError::InvalidField(field)) => {
            assert_eq!(field, "points or 1=1 or x")
        }
        other => panic!("expected InvalidField, got {other:?}"),
    }
    assert_eq!(err.status_code(), None);
    silent.any_request.assert_async().await;
}

#[tokio::test]
async fn test_goalie_summary_zero_limit_skips_request() {
    let silent = SilentServer::start().await;

    let result = silent
        .client
        .goalie_summary(&StatsQuery::new().limit(0))
        .await;

    assert!(matches!(
        result,
        Err(NHLApiError::InvalidStatsQuery(StatsQueryError::ZeroLimit))
    ));
    silent.any_request.assert_async().await;
}

#[tokio::test]
async fn test_team_summary_invalid_sort_field_skips_request() {
    let silent = SilentServer::start().await;
    let query = StatsQuery::new().sort_by("wins;", SortDirection::Asc);

    let result = silent.client.team_summary(&query).await;

    assert!(matches!(
        result,
        Err(NHLApiError::InvalidStatsQuery(
            StatsQueryError::InvalidField(_)
        ))
    ));
    silent.any_request.assert_async().await;
}

// ===== Default sort =====

/// A mock server answering `path` only for a request whose `sort` is `sort`.
async fn sort_server(
    path: &str,
    body: &'static str,
    sort: &str,
) -> (mockito::ServerGuard, mockito::Mock, Client) {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", path)
        .match_query(Matcher::UrlEncoded("sort".into(), sort.into()))
        .with_status(OK)
        .with_header("content-type", "application/json")
        .with_body(body)
        .create_async()
        .await;
    let client = Client::with_origin_override(server.url());
    (server, mock, client)
}

#[tokio::test]
async fn test_team_summary_aggregate_default_sort_breaks_ties_by_franchise() {
    let query = StatsQuery::new()
        .seasons(Season::new(2020), Season::new(2023))
        .aggregate(true)
        .unlimited();
    let (_server, mock, client) = sort_server(
        "/stats/rest/en/team/summary",
        TEAM_AGGREGATE_SUMMARY,
        TEAM_AGGREGATE_DEFAULT_SORT,
    )
    .await;

    client.team_summary(&query).await.unwrap();

    mock.assert_async().await;
}

#[tokio::test]
async fn test_summary_keeps_callers_sort_instead_of_default() {
    const CUSTOM_SORT: &str = r#"[{"property":"goals","direction":"DESC"}]"#;
    let query = season_query().sort_by("goals", SortDirection::Desc);
    let (_server, mock, client) =
        sort_server("/stats/rest/en/skater/summary", SKATER_SUMMARY, CUSTOM_SORT).await;

    client.skater_summary(&query).await.unwrap();

    mock.assert_async().await;
}

#[tokio::test]
async fn test_report_sends_no_default_sort() {
    let mut server = mockito::Server::new_async().await;
    let path = "/stats/rest/en/skater/bios";
    let mock = server
        .mock("GET", path)
        .match_query(Matcher::Any)
        .with_status(OK)
        .with_header("content-type", "application/json")
        .with_body(r#"{"data":[],"total":0}"#)
        .create_async()
        .await;
    let with_sort = server
        .mock("GET", path)
        .match_query(Matcher::Regex("sort=".into()))
        .expect(0)
        .create_async()
        .await;
    let client = Client::with_origin_override(server.url());

    let page = client
        .skater_report::<serde_json::Value>("bios", &StatsQuery::new())
        .await
        .unwrap();

    mock.assert_async().await;
    with_sort.assert_async().await;
    assert!(page.data.is_empty());
}

// ===== Response handling =====

#[tokio::test]
async fn test_skater_summary_returns_rows_and_total() {
    let mut server = mockito::Server::new_async().await;
    let case = &stats_cases()[5];
    let mock = mock_case(&mut server, case, OK).await;
    let client = Client::with_origin_override(server.url());

    let page = client.skater_summary(&season_query()).await.unwrap();

    mock.assert_async().await;
    assert_eq!(page.data[0].skater_full_name, "Nikita Kucherov");
    assert_eq!(page.total, 920);
}

#[tokio::test]
async fn test_team_summary_filters_and_sends_franchise_in_query() {
    let query = StatsQuery::new()
        .franchise(FranchiseId::new(6))
        .game(GameId::new(2024020001))
        .by_game(true)
        .unlimited()
        .sort_by("points", SortDirection::Desc);
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/stats/rest/en/team/summary")
        .match_query(query_of(&[
            ("cayenneExp", "franchiseId=6 and gameId=2024020001"),
            ("isGame", "true"),
            ("limit", "-1"),
        ]))
        .with_status(OK)
        .with_header("content-type", "application/json")
        .with_body(TEAM_SUMMARY)
        .create_async()
        .await;
    let client = Client::with_origin_override(server.url());

    client.team_summary(&query).await.unwrap();

    mock.assert_async().await;
}
