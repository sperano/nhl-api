//! Team monthly schedule and the weekly schedule calendar.

use serde::{Deserialize, Serialize};

use crate::date::Season;
use crate::ids::TeamId;

use super::common::LocalizedString;
use super::schedule::ScheduleGame;

/// Team schedule for one month (`/club-schedule/{team}/month/{YYYY-MM}`).
///
/// A month without games (the off-season, the 2004-05 lockout) has an empty
/// `games`; `previous_month`/`next_month` then skip to the nearest month that
/// has games.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TeamMonthScheduleResponse {
    /// `YYYY-MM`; `None` for the team's first month on record.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_month: Option<String>,
    /// `YYYY-MM`.
    pub current_month: String,
    /// `YYYY-MM`; `None` for the last month scheduled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_month: Option<String>,
    /// IANA name, e.g. `"America/Montreal"`.
    pub club_timezone: String,
    /// e.g. `"-04:00"`.
    #[serde(rename = "clubUTCOffset")]
    pub club_utc_offset: String,
    pub games: Vec<ScheduleGame>,
}

/// The teams playing in a week (`/schedule-calendar/{date}`).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ScheduleCalendar {
    /// `YYYY-MM-DD`, the requested date.
    pub start_date: String,
    /// `YYYY-MM-DD`, six days after `start_date`.
    pub end_date: String,
    /// `None` for the first week on record (December 1917).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_start_date: Option<String>,
    /// `None` for the newest week on record.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_start_date: Option<String>,
    pub teams: Vec<CalendarTeam>,
}

/// A team of a [`ScheduleCalendar`].
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CalendarTeam {
    pub id: TeamId,
    pub season_id: Season,
    pub abbrev: String,
    pub name: LocalizedString,
    pub common_name: LocalizedString,
    pub place_name: LocalizedString,
    /// Whether the team has French-language branding (e.g. Montréal).
    pub french: bool,
    pub logo: String,
    pub dark_logo: String,
}

#[cfg(test)]
#[path = "calendar_test.rs"]
mod tests;
