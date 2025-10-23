use std::time::Duration;

#[derive(Debug, Clone)]
pub struct ClientConfig {
    pub(crate) debug: bool,
    pub(crate) timeout: Duration,
    pub(crate) ssl_verify: bool,
    pub(crate) follow_redirects: bool,
    // pub(crate) api_web_base_url: String,
    // pub(crate) api_base_url: String,
    // pub(crate) api_web_api_ver: String,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            debug: false,
            timeout: Duration::from_secs(10),
            ssl_verify: true,
            follow_redirects: true,
            // api_web_base_url: "https://api-web.nhle.com".to_string(),
            // api_base_url: "https://api.nhle.com".to_string(),
            // api_web_api_ver: "/v1/".to_string(),
        }
    }
}
