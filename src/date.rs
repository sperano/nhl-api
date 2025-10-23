use chrono::{Datelike, NaiveDate};
use std::fmt;
use std::str::FromStr;

/// A date wrapper for NHL API calls that can be "now" or a specific date
#[derive(Debug, Clone, PartialEq)]
pub enum GameDate {
    /// Use the current date
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

    /// Parse from a string in YYYY-MM-DD format
    pub fn from_str(s: &str) -> Result<Self, chrono::ParseError> {
        s.parse()
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

impl Default for GameDate {
    fn default() -> Self {
        Self::Now
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
    /// Create a new season from the starting year
    pub fn new(start_year: u16) -> Self {
        Self { start_year }
    }

    /// Create a season from start and end years (e.g., 2023, 2024)
    pub fn from_years(start_year: u16, end_year: u16) -> Self {
        debug_assert_eq!(end_year, start_year + 1, "End year should be start year + 1");
        Self { start_year }
    }

    /// Get the end year of the season
    pub fn end_year(&self) -> u16 {
        self.start_year + 1
    }

    /// Convert to API string format (YYYYYYYY)
    pub fn to_api_string(&self) -> String {
        format!("{}{}", self.start_year, self.end_year())
    }

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
        let date = GameDate::from_str("2024-10-19").unwrap();
        assert_eq!(date.to_api_string(), "2024-10-19");

        let now = GameDate::from_str("now").unwrap();
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
}
