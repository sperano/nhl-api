//! Typed query parameters for the `stats/rest` report endpoints.
//!
//! [`StatsQuery`] collects the filters, sort order and paging a report takes,
//! and renders them as the request's query string. Filters are
//! [`CayenneExpr`]s (the API's filter language), either built by hand or
//! through the typed helpers such as [`StatsQuery::season`].

use std::borrow::Cow;

use serde::Serialize;
use thiserror::Error;

use crate::date::Season;
use crate::ids::{FranchiseId, GameId, PlayerId, TeamId};
use crate::types::{GameType, GoalieDecision, Handedness, HomeRoad, Position};

mod cayenne;

pub use cayenne::{CayenneExpr, CayenneField, CayenneValue, CompareOp};

const SEASON_ID_FIELD: &str = "seasonId";
const POSITION_CODE_FIELD: &str = "positionCode";
const IS_AGGREGATE_PARAM: &str = "isAggregate";
const IS_GAME_PARAM: &str = "isGame";
const CAYENNE_EXP_PARAM: &str = "cayenneExp";
const FACT_CAYENNE_EXP_PARAM: &str = "factCayenneExp";
const SORT_PARAM: &str = "sort";
const START_PARAM: &str = "start";
const LIMIT_PARAM: &str = "limit";
/// The `limit` value that makes the API return every matching row.
const UNLIMITED_LIMIT: &str = "-1";

/// Why a [`StatsQuery`] or a report name was rejected before any request.
#[derive(Error, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum StatsQueryError {
    /// A filter or sort field is not a plain identifier
    /// (`[A-Za-z][A-Za-z0-9_]*`).
    #[error("invalid field name {0:?}: expected letters, digits and underscores, starting with a letter")]
    InvalidField(String),

    /// A report name is not a plain identifier (`[A-Za-z][A-Za-z0-9_]*`).
    #[error("invalid report name {0:?}: expected letters, digits and underscores, starting with a letter")]
    InvalidReport(String),

    /// A filter on `field` compares it with NaN or an infinity.
    #[error("filter on {field:?} uses a non-finite number")]
    NonFiniteValue { field: String },

    /// A filter on this field has an empty `in ()` list.
    #[error("filter on {0:?} has an empty `in` list")]
    EmptyInList(String),

    /// `limit(0)` makes the API return every row; use
    /// [`StatsQuery::unlimited`] for that.
    #[error("limit must be at least 1; use unlimited() to fetch every row")]
    ZeroLimit,
}

/// The direction of one [`SortKey`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum SortDirection {
    #[serde(rename = "ASC")]
    Asc,
    #[serde(rename = "DESC")]
    Desc,
    /// Ascending, ignoring case (text fields).
    #[serde(rename = "ASC_CI")]
    AscIgnoreCase,
    /// Descending, ignoring case (text fields).
    #[serde(rename = "DESC_CI")]
    DescIgnoreCase,
}

/// One sort criterion: a report field and a direction.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SortKey {
    pub property: String,
    pub direction: SortDirection,
}

