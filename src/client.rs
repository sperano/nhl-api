use crate::config::ClientConfig;
use crate::date::{GameDate, Season};
use crate::error::NHLApiError;
use crate::http_client::{Endpoint, HttpClient};
use crate::ids::{GameId, PlayerId, TeamId};
use crate::types::{
    Boxscore, ClubStats, DailySchedule, DailyScores, EdgeGoalie5v5Detail, EdgeGoalieComparison,
    EdgeGoalieDetail, EdgeGoalieLanding, EdgeGoalieSavePctgDetail, EdgeGoalieShotLocationDetail,
    EdgeSkaterComparison, EdgeSkaterDetail, EdgeSkaterDistanceDetail, EdgeSkaterLanding,
    EdgeSkaterShotLocationDetail, EdgeSkaterShotSpeedDetail, EdgeSkaterSpeedDetail,
    EdgeSkaterZoneTimeDetail, EdgeTeamComparison, EdgeTeamDetail, EdgeTeamDistanceDetail,
    EdgeTeamLanding, EdgeTeamShotLocationDetail, EdgeTeamShotSpeedDetail, EdgeTeamSpeedDetail,
    EdgeTeamZoneTimeDetails, Franchise, FranchisesResponse, GameMatchup, GameStory, GameType,
    PlayByPlay, PlayerGameLog, PlayerLanding, PlayerSearchResult, Roster, SeasonGameTypes,
    SeasonInfo, SeasonSeriesMatchup, SeasonsResponse, ShiftChart, Standing, StandingsResponse,
    Team, TeamScheduleResponse, WeeklyScheduleResponse,
};
use std::collections::HashMap;

