use thiserror::Error;

/// Errors returned by the Cited Health API client.
#[derive(Debug, Error)]
pub enum CitedHealthError {
    /// Network or transport error from reqwest.
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    /// The requested resource was not found (HTTP 404).
    #[error("Resource not found: {resource}")]
    NotFound {
        /// Description of the resource that was not found.
        resource: String,
    },

    /// Rate limit exceeded (HTTP 429).
    #[error("Rate limit exceeded, retry after {retry_after}s")]
    RateLimit {
        /// Seconds to wait before retrying.
        retry_after: u64,
    },

    /// Other API error with HTTP status code.
    #[error("API error (HTTP {status}): {message}")]
    Api {
        /// HTTP status code.
        status: u16,
        /// Error message or response body.
        message: String,
    },
}
