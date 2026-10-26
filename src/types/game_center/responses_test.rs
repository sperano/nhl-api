use super::*;
use crate::ids::{GameId, PlayerId, TeamId};
use crate::types::{PeriodType, Position};

#[test]
fn test_roster_spot_deserialization() {
    let json = r#"{
            "teamId": 1,
            "playerId": 8474593,
            "firstName": {"default": "Jacob"},
            "lastName": {"default": "Markstrom"},
            "sweaterNumber": 25,
            "positionCode": "G",
            "headshot": "https://assets.nhle.com/mugs/nhl/20242025/NJD/8474593.png"
        }"#;

    let roster_spot: RosterSpot = serde_json::from_str(json).unwrap();
    assert_eq!(roster_spot.team_id, TeamId::new(1));
    assert_eq!(roster_spot.player_id, PlayerId::new(8474593));
    assert_eq!(roster_spot.first_name.default, "Jacob");
    assert_eq!(roster_spot.last_name.default, "Markstrom");
    assert_eq!(roster_spot.sweater_number, 25);
    assert_eq!(roster_spot.position, Some(Position::Goalie));
}

/// Historical roster spots (e.g. 1988 BOS) return an empty position code.
#[test]
fn test_roster_spot_empty_position() {
    let json = r#"{
            "teamId": 1,
            "playerId": 8474593,
            "firstName": {"default": "Jacob"},
            "lastName": {"default": "Markstrom"},
            "sweaterNumber": 25,
            "positionCode": "",
            "headshot": "https://assets.nhle.com/mugs/nhl/20242025/NJD/8474593.png"
        }"#;

    let roster_spot: RosterSpot = serde_json::from_str(json).unwrap();
    assert_eq!(roster_spot.position, None);
}

#[test]
fn test_roster_spot_missing_position() {
    let json = r#"{
            "teamId": 1,
            "playerId": 8474593,
            "firstName": {"default": "Jacob"},
            "lastName": {"default": "Markstrom"},
            "sweaterNumber": 25,
            "headshot": "https://assets.nhle.com/mugs/nhl/20242025/NJD/8474593.png"
        }"#;

    let roster_spot: RosterSpot = serde_json::from_str(json).unwrap();
    assert_eq!(roster_spot.position, None);
}

/// `RosterSpot.team_id`/`player_id` accept numeric strings as well as
/// integers (1.3).
#[test]
fn test_roster_spot_ids_deserialize_from_numeric_strings() {
    let json = r#"{
            "teamId": "1",
            "playerId": "8474593",
            "firstName": {"default": "Jacob"},
            "lastName": {"default": "Markstrom"},
            "sweaterNumber": 25,
            "positionCode": "G",
            "headshot": "https://assets.nhle.com/mugs/nhl/20242025/NJD/8474593.png"
        }"#;

    let roster_spot: RosterSpot = serde_json::from_str(json).unwrap();
    assert_eq!(roster_spot.team_id, TeamId::new(1));
    assert_eq!(roster_spot.player_id, PlayerId::new(8474593));
}

#[test]
fn test_game_outcome_deserialization() {
    let json = r#"{"lastPeriodType": "REG"}"#;
    let outcome: GameOutcome = serde_json::from_str(json).unwrap();
    assert_eq!(outcome.last_period_type, Some(PeriodType::Regulation));
}

/// Unplayed games return `lastPeriodType: ""`.
#[test]
fn test_game_outcome_empty_last_period_type() {
    let json = r#"{"lastPeriodType": ""}"#;
    let outcome: GameOutcome = serde_json::from_str(json).unwrap();
    assert_eq!(outcome.last_period_type, None);
}

#[test]
fn test_game_outcome_missing_last_period_type() {
    let json = r#"{}"#;
    let outcome: GameOutcome = serde_json::from_str(json).unwrap();
    assert_eq!(outcome.last_period_type, None);
}

#[test]
fn test_game_outcome_serialize_omits_none() {
    let outcome = GameOutcome {
        last_period_type: None,
    };
    assert_eq!(serde_json::to_string(&outcome).unwrap(), "{}");
}