/// Number of results [`Client::search_player`] requests when the caller passes
/// no explicit limit.
const DEFAULT_SEARCH_LIMIT: i32 = 20;

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
        Ok(Self {
            client: HttpClient::new(config)?,
        })
    }

    /// Resolve optional date to owned GameDate with a default value
    fn resolve_date_or(date: Option<GameDate>, default: GameDate) -> GameDate {
        date.unwrap_or(default)
    }

    pub async fn teams(&self, date: Option<GameDate>) -> Result<Vec<Team>, NHLApiError> {
        let date = Self::resolve_date_or(date, GameDate::default());
        let standings_response = self.fetch_standings_data(&date.to_api_string()).await?;
        let teams: Vec<Team> = standings_response
            .standings
            .iter()
            .map(|standing| standing.to_team())
            .collect();

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

    pub async fn league_standings_for_date(
        &self,
        date: &GameDate,
    ) -> Result<Vec<Standing>, NHLApiError> {
        Ok(self
            .fetch_standings_data(&date.to_api_string())
            .await?
            .standings)
    }

    pub async fn league_standings_for_season(
        &self,
        season_id: i64,
    ) -> Result<Vec<Standing>, NHLApiError> {
        let seasons = self.season_standing_manifest().await?;
        let season_data = seasons
            .iter()
            .find(|s| i64::from(s.id) == season_id)
            .ok_or_else(|| NHLApiError::Other(format!("Invalid Season Id {}", season_id)))?;
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

    /// Fetch data from a gamecenter endpoint
    async fn fetch_gamecenter<T: serde::de::DeserializeOwned>(
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

    pub async fn boxscore(&self, game_id: impl Into<GameId>) -> Result<Boxscore, NHLApiError> {
        self.fetch_gamecenter(game_id, "boxscore").await
    }

    pub async fn play_by_play(
        &self,
        game_id: impl Into<GameId>,
    ) -> Result<PlayByPlay, NHLApiError> {
        self.fetch_gamecenter(game_id, "play-by-play").await
    }

    /// Fetch game landing data (lighter than play-by-play, includes summary with period scores)
    pub async fn landing(&self, game_id: impl Into<GameId>) -> Result<GameMatchup, NHLApiError> {
        self.fetch_gamecenter(game_id, "landing").await
    }

    /// Fetch season series matchup data including head-to-head records
    pub async fn season_series(
        &self,
        game_id: impl Into<GameId>,
    ) -> Result<SeasonSeriesMatchup, NHLApiError> {
        self.fetch_gamecenter(game_id, "right-rail").await
    }

    /// Fetch game story narrative content
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

    /// Fetch shift chart data for a game
    pub async fn shift_chart(&self, game_id: impl Into<GameId>) -> Result<ShiftChart, NHLApiError> {
        let game_id = game_id.into();
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

    async fn fetch_weekly_schedule(
        &self,
        date_string: &str,
    ) -> Result<WeeklyScheduleResponse, NHLApiError> {
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
            .into_iter()
            .find(|day| day.date == date_string)
            .map(|day| day.games)
            .unwrap_or_default();

        DailySchedule {
            next_start_date: Some(schedule_data.next_start_date),
            previous_start_date: Some(schedule_data.previous_start_date),
            date: date_string,
            number_of_games: games.len(),
            games,
        }
    }

    pub async fn daily_schedule(
        &self,
        date: Option<GameDate>,
    ) -> Result<DailySchedule, NHLApiError> {
        let date = Self::resolve_date_or(date, GameDate::today());
        let date_string = date.to_api_string();
        let schedule_data = self.fetch_weekly_schedule(&date_string).await?;
        Ok(self.extract_daily_schedule(schedule_data, date_string))
    }

    /// Gets NHL schedule for a week starting from the specified date.
    ///
    /// # Arguments
    /// * `date` - Optional GameDate. If None, defaults to "now".
    pub async fn weekly_schedule(
        &self,
        date: Option<GameDate>,
    ) -> Result<WeeklyScheduleResponse, NHLApiError> {
        let date = Self::resolve_date_or(date, GameDate::default());
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

    /// Gets game-by-game log for a player's season
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
        // The API doesn't include player_id in the response, so we set it from the parameter
        game_log.player_id = player_id;
        Ok(game_log)
    }

    /// Search for players by name
    ///
    /// # Arguments
    /// * `query` - Search query (player name or partial name)
    /// * `limit` - Maximum number of results to return (defaults to
    ///   [`DEFAULT_SEARCH_LIMIT`] when `None`)
    pub async fn search_player(
        &self,
        query: &str,
        limit: Option<i32>,
    ) -> Result<Vec<PlayerSearchResult>, NHLApiError> {
        self.search_player_at(Endpoint::SearchV1, query, limit)
            .await
    }

    /// Endpoint-parameterized core of [`Self::search_player`], split out so the
    /// query-building (notably the [`DEFAULT_SEARCH_LIMIT`] fallback) can be
    /// exercised against a mock server.
    async fn search_player_at(
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
        let date = Self::resolve_date_or(date, GameDate::today());
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
        let date = Self::resolve_date_or(date, GameDate::today());
        self.client
            .get_json(
                Endpoint::ApiWebV1,
                &format!("club-schedule/{}/week/{}", team_abbr, date.to_api_string()),
                None,
            )
            .await
    }

    /// Gets the full schedule for a team in a given season
    ///
    /// Includes preseason, regular season, and playoff games for the team's
    /// entire season, unlike [`Self::team_weekly_schedule`] which only covers a
    /// single week.
    ///
    /// # Arguments
    /// * `team_abbr` - Team abbreviation (e.g., "MTL", "TOR", "BUF")
    /// * `season` - The NHL season to fetch the schedule for
    pub async fn club_schedule_season(
        &self,
        team_abbr: &str,
        season: Season,
    ) -> Result<TeamScheduleResponse, NHLApiError> {
        self.club_schedule_season_at(Endpoint::ApiWebV1, team_abbr, season)
            .await
    }

    /// Endpoint-parameterized core of [`Self::club_schedule_season`], split out
    /// so the exact request path can be exercised against a mock server.
    async fn club_schedule_season_at(
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

    /// Gets Edge puck/player-tracking overview stats for a skater's season.
    pub async fn edge_skater_detail(
        &self,
        player_id: impl Into<PlayerId>,
        season: Season,
        game_type: GameType,
    ) -> Result<EdgeSkaterDetail, NHLApiError> {
        self.edge_skater_detail_at(Endpoint::ApiWebV1, player_id, season, game_type)
            .await
    }

    /// Endpoint-parameterized core of [`Self::edge_skater_detail`], split out so the
    /// path construction can be exercised against a mock server.
    async fn edge_skater_detail_at(
        &self,
        endpoint: Endpoint,
        player_id: impl Into<PlayerId>,
        season: Season,
        game_type: GameType,
    ) -> Result<EdgeSkaterDetail, NHLApiError> {
        let player_id = player_id.into();
        self.client
            .get_json(
                endpoint,
                &format!(
                    "edge/skater-detail/{}/{}/{}",
                    player_id,
                    season.to_api_string(),
                    game_type.to_int()
                ),
                None,
            )
            .await
    }

    /// Gets Edge skating-speed detail (per-game top speeds) for a skater's season.
    pub async fn edge_skater_speed_detail(
        &self,
        player_id: impl Into<PlayerId>,
        season: Season,
        game_type: GameType,
    ) -> Result<EdgeSkaterSpeedDetail, NHLApiError> {
        self.edge_skater_speed_detail_at(Endpoint::ApiWebV1, player_id, season, game_type)
            .await
    }

    /// Endpoint-parameterized core of [`Self::edge_skater_speed_detail`], split out so the
    /// path construction can be exercised against a mock server.
    async fn edge_skater_speed_detail_at(
        &self,
        endpoint: Endpoint,
        player_id: impl Into<PlayerId>,
        season: Season,
        game_type: GameType,
    ) -> Result<EdgeSkaterSpeedDetail, NHLApiError> {
        let player_id = player_id.into();
        self.client
            .get_json(
                endpoint,
                &format!(
                    "edge/skater-skating-speed-detail/{}/{}/{}",
                    player_id,
                    season.to_api_string(),
                    game_type.to_int()
                ),
                None,
            )
            .await
    }

    /// Gets Edge skating-distance detail (per-game distance skated) for a skater's season.
    pub async fn edge_skater_distance_detail(
        &self,
        player_id: impl Into<PlayerId>,
        season: Season,
        game_type: GameType,
    ) -> Result<EdgeSkaterDistanceDetail, NHLApiError> {
        self.edge_skater_distance_detail_at(Endpoint::ApiWebV1, player_id, season, game_type)
            .await
    }

    /// Endpoint-parameterized core of [`Self::edge_skater_distance_detail`], split out so the
    /// path construction can be exercised against a mock server.
    async fn edge_skater_distance_detail_at(
        &self,
        endpoint: Endpoint,
        player_id: impl Into<PlayerId>,
        season: Season,
        game_type: GameType,
    ) -> Result<EdgeSkaterDistanceDetail, NHLApiError> {
        let player_id = player_id.into();
        self.client
            .get_json(
                endpoint,
                &format!(
                    "edge/skater-skating-distance-detail/{}/{}/{}",
                    player_id,
                    season.to_api_string(),
                    game_type.to_int()
                ),
                None,
            )
            .await
    }

    /// Gets Edge shot-speed detail (hardest shots) for a skater's season.
    pub async fn edge_skater_shot_speed_detail(
        &self,
        player_id: impl Into<PlayerId>,
        season: Season,
        game_type: GameType,
    ) -> Result<EdgeSkaterShotSpeedDetail, NHLApiError> {
        self.edge_skater_shot_speed_detail_at(Endpoint::ApiWebV1, player_id, season, game_type)
            .await
    }

    /// Endpoint-parameterized core of [`Self::edge_skater_shot_speed_detail`], split out so the
    /// path construction can be exercised against a mock server.
    async fn edge_skater_shot_speed_detail_at(
        &self,
        endpoint: Endpoint,
        player_id: impl Into<PlayerId>,
        season: Season,
        game_type: GameType,
    ) -> Result<EdgeSkaterShotSpeedDetail, NHLApiError> {
        let player_id = player_id.into();
        self.client
            .get_json(
                endpoint,
                &format!(
                    "edge/skater-shot-speed-detail/{}/{}/{}",
                    player_id,
                    season.to_api_string(),
                    game_type.to_int()
                ),
                None,
            )
            .await
    }

    /// Gets Edge shot-location detail (shot breakdown by rink area) for a skater's season.
    pub async fn edge_skater_shot_location_detail(
        &self,
        player_id: impl Into<PlayerId>,
        season: Season,
        game_type: GameType,
    ) -> Result<EdgeSkaterShotLocationDetail, NHLApiError> {
        self.edge_skater_shot_location_detail_at(Endpoint::ApiWebV1, player_id, season, game_type)
            .await
    }

    /// Endpoint-parameterized core of [`Self::edge_skater_shot_location_detail`], split out so the
    /// path construction can be exercised against a mock server.
    async fn edge_skater_shot_location_detail_at(
        &self,
        endpoint: Endpoint,
        player_id: impl Into<PlayerId>,
        season: Season,
        game_type: GameType,
    ) -> Result<EdgeSkaterShotLocationDetail, NHLApiError> {
        let player_id = player_id.into();
        self.client
            .get_json(
                endpoint,
                &format!(
                    "edge/skater-shot-location-detail/{}/{}/{}",
                    player_id,
                    season.to_api_string(),
                    game_type.to_int()
                ),
                None,
            )
            .await
    }

    /// Gets Edge zone-time detail (zone time breakdown by strength) for a skater's season.
    ///
    /// Note the path has no `-details` suffix, unlike the sibling detail
    /// endpoints (`edge/skater-zone-time`, not `edge/skater-zone-time-detail`).
    pub async fn edge_skater_zone_time(
        &self,
        player_id: impl Into<PlayerId>,
        season: Season,
        game_type: GameType,
    ) -> Result<EdgeSkaterZoneTimeDetail, NHLApiError> {
        self.edge_skater_zone_time_at(Endpoint::ApiWebV1, player_id, season, game_type)
            .await
    }

    /// Endpoint-parameterized core of [`Self::edge_skater_zone_time`], split out so the
    /// path construction can be exercised against a mock server.
    async fn edge_skater_zone_time_at(
        &self,
        endpoint: Endpoint,
        player_id: impl Into<PlayerId>,
        season: Season,
        game_type: GameType,
    ) -> Result<EdgeSkaterZoneTimeDetail, NHLApiError> {
        let player_id = player_id.into();
        self.client
            .get_json(
                endpoint,
                &format!(
                    "edge/skater-zone-time/{}/{}/{}",
                    player_id,
                    season.to_api_string(),
                    game_type.to_int()
                ),
                None,
            )
            .await
    }

    /// Gets the Edge head-to-head comparison composite for a skater's season.
    pub async fn edge_skater_comparison(
        &self,
        player_id: impl Into<PlayerId>,
        season: Season,
        game_type: GameType,
    ) -> Result<EdgeSkaterComparison, NHLApiError> {
        self.edge_skater_comparison_at(Endpoint::ApiWebV1, player_id, season, game_type)
            .await
    }

    /// Endpoint-parameterized core of [`Self::edge_skater_comparison`], split out so the
    /// path construction can be exercised against a mock server.
    async fn edge_skater_comparison_at(
        &self,
        endpoint: Endpoint,
        player_id: impl Into<PlayerId>,
        season: Season,
        game_type: GameType,
    ) -> Result<EdgeSkaterComparison, NHLApiError> {
        let player_id = player_id.into();
        self.client
            .get_json(
                endpoint,
                &format!(
                    "edge/skater-comparison/{}/{}/{}",
                    player_id,
                    season.to_api_string(),
                    game_type.to_int()
                ),
                None,
            )
            .await
    }

    /// Gets the league-wide Edge skater leaderboard for a season (no player id).
    pub async fn edge_skater_landing(
        &self,
        season: Season,
        game_type: GameType,
    ) -> Result<EdgeSkaterLanding, NHLApiError> {
        self.edge_skater_landing_at(Endpoint::ApiWebV1, season, game_type)
            .await
    }

    /// Endpoint-parameterized core of [`Self::edge_skater_landing`], split out so the
    /// path construction can be exercised against a mock server.
    async fn edge_skater_landing_at(
        &self,
        endpoint: Endpoint,
        season: Season,
        game_type: GameType,
    ) -> Result<EdgeSkaterLanding, NHLApiError> {
        self.client
            .get_json(
                endpoint,
                &format!(
                    "edge/skater-landing/{}/{}",
                    season.to_api_string(),
                    game_type.to_int()
                ),
                None,
            )
            .await
    }

    /// Gets Edge puck/player-tracking overview stats for a goalie's season.
    pub async fn edge_goalie_detail(
        &self,
        goalie_id: impl Into<PlayerId>,
        season: Season,
        game_type: GameType,
    ) -> Result<EdgeGoalieDetail, NHLApiError> {
        self.edge_goalie_detail_at(Endpoint::ApiWebV1, goalie_id, season, game_type)
            .await
    }

    /// Endpoint-parameterized core of [`Self::edge_goalie_detail`], split out so the
    /// path construction can be exercised against a mock server.
    async fn edge_goalie_detail_at(
        &self,
        endpoint: Endpoint,
        goalie_id: impl Into<PlayerId>,
        season: Season,
        game_type: GameType,
    ) -> Result<EdgeGoalieDetail, NHLApiError> {
        let goalie_id = goalie_id.into();
        self.client
            .get_json(
                endpoint,
                &format!(
                    "edge/goalie-detail/{}/{}/{}",
                    goalie_id,
                    season.to_api_string(),
                    game_type.to_int()
                ),
                None,
            )
            .await
    }

    /// Gets Edge 5v5 save-percentage detail (per-game) for a goalie's season.
    pub async fn edge_goalie_5v5_detail(
        &self,
        goalie_id: impl Into<PlayerId>,
        season: Season,
        game_type: GameType,
    ) -> Result<EdgeGoalie5v5Detail, NHLApiError> {
        self.edge_goalie_5v5_detail_at(Endpoint::ApiWebV1, goalie_id, season, game_type)
            .await
    }

    /// Endpoint-parameterized core of [`Self::edge_goalie_5v5_detail`], split out so the
    /// path construction can be exercised against a mock server.
    async fn edge_goalie_5v5_detail_at(
        &self,
        endpoint: Endpoint,
        goalie_id: impl Into<PlayerId>,
        season: Season,
        game_type: GameType,
    ) -> Result<EdgeGoalie5v5Detail, NHLApiError> {
        let goalie_id = goalie_id.into();
        self.client
            .get_json(
                endpoint,
                &format!(
                    "edge/goalie-5v5-detail/{}/{}/{}",
                    goalie_id,
                    season.to_api_string(),
                    game_type.to_int()
                ),
                None,
            )
            .await
    }

    /// Gets Edge shot-location detail (shot breakdown by rink area) for a goalie's season.
    pub async fn edge_goalie_shot_location_detail(
        &self,
        goalie_id: impl Into<PlayerId>,
        season: Season,
        game_type: GameType,
    ) -> Result<EdgeGoalieShotLocationDetail, NHLApiError> {
        self.edge_goalie_shot_location_detail_at(Endpoint::ApiWebV1, goalie_id, season, game_type)
            .await
    }

    /// Endpoint-parameterized core of [`Self::edge_goalie_shot_location_detail`], split out so the
    /// path construction can be exercised against a mock server.
    async fn edge_goalie_shot_location_detail_at(
        &self,
        endpoint: Endpoint,
        goalie_id: impl Into<PlayerId>,
        season: Season,
        game_type: GameType,
    ) -> Result<EdgeGoalieShotLocationDetail, NHLApiError> {
        let goalie_id = goalie_id.into();
        self.client
            .get_json(
                endpoint,
                &format!(
                    "edge/goalie-shot-location-detail/{}/{}/{}",
                    goalie_id,
                    season.to_api_string(),
                    game_type.to_int()
                ),
                None,
            )
            .await
    }

    /// Gets Edge save-percentage detail (per-game, plus aggregated stats) for a goalie's season.
    ///
    /// Note the path slug is spelled out (`goalie-save-percentage-detail`),
    /// not abbreviated to `goalie-save-pctg-detail`.
    pub async fn edge_goalie_save_pctg_detail(
        &self,
        goalie_id: impl Into<PlayerId>,
        season: Season,
        game_type: GameType,
    ) -> Result<EdgeGoalieSavePctgDetail, NHLApiError> {
        self.edge_goalie_save_pctg_detail_at(Endpoint::ApiWebV1, goalie_id, season, game_type)
            .await
    }

    /// Endpoint-parameterized core of [`Self::edge_goalie_save_pctg_detail`], split out so the
    /// path construction can be exercised against a mock server.
    async fn edge_goalie_save_pctg_detail_at(
        &self,
        endpoint: Endpoint,
        goalie_id: impl Into<PlayerId>,
        season: Season,
        game_type: GameType,
    ) -> Result<EdgeGoalieSavePctgDetail, NHLApiError> {
        let goalie_id = goalie_id.into();
        self.client
            .get_json(
                endpoint,
                &format!(
                    "edge/goalie-save-percentage-detail/{}/{}/{}",
                    goalie_id,
                    season.to_api_string(),
                    game_type.to_int()
                ),
                None,
            )
            .await
    }

    /// Gets the Edge head-to-head comparison composite for a goalie's season.
    pub async fn edge_goalie_comparison(
        &self,
        goalie_id: impl Into<PlayerId>,
        season: Season,
        game_type: GameType,
    ) -> Result<EdgeGoalieComparison, NHLApiError> {
        self.edge_goalie_comparison_at(Endpoint::ApiWebV1, goalie_id, season, game_type)
            .await
    }

    /// Endpoint-parameterized core of [`Self::edge_goalie_comparison`], split out so the
    /// path construction can be exercised against a mock server.
    async fn edge_goalie_comparison_at(
        &self,
        endpoint: Endpoint,
        goalie_id: impl Into<PlayerId>,
        season: Season,
        game_type: GameType,
    ) -> Result<EdgeGoalieComparison, NHLApiError> {
        let goalie_id = goalie_id.into();
        self.client
            .get_json(
                endpoint,
                &format!(
                    "edge/goalie-comparison/{}/{}/{}",
                    goalie_id,
                    season.to_api_string(),
                    game_type.to_int()
                ),
                None,
            )
            .await
    }

    /// Gets the league-wide Edge goalie leaderboard for a season (no goalie id).
    pub async fn edge_goalie_landing(
        &self,
        season: Season,
        game_type: GameType,
    ) -> Result<EdgeGoalieLanding, NHLApiError> {
        self.edge_goalie_landing_at(Endpoint::ApiWebV1, season, game_type)
            .await
    }

    /// Endpoint-parameterized core of [`Self::edge_goalie_landing`], split out so the
    /// path construction can be exercised against a mock server.
    async fn edge_goalie_landing_at(
        &self,
        endpoint: Endpoint,
        season: Season,
        game_type: GameType,
    ) -> Result<EdgeGoalieLanding, NHLApiError> {
        self.client
            .get_json(
                endpoint,
                &format!(
                    "edge/goalie-landing/{}/{}",
                    season.to_api_string(),
                    game_type.to_int()
                ),
                None,
            )
            .await
    }

    /// Gets Edge puck/player-tracking overview stats for a team's season.
    ///
    /// Team Edge stats are rank-based (1-32), unlike the percentile-based
    /// skater/goalie stats.
    pub async fn edge_team_detail(
        &self,
        team_id: impl Into<TeamId>,
        season: Season,
        game_type: GameType,
    ) -> Result<EdgeTeamDetail, NHLApiError> {
        self.edge_team_detail_at(Endpoint::ApiWebV1, team_id, season, game_type)
            .await
    }

    /// Endpoint-parameterized core of [`Self::edge_team_detail`], split out so the
    /// path construction can be exercised against a mock server.
    async fn edge_team_detail_at(
        &self,
        endpoint: Endpoint,
        team_id: impl Into<TeamId>,
        season: Season,
        game_type: GameType,
    ) -> Result<EdgeTeamDetail, NHLApiError> {
        let team_id = team_id.into();
        self.client
            .get_json(
                endpoint,
                &format!(
                    "edge/team-detail/{}/{}/{}",
                    team_id,
                    season.to_api_string(),
                    game_type.to_int()
                ),
                None,
            )
            .await
    }

    /// Gets Edge skating-speed detail (per-player top speeds) for a team's season.
    pub async fn edge_team_speed_detail(
        &self,
        team_id: impl Into<TeamId>,
        season: Season,
        game_type: GameType,
    ) -> Result<EdgeTeamSpeedDetail, NHLApiError> {
        self.edge_team_speed_detail_at(Endpoint::ApiWebV1, team_id, season, game_type)
            .await
    }

    /// Endpoint-parameterized core of [`Self::edge_team_speed_detail`], split out so the
    /// path construction can be exercised against a mock server.
    async fn edge_team_speed_detail_at(
        &self,
        endpoint: Endpoint,
        team_id: impl Into<TeamId>,
        season: Season,
        game_type: GameType,
    ) -> Result<EdgeTeamSpeedDetail, NHLApiError> {
        let team_id = team_id.into();
        self.client
            .get_json(
                endpoint,
                &format!(
                    "edge/team-skating-speed-detail/{}/{}/{}",
                    team_id,
                    season.to_api_string(),
                    game_type.to_int()
                ),
                None,
            )
            .await
    }

    /// Gets Edge skating-distance detail (per-game distance skated) for a team's season.
    pub async fn edge_team_distance_detail(
        &self,
        team_id: impl Into<TeamId>,
        season: Season,
        game_type: GameType,
    ) -> Result<EdgeTeamDistanceDetail, NHLApiError> {
        self.edge_team_distance_detail_at(Endpoint::ApiWebV1, team_id, season, game_type)
            .await
    }

    /// Endpoint-parameterized core of [`Self::edge_team_distance_detail`], split out so the
    /// path construction can be exercised against a mock server.
    async fn edge_team_distance_detail_at(
        &self,
        endpoint: Endpoint,
        team_id: impl Into<TeamId>,
        season: Season,
        game_type: GameType,
    ) -> Result<EdgeTeamDistanceDetail, NHLApiError> {
        let team_id = team_id.into();
        self.client
            .get_json(
                endpoint,
                &format!(
                    "edge/team-skating-distance-detail/{}/{}/{}",
                    team_id,
                    season.to_api_string(),
                    game_type.to_int()
                ),
                None,
            )
            .await
    }

    /// Gets Edge shot-speed detail (per-player hardest shots) for a team's season.
    pub async fn edge_team_shot_speed_detail(
        &self,
        team_id: impl Into<TeamId>,
        season: Season,
        game_type: GameType,
    ) -> Result<EdgeTeamShotSpeedDetail, NHLApiError> {
        self.edge_team_shot_speed_detail_at(Endpoint::ApiWebV1, team_id, season, game_type)
            .await
    }

    /// Endpoint-parameterized core of [`Self::edge_team_shot_speed_detail`], split out so the
    /// path construction can be exercised against a mock server.
    async fn edge_team_shot_speed_detail_at(
        &self,
        endpoint: Endpoint,
        team_id: impl Into<TeamId>,
        season: Season,
        game_type: GameType,
    ) -> Result<EdgeTeamShotSpeedDetail, NHLApiError> {
        let team_id = team_id.into();
        self.client
            .get_json(
                endpoint,
                &format!(
                    "edge/team-shot-speed-detail/{}/{}/{}",
                    team_id,
                    season.to_api_string(),
                    game_type.to_int()
                ),
                None,
            )
            .await
    }

    /// Gets Edge shot-location detail (shot breakdown by rink area) for a team's season.
    pub async fn edge_team_shot_location_detail(
        &self,
        team_id: impl Into<TeamId>,
        season: Season,
        game_type: GameType,
    ) -> Result<EdgeTeamShotLocationDetail, NHLApiError> {
        self.edge_team_shot_location_detail_at(Endpoint::ApiWebV1, team_id, season, game_type)
            .await
    }

    /// Endpoint-parameterized core of [`Self::edge_team_shot_location_detail`], split out so the
    /// path construction can be exercised against a mock server.
    async fn edge_team_shot_location_detail_at(
        &self,
        endpoint: Endpoint,
        team_id: impl Into<TeamId>,
        season: Season,
        game_type: GameType,
    ) -> Result<EdgeTeamShotLocationDetail, NHLApiError> {
        let team_id = team_id.into();
        self.client
            .get_json(
                endpoint,
                &format!(
                    "edge/team-shot-location-detail/{}/{}/{}",
                    team_id,
                    season.to_api_string(),
                    game_type.to_int()
                ),
                None,
            )
            .await
    }

    /// Gets Edge zone-time detail (zone time by strength code, with shot
    /// differential) for a team's season.
    ///
    /// Distinct from the zone-time summary embedded in [`EdgeTeamDetail`].
    /// Note the path has a `-details` suffix, unlike the skater equivalent
    /// (`edge/team-zone-time-details`, not `edge/team-zone-time`).
    pub async fn edge_team_zone_time_details(
        &self,
        team_id: impl Into<TeamId>,
        season: Season,
        game_type: GameType,
    ) -> Result<EdgeTeamZoneTimeDetails, NHLApiError> {
        self.edge_team_zone_time_details_at(Endpoint::ApiWebV1, team_id, season, game_type)
            .await
    }

    /// Endpoint-parameterized core of [`Self::edge_team_zone_time_details`], split out so the
    /// path construction can be exercised against a mock server.
    async fn edge_team_zone_time_details_at(
        &self,
        endpoint: Endpoint,
        team_id: impl Into<TeamId>,
        season: Season,
        game_type: GameType,
    ) -> Result<EdgeTeamZoneTimeDetails, NHLApiError> {
        let team_id = team_id.into();
        self.client
            .get_json(
                endpoint,
                &format!(
                    "edge/team-zone-time-details/{}/{}/{}",
                    team_id,
                    season.to_api_string(),
                    game_type.to_int()
                ),
                None,
            )
            .await
    }

    /// Gets the Edge head-to-head comparison composite for a team's season.
    pub async fn edge_team_comparison(
        &self,
        team_id: impl Into<TeamId>,
        season: Season,
        game_type: GameType,
    ) -> Result<EdgeTeamComparison, NHLApiError> {
        self.edge_team_comparison_at(Endpoint::ApiWebV1, team_id, season, game_type)
            .await
    }

    /// Endpoint-parameterized core of [`Self::edge_team_comparison`], split out so the
    /// path construction can be exercised against a mock server.
    async fn edge_team_comparison_at(
        &self,
        endpoint: Endpoint,
        team_id: impl Into<TeamId>,
        season: Season,
        game_type: GameType,
    ) -> Result<EdgeTeamComparison, NHLApiError> {
        let team_id = team_id.into();
        self.client
            .get_json(
                endpoint,
                &format!(
                    "edge/team-comparison/{}/{}/{}",
                    team_id,
                    season.to_api_string(),
                    game_type.to_int()
                ),
                None,
            )
            .await
    }

    /// Gets the league-wide Edge team leaderboard for a season (no team id).
    pub async fn edge_team_landing(
        &self,
        season: Season,
        game_type: GameType,
    ) -> Result<EdgeTeamLanding, NHLApiError> {
        self.edge_team_landing_at(Endpoint::ApiWebV1, season, game_type)
            .await
    }

    /// Endpoint-parameterized core of [`Self::edge_team_landing`], split out so the
    /// path construction can be exercised against a mock server.
    async fn edge_team_landing_at(
        &self,
        endpoint: Endpoint,
        season: Season,
        game_type: GameType,
    ) -> Result<EdgeTeamLanding, NHLApiError> {
        self.client
            .get_json(
                endpoint,
                &format!(
                    "edge/team-landing/{}/{}",
                    season.to_api_string(),
                    game_type.to_int()
                ),
                None,
            )
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::date::GameDate;
    use crate::ids::TeamId;
    use chrono::NaiveDate;
    use std::future::Future;
    use std::pin::Pin;

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
            ..Default::default()
        };
        let client = Client::with_config(config);
        assert!(client.is_ok());
    }

    // ===== Helper Method Tests =====

    #[test]
    fn test_resolve_date_or_with_some() {
        let date = GameDate::Date(NaiveDate::from_ymd_opt(2024, 1, 15).unwrap());
        let resolved = Client::resolve_date_or(Some(date.clone()), GameDate::default());
        assert_eq!(resolved.to_api_string(), "2024-01-15");
    }

    #[test]
    fn test_resolve_date_or_with_none_default_now() {
        let resolved = Client::resolve_date_or(None, GameDate::default());
        assert_eq!(resolved.to_api_string(), "now");
    }

    #[test]
    fn test_resolve_date_or_with_some_today() {
        let date = GameDate::Date(NaiveDate::from_ymd_opt(2024, 1, 15).unwrap());
        let resolved = Client::resolve_date_or(Some(date.clone()), GameDate::today());
        assert_eq!(resolved.to_api_string(), "2024-01-15");
    }

    #[test]
    fn test_resolve_date_or_with_none_default_today() {
        let resolved = Client::resolve_date_or(None, GameDate::today());
        // Should be today's date, not "now"
        assert_ne!(resolved.to_api_string(), "now");
    }

    #[tokio::test]
    async fn test_search_player_defaults_limit_to_twenty() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/search/player")
            .match_query(mockito::Matcher::UrlEncoded(
                "limit".into(),
                DEFAULT_SEARCH_LIMIT.to_string(),
            ))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body("[]")
            .create_async()
            .await;

        let client = Client::new().unwrap();
        let result = client
            .search_player_at(Endpoint::Custom(server.url()), "gretzky", None)
            .await;

        assert!(result.is_ok(), "search should succeed: {:?}", result.err());
        mock.assert_async().await;
    }

    #[test]
    fn test_extract_daily_schedule_found() {
        let client = Client::new().unwrap();

        let weekly_response = WeeklyScheduleResponse {
            next_start_date: "2024-01-15".to_string(),
            previous_start_date: "2024-01-01".to_string(),
            game_week: vec![crate::types::schedule::GameDay {
                date: "2024-01-08".to_string(),
                games: vec![],
            }],
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
            game_week: vec![crate::types::schedule::GameDay {
                date: "2024-01-08".to_string(),
                games: vec![],
            }],
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
            game_week: vec![crate::types::schedule::GameDay {
                date: "2024-01-08".to_string(),
                games: vec![ScheduleGame {
                    id: GameId::new(2023020001),
                    game_type: GameType::RegularSeason,
                    game_date: Some("2024-01-08".to_string()),
                    start_time_utc: "2024-01-08T23:00:00Z".to_string(),
                    away_team: ScheduleTeam {
                        id: TeamId::new(8),
                        abbrev: "MTL".to_string(),
                        logo: "logo.png".to_string(),
                        score: Some(2),
                        place_name: None,
                    },
                    home_team: ScheduleTeam {
                        id: TeamId::new(6),
                        abbrev: "BOS".to_string(),
                        logo: "logo.png".to_string(),
                        score: Some(3),
                        place_name: None,
                    },
                    game_state: GameState::Final,
                }],
            }],
        };

        let result = client.extract_daily_schedule(weekly_response, "2024-01-08".to_string());

        assert_eq!(result.date, "2024-01-08");
        assert_eq!(result.number_of_games, 1);
        assert_eq!(result.games.len(), 1);
        assert_eq!(result.games[0].id, GameId::new(2023020001));
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

    // ===== Into<GameId> Support Tests =====

    #[test]
    fn test_into_game_id_accepts_i64() {
        // Verify that methods accepting impl Into<GameId> work with raw i64
        let game_id_i64: i64 = 2023020001;

        // This should compile and convert i64 to GameId automatically
        // We're just testing the type conversion, not the actual API call
        let converted: GameId = game_id_i64.into();
        assert_eq!(converted.as_i64(), 2023020001);
    }

    #[test]
    fn test_game_id_from_i64() {
        // Verify that GameId can be created from i64
        let game_id = GameId::from(2023020001);
        assert_eq!(game_id.as_i64(), 2023020001);
    }

    // ===== club_schedule_season Tests =====

    #[tokio::test]
    async fn test_club_schedule_season_requests_exact_path() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/club-schedule-season/FLA/20232024")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"games": []}"#)
            .create_async()
            .await;

        let client = Client::new().unwrap();
        let result = client
            .club_schedule_season_at(Endpoint::Custom(server.url()), "FLA", Season::new(2023))
            .await;

        assert!(result.is_ok(), "request should succeed: {:?}", result.err());
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_club_schedule_season_deserializes_fixture() {
        use crate::types::game_state::GameState;

        // Fixture ported from the Go client's TestClubScheduleSeason
        // (nhl-api-go/nhl/client_test.go): a regular-season game and a
        // playoff game, both completed, on the same date.
        let fixture = r#"{
            "games": [
                {
                    "id": 2023020001,
                    "gameType": 2,
                    "gameDate": "2024-04-20",
                    "startTimeUTC": "2024-04-20T23:00:00Z",
                    "awayTeam": {
                        "id": 13,
                        "abbrev": "FLA",
                        "logo": "https://assets.nhle.com/logos/nhl/svg/FLA_light.svg"
                    },
                    "homeTeam": {
                        "id": 10,
                        "abbrev": "TOR",
                        "logo": "https://assets.nhle.com/logos/nhl/svg/TOR_light.svg"
                    },
                    "gameState": "OFF"
                },
                {
                    "id": 2023030111,
                    "gameType": 3,
                    "gameDate": "2024-04-20",
                    "startTimeUTC": "2024-04-20T23:00:00Z",
                    "awayTeam": {
                        "id": 13,
                        "abbrev": "FLA",
                        "logo": "https://assets.nhle.com/logos/nhl/svg/FLA_light.svg"
                    },
                    "homeTeam": {
                        "id": 10,
                        "abbrev": "TOR",
                        "logo": "https://assets.nhle.com/logos/nhl/svg/TOR_light.svg"
                    },
                    "gameState": "OFF"
                }
            ]
        }"#;

        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/club-schedule-season/FLA/20232024")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(fixture)
            .create_async()
            .await;

        let client = Client::new().unwrap();
        let result = client
            .club_schedule_season_at(Endpoint::Custom(server.url()), "FLA", Season::new(2023))
            .await
            .expect("deserialization should succeed");

        mock.assert_async().await;
        assert_eq!(result.games.len(), 2);
        assert_eq!(result.games[0].game_type, GameType::RegularSeason);
        assert_eq!(result.games[0].game_state, GameState::Off);
        assert_eq!(result.games[1].game_type, GameType::Playoffs);
        assert_eq!(result.games[1].id, GameId::new(2023030111));
    }

    // ===== Edge contract tables (step 6.6) =====
    //
    // Every Edge client method is exercised by both tables below via a single
    // `EdgeCase` entry. Adding a 23rd Edge method without adding a call fn +
    // entry here is conspicuous: the method itself compiles fine, but neither
    // the path-contract test nor the 404-propagation test will cover it.

    type EdgeMethodFuture<'a> = Pin<Box<dyn Future<Output = Result<(), NHLApiError>> + 'a>>;
    type EdgeMethodFn = for<'a> fn(&'a Client, Endpoint) -> EdgeMethodFuture<'a>;

    struct EdgeCase {
        name: &'static str,
        path: String,
        call: EdgeMethodFn,
    }

    // Shared fixtures: 2024-2025 regular season -> APIString "20242025", GameType.to_int() 2.
    const EDGE_TEST_PLAYER_ID: i64 = 8478402;
    const EDGE_TEST_TEAM_ID: i64 = 22;

    fn edge_test_season() -> Season {
        Season::new(2024)
    }

    const EDGE_TEST_GAME_TYPE: GameType = GameType::RegularSeason;

    fn edge_skater_detail_call(client: &Client, endpoint: Endpoint) -> EdgeMethodFuture<'_> {
        Box::pin(async move {
            client
                .edge_skater_detail_at(
                    endpoint,
                    PlayerId::new(EDGE_TEST_PLAYER_ID),
                    edge_test_season(),
                    EDGE_TEST_GAME_TYPE,
                )
                .await
                .map(|_| ())
        })
    }

    fn edge_skater_speed_detail_call(client: &Client, endpoint: Endpoint) -> EdgeMethodFuture<'_> {
        Box::pin(async move {
            client
                .edge_skater_speed_detail_at(
                    endpoint,
                    PlayerId::new(EDGE_TEST_PLAYER_ID),
                    edge_test_season(),
                    EDGE_TEST_GAME_TYPE,
                )
                .await
                .map(|_| ())
        })
    }

    fn edge_skater_distance_detail_call(
        client: &Client,
        endpoint: Endpoint,
    ) -> EdgeMethodFuture<'_> {
        Box::pin(async move {
            client
                .edge_skater_distance_detail_at(
                    endpoint,
                    PlayerId::new(EDGE_TEST_PLAYER_ID),
                    edge_test_season(),
                    EDGE_TEST_GAME_TYPE,
                )
                .await
                .map(|_| ())
        })
    }

    fn edge_skater_shot_speed_detail_call(
        client: &Client,
        endpoint: Endpoint,
    ) -> EdgeMethodFuture<'_> {
        Box::pin(async move {
            client
                .edge_skater_shot_speed_detail_at(
                    endpoint,
                    PlayerId::new(EDGE_TEST_PLAYER_ID),
                    edge_test_season(),
                    EDGE_TEST_GAME_TYPE,
                )
                .await
                .map(|_| ())
        })
    }

    fn edge_skater_shot_location_detail_call(
        client: &Client,
        endpoint: Endpoint,
    ) -> EdgeMethodFuture<'_> {
        Box::pin(async move {
            client
                .edge_skater_shot_location_detail_at(
                    endpoint,
                    PlayerId::new(EDGE_TEST_PLAYER_ID),
                    edge_test_season(),
                    EDGE_TEST_GAME_TYPE,
                )
                .await
                .map(|_| ())
        })
    }

    fn edge_skater_zone_time_call(client: &Client, endpoint: Endpoint) -> EdgeMethodFuture<'_> {
        Box::pin(async move {
            client
                .edge_skater_zone_time_at(
                    endpoint,
                    PlayerId::new(EDGE_TEST_PLAYER_ID),
                    edge_test_season(),
                    EDGE_TEST_GAME_TYPE,
                )
                .await
                .map(|_| ())
        })
    }

    fn edge_skater_comparison_call(client: &Client, endpoint: Endpoint) -> EdgeMethodFuture<'_> {
        Box::pin(async move {
            client
                .edge_skater_comparison_at(
                    endpoint,
                    PlayerId::new(EDGE_TEST_PLAYER_ID),
                    edge_test_season(),
                    EDGE_TEST_GAME_TYPE,
                )
                .await
                .map(|_| ())
        })
    }

    fn edge_skater_landing_call(client: &Client, endpoint: Endpoint) -> EdgeMethodFuture<'_> {
        Box::pin(async move {
            client
                .edge_skater_landing_at(endpoint, edge_test_season(), EDGE_TEST_GAME_TYPE)
                .await
                .map(|_| ())
        })
    }

    fn edge_goalie_detail_call(client: &Client, endpoint: Endpoint) -> EdgeMethodFuture<'_> {
        Box::pin(async move {
            client
                .edge_goalie_detail_at(
                    endpoint,
                    PlayerId::new(EDGE_TEST_PLAYER_ID),
                    edge_test_season(),
                    EDGE_TEST_GAME_TYPE,
                )
                .await
                .map(|_| ())
        })
    }

    fn edge_goalie_5v5_detail_call(client: &Client, endpoint: Endpoint) -> EdgeMethodFuture<'_> {
        Box::pin(async move {
            client
                .edge_goalie_5v5_detail_at(
                    endpoint,
                    PlayerId::new(EDGE_TEST_PLAYER_ID),
                    edge_test_season(),
                    EDGE_TEST_GAME_TYPE,
                )
                .await
                .map(|_| ())
        })
    }

    fn edge_goalie_shot_location_detail_call(
        client: &Client,
        endpoint: Endpoint,
    ) -> EdgeMethodFuture<'_> {
        Box::pin(async move {
            client
                .edge_goalie_shot_location_detail_at(
                    endpoint,
                    PlayerId::new(EDGE_TEST_PLAYER_ID),
                    edge_test_season(),
                    EDGE_TEST_GAME_TYPE,
                )
                .await
                .map(|_| ())
        })
    }

    fn edge_goalie_save_pctg_detail_call(
        client: &Client,
        endpoint: Endpoint,
    ) -> EdgeMethodFuture<'_> {
        Box::pin(async move {
            client
                .edge_goalie_save_pctg_detail_at(
                    endpoint,
                    PlayerId::new(EDGE_TEST_PLAYER_ID),
                    edge_test_season(),
                    EDGE_TEST_GAME_TYPE,
                )
                .await
                .map(|_| ())
        })
    }

    fn edge_goalie_comparison_call(client: &Client, endpoint: Endpoint) -> EdgeMethodFuture<'_> {
        Box::pin(async move {
            client
                .edge_goalie_comparison_at(
                    endpoint,
                    PlayerId::new(EDGE_TEST_PLAYER_ID),
                    edge_test_season(),
                    EDGE_TEST_GAME_TYPE,
                )
                .await
                .map(|_| ())
        })
    }

    fn edge_goalie_landing_call(client: &Client, endpoint: Endpoint) -> EdgeMethodFuture<'_> {
        Box::pin(async move {
            client
                .edge_goalie_landing_at(endpoint, edge_test_season(), EDGE_TEST_GAME_TYPE)
                .await
                .map(|_| ())
        })
    }

    fn edge_team_detail_call(client: &Client, endpoint: Endpoint) -> EdgeMethodFuture<'_> {
        Box::pin(async move {
            client
                .edge_team_detail_at(
                    endpoint,
                    TeamId::new(EDGE_TEST_TEAM_ID),
                    edge_test_season(),
                    EDGE_TEST_GAME_TYPE,
                )
                .await
                .map(|_| ())
        })
    }

    fn edge_team_speed_detail_call(client: &Client, endpoint: Endpoint) -> EdgeMethodFuture<'_> {
        Box::pin(async move {
            client
                .edge_team_speed_detail_at(
                    endpoint,
                    TeamId::new(EDGE_TEST_TEAM_ID),
                    edge_test_season(),
                    EDGE_TEST_GAME_TYPE,
                )
                .await
                .map(|_| ())
        })
    }

    fn edge_team_distance_detail_call(client: &Client, endpoint: Endpoint) -> EdgeMethodFuture<'_> {
        Box::pin(async move {
            client
                .edge_team_distance_detail_at(
                    endpoint,
                    TeamId::new(EDGE_TEST_TEAM_ID),
                    edge_test_season(),
                    EDGE_TEST_GAME_TYPE,
                )
                .await
                .map(|_| ())
        })
    }

    fn edge_team_shot_speed_detail_call(
        client: &Client,
        endpoint: Endpoint,
    ) -> EdgeMethodFuture<'_> {
        Box::pin(async move {
            client
                .edge_team_shot_speed_detail_at(
                    endpoint,
                    TeamId::new(EDGE_TEST_TEAM_ID),
                    edge_test_season(),
                    EDGE_TEST_GAME_TYPE,
                )
                .await
                .map(|_| ())
        })
    }

    fn edge_team_shot_location_detail_call(
        client: &Client,
        endpoint: Endpoint,
    ) -> EdgeMethodFuture<'_> {
        Box::pin(async move {
            client
                .edge_team_shot_location_detail_at(
                    endpoint,
                    TeamId::new(EDGE_TEST_TEAM_ID),
                    edge_test_season(),
                    EDGE_TEST_GAME_TYPE,
                )
                .await
                .map(|_| ())
        })
    }

    fn edge_team_zone_time_details_call(
        client: &Client,
        endpoint: Endpoint,
    ) -> EdgeMethodFuture<'_> {
        Box::pin(async move {
            client
                .edge_team_zone_time_details_at(
                    endpoint,
                    TeamId::new(EDGE_TEST_TEAM_ID),
                    edge_test_season(),
                    EDGE_TEST_GAME_TYPE,
                )
                .await
                .map(|_| ())
        })
    }

    fn edge_team_comparison_call(client: &Client, endpoint: Endpoint) -> EdgeMethodFuture<'_> {
        Box::pin(async move {
            client
                .edge_team_comparison_at(
                    endpoint,
                    TeamId::new(EDGE_TEST_TEAM_ID),
                    edge_test_season(),
                    EDGE_TEST_GAME_TYPE,
                )
                .await
                .map(|_| ())
        })
    }

    fn edge_team_landing_call(client: &Client, endpoint: Endpoint) -> EdgeMethodFuture<'_> {
        Box::pin(async move {
            client
                .edge_team_landing_at(endpoint, edge_test_season(), EDGE_TEST_GAME_TYPE)
                .await
                .map(|_| ())
        })
    }

    fn edge_cases() -> Vec<EdgeCase> {
        vec![
            EdgeCase {
                name: "edge_skater_detail",
                path: format!(
                    "/edge/skater-detail/{}/{}/{}",
                    EDGE_TEST_PLAYER_ID,
                    edge_test_season().to_api_string(),
                    EDGE_TEST_GAME_TYPE.to_int()
                ),
                call: edge_skater_detail_call,
            },
            EdgeCase {
                name: "edge_skater_speed_detail",
                path: format!(
                    "/edge/skater-skating-speed-detail/{}/{}/{}",
                    EDGE_TEST_PLAYER_ID,
                    edge_test_season().to_api_string(),
                    EDGE_TEST_GAME_TYPE.to_int()
                ),
                call: edge_skater_speed_detail_call,
            },
            EdgeCase {
                name: "edge_skater_distance_detail",
                path: format!(
                    "/edge/skater-skating-distance-detail/{}/{}/{}",
                    EDGE_TEST_PLAYER_ID,
                    edge_test_season().to_api_string(),
                    EDGE_TEST_GAME_TYPE.to_int()
                ),
                call: edge_skater_distance_detail_call,
            },
            EdgeCase {
                name: "edge_skater_shot_speed_detail",
                path: format!(
                    "/edge/skater-shot-speed-detail/{}/{}/{}",
                    EDGE_TEST_PLAYER_ID,
                    edge_test_season().to_api_string(),
                    EDGE_TEST_GAME_TYPE.to_int()
                ),
                call: edge_skater_shot_speed_detail_call,
            },
            EdgeCase {
                name: "edge_skater_shot_location_detail",
                path: format!(
                    "/edge/skater-shot-location-detail/{}/{}/{}",
                    EDGE_TEST_PLAYER_ID,
                    edge_test_season().to_api_string(),
                    EDGE_TEST_GAME_TYPE.to_int()
                ),
                call: edge_skater_shot_location_detail_call,
            },
            EdgeCase {
                name: "edge_skater_zone_time",
                path: format!(
                    "/edge/skater-zone-time/{}/{}/{}",
                    EDGE_TEST_PLAYER_ID,
                    edge_test_season().to_api_string(),
                    EDGE_TEST_GAME_TYPE.to_int()
                ),
                call: edge_skater_zone_time_call,
            },
            EdgeCase {
                name: "edge_skater_comparison",
                path: format!(
                    "/edge/skater-comparison/{}/{}/{}",
                    EDGE_TEST_PLAYER_ID,
                    edge_test_season().to_api_string(),
                    EDGE_TEST_GAME_TYPE.to_int()
                ),
                call: edge_skater_comparison_call,
            },
            EdgeCase {
                name: "edge_skater_landing",
                path: format!(
                    "/edge/skater-landing/{}/{}",
                    edge_test_season().to_api_string(),
                    EDGE_TEST_GAME_TYPE.to_int()
                ),
                call: edge_skater_landing_call,
            },
            EdgeCase {
                name: "edge_goalie_detail",
                path: format!(
                    "/edge/goalie-detail/{}/{}/{}",
                    EDGE_TEST_PLAYER_ID,
                    edge_test_season().to_api_string(),
                    EDGE_TEST_GAME_TYPE.to_int()
                ),
                call: edge_goalie_detail_call,
            },
            EdgeCase {
                name: "edge_goalie_5v5_detail",
                path: format!(
                    "/edge/goalie-5v5-detail/{}/{}/{}",
                    EDGE_TEST_PLAYER_ID,
                    edge_test_season().to_api_string(),
                    EDGE_TEST_GAME_TYPE.to_int()
                ),
                call: edge_goalie_5v5_detail_call,
            },
            EdgeCase {
                name: "edge_goalie_shot_location_detail",
                path: format!(
                    "/edge/goalie-shot-location-detail/{}/{}/{}",
                    EDGE_TEST_PLAYER_ID,
                    edge_test_season().to_api_string(),
                    EDGE_TEST_GAME_TYPE.to_int()
                ),
                call: edge_goalie_shot_location_detail_call,
            },
            EdgeCase {
                name: "edge_goalie_save_pctg_detail",
                path: format!(
                    "/edge/goalie-save-percentage-detail/{}/{}/{}",
                    EDGE_TEST_PLAYER_ID,
                    edge_test_season().to_api_string(),
                    EDGE_TEST_GAME_TYPE.to_int()
                ),
                call: edge_goalie_save_pctg_detail_call,
            },
            EdgeCase {
                name: "edge_goalie_comparison",
                path: format!(
                    "/edge/goalie-comparison/{}/{}/{}",
                    EDGE_TEST_PLAYER_ID,
                    edge_test_season().to_api_string(),
                    EDGE_TEST_GAME_TYPE.to_int()
                ),
                call: edge_goalie_comparison_call,
            },
            EdgeCase {
                name: "edge_goalie_landing",
                path: format!(
                    "/edge/goalie-landing/{}/{}",
                    edge_test_season().to_api_string(),
                    EDGE_TEST_GAME_TYPE.to_int()
                ),
                call: edge_goalie_landing_call,
            },
            EdgeCase {
                name: "edge_team_detail",
                path: format!(
                    "/edge/team-detail/{}/{}/{}",
                    EDGE_TEST_TEAM_ID,
                    edge_test_season().to_api_string(),
                    EDGE_TEST_GAME_TYPE.to_int()
                ),
                call: edge_team_detail_call,
            },
            EdgeCase {
                name: "edge_team_speed_detail",
                path: format!(
                    "/edge/team-skating-speed-detail/{}/{}/{}",
                    EDGE_TEST_TEAM_ID,
                    edge_test_season().to_api_string(),
                    EDGE_TEST_GAME_TYPE.to_int()
                ),
                call: edge_team_speed_detail_call,
            },
            EdgeCase {
                name: "edge_team_distance_detail",
                path: format!(
                    "/edge/team-skating-distance-detail/{}/{}/{}",
                    EDGE_TEST_TEAM_ID,
                    edge_test_season().to_api_string(),
                    EDGE_TEST_GAME_TYPE.to_int()
                ),
                call: edge_team_distance_detail_call,
            },
            EdgeCase {
                name: "edge_team_shot_speed_detail",
                path: format!(
                    "/edge/team-shot-speed-detail/{}/{}/{}",
                    EDGE_TEST_TEAM_ID,
                    edge_test_season().to_api_string(),
                    EDGE_TEST_GAME_TYPE.to_int()
                ),
                call: edge_team_shot_speed_detail_call,
            },
            EdgeCase {
                name: "edge_team_shot_location_detail",
                path: format!(
                    "/edge/team-shot-location-detail/{}/{}/{}",
                    EDGE_TEST_TEAM_ID,
                    edge_test_season().to_api_string(),
                    EDGE_TEST_GAME_TYPE.to_int()
                ),
                call: edge_team_shot_location_detail_call,
            },
            EdgeCase {
                name: "edge_team_zone_time_details",
                path: format!(
                    "/edge/team-zone-time-details/{}/{}/{}",
                    EDGE_TEST_TEAM_ID,
                    edge_test_season().to_api_string(),
                    EDGE_TEST_GAME_TYPE.to_int()
                ),
                call: edge_team_zone_time_details_call,
            },
            EdgeCase {
                name: "edge_team_comparison",
                path: format!(
                    "/edge/team-comparison/{}/{}/{}",
                    EDGE_TEST_TEAM_ID,
                    edge_test_season().to_api_string(),
                    EDGE_TEST_GAME_TYPE.to_int()
                ),
                call: edge_team_comparison_call,
            },
            EdgeCase {
                name: "edge_team_landing",
                path: format!(
                    "/edge/team-landing/{}/{}",
                    edge_test_season().to_api_string(),
                    EDGE_TEST_GAME_TYPE.to_int()
                ),
                call: edge_team_landing_call,
            },
        ]
    }

    /// Verifies every Edge client method requests the documented URL path.
    /// Each case is run against a mock server that only answers on the exact
    /// expected path with `200 {}` -- an empty JSON object deserializes cleanly
    /// into every Edge response type (the 6.2 `{}`-deserializes rule), and a
    /// request to any other path leaves the mock unmatched, failing the request.
    #[tokio::test]
    async fn test_edge_all_client_methods_path_contract() {
        for case in edge_cases() {
            let mut server = mockito::Server::new_async().await;
            let mock = server
                .mock("GET", case.path.as_str())
                .with_status(200)
                .with_header("content-type", "application/json")
                .with_body("{}")
                .create_async()
                .await;

            let client = Client::new().unwrap();
            let result = (case.call)(&client, Endpoint::Custom(server.url())).await;

            assert!(
                result.is_ok(),
                "{} did not request the expected path {:?}: {:?}",
                case.name,
                case.path,
                result.err()
            );
            mock.assert_async().await;
        }
    }

    /// Verifies every Edge client method propagates a 404 as
    /// `NHLApiError::ResourceNotFound` rather than swallowing or mis-mapping it.
    #[tokio::test]
    async fn test_edge_all_client_methods_404_propagates_error() {
        for case in edge_cases() {
            let mut server = mockito::Server::new_async().await;
            let mock = server
                .mock("GET", case.path.as_str())
                .with_status(404)
                .create_async()
                .await;

            let client = Client::new().unwrap();
            let result = (case.call)(&client, Endpoint::Custom(server.url())).await;

            match result {
                Err(NHLApiError::ResourceNotFound { .. }) => {}
                Err(other) => panic!("{} expected ResourceNotFound, got {:?}", case.name, other),
                Ok(_) => panic!("{} expected an error for 404, got Ok", case.name),
            }
            mock.assert_async().await;
        }
    }
}
