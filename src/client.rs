use std::sync::Arc;
use crate::config::ClientConfig;
use anyhow::{Result, Context};
use crate::date::GameDate;
use crate::error::NHLApiError;
use crate::http_client::{Endpoint, HttpClient};
use crate::ids::GameId;
use crate::types::{Boxscore, Team, StandingsResponse, SeasonInfo, SeasonsResponse, Standing};

pub struct Client {
    client: HttpClient,
}

impl Client {
    /// Create a new NHL client with default configuration
    pub fn new() -> Result<Self> {
        Self::with_config(ClientConfig::default())
    }

    /// Create a new NHL client with custom configuration
    pub fn with_config(config: ClientConfig) -> Result<Self> {
        //let http_client = Arc::new(HttpClient::new(config));
        Ok(Self {
            client: HttpClient::new(config)?,
        })
    }

    // TODO
    // /// Create a new NHL client with debug logging enabled
    // pub fn with_debug() -> Self {
    //     Self::with_config(ClientConfig::builder().debug(true).build())
    // }

    pub async fn teams(&self /*, date: Option<&GameDate>*/) -> Result<Vec<Team>> {
        //let date = date.cloned().unwrap_or_default();
        let date = GameDate::default();
        let standings_response = self.fetch_standings_data(&date.to_api_string()).await?;
        let mut teams: Vec<Team> = standings_response
            .standings
            .iter()
            .map(|standing| standing.to_team())
            .collect();

        // self.enrich_teams_with_franchise_ids(&mut teams).await?;
        Ok(teams)
    }

    async fn fetch_standings_data(&self, date: &str) -> Result<StandingsResponse> {
        self.client
            .get_json(Endpoint::ApiWebV1, &format!("standings/{}", date), None)
            .await
    }

    pub async fn current_league_standings(&self) -> Result<Vec<Standing>> {
        self.league_standings_for_date(&GameDate::default()).await
    }

    pub async fn league_standings_for_date(&self, date: &GameDate) -> Result<Vec<Standing>> {
        Ok(self.fetch_standings_data(&date.to_api_string()).await?.standings)
    }

    pub async fn league_standings_for_season(&self, season_id: i64) -> Result<Vec<Standing>> {
        let seasons = self.season_standing_manifest().await?;
        let season_data = seasons
            .iter()
            .find(|s| s.id == season_id)
            .ok_or_else(|| NHLApiError::Other(format!("Invalid Season Id {}", season_id)))?;
        let date = season_data.standings_end.clone();
        Ok(self.fetch_standings_data(date.as_str()).await?.standings)
    }

    /// Gets metadata for all NHL seasons.
    ///
    /// Returns information about every season including start date, end date, etc.
    pub async fn season_standing_manifest(&self) -> Result<Vec<SeasonInfo>> {
        let response: SeasonsResponse = self
            .client
            .get_json(Endpoint::ApiWebV1, "standings-season", None)
            .await?;
        Ok(response.seasons)
    }

    pub async fn boxscore(&self, game_id: &GameId) -> Result<Boxscore> {
        self.client
            .get_json(
                Endpoint::ApiWebV1,
                &format!("gamecenter/{}/boxscore", game_id),
                None,
            )
            .await
    }

}

