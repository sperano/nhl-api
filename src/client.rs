use crate::config::ClientConfig;
use crate::date::GameDate;
use crate::error::NHLApiError;
use crate::http_client::{Endpoint, HttpClient};
use crate::ids::GameId;
use crate::types::{
    Boxscore, ClubStats, DailySchedule, DailyScores, Franchise, FranchisesResponse, GameMatchup,
    GameStory, PlayByPlay, PlayerGameLog, PlayerLanding, PlayerSearchResult, Roster,
    SeasonGameTypes, SeasonInfo, SeasonSeriesMatchup, SeasonsResponse, ShiftChart, Standing,
    StandingsResponse, Team, TeamScheduleResponse, WeeklyScheduleResponse,
};
use std::collections::HashMap;

pub struct Client {
    client: HttpClient,
}

impl Client {
    /// Create a new NHL client with default configuration
    pub fn new() -> Result<Self, NHLApiError> {
        Self::with_config(ClientConfig::default())
    }

    /// Create a new NHL client with custom configuration
    pub fn with_config(config: ClientConfig) -> Result<Self, NHLApiError> {
        //let http_client = Arc::new(HttpClient::new(config));
        Ok(Self {
            client: HttpClient::new(config)?,
        })
    }

    /// Resolve optional date to owned GameDate, defaulting to "now"
    fn resolve_date(&self, date: Option<GameDate>) -> GameDate {
        date.unwrap_or_default()
    }

    /// Resolve optional date to owned GameDate, defaulting to today's date
    fn resolve_date_to_today(&self, date: Option<GameDate>) -> GameDate {
        date.unwrap_or_else(GameDate::today)
    }

    pub async fn teams(&self, date: Option<GameDate>) -> Result<Vec<Team>, NHLApiError> {
        let date = self.resolve_date(date);
        let standings_response = self.fetch_standings_data(&date.to_api_string()).await?;
        let teams: Vec<Team> = standings_response
            .standings
            .iter()
            .map(|standing| standing.to_team())
            .collect();

        // self.enrich_teams_with_franchise_ids(&mut teams).await?;
        Ok(teams)
    }

    async fn fetch_standings_data(&self, date: &str) -> Result<StandingsResponse, NHLApiError> {
        self.client
            .get_json(Endpoint::ApiWebV1, &format!("standings/{}", date), None)
            .await
    }

    pub async fn current_league_standings(&self) -> Result<Vec<Standing>, NHLApiError> {
        self.league_standings_for_date(&GameDate::default()).await
    }

    pub async fn league_standings_for_date(&self, date: &GameDate) -> Result<Vec<Standing>, NHLApiError> {
        Ok(self
            .fetch_standings_data(&date.to_api_string())
            .await?
            .standings)
    }

