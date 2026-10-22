//! Every `Client` method, called across game states (scheduled, in progress,
//! final), regular season and playoffs, eras (1920s, 1970s, 2010, current),
//! skaters and goalies, and empty schedule weeks.

mod edge;

use super::{Case, CurrentGames, Fetch, GameFixtures, Live};
use nhl_api::{
    Boxscore, ClubStats, DailyScores, FranchisesResponse, GameDate, GameId, GameMatchup, GameStory,
    GameType, PlayByPlay, PlayerGameLog, PlayerLanding, PlayerSearchResult, Roster, Season,
    SeasonGameTypes, SeasonSeriesMatchup, SeasonsResponse, ShiftChart, StandingsResponse,
    TeamScheduleResponse, WeeklyScheduleResponse,
};
use reqwest::Url;

const WEB_BASE: &str = "https://api-web.nhle.com/v1/";
const STATS_BASE: &str = "https://api.nhle.com/stats/rest/";
const SEARCH_BASE: &str = "https://search.d3.nhle.com/api/v1/";

const CONNOR_MCDAVID: i64 = 8478402;
const CAREY_PRICE: i64 = 8471679;
const FILIP_GUSTAVSSON: i64 = 8479406;
const WAYNE_GRETZKY: i64 = 8447400;
const BOBBY_ORR: i64 = 8450070;
const HOWIE_MORENZ: i64 = 8447888;
const GEORGES_VEZINA: i64 = 8450137;
const CANADIENS_ID: i64 = 8;
const SEARCH_LIMIT: i32 = 20;

/// Every case. Cases on a game in [`CurrentGames`] can only run live when that
/// game exists; pass `CurrentGames::default()` to list the recorded cases.
pub fn cases(current: &CurrentGames) -> Vec<Case> {
    let mut out = Vec::new();
    game_center_cases(&mut out, current);
    standings_cases(&mut out);
    schedule_cases(&mut out);
    player_cases(&mut out);
    team_cases(&mut out);
    edge::skater_cases(&mut out);
    edge::goalie_cases(&mut out);
    edge::team_cases(&mut out);
    out
}

fn url(base: &str, path: &str) -> Url {
    Url::parse(base)
        .and_then(|base| base.join(path))
        .expect("valid URL")
}

fn web(path: &str) -> Url {
    url(WEB_BASE, path)
}

fn with_query(base: &str, path: &str, query: &[(&str, &str)]) -> Url {
    let mut url = url(base, path);
    url.query_pairs_mut().extend_pairs(query);
    url
}

fn search_url(query: &str) -> Url {
    let limit = SEARCH_LIMIT.to_string();
    with_query(
        SEARCH_BASE,
        "search/player",
        &[("culture", "en-us"), ("q", query), ("limit", &limit)],
    )
}

fn date(api: &str) -> GameDate {
    api.parse().expect("valid YYYY-MM-DD date")
}

fn game_center_cases(out: &mut Vec<Case>, current: &CurrentGames) {
    game_cases(out, "fut", current.scheduled, Some(game_fixtures!("fut")));
    game_cases(out, "live", current.in_progress, None);
    let recorded = [
        recorded_game!("regular_20262027", 2026020152),
        recorded_game!("playoffs_20242025", 2024030415),
        recorded_game!("regular_20102011", 2010020001),
        recorded_game!("regular_19751976", 1975020001),
        recorded_game!("regular_19251926", 1925020001),
    ];
    for (label, id, fixtures) in recorded {
        game_cases(out, label, Some(GameId::new(id)), Some(fixtures));
    }
}

