use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Conference {
    pub abbr: String,
    pub name: String,
}

/// Division information for a team
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Division {
    pub abbr: String,
    pub name: String,
}

/// NHL Team information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Team {
    pub name: String,
    pub common_name: String,
    pub abbr: String,
    pub logo: String,
    pub conference: Conference,
    pub division: Division,
    // TODO?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub franchise_id: Option<i64>,
}

impl fmt::Display for Team {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.abbr)
    }
}

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

/// Localized string (NHL API returns {default: "value"})
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct LocalizedString {
    pub default: String,
}

/// Standings response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StandingsResponse {
    pub standings: Vec<Standing>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_team_serialization() {
        let team = Team {
            name: "Montreal Canadiens".to_string(),
            common_name: "Canadiens".to_string(),
            abbr: "MTL".to_string(),
            logo: "https://assets.nhle.com/logos/nhl/svg/MTL_light.svg".to_string(),
            conference: Conference {
                abbr: "E".to_string(),
                name: "Eastern".to_string(),
            },
            division: Division {
                abbr: "ATL".to_string(),
                name: "Atlantic".to_string(),
            },
            franchise_id: Some(19),
        };

        let json = serde_json::to_string(&team).unwrap();
        let deserialized: Team = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.name, "Montreal Canadiens");
        assert_eq!(deserialized.common_name, "Canadiens");
        assert_eq!(deserialized.abbr, "MTL");
        assert_eq!(deserialized.franchise_id, Some(19));
    }

    #[test]
    fn test_team_display() {
        // TODO a test wth Montreal
    }

    
}
