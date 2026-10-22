//! The 22 Edge (`/v1/edge/...`) methods for one completed regular season.

use super::{web, CANADIENS_ID, CONNOR_MCDAVID, FILIP_GUSTAVSSON};
use crate::common::Case;
use nhl_api::{
    EdgeGoalie5v5Detail, EdgeGoalieComparison, EdgeGoalieDetail, EdgeGoalieLanding,
    EdgeGoalieSavePctgDetail, EdgeGoalieShotLocationDetail, EdgeSkaterComparison, EdgeSkaterDetail,
    EdgeSkaterDistanceDetail, EdgeSkaterLanding, EdgeSkaterShotLocationDetail,
    EdgeSkaterShotSpeedDetail, EdgeSkaterSpeedDetail, EdgeSkaterZoneTimeDetail, EdgeTeamComparison,
    EdgeTeamDetail, EdgeTeamDistanceDetail, EdgeTeamLanding, EdgeTeamShotLocationDetail,
    EdgeTeamShotSpeedDetail, EdgeTeamSpeedDetail, EdgeTeamZoneTimeDetails, GameType, Season,
};
use reqwest::Url;

/// Start year of the season the Edge cases ask for (2025-26).
const EDGE_SEASON_START: u16 = 2025;
const GAME_TYPE: GameType = GameType::RegularSeason;

fn season() -> Season {
    Season::new(EDGE_SEASON_START)
}

/// `edge/<slug>/<id>/<season>/<game type>`, or with no id for leaderboards.
fn edge_url(slug: &str, id: Option<i64>) -> Url {
    let id = id.map(|id| format!("{id}/")).unwrap_or_default();
    let season = season().to_api_string();
    web(&format!("edge/{slug}/{id}{season}/{}", GAME_TYPE.to_int()))
}

/// `edge!(out, Type, "method" / "label", "slug", id, method)` for a per-player
/// or per-team method; `edge!(out, Type, "method" / "label", "slug", method)`
/// for a league-wide leaderboard.
macro_rules! edge {
    ($out:ident, $ty:ty, $name:literal / $label:literal, $slug:literal, $id:expr, $method:ident) => {
        case!($out, $ty, $name / $label, edge_url($slug, Some($id)), |c| c
            .$method($id, season(), GAME_TYPE))
    };
    ($out:ident, $ty:ty, $name:literal / $label:literal, $slug:literal, $method:ident) => {
        case!($out, $ty, $name / $label, edge_url($slug, None), |c| c
            .$method(season(), GAME_TYPE))
    };
}

// One case per line pair reads as a table; rustfmt would spread each over 8.
#[rustfmt::skip]
pub(super) fn skater_cases(out: &mut Vec<Case>) {
    const P: i64 = CONNOR_MCDAVID;
    edge!(out, EdgeSkaterDetail, "edge_skater_detail" / "mcdavid_20252026_regular",
        "skater-detail", P, edge_skater_detail);
    edge!(out, EdgeSkaterSpeedDetail, "edge_skater_speed_detail" / "mcdavid_20252026_regular",
        "skater-skating-speed-detail", P, edge_skater_speed_detail);
    edge!(out, EdgeSkaterDistanceDetail, "edge_skater_distance_detail" / "mcdavid_20252026_regular",
        "skater-skating-distance-detail", P, edge_skater_distance_detail);
    edge!(out, EdgeSkaterShotSpeedDetail, "edge_skater_shot_speed_detail" / "mcdavid_20252026_regular",
        "skater-shot-speed-detail", P, edge_skater_shot_speed_detail);
    edge!(out, EdgeSkaterShotLocationDetail, "edge_skater_shot_location_detail" / "mcdavid_20252026_regular",
        "skater-shot-location-detail", P, edge_skater_shot_location_detail);
    edge!(out, EdgeSkaterZoneTimeDetail, "edge_skater_zone_time" / "mcdavid_20252026_regular",
        "skater-zone-time", P, edge_skater_zone_time);
    edge!(out, EdgeSkaterComparison, "edge_skater_comparison" / "mcdavid_20252026_regular",
        "skater-comparison", P, edge_skater_comparison);
    edge!(out, EdgeSkaterLanding, "edge_skater_landing" / "20252026_regular",
        "skater-landing", edge_skater_landing);
}

#[rustfmt::skip]
pub(super) fn goalie_cases(out: &mut Vec<Case>) {
    const G: i64 = FILIP_GUSTAVSSON;
    edge!(out, EdgeGoalieDetail, "edge_goalie_detail" / "gustavsson_20252026_regular",
        "goalie-detail", G, edge_goalie_detail);
    edge!(out, EdgeGoalie5v5Detail, "edge_goalie_5v5_detail" / "gustavsson_20252026_regular",
        "goalie-5v5-detail", G, edge_goalie_5v5_detail);
    edge!(out, EdgeGoalieShotLocationDetail, "edge_goalie_shot_location_detail" / "gustavsson_20252026_regular",
        "goalie-shot-location-detail", G, edge_goalie_shot_location_detail);
    edge!(out, EdgeGoalieSavePctgDetail, "edge_goalie_save_pctg_detail" / "gustavsson_20252026_regular",
        "goalie-save-percentage-detail", G, edge_goalie_save_pctg_detail);
    edge!(out, EdgeGoalieComparison, "edge_goalie_comparison" / "gustavsson_20252026_regular",
        "goalie-comparison", G, edge_goalie_comparison);
    edge!(out, EdgeGoalieLanding, "edge_goalie_landing" / "20252026_regular",
        "goalie-landing", edge_goalie_landing);
}

#[rustfmt::skip]
pub(super) fn team_cases(out: &mut Vec<Case>) {
    const T: i64 = CANADIENS_ID;
    edge!(out, EdgeTeamDetail, "edge_team_detail" / "mtl_20252026_regular",
        "team-detail", T, edge_team_detail);
    edge!(out, EdgeTeamSpeedDetail, "edge_team_speed_detail" / "mtl_20252026_regular",
        "team-skating-speed-detail", T, edge_team_speed_detail);
    edge!(out, EdgeTeamDistanceDetail, "edge_team_distance_detail" / "mtl_20252026_regular",
        "team-skating-distance-detail", T, edge_team_distance_detail);
    edge!(out, EdgeTeamShotSpeedDetail, "edge_team_shot_speed_detail" / "mtl_20252026_regular",
        "team-shot-speed-detail", T, edge_team_shot_speed_detail);
    edge!(out, EdgeTeamShotLocationDetail, "edge_team_shot_location_detail" / "mtl_20252026_regular",
        "team-shot-location-detail", T, edge_team_shot_location_detail);
    edge!(out, EdgeTeamZoneTimeDetails, "edge_team_zone_time_details" / "mtl_20252026_regular",
        "team-zone-time-details", T, edge_team_zone_time_details);
    edge!(out, EdgeTeamComparison, "edge_team_comparison" / "mtl_20252026_regular",
        "team-comparison", T, edge_team_comparison);
    edge!(out, EdgeTeamLanding, "edge_team_landing" / "20252026_regular",
        "team-landing", edge_team_landing);
}
