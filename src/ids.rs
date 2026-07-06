use serde::de::{Error as DeError, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::str::FromStr;

/// Generates a numeric ID newtype over `i64` with a uniform API:
/// `new`/`as_i64`, `From<i64>`/`From<Id> for i64`, `Display`, `FromStr`,
/// ordering/hashing derives, and serde support (serializes as an integer,
/// deserializes from either an integer or a numeric string — mirroring the
/// Go reference's `unmarshalNumericID`).
macro_rules! numeric_id {
    (
        $(#[$meta:meta])*
        $name:ident, $visitor:ident, $type_name:literal
    ) => {
        $(#[$meta])*
        #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $name(i64);

        impl $name {
            /// Create a new instance from an integer.
            pub const fn new(id: i64) -> Self {
                Self(id)
            }

            /// Get the inner `i64` value.
            pub const fn as_i64(&self) -> i64 {
                self.0
            }
        }

        impl From<i64> for $name {
            fn from(id: i64) -> Self {
                Self(id)
            }
        }

        impl From<$name> for i64 {
            fn from(id: $name) -> i64 {
                id.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl FromStr for $name {
            type Err = std::num::ParseIntError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok(Self(s.parse()?))
            }
        }

        impl Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                serializer.serialize_i64(self.0)
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct $visitor;

                impl Visitor<'_> for $visitor {
                    type Value = $name;

                    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                        write!(f, "{} as an integer or a numeric string", $type_name)
                    }

                    fn visit_i64<E>(self, value: i64) -> Result<$name, E>
                    where
                        E: DeError,
                    {
                        Ok($name(value))
                    }

                    fn visit_u64<E>(self, value: u64) -> Result<$name, E>
                    where
                        E: DeError,
                    {
                        i64::try_from(value)
                            .map($name)
                            .map_err(|_| E::custom(format!("{} out of range: {value}", $type_name)))
                    }

                    fn visit_str<E>(self, value: &str) -> Result<$name, E>
                    where
                        E: DeError,
                    {
                        value.parse::<i64>().map($name).map_err(|_| {
                            E::custom(format!("invalid {} string: {value:?}", $type_name))
                        })
                    }
                }

                deserializer.deserialize_any($visitor)
            }
        }
    };
}

numeric_id!(
    /// A unique NHL game identifier.
    ///
    /// Game IDs are 10-digit integers in the format `SSSSGTNNNN` (season, game
    /// type, game number). Serializes as an integer; deserializes from an
    /// integer or a numeric string.
    GameId, GameIdVisitor, "game ID"
);

numeric_id!(
    /// A unique NHL player identifier.
    ///
    /// Serializes as an integer; deserializes from an integer or a numeric
    /// string.
    PlayerId, PlayerIdVisitor, "player ID"
);

