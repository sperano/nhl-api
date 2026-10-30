use std::fmt;

use serde::{Deserialize, Serialize};

/// Type of play event in a game.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub enum PlayEventType {
    #[serde(rename = "game-start")]
    GameStart,
    #[serde(rename = "period-start")]
    PeriodStart,
    #[serde(rename = "period-end")]
    PeriodEnd,
    #[serde(rename = "game-end")]
    GameEnd,
    Faceoff,
    Hit,
    Giveaway,
    Takeaway,
    #[serde(rename = "shot-on-goal")]
    ShotOnGoal,
    #[serde(rename = "missed-shot")]
    MissedShot,
    #[serde(rename = "blocked-shot")]
    BlockedShot,
    Goal,
    Penalty,
    Stoppage,
    #[serde(rename = "delayed-penalty")]
    DelayedPenalty,
    #[serde(rename = "failed-shot-attempt")]
    FailedShotAttempt,
    #[serde(rename = "shootout-complete")]
    ShootoutComplete,
    #[serde(other)]
    Unknown,
}

impl PlayEventType {
    /// Returns true if this event is a scoring chance (shot, goal, missed shot, blocked shot).
    pub fn is_scoring_chance(&self) -> bool {
        matches!(
            self,
            Self::ShotOnGoal | Self::Goal | Self::MissedShot | Self::BlockedShot
        )
    }

    /// Returns true if this event affects the score.
    pub fn is_goal(&self) -> bool {
        matches!(self, Self::Goal)
    }

    /// Returns true if this is a period boundary event.
    pub fn is_period_boundary(&self) -> bool {
        matches!(
            self,
            Self::PeriodStart | Self::PeriodEnd | Self::GameStart | Self::GameEnd
        )
    }
}

impl fmt::Display for PlayEventType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let display = match self {
            Self::GameStart => "Game Start",
            Self::PeriodStart => "Period Start",
            Self::PeriodEnd => "Period End",
            Self::GameEnd => "Game End",
            Self::Faceoff => "Faceoff",
            Self::Hit => "Hit",
            Self::Giveaway => "Giveaway",
            Self::Takeaway => "Takeaway",
            Self::ShotOnGoal => "Shot on Goal",
            Self::MissedShot => "Missed Shot",
            Self::BlockedShot => "Blocked Shot",
            Self::Goal => "Goal",
            Self::Penalty => "Penalty",
            Self::Stoppage => "Stoppage",
            Self::DelayedPenalty => "Delayed Penalty",
            Self::FailedShotAttempt => "Failed Shot Attempt",
            Self::ShootoutComplete => "Shootout Complete",
            Self::Unknown => "Unknown",
        };
        formatter.write_str(display)
    }
}
