//! Edge skater stats: `/edge/skater-*` endpoints.
//!
//! See the module-level documentation in [`super`](crate::types::edge) for the
//! two design rules every type here obeys (deserialize-from-`{}` and
//! no-`Option`-on-plain-scalar-counts) and the field-naming gotchas (in
//! particular gotcha C — detail payloads use `shots`, comparison/leader
//! payloads use `sog`).

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::common::{
    EdgeComparisonDistanceLast10Entry, EdgeComparisonShotLocationDetail,
    EdgeComparisonShotLocationTotal, EdgeComparisonShotSpeedDetails,
    EdgeComparisonSkatingDistanceDetails, EdgeComparisonSkatingSpeedDetails,
    EdgeComparisonZoneStarts, EdgeComparisonZoneTimeDetails, EdgeCountPercentileStat,
    EdgeLeaderShotLocation, EdgeMeasurement, EdgeOverlay, EdgeOverlayTeam, EdgePercentileStat,
    EdgePercentileStatWithOverlay, EdgeSeasonAvailability, EdgeSkaterPlayer,
};

/// Response from `edge/skater-detail/{player}/{season}/{gameType}`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct EdgeSkaterDetail {
    pub player: EdgeSkaterPlayer,
    pub seasons_with_edge_stats: Vec<EdgeSeasonAvailability>,
    pub top_shot_speed: EdgePercentileStatWithOverlay,
    pub skating_speed: EdgeSkaterSpeed,
    pub total_distance_skated: EdgePercentileStat,
    pub distance_max_game: EdgePercentileStatWithOverlay,
    pub sog_summary: Vec<EdgeSkaterSogSummary>,
    pub sog_details: Vec<EdgeSogAreaDetail>,
    pub zone_time_details: EdgeSkaterZoneTimeSummary,
}

/// Skating-speed stats with burst counts, embedded in [`EdgeSkaterDetail`].
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct EdgeSkaterSpeed {
    pub speed_max: EdgePercentileStatWithOverlay,
    pub bursts_over_20: EdgeCountPercentileStat,
}

/// Shot-on-goal summary for a rink area, embedded in [`EdgeSkaterDetail`].
///
/// Gotcha C: this is a *detail* payload, so shots-on-goal is keyed `shots`
/// (comparison/leader payloads use `sog` for the same concept).
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct EdgeSkaterSogSummary {
    pub location_code: String,
    pub shots: i32,
    pub shots_percentile: f64,
    pub shots_league_avg: f64,
    pub goals: i32,
    pub goals_percentile: f64,
    pub goals_league_avg: f64,
    pub shooting_pctg: f64,
    pub shooting_pctg_percentile: f64,
    pub shooting_pctg_league_avg: f64,
}

/// Shot-on-goal detail for a specific rink area, embedded in [`EdgeSkaterDetail`].
///
/// Gotcha C: detail payload, keyed `shots` (see [`EdgeSkaterSogSummary`]).
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct EdgeSogAreaDetail {
    pub area: String,
    pub shots: i32,
    pub shooting_pctg: f64,
    pub shots_percentile: f64,
}

/// Zone-time percentages and percentiles, embedded in [`EdgeSkaterDetail`].
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct EdgeSkaterZoneTimeSummary {
    pub offensive_zone_pctg: f64,
    pub offensive_zone_percentile: f64,
    pub offensive_zone_league_avg: f64,
    pub offensive_zone_ev_pctg: f64,
    pub offensive_zone_ev_percentile: f64,
    pub offensive_zone_ev_league_avg: f64,
    pub neutral_zone_pctg: f64,
    pub neutral_zone_percentile: f64,
    pub neutral_zone_league_avg: f64,
    pub defensive_zone_pctg: f64,
    pub defensive_zone_percentile: f64,
    pub defensive_zone_league_avg: f64,
}

