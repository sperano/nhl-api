//! Path contract of the monthly-schedule, calendar, prospect and playoff
//! methods, run by `methods_contract_test.rs`.

use super::{case, date, request, Client, MethodCase, MethodFn, MethodFuture, TEAM};
use crate::date::Season;
use crate::error::NHLApiError;

const PROSPECTS: &str = include_str!("../../tests/fixtures/team_prospects/mtl.json");
const TEAM_MONTH: &str =
    include_str!("../../tests/fixtures/team_monthly_schedule/mtl_2026-10.json");
const CALENDAR: &str = include_str!("../../tests/fixtures/schedule_calendar/2026-10-21.json");
const BRACKET: &str = include_str!("../../tests/fixtures/playoff_bracket/20252026.json");
const CAROUSEL: &str = include_str!("../../tests/fixtures/playoff_series_carousel/20252026.json");
const SERIES: &str =
    include_str!("../../tests/fixtures/playoff_series_schedule/20252026_o_final.json");

/// The 2025-26 postseason, whose bracket is keyed by its end year, 2026.
fn season() -> Season {
    Season::new(2025)
}

pub(super) fn cases() -> Vec<MethodCase> {
    vec![
        case(
            "team_monthly_schedule",
            request("/v1/club-schedule/MTL/month/2026-10", TEAM_MONTH),
            call!(c => c.team_monthly_schedule(TEAM, Some(date()))),
        ),
        case(
            "team_monthly_schedule(None)",
            request("/v1/club-schedule/MTL/month/now", TEAM_MONTH),
            call!(c => c.team_monthly_schedule(TEAM, None)),
        ),
        case(
            "schedule_calendar",
            request("/v1/schedule-calendar/2026-10-21", CALENDAR),
            call!(c => c.schedule_calendar(Some(date()))),
        ),
        case(
            "schedule_calendar(None)",
            request("/v1/schedule-calendar/now", CALENDAR),
            call!(c => c.schedule_calendar(None)),
        ),
        case(
            "team_prospects",
            request("/v1/prospects/MTL", PROSPECTS),
            call!(c => c.team_prospects(TEAM)),
        ),
        case(
            "playoff_bracket",
            request("/v1/playoff-bracket/2026", BRACKET),
            call!(c => c.playoff_bracket(season())),
        ),
        case(
            "playoff_series_carousel",
            request("/v1/playoff-series/carousel/20252026", CAROUSEL),
            call!(c => c.playoff_series_carousel(season())),
        ),
        case(
            "playoff_series_schedule",
            request("/v1/schedule/playoff-series/20252026/o", SERIES),
            call!(c => c.playoff_series_schedule(season(), 'o')),
        ),
        case(
            "playoff_series_schedule(uppercase)",
            request("/v1/schedule/playoff-series/20252026/o", SERIES),
            call!(c => c.playoff_series_schedule(season(), 'O')),
        ),
    ]
}

/// A letter that is not ASCII alphabetic is rejected before any request (the
/// mock server has no route, so a request would fail differently).
#[tokio::test]
async fn test_playoff_series_schedule_rejects_invalid_letter() {
    let server = mockito::Server::new_async().await;
    let client = Client::with_origin_override(server.url());
    for letter in ['1', '/', 'é', ' '] {
        match client.playoff_series_schedule(season(), letter).await {
            Err(NHLApiError::InvalidSeriesLetter(rejected)) => assert_eq!(rejected, letter),
            other => panic!("{letter:?}: expected InvalidSeriesLetter, got {other:?}"),
        }
    }
}

#[test]
fn test_invalid_series_letter_has_no_status_code() {
    let err = NHLApiError::InvalidSeriesLetter('1');
    assert_eq!(err.status_code(), None);
    assert!(err.to_string().contains("'1'"));
}