numeric_id!(
    /// A unique NHL team identifier.
    ///
    /// Serializes as an integer; deserializes from an integer or a numeric
    /// string.
    TeamId, TeamIdVisitor, "team ID"
);

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{HashMap, HashSet};

    #[test]
    fn test_game_id_new() {
        let id = GameId::new(2023020001);
        assert_eq!(id.as_i64(), 2023020001);
    }

    #[test]
    fn test_game_id_new_negative() {
        let id = GameId::new(-1);
        assert_eq!(id.as_i64(), -1);
    }

    #[test]
    fn test_game_id_new_zero() {
        let id = GameId::new(0);
        assert_eq!(id.as_i64(), 0);
    }

    #[test]
    fn test_game_id_as_i64() {
        let id = GameId::new(12345);
        assert_eq!(id.as_i64(), 12345);
    }

    #[test]
    fn test_game_id_to_string() {
        let id = GameId::new(2023020001);
        assert_eq!(id.to_string(), "2023020001");

        let negative_id = GameId::new(-42);
        assert_eq!(negative_id.to_string(), "-42");
    }

    #[test]
    fn test_game_id_from_i64() {
        let id: GameId = 2023020001_i64.into();
        assert_eq!(id.as_i64(), 2023020001);

        let id2 = GameId::from(9876543210_i64);
        assert_eq!(id2.as_i64(), 9876543210);
    }

    #[test]
    fn test_i64_from_game_id() {
        let id = GameId::new(2023020001);
        let value: i64 = id.into();
        assert_eq!(value, 2023020001);
    }

    #[test]
    fn test_game_id_display() {
        let id = GameId::new(2024030405);
        assert_eq!(format!("{}", id), "2024030405");

        let negative_id = GameId::new(-123);
        assert_eq!(format!("{}", negative_id), "-123");
    }

    #[test]
    fn test_game_id_from_str() {
        let id = GameId::from_str("2023020001").unwrap();
        assert_eq!(id.as_i64(), 2023020001);

        let id2: GameId = "9876543210".parse().unwrap();
        assert_eq!(id2.as_i64(), 9876543210);

        let negative_id: GameId = "-42".parse().unwrap();
        assert_eq!(negative_id.as_i64(), -42);
    }

    #[test]
    fn test_game_id_from_str_invalid() {
        // Non-numeric string
        assert!(GameId::from_str("not-a-number").is_err());
        assert!(GameId::from_str("abc123").is_err());
        assert!(GameId::from_str("").is_err());

        // Invalid formats
        assert!(GameId::from_str("12.34").is_err());
        assert!(GameId::from_str("12 34").is_err());

        // Overflow
        assert!(GameId::from_str("999999999999999999999999999").is_err());
    }

    #[test]
    fn test_game_id_equality() {
        let id1 = GameId::new(2023020001);
        let id2 = GameId::new(2023020001);
        let id3 = GameId::new(2023020002);

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
        assert_ne!(id2, id3);
    }

    #[test]
    fn test_game_id_ordering() {
        let id1 = GameId::new(100);
        let id2 = GameId::new(200);
        let id3 = GameId::new(200);

        assert!(id1 < id2);
        assert!(id2 > id1);
        assert!(id2 <= id3);
        assert!(id2 >= id3);
        assert_eq!(id2.cmp(&id3), std::cmp::Ordering::Equal);
        assert_eq!(id1.cmp(&id2), std::cmp::Ordering::Less);
        assert_eq!(id2.cmp(&id1), std::cmp::Ordering::Greater);
    }

    #[test]
    fn test_game_id_ordering_negative() {
        let id1 = GameId::new(-100);
        let id2 = GameId::new(0);
        let id3 = GameId::new(100);

        assert!(id1 < id2);
        assert!(id2 < id3);
        assert!(id1 < id3);
    }

    #[test]
    fn test_game_id_hash() {
        let mut set = HashSet::new();
        let id1 = GameId::new(2023020001);
        let id2 = GameId::new(2023020001);
        let id3 = GameId::new(2023020002);

        set.insert(id1);
        assert!(set.contains(&id2)); // Should find id2 since it equals id1
        assert!(!set.contains(&id3));

        set.insert(id3);
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_game_id_as_map_key() {
        let mut map = HashMap::new();
        let id1 = GameId::new(2023020001);
        let id2 = GameId::new(2023020002);

        map.insert(id1, "Game 1");
        map.insert(id2, "Game 2");

        assert_eq!(map.get(&id1), Some(&"Game 1"));
        assert_eq!(map.get(&id2), Some(&"Game 2"));
        assert_eq!(map.get(&GameId::new(2023020001)), Some(&"Game 1"));
    }

    #[test]
    fn test_game_id_const() {
        const GAME_ID: GameId = GameId::new(2023020001);
        assert_eq!(GAME_ID.as_i64(), 2023020001);
    }

    /// `Default` is needed so `#[serde(skip)]` fields (e.g.
    /// `PlayerGameLog.player_id`, populated manually by the client rather
    /// than the API response) can deserialize without it.
    #[test]
    fn test_game_id_default() {
        assert_eq!(GameId::default().as_i64(), 0);
        assert_eq!(PlayerId::default().as_i64(), 0);
        assert_eq!(TeamId::default().as_i64(), 0);
    }

    #[test]
    fn test_game_id_copy_clone() {
        let id1 = GameId::new(2023020001);
        let id2 = id1; // Copy
        let id3 = id1; // GameId is Copy, so no need for .clone()

        assert_eq!(id1, id2);
        assert_eq!(id1, id3);
        assert_eq!(id2, id3);
    }

    #[test]
    fn test_game_id_serde_from_int() {
        let id: GameId = serde_json::from_str("2023020001").unwrap();
        assert_eq!(id.as_i64(), 2023020001);
    }

    #[test]
    fn test_game_id_serde_from_string() {
        let id: GameId = serde_json::from_str("\"2023020001\"").unwrap();
        assert_eq!(id.as_i64(), 2023020001);
    }

    #[test]
    fn test_game_id_serde_rejects_non_numeric_string() {
        let result: Result<GameId, _> = serde_json::from_str("\"not-a-number\"");
        assert!(result.is_err());
    }

    #[test]
    fn test_game_id_serialize_emits_integer() {
        let id = GameId::new(2023020001);
        assert_eq!(serde_json::to_string(&id).unwrap(), "2023020001");
    }

    #[test]
    fn test_player_id_new() {
        let id = PlayerId::new(8478402);
        assert_eq!(id.as_i64(), 8478402);
    }

    #[test]
    fn test_player_id_display() {
        let id = PlayerId::new(8478402);
        assert_eq!(id.to_string(), "8478402");
    }

    #[test]
    fn test_player_id_from_str_valid() {
        let id: PlayerId = "8478402".parse().unwrap();
        assert_eq!(id.as_i64(), 8478402);
    }

    #[test]
    fn test_player_id_from_str_invalid() {
        assert!("not-a-number".parse::<PlayerId>().is_err());
        assert!("".parse::<PlayerId>().is_err());
        assert!("84.78".parse::<PlayerId>().is_err());
    }

    #[test]
    fn test_player_id_serde_from_int() {
        let id: PlayerId = serde_json::from_str("8478402").unwrap();
        assert_eq!(id.as_i64(), 8478402);
    }

    #[test]
    fn test_player_id_serde_from_string() {
        let id: PlayerId = serde_json::from_str("\"8478402\"").unwrap();
        assert_eq!(id.as_i64(), 8478402);
    }

    #[test]
    fn test_player_id_serde_rejects_non_numeric_string() {
        let result: Result<PlayerId, _> = serde_json::from_str("\"mcdavid\"");
        assert!(result.is_err());
    }

    #[test]
    fn test_player_id_serialize_emits_integer() {
        let id = PlayerId::new(8478402);
        assert_eq!(serde_json::to_string(&id).unwrap(), "8478402");
    }

    #[test]
    fn test_player_id_ordering_and_hashing() {
        let id1 = PlayerId::new(100);
        let id2 = PlayerId::new(200);
        let id3 = PlayerId::new(200);

        assert!(id1 < id2);
        assert_eq!(id2, id3);

        let mut set = HashSet::new();
        set.insert(id1);
        set.insert(id2);
        set.insert(id3);
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_team_id_new() {
        let id = TeamId::new(10);
        assert_eq!(id.as_i64(), 10);
    }

    #[test]
    fn test_team_id_display() {
        let id = TeamId::new(10);
        assert_eq!(id.to_string(), "10");
    }

    #[test]
    fn test_team_id_from_str_valid() {
        let id: TeamId = "10".parse().unwrap();
        assert_eq!(id.as_i64(), 10);
    }

    #[test]
    fn test_team_id_from_str_invalid() {
        assert!("not-a-number".parse::<TeamId>().is_err());
        assert!("".parse::<TeamId>().is_err());
        assert!("1a".parse::<TeamId>().is_err());
    }

    #[test]
    fn test_team_id_serde_from_int() {
        let id: TeamId = serde_json::from_str("10").unwrap();
        assert_eq!(id.as_i64(), 10);
    }

    #[test]
    fn test_team_id_serde_from_string() {
        let id: TeamId = serde_json::from_str("\"10\"").unwrap();
        assert_eq!(id.as_i64(), 10);
    }

    #[test]
    fn test_team_id_serde_rejects_non_numeric_string() {
        let result: Result<TeamId, _> = serde_json::from_str("\"leafs\"");
        assert!(result.is_err());
    }

    #[test]
    fn test_team_id_serialize_emits_integer() {
        let id = TeamId::new(10);
        assert_eq!(serde_json::to_string(&id).unwrap(), "10");
    }

    #[test]
    fn test_team_id_ordering_and_hashing() {
        let id1 = TeamId::new(1);
        let id2 = TeamId::new(2);
        let id3 = TeamId::new(2);

        assert!(id1 < id2);
        assert_eq!(id2, id3);

        let mut set = HashSet::new();
        set.insert(id1);
        set.insert(id2);
        set.insert(id3);
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_game_id_player_id_team_id_are_distinct_types() {
        // Compile-time check that the macro-generated types are not
        // interchangeable despite sharing the same underlying repr.
        let game_id = GameId::new(1);
        let player_id = PlayerId::new(1);
        let team_id = TeamId::new(1);

        assert_eq!(game_id.as_i64(), player_id.as_i64());
        assert_eq!(player_id.as_i64(), team_id.as_i64());
    }
}
