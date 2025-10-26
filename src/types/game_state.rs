use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// NHL game state representing the current status of a game
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum GameState {
    /// Future/scheduled game
    #[serde(rename = "FUT")]
    Future,

    /// Pre-game
    #[serde(rename = "PRE")]
    PreGame,

    /// Live/in progress
    #[serde(rename = "LIVE")]
    Live,

    /// Final/completed
    #[serde(rename = "FINAL")]
    Final,

    /// Off/completed (alternative to Final)
    #[serde(rename = "OFF")]
    Off,

    /// Postponed
    #[serde(rename = "PPD")]
    Postponed,

    /// Suspended
    #[serde(rename = "SUSP")]
    Suspended,

    /// Critical (close game, possibly final minutes)
    #[serde(rename = "CRIT")]
    Critical,
}

impl GameState {
    /// Returns true if the game has started (live or completed)
    pub fn has_started(&self) -> bool {
        matches!(self, GameState::Live | GameState::Critical | GameState::Final | GameState::Off)
    }

    /// Returns true if the game is completed
    pub fn is_final(&self) -> bool {
        matches!(self, GameState::Final | GameState::Off)
    }

    /// Returns true if the game is currently in progress
    pub fn is_live(&self) -> bool {
        matches!(self, GameState::Live | GameState::Critical)
    }

    /// Returns true if the game is scheduled but not started
    pub fn is_scheduled(&self) -> bool {
        matches!(self, GameState::Future | GameState::PreGame)
    }
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            GameState::Future => "FUT",
            GameState::PreGame => "PRE",
            GameState::Live => "LIVE",
            GameState::Final => "FINAL",
            GameState::Off => "OFF",
            GameState::Postponed => "PPD",
            GameState::Suspended => "SUSP",
            GameState::Critical => "CRIT",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for GameState {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "FUT" => Ok(GameState::Future),
            "PRE" => Ok(GameState::PreGame),
            "LIVE" => Ok(GameState::Live),
            "FINAL" => Ok(GameState::Final),
            "OFF" => Ok(GameState::Off),
            "PPD" => Ok(GameState::Postponed),
            "SUSP" => Ok(GameState::Suspended),
            "CRIT" => Ok(GameState::Critical),
            _ => Err(format!("Unknown game state: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_state_display() {
        assert_eq!(GameState::Future.to_string(), "FUT");
        assert_eq!(GameState::Live.to_string(), "LIVE");
        assert_eq!(GameState::Final.to_string(), "FINAL");
    }

    #[test]
    fn test_game_state_from_str() {
        assert_eq!("FUT".parse::<GameState>().unwrap(), GameState::Future);
        assert_eq!("LIVE".parse::<GameState>().unwrap(), GameState::Live);
        assert_eq!("FINAL".parse::<GameState>().unwrap(), GameState::Final);
    }

    #[test]
    fn test_has_started() {
        assert!(!GameState::Future.has_started());
        assert!(!GameState::PreGame.has_started());
        assert!(GameState::Live.has_started());
        assert!(GameState::Final.has_started());
        assert!(GameState::Off.has_started());
    }

    #[test]
    fn test_is_final() {
        assert!(!GameState::Live.is_final());
        assert!(GameState::Final.is_final());
        assert!(GameState::Off.is_final());
    }

    #[test]
    fn test_is_scheduled() {
        assert!(GameState::Future.is_scheduled());
        assert!(GameState::PreGame.is_scheduled());
        assert!(!GameState::Live.is_scheduled());
        assert!(!GameState::Final.is_scheduled());
    }

    #[test]
    fn test_serde_serialization() {
        let state = GameState::Live;
        let json = serde_json::to_string(&state).unwrap();
        assert_eq!(json, r#""LIVE""#);
    }

    #[test]
    fn test_serde_deserialization() {
        let json = r#""LIVE""#;
        let state: GameState = serde_json::from_str(json).unwrap();
        assert_eq!(state, GameState::Live);
    }
}
