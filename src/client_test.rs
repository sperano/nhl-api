use super::*;
use crate::date::Season;
use crate::ids::TeamId;
use std::future::Future;
use std::pin::Pin;

#[path = "client/core_test.rs"]
mod core;
#[path = "client/edge_contract_test.rs"]
mod edge_contract;

// ===== Edge contract tables =====
//
// Every Edge client method is exercised by both tables below via a single
// `EdgeCase` entry. Adding a 23rd Edge method without adding a call fn +
// entry here is conspicuous: the method itself compiles fine, but neither
// the path-contract test nor the 404-propagation test will cover it.

type EdgeMethodFuture<'a> = Pin<Box<dyn Future<Output = Result<(), NHLApiError>> + 'a>>;
type EdgeMethodFn = for<'a> fn(&'a Client, Endpoint) -> EdgeMethodFuture<'a>;

struct EdgeCase {
    name: &'static str,
    path: String,
    call: EdgeMethodFn,
}

// Shared fixtures: 2024-2025 regular season -> APIString "20242025", GameType.to_int() 2.
const EDGE_TEST_PLAYER_ID: i64 = 8478402;
const EDGE_TEST_TEAM_ID: i64 = 22;

fn edge_test_season() -> Season {
    Season::new(2024)
}

const EDGE_TEST_GAME_TYPE: GameType = GameType::RegularSeason;

fn edge_skater_detail_call(client: &Client, endpoint: Endpoint) -> EdgeMethodFuture<'_> {
    Box::pin(async move {
        client
            .edge_skater_detail_at(
                endpoint,
                PlayerId::new(EDGE_TEST_PLAYER_ID),
                edge_test_season(),
                EDGE_TEST_GAME_TYPE,
            )
            .await
            .map(|_| ())
    })
}

fn edge_skater_speed_detail_call(client: &Client, endpoint: Endpoint) -> EdgeMethodFuture<'_> {
    Box::pin(async move {
        client
            .edge_skater_speed_detail_at(
                endpoint,
                PlayerId::new(EDGE_TEST_PLAYER_ID),
                edge_test_season(),
                EDGE_TEST_GAME_TYPE,
            )
            .await
            .map(|_| ())
    })
}

fn edge_skater_distance_detail_call(client: &Client, endpoint: Endpoint) -> EdgeMethodFuture<'_> {
    Box::pin(async move {
        client
            .edge_skater_distance_detail_at(
                endpoint,
                PlayerId::new(EDGE_TEST_PLAYER_ID),
                edge_test_season(),
                EDGE_TEST_GAME_TYPE,
            )
            .await
            .map(|_| ())
    })
}

fn edge_skater_shot_speed_detail_call(client: &Client, endpoint: Endpoint) -> EdgeMethodFuture<'_> {
    Box::pin(async move {
        client
            .edge_skater_shot_speed_detail_at(
                endpoint,
                PlayerId::new(EDGE_TEST_PLAYER_ID),
                edge_test_season(),
                EDGE_TEST_GAME_TYPE,
            )
            .await
            .map(|_| ())
    })
}

fn edge_skater_shot_location_detail_call(
    client: &Client,
    endpoint: Endpoint,
) -> EdgeMethodFuture<'_> {
    Box::pin(async move {
        client
            .edge_skater_shot_location_detail_at(
                endpoint,
                PlayerId::new(EDGE_TEST_PLAYER_ID),
                edge_test_season(),
                EDGE_TEST_GAME_TYPE,
            )
            .await
            .map(|_| ())
    })
}

fn edge_skater_zone_time_call(client: &Client, endpoint: Endpoint) -> EdgeMethodFuture<'_> {
    Box::pin(async move {
        client
            .edge_skater_zone_time_at(
                endpoint,
                PlayerId::new(EDGE_TEST_PLAYER_ID),
                edge_test_season(),
                EDGE_TEST_GAME_TYPE,
            )
            .await
            .map(|_| ())
    })
}

fn edge_skater_comparison_call(client: &Client, endpoint: Endpoint) -> EdgeMethodFuture<'_> {
    Box::pin(async move {
        client
            .edge_skater_comparison_at(
                endpoint,
                PlayerId::new(EDGE_TEST_PLAYER_ID),
                edge_test_season(),
                EDGE_TEST_GAME_TYPE,
            )
            .await
            .map(|_| ())
    })
}

fn edge_skater_landing_call(client: &Client, endpoint: Endpoint) -> EdgeMethodFuture<'_> {
    Box::pin(async move {
        client
            .edge_skater_landing_at(endpoint, edge_test_season(), EDGE_TEST_GAME_TYPE)
            .await
            .map(|_| ())
    })
}

fn edge_goalie_detail_call(client: &Client, endpoint: Endpoint) -> EdgeMethodFuture<'_> {
    Box::pin(async move {
        client
            .edge_goalie_detail_at(
                endpoint,
                PlayerId::new(EDGE_TEST_PLAYER_ID),
                edge_test_season(),
                EDGE_TEST_GAME_TYPE,
            )
            .await
            .map(|_| ())
    })
}

