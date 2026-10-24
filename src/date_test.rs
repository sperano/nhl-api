use super::*;

fn date(year: i32, month: u32, day: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(year, month, day).unwrap()
}

#[test]
fn test_season_parse_non_ascii_does_not_panic() {
    // 8 bytes, but byte 4 falls inside the two-byte 'é'.
    assert_eq!(
        Season::parse("abcé123"),
        Err(SeasonError::InvalidFormat("abcé123".to_string()))
    );
    assert!(Season::parse("2023é24").is_err());
    assert!(Season::parse("2023-202é").is_err());
}

#[test]
fn test_season_serde_non_ascii_string_is_error() {
    assert!(serde_json::from_str::<Season>("\"abcé123\"").is_err());
}

#[test]
fn test_season_parse_requires_four_digit_years() {
    for input in [
        "+202+203",
        "+2023-+2024",
        "02023-2024",
        "2023-02024",
        "999-1000",
    ] {
        assert!(
            Season::parse(input).is_err(),
            "{input:?} should be rejected"
        );
    }
}

#[test]
fn test_season_parse_out_of_range_years() {
    assert_eq!(
        Season::parse("65535-65535"),
        Err(SeasonError::InvalidFormat("65535-65535".to_string()))
    );
    assert_eq!(
        Season::parse("09991000"),
        Err(SeasonError::YearOutOfRange {
            start: 999,
            end: 1000,
        })
    );
}

#[test]
fn test_season_from_years_rejects_out_of_range() {
    assert_eq!(
        Season::from_years(65535, 65535),
        Err(SeasonError::YearOutOfRange {
            start: 65535,
            end: 65535,
        })
    );
    assert_eq!(
        Season::from_years(9999, 10000),
        Err(SeasonError::YearOutOfRange {
            start: 9999,
            end: 10000,
        })
    );
    assert!(Season::from_years(999, 1000).is_err());
    assert!(Season::from_years(0, 0).is_err());
}

#[test]
fn test_season_from_years_bounds_round_trip() {
    for (start, end) in [(1000, 1000), (1000, 1001), (9998, 9999), (9999, 9999)] {
        let season = Season::from_years(start, end).unwrap();
        assert_eq!(season.to_api_string().len(), 2 * SEASON_YEAR_DIGITS);
        assert_eq!(Season::try_from(season.id()).unwrap(), season);
        let json = serde_json::to_string(&season).unwrap();
        assert_eq!(serde_json::from_str::<Season>(&json).unwrap(), season);
    }
}

#[test]
fn test_season_for_date_before_october_is_previous_season() {
    assert_eq!(Season::for_date(date(2024, 9, 30)), Season::new(2023));
    assert_eq!(Season::for_date(date(2024, 1, 1)), Season::new(2023));
}

#[test]
fn test_season_for_date_from_october_is_new_season() {
    assert_eq!(Season::for_date(date(2024, 10, 1)), Season::new(2024));
    assert_eq!(Season::for_date(date(2024, 12, 31)), Season::new(2024));
}

#[test]
fn test_season_try_from_end_half_below_years_is_invalid_integer() {
    // Inside 10001000..=99999999, but the end half (0500) is not a valid year.
    assert_eq!(
        Season::try_from(99_990_500_i64),
        Err(SeasonError::InvalidInteger(99_990_500))
    );
    assert_eq!(
        Season::try_from(10_500_000_i64),
        Err(SeasonError::InvalidInteger(10_500_000))
    );
}

#[test]
fn test_season_try_from_gap_year_is_invalid_years() {
    assert_eq!(
        Season::try_from(20_232_025_i64),
        Err(SeasonError::InvalidYears {
            start: 2023,
            end: 2025
        })
    );
}
