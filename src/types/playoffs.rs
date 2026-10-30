//! Playoff bracket, series carousel and series schedule responses.
//!
//! The three endpoints describe the same series from different angles: the
//! bracket (`/playoff-bracket/{year}`) lists every series of a postseason with
//! seeds and team names, the carousel (`/playoff-series/carousel/{season}`)
//! groups the series by round with each seed's wins, and the series schedule
//! (`/schedule/playoff-series/{season}/{letter}`) lists one series' games.
//!
//! While a postseason is in progress, series of later rounds are already
//! listed: the bracket omits their teams and URL, the carousel shows a
//! [`CarouselSeed::is_tbd`] placeholder seed, and neither has a winner yet.

use serde::{Deserialize, Deserializer, Serialize};

use crate::date::Season;
use crate::ids::{GameId, TeamId};

use super::boxscore::PeriodDescriptor;
use super::common::LocalizedString;
use super::enums::GameScheduleState;
use super::game_center::GameOutcome;
use super::game_state::GameState;
use super::game_type::GameType;

/// Team id of the carousel's placeholder seed for a series whose opponent is
/// not decided yet (abbreviation `TBD`).
const TBD_TEAM_ID: i64 = -1;

/// Reads a team id the API sends as `0` (or omits) for an undecided series.
fn decided_team_id<'de, D>(deserializer: D) -> Result<Option<TeamId>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(Option::<TeamId>::deserialize(deserializer)?.filter(|id| id.as_i64() != 0))
}

/// The full bracket of one postseason (`/playoff-bracket/{year}`).
///
/// A year without playoffs (2005) answers `{}`, and one not started yet an
/// empty `series`: both deserialize with no series and no logos.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PlayoffBracket {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bracket_logo: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bracket_logo_fr: Option<String>,
    /// Only sent, empty, before the postseason starts.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bracket_title: Option<LocalizedString>,
    /// Only sent, empty, before the postseason starts.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bracket_sub_title: Option<LocalizedString>,
    #[serde(default)]
    pub series: Vec<BracketSeries>,
}

/// One series of a [`PlayoffBracket`].
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BracketSeries {
    /// Site path of the series page; `None` until both teams are known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub series_url: Option<String>,
    /// e.g. `"1st Round"`, `"Stanley Cup Final"`.
    pub series_title: String,
    /// e.g. `"R1"`, `"SCF"`; `"SCQ"` for the 2020 qualifying round.
    pub series_abbrev: String,
    /// The letter [`crate::Client::playoff_series_schedule`] takes.
    pub series_letter: String,
    /// `0` for the 2020 qualifying round.
    pub playoff_round: i32,
    pub top_seed_rank: i32,
    /// e.g. `"D1"`, `"WC2"`; `None` until the seed is known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub top_seed_rank_abbrev: Option<String>,
    pub top_seed_wins: i32,
    pub bottom_seed_rank: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bottom_seed_rank_abbrev: Option<String>,
    pub bottom_seed_wins: i32,
    /// `None` while the series is undecided.
    #[serde(
        default,
        deserialize_with = "decided_team_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub winning_team_id: Option<TeamId>,
    /// `None` while the series is undecided.
    #[serde(
        default,
        deserialize_with = "decided_team_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub losing_team_id: Option<TeamId>,
    /// `None` until the team is known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub top_seed_team: Option<BracketTeam>,
    /// `None` until the team is known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bottom_seed_team: Option<BracketTeam>,
    /// Only sent for conference finals (`"E"`/`"W"`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conference_abbrev: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conference_name: Option<String>,
}

/// A team in a [`BracketSeries`].
///
/// Brackets served before the 2024-25 season sent only the id, abbreviation,
/// name and logo: the other names then read empty.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BracketTeam {
    pub id: TeamId,
    pub abbrev: String,
    pub name: LocalizedString,
    #[serde(default)]
    pub common_name: LocalizedString,
    #[serde(default)]
    pub place_name_with_preposition: LocalizedString,
    pub logo: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dark_logo: Option<String>,
}

/// Every series of a postseason up to the current round, grouped by round
/// (`/playoff-series/carousel/{season}`).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PlayoffSeriesCarousel {
    pub season_id: Season,
    pub current_round: i32,
    pub rounds: Vec<CarouselRound>,
}

/// One round of a [`PlayoffSeriesCarousel`].
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CarouselRound {
    /// `0` for the 2020 qualifying round.
    pub round_number: i32,
    /// e.g. `"1st-round"`, `"stanley-cup-final"`.
    pub round_label: String,
    /// e.g. `"R1"`, `"SCF"`.
    pub round_abbrev: String,
    pub series: Vec<CarouselSeries>,
}

