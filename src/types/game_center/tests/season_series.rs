use super::super::*;
use crate::date::Season;
use crate::ids::{GameId, PlayerId};
use crate::types::{GameState, PeriodType};

/// Regression test for the 2.3 fix: a season series containing an
/// unplayed/future game (empty `periodType`/`lastPeriodType`) used to
/// fail deserialization of the whole `SeasonSeriesMatchup` response.
#[test]
fn test_season_series_matchup_unplayed_game_deserialization() {
    let json = r#"{
            "seasonSeries": [
                {
                    "id": 2024020100,
                    "season": 20242025,
                    "gameType": 2,
                    "gameDate": "2024-11-01",
                    "startTimeUTC": "2024-11-01T23:00:00Z",
                    "easternUTCOffset": "-04:00",
                    "venueUTCOffset": "-04:00",
                    "gameState": "OFF",
                    "gameScheduleState": "OK",
                    "awayTeam": {"id": 1, "abbrev": "NJD", "logo": "https://a", "score": 3},
                    "homeTeam": {"id": 7, "abbrev": "BUF", "logo": "https://b", "score": 2},
                    "periodDescriptor": {
                        "number": 3,
                        "periodType": "REG",
                        "maxRegulationPeriods": 3
                    },
                    "gameCenterLink": "/gamecenter/2024020100",
                    "gameOutcome": {"lastPeriodType": "REG"}
                },
                {
                    "id": 2024020250,
                    "season": 20242025,
                    "gameType": 2,
                    "gameDate": "2025-02-14",
                    "startTimeUTC": "2025-02-14T23:00:00Z",
                    "easternUTCOffset": "-05:00",
                    "venueUTCOffset": "-05:00",
                    "gameState": "FUT",
                    "gameScheduleState": "OK",
                    "awayTeam": {"id": 1, "abbrev": "NJD", "logo": "https://a", "score": 0},
                    "homeTeam": {"id": 7, "abbrev": "BUF", "logo": "https://b", "score": 0},
                    "periodDescriptor": {
                        "number": 0,
                        "periodType": "",
                        "maxRegulationPeriods": 0
                    },
                    "gameCenterLink": "/gamecenter/2024020250",
                    "gameOutcome": {"lastPeriodType": ""}
                }
            ],
            "seasonSeriesWins": {"awayTeamWins": 1, "homeTeamWins": 0},
            "gameInfo": {
                "referees": [{"fullName": {"default": "J. Referee"}, "sweaterNumber": 1}],
                "linesmen": [{"fullName": {"default": "L. Linesman"}, "sweaterNumber": 2}],
                "awayTeam": {"headCoach": {"default": "Coach A"}, "scratches": []},
                "homeTeam": {"headCoach": {"default": "Coach B"}, "scratches": []}
            }
        }"#;

    let matchup: SeasonSeriesMatchup = serde_json::from_str(json).unwrap();
    assert_eq!(matchup.season_series.len(), 2);

    let played = &matchup.season_series[0];
    assert_eq!(
        played.period_descriptor.period_type,
        Some(PeriodType::Regulation)
    );
    assert_eq!(
        played.game_outcome.last_period_type,
        Some(PeriodType::Regulation)
    );

    let unplayed = &matchup.season_series[1];
    assert_eq!(unplayed.game_state, GameState::Future);
    assert_eq!(unplayed.period_descriptor.period_type, None);
    assert_eq!(unplayed.game_outcome.last_period_type, None);
}

/// Regression test for the right-rail fix: `gameInfo` officials
/// (`referees`/`linesmen`) are objects with a localized `fullName` and a
/// `sweaterNumber`, not bare localized strings. A localized object that
/// omits `default` must not fail the whole response either.
#[test]
fn test_season_series_matchup_officials_deserialization() {
    let json = r#"{
            "seasonSeries": [],
            "seasonSeriesWins": {"awayTeamWins": 0, "homeTeamWins": 0},
            "gameInfo": {
                "referees": [
                    {"fullName": {"default": "Kelly Sutherland"}, "sweaterNumber": 11},
                    {"fullName": {"default": "Mitch Dunning"}, "sweaterNumber": 20}
                ],
                "linesmen": [
                    {"fullName": {"default": "Derek Nansen"}, "sweaterNumber": 70}
                ],
                "awayTeam": {
                    "headCoach": {"default": "Anders Sorensen"},
                    "scratches": [
                        {"id": 8474166, "firstName": {"default": "Alec"}, "lastName": {"default": "Martinez"}}
                    ]
                },
                "homeTeam": {
                    "headCoach": {"fr": "Entraîneur"},
                    "scratches": []
                }
            }
        }"#;

    let matchup: SeasonSeriesMatchup = serde_json::from_str(json).unwrap();
    let game_info = matchup.game_info;

    assert_eq!(game_info.referees.len(), 2);
    assert_eq!(game_info.referees[0].full_name.default, "Kelly Sutherland");
    assert_eq!(game_info.referees[0].sweater_number, 11);
    assert_eq!(game_info.referees[1].full_name.default, "Mitch Dunning");
    assert_eq!(game_info.referees[1].sweater_number, 20);

    assert_eq!(game_info.linesmen.len(), 1);
    assert_eq!(game_info.linesmen[0].full_name.default, "Derek Nansen");
    assert_eq!(game_info.linesmen[0].sweater_number, 70);

    assert_eq!(game_info.away_team.head_coach.default, "Anders Sorensen");
    assert_eq!(game_info.away_team.scratches.len(), 1);
    assert_eq!(game_info.away_team.scratches[0].id, PlayerId::new(8474166));

    // `headCoach` sent only a localized variant (no `default`): it must
    // deserialize to an empty string rather than fail.
    assert_eq!(game_info.home_team.head_coach.default, "");
    assert!(game_info.home_team.scratches.is_empty());
}

