//! Shared building blocks for the NHL Edge stats endpoints.
//!
//! See the module-level documentation in [`super`](crate::types::edge) for the
//! two design rules every type here obeys (deserialize-from-`{}` and
//! no-`Option`-on-plain-scalar-counts) and the field-naming gotchas.

use serde::{Deserialize, Serialize};

use crate::types::{GameOutcome, LocalizedString, PeriodDescriptor};

/// A single value expressed in both imperial and metric units.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct EdgeMeasurement {
    pub imperial: f64,
    pub metric: f64,
}

/// An [`EdgeMeasurement`] carrying an optional "best-of" game-context overlay.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct EdgeMeasurementWithOverlay {
    pub imperial: f64,
    pub metric: f64,
    /// Present only when the value corresponds to a specific tracked moment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overlay: Option<EdgeOverlay>,
}

/// A measurement with its league-relative percentile and league average.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct EdgePercentileStat {
    pub imperial: f64,
    pub metric: f64,
    pub percentile: f64,
    pub league_avg: EdgeMeasurement,
}

/// An [`EdgePercentileStat`] carrying an optional game-context overlay.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct EdgePercentileStatWithOverlay {
    pub imperial: f64,
    pub metric: f64,
    pub percentile: f64,
    pub league_avg: EdgeMeasurement,
    /// Present only when the value corresponds to a specific tracked moment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overlay: Option<EdgeOverlay>,
}

/// League average for a count-based stat.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct EdgeCountLeagueAvg {
    pub value: f64,
}

/// A count-based stat with percentile and league average (skater / goalie).
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct EdgeCountPercentileStat {
    pub value: i32,
    pub percentile: f64,
    pub league_avg: EdgeCountLeagueAvg,
}

/// A count-based stat with a league **rank** (1–32) instead of a percentile.
///
/// Team Edge stats are rank-based; skater/goalie stats are percentile-based.
/// These are intentionally **not** unified with [`EdgeCountPercentileStat`] —
/// `rank` and `percentile` are different league-relative measures.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct EdgeRankStat {
    pub value: i32,
    pub rank: i32,
    /// Genuinely nullable: the API omits it on some rank stats.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub league_avg: Option<EdgeCountLeagueAvg>,
}

/// A measurement stat with a league **rank** and an optional overlay.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct EdgeRankStatWithOverlay {
    pub imperial: f64,
    pub metric: f64,
    pub rank: i32,
    pub league_avg: EdgeMeasurement,
    /// Present only when the value corresponds to a specific tracked moment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overlay: Option<EdgeOverlay>,
}

/// Game context for a "best-of" tracked stat (e.g. the game a top speed
/// occurred in).
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct EdgeOverlay {
    pub player: EdgeOverlayPlayer,
    pub game_date: String,
    pub away_team: EdgeOverlayTeam,
    pub home_team: EdgeOverlayTeam,
    /// Genuinely nullable: absent for in-progress or unplayed contexts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_outcome: Option<GameOutcome>,
    pub period_descriptor: PeriodDescriptor,
    pub time_in_period: String,
    pub game_type: i32,
}

/// Player identity embedded in an [`EdgeOverlay`].
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct EdgeOverlayPlayer {
    pub first_name: LocalizedString,
    pub last_name: LocalizedString,
}

/// Team identity embedded in an [`EdgeOverlay`] or per-game entry.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct EdgeOverlayTeam {
    pub abbrev: String,
    pub score: i32,
}

/// Which season / game-type combinations have Edge data available.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct EdgeSeasonAvailability {
    pub id: i32,
    pub game_types: Vec<i32>,
}

/// Light / dark logo URLs for a team.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct EdgeTeamLogo {
    pub light: String,
    pub dark: String,
}

