use thiserror::Error;

#[derive(Error, Debug)]
pub enum B2Error {
    #[error("HTTP request failed: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("File operation failed: {0}")]
    FileError(#[from] std::io::Error),

    #[error("Authentication failed: {0}")]
    AuthError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Upload failed: {status_code} - {message}")]
    UploadError { status_code: u16, message: String },

    #[error("JSON serialization/deserialization failed: {0}")]
    JsonError(#[from] serde_json::Error),
}
