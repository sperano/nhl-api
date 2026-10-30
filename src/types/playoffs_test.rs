use super::*;
use crate::types::PeriodType;

/// Completed postseasons, recorded by `capture_fixtures`.
const BRACKET_2026: &str = include_str!("../../tests/fixtures/playoff_bracket/20252026.json");
const BRACKET_2020: &str =
    include_str!("../../tests/fixtures/playoff_bracket/20192020_qualifiers.json");
const BRACKET_NO_PLAYOFFS: &str =
    include_str!("../../tests/fixtures/playoff_bracket/20042005_no_playoffs.json");
const BRACKET_NOT_STARTED: &str =
    include_str!("../../tests/fixtures/playoff_bracket/20262027_not_started.json");
const CAROUSEL_1918: &str =
    include_str!("../../tests/fixtures/playoff_series_carousel/19171918.json");
const SERIES_1918_FINAL: &str =
    include_str!("../../tests/fixtures/playoff_series_schedule/19171918_i_pcha_final.json");
const SERIES_1926: &str =
    include_str!("../../tests/fixtures/playoff_series_schedule/19251926_a.json");
const SERIES_2026_FINAL: &str =
    include_str!("../../tests/fixtures/playoff_series_schedule/20252026_o_final.json");

/// Postseasons in progress, archived from the live API (web.archive.org)
/// since they cannot be re-recorded: the 2024 bracket on 2024-04-25, the
/// 2025-26 carousel on 2026-05-01 and series 2023-24 A on 2024-04-22.
const BRACKET_IN_PROGRESS: &str =
    include_str!("../../tests/fixtures/playoff_bracket_in_progress.json");
const CAROUSEL_IN_PROGRESS: &str =
    include_str!("../../tests/fixtures/playoff_series_carousel_in_progress.json");
const SERIES_IN_PROGRESS: &str =
    include_str!("../../tests/fixtures/playoff_series_schedule_in_progress.json");

const HURRICANES: i64 = 12;
const GOLDEN_KNIGHTS: i64 = 54;

fn bracket(json: &str) -> PlayoffBracket {
    serde_json::from_str(json).unwrap()
}

fn series<'a>(bracket: &'a PlayoffBracket, letter: &str) -> &'a BracketSeries {
    bracket
        .series
        .iter()
        .find(|series| series.series_letter == letter)
        .unwrap()
}

#[test]
fn test_playoff_bracket_completed_final() {
    let bracket = bracket(BRACKET_2026);
    assert_eq!(bracket.series.len(), 15);

    let final_series = series(&bracket, "O");
    assert_eq!(final_series.series_abbrev, "SCF");
    assert_eq!(final_series.winning_team_id, Some(TeamId::new(HURRICANES)));
    assert_eq!(
        final_series.losing_team_id,
        Some(TeamId::new(GOLDEN_KNIGHTS))
    );
    assert_eq!(
        (final_series.top_seed_wins, final_series.bottom_seed_wins),
        (4, 2)
    );
    assert_eq!(final_series.top_seed_team.as_ref().unwrap().abbrev, "CAR");
    assert_eq!(final_series.conference_abbrev, None);
    assert_eq!(
        series(&bracket, "M").conference_abbrev.as_deref(),
        Some("E")
    );
}

#[test]
fn test_playoff_bracket_2020_qualifying_round_is_round_zero() {
    let bracket = bracket(BRACKET_2020);
    let qualifier = series(&bracket, "S");
    assert_eq!(qualifier.playoff_round, 0);
    assert_eq!(qualifier.series_abbrev, "SCQ");
}

#[test]
fn test_playoff_bracket_without_playoffs_is_empty() {
    assert_eq!(bracket(BRACKET_NO_PLAYOFFS), PlayoffBracket::default());

    let not_started = bracket(BRACKET_NOT_STARTED);
    assert!(not_started.series.is_empty());
    assert!(not_started.bracket_logo.is_some());
    assert_eq!(not_started.bracket_title, Some(LocalizedString::default()));
}

