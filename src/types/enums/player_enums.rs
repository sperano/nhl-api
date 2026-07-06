//! Player-related enums for NHL API types
//!
//! This module contains enums that describe player attributes.

use super::macros::nhl_string_enum;

// =============================================================================
// Position
// =============================================================================

nhl_string_enum! {
    error_name = "position",
    display = code,
    /// NHL player position
    pub enum Position {
        /// Center
        Center = "C", name = "Center";
        /// Left Wing
        LeftWing = "LW", name = "Left Wing", aliases = ["L"];
        /// Right Wing
        RightWing = "RW", name = "Right Wing", aliases = ["R"];
        /// Generic forward (used for historical games that don't distinguish
        /// center/left wing/right wing).
        Forward = "F", name = "Forward";
        /// Defense
        Defense = "D", name = "Defense";
        /// Goalie
        Goalie = "G", name = "Goalie";
    }
}

impl Position {
    /// Returns true if this is a forward position
    pub const fn is_forward(&self) -> bool {
        matches!(
            self,
            Position::Center | Position::LeftWing | Position::RightWing | Position::Forward
        )
    }

    /// Returns true if this is a skater position (not goalie)
    pub const fn is_skater(&self) -> bool {
        !matches!(self, Position::Goalie)
    }
}

// =============================================================================
// Handedness
// =============================================================================

nhl_string_enum! {
    error_name = "handedness",
    display = code,
    /// NHL player handedness (shoots/catches)
    pub enum Handedness {
        /// Left-handed
        Left = "L", name = "Left";
        /// Right-handed
        Right = "R", name = "Right";
    }
}

// =============================================================================
// GoalieDecision
// =============================================================================

