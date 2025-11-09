use std::time::Duration;

#[derive(Debug, Clone)]
pub struct ClientConfig {
    pub(crate) timeout: Duration,
    pub(crate) ssl_verify: bool,
    pub(crate) follow_redirects: bool,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(10),
            ssl_verify: true,
            follow_redirects: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_config_default() {
        let config = ClientConfig::default();
        assert_eq!(config.timeout, Duration::from_secs(10));
        assert!(config.ssl_verify);
        assert!(config.follow_redirects);
    }
}
