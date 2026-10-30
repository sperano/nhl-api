use super::super::{Client, DEFAULT_SEARCH_LIMIT};
use crate::config::ClientConfig;
use crate::date::{GameDate, Season};
use crate::error::NHLApiError;
use crate::http_client::Endpoint;
use crate::ids::{GameId, TeamId};
use crate::team_abbrev::TeamAbbrev;
use crate::types::{GameType, WeeklyScheduleResponse};
use chrono::NaiveDate;

// ===== Client Construction Tests =====

#[test]
fn test_client_new() {
    let client = Client::new();
    assert!(client.is_ok());
}

#[test]
fn test_client_with_default_config() {
    let config = ClientConfig::default();
    let client = Client::with_config(config);
    assert!(client.is_ok());
}

#[test]
fn test_client_with_custom_config() {
    let config = ClientConfig {
        timeout: std::time::Duration::from_secs(60),
        follow_redirects: false,
        ssl_verify: true,
        ..Default::default()
    };
    let client = Client::with_config(config);
    assert!(client.is_ok());
}

// ===== Helper Method Tests =====

#[test]
fn test_resolve_date_or_with_some() {
    let date = GameDate::Date(NaiveDate::from_ymd_opt(2024, 1, 15).unwrap());
    let resolved = Client::resolve_date_or(Some(date.clone()), GameDate::default());
    assert_eq!(resolved.to_api_string(), "2024-01-15");
}

#[test]
fn test_resolve_date_or_with_none_default_now() {
    let resolved = Client::resolve_date_or(None, GameDate::default());
    assert_eq!(resolved.to_api_string(), "now");
}

#[test]
fn test_resolve_date_or_with_some_today() {
    let date = GameDate::Date(NaiveDate::from_ymd_opt(2024, 1, 15).unwrap());
    let resolved = Client::resolve_date_or(Some(date.clone()), GameDate::today());
    assert_eq!(resolved.to_api_string(), "2024-01-15");
}

#[test]
fn test_resolve_date_or_with_none_default_today() {
    let resolved = Client::resolve_date_or(None, GameDate::today());
    // Should be today's date, not "now"
    assert_ne!(resolved.to_api_string(), "now");
}

#[tokio::test]
async fn test_search_player_defaults_limit_to_twenty() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/search/player")
        .match_query(mockito::Matcher::UrlEncoded(
            "limit".into(),
            DEFAULT_SEARCH_LIMIT.to_string(),
        ))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body("[]")
        .create_async()
        .await;

    let client = Client::new().unwrap();
    let result = client
        .search_player_at(Endpoint::Custom(server.url()), "gretzky", None)
        .await;

    assert!(result.is_ok(), "search should succeed: {:?}", result.err());
    mock.assert_async().await;
}

#[test]
fn test_extract_daily_schedule_found() {
    let client = Client::new().unwrap();

    let weekly_response = WeeklyScheduleResponse {
        next_start_date: Some("2024-01-15".to_string()),
        previous_start_date: Some("2024-01-01".to_string()),
        game_week: vec![crate::types::schedule::GameDay {
            date: "2024-01-08".to_string(),
            games: vec![],
        }],
    };

    let result = client.extract_daily_schedule(weekly_response, "2024-01-08".to_string());

    assert_eq!(result.date, "2024-01-08");
    assert_eq!(result.number_of_games, 0); // games vec is empty
    assert_eq!(result.next_start_date, Some("2024-01-15".to_string()));
    assert_eq!(result.previous_start_date, Some("2024-01-01".to_string()));
}

#[test]
fn test_extract_daily_schedule_not_found() {
    let client = Client::new().unwrap();

    let weekly_response = WeeklyScheduleResponse {
        next_start_date: Some("2024-01-15".to_string()),
        previous_start_date: Some("2024-01-01".to_string()),
        game_week: vec![crate::types::schedule::GameDay {
            date: "2024-01-08".to_string(),
            games: vec![],
        }],
    };

    let result = client.extract_daily_schedule(weekly_response, "2024-01-09".to_string());

    assert_eq!(result.date, "2024-01-09");
    assert_eq!(result.number_of_games, 0);
    assert!(result.games.is_empty());
}

#[test]
fn test_extract_daily_schedule_with_games() {
    use crate::types::enums::GameScheduleState;
    use crate::types::game_state::GameState;
    use crate::types::schedule::{ScheduleGame, ScheduleTeam};

    let client = Client::new().unwrap();

    let weekly_response = WeeklyScheduleResponse {
        next_start_date: Some("2024-01-15".to_string()),
        previous_start_date: Some("2024-01-01".to_string()),
        game_week: vec![crate::types::schedule::GameDay {
            date: "2024-01-08".to_string(),
            games: vec![ScheduleGame {
                id: GameId::new(2023020001),
                game_type: GameType::RegularSeason,
                game_date: Some("2024-01-08".to_string()),
                start_time_utc: "2024-01-08T23:00:00Z".to_string(),
                away_team: ScheduleTeam {
                    id: TeamId::new(8),
                    abbrev: "MTL".to_string(),
                    logo: "logo.png".to_string(),
                    score: Some(2),
                    place_name: None,
                },
                home_team: ScheduleTeam {
                    id: TeamId::new(6),
                    abbrev: "BOS".to_string(),
                    logo: "logo.png".to_string(),
                    score: Some(3),
                    place_name: None,
                },
                game_state: GameState::Final,
                game_schedule_state: GameScheduleState::Ok,
            }],
        }],
    };

    let result = client.extract_daily_schedule(weekly_response, "2024-01-08".to_string());

    assert_eq!(result.date, "2024-01-08");
    assert_eq!(result.number_of_games, 1);
    assert_eq!(result.games.len(), 1);
    assert_eq!(result.games[0].id, GameId::new(2023020001));
}