/// The six game-center methods for one game. `game` is `None` when the game
/// only exists at certain times (in progress, scheduled) and none does now.
fn game_cases(out: &mut Vec<Case>, label: &str, game: Option<GameId>, fx: Option<GameFixtures>) {
    let live = |path: &dyn Fn(GameId) -> Url, fetch: &dyn Fn(GameId) -> Fetch| {
        game.map(|id| Live {
            url: Some(path(id)),
            fetch: fetch(id),
        })
    };
    let name = |method: &str| format!("{method}/{label}");
    let fx = fx.as_ref();

    out.push(Case::new::<Boxscore>(
        name("boxscore"),
        fx.map(|f| f.boxscore),
        live(&|id| web(&format!("gamecenter/{id}/boxscore")), &|id| {
            call!(|c| c.boxscore(id))
        }),
    ));
    out.push(Case::new::<PlayByPlay>(
        name("play_by_play"),
        fx.map(|f| f.play_by_play),
        live(&|id| web(&format!("gamecenter/{id}/play-by-play")), &|id| {
            call!(|c| c.play_by_play(id))
        }),
    ));
    out.push(Case::new::<GameMatchup>(
        name("landing"),
        fx.map(|f| f.landing),
        live(&|id| web(&format!("gamecenter/{id}/landing")), &|id| {
            call!(|c| c.landing(id))
        }),
    ));
    out.push(Case::new::<SeasonSeriesMatchup>(
        name("season_series"),
        fx.map(|f| f.season_series),
        live(&|id| web(&format!("gamecenter/{id}/right-rail")), &|id| {
            call!(|c| c.season_series(id))
        }),
    ));
    out.push(Case::new::<GameStory>(
        name("game_story"),
        fx.map(|f| f.game_story),
        live(&|id| web(&format!("wsc/game-story/{id}")), &|id| {
            call!(|c| c.game_story(id))
        }),
    ));
    out.push(Case::new::<ShiftChart>(
        name("shift_chart"),
        fx.map(|f| f.shift_chart),
        live(&shift_chart_url, &|id| call!(|c| c.shift_chart(id))),
    ));
}

/// The query `Client::shift_chart` sends.
fn shift_chart_url(id: GameId) -> Url {
    let filter =
        format!("gameId={id} and ((duration != '00:00' and typeCode = 517) or typeCode != 517 )");
    with_query(
        STATS_BASE,
        "en/shiftcharts",
        &[("cayenneExp", &filter), ("exclude", "eventDetails")],
    )
}

fn standings_cases(out: &mut Vec<Case>) {
    case!(
        out,
        StandingsResponse,
        "league_standings_for_date" / "1925-03-01",
        web("standings/1925-03-01"),
        |c| c.league_standings_for_date(&date("1925-03-01"))
    );
    case!(
        out,
        StandingsResponse,
        "league_standings_for_date" / "1975-03-01",
        web("standings/1975-03-01"),
        |c| c.league_standings_for_date(&date("1975-03-01"))
    );
    case!(
        out,
        StandingsResponse,
        "league_standings_for_date" / "2011-03-01",
        web("standings/2011-03-01"),
        |c| c.league_standings_for_date(&date("2011-03-01"))
    );
    case!(
        out,
        StandingsResponse,
        "league_standings_for_date" / "2026-10-21",
        web("standings/2026-10-21"),
        |c| c.league_standings_for_date(&date("2026-10-21"))
    );
    case!(
        out,
        SeasonsResponse,
        "season_standing_manifest" / "all",
        web("standings-season"),
        |c| c.season_standing_manifest()
    );
    live_case!(out, "current_league_standings" / "now", |c| c
        .current_league_standings());
    live_case!(out, "league_standings_for_season" / "19751976", |c| c
        .league_standings_for_season(19751976));
    live_case!(out, "league_standings_for_season" / "20242025", |c| c
        .league_standings_for_season(20242025));
    live_case!(out, "teams" / "now", |c| c.teams(None));
    live_case!(out, "teams" / "1925-03-01", |c| c
        .teams(Some(date("1925-03-01"))));
}