#[test]
fn test_playoff_bracket_in_progress_undecided_series() {
    let bracket = bracket(BRACKET_IN_PROGRESS);

    // A first-round series being played: teams known, no winner (sent as 0).
    let first_round = series(&bracket, "A");
    let panthers = first_round.top_seed_team.as_ref().unwrap();
    assert_eq!(panthers.abbrev, "FLA");
    // Served before `commonName`, `darkLogo` and co. were added.
    assert_eq!(panthers.common_name, LocalizedString::default());
    assert_eq!(panthers.dark_logo, None);
    assert!(first_round.series_url.is_some());
    assert_eq!(first_round.winning_team_id, None);
    assert_eq!(first_round.losing_team_id, None);

    // A second-round series: no teams, URL or rank abbreviations yet.
    let second_round = series(&bracket, "I");
    assert_eq!(second_round.playoff_round, 2);
    assert_eq!(second_round.top_seed_team, None);
    assert_eq!(second_round.bottom_seed_team, None);
    assert_eq!(second_round.series_url, None);
    assert_eq!(second_round.top_seed_rank_abbrev, None);
    assert_eq!(second_round.winning_team_id, None);
}

#[test]
fn test_decided_team_id_reads_zero_and_missing_as_none() {
    let read = |json: &str| -> Option<TeamId> {
        let series: CarouselSeries = serde_json::from_str(&format!(
            r#"{{"seriesLetter": "A", "roundNumber": 1, "seriesLabel": "1st-round",
                "seriesLink": "/x", "neededToWin": 4,
                "topSeed": {{"id": 1, "abbrev": "NJD", "wins": 0, "logo": "", "darkLogo": ""}},
                "bottomSeed": {{"id": 2, "abbrev": "NYI", "wins": 0, "logo": "", "darkLogo": ""}}
                {json}}}"#
        ))
        .unwrap();
        series.winning_team_id
    };
    assert_eq!(read(""), None);
    assert_eq!(read(r#", "winningTeamId": 0"#), None);
    assert_eq!(read(r#", "winningTeamId": null"#), None);
    assert_eq!(read(r#", "winningTeamId": 1"#), Some(TeamId::new(1)));
}

#[test]
fn test_playoff_series_carousel_total_goals_series() {
    let carousel: PlayoffSeriesCarousel = serde_json::from_str(CAROUSEL_1918).unwrap();
    assert_eq!(carousel.season_id, Season::new(1917));
    assert_eq!(carousel.rounds.len(), 2);

    let nhl_final = &carousel.rounds[0].series[0];
    assert_eq!(carousel.rounds[0].round_abbrev, "NHLF");
    assert_eq!(nhl_final.needed_to_win, 0);
    assert_eq!(nhl_final.top_seed.abbrev, "TAN");
    assert!(nhl_final.winning_team_id.is_some());
}

#[test]
fn test_playoff_series_carousel_in_progress_tbd_seed() {
    let carousel: PlayoffSeriesCarousel = serde_json::from_str(CAROUSEL_IN_PROGRESS).unwrap();
    assert_eq!(carousel.current_round, 2);

    let all_series = || carousel.rounds.iter().flat_map(|round| &round.series);
    let undecided = all_series().find(|s| s.series_letter == "L").unwrap();
    assert!(undecided.top_seed.is_tbd());
    assert_eq!(undecided.top_seed.abbrev, "TBD");
    assert!(!undecided.bottom_seed.is_tbd());
    assert_eq!(undecided.winning_team_id, None);

    let decided = all_series().find(|s| s.series_letter == "C").unwrap();
    assert_eq!(decided.winning_team_id, Some(TeamId::new(HURRICANES)));
}

#[test]
fn test_playoff_series_schedule_completed_final() {
    let schedule: PlayoffSeriesSchedule = serde_json::from_str(SERIES_2026_FINAL).unwrap();
    assert_eq!(schedule.series_letter, "O");
    assert_eq!(schedule.length, 7);
    assert_eq!(schedule.top_seed_team.series_wins, 4);
    assert_eq!(
        schedule.top_seed_team.conference.as_ref().unwrap().abbrev,
        "E"
    );
    assert_eq!(schedule.games.len(), 6);

    let last = schedule.games.last().unwrap();
    assert_eq!(last.game_state, GameState::Off);
    assert_eq!(last.game_type, GameType::Playoffs);
    assert_eq!(
        last.series_status,
        Some(SeriesGameStatus {
            top_seed_wins: 4,
            bottom_seed_wins: 2
        })
    );
    assert!(last.home_team.score.is_some());
}

#[test]
fn test_playoff_series_schedule_against_pcha_team() {
    let schedule: PlayoffSeriesSchedule = serde_json::from_str(SERIES_1918_FINAL).unwrap();
    assert_eq!(schedule.bottom_seed_team.abbrev, "VMI");
    assert_eq!(schedule.top_seed_team.conference, None);
    assert_eq!(schedule.top_seed_team.division_abbrev, None);

    let game = &schedule.games[0];
    assert_eq!(game.away_team.abbrev, "VMI");
    assert_eq!(game.away_team.place_name, None);
    assert!(game.home_team.place_name.is_some());
}

#[test]
fn test_playoff_series_schedule_with_tied_game() {
    let schedule: PlayoffSeriesSchedule = serde_json::from_str(SERIES_1926).unwrap();
    assert_eq!(schedule.needed_to_win, 0);
    assert_eq!(schedule.top_seed_team.record, "0-1-1");
    let outcome = schedule.games[1].game_outcome.as_ref().unwrap();
    assert_eq!(outcome.last_period_type, Some(PeriodType::Regulation));
}

#[test]
fn test_playoff_series_schedule_in_progress_unplayed_games() {
    let schedule: PlayoffSeriesSchedule = serde_json::from_str(SERIES_IN_PROGRESS).unwrap();
    assert_eq!(schedule.games.len(), 7);

    let played = &schedule.games[0];
    assert_eq!(played.id, GameId::new(2023030111));
    assert!(played.period_descriptor.is_some());
    assert_eq!(played.series_status.as_ref().unwrap().top_seed_wins, 1);

    let upcoming = &schedule.games[1];
    assert_eq!(upcoming.game_state, GameState::Future);
    assert_eq!(upcoming.home_team.score, None);
    assert_eq!(upcoming.period_descriptor, None);
    assert_eq!(upcoming.game_outcome, None);
    assert_eq!(upcoming.series_status, None);

    let if_necessary = &schedule.games[6];
    assert!(if_necessary.if_necessary);
    assert_eq!(if_necessary.game_schedule_state, GameScheduleState::Tbd);
}

#[test]
fn test_playoff_types_serde_round_trip() {
    let bracket = bracket(BRACKET_IN_PROGRESS);
    let json = serde_json::to_string(&bracket).unwrap();
    assert!(!json.contains("winningTeamId"), "undecided ids omitted");
    assert_eq!(
        serde_json::from_str::<PlayoffBracket>(&json).unwrap(),
        bracket
    );

    let carousel: PlayoffSeriesCarousel = serde_json::from_str(CAROUSEL_IN_PROGRESS).unwrap();
    let json = serde_json::to_string(&carousel).unwrap();
    assert_eq!(
        serde_json::from_str::<PlayoffSeriesCarousel>(&json).unwrap(),
        carousel
    );

    let schedule: PlayoffSeriesSchedule = serde_json::from_str(SERIES_IN_PROGRESS).unwrap();
    let json = serde_json::to_string(&schedule).unwrap();
    assert_eq!(
        serde_json::from_str::<PlayoffSeriesSchedule>(&json).unwrap(),
        schedule
    );
}
