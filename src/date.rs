use chrono::{Datelike, NaiveDate};
use std::fmt;
use std::str::FromStr;
use thiserror::Error;

/// Smallest valid NHL season id in `YYYYYYYY` form (e.g. `10000000`).
const MIN_SEASON_ID: i64 = 10_000_000;
/// Largest valid NHL season id in `YYYYYYYY` form (e.g. `99999999`).
const MAX_SEASON_ID: i64 = 99_999_999;
/// Divisor separating the start and end year halves of a season id.
const SEASON_YEAR_DIVISOR: i64 = 10_000;

/// Errors produced when constructing or parsing a [`Season`].
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum SeasonError {
    /// The end year is neither equal to the start year nor start year + 1.
    #[error("invalid season years: {start}-{end} (end must equal start or start + 1)")]
    InvalidYears { start: u16, end: u16 },

    /// The integer is outside the valid `YYYYYYYY` range.
    #[error("invalid season integer: {0} (expected {MIN_SEASON_ID}..={MAX_SEASON_ID})")]
    InvalidInteger(i64),

    /// The string is not a recognized season format.
    #[error("invalid season format: {0:?}")]
    InvalidFormat(String),
}

/// A date wrapper for NHL API calls that can be "now" or a specific date
#[derive(Debug, Clone, PartialEq, Default)]
pub enum GameDate {
    /// Use the current date
    #[default]
    Now,
    /// Use a specific date
    Date(NaiveDate),
}

impl GameDate {
    /// Create a GameDate from today's date
    pub fn today() -> Self {
        Self::Date(chrono::Local::now().date_naive())
    }

    /// Create a GameDate from a specific date
    pub fn from_date(date: NaiveDate) -> Self {
        Self::Date(date)
    }

    /// Create a GameDate from year, month, day
    pub fn from_ymd(year: i32, month: u32, day: u32) -> Option<Self> {
        NaiveDate::from_ymd_opt(year, month, day).map(Self::Date)
    }

    /// Convert to API string format (YYYY-MM-DD or "now")
    pub fn to_api_string(&self) -> String {
        match self {
            Self::Now => "now".to_string(),
            Self::Date(date) => date.format("%Y-%m-%d").to_string(),
        }
    }

    /// Convert to a concrete date (resolves "now" to today's date)
    fn as_date(&self) -> NaiveDate {
        match self {
            Self::Now => chrono::Local::now().date_naive(),
            Self::Date(date) => *date,
        }
    }

    /// Add or subtract days from the date
    pub fn add_days(&self, days: i64) -> Self {
        Self::Date(self.as_date() + chrono::Duration::days(days))
    }
}

impl FromStr for GameDate {
    type Err = chrono::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "now" {
            Ok(Self::Now)
        } else {
            NaiveDate::parse_from_str(s, "%Y-%m-%d").map(Self::Date)
        }
    }
}

impl From<NaiveDate> for GameDate {
    fn from(date: NaiveDate) -> Self {
        Self::Date(date)
    }
}

impl fmt::Display for GameDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_api_string())
    }
}

/// An NHL season identifier (e.g., `20232024` for the 2023-2024 season).
///
/// Both the start and end years are stored explicitly rather than assuming
/// `end == start + 1`. NHL season ids are conventionally cross-year (e.g.
/// `20232024`), but single-calendar-year seasons exist — the COVID-shortened
/// 2020-21 season played entirely in 2021, and the 2004 World Cup uses
/// `20042004` — so the end year is recorded as parsed rather than derived.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Season {
    start_year: u16,
    end_year: u16,
}

impl Season {
    /// Create a new season from the starting year, using the conventional
    /// cross-year end (`start_year + 1`).
    pub fn new(start_year: u16) -> Self {
        Self {
            start_year,
            end_year: start_year.saturating_add(1),
        }
    }

    /// Create a season from explicit start and end years.
    ///
    /// The end year must equal the start year (a single-calendar-year season)
    /// or `start_year + 1` (the typical cross-year season); both are stored as
    /// given. Any other range returns [`SeasonError::InvalidYears`].
    pub fn from_years(start_year: u16, end_year: u16) -> Result<Self, SeasonError> {
        if end_year == start_year || start_year.checked_add(1) == Some(end_year) {
            Ok(Self {
                start_year,
                end_year,
            })
        } else {
            Err(SeasonError::InvalidYears {
                start: start_year,
                end: end_year,
            })
        }
    }

