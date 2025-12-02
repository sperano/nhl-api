use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

/// NHL Game Type
///
/// Represents the different types of NHL games.
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
}

impl GameType {
    /// Convert GameType to its integer representation
    pub fn to_int(self) -> i32 {
        match self {
            Self::Preseason => 1,
            Self::RegularSeason => 2,
            Self::Playoffs => 3,
            Self::AllStar => 4,
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
            _ => None,
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
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        Self::from_int(value).ok_or_else(|| {
            serde::de::Error::custom(format!("Unknown game type: {}", value))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_int() {
        assert_eq!(GameType::Preseason.to_int(), 1);
        assert_eq!(GameType::RegularSeason.to_int(), 2);
        assert_eq!(GameType::Playoffs.to_int(), 3);
        assert_eq!(GameType::AllStar.to_int(), 4);
    }

    #[test]
    fn test_from_int() {
        assert_eq!(GameType::from_int(1), Some(GameType::Preseason));
        assert_eq!(GameType::from_int(2), Some(GameType::RegularSeason));
        assert_eq!(GameType::from_int(3), Some(GameType::Playoffs));
        assert_eq!(GameType::from_int(4), Some(GameType::AllStar));
        assert_eq!(GameType::from_int(5), None);
        assert_eq!(GameType::from_int(0), None);
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", GameType::Preseason), "Preseason");
        assert_eq!(format!("{}", GameType::RegularSeason), "Regular Season");
        assert_eq!(format!("{}", GameType::Playoffs), "Playoffs");
        assert_eq!(format!("{}", GameType::AllStar), "All-Star");
    }

    #[test]
    fn test_serialize() {
        assert_eq!(
            serde_json::to_string(&GameType::Preseason).unwrap(),
            "1"
        );
        assert_eq!(
            serde_json::to_string(&GameType::RegularSeason).unwrap(),
            "2"
        );
        assert_eq!(
            serde_json::to_string(&GameType::Playoffs).unwrap(),
            "3"
        );
        assert_eq!(
            serde_json::to_string(&GameType::AllStar).unwrap(),
            "4"
        );
    }

    #[test]
    fn test_deserialize() {
        assert_eq!(
            serde_json::from_str::<GameType>("1").unwrap(),
            GameType::Preseason
        );
        assert_eq!(
            serde_json::from_str::<GameType>("2").unwrap(),
            GameType::RegularSeason
        );
        assert_eq!(
            serde_json::from_str::<GameType>("3").unwrap(),
            GameType::Playoffs
        );
        assert_eq!(
            serde_json::from_str::<GameType>("4").unwrap(),
            GameType::AllStar
        );
    }

    #[test]
    fn test_deserialize_unknown() {
        let result = serde_json::from_str::<GameType>("5");
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Unknown game type: 5"));
    }

    #[test]
    fn test_roundtrip() {
        let original = GameType::RegularSeason;
        let serialized = serde_json::to_string(&original).unwrap();
        let deserialized: GameType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(original, deserialized);
    }
}