    pub async fn league_standings_for_season(&self, season_id: i64) -> Result<Vec<Standing>, NHLApiError> {
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
    pub async fn season_standing_manifest(&self) -> Result<Vec<SeasonInfo>, NHLApiError> {
        let response: SeasonsResponse = self
            .client
            .get_json(Endpoint::ApiWebV1, "standings-season", None)
            .await?;
        Ok(response.seasons)
    }

    /// Fetch data from a gamecenter endpoint
    async fn fetch_gamecenter<T: serde::de::DeserializeOwned>(
        &self,
        game_id: &GameId,
        resource: &str,
    ) -> Result<T, NHLApiError> {
        self.client
            .get_json(
                Endpoint::ApiWebV1,
                &format!("gamecenter/{}/{}", game_id, resource),
                None,
            )
            .await
    }

    pub async fn boxscore(&self, game_id: &GameId) -> Result<Boxscore, NHLApiError> {
        self.fetch_gamecenter(game_id, "boxscore").await
    }

    pub async fn play_by_play(&self, game_id: &GameId) -> Result<PlayByPlay, NHLApiError> {
        self.fetch_gamecenter(game_id, "play-by-play").await
    }

    /// Fetch game landing data (lighter than play-by-play, includes summary with period scores)
    pub async fn landing(&self, game_id: &GameId) -> Result<GameMatchup, NHLApiError> {
        self.fetch_gamecenter(game_id, "landing").await
    }

    /// Fetch season series matchup data including head-to-head records
    pub async fn season_series(&self, game_id: &GameId) -> Result<SeasonSeriesMatchup, NHLApiError> {
        self.fetch_gamecenter(game_id, "right-rail").await
    }

    /// Fetch game story narrative content
    pub async fn game_story(&self, game_id: &GameId) -> Result<GameStory, NHLApiError> {
        self.client
            .get_json(
                Endpoint::ApiWebV1,
                &format!("wsc/game-story/{}", game_id),
                None,
            )
            .await
    }

    /// Fetch shift chart data for a game
    pub async fn shift_chart(&self, game_id: &GameId) -> Result<ShiftChart, NHLApiError> {
        let cayenne_expr = format!(
            "gameId={} and ((duration != '00:00' and typeCode = 517) or typeCode != 517 )",
            game_id
        );
        let mut params = HashMap::new();
        params.insert("cayenneExp".to_string(), cayenne_expr);
        params.insert("exclude".to_string(), "eventDetails".to_string());

        self.client
            .get_json(Endpoint::ApiStats, "en/shiftcharts", Some(params))
            .await
    }

    async fn fetch_weekly_schedule(&self, date_string: &str) -> Result<WeeklyScheduleResponse, NHLApiError> {
        self.client
            .get_json(
                Endpoint::ApiWebV1,
                &format!("schedule/{}", date_string),
                None,
            )
            .await
    }

    fn extract_daily_schedule(
        &self,
        schedule_data: WeeklyScheduleResponse,
        date_string: String,
    ) -> DailySchedule {
        let games = schedule_data
            .game_week
            .iter()
            .find(|day| day.date == date_string)
            .map(|day| day.games.clone())
            .unwrap_or_default();

        DailySchedule {
            next_start_date: Some(schedule_data.next_start_date),
            previous_start_date: Some(schedule_data.previous_start_date),
            date: date_string,
            number_of_games: games.len(),
            games,
        }
    }

    pub async fn daily_schedule(&self, date: Option<GameDate>) -> Result<DailySchedule, NHLApiError> {
        let date = self.resolve_date_to_today(date);
        let date_string = date.to_api_string();
        let schedule_data = self.fetch_weekly_schedule(&date_string).await?;
        Ok(self.extract_daily_schedule(schedule_data, date_string))
    }

    /// Gets NHL schedule for a week starting from the specified date.
    ///
    /// # Arguments
    /// * `date` - Optional GameDate. If None, defaults to "now".
    pub async fn weekly_schedule(&self, date: Option<GameDate>) -> Result<WeeklyScheduleResponse, NHLApiError> {
        let date = self.resolve_date(date);
        self.client
            .get_json(
                Endpoint::ApiWebV1,
                &format!("schedule/{}", date.to_api_string()),
                None,
            )
            .await
    }

    /// Gets comprehensive player profile data including biography, stats, and career history
    ///
    /// # Arguments
    /// * `player_id` - NHL player ID (7-digit integer)
    pub async fn player_landing(&self, player_id: i64) -> Result<PlayerLanding, NHLApiError> {
        self.client
            .get_json(
                Endpoint::ApiWebV1,
                &format!("player/{}/landing", player_id),
                None,
            )
            .await
    }

    /// Gets game-by-game log for a player's season
    ///
    /// # Arguments
    /// * `player_id` - NHL player ID
    /// * `season` - Season in YYYYYYYY format (e.g., 20232024)
    /// * `game_type` - 2 for regular season, 3 for playoffs
    pub async fn player_game_log(
        &self,
        player_id: i64,
        season: i32,
        game_type: i32,
    ) -> Result<PlayerGameLog, NHLApiError> {
        self.client
            .get_json(
                Endpoint::ApiWebV1,
                &format!("player/{}/game-log/{}/{}", player_id, season, game_type),
                None,
            )
            .await
    }

    /// Search for players by name
    ///
    /// # Arguments
    /// * `query` - Search query (player name or partial name)
    /// * `limit` - Maximum number of results to return (default 20)
    pub async fn search_player(
        &self,
        query: &str,
        limit: Option<i32>,
    ) -> Result<Vec<PlayerSearchResult>, NHLApiError> {
        let mut params = HashMap::new();
        params.insert("culture".to_string(), "en-us".to_string());
        params.insert("q".to_string(), query.to_string());
        params.insert("limit".to_string(), limit.unwrap_or(20).to_string());

        self.client
            .get_json(Endpoint::SearchV1, "search/player", Some(params))
            .await
    }

    /// Gets a list of all NHL franchises (past and current)
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

    /// Gets player statistics for a team in a specific season
    ///
    /// Returns skater and goalie statistics for all players on the team during the specified
    /// season and game type.
    ///
    /// # Arguments
    /// * `team_abbr` - Team abbreviation (e.g., "MTL", "TOR", "BUF")
    /// * `season` - Season in YYYYYYYY format (e.g., 20242025)
    /// * `game_type` - Game type: 2 for regular season, 3 for playoffs
    ///
    /// # Example
    /// ```no_run
    /// # use nhl_api::Client;
    /// # async fn example() -> Result<(), nhl_api::NHLApiError> {
    /// let client = Client::new()?;
    /// let stats = client.club_stats("MTL", 20242025, 2).await?;
    /// println!("Skaters: {}, Goalies: {}", stats.skaters.len(), stats.goalies.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn club_stats(
        &self,
        team_abbr: &str,
        season: i32,
        game_type: i32,
    ) -> Result<ClubStats, NHLApiError> {
        self.client
            .get_json(
                Endpoint::ApiWebV1,
                &format!("club-stats/{}/{}/{}", team_abbr, season, game_type),
                None,
            )
            .await
    }

