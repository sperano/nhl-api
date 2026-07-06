//! Edge goalie stats: `/edge/goalie-*` endpoints.
//!
//! See the module-level documentation in [`super`](crate::types::edge) for the
//! two design rules every type here obeys (deserialize-from-`{}` and
//! no-`Option`-on-plain-scalar-counts) and the field-naming gotchas.
//!
//! ## Gotcha A — `EdgeGoalieSavePctgDetail.savePctgDetails` is an object
//!
//! Unlike most `*Details` fields elsewhere in the Edge module (which are
//! arrays), [`EdgeGoalieSavePctgDetail::save_pctg_details`] is a single
//! nullable object ([`EdgeGoalieSavePctgStatDetail`]), matching Go's fixed
//! shape (commit `cfedbf1`) after an earlier version incorrectly typed it as
//! an array. Its two fields are independently nullable too, mirroring Go's
//! pointer fields exactly.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::common::{
    EdgeGoaliePlayer, EdgeLeaderShotLocation, EdgeOverlayTeam, EdgeSeasonAvailability,
};

/// Response from `edge/goalie-detail/{goalie}/{season}/{gameType}`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct EdgeGoalieDetail {
    pub player: EdgeGoaliePlayer,
    pub seasons_with_edge_stats: Vec<EdgeSeasonAvailability>,
    pub stats: EdgeGoalieStatsSummary,
    pub shot_location_summary: Vec<EdgeGoalieShotLocationSummary>,
    pub shot_location_details: Vec<EdgeGoalieShotLocationArea>,
}

/// Top-level goalie stat entries, embedded in [`EdgeGoalieDetail`].
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct EdgeGoalieStatsSummary {
    pub goals_against_avg: EdgeGoalieStatEntry,
    pub games_above_900: EdgeGoalieStatEntry,
    pub goal_differential_per_60: EdgeGoalieStatEntry,
    pub goal_support_avg: EdgeGoalieStatEntry,
    pub point_pctg: EdgeGoalieStatEntry,
}

/// A single goalie stat with percentile and league average.
///
/// Unlike the shared [`super::common::EdgeCountPercentileStat`], the goalie
/// endpoints send `leagueAvg` as a plain number here, not a nested
/// `{value: ...}` object — keep this a distinct type rather than unifying.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct EdgeGoalieStatEntry {
    pub value: f64,
    pub percentile: f64,
    pub league_avg: f64,
}

/// A shot-location summary by location code, embedded in [`EdgeGoalieDetail`].
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct EdgeGoalieShotLocationSummary {
    pub location_code: String,
    pub goals_against: i32,
    pub goals_against_percentile: f64,
    pub goals_against_league_avg: f64,
    pub saves: i32,
    pub saves_percentile: f64,
    pub saves_league_avg: f64,
    pub save_pctg: f64,
    pub save_pctg_percentile: f64,
    pub save_pctg_league_avg: f64,
}

/// A shot-location detail for a specific rink area, embedded in
/// [`EdgeGoalieDetail`].
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct EdgeGoalieShotLocationArea {
    pub area: String,
    pub saves: i32,
    pub saves_percentile: f64,
    pub save_pctg: f64,
    pub save_pctg_percentile: f64,
}

/// Response from `edge/goalie-5v5-detail/{goalie}/{season}/{gameType}`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct EdgeGoalie5v5Detail {
    pub player: EdgeGoaliePlayer,
    pub seasons_with_edge_stats: Vec<EdgeSeasonAvailability>,
    pub save_pctg_5v5_last10: Vec<EdgeGoalie5v5Entry>,
}

/// A per-game 5v5 save-percentage entry, embedded in [`EdgeGoalie5v5Detail`].
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct EdgeGoalie5v5Entry {
    pub game_date: String,
    pub away_team: EdgeOverlayTeam,
    pub home_team: EdgeOverlayTeam,
    pub save_pctg: f64,
}

