use std::future::Future;
use std::pin::Pin;

use super::super::Client;
use crate::date::Season;
use crate::error::NHLApiError;
use crate::http_client::Endpoint;
use crate::ids::{PlayerId, TeamId};
use crate::types::GameType;

type EdgeMethodFuture<'a> = Pin<Box<dyn Future<Output = Result<(), NHLApiError>> + 'a>>;
type EdgeMethodFn = for<'a> fn(&'a Client, Endpoint) -> EdgeMethodFuture<'a>;

struct EdgeCase {
    name: &'static str,
    path: String,
    call: EdgeMethodFn,
}

const EDGE_TEST_PLAYER_ID: i64 = 8478402;
const EDGE_TEST_TEAM_ID: i64 = 22;
const EDGE_TEST_GAME_TYPE: GameType = GameType::RegularSeason;

fn edge_test_season() -> Season {
    Season::new(2024)
}

macro_rules! define_edge_call {
    (player; $call:ident => $method_at:ident) => {
        fn $call(client: &Client, endpoint: Endpoint) -> EdgeMethodFuture<'_> {
            Box::pin(async move {
                client
                    .$method_at(
                        endpoint,
                        PlayerId::new(EDGE_TEST_PLAYER_ID),
                        edge_test_season(),
                        EDGE_TEST_GAME_TYPE,
                    )
                    .await
                    .map(|_| ())
            })
        }
    };
    (team; $call:ident => $method_at:ident) => {
        fn $call(client: &Client, endpoint: Endpoint) -> EdgeMethodFuture<'_> {
            Box::pin(async move {
                client
                    .$method_at(
                        endpoint,
                        TeamId::new(EDGE_TEST_TEAM_ID),
                        edge_test_season(),
                        EDGE_TEST_GAME_TYPE,
                    )
                    .await
                    .map(|_| ())
            })
        }
    };
    (landing; $call:ident => $method_at:ident) => {
        fn $call(client: &Client, endpoint: Endpoint) -> EdgeMethodFuture<'_> {
            Box::pin(async move {
                client
                    .$method_at(endpoint, edge_test_season(), EDGE_TEST_GAME_TYPE)
                    .await
                    .map(|_| ())
            })
        }
    };
}

macro_rules! edge_case {
    (player; $call:ident; $path:literal) => {
        EdgeCase {
            name: stringify!($call),
            path: format!(
                concat!("/edge/", $path, "/{}/{}/{}"),
                EDGE_TEST_PLAYER_ID,
                edge_test_season().to_api_string(),
                EDGE_TEST_GAME_TYPE.to_int()
            ),
            call: $call,
        }
    };
    (team; $call:ident; $path:literal) => {
        EdgeCase {
            name: stringify!($call),
            path: format!(
                concat!("/edge/", $path, "/{}/{}/{}"),
                EDGE_TEST_TEAM_ID,
                edge_test_season().to_api_string(),
                EDGE_TEST_GAME_TYPE.to_int()
            ),
            call: $call,
        }
    };
    (landing; $call:ident; $path:literal) => {
        EdgeCase {
            name: stringify!($call),
            path: format!(
                concat!("/edge/", $path, "/{}/{}"),
                edge_test_season().to_api_string(),
                EDGE_TEST_GAME_TYPE.to_int()
            ),
            call: $call,
        }
    };
}

macro_rules! define_edge_cases {
    ($( $kind:ident; $call:ident => $method_at:ident; $path:literal ),+ $(,)?) => {
        $(define_edge_call!($kind; $call => $method_at);)+

        fn edge_cases() -> Vec<EdgeCase> {
            vec![$(edge_case!($kind; $call; $path)),+]
        }
    };
}

// This single table generates both the endpoint calls and their expected paths.
// Adding an Edge method therefore requires only one corresponding test entry.
define_edge_cases! {
    player; edge_skater_detail => edge_skater_detail_at; "skater-detail",
    player; edge_skater_speed_detail => edge_skater_speed_detail_at; "skater-skating-speed-detail",
    player; edge_skater_distance_detail => edge_skater_distance_detail_at; "skater-skating-distance-detail",
    player; edge_skater_shot_speed_detail => edge_skater_shot_speed_detail_at; "skater-shot-speed-detail",
    player; edge_skater_shot_location_detail => edge_skater_shot_location_detail_at; "skater-shot-location-detail",
    player; edge_skater_zone_time => edge_skater_zone_time_at; "skater-zone-time",
    player; edge_skater_comparison => edge_skater_comparison_at; "skater-comparison",
    landing; edge_skater_landing => edge_skater_landing_at; "skater-landing",
    player; edge_goalie_detail => edge_goalie_detail_at; "goalie-detail",
    player; edge_goalie_5v5_detail => edge_goalie_5v5_detail_at; "goalie-5v5-detail",
    player; edge_goalie_shot_location_detail => edge_goalie_shot_location_detail_at; "goalie-shot-location-detail",
    player; edge_goalie_save_pctg_detail => edge_goalie_save_pctg_detail_at; "goalie-save-percentage-detail",
    player; edge_goalie_comparison => edge_goalie_comparison_at; "goalie-comparison",
    landing; edge_goalie_landing => edge_goalie_landing_at; "goalie-landing",
    team; edge_team_detail => edge_team_detail_at; "team-detail",
    team; edge_team_speed_detail => edge_team_speed_detail_at; "team-skating-speed-detail",
    team; edge_team_distance_detail => edge_team_distance_detail_at; "team-skating-distance-detail",
    team; edge_team_shot_speed_detail => edge_team_shot_speed_detail_at; "team-shot-speed-detail",
    team; edge_team_shot_location_detail => edge_team_shot_location_detail_at; "team-shot-location-detail",
    team; edge_team_zone_time_details => edge_team_zone_time_details_at; "team-zone-time-details",
    team; edge_team_comparison => edge_team_comparison_at; "team-comparison",
    landing; edge_team_landing => edge_team_landing_at; "team-landing",
}

/// Verifies every Edge client method requests the documented URL path.
#[tokio::test]
async fn test_edge_all_client_methods_path_contract() {
    for case in edge_cases() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", case.path.as_str())
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body("{}")
            .create_async()
            .await;

        let client = Client::new().unwrap();
        let result = (case.call)(&client, Endpoint::Custom(server.url())).await;

        assert!(
            result.is_ok(),
            "{} did not request the expected path {:?}: {:?}",
            case.name,
            case.path,
            result.err()
        );
        mock.assert_async().await;
    }
}

/// Verifies every Edge client method propagates a 404 as
/// `NHLApiError::ResourceNotFound` rather than swallowing or mis-mapping it.
#[tokio::test]
async fn test_edge_all_client_methods_404_propagates_error() {
    for case in edge_cases() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", case.path.as_str())
            .with_status(404)
            .create_async()
            .await;

        let client = Client::new().unwrap();
        let result = (case.call)(&client, Endpoint::Custom(server.url())).await;

        match result {
            Err(NHLApiError::ResourceNotFound { .. }) => {}
            Err(other) => panic!("{} expected ResourceNotFound, got {:?}", case.name, other),
            Ok(_) => panic!("{} expected an error for 404, got Ok", case.name),
        }
        mock.assert_async().await;
    }
}
