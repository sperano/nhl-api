use super::*;
use crate::types::StatsPage;

const SKATER_CURRENT: &str =
    include_str!("../../tests/fixtures/skater_summary/20242025_regular.json");
const SKATER_1925: &str = include_str!("../../tests/fixtures/skater_summary/19251926_regular.json");
const SKATER_1975_PLAYOFFS: &str =
    include_str!("../../tests/fixtures/skater_summary/19751976_playoffs.json");
const SKATER_AGGREGATE: &str =
    include_str!("../../tests/fixtures/skater_summary/20202021-20232024_aggregate.json");
const SKATER_BY_GAME: &str =
    include_str!("../../tests/fixtures/skater_summary/2024020001_by_game.json");
const SKATER_FORWARDS_SWE: &str =
    include_str!("../../tests/fixtures/skater_summary/20242025_forwards_swe.json");
const GOALIE_CURRENT: &str =
    include_str!("../../tests/fixtures/goalie_summary/20242025_regular.json");
const GOALIE_1997_PLAYOFFS: &str =
    include_str!("../../tests/fixtures/goalie_summary/19971998_playoffs.json");
const GOALIE_2005: &str = include_str!("../../tests/fixtures/goalie_summary/20052006_regular.json");
const GOALIE_BY_GAME: &str =
    include_str!("../../tests/fixtures/goalie_summary/2024020001_by_game.json");
const TEAM_CURRENT: &str = include_str!("../../tests/fixtures/team_summary/20242025_regular.json");
const TEAM_1975_PLAYOFFS: &str =
    include_str!("../../tests/fixtures/team_summary/19751976_playoffs.json");
const TEAM_AGGREGATE: &str =
    include_str!("../../tests/fixtures/team_summary/20202021-20232024_aggregate.json");
const TEAM_BY_GAME: &str =
    include_str!("../../tests/fixtures/team_summary/2024020001_by_game.json");

const OPENER_GAME_ID: i64 = 2024020001;
const OPENER_DATE: &str = "2024-10-04";
/// Skaters of the 2024-25 season in the recorded page of 50.
const SKATER_PAGE_SIZE: usize = 50;
/// Skaters with a regular-season record in 2024-25.
const SKATER_TOTAL: u32 = 920;
/// Teams in the league in 2024-25 (and franchises active 2020-24).
const LEAGUE_TEAMS: usize = 32;

fn skaters(json: &str) -> StatsPage<SkaterSummary> {
    serde_json::from_str(json).unwrap()
}

fn goalies(json: &str) -> StatsPage<GoalieSummary> {
    serde_json::from_str(json).unwrap()
}

fn teams(json: &str) -> StatsPage<TeamSummary> {
    serde_json::from_str(json).unwrap()
}

fn skater<'a>(page: &'a StatsPage<SkaterSummary>, last_name: &str) -> &'a SkaterSummary {
    page.data.iter().find(|s| s.last_name == last_name).unwrap()
}

fn goalie<'a>(page: &'a StatsPage<GoalieSummary>, last_name: &str) -> &'a GoalieSummary {
    page.data.iter().find(|g| g.last_name == last_name).unwrap()
}

// ===== Skaters =====

#[test]
fn test_skater_summary_current_season_row() {
    let page = skaters(SKATER_CURRENT);

    assert_eq!(page.data.len(), SKATER_PAGE_SIZE);
    assert_eq!(page.total, SKATER_TOTAL);
    let kucherov = &page.data[0];
    assert_eq!(kucherov.skater_full_name, "Nikita Kucherov");
    assert_eq!(kucherov.player_id, PlayerId::new(8476453));
    assert_eq!(kucherov.position, Position::RightWing);
    assert_eq!(kucherov.shoots_catches, Some(Handedness::Left));
    assert_eq!(kucherov.season_id, Some(Season::new(2024)));
    assert_eq!(kucherov.team_abbrevs.as_deref(), Some("TBL"));
    assert_eq!(kucherov.points, 121);
    assert_eq!(kucherov.plus_minus, Some(22));
    assert_eq!(kucherov.shots, Some(265));
    assert_eq!(kucherov.time_on_ice_per_game, Some(1271.3717));
    assert_eq!(kucherov.game_id, None);
    assert_eq!(kucherov.team_abbrev, None);
}

