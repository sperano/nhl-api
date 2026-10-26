use serde::{Deserialize, Serialize};

use crate::ids::{GameId, PlayerId, TeamId};

/// Shift chart data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ShiftChart {
    pub data: Vec<ShiftEntry>,
}

/// Individual shift entry for a player
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ShiftEntry {
    pub id: i64,
    #[serde(rename = "detailCode")]
    pub detail_code: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[serde(rename = "endTime")]
    pub end_time: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "eventDescription")]
    pub event_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "eventNumber")]
    pub event_number: Option<i64>,
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "gameId")]
    pub game_id: GameId,
    #[serde(rename = "hexValue")]
    pub hex_value: String,
    #[serde(rename = "lastName")]
    pub last_name: String,
    pub period: i32,
    #[serde(rename = "playerId")]
    pub player_id: PlayerId,
    #[serde(rename = "shiftNumber")]
    pub shift_number: i32,
    #[serde(rename = "startTime")]
    pub start_time: String,
    #[serde(rename = "teamAbbrev")]
    pub team_abbrev: String,
    #[serde(rename = "teamId")]
    pub team_id: TeamId,
    #[serde(rename = "teamName")]
    pub team_name: String,
    #[serde(rename = "typeCode")]
    pub type_code: i32,
}