    /// Get the start year of the season.
    pub fn start_year(&self) -> u16 {
        self.start_year
    }

    /// Get the end year of the season.
    pub fn end_year(&self) -> u16 {
        self.end_year
    }

    /// Get the season id as an integer in `YYYYYYYY` form (e.g. `20232024`).
    pub fn id(&self) -> i32 {
        self.start_year as i32 * SEASON_YEAR_DIVISOR as i32 + self.end_year as i32
    }

    /// Convert to API string format (`YYYYYYYY`, e.g. `"20232024"`).
    #[allow(clippy::wrong_self_convention)]
    pub fn to_api_string(&self) -> String {
        format!("{:04}{:04}", self.start_year, self.end_year)
    }

    /// A short human label (`"2023-24"`), using the conventional cross-year end.
    pub fn short_label(&self) -> String {
        format!(
            "{}-{:02}",
            self.start_year,
            self.start_year.saturating_add(1) % 100
        )
    }

    /// Parse a season from either `"YYYYYYYY"` (e.g. `"20232024"`) or
    /// `"YYYY-YYYY"` (e.g. `"2023-2024"`) form. Single-year seasons such as
    /// `"20042004"` are accepted.
    pub fn parse(s: &str) -> Result<Self, SeasonError> {
        let invalid = || SeasonError::InvalidFormat(s.to_string());

        if let Some((start, end)) = s.split_once('-') {
            let start_year = start.parse::<u16>().map_err(|_| invalid())?;
            let end_year = end.parse::<u16>().map_err(|_| invalid())?;
            return Self::from_years(start_year, end_year);
        }

        if s.len() == 8 {
            let start_year = s[0..4].parse::<u16>().map_err(|_| invalid())?;
            let end_year = s[4..8].parse::<u16>().map_err(|_| invalid())?;
            return Self::from_years(start_year, end_year);
        }

        Err(invalid())
    }

    /// Get the current NHL season based on the current date.
    ///
    /// NHL seasons typically start in October, so dates before October belong
    /// to the season that started the previous calendar year.
    pub fn current() -> Self {
        let now = chrono::Local::now().date_naive();
        let year = now.year() as u16;
        // If it's before October, we're still in the previous season
        if now.month() < 10 {
            Self::new(year - 1)
        } else {
            Self::new(year)
        }
    }
}

impl fmt::Display for Season {
    /// Human-readable `"YYYY-YYYY"` form (e.g. `"2023-2024"`). For the API wire
    /// format (`"20232024"`) use [`Season::to_api_string`].
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.start_year, self.end_year)
    }
}

impl TryFrom<i64> for Season {
    type Error = SeasonError;

    /// Create a Season from an i64 season id (e.g. `20232024`), validating the
    /// `YYYYYYYY` range and the start/end year relationship.
    fn try_from(season_id: i64) -> Result<Self, Self::Error> {
        if !(MIN_SEASON_ID..=MAX_SEASON_ID).contains(&season_id) {
            return Err(SeasonError::InvalidInteger(season_id));
        }
        let start_year = (season_id / SEASON_YEAR_DIVISOR) as u16;
        let end_year = (season_id % SEASON_YEAR_DIVISOR) as u16;
        Self::from_years(start_year, end_year)
    }
}

impl TryFrom<i32> for Season {
    type Error = SeasonError;

    /// Create a Season from an i32 season id (e.g. `20232024`).
    fn try_from(season_id: i32) -> Result<Self, Self::Error> {
        Self::try_from(season_id as i64)
    }
}

impl From<u16> for Season {
    /// Create a Season from a u16 starting year (e.g., 2023).
    fn from(start_year: u16) -> Self {
        Self::new(start_year)
    }
}

impl From<Season> for i32 {
    fn from(season: Season) -> Self {
        season.id()
    }
}

impl From<Season> for i64 {
    fn from(season: Season) -> Self {
        season.id() as i64
    }
}

impl FromStr for Season {
    type Err = SeasonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl serde::Serialize for Season {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i32(self.id())
    }
}

impl<'de> serde::Deserialize<'de> for Season {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct SeasonVisitor;

