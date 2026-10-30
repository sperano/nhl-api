use super::*;

/// Builder for creating test ScheduleTeam instances
struct TeamBuilder {
    id: i64,
    abbrev: String,
    place_name: Option<LocalizedString>,
    logo: String,
    score: Option<i32>,
}

impl TeamBuilder {
    fn new(abbrev: &str) -> Self {
        Self {
            id: 1,
            abbrev: abbrev.to_string(),
            place_name: None,
            logo: format!("https://assets.nhle.com/logos/nhl/svg/{}_light.svg", abbrev),
            score: None,
        }
    }

    fn id(mut self, id: i64) -> Self {
        self.id = id;
        self
    }

    #[allow(dead_code)]
    fn score(mut self, score: i32) -> Self {
        self.score = Some(score);
        self
    }

    fn build(self) -> ScheduleTeam {
        ScheduleTeam {
            id: TeamId::new(self.id),
            abbrev: self.abbrev,
            place_name: self.place_name,
            logo: self.logo,
            score: self.score,
        }
    }
}

/// Builder for creating test ScheduleGame instances
struct ScheduleGameBuilder {
    id: i64,
    game_type: GameType,
    game_date: Option<String>,
    start_time_utc: String,
    away_team: ScheduleTeam,
    home_team: ScheduleTeam,
    game_state: GameState,
}

impl ScheduleGameBuilder {
    fn new(away_abbrev: &str, home_abbrev: &str) -> Self {
        Self {
            id: 2023020001,
            game_type: GameType::RegularSeason,
            game_date: None,
            start_time_utc: "23:00:00Z".to_string(),
            away_team: TeamBuilder::new(away_abbrev).id(7).build(),
            home_team: TeamBuilder::new(home_abbrev).id(10).build(),
            game_state: GameState::Future,
        }
    }

    #[allow(dead_code)]
    fn id(mut self, id: i64) -> Self {
        self.id = id;
        self
    }

    fn game_date(mut self, date: &str) -> Self {
        self.game_date = Some(date.to_string());
        self
    }

    #[allow(dead_code)]
    fn game_state(mut self, state: GameState) -> Self {
        self.game_state = state;
        self
    }

    #[allow(dead_code)]
    fn away_score(mut self, score: i32) -> Self {
        self.away_team.score = Some(score);
        self
    }

    #[allow(dead_code)]
    fn home_score(mut self, score: i32) -> Self {
        self.home_team.score = Some(score);
        self
    }

    fn build(self) -> ScheduleGame {
        ScheduleGame {
            id: GameId::new(self.id),
            game_type: self.game_type,
            game_date: self.game_date,
            start_time_utc: self.start_time_utc,
            away_team: self.away_team,
            home_team: self.home_team,
            game_state: self.game_state,
            game_schedule_state: GameScheduleState::Ok,
        }
    }
}

/// Builder for creating test GameScore instances
struct GameScoreBuilder {
    id: i64,
    game_type: GameType,
    game_state: GameState,
    away_team: ScheduleTeam,
    home_team: ScheduleTeam,
}

impl GameScoreBuilder {
    fn new(away_abbrev: &str, home_abbrev: &str) -> Self {
        Self {
            id: 2023020001,
            game_type: GameType::RegularSeason,
            game_state: GameState::Future,
            away_team: TeamBuilder::new(away_abbrev).id(7).build(),
            home_team: TeamBuilder::new(home_abbrev).id(10).build(),
        }
    }

    #[allow(dead_code)]
    fn id(mut self, id: i64) -> Self {
        self.id = id;
        self
    }

    fn game_state(mut self, state: GameState) -> Self {
        self.game_state = state;
        self
    }

    fn away_score(mut self, score: i32) -> Self {
        self.away_team.score = Some(score);
        self
    }

    fn home_score(mut self, score: i32) -> Self {
        self.home_team.score = Some(score);
        self
    }

    fn build(self) -> GameScore {
        GameScore {
            id: GameId::new(self.id),
            game_type: self.game_type,
            game_state: self.game_state,
            game_schedule_state: GameScheduleState::Ok,
            away_team: self.away_team,
            home_team: self.home_team,
        }
    }
}

#[test]
fn test_daily_schedule_with_no_games() {
    let schedule = DailySchedule {
        next_start_date: Some("2024-10-20".to_string()),
        previous_start_date: Some("2024-10-18".to_string()),
        date: "2024-10-19".to_string(),
        games: vec![],
        number_of_games: 0,
    };

    assert_eq!(schedule.games.len(), 0);
    assert_eq!(schedule.number_of_games, 0);
}

#[test]
fn test_daily_scores_deserialization() {
    let json = r#"{
        "prevDate": "2024-10-18",
        "currentDate": "2024-10-19",
        "nextDate": "2024-10-20",
        "games": []
    }"#;

    let scores: DailyScores = serde_json::from_str(json).unwrap();
    assert_eq!(scores.current_date, "2024-10-19");
    assert_eq!(scores.games.len(), 0);
}

