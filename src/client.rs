use crate::config::ClientConfig;
use crate::date::GameDate;
use crate::error::NHLApiError;
use crate::http_client::HttpClient;

/// Number of results [`Client::search_player`] requests when the caller passes
/// no explicit limit.
const DEFAULT_SEARCH_LIMIT: i32 = 20;

mod endpoint_pairs;
mod game;
mod player;
mod playoffs;
mod schedule;
mod standings;
mod stats;
mod team;

/// High-level client for the NHL API.
///
/// Construct with [`Client::new`] (default configuration) or
/// [`Client::with_config`] for custom timeouts, TLS settings, redirect
/// handling, user agent, or an injected `reqwest::Client`.
pub struct Client {
    client: HttpClient,
}

impl Client {
    /// Create a new NHL client with default configuration.
    pub fn new() -> Result<Self, NHLApiError> {
        Self::with_config(ClientConfig::default())
    }

    /// Create a new NHL client with custom configuration.
    pub fn with_config(config: ClientConfig) -> Result<Self, NHLApiError> {
        Ok(Self {
            client: HttpClient::new(config)?,
        })
    }

    /// Create a default-configured client whose requests all go to `origin`
    /// (a mock server), keeping each endpoint's base path.
    #[cfg(test)]
    fn with_origin_override(origin: impl Into<String>) -> Self {
        Self {
            client: HttpClient::new(ClientConfig::default())
                .unwrap()
                .with_origin_override(origin),
        }
    }

    /// Resolve an optional date to an owned date with a caller-selected default.
    fn resolve_date_or(date: Option<GameDate>, default: GameDate) -> GameDate {
        date.unwrap_or(default)
    }
}

#[cfg(test)]
#[path = "client_test.rs"]
mod tests;