/// Response from `edge/goalie-shot-location-detail/{goalie}/{season}/{gameType}`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct EdgeGoalieShotLocationDetail {
    pub player: EdgeGoaliePlayer,
    pub seasons_with_edge_stats: Vec<EdgeSeasonAvailability>,
    pub shot_location_details: Vec<EdgeGoalieShotLocationEntry>,
}

/// A per-area shot-location detail, embedded in
/// [`EdgeGoalieShotLocationDetail`].
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct EdgeGoalieShotLocationEntry {
    pub area: String,
    pub saves: i32,
    pub save_pctg: f64,
}

/// Response from `edge/goalie-save-percentage-detail/{goalie}/{season}/{gameType}`.
///
/// Gotcha A: `save_pctg_details` is a single nullable object, not an array.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct EdgeGoalieSavePctgDetail {
    pub player: EdgeGoaliePlayer,
    pub seasons_with_edge_stats: Vec<EdgeSeasonAvailability>,
    pub save_pctg_last10: Vec<EdgeGoalieSavePctgEntry>,
    /// Gotcha A: an object (`{gamesAbove900, pctgGamesAbove900}`), not an
    /// array — genuinely nullable, matching Go's `*EdgeGoalieSavePctgStatDetail`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub save_pctg_details: Option<EdgeGoalieSavePctgStatDetail>,
}

/// Aggregated save-percentage statistics (gotcha A). Both fields are
/// independently nullable, mirroring Go's pointer fields.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct EdgeGoalieSavePctgStatDetail {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub games_above_900: Option<EdgeGoalieStatEntry>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pctg_games_above_900: Option<EdgeGoalieStatEntry>,
}

/// A per-game save-percentage entry, embedded in [`EdgeGoalieSavePctgDetail`].
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct EdgeGoalieSavePctgEntry {
    pub game_date: String,
    pub away_team: EdgeOverlayTeam,
    pub home_team: EdgeOverlayTeam,
    pub save_pctg: f64,
}

/// Shot totals by location code, used in [`EdgeGoalieComparison`].
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct EdgeGoalieComparisonShotSummary {
    pub location_code: String,
    pub shots_against: i32,
    pub goals_against: i32,
    pub saves: i32,
    pub save_pctg: f64,
}

/// Shot breakdown by rink area, used in [`EdgeGoalieComparison`].
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct EdgeGoalieComparisonShotDetail {
    pub area: String,
    pub shots_against: i32,
    pub goals_against: i32,
    pub saves: i32,
    pub save_pctg: f64,
}

/// Overall save-percentage details, used in [`EdgeGoalieComparison`].
///
/// Unlike [`EdgeGoalieSavePctgStatDetail`] (gotcha A), these fields are
/// plain scalars in Go (no pointer/`omitempty`) — only the parent field on
/// [`EdgeGoalieComparison`] is nullable.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct EdgeGoalieComparisonSavePctgDetails {
    pub games_above_900: i32,
    pub pctg_games_above_900: f64,
    pub point_pctg: f64,
    pub goals_against_avg: f64,
    pub save_pctg: f64,
}

/// 5v5 save-percentage details, used in [`EdgeGoalieComparison`].
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct EdgeGoalieComparisonSavePctg5v5Details {
    pub save_pctg: f64,
    pub save_pctg_close: f64,
    pub shots: i32,
    pub shots_per_60: f64,
}

/// A game entry in a goalie comparison's last-10 arrays.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct EdgeGoalieComparisonLast10Entry {
    pub game_date: String,
    pub save_pctg: f64,
    pub shots_against: i32,
    pub goals_against: i32,
}

/// Response from `edge/goalie-comparison/{goalie}/{season}/{gameType}`.
///
/// A rich composite for head-to-head display; each detail sub-object is
/// genuinely nullable (only populated when the comparison includes that
/// category).
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct EdgeGoalieComparison {
    pub player: EdgeGoaliePlayer,
    pub seasons_with_edge_stats: Vec<EdgeSeasonAvailability>,
    pub shot_location_summary: Vec<EdgeGoalieComparisonShotSummary>,
    pub shot_location_details: Vec<EdgeGoalieComparisonShotDetail>,
    pub save_pctg_5v5_last10: Vec<EdgeGoalieComparisonLast10Entry>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub save_pctg_5v5_details: Option<EdgeGoalieComparisonSavePctg5v5Details>,
    pub save_pctg_last10: Vec<EdgeGoalieComparisonLast10Entry>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub save_pctg_details: Option<EdgeGoalieComparisonSavePctgDetails>,
}