impl SortKey {
    pub fn new(property: impl Into<String>, direction: SortDirection) -> Self {
        Self {
            property: property.into(),
            direction,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RowLimit {
    Count(u32),
    Unlimited,
}

/// Filters, sort order and paging for a `stats/rest` report.
///
/// Every method consumes and returns the query. All filters are ANDed
/// together, and without [`limit`](Self::limit) the API returns its default
/// page of 50 rows.
///
/// ```
/// use nhl_api::{GameType, Season, SortDirection, StatsQuery};
///
/// let query = StatsQuery::new()
///     .season(Season::new(2024))
///     .game_type(GameType::RegularSeason)
///     .sort_by("points", SortDirection::Desc)
///     .limit(10);
/// assert_eq!(query.cayenne_exp(), "seasonId=20242025 and gameTypeId=2");
/// ```
#[derive(Debug, Clone, Default, PartialEq)]
pub struct StatsQuery {
    filters: Vec<CayenneExpr>,
    fact_filters: Vec<CayenneExpr>,
    sort: Vec<SortKey>,
    start: u32,
    limit: Option<RowLimit>,
    aggregate: bool,
    by_game: bool,
}

impl StatsQuery {
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a row filter (`cayenneExp`).
    pub fn filter(mut self, expr: CayenneExpr) -> Self {
        self.filters.push(expr);
        self
    }

    /// Adds a filter on the report's computed columns (`factCayenneExp`),
    /// e.g. `gamesPlayed>=80`.
    pub fn fact_filter(mut self, expr: CayenneExpr) -> Self {
        self.fact_filters.push(expr);
        self
    }

    /// Only rows of this season (`seasonId=20242025`).
    pub fn season(self, season: Season) -> Self {
        self.filter(CayenneExpr::field(SEASON_ID_FIELD).eq(season))
    }

    /// Only rows from `from` through `to`, inclusive.
    pub fn seasons(self, from: Season, to: Season) -> Self {
        self.filter(
            CayenneExpr::field(SEASON_ID_FIELD)
                .ge(from)
                .and(CayenneExpr::field(SEASON_ID_FIELD).le(to)),
        )
    }

    pub fn game_type(self, game_type: GameType) -> Self {
        self.filter(CayenneExpr::field("gameTypeId").eq(game_type))
    }

    pub fn franchise(self, franchise: FranchiseId) -> Self {
        self.filter(CayenneExpr::field("franchiseId").eq(franchise))
    }

    pub fn opponent_franchise(self, franchise: FranchiseId) -> Self {
        self.filter(CayenneExpr::field("opponentFranchiseId").eq(franchise))
    }

    pub fn team(self, team: TeamId) -> Self {
        self.filter(CayenneExpr::field("teamId").eq(team))
    }

    pub fn player(self, player: PlayerId) -> Self {
        self.filter(CayenneExpr::field("playerId").eq(player))
    }

    pub fn game(self, game: GameId) -> Self {
        self.filter(CayenneExpr::field("gameId").eq(game))
    }

    /// Only skaters/goalies in this position. The stats API codes wingers
    /// `L`/`R` where [`Position::code`] says `LW`/`RW`, and
    /// [`Position::Forward`] matches centers and both wings.
    pub fn position(self, position: Position) -> Self {
        let code = |code: &str| CayenneExpr::field(POSITION_CODE_FIELD).eq(code);
        self.filter(match position {
            Position::Center => code("C"),
            Position::LeftWing => code("L"),
            Position::RightWing => code("R"),
            Position::Defense => code("D"),
            Position::Goalie => code("G"),
            Position::Forward => code("C").or(code("L")).or(code("R")),
        })
    }

    /// Only players of this nationality, by three-letter code (`SWE`).
    pub fn nationality(self, code: impl Into<String>) -> Self {
        self.filter(CayenneExpr::field("nationalityCode").eq(code.into()))
    }

    /// Only players born in this country, by three-letter code (`USA`).
    pub fn birth_country(self, code: impl Into<String>) -> Self {
        self.filter(CayenneExpr::field("birthCountryCode").eq(code.into()))
    }

    pub fn draft_year(self, year: u16) -> Self {
        self.filter(CayenneExpr::field("draftYear").eq(year))
    }

    pub fn draft_round(self, round: u8) -> Self {
        self.filter(CayenneExpr::field("draftRound").eq(round))
    }

    /// Only rookies (`true`) or only non-rookies (`false`).
    pub fn rookie(self, rookie: bool) -> Self {
        self.filter(CayenneExpr::field("isRookie").eq(u8::from(rookie)))
    }

    /// Only players still active.
    pub fn active(self) -> Self {
        self.filter(CayenneExpr::field("active").eq(1u8))
    }

    /// Only Hall of Fame members.
    pub fn hall_of_fame(self) -> Self {
        self.filter(CayenneExpr::field("isInHallOfFame").eq(1u8))
    }

    pub fn shoots_catches(self, handedness: Handedness) -> Self {
        self.filter(CayenneExpr::field("shootsCatches").eq(handedness.code()))
    }

    /// Only games played at home or on the road (use with
    /// [`by_game`](Self::by_game)).
    pub fn home_road(self, home_road: HomeRoad) -> Self {
        self.filter(CayenneExpr::field("homeRoad").eq(home_road.code()))
    }

    /// Only goalie games with this decision (use with
    /// [`by_game`](Self::by_game)).
    pub fn decision(self, decision: GoalieDecision) -> Self {
        self.filter(CayenneExpr::field("decision").eq(decision.code()))
    }

    /// Appends a sort criterion; earlier ones take precedence.
    pub fn sort_by(mut self, field: impl Into<String>, direction: SortDirection) -> Self {
        self.sort.push(SortKey::new(field, direction));
        self
    }

    /// Skips the first `offset` rows.
    pub fn start(mut self, offset: u32) -> Self {
        self.start = offset;
        self
    }

    /// Returns at most `count` rows. Zero is rejected by
    /// [`validate`](Self::validate); use [`unlimited`](Self::unlimited).
    pub fn limit(mut self, count: u32) -> Self {
        self.limit = Some(RowLimit::Count(count));
        self
    }

    /// Returns every matching row instead of the API's default page of 50.
    pub fn unlimited(mut self) -> Self {
        self.limit = Some(RowLimit::Unlimited);
        self
    }

    /// One row per player or team across the filtered seasons, instead of one
    /// per season.
    pub fn aggregate(mut self, aggregate: bool) -> Self {
        self.aggregate = aggregate;
        self
    }

    /// One row per game instead of one per season.
    pub fn by_game(mut self, by_game: bool) -> Self {
        self.by_game = by_game;
        self
    }

    /// The `cayenneExp` text: every filter ANDed; empty without filters.
    pub fn cayenne_exp(&self) -> String {
        cayenne::render_conjunction(&self.filters)
    }

    /// The `factCayenneExp` text, `None` without fact filters.
    pub fn fact_cayenne_exp(&self) -> Option<String> {
        let exp = cayenne::render_conjunction(&self.fact_filters);
        (!exp.is_empty()).then_some(exp)
    }

    pub fn sort_keys(&self) -> &[SortKey] {
        &self.sort
    }

    pub fn is_aggregate(&self) -> bool {
        self.aggregate
    }

    pub fn is_by_game(&self) -> bool {
        self.by_game
    }

    /// The query string parameters of the request.
    ///
    /// `cayenneExp` is always sent, empty without filters, because the API
    /// answers 500 when it is missing. `limit`, `factCayenneExp` and `sort`
    /// are only sent when set.
    pub fn query_pairs(&self) -> Vec<(&'static str, String)> {
        let mut pairs = vec![
            (CAYENNE_EXP_PARAM, self.cayenne_exp()),
            (IS_AGGREGATE_PARAM, self.aggregate.to_string()),
            (IS_GAME_PARAM, self.by_game.to_string()),
            (START_PARAM, self.start.to_string()),
        ];
        match self.limit {
            Some(RowLimit::Count(count)) => pairs.push((LIMIT_PARAM, count.to_string())),
            Some(RowLimit::Unlimited) => pairs.push((LIMIT_PARAM, UNLIMITED_LIMIT.to_string())),
            None => {}
        }
        if let Some(fact_exp) = self.fact_cayenne_exp() {
            pairs.push((FACT_CAYENNE_EXP_PARAM, fact_exp));
        }
        if !self.sort.is_empty() {
            let sort = serde_json::to_string(&self.sort).expect("sort keys serialize to JSON");
            pairs.push((SORT_PARAM, sort));
        }
        pairs
    }

    /// Checks the query can be sent: field names are plain identifiers (so
    /// they cannot smuggle in other syntax), numbers are finite, no `in ()`
    /// list is empty, and the limit is not zero.
    pub fn validate(&self) -> Result<(), StatsQueryError> {
        if self.limit == Some(RowLimit::Count(0)) {
            return Err(StatsQueryError::ZeroLimit);
        }
        for key in &self.sort {
            validate_field(&key.property)?;
        }
        self.filters
            .iter()
            .chain(&self.fact_filters)
            .try_for_each(validate_expr)
    }

    /// The query with `keys` as its sort order when it has none of its own.
    pub(crate) fn with_default_sort(&self, keys: &[(&str, SortDirection)]) -> Cow<'_, Self> {
        if !self.sort.is_empty() {
            return Cow::Borrowed(self);
        }
        let mut query = self.clone();
        for (field, direction) in keys {
            query = query.sort_by(*field, *direction);
        }
        Cow::Owned(query)
    }
}

fn is_identifier(name: &str) -> bool {
    let mut chars = name.chars();
    chars.next().is_some_and(|c| c.is_ascii_alphabetic())
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

fn validate_field(name: &str) -> Result<(), StatsQueryError> {
    if is_identifier(name) {
        Ok(())
    } else {
        Err(StatsQueryError::InvalidField(name.to_string()))
    }
}

/// Checks a report name (`summary`, `realtime`, ...) is a plain identifier.
pub(crate) fn validate_report(name: &str) -> Result<(), StatsQueryError> {
    if is_identifier(name) {
        Ok(())
    } else {
        Err(StatsQueryError::InvalidReport(name.to_string()))
    }
}

fn validate_expr(expr: &CayenneExpr) -> Result<(), StatsQueryError> {
    let mut result = Ok(());
    expr.visit_leaves(&mut |field, values| {
        if result.is_err() {
            return;
        }
        result = validate_field(field).and_then(|()| {
            if values.iter().all(CayenneValue::is_finite) {
                Ok(())
            } else {
                Err(StatsQueryError::NonFiniteValue {
                    field: field.to_string(),
                })
            }
        });
    });
    result?;
    match expr.empty_in_field() {
        Some(field) => Err(StatsQueryError::EmptyInList(field.to_string())),
        None => Ok(()),
    }
}

#[cfg(test)]
#[path = "stats_query_test.rs"]
mod tests;
