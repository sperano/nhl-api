use super::*;
fn edge_cases() -> Vec<EdgeCase> {
    vec![
        EdgeCase {
            name: "edge_skater_detail",
            path: format!(
                "/edge/skater-detail/{}/{}/{}",
                EDGE_TEST_PLAYER_ID,
                edge_test_season().to_api_string(),
                EDGE_TEST_GAME_TYPE.to_int()
            ),
            call: edge_skater_detail_call,
        },
        EdgeCase {
            name: "edge_skater_speed_detail",
            path: format!(
                "/edge/skater-skating-speed-detail/{}/{}/{}",
                EDGE_TEST_PLAYER_ID,
                edge_test_season().to_api_string(),
                EDGE_TEST_GAME_TYPE.to_int()
            ),
            call: edge_skater_speed_detail_call,
        },
        EdgeCase {
            name: "edge_skater_distance_detail",
            path: format!(
                "/edge/skater-skating-distance-detail/{}/{}/{}",
                EDGE_TEST_PLAYER_ID,
                edge_test_season().to_api_string(),
                EDGE_TEST_GAME_TYPE.to_int()
            ),
            call: edge_skater_distance_detail_call,
        },
        EdgeCase {
            name: "edge_skater_shot_speed_detail",
            path: format!(
                "/edge/skater-shot-speed-detail/{}/{}/{}",
                EDGE_TEST_PLAYER_ID,
                edge_test_season().to_api_string(),
                EDGE_TEST_GAME_TYPE.to_int()
            ),
            call: edge_skater_shot_speed_detail_call,
        },
        EdgeCase {
            name: "edge_skater_shot_location_detail",
            path: format!(
                "/edge/skater-shot-location-detail/{}/{}/{}",
                EDGE_TEST_PLAYER_ID,
                edge_test_season().to_api_string(),
                EDGE_TEST_GAME_TYPE.to_int()
            ),
            call: edge_skater_shot_location_detail_call,
        },
        EdgeCase {
            name: "edge_skater_zone_time",
            path: format!(
                "/edge/skater-zone-time/{}/{}/{}",
                EDGE_TEST_PLAYER_ID,
                edge_test_season().to_api_string(),
                EDGE_TEST_GAME_TYPE.to_int()
            ),
            call: edge_skater_zone_time_call,
        },
        EdgeCase {
            name: "edge_skater_comparison",
            path: format!(
                "/edge/skater-comparison/{}/{}/{}",
                EDGE_TEST_PLAYER_ID,
                edge_test_season().to_api_string(),
                EDGE_TEST_GAME_TYPE.to_int()
            ),
            call: edge_skater_comparison_call,
        },
        EdgeCase {
            name: "edge_skater_landing",
            path: format!(
                "/edge/skater-landing/{}/{}",
                edge_test_season().to_api_string(),
                EDGE_TEST_GAME_TYPE.to_int()
            ),
            call: edge_skater_landing_call,
        },
        EdgeCase {
            name: "edge_goalie_detail",
            path: format!(
                "/edge/goalie-detail/{}/{}/{}",
                EDGE_TEST_PLAYER_ID,
                edge_test_season().to_api_string(),
                EDGE_TEST_GAME_TYPE.to_int()
            ),
            call: edge_goalie_detail_call,
        },
        EdgeCase {
            name: "edge_goalie_5v5_detail",
            path: format!(
                "/edge/goalie-5v5-detail/{}/{}/{}",
                EDGE_TEST_PLAYER_ID,
                edge_test_season().to_api_string(),
                EDGE_TEST_GAME_TYPE.to_int()
            ),
            call: edge_goalie_5v5_detail_call,
        },
        EdgeCase {
            name: "edge_goalie_shot_location_detail",
            path: format!(
                "/edge/goalie-shot-location-detail/{}/{}/{}",
                EDGE_TEST_PLAYER_ID,
                edge_test_season().to_api_string(),
                EDGE_TEST_GAME_TYPE.to_int()
            ),
            call: edge_goalie_shot_location_detail_call,
        },
        EdgeCase {
            name: "edge_goalie_save_pctg_detail",
            path: format!(
                "/edge/goalie-save-percentage-detail/{}/{}/{}",
                EDGE_TEST_PLAYER_ID,
                edge_test_season().to_api_string(),
                EDGE_TEST_GAME_TYPE.to_int()
            ),
            call: edge_goalie_save_pctg_detail_call,
        },
        EdgeCase {
            name: "edge_goalie_comparison",
            path: format!(
                "/edge/goalie-comparison/{}/{}/{}",
                EDGE_TEST_PLAYER_ID,
                edge_test_season().to_api_string(),
                EDGE_TEST_GAME_TYPE.to_int()
            ),
            call: edge_goalie_comparison_call,
        },
        EdgeCase {
            name: "edge_goalie_landing",
            path: format!(
                "/edge/goalie-landing/{}/{}",
                edge_test_season().to_api_string(),
                EDGE_TEST_GAME_TYPE.to_int()
            ),
            call: edge_goalie_landing_call,
        },
        EdgeCase {
            name: "edge_team_detail",
            path: format!(
                "/edge/team-detail/{}/{}/{}",
                EDGE_TEST_TEAM_ID,
                edge_test_season().to_api_string(),
                EDGE_TEST_GAME_TYPE.to_int()
            ),
            call: edge_team_detail_call,
        },
        EdgeCase {
            name: "edge_team_speed_detail",
            path: format!(
                "/edge/team-skating-speed-detail/{}/{}/{}",
                EDGE_TEST_TEAM_ID,
                edge_test_season().to_api_string(),
                EDGE_TEST_GAME_TYPE.to_int()
            ),
            call: edge_team_speed_detail_call,
        },
        EdgeCase {
            name: "edge_team_distance_detail",
            path: format!(
                "/edge/team-skating-distance-detail/{}/{}/{}",
                EDGE_TEST_TEAM_ID,
                edge_test_season().to_api_string(),
                EDGE_TEST_GAME_TYPE.to_int()
            ),
            call: edge_team_distance_detail_call,
        },
        EdgeCase {
            name: "edge_team_shot_speed_detail",
            path: format!(
                "/edge/team-shot-speed-detail/{}/{}/{}",
                EDGE_TEST_TEAM_ID,
                edge_test_season().to_api_string(),
                EDGE_TEST_GAME_TYPE.to_int()
            ),
            call: edge_team_shot_speed_detail_call,
        },
        EdgeCase {
            name: "edge_team_shot_location_detail",
            path: format!(
                "/edge/team-shot-location-detail/{}/{}/{}",
                EDGE_TEST_TEAM_ID,
                edge_test_season().to_api_string(),
                EDGE_TEST_GAME_TYPE.to_int()
            ),
            call: edge_team_shot_location_detail_call,
        },
        EdgeCase {
            name: "edge_team_zone_time_details",
            path: format!(
                "/edge/team-zone-time-details/{}/{}/{}",
                EDGE_TEST_TEAM_ID,
                edge_test_season().to_api_string(),
                EDGE_TEST_GAME_TYPE.to_int()
            ),
            call: edge_team_zone_time_details_call,
        },
        EdgeCase {
            name: "edge_team_comparison",
            path: format!(
                "/edge/team-comparison/{}/{}/{}",
                EDGE_TEST_TEAM_ID,
                edge_test_season().to_api_string(),
                EDGE_TEST_GAME_TYPE.to_int()
            ),
            call: edge_team_comparison_call,
        },
        EdgeCase {
            name: "edge_team_landing",
            path: format!(
                "/edge/team-landing/{}/{}",
                edge_test_season().to_api_string(),
                EDGE_TEST_GAME_TYPE.to_int()
            ),
            call: edge_team_landing_call,
        },
    ]
}

/// Verifies every Edge client method requests the documented URL path.
/// Each case is run against a mock server that only answers on the exact
/// expected path with `200 {}` -- an empty JSON object deserializes cleanly
/// into every Edge response type (the 6.2 `{}`-deserializes rule), and a
/// request to any other path leaves the mock unmatched, failing the request.
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
