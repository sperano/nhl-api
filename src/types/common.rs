use serde::{Deserialize, Serialize};
use std::fmt;

/// Localized string (NHL API returns {default: "value"})
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct LocalizedString {
    pub default: String,
}

/// Conference information for a team
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub franchise_id: Option<i64>,
}

impl fmt::Display for Team {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.abbr)
    }
}

/// Franchise information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Franchise {
    pub id: i64,
    #[serde(rename = "fullName")]
    pub full_name: String,
    #[serde(rename = "teamCommonName")]
    pub team_common_name: Option<String>,
}

/// Response from the franchises endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FranchisesResponse {
    pub data: Vec<Franchise>,
}

/// Team roster information
/// Team roster with players by position
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Roster {
    #[serde(default)]
    pub forwards: Vec<RosterPlayer>,
    #[serde(default)]
    pub defensemen: Vec<RosterPlayer>,
    #[serde(default)]
    pub goalies: Vec<RosterPlayer>,
}

/// Individual player in a team roster
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RosterPlayer {
    pub id: i64,
    pub headshot: String,
    #[serde(rename = "firstName")]
    pub first_name: LocalizedString,
    #[serde(rename = "lastName")]
    pub last_name: LocalizedString,
    #[serde(rename = "sweaterNumber")]
    pub sweater_number: i32,
    #[serde(rename = "positionCode")]
    pub position_code: String,
    #[serde(rename = "shootsCatches")]
    pub shoots_catches: String,
    #[serde(rename = "heightInInches")]
    pub height_in_inches: i32,
    #[serde(rename = "weightInPounds")]
    pub weight_in_pounds: i32,
    #[serde(rename = "heightInCentimeters")]
    pub height_in_centimeters: i32,
    #[serde(rename = "weightInKilograms")]
    pub weight_in_kilograms: i32,
    #[serde(rename = "birthDate")]
    pub birth_date: String,
    #[serde(rename = "birthCity")]
    pub birth_city: LocalizedString,
    #[serde(rename = "birthCountry")]
    pub birth_country: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "birthStateProvince")]
    pub birth_state_province: Option<LocalizedString>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_team_serialization() {
        let team = Team {
            name: "Buffalo Sabres".to_string(),
            common_name: "Sabres".to_string(),
            abbr: "BUF".to_string(),
            logo: "https://assets.nhle.com/logos/nhl/svg/BUF_light.svg".to_string(),
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

        assert_eq!(deserialized.name, "Buffalo Sabres");
        assert_eq!(deserialized.abbr, "BUF");
        assert_eq!(deserialized.franchise_id, Some(19));
    }

    #[test]
    fn test_franchise_response_deserialization() {
        let json = r#"{
            "data": [
                {
                    "id": 19,
                    "fullName": "Buffalo Sabres",
                    "teamCommonName": "Sabres"
                }
            ]
        }"#;

        let response: FranchisesResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data[0].id, 19);
        assert_eq!(response.data[0].full_name, "Buffalo Sabres");
    }

    #[test]
    fn test_team_display() {
        let team = Team {
            name: "Buffalo Sabres".to_string(),
            common_name: "Sabres".to_string(),
            abbr: "BUF".to_string(),
            logo: "https://assets.nhle.com/logos/nhl/svg/BUF_light.svg".to_string(),
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

        assert_eq!(team.to_string(), "Buffalo Sabres (BUF)");
    }
}