#[test]
fn test_schedule_game_display() {
    let game = ScheduleGameBuilder::new("BUF", "TOR")
        .game_date("2023-10-10")
        .build();

    assert_eq!(game.to_string(), "BUF @ TOR on 2023-10-10 [FUT]");
}

#[test]
fn test_schedule_game_display_without_date() {
    let game = ScheduleGameBuilder::new("BUF", "TOR").build();

    assert_eq!(game.to_string(), "BUF @ TOR [FUT]");
}

#[test]
fn test_schedule_game_deserialization_without_game_date() {
    let json = r#"{
        "id": 2024020001,
        "gameType": 2,
        "startTimeUTC": "23:00:00Z",
        "awayTeam": {
            "id": 7,
            "abbrev": "BUF",
            "logo": "https://assets.nhle.com/logos/nhl/svg/BUF_light.svg"
        },
        "homeTeam": {
            "id": 10,
            "abbrev": "TOR",
            "logo": "https://assets.nhle.com/logos/nhl/svg/TOR_light.svg"
        },
        "gameState": "FUT",
        "gameScheduleState": "OK"
    }"#;

    let game: ScheduleGame = serde_json::from_str(json).unwrap();
    assert_eq!(game.id, GameId::new(2024020001));
    assert_eq!(game.game_date, None);
    assert_eq!(game.away_team.abbrev, "BUF");
    assert_eq!(game.home_team.abbrev, "TOR");
}

/// `ScheduleGame.id`/`ScheduleTeam.id` accept numeric-string forms as well
/// as integers (1.3).
#[test]
fn test_schedule_game_ids_deserialize_from_numeric_strings() {
    let json = r#"{
        "id": "2024020001",
        "gameType": 2,
        "startTimeUTC": "23:00:00Z",
        "awayTeam": {
            "id": "7",
            "abbrev": "BUF",
            "logo": "https://assets.nhle.com/logos/nhl/svg/BUF_light.svg"
        },
        "homeTeam": {
            "id": "10",
            "abbrev": "TOR",
            "logo": "https://assets.nhle.com/logos/nhl/svg/TOR_light.svg"
        },
        "gameState": "FUT",
        "gameScheduleState": "OK"
    }"#;

    let game: ScheduleGame = serde_json::from_str(json).unwrap();
    assert_eq!(game.id, GameId::new(2024020001));
    assert_eq!(game.away_team.id, TeamId::new(7));
    assert_eq!(game.home_team.id, TeamId::new(10));
}

#[test]
fn test_game_score_display() {
    let game = GameScoreBuilder::new("BUF", "TOR")
        .away_score(3)
        .home_score(2)
        .game_state(GameState::Final)
        .build();

    assert_eq!(game.to_string(), "BUF 3 @ TOR 2 [FINAL]");
}

#[test]
fn test_game_score_display_with_no_scores() {
    // Test the None => Cow::Borrowed("-") branch
    let game = GameScoreBuilder::new("BUF", "TOR").build();

    assert_eq!(game.to_string(), "BUF - @ TOR - [FUT]");
}

#[test]
fn test_game_score_display_with_partial_score() {
    // Test mixed Some/None scores (one team has score, other doesn't)
    let game = GameScoreBuilder::new("BUF", "TOR")
        .away_score(1)
        .game_state(GameState::Live)
        .build();

    assert_eq!(game.to_string(), "BUF 1 @ TOR - [LIVE]");
}

#[test]
fn test_game_score_display_with_zero_scores() {
    // Test that zero scores display as "0" not "-"
    let game = GameScoreBuilder::new("BUF", "TOR")
        .away_score(0)
        .home_score(0)
        .game_state(GameState::Live)
        .build();

    assert_eq!(game.to_string(), "BUF 0 @ TOR 0 [LIVE]");
}

/// The first week on record: no previous week.
const WEEKLY_FIRST: &str = include_str!("../../tests/fixtures/weekly_schedule/1917-12-19.json");
/// The newest week on record: no next week.
const WEEKLY_NEWEST: &str =
    include_str!("../../tests/fixtures/weekly_schedule/2027-07-20_newest.json");
const WEEKLY_MODERN: &str = include_str!("../../tests/fixtures/weekly_schedule/2026-10-21.json");
/// Holds game 2022010034, a preseason game cancelled by Hurricane Ian.
const WEEKLY_CANCELLED: &str =
    include_str!("../../tests/fixtures/weekly_schedule/2022-09-28_cancelled.json");
const SCORES_CANCELLED: &str =
    include_str!("../../tests/fixtures/daily_scores/2022-09-28_cancelled.json");
const TEAM_WEEK: &str =
    include_str!("../../tests/fixtures/team_weekly_schedule/mtl_2026-10-21.json");
const TEAM_WEEK_EMPTY: &str =
    include_str!("../../tests/fixtures/team_weekly_schedule/mtl_2026-07-15_empty.json");
const TEAM_SEASON_1925: &str =
    include_str!("../../tests/fixtures/club_schedule_season/mtl_19251926.json");

