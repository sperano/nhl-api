use super::*;

#[test]
fn test_player_stats_deserialization() {
    let json = r#"{
        "gamesPlayed": 82,
        "goals": 41,
        "assists": 52,
        "points": 93,
        "plusMinus": 21,
        "pim": 40,
        "powerPlayGoals": 15,
        "powerPlayPoints": 40,
        "shots": 305,
        "shootingPctg": 0.134,
        "faceoffWinPctg": 0.489,
        "avgToi": "21:30"
    }"#;

    let stats: PlayerStats = serde_json::from_str(json).unwrap();
    assert_eq!(stats.games_played, Some(82));
    assert_eq!(stats.goals, Some(41));
    assert_eq!(stats.assists, Some(52));
    assert_eq!(stats.points, Some(93));
    assert_eq!(stats.plus_minus, Some(21));
}

#[test]
fn test_draft_details_deserialization() {
    let json = r#"{
        "year": 2015,
        "teamAbbrev": "EDM",
        "round": 1,
        "pickInRound": 1,
        "overallPick": 1
    }"#;

    let draft: DraftDetails = serde_json::from_str(json).unwrap();
    assert_eq!(draft.year, 2015);
    assert_eq!(draft.team_abbrev, "EDM");
    assert_eq!(draft.overall_pick, 1);
}

#[test]
fn test_player_search_result_deserialization() {
    let json = r#"{
        "playerId": "8478402",
        "name": "Connor McDavid",
        "positionCode": "C",
        "teamId": "22",
        "teamAbbrev": "EDM",
        "sweaterNumber": 97,
        "active": true,
        "height": "6'1\"",
        "birthCity": "Richmond Hill",
        "birthStateProvince": "ON",
        "birthCountry": "CAN"
    }"#;

    let result: PlayerSearchResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.player_id, PlayerId::new(8478402));
    assert_eq!(result.team_id, Some(TeamId::new(22)));
    assert_eq!(result.name, "Connor McDavid");
    assert_eq!(result.position, Some(Position::Center));
    assert!(result.active);
}

/// `PlayerSearchResult.player_id`/`team_id` also accept plain-integer
/// forms (the search endpoint normally returns numeric strings, but the
/// `PlayerId`/`TeamId` deserializer accepts either, 1.3).
#[test]
fn test_player_search_result_ids_deserialize_from_integer_form() {
    let json = r#"{
        "playerId": 8478402,
        "name": "Connor McDavid",
        "teamId": 22,
        "active": true
    }"#;

    let result: PlayerSearchResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.player_id, PlayerId::new(8478402));
    assert_eq!(result.team_id, Some(TeamId::new(22)));
}

#[test]
fn test_player_search_result_empty_position() {
    let json = r#"{
        "playerId": "8478402",
        "name": "Connor McDavid",
        "positionCode": "",
        "active": true
    }"#;

    let result: PlayerSearchResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.position, None);
}

#[test]
fn test_player_search_result_missing_position() {
    let json = r#"{
        "playerId": "8478402",
        "name": "Connor McDavid",
        "active": true
    }"#;

    let result: PlayerSearchResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.position, None);
}

/// The player landing endpoint returns empty `position`/`shootsCatches`
/// strings for some historical players and players with missing bio data.
#[test]
fn test_player_landing_empty_position_and_handedness() {
    let json = r#"{
        "playerId": 8449312,
        "isActive": false,
        "firstName": {"default": "Historical"},
        "lastName": {"default": "Player"},
        "position": "",
        "headshot": "https://assets.nhle.com/mugs/nhl/default.png",
        "heightInInches": 72,
        "weightInPounds": 180,
        "birthDate": "1950-01-01",
        "shootsCatches": ""
    }"#;

    let landing: PlayerLanding = serde_json::from_str(json).unwrap();
    assert_eq!(landing.position, None);
    assert_eq!(landing.shoots_catches, None);
}

#[test]
fn test_player_landing_missing_position_and_handedness() {
    let json = r#"{
        "playerId": 8449312,
        "isActive": false,
        "firstName": {"default": "Historical"},
        "lastName": {"default": "Player"},
        "headshot": "https://assets.nhle.com/mugs/nhl/default.png",
        "heightInInches": 72,
        "weightInPounds": 180,
        "birthDate": "1950-01-01"
    }"#;

    let landing: PlayerLanding = serde_json::from_str(json).unwrap();
    assert_eq!(landing.position, None);
    assert_eq!(landing.shoots_catches, None);
}

