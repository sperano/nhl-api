use crate::config::ClientConfig;
use anyhow::{Result};
use crate::date::GameDate;
use crate::error::NHLApiError;
use crate::http_client::{Endpoint, HttpClient};
use crate::ids::GameId;
use crate::types::{Boxscore, Team, StandingsResponse, SeasonInfo, SeasonsResponse, Standing, DailySchedule, WeeklyScheduleResponse, PlayByPlay, GameMatchup};

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

    /// Create a new NHL client with debug logging enabled
    pub fn with_debug() -> Result<Self> {
        let mut config = ClientConfig::default();
        config.debug = true;
        Self::with_config(config)
    }

    pub async fn teams(&self, date: Option<&GameDate>) -> Result<Vec<Team>> {
        let date = date.cloned().unwrap_or_default();
        let standings_response = self.fetch_standings_data(&date.to_api_string()).await?;
        let teams: Vec<Team> = standings_response
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

    pub async fn play_by_play(&self, game_id: &GameId) -> Result<PlayByPlay> {
        self.client
            .get_json(
                Endpoint::ApiWebV1,
                &format!("gamecenter/{}/play-by-play", game_id),
                None,
            )
            .await
    }

    /// Fetch game landing data (lighter than play-by-play, includes summary with period scores)
    pub async fn landing(&self, game_id: &GameId) -> Result<GameMatchup> {
        self.client
            .get_json(
                Endpoint::ApiWebV1,
                &format!("gamecenter/{}/landing", game_id),
                None,
            )
            .await
    }

    pub async fn daily_schedule(&self, date: Option<&GameDate>) -> Result<DailySchedule> {
        let date = date.cloned().unwrap_or_else(GameDate::today);
        let date_string = date.to_api_string();

        let schedule_data: WeeklyScheduleResponse = self
            .client
            .get_json(
                Endpoint::ApiWebV1,
                &format!("schedule/{}", date_string),
                None,
            )
            .await?;

        let mut games = Vec::new();

        if let Some(matching_day) = schedule_data
            .game_week
            .iter()
            .find(|day| day.date == date_string)
        {
            games = matching_day.games.clone();
        }

        Ok(DailySchedule {
            next_start_date: Some(schedule_data.next_start_date),
            previous_start_date: Some(schedule_data.previous_start_date),
            date: date_string.clone(),
            number_of_games: games.len(),
            games,
        })
    }

    /// Gets NHL schedule for a week starting from the specified date.
    ///
    /// # Arguments
    /// * `date` - Optional GameDate. If None, defaults to "now".
    pub async fn weekly_schedule(&self, date: Option<&GameDate>) -> Result<WeeklyScheduleResponse> {
        let date = date.cloned().unwrap_or_default();
        self.client
            .get_json(
                Endpoint::ApiWebV1,
                &format!("schedule/{}", date.to_api_string()),
                None,
            )
            .await
    }

}

