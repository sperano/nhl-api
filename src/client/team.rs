use super::Client;
use crate::date::Season;
use crate::error::NHLApiError;
use crate::http_client::Endpoint;
use crate::team_abbrev::TeamAbbrev;
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
    /// * `team_abbr` - Team abbreviation (e.g., "MTL", "TOR", "BUF"), validated as a [`TeamAbbrev`]
    /// * `season` - The season to fetch, e.g. `Season::new(2024)` for 2024-25
    /// * `game_type` - Game type (RegularSeason, Playoffs, etc.)
    ///
    /// # Example
    /// ```no_run
    /// # use nhl_api::{Client, GameType, Season};
    /// # async fn example() -> Result<(), nhl_api::NHLApiError> {
    /// let client = Client::new()?;
    /// let stats = client
    ///     .club_stats("MTL", Season::new(2024), GameType::RegularSeason)
    ///     .await?;
    /// println!("Skaters: {}, Goalies: {}", stats.skaters.len(), stats.goalies.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn club_stats(
        &self,
        team_abbr: impl AsRef<str>,
        season: Season,
        game_type: GameType,
    ) -> Result<ClubStats, NHLApiError> {
        let team = TeamAbbrev::new(team_abbr.as_ref())?;
        self.client
            .get_json(
                Endpoint::ApiWebV1,
                &format!(
                    "club-stats/{}/{}/{}",
                    team,
                    season.to_api_string(),
                    game_type.to_int()
                ),
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
    /// * `team_abbr` - Team abbreviation (e.g., "MTL", "TOR", "BUF"), validated as a [`TeamAbbrev`]
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
        team_abbr: impl AsRef<str>,
    ) -> Result<Vec<SeasonGameTypes>, NHLApiError> {
        let team = TeamAbbrev::new(team_abbr.as_ref())?;
        self.client
            .get_json(
                Endpoint::ApiWebV1,
                &format!("club-stats-season/{}", team),
                None,
            )
            .await
    }

    /// Gets the current roster for a team.
    ///
    /// # Arguments
    /// * `team_abbr` - Team abbreviation (e.g., "MTL", "TOR", "BUF"), validated as a [`TeamAbbrev`]
    pub async fn roster_current(&self, team_abbr: impl AsRef<str>) -> Result<Roster, NHLApiError> {
        let team = TeamAbbrev::new(team_abbr.as_ref())?;
        self.client
            .get_json(
                Endpoint::ApiWebV1,
                &format!("roster/{}/current", team),
                None,
            )
            .await
    }

    /// Gets the roster for a team in a specific season.
    ///
    /// # Arguments
    /// * `team_abbr` - Team abbreviation (e.g., "MTL", "TOR", "BUF"), validated as a [`TeamAbbrev`]
    /// * `season` - The season to fetch, e.g. `Season::new(2024)` for 2024-25
    pub async fn roster_season(
        &self,
        team_abbr: impl AsRef<str>,
        season: Season,
    ) -> Result<Roster, NHLApiError> {
        let team = TeamAbbrev::new(team_abbr.as_ref())?;
        self.client
            .get_json(
                Endpoint::ApiWebV1,
                &format!("roster/{}/{}", team, season.to_api_string()),
                None,
            )
            .await
    }

    /// Gets a team's prospect pool: players whose rights the team holds who
    /// are not on its roster.
    ///
    /// Prospects without an NHL number have no
    /// [`sweater_number`](crate::RosterPlayer::sweater_number). A defunct or
    /// relocated team (e.g. "ARI") answers an empty pool.
    ///
    /// # Arguments
    /// * `team_abbr` - Team abbreviation (e.g., "MTL", "TOR", "BUF"), validated as a [`TeamAbbrev`]
    pub async fn team_prospects(&self, team_abbr: impl AsRef<str>) -> Result<Roster, NHLApiError> {
        let team = TeamAbbrev::new(team_abbr.as_ref())?;
        self.client
            .get_json(Endpoint::ApiWebV1, &format!("prospects/{}", team), None)
            .await
    }
}
