//! Shared enums for NHL API types
//!
//! This module contains enums representing categorical data from the NHL API.
//! All enums implement serde Serialize/Deserialize to match their API string representations.

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
pub struct ParsePositionError(String);

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
// PeriodType
// =============================================================================

/// Error type for parsing PeriodType from string
#[derive(Error, Debug, PartialEq)]
#[error("Unknown period type: {0}")]
pub struct ParsePeriodTypeError(String);

/// NHL period type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PeriodType {
    /// Regulation period
    #[serde(rename = "REG")]
    Regulation,

    /// Overtime period
    #[serde(rename = "OT")]
    Overtime,

    /// Shootout
    #[serde(rename = "SO")]
    Shootout,
}

impl PeriodType {
    /// Returns the short code for this period type
    pub const fn code(&self) -> &'static str {
        match self {
            PeriodType::Regulation => "REG",
            PeriodType::Overtime => "OT",
            PeriodType::Shootout => "SO",
        }
    }

    /// Returns the full name for this period type
    pub const fn name(&self) -> &'static str {
        match self {
            PeriodType::Regulation => "Regulation",
            PeriodType::Overtime => "Overtime",
            PeriodType::Shootout => "Shootout",
        }
    }

    /// Returns true if this is overtime (includes shootout)
    pub const fn is_overtime(&self) -> bool {
        matches!(self, PeriodType::Overtime | PeriodType::Shootout)
    }
}

impl fmt::Display for PeriodType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.code())
    }
}

impl FromStr for PeriodType {
    type Err = ParsePeriodTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "REG" => Ok(PeriodType::Regulation),
            "OT" => Ok(PeriodType::Overtime),
            "SO" => Ok(PeriodType::Shootout),
            _ => Err(ParsePeriodTypeError(s.to_string())),
        }
    }
}

// =============================================================================
// Handedness
// =============================================================================

/// Error type for parsing Handedness from string
#[derive(Error, Debug, PartialEq)]
#[error("Unknown handedness: {0}")]
pub struct ParseHandednessError(String);

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
// HomeRoad
// =============================================================================

/// Error type for parsing HomeRoad from string
#[derive(Error, Debug, PartialEq)]
#[error("Unknown home/road value: {0}")]
pub struct ParseHomeRoadError(String);

/// Home or road game indicator
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HomeRoad {
    /// Home game
    #[serde(rename = "H")]
    Home,

    /// Road (away) game
    #[serde(rename = "R")]
    Road,
}

impl HomeRoad {
    pub const fn code(&self) -> &'static str {
        match self {
            HomeRoad::Home => "H",
            HomeRoad::Road => "R",
        }
    }

    pub const fn name(&self) -> &'static str {
        match self {
            HomeRoad::Home => "Home",
            HomeRoad::Road => "Road",
        }
    }
}

impl fmt::Display for HomeRoad {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.code())
    }
}

impl FromStr for HomeRoad {
    type Err = ParseHomeRoadError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "H" => Ok(HomeRoad::Home),
            "R" => Ok(HomeRoad::Road),
            _ => Err(ParseHomeRoadError(s.to_string())),
        }
    }
}

// =============================================================================
// ZoneCode
// =============================================================================

/// Error type for parsing ZoneCode from string
#[derive(Error, Debug, PartialEq)]
#[error("Unknown zone code: {0}")]
pub struct ParseZoneCodeError(String);

/// Ice zone where play event occurred
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ZoneCode {
    /// Offensive zone
    #[serde(rename = "O")]
    Offensive,

    /// Defensive zone
    #[serde(rename = "D")]
    Defensive,

    /// Neutral zone
    #[serde(rename = "N")]
    Neutral,
}

impl ZoneCode {
    pub const fn code(&self) -> &'static str {
        match self {
            ZoneCode::Offensive => "O",
            ZoneCode::Defensive => "D",
            ZoneCode::Neutral => "N",
        }
    }

    pub const fn name(&self) -> &'static str {
        match self {
            ZoneCode::Offensive => "Offensive",
            ZoneCode::Defensive => "Defensive",
            ZoneCode::Neutral => "Neutral",
        }
    }
}

impl fmt::Display for ZoneCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.code())
    }
}

impl FromStr for ZoneCode {
    type Err = ParseZoneCodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "O" => Ok(ZoneCode::Offensive),
            "D" => Ok(ZoneCode::Defensive),
            "N" => Ok(ZoneCode::Neutral),
            _ => Err(ParseZoneCodeError(s.to_string())),
        }
    }
}

