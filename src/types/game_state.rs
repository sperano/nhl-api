use super::enums::macros::nhl_string_enum;

nhl_string_enum! {
    error_name = "game state",
    display = code,
    /// NHL game state representing the current status of a game
    #[non_exhaustive]
    pub enum GameState {
        /// Future/scheduled game
        Future = "FUT", name = "Future";
        /// Pre-game
        PreGame = "PRE", name = "Pre-Game";
        /// Live/in progress
        Live = "LIVE", name = "Live";
        /// Final/completed
        Final = "FINAL", name = "Final";
        /// Off/completed (alternative to Final)
        Off = "OFF", name = "Off";
        /// Postponed
        Postponed = "PPD", name = "Postponed";
        /// Suspended
        Suspended = "SUSP", name = "Suspended";
        /// Critical (close game, possibly final minutes)
        Critical = "CRIT", name = "Critical";
    }
}

impl GameState {
    /// Returns true if the game has started (live or completed)
    pub fn has_started(&self) -> bool {
        matches!(
            self,
            GameState::Live | GameState::Critical | GameState::Final | GameState::Off
        )
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

#[cfg(test)]
#[path = "game_state_test.rs"]
mod tests;
