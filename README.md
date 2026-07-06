# nhl-api

A Rust client library for the NHL API. Get teams, scores, schedules, standings, and game stats.

Based on the excellent [nhl-api-py](https://github.com/coreyjs/nhl-api-py).

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
nhl_api = "0.8"
tokio = { version = "1", features = ["full"] }
```

Enable the `fixtures` feature if your own tests need throwaway `Boxscore`/`PlayByPlay`/etc. values:

```toml
nhl_api = { version = "0.8", features = ["fixtures"] }
```

## Quick Start

```rust
use nhl_api::{Client, GameDate};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()?;

    // Get current standings
    let standings = client.current_league_standings().await?;
    for standing in standings {
        println!("{}: {} pts", standing.team_name.default, standing.points);
    }

    // Get today's schedule
    let schedule = client.daily_schedule(None).await?;
    println!("{}: {} games", schedule.date, schedule.number_of_games);
    for game in schedule.games {
        println!("  {} @ {}", game.away_team.abbrev, game.home_team.abbrev);
    }

    // Get boxscore for a specific game
    let boxscore = client.boxscore(2024020001).await?;
    println!("{} {} - {} {}",
        boxscore.away_team.abbrev, boxscore.away_team.score,
        boxscore.home_team.abbrev, boxscore.home_team.score
    );

    Ok(())
}
```

## API Coverage

### Standings

```rust
// Current standings
let standings = client.current_league_standings().await?;

// Standings for a specific date
let date = NaiveDate::from_ymd_opt(2024, 1, 15).unwrap();
let standings = client.league_standings_for_date(&GameDate::Date(date)).await?;

// Standings for a season (by season ID)
let standings = client.league_standings_for_season(20232024).await?;

// Season metadata (date ranges, etc.)
let seasons = client.season_standing_manifest().await?;
```

### Schedule

```rust
// Today's schedule
let schedule = client.daily_schedule(None).await?;

// Schedule for a specific date
let date = NaiveDate::from_ymd_opt(2024, 1, 15).unwrap();
let schedule = client.daily_schedule(Some(GameDate::Date(date))).await?;

// Weekly schedule
let schedule = client.weekly_schedule(None).await?;

// Team-specific weekly schedule
let schedule = client.team_weekly_schedule("BOS", None).await?;

// Full-season team schedule
use nhl_api::Season;
let schedule = client.club_schedule_season("BOS", Season::new(2024)).await?;

// Daily scores
let scores = client.daily_scores(None).await?;
```

### Game Data

```rust
// Boxscore
let boxscore = client.boxscore(2024020001).await?;

// Play-by-play
let pbp = client.play_by_play(2024020001).await?;

// Game landing (lighter than play-by-play)
let landing = client.landing(2024020001).await?;

// Game story narrative
let story = client.game_story(2024020001).await?;

// Season series matchup
let series = client.season_series(2024020001).await?;

// Shift chart data
let shifts = client.shift_chart(2024020001).await?;
```

### Players

```rust
// Player profile and stats
let player = client.player_landing(8478402).await?; // Connor McDavid

// Player game log
use nhl_api::GameType;
let log = client.player_game_log(8478402, 20242025, GameType::RegularSeason).await?;

// Search players
let results = client.search_player("McDavid", Some(10)).await?;
```

### Teams

```rust
// All teams (derived from standings)
let teams = client.teams(None).await?;

// Current roster
let roster = client.roster_current("BOS").await?;

// Historical roster
let roster = client.roster_season("BOS", 20232024).await?;

// Club stats for a season
let stats = client.club_stats("BOS", 20242025, GameType::RegularSeason).await?;

// Available seasons for a team
let seasons = client.club_stats_season("BOS").await?;

// All franchises (including historical)
let franchises = client.franchises().await?;
```

### Edge Stats

Puck/player-tracking stats from the NHL Edge system — 22 methods across skaters, goalies, and
teams, each with a per-player/team `_detail` family plus a league-wide `_landing` leaderboard.

```rust
use nhl_api::{GameType, Season};

// Skater overview stats for a season
let detail = client.edge_skater_detail(8478402, Season::new(2024), GameType::RegularSeason).await?;

// Team overview stats (rank-based, 1-32)
let team_detail = client.edge_team_detail(10, Season::new(2024), GameType::RegularSeason).await?;

// League-wide skater leaderboard (no player id)
let leaders = client.edge_skater_landing(Season::new(2024), GameType::RegularSeason).await?;
```

## Configuration

```rust
use nhl_api::{Client, ClientConfig};
use std::time::Duration;

let config = ClientConfig::default()
    .with_timeout(Duration::from_secs(30))
    .with_user_agent("my-app/1.0");
let client = Client::with_config(config)?;
```

`ClientConfig` also supports `with_ssl_verify()`, `with_follow_redirects()`, and
`with_http_client(reqwest::Client)` — the last one is an escape hatch for retry/backoff or
instrumentation middleware; when set, the other transport options are ignored and the injected
client's configuration is used as-is.

## Types

The library provides strongly-typed responses for all API endpoints. Key types include:

- `Standing` - Team standings with points, wins, losses, etc.
- `ScheduleGame` - Scheduled game with teams and start time
- `Boxscore` - Complete game boxscore with player stats
- `PlayByPlay` - All play events from a game
- `PlayerLanding` - Player profile with career stats
- `Roster` - Team roster with player details
- `Season` - An NHL season (e.g. `2023-2024`); parses from `"20232024"`, `"2023-2024"`, or an
  integer, and serializes/deserializes accordingly
- `GameId`, `PlayerId`, `TeamId` - Typed numeric identifiers used throughout response structs and
  client method parameters (`impl Into<GameId>` etc., so plain `i64` call sites still work)
- `GameType` - 15 variants (`RegularSeason`, `Playoffs`, `Preseason`, `AllStar`, plus historical/
  special event types); `label()` returns a stable snake_case string (e.g. `"regular_season"`)
- `GameState` - FUT, PRE, LIVE, CRIT, FINAL, OFF
- `GameDate` - Either `Now` or `Date(NaiveDate)`
- Edge stats types (`EdgeSkaterDetail`, `EdgeGoalieDetail`, `EdgeTeamDetail`, and friends) - puck/
  player-tracking data returned by the `edge_*` client methods

## Error Handling

All client methods return `Result<T, NHLApiError>`. Error variants include:

- `ResourceNotFound` - 404 errors
- `RateLimitExceeded` - 429 errors
- `BadRequest` - 400 errors
- `ServerError` - 5xx errors
- `RequestError` - Network/connection issues
- `JsonError` - Deserialization failures; carries the request URL and the underlying
  `serde_json::Error`

Error messages for non-2xx responses include a snippet of the response body (truncated to 4096
bytes) for easier diagnosis. Unrecognized enum values from the API (e.g. a new game type NHL adds
before this library is updated) surface as an `UnknownEnumValue { enum_name, value }` error from
`FromStr`, or as a descriptive message at the serde boundary.

## License

GPL-3.0-or-later
