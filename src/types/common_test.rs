//! Roster deserialization against recorded responses; the helper tests are
//! inline in `common.rs`.

use super::*;

const PROSPECTS: &str = include_str!("../../tests/fixtures/team_prospects/mtl.json");
const PROSPECTS_RELOCATED: &str =
    include_str!("../../tests/fixtures/team_prospects/ari_relocated.json");
const ROSTER_1925: &str = include_str!("../../tests/fixtures/roster_season/mtl_19251926.json");

/// A Canadiens prospect with no NHL number in the recorded pool.
const UNNUMBERED_PROSPECT: i64 = 8484448;

fn roster(json: &str) -> Roster {
    serde_json::from_str(json).unwrap()
}

fn players(roster: &Roster) -> impl Iterator<Item = &RosterPlayer> {
    roster
        .forwards
        .iter()
        .chain(&roster.defensemen)
        .chain(&roster.goalies)
}

#[test]
fn test_prospects_without_sweater_numbers() {
    let prospects = roster(PROSPECTS);
    assert!(!prospects.goalies.is_empty());

    let unnumbered = players(&prospects)
        .find(|player| player.id == PlayerId::new(UNNUMBERED_PROSPECT))
        .unwrap();
    assert_eq!(unnumbered.sweater_number, None);
    assert!(players(&prospects).any(|player| player.sweater_number.is_some()));
}

#[test]
fn test_prospects_of_relocated_team_are_empty() {
    let prospects = roster(PROSPECTS_RELOCATED);
    assert_eq!(players(&prospects).count(), 0);
}

#[test]
fn test_roster_1925_player_without_sweater_number() {
    let roster = roster(ROSTER_1925);
    assert!(players(&roster).any(|player| player.sweater_number.is_none()));
}

#[test]
fn test_roster_player_without_sweater_number_round_trips() {
    let prospects = roster(PROSPECTS);
    let json = serde_json::to_string(&prospects).unwrap();
    assert_eq!(roster(&json), prospects);

    let unnumbered = players(&prospects)
        .find(|player| player.sweater_number.is_none())
        .unwrap();
    let json = serde_json::to_string(unnumbered).unwrap();
    assert!(!json.contains("sweaterNumber"), "omitted when None: {json}");
}