#[test]
fn test_extract_daily_schedule_empty_game_week() {
    let client = Client::new().unwrap();

    let weekly_response = WeeklyScheduleResponse {
        next_start_date: Some("2024-01-15".to_string()),
        previous_start_date: Some("2024-01-01".to_string()),
        game_week: vec![],
    };

    let result = client.extract_daily_schedule(weekly_response, "2024-01-08".to_string());

    assert_eq!(result.date, "2024-01-08");
    assert_eq!(result.number_of_games, 0);
    assert!(result.games.is_empty());
}

// ===== Into<GameId> Support Tests =====

#[test]
fn test_into_game_id_accepts_i64() {
    // Verify that methods accepting impl Into<GameId> work with raw i64
    let game_id_i64: i64 = 2023020001;

    // This should compile and convert i64 to GameId automatically
    // We're just testing the type conversion, not the actual API call
    let converted: GameId = game_id_i64.into();
    assert_eq!(converted.as_i64(), 2023020001);
}

#[test]
fn test_game_id_from_i64() {
    // Verify that GameId can be created from i64
    let game_id = GameId::from(2023020001);
    assert_eq!(game_id.as_i64(), 2023020001);
}

// ===== club_schedule_season Tests =====

#[tokio::test]
async fn test_club_schedule_season_requests_exact_path() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/club-schedule-season/FLA/20232024")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"games": []}"#)
        .create_async()
        .await;

    let client = Client::new().unwrap();
    let result = client
        .club_schedule_season_at(Endpoint::Custom(server.url()), "FLA", Season::new(2023))
        .await;

    assert!(result.is_ok(), "request should succeed: {:?}", result.err());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_club_schedule_season_deserializes_fixture() {
    use crate::types::game_state::GameState;

    // Fixture ported from the Go client's TestClubScheduleSeason
    // (nhl-api-go/nhl/client_test.go): a regular-season game and a
    // playoff game, both completed, on the same date.
    let fixture = r#"{
            "games": [
                {
                    "id": 2023020001,
                    "gameType": 2,
                    "gameDate": "2024-04-20",
                    "startTimeUTC": "2024-04-20T23:00:00Z",
                    "awayTeam": {
                        "id": 13,
                        "abbrev": "FLA",
                        "logo": "https://assets.nhle.com/logos/nhl/svg/FLA_light.svg"
                    },
                    "homeTeam": {
                        "id": 10,
                        "abbrev": "TOR",
                        "logo": "https://assets.nhle.com/logos/nhl/svg/TOR_light.svg"
                    },
                    "gameState": "OFF",
                    "gameScheduleState": "OK"
                },
                {
                    "id": 2023030111,
                    "gameType": 3,
                    "gameDate": "2024-04-20",
                    "startTimeUTC": "2024-04-20T23:00:00Z",
                    "awayTeam": {
                        "id": 13,
                        "abbrev": "FLA",
                        "logo": "https://assets.nhle.com/logos/nhl/svg/FLA_light.svg"
                    },
                    "homeTeam": {
                        "id": 10,
                        "abbrev": "TOR",
                        "logo": "https://assets.nhle.com/logos/nhl/svg/TOR_light.svg"
                    },
                    "gameState": "OFF",
                    "gameScheduleState": "OK"
                }
            ]
        }"#;

    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/club-schedule-season/FLA/20232024")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(fixture)
        .create_async()
        .await;

    let client = Client::new().unwrap();
    let result = client
        .club_schedule_season_at(Endpoint::Custom(server.url()), "FLA", Season::new(2023))
        .await
        .expect("deserialization should succeed");

    mock.assert_async().await;
    assert_eq!(result.games.len(), 2);
    assert_eq!(result.games[0].game_type, GameType::RegularSeason);
    assert_eq!(result.games[0].game_state, GameState::Off);
    assert_eq!(result.games[1].game_type, GameType::Playoffs);
    assert_eq!(result.games[1].id, GameId::new(2023030111));
}

// ===== Team abbreviation validation Tests =====

#[tokio::test]
async fn test_club_schedule_season_normalizes_team_abbrev() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/club-schedule-season/FLA/20232024")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"games": []}"#)
        .expect(2)
        .create_async()
        .await;

    let client = Client::new().unwrap();
    let endpoint = || Endpoint::Custom(server.url());
    let abbrev = TeamAbbrev::new("FLA").unwrap();
    for result in [
        client
            .club_schedule_season_at(endpoint(), "fla", Season::new(2023))
            .await,
        client
            .club_schedule_season_at(endpoint(), &abbrev, Season::new(2023))
            .await,
    ] {
        assert!(result.is_ok(), "request should succeed: {:?}", result.err());
    }
    mock.assert_async().await;
}

/// Every team-abbreviation method rejects a path-changing argument before
/// sending a request (the default endpoint is never reached).
#[tokio::test]
async fn test_team_abbrev_methods_reject_invalid_abbrev() {
    const BAD: &str = "../MTL";
    let client = Client::new().unwrap();
    let season = Season::new(2023);
    let errors = [
        client
            .club_stats(BAD, season, GameType::RegularSeason)
            .await
            .err(),
        client.club_stats_season(BAD).await.err(),
        client.roster_current(BAD).await.err(),
        client.roster_season(BAD, season).await.err(),
        client.team_weekly_schedule(BAD, None).await.err(),
        client.club_schedule_season(BAD, season).await.err(),
    ];
    for error in errors {
        match error {
            Some(NHLApiError::InvalidTeamAbbrev(err)) => assert_eq!(err.value(), BAD),
            other => panic!("expected InvalidTeamAbbrev, got {other:?}"),
        }
    }
}
