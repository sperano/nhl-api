use super::*;
use crate::ids::PlayerId;
use crate::types::{DefendingSide, Position, ZoneCode};

#[test]
fn test_play_event_goal_deserialization() {
    let json = r#"{
            "eventId": 274,
            "periodDescriptor": {
                "number": 1,
                "periodType": "REG",
                "maxRegulationPeriods": 3
            },
            "timeInPeriod": "08:39",
            "timeRemaining": "11:21",
            "situationCode": "1551",
            "homeTeamDefendingSide": "right",
            "typeCode": 505,
            "typeDescKey": "goal",
            "sortOrder": 146,
            "details": {
                "xCoord": 71,
                "yCoord": -12,
                "zoneCode": "O",
                "shotType": "snap",
                "scoringPlayerId": 8476474,
                "scoringPlayerTotal": 1,
                "assist1PlayerId": 8480192,
                "assist1PlayerTotal": 1,
                "eventOwnerTeamId": 1,
                "goalieInNetId": 8480045,
                "awayScore": 1,
                "homeScore": 0,
                "highlightClip": 6362848229112,
                "discreteClip": 6362846260112
            }
        }"#;

    let event: PlayEvent = serde_json::from_str(json).unwrap();
    assert_eq!(event.event_id, 274);
    assert_eq!(event.type_desc_key, PlayEventType::Goal);
    assert_eq!(event.time_in_period, "08:39");

    let details = event.details.unwrap();
    assert_eq!(details.scoring_player_id, Some(PlayerId::new(8476474)));
    assert_eq!(details.scoring_player_total, Some(1));
    assert_eq!(details.assist1_player_id, Some(PlayerId::new(8480192)));
    assert_eq!(details.away_score, Some(1));
    assert_eq!(details.home_score, Some(0));
    assert_eq!(details.shot_type, Some("snap".to_string()));
}

#[test]
fn test_play_event_penalty_deserialization() {
    let json = r#"{
            "eventId": 135,
            "periodDescriptor": {
                "number": 1,
                "periodType": "REG",
                "maxRegulationPeriods": 3
            },
            "timeInPeriod": "01:37",
            "timeRemaining": "18:23",
            "situationCode": "1560",
            "homeTeamDefendingSide": "right",
            "typeCode": 509,
            "typeDescKey": "penalty",
            "sortOrder": 45,
            "details": {
                "xCoord": 1,
                "yCoord": -37,
                "zoneCode": "N",
                "typeCode": "MIN",
                "descKey": "slashing",
                "duration": 2,
                "committedByPlayerId": 8475287,
                "drawnByPlayerId": 8479420,
                "eventOwnerTeamId": 1
            }
        }"#;

    let event: PlayEvent = serde_json::from_str(json).unwrap();
    assert_eq!(event.event_id, 135);
    assert_eq!(event.type_desc_key, PlayEventType::Penalty);

    let details = event.details.unwrap();
    assert_eq!(details.type_code, Some("MIN".to_string()));
    assert_eq!(details.desc_key, Some("slashing".to_string()));
    assert_eq!(details.duration, Some(2));
    assert_eq!(details.committed_by_player_id, Some(PlayerId::new(8475287)));
    assert_eq!(details.drawn_by_player_id, Some(PlayerId::new(8479420)));
}

#[test]
fn test_play_event_shot_deserialization() {
    let json = r#"{
            "eventId": 103,
            "periodDescriptor": {
                "number": 1,
                "periodType": "REG",
                "maxRegulationPeriods": 3
            },
            "timeInPeriod": "00:08",
            "timeRemaining": "19:52",
            "situationCode": "1551",
            "homeTeamDefendingSide": "right",
            "typeCode": 506,
            "typeDescKey": "shot-on-goal",
            "sortOrder": 13,
            "details": {
                "xCoord": 56,
                "yCoord": -39,
                "zoneCode": "O",
                "shotType": "wrist",
                "shootingPlayerId": 8483495,
                "goalieInNetId": 8480045,
                "eventOwnerTeamId": 1,
                "awaySOG": 1,
                "homeSOG": 0
            }
        }"#;

    let event: PlayEvent = serde_json::from_str(json).unwrap();
    assert_eq!(event.event_id, 103);
    assert_eq!(event.type_desc_key, PlayEventType::ShotOnGoal);

    let details = event.details.unwrap();
    assert_eq!(details.shot_type, Some("wrist".to_string()));
    assert_eq!(details.shooting_player_id, Some(PlayerId::new(8483495)));
    assert_eq!(details.goalie_in_net_id, Some(PlayerId::new(8480045)));
    assert_eq!(details.away_sog, Some(1));
    assert_eq!(details.home_sog, Some(0));
}

#[test]
fn test_play_event_faceoff_deserialization() {
    let json = r#"{
            "eventId": 151,
            "periodDescriptor": {
                "number": 1,
                "periodType": "REG",
                "maxRegulationPeriods": 3
            },
            "timeInPeriod": "00:00",
            "timeRemaining": "20:00",
            "situationCode": "1551",
            "homeTeamDefendingSide": "right",
            "typeCode": 502,
            "typeDescKey": "faceoff",
            "sortOrder": 11,
            "details": {
                "eventOwnerTeamId": 1,
                "losingPlayerId": 8478043,
                "winningPlayerId": 8480002,
                "xCoord": 0,
                "yCoord": 0,
                "zoneCode": "N"
            }
        }"#;

    let event: PlayEvent = serde_json::from_str(json).unwrap();
    assert_eq!(event.event_id, 151);
    assert_eq!(event.type_desc_key, PlayEventType::Faceoff);

    let details = event.details.unwrap();
    assert_eq!(details.winning_player_id, Some(PlayerId::new(8480002)));
    assert_eq!(details.losing_player_id, Some(PlayerId::new(8478043)));
    assert_eq!(details.zone_code, Some(ZoneCode::Neutral));
}