nhl_string_enum! {
    error_name = "goalie decision",
    display = code,
    /// Goalie game decision (win/loss/tie/OT loss)
    pub enum GoalieDecision {
        /// Win
        Win = "W", name = "Win";
        /// Loss
        Loss = "L", name = "Loss";
        /// Tie (pre-shootout era, before the 2005-06 season)
        Tie = "T", name = "Tie";
        /// Overtime loss
        OvertimeLoss = "O", name = "Overtime Loss", aliases = ["OTL"];
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::enums::UnknownEnumValue;

    mod position_tests {
        use super::*;

        #[test]
        fn test_position_code() {
            assert_eq!(Position::Center.code(), "C");
            assert_eq!(Position::LeftWing.code(), "LW");
            assert_eq!(Position::RightWing.code(), "RW");
            assert_eq!(Position::Forward.code(), "F");
            assert_eq!(Position::Defense.code(), "D");
            assert_eq!(Position::Goalie.code(), "G");
        }

        #[test]
        fn test_position_name() {
            assert_eq!(Position::Center.name(), "Center");
            assert_eq!(Position::LeftWing.name(), "Left Wing");
            assert_eq!(Position::RightWing.name(), "Right Wing");
            assert_eq!(Position::Forward.name(), "Forward");
            assert_eq!(Position::Defense.name(), "Defense");
            assert_eq!(Position::Goalie.name(), "Goalie");
        }

        #[test]
        fn test_position_is_forward() {
            assert!(Position::Center.is_forward());
            assert!(Position::LeftWing.is_forward());
            assert!(Position::RightWing.is_forward());
            assert!(Position::Forward.is_forward());
            assert!(!Position::Defense.is_forward());
            assert!(!Position::Goalie.is_forward());
        }

        #[test]
        fn test_position_is_skater() {
            assert!(Position::Center.is_skater());
            assert!(Position::LeftWing.is_skater());
            assert!(Position::RightWing.is_skater());
            assert!(Position::Forward.is_skater());
            assert!(Position::Defense.is_skater());
            assert!(!Position::Goalie.is_skater());
        }

        #[test]
        fn test_position_display() {
            assert_eq!(Position::Center.to_string(), "C");
            assert_eq!(Position::LeftWing.to_string(), "LW");
            assert_eq!(Position::RightWing.to_string(), "RW");
            assert_eq!(Position::Forward.to_string(), "F");
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
            assert_eq!("F".parse::<Position>().unwrap(), Position::Forward);
            assert_eq!("D".parse::<Position>().unwrap(), Position::Defense);
            assert_eq!("G".parse::<Position>().unwrap(), Position::Goalie);
        }

        #[test]
        fn test_position_from_str_invalid() {
            let result = "X".parse::<Position>();
            assert!(result.is_err());
            assert_eq!(
                result.unwrap_err(),
                UnknownEnumValue {
                    enum_name: "position",
                    value: "X".to_string(),
                }
            );
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
            assert_eq!(serde_json::to_string(&Position::Forward).unwrap(), r#""F""#);
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

        /// Historical games (pre-modern-era tracking) sometimes only record a
        /// generic "F" (forward) rather than a specific forward position.
        #[test]
        fn test_position_deserialize_historical_forward() {
            #[derive(serde::Deserialize)]
            struct RosterEntryFixture {
                #[serde(rename = "positionCode")]
                position_code: Position,
            }

            let json = r#"{"positionCode": "F"}"#;
            let entry: RosterEntryFixture = serde_json::from_str(json).unwrap();
            assert_eq!(entry.position_code, Position::Forward);
            assert!(entry.position_code.is_forward());
        }

        #[test]
        fn test_position_roundtrip() {
            for pos in [
                Position::Center,
                Position::LeftWing,
                Position::RightWing,
                Position::Forward,
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
            assert_eq!(GoalieDecision::Tie.code(), "T");
            assert_eq!(GoalieDecision::OvertimeLoss.code(), "O");
        }

        #[test]
        fn test_goalie_decision_name() {
            assert_eq!(GoalieDecision::Win.name(), "Win");
            assert_eq!(GoalieDecision::Loss.name(), "Loss");
            assert_eq!(GoalieDecision::Tie.name(), "Tie");
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
                serde_json::to_string(&GoalieDecision::Tie).unwrap(),
                r#""T""#
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
                serde_json::from_str::<GoalieDecision>(r#""T""#).unwrap(),
                GoalieDecision::Tie
            );
            assert_eq!(
                serde_json::from_str::<GoalieDecision>(r#""O""#).unwrap(),
                GoalieDecision::OvertimeLoss
            );
        }

        #[test]
        fn test_goalie_decision_deserialize_otl_alias() {
            // "OTL" is Go's canonical form; kept here as a parse alias so both
            // shapes seen in the wild deserialize to the same variant.
            assert_eq!(
                serde_json::from_str::<GoalieDecision>(r#""OTL""#).unwrap(),
                GoalieDecision::OvertimeLoss
            );
        }

        #[test]
        fn test_goalie_decision_from_str() {
            assert_eq!("W".parse::<GoalieDecision>().unwrap(), GoalieDecision::Win);
            assert_eq!("L".parse::<GoalieDecision>().unwrap(), GoalieDecision::Loss);
            assert_eq!("T".parse::<GoalieDecision>().unwrap(), GoalieDecision::Tie);
            assert_eq!(
                "O".parse::<GoalieDecision>().unwrap(),
                GoalieDecision::OvertimeLoss
            );
            assert_eq!(
                "OTL".parse::<GoalieDecision>().unwrap(),
                GoalieDecision::OvertimeLoss
            );
        }

        #[test]
        fn test_goalie_decision_from_str_invalid() {
            let result = "X".parse::<GoalieDecision>();
            assert!(result.is_err());
            assert_eq!(
                result.unwrap_err(),
                UnknownEnumValue {
                    enum_name: "goalie decision",
                    value: "X".to_string(),
                }
            );
        }

        #[test]
        fn test_goalie_decision_display() {
            assert_eq!(GoalieDecision::Win.to_string(), "W");
            assert_eq!(GoalieDecision::Loss.to_string(), "L");
            assert_eq!(GoalieDecision::Tie.to_string(), "T");
            assert_eq!(GoalieDecision::OvertimeLoss.to_string(), "O");
        }

        /// Pre-shootout-era (before 2005-06) game logs record ties.
        #[test]
        fn test_goalie_decision_deserialize_historical_tie() {
            #[derive(serde::Deserialize)]
            struct GameLogEntryFixture {
                decision: GoalieDecision,
            }

            let json = r#"{"decision": "T"}"#;
            let entry: GameLogEntryFixture = serde_json::from_str(json).unwrap();
            assert_eq!(entry.decision, GoalieDecision::Tie);
        }

        #[test]
        fn test_goalie_decision_roundtrip() {
            for gd in [
                GoalieDecision::Win,
                GoalieDecision::Loss,
                GoalieDecision::Tie,
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
            set.insert(GoalieDecision::Tie);
            set.insert(GoalieDecision::OvertimeLoss);
            assert_eq!(set.len(), 4);
        }
    }

    mod error_display_tests {
        use super::*;

        #[test]
        fn test_unknown_position_error_display() {
            let err = UnknownEnumValue {
                enum_name: "position",
                value: "X".to_string(),
            };
            assert_eq!(format!("{}", err), r#"invalid position: "X""#);
        }

        #[test]
        fn test_unknown_goalie_decision_error_display() {
            let err = UnknownEnumValue {
                enum_name: "goalie decision",
                value: "X".to_string(),
            };
            assert_eq!(format!("{}", err), r#"invalid goalie decision: "X""#);
        }
    }
}
