//! Edge team stats: `/edge/team-*` endpoints.
//!
//! See the module-level documentation in [`super`](crate::types::edge) for the
//! two design rules every type here obeys (deserialize-from-`{}` and
//! no-`Option`-on-plain-scalar-counts) and the field-naming gotchas.
//!
//! Team Edge stats are **rank**-based (1-32), not percentile-based like the
//! skater/goalie endpoints — see [`super::common::EdgeRankStat`] /
//! [`super::common::EdgeRankStatWithOverlay`], which are deliberately not
//! unified with the percentile-based stats.
//!
//! ## Gotcha B — `EdgeTeamZoneTimeDetails` is distinct from the embedded summary
//!
//! [`EdgeTeamZoneTimeDetails`] is the response from the standalone
//! `edge/team-zone-time-details` endpoint. It is **not** the same shape as
//! [`EdgeTeamZoneTime`] (the zone-time summary embedded in [`EdgeTeamDetail`]):
//! it breaks zone time down by strength code (`all`/`es`/`pp`/`pk`) and adds a
//! `shotDifferential` object. `shotDifferential` is a single object
//! (`{shotAttemptDifferential, shotAttemptDifferentialRank, sogDifferential,
//! sogDifferentialRank}` — the SOG pair's JSON key is `sogDifferential`, not a
//! `shotAttemptDifferential`-style name), matching Go's fixed shape (commit
//! `f3ada28`) after an earlier version incorrectly modeled it as a
//! per-strength-code array. The parent's `team` and `seasonsWithEdgeStats` are
//! genuinely nullable here — unlike every other top-level Edge response type —
//! and [`EdgeTeamZoneTimeByStrength`]'s three league-average fields are
//! nullable too (same commit added `omitempty` after finding the API omits
//! them for some strength codes).

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::common::{
    EdgeComparisonDistanceLast10Entry, EdgeComparisonShotLocationDetail,
    EdgeComparisonShotLocationTotal, EdgeComparisonShotSpeedDetails,
    EdgeComparisonSkatingDistanceDetails, EdgeComparisonSkatingSpeedDetails,
    EdgeComparisonZoneTimeDetails, EdgeLeaderShotLocation, EdgeMeasurement, EdgeOverlayPlayer,
    EdgeOverlayTeam, EdgeRankStat, EdgeRankStatWithOverlay, EdgeSeasonAvailability, EdgeTeamInfo,
};

/// Response from `edge/team-detail/{team}/{season}/{gameType}`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct EdgeTeamDetail {
    pub team: EdgeTeamInfo,
    pub seasons_with_edge_stats: Vec<EdgeSeasonAvailability>,
    pub shot_speed: EdgeTeamShotSpeed,
    pub skating_speed: EdgeTeamSkatingSpeed,
    pub distance_skated: EdgeTeamDistance,
    pub sog_summary: Vec<EdgeTeamSogSummary>,
    pub sog_details: Vec<EdgeTeamSogAreaDetail>,
    pub zone_time_details: EdgeTeamZoneTime,
}

/// Team shot-speed stats, embedded in [`EdgeTeamDetail`].
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct EdgeTeamShotSpeed {
    pub shot_attempts_over_90: EdgeRankStat,
    pub top_shot_speed: EdgeRankStatWithOverlay,
}

/// Team skating-speed stats, embedded in [`EdgeTeamDetail`].
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct EdgeTeamSkatingSpeed {
    pub bursts_over_22: EdgeRankStat,
    pub bursts_over_20: EdgeRankStat,
    pub speed_max: EdgeRankStatWithOverlay,
}

/// Team distance-skated stats, embedded in [`EdgeTeamDetail`].
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct EdgeTeamDistance {
    pub total: EdgeRankStat,
}

/// Team shot-on-goal summary for a rink area, embedded in [`EdgeTeamDetail`].
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct EdgeTeamSogSummary {
    pub location_code: String,
    pub shots: i32,
    pub shots_rank: i32,
    pub shots_league_avg: f64,
    pub goals: i32,
    pub goals_rank: i32,
    pub goals_league_avg: f64,
    pub shooting_pctg: f64,
    pub shooting_pctg_rank: i32,
    pub shooting_pctg_league_avg: f64,
}

/// Team shot-on-goal detail for a specific rink area, embedded in
/// [`EdgeTeamDetail`].
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct EdgeTeamSogAreaDetail {
    pub area: String,
    pub shots: i32,
    pub shots_rank: i32,
}