/// Response from `edge/skater-skating-speed-detail/{player}/{season}/{gameType}`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct EdgeSkaterSpeedDetail {
    pub player: EdgeSkaterPlayer,
    pub seasons_with_edge_stats: Vec<EdgeSeasonAvailability>,
    pub top_skating_speeds: Vec<EdgeSpeedEntry>,
}

/// A per-game skating-speed entry, embedded in [`EdgeSkaterSpeedDetail`].
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct EdgeSpeedEntry {
    pub game_date: String,
    pub away_team: EdgeOverlayTeam,
    pub home_team: EdgeOverlayTeam,
    pub speed: EdgeMeasurement,
}

/// Response from `edge/skater-skating-distance-detail/{player}/{season}/{gameType}`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct EdgeSkaterDistanceDetail {
    pub player: EdgeSkaterPlayer,
    pub seasons_with_edge_stats: Vec<EdgeSeasonAvailability>,
    pub skating_distance_last10: Vec<EdgeDistanceEntry>,
}

/// A per-game distance-skated entry, embedded in [`EdgeSkaterDistanceDetail`].
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct EdgeDistanceEntry {
    pub game_date: String,
    pub away_team: EdgeOverlayTeam,
    pub home_team: EdgeOverlayTeam,
    pub distance: EdgeMeasurement,
}

/// Response from `edge/skater-shot-speed-detail/{player}/{season}/{gameType}`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct EdgeSkaterShotSpeedDetail {
    pub player: EdgeSkaterPlayer,
    pub seasons_with_edge_stats: Vec<EdgeSeasonAvailability>,
    pub hardest_shots: Vec<EdgeShotSpeedEntry>,
}

/// A per-game shot-speed entry, embedded in [`EdgeSkaterShotSpeedDetail`].
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct EdgeShotSpeedEntry {
    pub game_date: String,
    pub away_team: EdgeOverlayTeam,
    pub home_team: EdgeOverlayTeam,
    pub speed: EdgeMeasurement,
}

/// Response from `edge/skater-shot-location-detail/{player}/{season}/{gameType}`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct EdgeSkaterShotLocationDetail {
    pub player: EdgeSkaterPlayer,
    pub seasons_with_edge_stats: Vec<EdgeSeasonAvailability>,
    pub shot_location_details: Vec<EdgeShotLocationEntry>,
}

/// A shot-location breakdown for a rink area, embedded in
/// [`EdgeSkaterShotLocationDetail`].
///
/// Gotcha C: detail payload, keyed `shots` (see [`EdgeSkaterSogSummary`]).
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct EdgeShotLocationEntry {
    pub area: String,
    pub shots: i32,
    pub goals: i32,
    pub shooting_pctg: f64,
}

/// Response from `edge/skater-zone-time/{player}/{season}/{gameType}` (note:
/// no `-details` suffix on the path, unlike the client method name suggests).
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct EdgeSkaterZoneTimeDetail {
    pub player: EdgeSkaterPlayer,
    pub seasons_with_edge_stats: Vec<EdgeSeasonAvailability>,
    pub zone_time_details: Vec<EdgeZoneTimeEntry>,
}

/// A zone-time breakdown by strength code, embedded in
/// [`EdgeSkaterZoneTimeDetail`].
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct EdgeZoneTimeEntry {
    pub strength_code: String,
    pub offensive_zone_pctg: f64,
    pub neutral_zone_pctg: f64,
    pub defensive_zone_pctg: f64,
}

/// Response from `edge/skater-comparison/{player}/{season}/{gameType}`.
///
/// A rich composite for head-to-head display; each detail sub-object is
/// genuinely nullable (only populated when the comparison includes that
/// category).
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct EdgeSkaterComparison {
    pub player: EdgeSkaterPlayer,
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
    pub zone_starts: Option<EdgeComparisonZoneStarts>,
}

