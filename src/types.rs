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
}