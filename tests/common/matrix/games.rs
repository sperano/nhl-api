//! The six game-center methods for each game.

use super::{web, with_query, STATS_BASE};
use crate::common::{parse_as, Case, CurrentGames, Fetch, GameFixtures, Live, Parse};
use nhl_api::{
    Boxscore, GameId, GameMatchup, GameStory, PlayByPlay, SeasonSeriesMatchup, ShiftChart,
};
use reqwest::Url;

/// One game-center method: how to name, record, parse and call it.
struct GameEndpoint {
    method: &'static str,
    fixture: fn(&GameFixtures) -> &'static str,
    parse: Parse,
    url: fn(GameId) -> Url,
    fetch: fn(GameId) -> Fetch,
}

const ENDPOINTS: [GameEndpoint; 6] = [
    GameEndpoint {
        method: "boxscore",
        fixture: |f| f.boxscore,
        parse: parse_as::<Boxscore>,
        url: |id| web(&format!("gamecenter/{id}/boxscore")),
        fetch: |id| call!(|c| c.boxscore(id)),
    },
    GameEndpoint {
        method: "play_by_play",
        fixture: |f| f.play_by_play,
        parse: parse_as::<PlayByPlay>,
        url: |id| web(&format!("gamecenter/{id}/play-by-play")),
        fetch: |id| call!(|c| c.play_by_play(id)),
    },
    GameEndpoint {
        method: "landing",
        fixture: |f| f.landing,
        parse: parse_as::<GameMatchup>,
        url: |id| web(&format!("gamecenter/{id}/landing")),
        fetch: |id| call!(|c| c.landing(id)),
    },
    GameEndpoint {
        method: "season_series",
        fixture: |f| f.season_series,
        parse: parse_as::<SeasonSeriesMatchup>,
        url: |id| web(&format!("gamecenter/{id}/right-rail")),
        fetch: |id| call!(|c| c.season_series(id)),
    },
    GameEndpoint {
        method: "game_story",
        fixture: |f| f.game_story,
        parse: parse_as::<GameStory>,
        url: |id| web(&format!("wsc/game-story/{id}")),
        fetch: |id| call!(|c| c.game_story(id)),
    },
    GameEndpoint {
        method: "shift_chart",
        fixture: |f| f.shift_chart,
        parse: parse_as::<ShiftChart>,
        url: shift_chart_url,
        fetch: |id| call!(|c| c.shift_chart(id)),
    },
];

pub(super) fn cases(out: &mut Vec<Case>, current: &CurrentGames) {
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

/// Every game-center method for one game. `game` is `None` when the game only
/// exists at certain times (scheduled, in progress) and none does now.
fn game_cases(out: &mut Vec<Case>, label: &str, game: Option<GameId>, fx: Option<GameFixtures>) {
    for endpoint in &ENDPOINTS {
        out.push(Case {
            name: format!("{}/{label}", endpoint.method),
            fixture: fx.as_ref().map(endpoint.fixture),
            parse: endpoint.parse,
            live: game.map(|id| Live {
                url: Some((endpoint.url)(id)),
                fetch: (endpoint.fetch)(id),
            }),
        });
    }
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
