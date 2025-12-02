use serde::{Deserialize, Serialize};

use super::common::LocalizedString;
use super::enums::{GoalieDecision, PeriodType, Position};
use super::game_state::GameState;
use super::game_type::GameType;

/// Boxscore response with detailed game and player statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Boxscore {
    pub id: i64,
    pub season: i64,
    #[serde(rename = "gameType")]
    pub game_type: GameType,
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
    pub period_type: PeriodType,
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

/// Aggregated team statistics for game comparison
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TeamGameStats {
    pub shots_on_goal: i32,
    pub faceoff_wins: i32,
    pub faceoff_total: i32,
    pub power_play_goals: i32,
    pub power_play_opportunities: i32,
    pub penalty_minutes: i32,
    pub hits: i32,
    pub blocked_shots: i32,
    pub giveaways: i32,
    pub takeaways: i32,
}

impl TeamGameStats {
    /// Calculate aggregated team statistics from individual player stats
    pub fn from_team_player_stats(stats: &TeamPlayerStats) -> Self {
        let mut team_stats = Self::default();

        Self::aggregate_skater_stats(&mut team_stats, stats);
        Self::aggregate_goalie_stats(&mut team_stats, stats);

        team_stats
    }

    fn aggregate_skater_stats(team_stats: &mut TeamGameStats, stats: &TeamPlayerStats) {
        for skater in stats.forwards.iter().chain(stats.defense.iter()) {
            team_stats.shots_on_goal += skater.sog;
            team_stats.power_play_goals += skater.power_play_goals;
            team_stats.penalty_minutes += skater.pim;
            team_stats.hits += skater.hits;
            team_stats.blocked_shots += skater.blocked_shots;
            team_stats.giveaways += skater.giveaways;
            team_stats.takeaways += skater.takeaways;

            Self::add_faceoff_stats(team_stats, skater);
        }
    }

    fn add_faceoff_stats(team_stats: &mut TeamGameStats, skater: &SkaterStats) {
        // TODO: Revisit this logic - not sure only counting centers for faceoffs is correct.
        // Wings can also take faceoffs in certain situations.
        if skater.position == Position::Center && skater.faceoff_winning_pctg > 0.0 {
            // Estimate total faceoffs using shifts as a proxy for faceoff participation
            let estimated_faceoffs = skater.shifts;
            team_stats.faceoff_total += estimated_faceoffs;
            team_stats.faceoff_wins +=
                (estimated_faceoffs as f64 * skater.faceoff_winning_pctg).round() as i32;
        }
    }

    fn aggregate_goalie_stats(team_stats: &mut TeamGameStats, stats: &TeamPlayerStats) {
        for goalie in &stats.goalies {
            if let Some(pim) = goalie.pim {
                team_stats.penalty_minutes += pim;
            }
            // Count power play opportunities from goals against
            team_stats.power_play_opportunities += goalie.power_play_goals_against;
        }
    }

    pub fn faceoff_percentage(&self) -> f64 {
        if self.faceoff_total > 0 {
            (self.faceoff_wins as f64 / self.faceoff_total as f64) * 100.0
        } else {
            0.0
        }
    }

    pub fn power_play_percentage(&self) -> f64 {
        if self.power_play_opportunities > 0 {
            (self.power_play_goals as f64 / self.power_play_opportunities as f64) * 100.0
        } else {
            0.0
        }
    }
}

