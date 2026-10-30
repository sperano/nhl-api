use super::Client;
use crate::date::Season;
use crate::error::NHLApiError;
use crate::http_client::Endpoint;
use crate::types::{PlayoffBracket, PlayoffSeriesCarousel, PlayoffSeriesSchedule};

impl Client {
    /// Gets the full playoff bracket of a season.
    ///
    /// The endpoint is keyed by the year the playoffs end in
    /// (`/playoff-bracket/{year}`), so `Season::new(2024)` requests 2025. A
    /// season without playoffs (2004-05) or whose playoffs have not started
    /// returns a bracket with no series.
    ///
    /// # Example
    /// ```no_run
    /// # use nhl_api::{Client, Season};
    /// # async fn example() -> Result<(), nhl_api::NHLApiError> {
    /// let client = Client::new()?;
    /// let bracket = client.playoff_bracket(Season::new(2024)).await?;
    /// for series in &bracket.series {
    ///     println!("{} {}", series.series_letter, series.series_title);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn playoff_bracket(&self, season: Season) -> Result<PlayoffBracket, NHLApiError> {
        self.client
            .get_json(
                Endpoint::ApiWebV1,
                &format!("playoff-bracket/{}", season.end_year()),
                None,
            )
            .await
    }

    /// Gets every playoff series of a season up to the current round, grouped
    /// by round.
    ///
    /// A season without playoffs, or whose playoffs have not started, is a
    /// [`NHLApiError::ResourceNotFound`].
    pub async fn playoff_series_carousel(
        &self,
        season: Season,
    ) -> Result<PlayoffSeriesCarousel, NHLApiError> {
        self.client
            .get_json(
                Endpoint::ApiWebV1,
                &format!("playoff-series/carousel/{}", season.to_api_string()),
                None,
            )
            .await
    }

    /// Gets the games of one playoff series.
    ///
    /// # Arguments
    /// * `season` - The season of the playoffs
    /// * `series_letter` - The series' letter, as in
    ///   [`BracketSeries::series_letter`](crate::BracketSeries::series_letter)
    ///   (either case). Anything but an ASCII letter is a
    ///   [`NHLApiError::InvalidSeriesLetter`], with no request sent; a letter
    ///   the season has no series for is a [`NHLApiError::ResourceNotFound`].
    ///
    /// # Example
    /// ```no_run
    /// # use nhl_api::{Client, Season};
    /// # async fn example() -> Result<(), nhl_api::NHLApiError> {
    /// let client = Client::new()?;
    /// let series = client.playoff_series_schedule(Season::new(2024), 'a').await?;
    /// println!("{} games", series.games.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn playoff_series_schedule(
        &self,
        season: Season,
        series_letter: char,
    ) -> Result<PlayoffSeriesSchedule, NHLApiError> {
        if !series_letter.is_ascii_alphabetic() {
            return Err(NHLApiError::InvalidSeriesLetter(series_letter));
        }
        self.client
            .get_json(
                Endpoint::ApiWebV1,
                &format!(
                    "schedule/playoff-series/{}/{}",
                    season.to_api_string(),
                    series_letter.to_ascii_lowercase()
                ),
                None,
            )
            .await
    }
}
