use serde::{Deserialize, Serialize};
use std::fmt;

use crate::date::Season;

use super::common::{Conference, Division, LocalizedString, Team};

/// Standing entry for a team
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Standing {
    #[serde(rename = "conferenceAbbrev", skip_serializing_if = "Option::is_none")]
    pub conference_abbrev: Option<String>,
    #[serde(rename = "conferenceName", skip_serializing_if = "Option::is_none")]
    pub conference_name: Option<String>,
    #[serde(rename = "divisionAbbrev")]
    pub division_abbrev: String,
    #[serde(rename = "divisionName")]
    pub division_name: String,
    #[serde(rename = "teamName")]
    pub team_name: LocalizedString,
    #[serde(rename = "teamCommonName")]
    pub team_common_name: LocalizedString,
    #[serde(rename = "teamAbbrev")]
    pub team_abbrev: LocalizedString,
    #[serde(rename = "teamLogo")]
    pub team_logo: String,
    #[serde(rename = "wins")]
    pub wins: i32,
    #[serde(rename = "losses")]
    pub losses: i32,
    #[serde(rename = "otLosses")]
    pub ot_losses: i32,
    #[serde(rename = "points")]
    pub points: i32,
}

impl Standing {
    const UNKNOWN_CONFERENCE_ABBR: &'static str = "UNK";
    const UNKNOWN_CONFERENCE_NAME: &'static str = "Unknown";

    fn conference_abbrev(&self) -> &str {
        self.conference_abbrev
            .as_deref()
            .unwrap_or(Self::UNKNOWN_CONFERENCE_ABBR)
    }

    fn conference_name(&self) -> &str {
        self.conference_name
            .as_deref()
            .unwrap_or(Self::UNKNOWN_CONFERENCE_NAME)
    }

    /// Convert a Standing entry into a Team struct
    pub fn to_team(&self) -> Team {
        Team {
            name: self.team_name.default.clone(),
            common_name: self.team_common_name.default.clone(),
            place_name: LocalizedString {
                default: place_name(&self.team_name.default, &self.team_common_name.default),
            },
            abbr: self.team_abbrev.default.clone(),
            logo: self.team_logo.clone(),
            conference: Conference {
                abbr: self.conference_abbrev().to_string(),
                name: self.conference_name().to_string(),
            },
            division: Division {
                abbr: self.division_abbrev.clone(),
                name: self.division_name.clone(),
            },
            franchise_id: None,
        }
    }

    pub fn games_played(&self) -> i32 {
        self.wins + self.losses + self.ot_losses
    }
}

/// Reconstructs a team's place name (e.g. `"Toronto"`) from its full name
/// (e.g. `"Toronto Maple Leafs"`) by removing the common name (e.g. `"Maple
/// Leafs"`). The NHL standings endpoint carries no dedicated place-name
/// field, so it must be derived. The common name may sit at either the start
/// or the end of the full name, so only its first occurrence is removed,
/// wherever it appears, and the remaining whitespace is normalized. If the
/// common name is empty or not found within the full name, the full name is
/// returned unchanged.
fn place_name(full_name: &str, common_name: &str) -> String {
    if common_name.is_empty() {
        return full_name.to_string();
    }
    let stripped = full_name.replacen(common_name, "", 1);
    stripped.split_whitespace().collect::<Vec<_>>().join(" ")
}

impl fmt::Display for Standing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {} pts ({}-{}-{})",
            self.team_abbrev.default, self.points, self.wins, self.losses, self.ot_losses
        )
    }
}

/// Standings response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StandingsResponse {
    pub standings: Vec<Standing>,
}

/// Season manifest entry
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SeasonInfo {
    pub id: Season,
    #[serde(rename = "standingsStart")]
    pub standings_start: String,
    #[serde(rename = "standingsEnd")]
    pub standings_end: String,
}