/// Skater (forward/defense) statistics
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SkaterStats {
    #[serde(rename = "playerId")]
    pub player_id: i64,
    #[serde(rename = "sweaterNumber")]
    pub sweater_number: i32,
    pub name: LocalizedString,
    pub position: Position,
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
    pub position: Position,
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
    pub decision: Option<GoalieDecision>,
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
        assert_eq!(boxscore.game_type, GameType::RegularSeason);
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
        assert_eq!(stats.position, Position::Center);
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
        assert_eq!(stats.position, Position::Goalie);
        assert_eq!(stats.save_pctg, Some(0.967));
        assert_eq!(stats.goals_against, 1);
        assert_eq!(stats.saves, 30);
        assert_eq!(stats.shots_against, 31);
        assert_eq!(stats.starter, Some(true));
        assert_eq!(stats.decision, Some(GoalieDecision::Win));
    }

    #[test]
    fn test_tv_broadcast_deserialization() {
        let json = r#"{
            "id": 123,
            "market": "NATIONAL",
            "countryCode": "US",
            "network": "ESPN",
            "sequenceNumber": 1
        }"#;

        let broadcast: TvBroadcast = serde_json::from_str(json).unwrap();
        assert_eq!(broadcast.id, 123);
        assert_eq!(broadcast.market, "NATIONAL");
        assert_eq!(broadcast.country_code, "US");
        assert_eq!(broadcast.network, "ESPN");
        assert_eq!(broadcast.sequence_number, 1);
    }

    #[test]
    fn test_special_event_deserialization() {
        let json = r#"{
            "parentId": 999,
            "name": {"default": "Winter Classic"},
            "lightLogoUrl": {"default": "https://example.com/logo.png"}
        }"#;

        let event: SpecialEvent = serde_json::from_str(json).unwrap();
        assert_eq!(event.parent_id, 999);
        assert_eq!(event.name.default, "Winter Classic");
        assert_eq!(event.light_logo_url.default, "https://example.com/logo.png");
    }

    #[test]
    fn test_period_descriptor_deserialization() {
        let json = r#"{
            "number": 3,
            "periodType": "REG",
            "maxRegulationPeriods": 3
        }"#;

        let period: PeriodDescriptor = serde_json::from_str(json).unwrap();
        assert_eq!(period.number, 3);
        assert_eq!(period.period_type, PeriodType::Regulation);
        assert_eq!(period.max_regulation_periods, 3);
    }

    #[test]
    fn test_period_descriptor_overtime() {
        let json = r#"{
            "number": 4,
            "periodType": "OT",
            "maxRegulationPeriods": 3
        }"#;

        let period: PeriodDescriptor = serde_json::from_str(json).unwrap();
        assert_eq!(period.number, 4);
        assert_eq!(period.period_type, PeriodType::Overtime);
    }

    #[test]
    fn test_boxscore_team_deserialization() {
        let json = r#"{
            "id": 8,
            "commonName": {"default": "Canadiens"},
            "abbrev": "MTL",
            "score": 3,
            "sog": 28,
            "logo": "https://assets.nhle.com/logos/nhl/svg/MTL_light.svg",
            "darkLogo": "https://assets.nhle.com/logos/nhl/svg/MTL_dark.svg",
            "placeName": {"default": "Montréal"},
            "placeNameWithPreposition": {"default": "Montréal"}
        }"#;

        let team: BoxscoreTeam = serde_json::from_str(json).unwrap();
        assert_eq!(team.id, 8);
        assert_eq!(team.common_name.default, "Canadiens");
        assert_eq!(team.abbrev, "MTL");
        assert_eq!(team.score, 3);
        assert_eq!(team.sog, 28);
    }

    #[test]
    fn test_game_clock_deserialization() {
        let json = r#"{
            "timeRemaining": "05:30",
            "secondsRemaining": 330,
            "running": false,
            "inIntermission": true
        }"#;

        let clock: GameClock = serde_json::from_str(json).unwrap();
        assert_eq!(clock.time_remaining, "05:30");
        assert_eq!(clock.seconds_remaining, 330);
        assert!(!clock.running);
        assert!(clock.in_intermission);
    }

    #[test]
    fn test_game_clock_end_of_period() {
        let json = r#"{
            "timeRemaining": "00:00",
            "secondsRemaining": 0,
            "running": false,
            "inIntermission": true
        }"#;

        let clock: GameClock = serde_json::from_str(json).unwrap();
        assert_eq!(clock.time_remaining, "00:00");
        assert_eq!(clock.seconds_remaining, 0);
        assert!(!clock.running);
        assert!(clock.in_intermission);
    }

    #[test]
    fn test_boxscore_with_special_event() {
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
                "number": 1,
                "periodType": "REG",
                "maxRegulationPeriods": 3
            },
            "specialEvent": {
                "parentId": 1000,
                "name": {"default": "Stadium Series"},
                "lightLogoUrl": {"default": "https://example.com/stadium.png"}
            },
            "awayTeam": {
                "id": 1,
                "commonName": {"default": "Devils"},
                "abbrev": "NJD",
                "score": 0,
                "sog": 0,
                "logo": "https://assets.nhle.com/logos/nhl/svg/NJD_light.svg",
                "darkLogo": "https://assets.nhle.com/logos/nhl/svg/NJD_dark.svg",
                "placeName": {"default": "New Jersey"},
                "placeNameWithPreposition": {"default": "New Jersey"}
            },
            "homeTeam": {
                "id": 7,
                "commonName": {"default": "Sabres"},
                "abbrev": "BUF",
                "score": 0,
                "sog": 0,
                "logo": "https://assets.nhle.com/logos/nhl/svg/BUF_light.svg",
                "darkLogo": "https://assets.nhle.com/logos/nhl/svg/BUF_dark.svg",
                "placeName": {"default": "Buffalo"},
                "placeNameWithPreposition": {"default": "Buffalo"}
            },
            "clock": {
                "timeRemaining": "20:00",
                "secondsRemaining": 1200,
                "running": false,
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
        assert!(boxscore.special_event.is_some());
        let event = boxscore.special_event.unwrap();
        assert_eq!(event.name.default, "Stadium Series");
    }

    #[test]
    fn test_boxscore_with_tv_broadcasts() {
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
            "tvBroadcasts": [
                {
                    "id": 1,
                    "market": "NATIONAL",
                    "countryCode": "US",
                    "network": "ESPN",
                    "sequenceNumber": 1
                },
                {
                    "id": 2,
                    "market": "AWAY",
                    "countryCode": "US",
                    "network": "MSG",
                    "sequenceNumber": 2
                }
            ],
            "gameState": "LIVE",
            "gameScheduleState": "OK",
            "periodDescriptor": {
                "number": 1,
                "periodType": "REG",
                "maxRegulationPeriods": 3
            },
            "awayTeam": {
                "id": 1,
                "commonName": {"default": "Devils"},
                "abbrev": "NJD",
                "score": 0,
                "sog": 0,
                "logo": "https://assets.nhle.com/logos/nhl/svg/NJD_light.svg",
                "darkLogo": "https://assets.nhle.com/logos/nhl/svg/NJD_dark.svg",
                "placeName": {"default": "New Jersey"},
                "placeNameWithPreposition": {"default": "New Jersey"}
            },
            "homeTeam": {
                "id": 7,
                "commonName": {"default": "Sabres"},
                "abbrev": "BUF",
                "score": 0,
                "sog": 0,
                "logo": "https://assets.nhle.com/logos/nhl/svg/BUF_light.svg",
                "darkLogo": "https://assets.nhle.com/logos/nhl/svg/BUF_dark.svg",
                "placeName": {"default": "Buffalo"},
                "placeNameWithPreposition": {"default": "Buffalo"}
            },
            "clock": {
                "timeRemaining": "20:00",
                "secondsRemaining": 1200,
                "running": false,
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
        assert_eq!(boxscore.tv_broadcasts.len(), 2);
        assert_eq!(boxscore.tv_broadcasts[0].network, "ESPN");
        assert_eq!(boxscore.tv_broadcasts[1].network, "MSG");
    }

    #[test]
    fn test_goalie_stats_missing_optional_fields() {
        let json = r#"{
            "playerId": 8475123,
            "sweaterNumber": 30,
            "name": {"default": "J. Doe"},
            "position": "G",
            "evenStrengthShotsAgainst": "0/0",
            "powerPlayShotsAgainst": "0/0",
            "shorthandedShotsAgainst": "0/0",
            "saveShotsAgainst": "0/0",
            "evenStrengthGoalsAgainst": 0,
            "powerPlayGoalsAgainst": 0,
            "shorthandedGoalsAgainst": 0,
            "goalsAgainst": 0,
            "toi": "00:00",
            "shotsAgainst": 0,
            "saves": 0
        }"#;

        let stats: GoalieStats = serde_json::from_str(json).unwrap();
        assert_eq!(stats.player_id, 8475123);
        assert_eq!(stats.save_pctg, None);
        assert_eq!(stats.pim, None);
        assert_eq!(stats.starter, None);
        assert_eq!(stats.decision, None);
    }

    #[test]
    fn test_team_player_stats_deserialization() {
        let json = r#"{
            "forwards": [
                {
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
                }
            ],
            "defense": [],
            "goalies": []
        }"#;

        let stats: TeamPlayerStats = serde_json::from_str(json).unwrap();
        assert_eq!(stats.forwards.len(), 1);
        assert_eq!(stats.defense.len(), 0);
        assert_eq!(stats.goalies.len(), 0);
        assert_eq!(stats.forwards[0].player_id, 8480002);
    }

    #[test]
    fn test_team_player_stats_empty_arrays() {
        let json = r#"{
            "forwards": [],
            "defense": [],
            "goalies": []
        }"#;

        let stats: TeamPlayerStats = serde_json::from_str(json).unwrap();
        assert_eq!(stats.forwards.len(), 0);
        assert_eq!(stats.defense.len(), 0);
        assert_eq!(stats.goalies.len(), 0);
    }

    #[test]
    fn test_team_player_stats_missing_arrays() {
        let json = r#"{}"#;

        let stats: TeamPlayerStats = serde_json::from_str(json).unwrap();
        assert_eq!(stats.forwards.len(), 0);
        assert_eq!(stats.defense.len(), 0);
        assert_eq!(stats.goalies.len(), 0);
    }

    #[test]
    fn test_team_game_stats_from_empty_team() {
        let team_stats = TeamPlayerStats {
            forwards: vec![],
            defense: vec![],
            goalies: vec![],
        };

        let game_stats = TeamGameStats::from_team_player_stats(&team_stats);
        assert_eq!(game_stats.shots_on_goal, 0);
        assert_eq!(game_stats.hits, 0);
        assert_eq!(game_stats.penalty_minutes, 0);
    }

    #[test]
    fn test_team_game_stats_from_skaters() {
        let team_stats = TeamPlayerStats {
            forwards: vec![SkaterStats {
                player_id: 1,
                sweater_number: 13,
                name: LocalizedString {
                    default: "Player 1".to_string(),
                },
                position: Position::Center,
                goals: 1,
                assists: 2,
                points: 3,
                plus_minus: 1,
                pim: 2,
                hits: 5,
                power_play_goals: 1,
                sog: 4,
                faceoff_winning_pctg: 0.6,
                toi: "18:00".to_string(),
                blocked_shots: 2,
                shifts: 25,
                giveaways: 1,
                takeaways: 3,
            }],
            defense: vec![SkaterStats {
                player_id: 2,
                sweater_number: 44,
                name: LocalizedString {
                    default: "Player 2".to_string(),
                },
                position: Position::Defense,
                goals: 0,
                assists: 1,
                points: 1,
                plus_minus: 0,
                pim: 4,
                hits: 8,
                power_play_goals: 0,
                sog: 3,
                faceoff_winning_pctg: 0.0,
                toi: "22:00".to_string(),
                blocked_shots: 5,
                shifts: 30,
                giveaways: 2,
                takeaways: 1,
            }],
            goalies: vec![],
        };

        let game_stats = TeamGameStats::from_team_player_stats(&team_stats);
        assert_eq!(game_stats.shots_on_goal, 7); // 4 + 3
        assert_eq!(game_stats.hits, 13); // 5 + 8
        assert_eq!(game_stats.penalty_minutes, 6); // 2 + 4
        assert_eq!(game_stats.power_play_goals, 1);
        assert_eq!(game_stats.blocked_shots, 7); // 2 + 5
        assert_eq!(game_stats.giveaways, 3); // 1 + 2
        assert_eq!(game_stats.takeaways, 4); // 3 + 1
    }

    #[test]
    fn test_team_game_stats_with_goalies() {
        let team_stats = TeamPlayerStats {
            forwards: vec![],
            defense: vec![],
            goalies: vec![GoalieStats {
                player_id: 1,
                sweater_number: 35,
                name: LocalizedString {
                    default: "Goalie 1".to_string(),
                },
                position: Position::Goalie,
                even_strength_shots_against: "20/22".to_string(),
                power_play_shots_against: "3/5".to_string(),
                shorthanded_shots_against: "0/0".to_string(),
                save_shots_against: "23/27".to_string(),
                save_pctg: Some(0.852),
                even_strength_goals_against: 2,
                power_play_goals_against: 2,
                shorthanded_goals_against: 0,
                pim: Some(2),
                goals_against: 4,
                toi: "60:00".to_string(),
                starter: Some(true),
                decision: Some(GoalieDecision::Loss),
                shots_against: 27,
                saves: 23,
            }],
        };

        let game_stats = TeamGameStats::from_team_player_stats(&team_stats);
        assert_eq!(game_stats.penalty_minutes, 2);
        assert_eq!(game_stats.power_play_opportunities, 2);
    }

    #[test]
    fn test_team_game_stats_faceoff_percentage_zero_faceoffs() {
        let game_stats = TeamGameStats {
            shots_on_goal: 30,
            faceoff_wins: 0,
            faceoff_total: 0,
            power_play_goals: 1,
            power_play_opportunities: 4,
            penalty_minutes: 8,
            hits: 25,
            blocked_shots: 15,
            giveaways: 5,
            takeaways: 7,
        };

        assert_eq!(game_stats.faceoff_percentage(), 0.0);
    }

    #[test]
    fn test_team_game_stats_faceoff_percentage() {
        let game_stats = TeamGameStats {
            shots_on_goal: 30,
            faceoff_wins: 30,
            faceoff_total: 60,
            power_play_goals: 1,
            power_play_opportunities: 4,
            penalty_minutes: 8,
            hits: 25,
            blocked_shots: 15,
            giveaways: 5,
            takeaways: 7,
        };

        assert_eq!(game_stats.faceoff_percentage(), 50.0);
    }

    #[test]
    fn test_team_game_stats_power_play_percentage_zero_opportunities() {
        let game_stats = TeamGameStats {
            shots_on_goal: 30,
            faceoff_wins: 30,
            faceoff_total: 60,
            power_play_goals: 0,
            power_play_opportunities: 0,
            penalty_minutes: 8,
            hits: 25,
            blocked_shots: 15,
            giveaways: 5,
            takeaways: 7,
        };

        assert_eq!(game_stats.power_play_percentage(), 0.0);
    }

    #[test]
    fn test_team_game_stats_power_play_percentage() {
        let game_stats = TeamGameStats {
            shots_on_goal: 30,
            faceoff_wins: 30,
            faceoff_total: 60,
            power_play_goals: 2,
            power_play_opportunities: 5,
            penalty_minutes: 8,
            hits: 25,
            blocked_shots: 15,
            giveaways: 5,
            takeaways: 7,
        };

        assert_eq!(game_stats.power_play_percentage(), 40.0);
    }

    #[test]
    fn test_skater_stats_zero_values() {
        let json = r#"{
            "playerId": 8480000,
            "sweaterNumber": 99,
            "name": {"default": "Test Player"},
            "position": "LW",
            "goals": 0,
            "assists": 0,
            "points": 0,
            "plusMinus": 0,
            "pim": 0,
            "hits": 0,
            "powerPlayGoals": 0,
            "sog": 0,
            "faceoffWinningPctg": 0.0,
            "toi": "00:00",
            "blockedShots": 0,
            "shifts": 0,
            "giveaways": 0,
            "takeaways": 0
        }"#;

        let stats: SkaterStats = serde_json::from_str(json).unwrap();
        assert_eq!(stats.goals, 0);
        assert_eq!(stats.assists, 0);
        assert_eq!(stats.sog, 0);
        assert_eq!(stats.faceoff_winning_pctg, 0.0);
    }

    #[test]
    fn test_skater_stats_negative_plus_minus() {
        let json = r#"{
            "playerId": 8480000,
            "sweaterNumber": 99,
            "name": {"default": "Test Player"},
            "position": "RW",
            "goals": 1,
            "assists": 0,
            "points": 1,
            "plusMinus": -3,
            "pim": 2,
            "hits": 5,
            "powerPlayGoals": 0,
            "sog": 3,
            "faceoffWinningPctg": 0.0,
            "toi": "12:30",
            "blockedShots": 0,
            "shifts": 18,
            "giveaways": 2,
            "takeaways": 0
        }"#;

        let stats: SkaterStats = serde_json::from_str(json).unwrap();
        assert_eq!(stats.plus_minus, -3);
    }
}