/// Missing `referees`/`linesmen`/`scratches` arrays default to empty
/// rather than failing deserialization.
#[test]
fn test_season_series_matchup_missing_official_arrays() {
    let json = r#"{
            "seasonSeries": [],
            "seasonSeriesWins": {"awayTeamWins": 0, "homeTeamWins": 0},
            "gameInfo": {
                "awayTeam": {"headCoach": {"default": "A"}},
                "homeTeam": {"headCoach": {"default": "B"}}
            }
        }"#;

    let matchup: SeasonSeriesMatchup = serde_json::from_str(json).unwrap();
    assert!(matchup.game_info.referees.is_empty());
    assert!(matchup.game_info.linesmen.is_empty());
    assert!(matchup.game_info.away_team.scratches.is_empty());
    assert!(matchup.game_info.home_team.scratches.is_empty());
}

/// Minimal fields required to deserialize a `PlayByPlay`, with an
/// optional trailing `regPeriods` fragment appended by the caller.
fn play_by_play_json(reg_periods_fragment: &str) -> String {
    format!(
        r#"{{
                "id": 2024020444,
                "season": 20242025,
                "gameType": 2,
                "limitedScoring": false,
                "gameDate": "2024-11-01",
                "venue": {{"default": "Test Arena"}},
                "venueLocation": {{"default": "Test City"}},
                "startTimeUTC": "2024-11-01T19:00:00Z",
                "easternUTCOffset": "-04:00",
                "venueUTCOffset": "-04:00",
                "gameState": "FINAL",
                "gameScheduleState": "OK",
                "periodDescriptor": {{}},
                "awayTeam": {{
                    "id": 1,
                    "commonName": {{"default": "Devils"}},
                    "abbrev": "NJD",
                    "score": 2,
                    "sog": 15,
                    "logo": "https://example.com/njd_light.svg",
                    "darkLogo": "https://example.com/njd_dark.svg",
                    "placeName": {{"default": "New Jersey"}},
                    "placeNameWithPreposition": {{"default": "New Jersey"}}
                }},
                "homeTeam": {{
                    "id": 7,
                    "commonName": {{"default": "Sabres"}},
                    "abbrev": "BUF",
                    "score": 1,
                    "sog": 12,
                    "logo": "https://example.com/buf_light.svg",
                    "darkLogo": "https://example.com/buf_dark.svg",
                    "placeName": {{"default": "Buffalo"}},
                    "placeNameWithPreposition": {{"default": "Buffalo"}}
                }},
                "shootoutInUse": false,
                "otInUse": false,
                "clock": {{
                    "timeRemaining": "00:00",
                    "secondsRemaining": 0,
                    "running": false,
                    "inIntermission": false
                }},
                "displayPeriod": 3,
                "maxPeriods": 3
                {reg_periods_fragment}
            }}"#
    )
}

#[test]
fn test_play_by_play_missing_reg_periods_defaults_to_zero() {
    let json = play_by_play_json("");
    let pbp: PlayByPlay = serde_json::from_str(&json).unwrap();
    assert_eq!(pbp.reg_periods, 0);
}

#[test]
fn test_play_by_play_reg_periods_present() {
    let json = play_by_play_json(r#", "regPeriods": 3"#);
    let pbp: PlayByPlay = serde_json::from_str(&json).unwrap();
    assert_eq!(pbp.reg_periods, 3);
}

/// `PlayByPlay.id`/`season` accept numeric-string forms, matching the
/// `GameId`/`Season` serde support from 1.1/1.3.
#[test]
fn test_play_by_play_id_and_season_deserialize_from_numeric_strings() {
    let json = play_by_play_json("").replacen("2024020444", "\"2024020444\"", 1);
    let json = json.replacen("20242025", "\"20242025\"", 1);
    let pbp: PlayByPlay = serde_json::from_str(&json).unwrap();
    assert_eq!(pbp.id, GameId::new(2024020444));
    assert_eq!(pbp.season, Season::new(2024));
}

#[test]
fn test_game_summary_missing_shootout_and_three_stars() {
    let json = r#"{
            "scoring": [],
            "penalties": []
        }"#;

    let summary: GameSummary = serde_json::from_str(json).unwrap();
    assert!(summary.shootout.is_empty());
    assert!(summary.three_stars.is_empty());
}

#[test]
fn test_game_summary_with_shootout_and_three_stars() {
    let json = r#"{
            "scoring": [],
            "shootout": [{
                "sequence": 1,
                "playerId": 8478402,
                "teamAbbrev": {"default": "EDM"},
                "firstName": {"default": "Connor"},
                "lastName": {"default": "McDavid"},
                "shotType": "wrist",
                "result": "goal",
                "headshot": "https://assets.nhle.com/mugs/nhl/20242025/EDM/8478402.png",
                "gameWinner": true
            }],
            "threeStars": [{
                "star": 1,
                "playerId": 8478402,
                "teamAbbrev": "EDM",
                "headshot": "https://assets.nhle.com/mugs/nhl/20242025/EDM/8478402.png",
                "name": "C. McDavid",
                "sweaterNo": 97
            }],
            "penalties": []
        }"#;

    let summary: GameSummary = serde_json::from_str(json).unwrap();
    assert_eq!(summary.shootout.len(), 1);
    assert!(summary.shootout[0].game_winner);
    assert_eq!(summary.three_stars.len(), 1);
    assert_eq!(summary.three_stars[0].player_id, PlayerId::new(8478402));
}
