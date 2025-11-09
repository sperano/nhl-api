use thiserror::Error;

#[derive(Error, Debug)]
pub enum NHLApiError {
    #[error("Resource not found: {message}")]
    ResourceNotFound { message: String, status_code: u16 },

    #[error("Rate limit exceeded: {message}")]
    RateLimitExceeded { message: String, status_code: u16 },

    #[error("Server error: {message}")]
    ServerError { message: String, status_code: u16 },

    #[error("Bad request: {message}")]
    BadRequest { message: String, status_code: u16 },

    #[error("Unauthorized: {message}")]
    Unauthorized { message: String, status_code: u16 },

    #[error("NHL API error: {message}")]
    ApiError {
        message: String,
        status_code: u16,
    },

    #[error("HTTP request failed: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("JSON parsing failed: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("{0}")]
    Other(String),
}
