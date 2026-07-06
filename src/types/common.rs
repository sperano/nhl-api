use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Serialize};
use std::fmt;

use super::enums::{empty_string_as_none, Handedness, Position};

/// Number of inches in a foot, used by [`RosterPlayer::height_feet_inches`].
const INCHES_PER_FOOT: i32 = 12;

/// `chrono` format string matching the NHL API's `birthDate` field
/// (e.g. `"1997-01-13"`).
const BIRTH_DATE_FORMAT: &str = "%Y-%m-%d";

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

impl RosterPlayer {
    /// The player's full name (first name + last name).
    pub fn full_name(&self) -> String {
        format!("{} {}", self.first_name.default, self.last_name.default)
    }

    /// A comma-joined birth place built from whichever of city, state/
    /// province, and country are present and non-empty (e.g.
    /// `"Richmond Hill, ON, CAN"`, or `"Boston, USA"` when there's no
    /// state/province on file).
    pub fn birth_place(&self) -> String {
        let state_province = self
            .birth_state_province
            .as_ref()
            .map(|s| s.default.as_str());

        [
            self.birth_city.default.as_str(),
            state_province.unwrap_or(""),
            self.birth_country.as_str(),
        ]
        .into_iter()
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(", ")
    }

    /// The player's height formatted as feet and inches (e.g. `6'2"` for a
    /// player who is 74 inches tall).
    pub fn height_feet_inches(&self) -> String {
        let feet = self.height_in_inches / INCHES_PER_FOOT;
        let inches = self.height_in_inches % INCHES_PER_FOOT;
        format!("{feet}'{inches}\"")
    }

    /// The player's age in whole years as of `on`, or `None` if `birth_date`
    /// isn't a parseable `YYYY-MM-DD` date. Takes the reference date as a
    /// parameter (rather than reading the wall clock internally) so the
    /// calculation stays pure and testable.
    pub fn age(&self, on: NaiveDate) -> Option<u32> {
        let birth_date = NaiveDate::parse_from_str(&self.birth_date, BIRTH_DATE_FORMAT).ok()?;
        let mut years = on.year() - birth_date.year();
        if (on.month(), on.day()) < (birth_date.month(), birth_date.day()) {
            years -= 1;
        }
        u32::try_from(years).ok()
    }
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

    /// Builds a baseline McDavid-like `RosterPlayer`; individual fields are
    /// overridden per test via struct-update syntax.
    fn sample_roster_player() -> RosterPlayer {
        RosterPlayer {
            id: 8478402,
            headshot: "https://assets.nhle.com/mugs/nhl/default.png".to_string(),
            first_name: LocalizedString {
                default: "Connor".to_string(),
            },
            last_name: LocalizedString {
                default: "McDavid".to_string(),
            },
            sweater_number: 97,
            position: Some(Position::Center),
            shoots_catches: Some(Handedness::Left),
            height_in_inches: 73,
            weight_in_pounds: 193,
            height_in_centimeters: 185,
            weight_in_kilograms: 88,
            birth_date: "1997-01-13".to_string(),
            birth_city: LocalizedString {
                default: "Richmond Hill".to_string(),
            },
            birth_country: "CAN".to_string(),
            birth_state_province: Some(LocalizedString {
                default: "ON".to_string(),
            }),
        }
    }

    #[test]
    fn test_roster_player_full_name() {
        let player = sample_roster_player();
        assert_eq!(player.full_name(), "Connor McDavid");
    }

    #[test]
    fn test_roster_player_birth_place_all_parts() {
        let player = sample_roster_player();
        assert_eq!(player.birth_place(), "Richmond Hill, ON, CAN");
    }

    #[test]
    fn test_roster_player_birth_place_missing_state() {
        let player = RosterPlayer {
            birth_state_province: None,
            ..sample_roster_player()
        };
        assert_eq!(player.birth_place(), "Richmond Hill, CAN");
    }

    #[test]
    fn test_roster_player_birth_place_city_only() {
        let player = RosterPlayer {
            birth_state_province: None,
            birth_country: String::new(),
            ..sample_roster_player()
        };
        assert_eq!(player.birth_place(), "Richmond Hill");
    }

    /// An empty `LocalizedString` state/province (rather than `None`) is
    /// also treated as absent, not rendered as a bare `", "`.
    #[test]
    fn test_roster_player_birth_place_empty_state_string_treated_as_absent() {
        let player = RosterPlayer {
            birth_state_province: Some(LocalizedString::default()),
            ..sample_roster_player()
        };
        assert_eq!(player.birth_place(), "Richmond Hill, CAN");
    }

    #[test]
    fn test_roster_player_birth_place_none() {
        let player = RosterPlayer {
            birth_city: LocalizedString::default(),
            birth_state_province: None,
            birth_country: String::new(),
            ..sample_roster_player()
        };
        assert_eq!(player.birth_place(), "");
    }

    #[test]
    fn test_roster_player_height_feet_inches() {
        let player = sample_roster_player();
        assert_eq!(player.height_feet_inches(), "6'1\"");

        let short_player = RosterPlayer {
            height_in_inches: 72,
            ..sample_roster_player()
        };
        assert_eq!(short_player.height_feet_inches(), "6'0\"");

        let exact_foot = RosterPlayer {
            height_in_inches: 84,
            ..sample_roster_player()
        };
        assert_eq!(exact_foot.height_feet_inches(), "7'0\"");
    }

    #[test]
    fn test_roster_player_age_before_birthday_this_year() {
        let player = sample_roster_player(); // born 1997-01-13
        let on = NaiveDate::from_ymd_opt(2024, 1, 12).unwrap();
        assert_eq!(player.age(on), Some(26));
    }

    #[test]
    fn test_roster_player_age_on_birthday() {
        let player = sample_roster_player();
        let on = NaiveDate::from_ymd_opt(2024, 1, 13).unwrap();
        assert_eq!(player.age(on), Some(27));
    }

    #[test]
    fn test_roster_player_age_after_birthday_this_year() {
        let player = sample_roster_player();
        let on = NaiveDate::from_ymd_opt(2024, 1, 14).unwrap();
        assert_eq!(player.age(on), Some(27));
    }

    #[test]
    fn test_roster_player_age_unparseable_birth_date() {
        let player = RosterPlayer {
            birth_date: "not-a-date".to_string(),
            ..sample_roster_player()
        };
        let on = NaiveDate::from_ymd_opt(2024, 1, 14).unwrap();
        assert_eq!(player.age(on), None);
    }
}
