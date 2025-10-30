use std::fmt;
use std::str::FromStr;

/// A unique NHL game identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GameId(i64);

impl GameId {
    /// Create a new GameId from an integer
    pub const fn new(id: i64) -> Self {
        Self(id)
    }

    /// Get the inner value
    pub const fn as_i64(&self) -> i64 {
        self.0
    }
}

impl From<i64> for GameId {
    fn from(id: i64) -> Self {
        Self(id)
    }
}

impl From<GameId> for i64 {
    fn from(id: GameId) -> i64 {
        id.0
    }
}

impl fmt::Display for GameId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for GameId {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}

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

    #[test]
    fn test_game_id_copy_clone() {
        let id1 = GameId::new(2023020001);
        let id2 = id1; // Copy
        let id3 = id1; // GameId is Copy, so no need for .clone()

        assert_eq!(id1, id2);
        assert_eq!(id1, id3);
        assert_eq!(id2, id3);
    }
}
