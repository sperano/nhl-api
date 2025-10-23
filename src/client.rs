use std::sync::Arc;
use crate::config::ClientConfig;
use anyhow::{Result, Context};
use crate::http_client::{Endpoint, HttpClient};
use crate::types::{Team, StandingsResponse};
use crate::date::GameDate;

pub struct Client {
    client: HttpClient,
    // pub teams: Teams,
    // pub standings: Standings,
    // pub schedule: Schedule,
    // pub game_center: GameCenter,
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
            // teams: Teams::new(Arc::clone(&http_client)),
            // standings: Standings::new(Arc::clone(&http_client)),
            // schedule: Schedule::new(Arc::clone(&http_client)),
            // game_center: GameCenter::new(Arc::clone(&http_client)),
        })
    }

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


}

