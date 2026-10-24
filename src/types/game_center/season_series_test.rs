use super::*;
use crate::ids::PlayerId;
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
