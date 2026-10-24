use super::Client;
use crate::date::Season;
use crate::error::NHLApiError;
use crate::http_client::Endpoint;
use crate::ids::{PlayerId, TeamId};
use crate::types::{
    EdgeGoalie5v5Detail, EdgeGoalieComparison, EdgeGoalieDetail, EdgeGoalieLanding,
    EdgeGoalieSavePctgDetail, EdgeGoalieShotLocationDetail, EdgeSkaterComparison, EdgeSkaterDetail,
    EdgeSkaterDistanceDetail, EdgeSkaterLanding, EdgeSkaterShotLocationDetail,
    EdgeSkaterShotSpeedDetail, EdgeSkaterSpeedDetail, EdgeSkaterZoneTimeDetail, EdgeTeamComparison,
    EdgeTeamDetail, EdgeTeamDistanceDetail, EdgeTeamLanding, EdgeTeamShotLocationDetail,
    EdgeTeamShotSpeedDetail, EdgeTeamSpeedDetail, EdgeTeamZoneTimeDetails, GameType,
    TeamScheduleResponse,
};

macro_rules! client_endpoint_pair {
    (
        schedule;
        $(#[$meta:meta])*
        $method:ident => $method_at:ident
    ) => {
        $(#[$meta])*
        pub async fn $method(
            &self,
            team_abbr: &str,
            season: Season,
        ) -> Result<TeamScheduleResponse, NHLApiError> {
            self.$method_at(Endpoint::ApiWebV1, team_abbr, season).await
        }

        pub(super) async fn $method_at(
            &self,
            endpoint: Endpoint,
            team_abbr: &str,
            season: Season,
        ) -> Result<TeamScheduleResponse, NHLApiError> {
            self.client
                .get_json(
                    endpoint,
                    &format!(
                        "club-schedule-season/{}/{}",
                        team_abbr,
                        season.to_api_string()
                    ),
                    None,
                )
                .await
        }
    };
    (
        edge_id;
        $(#[$meta:meta])*
        $method:ident => $method_at:ident;
        $id:ident: $id_type:ty;
        $response:ty;
        $path:literal
    ) => {
        $(#[$meta])*
        pub async fn $method(
            &self,
            $id: impl Into<$id_type>,
            season: Season,
            game_type: GameType,
        ) -> Result<$response, NHLApiError> {
            self.$method_at(Endpoint::ApiWebV1, $id, season, game_type)
                .await
        }

        pub(super) async fn $method_at(
            &self,
            endpoint: Endpoint,
            $id: impl Into<$id_type>,
            season: Season,
            game_type: GameType,
        ) -> Result<$response, NHLApiError> {
            let $id = $id.into();
            self.client
                .get_json(
                    endpoint,
                    &format!(
                        concat!("edge/", $path, "/{}/{}/{}"),
                        $id,
                        season.to_api_string(),
                        game_type.to_int()
                    ),
                    None,
                )
                .await
        }
    };
    (
        edge_landing;
        $(#[$meta:meta])*
        $method:ident => $method_at:ident;
        $response:ty;
        $path:literal
    ) => {
        $(#[$meta])*
        pub async fn $method(
            &self,
            season: Season,
            game_type: GameType,
        ) -> Result<$response, NHLApiError> {
            self.$method_at(Endpoint::ApiWebV1, season, game_type)
                .await
        }

        pub(super) async fn $method_at(
            &self,
            endpoint: Endpoint,
            season: Season,
            game_type: GameType,
        ) -> Result<$response, NHLApiError> {
            self.client
                .get_json(
                    endpoint,
                    &format!(
                        concat!("edge/", $path, "/{}/{}"),
                        season.to_api_string(),
                        game_type.to_int()
                    ),
                    None,
                )
                .await
        }
    };
}

impl Client {
    client_endpoint_pair! {
        schedule;
        /// Gets the full schedule for a team in a given season
        ///
        /// Includes preseason, regular season, and playoff games for the team's
        /// entire season, unlike [`Self::team_weekly_schedule`] which only covers a
        /// single week.
        ///
        /// # Arguments
        /// * `team_abbr` - Team abbreviation (e.g., "MTL", "TOR", "BUF")
        /// * `season` - The NHL season to fetch the schedule for
        club_schedule_season => club_schedule_season_at
    }

    client_endpoint_pair! {
        edge_id;
        /// Gets Edge puck/player-tracking overview stats for a skater's season.
        edge_skater_detail => edge_skater_detail_at;
        player_id: PlayerId;
        EdgeSkaterDetail;
        "skater-detail"
    }

    client_endpoint_pair! {
        edge_id;
        /// Gets Edge skating-speed detail (per-game top speeds) for a skater's season.
        edge_skater_speed_detail => edge_skater_speed_detail_at;
        player_id: PlayerId;
        EdgeSkaterSpeedDetail;
        "skater-skating-speed-detail"
    }

    client_endpoint_pair! {
        edge_id;
        /// Gets Edge skating-distance detail (per-game distance skated) for a skater's season.
        edge_skater_distance_detail => edge_skater_distance_detail_at;
        player_id: PlayerId;
        EdgeSkaterDistanceDetail;
        "skater-skating-distance-detail"
    }

    client_endpoint_pair! {
        edge_id;
        /// Gets Edge shot-speed detail (hardest shots) for a skater's season.
        edge_skater_shot_speed_detail => edge_skater_shot_speed_detail_at;
        player_id: PlayerId;
        EdgeSkaterShotSpeedDetail;
        "skater-shot-speed-detail"
    }

    client_endpoint_pair! {
        edge_id;
        /// Gets Edge shot-location detail (shot breakdown by rink area) for a skater's season.
        edge_skater_shot_location_detail => edge_skater_shot_location_detail_at;
        player_id: PlayerId;
        EdgeSkaterShotLocationDetail;
        "skater-shot-location-detail"
    }

    client_endpoint_pair! {
        edge_id;
        /// Gets Edge zone-time detail (zone time breakdown by strength) for a skater's season.
        ///
        /// Note the path has no `-details` suffix, unlike the sibling detail
        /// endpoints (`edge/skater-zone-time`, not `edge/skater-zone-time-detail`).
        edge_skater_zone_time => edge_skater_zone_time_at;
        player_id: PlayerId;
        EdgeSkaterZoneTimeDetail;
        "skater-zone-time"
    }

    client_endpoint_pair! {
        edge_id;
        /// Gets the Edge head-to-head comparison composite for a skater's season.
        edge_skater_comparison => edge_skater_comparison_at;
        player_id: PlayerId;
        EdgeSkaterComparison;
        "skater-comparison"
    }

    client_endpoint_pair! {
        edge_landing;
        /// Gets the league-wide Edge skater leaderboard for a season (no player id).
        edge_skater_landing => edge_skater_landing_at;
        EdgeSkaterLanding;
        "skater-landing"
    }

    client_endpoint_pair! {
        edge_id;
        /// Gets Edge puck/player-tracking overview stats for a goalie's season.
        edge_goalie_detail => edge_goalie_detail_at;
        goalie_id: PlayerId;
        EdgeGoalieDetail;
        "goalie-detail"
    }

    client_endpoint_pair! {
        edge_id;
        /// Gets Edge 5v5 save-percentage detail (per-game) for a goalie's season.
        edge_goalie_5v5_detail => edge_goalie_5v5_detail_at;
        goalie_id: PlayerId;
        EdgeGoalie5v5Detail;
        "goalie-5v5-detail"
    }

    client_endpoint_pair! {
        edge_id;
        /// Gets Edge shot-location detail (shot breakdown by rink area) for a goalie's season.
        edge_goalie_shot_location_detail => edge_goalie_shot_location_detail_at;
        goalie_id: PlayerId;
        EdgeGoalieShotLocationDetail;
        "goalie-shot-location-detail"
    }

    client_endpoint_pair! {
        edge_id;
        /// Gets Edge save-percentage detail (per-game, plus aggregated stats) for a goalie's season.
        ///
        /// Note the path slug is spelled out (`goalie-save-percentage-detail`),
        /// not abbreviated to `goalie-save-pctg-detail`.
        edge_goalie_save_pctg_detail => edge_goalie_save_pctg_detail_at;
        goalie_id: PlayerId;
        EdgeGoalieSavePctgDetail;
        "goalie-save-percentage-detail"
    }

    client_endpoint_pair! {
        edge_id;
        /// Gets the Edge head-to-head comparison composite for a goalie's season.
        edge_goalie_comparison => edge_goalie_comparison_at;
        goalie_id: PlayerId;
        EdgeGoalieComparison;
        "goalie-comparison"
    }

    client_endpoint_pair! {
        edge_landing;
        /// Gets the league-wide Edge goalie leaderboard for a season (no goalie id).
        edge_goalie_landing => edge_goalie_landing_at;
        EdgeGoalieLanding;
        "goalie-landing"
    }

    client_endpoint_pair! {
        edge_id;
        /// Gets Edge puck/player-tracking overview stats for a team's season.
        ///
        /// Team Edge stats are rank-based (1-32), unlike the percentile-based
        /// skater/goalie stats.
        edge_team_detail => edge_team_detail_at;
        team_id: TeamId;
        EdgeTeamDetail;
        "team-detail"
    }

    client_endpoint_pair! {
        edge_id;
        /// Gets Edge skating-speed detail (per-player top speeds) for a team's season.
        edge_team_speed_detail => edge_team_speed_detail_at;
        team_id: TeamId;
        EdgeTeamSpeedDetail;
        "team-skating-speed-detail"
    }

    client_endpoint_pair! {
        edge_id;
        /// Gets Edge skating-distance detail (per-game distance skated) for a team's season.
        edge_team_distance_detail => edge_team_distance_detail_at;
        team_id: TeamId;
        EdgeTeamDistanceDetail;
        "team-skating-distance-detail"
    }

    client_endpoint_pair! {
        edge_id;
        /// Gets Edge shot-speed detail (per-player hardest shots) for a team's season.
        edge_team_shot_speed_detail => edge_team_shot_speed_detail_at;
        team_id: TeamId;
        EdgeTeamShotSpeedDetail;
        "team-shot-speed-detail"
    }

    client_endpoint_pair! {
        edge_id;
        /// Gets Edge shot-location detail (shot breakdown by rink area) for a team's season.
        edge_team_shot_location_detail => edge_team_shot_location_detail_at;
        team_id: TeamId;
        EdgeTeamShotLocationDetail;
        "team-shot-location-detail"
    }

    client_endpoint_pair! {
        edge_id;
        /// Gets Edge zone-time detail (zone time by strength code, with shot
        /// differential) for a team's season.
        ///
        /// Distinct from the zone-time summary embedded in [`EdgeTeamDetail`].
        /// Note the path has a `-details` suffix, unlike the skater equivalent
        /// (`edge/team-zone-time-details`, not `edge/team-zone-time`).
        edge_team_zone_time_details => edge_team_zone_time_details_at;
        team_id: TeamId;
        EdgeTeamZoneTimeDetails;
        "team-zone-time-details"
    }

    client_endpoint_pair! {
        edge_id;
        /// Gets the Edge head-to-head comparison composite for a team's season.
        edge_team_comparison => edge_team_comparison_at;
        team_id: TeamId;
        EdgeTeamComparison;
        "team-comparison"
    }

    client_endpoint_pair! {
        edge_landing;
        /// Gets the league-wide Edge team leaderboard for a season (no team id).
        edge_team_landing => edge_team_landing_at;
        EdgeTeamLanding;
        "team-landing"
    }
}
