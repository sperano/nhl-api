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
