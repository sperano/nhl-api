use anyhow::Result;
use reqwest::{Client, Response};
use crate::error::NHLApiError;
use crate::config::ClientConfig;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Endpoint {
    ApiWebV1,
    ApiCore,
    ApiStats,
}

impl Endpoint {
    pub fn base_url(&self) -> &str {
        match self {
            Endpoint::ApiWebV1 => "https://api-web.nhle.com/v1/",
            Endpoint::ApiCore => "https://api.nhle.com/",
            Endpoint::ApiStats => "https://api.nhle.com/stats/rest/",
        }
    }
}

pub struct HttpClient {
    config: ClientConfig,
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
        Ok(Self { config, client })
    }

    fn handle_response(&self, response: Response, url: &str) -> Result<Response, NHLApiError> {
        let status = response.status();
        if status.is_success() {
            return Ok(response);
        }

        let status_code = status.as_u16();
        let error_message = format!("Request to {} failed", url);
        match status_code {
            404 => Err(NHLApiError::ResourceNotFound {
                message: error_message,
                status_code,
            }),
            429 => Err(NHLApiError::RateLimitExceeded {
                message: error_message,
                status_code,
            }),
            400 => Err(NHLApiError::BadRequest {
                message: error_message,
                status_code,
            }),
            401 => Err(NHLApiError::Unauthorized {
                message: error_message,
                status_code,
            }),
            500..=599 => Err(NHLApiError::ServerError {
                message: error_message,
                status_code,
            }),
            _ => Err(NHLApiError::ApiError {
                message: format!("Unexpected error: {}", error_message),
                status_code,
                error_code: None,
            }),
        }
    }

    pub async fn get_json<T: serde::de::DeserializeOwned>(
        &self,
        endpoint: Endpoint,
        resource: &str,
        query_params: Option<HashMap<String, String>>,
    ) -> Result<T> {
        let full_url = format!("{}{}", endpoint.base_url(), resource);

        if self.config.debug {
            eprintln!("GET: {}", full_url);
        }

        let mut request = self.client.get(&full_url);

        if let Some(params) = query_params {
            request = request.query(&params);
        }

        let response = request.send().await?;
        let response = self.handle_response(response, resource)?;

        let json = response.json::<T>().await?;
        Ok(json)
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

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
    fn test_endpoint_clone() {
        let endpoint1 = Endpoint::ApiWebV1;
        let endpoint2 = endpoint1.clone();
        assert_eq!(endpoint1.base_url(), endpoint2.base_url());
    }

    #[test]
    fn test_http_client_new_default_config() {
        let config = ClientConfig::default();
        let client = HttpClient::new(config);
        assert!(client.is_ok());
    }

    #[test]
    fn test_http_client_new_with_custom_timeout() {
        let config = ClientConfig {
            timeout: Duration::from_secs(60),
            ..Default::default()
        };
        let client = HttpClient::new(config);
        assert!(client.is_ok());
    }

    #[test]
    fn test_http_client_new_with_redirects_disabled() {
        let config = ClientConfig {
            follow_redirects: false,
            ..Default::default()
        };
        let client = HttpClient::new(config);
        assert!(client.is_ok());
    }

    #[test]
    fn test_http_client_new_with_ssl_verify_disabled() {
        let config = ClientConfig {
            ssl_verify: false,
            ..Default::default()
        };
        let client = HttpClient::new(config);
        assert!(client.is_ok());
    }

    #[test]
    fn test_http_client_new_with_debug() {
        let config = ClientConfig {
            debug: true,
            ..Default::default()
        };
        let client = HttpClient::new(config);
        assert!(client.is_ok());
    }

    #[test]
    fn test_http_client_new_with_all_options() {
        let config = ClientConfig {
            timeout: Duration::from_secs(120),
            follow_redirects: false,
            ssl_verify: false,
            debug: true,
        };
        let client = HttpClient::new(config);
        assert!(client.is_ok());
    }

    #[test]
    fn test_http_client_new_with_very_short_timeout() {
        let config = ClientConfig {
            timeout: Duration::from_millis(100),
            ..Default::default()
        };
        let client = HttpClient::new(config);
        assert!(client.is_ok());
    }

    #[test]
    fn test_http_client_new_with_zero_timeout() {
        let config = ClientConfig {
            timeout: Duration::from_secs(0),
            ..Default::default()
        };
        let client = HttpClient::new(config);
        // Should still create successfully, though this might cause issues in practice
        assert!(client.is_ok());
    }
}