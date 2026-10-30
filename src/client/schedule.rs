use super::Client;
use crate::date::GameDate;
use crate::error::NHLApiError;
use crate::http_client::Endpoint;
use crate::team_abbrev::TeamAbbrev;
use crate::types::{
    DailySchedule, DailyScores, ScheduleCalendar, TeamMonthScheduleResponse, TeamScheduleResponse,
    WeeklyScheduleResponse,
};

impl Client {
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

    pub(super) fn extract_daily_schedule(
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
            next_start_date: schedule_data.next_start_date,
            previous_start_date: schedule_data.previous_start_date,
            date: date_string,
            number_of_games: games.len(),
            games,
        }
    }

    /// Gets the NHL schedule for a specific date.
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

    /// Gets daily game scores for a specific date.
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

    /// Gets weekly schedule for a specific team.
    ///
    /// # Arguments
    /// * `team_abbr` - Team abbreviation (e.g., "MTL", "TOR", "BUF"), validated as a [`TeamAbbrev`]
    /// * `date` - Optional GameDate for the week start. If None, defaults to today's date.
    pub async fn team_weekly_schedule(
        &self,
        team_abbr: impl AsRef<str>,
        date: Option<GameDate>,
    ) -> Result<TeamScheduleResponse, NHLApiError> {
        let team = TeamAbbrev::new(team_abbr.as_ref())?;
        let date = Self::resolve_date_or(date, GameDate::today());
        self.client
            .get_json(
                Endpoint::ApiWebV1,
                &format!("club-schedule/{}/week/{}", team, date.to_api_string()),
                None,
            )
            .await
    }

    /// Gets a team's schedule for one month.
    ///
    /// # Arguments
    /// * `team_abbr` - Team abbreviation (e.g., "MTL", "TOR", "BUF"), validated as a [`TeamAbbrev`]
    /// * `month` - Optional GameDate in the month (only its year and month are sent). If None,
    ///   defaults to "now".
    ///
    /// # Example
    /// ```no_run
    /// # use nhl_api::{Client, GameDate};
    /// # async fn example() -> Result<(), nhl_api::NHLApiError> {
    /// let client = Client::new()?;
    /// let month = client
    ///     .team_monthly_schedule("MTL", GameDate::from_ymd(2026, 10, 1))
    ///     .await?;
    /// println!("{}: {} games", month.current_month, month.games.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn team_monthly_schedule(
        &self,
        team_abbr: impl AsRef<str>,
        month: Option<GameDate>,
    ) -> Result<TeamMonthScheduleResponse, NHLApiError> {
        let team = TeamAbbrev::new(team_abbr.as_ref())?;
        let month = Self::resolve_date_or(month, GameDate::default());
        self.client
            .get_json(
                Endpoint::ApiWebV1,
                &format!(
                    "club-schedule/{}/month/{}",
                    team,
                    month.to_api_month_string()
                ),
                None,
            )
            .await
    }

    /// Gets the teams playing in the week starting on a date (the calendar
    /// view of the schedule).
    ///
    /// # Arguments
    /// * `date` - Optional GameDate for the week start. If None, defaults to "now".
    pub async fn schedule_calendar(
        &self,
        date: Option<GameDate>,
    ) -> Result<ScheduleCalendar, NHLApiError> {
        let date = Self::resolve_date_or(date, GameDate::default());
        self.client
            .get_json(
                Endpoint::ApiWebV1,
                &format!("schedule-calendar/{}", date.to_api_string()),
                None,
            )
            .await
    }
}
