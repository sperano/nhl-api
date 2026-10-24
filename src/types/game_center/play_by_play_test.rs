use super::*;
use crate::date::Season;
use crate::ids::GameId;

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