#[test]
fn test_shift_entry_deserialization() {
    let json = r##"{
            "id": 14376602,
            "detailCode": 0,
            "duration": "17:15",
            "endTime": "17:15",
            "eventDescription": null,
            "eventNumber": 101,
            "firstName": "Jacob",
            "gameId": 2024020001,
            "hexValue": "#C8102E",
            "lastName": "Markstrom",
            "period": 1,
            "playerId": 8474593,
            "shiftNumber": 1,
            "startTime": "00:00",
            "teamAbbrev": "NJD",
            "teamId": 1,
            "teamName": "New Jersey Devils",
            "typeCode": 517
        }"##;

    let shift: ShiftEntry = serde_json::from_str(json).unwrap();
    assert_eq!(shift.id, 14376602);
    assert_eq!(shift.detail_code, 0);
    assert_eq!(shift.duration, Some("17:15".to_string()));
    assert_eq!(shift.end_time, "17:15");
    assert_eq!(shift.event_description, None);
    assert_eq!(shift.event_number, Some(101));
    assert_eq!(shift.first_name, "Jacob");
    assert_eq!(shift.game_id, GameId::new(2024020001));
    assert_eq!(shift.hex_value, "#C8102E");
    assert_eq!(shift.last_name, "Markstrom");
    assert_eq!(shift.period, 1);
    assert_eq!(shift.player_id, PlayerId::new(8474593));
    assert_eq!(shift.shift_number, 1);
    assert_eq!(shift.start_time, "00:00");
    assert_eq!(shift.team_abbrev, "NJD");
    assert_eq!(shift.team_id, TeamId::new(1));
    assert_eq!(shift.team_name, "New Jersey Devils");
    assert_eq!(shift.type_code, 517);
}

#[test]
fn test_shift_chart_deserialization() {
    let json = r##"{
            "data": [
                {
                    "id": 14376602,
                    "detailCode": 0,
                    "duration": "17:15",
                    "endTime": "17:15",
                    "eventDescription": null,
                    "eventNumber": 101,
                    "firstName": "Jacob",
                    "gameId": 2024020001,
                    "hexValue": "#C8102E",
                    "lastName": "Markstrom",
                    "period": 1,
                    "playerId": 8474593,
                    "shiftNumber": 1,
                    "startTime": "00:00",
                    "teamAbbrev": "NJD",
                    "teamId": 1,
                    "teamName": "New Jersey Devils",
                    "typeCode": 517
                }
            ]
        }"##;

    let chart: ShiftChart = serde_json::from_str(json).unwrap();
    assert_eq!(chart.data.len(), 1);
    assert_eq!(chart.data[0].player_id, PlayerId::new(8474593));
    assert_eq!(chart.data[0].first_name, "Jacob");
    assert_eq!(chart.data[0].last_name, "Markstrom");
}

/// Taken from a real `/stats/rest/en/shiftcharts` response (game 2024020444), where
/// goal/penalty rows (typeCode != 517) report a null `duration` instead of the usual
/// "MM:SS" string.
#[test]
fn test_shift_entry_deserialization_null_duration() {
    let json = r##"{
            "id": 14725214,
            "detailCode": 810,
            "duration": null,
            "endTime": "06:16",
            "eventDescription": "EVG",
            "eventNumber": 125,
            "firstName": "Taylor",
            "gameId": 2024020444,
            "hexValue": "#111111",
            "lastName": "Hall",
            "period": 2,
            "playerId": 8475791,
            "shiftNumber": 0,
            "startTime": "06:16",
            "teamAbbrev": "CHI",
            "teamId": 16,
            "teamName": "Chicago Blackhawks",
            "typeCode": 505
        }"##;

    let shift: ShiftEntry = serde_json::from_str(json).unwrap();
    assert_eq!(shift.duration, None);
    assert_eq!(shift.event_description, Some("EVG".to_string()));
    assert_eq!(shift.event_number, Some(125));
}

/// Taken from a real `/stats/rest/en/shiftcharts` response (game 2010020001, from before
/// `eventNumber` was consistently populated), where `eventNumber` and `eventDescription`
/// are both null on an ordinary shift row (typeCode == 517).
#[test]
fn test_shift_entry_deserialization_null_event_number() {
    let json = r##"{
            "id": 1,
            "detailCode": 0,
            "duration": "00:43",
            "endTime": "00:43",
            "eventDescription": null,
            "eventNumber": null,
            "firstName": "Hal",
            "gameId": 2010020001,
            "hexValue": "#AF1E2D",
            "lastName": "Gill",
            "period": 1,
            "playerId": 8459628,
            "shiftNumber": 1,
            "startTime": "00:00",
            "teamAbbrev": "MTL",
            "teamId": 8,
            "teamName": "Montréal Canadiens",
            "typeCode": 517
        }"##;

    let shift: ShiftEntry = serde_json::from_str(json).unwrap();
    assert_eq!(shift.duration, Some("00:43".to_string()));
    assert_eq!(shift.event_description, None);
    assert_eq!(shift.event_number, None);
}