/// Team zone-time percentages and ranks, embedded in [`EdgeTeamDetail`].
///
/// Distinct from [`EdgeTeamZoneTimeByStrength`] (used by the standalone
/// `edge/team-zone-time-details` endpoint, see [`EdgeTeamZoneTimeDetails`]) —
/// this is a single all-strengths summary, not broken down by strength code.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct EdgeTeamZoneTime {
    pub offensive_zone_pctg: f64,
    pub offensive_zone_rank: i32,
    pub offensive_zone_league_avg: f64,
    pub offensive_zone_ev_pctg: f64,
    pub offensive_zone_ev_rank: i32,
    pub neutral_zone_pctg: f64,
    pub neutral_zone_rank: i32,
    pub neutral_zone_league_avg: f64,
    pub defensive_zone_pctg: f64,
    pub defensive_zone_rank: i32,
    pub defensive_zone_league_avg: f64,
}

/// Response from `edge/team-skating-speed-detail/{team}/{season}/{gameType}`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct EdgeTeamSpeedDetail {
    pub team: EdgeTeamInfo,
    pub seasons_with_edge_stats: Vec<EdgeSeasonAvailability>,
    pub top_skating_speeds: Vec<EdgeTeamSpeedEntry>,
}

/// A per-player skating-speed entry within a team, embedded in
/// [`EdgeTeamSpeedDetail`].
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct EdgeTeamSpeedEntry {
    pub player: EdgeOverlayPlayer,
    pub speed: EdgeMeasurement,
}

/// Response from `edge/team-skating-distance-detail/{team}/{season}/{gameType}`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct EdgeTeamDistanceDetail {
    pub team: EdgeTeamInfo,
    pub seasons_with_edge_stats: Vec<EdgeSeasonAvailability>,
    pub skating_distance_last10: Vec<EdgeTeamDistanceEntry>,
}

/// A per-game team distance entry, embedded in [`EdgeTeamDistanceDetail`].
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct EdgeTeamDistanceEntry {
    pub game_date: String,
    pub away_team: EdgeOverlayTeam,
    pub home_team: EdgeOverlayTeam,
    pub distance: EdgeMeasurement,
}

/// Response from `edge/team-shot-speed-detail/{team}/{season}/{gameType}`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct EdgeTeamShotSpeedDetail {
    pub team: EdgeTeamInfo,
    pub seasons_with_edge_stats: Vec<EdgeSeasonAvailability>,
    pub hardest_shots: Vec<EdgeTeamShotSpeedEntry>,
}

/// A per-player shot-speed entry within a team, embedded in
/// [`EdgeTeamShotSpeedDetail`].
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct EdgeTeamShotSpeedEntry {
    pub player: EdgeOverlayPlayer,
    pub speed: EdgeMeasurement,
}

/// Response from `edge/team-shot-location-detail/{team}/{season}/{gameType}`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct EdgeTeamShotLocationDetail {
    pub team: EdgeTeamInfo,
    pub seasons_with_edge_stats: Vec<EdgeSeasonAvailability>,
    pub shot_location_details: Vec<EdgeTeamShotLocationEntry>,
}

/// A shot-location breakdown for a rink area, embedded in
/// [`EdgeTeamShotLocationDetail`].
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct EdgeTeamShotLocationEntry {
    pub area: String,
    pub shots: i32,
    pub goals: i32,
}

/// Response from `edge/team-zone-time-details/{team}/{season}/{gameType}`.
///
/// Gotcha B: distinct from [`EdgeTeamZoneTime`] (the summary embedded in
/// [`EdgeTeamDetail`]) — this breaks zone time down by strength code and adds
/// a shot-differential object. `team` and `seasons_with_edge_stats` are
/// genuinely nullable here, unlike every other top-level Edge response type.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct EdgeTeamZoneTimeDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team: Option<EdgeTeamInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seasons_with_edge_stats: Option<Vec<EdgeSeasonAvailability>>,
    pub zone_time_details: Vec<EdgeTeamZoneTimeByStrength>,
    /// Gotcha B: a single object (`{shotAttemptDifferential,
    /// shotAttemptDifferentialRank, sogDifferential, sogDifferentialRank}`),
    /// not a per-strength-code array — genuinely nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shot_differential: Option<EdgeTeamShotDifferential>,
}

/// Zone time broken down by strength code (`all`/`es`/`pp`/`pk`), embedded in
/// [`EdgeTeamZoneTimeDetails`].
///
/// The three league-average fields are genuinely nullable: the API omits them
/// for some strength codes (Go commit `f3ada28`).
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct EdgeTeamZoneTimeByStrength {
    pub strength_code: String,
    pub offensive_zone_pctg: f64,
    pub offensive_zone_rank: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offensive_zone_league_avg: Option<f64>,
    pub neutral_zone_pctg: f64,
    pub neutral_zone_rank: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub neutral_zone_league_avg: Option<f64>,
    pub defensive_zone_pctg: f64,
    pub defensive_zone_rank: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defensive_zone_league_avg: Option<f64>,
}