/// Seasons manifest response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SeasonsResponse {
    pub seasons: Vec<SeasonInfo>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standings_response_deserialization() {
        let json = r#"{
            "standings": [
                {
                    "conferenceAbbrev": "E",
                    "conferenceName": "Eastern",
                    "divisionAbbrev": "ATL",
                    "divisionName": "Atlantic",
                    "teamName": {"default": "Buffalo Sabres"},
                    "teamCommonName": {"default": "Sabres"},
                    "teamAbbrev": {"default": "BUF"},
                    "teamLogo": "https://assets.nhle.com/logos/nhl/svg/BUF_light.svg",
                    "wins": 10,
                    "losses": 5,
                    "otLosses": 2,
                    "points": 22
                }
            ]
        }"#;

        let response: StandingsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.standings.len(), 1);
        assert_eq!(response.standings[0].team_abbrev.default, "BUF");
        assert_eq!(response.standings[0].wins, 10);
        assert_eq!(response.standings[0].points, 22);
    }

    #[test]
    fn test_standing_to_team_conversion() {
        let standing = Standing {
            conference_abbrev: Some("W".to_string()),
            conference_name: Some("Western".to_string()),
            division_abbrev: "PAC".to_string(),
            division_name: "Pacific".to_string(),
            team_name: LocalizedString {
                default: "Vegas Golden Knights".to_string(),
            },
            team_common_name: LocalizedString {
                default: "Golden Knights".to_string(),
            },
            team_abbrev: LocalizedString {
                default: "VGK".to_string(),
            },
            team_logo: "https://assets.nhle.com/logos/nhl/svg/VGK_light.svg".to_string(),
            wins: 12,
            losses: 3,
            ot_losses: 1,
            points: 25,
        };

        let team = standing.to_team();

        assert_eq!(team.name, "Vegas Golden Knights");
        assert_eq!(team.common_name, "Golden Knights");
        assert_eq!(team.place_name.default, "Vegas");
        assert_eq!(team.abbr, "VGK");
        assert_eq!(
            team.logo,
            "https://assets.nhle.com/logos/nhl/svg/VGK_light.svg"
        );
        assert_eq!(team.conference.abbr, "W");
        assert_eq!(team.conference.name, "Western");
        assert_eq!(team.division.abbr, "PAC");
        assert_eq!(team.division.name, "Pacific");
        assert_eq!(team.franchise_id, None);
    }

    #[test]
    fn test_standing_display() {
        let standing = Standing {
            conference_abbrev: Some("E".to_string()),
            conference_name: Some("Eastern".to_string()),
            division_abbrev: "ATL".to_string(),
            division_name: "Atlantic".to_string(),
            team_name: LocalizedString {
                default: "Boston Bruins".to_string(),
            },
            team_common_name: LocalizedString {
                default: "Bruins".to_string(),
            },
            team_abbrev: LocalizedString {
                default: "BOS".to_string(),
            },
            team_logo: "https://assets.nhle.com/logos/nhl/svg/BOS_light.svg".to_string(),
            wins: 15,
            losses: 2,
            ot_losses: 1,
            points: 31,
        };

        assert_eq!(standing.to_string(), "BOS: 31 pts (15-2-1)");
    }

    #[test]
    fn test_standings_response_with_extra_fields() {
        // Test that deserialization works even with extra API fields
        let json = r#"{
            "wildCardIndicator": true,
            "standingsDateTimeUtc": "2024-01-15T12:00:00Z",
            "standings": [
                {
                    "conferenceAbbrev": "E",
                    "conferenceName": "Eastern",
                    "divisionAbbrev": "ATL",
                    "divisionName": "Atlantic",
                    "teamName": {"default": "Buffalo Sabres"},
                    "teamCommonName": {"default": "Sabres"},
                    "teamAbbrev": {"default": "BUF"},
                    "teamLogo": "https://assets.nhle.com/logos/nhl/svg/BUF_light.svg",
                    "wins": 10,
                    "losses": 5,
                    "otLosses": 2,
                    "points": 22
                }
            ]
        }"#;

        let response: StandingsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.standings.len(), 1);
        assert_eq!(response.standings[0].team_abbrev.default, "BUF");
    }

    #[test]
    fn test_standings_response_empty() {
        // Test that empty standings array works (like for dates before NHL existed)
        let json = r#"{
            "wildCardIndicator": true,
            "standings": []
        }"#;

        let response: StandingsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.standings.len(), 0);
    }

    #[test]
    fn test_standings_without_conference_fields() {
        // Test deserialization of historical data without conference fields (pre-1975)
        let json = r#"{
            "standings": [
                {
                    "divisionAbbrev": "EAST",
                    "divisionName": "East",
                    "teamName": {"default": "Boston Bruins"},
                    "teamCommonName": {"default": "Bruins"},
                    "teamAbbrev": {"default": "BOS"},
                    "teamLogo": "https://assets.nhle.com/logos/nhl/svg/BOS_light.svg",
                    "wins": 20,
                    "losses": 10,
                    "otLosses": 5,
                    "points": 45
                }
            ]
        }"#;

        let response: StandingsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.standings.len(), 1);

        let standing = &response.standings[0];
        assert_eq!(standing.conference_abbrev, None);
        assert_eq!(standing.conference_name, None);
        assert_eq!(standing.division_abbrev, "EAST");
        assert_eq!(standing.team_abbrev.default, "BOS");
        assert_eq!(standing.wins, 20);
    }

    #[test]
    fn test_standing_to_team_without_conference() {
        // Test that to_team() works with None conference values
        let standing = Standing {
            conference_abbrev: None,
            conference_name: None,
            division_abbrev: "EAST".to_string(),
            division_name: "East".to_string(),
            team_name: LocalizedString {
                default: "Montreal Canadiens".to_string(),
            },
            team_common_name: LocalizedString {
                default: "Canadiens".to_string(),
            },
            team_abbrev: LocalizedString {
                default: "MTL".to_string(),
            },
            team_logo: "https://assets.nhle.com/logos/nhl/svg/MTL_light.svg".to_string(),
            wins: 25,
            losses: 8,
            ot_losses: 3,
            points: 53,
        };

        let team = standing.to_team();

        assert_eq!(team.name, "Montreal Canadiens");
        assert_eq!(team.common_name, "Canadiens");
        assert_eq!(team.place_name.default, "Montreal");
        assert_eq!(team.abbr, "MTL");
        assert_eq!(team.conference.abbr, "UNK");
        assert_eq!(team.conference.name, "Unknown");
        assert_eq!(team.division.abbr, "EAST");
        assert_eq!(team.division.name, "East");
    }

    #[test]
    fn test_games_played_typical_season() {
        let standing = Standing {
            conference_abbrev: Some("E".to_string()),
            conference_name: Some("Eastern".to_string()),
            division_abbrev: "ATL".to_string(),
            division_name: "Atlantic".to_string(),
            team_name: LocalizedString {
                default: "Toronto Maple Leafs".to_string(),
            },
            team_common_name: LocalizedString {
                default: "Maple Leafs".to_string(),
            },
            team_abbrev: LocalizedString {
                default: "TOR".to_string(),
            },
            team_logo: "https://assets.nhle.com/logos/nhl/svg/TOR_light.svg".to_string(),
            wins: 15,
            losses: 10,
            ot_losses: 2,
            points: 32,
        };

        assert_eq!(standing.games_played(), 27); // 15 + 10 + 2
    }

    #[test]
    fn test_games_played_zero_games() {
        let standing = Standing {
            conference_abbrev: Some("W".to_string()),
            conference_name: Some("Western".to_string()),
            division_abbrev: "CEN".to_string(),
            division_name: "Central".to_string(),
            team_name: LocalizedString {
                default: "Test Team".to_string(),
            },
            team_common_name: LocalizedString {
                default: "Test".to_string(),
            },
            team_abbrev: LocalizedString {
                default: "TST".to_string(),
            },
            team_logo: "https://example.com/logo.svg".to_string(),
            wins: 0,
            losses: 0,
            ot_losses: 0,
            points: 0,
        };

        assert_eq!(standing.games_played(), 0);
    }

    #[test]
    fn test_games_played_only_wins() {
        let standing = Standing {
            conference_abbrev: Some("E".to_string()),
            conference_name: Some("Eastern".to_string()),
            division_abbrev: "ATL".to_string(),
            division_name: "Atlantic".to_string(),
            team_name: LocalizedString {
                default: "Undefeated Team".to_string(),
            },
            team_common_name: LocalizedString {
                default: "Undefeated".to_string(),
            },
            team_abbrev: LocalizedString {
                default: "UND".to_string(),
            },
            team_logo: "https://example.com/logo.svg".to_string(),
            wins: 10,
            losses: 0,
            ot_losses: 0,
            points: 20,
        };

        assert_eq!(standing.games_played(), 10);
    }

    #[test]
    fn test_games_played_only_losses() {
        let standing = Standing {
            conference_abbrev: Some("W".to_string()),
            conference_name: Some("Western".to_string()),
            division_abbrev: "PAC".to_string(),
            division_name: "Pacific".to_string(),
            team_name: LocalizedString {
                default: "Winless Team".to_string(),
            },
            team_common_name: LocalizedString {
                default: "Winless".to_string(),
            },
            team_abbrev: LocalizedString {
                default: "WLS".to_string(),
            },
            team_logo: "https://example.com/logo.svg".to_string(),
            wins: 0,
            losses: 15,
            ot_losses: 0,
            points: 0,
        };

        assert_eq!(standing.games_played(), 15);
    }

    #[test]
    fn test_games_played_only_ot_losses() {
        let standing = Standing {
            conference_abbrev: Some("E".to_string()),
            conference_name: Some("Eastern".to_string()),
            division_abbrev: "MET".to_string(),
            division_name: "Metropolitan".to_string(),
            team_name: LocalizedString {
                default: "OT Loss Team".to_string(),
            },
            team_common_name: LocalizedString {
                default: "OT Loss".to_string(),
            },
            team_abbrev: LocalizedString {
                default: "OTL".to_string(),
            },
            team_logo: "https://example.com/logo.svg".to_string(),
            wins: 0,
            losses: 0,
            ot_losses: 5,
            points: 5,
        };

        assert_eq!(standing.games_played(), 5);
    }

    #[test]
    fn test_games_played_full_season() {
        let standing = Standing {
            conference_abbrev: Some("W".to_string()),
            conference_name: Some("Western".to_string()),
            division_abbrev: "CEN".to_string(),
            division_name: "Central".to_string(),
            team_name: LocalizedString {
                default: "Colorado Avalanche".to_string(),
            },
            team_common_name: LocalizedString {
                default: "Avalanche".to_string(),
            },
            team_abbrev: LocalizedString {
                default: "COL".to_string(),
            },
            team_logo: "https://assets.nhle.com/logos/nhl/svg/COL_light.svg".to_string(),
            wins: 50,
            losses: 20,
            ot_losses: 12,
            points: 112,
        };

        assert_eq!(standing.games_played(), 82); // Full 82-game season
    }

    #[test]
    fn test_games_played_with_existing_standings() {
        // Verify calculation matches the standings used in other tests
        let standing = Standing {
            conference_abbrev: Some("E".to_string()),
            conference_name: Some("Eastern".to_string()),
            division_abbrev: "ATL".to_string(),
            division_name: "Atlantic".to_string(),
            team_name: LocalizedString {
                default: "Buffalo Sabres".to_string(),
            },
            team_common_name: LocalizedString {
                default: "Sabres".to_string(),
            },
            team_abbrev: LocalizedString {
                default: "BUF".to_string(),
            },
            team_logo: "https://assets.nhle.com/logos/nhl/svg/BUF_light.svg".to_string(),
            wins: 10,
            losses: 5,
            ot_losses: 2,
            points: 22,
        };

        assert_eq!(standing.games_played(), 17); // 10 + 5 + 2
    }

    // Port of Go's `TestPlaceName` table (`nhl/standings_test.go`).
    mod place_name_tests {
        use super::place_name;

        #[test]
        fn test_place_name_common_name_at_end() {
            assert_eq!(place_name("Toronto Maple Leafs", "Maple Leafs"), "Toronto");
        }

        #[test]
        fn test_place_name_common_name_at_start() {
            assert_eq!(place_name("Maple Leafs Toronto", "Maple Leafs"), "Toronto");
        }

        #[test]
        fn test_place_name_single_word_place() {
            assert_eq!(place_name("Boston Bruins", "Bruins"), "Boston");
        }

        #[test]
        fn test_place_name_multi_word_place() {
            assert_eq!(place_name("Tampa Bay Lightning", "Lightning"), "Tampa Bay");
        }

        #[test]
        fn test_place_name_common_name_not_found_falls_back_to_full_name() {
            assert_eq!(
                place_name("Vegas Golden Knights", "Senators"),
                "Vegas Golden Knights"
            );
        }

        #[test]
        fn test_place_name_empty_common_name_falls_back_to_full_name() {
            assert_eq!(
                place_name("Vegas Golden Knights", ""),
                "Vegas Golden Knights"
            );
        }

        #[test]
        fn test_place_name_full_name_equals_common_name_yields_empty() {
            assert_eq!(place_name("Wild", "Wild"), "");
        }

        #[test]
        fn test_place_name_interior_whitespace_is_collapsed() {
            assert_eq!(
                place_name("Montreal Habs Canadiens", "Habs"),
                "Montreal Canadiens"
            );
        }
    }

    /// `to_team()` falls back to the full name when the common name can't be
    /// found within it (mirrors the Go `TestPlaceName` fallback case, applied
    /// end-to-end through the conversion).
    #[test]
    fn test_standing_to_team_place_name_fallback_when_common_name_not_found() {
        let standing = Standing {
            conference_abbrev: Some("W".to_string()),
            conference_name: Some("Western".to_string()),
            division_abbrev: "PAC".to_string(),
            division_name: "Pacific".to_string(),
            team_name: LocalizedString {
                default: "Vegas Golden Knights".to_string(),
            },
            team_common_name: LocalizedString {
                default: "Senators".to_string(),
            },
            team_abbrev: LocalizedString {
                default: "VGK".to_string(),
            },
            team_logo: "https://assets.nhle.com/logos/nhl/svg/VGK_light.svg".to_string(),
            wins: 0,
            losses: 0,
            ot_losses: 0,
            points: 0,
        };

        let team = standing.to_team();
        assert_eq!(team.place_name.default, "Vegas Golden Knights");
    }

    #[test]
    fn test_season_info_deserialization() {
        let json = r#"{
            "id": 20242025,
            "standingsStart": "2024-10-04",
            "standingsEnd": "2025-04-17"
        }"#;

        let season_info: SeasonInfo = serde_json::from_str(json).unwrap();
        assert_eq!(season_info.id, Season::new(2024));
        assert_eq!(season_info.standings_start, "2024-10-04");
        assert_eq!(season_info.standings_end, "2025-04-17");
    }

    /// `SeasonInfo.id` is typed `Season`, which accepts the API's string
    /// forms (e.g. `"20242025"`) as well as the plain-integer form (1.1).
    #[test]
    fn test_season_info_id_deserializes_from_string_form() {
        let json = r#"{
            "id": "20242025",
            "standingsStart": "2024-10-04",
            "standingsEnd": "2025-04-17"
        }"#;

        let season_info: SeasonInfo = serde_json::from_str(json).unwrap();
        assert_eq!(season_info.id, Season::new(2024));
    }

    #[test]
    fn test_seasons_response_deserialization() {
        let json = r#"{
            "seasons": [
                {
                    "id": 20242025,
                    "standingsStart": "2024-10-04",
                    "standingsEnd": "2025-04-17"
                },
                {
                    "id": 20232024,
                    "standingsStart": "2023-10-10",
                    "standingsEnd": "2024-04-18"
                }
            ]
        }"#;

        let response: SeasonsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.seasons.len(), 2);
        assert_eq!(response.seasons[0].id, Season::new(2024));
        assert_eq!(response.seasons[1].id, Season::new(2023));
    }
}