#[test]
fn test_skater_summary_1925_row_has_no_modern_stats() {
    let page = skaters(SKATER_1925);

    let stewart = skater(&page, "Stewart");
    assert_eq!(stewart.goals, 34);
    assert_eq!(stewart.position, Position::Center);
    assert_eq!(stewart.team_abbrevs.as_deref(), Some("MMR"));
    assert_eq!(stewart.plus_minus, None);
    assert_eq!(stewart.ev_goals, None);
    assert_eq!(stewart.pp_points, None);
    assert_eq!(stewart.sh_goals, None);
    assert_eq!(stewart.shots, None);
    assert_eq!(stewart.shooting_pct, None);
    assert_eq!(stewart.faceoff_win_pct, None);
    assert_eq!(stewart.time_on_ice_per_game, None);
}

#[test]
fn test_skater_summary_1975_playoff_row_has_no_toi_and_some_no_handedness() {
    let page = skaters(SKATER_1975_PLAYOFFS);

    let leach = skater(&page, "Leach");
    assert_eq!(leach.plus_minus, Some(14));
    assert_eq!(leach.time_on_ice_per_game, None);
    assert_eq!(leach.faceoff_win_pct, None);
    assert_eq!(skater(&page, "Saleski").shoots_catches, None);
}

#[test]
fn test_skater_summary_aggregate_row_drops_season_and_teams() {
    let page = skaters(SKATER_AGGREGATE);

    let mcdavid = skater(&page, "McDavid");
    assert_eq!(mcdavid.season_id, None);
    assert_eq!(mcdavid.team_abbrevs, None);
    assert_eq!(mcdavid.games_played, 294);
    assert_eq!(mcdavid.points, 513);
    assert_eq!(page.total, 1351);
}

#[test]
fn test_skater_summary_by_game_row_has_game_fields() {
    let page = skaters(SKATER_BY_GAME);

    let bastian = skater(&page, "Bastian");
    assert_eq!(bastian.game_id, Some(GameId::new(OPENER_GAME_ID)));
    assert_eq!(bastian.game_date.as_deref(), Some(OPENER_DATE));
    assert_eq!(bastian.home_road, Some(HomeRoad::Road));
    assert_eq!(bastian.opponent_team_abbrev.as_deref(), Some("BUF"));
    assert_eq!(bastian.team_abbrev.as_deref(), Some("NJD"));
    assert_eq!(bastian.season_id, None);
    assert_eq!(bastian.team_abbrevs, None);
    assert_eq!(bastian.games_played, 1);
}

#[test]
fn test_skater_summary_wingers_parse_from_stats_codes() {
    let page = skaters(SKATER_FORWARDS_SWE);

    let positions: Vec<Position> = page.data.iter().map(|s| s.position).collect();
    assert!(positions.contains(&Position::LeftWing));
    assert!(positions.contains(&Position::RightWing));
    assert!(positions.contains(&Position::Center));
    assert!(positions.iter().all(|p| p.is_forward()));
}

#[test]
fn test_skater_summary_shooting_pct_none_without_shots() {
    let page = skaters(SKATER_BY_GAME);

    let no_shot = page.data.iter().find(|s| s.shots == Some(0)).unwrap();
    assert_eq!(no_shot.shooting_pct, None);
}

#[test]
fn test_skater_summary_round_trips_and_omits_none() {
    let page = skaters(SKATER_1925);
    let stewart = skater(&page, "Stewart");

    let json = serde_json::to_value(stewart).unwrap();

    assert!(json.get("plusMinus").is_none());
    assert_eq!(json["positionCode"], serde_json::json!("C"));
    assert_eq!(
        serde_json::from_value::<SkaterSummary>(json).unwrap(),
        *stewart
    );
}