/// Aggregated shot-differential stats (gotcha B). Shared between
/// [`EdgeTeamZoneTimeDetails`] and [`EdgeTeamComparison`].
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct EdgeTeamShotDifferential {
    pub shot_attempt_differential: f64,
    pub shot_attempt_differential_rank: i32,
    pub sog_differential: f64,
    pub sog_differential_rank: i32,
}

/// Response from `edge/team-comparison/{team}/{season}/{gameType}`.
///
/// A rich composite for head-to-head display; each detail sub-object is
/// genuinely nullable (only populated when the comparison includes that
/// category). Reuses the shared `EdgeComparison*` family from the `edge::common`
/// module — team comparisons have no `zoneStarts` field (unlike
/// [`crate::EdgeSkaterComparison`]).
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct EdgeTeamComparison {
    pub team: EdgeTeamInfo,
    pub seasons_with_edge_stats: Vec<EdgeSeasonAvailability>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shot_speed_details: Option<EdgeComparisonShotSpeedDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skating_speed_details: Option<EdgeComparisonSkatingSpeedDetails>,
    pub skating_distance_last10: Vec<EdgeComparisonDistanceLast10Entry>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skating_distance_details: Option<EdgeComparisonSkatingDistanceDetails>,
    pub shot_location_details: Vec<EdgeComparisonShotLocationDetail>,
    pub shot_location_totals: Vec<EdgeComparisonShotLocationTotal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_time_details: Option<EdgeComparisonZoneTimeDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shot_differential: Option<EdgeTeamShotDifferential>,
}

/// A single leader entry in [`EdgeTeamLanding::leaders`].
///
/// Only the stat field(s) relevant to the entry's category key are populated;
/// the rest are `None`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct EdgeTeamLeader {
    pub team: EdgeTeamInfo,
    /// Populated for the `burstsOver22` category.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bursts: Option<i32>,
    /// Populated for the `shotAttemptsOver90` category.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attempts: Option<i32>,
    /// Populated for the `distancePer60` category.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance_skated: Option<EdgeMeasurement>,
    /// Populated for the `highDangerSog` category (gotcha C: leader payloads
    /// use `sog`, not `shots`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sog: Option<i32>,
    pub shot_location_details: Vec<EdgeLeaderShotLocation>,
    /// Populated for the `offensiveZoneTime` / `defensiveZoneTime` /
    /// `neutralZoneTime` categories.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_time: Option<f64>,
}

