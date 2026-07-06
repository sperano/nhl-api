//! Game/play-related enums for NHL API types
//!
//! This module contains enums related to game state and play events.

use super::macros::nhl_string_enum;

// =============================================================================
// PeriodType
// =============================================================================

nhl_string_enum! {
    error_name = "period type",
    display = code,
    /// NHL period type
    pub enum PeriodType {
        /// Regulation period
        Regulation = "REG", name = "Regulation";
        /// Overtime period
        Overtime = "OT", name = "Overtime";
        /// Shootout
        Shootout = "SO", name = "Shootout";
    }
}

impl PeriodType {
    /// Returns true if this is overtime (includes shootout)
    pub const fn is_overtime(&self) -> bool {
        matches!(self, PeriodType::Overtime | PeriodType::Shootout)
    }
}

// =============================================================================
// HomeRoad
// =============================================================================

nhl_string_enum! {
    error_name = "home/road value",
    display = code,
    /// Home or road game indicator
    pub enum HomeRoad {
        /// Home game
        Home = "H", name = "Home";
        /// Road (away) game
        Road = "R", name = "Road";
    }
}

// =============================================================================
// ZoneCode
// =============================================================================

nhl_string_enum! {
    error_name = "zone code",
    display = code,
    /// Ice zone where play event occurred
    pub enum ZoneCode {
        /// Offensive zone
        Offensive = "O", name = "Offensive";
        /// Defensive zone
        Defensive = "D", name = "Defensive";
        /// Neutral zone
        Neutral = "N", name = "Neutral";
    }
}

// =============================================================================
// DefendingSide
// =============================================================================

nhl_string_enum! {
    error_name = "defending side",
    display = code,
    /// Which side of the ice the home team is defending
    pub enum DefendingSide {
        /// Defending left side
        Left = "left", name = "left", aliases = ["LEFT", "Left"];
        /// Defending right side
        Right = "right", name = "right", aliases = ["RIGHT", "Right"];
    }
}

// =============================================================================
// GameScheduleState
// =============================================================================

nhl_string_enum! {
    error_name = "game schedule state",
    display = code,
    /// Game schedule state (OK, postponed, etc.)
    pub enum GameScheduleState {
        /// Game is scheduled as planned
        Ok = "OK", name = "OK";
        /// Game will not be played (historical/administrative games)
        DontPlay = "DONT_PLAY", name = "Don't Play";
        /// Game is postponed
        Postponed = "PPD", name = "Postponed";
        /// Game is suspended
        Suspended = "SUSP", name = "Suspended";
        /// Game time is to be determined
        Tbd = "TBD", name = "TBD";
        /// Game has been completed
        Completed = "COMPLETED", name = "Completed";
        /// Game is cancelled
        Cancelled = "CNCL", name = "Cancelled";
    }
}

