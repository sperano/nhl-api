use std::collections::HashMap;

use serde::de::DeserializeOwned;

use super::Client;
use crate::error::NHLApiError;
use crate::http_client::Endpoint;
use crate::ids::GameId;
use crate::types::{Boxscore, GameMatchup, GameStory, PlayByPlay, SeasonSeriesMatchup, ShiftChart};

impl Client {
    /// Fetch data from a game-center endpoint.
    async fn fetch_gamecenter<T: DeserializeOwned>(
        &self,
        game_id: impl Into<GameId>,
        resource: &str,
    ) -> Result<T, NHLApiError> {
        let game_id = game_id.into();
        self.client
            .get_json(
                Endpoint::ApiWebV1,
                &format!("gamecenter/{}/{}", game_id, resource),
                None,
            )
            .await
    }

    /// Returns the boxscore for a game.
    ///
    /// # Errors
    /// Returns an [`NHLApiError`] if the request fails or the response cannot
    /// be deserialized.
    pub async fn boxscore(&self, game_id: impl Into<GameId>) -> Result<Boxscore, NHLApiError> {
        self.fetch_gamecenter(game_id, "boxscore").await
    }

    /// Returns play-by-play data for a game.
    ///
    /// # Errors
    /// Returns an [`NHLApiError`] if the request fails or the response cannot
    /// be deserialized.
    pub async fn play_by_play(
        &self,
        game_id: impl Into<GameId>,
    ) -> Result<PlayByPlay, NHLApiError> {
        self.fetch_gamecenter(game_id, "play-by-play").await
    }

    /// Fetch game landing data (lighter than play-by-play, includes summary with period scores).
    pub async fn landing(&self, game_id: impl Into<GameId>) -> Result<GameMatchup, NHLApiError> {
        self.fetch_gamecenter(game_id, "landing").await
    }

    /// Fetch season series matchup data including head-to-head records.
    pub async fn season_series(
        &self,
        game_id: impl Into<GameId>,
    ) -> Result<SeasonSeriesMatchup, NHLApiError> {
        self.fetch_gamecenter(game_id, "right-rail").await
    }

    /// Fetch game story narrative content.
    pub async fn game_story(&self, game_id: impl Into<GameId>) -> Result<GameStory, NHLApiError> {
        let game_id = game_id.into();
        self.client
            .get_json(
                Endpoint::ApiWebV1,
                &format!("wsc/game-story/{}", game_id),
                None,
            )
            .await
    }

    /// Fetch shift chart data for a game.
    pub async fn shift_chart(&self, game_id: impl Into<GameId>) -> Result<ShiftChart, NHLApiError> {
        let game_id = game_id.into();
        let cayenne_expression = format!(
            "gameId={} and ((duration != '00:00' and typeCode = 517) or typeCode != 517 )",
            game_id
        );
        let mut params = HashMap::new();
        params.insert("cayenneExp".to_string(), cayenne_expression);
        params.insert("exclude".to_string(), "eventDetails".to_string());

        self.client
            .get_json(Endpoint::ApiStats, "en/shiftcharts", Some(params))
            .await
    }
}