/// Response from `edge/team-landing/{season}/{gameType}` (no team id — this
/// is the league-wide leaderboard).
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct EdgeTeamLanding {
    pub seasons_with_edge_stats: Vec<EdgeSeasonAvailability>,
    /// Keyed by leader category (e.g. `"burstsOver22"`, `"highDangerSog"`).
    pub leaders: HashMap<String, EdgeTeamLeader>,
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
    fn test_edge_team_detail_empty_object() {
        assert_empty_object_deserializes::<EdgeTeamDetail>();
    }

    #[test]
    fn test_edge_team_shot_speed_empty_object() {
        assert_empty_object_deserializes::<EdgeTeamShotSpeed>();
    }

    #[test]
    fn test_edge_team_skating_speed_empty_object() {
        assert_empty_object_deserializes::<EdgeTeamSkatingSpeed>();
    }

    #[test]
    fn test_edge_team_distance_empty_object() {
        assert_empty_object_deserializes::<EdgeTeamDistance>();
    }

    #[test]
    fn test_edge_team_sog_summary_empty_object() {
        assert_empty_object_deserializes::<EdgeTeamSogSummary>();
    }

    #[test]
    fn test_edge_team_sog_area_detail_empty_object() {
        assert_empty_object_deserializes::<EdgeTeamSogAreaDetail>();
    }

    #[test]
    fn test_edge_team_zone_time_empty_object() {
        assert_empty_object_deserializes::<EdgeTeamZoneTime>();
    }

    #[test]
    fn test_edge_team_speed_detail_empty_object() {
        assert_empty_object_deserializes::<EdgeTeamSpeedDetail>();
    }

    #[test]
    fn test_edge_team_speed_entry_empty_object() {
        assert_empty_object_deserializes::<EdgeTeamSpeedEntry>();
    }

    #[test]
    fn test_edge_team_distance_detail_empty_object() {
        assert_empty_object_deserializes::<EdgeTeamDistanceDetail>();
    }

    #[test]
    fn test_edge_team_distance_entry_empty_object() {
        assert_empty_object_deserializes::<EdgeTeamDistanceEntry>();
    }

    #[test]
    fn test_edge_team_shot_speed_detail_empty_object() {
        assert_empty_object_deserializes::<EdgeTeamShotSpeedDetail>();
    }

    #[test]
    fn test_edge_team_shot_speed_entry_empty_object() {
        assert_empty_object_deserializes::<EdgeTeamShotSpeedEntry>();
    }

    #[test]
    fn test_edge_team_shot_location_detail_empty_object() {
        assert_empty_object_deserializes::<EdgeTeamShotLocationDetail>();
    }

    #[test]
    fn test_edge_team_shot_location_entry_empty_object() {
        assert_empty_object_deserializes::<EdgeTeamShotLocationEntry>();
    }

    #[test]
    fn test_edge_team_zone_time_details_empty_object() {
        assert_empty_object_deserializes::<EdgeTeamZoneTimeDetails>();
    }

    #[test]
    fn test_edge_team_zone_time_by_strength_empty_object() {
        assert_empty_object_deserializes::<EdgeTeamZoneTimeByStrength>();
    }

    #[test]
    fn test_edge_team_shot_differential_empty_object() {
        assert_empty_object_deserializes::<EdgeTeamShotDifferential>();
    }

    #[test]
    fn test_edge_team_comparison_empty_object() {
        assert_empty_object_deserializes::<EdgeTeamComparison>();
    }

    #[test]
    fn test_edge_team_leader_empty_object() {
        assert_empty_object_deserializes::<EdgeTeamLeader>();
    }

    #[test]
    fn test_edge_team_landing_empty_object() {
        assert_empty_object_deserializes::<EdgeTeamLanding>();
    }

    // ----- fixture deserialization, ported from Go edge_test.go -----

    /// Ported from Go's `TestEdgeTeamDetail_Deserialization`
    /// (`edge_test.go:244-345`).
    #[test]
    fn test_edge_team_detail_deserializes_fixture() {
        let json = r#"{
            "team": {
                "id": 22,
                "commonName": {"default": "Oilers"},
                "placeNameWithPreposition": {"default": "Edmonton"},
                "abbrev": "EDM",
                "teamLogo": {"light": "https://assets.nhle.com/logos/nhl/svg/EDM_light.svg", "dark": "https://assets.nhle.com/logos/nhl/svg/EDM_dark.svg"},
                "slug": "edmonton-oilers",
                "conference": "Western",
                "division": "Pacific",
                "wins": 35,
                "losses": 15,
                "otLosses": 5,
                "gamesPlayed": 55,
                "points": 75
            },
            "seasonsWithEdgeStats": [{"id": 20242025, "gameTypes": [2]}],
            "shotSpeed": {
                "shotAttemptsOver90": {"value": 120, "rank": 3, "leagueAvg": {"value": 95.5}},
                "topShotSpeed": {
                    "imperial": 105.2,
                    "metric": 169.3,
                    "rank": 5,
                    "leagueAvg": {"imperial": 98.0, "metric": 157.7}
                }
            },
            "skatingSpeed": {
                "burstsOver22": {"value": 80, "rank": 2},
                "burstsOver20": {"value": 500, "rank": 8, "leagueAvg": {"value": 420.0}},
                "speedMax": {
                    "imperial": 24.5,
                    "metric": 39.4,
                    "rank": 1,
                    "leagueAvg": {"imperial": 22.8, "metric": 36.7}
                }
            },
            "distanceSkated": {
                "total": {"value": 5000, "rank": 12, "leagueAvg": {"value": 4800.0}}
            },
            "sogSummary": [
                {
                    "locationCode": "all",
                    "shots": 1800,
                    "shotsRank": 5,
                    "shotsLeagueAvg": 1600.0,
                    "goals": 200,
                    "goalsRank": 3,
                    "goalsLeagueAvg": 170.0,
                    "shootingPctg": 0.111,
                    "shootingPctgRank": 8,
                    "shootingPctgLeagueAvg": 0.106
                }
            ],
            "sogDetails": [
                {"area": "High Slot", "shots": 450, "shotsRank": 4}
            ],
            "zoneTimeDetails": {
                "offensiveZonePctg": 0.34,
                "offensiveZoneRank": 5,
                "offensiveZoneLeagueAvg": 0.31,
                "offensiveZoneEvPctg": 0.32,
                "offensiveZoneEvRank": 6,
                "neutralZonePctg": 0.34,
                "neutralZoneRank": 15,
                "neutralZoneLeagueAvg": 0.35,
                "defensiveZonePctg": 0.32,
                "defensiveZoneRank": 20,
                "defensiveZoneLeagueAvg": 0.34
            }
        }"#;

        let detail: EdgeTeamDetail = serde_json::from_str(json).expect("must deserialize");

        assert_eq!(detail.team.id, 22);
        assert_eq!(detail.team.abbrev, "EDM");
        assert_eq!(detail.seasons_with_edge_stats.len(), 1);
        assert_eq!(detail.shot_speed.shot_attempts_over_90.rank, 3);
        assert_eq!(
            detail.shot_speed.shot_attempts_over_90.league_avg,
            Some(crate::types::EdgeCountLeagueAvg { value: 95.5 })
        );
        assert_eq!(detail.skating_speed.bursts_over_22.league_avg, None);
        assert_eq!(detail.skating_speed.speed_max.rank, 1);
        assert_eq!(detail.distance_skated.total.value, 5000);
        assert_eq!(detail.sog_summary.len(), 1);
        assert_eq!(detail.sog_summary[0].shots, 1800);
        assert_eq!(detail.sog_details.len(), 1);
        assert_eq!(detail.sog_details[0].area, "High Slot");
        assert_eq!(detail.zone_time_details.offensive_zone_rank, 5);
    }

    /// Speed-detail fixture. No dedicated Go test exists for this type; keys
    /// verified directly against `EdgeTeamSpeedDetail` / `EdgeTeamSpeedEntry`
    /// struct tags in Go's `edge_team.go`.
    #[test]
    fn test_edge_team_speed_detail_deserializes_fixture() {
        let json = r#"{
            "team": {"id": 22, "commonName": {"default": "Oilers"}, "abbrev": "EDM"},
            "seasonsWithEdgeStats": [{"id": 20242025, "gameTypes": [2]}],
            "topSkatingSpeeds": [
                {"player": {"firstName": {"default": "Connor"}, "lastName": {"default": "McDavid"}}, "speed": {"imperial": 23.1, "metric": 37.2}}
            ]
        }"#;

        let detail: EdgeTeamSpeedDetail = serde_json::from_str(json).expect("must deserialize");

        assert_eq!(detail.top_skating_speeds.len(), 1);
        assert_eq!(
            detail.top_skating_speeds[0].player.first_name.default,
            "Connor"
        );
        assert_eq!(detail.top_skating_speeds[0].speed.imperial, 23.1);
    }

    /// Distance-detail fixture, following the shape of the speed-detail test
    /// but for `skatingDistanceLast10`.
    #[test]
    fn test_edge_team_distance_detail_deserializes_fixture() {
        let json = r#"{
            "team": {"id": 22, "commonName": {"default": "Oilers"}, "abbrev": "EDM"},
            "seasonsWithEdgeStats": [{"id": 20242025, "gameTypes": [2]}],
            "skatingDistanceLast10": [
                {"gameDate": "2025-01-15", "awayTeam": {"abbrev": "CGY", "score": 2}, "homeTeam": {"abbrev": "EDM", "score": 5}, "distance": {"imperial": 350.0, "metric": 563.3}}
            ]
        }"#;

        let detail: EdgeTeamDistanceDetail = serde_json::from_str(json).expect("must deserialize");

        assert_eq!(detail.skating_distance_last10.len(), 1);
        assert_eq!(detail.skating_distance_last10[0].distance.imperial, 350.0);
        assert_eq!(detail.skating_distance_last10[0].home_team.abbrev, "EDM");
    }

    /// Shot-speed-detail fixture, mirroring `EdgeTeamSpeedDetail`'s shape but
    /// for `hardestShots`.
    #[test]
    fn test_edge_team_shot_speed_detail_deserializes_fixture() {
        let json = r#"{
            "team": {"id": 22, "commonName": {"default": "Oilers"}, "abbrev": "EDM"},
            "seasonsWithEdgeStats": [{"id": 20242025, "gameTypes": [2]}],
            "hardestShots": [
                {"player": {"firstName": {"default": "Leon"}, "lastName": {"default": "Draisaitl"}}, "speed": {"imperial": 101.4, "metric": 163.2}}
            ]
        }"#;

        let detail: EdgeTeamShotSpeedDetail = serde_json::from_str(json).expect("must deserialize");

        assert_eq!(detail.hardest_shots.len(), 1);
        assert_eq!(
            detail.hardest_shots[0].player.last_name.default,
            "Draisaitl"
        );
        assert_eq!(detail.hardest_shots[0].speed.imperial, 101.4);
    }

    /// Shot-location-detail fixture. No dedicated Go test exists for this
    /// type; keys verified directly against `EdgeTeamShotLocationEntry`
    /// struct tags in Go's `edge_team.go` (`shots`/`goals`, no
    /// `shootingPctg`).
    #[test]
    fn test_edge_team_shot_location_detail_deserializes_fixture() {
        let json = r#"{
            "team": {"id": 22, "commonName": {"default": "Oilers"}, "abbrev": "EDM"},
            "seasonsWithEdgeStats": [{"id": 20242025, "gameTypes": [2]}],
            "shotLocationDetails": [
                {"area": "Crease", "shots": 400, "goals": 80}
            ]
        }"#;

        let detail: EdgeTeamShotLocationDetail =
            serde_json::from_str(json).expect("must deserialize");

        assert_eq!(detail.shot_location_details.len(), 1);
        assert_eq!(detail.shot_location_details[0].shots, 400);
        assert_eq!(detail.shot_location_details[0].goals, 80);
    }

    /// Gotcha B, mandatory fixture. Ported from Go's
    /// `TestEdgeTeamZoneTimeDetails_Deserialization` (`edge_test.go:347-384`):
    /// `shotDifferential` is a real API object
    /// (`{shotAttemptDifferential, shotAttemptDifferentialRank,
    /// sogDifferential, sogDifferentialRank}`), and `team` /
    /// `seasonsWithEdgeStats` are absent from this real response (they are
    /// genuinely nullable, unlike every other top-level Edge type).
    #[test]
    fn test_edge_team_zone_time_details_deserializes_fixture_gotcha_b() {
        let json = r#"{
            "zoneTimeDetails": [
                {"strengthCode": "all", "offensiveZonePctg": 0.43, "offensiveZoneRank": 3, "offensiveZoneLeagueAvg": 0.41, "neutralZonePctg": 0.17, "neutralZoneRank": 30, "neutralZoneLeagueAvg": 0.18, "defensiveZonePctg": 0.40, "defensiveZoneRank": 5, "defensiveZoneLeagueAvg": 0.41},
                {"strengthCode": "es", "offensiveZonePctg": 0.42, "offensiveZoneRank": 4, "offensiveZoneLeagueAvg": 0.41, "neutralZonePctg": 0.18, "neutralZoneRank": 30, "neutralZoneLeagueAvg": 0.19, "defensiveZonePctg": 0.40, "defensiveZoneRank": 6, "defensiveZoneLeagueAvg": 0.41},
                {"strengthCode": "pp", "offensiveZonePctg": 0.62, "offensiveZoneRank": 4, "offensiveZoneLeagueAvg": 0.59, "neutralZonePctg": 0.14, "neutralZoneRank": 24, "neutralZoneLeagueAvg": 0.14, "defensiveZonePctg": 0.25, "defensiveZoneRank": 4, "defensiveZoneLeagueAvg": 0.27},
                {"strengthCode": "pk", "offensiveZonePctg": 0.29, "offensiveZoneRank": 3, "offensiveZoneLeagueAvg": 0.27, "neutralZonePctg": 0.14, "neutralZoneRank": 13, "neutralZoneLeagueAvg": 0.14, "defensiveZonePctg": 0.57, "defensiveZoneRank": 6, "defensiveZoneLeagueAvg": 0.59}
            ],
            "shotDifferential": {
                "shotAttemptDifferential": 5.01,
                "shotAttemptDifferentialRank": 3,
                "sogDifferential": 0.12,
                "sogDifferentialRank": 2
            }
        }"#;

        let detail: EdgeTeamZoneTimeDetails = serde_json::from_str(json).expect("must deserialize");

        // Genuinely nullable and absent from this real fixture.
        assert_eq!(detail.team, None);
        assert_eq!(detail.seasons_with_edge_stats, None);

        assert_eq!(detail.zone_time_details.len(), 4);
        assert_eq!(detail.zone_time_details[0].strength_code, "all");
        assert_eq!(detail.zone_time_details[2].offensive_zone_pctg, 0.62);
        assert_eq!(
            detail.zone_time_details[2].offensive_zone_league_avg,
            Some(0.59)
        );

        let shot_differential = detail
            .shot_differential
            .as_ref()
            .expect("shotDifferential present (not an array)");
        assert_eq!(shot_differential.shot_attempt_differential, 5.01);
        assert_eq!(shot_differential.shot_attempt_differential_rank, 3);
        assert_eq!(shot_differential.sog_differential, 0.12);
        assert_eq!(shot_differential.sog_differential_rank, 2);

        // Round-trip: `team`/`seasonsWithEdgeStats` stay omitted when `None`.
        let round_tripped = serde_json::to_string(&detail).unwrap();
        assert!(!round_tripped.contains("\"team\""));
        assert!(!round_tripped.contains("seasonsWithEdgeStats"));
        let back: EdgeTeamZoneTimeDetails = serde_json::from_str(&round_tripped).unwrap();
        assert_eq!(back, detail);
    }

    /// `team` and `seasonsWithEdgeStats` deserialize to `Some` when the API
    /// does include them (the counterpart case to the gotcha B fixture
    /// above — both shapes are genuinely observed).
    #[test]
    fn test_edge_team_zone_time_details_with_team_present() {
        let json = r#"{
            "team": {"id": 22, "abbrev": "EDM"},
            "seasonsWithEdgeStats": [{"id": 20242025, "gameTypes": [2]}],
            "zoneTimeDetails": []
        }"#;
        let detail: EdgeTeamZoneTimeDetails = serde_json::from_str(json).expect("must deserialize");

        assert_eq!(detail.team.expect("team present").abbrev, "EDM");
        assert_eq!(
            detail
                .seasons_with_edge_stats
                .expect("seasonsWithEdgeStats present")
                .len(),
            1
        );
    }

    /// League-average fields on a per-strength entry are independently
    /// nullable — the API omits them for some strength codes (Go commit
    /// `f3ada28`).
    #[test]
    fn test_edge_team_zone_time_by_strength_missing_league_avgs_is_none() {
        let json = r#"{"strengthCode": "pp", "offensiveZonePctg": 0.62, "offensiveZoneRank": 4, "neutralZonePctg": 0.14, "neutralZoneRank": 24, "defensiveZonePctg": 0.25, "defensiveZoneRank": 4}"#;
        let entry: EdgeTeamZoneTimeByStrength =
            serde_json::from_str(json).expect("must deserialize");

        assert_eq!(entry.offensive_zone_league_avg, None);
        assert_eq!(entry.neutral_zone_league_avg, None);
        assert_eq!(entry.defensive_zone_league_avg, None);
    }

    /// Ported from Go's `TestEdgeTeamComparison_RealAPIStructure`
    /// (`edge_test.go:865-941`), with the same class of corrections applied
    /// as the skater comparison fixture (per `REVIEW.md`'s findings that the
    /// Go fixture itself used the wrong keys, unnoticed because the Go test
    /// never asserted on those fields): `shotLocationDetails`/
    /// `shotLocationTotals` use `sog` (gotcha C), not `shots`;
    /// `skatingSpeedDetails` has a `maxSkatingSpeed` field, not
    /// `topSkatingSpeed`/`avgSkatingSpeed`; `skatingDistanceDetails` is keyed
    /// `distanceTotal`, not `totalDistance`. The mandatory `shotDifferential`
    /// object fixture (`edge_test.go:900-915`) is preserved unmodified.
    #[test]
    fn test_edge_team_comparison_deserializes_fixture() {
        let json = r#"{
            "team": {
                "id": 22,
                "commonName": {"default": "Oilers"},
                "placeNameWithPreposition": {"default": "Edmonton"},
                "abbrev": "EDM",
                "teamLogo": {"light": "l", "dark": "d"},
                "slug": "edmonton-oilers",
                "conference": "Western",
                "division": "Pacific",
                "wins": 50,
                "losses": 25,
                "otLosses": 7,
                "gamesPlayed": 82,
                "points": 107
            },
            "seasonsWithEdgeStats": [{"id": 20232024, "gameTypes": [2, 3]}],
            "shotSpeedDetails": {
                "topShotSpeed": {"imperial": 100.0, "metric": 160.9},
                "avgShotSpeed": {"imperial": 88.0, "metric": 141.6},
                "shotAttemptsOver100": 10
            },
            "skatingSpeedDetails": {
                "maxSkatingSpeed": {"imperial": 24.0, "metric": 38.6},
                "burstsOver22": 12
            },
            "skatingDistanceLast10": [
                {"gameDate": "2024-04-15", "distance": {"imperial": 300.0, "metric": 482.8}, "homeTeam": {"abbrev": "EDM", "score": 4}, "awayTeam": {"abbrev": "CGY", "score": 2}}
            ],
            "skatingDistanceDetails": {
                "distanceTotal": {"imperial": 2900.0, "metric": 4667.0}
            },
            "shotLocationDetails": [
                {"area": "Crease", "sog": 400, "goals": 80}
            ],
            "shotLocationTotals": [
                {"locationCode": "all", "sog": 2800, "goals": 300, "shootingPctg": 0.107}
            ],
            "zoneTimeDetails": {
                "offensiveZonePctg": 0.35,
                "neutralZonePctg": 0.32,
                "defensiveZonePctg": 0.33
            },
            "shotDifferential": {
                "shotAttemptDifferential": 5.5,
                "shotAttemptDifferentialRank": 3,
                "sogDifferential": 2.1,
                "sogDifferentialRank": 5
            }
        }"#;

        let comparison: EdgeTeamComparison = serde_json::from_str(json).expect("must deserialize");

        assert_eq!(comparison.team.abbrev, "EDM");

        let shot_speed = comparison
            .shot_speed_details
            .expect("shotSpeedDetails present");
        assert_eq!(
            shot_speed
                .avg_shot_speed
                .expect("avgShotSpeed present")
                .imperial,
            88.0
        );

        let skating_speed = comparison
            .skating_speed_details
            .expect("skatingSpeedDetails present");
        assert_eq!(
            skating_speed
                .max_skating_speed
                .expect("maxSkatingSpeed present")
                .imperial,
            24.0
        );
        assert_eq!(skating_speed.bursts_over_22, 12);

        assert_eq!(comparison.skating_distance_last10.len(), 1);
        assert_eq!(
            comparison.skating_distance_last10[0]
                .distance
                .as_ref()
                .expect("distance present")
                .imperial,
            300.0
        );
        assert_eq!(
            comparison.skating_distance_last10[0]
                .home_team
                .as_ref()
                .expect("homeTeam present")
                .abbrev,
            "EDM"
        );

        let skating_distance = comparison
            .skating_distance_details
            .expect("skatingDistanceDetails present");
        assert_eq!(
            skating_distance
                .distance_total
                .expect("distanceTotal present")
                .imperial,
            2900.0
        );

        assert_eq!(comparison.shot_location_details.len(), 1);
        assert_eq!(comparison.shot_location_details[0].sog, 400);
        assert_eq!(comparison.shot_location_totals.len(), 1);
        assert_eq!(comparison.shot_location_totals[0].sog, 2800);

        let zone_time = comparison
            .zone_time_details
            .expect("zoneTimeDetails present");
        assert_eq!(zone_time.offensive_zone_pctg, 0.35);

        // Mandatory shotDifferential fixture (edge_test.go:900-915).
        let shot_differential = comparison
            .shot_differential
            .expect("shotDifferential present");
        assert_eq!(shot_differential.shot_attempt_differential, 5.5);
        assert_eq!(shot_differential.shot_attempt_differential_rank, 3);
        assert_eq!(shot_differential.sog_differential, 2.1);
        assert_eq!(shot_differential.sog_differential_rank, 5);
    }

    /// Landing fixture exercising every `EdgeTeamLeader` stat-field tag
    /// against real category keys. No dedicated Go fixture exists for team
    /// landing (only the skater-landing shape is exercised in
    /// `edge_test.go`), so this covers what the plan flags as
    /// previously-unverified via `{}`-only tests.
    #[test]
    fn test_edge_team_landing_deserializes_fixture() {
        let json = r#"{
            "seasonsWithEdgeStats": [{"id": 20242025, "gameTypes": [2]}],
            "leaders": {
                "burstsOver22": {
                    "team": {"id": 22, "abbrev": "EDM"},
                    "bursts": 210
                },
                "shotAttemptsOver90": {
                    "team": {"id": 6, "abbrev": "BOS"},
                    "attempts": 340
                },
                "distancePer60": {
                    "team": {"id": 10, "abbrev": "TOR"},
                    "distanceSkated": {"imperial": 41.2, "metric": 66.3}
                },
                "highDangerSog": {
                    "team": {"id": 22, "abbrev": "EDM"},
                    "sog": 620,
                    "shotLocationDetails": [
                        {"area": "HighDanger", "sog": 620, "sogPercentile": 0.88}
                    ]
                },
                "offensiveZoneTime": {
                    "team": {"id": 22, "abbrev": "EDM"},
                    "zoneTime": 0.36
                }
            }
        }"#;

        let landing: EdgeTeamLanding = serde_json::from_str(json).expect("must deserialize");

        assert_eq!(landing.seasons_with_edge_stats.len(), 1);
        assert_eq!(landing.leaders.len(), 5);

        let bursts = landing.leaders.get("burstsOver22").expect("present");
        assert_eq!(bursts.bursts, Some(210));
        assert_eq!(bursts.attempts, None);

        let attempts = landing.leaders.get("shotAttemptsOver90").expect("present");
        assert_eq!(attempts.attempts, Some(340));
        assert_eq!(attempts.bursts, None);

        let distance = landing.leaders.get("distancePer60").expect("present");
        assert_eq!(
            distance
                .distance_skated
                .as_ref()
                .expect("distanceSkated present")
                .imperial,
            41.2
        );

        let high_danger = landing.leaders.get("highDangerSog").expect("present");
        assert_eq!(high_danger.sog, Some(620));
        assert_eq!(high_danger.shot_location_details.len(), 1);
        assert_eq!(high_danger.shot_location_details[0].sog, Some(620));

        let zone_time = landing.leaders.get("offensiveZoneTime").expect("present");
        assert_eq!(zone_time.zone_time, Some(0.36));
        assert_eq!(zone_time.sog, None);
    }

    #[test]
    fn test_edge_team_leader_mutually_exclusive_stat_fields() {
        let json = r#"{"team": {"id": 1}, "zoneTime": 0.42}"#;
        let leader: EdgeTeamLeader = serde_json::from_str(json).expect("must deserialize");

        assert_eq!(leader.zone_time, Some(0.42));
        assert_eq!(leader.bursts, None);
        assert_eq!(leader.attempts, None);
        assert_eq!(leader.distance_skated, None);
        assert_eq!(leader.sog, None);
        assert!(leader.shot_location_details.is_empty());
    }
}
