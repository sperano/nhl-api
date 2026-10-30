//! Franchises, rosters and club stats.

use super::{season_from_id, url, web, CANADIENS, STATS_BASE};
use crate::common::Case;
use nhl_api::{ClubStats, FranchisesResponse, GameType, Roster, SeasonGameTypes};

const REGULAR: GameType = GameType::RegularSeason;
const PLAYOFFS: GameType = GameType::Playoffs;

/// `roster_season!(out, "label", season_id)`.
macro_rules! roster_season {
    ($out:ident, $label:literal, $season:literal) => {
        case!(
            $out,
            Roster,
            "roster_season" / $label,
            web(&format!("roster/{CANADIENS}/{}", $season)),
            |c| c.roster_season(CANADIENS, season_from_id($season))
        )
    };
}

/// `club_stats!(out, "label", season_id, game_type)`.
macro_rules! club_stats {
    ($out:ident, $label:literal, $season:literal, $game_type:expr) => {
        case!(
            $out,
            ClubStats,
            "club_stats" / $label,
            web(&format!(
                "club-stats/{CANADIENS}/{}/{}",
                $season,
                $game_type.to_int()
            )),
            |c| c.club_stats(CANADIENS, season_from_id($season), $game_type)
        )
    };
}

pub(super) fn roster_cases(out: &mut Vec<Case>) {
    case!(
        out,
        FranchisesResponse,
        "franchises" / "all",
        url(STATS_BASE, "en/franchise"),
        |c| c.franchises()
    );
    case!(
        out,
        Roster,
        "roster_current" / "mtl",
        web(&format!("roster/{CANADIENS}/current")),
        |c| c.roster_current(CANADIENS)
    );
    roster_season!(out, "mtl_20102011", 20102011);
    roster_season!(out, "mtl_19751976", 19751976);
    roster_season!(out, "mtl_19251926", 19251926);
}

pub(super) fn club_stats_cases(out: &mut Vec<Case>) {
    club_stats!(out, "mtl_20252026_regular", 20252026, REGULAR);
    club_stats!(out, "mtl_20242025_playoffs", 20242025, PLAYOFFS);
    club_stats!(out, "mtl_19801981_regular", 19801981, REGULAR);
    club_stats!(out, "mtl_19251926_regular", 19251926, REGULAR);
    case!(
        out,
        Vec<SeasonGameTypes>,
        "club_stats_season" / "mtl",
        web(&format!("club-stats-season/{CANADIENS}")),
        |c| c.club_stats_season(CANADIENS)
    );
}
