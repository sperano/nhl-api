//! Team faceoff totals counted from play-by-play `faceoff` events.
//!
//! The boxscore only carries each skater's `faceoffWinningPctg`, not how many
//! faceoffs each skater took, so team totals cannot be derived from it.
//! Play-by-play records every faceoff with the winning team as
//! `eventOwnerTeamId`, which gives exact counts whatever the position of the
//! player who took it.

use crate::ids::TeamId;

use super::game_center::{PlayByPlay, PlayEvent, PlayEventType};

/// Faceoffs won and taken by one team in one game
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct FaceoffTotals {
    pub wins: i32,
    pub total: i32,
}

impl FaceoffTotals {
    /// Faceoff winning percentage (0-100), or `None` when no faceoff was taken
    pub fn percentage(&self) -> Option<f64> {
        (self.total > 0).then(|| f64::from(self.wins) / f64::from(self.total) * 100.0)
    }
}

/// Faceoff totals for both teams of a game
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct GameFaceoffs {
    pub away: FaceoffTotals,
    pub home: FaceoffTotals,
}

impl PlayByPlay {
    /// Count both teams' faceoffs from the play-by-play `faceoff` events.
    ///
    /// A faceoff whose `eventOwnerTeamId` is missing or names neither team
    /// has no known winner and is left out of both teams' totals, so
    /// `away.wins + home.wins == away.total == home.total` always holds.
    pub fn faceoff_totals(&self) -> GameFaceoffs {
        let mut faceoffs = GameFaceoffs::default();
        for winner in self.plays.iter().filter_map(faceoff_winner) {
            if winner == self.away_team.id {
                faceoffs.away.wins += 1;
            } else if winner == self.home_team.id {
                faceoffs.home.wins += 1;
            } else {
                continue;
            }
            faceoffs.away.total += 1;
            faceoffs.home.total += 1;
        }
        faceoffs
    }
}

/// Winning team of a faceoff event, `None` for any other event
fn faceoff_winner(play: &PlayEvent) -> Option<TeamId> {
    if play.type_desc_key != PlayEventType::Faceoff {
        return None;
    }
    play.details.as_ref()?.event_owner_team_id
}

#[cfg(test)]
#[path = "faceoffs_test.rs"]
mod tests;
