//! Every `Client` method, called across game states (scheduled, in progress,
//! final), regular season and playoffs, eras (1920s, 1970s, 2010, current),
//! skaters and goalies, and empty schedule weeks.
//!
//! Each submodule's macros build a case's URL from the same arguments as its
//! `Client` call, so a case states its arguments once.

mod edge;
mod games;
mod league;
mod players;
mod teams;

use super::{Case, CurrentGames};
use nhl_api::GameDate;
use reqwest::Url;

const WEB_BASE: &str = "https://api-web.nhle.com/v1/";
const STATS_BASE: &str = "https://api.nhle.com/stats/rest/";
const SEARCH_BASE: &str = "https://search.d3.nhle.com/api/v1/";

const CONNOR_MCDAVID: i64 = 8478402;
const FILIP_GUSTAVSSON: i64 = 8479406;
const CANADIENS: &str = "MTL";
const CANADIENS_ID: i64 = 8;

/// Every case. Cases on a game in [`CurrentGames`] can only run live when that
/// game exists; pass `CurrentGames::default()` to list the recorded cases.
pub fn cases(current: &CurrentGames) -> Vec<Case> {
    let mut out = Vec::new();
    games::cases(&mut out, current);
    league::standings_cases(&mut out);
    league::schedule_cases(&mut out);
    players::landing_cases(&mut out);
    players::game_log_cases(&mut out);
    players::search_cases(&mut out);
    teams::roster_cases(&mut out);
    teams::club_stats_cases(&mut out);
    edge::skater_cases(&mut out);
    edge::goalie_cases(&mut out);
    edge::team_cases(&mut out);
    out
}

fn url(base: &str, path: &str) -> Url {
    Url::parse(base)
        .and_then(|base| base.join(path))
        .expect("valid URL")
}

fn web(path: &str) -> Url {
    url(WEB_BASE, path)
}

fn with_query(base: &str, path: &str, query: &[(&str, &str)]) -> Url {
    let mut url = url(base, path);
    url.query_pairs_mut().extend_pairs(query);
    url
}

fn date(api: &str) -> GameDate {
    api.parse().expect("valid YYYY-MM-DD date")
}
