use crate::config::ClientConfig;
use crate::error::NHLApiError;
use reqwest::{Client, Response};
use std::collections::HashMap;
use tracing::debug;

#[derive(Debug, Clone)]
pub enum Endpoint {
    ApiWebV1,
    #[allow(dead_code)]
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

        macro_rules! error_variant {
            ($variant:ident) => {
                NHLApiError::$variant {
                    message,
                    status_code,
                }
            };
        }

        match status_code {
            404 => error_variant!(ResourceNotFound),
            429 => error_variant!(RateLimitExceeded),
            400 => error_variant!(BadRequest),
            401 => error_variant!(Unauthorized),
            500..=599 => error_variant!(ServerError),
            _ => NHLApiError::ApiError {
                message: format!("Unexpected error: {}", message),
                status_code,
            },
        }
    }

    fn build_url(base: &str, resource: &str) -> String {
        if base.ends_with('/') && resource.starts_with('/') {
            format!("{}{}", base, &resource[1..])
        } else if !base.ends_with('/') && !resource.starts_with('/') {
            format!("{}/{}", base, resource)
        } else {
            format!("{}{}", base, resource)
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
        let full_url = Self::build_url(endpoint.base_url(), resource);

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

    // ===== URL Building Tests =====

    #[test]
    fn test_build_url_both_have_slash() {
        // base ends with /, resource starts with /
        let result = HttpClient::build_url("https://api.example.com/", "/resource");
        assert_eq!(result, "https://api.example.com/resource");
    }

    #[test]
    fn test_build_url_neither_has_slash() {
        // base doesn't end with /, resource doesn't start with /
        let result = HttpClient::build_url("https://api.example.com", "resource");
        assert_eq!(result, "https://api.example.com/resource");
    }

    #[test]
    fn test_build_url_base_has_slash_only() {
        // base ends with /, resource doesn't start with /
        let result = HttpClient::build_url("https://api.example.com/", "resource");
        assert_eq!(result, "https://api.example.com/resource");
    }

    #[test]
    fn test_build_url_resource_has_slash_only() {
        // base doesn't end with /, resource starts with /
        let result = HttpClient::build_url("https://api.example.com", "/resource");
        assert_eq!(result, "https://api.example.com/resource");
    }

    #[test]
    fn test_build_url_with_path_segments() {
        let result = HttpClient::build_url("https://api.example.com/v1/", "/data/items");
        assert_eq!(result, "https://api.example.com/v1/data/items");
    }

    #[test]
    fn test_build_url_empty_resource() {
        let result = HttpClient::build_url("https://api.example.com/", "");
        assert_eq!(result, "https://api.example.com/");
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

    /// Helper function to test that a status code produces the expected error variant
    fn assert_error_matches(
        status_code: u16,
        expected_variant: fn(&NHLApiError) -> bool,
        expected_message_contains: &str,
    ) {
        let error = HttpClient::error_from_status(status_code, "/test/resource");

        assert!(
            expected_variant(&error),
            "Expected specific error variant for status {}, got {:?}",
            status_code,
            error
        );

        // Verify the status code is preserved in the error
        let actual_status = match &error {
            NHLApiError::ResourceNotFound { status_code, .. } => *status_code,
            NHLApiError::RateLimitExceeded { status_code, .. } => *status_code,
            NHLApiError::BadRequest { status_code, .. } => *status_code,
            NHLApiError::Unauthorized { status_code, .. } => *status_code,
            NHLApiError::ServerError { status_code, .. } => *status_code,
            NHLApiError::ApiError { status_code, .. } => *status_code,
            _ => panic!("Unexpected error variant: {:?}", error),
        };
        assert_eq!(actual_status, status_code);

        // Verify the message contains expected text
        let actual_message = match &error {
            NHLApiError::ResourceNotFound { message, .. } => message,
            NHLApiError::RateLimitExceeded { message, .. } => message,
            NHLApiError::BadRequest { message, .. } => message,
            NHLApiError::Unauthorized { message, .. } => message,
            NHLApiError::ServerError { message, .. } => message,
            NHLApiError::ApiError { message, .. } => message,
            _ => panic!("Unexpected error variant"),
        };
        assert!(
            actual_message.contains(expected_message_contains),
            "Expected message to contain '{}', got '{}'",
            expected_message_contains,
            actual_message
        );
    }

    #[test]
    fn test_error_from_status_mapping() {
        struct ErrorTestCase {
            status_code: u16,
            expected_variant: fn(&NHLApiError) -> bool,
            expected_message_contains: &'static str,
        }

        let test_cases = [
            // Specific HTTP error codes
            ErrorTestCase {
                status_code: 404,
                expected_variant: |e| matches!(e, NHLApiError::ResourceNotFound { .. }),
                expected_message_contains: "Request to /test/resource failed",
            },
            ErrorTestCase {
                status_code: 429,
                expected_variant: |e| matches!(e, NHLApiError::RateLimitExceeded { .. }),
                expected_message_contains: "Request to /test/resource failed",
            },
            ErrorTestCase {
                status_code: 400,
                expected_variant: |e| matches!(e, NHLApiError::BadRequest { .. }),
                expected_message_contains: "Request to /test/resource failed",
            },
            ErrorTestCase {
                status_code: 401,
                expected_variant: |e| matches!(e, NHLApiError::Unauthorized { .. }),
                expected_message_contains: "Request to /test/resource failed",
            },
            // Server error range (500-599)
            ErrorTestCase {
                status_code: 500,
                expected_variant: |e| matches!(e, NHLApiError::ServerError { .. }),
                expected_message_contains: "Request to /test/resource failed",
            },
            ErrorTestCase {
                status_code: 502,
                expected_variant: |e| matches!(e, NHLApiError::ServerError { .. }),
                expected_message_contains: "Request to /test/resource failed",
            },
            ErrorTestCase {
                status_code: 503,
                expected_variant: |e| matches!(e, NHLApiError::ServerError { .. }),
                expected_message_contains: "Request to /test/resource failed",
            },
            ErrorTestCase {
                status_code: 599,
                expected_variant: |e| matches!(e, NHLApiError::ServerError { .. }),
                expected_message_contains: "Request to /test/resource failed",
            },
            // Catch-all API errors
            ErrorTestCase {
                status_code: 418,
                expected_variant: |e| matches!(e, NHLApiError::ApiError { .. }),
                expected_message_contains: "Unexpected error",
            },
            ErrorTestCase {
                status_code: 402,
                expected_variant: |e| matches!(e, NHLApiError::ApiError { .. }),
                expected_message_contains: "Unexpected error",
            },
            ErrorTestCase {
                status_code: 403,
                expected_variant: |e| matches!(e, NHLApiError::ApiError { .. }),
                expected_message_contains: "Unexpected error",
            },
            ErrorTestCase {
                status_code: 600,
                expected_variant: |e| matches!(e, NHLApiError::ApiError { .. }),
                expected_message_contains: "Unexpected error",
            },
            ErrorTestCase {
                status_code: 100,
                expected_variant: |e| matches!(e, NHLApiError::ApiError { .. }),
                expected_message_contains: "Unexpected error",
            },
            ErrorTestCase {
                status_code: 300,
                expected_variant: |e| matches!(e, NHLApiError::ApiError { .. }),
                expected_message_contains: "Unexpected error",
            },
        ];

        for test_case in &test_cases {
            assert_error_matches(
                test_case.status_code,
                test_case.expected_variant,
                test_case.expected_message_contains,
            );
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
        struct TestResponse {}

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
        struct TestResponse {}

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
        struct TestResponse {}

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
    async fn test_handle_response_error_delegation() {
        // Single test to verify handle_response delegates to error_from_status correctly
        // Detailed error mapping is tested in test_error_from_status_* tests
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
        assert!(result.is_err(), "Expected error response for 404 status");

        // Verify it's the right error type (details are tested in error_from_status tests)
        assert!(
            matches!(result.unwrap_err(), NHLApiError::ResourceNotFound { .. }),
            "Expected ResourceNotFound error"
        );
    }
}
