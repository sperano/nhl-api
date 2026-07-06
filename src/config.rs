use std::time::Duration;

use reqwest::Client;

/// `User-Agent` header sent with every library-issued request unless the caller
/// overrides it via [`ClientConfig::with_user_agent`].
pub const DEFAULT_USER_AGENT: &str = concat!("nhl-api/", env!("CARGO_PKG_VERSION"));

const DEFAULT_TIMEOUT: Duration = Duration::from_secs(10);

/// Configuration for the NHL API client.
///
/// Construct via [`ClientConfig::default`] and refine with the chainable
/// `with_*` methods:
///
/// ```
/// use std::time::Duration;
/// use nhl_api::ClientConfig;
///
/// let config = ClientConfig::default()
///     .with_timeout(Duration::from_secs(30))
///     .with_user_agent("my-app/1.0");
/// ```
///
/// # Custom HTTP client
///
/// [`with_http_client`](Self::with_http_client) supplies a pre-built
/// [`reqwest::Client`] used as-is — the escape hatch for retry/backoff,
/// instrumentation, or custom-transport middleware. When set, the
/// transport-shaping options (`timeout`, `ssl_verify`, `follow_redirects`) and
/// the library's default `User-Agent`/`Accept` headers are **ignored**: the
/// injected client owns its full configuration.
#[derive(Debug, Clone)]
pub struct ClientConfig {
    pub(crate) timeout: Duration,
    pub(crate) ssl_verify: bool,
    pub(crate) follow_redirects: bool,
    pub(crate) user_agent: Option<String>,
    pub(crate) client: Option<Client>,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            timeout: DEFAULT_TIMEOUT,
            ssl_verify: true,
            follow_redirects: true,
            user_agent: None,
            client: None,
        }
    }
}

impl ClientConfig {
    /// Sets the maximum duration for each HTTP request.
    ///
    /// Ignored when a custom client is supplied via
    /// [`with_http_client`](Self::with_http_client).
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Sets whether TLS certificates are verified.
    ///
    /// Ignored when a custom client is supplied via
    /// [`with_http_client`](Self::with_http_client).
    pub fn with_ssl_verify(mut self, verify: bool) -> Self {
        self.ssl_verify = verify;
        self
    }

    /// Sets whether HTTP redirects are followed.
    ///
    /// Ignored when a custom client is supplied via
    /// [`with_http_client`](Self::with_http_client).
    pub fn with_follow_redirects(mut self, follow: bool) -> Self {
        self.follow_redirects = follow;
        self
    }

    /// Overrides the `User-Agent` header sent with every request. Defaults to
    /// [`DEFAULT_USER_AGENT`] when unset.
    ///
    /// Ignored when a custom client is supplied via
    /// [`with_http_client`](Self::with_http_client).
    pub fn with_user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = Some(user_agent.into());
        self
    }

    /// Supplies a pre-built [`reqwest::Client`] to use as-is.
    ///
    /// This is the escape hatch for retry/backoff, instrumentation, or
    /// custom-transport middleware. When set, the client is used unchanged and
    /// every other option on this config (`timeout`, `ssl_verify`,
    /// `follow_redirects`, `user_agent`, and the default `Accept`/`User-Agent`
    /// headers) is ignored: the caller owns the client's full configuration.
    pub fn with_http_client(mut self, client: Client) -> Self {
        self.client = Some(client);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_config_default() {
        let config = ClientConfig::default();
        assert_eq!(config.timeout, DEFAULT_TIMEOUT);
        assert!(config.ssl_verify);
        assert!(config.follow_redirects);
        assert!(config.user_agent.is_none());
        assert!(config.client.is_none());
    }

    #[test]
    fn test_client_config_default_user_agent_const() {
        assert!(DEFAULT_USER_AGENT.starts_with("nhl-api/"));
        assert!(DEFAULT_USER_AGENT.len() > "nhl-api/".len());
    }

    #[test]
    fn test_client_config_builder_methods() {
        let config = ClientConfig::default()
            .with_timeout(Duration::from_secs(30))
            .with_ssl_verify(false)
            .with_follow_redirects(false)
            .with_user_agent("test-agent/9.9");

        assert_eq!(config.timeout, Duration::from_secs(30));
        assert!(!config.ssl_verify);
        assert!(!config.follow_redirects);
        assert_eq!(config.user_agent.as_deref(), Some("test-agent/9.9"));
    }

    #[test]
    fn test_client_config_with_http_client_sets_field() {
        let injected = Client::new();
        let config = ClientConfig::default().with_http_client(injected);
        assert!(config.client.is_some());
    }
}
