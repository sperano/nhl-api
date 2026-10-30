use super::*;
use crate::types::GameState;

const MONTH_FIRST: &str =
    include_str!("../../tests/fixtures/team_monthly_schedule/mtl_1917-12_first.json");
const MONTH_LAST: &str =
    include_str!("../../tests/fixtures/team_monthly_schedule/mtl_2027-04_last.json");
const MONTH_LOCKOUT: &str =
    include_str!("../../tests/fixtures/team_monthly_schedule/mtl_2005-01_lockout.json");
const MONTH_MODERN: &str =
    include_str!("../../tests/fixtures/team_monthly_schedule/mtl_2026-10.json");
const CALENDAR_FIRST: &str = include_str!("../../tests/fixtures/schedule_calendar/1917-12-19.json");
const CALENDAR_NEWEST: &str =
    include_str!("../../tests/fixtures/schedule_calendar/2027-07-20_newest.json");
const CALENDAR_MODERN: &str =
    include_str!("../../tests/fixtures/schedule_calendar/2026-10-21.json");

const CANADIENS: i64 = 8;
const TEAMS_IN_2026_27: usize = 32;

fn month(json: &str) -> TeamMonthScheduleResponse {
    serde_json::from_str(json).unwrap()
}

fn calendar(json: &str) -> ScheduleCalendar {
    serde_json::from_str(json).unwrap()
}

#[test]
fn test_team_month_schedule_modern() {
    let month = month(MONTH_MODERN);
    assert_eq!(month.previous_month.as_deref(), Some("2026-09"));
    assert_eq!(month.current_month, "2026-10");
    assert_eq!(month.next_month.as_deref(), Some("2026-11"));
    assert_eq!(month.club_timezone, "America/Montreal");
    assert_eq!(month.club_utc_offset, "-04:00");
    assert_eq!(month.games.len(), 13);
    assert!(month.games.iter().all(|game| game
        .game_date
        .as_deref()
        .unwrap()
        .starts_with("2026-10")));
}

#[test]
fn test_team_month_schedule_first_month_has_no_previous() {
    let month = month(MONTH_FIRST);
    assert_eq!(month.previous_month, None);
    assert_eq!(month.next_month.as_deref(), Some("1918-01"));
    assert!(month
        .games
        .iter()
        .all(|game| game.game_state == GameState::Off));
}

#[test]
fn test_team_month_schedule_last_month_has_no_next() {
    let month = month(MONTH_LAST);
    assert_eq!(month.next_month, None);
    assert!(month
        .games
        .iter()
        .all(|game| game.game_state == GameState::Future && game.home_team.score.is_none()));
}

#[test]
fn test_team_month_schedule_lockout_skips_to_months_with_games() {
    let month = month(MONTH_LOCKOUT);
    assert!(month.games.is_empty());
    assert_eq!(month.previous_month.as_deref(), Some("2004-04"));
    assert_eq!(month.next_month.as_deref(), Some("2005-09"));
}

#[test]
fn test_schedule_calendar_modern_week() {
    let calendar = calendar(CALENDAR_MODERN);
    assert_eq!(calendar.start_date, "2026-10-21");
    assert_eq!(calendar.end_date, "2026-10-27");
    assert_eq!(calendar.previous_start_date.as_deref(), Some("2026-10-14"));
    assert_eq!(calendar.next_start_date.as_deref(), Some("2026-10-28"));
    assert_eq!(calendar.teams.len(), TEAMS_IN_2026_27);

    let canadiens = calendar
        .teams
        .iter()
        .find(|team| team.id == TeamId::new(CANADIENS))
        .unwrap();
    assert_eq!(canadiens.abbrev, "MTL");
    assert_eq!(canadiens.season_id, Season::new(2026));
    assert!(canadiens.french);
}

#[test]
fn test_schedule_calendar_first_and_newest_weeks() {
    let first = calendar(CALENDAR_FIRST);
    assert_eq!(first.previous_start_date, None);
    assert_eq!(first.next_start_date.as_deref(), Some("1917-12-26"));
    assert_eq!(first.teams.len(), 4);

    let newest = calendar(CALENDAR_NEWEST);
    assert_eq!(newest.next_start_date, None);
    assert_eq!(newest.previous_start_date.as_deref(), Some("2027-04-06"));
}

#[test]
fn test_calendar_types_serde_round_trip() {
    let first = month(MONTH_FIRST);
    let json = serde_json::to_string(&first).unwrap();
    assert!(
        !json.contains("previousMonth"),
        "absent month omitted: {json}"
    );
    assert!(json.contains("clubUTCOffset"));
    assert_eq!(month(&json), first);

    let newest = calendar(CALENDAR_NEWEST);
    let json = serde_json::to_string(&newest).unwrap();
    assert!(
        !json.contains("nextStartDate"),
        "absent date omitted: {json}"
    );
    assert_eq!(calendar(&json), newest);
}