/// Team metadata embedded throughout the Edge responses.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct EdgeTeamInfo {
    pub id: i32,
    pub common_name: LocalizedString,
    pub place_name_with_preposition: LocalizedString,
    pub abbrev: String,
    pub team_logo: EdgeTeamLogo,
    pub slug: String,
    pub conference: String,
    pub division: String,
    pub wins: i32,
    pub losses: i32,
    pub ot_losses: i32,
    pub games_played: i32,
    pub points: i32,
}

/// Player metadata embedded in skater Edge responses.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct EdgeSkaterPlayer {
    pub id: i32,
    pub first_name: LocalizedString,
    pub last_name: LocalizedString,
    pub birth_date: String,
    pub shoots_catches: String,
    pub sweater_number: i32,
    pub position: String,
    pub slug: String,
    pub headshot: String,
    pub goals: i32,
    pub assists: i32,
    pub points: i32,
    pub games_played: i32,
    pub team: EdgeTeamInfo,
}

/// Player metadata embedded in goalie Edge responses.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct EdgeGoaliePlayer {
    pub id: i32,
    pub first_name: LocalizedString,
    pub last_name: LocalizedString,
    pub birth_date: String,
    pub shoots_catches: String,
    pub sweater_number: i32,
    pub slug: String,
    pub headshot: String,
    pub wins: i32,
    pub losses: i32,
    pub overtime_losses: i32,
    pub goals_against_avg: f64,
    pub save_pctg: f64,
    pub games_played: i32,
    pub team: EdgeTeamInfo,
}

// ===== Comparison family (shared across skater / goalie / team comparisons) =====

/// Shot-speed breakdown used in comparison endpoints.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct EdgeComparisonShotSpeedDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_shot_speed: Option<EdgeMeasurementWithOverlay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_shot_speed: Option<EdgeMeasurement>,
    pub shot_attempts_over_100: i32,
    pub shot_attempts_90_to_100: i32,
    pub shot_attempts_80_to_90: i32,
    pub shot_attempts_70_to_80: i32,
}

/// Skating-speed breakdown used in comparison endpoints.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct EdgeComparisonSkatingSpeedDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_skating_speed: Option<EdgeMeasurementWithOverlay>,
    pub bursts_over_22: i32,
    pub bursts_20_to_22: i32,
    pub bursts_18_to_20: i32,
}

/// Skating-distance breakdown used in comparison endpoints.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct EdgeComparisonSkatingDistanceDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance_total: Option<EdgeMeasurement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance_per_60: Option<EdgeMeasurement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance_max_game: Option<EdgeMeasurementWithOverlay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance_max_period: Option<EdgeMeasurementWithOverlay>,
}

/// Zone-time percentages used in comparison endpoints.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct EdgeComparisonZoneTimeDetails {
    pub offensive_zone_pctg: f64,
    pub offensive_zone_league_avg: f64,
    pub neutral_zone_pctg: f64,
    pub neutral_zone_league_avg: f64,
    pub defensive_zone_pctg: f64,
    pub defensive_zone_league_avg: f64,
}

/// Shot-location detail by rink area in comparison endpoints.
///
/// Gotcha C: comparison and leader payloads name the shots-on-goal field
/// `sog`, whereas the skater/goalie *detail* payloads name the same concept
/// `shots`. These are intentionally not unified.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct EdgeComparisonShotLocationDetail {
    pub area: String,
    pub sog: i32,
    pub goals: i32,
    pub shooting_pctg: f64,
}

/// Shot totals by location code in comparison endpoints (`sog`, see gotcha C).
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct EdgeComparisonShotLocationTotal {
    pub location_code: String,
    pub sog: i32,
    pub goals: i32,
    pub shooting_pctg: f64,
}

