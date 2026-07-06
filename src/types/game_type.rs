use crate::types::enums::UnknownEnumValue;
use serde::de::{Error as DeError, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::str::FromStr;

/// Label used in [`UnknownEnumValue`] errors raised by this module.
const ENUM_NAME: &str = "game type";

/// NHL Game Type
///
/// Represents the different types of NHL games, including the modern regular
/// schedule as well as historical and special event game types (World Cup,
/// Olympics, All-Star Weekend variants, etc.) that appear on older schedule
/// and boxscore data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameType {
    /// Preseason games
    Preseason,
    /// Regular season games
    RegularSeason,
    /// Playoff games
    Playoffs,
    /// All-Star game
    AllStar,
    /// World Cup of Hockey game
    WorldCup,
    /// World Cup 2004 (the NHL API uses both code 6 and 7 for World Cup games)
    WorldCup2004,
    /// World Cup pre-tournament game
    WorldCupPreTournament,
    /// Olympic tournament game
    Olympics,
    /// YoungStars game (rookies/sophomores) during All-Star Weekend
    YoungStars,
    /// PWHL 3-on-3 showcase game during All-Star Weekend
    PwhlShowcase,
    /// Game lost due to a lockout
    LockoutLost,
    /// Canada Cup game
    CanadaCup,
    /// Exhibition game played overseas
    ExhibitionOverseas,
    /// Women's All-Star game
    WomensAllStar,
    /// 4 Nations Face-Off game
    FourNations,
}

impl GameType {
    /// Convert GameType to its integer representation
    pub fn to_int(self) -> i32 {
        match self {
            Self::Preseason => 1,
            Self::RegularSeason => 2,
            Self::Playoffs => 3,
            Self::AllStar => 4,
            Self::WorldCup => 6,
            Self::WorldCup2004 => 7,
            Self::WorldCupPreTournament => 8,
            Self::Olympics => 9,
            Self::YoungStars => 10,
            Self::PwhlShowcase => 12,
            Self::LockoutLost => 13,
            Self::CanadaCup => 14,
            Self::ExhibitionOverseas => 18,
            Self::WomensAllStar => 19,
            Self::FourNations => 20,
        }
    }

    /// Convert integer to GameType
    ///
    /// Returns None for unknown game type values
    pub fn from_int(value: i32) -> Option<Self> {
        match value {
            1 => Some(Self::Preseason),
            2 => Some(Self::RegularSeason),
            3 => Some(Self::Playoffs),
            4 => Some(Self::AllStar),
            6 => Some(Self::WorldCup),
            7 => Some(Self::WorldCup2004),
            8 => Some(Self::WorldCupPreTournament),
            9 => Some(Self::Olympics),
            10 => Some(Self::YoungStars),
            12 => Some(Self::PwhlShowcase),
            13 => Some(Self::LockoutLost),
            14 => Some(Self::CanadaCup),
            18 => Some(Self::ExhibitionOverseas),
            19 => Some(Self::WomensAllStar),
            20 => Some(Self::FourNations),
            _ => None,
        }
    }

    /// Returns the snake_case label for the GameType, suitable for use as a
    /// database enum value or a normalized identifier.
    ///
    /// These strings mirror the Go client's `GameType.Label()` byte-for-byte;
    /// downstream consumers persist them as database enum values, so they
    /// must not be changed without a coordinated migration.
    pub fn label(&self) -> &'static str {
        match self {
            Self::Preseason => "preseason",
            Self::RegularSeason => "regular_season",
            Self::Playoffs => "playoffs",
            Self::AllStar => "all_star",
            Self::WorldCup => "world_cup",
            Self::WorldCup2004 => "world_cup_2004",
            Self::WorldCupPreTournament => "world_cup_pre_tournament",
            Self::Olympics => "olympics",
            Self::YoungStars => "young_stars",
            Self::PwhlShowcase => "pwhl_showcase",
            Self::LockoutLost => "lockout_lost",
            Self::CanadaCup => "canada_cup",
            Self::ExhibitionOverseas => "exhibition_overseas",
            Self::WomensAllStar => "womens_all_star",
            Self::FourNations => "four_nations",
        }
    }
}

