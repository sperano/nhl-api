//! Player-related enums for NHL API types
//!
//! This module contains enums that describe player attributes.

use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
use thiserror::Error;

// =============================================================================
// Position
// =============================================================================

/// Error type for parsing Position from string
#[derive(Error, Debug, PartialEq)]
#[error("Unknown position: {0}")]
pub struct ParsePositionError(pub String);

/// NHL player position
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Position {
    /// Center
    #[serde(rename = "C")]
    Center,

    /// Left Wing
    #[serde(alias = "L", rename = "LW")]
    LeftWing,

    /// Right Wing
    #[serde(alias = "R", rename = "RW")]
    RightWing,

    /// Defense
    #[serde(rename = "D")]
    Defense,

    /// Goalie
    #[serde(rename = "G")]
    Goalie,
}

impl Position {
    /// Returns the short code for this position
    pub const fn code(&self) -> &'static str {
        match self {
            Position::Center => "C",
            Position::LeftWing => "LW",
            Position::RightWing => "RW",
            Position::Defense => "D",
            Position::Goalie => "G",
        }
    }

    /// Returns the full name for this position
    pub const fn name(&self) -> &'static str {
        match self {
            Position::Center => "Center",
            Position::LeftWing => "Left Wing",
            Position::RightWing => "Right Wing",
            Position::Defense => "Defense",
            Position::Goalie => "Goalie",
        }
    }

    /// Returns true if this is a forward position
    pub const fn is_forward(&self) -> bool {
        matches!(
            self,
            Position::Center | Position::LeftWing | Position::RightWing
        )
    }

    /// Returns true if this is a skater position (not goalie)
    pub const fn is_skater(&self) -> bool {
        !matches!(self, Position::Goalie)
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.code())
    }
}

impl FromStr for Position {
    type Err = ParsePositionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "C" => Ok(Position::Center),
            "L" | "LW" => Ok(Position::LeftWing),
            "R" | "RW" => Ok(Position::RightWing),
            "D" => Ok(Position::Defense),
            "G" => Ok(Position::Goalie),
            _ => Err(ParsePositionError(s.to_string())),
        }
    }
}

// =============================================================================
// Handedness
// =============================================================================

/// Error type for parsing Handedness from string
#[derive(Error, Debug, PartialEq)]
#[error("Unknown handedness: {0}")]
pub struct ParseHandednessError(pub String);

/// NHL player handedness (shoots/catches)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Handedness {
    /// Left-handed
    #[serde(rename = "L")]
    Left,

    /// Right-handed
    #[serde(rename = "R")]
    Right,
}

impl Handedness {
    /// Returns the short code for this handedness
    pub const fn code(&self) -> &'static str {
        match self {
            Handedness::Left => "L",
            Handedness::Right => "R",
        }
    }

    /// Returns the full name for this handedness
    pub const fn name(&self) -> &'static str {
        match self {
            Handedness::Left => "Left",
            Handedness::Right => "Right",
        }
    }
}

impl fmt::Display for Handedness {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.code())
    }
}

impl FromStr for Handedness {
    type Err = ParseHandednessError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Handedness::Left),
            "R" => Ok(Handedness::Right),
            _ => Err(ParseHandednessError(s.to_string())),
        }
    }
}

// =============================================================================
// GoalieDecision
// =============================================================================

/// Error type for parsing GoalieDecision from string
#[derive(Error, Debug, PartialEq)]
#[error("Unknown goalie decision: {0}")]
pub struct ParseGoalieDecisionError(pub String);

/// Goalie game decision (win/loss/OT loss)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GoalieDecision {
    /// Win
    #[serde(rename = "W")]
    Win,

    /// Loss
    #[serde(rename = "L")]
    Loss,

    /// Overtime loss
    #[serde(rename = "O")]
    OvertimeLoss,
}