/// A per-game distance entry in a comparison's last-10 array.
///
/// Gotcha C: the skater shape uses the `distanceSkated` key while the team
/// shape uses `distance`, and only the team shape carries `homeTeam` /
/// `awayTeam`. All four are `Option` so a single struct deserializes both.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct EdgeComparisonDistanceLast10Entry {
    pub game_center_link: String,
    pub game_date: String,
    pub player_on_home_team: bool,
    /// Skater shape (`distanceSkated`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance_skated: Option<EdgeMeasurement>,
    pub toi: f64,
    /// Team shape only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_team: Option<EdgeOverlayTeam>,
    /// Team shape only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub away_team: Option<EdgeOverlayTeam>,
    /// Team shape (`distance`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance: Option<EdgeMeasurement>,
}

/// Zone-start percentages used in comparison endpoints.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct EdgeComparisonZoneStarts {
    pub offensive_zone_starts: f64,
    pub neutral_zone_starts: f64,
    pub defensive_zone_starts: f64,
}

/// Shot-location detail inside a landing/leader response.
///
/// Skater leaders populate `sog` / `sogPercentile`; goalie leaders populate
/// `savePctg` / `savePctgPercentile`. Every stat field is nullable — only the
/// pair relevant to the leader category is present.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct EdgeLeaderShotLocation {
    pub area: String,
    /// Skater: shots on goal (gotcha C — leader payloads use `sog`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sog: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sog_percentile: Option<f64>,
    /// Goalie: save percentage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub save_pctg: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub save_pctg_percentile: Option<f64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Asserts a type deserializes from an empty JSON object into its
    /// `Default` value — the invariant every Edge struct must uphold so the
    /// path-contract tests (which mock `200 {}`) work for all endpoints.
    fn assert_empty_object_deserializes<T>()
    where
        T: serde::de::DeserializeOwned + Default + PartialEq + std::fmt::Debug,
    {
        let value: T = serde_json::from_str("{}").expect("must deserialize from {}");
        assert_eq!(value, T::default(), "{{}} must equal Default");
    }

    /// Serializes then deserializes `value`, asserting it survives unchanged.
    fn assert_round_trip<T>(value: &T)
    where
        T: Serialize + serde::de::DeserializeOwned + PartialEq + std::fmt::Debug,
    {
        let json = serde_json::to_string(value).expect("serialize");
        let back: T = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(*value, back, "round-trip mismatch");
    }

    fn sample_overlay() -> EdgeOverlay {
        EdgeOverlay {
            player: EdgeOverlayPlayer {
                first_name: LocalizedString {
                    default: "Connor".to_string(),
                },
                last_name: LocalizedString {
                    default: "McDavid".to_string(),
                },
            },
            game_date: "2024-04-15".to_string(),
            away_team: EdgeOverlayTeam {
                abbrev: "CGY".to_string(),
                score: 2,
            },
            home_team: EdgeOverlayTeam {
                abbrev: "EDM".to_string(),
                score: 4,
            },
            game_outcome: None,
            period_descriptor: PeriodDescriptor::default(),
            time_in_period: "12:34".to_string(),
            game_type: 2,
        }
    }

    fn sample_team_info() -> EdgeTeamInfo {
        EdgeTeamInfo {
            id: 22,
            common_name: LocalizedString {
                default: "Oilers".to_string(),
            },
            place_name_with_preposition: LocalizedString {
                default: "Edmonton".to_string(),
            },
            abbrev: "EDM".to_string(),
            team_logo: EdgeTeamLogo {
                light: "l".to_string(),
                dark: "d".to_string(),
            },
            slug: "edmonton-oilers".to_string(),
            conference: "W".to_string(),
            division: "P".to_string(),
            wins: 50,
            losses: 25,
            ot_losses: 7,
            games_played: 82,
            points: 107,
        }
    }

    // ----- deserialize-from-`{}` guard, one per shared struct -----

    #[test]
    fn test_edge_measurement_empty_object() {
        assert_empty_object_deserializes::<EdgeMeasurement>();
    }

    #[test]
    fn test_edge_measurement_with_overlay_empty_object() {
        assert_empty_object_deserializes::<EdgeMeasurementWithOverlay>();
    }

    #[test]
    fn test_edge_percentile_stat_empty_object() {
        assert_empty_object_deserializes::<EdgePercentileStat>();
    }

    #[test]
    fn test_edge_percentile_stat_with_overlay_empty_object() {
        assert_empty_object_deserializes::<EdgePercentileStatWithOverlay>();
    }

    #[test]
    fn test_edge_count_league_avg_empty_object() {
        assert_empty_object_deserializes::<EdgeCountLeagueAvg>();
    }

    #[test]
    fn test_edge_count_percentile_stat_empty_object() {
        assert_empty_object_deserializes::<EdgeCountPercentileStat>();
    }

    #[test]
    fn test_edge_rank_stat_empty_object() {
        assert_empty_object_deserializes::<EdgeRankStat>();
    }

    #[test]
    fn test_edge_rank_stat_with_overlay_empty_object() {
        assert_empty_object_deserializes::<EdgeRankStatWithOverlay>();
    }

    #[test]
    fn test_edge_overlay_empty_object() {
        assert_empty_object_deserializes::<EdgeOverlay>();
    }

    #[test]
    fn test_edge_overlay_player_empty_object() {
        assert_empty_object_deserializes::<EdgeOverlayPlayer>();
    }

    #[test]
    fn test_edge_overlay_team_empty_object() {
        assert_empty_object_deserializes::<EdgeOverlayTeam>();
    }

    #[test]
    fn test_edge_season_availability_empty_object() {
        assert_empty_object_deserializes::<EdgeSeasonAvailability>();
    }

    #[test]
    fn test_edge_team_logo_empty_object() {
        assert_empty_object_deserializes::<EdgeTeamLogo>();
    }

    #[test]
    fn test_edge_team_info_empty_object() {
        assert_empty_object_deserializes::<EdgeTeamInfo>();
    }

    #[test]
    fn test_edge_skater_player_empty_object() {
        assert_empty_object_deserializes::<EdgeSkaterPlayer>();
    }

    #[test]
    fn test_edge_goalie_player_empty_object() {
        assert_empty_object_deserializes::<EdgeGoaliePlayer>();
    }

    #[test]
    fn test_edge_comparison_shot_speed_details_empty_object() {
        assert_empty_object_deserializes::<EdgeComparisonShotSpeedDetails>();
    }

    #[test]
    fn test_edge_comparison_skating_speed_details_empty_object() {
        assert_empty_object_deserializes::<EdgeComparisonSkatingSpeedDetails>();
    }

    #[test]
    fn test_edge_comparison_skating_distance_details_empty_object() {
        assert_empty_object_deserializes::<EdgeComparisonSkatingDistanceDetails>();
    }

    #[test]
    fn test_edge_comparison_zone_time_details_empty_object() {
        assert_empty_object_deserializes::<EdgeComparisonZoneTimeDetails>();
    }

    #[test]
    fn test_edge_comparison_shot_location_detail_empty_object() {
        assert_empty_object_deserializes::<EdgeComparisonShotLocationDetail>();
    }

    #[test]
    fn test_edge_comparison_shot_location_total_empty_object() {
        assert_empty_object_deserializes::<EdgeComparisonShotLocationTotal>();
    }

    #[test]
    fn test_edge_comparison_distance_last10_entry_empty_object() {
        assert_empty_object_deserializes::<EdgeComparisonDistanceLast10Entry>();
    }

    #[test]
    fn test_edge_comparison_zone_starts_empty_object() {
        assert_empty_object_deserializes::<EdgeComparisonZoneStarts>();
    }

    #[test]
    fn test_edge_leader_shot_location_empty_object() {
        assert_empty_object_deserializes::<EdgeLeaderShotLocation>();
    }

    // ----- round-trip of each shared struct with representative data -----

    #[test]
    fn test_edge_measurement_round_trip() {
        assert_round_trip(&EdgeMeasurement {
            imperial: 100.0,
            metric: 160.9,
        });
    }

    #[test]
    fn test_edge_measurement_with_overlay_round_trip() {
        assert_round_trip(&EdgeMeasurementWithOverlay {
            imperial: 98.5,
            metric: 158.5,
            overlay: Some(sample_overlay()),
        });
    }

    #[test]
    fn test_edge_percentile_stat_round_trip() {
        assert_round_trip(&EdgePercentileStat {
            imperial: 400.0,
            metric: 643.7,
            percentile: 0.8,
            league_avg: EdgeMeasurement {
                imperial: 380.0,
                metric: 611.5,
            },
        });
    }

    #[test]
    fn test_edge_percentile_stat_with_overlay_round_trip() {
        assert_round_trip(&EdgePercentileStatWithOverlay {
            imperial: 100.0,
            metric: 160.9,
            percentile: 0.85,
            league_avg: EdgeMeasurement {
                imperial: 90.0,
                metric: 144.8,
            },
            overlay: Some(sample_overlay()),
        });
    }

    #[test]
    fn test_edge_count_league_avg_round_trip() {
        assert_round_trip(&EdgeCountLeagueAvg { value: 90.5 });
    }

    #[test]
    fn test_edge_count_percentile_stat_round_trip() {
        assert_round_trip(&EdgeCountPercentileStat {
            value: 100,
            percentile: 0.8,
            league_avg: EdgeCountLeagueAvg { value: 90.0 },
        });
    }

    #[test]
    fn test_edge_rank_stat_round_trip() {
        assert_round_trip(&EdgeRankStat {
            value: 80,
            rank: 2,
            league_avg: Some(EdgeCountLeagueAvg { value: 60.5 }),
        });
    }

    #[test]
    fn test_edge_rank_stat_with_overlay_round_trip() {
        assert_round_trip(&EdgeRankStatWithOverlay {
            imperial: 100.0,
            metric: 160.9,
            rank: 3,
            league_avg: EdgeMeasurement {
                imperial: 95.0,
                metric: 152.9,
            },
            overlay: None,
        });
    }

    #[test]
    fn test_edge_overlay_round_trip() {
        assert_round_trip(&sample_overlay());
    }

    #[test]
    fn test_edge_overlay_player_round_trip() {
        assert_round_trip(&EdgeOverlayPlayer {
            first_name: LocalizedString {
                default: "Connor".to_string(),
            },
            last_name: LocalizedString {
                default: "McDavid".to_string(),
            },
        });
    }

    #[test]
    fn test_edge_overlay_team_round_trip() {
        assert_round_trip(&EdgeOverlayTeam {
            abbrev: "EDM".to_string(),
            score: 4,
        });
    }

    #[test]
    fn test_edge_season_availability_round_trip() {
        assert_round_trip(&EdgeSeasonAvailability {
            id: 20232024,
            game_types: vec![2, 3],
        });
    }

    #[test]
    fn test_edge_team_logo_round_trip() {
        assert_round_trip(&EdgeTeamLogo {
            light: "light.svg".to_string(),
            dark: "dark.svg".to_string(),
        });
    }

    #[test]
    fn test_edge_team_info_round_trip() {
        assert_round_trip(&sample_team_info());
    }

    #[test]
    fn test_edge_skater_player_round_trip() {
        assert_round_trip(&EdgeSkaterPlayer {
            id: 8478402,
            first_name: LocalizedString {
                default: "Connor".to_string(),
            },
            last_name: LocalizedString {
                default: "McDavid".to_string(),
            },
            birth_date: "1997-01-13".to_string(),
            shoots_catches: "L".to_string(),
            sweater_number: 97,
            position: "C".to_string(),
            slug: "connor-mcdavid".to_string(),
            headshot: "h".to_string(),
            goals: 64,
            assists: 89,
            points: 153,
            games_played: 82,
            team: sample_team_info(),
        });
    }

    #[test]
    fn test_edge_goalie_player_round_trip() {
        assert_round_trip(&EdgeGoaliePlayer {
            id: 8479979,
            first_name: LocalizedString {
                default: "Stuart".to_string(),
            },
            last_name: LocalizedString {
                default: "Skinner".to_string(),
            },
            birth_date: "1998-11-01".to_string(),
            shoots_catches: "L".to_string(),
            sweater_number: 74,
            slug: "stuart-skinner".to_string(),
            headshot: "h".to_string(),
            wins: 36,
            losses: 20,
            overtime_losses: 4,
            goals_against_avg: 2.62,
            save_pctg: 0.905,
            games_played: 60,
            team: sample_team_info(),
        });
    }

    #[test]
    fn test_edge_comparison_shot_speed_details_round_trip() {
        assert_round_trip(&EdgeComparisonShotSpeedDetails {
            top_shot_speed: Some(EdgeMeasurementWithOverlay {
                imperial: 98.5,
                metric: 158.5,
                overlay: None,
            }),
            avg_shot_speed: Some(EdgeMeasurement {
                imperial: 85.0,
                metric: 136.8,
            }),
            shot_attempts_over_100: 5,
            shot_attempts_90_to_100: 20,
            shot_attempts_80_to_90: 30,
            shot_attempts_70_to_80: 40,
        });
    }

    #[test]
    fn test_edge_comparison_skating_speed_details_round_trip() {
        assert_round_trip(&EdgeComparisonSkatingSpeedDetails {
            max_skating_speed: None,
            bursts_over_22: 7,
            bursts_20_to_22: 3,
            bursts_18_to_20: 12,
        });
    }

    #[test]
    fn test_edge_comparison_skating_distance_details_round_trip() {
        assert_round_trip(&EdgeComparisonSkatingDistanceDetails {
            distance_total: Some(EdgeMeasurement {
                imperial: 500.0,
                metric: 804.7,
            }),
            distance_per_60: None,
            distance_max_game: None,
            distance_max_period: None,
        });
    }

    #[test]
    fn test_edge_comparison_zone_time_details_round_trip() {
        assert_round_trip(&EdgeComparisonZoneTimeDetails {
            offensive_zone_pctg: 0.38,
            offensive_zone_league_avg: 0.32,
            neutral_zone_pctg: 0.30,
            neutral_zone_league_avg: 0.34,
            defensive_zone_pctg: 0.32,
            defensive_zone_league_avg: 0.34,
        });
    }

    #[test]
    fn test_edge_comparison_shot_location_detail_round_trip() {
        assert_round_trip(&EdgeComparisonShotLocationDetail {
            area: "Crease".to_string(),
            sog: 50,
            goals: 15,
            shooting_pctg: 0.30,
        });
    }

    #[test]
    fn test_edge_comparison_shot_location_total_round_trip() {
        assert_round_trip(&EdgeComparisonShotLocationTotal {
            location_code: "all".to_string(),
            sog: 300,
            goals: 64,
            shooting_pctg: 0.213,
        });
    }

    #[test]
    fn test_edge_comparison_distance_last10_entry_skater_round_trip() {
        assert_round_trip(&EdgeComparisonDistanceLast10Entry {
            game_center_link: "/game/1".to_string(),
            game_date: "2024-04-15".to_string(),
            player_on_home_team: true,
            distance_skated: Some(EdgeMeasurement {
                imperial: 5.2,
                metric: 8.4,
            }),
            toi: 21.5,
            home_team: None,
            away_team: None,
            distance: None,
        });
    }

    #[test]
    fn test_edge_comparison_zone_starts_round_trip() {
        assert_round_trip(&EdgeComparisonZoneStarts {
            offensive_zone_starts: 55.0,
            neutral_zone_starts: 25.0,
            defensive_zone_starts: 20.0,
        });
    }

    #[test]
    fn test_edge_leader_shot_location_skater_round_trip() {
        assert_round_trip(&EdgeLeaderShotLocation {
            area: "HighDanger".to_string(),
            sog: Some(42),
            sog_percentile: Some(0.91),
            save_pctg: None,
            save_pctg_percentile: None,
        });
    }

    // ----- gotcha & rule regression tests -----

    /// Gotcha C: the skater `distanceSkated` key and the team `distance` /
    /// `homeTeam` / `awayTeam` keys both deserialize into one struct.
    #[test]
    fn test_edge_comparison_distance_last10_dual_key_shapes() {
        let skater: EdgeComparisonDistanceLast10Entry = serde_json::from_str(
            r#"{"gameDate": "2024-04-15", "distanceSkated": {"imperial": 5.2, "metric": 8.4}}"#,
        )
        .unwrap();
        assert_eq!(
            skater.distance_skated,
            Some(EdgeMeasurement {
                imperial: 5.2,
                metric: 8.4
            })
        );
        assert_eq!(skater.distance, None);

        let team: EdgeComparisonDistanceLast10Entry = serde_json::from_str(
            r#"{"gameDate": "2024-04-15", "distance": {"imperial": 5.0, "metric": 8.0},
                "homeTeam": {"abbrev": "EDM", "score": 4}, "awayTeam": {"abbrev": "CGY", "score": 2}}"#,
        )
        .unwrap();
        assert_eq!(
            team.distance,
            Some(EdgeMeasurement {
                imperial: 5.0,
                metric: 8.0
            })
        );
        assert_eq!(team.distance_skated, None);
        assert_eq!(team.home_team.unwrap().abbrev, "EDM");
        assert_eq!(team.away_team.unwrap().abbrev, "CGY");
    }

    /// Ported from Go's `TestEdgeScalarZeroValueRoundTrips`: a legitimate `0`
    /// on a plain scalar count must survive re-serialization (no omitempty /
    /// `skip_serializing_if` on non-`Option` scalars).
    #[test]
    fn test_edge_scalar_zero_value_round_trips() {
        let input = EdgeComparisonSkatingSpeedDetails {
            max_skating_speed: None,
            bursts_over_22: 0,
            bursts_20_to_22: 3,
            bursts_18_to_20: 0,
        };
        let json = serde_json::to_string(&input).unwrap();
        for key in ["burstsOver22", "bursts20To22", "bursts18To20"] {
            assert!(
                json.contains(key),
                "serialized JSON is missing {key:?} (zero value dropped?): {json}"
            );
        }
        let output: EdgeComparisonSkatingSpeedDetails = serde_json::from_str(&json).unwrap();
        assert_eq!(input, output);
    }

    /// `EdgeRankStat.leagueAvg` is genuinely optional (matches Go's
    /// `TestEdgeRankStat_OptionalLeagueAvg`).
    #[test]
    fn test_edge_rank_stat_optional_league_avg() {
        let with: EdgeRankStat =
            serde_json::from_str(r#"{"value": 80, "rank": 2, "leagueAvg": {"value": 60.5}}"#)
                .unwrap();
        assert_eq!(with.league_avg, Some(EdgeCountLeagueAvg { value: 60.5 }));

        let without: EdgeRankStat = serde_json::from_str(r#"{"value": 80, "rank": 2}"#).unwrap();
        assert_eq!(without.league_avg, None);
    }

    /// A `None` overlay is omitted on serialization and absent overlays
    /// deserialize back to `None` (matches Go's optional-overlay round-trip).
    #[test]
    fn test_edge_optional_overlay_omitted_when_none() {
        let stat = EdgePercentileStatWithOverlay {
            imperial: 100.0,
            metric: 160.9,
            percentile: 0.85,
            league_avg: EdgeMeasurement {
                imperial: 90.0,
                metric: 144.8,
            },
            overlay: None,
        };
        let json = serde_json::to_string(&stat).unwrap();
        assert!(!json.contains("overlay"), "overlay must be omitted: {json}");
        let back: EdgePercentileStatWithOverlay = serde_json::from_str(&json).unwrap();
        assert_eq!(back.overlay, None);
    }
}