impl fmt::Display for GameType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Preseason => write!(f, "Preseason"),
            Self::RegularSeason => write!(f, "Regular Season"),
            Self::Playoffs => write!(f, "Playoffs"),
            Self::AllStar => write!(f, "All-Star"),
            Self::WorldCup => write!(f, "World Cup"),
            Self::WorldCup2004 => write!(f, "World Cup 2004"),
            Self::WorldCupPreTournament => write!(f, "World Cup Pre-Tournament"),
            Self::Olympics => write!(f, "Olympics"),
            Self::YoungStars => write!(f, "YoungStars"),
            Self::PwhlShowcase => write!(f, "PWHL Showcase"),
            Self::LockoutLost => write!(f, "Lockout Lost"),
            Self::CanadaCup => write!(f, "Canada Cup"),
            Self::ExhibitionOverseas => write!(f, "Exhibition Overseas"),
            Self::WomensAllStar => write!(f, "Women's All-Star"),
            Self::FourNations => write!(f, "4 Nations Face-Off"),
        }
    }
}

impl FromStr for GameType {
    type Err = UnknownEnumValue;

    /// Parses a numeric string (`"7"`), a display name (`"World Cup 2004"`,
    /// case/hyphenation variants such as `"WorldCup2004"`), or a snake_case
    /// [`label`](Self::label) (`"world_cup_2004"`) into a [`GameType`].
    ///
    /// Mirrors the Go client's `GameTypeFromString`.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" | "Preseason" | "preseason" => Ok(Self::Preseason),
            "2" | "Regular Season" | "RegularSeason" | "regular_season" => Ok(Self::RegularSeason),
            "3" | "Playoffs" | "playoffs" => Ok(Self::Playoffs),
            "4" | "All-Star" | "AllStar" | "all_star" => Ok(Self::AllStar),
            "6" | "World Cup" | "WorldCup" | "world_cup" => Ok(Self::WorldCup),
            "7" | "World Cup 2004" | "WorldCup2004" | "world_cup_2004" => Ok(Self::WorldCup2004),
            "8"
            | "World Cup Pre-Tournament"
            | "WorldCupPreTournament"
            | "world_cup_pre_tournament" => Ok(Self::WorldCupPreTournament),
            "9" | "Olympics" | "olympics" => Ok(Self::Olympics),
            "10" | "YoungStars" | "Young Stars" | "young_stars" => Ok(Self::YoungStars),
            "12" | "PWHL Showcase" | "PWHLShowcase" | "pwhl_showcase" => Ok(Self::PwhlShowcase),
            "13" | "Lockout Lost" | "LockoutLost" | "lockout_lost" => Ok(Self::LockoutLost),
            "14" | "Canada Cup" | "CanadaCup" | "canada_cup" => Ok(Self::CanadaCup),
            "18" | "Exhibition Overseas" | "ExhibitionOverseas" | "exhibition_overseas" => {
                Ok(Self::ExhibitionOverseas)
            }
            "19" | "Women's All-Star" | "WomensAllStar" | "womens_all_star" => {
                Ok(Self::WomensAllStar)
            }
            "20" | "4 Nations Face-Off" | "4NationsFaceOff" | "four_nations" => {
                Ok(Self::FourNations)
            }
            _ => Err(UnknownEnumValue {
                enum_name: ENUM_NAME,
                value: s.to_string(),
            }),
        }
    }
}

impl Serialize for GameType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.to_int())
    }
}

impl<'de> Deserialize<'de> for GameType {
    /// Accepts the primary integer wire form (`2`) as well as a string
    /// fallback (`"2"`, a display name, or a [`label`](Self::label)) — the
    /// NHL API is consistently integer-coded, but this mirrors the Go
    /// client's tolerant `UnmarshalJSON`, which tries int first and falls
    /// back to string parsing.
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GameTypeVisitor;

