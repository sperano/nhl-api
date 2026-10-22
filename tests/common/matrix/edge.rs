//! The 22 Edge (`/v1/edge/...`) methods for one completed regular season.

use super::{web, Case, CANADIENS_ID, CONNOR_MCDAVID, FILIP_GUSTAVSSON};
use nhl_api::{
    EdgeGoalie5v5Detail, EdgeGoalieComparison, EdgeGoalieDetail, EdgeGoalieLanding,
    EdgeGoalieSavePctgDetail, EdgeGoalieShotLocationDetail, EdgeSkaterComparison, EdgeSkaterDetail,
    EdgeSkaterDistanceDetail, EdgeSkaterLanding, EdgeSkaterShotLocationDetail,
    EdgeSkaterShotSpeedDetail, EdgeSkaterSpeedDetail, EdgeSkaterZoneTimeDetail, EdgeTeamComparison,
    EdgeTeamDetail, EdgeTeamDistanceDetail, EdgeTeamLanding, EdgeTeamShotLocationDetail,
    EdgeTeamShotSpeedDetail, EdgeTeamSpeedDetail, EdgeTeamZoneTimeDetails, GameType, Season,
};

/// Start year of the season the Edge cases ask for (2025-26).
const EDGE_SEASON_START: u16 = 2025;

fn season() -> Season {
    Season::new(EDGE_SEASON_START)
}

/// `edge/<slug>/<id>/20252026/2`, or `edge/<slug>/20252026/2` with no id.
fn edge_url(slug: &str, id: Option<i64>) -> super::Url {
    let id = id.map(|id| format!("{id}/")).unwrap_or_default();
    web(&format!("edge/{slug}/{id}{}/2", season().to_api_string()))
}

pub(super) fn skater_cases(out: &mut Vec<Case>) {
    const P: i64 = CONNOR_MCDAVID;
    const RS: GameType = GameType::RegularSeason;
    case!(
        out,
        EdgeSkaterDetail,
        "edge_skater_detail" / "mcdavid_20252026_regular",
        edge_url("skater-detail", Some(P)),
        |c| c.edge_skater_detail(P, season(), RS)
    );
    case!(
        out,
        EdgeSkaterSpeedDetail,
        "edge_skater_speed_detail" / "mcdavid_20252026_regular",
        edge_url("skater-skating-speed-detail", Some(P)),
        |c| c.edge_skater_speed_detail(P, season(), RS)
    );
    case!(
        out,
        EdgeSkaterDistanceDetail,
        "edge_skater_distance_detail" / "mcdavid_20252026_regular",
        edge_url("skater-skating-distance-detail", Some(P)),
        |c| c.edge_skater_distance_detail(P, season(), RS)
    );
    case!(
        out,
        EdgeSkaterShotSpeedDetail,
        "edge_skater_shot_speed_detail" / "mcdavid_20252026_regular",
        edge_url("skater-shot-speed-detail", Some(P)),
        |c| c.edge_skater_shot_speed_detail(P, season(), RS)
    );
    case!(
        out,
        EdgeSkaterShotLocationDetail,
        "edge_skater_shot_location_detail" / "mcdavid_20252026_regular",
        edge_url("skater-shot-location-detail", Some(P)),
        |c| c.edge_skater_shot_location_detail(P, season(), RS)
    );
    case!(
        out,
        EdgeSkaterZoneTimeDetail,
        "edge_skater_zone_time" / "mcdavid_20252026_regular",
        edge_url("skater-zone-time", Some(P)),
        |c| c.edge_skater_zone_time(P, season(), RS)
    );
    case!(
        out,
        EdgeSkaterComparison,
        "edge_skater_comparison" / "mcdavid_20252026_regular",
        edge_url("skater-comparison", Some(P)),
        |c| c.edge_skater_comparison(P, season(), RS)
    );
    case!(
        out,
        EdgeSkaterLanding,
        "edge_skater_landing" / "20252026_regular",
        edge_url("skater-landing", None),
        |c| c.edge_skater_landing(season(), RS)
    );
}