// =============================================================================
// DefendingSide
// =============================================================================

/// Error type for parsing DefendingSide from string
#[derive(Error, Debug, PartialEq)]
#[error("Unknown defending side: {0}")]
pub struct ParseDefendingSideError(String);

/// Which side of the ice the home team is defending
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DefendingSide {
    /// Defending left side
    Left,

    /// Defending right side
    Right,
}

impl DefendingSide {
    pub const fn name(&self) -> &'static str {
        match self {
            DefendingSide::Left => "left",
            DefendingSide::Right => "right",
        }
    }
}

impl fmt::Display for DefendingSide {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl FromStr for DefendingSide {
    type Err = ParseDefendingSideError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "left" => Ok(DefendingSide::Left),
            "right" => Ok(DefendingSide::Right),
            _ => Err(ParseDefendingSideError(s.to_string())),
        }
    }
}

// =============================================================================
// GoalieDecision
// =============================================================================

/// Error type for parsing GoalieDecision from string
#[derive(Error, Debug, PartialEq)]
#[error("Unknown goalie decision: {0}")]
pub struct ParseGoalieDecisionError(String);

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

// =============================================================================
// GameScheduleState
// =============================================================================

/// Error type for parsing GameScheduleState from string
#[derive(Error, Debug, PartialEq)]
#[error("Unknown game schedule state: {0}")]
pub struct ParseGameScheduleStateError(String);

/// Game schedule state (OK, postponed, etc.)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GameScheduleState {
    /// Game is scheduled as planned
    #[serde(rename = "OK")]
    Ok,

    /// Game is postponed
    #[serde(rename = "PPD")]
    Postponed,

    /// Game is suspended
    #[serde(rename = "SUSP")]
    Suspended,

    /// Game is cancelled
    #[serde(rename = "CNCL")]
    Cancelled,
}

impl GameScheduleState {
    pub const fn code(&self) -> &'static str {
        match self {
            GameScheduleState::Ok => "OK",
            GameScheduleState::Postponed => "PPD",
            GameScheduleState::Suspended => "SUSP",
            GameScheduleState::Cancelled => "CNCL",
        }
    }

    pub const fn name(&self) -> &'static str {
        match self {
            GameScheduleState::Ok => "OK",
            GameScheduleState::Postponed => "Postponed",
            GameScheduleState::Suspended => "Suspended",
            GameScheduleState::Cancelled => "Cancelled",
        }
    }

    /// Returns true if the game is playable (not postponed/cancelled/suspended)
    pub const fn is_playable(&self) -> bool {
        matches!(self, GameScheduleState::Ok)
    }
}

impl fmt::Display for GameScheduleState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.code())
    }
}

impl FromStr for GameScheduleState {
    type Err = ParseGameScheduleStateError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "OK" => Ok(GameScheduleState::Ok),
            "PPD" => Ok(GameScheduleState::Postponed),
            "SUSP" => Ok(GameScheduleState::Suspended),
            "CNCL" => Ok(GameScheduleState::Cancelled),
            _ => Err(ParseGameScheduleStateError(s.to_string())),
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
            assert_eq!(
                serde_json::to_string(&Position::Defense).unwrap(),
                r#""D""#
            );
            assert_eq!(
                serde_json::to_string(&Position::Goalie).unwrap(),
                r#""G""#
            );
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

    mod period_type_tests {
        use super::*;

        #[test]
        fn test_period_type_code() {
            assert_eq!(PeriodType::Regulation.code(), "REG");
            assert_eq!(PeriodType::Overtime.code(), "OT");
            assert_eq!(PeriodType::Shootout.code(), "SO");
        }

        #[test]
        fn test_period_type_name() {
            assert_eq!(PeriodType::Regulation.name(), "Regulation");
            assert_eq!(PeriodType::Overtime.name(), "Overtime");
            assert_eq!(PeriodType::Shootout.name(), "Shootout");
        }

        #[test]
        fn test_period_type_is_overtime() {
            assert!(!PeriodType::Regulation.is_overtime());
            assert!(PeriodType::Overtime.is_overtime());
            assert!(PeriodType::Shootout.is_overtime());
        }