fn schedule_cases(out: &mut Vec<Case>) {
    case!(
        out,
        WeeklyScheduleResponse,
        "weekly_schedule" / "2026-10-21",
        web("schedule/2026-10-21"),
        |c| c.weekly_schedule(Some(date("2026-10-21")))
    );
    case!(
        out,
        WeeklyScheduleResponse,
        "weekly_schedule" / "2026-07-15_empty",
        web("schedule/2026-07-15"),
        |c| c.weekly_schedule(Some(date("2026-07-15")))
    );
    case!(
        out,
        WeeklyScheduleResponse,
        "weekly_schedule" / "1917-12-19",
        web("schedule/1917-12-19"),
        |c| c.weekly_schedule(Some(date("1917-12-19")))
    );
    live_case!(out, "weekly_schedule" / "now", |c| c.weekly_schedule(None));
    live_case!(out, "daily_schedule" / "2026-10-21", |c| c
        .daily_schedule(Some(date("2026-10-21"))));
    live_case!(out, "daily_schedule" / "now", |c| c.daily_schedule(None));
    case!(
        out,
        DailyScores,
        "daily_scores" / "2026-10-21",
        web("score/2026-10-21"),
        |c| c.daily_scores(Some(date("2026-10-21")))
    );
    case!(
        out,
        DailyScores,
        "daily_scores" / "2026-07-15_empty",
        web("score/2026-07-15"),
        |c| c.daily_scores(Some(date("2026-07-15")))
    );
    case!(
        out,
        DailyScores,
        "daily_scores" / "1975-10-09",
        web("score/1975-10-09"),
        |c| c.daily_scores(Some(date("1975-10-09")))
    );
    live_case!(out, "daily_scores" / "now", |c| c.daily_scores(None));
    case!(
        out,
        TeamScheduleResponse,
        "team_weekly_schedule" / "mtl_2026-10-21",
        web("club-schedule/MTL/week/2026-10-21"),
        |c| c.team_weekly_schedule("MTL", Some(date("2026-10-21")))
    );
    case!(
        out,
        TeamScheduleResponse,
        "team_weekly_schedule" / "mtl_2026-07-15_empty",
        web("club-schedule/MTL/week/2026-07-15"),
        |c| c.team_weekly_schedule("MTL", Some(date("2026-07-15")))
    );
    case!(
        out,
        TeamScheduleResponse,
        "club_schedule_season" / "mtl_20252026",
        web("club-schedule-season/MTL/20252026"),
        |c| c.club_schedule_season("MTL", Season::new(2025))
    );
    case!(
        out,
        TeamScheduleResponse,
        "club_schedule_season" / "mtl_19251926",
        web("club-schedule-season/MTL/19251926"),
        |c| c.club_schedule_season("MTL", Season::new(1925))
    );
}

