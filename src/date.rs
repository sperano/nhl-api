use chrono::{Datelike, NaiveDate};
use std::fmt;
use std::str::FromStr;

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

/// A season identifier (e.g., 20232024 for the 2023-2024 season)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Season {
    /// The starting year of the season
    pub start_year: u16,
}

impl Season {
    #[allow(dead_code)]
    /// Create a new season from the starting year
    pub fn new(start_year: u16) -> Self {
        Self { start_year }
    }

    #[allow(dead_code)]
    /// Create a season from start and end years (e.g., 2023, 2024)
    pub fn from_years(start_year: u16, end_year: u16) -> Self {
        debug_assert_eq!(
            end_year,
            start_year + 1,
            "End year should be start year + 1"
        );
        Self { start_year }
    }

    /// Get the end year of the season
    pub fn end_year(&self) -> u16 {
        self.start_year + 1
    }

    /// Convert to API string format (YYYYYYYY)
    #[allow(clippy::wrong_self_convention)]
    pub fn to_api_string(&self) -> String {
        format!("{}{}", self.start_year, self.end_year())
    }

    #[allow(dead_code)]
    /// Parse from API string format (YYYYYYYY)
    pub fn from_str(s: &str) -> Option<Self> {
        if s.len() != 8 {
            return None;
        }
        let start_year: u16 = s[0..4].parse().ok()?;
        let end_year: u16 = s[4..8].parse().ok()?;
        if end_year != start_year + 1 {
            return None;
        }
        Some(Self { start_year })
    }

    #[allow(dead_code)]
    /// Get the current NHL season based on the current date
    /// NHL seasons typically start in October
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_api_string())
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
    fn test_season_from_str() {
        let season = Season::from_str("20232024").unwrap();
        assert_eq!(season.start_year, 2023);
        assert_eq!(season.end_year(), 2024);

        // Invalid formats should return None
        assert!(Season::from_str("2023").is_none());
        assert!(Season::from_str("20232025").is_none()); // Not consecutive years
    }

    #[test]
    fn test_season_from_years() {
        let season = Season::from_years(2023, 2024);
        assert_eq!(season.start_year, 2023);
        assert_eq!(season.to_api_string(), "20232024");
    }

    #[test]
    fn test_season_current() {
        let season = Season::current();
        // Just verify it returns a valid season
        assert!(season.start_year >= 2024);
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
        assert_eq!(format!("{}", season), "20232024");

        let season2 = Season::new(2019);
        assert_eq!(format!("{}", season2), "20192020");
    }

    #[test]
    fn test_season_from_str_edge_cases() {
        // Empty string
        assert!(Season::from_str("").is_none());

        // Too short
        assert!(Season::from_str("2023").is_none());
        assert!(Season::from_str("202324").is_none());

        // Too long
        assert!(Season::from_str("202320240").is_none());

        // Non-numeric characters
        assert!(Season::from_str("abcd efgh").is_none());
        assert!(Season::from_str("2023abcd").is_none());

        // Years not consecutive
        assert!(Season::from_str("20232025").is_none());
        assert!(Season::from_str("20232023").is_none());
        assert!(Season::from_str("20242023").is_none());

        // Valid edge cases
        let season = Season::from_str("19992000").unwrap();
        assert_eq!(season.start_year, 1999);

        let season = Season::from_str("20502051").unwrap();
        assert_eq!(season.start_year, 2050);
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
}
