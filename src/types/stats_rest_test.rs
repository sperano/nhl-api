use super::*;

const GLOSSARY: &str = include_str!("../../tests/fixtures/glossary/all.json");
const CONFIG: &str = include_str!("../../tests/fixtures/stats_config/all.json");
const COUNTRIES: &str = include_str!("../../tests/fixtures/countries/all.json");
const SEASON_RULES: &str = include_str!("../../tests/fixtures/season_rules/all.json");
const DRAFT_ROUNDS: &str = include_str!("../../tests/fixtures/draft_rounds/all.json");

const MODERN_SEASON: i32 = 20242025;
const FIRST_SEASON: i32 = 19171918;
/// A season from before the API recorded preseason start dates.
const NO_PRESEASON_SEASON: i32 = 19531954;

fn page<T: serde::de::DeserializeOwned>(json: &str) -> StatsPage<T> {
    serde_json::from_str(json).unwrap()
}

// ===== StatsPage =====

#[test]
fn test_stats_page_reads_rows_and_total() {
    let page: StatsPage<DraftRounds> =
        page(r#"{"data":[{"id":1,"draftYear":1989,"rounds":12}],"total":64}"#);

    assert_eq!(page.total, 64);
    assert_eq!(
        page.data,
        [DraftRounds {
            id: 1,
            draft_year: 1989,
            rounds: 12
        }]
    );
}

#[test]
fn test_stats_page_empty_data() {
    let page: StatsPage<serde_json::Value> = page(r#"{"data":[],"total":0}"#);
    assert!(page.data.is_empty());
    assert_eq!(page.total, 0);
}

#[test]
fn test_stats_page_missing_total_is_an_error() {
    assert!(serde_json::from_str::<StatsPage<serde_json::Value>>(r#"{"data":[]}"#).is_err());
}

// ===== int_flag =====

#[derive(Debug, Deserialize)]
struct Flag {
    #[serde(deserialize_with = "int_flag")]
    value: bool,
}

fn flag(json: &str) -> Result<bool, serde_json::Error> {
    serde_json::from_str::<Flag>(json).map(|flag| flag.value)
}

#[test]
fn test_int_flag_reads_zero_and_one() {
    assert!(!flag(r#"{"value":0}"#).unwrap());
    assert!(flag(r#"{"value":1}"#).unwrap());
}

#[test]
fn test_int_flag_reads_json_booleans() {
    assert!(flag(r#"{"value":true}"#).unwrap());
    assert!(!flag(r#"{"value":false}"#).unwrap());
}

#[test]
fn test_int_flag_rejects_other_integers() {
    for json in [r#"{"value":2}"#, r#"{"value":-1}"#, r#"{"value":100000}"#] {
        let err = flag(json).unwrap_err().to_string();
        assert!(err.contains("0 or 1"), "{json}: {err}");
    }
}

#[test]
fn test_int_flag_rejects_strings_and_null() {
    assert!(flag(r#"{"value":"1"}"#).is_err());
    assert!(flag(r#"{"value":null}"#).is_err());
}

// ===== Glossary =====

#[test]
fn test_glossary_fixture_deserializes() {
    let page: StatsPage<GlossaryEntry> = page(GLOSSARY);

    assert_eq!(page.data.len() as u32, page.total);
    assert!(page.data.iter().all(|entry| !entry.abbreviation.is_empty()));
}

#[test]
fn test_glossary_first_season_is_optional() {
    let page: StatsPage<GlossaryEntry> = page(GLOSSARY);

    let bench = page
        .data
        .iter()
        .find(|e| e.abbreviation == "Bench")
        .unwrap();
    assert_eq!(bench.first_season_for_stat, None);
    assert_eq!(bench.language_code, "en");

    let assists = page
        .data
        .iter()
        .find(|e| e.abbreviation == "A/60 (5-on-5)")
        .unwrap();
    assert_eq!(assists.first_season_for_stat, Some(Season::new(2009)));
    assert_eq!(
        assists.to_string(),
        "A/60 (5-on-5): Assists per 60 minutes (5-on-5)"
    );
}

// ===== Countries =====

#[test]
fn test_countries_fixture_reads_flags_as_bools() {
    let page: StatsPage<Country> = page(COUNTRIES);

    let russia = page.data.iter().find(|c| c.id == "RUS").unwrap();
    assert_eq!(russia.country3_code, "RUS");
    assert_eq!(russia.country_code, "RU");
    assert_eq!(russia.nationality_name, "Russian");
    assert!(russia.has_player_stats);
    assert!(russia.is_active);
    assert_eq!(
        russia.image_url.as_deref(),
        Some("/images/country/48/RUS.png")
    );
    assert_eq!(russia.to_string(), "Russia (RUS)");
}

#[test]
fn test_countries_fixture_olympic_url_is_optional() {
    let page: StatsPage<Country> = page(COUNTRIES);

    let nigeria = page.data.iter().find(|c| c.id == "NGA").unwrap();
    assert_eq!(nigeria.olympic_url, None);
    assert_eq!(nigeria.ioc_code, "NGR");
}

#[test]
fn test_country_serializes_flags_as_bools_and_round_trips() {
    let page: StatsPage<Country> = page(COUNTRIES);
    let country = &page.data[0];

    let json = serde_json::to_value(country).unwrap();
    assert_eq!(json["hasPlayerStats"], serde_json::json!(true));
    assert_eq!(
        json["country3Code"],
        serde_json::json!(country.country3_code)
    );

    let back: Country = serde_json::from_value(json).unwrap();
    assert_eq!(&back, country);
}

// ===== Season rules =====

#[test]
fn test_season_rules_fixture_deserializes_every_season() {
    let page: StatsPage<SeasonRules> = page(SEASON_RULES);

    assert_eq!(page.data.len() as u32, page.total);
    assert!(page.data.iter().any(|s| s.id == Season::new(1917)));
    assert!(page.data.iter().any(|s| s.id == Season::new(2025)));
}

#[test]
fn test_season_rules_modern_season_flags_and_fields() {
    let page: StatsPage<SeasonRules> = page(SEASON_RULES);

    let rules = page
        .data
        .iter()
        .find(|s| s.id.id() == MODERN_SEASON)
        .unwrap();
    assert_eq!(rules.formatted_season_id, "2024-25");
    assert_eq!(rules.number_of_games, 82);
    assert!(rules.conferences_in_use);
    assert!(rules.divisions_in_use);
    assert!(rules.point_for_ot_loss_in_use);
    assert!(rules.wildcard_in_use);
    assert!(!rules.ties_in_use);
    assert!(rules.preseason_startdate.is_some());
    assert_eq!(rules.to_string(), "2024-25: 82 games per team");
}

#[test]
fn test_season_rules_early_season_has_no_preseason_and_ties() {
    let page: StatsPage<SeasonRules> = page(SEASON_RULES);

    let rules = page
        .data
        .iter()
        .find(|s| s.id.id() == NO_PRESEASON_SEASON)
        .unwrap();
    assert_eq!(rules.preseason_startdate, None);
    assert!(rules.ties_in_use);
    assert!(!rules.conferences_in_use);
    assert!(!rules.point_for_ot_loss_in_use);

    let first = page
        .data
        .iter()
        .find(|s| s.id.id() == FIRST_SEASON)
        .unwrap();
    assert_eq!(first.formatted_season_id, "1917-18");
}

#[test]
fn test_season_rules_round_trip_keeps_renamed_keys() {
    let page: StatsPage<SeasonRules> = page(SEASON_RULES);
    let rules = page
        .data
        .iter()
        .find(|s| s.id.id() == MODERN_SEASON)
        .unwrap();

    let json = serde_json::to_value(rules).unwrap();

    assert_eq!(json["pointForOTLossInUse"], serde_json::json!(true));
    assert!(json.get("preseasonStartdate").is_some());
    assert_eq!(serde_json::from_value::<SeasonRules>(json).unwrap(), *rules);
}

#[test]
fn test_season_rules_none_preseason_is_omitted_when_serialized() {
    let page: StatsPage<SeasonRules> = page(SEASON_RULES);
    let rules = page
        .data
        .iter()
        .find(|s| s.id.id() == NO_PRESEASON_SEASON)
        .unwrap();

    let json = serde_json::to_value(rules).unwrap();

    assert!(json.get("preseasonStartdate").is_none());
    assert_eq!(serde_json::from_value::<SeasonRules>(json).unwrap(), *rules);
}

#[test]
fn test_season_rules_rejects_flag_other_than_zero_or_one() {
    let page: StatsPage<serde_json::Value> = page(SEASON_RULES);
    let mut row = page.data[0].clone();
    row["wildcardInUse"] = serde_json::json!(2);

    let err = serde_json::from_value::<SeasonRules>(row).unwrap_err();

    assert!(err.to_string().contains("0 or 1"), "{err}");
}

// ===== Draft rounds =====

#[test]
fn test_draft_rounds_fixture_deserializes() {
    let page: StatsPage<DraftRounds> = page(DRAFT_ROUNDS);

    assert_eq!(page.data.len() as u32, page.total);
    let year_1989 = page.data.iter().find(|d| d.draft_year == 1989).unwrap();
    assert_eq!(year_1989.rounds, 12);
}

// ===== Stats config =====

#[test]
fn test_stats_config_fixture_lists_report_names() {
    let config: StatsConfig = serde_json::from_str(CONFIG).unwrap();

    for name in ["summary", "realtime", "bios"] {
        assert!(
            config.player_report_data.contains_key(name),
            "skater {name}"
        );
    }
    assert!(config.goalie_report_data.contains_key("summary"));
    assert!(config.team_report_data.contains_key("summary"));
    assert!(config.aggregated_columns.contains(&"seasonId".to_string()));
    assert_eq!(config.individual_columns, ["franchiseId", "franchiseName"]);
}

#[test]
fn test_stats_config_report_columns_per_granularity() {
    let config: StatsConfig = serde_json::from_str(CONFIG).unwrap();

    let summary = &config.player_report_data["summary"];
    assert!(summary
        .season
        .display_items
        .contains(&"seasonId".to_string()));
    assert!(summary.game.display_items.contains(&"gameId".to_string()));
    assert!(summary.game.result_filters.contains(&"points".to_string()));
    assert_eq!(summary.season.sort_keys, ["points", "goals", "assists"]);
}

#[test]
fn test_report_columns_accepts_empty_sort_keys() {
    let columns: ReportColumns =
        serde_json::from_str(r#"{"displayItems":["a"],"resultFilters":[],"sortKeys":[]}"#).unwrap();

    assert_eq!(columns.display_items, ["a"]);
    assert!(columns.result_filters.is_empty());
    assert!(columns.sort_keys.is_empty());
}

#[test]
fn test_stats_config_round_trips() {
    let config: StatsConfig = serde_json::from_str(CONFIG).unwrap();

    let json = serde_json::to_string(&config).unwrap();

    assert_eq!(serde_json::from_str::<StatsConfig>(&json).unwrap(), config);
}
