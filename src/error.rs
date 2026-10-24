use thiserror::Error;

/// Errors returned by this crate's HTTP calls and response deserialization.
///
/// `#[non_exhaustive]`: new variants (and new status-code mappings) may be
/// added without that being a breaking change, so a `match` on this type
/// must always carry a wildcard arm.
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum NHLApiError {
    /// The NHL API returned `404 Not Found` for the requested resource.
    #[error("Resource not found: {message}")]
    ResourceNotFound { message: String, status_code: u16 },

    /// The NHL API returned `429 Too Many Requests`.
    #[error("Rate limit exceeded: {message}")]
    RateLimitExceeded { message: String, status_code: u16 },

    /// The NHL API returned a `5xx` server error.
    #[error("Server error: {message}")]
    ServerError { message: String, status_code: u16 },

    /// The NHL API returned `400 Bad Request`.
    #[error("Bad request: {message}")]
    BadRequest { message: String, status_code: u16 },

    /// The NHL API returned `401 Unauthorized`.
    #[error("Unauthorized: {message}")]
    Unauthorized { message: String, status_code: u16 },

    /// The NHL API returned `403 Forbidden`.
    #[error("Forbidden: {message}")]
    Forbidden { message: String, status_code: u16 },

    /// The NHL API returned a non-2xx status code not covered by a more
    /// specific variant above.
    #[error("NHL API error: {message}")]
    ApiError { message: String, status_code: u16 },

    /// The request itself failed at the transport level (connection,
    /// timeout, TLS, ...) or while reading the response body, even after a 2xx status.
    #[error("HTTP request failed: {0}")]
    RequestError(#[from] reqwest::Error),

    /// The response body could not be deserialized into the expected type.
    #[error("unmarshaling response from {url}: {source}")]
    JsonError {
        url: String,
        #[source]
        source: serde_json::Error,
    },

    /// Any other error not tied to an API response, e.g. an invalid season id
    /// passed by the caller or a failure building the HTTP client.
    #[error("{0}")]
    Other(String),
}

impl NHLApiError {
    /// The HTTP status code the NHL API returned, if this error came from a
    /// non-2xx response. `None` for transport-level failures
    /// ([`NHLApiError::RequestError`]), deserialization failures
    /// ([`NHLApiError::JsonError`]), and [`NHLApiError::Other`].
    pub fn status_code(&self) -> Option<u16> {
        match self {
            NHLApiError::ResourceNotFound { status_code, .. }
            | NHLApiError::RateLimitExceeded { status_code, .. }
            | NHLApiError::ServerError { status_code, .. }
            | NHLApiError::BadRequest { status_code, .. }
            | NHLApiError::Unauthorized { status_code, .. }
            | NHLApiError::Forbidden { status_code, .. }
            | NHLApiError::ApiError { status_code, .. } => Some(*status_code),
            NHLApiError::RequestError(_)
            | NHLApiError::JsonError { .. }
            | NHLApiError::Other(_) => None,
        }
    }
}