/// Some play events return `zoneCode: ""`; the field must resolve to
/// `None` rather than fail the whole deserialize.
#[test]
fn test_play_event_details_empty_zone_code() {
    let json = r#"{"zoneCode": ""}"#;

    let details: PlayEventDetails = serde_json::from_str(json).unwrap();
    assert_eq!(details.zone_code, None);
}

#[test]
fn test_play_event_details_missing_zone_code() {
    let details: PlayEventDetails = serde_json::from_str("{}").unwrap();
    assert_eq!(details.zone_code, None);
}

#[test]
fn test_play_event_details_serialize_omits_none_zone_code() {
    let details: PlayEventDetails = serde_json::from_str("{}").unwrap();
    let json = serde_json::to_string(&details).unwrap();
    assert_eq!(json, "{}", "expected zoneCode (and all else) to be omitted");
}

/// Old games (pre-1920s) lack `homeTeamDefendingSide` data.
#[test]
fn test_play_event_empty_defending_side() {
    let json = r#"{
            "eventId": 1,
            "periodDescriptor": {
                "number": 1,
                "periodType": "REG",
                "maxRegulationPeriods": 3
            },
            "timeInPeriod": "00:00",
            "timeRemaining": "20:00",
            "situationCode": "1551",
            "homeTeamDefendingSide": "",
            "typeCode": 502,
            "typeDescKey": "faceoff",
            "sortOrder": 1
        }"#;

    let event: PlayEvent = serde_json::from_str(json).unwrap();
    assert_eq!(event.home_team_defending_side, None);
}

#[test]
fn test_play_event_missing_defending_side() {
    let json = r#"{
            "eventId": 1,
            "periodDescriptor": {
                "number": 1,
                "periodType": "REG",
                "maxRegulationPeriods": 3
            },
            "timeInPeriod": "00:00",
            "timeRemaining": "20:00",
            "situationCode": "1551",
            "typeCode": 502,
            "typeDescKey": "faceoff",
            "sortOrder": 1
        }"#;

    let event: PlayEvent = serde_json::from_str(json).unwrap();
    assert_eq!(event.home_team_defending_side, None);
}

/// Builds a minimal, valid `GoalSummary` JSON fixture with the given
/// `homeTeamDefendingSide` fragment spliced in (e.g. `"\"left\""`,
/// `"\"\""`, or `""` to omit the key entirely).
fn goal_summary_json_with_defending_side(defending_side_field: &str) -> String {
    format!(
        r#"{{
                "situationCode": "1551",
                "eventId": 1,
                "strength": "ev",
                "playerId": 8478402,
                "firstName": {{"default": "Connor"}},
                "lastName": {{"default": "McDavid"}},
                "name": {{"default": "C. McDavid"}},
                "teamAbbrev": {{"default": "EDM"}},
                "headshot": "https://assets.nhle.com/mugs/nhl/default.png",
                "awayScore": 1,
                "homeScore": 0,
                "timeInPeriod": "10:00",
                "shotType": "wrist",
                "goalModifier": "",
                {defending_side}
                "isHome": false
            }}"#,
        defending_side = defending_side_field,
    )
}

#[test]
fn test_goal_summary_real_defending_side() {
    let json = goal_summary_json_with_defending_side(r#""homeTeamDefendingSide": "left","#);
    let goal: GoalSummary = serde_json::from_str(&json).unwrap();
    assert_eq!(goal.home_team_defending_side, Some(DefendingSide::Left));
}

#[test]
fn test_goal_summary_empty_defending_side() {
    let json = goal_summary_json_with_defending_side(r#""homeTeamDefendingSide": "","#);
    let goal: GoalSummary = serde_json::from_str(&json).unwrap();
    assert_eq!(goal.home_team_defending_side, None);
}

#[test]
fn test_goal_summary_missing_defending_side() {
    let json = goal_summary_json_with_defending_side("");
    let goal: GoalSummary = serde_json::from_str(&json).unwrap();
    assert_eq!(goal.home_team_defending_side, None);
}

#[test]
fn test_three_star_position_real_empty_and_missing() {
    let real: ThreeStar = serde_json::from_str(
        r#"{
                "star": 1,
                "playerId": 8478402,
                "teamAbbrev": "EDM",
                "headshot": "https://assets.nhle.com/mugs/nhl/20242025/EDM/8478402.png",
                "name": "C. McDavid",
                "sweaterNo": 97,
                "position": "C"
            }"#,
    )
    .unwrap();
    assert_eq!(real.position, Some(Position::Center));

    let empty: ThreeStar = serde_json::from_str(
        r#"{
                "star": 1,
                "playerId": 8478402,
                "teamAbbrev": "EDM",
                "headshot": "https://assets.nhle.com/mugs/nhl/20242025/EDM/8478402.png",
                "name": "C. McDavid",
                "sweaterNo": 97,
                "position": ""
            }"#,
    )
    .unwrap();
    assert_eq!(empty.position, None);

    let missing: ThreeStar = serde_json::from_str(
        r#"{
                "star": 1,
                "playerId": 8478402,
                "teamAbbrev": "EDM",
                "headshot": "https://assets.nhle.com/mugs/nhl/20242025/EDM/8478402.png",
                "name": "C. McDavid",
                "sweaterNo": 97
            }"#,
    )
    .unwrap();
    assert_eq!(missing.position, None);
}
