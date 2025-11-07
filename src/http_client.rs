use crate::config::ClientConfig;
use crate::error::NHLApiError;
use reqwest::{Client, Response};
use std::collections::HashMap;
use tracing::debug;

#[derive(Debug, Clone)]
pub enum Endpoint {
    ApiWebV1,
    ApiCore,
    ApiStats,
    SearchV1,
    #[cfg(test)]
    Custom(String),
}

impl Endpoint {
    pub fn base_url(&self) -> &str {
        match self {
            Endpoint::ApiWebV1 => "https://api-web.nhle.com/v1/",
            Endpoint::ApiCore => "https://api.nhle.com/",
            Endpoint::ApiStats => "https://api.nhle.com/stats/rest/",
            Endpoint::SearchV1 => "https://search.d3.nhle.com/api/v1/",
            #[cfg(test)]
            Endpoint::Custom(url) => url.as_str(),
        }
    }
}

pub struct HttpClient {
    client: Client,
}

impl HttpClient {
    pub fn new(config: ClientConfig) -> Result<Self, NHLApiError> {
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
    ) -> Result<T, NHLApiError> {
        let base = endpoint.base_url();
        let full_url = if base.ends_with('/') && resource.starts_with('/') {
            format!("{}{}", base, &resource[1..])
        } else if !base.ends_with('/') && !resource.starts_with('/') {
            format!("{}/{}", base, resource)
        } else {
            format!("{}{}", base, resource)
        };

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

    // ===== Endpoint Tests =====

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
    fn test_endpoint_debug_format() {
        let endpoint = Endpoint::ApiWebV1;
        let debug_str = format!("{:?}", endpoint);
        assert!(debug_str.contains("ApiWebV1"));
    }

    #[test]
    fn test_endpoint_custom() {
        let custom_url = "http://localhost:8080/api/";
        let endpoint = Endpoint::Custom(custom_url.to_string());
        assert_eq!(endpoint.base_url(), custom_url);
    }

    #[test]
    fn test_endpoint_custom_clone() {
        let custom_url = "http://localhost:8080/api/";
        let endpoint1 = Endpoint::Custom(custom_url.to_string());
        let endpoint2 = endpoint1.clone();
        assert_eq!(endpoint1.base_url(), endpoint2.base_url());
    }

    // ===== HttpClient Configuration Tests =====

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

    // ===== Error Mapping Tests =====

    #[test]
    fn test_error_from_status_404_not_found() {
        let error = HttpClient::error_from_status(404, "/test/resource");
        match error {
            NHLApiError::ResourceNotFound {
                message,
                status_code,
            } => {
                assert_eq!(status_code, 404);
                assert!(message.contains("Request to /test/resource failed"));
            }
            _ => panic!("Expected ResourceNotFound error"),
        }
    }

    #[test]
    fn test_error_from_status_429_rate_limit() {
        let error = HttpClient::error_from_status(429, "/test/resource");
        match error {
            NHLApiError::RateLimitExceeded {
                message,
                status_code,
            } => {
                assert_eq!(status_code, 429);
                assert!(message.contains("Request to /test/resource failed"));
            }
            _ => panic!("Expected RateLimitExceeded error"),
        }
    }

    #[test]
    fn test_error_from_status_400_bad_request() {
        let error = HttpClient::error_from_status(400, "/test/resource");
        match error {
            NHLApiError::BadRequest {
                message,
                status_code,
            } => {
                assert_eq!(status_code, 400);
                assert!(message.contains("Request to /test/resource failed"));
            }
            _ => panic!("Expected BadRequest error"),
        }
    }

    #[test]
    fn test_error_from_status_401_unauthorized() {
        let error = HttpClient::error_from_status(401, "/test/resource");
        match error {
            NHLApiError::Unauthorized {
                message,
                status_code,
            } => {
                assert_eq!(status_code, 401);
                assert!(message.contains("Request to /test/resource failed"));
            }
            _ => panic!("Expected Unauthorized error"),
        }
    }

    #[test]
    fn test_error_from_status_500_server_error() {
        let error = HttpClient::error_from_status(500, "/test/resource");
        match error {
            NHLApiError::ServerError {
                message,
                status_code,
            } => {
                assert_eq!(status_code, 500);
                assert!(message.contains("Request to /test/resource failed"));
            }
            _ => panic!("Expected ServerError error"),
        }
    }

    #[test]
    fn test_error_from_status_502_server_error() {
        let error = HttpClient::error_from_status(502, "/test/resource");
        match error {
            NHLApiError::ServerError {
                message,
                status_code,
            } => {
                assert_eq!(status_code, 502);
                assert!(message.contains("Request to /test/resource failed"));
            }
            _ => panic!("Expected ServerError error"),
        }
    }

    #[test]
    fn test_error_from_status_503_server_error() {
        let error = HttpClient::error_from_status(503, "/test/resource");
        match error {
            NHLApiError::ServerError {
                message,
                status_code,
            } => {
                assert_eq!(status_code, 503);
                assert!(message.contains("Request to /test/resource failed"));
            }
            _ => panic!("Expected ServerError error"),
        }
    }

    #[test]
    fn test_error_from_status_599_server_error_boundary() {
        let error = HttpClient::error_from_status(599, "/test/resource");
        match error {
            NHLApiError::ServerError {
                message,
                status_code,
            } => {
                assert_eq!(status_code, 599);
                assert!(message.contains("Request to /test/resource failed"));
            }
            _ => panic!("Expected ServerError error"),
        }
    }

    #[test]
    fn test_error_from_status_418_teapot_api_error() {
        let error = HttpClient::error_from_status(418, "/test/resource");
        match error {
            NHLApiError::ApiError {
                message,
                status_code,
                error_code,
            } => {
                assert_eq!(status_code, 418);
                assert!(message.contains("Unexpected error"));
                assert!(error_code.is_none());
            }
            _ => panic!("Expected ApiError for unexpected status"),
        }
    }

    #[test]
    fn test_error_from_status_402_payment_required_api_error() {
        let error = HttpClient::error_from_status(402, "/test/resource");
        match error {
            NHLApiError::ApiError {
                message,
                status_code,
                error_code,
            } => {
                assert_eq!(status_code, 402);
                assert!(message.contains("Unexpected error"));
                assert!(error_code.is_none());
            }
            _ => panic!("Expected ApiError for unexpected status"),
        }
    }

    #[test]
    fn test_error_from_status_403_forbidden_api_error() {
        let error = HttpClient::error_from_status(403, "/test/resource");
        match error {
            NHLApiError::ApiError {
                message,
                status_code,
                error_code,
            } => {
                assert_eq!(status_code, 403);
                assert!(message.contains("Unexpected error"));
                assert!(error_code.is_none());
            }
            _ => panic!("Expected ApiError for unexpected status"),
        }
    }

    #[test]
    fn test_error_from_status_600_invalid_status_api_error() {
        let error = HttpClient::error_from_status(600, "/test/resource");
        match error {
            NHLApiError::ApiError {
                message,
                status_code,
                error_code,
            } => {
                assert_eq!(status_code, 600);
                assert!(message.contains("Unexpected error"));
                assert!(error_code.is_none());
            }
            _ => panic!("Expected ApiError for out-of-range status"),
        }
    }

    #[test]
    fn test_error_from_status_100_informational_api_error() {
        let error = HttpClient::error_from_status(100, "/test/resource");
        match error {
            NHLApiError::ApiError {
                message,
                status_code,
                error_code,
            } => {
                assert_eq!(status_code, 100);
                assert!(message.contains("Unexpected error"));
                assert!(error_code.is_none());
            }
            _ => panic!("Expected ApiError for informational status"),
        }
    }

    #[test]
    fn test_error_from_status_300_redirect_api_error() {
        let error = HttpClient::error_from_status(300, "/test/resource");
        match error {
            NHLApiError::ApiError {
                message,
                status_code,
                error_code,
            } => {
                assert_eq!(status_code, 300);
                assert!(message.contains("Unexpected error"));
                assert!(error_code.is_none());
            }
            _ => panic!("Expected ApiError for redirect status"),
        }
    }

    // ===== Integration Tests for get_json with mockito =====

    #[tokio::test]
    async fn test_get_json_success_with_mock() {
        use serde::Deserialize;

        #[derive(Debug, Deserialize, PartialEq)]
        struct TestResponse {
            id: i32,
            name: String,
        }

        let mut server = mockito::Server::new_async().await;
        let _mock = server
            .mock("GET", "/test/resource")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"id": 42, "name": "test data"}"#)
            .create_async()
            .await;

        let config = ClientConfig::default();
        let http_client = HttpClient::new(config).unwrap();

        // Use Custom endpoint with mock server URL
        let endpoint = Endpoint::Custom(server.url());
        let result: Result<TestResponse, NHLApiError> =
            http_client.get_json(endpoint, "test/resource", None).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.id, 42);
        assert_eq!(response.name, "test data");
    }

    #[tokio::test]
    async fn test_get_json_with_query_params_mock() {
        use serde::Deserialize;

        #[derive(Debug, Deserialize, PartialEq)]
        struct TestResponse {
            count: i32,
        }

        let mut server = mockito::Server::new_async().await;
        let _mock = server
            .mock("GET", "/search")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("q".into(), "test".into()),
                mockito::Matcher::UrlEncoded("limit".into(), "10".into()),
            ]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"count": 5}"#)
            .create_async()
            .await;

        let config = ClientConfig::default();
        let http_client = HttpClient::new(config).unwrap();

        let mut params = HashMap::new();
        params.insert("q".to_string(), "test".to_string());
        params.insert("limit".to_string(), "10".to_string());

        let endpoint = Endpoint::Custom(server.url());
        let result: Result<TestResponse, NHLApiError> =
            http_client.get_json(endpoint, "search", Some(params)).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().count, 5);
    }

    #[tokio::test]
    async fn test_get_json_404_not_found() {
        use serde::Deserialize;

        #[derive(Debug, Deserialize)]
        struct TestResponse {
            id: i32,
        }

        let mut server = mockito::Server::new_async().await;
        let _mock = server
            .mock("GET", "/missing")
            .with_status(404)
            .create_async()
            .await;

        let config = ClientConfig::default();
        let http_client = HttpClient::new(config).unwrap();

        let endpoint = Endpoint::Custom(server.url());
        let result: Result<TestResponse, NHLApiError> =
            http_client.get_json(endpoint, "missing", None).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            NHLApiError::ResourceNotFound { status_code, .. } => {
                assert_eq!(status_code, 404);
            }
            _ => panic!("Expected ResourceNotFound error"),
        }
    }

    #[tokio::test]
    async fn test_get_json_deserialization_error() {
        use serde::Deserialize;

        #[derive(Debug, Deserialize, PartialEq)]
        struct TestResponse {
            id: i32,
            name: String,
        }

        let mut server = mockito::Server::new_async().await;
        let _mock = server
            .mock("GET", "/bad-json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"invalid": "structure", "missing": "required_fields"}"#)
            .create_async()
            .await;

        let config = ClientConfig::default();
        let http_client = HttpClient::new(config).unwrap();

        let endpoint = Endpoint::Custom(server.url());
        let result: Result<TestResponse, NHLApiError> =
            http_client.get_json(endpoint, "bad-json", None).await;

        // Should fail during deserialization
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_json_server_error() {
        use serde::Deserialize;

        #[derive(Debug, Deserialize)]
        struct TestResponse {
            id: i32,
        }

        let mut server = mockito::Server::new_async().await;
        let _mock = server
            .mock("GET", "/error")
            .with_status(503)
            .create_async()
            .await;

        let config = ClientConfig::default();
        let http_client = HttpClient::new(config).unwrap();

        let endpoint = Endpoint::Custom(server.url());
        let result: Result<TestResponse, NHLApiError> =
            http_client.get_json(endpoint, "error", None).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            NHLApiError::ServerError { status_code, .. } => {
                assert_eq!(status_code, 503);
            }
            _ => panic!("Expected ServerError"),
        }
    }

    #[tokio::test]
    async fn test_get_json_rate_limit() {
        use serde::Deserialize;

        #[derive(Debug, Deserialize)]
        struct TestResponse {
            id: i32,
        }

        let mut server = mockito::Server::new_async().await;
        let _mock = server
            .mock("GET", "/rate-limited")
            .with_status(429)
            .create_async()
            .await;

        let config = ClientConfig::default();
        let http_client = HttpClient::new(config).unwrap();

        let endpoint = Endpoint::Custom(server.url());
        let result: Result<TestResponse, NHLApiError> =
            http_client.get_json(endpoint, "rate-limited", None).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            NHLApiError::RateLimitExceeded { status_code, .. } => {
                assert_eq!(status_code, 429);
            }
            _ => panic!("Expected RateLimitExceeded"),
        }
    }

    #[tokio::test]
    async fn test_handle_response_success() {
        let mut server = mockito::Server::new_async().await;
        let _mock = server
            .mock("GET", "/")
            .with_status(200)
            .with_header("content-type", "text/plain")
            .with_body("OK")
            .create_async()
            .await;

        let config = ClientConfig::default();
        let http_client = HttpClient::new(config).unwrap();

        // Make a real request to get a Response object
        let response = http_client.client.get(server.url()).send().await.unwrap();

        // Test handle_response with successful response
        let result = http_client.handle_response(response, "/test");
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_response_404_error() {
        let mut server = mockito::Server::new_async().await;
        let _mock = server
            .mock("GET", "/")
            .with_status(404)
            .create_async()
            .await;

        let config = ClientConfig::default();
        let http_client = HttpClient::new(config).unwrap();

        let response = http_client.client.get(server.url()).send().await.unwrap();

        let result = http_client.handle_response(response, "/test/resource");
        assert!(result.is_err());

        match result.unwrap_err() {
            NHLApiError::ResourceNotFound {
                message,
                status_code,
            } => {
                assert_eq!(status_code, 404);
                assert!(message.contains("Request to /test/resource failed"));
            }
            _ => panic!("Expected ResourceNotFound error"),
        }
    }

    #[tokio::test]
    async fn test_handle_response_429_rate_limit() {
        let mut server = mockito::Server::new_async().await;
        let _mock = server
            .mock("GET", "/")
            .with_status(429)
            .create_async()
            .await;

        let config = ClientConfig::default();
        let http_client = HttpClient::new(config).unwrap();

        let response = http_client.client.get(server.url()).send().await.unwrap();

        let result = http_client.handle_response(response, "/test/resource");
        assert!(result.is_err());

        match result.unwrap_err() {
            NHLApiError::RateLimitExceeded {
                message,
                status_code,
            } => {
                assert_eq!(status_code, 429);
                assert!(message.contains("Request to /test/resource failed"));
            }
            _ => panic!("Expected RateLimitExceeded error"),
        }
    }

    #[tokio::test]
    async fn test_handle_response_500_server_error() {
        let mut server = mockito::Server::new_async().await;
        let _mock = server
            .mock("GET", "/")
            .with_status(500)
            .create_async()
            .await;

        let config = ClientConfig::default();
        let http_client = HttpClient::new(config).unwrap();

        let response = http_client.client.get(server.url()).send().await.unwrap();

        let result = http_client.handle_response(response, "/test/resource");
        assert!(result.is_err());

        match result.unwrap_err() {
            NHLApiError::ServerError {
                message,
                status_code,
            } => {
                assert_eq!(status_code, 500);
                assert!(message.contains("Request to /test/resource failed"));
            }
            _ => panic!("Expected ServerError error"),
        }
    }

    #[tokio::test]
    async fn test_handle_response_403_api_error() {
        let mut server = mockito::Server::new_async().await;
        let _mock = server
            .mock("GET", "/")
            .with_status(403)
            .create_async()
            .await;

        let config = ClientConfig::default();
        let http_client = HttpClient::new(config).unwrap();

        let response = http_client.client.get(server.url()).send().await.unwrap();

        let result = http_client.handle_response(response, "/test/resource");
        assert!(result.is_err());

        match result.unwrap_err() {
            NHLApiError::ApiError {
                message,
                status_code,
                error_code,
            } => {
                assert_eq!(status_code, 403);
                assert!(message.contains("Unexpected error"));
                assert!(error_code.is_none());
            }
            _ => panic!("Expected ApiError for 403"),
        }
    }
}
