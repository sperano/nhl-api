use serde::{Deserialize, Serialize};

use super::common::LocalizedString;
use super::game_state::GameState;

/// Boxscore response with detailed game and player statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Boxscore {
    pub id: i64,
    pub season: i64,
    #[serde(rename = "gameType")]
    pub game_type: i32,
    #[serde(rename = "limitedScoring")]
    pub limited_scoring: bool,
    #[serde(rename = "gameDate")]
    pub game_date: String,
    pub venue: LocalizedString,
    #[serde(rename = "venueLocation")]
    pub venue_location: LocalizedString,
    #[serde(rename = "startTimeUTC")]
    pub start_time_utc: String,
    #[serde(rename = "easternUTCOffset")]
    pub eastern_utc_offset: String,
    #[serde(rename = "venueUTCOffset")]
    pub venue_utc_offset: String,
    #[serde(rename = "tvBroadcasts", default)]
    pub tv_broadcasts: Vec<TvBroadcast>,
    #[serde(rename = "gameState")]
    pub game_state: GameState,
    #[serde(rename = "gameScheduleState")]
    pub game_schedule_state: String,
    #[serde(rename = "periodDescriptor")]
    pub period_descriptor: PeriodDescriptor,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "specialEvent")]
    pub special_event: Option<SpecialEvent>,
    #[serde(rename = "awayTeam")]
    pub away_team: BoxscoreTeam,
    #[serde(rename = "homeTeam")]
    pub home_team: BoxscoreTeam,
    pub clock: GameClock,
    #[serde(rename = "playerByGameStats")]
    pub player_by_game_stats: PlayerByGameStats,
}

/// TV broadcast information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TvBroadcast {
    pub id: i64,
    pub market: String,
    #[serde(rename = "countryCode")]
    pub country_code: String,
    pub network: String,
    #[serde(rename = "sequenceNumber")]
    pub sequence_number: i32,
}

/// Special event information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SpecialEvent {
    #[serde(rename = "parentId")]
    pub parent_id: i64,
    pub name: LocalizedString,
    #[serde(rename = "lightLogoUrl")]
    pub light_logo_url: LocalizedString,
}

/// Period descriptor with game period information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PeriodDescriptor {
    pub number: i32,
    #[serde(rename = "periodType")]
    pub period_type: String,
    #[serde(rename = "maxRegulationPeriods")]
    pub max_regulation_periods: i32,
}

/// Team information in boxscore
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BoxscoreTeam {
    pub id: i64,
    #[serde(rename = "commonName")]
    pub common_name: LocalizedString,
    pub abbrev: String,
    pub score: i32,
    pub sog: i32,
    pub logo: String,
    #[serde(rename = "darkLogo")]
    pub dark_logo: String,
    #[serde(rename = "placeName")]
    pub place_name: LocalizedString,
    #[serde(rename = "placeNameWithPreposition")]
    pub place_name_with_preposition: LocalizedString,
}

/// Game clock information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GameClock {
    #[serde(rename = "timeRemaining")]
    pub time_remaining: String,
    #[serde(rename = "secondsRemaining")]
    pub seconds_remaining: i32,
    pub running: bool,
    #[serde(rename = "inIntermission")]
    pub in_intermission: bool,
}

/// Player statistics organized by team
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerByGameStats {
    #[serde(rename = "awayTeam")]
    pub away_team: TeamPlayerStats,
    #[serde(rename = "homeTeam")]
    pub home_team: TeamPlayerStats,
}

/// Team's player statistics grouped by position
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamPlayerStats {
    #[serde(default)]
    pub forwards: Vec<SkaterStats>,
    #[serde(default)]
    pub defense: Vec<SkaterStats>,
    #[serde(default)]
    pub goalies: Vec<GoalieStats>,
}

/// Skater (forward/defense) statistics
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SkaterStats {
    #[serde(rename = "playerId")]
    pub player_id: i64,
    #[serde(rename = "sweaterNumber")]
    pub sweater_number: i32,
    pub name: LocalizedString,
    pub position: String,
    pub goals: i32,
    pub assists: i32,
    pub points: i32,
    #[serde(rename = "plusMinus")]
    pub plus_minus: i32,
    pub pim: i32,
    pub hits: i32,
    #[serde(rename = "powerPlayGoals")]
    pub power_play_goals: i32,
    pub sog: i32,
    #[serde(rename = "faceoffWinningPctg")]
    pub faceoff_winning_pctg: f64,
    pub toi: String,
    #[serde(rename = "blockedShots")]
    pub blocked_shots: i32,
    pub shifts: i32,
    pub giveaways: i32,
    pub takeaways: i32,
}

