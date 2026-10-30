use super::*;
use crate::types::enums::UnknownEnumValue;

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

#[test]
fn test_game_state_display_all_variants() {
    assert_eq!(GameState::PreGame.to_string(), "PRE");
    assert_eq!(GameState::Off.to_string(), "OFF");
    assert_eq!(GameState::Postponed.to_string(), "PPD");
    assert_eq!(GameState::Suspended.to_string(), "SUSP");
    assert_eq!(GameState::Critical.to_string(), "CRIT");
}

#[test]
fn test_game_state_from_str_all_variants() {
    assert_eq!("PRE".parse::<GameState>().unwrap(), GameState::PreGame);
    assert_eq!("OFF".parse::<GameState>().unwrap(), GameState::Off);
    assert_eq!("PPD".parse::<GameState>().unwrap(), GameState::Postponed);
    assert_eq!("SUSP".parse::<GameState>().unwrap(), GameState::Suspended);
    assert_eq!("CRIT".parse::<GameState>().unwrap(), GameState::Critical);
}

#[test]
fn test_game_state_from_str_invalid() {
    let result = "INVALID".parse::<GameState>();
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        UnknownEnumValue {
            enum_name: "game state",
            value: "INVALID".to_string(),
        }
    );
}

#[test]
fn test_game_state_from_str_lowercase() {
    // Ensure case matters
    let result = "live".parse::<GameState>();
    assert!(result.is_err());
}

#[test]
fn test_has_started_critical() {
    assert!(GameState::Critical.has_started());
}

#[test]
fn test_has_started_postponed_suspended() {
    assert!(!GameState::Postponed.has_started());
    assert!(!GameState::Suspended.has_started());
}

#[test]
fn test_is_live() {
    assert!(GameState::Live.is_live());
    assert!(GameState::Critical.is_live());
    assert!(!GameState::Future.is_live());
    assert!(!GameState::PreGame.is_live());
    assert!(!GameState::Final.is_live());
    assert!(!GameState::Off.is_live());
}

#[test]
fn test_is_live_with_postponed_suspended() {
    assert!(!GameState::Postponed.is_live());
    assert!(!GameState::Suspended.is_live());
}

#[test]
fn test_serde_serialization_all_variants() {
    assert_eq!(
        serde_json::to_string(&GameState::Future).unwrap(),
        r#""FUT""#
    );
    assert_eq!(
        serde_json::to_string(&GameState::PreGame).unwrap(),
        r#""PRE""#
    );
    assert_eq!(
        serde_json::to_string(&GameState::Final).unwrap(),
        r#""FINAL""#
    );
    assert_eq!(serde_json::to_string(&GameState::Off).unwrap(), r#""OFF""#);
    assert_eq!(
        serde_json::to_string(&GameState::Postponed).unwrap(),
        r#""PPD""#
    );
    assert_eq!(
        serde_json::to_string(&GameState::Suspended).unwrap(),
        r#""SUSP""#
    );
    assert_eq!(
        serde_json::to_string(&GameState::Critical).unwrap(),
        r#""CRIT""#
    );
}

#[test]
fn test_serde_deserialization_all_variants() {
    assert_eq!(
        serde_json::from_str::<GameState>(r#""FUT""#).unwrap(),
        GameState::Future
    );
    assert_eq!(
        serde_json::from_str::<GameState>(r#""PRE""#).unwrap(),
        GameState::PreGame
    );
    assert_eq!(
        serde_json::from_str::<GameState>(r#""FINAL""#).unwrap(),
        GameState::Final
    );
    assert_eq!(
        serde_json::from_str::<GameState>(r#""OFF""#).unwrap(),
        GameState::Off
    );
    assert_eq!(
        serde_json::from_str::<GameState>(r#""PPD""#).unwrap(),
        GameState::Postponed
    );
    assert_eq!(
        serde_json::from_str::<GameState>(r#""SUSP""#).unwrap(),
        GameState::Suspended
    );
    assert_eq!(
        serde_json::from_str::<GameState>(r#""CRIT""#).unwrap(),
        GameState::Critical
    );
}

#[test]
fn test_game_state_hash() {
    use std::collections::HashSet;
    let mut set = HashSet::new();
    set.insert(GameState::Future);
    set.insert(GameState::Live);
    set.insert(GameState::Final);

    assert!(set.contains(&GameState::Future));
    assert!(set.contains(&GameState::Live));
    assert!(set.contains(&GameState::Final));
    assert!(!set.contains(&GameState::PreGame));
}

#[test]
fn test_game_state_equality() {
    assert_eq!(GameState::Future, GameState::Future);
    assert_ne!(GameState::Future, GameState::Live);
    assert_eq!(GameState::Final, GameState::Final);
    assert_ne!(GameState::Final, GameState::Off);
}

#[test]
fn test_game_state_code_and_name() {
    assert_eq!(GameState::PreGame.code(), "PRE");
    assert_eq!(GameState::PreGame.name(), "Pre-Game");
    assert_eq!(GameState::Critical.name(), "Critical");
}

#[test]
fn test_game_state_deserialize_unknown_error_message() {
    let err = serde_json::from_str::<GameState>(r#""NOPE""#).unwrap_err();
    assert!(err.to_string().contains(r#"invalid game state: "NOPE""#));
}