fn player_cases(out: &mut Vec<Case>) {
    case!(
        out,
        PlayerLanding,
        "player_landing" / "mcdavid",
        web(&format!("player/{CONNOR_MCDAVID}/landing")),
        |c| c.player_landing(CONNOR_MCDAVID)
    );
    case!(
        out,
        PlayerLanding,
        "player_landing" / "price_goalie",
        web(&format!("player/{CAREY_PRICE}/landing")),
        |c| c.player_landing(CAREY_PRICE)
    );
    case!(
        out,
        PlayerLanding,
        "player_landing" / "gretzky",
        web(&format!("player/{WAYNE_GRETZKY}/landing")),
        |c| c.player_landing(WAYNE_GRETZKY)
    );
    case!(
        out,
        PlayerLanding,
        "player_landing" / "orr",
        web(&format!("player/{BOBBY_ORR}/landing")),
        |c| c.player_landing(BOBBY_ORR)
    );
    case!(
        out,
        PlayerLanding,
        "player_landing" / "morenz",
        web(&format!("player/{HOWIE_MORENZ}/landing")),
        |c| c.player_landing(HOWIE_MORENZ)
    );
    case!(
        out,
        PlayerLanding,
        "player_landing" / "vezina_goalie",
        web(&format!("player/{GEORGES_VEZINA}/landing")),
        |c| c.player_landing(GEORGES_VEZINA)
    );

    case!(
        out,
        PlayerGameLog,
        "player_game_log" / "mcdavid_20252026_regular",
        web(&format!("player/{CONNOR_MCDAVID}/game-log/20252026/2")),
        |c| c.player_game_log(CONNOR_MCDAVID, 20252026, GameType::RegularSeason)
    );
    case!(
        out,
        PlayerGameLog,
        "player_game_log" / "mcdavid_20242025_playoffs",
        web(&format!("player/{CONNOR_MCDAVID}/game-log/20242025/3")),
        |c| c.player_game_log(CONNOR_MCDAVID, 20242025, GameType::Playoffs)
    );
    case!(
        out,
        PlayerGameLog,
        "player_game_log" / "price_20192020_regular",
        web(&format!("player/{CAREY_PRICE}/game-log/20192020/2")),
        |c| c.player_game_log(CAREY_PRICE, 20192020, GameType::RegularSeason)
    );
    case!(
        out,
        PlayerGameLog,
        "player_game_log" / "price_20102011_playoffs",
        web(&format!("player/{CAREY_PRICE}/game-log/20102011/3")),
        |c| c.player_game_log(CAREY_PRICE, 20102011, GameType::Playoffs)
    );
    case!(
        out,
        PlayerGameLog,
        "player_game_log" / "orr_19701971_regular",
        web(&format!("player/{BOBBY_ORR}/game-log/19701971/2")),
        |c| c.player_game_log(BOBBY_ORR, 19701971, GameType::RegularSeason)
    );
    case!(
        out,
        PlayerGameLog,
        "player_game_log" / "morenz_19271928_regular",
        web(&format!("player/{HOWIE_MORENZ}/game-log/19271928/2")),
        |c| c.player_game_log(HOWIE_MORENZ, 19271928, GameType::RegularSeason)
    );
    case!(
        out,
        PlayerGameLog,
        "player_game_log" / "vezina_19241925_regular",
        web(&format!("player/{GEORGES_VEZINA}/game-log/19241925/2")),
        |c| c.player_game_log(GEORGES_VEZINA, 19241925, GameType::RegularSeason)
    );

    case!(
        out,
        Vec<PlayerSearchResult>,
        "search_player" / "mcdavid",
        search_url("mcdavid"),
        |c| c.search_player("mcdavid", Some(SEARCH_LIMIT))
    );
    case!(
        out,
        Vec<PlayerSearchResult>,
        "search_player" / "no_match",
        search_url("zzqqxx"),
        |c| c.search_player("zzqqxx", Some(SEARCH_LIMIT))
    );
}

fn team_cases(out: &mut Vec<Case>) {
    case!(
        out,
        FranchisesResponse,
        "franchises" / "all",
        with_query(STATS_BASE, "en/franchise", &[]),
        |c| c.franchises()
    );
    case!(
        out,
        Roster,
        "roster_current" / "mtl",
        web("roster/MTL/current"),
        |c| c.roster_current("MTL")
    );
    case!(
        out,
        Roster,
        "roster_season" / "mtl_20102011",
        web("roster/MTL/20102011"),
        |c| c.roster_season("MTL", 20102011)
    );
    case!(
        out,
        Roster,
        "roster_season" / "mtl_19751976",
        web("roster/MTL/19751976"),
        |c| c.roster_season("MTL", 19751976)
    );
    case!(
        out,
        Roster,
        "roster_season" / "mtl_19251926",
        web("roster/MTL/19251926"),
        |c| c.roster_season("MTL", 19251926)
    );
    case!(
        out,
        ClubStats,
        "club_stats" / "mtl_20252026_regular",
        web("club-stats/MTL/20252026/2"),
        |c| c.club_stats("MTL", 20252026, GameType::RegularSeason)
    );
    case!(
        out,
        ClubStats,
        "club_stats" / "mtl_20242025_playoffs",
        web("club-stats/MTL/20242025/3"),
        |c| c.club_stats("MTL", 20242025, GameType::Playoffs)
    );
    case!(
        out,
        ClubStats,
        "club_stats" / "mtl_19801981_regular",
        web("club-stats/MTL/19801981/2"),
        |c| c.club_stats("MTL", 19801981, GameType::RegularSeason)
    );
    case!(
        out,
        ClubStats,
        "club_stats" / "mtl_19251926_regular",
        web("club-stats/MTL/19251926/2"),
        |c| c.club_stats("MTL", 19251926, GameType::RegularSeason)
    );
    case!(
        out,
        Vec<SeasonGameTypes>,
        "club_stats_season" / "mtl",
        web("club-stats-season/MTL"),
        |c| c.club_stats_season("MTL")
    );
}
