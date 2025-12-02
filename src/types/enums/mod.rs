//! Shared enums for NHL API types
//!
//! This module re-exports all enum types from their logical groupings:
//! - `player_enums`: Player-related enums (Position, Handedness, GoalieDecision)
//! - `game_enums`: Game/play-related enums (PeriodType, HomeRoad, ZoneCode, DefendingSide, GameScheduleState)

mod game_enums;
mod player_enums;

pub use game_enums::*;
pub use player_enums::*;
