use super::Client;
use crate::date::{GameDate, Season};
use crate::error::NHLApiError;
use crate::http_client::Endpoint;
use crate::types::{SeasonInfo, SeasonsResponse, Standing, StandingsResponse, Team};

impl Client {
    /// Returns all NHL teams as of a given date, derived from the standings
    /// endpoint so a historical date returns the teams active at that time.
    ///
    /// # Arguments
    /// * `date` - Optional [`GameDate`]. If `None`, defaults to the current date.
    ///
    /// # Errors
    /// Returns an [`NHLApiError`] if the standings request fails or the
    /// response cannot be deserialized.
    pub async fn teams(&self, date: Option<GameDate>) -> Result<Vec<Team>, NHLApiError> {
        let date = Self::resolve_date_or(date, GameDate::default());
        let standings_response = self.fetch_standings_data(&date.to_api_string()).await?;
        let teams = standings_response
            .standings
            .iter()
            .map(Standing::to_team)
            .collect();

        Ok(teams)
    }

    async fn fetch_standings_data(&self, date: &str) -> Result<StandingsResponse, NHLApiError> {
        self.client
            .get_json(Endpoint::ApiWebV1, &format!("standings/{}", date), None)
            .await
    }

    /// Returns the current league standings (all teams, current date).
    ///
    /// # Errors
    /// Returns an [`NHLApiError`] if the standings request fails or the
    /// response cannot be deserialized.
    pub async fn current_league_standings(&self) -> Result<Vec<Standing>, NHLApiError> {
        self.league_standings_for_date(&GameDate::default()).await
    }

    /// Returns the league standings for a specific date.
    ///
    /// # Errors
    /// Returns an [`NHLApiError`] if the standings request fails or the
    /// response cannot be deserialized.
    pub async fn league_standings_for_date(
        &self,
        date: &GameDate,
    ) -> Result<Vec<Standing>, NHLApiError> {
        Ok(self
            .fetch_standings_data(&date.to_api_string())
            .await?
            .standings)
    }

    /// Returns the league standings at the end of a season.
    ///
    /// # Errors
    /// Returns an [`NHLApiError`] if the season manifest cannot be fetched, the
    /// season is not in it, or the standings request fails.
    pub async fn league_standings_for_season(
        &self,
        season: Season,
    ) -> Result<Vec<Standing>, NHLApiError> {
        let seasons = self.season_standing_manifest().await?;
        let season_data = seasons
            .iter()
            .find(|info| info.id == season)
            .ok_or_else(|| NHLApiError::Other(format!("Invalid Season Id {}", season.id())))?;
        Ok(self
            .fetch_standings_data(&season_data.standings_end)
            .await?
            .standings)
    }

    /// Gets metadata for all NHL seasons.
    ///
    /// Returns information about every season including start date, end date, etc.
    pub async fn season_standing_manifest(&self) -> Result<Vec<SeasonInfo>, NHLApiError> {
        let response: SeasonsResponse = self
            .client
            .get_json(Endpoint::ApiWebV1, "standings-season", None)
            .await?;
        Ok(response.seasons)
    }
}