pub(super) fn goalie_cases(out: &mut Vec<Case>) {
    const G: i64 = FILIP_GUSTAVSSON;
    const RS: GameType = GameType::RegularSeason;
    case!(
        out,
        EdgeGoalieDetail,
        "edge_goalie_detail" / "gustavsson_20252026_regular",
        edge_url("goalie-detail", Some(G)),
        |c| c.edge_goalie_detail(G, season(), RS)
    );
    case!(
        out,
        EdgeGoalie5v5Detail,
        "edge_goalie_5v5_detail" / "gustavsson_20252026_regular",
        edge_url("goalie-5v5-detail", Some(G)),
        |c| c.edge_goalie_5v5_detail(G, season(), RS)
    );
    case!(
        out,
        EdgeGoalieShotLocationDetail,
        "edge_goalie_shot_location_detail" / "gustavsson_20252026_regular",
        edge_url("goalie-shot-location-detail", Some(G)),
        |c| c.edge_goalie_shot_location_detail(G, season(), RS)
    );
    case!(
        out,
        EdgeGoalieSavePctgDetail,
        "edge_goalie_save_pctg_detail" / "gustavsson_20252026_regular",
        edge_url("goalie-save-percentage-detail", Some(G)),
        |c| c.edge_goalie_save_pctg_detail(G, season(), RS)
    );
    case!(
        out,
        EdgeGoalieComparison,
        "edge_goalie_comparison" / "gustavsson_20252026_regular",
        edge_url("goalie-comparison", Some(G)),
        |c| c.edge_goalie_comparison(G, season(), RS)
    );
    case!(
        out,
        EdgeGoalieLanding,
        "edge_goalie_landing" / "20252026_regular",
        edge_url("goalie-landing", None),
        |c| c.edge_goalie_landing(season(), RS)
    );
}

pub(super) fn team_cases(out: &mut Vec<Case>) {
    const T: i64 = CANADIENS_ID;
    const RS: GameType = GameType::RegularSeason;
    case!(
        out,
        EdgeTeamDetail,
        "edge_team_detail" / "mtl_20252026_regular",
        edge_url("team-detail", Some(T)),
        |c| c.edge_team_detail(T, season(), RS)
    );
    case!(
        out,
        EdgeTeamSpeedDetail,
        "edge_team_speed_detail" / "mtl_20252026_regular",
        edge_url("team-skating-speed-detail", Some(T)),
        |c| c.edge_team_speed_detail(T, season(), RS)
    );
    case!(
        out,
        EdgeTeamDistanceDetail,
        "edge_team_distance_detail" / "mtl_20252026_regular",
        edge_url("team-skating-distance-detail", Some(T)),
        |c| c.edge_team_distance_detail(T, season(), RS)
    );
    case!(
        out,
        EdgeTeamShotSpeedDetail,
        "edge_team_shot_speed_detail" / "mtl_20252026_regular",
        edge_url("team-shot-speed-detail", Some(T)),
        |c| c.edge_team_shot_speed_detail(T, season(), RS)
    );
    case!(
        out,
        EdgeTeamShotLocationDetail,
        "edge_team_shot_location_detail" / "mtl_20252026_regular",
        edge_url("team-shot-location-detail", Some(T)),
        |c| c.edge_team_shot_location_detail(T, season(), RS)
    );
    case!(
        out,
        EdgeTeamZoneTimeDetails,
        "edge_team_zone_time_details" / "mtl_20252026_regular",
        edge_url("team-zone-time-details", Some(T)),
        |c| c.edge_team_zone_time_details(T, season(), RS)
    );
    case!(
        out,
        EdgeTeamComparison,
        "edge_team_comparison" / "mtl_20252026_regular",
        edge_url("team-comparison", Some(T)),
        |c| c.edge_team_comparison(T, season(), RS)
    );
    case!(
        out,
        EdgeTeamLanding,
        "edge_team_landing" / "20252026_regular",
        edge_url("team-landing", None),
        |c| c.edge_team_landing(season(), RS)
    );
}
