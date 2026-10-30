//! League-wide standings and schedules.

use super::{date, season_from_id, web, CANADIENS};
use crate::common::Case;
use nhl_api::{
    DailyScores, ScheduleCalendar, Season, SeasonsResponse, StandingsResponse,
    TeamMonthScheduleResponse, TeamScheduleResponse, WeeklyScheduleResponse,
};

/// `standings!(out, "YYYY-MM-DD")`: `league_standings_for_date` on that date.
macro_rules! standings {
    ($out:ident, $date:literal) => {
        case!(
            $out,
            StandingsResponse,
            "league_standings_for_date" / $date,
            web(concat!("standings/", $date)),
            |c| c.league_standings_for_date(&date($date))
        )
    };
}

/// `$method!(out, "label", "YYYY-MM-DD")` for the weekly schedule and scores.
macro_rules! weekly_schedule {
    ($out:ident, $label:literal, $date:literal) => {
        case!(
            $out,
            WeeklyScheduleResponse,
            "weekly_schedule" / $label,
            web(concat!("schedule/", $date)),
            |c| c.weekly_schedule(Some(date($date)))
        )
    };
}

macro_rules! daily_scores {
    ($out:ident, $label:literal, $date:literal) => {
        case!(
            $out,
            DailyScores,
            "daily_scores" / $label,
            web(concat!("score/", $date)),
            |c| c.daily_scores(Some(date($date)))
        )
    };
}

macro_rules! team_weekly_schedule {
    ($out:ident, $label:literal, $date:literal) => {
        case!(
            $out,
            TeamScheduleResponse,
            "team_weekly_schedule" / $label,
            web(&format!("club-schedule/{CANADIENS}/week/{}", $date)),
            |c| c.team_weekly_schedule(CANADIENS, Some(date($date)))
        )
    };
}

/// `team_monthly_schedule!(out, "label", "YYYY-MM")`, called with the 1st of
/// the month.
macro_rules! team_monthly_schedule {
    ($out:ident, $label:literal, $month:literal) => {
        case!(
            $out,
            TeamMonthScheduleResponse,
            "team_monthly_schedule" / $label,
            web(&format!("club-schedule/{CANADIENS}/month/{}", $month)),
            |c| c.team_monthly_schedule(CANADIENS, Some(date(concat!($month, "-01"))))
        )
    };
}

macro_rules! schedule_calendar {
    ($out:ident, $label:literal, $date:literal) => {
        case!(
            $out,
            ScheduleCalendar,
            "schedule_calendar" / $label,
            web(concat!("schedule-calendar/", $date)),
            |c| c.schedule_calendar(Some(date($date)))
        )
    };
}

/// `club_schedule_season!(out, "label", start_year)`.
macro_rules! club_schedule_season {
    ($out:ident, $label:literal, $start:literal) => {
        case!(
            $out,
            TeamScheduleResponse,
            "club_schedule_season" / $label,
            web(&format!(
                "club-schedule-season/{CANADIENS}/{}",
                Season::new($start).to_api_string()
            )),
            |c| c.club_schedule_season(CANADIENS, Season::new($start))
        )
    };
}

#[rustfmt::skip]
pub(super) fn standings_cases(out: &mut Vec<Case>) {
    standings!(out, "1925-03-01");
    standings!(out, "1975-03-01");
    standings!(out, "2011-03-01");
    standings!(out, "2026-10-21");
    case!(out, SeasonsResponse, "season_standing_manifest" / "all",
        web("standings-season"), |c| c.season_standing_manifest());
    live_case!(out, "current_league_standings" / "now", |c| c.current_league_standings());
    live_case!(out, "league_standings_for_season" / "19751976",
        |c| c.league_standings_for_season(season_from_id(19751976)));
    live_case!(out, "league_standings_for_season" / "20242025",
        |c| c.league_standings_for_season(season_from_id(20242025)));
    live_case!(out, "teams" / "now", |c| c.teams(None));
    live_case!(out, "teams" / "1925-03-01", |c| c.teams(Some(date("1925-03-01"))));
}

#[rustfmt::skip]
pub(super) fn schedule_cases(out: &mut Vec<Case>) {
    weekly_schedule!(out, "2026-10-21", "2026-10-21");
    weekly_schedule!(out, "2026-07-15_empty", "2026-07-15");
    weekly_schedule!(out, "1917-12-19", "1917-12-19");
    weekly_schedule!(out, "2027-07-20_newest", "2027-07-20");
    weekly_schedule!(out, "2022-09-28_cancelled", "2022-09-28");
    live_case!(out, "weekly_schedule" / "now", |c| c.weekly_schedule(None));
    live_case!(out, "daily_schedule" / "2026-10-21",
        |c| c.daily_schedule(Some(date("2026-10-21"))));
    live_case!(out, "daily_schedule" / "now", |c| c.daily_schedule(None));
    daily_scores!(out, "2026-10-21", "2026-10-21");
    daily_scores!(out, "2026-07-15_empty", "2026-07-15");
    daily_scores!(out, "1975-10-09", "1975-10-09");
    daily_scores!(out, "2022-09-28_cancelled", "2022-09-28");
    live_case!(out, "daily_scores" / "now", |c| c.daily_scores(None));
    team_weekly_schedule!(out, "mtl_2026-10-21", "2026-10-21");
    team_weekly_schedule!(out, "mtl_2026-07-15_empty", "2026-07-15");
    club_schedule_season!(out, "mtl_20252026", 2025);
    club_schedule_season!(out, "mtl_19251926", 1925);
}

#[rustfmt::skip]
pub(super) fn calendar_cases(out: &mut Vec<Case>) {
    team_monthly_schedule!(out, "mtl_1917-12_first", "1917-12");
    team_monthly_schedule!(out, "mtl_1926-01", "1926-01");
    team_monthly_schedule!(out, "mtl_1976-04_playoffs", "1976-04");
    team_monthly_schedule!(out, "mtl_1995-02", "1995-02");
    team_monthly_schedule!(out, "mtl_2005-01_lockout", "2005-01");
    team_monthly_schedule!(out, "mtl_2011-03", "2011-03");
    team_monthly_schedule!(out, "mtl_2026-07_empty", "2026-07");
    team_monthly_schedule!(out, "mtl_2026-10", "2026-10");
    team_monthly_schedule!(out, "mtl_2027-04_last", "2027-04");
    live_case!(out, "team_monthly_schedule" / "mtl_now",
        |c| c.team_monthly_schedule(CANADIENS, None));
    schedule_calendar!(out, "1917-12-19", "1917-12-19");
    schedule_calendar!(out, "1926-01-05", "1926-01-05");
    schedule_calendar!(out, "1976-04-10", "1976-04-10");
    schedule_calendar!(out, "1995-02-01", "1995-02-01");
    schedule_calendar!(out, "2004-12-01_lockout", "2004-12-01");
    schedule_calendar!(out, "2011-03-01", "2011-03-01");
    schedule_calendar!(out, "2026-07-15_empty", "2026-07-15");
    schedule_calendar!(out, "2026-10-21", "2026-10-21");
    schedule_calendar!(out, "2027-07-20_newest", "2027-07-20");
    live_case!(out, "schedule_calendar" / "now", |c| c.schedule_calendar(None));
}
