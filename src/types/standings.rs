use serde::{Deserialize, Serialize};
use std::fmt;

use super::common::{Conference, Division, LocalizedString, Team};

/// Standing entry for a team
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Standing {
    #[serde(rename = "conferenceAbbrev")]
    pub conference_abbrev: String,
    #[serde(rename = "conferenceName")]
    pub conference_name: String,
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
    /// Convert a Standing entry into a Team struct
    pub fn to_team(&self) -> Team {
        Team {
            name: self.team_name.default.clone(),
            common_name: self.team_common_name.default.clone(),
            abbr: self.team_abbrev.default.clone(),
            logo: self.team_logo.clone(),
            conference: Conference {
                abbr: self.conference_abbrev.clone(),
                name: self.conference_name.clone(),
            },
            division: Division {
                abbr: self.division_abbrev.clone(),
                name: self.division_name.clone(),
            },
            franchise_id: None,
        }
    }
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
    pub id: i64,
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
            conference_abbrev: "W".to_string(),
            conference_name: "Western".to_string(),
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
        assert_eq!(team.abbr, "VGK");
        assert_eq!(team.logo, "https://assets.nhle.com/logos/nhl/svg/VGK_light.svg");
        assert_eq!(team.conference.abbr, "W");
        assert_eq!(team.conference.name, "Western");
        assert_eq!(team.division.abbr, "PAC");
        assert_eq!(team.division.name, "Pacific");
        assert_eq!(team.franchise_id, None);
    }

    #[test]
    fn test_standing_display() {
        let standing = Standing {
            conference_abbrev: "E".to_string(),
            conference_name: "Eastern".to_string(),
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
}