impl GameScheduleState {
    /// Returns true if the game is playable (not postponed/cancelled/suspended)
    pub const fn is_playable(&self) -> bool {
        matches!(self, GameScheduleState::Ok)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::enums::UnknownEnumValue;

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
        fn test_period_type_from_str_unknown_enum_value() {
            let err = "INVALID".parse::<PeriodType>().unwrap_err();
            assert_eq!(
                err,
                UnknownEnumValue {
                    enum_name: "period type",
                    value: "INVALID".to_string(),
                }
            );
            assert_eq!(err.enum_name, "period type");
            assert_eq!(err.value, "INVALID");
        }

        #[test]
        fn test_period_type_deserialize_unknown_error_message() {
            let err = serde_json::from_str::<PeriodType>(r#""INVALID""#).unwrap_err();
            let message = err.to_string();
            assert!(
                message.contains("period type"),
                "message missing enum name: {message}"
            );
            assert!(
                message.contains("INVALID"),
                "message missing offending value: {message}"
            );
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
            assert_eq!(
                result.unwrap_err(),
                UnknownEnumValue {
                    enum_name: "home/road value",
                    value: "X".to_string(),
                }
            );
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
            assert_eq!(
                result.unwrap_err(),
                UnknownEnumValue {
                    enum_name: "zone code",
                    value: "X".to_string(),
                }
            );
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
                UnknownEnumValue {
                    enum_name: "defending side",
                    value: "center".to_string(),
                }
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
            assert_eq!(GameScheduleState::DontPlay.code(), "DONT_PLAY");
            assert_eq!(GameScheduleState::Postponed.code(), "PPD");
            assert_eq!(GameScheduleState::Suspended.code(), "SUSP");
            assert_eq!(GameScheduleState::Tbd.code(), "TBD");
            assert_eq!(GameScheduleState::Completed.code(), "COMPLETED");
            assert_eq!(GameScheduleState::Cancelled.code(), "CNCL");
        }

        #[test]
        fn test_game_schedule_state_name() {
            assert_eq!(GameScheduleState::Ok.name(), "OK");
            assert_eq!(GameScheduleState::DontPlay.name(), "Don't Play");
            assert_eq!(GameScheduleState::Postponed.name(), "Postponed");
            assert_eq!(GameScheduleState::Suspended.name(), "Suspended");
            assert_eq!(GameScheduleState::Tbd.name(), "TBD");
            assert_eq!(GameScheduleState::Completed.name(), "Completed");
            assert_eq!(GameScheduleState::Cancelled.name(), "Cancelled");
        }

        #[test]
        fn test_game_schedule_state_serialize() {
            assert_eq!(
                serde_json::to_string(&GameScheduleState::Ok).unwrap(),
                r#""OK""#
            );
            assert_eq!(
                serde_json::to_string(&GameScheduleState::DontPlay).unwrap(),
                r#""DONT_PLAY""#
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
                serde_json::to_string(&GameScheduleState::Tbd).unwrap(),
                r#""TBD""#
            );
            assert_eq!(
                serde_json::to_string(&GameScheduleState::Completed).unwrap(),
                r#""COMPLETED""#
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
                serde_json::from_str::<GameScheduleState>(r#""DONT_PLAY""#).unwrap(),
                GameScheduleState::DontPlay
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
                serde_json::from_str::<GameScheduleState>(r#""TBD""#).unwrap(),
                GameScheduleState::Tbd
            );
            assert_eq!(
                serde_json::from_str::<GameScheduleState>(r#""COMPLETED""#).unwrap(),
                GameScheduleState::Completed
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
                "DONT_PLAY".parse::<GameScheduleState>().unwrap(),
                GameScheduleState::DontPlay
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
                "TBD".parse::<GameScheduleState>().unwrap(),
                GameScheduleState::Tbd
            );
            assert_eq!(
                "COMPLETED".parse::<GameScheduleState>().unwrap(),
                GameScheduleState::Completed
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
                UnknownEnumValue {
                    enum_name: "game schedule state",
                    value: "UNKNOWN".to_string(),
                }
            );
        }

        #[test]
        fn test_game_schedule_state_display() {
            assert_eq!(GameScheduleState::Ok.to_string(), "OK");
            assert_eq!(GameScheduleState::DontPlay.to_string(), "DONT_PLAY");
            assert_eq!(GameScheduleState::Postponed.to_string(), "PPD");
            assert_eq!(GameScheduleState::Suspended.to_string(), "SUSP");
            assert_eq!(GameScheduleState::Tbd.to_string(), "TBD");
            assert_eq!(GameScheduleState::Completed.to_string(), "COMPLETED");
            assert_eq!(GameScheduleState::Cancelled.to_string(), "CNCL");
        }

        #[test]
        fn test_game_schedule_state_is_playable() {
            assert!(GameScheduleState::Ok.is_playable());
            assert!(!GameScheduleState::DontPlay.is_playable());
            assert!(!GameScheduleState::Postponed.is_playable());
            assert!(!GameScheduleState::Suspended.is_playable());
            assert!(!GameScheduleState::Tbd.is_playable());
            assert!(!GameScheduleState::Completed.is_playable());
            assert!(!GameScheduleState::Cancelled.is_playable());
        }

        /// Historical/administrative schedule entries use these three states;
        /// exercise them via a boxscore-shaped fixture rather than bare strings.
        #[test]
        fn test_game_schedule_state_deserialize_historical_fixture() {
            #[derive(serde::Deserialize)]
            struct GameFixture {
                #[serde(rename = "gameScheduleState")]
                game_schedule_state: GameScheduleState,
            }

            let dont_play: GameFixture =
                serde_json::from_str(r#"{"gameScheduleState": "DONT_PLAY"}"#).unwrap();
            assert_eq!(dont_play.game_schedule_state, GameScheduleState::DontPlay);

            let tbd: GameFixture = serde_json::from_str(r#"{"gameScheduleState": "TBD"}"#).unwrap();
            assert_eq!(tbd.game_schedule_state, GameScheduleState::Tbd);

            let completed: GameFixture =
                serde_json::from_str(r#"{"gameScheduleState": "COMPLETED"}"#).unwrap();
            assert_eq!(completed.game_schedule_state, GameScheduleState::Completed);
        }

        #[test]
        fn test_game_schedule_state_roundtrip() {
            for gss in [
                GameScheduleState::Ok,
                GameScheduleState::DontPlay,
                GameScheduleState::Postponed,
                GameScheduleState::Suspended,
                GameScheduleState::Tbd,
                GameScheduleState::Completed,
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
            set.insert(GameScheduleState::DontPlay);
            set.insert(GameScheduleState::Postponed);
            set.insert(GameScheduleState::Suspended);
            set.insert(GameScheduleState::Tbd);
            set.insert(GameScheduleState::Completed);
            set.insert(GameScheduleState::Cancelled);
            assert_eq!(set.len(), 7);
        }
    }
}
