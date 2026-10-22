//! Player profiles, game logs and search.

use super::{web, with_query, CONNOR_MCDAVID, SEARCH_BASE};
use crate::common::Case;
use nhl_api::{GameType, PlayerGameLog, PlayerLanding, PlayerSearchResult};

const CAREY_PRICE: i64 = 8471679;
const WAYNE_GRETZKY: i64 = 8447400;
const BOBBY_ORR: i64 = 8450070;
const HOWIE_MORENZ: i64 = 8447888;
const GEORGES_VEZINA: i64 = 8450137;
const SEARCH_LIMIT: i32 = 20;
const REGULAR: GameType = GameType::RegularSeason;
const PLAYOFFS: GameType = GameType::Playoffs;

/// `player_landing!(out, "label", player_id)`.
macro_rules! player_landing {
    ($out:ident, $label:literal, $player:expr) => {
        case!(
            $out,
            PlayerLanding,
            "player_landing" / $label,
            web(&format!("player/{}/landing", $player)),
            |c| c.player_landing($player)
        )
    };
}

/// `player_game_log!(out, "label", player_id, season_id, game_type)`.
macro_rules! player_game_log {
    ($out:ident, $label:literal, $player:expr, $season:literal, $game_type:expr) => {
        case!(
            $out,
            PlayerGameLog,
            "player_game_log" / $label,
            web(&format!(
                "player/{}/game-log/{}/{}",
                $player,
                $season,
                $game_type.to_int()
            )),
            |c| c.player_game_log($player, $season, $game_type)
        )
    };
}

/// `search_player!(out, "label", "query")`.
macro_rules! search_player {
    ($out:ident, $label:literal, $query:literal) => {
        case!(
            $out,
            Vec<PlayerSearchResult>,
            "search_player" / $label,
            with_query(
                SEARCH_BASE,
                "search/player",
                &[
                    ("culture", "en-us"),
                    ("q", $query),
                    ("limit", &SEARCH_LIMIT.to_string()),
                ]
            ),
            |c| c.search_player($query, Some(SEARCH_LIMIT))
        )
    };
}

pub(super) fn landing_cases(out: &mut Vec<Case>) {
    player_landing!(out, "mcdavid", CONNOR_MCDAVID);
    player_landing!(out, "price_goalie", CAREY_PRICE);
    player_landing!(out, "gretzky", WAYNE_GRETZKY);
    player_landing!(out, "orr", BOBBY_ORR);
    player_landing!(out, "morenz", HOWIE_MORENZ);
    player_landing!(out, "vezina_goalie", GEORGES_VEZINA);
}

#[rustfmt::skip]
pub(super) fn game_log_cases(out: &mut Vec<Case>) {
    player_game_log!(out, "mcdavid_20252026_regular", CONNOR_MCDAVID, 20252026, REGULAR);
    player_game_log!(out, "mcdavid_20242025_playoffs", CONNOR_MCDAVID, 20242025, PLAYOFFS);
    player_game_log!(out, "price_20192020_regular", CAREY_PRICE, 20192020, REGULAR);
    player_game_log!(out, "price_20102011_playoffs", CAREY_PRICE, 20102011, PLAYOFFS);
    player_game_log!(out, "orr_19701971_regular", BOBBY_ORR, 19701971, REGULAR);
    player_game_log!(out, "morenz_19271928_regular", HOWIE_MORENZ, 19271928, REGULAR);
    player_game_log!(out, "vezina_19241925_regular", GEORGES_VEZINA, 19241925, REGULAR);
}

pub(super) fn search_cases(out: &mut Vec<Case>) {
    search_player!(out, "mcdavid", "mcdavid");
    search_player!(out, "no_match", "zzqqxx");
}