#[test]
fn test_play_event_blocked_shot_deserialization() {
    let json = r#"{
            "eventId": 63,
            "periodDescriptor": {
                "number": 1,
                "periodType": "REG",
                "maxRegulationPeriods": 3
            },
            "timeInPeriod": "01:15",
            "timeRemaining": "18:45",
            "situationCode": "1551",
            "homeTeamDefendingSide": "right",
            "typeCode": 508,
            "typeDescKey": "blocked-shot",
            "sortOrder": 24,
            "details": {
                "xCoord": -73,
                "yCoord": 28,
                "zoneCode": "D",
                "blockingPlayerId": 8481568,
                "shootingPlayerId": 8479323,
                "eventOwnerTeamId": 3,
                "reason": "blocked"
            }
        }"#;

    let event: PlayEvent = serde_json::from_str(json).unwrap();
    assert_eq!(event.type_desc_key, PlayEventType::BlockedShot);

    let details = event.details.unwrap();
    assert_eq!(details.blocking_player_id, Some(PlayerId::new(8481568)));
    assert_eq!(details.shooting_player_id, Some(PlayerId::new(8479323)));
}

#[test]
fn test_play_event_type_serialization() {
    assert_eq!(
        serde_json::to_string(&PlayEventType::Goal).unwrap(),
        "\"goal\""
    );
    assert_eq!(
        serde_json::to_string(&PlayEventType::ShotOnGoal).unwrap(),
        "\"shot-on-goal\""
    );
    assert_eq!(
        serde_json::to_string(&PlayEventType::BlockedShot).unwrap(),
        "\"blocked-shot\""
    );
}

#[test]
fn test_play_event_type_helper_methods() {
    assert!(PlayEventType::Goal.is_goal());
    assert!(!PlayEventType::ShotOnGoal.is_goal());

    assert!(PlayEventType::Goal.is_scoring_chance());
    assert!(PlayEventType::ShotOnGoal.is_scoring_chance());
    assert!(PlayEventType::MissedShot.is_scoring_chance());
    assert!(PlayEventType::BlockedShot.is_scoring_chance());
    assert!(!PlayEventType::Hit.is_scoring_chance());

    assert!(PlayEventType::PeriodStart.is_period_boundary());
    assert!(PlayEventType::PeriodEnd.is_period_boundary());
    assert!(PlayEventType::GameStart.is_period_boundary());
    assert!(PlayEventType::GameEnd.is_period_boundary());
    assert!(!PlayEventType::Goal.is_period_boundary());
}

#[test]
fn test_play_event_type_display() {
    assert_eq!(format!("{}", PlayEventType::GameStart), "Game Start");
    assert_eq!(format!("{}", PlayEventType::PeriodStart), "Period Start");
    assert_eq!(format!("{}", PlayEventType::PeriodEnd), "Period End");
    assert_eq!(format!("{}", PlayEventType::GameEnd), "Game End");
    assert_eq!(format!("{}", PlayEventType::Faceoff), "Faceoff");
    assert_eq!(format!("{}", PlayEventType::Hit), "Hit");
    assert_eq!(format!("{}", PlayEventType::Giveaway), "Giveaway");
    assert_eq!(format!("{}", PlayEventType::Takeaway), "Takeaway");
    assert_eq!(format!("{}", PlayEventType::ShotOnGoal), "Shot on Goal");
    assert_eq!(format!("{}", PlayEventType::MissedShot), "Missed Shot");
    assert_eq!(format!("{}", PlayEventType::BlockedShot), "Blocked Shot");
    assert_eq!(format!("{}", PlayEventType::Goal), "Goal");
    assert_eq!(format!("{}", PlayEventType::Penalty), "Penalty");
    assert_eq!(format!("{}", PlayEventType::Stoppage), "Stoppage");
    assert_eq!(
        format!("{}", PlayEventType::DelayedPenalty),
        "Delayed Penalty"
    );
    assert_eq!(
        format!("{}", PlayEventType::FailedShotAttempt),
        "Failed Shot Attempt"
    );
    assert_eq!(
        format!("{}", PlayEventType::ShootoutComplete),
        "Shootout Complete"
    );
    assert_eq!(format!("{}", PlayEventType::Unknown), "Unknown");
}
