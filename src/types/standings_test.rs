use super::{Standing, StandingsResponse};

const HISTORICAL_STANDINGS_JSON: &str =
    include_str!("../../tests/fixtures/league_standings_for_date/1925-03-01.json");

#[test]
fn test_standings_historical_fixture_converts_team() {
    let response: StandingsResponse = serde_json::from_str(HISTORICAL_STANDINGS_JSON).unwrap();
    let standing: &Standing = &response.standings[0];

    assert_eq!(standing.team_abbrev.default, "HAM");
    assert_eq!(standing.ties, 1);
    assert_eq!(standing.games_played(), 27);

    let team = standing.to_team();

    assert_eq!(team.name, "Hamilton Tigers");
    assert_eq!(team.place_name.default, "Hamilton");
    assert_eq!(team.conference.abbr, "UNK");
    assert_eq!(team.conference.name, "Unknown");
    assert_eq!(team.division.abbr, "UNK");
    assert_eq!(team.division.name, "Unknown");
}

#[test]
fn test_standing_games_played_includes_ties() {
    let json = r#"{
        "teamName": {"default": "Tie Team"},
        "teamCommonName": {"default": "Tie Team"},
        "teamAbbrev": {"default": "TIE"},
        "teamLogo": "",
        "wins": 2,
        "losses": 3,
        "otLosses": 0,
        "ties": 4,
        "points": 8
    }"#;

    let standing: Standing = serde_json::from_str(json).unwrap();

    assert_eq!(standing.games_played(), 9);
}
