use std::collections::HashMap;

use serde::de::DeserializeOwned;

use super::Client;
use crate::error::NHLApiError;
use crate::http_client::Endpoint;
use crate::stats_query::{validate_report, SortDirection, StatsQuery};
use crate::types::{
    Country, DraftRounds, GlossaryEntry, GoalieSummary, SeasonRules, SkaterSummary, StatsConfig,
    StatsPage, TeamSummary,
};

const SUMMARY_REPORT: &str = "summary";

/// Sort applied to `skater_summary` queries without one: the nhl-api-py default.
const SKATER_SUMMARY_SORT: &[(&str, SortDirection)] = &[
    ("points", SortDirection::Desc),
    ("gamesPlayed", SortDirection::Asc),
    ("playerId", SortDirection::Asc),
];

/// Sort applied to `goalie_summary` queries without one: the nhl-api-py default.
const GOALIE_SUMMARY_SORT: &[(&str, SortDirection)] = &[
    ("wins", SortDirection::Desc),
    ("gamesPlayed", SortDirection::Asc),
    ("playerId", SortDirection::Asc),
];

/// Sort applied to `team_summary` queries without one: the nhl-api-py
/// default, with the tiebreaker the report accepts in the query's mode.
const TEAM_SUMMARY_PRIMARY_SORT: &[(&str, SortDirection)] = &[
    ("points", SortDirection::Desc),
    ("wins", SortDirection::Desc),
];
const TEAM_SUMMARY_TIEBREAKER: (&str, SortDirection) = ("teamId", SortDirection::Asc);
/// Aggregate rows have no `teamId`, and sorting by it is a 400.
const TEAM_SUMMARY_AGGREGATE_TIEBREAKER: (&str, SortDirection) =
    ("franchiseId", SortDirection::Asc);

/// The stats endpoint families a report belongs to.
#[derive(Debug, Clone, Copy)]
enum ReportKind {
    Skater,
    Goalie,
    Team,
}

impl ReportKind {
    fn path(self, report: &str) -> String {
        let family = match self {
            Self::Skater => "skater",
            Self::Goalie => "goalie",
            Self::Team => "team",
        };
        format!("en/{family}/{report}")
    }
}

impl Client {
    /// Gets the glossary of statistic definitions.
    pub async fn glossary(&self) -> Result<Vec<GlossaryEntry>, NHLApiError> {
        self.stats_list("en/glossary").await
    }

    /// Gets the reports the stats endpoints offer, with their columns,
    /// filterable fields and sort keys. The keys of its report maps are the
    /// valid report names for [`skater_report`](Self::skater_report),
    /// [`goalie_report`](Self::goalie_report) and
    /// [`team_report`](Self::team_report).
    pub async fn stats_config(&self) -> Result<StatsConfig, NHLApiError> {
        self.client
            .get_json(Endpoint::ApiStats, "en/config", None)
            .await
    }

    /// Gets every country players have been born in or represented.
    pub async fn countries(&self) -> Result<Vec<Country>, NHLApiError> {
        self.stats_list("en/country").await
    }

    /// Gets the rules and calendar of every season since 1917-18.
    pub async fn season_rules(&self) -> Result<Vec<SeasonRules>, NHLApiError> {
        self.stats_list("en/season").await
    }

    /// Gets the number of rounds of every entry draft.
    pub async fn draft_rounds(&self) -> Result<Vec<DraftRounds>, NHLApiError> {
        self.stats_list("en/draft").await
    }

    /// Gets a skater report page, deserializing each row as `T`.
    ///
    /// Report names come from [`stats_config`](Self::stats_config)
    /// (`player_report_data`). Use [`SkaterSummary`] for `summary` (or
    /// [`skater_summary`](Self::skater_summary)), or
    /// `serde_json::Value` for a report without a typed row.
    ///
    /// The report name and the query are validated first; an invalid one
    /// returns [`NHLApiError::InvalidStatsQuery`] without a request.
    pub async fn skater_report<T: DeserializeOwned>(
        &self,
        report: &str,
        query: &StatsQuery,
    ) -> Result<StatsPage<T>, NHLApiError> {
        self.stats_report(ReportKind::Skater, report, query).await
    }

    /// Gets a goalie report page, deserializing each row as `T`; see
    /// [`skater_report`](Self::skater_report). Report names are in
    /// `goalie_report_data` of [`stats_config`](Self::stats_config).
    pub async fn goalie_report<T: DeserializeOwned>(
        &self,
        report: &str,
        query: &StatsQuery,
    ) -> Result<StatsPage<T>, NHLApiError> {
        self.stats_report(ReportKind::Goalie, report, query).await
    }

