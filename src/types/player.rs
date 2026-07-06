use crate::types::common::LocalizedString;
use crate::types::enums::{empty_string_as_none, Handedness, HomeRoad, Position};
use crate::types::game_type::GameType;
use serde::{Deserialize, Serialize};

/// Player landing page data - comprehensive player profile
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PlayerLanding {
    pub player_id: i64,
    pub is_active: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_team_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_team_abbrev: Option<String>,

    pub first_name: LocalizedString,
    pub last_name: LocalizedString,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sweater_number: Option<i32>,

    /// `None` when the API returns an empty position code.
    #[serde(deserialize_with = "empty_string_as_none", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<Position>,
    pub headshot: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hero_image: Option<String>,

    pub height_in_inches: i32,
    pub weight_in_pounds: i32,
    pub birth_date: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_city: Option<LocalizedString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_state_province: Option<LocalizedString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_country: Option<String>,

    /// `None` for players with missing handedness data from the API.
    #[serde(deserialize_with = "empty_string_as_none", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shoots_catches: Option<Handedness>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub draft_details: Option<DraftDetails>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub player_slug: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub featured_stats: Option<FeaturedStats>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub career_totals: Option<CareerTotals>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub season_totals: Option<Vec<SeasonTotal>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub awards: Option<Vec<Award>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_five_games: Option<Vec<GameLog>>,
}

/// Draft details for a player
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DraftDetails {
    pub year: i32,
    pub team_abbrev: String,
    pub round: i32,
    pub pick_in_round: i32,
    pub overall_pick: i32,
}

/// Featured stats shown prominently on player page
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FeaturedStats {
    pub season: i32,
    pub regular_season: PlayerStats,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub playoffs: Option<PlayerStats>,
}

/// Career totals for regular season and playoffs
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CareerTotals {
    pub regular_season: PlayerStats,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub playoffs: Option<PlayerStats>,
}

/// Player statistics (skater or goalie)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PlayerStats {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub games_played: Option<i32>,

    // Skater stats
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goals: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub assists: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub points: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub plus_minus: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pim: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub power_play_goals: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub power_play_points: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_handed_goals: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_handed_points: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub shots: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub shooting_pctg: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub faceoff_win_pctg: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_toi: Option<String>,

    // Goalie stats
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wins: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub losses: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ot_losses: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub shutouts: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub goals_against_avg: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub save_pctg: Option<f64>,
}

/// Season-by-season statistics
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SeasonTotal {
    pub season: i32,
    #[serde(rename = "gameTypeId")]
    pub game_type: GameType,
    pub league_abbrev: String,
    pub team_name: LocalizedString,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_common_name: Option<LocalizedString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence: Option<i32>,

    pub games_played: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub goals: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub assists: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub points: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub plus_minus: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pim: Option<i32>,
}

/// Award won by player
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Award {
    pub trophy: LocalizedString,
    pub seasons: Vec<AwardSeason>,
}

/// Season when award was won
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AwardSeason {
    pub season_id: i32,
}

/// Game log entry for a single game
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GameLog {
    pub game_id: i64,
    pub game_date: String,
    pub team_abbrev: String,
    pub home_road_flag: HomeRoad,
    pub opponent_abbrev: String,
    pub goals: i32,
    pub assists: i32,
    pub points: i32,
    pub plus_minus: i32,
    pub power_play_goals: i32,
    pub power_play_points: i32,
    pub shots: i32,
    pub shifts: i32,
    pub toi: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_winning_goals: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ot_goals: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pim: Option<i32>,
}

/// Player game log response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PlayerGameLog {
    /// The player ID is not in the API response, we track it ourselves
    #[serde(skip)]
    pub player_id: i64,

    #[serde(rename = "seasonId")]
    pub season: i32,

    #[serde(rename = "gameTypeId")]
    pub game_type: GameType,

    pub game_log: Vec<GameLog>,
}

/// Player search result
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PlayerSearchResult {
    #[serde(rename = "playerId")]
    pub player_id: String,
    pub name: String,
    /// `None` when the API returns an empty position code.
    #[serde(
        rename = "positionCode",
        deserialize_with = "empty_string_as_none",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<Position>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_abbrev: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sweater_number: Option<i32>,

    pub active: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_city: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_state_province: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_country: Option<String>,
}

#[cfg(test)]
mod tests {
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
        assert_eq!(result.player_id, "8478402");
        assert_eq!(result.name, "Connor McDavid");
        assert_eq!(result.position, Some(Position::Center));
        assert!(result.active);
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
}