        impl Visitor<'_> for GameTypeVisitor {
            type Value = GameType;

            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "a game type as an integer or a string")
            }

            fn visit_i64<E>(self, value: i64) -> Result<GameType, E>
            where
                E: DeError,
            {
                GameType::from_int(value as i32).ok_or_else(|| {
                    E::custom(UnknownEnumValue {
                        enum_name: ENUM_NAME,
                        value: value.to_string(),
                    })
                })
            }

            fn visit_u64<E>(self, value: u64) -> Result<GameType, E>
            where
                E: DeError,
            {
                self.visit_i64(value as i64)
            }

            fn visit_str<E>(self, value: &str) -> Result<GameType, E>
            where
                E: DeError,
            {
                value.parse::<GameType>().map_err(E::custom)
            }
        }

        deserializer.deserialize_any(GameTypeVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// All 15 variants paired with their Go-defined integer code, `Display`
    /// string, and snake_case `label()` — used to parameterize the
    /// round-trip and label tests below instead of duplicating 15 near
    /// identical assertions per test.
    const ALL_VARIANTS: [(GameType, i32, &str, &str); 15] = [
        (GameType::Preseason, 1, "Preseason", "preseason"),
        (
            GameType::RegularSeason,
            2,
            "Regular Season",
            "regular_season",
        ),
        (GameType::Playoffs, 3, "Playoffs", "playoffs"),
        (GameType::AllStar, 4, "All-Star", "all_star"),
        (GameType::WorldCup, 6, "World Cup", "world_cup"),
        (
            GameType::WorldCup2004,
            7,
            "World Cup 2004",
            "world_cup_2004",
        ),
        (
            GameType::WorldCupPreTournament,
            8,
            "World Cup Pre-Tournament",
            "world_cup_pre_tournament",
        ),
        (GameType::Olympics, 9, "Olympics", "olympics"),
        (GameType::YoungStars, 10, "YoungStars", "young_stars"),
        (GameType::PwhlShowcase, 12, "PWHL Showcase", "pwhl_showcase"),
        (GameType::LockoutLost, 13, "Lockout Lost", "lockout_lost"),
        (GameType::CanadaCup, 14, "Canada Cup", "canada_cup"),
        (
            GameType::ExhibitionOverseas,
            18,
            "Exhibition Overseas",
            "exhibition_overseas",
        ),
        (
            GameType::WomensAllStar,
            19,
            "Women's All-Star",
            "womens_all_star",
        ),
        (
            GameType::FourNations,
            20,
            "4 Nations Face-Off",
            "four_nations",
        ),
    ];

    #[test]
    fn test_to_int_all_variants() {
        for (variant, code, _, _) in ALL_VARIANTS {
            assert_eq!(variant.to_int(), code, "{variant:?} to_int mismatch");
        }
    }

    #[test]
    fn test_from_int_all_variants() {
        for (variant, code, _, _) in ALL_VARIANTS {
            assert_eq!(
                GameType::from_int(code),
                Some(variant),
                "from_int({code}) mismatch"
            );
        }
    }

    #[test]
    fn test_from_int_unknown() {
        assert_eq!(GameType::from_int(5), None);
        assert_eq!(GameType::from_int(0), None);
        assert_eq!(GameType::from_int(11), None);
        assert_eq!(GameType::from_int(15), None);
        assert_eq!(GameType::from_int(100), None);
    }

    #[test]
    fn test_int_round_trip_all_variants() {
        for (variant, _, _, _) in ALL_VARIANTS {
            assert_eq!(GameType::from_int(variant.to_int()), Some(variant));
        }
    }

    #[test]
    fn test_display_all_variants() {
        for (variant, _, display, _) in ALL_VARIANTS {
            assert_eq!(
                format!("{variant}"),
                display,
                "{variant:?} Display mismatch"
            );
        }
    }

    #[test]
    fn test_label_all_variants() {
        for (variant, _, _, label) in ALL_VARIANTS {
            assert_eq!(variant.label(), label, "{variant:?} label mismatch");
        }
    }

    #[test]
    fn test_from_str_numeric() {
        for (variant, code, _, _) in ALL_VARIANTS {
            assert_eq!(
                code.to_string().parse::<GameType>(),
                Ok(variant),
                "numeric parse for {code} mismatch"
            );
        }
    }

    #[test]
    fn test_from_str_display_name() {
        for (variant, _, display, _) in ALL_VARIANTS {
            assert_eq!(
                display.parse::<GameType>(),
                Ok(variant),
                "display-name parse for {display:?} mismatch"
            );
        }
    }

    #[test]
    fn test_from_str_label() {
        for (variant, _, _, label) in ALL_VARIANTS {
            assert_eq!(
                label.parse::<GameType>(),
                Ok(variant),
                "label parse for {label:?} mismatch"
            );
        }
    }

    #[test]
    fn test_from_str_alternate_aliases() {
        // Aliases beyond the canonical display name / label, mirroring Go's
        // GameTypeFromString exactly (PascalCase variants and Go's chosen
        // hyphen-free forms).
        assert_eq!(
            "RegularSeason".parse::<GameType>(),
            Ok(GameType::RegularSeason)
        );
        assert_eq!("AllStar".parse::<GameType>(), Ok(GameType::AllStar));
        assert_eq!("WorldCup".parse::<GameType>(), Ok(GameType::WorldCup));
        assert_eq!(
            "WorldCup2004".parse::<GameType>(),
            Ok(GameType::WorldCup2004)
        );
        assert_eq!(
            "WorldCupPreTournament".parse::<GameType>(),
            Ok(GameType::WorldCupPreTournament)
        );
        assert_eq!("Young Stars".parse::<GameType>(), Ok(GameType::YoungStars));
        assert_eq!(
            "PWHLShowcase".parse::<GameType>(),
            Ok(GameType::PwhlShowcase)
        );
        assert_eq!("LockoutLost".parse::<GameType>(), Ok(GameType::LockoutLost));
        assert_eq!("CanadaCup".parse::<GameType>(), Ok(GameType::CanadaCup));
        assert_eq!(
            "ExhibitionOverseas".parse::<GameType>(),
            Ok(GameType::ExhibitionOverseas)
        );
        assert_eq!(
            "WomensAllStar".parse::<GameType>(),
            Ok(GameType::WomensAllStar)
        );
        assert_eq!(
            "4NationsFaceOff".parse::<GameType>(),
            Ok(GameType::FourNations)
        );
    }

    #[test]
    fn test_from_str_unknown() {
        let err = "bogus".parse::<GameType>().unwrap_err();
        assert_eq!(err.enum_name, ENUM_NAME);
        assert_eq!(err.value, "bogus");
    }

    #[test]
    fn test_serialize_all_variants() {
        for (variant, code, _, _) in ALL_VARIANTS {
            assert_eq!(
                serde_json::to_string(&variant).unwrap(),
                code.to_string(),
                "{variant:?} serialize mismatch"
            );
        }
    }

    #[test]
    fn test_deserialize_int_all_variants() {
        for (variant, code, _, _) in ALL_VARIANTS {
            assert_eq!(
                serde_json::from_str::<GameType>(&code.to_string()).unwrap(),
                variant,
                "deserialize int {code} mismatch"
            );
        }
    }

    #[test]
    fn test_deserialize_string_numeric() {
        for (variant, code, _, _) in ALL_VARIANTS {
            let json = format!("\"{code}\"");
            assert_eq!(
                serde_json::from_str::<GameType>(&json).unwrap(),
                variant,
                "deserialize numeric string {code} mismatch"
            );
        }
    }

    #[test]
    fn test_deserialize_string_label() {
        for (variant, _, _, label) in ALL_VARIANTS {
            let json = format!("\"{label}\"");
            assert_eq!(
                serde_json::from_str::<GameType>(&json).unwrap(),
                variant,
                "deserialize label {label:?} mismatch"
            );
        }
    }

    #[test]
    fn test_deserialize_string_display_name() {
        let json = "\"World Cup 2004\"";
        assert_eq!(
            serde_json::from_str::<GameType>(json).unwrap(),
            GameType::WorldCup2004
        );
    }

    #[test]
    fn test_deserialize_unknown_int() {
        let result = serde_json::from_str::<GameType>("5");
        assert!(result.is_err());
        let message = result.unwrap_err().to_string();
        assert!(
            message.contains("game type") && message.contains('5'),
            "message missing enum name or value: {message}"
        );
    }

    #[test]
    fn test_deserialize_unknown_string() {
        let result = serde_json::from_str::<GameType>("\"not-a-game-type\"");
        assert!(result.is_err());
        let message = result.unwrap_err().to_string();
        assert!(
            message.contains("game type") && message.contains("not-a-game-type"),
            "message missing enum name or value: {message}"
        );
    }

    #[test]
    fn test_roundtrip() {
        for (variant, _, _, _) in ALL_VARIANTS {
            let serialized = serde_json::to_string(&variant).unwrap();
            let deserialized: GameType = serde_json::from_str(&serialized).unwrap();
            assert_eq!(variant, deserialized);
        }
    }
}
