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