//! The `stats/rest` endpoints: reference lists and the summary reports.
//!
//! A report case builds its query once as an expression that both its URL and
//! its call use, so the recorded request is the one the method sends. Every
//! report case sorts explicitly.

use super::{season_from_id, url, with_query, STATS_BASE};
use crate::common::Case;
use nhl_api::{
    Country, DraftRounds, GameId, GameType, GlossaryEntry, GoalieSummary, Position, SeasonRules,
    SkaterSummary, SortDirection, StatsConfig, StatsPage, StatsQuery, TeamSummary,
};
use reqwest::Url;

const REGULAR: GameType = GameType::RegularSeason;
const PLAYOFFS: GameType = GameType::Playoffs;
/// The 2024-25 season opener played in Prague.
const OPENER_2024: i64 = 2024020001;
/// Rows requested by the paged cases; the API default is 50 as well.
const PAGE_SIZE: u32 = 50;

/// The URL of a report request for `query`.
fn report_url(path: &str, query: &StatsQuery) -> Url {
    let pairs = query.query_pairs();
    let pairs: Vec<(&str, &str)> = pairs.iter().map(|(k, v)| (*k, v.as_str())).collect();
    with_query(STATS_BASE, path, &pairs)
}

/// One season of one game type.
fn season_query(season_id: i32, game_type: GameType) -> StatsQuery {
    StatsQuery::new()
        .season(season_from_id(season_id))
        .game_type(game_type)
}

/// The league-wide seasons of the aggregate cases.
fn seasons_2020_to_2023(game_type: GameType) -> StatsQuery {
    StatsQuery::new()
        .seasons(season_from_id(20202021), season_from_id(20232024))
        .game_type(game_type)
        .aggregate(true)
}

fn game_query(game_id: i64) -> StatsQuery {
    StatsQuery::new()
        .game(GameId::new(game_id))
        .by_game(true)
        .unlimited()
}

fn by_points(query: StatsQuery) -> StatsQuery {
    query
        .sort_by("points", SortDirection::Desc)
        .sort_by("playerId", SortDirection::Asc)
}

fn by_wins(query: StatsQuery) -> StatsQuery {
    query
        .sort_by("wins", SortDirection::Desc)
        .sort_by("playerId", SortDirection::Asc)
}

fn by_standing(query: StatsQuery, tiebreaker: &str) -> StatsQuery {
    query
        .sort_by("points", SortDirection::Desc)
        .sort_by("wins", SortDirection::Desc)
        .sort_by(tiebreaker, SortDirection::Asc)
}

/// `report_case!(out, RowType, "method" / "label", "skater", query, method)`.
macro_rules! report_case {
    ($out:ident, $row:ty, $method:literal / $label:literal, $family:literal, $query:expr, $call:ident) => {
        case!(
            $out,
            StatsPage<$row>,
            $method / $label,
            report_url(concat!("en/", $family, "/summary"), &$query),
            |c| c.$call(&$query)
        )
    };
}

pub(super) fn reference_cases(out: &mut Vec<Case>) {
    case!(
        out,
        StatsPage<GlossaryEntry>,
        "glossary" / "all",
        url(STATS_BASE, "en/glossary"),
        |c| c.glossary()
    );
    case!(
        out,
        StatsConfig,
        "stats_config" / "all",
        url(STATS_BASE, "en/config"),
        |c| c.stats_config()
    );
    case!(
        out,
        StatsPage<Country>,
        "countries" / "all",
        url(STATS_BASE, "en/country"),
        |c| c.countries()
    );
    case!(
        out,
        StatsPage<SeasonRules>,
        "season_rules" / "all",
        url(STATS_BASE, "en/season"),
        |c| c.season_rules()
    );
    case!(
        out,
        StatsPage<DraftRounds>,
        "draft_rounds" / "all",
        url(STATS_BASE, "en/draft"),
        |c| c.draft_rounds()
    );
}

pub(super) fn skater_cases(out: &mut Vec<Case>) {
    report_case!(
        out,
        SkaterSummary,
        "skater_summary" / "20242025_regular",
        "skater",
        by_points(season_query(20242025, REGULAR).limit(PAGE_SIZE)),
        skater_summary
    );
    report_case!(
        out,
        SkaterSummary,
        "skater_summary" / "19251926_regular",
        "skater",
        by_points(season_query(19251926, REGULAR).unlimited()),
        skater_summary
    );
    report_case!(
        out,
        SkaterSummary,
        "skater_summary" / "19751976_playoffs",
        "skater",
        by_points(season_query(19751976, PLAYOFFS).unlimited()),
        skater_summary
    );
    report_case!(
        out,
        SkaterSummary,
        "skater_summary" / "20202021-20232024_aggregate",
        "skater",
        by_points(seasons_2020_to_2023(REGULAR).limit(PAGE_SIZE)),
        skater_summary
    );
    report_case!(
        out,
        SkaterSummary,
        "skater_summary" / "2024020001_by_game",
        "skater",
        by_points(game_query(OPENER_2024)),
        skater_summary
    );
    report_case!(
        out,
        SkaterSummary,
        "skater_summary" / "20242025_forwards_swe",
        "skater",
        by_points(
            season_query(20242025, REGULAR)
                .position(Position::Forward)
                .nationality("SWE")
                .unlimited()
        ),
        skater_summary
    );
}

pub(super) fn goalie_cases(out: &mut Vec<Case>) {
    report_case!(
        out,
        GoalieSummary,
        "goalie_summary" / "20242025_regular",
        "goalie",
        by_wins(season_query(20242025, REGULAR).unlimited()),
        goalie_summary
    );
    report_case!(
        out,
        GoalieSummary,
        "goalie_summary" / "19971998_playoffs",
        "goalie",
        by_wins(season_query(19971998, PLAYOFFS).unlimited()),
        goalie_summary
    );
    report_case!(
        out,
        GoalieSummary,
        "goalie_summary" / "20052006_regular",
        "goalie",
        by_wins(season_query(20052006, REGULAR).unlimited()),
        goalie_summary
    );
    report_case!(
        out,
        GoalieSummary,
        "goalie_summary" / "2024020001_by_game",
        "goalie",
        by_wins(game_query(OPENER_2024)),
        goalie_summary
    );
}

pub(super) fn team_cases(out: &mut Vec<Case>) {
    report_case!(
        out,
        TeamSummary,
        "team_summary" / "20242025_regular",
        "team",
        by_standing(season_query(20242025, REGULAR).unlimited(), "teamId"),
        team_summary
    );
    report_case!(
        out,
        TeamSummary,
        "team_summary" / "19751976_playoffs",
        "team",
        by_standing(season_query(19751976, PLAYOFFS).unlimited(), "teamId"),
        team_summary
    );
    report_case!(
        out,
        TeamSummary,
        "team_summary" / "20202021-20232024_aggregate",
        "team",
        by_standing(seasons_2020_to_2023(REGULAR).unlimited(), "franchiseId"),
        team_summary
    );
    report_case!(
        out,
        TeamSummary,
        "team_summary" / "2024020001_by_game",
        "team",
        by_standing(game_query(OPENER_2024), "teamId"),
        team_summary
    );
}