fn edge_goalie_5v5_detail_call(client: &Client, endpoint: Endpoint) -> EdgeMethodFuture<'_> {
    Box::pin(async move {
        client
            .edge_goalie_5v5_detail_at(
                endpoint,
                PlayerId::new(EDGE_TEST_PLAYER_ID),
                edge_test_season(),
                EDGE_TEST_GAME_TYPE,
            )
            .await
            .map(|_| ())
    })
}

fn edge_goalie_shot_location_detail_call(
    client: &Client,
    endpoint: Endpoint,
) -> EdgeMethodFuture<'_> {
    Box::pin(async move {
        client
            .edge_goalie_shot_location_detail_at(
                endpoint,
                PlayerId::new(EDGE_TEST_PLAYER_ID),
                edge_test_season(),
                EDGE_TEST_GAME_TYPE,
            )
            .await
            .map(|_| ())
    })
}

fn edge_goalie_save_pctg_detail_call(client: &Client, endpoint: Endpoint) -> EdgeMethodFuture<'_> {
    Box::pin(async move {
        client
            .edge_goalie_save_pctg_detail_at(
                endpoint,
                PlayerId::new(EDGE_TEST_PLAYER_ID),
                edge_test_season(),
                EDGE_TEST_GAME_TYPE,
            )
            .await
            .map(|_| ())
    })
}

fn edge_goalie_comparison_call(client: &Client, endpoint: Endpoint) -> EdgeMethodFuture<'_> {
    Box::pin(async move {
        client
            .edge_goalie_comparison_at(
                endpoint,
                PlayerId::new(EDGE_TEST_PLAYER_ID),
                edge_test_season(),
                EDGE_TEST_GAME_TYPE,
            )
            .await
            .map(|_| ())
    })
}

fn edge_goalie_landing_call(client: &Client, endpoint: Endpoint) -> EdgeMethodFuture<'_> {
    Box::pin(async move {
        client
            .edge_goalie_landing_at(endpoint, edge_test_season(), EDGE_TEST_GAME_TYPE)
            .await
            .map(|_| ())
    })
}

fn edge_team_detail_call(client: &Client, endpoint: Endpoint) -> EdgeMethodFuture<'_> {
    Box::pin(async move {
        client
            .edge_team_detail_at(
                endpoint,
                TeamId::new(EDGE_TEST_TEAM_ID),
                edge_test_season(),
                EDGE_TEST_GAME_TYPE,
            )
            .await
            .map(|_| ())
    })
}

fn edge_team_speed_detail_call(client: &Client, endpoint: Endpoint) -> EdgeMethodFuture<'_> {
    Box::pin(async move {
        client
            .edge_team_speed_detail_at(
                endpoint,
                TeamId::new(EDGE_TEST_TEAM_ID),
                edge_test_season(),
                EDGE_TEST_GAME_TYPE,
            )
            .await
            .map(|_| ())
    })
}

fn edge_team_distance_detail_call(client: &Client, endpoint: Endpoint) -> EdgeMethodFuture<'_> {
    Box::pin(async move {
        client
            .edge_team_distance_detail_at(
                endpoint,
                TeamId::new(EDGE_TEST_TEAM_ID),
                edge_test_season(),
                EDGE_TEST_GAME_TYPE,
            )
            .await
            .map(|_| ())
    })
}

fn edge_team_shot_speed_detail_call(client: &Client, endpoint: Endpoint) -> EdgeMethodFuture<'_> {
    Box::pin(async move {
        client
            .edge_team_shot_speed_detail_at(
                endpoint,
                TeamId::new(EDGE_TEST_TEAM_ID),
                edge_test_season(),
                EDGE_TEST_GAME_TYPE,
            )
            .await
            .map(|_| ())
    })
}

fn edge_team_shot_location_detail_call(
    client: &Client,
    endpoint: Endpoint,
) -> EdgeMethodFuture<'_> {
    Box::pin(async move {
        client
            .edge_team_shot_location_detail_at(
                endpoint,
                TeamId::new(EDGE_TEST_TEAM_ID),
                edge_test_season(),
                EDGE_TEST_GAME_TYPE,
            )
            .await
            .map(|_| ())
    })
}

fn edge_team_zone_time_details_call(client: &Client, endpoint: Endpoint) -> EdgeMethodFuture<'_> {
    Box::pin(async move {
        client
            .edge_team_zone_time_details_at(
                endpoint,
                TeamId::new(EDGE_TEST_TEAM_ID),
                edge_test_season(),
                EDGE_TEST_GAME_TYPE,
            )
            .await
            .map(|_| ())
    })
}

fn edge_team_comparison_call(client: &Client, endpoint: Endpoint) -> EdgeMethodFuture<'_> {
    Box::pin(async move {
        client
            .edge_team_comparison_at(
                endpoint,
                TeamId::new(EDGE_TEST_TEAM_ID),
                edge_test_season(),
                EDGE_TEST_GAME_TYPE,
            )
            .await
            .map(|_| ())
    })
}

fn edge_team_landing_call(client: &Client, endpoint: Endpoint) -> EdgeMethodFuture<'_> {
    Box::pin(async move {
        client
            .edge_team_landing_at(endpoint, edge_test_season(), EDGE_TEST_GAME_TYPE)
            .await
            .map(|_| ())
    })
}