#[test]
fn test_player_landing_real_position_and_handedness() {
    let json = r#"{
        "playerId": 8478402,
        "isActive": true,
        "firstName": {"default": "Connor"},
        "lastName": {"default": "McDavid"},
        "position": "C",
        "headshot": "https://assets.nhle.com/mugs/nhl/default.png",
        "heightInInches": 73,
        "weightInPounds": 193,
        "birthDate": "1997-01-13",
        "shootsCatches": "L"
    }"#;

    let landing: PlayerLanding = serde_json::from_str(json).unwrap();
    assert_eq!(landing.position, Some(Position::Center));
    assert_eq!(landing.shoots_catches, Some(Handedness::Left));
}

#[test]
fn test_player_landing_serialize_omits_none_position_and_handedness() {
    let json = r#"{
        "playerId": 8449312,
        "isActive": false,
        "firstName": {"default": "Historical"},
        "lastName": {"default": "Player"},
        "headshot": "https://assets.nhle.com/mugs/nhl/default.png",
        "heightInInches": 72,
        "weightInPounds": 180,
        "birthDate": "1950-01-01"
    }"#;

    let landing: PlayerLanding = serde_json::from_str(json).unwrap();
    let serialized = serde_json::to_string(&landing).unwrap();
    assert!(
        !serialized.contains("\"position\""),
        "expected position to be omitted: {serialized}"
    );
    assert!(
        !serialized.contains("shootsCatches"),
        "expected shootsCatches to be omitted: {serialized}"
    );
}

#[test]
fn test_featured_stats_deserialization() {
    let json = r#"{
        "season": 20242025,
        "regularSeason": {"gamesPlayed": 10, "goals": 5}
    }"#;

    let stats: FeaturedStats = serde_json::from_str(json).unwrap();
    assert_eq!(stats.season, Season::new(2024));
    assert_eq!(stats.regular_season.games_played, Some(10));
}

/// `FeaturedStats.season` accepts the API's string forms as well as the
/// plain-integer form (1.1).
#[test]
fn test_featured_stats_season_deserializes_from_string_form() {
    let json = r#"{
        "season": "20242025",
        "regularSeason": {}
    }"#;

    let stats: FeaturedStats = serde_json::from_str(json).unwrap();
    assert_eq!(stats.season, Season::new(2024));
}

#[test]
fn test_season_total_deserialization() {
    let json = r#"{
        "season": 20232024,
        "gameTypeId": 2,
        "leagueAbbrev": "NHL",
        "teamName": {"default": "Edmonton Oilers"},
        "gamesPlayed": 82,
        "goals": 64,
        "assists": 68
    }"#;

    let total: SeasonTotal = serde_json::from_str(json).unwrap();
    assert_eq!(total.season, Season::new(2023));
    assert_eq!(total.game_type, GameType::RegularSeason);
    assert_eq!(total.games_played, Some(82));
    assert_eq!(total.goals, Some(64));
}

#[test]
fn test_award_season_deserialization() {
    let json = r#"{"seasonId": 20142015}"#;
    let award_season: AwardSeason = serde_json::from_str(json).unwrap();
    assert_eq!(award_season.season_id, Season::new(2014));
}

/// `AwardSeason.season_id` accepts the API's string forms too (1.1).
#[test]
fn test_award_season_id_deserializes_from_string_form() {
    let json = r#"{"seasonId": "20142015"}"#;
    let award_season: AwardSeason = serde_json::from_str(json).unwrap();
    assert_eq!(award_season.season_id, Season::new(2014));
}

#[test]
fn test_award_deserialization() {
    let json = r#"{
        "trophy": {"default": "Hart Memorial Trophy"},
        "seasons": [{"seasonId": 20142015}, {"seasonId": 20162017}]
    }"#;

    let award: Award = serde_json::from_str(json).unwrap();
    assert_eq!(award.trophy.default, "Hart Memorial Trophy");
    assert_eq!(award.seasons.len(), 2);
    assert_eq!(award.seasons[0].season_id, Season::new(2014));
}

#[test]
fn test_player_game_log_deserialization() {
    let json = r#"{
        "seasonId": 20232024,
        "gameTypeId": 2,
        "gameLog": []
    }"#;

    let mut game_log: PlayerGameLog = serde_json::from_str(json).unwrap();
    // Not present in the API response; the client sets it from the request parameter.
    assert_eq!(game_log.player_id, PlayerId::default());
    game_log.player_id = PlayerId::new(8478402);
    assert_eq!(game_log.player_id, PlayerId::new(8478402));
    assert_eq!(game_log.season, Season::new(2023));
    assert_eq!(game_log.game_type, GameType::RegularSeason);
}

/// Wayne Gretzky (live API response): his 1974-75 Brantford row has only team
/// and league, no stats.
const GRETZKY_FIXTURE: &str = include_str!("../../tests/fixtures/player_landing/gretzky.json");

/// Bobby Orr (live API response): his 1972 Summit Series row has no stats.
const ORR_FIXTURE: &str = include_str!("../../tests/fixtures/player_landing/orr.json");