        #[test]
        fn test_period_type_display() {
            assert_eq!(PeriodType::Regulation.to_string(), "REG");
            assert_eq!(PeriodType::Overtime.to_string(), "OT");
            assert_eq!(PeriodType::Shootout.to_string(), "SO");
        }

        #[test]
        fn test_period_type_from_str() {
            assert_eq!("REG".parse::<PeriodType>().unwrap(), PeriodType::Regulation);
            assert_eq!("OT".parse::<PeriodType>().unwrap(), PeriodType::Overtime);
            assert_eq!("SO".parse::<PeriodType>().unwrap(), PeriodType::Shootout);
        }

        #[test]
        fn test_period_type_from_str_invalid() {
            let result = "INVALID".parse::<PeriodType>();
            assert!(result.is_err());
        }

        #[test]
        fn test_period_type_serialize() {
            assert_eq!(serde_json::to_string(&PeriodType::Regulation).unwrap(), r#""REG""#);
            assert_eq!(serde_json::to_string(&PeriodType::Overtime).unwrap(), r#""OT""#);
            assert_eq!(serde_json::to_string(&PeriodType::Shootout).unwrap(), r#""SO""#);
        }

        #[test]
        fn test_period_type_deserialize() {
            assert_eq!(serde_json::from_str::<PeriodType>(r#""REG""#).unwrap(), PeriodType::Regulation);
            assert_eq!(serde_json::from_str::<PeriodType>(r#""OT""#).unwrap(), PeriodType::Overtime);
            assert_eq!(serde_json::from_str::<PeriodType>(r#""SO""#).unwrap(), PeriodType::Shootout);
        }

        #[test]
        fn test_period_type_roundtrip() {
            for pt in [PeriodType::Regulation, PeriodType::Overtime, PeriodType::Shootout] {
                let serialized = serde_json::to_string(&pt).unwrap();
                let deserialized: PeriodType = serde_json::from_str(&serialized).unwrap();
                assert_eq!(pt, deserialized);
            }
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
            assert_eq!(serde_json::from_str::<Handedness>(r#""L""#).unwrap(), Handedness::Left);
            assert_eq!(serde_json::from_str::<Handedness>(r#""R""#).unwrap(), Handedness::Right);
        }

        #[test]
        fn test_handedness_roundtrip() {
            for h in [Handedness::Left, Handedness::Right] {
                let serialized = serde_json::to_string(&h).unwrap();
                let deserialized: Handedness = serde_json::from_str(&serialized).unwrap();
                assert_eq!(h, deserialized);
            }
        }
    }

    mod home_road_tests {
        use super::*;

        #[test]
        fn test_home_road_serialize() {
            assert_eq!(serde_json::to_string(&HomeRoad::Home).unwrap(), r#""H""#);
            assert_eq!(serde_json::to_string(&HomeRoad::Road).unwrap(), r#""R""#);
        }

        #[test]
        fn test_home_road_deserialize() {
            assert_eq!(serde_json::from_str::<HomeRoad>(r#""H""#).unwrap(), HomeRoad::Home);
            assert_eq!(serde_json::from_str::<HomeRoad>(r#""R""#).unwrap(), HomeRoad::Road);
        }

        #[test]
        fn test_home_road_from_str() {
            assert_eq!("H".parse::<HomeRoad>().unwrap(), HomeRoad::Home);
            assert_eq!("R".parse::<HomeRoad>().unwrap(), HomeRoad::Road);
        }

        #[test]
        fn test_home_road_display() {
            assert_eq!(HomeRoad::Home.to_string(), "H");
            assert_eq!(HomeRoad::Road.to_string(), "R");
        }
    }

    mod zone_code_tests {
        use super::*;

        #[test]
        fn test_zone_code_serialize() {
            assert_eq!(serde_json::to_string(&ZoneCode::Offensive).unwrap(), r#""O""#);
            assert_eq!(serde_json::to_string(&ZoneCode::Defensive).unwrap(), r#""D""#);
            assert_eq!(serde_json::to_string(&ZoneCode::Neutral).unwrap(), r#""N""#);
        }

        #[test]
        fn test_zone_code_deserialize() {
            assert_eq!(serde_json::from_str::<ZoneCode>(r#""O""#).unwrap(), ZoneCode::Offensive);
            assert_eq!(serde_json::from_str::<ZoneCode>(r#""D""#).unwrap(), ZoneCode::Defensive);
            assert_eq!(serde_json::from_str::<ZoneCode>(r#""N""#).unwrap(), ZoneCode::Neutral);
        }

        #[test]
        fn test_zone_code_from_str() {
            assert_eq!("O".parse::<ZoneCode>().unwrap(), ZoneCode::Offensive);
            assert_eq!("D".parse::<ZoneCode>().unwrap(), ZoneCode::Defensive);
            assert_eq!("N".parse::<ZoneCode>().unwrap(), ZoneCode::Neutral);
        }
    }

    mod defending_side_tests {
        use super::*;

        #[test]
        fn test_defending_side_serialize() {
            assert_eq!(serde_json::to_string(&DefendingSide::Left).unwrap(), r#""left""#);
            assert_eq!(serde_json::to_string(&DefendingSide::Right).unwrap(), r#""right""#);
        }

        #[test]
        fn test_defending_side_deserialize() {
            assert_eq!(serde_json::from_str::<DefendingSide>(r#""left""#).unwrap(), DefendingSide::Left);
            assert_eq!(serde_json::from_str::<DefendingSide>(r#""right""#).unwrap(), DefendingSide::Right);
        }

        #[test]
        fn test_defending_side_from_str() {
            assert_eq!("left".parse::<DefendingSide>().unwrap(), DefendingSide::Left);
            assert_eq!("right".parse::<DefendingSide>().unwrap(), DefendingSide::Right);
            assert_eq!("LEFT".parse::<DefendingSide>().unwrap(), DefendingSide::Left);
        }
    }

    mod goalie_decision_tests {
        use super::*;

        #[test]
        fn test_goalie_decision_serialize() {
            assert_eq!(serde_json::to_string(&GoalieDecision::Win).unwrap(), r#""W""#);
            assert_eq!(serde_json::to_string(&GoalieDecision::Loss).unwrap(), r#""L""#);
            assert_eq!(serde_json::to_string(&GoalieDecision::OvertimeLoss).unwrap(), r#""O""#);
        }

        #[test]
        fn test_goalie_decision_deserialize() {
            assert_eq!(serde_json::from_str::<GoalieDecision>(r#""W""#).unwrap(), GoalieDecision::Win);
            assert_eq!(serde_json::from_str::<GoalieDecision>(r#""L""#).unwrap(), GoalieDecision::Loss);
            assert_eq!(serde_json::from_str::<GoalieDecision>(r#""O""#).unwrap(), GoalieDecision::OvertimeLoss);
        }

        #[test]
        fn test_goalie_decision_from_str() {
            assert_eq!("W".parse::<GoalieDecision>().unwrap(), GoalieDecision::Win);
            assert_eq!("L".parse::<GoalieDecision>().unwrap(), GoalieDecision::Loss);
            assert_eq!("O".parse::<GoalieDecision>().unwrap(), GoalieDecision::OvertimeLoss);
        }
    }

    mod game_schedule_state_tests {
        use super::*;

        #[test]
        fn test_game_schedule_state_serialize() {
            assert_eq!(serde_json::to_string(&GameScheduleState::Ok).unwrap(), r#""OK""#);
            assert_eq!(serde_json::to_string(&GameScheduleState::Postponed).unwrap(), r#""PPD""#);
            assert_eq!(serde_json::to_string(&GameScheduleState::Suspended).unwrap(), r#""SUSP""#);
            assert_eq!(serde_json::to_string(&GameScheduleState::Cancelled).unwrap(), r#""CNCL""#);
        }

        #[test]
        fn test_game_schedule_state_deserialize() {
            assert_eq!(serde_json::from_str::<GameScheduleState>(r#""OK""#).unwrap(), GameScheduleState::Ok);
            assert_eq!(serde_json::from_str::<GameScheduleState>(r#""PPD""#).unwrap(), GameScheduleState::Postponed);
            assert_eq!(serde_json::from_str::<GameScheduleState>(r#""SUSP""#).unwrap(), GameScheduleState::Suspended);
            assert_eq!(serde_json::from_str::<GameScheduleState>(r#""CNCL""#).unwrap(), GameScheduleState::Cancelled);
        }

        #[test]
        fn test_game_schedule_state_is_playable() {
            assert!(GameScheduleState::Ok.is_playable());
            assert!(!GameScheduleState::Postponed.is_playable());
            assert!(!GameScheduleState::Suspended.is_playable());
            assert!(!GameScheduleState::Cancelled.is_playable());
        }
    }
}
