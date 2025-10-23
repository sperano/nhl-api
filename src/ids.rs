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

    /// Convert to a string for API calls
    pub fn to_string(&self) -> String {
        self.0.to_string()
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
