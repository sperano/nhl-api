use super::Client;
use crate::error::NHLApiError;
use crate::http_client::Endpoint;
use crate::types::{ClubStats, Franchise, FranchisesResponse, GameType, Roster, SeasonGameTypes};

impl Client {
    /// Gets a list of all NHL franchises (past and current).
    ///
    /// Returns information about every franchise including historical/defunct teams.
    /// Each franchise includes the franchise ID, full name, common name, and place name.
    pub async fn franchises(&self) -> Result<Vec<Franchise>, NHLApiError> {
        let response: FranchisesResponse = self
            .client
            .get_json(Endpoint::ApiStats, "en/franchise", None)
            .await?;
        Ok(response.data)
    }

    /// Gets player statistics for a team in a specific season.
    ///
    /// Returns skater and goalie statistics for all players on the team during the specified
    /// season and game type.
    ///
    /// # Arguments
    /// * `team_abbr` - Team abbreviation (e.g., "MTL", "TOR", "BUF")
    /// * `season` - Season in YYYYYYYY format (e.g., 20242025)
    /// * `game_type` - Game type (RegularSeason, Playoffs, etc.)
    ///
    /// # Example
    /// ```no_run
    /// # use nhl_api::{Client, GameType};
    /// # async fn example() -> Result<(), nhl_api::NHLApiError> {
    /// let client = Client::new()?;
    /// let stats = client.club_stats("MTL", 20242025, GameType::RegularSeason).await?;
    /// println!("Skaters: {}, Goalies: {}", stats.skaters.len(), stats.goalies.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn club_stats(
        &self,
        team_abbr: &str,
        season: i32,
        game_type: GameType,
    ) -> Result<ClubStats, NHLApiError> {
        self.client
            .get_json(
                Endpoint::ApiWebV1,
                &format!("club-stats/{}/{}/{}", team_abbr, season, game_type.to_int()),
                None,
            )
            .await
    }

    /// Gets available seasons and game types for a team.
    ///
    /// Returns a list of all seasons the team has data for, along with the available
    /// game types (regular season, playoffs) for each season.
    ///
    /// # Arguments
    /// * `team_abbr` - Team abbreviation (e.g., "MTL", "TOR", "BUF")
    ///
    /// # Example
    /// ```no_run
    /// # use nhl_api::Client;
    /// # async fn example() -> Result<(), nhl_api::NHLApiError> {
    /// let client = Client::new()?;
    /// let seasons = client.club_stats_season("MTL").await?;
    /// for season in seasons {
    ///     println!("{}", season); // Displays "20242025: Regular Season, Playoffs"
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn club_stats_season(
        &self,
        team_abbr: &str,
    ) -> Result<Vec<SeasonGameTypes>, NHLApiError> {
        self.client
            .get_json(
                Endpoint::ApiWebV1,
                &format!("club-stats-season/{}", team_abbr),
                None,
            )
            .await
    }

    /// Gets the current roster for a team.
    ///
    /// # Arguments
    /// * `team_abbr` - Team abbreviation (e.g., "MTL", "TOR", "BUF")
    pub async fn roster_current(&self, team_abbr: &str) -> Result<Roster, NHLApiError> {
        self.client
            .get_json(
                Endpoint::ApiWebV1,
                &format!("roster/{}/current", team_abbr),
                None,
            )
            .await
    }

    /// Gets the roster for a team in a specific season.
    ///
    /// # Arguments
    /// * `team_abbr` - Team abbreviation (e.g., "MTL", "TOR", "BUF")
    /// * `season` - Season in YYYYYYYY format (e.g., 20242025)
    pub async fn roster_season(&self, team_abbr: &str, season: i32) -> Result<Roster, NHLApiError> {
        self.client
            .get_json(
                Endpoint::ApiWebV1,
                &format!("roster/{}/{}", team_abbr, season),
                None,
            )
            .await
    }
}