const CANCELLED_GAME: GameId = GameId::new(2022010034);
const DAYS_PER_WEEK: usize = 7;

fn weekly(json: &str) -> WeeklyScheduleResponse {
    serde_json::from_str(json).unwrap()
}

fn week_games(week: &WeeklyScheduleResponse) -> impl Iterator<Item = &ScheduleGame> {
    week.game_week.iter().flat_map(|day| &day.games)
}

#[test]
fn test_weekly_schedule_first_week_has_no_previous_start_date() {
    let week = weekly(WEEKLY_FIRST);

    assert_eq!(week.previous_start_date, None);
    assert_eq!(week.next_start_date.as_deref(), Some("1917-12-26"));
    assert_eq!(week.game_week.len(), DAYS_PER_WEEK);
    assert_eq!(week_games(&week).count(), 4);
}

#[test]
fn test_weekly_schedule_newest_week_has_no_next_start_date() {
    let week = weekly(WEEKLY_NEWEST);

    assert_eq!(week.next_start_date, None);
    assert_eq!(week.previous_start_date.as_deref(), Some("2027-04-06"));
    assert_eq!(week_games(&week).count(), 0);
}

#[test]
fn test_weekly_schedule_modern_week_has_both_start_dates() {
    let week = weekly(WEEKLY_MODERN);

    assert_eq!(week.previous_start_date.as_deref(), Some("2026-10-14"));
    assert_eq!(week.next_start_date.as_deref(), Some("2026-10-28"));
    assert_eq!(week.game_week.len(), DAYS_PER_WEEK);
    assert_eq!(week.game_week[0].date, "2026-10-21");
    assert!(week_games(&week).all(|game| game.game_schedule_state == GameScheduleState::Ok));
}

#[test]
fn test_weekly_schedule_missing_start_dates_are_not_serialized() {
    let json = serde_json::to_value(weekly(WEEKLY_NEWEST)).unwrap();

    assert!(json.get("nextStartDate").is_none());
    assert_eq!(json["previousStartDate"], "2027-04-06");
}

#[test]
fn test_weekly_schedule_cancelled_game() {
    let week = weekly(WEEKLY_CANCELLED);
    let game = week_games(&week)
        .find(|game| game.id == CANCELLED_GAME)
        .unwrap();

    assert_eq!(game.game_schedule_state, GameScheduleState::Cancelled);
    assert!(!game.game_schedule_state.is_playable());
    // A cancelled game is never played, so it stays `FUT`.
    assert_eq!(game.game_state, GameState::Future);
    assert_eq!(game.game_type, GameType::Preseason);
}

#[test]
fn test_daily_scores_cancelled_game() {
    let scores: DailyScores = serde_json::from_str(SCORES_CANCELLED).unwrap();
    let (cancelled, played): (Vec<_>, Vec<_>) = scores
        .games
        .iter()
        .partition(|game| game.id == CANCELLED_GAME);

    assert_eq!(cancelled.len(), 1);
    assert_eq!(
        cancelled[0].game_schedule_state,
        GameScheduleState::Cancelled
    );
    assert!(!played.is_empty());
    assert!(played
        .iter()
        .all(|game| game.game_schedule_state == GameScheduleState::Ok));
}

#[test]
fn test_game_day_deserialization() {
    let json = r#"{
        "date": "2026-10-21",
        "dayAbbrev": "WED",
        "numberOfGames": 0,
        "games": []
    }"#;

    let day: GameDay = serde_json::from_str(json).unwrap();
    assert_eq!(day.date, "2026-10-21");
    assert!(day.games.is_empty());
}

#[test]
fn test_game_day_deserialization_from_recorded_week() {
    let week: serde_json::Value = serde_json::from_str(WEEKLY_MODERN).unwrap();
    let day: GameDay = serde_json::from_value(week["gameWeek"][0].clone()).unwrap();

    assert_eq!(day.date, "2026-10-21");
    assert!(!day.games.is_empty());
    assert!(day.games.iter().all(|game| game.game_date.is_none()));
}

#[test]
fn test_team_schedule_response_deserialization() {
    let schedule: TeamScheduleResponse = serde_json::from_str(TEAM_WEEK).unwrap();

    assert_eq!(schedule.games.len(), 3);
    assert!(schedule
        .games
        .iter()
        .all(|game| { game.away_team.abbrev == "MTL" || game.home_team.abbrev == "MTL" }));
}

#[test]
fn test_team_schedule_response_empty_week() {
    let schedule: TeamScheduleResponse = serde_json::from_str(TEAM_WEEK_EMPTY).unwrap();

    assert!(schedule.games.is_empty());
}

#[test]
fn test_team_schedule_response_1925_season() {
    let schedule: TeamScheduleResponse = serde_json::from_str(TEAM_SEASON_1925).unwrap();

    assert_eq!(schedule.games.len(), 36);
    assert!(schedule
        .games
        .iter()
        .all(|game| game.game_schedule_state == GameScheduleState::Ok));
}
