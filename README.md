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

## Usage

```rust
use nhl_api::{Client, GameDate};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    // Get current standings
    let standings = client.league_standings_for_date(GameDate::Now).await?;
    for standing in standings.standings {
        println!("{}: {} pts", standing.team_name.default, standing.points);
    }

    // Get today's schedule
    let schedule = client.daily_schedule(GameDate::Now).await?;
    for game_day in schedule.game_week {
        println!("{}", game_day.date);
        for game in game_day.games {
            println!("  {} @ {}",
                game.away_team.abbrev,
                game.home_team.abbrev
            );
        }
    }

    // Get boxscore for a specific game
    let boxscore = client.boxscore(2023020001).await?;
    println!("{} {} - {} {}",
        boxscore.away_team.abbrev, boxscore.away_team.score,
        boxscore.home_team.abbrev, boxscore.home_team.score
    );

    Ok(())
}
```

## Examples

**Get standings for a specific date:**

```rust
use chrono::NaiveDate;
use nhl_api::{Client, GameDate};

let date = NaiveDate::from_ymd_opt(2024, 1, 15).unwrap();
let standings = client.league_standings_for_date(GameDate::Date(date)).await?;
```

**Get a week's schedule:**

```rust
let schedule = client.weekly_schedule(GameDate::Now).await?;
```

## License

GPL-3.0-or-later
