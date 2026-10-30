//! Playoff brackets, series carousels and series schedules, from the first
//! NHL postseason (1918) to the latest, the 2020 qualifying round, and the
//! 2004-05 lockout.

use super::{season_from_id, web};
use crate::common::Case;
use nhl_api::{PlayoffBracket, PlayoffSeriesCarousel, PlayoffSeriesSchedule};

/// `playoff_bracket!(out, "label", season_id)`: the URL holds the end year.
macro_rules! playoff_bracket {
    ($out:ident, $label:literal, $season:literal) => {
        case!(
            $out,
            PlayoffBracket,
            "playoff_bracket" / $label,
            web(&format!(
                "playoff-bracket/{}",
                season_from_id($season).end_year()
            )),
            |c| c.playoff_bracket(season_from_id($season))
        )
    };
}

/// `playoff_series_carousel!(out, "label", season_id)`.
macro_rules! playoff_series_carousel {
    ($out:ident, $label:literal, $season:literal) => {
        case!(
            $out,
            PlayoffSeriesCarousel,
            "playoff_series_carousel" / $label,
            web(&format!("playoff-series/carousel/{}", $season)),
            |c| c.playoff_series_carousel(season_from_id($season))
        )
    };
}

/// `playoff_series_schedule!(out, "label", season_id, 'letter')`.
macro_rules! playoff_series_schedule {
    ($out:ident, $label:literal, $season:literal, $letter:literal) => {
        case!(
            $out,
            PlayoffSeriesSchedule,
            "playoff_series_schedule" / $label,
            web(&format!("schedule/playoff-series/{}/{}", $season, $letter)),
            |c| c.playoff_series_schedule(season_from_id($season), $letter)
        )
    };
}

#[rustfmt::skip]
pub(super) fn bracket_cases(out: &mut Vec<Case>) {
    playoff_bracket!(out, "19171918", 19171918);
    playoff_bracket!(out, "19251926", 19251926);
    playoff_bracket!(out, "19751976", 19751976);
    playoff_bracket!(out, "19941995", 19941995);
    playoff_bracket!(out, "20042005_no_playoffs", 20042005);
    playoff_bracket!(out, "20102011", 20102011);
    playoff_bracket!(out, "20192020_qualifiers", 20192020);
    playoff_bracket!(out, "20252026", 20252026);
    playoff_bracket!(out, "20262027_not_started", 20262027);
}

/// No case for a season without playoffs or not started yet: both are 404s.
#[rustfmt::skip]
pub(super) fn carousel_cases(out: &mut Vec<Case>) {
    playoff_series_carousel!(out, "19171918", 19171918);
    playoff_series_carousel!(out, "19251926", 19251926);
    playoff_series_carousel!(out, "19751976", 19751976);
    playoff_series_carousel!(out, "19941995", 19941995);
    playoff_series_carousel!(out, "20102011", 20102011);
    playoff_series_carousel!(out, "20192020_qualifiers", 20192020);
    playoff_series_carousel!(out, "20252026", 20252026);
}

#[rustfmt::skip]
pub(super) fn series_schedule_cases(out: &mut Vec<Case>) {
    playoff_series_schedule!(out, "19171918_a_total_goals", 19171918, 'a');
    playoff_series_schedule!(out, "19171918_i_pcha_final", 19171918, 'i');
    playoff_series_schedule!(out, "19251926_a", 19251926, 'a');
    playoff_series_schedule!(out, "19751976_a_best_of_3", 19751976, 'a');
    playoff_series_schedule!(out, "19941995_a", 19941995, 'a');
    playoff_series_schedule!(out, "20102011_a", 20102011, 'a');
    playoff_series_schedule!(out, "20192020_w_qualifier", 20192020, 'w');
    playoff_series_schedule!(out, "20252026_o_final", 20252026, 'o');
}
