//! Responses of the `stats/rest` endpoints that are not report rows: the
//! paging envelope, glossary, countries, season rules, draft rounds and the
//! report configuration. Summary report rows are in [`super::stats_summary`].

use std::collections::BTreeMap;
use std::fmt;

use serde::de::{self, Unexpected, Visitor};
use serde::{Deserialize, Deserializer, Serialize};

use crate::date::Season;

/// One page of a `stats/rest` list: the rows plus how many rows match in all.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StatsPage<T> {
    /// The rows of this page.
    pub data: Vec<T>,
    /// The number of rows matching the query, across all pages.
    pub total: u32,
}

/// Reads a flag the API sends as `0`/`1`; JSON booleans are accepted too so
/// a serialized value reads back. Any other number is an error.
pub(crate) fn int_flag<'de, D: Deserializer<'de>>(deserializer: D) -> Result<bool, D::Error> {
    struct FlagVisitor;

    impl Visitor<'_> for FlagVisitor {
        type Value = bool;

        fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("a 0/1 flag or a boolean")
        }

        fn visit_bool<E: de::Error>(self, value: bool) -> Result<bool, E> {
            Ok(value)
        }

        fn visit_i64<E: de::Error>(self, value: i64) -> Result<bool, E> {
            match value {
                0 => Ok(false),
                1 => Ok(true),
                _ => Err(E::invalid_value(Unexpected::Signed(value), &"0 or 1")),
            }
        }

        fn visit_u64<E: de::Error>(self, value: u64) -> Result<bool, E> {
            match value {
                0 => Ok(false),
                1 => Ok(true),
                _ => Err(E::invalid_value(Unexpected::Unsigned(value), &"0 or 1")),
            }
        }
    }

    deserializer.deserialize_any(FlagVisitor)
}

/// A statistic definition from `glossary()`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GlossaryEntry {
    pub id: i64,
    pub abbreviation: String,
    pub definition: String,
    /// `None` for entries that are not tied to a season.
    pub first_season_for_stat: Option<Season>,
    pub full_name: String,
    pub language_code: String,
    pub last_updated: String,
}

impl fmt::Display for GlossaryEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.abbreviation, self.full_name)
    }
}

/// A country from `countries()`. `has_player_stats` and `is_active` are
/// `0`/`1` in the response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Country {
    /// The three-letter code, e.g. `"SWE"`.
    pub id: String,
    pub country3_code: String,
    pub country_code: String,
    pub country_name: String,
    #[serde(deserialize_with = "int_flag")]
    pub has_player_stats: bool,
    pub image_url: Option<String>,
    pub ioc_code: String,
    #[serde(deserialize_with = "int_flag")]
    pub is_active: bool,
    pub nationality_name: String,
    pub olympic_url: Option<String>,
    pub thumbnail_url: Option<String>,
}

impl fmt::Display for Country {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.country_name, self.country3_code)
    }
}

/// The rules and calendar of one season, from `season_rules()`. The `*_in_use`
/// flags and `nhl_stanley_cup_owner`/`olympics_participation` are `0`/`1` in
/// the response. `preseason_startdate` is `None` for seasons before the
/// API recorded preseasons.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SeasonRules {
    pub id: Season,
    #[serde(deserialize_with = "int_flag")]
    pub all_star_game_in_use: bool,
    #[serde(deserialize_with = "int_flag")]
    pub conferences_in_use: bool,
    #[serde(deserialize_with = "int_flag")]
    pub divisions_in_use: bool,
    pub end_date: String,
    #[serde(deserialize_with = "int_flag")]
    pub entry_draft_in_use: bool,
    /// The display form, e.g. `"2024-25"`.
    pub formatted_season_id: String,
    pub minimum_playoff_minutes_for_goalie_stats_leaders: i32,
    pub minimum_regular_games_for_goalie_stats_leaders: i32,
    #[serde(deserialize_with = "int_flag")]
    pub nhl_stanley_cup_owner: bool,
    pub number_of_games: i32,
    #[serde(deserialize_with = "int_flag")]
    pub olympics_participation: bool,
    #[serde(rename = "pointForOTLossInUse", deserialize_with = "int_flag")]
    pub point_for_ot_loss_in_use: bool,
    #[serde(
        rename = "preseasonStartdate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub preseason_startdate: Option<String>,
    pub regular_season_end_date: String,
    #[serde(deserialize_with = "int_flag")]
    pub row_in_use: bool,
    pub season_ordinal: i32,
    pub start_date: String,
    #[serde(deserialize_with = "int_flag")]
    pub supplemental_draft_in_use: bool,
    #[serde(deserialize_with = "int_flag")]
    pub ties_in_use: bool,
    pub total_playoff_games: i32,
    pub total_regular_season_games: i32,
    #[serde(deserialize_with = "int_flag")]
    pub wildcard_in_use: bool,
}

impl fmt::Display for SeasonRules {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {} games per team",
            self.formatted_season_id, self.number_of_games
        )
    }
}

/// How many rounds one year's entry draft had, from `draft_rounds()`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DraftRounds {
    pub id: i64,
    pub draft_year: i32,
    pub rounds: i32,
}

/// The columns, filters and sort keys of one report, for one row granularity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportColumns {
    pub display_items: Vec<String>,
    /// Fields usable in a [`StatsQuery::fact_filter`](crate::StatsQuery::fact_filter).
    pub result_filters: Vec<String>,
    /// Fields the report can be sorted by; empty for some reports.
    pub sort_keys: Vec<String>,
}

/// A report's columns per season row and per game row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReportConfig {
    pub game: ReportColumns,
    pub season: ReportColumns,
}

/// What `stats_config()` returns: the reports each endpoint family offers.
/// The keys of the report maps are the valid `{report}` names for
/// `skater_report()`, `goalie_report()` and `team_report()`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatsConfig {
    pub player_report_data: BTreeMap<String, ReportConfig>,
    pub goalie_report_data: BTreeMap<String, ReportConfig>,
    pub team_report_data: BTreeMap<String, ReportConfig>,
    /// Columns an aggregate query leaves out of its rows.
    pub aggregated_columns: Vec<String>,
    /// Columns an aggregate query adds (the franchise id and name of team rows).
    pub individual_columns: Vec<String>,
}

#[cfg(test)]
#[path = "stats_rest_test.rs"]
mod tests;