/// Goalie statistics
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GoalieStats {
    #[serde(rename = "playerId")]
    pub player_id: i64,
    #[serde(rename = "sweaterNumber")]
    pub sweater_number: i32,
    pub name: LocalizedString,
    pub position: String,
    #[serde(rename = "evenStrengthShotsAgainst")]
    pub even_strength_shots_against: String,
    #[serde(rename = "powerPlayShotsAgainst")]
    pub power_play_shots_against: String,
    #[serde(rename = "shorthandedShotsAgainst")]
    pub shorthanded_shots_against: String,
    #[serde(rename = "saveShotsAgainst")]
    pub save_shots_against: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "savePctg")]
    pub save_pctg: Option<f64>,
    #[serde(rename = "evenStrengthGoalsAgainst")]
    pub even_strength_goals_against: i32,
    #[serde(rename = "powerPlayGoalsAgainst")]
    pub power_play_goals_against: i32,
    #[serde(rename = "shorthandedGoalsAgainst")]
    pub shorthanded_goals_against: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pim: Option<i32>,
    #[serde(rename = "goalsAgainst")]
    pub goals_against: i32,
    pub toi: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starter: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decision: Option<String>,
    #[serde(rename = "shotsAgainst")]
    pub shots_against: i32,
    pub saves: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boxscore_deserialization() {
        let json = r#"{
            "id": 2024020001,
            "season": 20242025,
            "gameType": 2,
            "limitedScoring": false,
            "gameDate": "2024-10-04",
            "venue": {"default": "Test Arena"},
            "venueLocation": {"default": "Test City"},
            "startTimeUTC": "2024-10-04T19:00:00Z",
            "easternUTCOffset": "-04:00",
            "venueUTCOffset": "-04:00",
            "tvBroadcasts": [],
            "gameState": "LIVE",
            "gameScheduleState": "OK",
            "periodDescriptor": {
                "number": 2,
                "periodType": "REG",
                "maxRegulationPeriods": 3
            },
            "awayTeam": {
                "id": 1,
                "commonName": {"default": "Devils"},
                "abbrev": "NJD",
                "score": 2,
                "sog": 15,
                "logo": "https://assets.nhle.com/logos/nhl/svg/NJD_light.svg",
                "darkLogo": "https://assets.nhle.com/logos/nhl/svg/NJD_dark.svg",
                "placeName": {"default": "New Jersey"},
                "placeNameWithPreposition": {"default": "New Jersey"}
            },
            "homeTeam": {
                "id": 7,
                "commonName": {"default": "Sabres"},
                "abbrev": "BUF",
                "score": 1,
                "sog": 12,
                "logo": "https://assets.nhle.com/logos/nhl/svg/BUF_light.svg",
                "darkLogo": "https://assets.nhle.com/logos/nhl/svg/BUF_dark.svg",
                "placeName": {"default": "Buffalo"},
                "placeNameWithPreposition": {"default": "Buffalo"}
            },
            "clock": {
                "timeRemaining": "10:15",
                "secondsRemaining": 615,
                "running": true,
                "inIntermission": false
            },
            "playerByGameStats": {
                "awayTeam": {
                    "forwards": [],
                    "defense": [],
                    "goalies": []
                },
                "homeTeam": {
                    "forwards": [],
                    "defense": [],
                    "goalies": []
                }
            }
        }"#;

        let boxscore: Boxscore = serde_json::from_str(json).unwrap();
        assert_eq!(boxscore.id, 2024020001);
        assert_eq!(boxscore.season, 20242025);
        assert_eq!(boxscore.game_type, 2);
        assert_eq!(boxscore.game_state, GameState::Live);
        assert_eq!(boxscore.away_team.abbrev, "NJD");
        assert_eq!(boxscore.home_team.abbrev, "BUF");
        assert_eq!(boxscore.away_team.score, 2);
        assert_eq!(boxscore.home_team.score, 1);
        assert_eq!(boxscore.clock.time_remaining, "10:15");
        assert_eq!(boxscore.clock.seconds_remaining, 615);
        assert!(boxscore.clock.running);
        assert_eq!(boxscore.period_descriptor.number, 2);
    }

    #[test]
    fn test_skater_stats_deserialization() {
        let json = r#"{
            "playerId": 8480002,
            "sweaterNumber": 13,
            "name": {"default": "N. Hischier"},
            "position": "C",
            "goals": 1,
            "assists": 2,
            "points": 3,
            "plusMinus": 2,
            "pim": 0,
            "hits": 3,
            "powerPlayGoals": 1,
            "sog": 4,
            "faceoffWinningPctg": 0.55,
            "toi": "18:15",
            "blockedShots": 1,
            "shifts": 27,
            "giveaways": 0,
            "takeaways": 2
        }"#;

        let stats: SkaterStats = serde_json::from_str(json).unwrap();
        assert_eq!(stats.player_id, 8480002);
        assert_eq!(stats.sweater_number, 13);
        assert_eq!(stats.name.default, "N. Hischier");
        assert_eq!(stats.position, "C");
        assert_eq!(stats.goals, 1);
        assert_eq!(stats.assists, 2);
        assert_eq!(stats.points, 3);
        assert_eq!(stats.plus_minus, 2);
        assert_eq!(stats.faceoff_winning_pctg, 0.55);
    }

    #[test]
    fn test_goalie_stats_deserialization() {
        let json = r#"{
            "playerId": 8474593,
            "sweaterNumber": 25,
            "name": {"default": "J. Markstrom"},
            "position": "G",
            "evenStrengthShotsAgainst": "25/26",
            "powerPlayShotsAgainst": "5/5",
            "shorthandedShotsAgainst": "0/0",
            "saveShotsAgainst": "30/31",
            "savePctg": 0.967,
            "evenStrengthGoalsAgainst": 1,
            "powerPlayGoalsAgainst": 0,
            "shorthandedGoalsAgainst": 0,
            "pim": 0,
            "goalsAgainst": 1,
            "toi": "59:38",
            "starter": true,
            "decision": "W",
            "shotsAgainst": 31,
            "saves": 30
        }"#;

        let stats: GoalieStats = serde_json::from_str(json).unwrap();
        assert_eq!(stats.player_id, 8474593);
        assert_eq!(stats.sweater_number, 25);
        assert_eq!(stats.name.default, "J. Markstrom");
        assert_eq!(stats.position, "G");
        assert_eq!(stats.save_pctg, Some(0.967));
        assert_eq!(stats.goals_against, 1);
        assert_eq!(stats.saves, 30);
        assert_eq!(stats.shots_against, 31);
        assert_eq!(stats.starter, Some(true));
        assert_eq!(stats.decision, Some("W".to_string()));
    }
}