    /// Gets a team report page, deserializing each row as `T`; see
    /// [`skater_report`](Self::skater_report). Report names are in
    /// `team_report_data` of [`stats_config`](Self::stats_config).
    pub async fn team_report<T: DeserializeOwned>(
        &self,
        report: &str,
        query: &StatsQuery,
    ) -> Result<StatsPage<T>, NHLApiError> {
        self.stats_report(ReportKind::Team, report, query).await
    }

    /// Gets the skater `summary` report.
    ///
    /// Without a sort in `query` the rows are ordered by points (descending),
    /// then games played and player id (ascending), as nhl-api-py does.
    /// Without a limit the API returns 50 rows.
    ///
    /// # Example
    /// ```no_run
    /// # use nhl_api::{Client, GameType, Season, StatsQuery};
    /// # async fn example() -> Result<(), nhl_api::NHLApiError> {
    /// let client = Client::new()?;
    /// let query = StatsQuery::new()
    ///     .season(Season::new(2024))
    ///     .game_type(GameType::RegularSeason)
    ///     .limit(10);
    /// let page = client.skater_summary(&query).await?;
    /// for skater in &page.data {
    ///     println!("{skater}");
    /// }
    /// println!("{} skaters in all", page.total);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn skater_summary(
        &self,
        query: &StatsQuery,
    ) -> Result<StatsPage<SkaterSummary>, NHLApiError> {
        let query = query.with_default_sort(SKATER_SUMMARY_SORT);
        self.skater_report(SUMMARY_REPORT, &query).await
    }

    /// Gets the goalie `summary` report.
    ///
    /// Without a sort in `query` the rows are ordered by wins (descending),
    /// then games played and player id (ascending), as nhl-api-py does.
    /// Without a limit the API returns 50 rows.
    ///
    /// # Example
    /// ```no_run
    /// # use nhl_api::{Client, GameType, Season, StatsQuery};
    /// # async fn example() -> Result<(), nhl_api::NHLApiError> {
    /// let client = Client::new()?;
    /// let query = StatsQuery::new()
    ///     .season(Season::new(2024))
    ///     .game_type(GameType::RegularSeason)
    ///     .limit(5);
    /// let page = client.goalie_summary(&query).await?;
    /// for goalie in &page.data {
    ///     println!("{goalie}");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn goalie_summary(
        &self,
        query: &StatsQuery,
    ) -> Result<StatsPage<GoalieSummary>, NHLApiError> {
        let query = query.with_default_sort(GOALIE_SUMMARY_SORT);
        self.goalie_report(SUMMARY_REPORT, &query).await
    }

    /// Gets the team `summary` report.
    ///
    /// Without a sort in `query` the rows are ordered by points and wins
    /// (descending), then team id (ascending), as nhl-api-py does; an
    /// [aggregate](StatsQuery::aggregate) query has no team id, so it
    /// breaks ties by franchise id instead.
    ///
    /// # Example
    /// ```no_run
    /// # use nhl_api::{Client, GameType, Season, StatsQuery};
    /// # async fn example() -> Result<(), nhl_api::NHLApiError> {
    /// let client = Client::new()?;
    /// let query = StatsQuery::new()
    ///     .season(Season::new(2024))
    ///     .game_type(GameType::RegularSeason)
    ///     .unlimited();
    /// let page = client.team_summary(&query).await?;
    /// for team in &page.data {
    ///     println!("{team}");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn team_summary(
        &self,
        query: &StatsQuery,
    ) -> Result<StatsPage<TeamSummary>, NHLApiError> {
        let tiebreaker = if query.is_aggregate() {
            TEAM_SUMMARY_AGGREGATE_TIEBREAKER
        } else {
            TEAM_SUMMARY_TIEBREAKER
        };
        let default_sort = [TEAM_SUMMARY_PRIMARY_SORT, &[tiebreaker]].concat();
        let query = query.with_default_sort(&default_sort);
        self.team_report(SUMMARY_REPORT, &query).await
    }

    /// Gets a `{"data": [...], "total": n}` list and returns its rows.
    async fn stats_list<T: DeserializeOwned>(&self, resource: &str) -> Result<Vec<T>, NHLApiError> {
        let page: StatsPage<T> = self
            .client
            .get_json(Endpoint::ApiStats, resource, None)
            .await?;
        Ok(page.data)
    }

    async fn stats_report<T: DeserializeOwned>(
        &self,
        kind: ReportKind,
        report: &str,
        query: &StatsQuery,
    ) -> Result<StatsPage<T>, NHLApiError> {
        validate_report(report)?;
        query.validate()?;
        let params: HashMap<String, String> = query
            .query_pairs()
            .into_iter()
            .map(|(name, value)| (name.to_string(), value))
            .collect();
        self.client
            .get_json(Endpoint::ApiStats, &kind.path(report), Some(params))
            .await
    }
}