/// A single leader entry in [`EdgeSkaterLanding::leaders`].
///
/// Only the stat field(s) relevant to the entry's category key are populated;
/// the rest are `None`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct EdgeSkaterLeader {
    pub player: EdgeSkaterPlayer,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overlay: Option<EdgeOverlay>,
    /// Populated for the `hardestShot` category.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shot_speed: Option<EdgeMeasurement>,
    /// Populated for the `maxSkatingSpeed` category.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skating_speed: Option<EdgeMeasurement>,
    /// Populated for the `totalDistanceSkated` / `distanceMaxGame` categories.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance_skated: Option<EdgeMeasurement>,
    /// Populated for the `highDangerSog` category (gotcha C: leader payloads
    /// use `sog`, not `shots`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sog: Option<i32>,
    pub shot_location_details: Vec<EdgeLeaderShotLocation>,
    /// Populated for the `offensiveZoneTime` / `defensiveZoneTime` categories.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_time: Option<f64>,
}

/// Response from `edge/skater-landing/{season}/{gameType}` (no player id —
/// this is the league-wide leaderboard).
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct EdgeSkaterLanding {
    pub seasons_with_edge_stats: Vec<EdgeSeasonAvailability>,
    /// Keyed by leader category (e.g. `"hardestShot"`, `"maxSkatingSpeed"`).
    pub leaders: HashMap<String, EdgeSkaterLeader>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::LocalizedString;

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
    fn test_edge_skater_detail_empty_object() {
        assert_empty_object_deserializes::<EdgeSkaterDetail>();
    }

    #[test]
    fn test_edge_skater_speed_empty_object() {
        assert_empty_object_deserializes::<EdgeSkaterSpeed>();
    }

    #[test]
    fn test_edge_skater_sog_summary_empty_object() {
        assert_empty_object_deserializes::<EdgeSkaterSogSummary>();
    }

    #[test]
    fn test_edge_sog_area_detail_empty_object() {
        assert_empty_object_deserializes::<EdgeSogAreaDetail>();
    }

    #[test]
    fn test_edge_skater_zone_time_summary_empty_object() {
        assert_empty_object_deserializes::<EdgeSkaterZoneTimeSummary>();
    }

    #[test]
    fn test_edge_skater_speed_detail_empty_object() {
        assert_empty_object_deserializes::<EdgeSkaterSpeedDetail>();
    }

    #[test]
    fn test_edge_speed_entry_empty_object() {
        assert_empty_object_deserializes::<EdgeSpeedEntry>();
    }

    #[test]
    fn test_edge_skater_distance_detail_empty_object() {
        assert_empty_object_deserializes::<EdgeSkaterDistanceDetail>();
    }

    #[test]
    fn test_edge_distance_entry_empty_object() {
        assert_empty_object_deserializes::<EdgeDistanceEntry>();
    }

    #[test]
    fn test_edge_skater_shot_speed_detail_empty_object() {
        assert_empty_object_deserializes::<EdgeSkaterShotSpeedDetail>();
    }

    #[test]
    fn test_edge_shot_speed_entry_empty_object() {
        assert_empty_object_deserializes::<EdgeShotSpeedEntry>();
    }

    #[test]
    fn test_edge_skater_shot_location_detail_empty_object() {
        assert_empty_object_deserializes::<EdgeSkaterShotLocationDetail>();
    }

    #[test]
    fn test_edge_shot_location_entry_empty_object() {
        assert_empty_object_deserializes::<EdgeShotLocationEntry>();
    }

    #[test]
    fn test_edge_skater_zone_time_detail_empty_object() {
        assert_empty_object_deserializes::<EdgeSkaterZoneTimeDetail>();
    }

    #[test]
    fn test_edge_zone_time_entry_empty_object() {
        assert_empty_object_deserializes::<EdgeZoneTimeEntry>();
    }

    #[test]
    fn test_edge_skater_comparison_empty_object() {
        assert_empty_object_deserializes::<EdgeSkaterComparison>();
    }

    #[test]
    fn test_edge_skater_leader_empty_object() {
        assert_empty_object_deserializes::<EdgeSkaterLeader>();
    }

    #[test]
    fn test_edge_skater_landing_empty_object() {
        assert_empty_object_deserializes::<EdgeSkaterLanding>();
    }

    // ----- fixture deserialization, ported from Go edge_test.go -----

    /// Ported from Go's `TestEdgeSkaterDetail_Deserialization`
    /// (`edge_test.go:16-160`).
    #[test]
    fn test_edge_skater_detail_deserializes_fixture() {
        let json = r#"{
            "player": {
                "id": 8478402,
                "firstName": {"default": "Connor"},
                "lastName": {"default": "McDavid"},
                "birthDate": "1997-01-13",
                "shootsCatches": "L",
                "sweaterNumber": 97,
                "position": "C",
                "slug": "connor-mcdavid-8478402",
                "headshot": "https://assets.nhle.com/mugs/nhl/20242025/EDM/8478402.png",
                "goals": 30,
                "assists": 60,
                "points": 90,
                "gamesPlayed": 50,
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
                }
            },
            "seasonsWithEdgeStats": [{"id": 20242025, "gameTypes": [2]}, {"id": 20232024, "gameTypes": [2, 3]}],
            "topShotSpeed": {
                "imperial": 102.3,
                "metric": 164.6,
                "percentile": 0.95,
                "leagueAvg": {"imperial": 85.0, "metric": 136.8},
                "overlay": {
                    "player": {"firstName": {"default": "Connor"}, "lastName": {"default": "McDavid"}},
                    "gameDate": "2025-01-15",
                    "awayTeam": {"abbrev": "CGY", "score": 2},
                    "homeTeam": {"abbrev": "EDM", "score": 5},
                    "periodDescriptor": {"number": 2, "periodType": "REG", "maxRegulationPeriods": 3},
                    "timeInPeriod": "14:32",
                    "gameType": 2
                }
            },
            "skatingSpeed": {
                "speedMax": {
                    "imperial": 23.1,
                    "metric": 37.2,
                    "percentile": 0.98,
                    "leagueAvg": {"imperial": 21.5, "metric": 34.6},
                    "overlay": {
                        "player": {"firstName": {"default": "Connor"}, "lastName": {"default": "McDavid"}},
                        "gameDate": "2025-02-01",
                        "awayTeam": {"abbrev": "EDM", "score": 4},
                        "homeTeam": {"abbrev": "VAN", "score": 1},
                        "periodDescriptor": {"number": 3, "periodType": "REG", "maxRegulationPeriods": 3},
                        "timeInPeriod": "08:15",
                        "gameType": 2
                    }
                },
                "burstsOver20": {"value": 150, "percentile": 0.92, "leagueAvg": {"value": 110.5}}
            },
            "totalDistanceSkated": {"imperial": 450.2, "metric": 724.5, "percentile": 0.88, "leagueAvg": {"imperial": 400.0, "metric": 643.7}},
            "distanceMaxGame": {
                "imperial": 12.5,
                "metric": 20.1,
                "percentile": 0.91,
                "leagueAvg": {"imperial": 10.0, "metric": 16.1}
            },
            "sogSummary": [
                {
                    "locationCode": "all",
                    "shots": 200,
                    "shotsPercentile": 0.90,
                    "shotsLeagueAvg": 150.0,
                    "goals": 30,
                    "goalsPercentile": 0.95,
                    "goalsLeagueAvg": 20.0,
                    "shootingPctg": 0.15,
                    "shootingPctgPercentile": 0.88,
                    "shootingPctgLeagueAvg": 0.12
                }
            ],
            "sogDetails": [
                {"area": "Crease", "shots": 40, "shootingPctg": 0.25, "shotsPercentile": 0.85}
            ],
            "zoneTimeDetails": {
                "offensiveZonePctg": 0.35,
                "offensiveZonePercentile": 0.80,
                "offensiveZoneLeagueAvg": 0.30,
                "offensiveZoneEvPctg": 0.33,
                "offensiveZoneEvPercentile": 0.78,
                "offensiveZoneEvLeagueAvg": 0.29,
                "neutralZonePctg": 0.35,
                "neutralZonePercentile": 0.50,
                "neutralZoneLeagueAvg": 0.36,
                "defensiveZonePctg": 0.30,
                "defensiveZonePercentile": 0.70,
                "defensiveZoneLeagueAvg": 0.34
            }
        }"#;

        let detail: EdgeSkaterDetail = serde_json::from_str(json).expect("must deserialize");

        assert_eq!(detail.player.id, 8478402);
        assert_eq!(detail.player.first_name.default, "Connor");
        assert_eq!(detail.player.team.abbrev, "EDM");
        assert_eq!(detail.seasons_with_edge_stats.len(), 2);
        assert_eq!(detail.top_shot_speed.imperial, 102.3);
        let overlay = detail.top_shot_speed.overlay.expect("overlay present");
        assert_eq!(overlay.home_team.abbrev, "EDM");
        assert_eq!(detail.skating_speed.bursts_over_20.value, 150);
        assert_eq!(detail.skating_speed.bursts_over_20.league_avg.value, 110.5);
        assert_eq!(detail.sog_summary.len(), 1);
        assert_eq!(detail.sog_summary[0].shots, 200);
        assert_eq!(detail.sog_details.len(), 1);
        assert_eq!(detail.sog_details[0].area, "Crease");
        assert_eq!(detail.zone_time_details.offensive_zone_pctg, 0.35);
    }

    /// Ported from Go's `TestEdgeSkaterSpeedDetail_Deserialization`
    /// (`edge_test.go:386-414`).
    #[test]
    fn test_edge_skater_speed_detail_deserializes_fixture() {
        let json = r#"{
            "player": {
                "id": 8478402,
                "firstName": {"default": "Connor"}, "lastName": {"default": "McDavid"},
                "birthDate": "1997-01-13", "shootsCatches": "L", "sweaterNumber": 97,
                "position": "C", "slug": "connor-mcdavid-8478402", "headshot": "h",
                "goals": 30, "assists": 60, "points": 90, "gamesPlayed": 50,
                "team": {"id": 22, "commonName": {"default": "Oilers"}, "placeNameWithPreposition": {"default": "Edmonton"}, "abbrev": "EDM", "teamLogo": {"light": "l", "dark": "d"}, "slug": "s", "conference": "W", "division": "P", "wins": 35, "losses": 15, "otLosses": 5, "gamesPlayed": 55, "points": 75}
            },
            "seasonsWithEdgeStats": [{"id": 20242025, "gameTypes": [2]}],
            "topSkatingSpeeds": [
                {"gameDate": "2025-01-15", "awayTeam": {"abbrev": "CGY", "score": 2}, "homeTeam": {"abbrev": "EDM", "score": 5}, "speed": {"imperial": 23.1, "metric": 37.2}},
                {"gameDate": "2025-01-10", "awayTeam": {"abbrev": "EDM", "score": 3}, "homeTeam": {"abbrev": "VAN", "score": 1}, "speed": {"imperial": 22.8, "metric": 36.7}}
            ]
        }"#;

        let detail: EdgeSkaterSpeedDetail = serde_json::from_str(json).expect("must deserialize");

        assert_eq!(detail.top_skating_speeds.len(), 2);
        assert_eq!(detail.top_skating_speeds[0].speed.imperial, 23.1);
        assert_eq!(detail.top_skating_speeds[1].away_team.abbrev, "EDM");
    }

    /// Distance-detail fixture, following the shape of Go's speed-detail test
    /// (`edge_test.go:386-414`) but for `skatingDistanceLast10`.
    #[test]
    fn test_edge_skater_distance_detail_deserializes_fixture() {
        let json = r#"{
            "player": {"id": 8478402, "firstName": {"default": "Connor"}, "lastName": {"default": "McDavid"}},
            "seasonsWithEdgeStats": [{"id": 20242025, "gameTypes": [2]}],
            "skatingDistanceLast10": [
                {"gameDate": "2025-01-15", "awayTeam": {"abbrev": "CGY", "score": 2}, "homeTeam": {"abbrev": "EDM", "score": 5}, "distance": {"imperial": 5.2, "metric": 8.4}}
            ]
        }"#;

        let detail: EdgeSkaterDistanceDetail =
            serde_json::from_str(json).expect("must deserialize");

        assert_eq!(detail.skating_distance_last10.len(), 1);
        assert_eq!(detail.skating_distance_last10[0].distance.imperial, 5.2);
        assert_eq!(detail.skating_distance_last10[0].home_team.abbrev, "EDM");
    }

    /// Shot-speed-detail fixture, mirroring `EdgeSkaterSpeedDetail`'s shape
    /// but for `hardestShots`.
    #[test]
    fn test_edge_skater_shot_speed_detail_deserializes_fixture() {
        let json = r#"{
            "player": {"id": 8478402, "firstName": {"default": "Connor"}, "lastName": {"default": "McDavid"}},
            "seasonsWithEdgeStats": [{"id": 20242025, "gameTypes": [2]}],
            "hardestShots": [
                {"gameDate": "2025-01-15", "awayTeam": {"abbrev": "CGY", "score": 2}, "homeTeam": {"abbrev": "EDM", "score": 5}, "speed": {"imperial": 100.2, "metric": 161.2}}
            ]
        }"#;

        let detail: EdgeSkaterShotSpeedDetail =
            serde_json::from_str(json).expect("must deserialize");

        assert_eq!(detail.hardest_shots.len(), 1);
        assert_eq!(detail.hardest_shots[0].speed.imperial, 100.2);
    }

    /// Shot-location-detail fixture. Gotcha C: this is a *detail* payload, so
    /// shots-on-goal is keyed `shots` (not `sog`, unlike comparison/leader
    /// payloads).
    #[test]
    fn test_edge_skater_shot_location_detail_deserializes_fixture() {
        let json = r#"{
            "player": {"id": 8478402, "firstName": {"default": "Connor"}, "lastName": {"default": "McDavid"}},
            "seasonsWithEdgeStats": [{"id": 20242025, "gameTypes": [2]}],
            "shotLocationDetails": [
                {"area": "Crease", "shots": 40, "goals": 12, "shootingPctg": 0.30}
            ]
        }"#;

        let detail: EdgeSkaterShotLocationDetail =
            serde_json::from_str(json).expect("must deserialize");

        assert_eq!(detail.shot_location_details.len(), 1);
        assert_eq!(detail.shot_location_details[0].shots, 40);
        assert_eq!(detail.shot_location_details[0].goals, 12);
    }

    /// Zone-time-detail fixture (`edge/skater-zone-time`, no `-details`
    /// suffix on the path — see [`super::EdgeSkaterZoneTimeDetail`]).
    #[test]
    fn test_edge_skater_zone_time_detail_deserializes_fixture() {
        let json = r#"{
            "player": {"id": 8478402, "firstName": {"default": "Connor"}, "lastName": {"default": "McDavid"}},
            "seasonsWithEdgeStats": [{"id": 20242025, "gameTypes": [2]}],
            "zoneTimeDetails": [
                {"strengthCode": "5v5", "offensiveZonePctg": 0.35, "neutralZonePctg": 0.30, "defensiveZonePctg": 0.35}
            ]
        }"#;

        let detail: EdgeSkaterZoneTimeDetail =
            serde_json::from_str(json).expect("must deserialize");

        assert_eq!(detail.zone_time_details.len(), 1);
        assert_eq!(detail.zone_time_details[0].strength_code, "5v5");
        assert_eq!(detail.zone_time_details[0].offensive_zone_pctg, 0.35);
    }

    /// Ported from Go's `TestEdgeSkaterComparison_RealAPIStructure`
    /// (`edge_test.go:779-863`), with three corrections applied per
    /// `REVIEW.md`'s findings that the Go fixture itself used the wrong keys
    /// (unnoticed because the Go test never asserted on those fields):
    /// `shotLocationDetails`/`shotLocationTotals` use `sog` (gotcha C), not
    /// `shots`; `skatingSpeedDetails` has a `maxSkatingSpeed` field, not
    /// `topSkatingSpeed`/`avgSkatingSpeed`; `skatingDistanceDetails` is keyed
    /// `distanceTotal`, not `totalDistance`; and the skater shape of
    /// `skatingDistanceLast10` uses `distanceSkated`, not the team-only
    /// `distance` key.
    #[test]
    fn test_edge_skater_comparison_deserializes_fixture() {
        let json = r#"{
            "player": {
                "id": 8478402,
                "firstName": {"default": "Connor"},
                "lastName": {"default": "McDavid"},
                "birthDate": "1997-01-13",
                "shootsCatches": "L",
                "sweaterNumber": 97,
                "position": "C",
                "slug": "connor-mcdavid",
                "headshot": "h",
                "goals": 64,
                "assists": 89,
                "points": 153,
                "gamesPlayed": 82,
                "team": {"id": 22, "commonName": {"default": "Oilers"}, "placeNameWithPreposition": {"default": "Edmonton"}, "abbrev": "EDM", "teamLogo": {"light": "l", "dark": "d"}, "slug": "s", "conference": "W", "division": "P", "wins": 50, "losses": 25, "otLosses": 7, "gamesPlayed": 82, "points": 107}
            },
            "seasonsWithEdgeStats": [{"id": 20232024, "gameTypes": [2, 3]}],
            "shotSpeedDetails": {
                "topShotSpeed": {"imperial": 98.5, "metric": 158.5},
                "avgShotSpeed": {"imperial": 85.0, "metric": 136.8},
                "shotAttemptsOver100": 5,
                "shotAttempts90To100": 20
            },
            "skatingSpeedDetails": {
                "maxSkatingSpeed": {"imperial": 23.5, "metric": 37.8},
                "burstsOver22": 7,
                "bursts20To22": 3,
                "bursts18To20": 12
            },
            "skatingDistanceLast10": [
                {"gameDate": "2024-04-15", "distanceSkated": {"imperial": 5.2, "metric": 8.4}}
            ],
            "skatingDistanceDetails": {
                "distanceTotal": {"imperial": 500.0, "metric": 804.7}
            },
            "shotLocationDetails": [
                {"area": "Crease", "sog": 50, "goals": 15, "shootingPctg": 0.30}
            ],
            "shotLocationTotals": [
                {"locationCode": "all", "sog": 300, "goals": 64, "shootingPctg": 0.213}
            ],
            "zoneTimeDetails": {
                "offensiveZonePctg": 0.38,
                "offensiveZoneLeagueAvg": 0.32,
                "neutralZonePctg": 0.30,
                "defensiveZonePctg": 0.32
            },
            "zoneStarts": {
                "offensiveZoneStarts": 55.0,
                "neutralZoneStarts": 25.0,
                "defensiveZoneStarts": 20.0
            }
        }"#;

        let comparison: EdgeSkaterComparison =
            serde_json::from_str(json).expect("must deserialize");

        assert_eq!(comparison.player.id, 8478402);

        let shot_speed = comparison
            .shot_speed_details
            .expect("shotSpeedDetails present");
        assert_eq!(
            shot_speed
                .avg_shot_speed
                .expect("avgShotSpeed present")
                .imperial,
            85.0
        );

        let skating_speed = comparison
            .skating_speed_details
            .expect("skatingSpeedDetails present");
        assert_eq!(
            skating_speed
                .max_skating_speed
                .expect("maxSkatingSpeed present")
                .imperial,
            23.5
        );
        assert_eq!(skating_speed.bursts_over_22, 7);

        assert_eq!(comparison.skating_distance_last10.len(), 1);
        assert_eq!(
            comparison.skating_distance_last10[0]
                .distance_skated
                .as_ref()
                .expect("distanceSkated present")
                .imperial,
            5.2
        );

        let skating_distance = comparison
            .skating_distance_details
            .expect("skatingDistanceDetails present");
        assert_eq!(
            skating_distance
                .distance_total
                .expect("distanceTotal present")
                .imperial,
            500.0
        );

        assert_eq!(comparison.shot_location_details.len(), 1);
        assert_eq!(comparison.shot_location_details[0].sog, 50);
        assert_eq!(comparison.shot_location_totals.len(), 1);
        assert_eq!(comparison.shot_location_totals[0].sog, 300);

        let zone_time = comparison
            .zone_time_details
            .expect("zoneTimeDetails present");
        assert_eq!(zone_time.offensive_zone_pctg, 0.38);

        let zone_starts = comparison.zone_starts.expect("zoneStarts present");
        assert_eq!(zone_starts.offensive_zone_starts, 55.0);
    }

    /// Ported from Go's `TestEdgeLanding_Client` fixture body
    /// (`edge_test.go:561-593`): the real API returns leader entries as
    /// objects keyed by category, and only one stat field is populated per
    /// category.
    #[test]
    fn test_edge_skater_landing_deserializes_fixture() {
        let json = r#"{
            "seasonsWithEdgeStats": [{"id": 20242025, "gameTypes": [2]}],
            "leaders": {
                "hardestShot": {
                    "player": {"id": 8478402, "firstName": {"default": "Connor"}, "lastName": {"default": "McDavid"}},
                    "shotSpeed": {"imperial": 100.0, "metric": 160.9}
                },
                "highDangerSog": {
                    "player": {"id": 8479318, "firstName": {"default": "Igor"}, "lastName": {"default": "Shesterkin"}},
                    "sog": 42,
                    "shotLocationDetails": [
                        {"area": "HighDanger", "sog": 42, "sogPercentile": 0.91}
                    ]
                }
            }
        }"#;

        let landing: EdgeSkaterLanding = serde_json::from_str(json).expect("must deserialize");

        assert_eq!(landing.seasons_with_edge_stats.len(), 1);
        assert_eq!(landing.leaders.len(), 2);

        let hardest_shot = landing.leaders.get("hardestShot").expect("present");
        assert_eq!(hardest_shot.player.id, 8478402);
        assert_eq!(
            hardest_shot
                .shot_speed
                .as_ref()
                .expect("shotSpeed present")
                .imperial,
            100.0
        );
        assert_eq!(hardest_shot.sog, None);

        let high_danger = landing.leaders.get("highDangerSog").expect("present");
        assert_eq!(high_danger.sog, Some(42));
        assert_eq!(high_danger.shot_location_details.len(), 1);
        assert_eq!(high_danger.shot_location_details[0].sog, Some(42));
    }

    #[test]
    fn test_edge_skater_leader_mutually_exclusive_stat_fields() {
        let json = r#"{"player": {"id": 1}, "skatingSpeed": {"imperial": 22.0, "metric": 35.4}}"#;
        let leader: EdgeSkaterLeader = serde_json::from_str(json).expect("must deserialize");

        assert!(leader.skating_speed.is_some());
        assert_eq!(leader.shot_speed, None);
        assert_eq!(leader.distance_skated, None);
        assert_eq!(leader.sog, None);
        assert_eq!(leader.zone_time, None);
    }

    #[test]
    fn test_edge_skater_player_localized_name_round_trip() {
        let player = EdgeSkaterPlayer {
            id: 8478402,
            first_name: LocalizedString {
                default: "Connor".to_string(),
            },
            ..Default::default()
        };
        let json = serde_json::to_string(&player).unwrap();
        let back: EdgeSkaterPlayer = serde_json::from_str(&json).unwrap();
        assert_eq!(player, back);
    }
}
