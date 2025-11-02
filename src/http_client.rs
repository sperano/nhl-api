use crate::config::ClientConfig;
use crate::error::NHLApiError;
use anyhow::Result;
use reqwest::{Client, Response};
use std::collections::HashMap;
use tracing::debug;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Endpoint {
    ApiWebV1,
    ApiCore,
    ApiStats,
    SearchV1,
}

impl Endpoint {
    pub fn base_url(&self) -> &str {
        match self {
            Endpoint::ApiWebV1 => "https://api-web.nhle.com/v1/",
            Endpoint::ApiCore => "https://api.nhle.com/",
            Endpoint::ApiStats => "https://api.nhle.com/stats/rest/",
            Endpoint::SearchV1 => "https://search.d3.nhle.com/api/v1/",
        }
    }
}

pub struct HttpClient {
    client: Client,
}

impl HttpClient {
    pub fn new(config: ClientConfig) -> Result<Self> {
        let mut client_builder = Client::builder()
            .timeout(config.timeout)
            .danger_accept_invalid_certs(!config.ssl_verify);

        if config.follow_redirects {
            client_builder = client_builder.redirect(reqwest::redirect::Policy::limited(10));
        } else {
            client_builder = client_builder.redirect(reqwest::redirect::Policy::none());
        }

        let client = client_builder.build()?;
        Ok(Self { client })
    }

    fn error_from_status(status_code: u16, url: &str) -> NHLApiError {
        let message = format!("Request to {} failed", url);

        match status_code {
            404 => NHLApiError::ResourceNotFound {
                message,
                status_code,
            },
            429 => NHLApiError::RateLimitExceeded {
                message,
                status_code,
            },
            400 => NHLApiError::BadRequest {
                message,
                status_code,
            },
            401 => NHLApiError::Unauthorized {
                message,
                status_code,
            },
            500..=599 => NHLApiError::ServerError {
                message,
                status_code,
            },
            _ => NHLApiError::ApiError {
                message: format!("Unexpected error: {}", message),
                status_code,
                error_code: None,
            },
        }
    }

    fn handle_response(&self, response: Response, url: &str) -> Result<Response, NHLApiError> {
        let status = response.status();
        if status.is_success() {
            return Ok(response);
        }

        Err(Self::error_from_status(status.as_u16(), url))
    }

    pub async fn get_json<T: serde::de::DeserializeOwned>(
        &self,
        endpoint: Endpoint,
        resource: &str,
        query_params: Option<HashMap<String, String>>,
    ) -> Result<T> {
        let full_url = format!("{}{}", endpoint.base_url(), resource);

        debug!(url = %full_url, "Sending HTTP GET request");

        let mut request = self.client.get(&full_url);

        if let Some(params) = query_params {
            debug!(params = ?params, "Adding query parameters");
            request = request.query(&params);
        }

        let response = request.send().await?;
        debug!(status = %response.status(), url = %full_url, "Received HTTP response");

        let response = self.handle_response(response, resource)?;

        let json = response.json::<T>().await?;
        debug!(url = %full_url, "Successfully deserialized response");
        Ok(json)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    fn assert_config_creates_client(config: ClientConfig) {
        let client = HttpClient::new(config);
        assert!(client.is_ok(), "HttpClient creation should succeed");
    }

    #[test]
    fn test_endpoint_base_url_api_web_v1() {
        let endpoint = Endpoint::ApiWebV1;
        assert_eq!(endpoint.base_url(), "https://api-web.nhle.com/v1/");
    }

    #[test]
    fn test_endpoint_base_url_api_core() {
        let endpoint = Endpoint::ApiCore;
        assert_eq!(endpoint.base_url(), "https://api.nhle.com/");
    }

    #[test]
    fn test_endpoint_base_url_api_stats() {
        let endpoint = Endpoint::ApiStats;
        assert_eq!(endpoint.base_url(), "https://api.nhle.com/stats/rest/");
    }

    #[test]
    fn test_endpoint_base_url_search_v1() {
        let endpoint = Endpoint::SearchV1;
        assert_eq!(endpoint.base_url(), "https://search.d3.nhle.com/api/v1/");
    }

    #[test]
    fn test_endpoint_clone() {
        let endpoint1 = Endpoint::ApiWebV1;
        let endpoint2 = endpoint1.clone();
        assert_eq!(endpoint1.base_url(), endpoint2.base_url());
    }

    #[test]
    fn test_http_client_new_default_config() {
        assert_config_creates_client(ClientConfig::default());
    }

    #[test]
    fn test_http_client_new_with_custom_timeout() {
        assert_config_creates_client(ClientConfig {
            timeout: Duration::from_secs(60),
            ..Default::default()
        });
    }

    #[test]
    fn test_http_client_new_with_redirects_disabled() {
        assert_config_creates_client(ClientConfig {
            follow_redirects: false,
            ..Default::default()
        });
    }

    #[test]
    fn test_http_client_new_with_ssl_verify_disabled() {
        assert_config_creates_client(ClientConfig {
            ssl_verify: false,
            ..Default::default()
        });
    }

    #[test]
    fn test_http_client_new_with_all_options() {
        assert_config_creates_client(ClientConfig {
            timeout: Duration::from_secs(120),
            follow_redirects: false,
            ssl_verify: false,
        });
    }

    #[test]
    fn test_http_client_new_with_very_short_timeout() {
        assert_config_creates_client(ClientConfig {
            timeout: Duration::from_millis(100),
            ..Default::default()
        });
    }

    #[test]
    fn test_http_client_new_with_zero_timeout() {
        assert_config_creates_client(ClientConfig {
            timeout: Duration::from_secs(0),
            ..Default::default()
        });
    }
}
