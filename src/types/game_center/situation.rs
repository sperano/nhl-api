use std::fmt;

/// Parsed game situation from situation code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GameSituation {
    /// Number of away team skaters on ice (1-6).
    pub away_skaters: u8,
    /// Whether away team has goalie in net.
    pub away_goalie_in: bool,
    /// Number of home team skaters on ice (1-6).
    pub home_skaters: u8,
    /// Whether home team has goalie in net.
    pub home_goalie_in: bool,
}

/// Minimum valid skater count encoded in a situation code (goalie excluded).
const MIN_SITUATION_SKATERS: u32 = 1;
/// Maximum valid skater count encoded in a situation code (goalie excluded).
const MAX_SITUATION_SKATERS: u32 = 6;
/// One encoded skater is the goalie-replacement attacker when the goalie is pulled.
const PULLED_GOALIE_SKATER_ADJUSTMENT: u8 = 1;

impl GameSituation {
    /// Parse a situation code string (e.g., "1551").
    ///
    /// Returns `None` if the code is not exactly 4 ASCII digits, either
    /// goalie flag is not `0`/`1`, or either skater count falls outside
    /// `MIN_SITUATION_SKATERS..=MAX_SITUATION_SKATERS`.
    pub fn from_code(code: &str) -> Option<Self> {
        if code.len() != 4 {
            return None;
        }

        let digits: Vec<u32> = code
            .chars()
            .map(|character| character.to_digit(10))
            .collect::<Option<_>>()?;
        let (away_goalie, away_skaters, home_skaters, home_goalie) =
            (digits[0], digits[1], digits[2], digits[3]);

        if away_goalie > 1 || home_goalie > 1 {
            return None;
        }

        let skater_range = MIN_SITUATION_SKATERS..=MAX_SITUATION_SKATERS;
        if !skater_range.contains(&away_skaters) || !skater_range.contains(&home_skaters) {
            return None;
        }

        Some(Self {
            away_goalie_in: away_goalie == 1,
            away_skaters: away_skaters as u8,
            home_skaters: home_skaters as u8,
            home_goalie_in: home_goalie == 1,
        })
    }

    /// Returns true when both teams have the same number of effective skaters.
    ///
    /// The extra attacker replacing a pulled goalie is excluded, so ordinary
    /// 6-on-5 empty-net play remains even strength.
    pub fn is_even_strength(&self) -> bool {
        self.away_effective_skaters() == self.home_effective_skaters()
    }

    /// Returns true if the away team has a power play.
    pub fn is_away_power_play(&self) -> bool {
        self.away_effective_skaters() > self.home_effective_skaters()
    }

    /// Returns true if the home team has a power play.
    pub fn is_home_power_play(&self) -> bool {
        self.home_effective_skaters() > self.away_effective_skaters()
    }

    /// Returns true if either team has pulled their goalie.
    pub fn is_empty_net(&self) -> bool {
        !self.away_goalie_in || !self.home_goalie_in
    }

    /// Returns the strength description (e.g., "5v5", "5v4 PP", "6v5 EN").
    pub fn strength_description(&self) -> String {
        let base = format!("{}v{}", self.away_skaters, self.home_skaters);

        if self.is_empty_net() {
            format!("{} EN", base)
        } else if !self.is_even_strength() {
            format!("{} PP", base)
        } else {
            base
        }
    }

    fn away_effective_skaters(&self) -> u8 {
        effective_skaters(self.away_skaters, self.away_goalie_in)
    }

    fn home_effective_skaters(&self) -> u8 {
        effective_skaters(self.home_skaters, self.home_goalie_in)
    }
}

fn effective_skaters(skaters: u8, goalie_in: bool) -> u8 {
    if goalie_in {
        skaters
    } else {
        skaters.saturating_sub(PULLED_GOALIE_SKATER_ADJUSTMENT)
    }
}

impl fmt::Display for GameSituation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.strength_description())
    }
}