        impl serde::de::Visitor<'_> for SeasonVisitor {
            type Value = Season;

            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(
                    "a season as an integer (20232024) or string (\"20232024\" or \"2023-2024\")",
                )
            }

            fn visit_i64<E>(self, value: i64) -> Result<Season, E>
            where
                E: serde::de::Error,
            {
                Season::try_from(value).map_err(E::custom)
            }

            fn visit_u64<E>(self, value: u64) -> Result<Season, E>
            where
                E: serde::de::Error,
            {
                match i64::try_from(value) {
                    Ok(v) => Season::try_from(v).map_err(E::custom),
                    Err(_) => Err(E::custom(format!("invalid season integer: {value}"))),
                }
            }

            fn visit_str<E>(self, value: &str) -> Result<Season, E>
            where
                E: serde::de::Error,
            {
                Season::parse(value).map_err(E::custom)
            }
        }

        deserializer.deserialize_any(SeasonVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_date_now() {
        let date = GameDate::Now;
        assert_eq!(date.to_api_string(), "now");
    }

    #[test]
    fn test_game_date_specific() {
        let date = GameDate::from_ymd(2024, 10, 19).unwrap();
        assert_eq!(date.to_api_string(), "2024-10-19");
    }

    #[test]
    fn test_game_date_from_str() {
        let date: GameDate = "2024-10-19".parse().unwrap();
        assert_eq!(date.to_api_string(), "2024-10-19");

        let now: GameDate = "now".parse().unwrap();
        assert_eq!(now, GameDate::Now);
    }

    #[test]
    fn test_game_date_from_str_trait() {
        // Test using the FromStr trait via .parse()
        let date: GameDate = "2024-10-19".parse().unwrap();
        assert_eq!(date.to_api_string(), "2024-10-19");

        let now: GameDate = "now".parse().unwrap();
        assert_eq!(now, GameDate::Now);
        assert_eq!(now.to_api_string(), "now");
    }

    #[test]
    fn test_game_date_today() {
        let date = GameDate::today();
        match date {
            GameDate::Date(_) => {} // Success
            GameDate::Now => panic!("today() should return a specific date"),
        }
    }

    #[test]
    fn test_season_to_string() {
        let season = Season::new(2023);
        assert_eq!(season.to_api_string(), "20232024");
        assert_eq!(season.end_year(), 2024);
    }

    #[test]
    fn test_season_from_str_valid() {
        let season = Season::from_str("20232024").unwrap();
        assert_eq!(season.start_year(), 2023);
        assert_eq!(season.end_year(), 2024);

        // Invalid formats should return Err
        assert!(Season::from_str("2023").is_err());
        assert!(Season::from_str("20232025").is_err()); // Gap year
    }

    #[test]
    fn test_season_from_str_hyphenated() {
        let season = Season::from_str("2023-2024").unwrap();
        assert_eq!(season.start_year(), 2023);
        assert_eq!(season.end_year(), 2024);
        assert_eq!(season.to_api_string(), "20232024");
    }

    #[test]
    fn test_season_from_years_valid() {
        let season = Season::from_years(2023, 2024).unwrap();
        assert_eq!(season.start_year(), 2023);
        assert_eq!(season.to_api_string(), "20232024");
    }

    #[test]
    fn test_season_from_years_rejects_gap() {
        // A two-year gap is neither a same-year nor a cross-year season.
        assert_eq!(
            Season::from_years(2023, 2025),
            Err(SeasonError::InvalidYears {
                start: 2023,
                end: 2025,
            })
        );
        // End before start is also rejected.
        assert!(Season::from_years(2024, 2023).is_err());
    }

    #[test]
    fn test_season_single_year_construction() {
        // The 2004 World Cup is encoded as 20042004 (end == start).
        let season = Season::from_years(2004, 2004).unwrap();
        assert_eq!(season.start_year(), 2004);
        assert_eq!(season.end_year(), 2004);
        assert_eq!(season.to_api_string(), "20042004");
        assert_eq!(format!("{}", season), "2004-2004");

        let parsed = Season::from_str("20042004").unwrap();
        assert_eq!(parsed, season);

        // Round-trip through the id/TryFrom path.
        assert_eq!(season.id(), 20042004);
        assert_eq!(Season::try_from(season.id()).unwrap(), season);
    }

    #[test]
    fn test_season_covid_20202021() {
        // The 2020-21 season was played entirely in 2021 but keeps a cross-year
        // id; its derived values must be identical to any normal season.
        let covid = Season::from_str("20202021").unwrap();
        assert_eq!(covid.start_year(), 2020);
        assert_eq!(covid.end_year(), 2021);
        assert_eq!(covid, Season::new(2020));
        assert_eq!(covid.to_api_string(), "20202021");
        assert_eq!(format!("{}", covid), "2020-2021");
        assert_eq!(covid.short_label(), "2020-21");
        assert_eq!(covid.id(), 20202021);
    }

    #[test]
    fn test_season_id_and_conversions() {
        let season = Season::new(2023);
        assert_eq!(season.id(), 20232024);
        assert_eq!(i32::from(season), 20232024);
        assert_eq!(i64::from(season), 20232024_i64);
    }

    #[test]
    fn test_season_short_label() {
        assert_eq!(Season::new(2023).short_label(), "2023-24");
        assert_eq!(Season::new(1999).short_label(), "1999-00");
        assert_eq!(Season::new(2009).short_label(), "2009-10");
    }

    #[test]
    fn test_season_display_vs_api_string() {
        let season = Season::new(2023);
        assert_eq!(format!("{}", season), "2023-2024");
        assert_eq!(season.to_api_string(), "20232024");
    }

    #[test]
    fn test_season_current() {
        let season = Season::current();
        // Just verify it returns a plausible recent season without panicking.
        assert!(season.start_year() >= 2024);
    }

    #[test]
    fn test_game_date_from_date() {
        let naive_date = NaiveDate::from_ymd_opt(2024, 3, 15).unwrap();
        let game_date = GameDate::from_date(naive_date);
        assert_eq!(game_date.to_api_string(), "2024-03-15");
    }

    #[test]
    fn test_game_date_default() {
        let default_date = GameDate::default();
        assert_eq!(default_date, GameDate::Now);
        assert_eq!(default_date.to_api_string(), "now");
    }

    #[test]
    fn test_game_date_from_naive_date() {
        let naive_date = NaiveDate::from_ymd_opt(2024, 12, 25).unwrap();
        let game_date: GameDate = naive_date.into();
        assert_eq!(game_date.to_api_string(), "2024-12-25");
    }

    #[test]
    fn test_game_date_display() {
        let now = GameDate::Now;
        assert_eq!(format!("{}", now), "now");

        // Use different month and day values to verify correct ordering (YYYY-MM-DD)
        let date = GameDate::from_ymd(2024, 3, 15).unwrap();
        assert_eq!(format!("{}", date), "2024-03-15");
    }

    #[test]
    fn test_game_date_from_ymd_invalid() {
        // Invalid month
        assert!(GameDate::from_ymd(2024, 13, 1).is_none());

        // Invalid day
        assert!(GameDate::from_ymd(2024, 2, 30).is_none());

        // Invalid day for month
        assert!(GameDate::from_ymd(2024, 4, 31).is_none());

        // Day 0
        assert!(GameDate::from_ymd(2024, 1, 0).is_none());

        // Month 0
        assert!(GameDate::from_ymd(2024, 0, 1).is_none());
    }

    #[test]
    fn test_game_date_from_str_invalid() {
        // Invalid format
        assert!("2024/10/19".parse::<GameDate>().is_err());
        assert!("10-19-2024".parse::<GameDate>().is_err());
        assert!("2024-10".parse::<GameDate>().is_err());
        assert!("".parse::<GameDate>().is_err());
        assert!("not-a-date".parse::<GameDate>().is_err());

        // Invalid date values
        assert!("2024-13-01".parse::<GameDate>().is_err());
        assert!("2024-02-30".parse::<GameDate>().is_err());
    }

    #[test]
    fn test_game_date_equality() {
        let date1 = GameDate::from_ymd(2024, 10, 19).unwrap();
        let date2 = GameDate::from_ymd(2024, 10, 19).unwrap();
        let date3 = GameDate::from_ymd(2024, 10, 20).unwrap();

        assert_eq!(date1, date2);
        assert_ne!(date1, date3);
        assert_ne!(date1, GameDate::Now);

        let now1 = GameDate::Now;
        let now2 = GameDate::Now;
        assert_eq!(now1, now2);
    }

    #[test]
    fn test_season_display() {
        let season = Season::new(2023);
        assert_eq!(format!("{}", season), "2023-2024");

        let season2 = Season::new(2019);
        assert_eq!(format!("{}", season2), "2019-2020");
    }

    #[test]
    fn test_season_from_str_edge_cases() {
        // Empty string
        assert!(Season::from_str("").is_err());

        // Too short
        assert!(Season::from_str("2023").is_err());
        assert!(Season::from_str("202324").is_err());

        // Too long
        assert!(Season::from_str("202320240").is_err());

        // Non-numeric characters
        assert!(Season::from_str("abcd efgh").is_err());
        assert!(Season::from_str("2023abcd").is_err());

        // Gap years
        assert!(Season::from_str("20232025").is_err());
        assert!(Season::from_str("20242023").is_err());

        // Single-year season is now valid (end == start)
        let single = Season::from_str("20232023").unwrap();
        assert_eq!(single.start_year(), 2023);
        assert_eq!(single.end_year(), 2023);

        // Valid edge cases
        let season = Season::from_str("19992000").unwrap();
        assert_eq!(season.start_year(), 1999);

        let season = Season::from_str("20502051").unwrap();
        assert_eq!(season.start_year(), 2050);
    }

    #[test]
    fn test_add_days_with_specific_date() {
        let date = GameDate::from_ymd(2024, 10, 19).unwrap();

        // Add positive days
        let future = date.add_days(5);
        assert_eq!(future.to_api_string(), "2024-10-24");

        // Add negative days (subtract)
        let past = date.add_days(-5);
        assert_eq!(past.to_api_string(), "2024-10-14");

        // Add zero days
        let same = date.add_days(0);
        assert_eq!(same.to_api_string(), "2024-10-19");
    }

    #[test]
    fn test_add_days_across_month_boundary() {
        let date = GameDate::from_ymd(2024, 10, 30).unwrap();

        // Cross into next month
        let next_month = date.add_days(5);
        assert_eq!(next_month.to_api_string(), "2024-11-04");

        // Cross into previous month
        let date2 = GameDate::from_ymd(2024, 11, 3).unwrap();
        let prev_month = date2.add_days(-5);
        assert_eq!(prev_month.to_api_string(), "2024-10-29");
    }

    #[test]
    fn test_add_days_across_year_boundary() {
        let date = GameDate::from_ymd(2024, 12, 30).unwrap();

        // Cross into next year
        let next_year = date.add_days(5);
        assert_eq!(next_year.to_api_string(), "2025-01-04");

        // Cross into previous year
        let date2 = GameDate::from_ymd(2025, 1, 3).unwrap();
        let prev_year = date2.add_days(-5);
        assert_eq!(prev_year.to_api_string(), "2024-12-29");
    }

    #[test]
    fn test_add_days_leap_year() {
        // 2024 is a leap year
        let date = GameDate::from_ymd(2024, 2, 28).unwrap();

        // February 29 exists in 2024
        let leap_day = date.add_days(1);
        assert_eq!(leap_day.to_api_string(), "2024-02-29");

        let march_1 = date.add_days(2);
        assert_eq!(march_1.to_api_string(), "2024-03-01");

        // 2023 is not a leap year
        let date_2023 = GameDate::from_ymd(2023, 2, 28).unwrap();
        let march_1_2023 = date_2023.add_days(1);
        assert_eq!(march_1_2023.to_api_string(), "2023-03-01");
    }

    #[test]
    fn test_add_days_with_now() {
        // Note: This test is time-dependent but should work
        // We're just verifying that "now" gets converted to a date
        let now = GameDate::Now;
        let future = now.add_days(7);

        // Should return a Date variant, not Now
        match future {
            GameDate::Date(_) => {} // Success
            GameDate::Now => panic!("add_days(7) on Now should return a specific date"),
        }

        // Verify the format is a date string
        let future_str = future.to_api_string();
        assert_ne!(future_str, "now");
        assert!(future_str.contains("-")); // Should be YYYY-MM-DD format
    }

    #[test]
    fn test_add_days_large_values() {
        let date = GameDate::from_ymd(2024, 1, 1).unwrap();

        // Add a full year (2024 is a leap year with 366 days)
        let next_year = date.add_days(366);
        assert_eq!(next_year.to_api_string(), "2025-01-01");

        // Subtract a full year (365 days takes us back to Jan 1, 2023)
        let prev_year = date.add_days(-365);
        assert_eq!(prev_year.to_api_string(), "2023-01-01");

        // Add multiple years (365 * 5 = 1825 days from 2024-01-01)
        // 2024 has 366 days (leap year), others have 365
        // Result is 2028-12-30
        let far_future = date.add_days(365 * 5);
        assert_eq!(far_future.to_api_string(), "2028-12-30");
    }

    #[test]
    fn test_add_days_chaining() {
        let date = GameDate::from_ymd(2024, 10, 15).unwrap();

        // Chain multiple add_days calls
        let result = date.add_days(5).add_days(3).add_days(-2);
        assert_eq!(result.to_api_string(), "2024-10-21");

        // Should be equivalent to adding all at once
        let direct = date.add_days(6);
        assert_eq!(result.to_api_string(), direct.to_api_string());
    }

    #[test]
    fn test_season_try_from_i32() {
        let season = Season::try_from(20232024_i32).unwrap();
        assert_eq!(season.start_year(), 2023);
        assert_eq!(season.end_year(), 2024);

        let early = Season::try_from(19171918_i32).unwrap();
        assert_eq!(early.start_year(), 1917);

        let future = Season::try_from(20502051_i32).unwrap();
        assert_eq!(future.start_year(), 2050);
    }

    #[test]
    fn test_season_try_from_i64() {
        let season = Season::try_from(20232024_i64).unwrap();
        assert_eq!(season.start_year(), 2023);
        assert_eq!(season.end_year(), 2024);

        // i32 and i64 paths agree.
        assert_eq!(
            Season::try_from(20232024_i32).unwrap(),
            Season::try_from(20232024_i64).unwrap()
        );
    }

    #[test]
    fn test_season_try_from_range_rejection() {
        // Below the YYYYYYYY range.
        assert_eq!(
            Season::try_from(2023_i32),
            Err(SeasonError::InvalidInteger(2023))
        );
        // Above the range.
        assert!(Season::try_from(202320240_i64).is_err());
        // Negative.
        assert!(Season::try_from(-20232024_i64).is_err());
        // In range but a gap year routes through from_years and fails.
        assert!(Season::try_from(20232025_i32).is_err());
    }

    #[test]
    fn test_season_serde_integer() {
        let season = Season::new(2023);
        let json = serde_json::to_string(&season).unwrap();
        assert_eq!(json, "20232024");

        let de: Season = serde_json::from_str("20232024").unwrap();
        assert_eq!(de, season);
    }

    #[test]
    fn test_season_serde_string_api_form() {
        let de: Season = serde_json::from_str("\"20232024\"").unwrap();
        assert_eq!(de, Season::new(2023));
    }

    #[test]
    fn test_season_serde_string_hyphenated_form() {
        let de: Season = serde_json::from_str("\"2023-2024\"").unwrap();
        assert_eq!(de, Season::new(2023));
    }

    #[test]
    fn test_season_serde_round_trip() {
        let season = Season::from_str("20242025").unwrap();
        let json = serde_json::to_string(&season).unwrap();
        let de: Season = serde_json::from_str(&json).unwrap();
        assert_eq!(de, season);
    }

    #[test]
    fn test_season_serde_rejects_invalid() {
        // Out-of-range integer.
        assert!(serde_json::from_str::<Season>("2023").is_err());
        // Gap-year string.
        assert!(serde_json::from_str::<Season>("\"20232025\"").is_err());
        // Non-numeric string.
        assert!(serde_json::from_str::<Season>("\"nope\"").is_err());
    }

    #[test]
    fn test_season_from_u16() {
        // Standard starting year
        let season: Season = 2023_u16.into();
        assert_eq!(season.start_year(), 2023);
        assert_eq!(season.end_year(), 2024);
        assert_eq!(season.to_api_string(), "20232024");

        // Earlier year
        let season: Season = 1999_u16.into();
        assert_eq!(season.start_year(), 1999);

        // Verify it's equivalent to Season::new
        let from_new = Season::new(2023);
        let from_u16: Season = 2023_u16.into();
        assert_eq!(from_new, from_u16);
    }

    #[test]
    fn test_season_hash() {
        use std::collections::HashSet;

        let season1 = Season::new(2023);
        let season2 = Season::new(2023);
        let season3 = Season::new(2024);

        let mut set = HashSet::new();
        set.insert(season1);
        set.insert(season2); // Should not add duplicate
        set.insert(season3);

        assert_eq!(set.len(), 2);
        assert!(set.contains(&Season::new(2023)));
        assert!(set.contains(&Season::new(2024)));
        assert!(!set.contains(&Season::new(2022)));
    }

    #[test]
    fn test_season_copy() {
        let season1 = Season::new(2023);
        let season2 = season1; // Copy, not move

        // Both should still be usable
        assert_eq!(season1.start_year(), 2023);
        assert_eq!(season2.start_year(), 2023);
        assert_eq!(season1, season2);
    }

    #[test]
    fn test_season_eq() {
        let season1 = Season::new(2023);
        let season2 = Season::new(2023);
        let season3 = Season::new(2024);

        // Test Eq (reflexive, symmetric, transitive)
        assert_eq!(season1, season1); // Reflexive
        assert_eq!(season1, season2); // Symmetric
        assert_eq!(season2, season1);
        assert_ne!(season1, season3);
        assert_ne!(season3, season1);
    }

    #[test]
    fn test_season_parse_non_numeric() {
        // Non-numeric in first half
        assert!(Season::parse("abcd2024").is_err());

        // Non-numeric in second half
        assert!(Season::parse("2023abcd").is_err());

        // Hyphenated but non-numeric halves
        assert!(Season::parse("2023/024").is_err());

        // Spaces
        assert!(Season::parse("2023 024").is_err());
        assert!(Season::parse(" 2032024").is_err());
    }

    #[test]
    fn test_game_date_clone() {
        let date = GameDate::from_ymd(2024, 10, 19).unwrap();
        let cloned = date.clone();

        assert_eq!(date, cloned);
        assert_eq!(date.to_api_string(), cloned.to_api_string());

        let now = GameDate::Now;
        let now_cloned = now.clone();
        assert_eq!(now, now_cloned);
    }

    #[test]
    fn test_game_date_debug() {
        let date = GameDate::from_ymd(2024, 10, 19).unwrap();
        let debug_str = format!("{:?}", date);
        assert!(debug_str.contains("Date"));
        assert!(debug_str.contains("2024"));

        let now = GameDate::Now;
        let debug_str = format!("{:?}", now);
        assert_eq!(debug_str, "Now");
    }

    #[test]
    fn test_season_debug() {
        let season = Season::new(2023);
        let debug_str = format!("{:?}", season);
        assert!(debug_str.contains("Season"));
        assert!(debug_str.contains("2023"));
    }

    #[test]
    #[allow(clippy::clone_on_copy)]
    fn test_season_clone() {
        let season = Season::new(2023);
        let cloned = season.clone();

        assert_eq!(season, cloned);
        assert_eq!(season.start_year(), cloned.start_year());
    }

    #[test]
    fn test_season_parse_boundary_years() {
        // Year 0000-0001 (valid cross-year at the low boundary)
        assert!(Season::parse("00000001").is_ok());

        // 99999999 is now a valid single-calendar-year season (end == start).
        let single = Season::parse("99999999").unwrap();
        assert_eq!(single.start_year(), 9999);
        assert_eq!(single.end_year(), 9999);

        // Valid high cross-year season.
        let season = Season::parse("99989999").unwrap();
        assert_eq!(season.start_year(), 9998);
    }

    #[test]
    fn test_game_date_as_date_now_variant() {
        // This tests the as_date method indirectly through add_days
        // when starting from Now
        let now = GameDate::Now;
        let tomorrow = now.add_days(1);
        let yesterday_from_tomorrow = tomorrow.add_days(-1);

        // The dates should match (both resolve to "today")
        let today = GameDate::today();
        assert_eq!(
            yesterday_from_tomorrow.to_api_string(),
            today.to_api_string()
        );
    }

    #[test]
    fn test_season_from_years_equivalence() {
        // Verify from_years produces same result as new
        let from_years = Season::from_years(2023, 2024).unwrap();
        let from_new = Season::new(2023);

        assert_eq!(from_years, from_new);
        assert_eq!(from_years.start_year(), from_new.start_year());
        assert_eq!(from_years.end_year(), from_new.end_year());
    }
}
