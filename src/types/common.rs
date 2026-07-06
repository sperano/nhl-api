use serde::{Deserialize, Serialize};
use std::fmt;

use super::enums::{empty_string_as_none, Handedness, Position};

/// Localized string (NHL API returns {default: "value"})
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq, Hash)]
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
    /// Team's place name (e.g. "Toronto"), reconstructed from `name` and
    /// `common_name` where the source data has no dedicated field for it
    /// (see `Standing::to_team`). `#[serde(default)]` keeps existing JSON
    /// paths working for callers that don't supply it.
    #[serde(default)]
    pub place_name: LocalizedString,
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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Franchise {
    pub id: i32,
    #[serde(rename = "fullName")]
    pub full_name: String,
    #[serde(rename = "teamCommonName")]
    pub team_common_name: String,
    #[serde(rename = "teamPlaceName")]
    pub team_place_name: String,
}

impl fmt::Display for Franchise {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (ID: {})", self.full_name, self.id)
    }
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
    /// `None` for historical roster entries (e.g. 1988 BOS) where the API
    /// returns an empty position code.
    #[serde(
        rename = "positionCode",
        deserialize_with = "empty_string_as_none",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<Position>,
    /// `None` for players with missing handedness data from the API.
    #[serde(
        rename = "shootsCatches",
        deserialize_with = "empty_string_as_none",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shoots_catches: Option<Handedness>,
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
            place_name: LocalizedString {
                default: "Buffalo".to_string(),
            },
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
        assert_eq!(deserialized.place_name.default, "Buffalo");
    }

    /// Older serialized `Team` JSON predating the `place_name` field must
    /// still deserialize, defaulting to an empty `LocalizedString`.
    #[test]
    fn test_team_deserialization_without_place_name() {
        let json = r#"{
            "name": "Buffalo Sabres",
            "common_name": "Sabres",
            "abbr": "BUF",
            "logo": "https://assets.nhle.com/logos/nhl/svg/BUF_light.svg",
            "conference": {"abbr": "E", "name": "Eastern"},
            "division": {"abbr": "ATL", "name": "Atlantic"}
        }"#;

        let team: Team = serde_json::from_str(json).unwrap();
        assert_eq!(team.place_name, LocalizedString::default());
        assert_eq!(team.place_name.default, "");
    }

    #[test]
    fn test_franchise_deserialization() {
        let json = r#"{
            "id": 32,
            "fullName": "Anaheim Ducks",
            "teamCommonName": "Ducks",
            "teamPlaceName": "Anaheim"
        }"#;

        let franchise: Franchise = serde_json::from_str(json).unwrap();
        assert_eq!(franchise.id, 32);
        assert_eq!(franchise.full_name, "Anaheim Ducks");
        assert_eq!(franchise.team_common_name, "Ducks");
        assert_eq!(franchise.team_place_name, "Anaheim");
    }

    #[test]
    fn test_franchise_display() {
        let franchise = Franchise {
            id: 16,
            full_name: "Philadelphia Flyers".to_string(),
            team_common_name: "Flyers".to_string(),
            team_place_name: "Philadelphia".to_string(),
        };

        assert_eq!(format!("{}", franchise), "Philadelphia Flyers (ID: 16)");
    }

    #[test]
    fn test_franchise_response_deserialization() {
        let json = r#"{
            "data": [
                {
                    "id": 32,
                    "fullName": "Anaheim Ducks",
                    "teamCommonName": "Ducks",
                    "teamPlaceName": "Anaheim"
                },
                {
                    "id": 8,
                    "fullName": "Brooklyn Americans",
                    "teamCommonName": "Americans",
                    "teamPlaceName": "Brooklyn"
                }
            ]
        }"#;

        let response: FranchisesResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.data.len(), 2);
        assert_eq!(response.data[0].id, 32);
        assert_eq!(response.data[0].full_name, "Anaheim Ducks");
        assert_eq!(response.data[1].id, 8);
        assert_eq!(response.data[1].full_name, "Brooklyn Americans");
    }

    #[test]
    fn test_franchise_clone() {
        let franchise = Franchise {
            id: 1,
            full_name: "Montreal Canadiens".to_string(),
            team_common_name: "Canadiens".to_string(),
            team_place_name: "Montreal".to_string(),
        };

        let cloned = franchise.clone();
        assert_eq!(franchise, cloned);
    }

    #[test]
    fn test_franchise_equality() {
        let franchise1 = Franchise {
            id: 1,
            full_name: "Montreal Canadiens".to_string(),
            team_common_name: "Canadiens".to_string(),
            team_place_name: "Montreal".to_string(),
        };

        let franchise2 = Franchise {
            id: 1,
            full_name: "Montreal Canadiens".to_string(),
            team_common_name: "Canadiens".to_string(),
            team_place_name: "Montreal".to_string(),
        };

        assert_eq!(franchise1, franchise2);
    }

    #[test]
    fn test_franchise_inequality() {
        let franchise1 = Franchise {
            id: 1,
            full_name: "Montreal Canadiens".to_string(),
            team_common_name: "Canadiens".to_string(),
            team_place_name: "Montreal".to_string(),
        };

        let franchise2 = Franchise {
            id: 6,
            full_name: "Boston Bruins".to_string(),
            team_common_name: "Bruins".to_string(),
            team_place_name: "Boston".to_string(),
        };

        assert_ne!(franchise1, franchise2);
    }

    #[test]
    fn test_franchise_empty_response() {
        let json = r#"{"data": []}"#;
        let response: FranchisesResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.data.len(), 0);
    }

    #[test]
    fn test_team_display() {
        let team = Team {
            name: "Buffalo Sabres".to_string(),
            common_name: "Sabres".to_string(),
            place_name: LocalizedString {
                default: "Buffalo".to_string(),
            },
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

    /// 1988 BOS-style historical roster entries return empty position/handedness codes.
    #[test]
    fn test_roster_player_empty_position_and_handedness() {
        let json = r#"{
            "id": 1,
            "headshot": "https://assets.nhle.com/mugs/nhl/default.png",
            "firstName": {"default": "Historical"},
            "lastName": {"default": "Player"},
            "sweaterNumber": 9,
            "positionCode": "",
            "shootsCatches": "",
            "heightInInches": 72,
            "weightInPounds": 180,
            "heightInCentimeters": 183,
            "weightInKilograms": 82,
            "birthDate": "1960-01-01",
            "birthCity": {"default": "Boston"},
            "birthCountry": "USA"
        }"#;

        let player: RosterPlayer = serde_json::from_str(json).unwrap();
        assert_eq!(player.position, None);
        assert_eq!(player.shoots_catches, None);
    }

    #[test]
    fn test_roster_player_missing_position_and_handedness() {
        let json = r#"{
            "id": 1,
            "headshot": "https://assets.nhle.com/mugs/nhl/default.png",
            "firstName": {"default": "Historical"},
            "lastName": {"default": "Player"},
            "sweaterNumber": 9,
            "heightInInches": 72,
            "weightInPounds": 180,
            "heightInCentimeters": 183,
            "weightInKilograms": 82,
            "birthDate": "1960-01-01",
            "birthCity": {"default": "Boston"},
            "birthCountry": "USA"
        }"#;

        let player: RosterPlayer = serde_json::from_str(json).unwrap();
        assert_eq!(player.position, None);
        assert_eq!(player.shoots_catches, None);
    }

    #[test]
    fn test_roster_player_real_position_and_handedness() {
        let json = r#"{
            "id": 1,
            "headshot": "https://assets.nhle.com/mugs/nhl/default.png",
            "firstName": {"default": "Connor"},
            "lastName": {"default": "McDavid"},
            "sweaterNumber": 97,
            "positionCode": "C",
            "shootsCatches": "L",
            "heightInInches": 73,
            "weightInPounds": 193,
            "heightInCentimeters": 185,
            "weightInKilograms": 88,
            "birthDate": "1997-01-13",
            "birthCity": {"default": "Richmond Hill"},
            "birthCountry": "CAN"
        }"#;

        let player: RosterPlayer = serde_json::from_str(json).unwrap();
        assert_eq!(player.position, Some(Position::Center));
        assert_eq!(player.shoots_catches, Some(Handedness::Left));
    }

    #[test]
    fn test_roster_player_serialize_omits_none_position_and_handedness() {
        let json = r#"{
            "id": 1,
            "headshot": "https://assets.nhle.com/mugs/nhl/default.png",
            "firstName": {"default": "Historical"},
            "lastName": {"default": "Player"},
            "sweaterNumber": 9,
            "heightInInches": 72,
            "weightInPounds": 180,
            "heightInCentimeters": 183,
            "weightInKilograms": 82,
            "birthDate": "1960-01-01",
            "birthCity": {"default": "Boston"},
            "birthCountry": "USA"
        }"#;

        let player: RosterPlayer = serde_json::from_str(json).unwrap();
        let serialized = serde_json::to_string(&player).unwrap();
        assert!(
            !serialized.contains("positionCode"),
            "expected positionCode to be omitted: {serialized}"
        );
        assert!(
            !serialized.contains("shootsCatches"),
            "expected shootsCatches to be omitted: {serialized}"
        );
    }
}