    /// Gets available seasons and game types for a team
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
    pub async fn club_stats_season(&self, team_abbr: &str) -> Result<Vec<SeasonGameTypes>, NHLApiError> {
        self.client
            .get_json(
                Endpoint::ApiWebV1,
                &format!("club-stats-season/{}", team_abbr),
                None,
            )
            .await
    }

    /// Gets the current roster for a team
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

    /// Gets the roster for a team in a specific season
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

    /// Gets daily game scores for a specific date
    ///
    /// # Arguments
    /// * `date` - Optional GameDate. If None, defaults to today's date.
    pub async fn daily_scores(&self, date: Option<GameDate>) -> Result<DailyScores, NHLApiError> {
        let date = self.resolve_date_to_today(date);
        self.client
            .get_json(
                Endpoint::ApiWebV1,
                &format!("score/{}", date.to_api_string()),
                None,
            )
            .await
    }

    /// Gets weekly schedule for a specific team
    ///
    /// # Arguments
    /// * `team_abbr` - Team abbreviation (e.g., "MTL", "TOR", "BUF")
    /// * `date` - Optional GameDate for the week start. If None, defaults to today's date.
    pub async fn team_weekly_schedule(
        &self,
        team_abbr: &str,
        date: Option<GameDate>,
    ) -> Result<TeamScheduleResponse, NHLApiError> {
        let date = self.resolve_date_to_today(date);
        self.client
            .get_json(
                Endpoint::ApiWebV1,
                &format!("club-schedule/{}/week/{}", team_abbr, date.to_api_string()),
                None,
            )
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::date::GameDate;
    use chrono::NaiveDate;

    // ===== Client Construction Tests =====

    #[test]
    fn test_client_new() {
        let client = Client::new();
        assert!(client.is_ok());
    }

    #[test]
    fn test_client_with_default_config() {
        let config = ClientConfig::default();
        let client = Client::with_config(config);
        assert!(client.is_ok());
    }

    #[test]
    fn test_client_with_custom_config() {
        let config = ClientConfig {
            timeout: std::time::Duration::from_secs(60),
            follow_redirects: false,
            ssl_verify: true,
        };
        let client = Client::with_config(config);
        assert!(client.is_ok());
    }

    // ===== Helper Method Tests =====

    #[test]
    fn test_resolve_date_with_some() {
        let client = Client::new().unwrap();
        let date = GameDate::Date(NaiveDate::from_ymd_opt(2024, 1, 15).unwrap());
        let resolved = client.resolve_date(Some(date.clone()));
        assert_eq!(resolved.to_api_string(), "2024-01-15");
    }

    #[test]
    fn test_resolve_date_with_none() {
        let client = Client::new().unwrap();
        let resolved = client.resolve_date(None);
        assert_eq!(resolved.to_api_string(), "now");
    }

    #[test]
    fn test_resolve_date_to_today_with_some() {
        let client = Client::new().unwrap();
        let date = GameDate::Date(NaiveDate::from_ymd_opt(2024, 1, 15).unwrap());
        let resolved = client.resolve_date_to_today(Some(date.clone()));
        assert_eq!(resolved.to_api_string(), "2024-01-15");
    }

    #[test]
    fn test_resolve_date_to_today_with_none() {
        let client = Client::new().unwrap();
        let resolved = client.resolve_date_to_today(None);
        // Should be today's date, not "now"
        assert_ne!(resolved.to_api_string(), "now");
    }

    #[test]
    fn test_extract_daily_schedule_found() {
        let client = Client::new().unwrap();

        let weekly_response = WeeklyScheduleResponse {
            next_start_date: "2024-01-15".to_string(),
            previous_start_date: "2024-01-01".to_string(),
            game_week: vec![
                crate::types::schedule::GameDay {
                    date: "2024-01-08".to_string(),
                    games: vec![],
                },
            ],
        };

        let result = client.extract_daily_schedule(weekly_response, "2024-01-08".to_string());

        assert_eq!(result.date, "2024-01-08");
        assert_eq!(result.number_of_games, 0); // games vec is empty
        assert_eq!(result.next_start_date, Some("2024-01-15".to_string()));
        assert_eq!(result.previous_start_date, Some("2024-01-01".to_string()));
    }

    #[test]
    fn test_extract_daily_schedule_not_found() {
        let client = Client::new().unwrap();

        let weekly_response = WeeklyScheduleResponse {
            next_start_date: "2024-01-15".to_string(),
            previous_start_date: "2024-01-01".to_string(),
            game_week: vec![
                crate::types::schedule::GameDay {
                    date: "2024-01-08".to_string(),
                    games: vec![],
                },
            ],
        };

        let result = client.extract_daily_schedule(weekly_response, "2024-01-09".to_string());

        assert_eq!(result.date, "2024-01-09");
        assert_eq!(result.number_of_games, 0);
        assert!(result.games.is_empty());
    }

    #[test]
    fn test_extract_daily_schedule_with_games() {
        use crate::types::game_state::GameState;
        use crate::types::schedule::{ScheduleGame, ScheduleTeam};

        let client = Client::new().unwrap();

        let weekly_response = WeeklyScheduleResponse {
            next_start_date: "2024-01-15".to_string(),
            previous_start_date: "2024-01-01".to_string(),
            game_week: vec![
                crate::types::schedule::GameDay {
                    date: "2024-01-08".to_string(),
                    games: vec![
                        ScheduleGame {
                            id: 2023020001,
                            game_type: 2,
                            game_date: Some("2024-01-08".to_string()),
                            start_time_utc: "2024-01-08T23:00:00Z".to_string(),
                            away_team: ScheduleTeam {
                                id: 8,
                                abbrev: "MTL".to_string(),
                                logo: "logo.png".to_string(),
                                score: Some(2),
                                place_name: None,
                            },
                            home_team: ScheduleTeam {
                                id: 6,
                                abbrev: "BOS".to_string(),
                                logo: "logo.png".to_string(),
                                score: Some(3),
                                place_name: None,
                            },
                            game_state: GameState::Final,
                        },
                    ],
                },
            ],
        };

        let result = client.extract_daily_schedule(weekly_response, "2024-01-08".to_string());

        assert_eq!(result.date, "2024-01-08");
        assert_eq!(result.number_of_games, 1);
        assert_eq!(result.games.len(), 1);
        assert_eq!(result.games[0].id, 2023020001);
    }

    #[test]
    fn test_extract_daily_schedule_empty_game_week() {
        let client = Client::new().unwrap();

        let weekly_response = WeeklyScheduleResponse {
            next_start_date: "2024-01-15".to_string(),
            previous_start_date: "2024-01-01".to_string(),
            game_week: vec![],
        };

        let result = client.extract_daily_schedule(weekly_response, "2024-01-08".to_string());

        assert_eq!(result.date, "2024-01-08");
        assert_eq!(result.number_of_games, 0);
        assert!(result.games.is_empty());
    }

}
