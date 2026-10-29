//! Cases that fail today because of a deserialization bug not fixed yet.
//!
//! Each entry pins the error the case fails with, so the contract tests (and,
//! for cases with no recorded fixture, the live sweep) stay green while the
//! bug is open and turn red as soon as it is fixed (the case starts passing)
//! or its failure changes. Fixing a bug means deleting its entries here.

pub struct KnownFailure {
    /// `<method>/<label>`, as in [`super::Case::name`].
    pub case: &'static str,
    /// A substring of the error the case fails with.
    pub error: &'static str,
}

pub const KNOWN_FAILURES: &[KnownFailure] = &[
    // Scheduled games: no `score` before puck drop, including the unplayed
    // games listed in a season series, and no `periodDescriptor` in play-by-play.
    known("boxscore/fut", "missing field `score`"),
    known("play_by_play/fut", "missing field `periodDescriptor`"),
    known("landing/fut", "missing field `score`"),
    known("season_series/fut", "missing field `score`"),
    known("game_story/fut", "missing field `score`"),
    // Its series has unplayed games until 2027-02-24; after that the live
    // response passes and the fixture needs re-recording before this goes.
    known("season_series/regular_20262027", "missing field `score`"),
    // The first week on record has no previous week.
    known(
        "weekly_schedule/1917-12-19",
        "missing field `previousStartDate`",
    ),
    // Historical game logs lack modern stats; the 1920s goalie log has no
    // `shotsAgainst`, so it is read as a skater log.
    known(
        "player_game_log/orr_19701971_regular",
        "missing field `shifts`",
    ),
    known(
        "player_game_log/morenz_19271928_regular",
        "missing field `plusMinus`",
    ),
    known(
        "player_game_log/vezina_19241925_regular",
        "missing field `points`",
    ),
    // 1920s rosters without sweater numbers.
    known(
        "roster_season/mtl_19251926",
        "missing field `sweaterNumber`",
    ),
    // Club stats rows without era- or playoff-dependent fields.
    known(
        "club_stats/mtl_20242025_playoffs",
        "missing field `penaltyMinutes`",
    ),
    known(
        "club_stats/mtl_19801981_regular",
        "missing field `avgTimeOnIcePerGame`",
    ),
    known(
        "club_stats/mtl_19251926_regular",
        "missing field `plusMinus`",
    ),
    // `shootingPctg` is null for an area with no shots on goal.
    known(
        "edge_skater_shot_location_detail/mcdavid_20252026_regular",
        "invalid type: null, expected f64",
    ),
];

const fn known(case: &'static str, error: &'static str) -> KnownFailure {
    KnownFailure { case, error }
}

pub fn known_failure(case: &str) -> Option<&'static KnownFailure> {
    KNOWN_FAILURES.iter().find(|known| known.case == case)
}
