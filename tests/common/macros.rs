//! Shorthand for building [`crate::common::Case`]s.

/// The recorded response for `<method>/<label>`.
macro_rules! fixture {
    ($method:literal, $label:literal) => {
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/",
            $method,
            "/",
            $label,
            ".json"
        ))
    };
}

/// A [`crate::common::Fetch`] calling one `Client` method:
/// `call!(|c| c.boxscore(id))`.
macro_rules! call {
    (|$c:ident| $call:expr) => {
        crate::common::fetch(move |$c| Box::pin(async move { $call.await.map(drop) }))
    };
}

/// A recorded case: `case!(out, Type, "method" / "label", url, |c| call)`,
/// where `Type` is what the recorded response deserializes as.
macro_rules! case {
    ($out:ident, $ty:ty, $method:literal / $label:literal, $url:expr, |$c:ident| $call:expr) => {
        $out.push(crate::common::Case::new::<$ty>(
            concat!($method, "/", $label),
            Some(fixture!($method, $label)),
            Some(crate::common::Live {
                url: Some($url),
                fetch: call!(|$c| $call),
            }),
        ))
    };
}

/// A case with no single recordable response (it makes several requests, or
/// repeats a recorded request with a moving date), run by the live sweep only.
macro_rules! live_case {
    ($out:ident, $method:literal / $label:literal, |$c:ident| $call:expr) => {
        $out.push(crate::common::Case::new::<serde::de::IgnoredAny>(
            concat!($method, "/", $label),
            None,
            Some(crate::common::Live {
                url: None,
                fetch: call!(|$c| $call),
            }),
        ))
    };
}

/// The six game-center fixtures recorded for one game label.
macro_rules! game_fixtures {
    ($label:literal) => {
        crate::common::GameFixtures {
            boxscore: fixture!("boxscore", $label),
            play_by_play: fixture!("play_by_play", $label),
            landing: fixture!("landing", $label),
            season_series: fixture!("season_series", $label),
            game_story: fixture!("game_story", $label),
            shift_chart: fixture!("shift_chart", $label),
        }
    };
}

/// A recorded game: `(label, game id, fixtures)`.
macro_rules! recorded_game {
    ($label:literal, $id:literal) => {
        ($label, $id, game_fixtures!($label))
    };
}