/// A single leader entry in [`EdgeGoalieLanding::leaders`].
///
/// Only the stat field(s) relevant to the entry's category key are populated;
/// the rest are `None`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct EdgeGoalieLeader {
    pub player: EdgeGoaliePlayer,
    /// Populated for the `gamesAbove900` category.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub games: Option<i32>,
    /// Populated for the `highDangerGoalsAgainst` category.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goals_against: Option<i32>,
    /// Populated for the `highDangerSavePctg` / `savePctg5v5` categories.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub save_pctg: Option<f64>,
    /// Populated for the `highDangerSaves` category.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub saves: Option<i32>,
    pub shot_location_details: Vec<EdgeLeaderShotLocation>,
}

/// Response from `edge/goalie-landing/{season}/{gameType}` (no goalie id —
/// this is the league-wide leaderboard).
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct EdgeGoalieLanding {
    pub seasons_with_edge_stats: Vec<EdgeSeasonAvailability>,
    /// Keyed by leader category (e.g. `"gamesAbove900"`, `"highDangerSaves"`).
    pub leaders: HashMap<String, EdgeGoalieLeader>,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Asserts a type deserializes from an empty JSON object into its
    /// `Default` value (mirrors the guard in `edge::common`'s tests).
    fn assert_empty_object_deserializes<T>()
    where
        T: serde::de::DeserializeOwned + Default + PartialEq + std::fmt::Debug,
    {
        let value: T = serde_json::from_str("{}").expect("must deserialize from {}");
        assert_eq!(value, T::default(), "{{}} must equal Default");
    }

    // ----- deserialize-from-`{}` guard, one per struct -----

    #[test]
    fn test_edge_goalie_detail_empty_object() {
        assert_empty_object_deserializes::<EdgeGoalieDetail>();
    }

    #[test]
    fn test_edge_goalie_stats_summary_empty_object() {
        assert_empty_object_deserializes::<EdgeGoalieStatsSummary>();
    }

    #[test]
    fn test_edge_goalie_stat_entry_empty_object() {
        assert_empty_object_deserializes::<EdgeGoalieStatEntry>();
    }

    #[test]
    fn test_edge_goalie_shot_location_summary_empty_object() {
        assert_empty_object_deserializes::<EdgeGoalieShotLocationSummary>();
    }

    #[test]
    fn test_edge_goalie_shot_location_area_empty_object() {
        assert_empty_object_deserializes::<EdgeGoalieShotLocationArea>();
    }

    #[test]
    fn test_edge_goalie_5v5_detail_empty_object() {
        assert_empty_object_deserializes::<EdgeGoalie5v5Detail>();
    }

    #[test]
    fn test_edge_goalie_5v5_entry_empty_object() {
        assert_empty_object_deserializes::<EdgeGoalie5v5Entry>();
    }

    #[test]
    fn test_edge_goalie_shot_location_detail_empty_object() {
        assert_empty_object_deserializes::<EdgeGoalieShotLocationDetail>();
    }

    #[test]
    fn test_edge_goalie_shot_location_entry_empty_object() {
        assert_empty_object_deserializes::<EdgeGoalieShotLocationEntry>();
    }

    #[test]
    fn test_edge_goalie_save_pctg_detail_empty_object() {
        assert_empty_object_deserializes::<EdgeGoalieSavePctgDetail>();
    }

    #[test]
    fn test_edge_goalie_save_pctg_stat_detail_empty_object() {
        assert_empty_object_deserializes::<EdgeGoalieSavePctgStatDetail>();
    }

    #[test]
    fn test_edge_goalie_save_pctg_entry_empty_object() {
        assert_empty_object_deserializes::<EdgeGoalieSavePctgEntry>();
    }

    #[test]
    fn test_edge_goalie_comparison_shot_summary_empty_object() {
        assert_empty_object_deserializes::<EdgeGoalieComparisonShotSummary>();
    }

    #[test]
    fn test_edge_goalie_comparison_shot_detail_empty_object() {
        assert_empty_object_deserializes::<EdgeGoalieComparisonShotDetail>();
    }

    #[test]
    fn test_edge_goalie_comparison_save_pctg_details_empty_object() {
        assert_empty_object_deserializes::<EdgeGoalieComparisonSavePctgDetails>();
    }

    #[test]
    fn test_edge_goalie_comparison_save_pctg_5v5_details_empty_object() {
        assert_empty_object_deserializes::<EdgeGoalieComparisonSavePctg5v5Details>();
    }

    #[test]
    fn test_edge_goalie_comparison_last10_entry_empty_object() {
        assert_empty_object_deserializes::<EdgeGoalieComparisonLast10Entry>();
    }

    #[test]
    fn test_edge_goalie_comparison_empty_object() {
        assert_empty_object_deserializes::<EdgeGoalieComparison>();
    }

    #[test]
    fn test_edge_goalie_leader_empty_object() {
        assert_empty_object_deserializes::<EdgeGoalieLeader>();
    }

    #[test]
    fn test_edge_goalie_landing_empty_object() {
        assert_empty_object_deserializes::<EdgeGoalieLanding>();
    }

    // ----- fixture deserialization, ported from Go edge_test.go -----

    /// Ported from Go's `TestEdgeGoalieDetail_Deserialization`
    /// (`edge_test.go:162-242`).
    #[test]
    fn test_edge_goalie_detail_deserializes_fixture() {
        let json = r#"{
            "player": {
                "id": 8479318,
                "firstName": {"default": "Igor"},
                "lastName": {"default": "Shesterkin"},
                "birthDate": "1995-12-30",
                "shootsCatches": "L",
                "sweaterNumber": 31,
                "slug": "igor-shesterkin-8479318",
                "headshot": "https://assets.nhle.com/mugs/nhl/20242025/NYR/8479318.png",
                "wins": 25,
                "losses": 10,
                "overtimeLosses": 3,
                "goalsAgainstAvg": 2.15,
                "savePctg": 0.928,
                "gamesPlayed": 38,
                "team": {
                    "id": 3,
                    "commonName": {"default": "Rangers"},
                    "placeNameWithPreposition": {"default": "New York"},
                    "abbrev": "NYR",
                    "teamLogo": {"light": "https://assets.nhle.com/logos/nhl/svg/NYR_light.svg", "dark": "https://assets.nhle.com/logos/nhl/svg/NYR_dark.svg"},
                    "slug": "new-york-rangers",
                    "conference": "Eastern",
                    "division": "Metropolitan",
                    "wins": 30,
                    "losses": 18,
                    "otLosses": 7,
                    "gamesPlayed": 55,
                    "points": 67
                }
            },
            "seasonsWithEdgeStats": [{"id": 20242025, "gameTypes": [2]}],
            "stats": {
                "goalsAgainstAvg": {"value": 2.15, "percentile": 0.90, "leagueAvg": 2.85},
                "gamesAbove900": {"value": 28.0, "percentile": 0.92, "leagueAvg": 18.5},
                "goalDifferentialPer60": {"value": 1.2, "percentile": 0.88, "leagueAvg": 0.5},
                "goalSupportAvg": {"value": 3.1, "percentile": 0.65, "leagueAvg": 3.0},
                "pointPctg": {"value": 0.68, "percentile": 0.85, "leagueAvg": 0.55}
            },
            "shotLocationSummary": [
                {
                    "locationCode": "all",
                    "goalsAgainst": 80,
                    "goalsAgainstPercentile": 0.85,
                    "goalsAgainstLeagueAvg": 100.0,
                    "saves": 1000,
                    "savesPercentile": 0.90,
                    "savesLeagueAvg": 850.0,
                    "savePctg": 0.926,
                    "savePctgPercentile": 0.88,
                    "savePctgLeagueAvg": 0.905
                }
            ],
            "shotLocationDetails": [
                {"area": "Crease", "saves": 200, "savesPercentile": 0.85, "savePctg": 0.88, "savePctgPercentile": 0.80}
            ]
        }"#;

        let detail: EdgeGoalieDetail = serde_json::from_str(json).expect("must deserialize");

        assert_eq!(detail.player.id, 8479318);
        assert_eq!(detail.player.save_pctg, 0.928);
        assert_eq!(detail.player.team.abbrev, "NYR");
        assert_eq!(detail.seasons_with_edge_stats.len(), 1);
        assert_eq!(detail.stats.goals_against_avg.value, 2.15);
        assert_eq!(detail.stats.games_above_900.league_avg, 18.5);
        assert_eq!(detail.stats.goal_differential_per_60.value, 1.2);
        assert_eq!(detail.stats.goal_support_avg.percentile, 0.65);
        assert_eq!(detail.stats.point_pctg.value, 0.68);
        assert_eq!(detail.shot_location_summary.len(), 1);
        assert_eq!(detail.shot_location_summary[0].save_pctg, 0.926);
        assert_eq!(detail.shot_location_summary[0].goals_against, 80);
        assert_eq!(detail.shot_location_details.len(), 1);
        assert_eq!(detail.shot_location_details[0].area, "Crease");
        assert_eq!(detail.shot_location_details[0].saves, 200);
    }

    /// Ported from Go's `TestEdgeGoalie5v5Detail_Deserialization`
    /// (`edge_test.go:416-445`).
    #[test]
    fn test_edge_goalie_5v5_detail_deserializes_fixture() {
        let json = r#"{
            "player": {
                "id": 8479318,
                "firstName": {"default": "Igor"}, "lastName": {"default": "Shesterkin"},
                "birthDate": "1995-12-30", "shootsCatches": "L", "sweaterNumber": 31,
                "slug": "igor-shesterkin-8479318", "headshot": "h",
                "wins": 25, "losses": 10, "overtimeLosses": 3, "goalsAgainstAvg": 2.15,
                "savePctg": 0.928, "gamesPlayed": 38,
                "team": {"id": 3, "commonName": {"default": "Rangers"}, "placeNameWithPreposition": {"default": "New York"}, "abbrev": "NYR", "teamLogo": {"light": "l", "dark": "d"}, "slug": "s", "conference": "E", "division": "M", "wins": 30, "losses": 18, "otLosses": 7, "gamesPlayed": 55, "points": 67}
            },
            "seasonsWithEdgeStats": [{"id": 20242025, "gameTypes": [2]}],
            "savePctg5v5Last10": [
                {"gameDate": "2025-02-01", "awayTeam": {"abbrev": "NYR", "score": 3}, "homeTeam": {"abbrev": "BOS", "score": 1}, "savePctg": 0.950},
                {"gameDate": "2025-01-28", "awayTeam": {"abbrev": "PIT", "score": 2}, "homeTeam": {"abbrev": "NYR", "score": 4}, "savePctg": 0.935}
            ]
        }"#;

        let detail: EdgeGoalie5v5Detail = serde_json::from_str(json).expect("must deserialize");

        assert_eq!(detail.save_pctg_5v5_last10.len(), 2);
        assert_eq!(detail.save_pctg_5v5_last10[0].save_pctg, 0.950);
        assert_eq!(detail.save_pctg_5v5_last10[0].home_team.abbrev, "BOS");
        assert_eq!(detail.save_pctg_5v5_last10[1].away_team.abbrev, "PIT");
    }

    /// Shot-location-detail fixture. No dedicated Go test exists for this
    /// type; keys are verified directly against the `EdgeGoalieShotLocationDetail`
    /// / `EdgeGoalieShotLocationEntry` struct tags in Go's `edge_goalie.go`.
    #[test]
    fn test_edge_goalie_shot_location_detail_deserializes_fixture() {
        let json = r#"{
            "player": {"id": 8479318, "firstName": {"default": "Igor"}, "lastName": {"default": "Shesterkin"}},
            "seasonsWithEdgeStats": [{"id": 20242025, "gameTypes": [2]}],
            "shotLocationDetails": [
                {"area": "Crease", "saves": 40, "savePctg": 0.85}
            ]
        }"#;

        let detail: EdgeGoalieShotLocationDetail =
            serde_json::from_str(json).expect("must deserialize");

        assert_eq!(detail.shot_location_details.len(), 1);
        assert_eq!(detail.shot_location_details[0].area, "Crease");
        assert_eq!(detail.shot_location_details[0].saves, 40);
        assert_eq!(detail.shot_location_details[0].save_pctg, 0.85);
    }

    /// Gotcha A, mandatory fixture. Ported from Go's
    /// `TestEdgeGoalieSavePctgDetail_RealAPIStructure` (`edge_test.go:615-704`):
    /// `savePctgDetails` is a real API object (`{gamesAbove900,
    /// pctgGamesAbove900}`), not an array — this was a fixed bug in Go
    /// (commit `cfedbf1`) and the case this type exists to encode.
    #[test]
    fn test_edge_goalie_save_pctg_detail_deserializes_fixture_gotcha_a() {
        let json = r#"{
            "player": {
                "id": 8480382,
                "firstName": {"default": "Joonas"},
                "lastName": {"default": "Korpisalo"},
                "birthDate": "1994-04-28",
                "shootsCatches": "L",
                "sweaterNumber": 70,
                "slug": "joonas-korpisalo-8480382",
                "headshot": "https://assets.nhle.com/mugs/nhl/20212022/CBJ/8480382.png",
                "wins": 0,
                "losses": 1,
                "overtimeLosses": 0,
                "goalsAgainstAvg": 6.0,
                "savePctg": 0.75,
                "gamesPlayed": 1,
                "team": {
                    "id": 29,
                    "commonName": {"default": "Blue Jackets"},
                    "placeNameWithPreposition": {"default": "Columbus"},
                    "abbrev": "CBJ",
                    "teamLogo": {"light": "l", "dark": "d"},
                    "slug": "columbus-blue-jackets",
                    "conference": "Eastern",
                    "division": "Metropolitan",
                    "wins": 0,
                    "losses": 4,
                    "otLosses": 0,
                    "gamesPlayed": 4,
                    "points": 0
                }
            },
            "seasonsWithEdgeStats": [{"id": 20212022, "gameTypes": [2, 3]}],
            "savePctgLast10": [
                {"gameDate": "2022-05-02", "awayTeam": {"abbrev": "CBJ", "score": 0}, "homeTeam": {"abbrev": "TBL", "score": 5}, "savePctg": 0.75}
            ],
            "savePctgDetails": {
                "gamesAbove900": {"value": 0, "percentile": 0.0, "leagueAvg": 4.4},
                "pctgGamesAbove900": {"value": 0.0, "percentile": 0.0, "leagueAvg": 0.618}
            }
        }"#;

        let detail: EdgeGoalieSavePctgDetail =
            serde_json::from_str(json).expect("must deserialize");

        assert_eq!(detail.player.id, 8480382);
        assert_eq!(detail.save_pctg_last10.len(), 1);
        assert_eq!(detail.save_pctg_last10[0].save_pctg, 0.75);

        let stat_detail = detail
            .save_pctg_details
            .as_ref()
            .expect("savePctgDetails present (not an array)");
        let games_above_900 = stat_detail
            .games_above_900
            .as_ref()
            .expect("gamesAbove900 present");
        assert_eq!(games_above_900.value, 0.0);
        assert_eq!(games_above_900.league_avg, 4.4);
        let pctg_games_above_900 = stat_detail
            .pctg_games_above_900
            .as_ref()
            .expect("pctgGamesAbove900 present");
        assert_eq!(pctg_games_above_900.league_avg, 0.618);

        // Round-trip, mirroring the Go test's marshal/unmarshal assertion.
        let json = serde_json::to_string(&detail).unwrap();
        let round_trip: EdgeGoalieSavePctgDetail =
            serde_json::from_str(&json).expect("round-trip deserialize");
        assert_eq!(round_trip, detail);
    }

    /// `savePctgDetails` absent must deserialize to `None`, not an empty
    /// object or an error — the counterpart case to the gotcha A fixture
    /// above.
    #[test]
    fn test_edge_goalie_save_pctg_detail_missing_details_is_none() {
        let json = r#"{"player": {"id": 1}, "savePctgLast10": []}"#;
        let detail: EdgeGoalieSavePctgDetail =
            serde_json::from_str(json).expect("must deserialize");
        assert_eq!(detail.save_pctg_details, None);
    }

    /// Ported from Go's `TestEdgeGoalieComparison_RealAPIStructure`
    /// (`edge_test.go:706-777`).
    #[test]
    fn test_edge_goalie_comparison_deserializes_fixture() {
        let json = r#"{
            "player": {
                "id": 8480382,
                "firstName": {"default": "Joonas"},
                "lastName": {"default": "Korpisalo"},
                "birthDate": "1994-04-28",
                "shootsCatches": "L",
                "sweaterNumber": 70,
                "slug": "joonas-korpisalo",
                "headshot": "h",
                "wins": 0,
                "losses": 1,
                "overtimeLosses": 0,
                "goalsAgainstAvg": 6.0,
                "savePctg": 0.75,
                "gamesPlayed": 1,
                "team": {"id": 29, "commonName": {"default": "Blue Jackets"}, "placeNameWithPreposition": {"default": "Columbus"}, "abbrev": "CBJ", "teamLogo": {"light": "l", "dark": "d"}, "slug": "s", "conference": "E", "division": "M", "wins": 0, "losses": 4, "otLosses": 0, "gamesPlayed": 4, "points": 0}
            },
            "seasonsWithEdgeStats": [{"id": 20212022, "gameTypes": [2, 3]}],
            "shotLocationSummary": [
                {"locationCode": "all", "shotsAgainst": 24, "goalsAgainst": 6, "saves": 18, "savePctg": 0.75}
            ],
            "shotLocationDetails": [
                {"area": "Crease", "shotsAgainst": 8, "goalsAgainst": 3, "saves": 5, "savePctg": 0.625}
            ],
            "savePctg5v5Last10": [
                {"gameDate": "2022-05-02", "savePctg": 0.75, "shotsAgainst": 20, "goalsAgainst": 5}
            ],
            "savePctg5v5Details": {
                "savePctg": 0.75,
                "savePctgClose": 0.80,
                "shots": 20,
                "shotsPer60": 45.0
            },
            "savePctgLast10": [
                {"gameDate": "2022-05-02", "savePctg": 0.75, "shotsAgainst": 24, "goalsAgainst": 6}
            ],
            "savePctgDetails": {
                "gamesAbove900": 0,
                "pctgGamesAbove900": 0.0,
                "pointPctg": 0.0,
                "goalsAgainstAvg": 6.0,
                "savePctg": 0.75
            }
        }"#;

        let comparison: EdgeGoalieComparison =
            serde_json::from_str(json).expect("must deserialize");

        assert_eq!(comparison.player.id, 8480382);
        assert_eq!(comparison.shot_location_summary.len(), 1);
        assert_eq!(comparison.shot_location_summary[0].shots_against, 24);
        assert_eq!(comparison.shot_location_details.len(), 1);
        assert_eq!(comparison.shot_location_details[0].saves, 5);
        assert_eq!(comparison.save_pctg_5v5_last10.len(), 1);
        assert_eq!(comparison.save_pctg_5v5_last10[0].goals_against, 5);

        let five_v_five = comparison
            .save_pctg_5v5_details
            .expect("savePctg5v5Details present");
        assert_eq!(five_v_five.shots_per_60, 45.0);
        assert_eq!(five_v_five.save_pctg_close, 0.80);

        assert_eq!(comparison.save_pctg_last10.len(), 1);
        assert_eq!(comparison.save_pctg_last10[0].shots_against, 24);

        let details = comparison
            .save_pctg_details
            .expect("savePctgDetails present");
        assert_eq!(details.goals_against_avg, 6.0);
        assert_eq!(details.games_above_900, 0);
        assert_eq!(details.save_pctg, 0.75);
    }

    /// Landing fixture exercising every `EdgeGoalieLeader` stat-field tag
    /// against real category keys. No dedicated Go fixture exists for goalie
    /// landing (only the skater-landing shape is exercised in `edge_test.go`),
    /// so this covers what the plan flags as previously-unverified via
    /// `{}`-only tests.
    #[test]
    fn test_edge_goalie_landing_deserializes_fixture() {
        let json = r#"{
            "seasonsWithEdgeStats": [{"id": 20242025, "gameTypes": [2]}],
            "leaders": {
                "gamesAbove900": {
                    "player": {"id": 8479318, "firstName": {"default": "Igor"}, "lastName": {"default": "Shesterkin"}},
                    "games": 30
                },
                "highDangerGoalsAgainst": {
                    "player": {"id": 8480382, "firstName": {"default": "Joonas"}, "lastName": {"default": "Korpisalo"}},
                    "goalsAgainst": 12
                },
                "highDangerSavePctg": {
                    "player": {"id": 8479318, "firstName": {"default": "Igor"}, "lastName": {"default": "Shesterkin"}},
                    "savePctg": 0.82,
                    "shotLocationDetails": [
                        {"area": "HighDanger", "savePctg": 0.82, "savePctgPercentile": 0.77}
                    ]
                },
                "highDangerSaves": {
                    "player": {"id": 8479318, "firstName": {"default": "Igor"}, "lastName": {"default": "Shesterkin"}},
                    "saves": 55
                },
                "savePctg5v5": {
                    "player": {"id": 8479318, "firstName": {"default": "Igor"}, "lastName": {"default": "Shesterkin"}},
                    "savePctg": 0.91
                }
            }
        }"#;

        let landing: EdgeGoalieLanding = serde_json::from_str(json).expect("must deserialize");

        assert_eq!(landing.seasons_with_edge_stats.len(), 1);
        assert_eq!(landing.leaders.len(), 5);

        let games_above_900 = landing.leaders.get("gamesAbove900").expect("present");
        assert_eq!(games_above_900.games, Some(30));
        assert_eq!(games_above_900.goals_against, None);

        let high_danger_ga = landing
            .leaders
            .get("highDangerGoalsAgainst")
            .expect("present");
        assert_eq!(high_danger_ga.goals_against, Some(12));
        assert_eq!(high_danger_ga.games, None);

        let high_danger_save_pctg = landing.leaders.get("highDangerSavePctg").expect("present");
        assert_eq!(high_danger_save_pctg.save_pctg, Some(0.82));
        assert_eq!(high_danger_save_pctg.shot_location_details.len(), 1);
        assert_eq!(
            high_danger_save_pctg.shot_location_details[0].save_pctg,
            Some(0.82)
        );
        assert_eq!(
            high_danger_save_pctg.shot_location_details[0].save_pctg_percentile,
            Some(0.77)
        );

        let high_danger_saves = landing.leaders.get("highDangerSaves").expect("present");
        assert_eq!(high_danger_saves.saves, Some(55));
        assert_eq!(high_danger_saves.save_pctg, None);

        let save_pctg_5v5 = landing.leaders.get("savePctg5v5").expect("present");
        assert_eq!(save_pctg_5v5.save_pctg, Some(0.91));
        assert_eq!(save_pctg_5v5.saves, None);
    }

    #[test]
    fn test_edge_goalie_leader_mutually_exclusive_stat_fields() {
        let json = r#"{"player": {"id": 1}, "saves": 12}"#;
        let leader: EdgeGoalieLeader = serde_json::from_str(json).expect("must deserialize");

        assert_eq!(leader.saves, Some(12));
        assert_eq!(leader.games, None);
        assert_eq!(leader.goals_against, None);
        assert_eq!(leader.save_pctg, None);
        assert!(leader.shot_location_details.is_empty());
    }
}