fn season_total(
    landing: &PlayerLanding,
    season: Season,
    game_type: GameType,
    league: &str,
) -> SeasonTotal {
    landing
        .season_totals
        .as_ref()
        .expect("season totals")
        .iter()
        .find(|total| {
            total.season == season && total.game_type == game_type && total.league_abbrev == league
        })
        .cloned()
        .expect("season total row")
}

fn assert_no_stats(total: &SeasonTotal) {
    assert_eq!(total.games_played, None);
    assert_eq!(total.goals, None);
    assert_eq!(total.assists, None);
    assert_eq!(total.points, None);
    assert_eq!(total.plus_minus, None);
    assert_eq!(total.pim, None);
}

#[test]
fn test_player_landing_gretzky_stat_less_minor_league_row() {
    let landing: PlayerLanding = serde_json::from_str(GRETZKY_FIXTURE).unwrap();
    let brantford = season_total(
        &landing,
        Season::new(1974),
        GameType::RegularSeason,
        "Minor-ON",
    );
    assert_eq!(brantford.team_name.default, "Brantford");
    assert_no_stats(&brantford);

    let rookie_nhl = season_total(&landing, Season::new(1979), GameType::RegularSeason, "NHL");
    assert_eq!(rookie_nhl.games_played, Some(79));
    assert_eq!(rookie_nhl.points, Some(137));
}

#[test]
fn test_player_landing_orr_stat_less_tournament_row() {
    let landing: PlayerLanding = serde_json::from_str(ORR_FIXTURE).unwrap();
    let summit = season_total(
        &landing,
        Season::new(1972),
        GameType::RegularSeason,
        "Summit-72",
    );
    assert_eq!(summit.sequence, Some(2));
    assert_no_stats(&summit);
}

#[test]
fn test_season_total_stat_less_row_round_trips() {
    let json = r#"{
        "gameTypeId": 2,
        "leagueAbbrev": "Minor-ON",
        "season": 19741975,
        "sequence": 1,
        "teamName": {"default": "Brantford"}
    }"#;

    let total: SeasonTotal = serde_json::from_str(json).unwrap();
    assert_no_stats(&total);

    let value = serde_json::to_value(&total).unwrap();
    assert!(value.get("gamesPlayed").is_none());
    assert_eq!(serde_json::from_value::<SeasonTotal>(value).unwrap(), total);
}

#[test]
fn test_season_total_zero_games_is_not_stat_less() {
    let json = r#"{
        "season": 20242025,
        "gameTypeId": 3,
        "leagueAbbrev": "NHL",
        "teamName": {"default": "Edmonton Oilers"},
        "gamesPlayed": 0,
        "goals": 0,
        "assists": 0,
        "points": 0,
        "plusMinus": 0,
        "pim": 0
    }"#;

    let total: SeasonTotal = serde_json::from_str(json).unwrap();
    assert_eq!(total.games_played, Some(0));
    assert_eq!(total.points, Some(0));

    let value = serde_json::to_value(&total).unwrap();
    assert_eq!(value["gamesPlayed"], 0);
    assert_eq!(serde_json::from_value::<SeasonTotal>(value).unwrap(), total);
}

#[test]
fn test_player_landing_rookie_single_season_no_playoffs() {
    let json = r#"{
        "playerId": 8485000,
        "isActive": true,
        "currentTeamId": 22,
        "currentTeamAbbrev": "EDM",
        "firstName": {"default": "Rookie"},
        "lastName": {"default": "Player"},
        "position": "C",
        "headshot": "https://assets.nhle.com/mugs/nhl/default.png",
        "heightInInches": 72,
        "weightInPounds": 185,
        "birthDate": "2006-01-01",
        "shootsCatches": "L",
        "careerTotals": {
            "regularSeason": {"gamesPlayed": 3, "goals": 1, "assists": 0, "points": 1}
        },
        "seasonTotals": [{
            "season": 20262027,
            "gameTypeId": 2,
            "leagueAbbrev": "NHL",
            "teamName": {"default": "Edmonton Oilers"},
            "sequence": 1,
            "gamesPlayed": 3,
            "goals": 1,
            "assists": 0,
            "points": 1
        }]
    }"#;

    let landing: PlayerLanding = serde_json::from_str(json).unwrap();
    let career = landing.career_totals.expect("career totals");
    assert_eq!(career.regular_season.games_played, Some(3));
    assert_eq!(career.playoffs, None);
    assert_eq!(landing.featured_stats, None);
    assert_eq!(landing.awards, None);

    let totals = landing.season_totals.expect("season totals");
    assert_eq!(totals.len(), 1);
    assert_eq!(totals[0].games_played, Some(3));
    assert_eq!(totals[0].assists, Some(0));
}
