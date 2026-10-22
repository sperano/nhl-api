//! Recorded responses that deserialize, but whose type drops data.
//!
//! Every type ignores some of the fields the API sends (translations, logos,
//! links), so the contract tests do not fail on dropped fields in general.
//! This ledger lists the ones that are bugs: a value the type has a field for
//! under another name or shape, so that field silently reads its default (a
//! zero, `false`, `None`), or a whole section of the response the method
//! exists to return. The contract tests check that each listed field is still
//! dropped, so they turn red when the type is fixed. Fixing a bug means
//! deleting its paths here.
//!
//! Not detected this way: fields the API does not send at all, but that the
//! type reads as a default anyway; fields inside values a custom `Deserialize`
//! buffers through `serde_json::Value` or an untagged enum before reading them
//! (`GameLog` entries, `ThreeStar.name`); and cases listed in
//! `KNOWN_FAILURES`, whose deserialization stops at the error.

pub struct KnownDropped {
    /// `<method>/<label>`, as in [`super::Case::name`].
    pub case: &'static str,
    /// Fields in the [`super::DroppedFields`] notation.
    pub fields: &'static [&'static str],
}

pub const KNOWN_DROPPED: &[KnownDropped] = &[
    // `PlayerLanding.last_five_games` expects `lastFiveGames`; the API sends
    // `last5Games`, whose entries also lack fields `GameLog` requires
    // (`powerPlayPoints`; `shutouts`, `goals` and `assists` for goalies).
    dropped("player_landing/mcdavid", &["last5Games"]),
    dropped("player_landing/price_goalie", &["last5Games"]),
    dropped("player_landing/morenz", &["last5Games"]),
    dropped("player_landing/vezina_goalie", &["last5Games"]),
    // Speeds come as `skatingSpeed`/`shotSpeed`, not `speed`, and the
    // per-game distance entries as `distanceSkatedAll` (plus even-strength,
    // power-play and penalty-kill splits with their TOI), not `distance`.
    // Every one of these responses also has a `*Details` summary section.
    dropped(
        "edge_skater_speed_detail/mcdavid_20252026_regular",
        &["topSkatingSpeeds.[].skatingSpeed", "skatingSpeedDetails"],
    ),
    dropped(
        "edge_skater_distance_detail/mcdavid_20252026_regular",
        &[
            "skatingDistanceLast10.[].distanceSkatedAll",
            "skatingDistanceDetails",
        ],
    ),
    dropped(
        "edge_skater_shot_speed_detail/mcdavid_20252026_regular",
        &["hardestShots.[].shotSpeed", "shotSpeedDetails"],
    ),
    dropped(
        "edge_skater_zone_time/mcdavid_20252026_regular",
        &["zoneStarts"],
    ),
    dropped(
        "edge_goalie_5v5_detail/gustavsson_20252026_regular",
        &["savePctg5v5Details"],
    ),
    dropped(
        "edge_goalie_shot_location_detail/gustavsson_20252026_regular",
        &["shotLocationTotals"],
    ),
    // `distanceSkated.total` is a measurement, not the integer `value`.
    dropped(
        "edge_team_detail/mtl_20252026_regular",
        &[
            "distanceSkated.total.imperial",
            "distanceSkated.total.metric",
        ],
    ),
    dropped(
        "edge_team_speed_detail/mtl_20252026_regular",
        &["topSkatingSpeeds.[].skatingSpeed", "skatingSpeedDetails"],
    ),
    dropped(
        "edge_team_distance_detail/mtl_20252026_regular",
        &[
            "skatingDistanceLast10.[].distanceSkatedAll",
            "skatingDistanceDetails",
        ],
    ),
    dropped(
        "edge_team_shot_speed_detail/mtl_20252026_regular",
        &["hardestShots.[].shotSpeed", "shotSpeedDetails"],
    ),
    // Shots on goal come as `sog`, not `shots`.
    dropped(
        "edge_team_shot_location_detail/mtl_20252026_regular",
        &["shotLocationDetails.[].sog", "shotLocationTotals"],
    ),
    // Team responses say `isHomeTeam` where `player_on_home_team` expects
    // `playerOnHomeTeam`.
    dropped(
        "edge_team_comparison/mtl_20252026_regular",
        &["skatingDistanceLast10.[].isHomeTeam"],
    ),
];

const fn dropped(case: &'static str, fields: &'static [&'static str]) -> KnownDropped {
    KnownDropped { case, fields }
}