impl GoalieDecision {
    pub const fn code(&self) -> &'static str {
        match self {
            GoalieDecision::Win => "W",
            GoalieDecision::Loss => "L",
            GoalieDecision::OvertimeLoss => "O",
        }
    }

    pub const fn name(&self) -> &'static str {
        match self {
            GoalieDecision::Win => "Win",
            GoalieDecision::Loss => "Loss",
            GoalieDecision::OvertimeLoss => "Overtime Loss",
        }
    }
}

impl fmt::Display for GoalieDecision {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.code())
    }
}

impl FromStr for GoalieDecision {
    type Err = ParseGoalieDecisionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "W" => Ok(GoalieDecision::Win),
            "L" => Ok(GoalieDecision::Loss),
            "O" => Ok(GoalieDecision::OvertimeLoss),
            _ => Err(ParseGoalieDecisionError(s.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod position_tests {
        use super::*;

        #[test]
        fn test_position_code() {
            assert_eq!(Position::Center.code(), "C");
            assert_eq!(Position::LeftWing.code(), "LW");
            assert_eq!(Position::RightWing.code(), "RW");
            assert_eq!(Position::Defense.code(), "D");
            assert_eq!(Position::Goalie.code(), "G");
        }

        #[test]
        fn test_position_name() {
            assert_eq!(Position::Center.name(), "Center");
            assert_eq!(Position::LeftWing.name(), "Left Wing");
            assert_eq!(Position::RightWing.name(), "Right Wing");
            assert_eq!(Position::Defense.name(), "Defense");
            assert_eq!(Position::Goalie.name(), "Goalie");
        }

        #[test]
        fn test_position_is_forward() {
            assert!(Position::Center.is_forward());
            assert!(Position::LeftWing.is_forward());
            assert!(Position::RightWing.is_forward());
            assert!(!Position::Defense.is_forward());
            assert!(!Position::Goalie.is_forward());
        }

        #[test]
        fn test_position_is_skater() {
            assert!(Position::Center.is_skater());
            assert!(Position::LeftWing.is_skater());
            assert!(Position::RightWing.is_skater());
            assert!(Position::Defense.is_skater());
            assert!(!Position::Goalie.is_skater());
        }

        #[test]
        fn test_position_display() {
            assert_eq!(Position::Center.to_string(), "C");
            assert_eq!(Position::LeftWing.to_string(), "LW");
            assert_eq!(Position::RightWing.to_string(), "RW");
            assert_eq!(Position::Defense.to_string(), "D");
            assert_eq!(Position::Goalie.to_string(), "G");
        }

        #[test]
        fn test_position_from_str() {
            assert_eq!("C".parse::<Position>().unwrap(), Position::Center);
            assert_eq!("LW".parse::<Position>().unwrap(), Position::LeftWing);
            assert_eq!("L".parse::<Position>().unwrap(), Position::LeftWing);
            assert_eq!("RW".parse::<Position>().unwrap(), Position::RightWing);
            assert_eq!("R".parse::<Position>().unwrap(), Position::RightWing);
            assert_eq!("D".parse::<Position>().unwrap(), Position::Defense);
            assert_eq!("G".parse::<Position>().unwrap(), Position::Goalie);
        }

        #[test]
        fn test_position_from_str_invalid() {
            let result = "X".parse::<Position>();
            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), ParsePositionError("X".to_string()));
        }

        #[test]
        fn test_position_serialize() {
            assert_eq!(serde_json::to_string(&Position::Center).unwrap(), r#""C""#);
            assert_eq!(
                serde_json::to_string(&Position::LeftWing).unwrap(),
                r#""LW""#
            );
            assert_eq!(
                serde_json::to_string(&Position::RightWing).unwrap(),
                r#""RW""#
            );
            assert_eq!(serde_json::to_string(&Position::Defense).unwrap(), r#""D""#);
            assert_eq!(serde_json::to_string(&Position::Goalie).unwrap(), r#""G""#);
        }

        #[test]
        fn test_position_deserialize() {
            assert_eq!(
                serde_json::from_str::<Position>(r#""C""#).unwrap(),
                Position::Center
            );
            assert_eq!(
                serde_json::from_str::<Position>(r#""LW""#).unwrap(),
                Position::LeftWing
            );
            assert_eq!(
                serde_json::from_str::<Position>(r#""RW""#).unwrap(),
                Position::RightWing
            );
            assert_eq!(
                serde_json::from_str::<Position>(r#""D""#).unwrap(),
                Position::Defense
            );
            assert_eq!(
                serde_json::from_str::<Position>(r#""G""#).unwrap(),
                Position::Goalie
            );
        }

        #[test]
        fn test_position_deserialize_alias() {
            // "L" and "R" are aliases used by some API endpoints
            assert_eq!(
                serde_json::from_str::<Position>(r#""L""#).unwrap(),
                Position::LeftWing
            );
            assert_eq!(
                serde_json::from_str::<Position>(r#""R""#).unwrap(),
                Position::RightWing
            );
        }

        #[test]
        fn test_position_roundtrip() {
            for pos in [
                Position::Center,
                Position::LeftWing,
                Position::RightWing,
                Position::Defense,
                Position::Goalie,
            ] {
                let serialized = serde_json::to_string(&pos).unwrap();
                let deserialized: Position = serde_json::from_str(&serialized).unwrap();
                assert_eq!(pos, deserialized);
            }
        }

        #[test]
        fn test_position_hash() {
            use std::collections::HashSet;
            let mut set = HashSet::new();
            set.insert(Position::Center);
            set.insert(Position::LeftWing);
            set.insert(Position::Defense);

            assert!(set.contains(&Position::Center));
            assert!(set.contains(&Position::LeftWing));
            assert!(set.contains(&Position::Defense));
            assert!(!set.contains(&Position::RightWing));
            assert!(!set.contains(&Position::Goalie));
        }
    }

    mod handedness_tests {
        use super::*;

        #[test]
        fn test_handedness_code() {
            assert_eq!(Handedness::Left.code(), "L");
            assert_eq!(Handedness::Right.code(), "R");
        }

        #[test]
        fn test_handedness_name() {
            assert_eq!(Handedness::Left.name(), "Left");
            assert_eq!(Handedness::Right.name(), "Right");
        }

        #[test]
        fn test_handedness_display() {
            assert_eq!(Handedness::Left.to_string(), "L");
            assert_eq!(Handedness::Right.to_string(), "R");
        }

        #[test]
        fn test_handedness_from_str() {
            assert_eq!("L".parse::<Handedness>().unwrap(), Handedness::Left);
            assert_eq!("R".parse::<Handedness>().unwrap(), Handedness::Right);
        }

        #[test]
        fn test_handedness_from_str_invalid() {
            let result = "X".parse::<Handedness>();
            assert!(result.is_err());
        }

        #[test]
        fn test_handedness_serialize() {
            assert_eq!(serde_json::to_string(&Handedness::Left).unwrap(), r#""L""#);
            assert_eq!(serde_json::to_string(&Handedness::Right).unwrap(), r#""R""#);
        }

        #[test]
        fn test_handedness_deserialize() {
            assert_eq!(
                serde_json::from_str::<Handedness>(r#""L""#).unwrap(),
                Handedness::Left
            );
            assert_eq!(
                serde_json::from_str::<Handedness>(r#""R""#).unwrap(),
                Handedness::Right
            );
        }

        #[test]
        fn test_handedness_roundtrip() {
            for h in [Handedness::Left, Handedness::Right] {
                let serialized = serde_json::to_string(&h).unwrap();
                let deserialized: Handedness = serde_json::from_str(&serialized).unwrap();
                assert_eq!(h, deserialized);
            }
        }

        #[test]
        fn test_handedness_hash() {
            use std::collections::HashSet;
            let mut set = HashSet::new();
            set.insert(Handedness::Left);
            set.insert(Handedness::Right);
            assert_eq!(set.len(), 2);
        }
    }

    mod goalie_decision_tests {
        use super::*;

        #[test]
        fn test_goalie_decision_code() {
            assert_eq!(GoalieDecision::Win.code(), "W");
            assert_eq!(GoalieDecision::Loss.code(), "L");
            assert_eq!(GoalieDecision::OvertimeLoss.code(), "O");
        }

        #[test]
        fn test_goalie_decision_name() {
            assert_eq!(GoalieDecision::Win.name(), "Win");
            assert_eq!(GoalieDecision::Loss.name(), "Loss");
            assert_eq!(GoalieDecision::OvertimeLoss.name(), "Overtime Loss");
        }

        #[test]
        fn test_goalie_decision_serialize() {
            assert_eq!(
                serde_json::to_string(&GoalieDecision::Win).unwrap(),
                r#""W""#
            );
            assert_eq!(
                serde_json::to_string(&GoalieDecision::Loss).unwrap(),
                r#""L""#
            );
            assert_eq!(
                serde_json::to_string(&GoalieDecision::OvertimeLoss).unwrap(),
                r#""O""#
            );
        }

        #[test]
        fn test_goalie_decision_deserialize() {
            assert_eq!(
                serde_json::from_str::<GoalieDecision>(r#""W""#).unwrap(),
                GoalieDecision::Win
            );
            assert_eq!(
                serde_json::from_str::<GoalieDecision>(r#""L""#).unwrap(),
                GoalieDecision::Loss
            );
            assert_eq!(
                serde_json::from_str::<GoalieDecision>(r#""O""#).unwrap(),
                GoalieDecision::OvertimeLoss
            );
        }

        #[test]
        fn test_goalie_decision_from_str() {
            assert_eq!("W".parse::<GoalieDecision>().unwrap(), GoalieDecision::Win);
            assert_eq!("L".parse::<GoalieDecision>().unwrap(), GoalieDecision::Loss);
            assert_eq!(
                "O".parse::<GoalieDecision>().unwrap(),
                GoalieDecision::OvertimeLoss
            );
        }

        #[test]
        fn test_goalie_decision_from_str_invalid() {
            let result = "X".parse::<GoalieDecision>();
            assert!(result.is_err());
            assert_eq!(
                result.unwrap_err(),
                ParseGoalieDecisionError("X".to_string())
            );
        }

        #[test]
        fn test_goalie_decision_display() {
            assert_eq!(GoalieDecision::Win.to_string(), "W");
            assert_eq!(GoalieDecision::Loss.to_string(), "L");
            assert_eq!(GoalieDecision::OvertimeLoss.to_string(), "O");
        }

        #[test]
        fn test_goalie_decision_roundtrip() {
            for gd in [
                GoalieDecision::Win,
                GoalieDecision::Loss,
                GoalieDecision::OvertimeLoss,
            ] {
                let serialized = serde_json::to_string(&gd).unwrap();
                let deserialized: GoalieDecision = serde_json::from_str(&serialized).unwrap();
                assert_eq!(gd, deserialized);
            }
        }

        #[test]
        fn test_goalie_decision_hash() {
            use std::collections::HashSet;
            let mut set = HashSet::new();
            set.insert(GoalieDecision::Win);
            set.insert(GoalieDecision::Loss);
            set.insert(GoalieDecision::OvertimeLoss);
            assert_eq!(set.len(), 3);
        }
    }

    mod error_display_tests {
        use super::*;

        #[test]
        fn test_parse_position_error_display() {
            let err = ParsePositionError("X".to_string());
            assert_eq!(format!("{}", err), "Unknown position: X");
        }

        #[test]
        fn test_parse_handedness_error_display() {
            let err = ParseHandednessError("X".to_string());
            assert_eq!(format!("{}", err), "Unknown handedness: X");
        }

        #[test]
        fn test_parse_goalie_decision_error_display() {
            let err = ParseGoalieDecisionError("X".to_string());
            assert_eq!(format!("{}", err), "Unknown goalie decision: X");
        }
    }
}
