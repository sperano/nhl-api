//! Game/play-related enums for NHL API types
//!
//! This module contains enums related to game state and play events.

use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
use thiserror::Error;

// =============================================================================
// PeriodType
// =============================================================================

/// Error type for parsing PeriodType from string
#[derive(Error, Debug, PartialEq)]
#[error("Unknown period type: {0}")]
pub struct ParsePeriodTypeError(pub String);

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
// HomeRoad
// =============================================================================

/// Error type for parsing HomeRoad from string
#[derive(Error, Debug, PartialEq)]
#[error("Unknown home/road value: {0}")]
pub struct ParseHomeRoadError(pub String);

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
pub struct ParseZoneCodeError(pub String);

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
pub struct ParseDefendingSideError(pub String);

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
// GameScheduleState
// =============================================================================

/// Error type for parsing GameScheduleState from string
#[derive(Error, Debug, PartialEq)]
#[error("Unknown game schedule state: {0}")]
pub struct ParseGameScheduleStateError(pub String);

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
            assert_eq!(
                serde_json::to_string(&PeriodType::Regulation).unwrap(),
                r#""REG""#
            );
            assert_eq!(
                serde_json::to_string(&PeriodType::Overtime).unwrap(),
                r#""OT""#
            );
            assert_eq!(
                serde_json::to_string(&PeriodType::Shootout).unwrap(),
                r#""SO""#
            );
        }

        #[test]
        fn test_period_type_deserialize() {
            assert_eq!(
                serde_json::from_str::<PeriodType>(r#""REG""#).unwrap(),
                PeriodType::Regulation
            );
            assert_eq!(
                serde_json::from_str::<PeriodType>(r#""OT""#).unwrap(),
                PeriodType::Overtime
            );
            assert_eq!(
                serde_json::from_str::<PeriodType>(r#""SO""#).unwrap(),
                PeriodType::Shootout
            );
        }

        #[test]
        fn test_period_type_roundtrip() {
            for pt in [
                PeriodType::Regulation,
                PeriodType::Overtime,
                PeriodType::Shootout,
            ] {
                let serialized = serde_json::to_string(&pt).unwrap();
                let deserialized: PeriodType = serde_json::from_str(&serialized).unwrap();
                assert_eq!(pt, deserialized);
            }
        }

        #[test]
        fn test_period_type_hash() {
            use std::collections::HashSet;
            let mut set = HashSet::new();
            set.insert(PeriodType::Regulation);
            set.insert(PeriodType::Overtime);
            set.insert(PeriodType::Shootout);
            assert_eq!(set.len(), 3);
        }
    }

    mod home_road_tests {
        use super::*;

        #[test]
        fn test_home_road_code() {
            assert_eq!(HomeRoad::Home.code(), "H");
            assert_eq!(HomeRoad::Road.code(), "R");
        }

        #[test]
        fn test_home_road_name() {
            assert_eq!(HomeRoad::Home.name(), "Home");
            assert_eq!(HomeRoad::Road.name(), "Road");
        }

        #[test]
        fn test_home_road_serialize() {
            assert_eq!(serde_json::to_string(&HomeRoad::Home).unwrap(), r#""H""#);
            assert_eq!(serde_json::to_string(&HomeRoad::Road).unwrap(), r#""R""#);
        }

        #[test]
        fn test_home_road_deserialize() {
            assert_eq!(
                serde_json::from_str::<HomeRoad>(r#""H""#).unwrap(),
                HomeRoad::Home
            );
            assert_eq!(
                serde_json::from_str::<HomeRoad>(r#""R""#).unwrap(),
                HomeRoad::Road
            );
        }

        #[test]
        fn test_home_road_from_str() {
            assert_eq!("H".parse::<HomeRoad>().unwrap(), HomeRoad::Home);
            assert_eq!("R".parse::<HomeRoad>().unwrap(), HomeRoad::Road);
        }

        #[test]
        fn test_home_road_from_str_invalid() {
            let result = "X".parse::<HomeRoad>();
            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), ParseHomeRoadError("X".to_string()));
        }

        #[test]
        fn test_home_road_display() {
            assert_eq!(HomeRoad::Home.to_string(), "H");
            assert_eq!(HomeRoad::Road.to_string(), "R");
        }

        #[test]
        fn test_home_road_roundtrip() {
            for hr in [HomeRoad::Home, HomeRoad::Road] {
                let serialized = serde_json::to_string(&hr).unwrap();
                let deserialized: HomeRoad = serde_json::from_str(&serialized).unwrap();
                assert_eq!(hr, deserialized);
            }
        }

        #[test]
        fn test_home_road_hash() {
            use std::collections::HashSet;
            let mut set = HashSet::new();
            set.insert(HomeRoad::Home);
            set.insert(HomeRoad::Road);
            set.insert(HomeRoad::Home); // Duplicate
            assert_eq!(set.len(), 2);
        }
    }

    mod zone_code_tests {
        use super::*;

        #[test]
        fn test_zone_code_code() {
            assert_eq!(ZoneCode::Offensive.code(), "O");
            assert_eq!(ZoneCode::Defensive.code(), "D");
            assert_eq!(ZoneCode::Neutral.code(), "N");
        }

        #[test]
        fn test_zone_code_name() {
            assert_eq!(ZoneCode::Offensive.name(), "Offensive");
            assert_eq!(ZoneCode::Defensive.name(), "Defensive");
            assert_eq!(ZoneCode::Neutral.name(), "Neutral");
        }

        #[test]
        fn test_zone_code_serialize() {
            assert_eq!(
                serde_json::to_string(&ZoneCode::Offensive).unwrap(),
                r#""O""#
            );
            assert_eq!(
                serde_json::to_string(&ZoneCode::Defensive).unwrap(),
                r#""D""#
            );
            assert_eq!(serde_json::to_string(&ZoneCode::Neutral).unwrap(), r#""N""#);
        }

        #[test]
        fn test_zone_code_deserialize() {
            assert_eq!(
                serde_json::from_str::<ZoneCode>(r#""O""#).unwrap(),
                ZoneCode::Offensive
            );
            assert_eq!(
                serde_json::from_str::<ZoneCode>(r#""D""#).unwrap(),
                ZoneCode::Defensive
            );
            assert_eq!(
                serde_json::from_str::<ZoneCode>(r#""N""#).unwrap(),
                ZoneCode::Neutral
            );
        }

        #[test]
        fn test_zone_code_from_str() {
            assert_eq!("O".parse::<ZoneCode>().unwrap(), ZoneCode::Offensive);
            assert_eq!("D".parse::<ZoneCode>().unwrap(), ZoneCode::Defensive);
            assert_eq!("N".parse::<ZoneCode>().unwrap(), ZoneCode::Neutral);
        }

        #[test]
        fn test_zone_code_from_str_invalid() {
            let result = "X".parse::<ZoneCode>();
            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), ParseZoneCodeError("X".to_string()));
        }

        #[test]
        fn test_zone_code_display() {
            assert_eq!(ZoneCode::Offensive.to_string(), "O");
            assert_eq!(ZoneCode::Defensive.to_string(), "D");
            assert_eq!(ZoneCode::Neutral.to_string(), "N");
        }

        #[test]
        fn test_zone_code_roundtrip() {
            for zc in [ZoneCode::Offensive, ZoneCode::Defensive, ZoneCode::Neutral] {
                let serialized = serde_json::to_string(&zc).unwrap();
                let deserialized: ZoneCode = serde_json::from_str(&serialized).unwrap();
                assert_eq!(zc, deserialized);
            }
        }

        #[test]
        fn test_zone_code_hash() {
            use std::collections::HashSet;
            let mut set = HashSet::new();
            set.insert(ZoneCode::Offensive);
            set.insert(ZoneCode::Defensive);
            set.insert(ZoneCode::Neutral);
            assert_eq!(set.len(), 3);
        }
    }

    mod defending_side_tests {
        use super::*;

        #[test]
        fn test_defending_side_name() {
            assert_eq!(DefendingSide::Left.name(), "left");
            assert_eq!(DefendingSide::Right.name(), "right");
        }

        #[test]
        fn test_defending_side_serialize() {
            assert_eq!(
                serde_json::to_string(&DefendingSide::Left).unwrap(),
                r#""left""#
            );
            assert_eq!(
                serde_json::to_string(&DefendingSide::Right).unwrap(),
                r#""right""#
            );
        }

        #[test]
        fn test_defending_side_deserialize() {
            assert_eq!(
                serde_json::from_str::<DefendingSide>(r#""left""#).unwrap(),
                DefendingSide::Left
            );
            assert_eq!(
                serde_json::from_str::<DefendingSide>(r#""right""#).unwrap(),
                DefendingSide::Right
            );
        }

        #[test]
        fn test_defending_side_from_str() {
            assert_eq!(
                "left".parse::<DefendingSide>().unwrap(),
                DefendingSide::Left
            );
            assert_eq!(
                "right".parse::<DefendingSide>().unwrap(),
                DefendingSide::Right
            );
            assert_eq!(
                "LEFT".parse::<DefendingSide>().unwrap(),
                DefendingSide::Left
            );
            assert_eq!(
                "Right".parse::<DefendingSide>().unwrap(),
                DefendingSide::Right
            );
        }

        #[test]
        fn test_defending_side_from_str_invalid() {
            let result = "center".parse::<DefendingSide>();
            assert!(result.is_err());
            assert_eq!(
                result.unwrap_err(),
                ParseDefendingSideError("center".to_string())
            );
        }

        #[test]
        fn test_defending_side_display() {
            assert_eq!(DefendingSide::Left.to_string(), "left");
            assert_eq!(DefendingSide::Right.to_string(), "right");
        }

        #[test]
        fn test_defending_side_roundtrip() {
            for ds in [DefendingSide::Left, DefendingSide::Right] {
                let serialized = serde_json::to_string(&ds).unwrap();
                let deserialized: DefendingSide = serde_json::from_str(&serialized).unwrap();
                assert_eq!(ds, deserialized);
            }
        }

        #[test]
        fn test_defending_side_hash() {
            use std::collections::HashSet;
            let mut set = HashSet::new();
            set.insert(DefendingSide::Left);
            set.insert(DefendingSide::Right);
            assert_eq!(set.len(), 2);
        }
    }

    mod game_schedule_state_tests {
        use super::*;

        #[test]
        fn test_game_schedule_state_code() {
            assert_eq!(GameScheduleState::Ok.code(), "OK");
            assert_eq!(GameScheduleState::Postponed.code(), "PPD");
            assert_eq!(GameScheduleState::Suspended.code(), "SUSP");
            assert_eq!(GameScheduleState::Cancelled.code(), "CNCL");
        }

        #[test]
        fn test_game_schedule_state_name() {
            assert_eq!(GameScheduleState::Ok.name(), "OK");
            assert_eq!(GameScheduleState::Postponed.name(), "Postponed");
            assert_eq!(GameScheduleState::Suspended.name(), "Suspended");
            assert_eq!(GameScheduleState::Cancelled.name(), "Cancelled");
        }

        #[test]
        fn test_game_schedule_state_serialize() {
            assert_eq!(
                serde_json::to_string(&GameScheduleState::Ok).unwrap(),
                r#""OK""#
            );
            assert_eq!(
                serde_json::to_string(&GameScheduleState::Postponed).unwrap(),
                r#""PPD""#
            );
            assert_eq!(
                serde_json::to_string(&GameScheduleState::Suspended).unwrap(),
                r#""SUSP""#
            );
            assert_eq!(
                serde_json::to_string(&GameScheduleState::Cancelled).unwrap(),
                r#""CNCL""#
            );
        }

        #[test]
        fn test_game_schedule_state_deserialize() {
            assert_eq!(
                serde_json::from_str::<GameScheduleState>(r#""OK""#).unwrap(),
                GameScheduleState::Ok
            );
            assert_eq!(
                serde_json::from_str::<GameScheduleState>(r#""PPD""#).unwrap(),
                GameScheduleState::Postponed
            );
            assert_eq!(
                serde_json::from_str::<GameScheduleState>(r#""SUSP""#).unwrap(),
                GameScheduleState::Suspended
            );
            assert_eq!(
                serde_json::from_str::<GameScheduleState>(r#""CNCL""#).unwrap(),
                GameScheduleState::Cancelled
            );
        }

        #[test]
        fn test_game_schedule_state_from_str() {
            assert_eq!(
                "OK".parse::<GameScheduleState>().unwrap(),
                GameScheduleState::Ok
            );
            assert_eq!(
                "PPD".parse::<GameScheduleState>().unwrap(),
                GameScheduleState::Postponed
            );
            assert_eq!(
                "SUSP".parse::<GameScheduleState>().unwrap(),
                GameScheduleState::Suspended
            );
            assert_eq!(
                "CNCL".parse::<GameScheduleState>().unwrap(),
                GameScheduleState::Cancelled
            );
        }

        #[test]
        fn test_game_schedule_state_from_str_invalid() {
            let result = "UNKNOWN".parse::<GameScheduleState>();
            assert!(result.is_err());
            assert_eq!(
                result.unwrap_err(),
                ParseGameScheduleStateError("UNKNOWN".to_string())
            );
        }

        #[test]
        fn test_game_schedule_state_display() {
            assert_eq!(GameScheduleState::Ok.to_string(), "OK");
            assert_eq!(GameScheduleState::Postponed.to_string(), "PPD");
            assert_eq!(GameScheduleState::Suspended.to_string(), "SUSP");
            assert_eq!(GameScheduleState::Cancelled.to_string(), "CNCL");
        }

        #[test]
        fn test_game_schedule_state_is_playable() {
            assert!(GameScheduleState::Ok.is_playable());
            assert!(!GameScheduleState::Postponed.is_playable());
            assert!(!GameScheduleState::Suspended.is_playable());
            assert!(!GameScheduleState::Cancelled.is_playable());
        }

        #[test]
        fn test_game_schedule_state_roundtrip() {
            for gss in [
                GameScheduleState::Ok,
                GameScheduleState::Postponed,
                GameScheduleState::Suspended,
                GameScheduleState::Cancelled,
            ] {
                let serialized = serde_json::to_string(&gss).unwrap();
                let deserialized: GameScheduleState = serde_json::from_str(&serialized).unwrap();
                assert_eq!(gss, deserialized);
            }
        }

        #[test]
        fn test_game_schedule_state_hash() {
            use std::collections::HashSet;
            let mut set = HashSet::new();
            set.insert(GameScheduleState::Ok);
            set.insert(GameScheduleState::Postponed);
            set.insert(GameScheduleState::Suspended);
            set.insert(GameScheduleState::Cancelled);
            assert_eq!(set.len(), 4);
        }
    }

    mod error_display_tests {
        use super::*;

        #[test]
        fn test_parse_period_type_error_display() {
            let err = ParsePeriodTypeError("INVALID".to_string());
            assert_eq!(format!("{}", err), "Unknown period type: INVALID");
        }

        #[test]
        fn test_parse_home_road_error_display() {
            let err = ParseHomeRoadError("X".to_string());
            assert_eq!(format!("{}", err), "Unknown home/road value: X");
        }

        #[test]
        fn test_parse_zone_code_error_display() {
            let err = ParseZoneCodeError("X".to_string());
            assert_eq!(format!("{}", err), "Unknown zone code: X");
        }

        #[test]
        fn test_parse_defending_side_error_display() {
            let err = ParseDefendingSideError("center".to_string());
            assert_eq!(format!("{}", err), "Unknown defending side: center");
        }

        #[test]
        fn test_parse_game_schedule_state_error_display() {
            let err = ParseGameScheduleStateError("UNKNOWN".to_string());
            assert_eq!(format!("{}", err), "Unknown game schedule state: UNKNOWN");
        }
    }
}