/// One series of a [`CarouselRound`].
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CarouselSeries {
    /// The letter [`crate::Client::playoff_series_schedule`] takes.
    pub series_letter: String,
    pub round_number: i32,
    pub series_label: String,
    /// Site path of the series page.
    pub series_link: String,
    pub top_seed: CarouselSeed,
    pub bottom_seed: CarouselSeed,
    /// Wins that take the series; `0` for the early two-game total-goals
    /// series.
    pub needed_to_win: i32,
    /// `None` while the series is undecided.
    #[serde(
        default,
        deserialize_with = "decided_team_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub winning_team_id: Option<TeamId>,
    /// `None` while the series is undecided.
    #[serde(
        default,
        deserialize_with = "decided_team_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub losing_team_id: Option<TeamId>,
}

/// A seed of a [`CarouselSeries`].
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CarouselSeed {
    /// `-1` for a seed not decided yet; see [`Self::is_tbd`].
    pub id: TeamId,
    /// `"TBD"` for a seed not decided yet.
    pub abbrev: String,
    pub wins: i32,
    pub logo: String,
    pub dark_logo: String,
}

impl CarouselSeed {
    /// Whether this is the placeholder for an opponent not decided yet.
    pub fn is_tbd(&self) -> bool {
        self.id.as_i64() == TBD_TEAM_ID
    }
}

/// One series and its games (`/schedule/playoff-series/{season}/{letter}`).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PlayoffSeriesSchedule {
    /// `0` for the 2020 qualifying round.
    pub round: i32,
    pub round_abbrev: String,
    pub round_label: String,
    pub series_letter: String,
    /// Wins that take the series; `0` for the early two-game total-goals
    /// series.
    pub needed_to_win: i32,
    /// Most games the series can last.
    pub length: i32,
    pub top_seed_team: PlayoffSeriesTeam,
    pub bottom_seed_team: PlayoffSeriesTeam,
    pub games: Vec<PlayoffSeriesGame>,
}

/// A team of a [`PlayoffSeriesSchedule`].
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PlayoffSeriesTeam {
    pub id: TeamId,
    pub abbrev: String,
    pub name: LocalizedString,
    pub place_name: LocalizedString,
    pub logo: String,
    pub dark_logo: String,
    /// `None` for seasons before conferences (the 1910s-1920s).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conference: Option<PlayoffConference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub division_abbrev: Option<String>,
    /// The team's record in the series, e.g. `"4-2"` (`W-L-T` when the
    /// series had ties).
    pub record: String,
    pub seed: i32,
    pub series_wins: i32,
}

/// Conference of a [`PlayoffSeriesTeam`].
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlayoffConference {
    pub abbrev: String,
    pub name: String,
}

/// A game of a [`PlayoffSeriesSchedule`], including "if necessary" games not
/// played (yet).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PlayoffSeriesGame {
    pub id: GameId,
    pub season: Season,
    pub game_type: GameType,
    pub game_number: i32,
    pub if_necessary: bool,
    #[serde(rename = "startTimeUTC")]
    pub start_time_utc: String,
    pub venue: LocalizedString,
    pub neutral_site: bool,
    pub game_state: GameState,
    /// `TBD` for an "if necessary" game not scheduled yet.
    pub game_schedule_state: GameScheduleState,
    pub away_team: PlayoffGameTeam,
    pub home_team: PlayoffGameTeam,
    /// `None` before puck drop.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub period_descriptor: Option<PeriodDescriptor>,
    /// `None` until the game is final.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub game_outcome: Option<GameOutcome>,
    /// Series wins after this game; `None` before puck drop.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub series_status: Option<SeriesGameStatus>,
}

/// A team of a [`PlayoffSeriesGame`].
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PlayoffGameTeam {
    pub id: TeamId,
    pub abbrev: String,
    /// Empty in responses served before the 2024-25 season, which did not
    /// send it.
    #[serde(default)]
    pub common_name: LocalizedString,
    /// `None` for non-NHL Stanley Cup opponents (e.g. the PCHA's Vancouver
    /// Millionaires in 1918).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub place_name: Option<LocalizedString>,
    /// `None` before puck drop.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score: Option<i32>,
}

/// Series wins after a [`PlayoffSeriesGame`].
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SeriesGameStatus {
    pub top_seed_wins: i32,
    pub bottom_seed_wins: i32,
}

#[cfg(test)]
#[path = "playoffs_test.rs"]
mod tests;
