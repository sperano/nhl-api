/// Common types shared across the API
pub mod common;
/// Boxscore and player statistics types
pub mod boxscore;
/// Game center types (play-by-play, matchup, etc.)
pub mod game_center;
/// Schedule and game types
pub mod schedule;
/// Standings and season types
pub mod standings;

// Re-export all types for backward compatibility
pub use common::*;
pub use boxscore::*;
pub use game_center::*;
pub use schedule::*;
pub use standings::*;
