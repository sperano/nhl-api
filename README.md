# nhl-api

A Rust client library for the NHL API. Get teams, scores, schedules, standings, and game stats.

Based on the excellent [nhl-api-py](https://github.com/coreyjs/nhl-api-py).

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
nhl_api = "0.5"
tokio = { version = "1", features = ["full"] }
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

## Configuration

```rust
use nhl_api::{Client, ClientConfig};
use std::time::Duration;

let config = ClientConfig {
    timeout: Duration::from_secs(30),
    ssl_verify: true,
    follow_redirects: true,
};
let client = Client::with_config(config)?;
```

## Types

The library provides strongly-typed responses for all API endpoints. Key types include:

- `Standing` - Team standings with points, wins, losses, etc.
- `ScheduleGame` - Scheduled game with teams and start time
- `Boxscore` - Complete game boxscore with player stats
- `PlayByPlay` - All play events from a game
- `PlayerLanding` - Player profile with career stats
- `Roster` - Team roster with player details
- `GameType` - Preseason, RegularSeason, Playoffs, AllStar
- `GameState` - FUT, PRE, LIVE, CRIT, FINAL, OFF
- `GameDate` - Either `Now` or `Date(NaiveDate)`

## Error Handling

All client methods return `Result<T, NHLApiError>`. Error variants include:

- `ResourceNotFound` - 404 errors
- `RateLimitExceeded` - 429 errors
- `BadRequest` - 400 errors
- `ServerError` - 5xx errors
- `RequestError` - Network/connection issues
- `JsonError` - Deserialization failures

## License

GPL-3.0-or-later