#[test]
fn test_skater_summary_display() {
    let page = skaters(SKATER_CURRENT);
    assert_eq!(
        page.data[0].to_string(),
        "Nikita Kucherov - 78 GP, 37 G, 84 A, 121 PTS"
    );
}

#[test]
fn test_skater_summary_missing_required_field_is_an_error() {
    let err = serde_json::from_str::<SkaterSummary>(r#"{"playerId":1}"#).unwrap_err();
    assert!(err.to_string().contains("missing field"), "{err}");
}

// ===== Goalies =====

#[test]
fn test_goalie_summary_current_season_row() {
    let page = goalies(GOALIE_CURRENT);

    let hellebuyck = goalie(&page, "Hellebuyck");
    assert_eq!(hellebuyck.goalie_full_name, "Connor Hellebuyck");
    assert_eq!(hellebuyck.shoots_catches, Handedness::Left);
    assert_eq!(hellebuyck.wins, 47);
    assert_eq!(hellebuyck.time_on_ice, 224482);
    assert_eq!(hellebuyck.ot_losses, Some(3));
    assert_eq!(hellebuyck.ties, None);
    assert_eq!(hellebuyck.saves, Some(1539));
    assert_eq!(hellebuyck.shots_against, Some(1664));
    assert_eq!(hellebuyck.save_pct, Some(0.92487));
    assert_eq!(hellebuyck.season_id, Some(Season::new(2024)));
    assert_eq!(hellebuyck.team_abbrevs.as_deref(), Some("WPG"));
}

#[test]
fn test_goalie_summary_1997_playoffs_has_ties_not_overtime_losses() {
    let page = goalies(GOALIE_1997_PLAYOFFS);

    let osgood = goalie(&page, "Osgood");
    assert_eq!(osgood.ties, Some(0));
    assert_eq!(osgood.ot_losses, None);
    assert_eq!(osgood.wins, 16);
}

#[test]
fn test_goalie_summary_save_pct_none_without_shots_faced() {
    let page = goalies(GOALIE_1997_PLAYOFFS);

    let billington = goalie(&page, "Billington");
    assert_eq!(billington.shots_against, Some(0));
    assert_eq!(billington.save_pct, None);
}

#[test]
fn test_goalie_summary_2005_row_has_overtime_losses_not_ties() {
    let page = goalies(GOALIE_2005);

    let brodeur = goalie(&page, "Brodeur");
    assert_eq!(brodeur.ties, None);
    assert_eq!(brodeur.ot_losses, Some(7));
}

#[test]
fn test_goalie_summary_by_game_row_has_game_fields() {
    let page = goalies(GOALIE_BY_GAME);

    let markstrom = goalie(&page, "Markstrom");
    assert_eq!(markstrom.game_id, Some(GameId::new(OPENER_GAME_ID)));
    assert_eq!(markstrom.game_date.as_deref(), Some(OPENER_DATE));
    assert_eq!(markstrom.home_road, Some(HomeRoad::Road));
    assert_eq!(markstrom.team_abbrev.as_deref(), Some("NJD"));
    assert_eq!(markstrom.season_id, None);
    assert_eq!(markstrom.team_abbrevs, None);
}

#[test]
fn test_goalie_summary_display_with_and_without_save_pct() {
    let page = goalies(GOALIE_1997_PLAYOFFS);

    assert_eq!(
        goalie(&page, "Osgood").to_string(),
        "Chris Osgood - 22 GP, 16-6, 2.12 GAA, 0.918 SV%"
    );
    assert_eq!(
        goalie(&page, "Billington").to_string(),
        "Craig Billington - 1 GP, 0-0, 0.00 GAA"
    );
}

#[test]
fn test_goalie_summary_round_trips() {
    let page = goalies(GOALIE_1997_PLAYOFFS);
    let osgood = goalie(&page, "Osgood");

    let json = serde_json::to_value(osgood).unwrap();

    assert!(json.get("otLosses").is_none());
    assert_eq!(
        serde_json::from_value::<GoalieSummary>(json).unwrap(),
        *osgood
    );
}

// ===== Teams =====

#[test]
fn test_team_summary_current_season_row() {
    let page = teams(TEAM_CURRENT);

    assert_eq!(page.data.len(), LEAGUE_TEAMS);
    let jets = &page.data[0];
    assert_eq!(jets.team_full_name.as_deref(), Some("Winnipeg Jets"));
    assert_eq!(jets.team_id, Some(TeamId::new(52)));
    assert_eq!(jets.season_id, Some(Season::new(2024)));
    assert_eq!(jets.points, 116);
    assert_eq!(jets.ot_losses, Some(4));
    assert_eq!(jets.ties, None);
    assert_eq!(jets.power_play_pct, Some(0.28899));
    assert_eq!(jets.franchise_id, None);
    assert_eq!(jets.name(), Some("Winnipeg Jets"));
}

#[test]
fn test_team_summary_1975_playoff_row_has_no_special_teams() {
    let page = teams(TEAM_1975_PLAYOFFS);

    let canadiens = page
        .data
        .iter()
        .find(|t| t.team_id == Some(TeamId::new(8)))
        .unwrap();
    assert_eq!(canadiens.wins, 12);
    assert_eq!(canadiens.ot_losses, None);
    assert_eq!(canadiens.faceoff_win_pct, None);
    assert_eq!(canadiens.power_play_pct, None);
    assert_eq!(canadiens.power_play_net_pct, None);
    assert_eq!(canadiens.penalty_kill_pct, None);
    assert_eq!(canadiens.penalty_kill_net_pct, None);
    assert_eq!(canadiens.shots_for_per_game, Some(30.84615));
}

#[test]
fn test_team_summary_aggregate_row_has_franchise_instead_of_team() {
    let page = teams(TEAM_AGGREGATE);

    assert_eq!(page.data.len(), LEAGUE_TEAMS);
    let bruins = page
        .data
        .iter()
        .find(|t| t.franchise_id == Some(FranchiseId::new(6)))
        .unwrap();
    assert_eq!(bruins.franchise_name.as_deref(), Some("Boston Bruins"));
    assert_eq!(bruins.team_id, None);
    assert_eq!(bruins.team_full_name, None);
    assert_eq!(bruins.season_id, None);
    assert_eq!(bruins.games_played, 302);
    assert_eq!(bruins.name(), Some("Boston Bruins"));
}

#[test]
fn test_team_summary_by_game_row_has_game_fields() {
    let page = teams(TEAM_BY_GAME);

    let devils = page
        .data
        .iter()
        .find(|t| t.team_full_name.as_deref() == Some("New Jersey Devils"))
        .unwrap();
    assert_eq!(devils.game_id, Some(GameId::new(OPENER_GAME_ID)));
    assert_eq!(devils.game_date.as_deref(), Some(OPENER_DATE));
    assert_eq!(devils.home_road, Some(HomeRoad::Road));
    assert_eq!(devils.opponent_team_abbrev.as_deref(), Some("BUF"));
    assert_eq!(devils.season_id, None);
    assert_eq!(devils.goals_for, 4);
}

#[test]
fn test_team_summary_display_and_name_fallback() {
    let page = teams(TEAM_CURRENT);
    assert_eq!(
        page.data[0].to_string(),
        "Winnipeg Jets - 82 GP, 56 W, 22 L, 116 PTS"
    );

    let mut nameless = page.data[0].clone();
    nameless.team_full_name = None;
    assert_eq!(nameless.name(), None);
    assert!(nameless.to_string().starts_with("Unknown - "));
}

#[test]
fn test_team_summary_round_trips() {
    let page = teams(TEAM_AGGREGATE);
    let row = &page.data[0];

    let json = serde_json::to_value(row).unwrap();

    assert!(json.get("teamId").is_none());
    assert_eq!(serde_json::from_value::<TeamSummary>(json).unwrap(), *row);
}
