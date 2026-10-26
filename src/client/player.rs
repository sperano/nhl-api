use std::collections::HashMap;

use super::{Client, DEFAULT_SEARCH_LIMIT};
use crate::error::NHLApiError;
use crate::http_client::Endpoint;
use crate::ids::PlayerId;
use crate::types::{GameType, PlayerGameLog, PlayerLanding, PlayerSearchResult};

impl Client {
    /// Gets comprehensive player profile data including biography, stats, and career history.
    ///
    /// # Arguments
    /// * `player_id` - NHL player ID (7-digit integer)
    pub async fn player_landing(
        &self,
        player_id: impl Into<PlayerId>,
    ) -> Result<PlayerLanding, NHLApiError> {
        let player_id = player_id.into();
        self.client
            .get_json(
                Endpoint::ApiWebV1,
                &format!("player/{}/landing", player_id),
                None,
            )
            .await
    }

    /// Gets game-by-game log for a player's season.
    ///
    /// # Arguments
    /// * `player_id` - NHL player ID
    /// * `season` - Season in YYYYYYYY format (e.g., 20232024)
    /// * `game_type` - Game type (RegularSeason, Playoffs, etc.)
    pub async fn player_game_log(
        &self,
        player_id: impl Into<PlayerId>,
        season: i32,
        game_type: GameType,
    ) -> Result<PlayerGameLog, NHLApiError> {
        let player_id = player_id.into();
        let mut game_log: PlayerGameLog = self
            .client
            .get_json(
                Endpoint::ApiWebV1,
                &format!(
                    "player/{}/game-log/{}/{}",
                    player_id,
                    season,
                    game_type.to_int()
                ),
                None,
            )
            .await?;
        // The API doesn't include player_id in the response, so set it from the parameter.
        game_log.player_id = player_id;
        Ok(game_log)
    }

    /// Search for players by name.
    ///
    /// # Arguments
    /// * `query` - Search query (player name or partial name)
    /// * `limit` - Maximum number of results to return (defaults to
    ///   `DEFAULT_SEARCH_LIMIT` when `None`)
    pub async fn search_player(
        &self,
        query: &str,
        limit: Option<i32>,
    ) -> Result<Vec<PlayerSearchResult>, NHLApiError> {
        self.search_player_at(Endpoint::SearchV1, query, limit)
            .await
    }

    /// Endpoint-parameterized core of [`Self::search_player`], split out so the
    /// query-building (notably the `DEFAULT_SEARCH_LIMIT` fallback) can be
    /// exercised against a mock server.
    pub(super) async fn search_player_at(
        &self,
        endpoint: Endpoint,
        query: &str,
        limit: Option<i32>,
    ) -> Result<Vec<PlayerSearchResult>, NHLApiError> {
        let mut params = HashMap::new();
        params.insert("culture".to_string(), "en-us".to_string());
        params.insert("q".to_string(), query.to_string());
        params.insert(
            "limit".to_string(),
            limit.unwrap_or(DEFAULT_SEARCH_LIMIT).to_string(),
        );

        self.client
            .get_json(endpoint, "search/player", Some(params))
            .await
    }
}
