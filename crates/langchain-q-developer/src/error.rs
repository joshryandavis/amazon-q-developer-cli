//! Error types for the LangChain Q Developer provider

use thiserror::Error;

pub type Result<T> = std::result::Result<T, QDeveloperError>;

#[derive(Debug, Error)]
pub enum QDeveloperError {
    #[error("Failed to initialize Q Developer client: {0}")]
    InitializationError(String),

    #[error("Authentication error: {0}")]
    AuthError(String),

    #[error("API request failed: {0}")]
    ApiError(String),

    #[error("Streaming error: {0}")]
    StreamError(String),

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Invalid configuration: {0}")]
    ConfigError(String),

    #[error("Model not found: {0}")]
    ModelNotFound(String),

    #[error("Conversation error: {0}")]
    ConversationError(String),

    #[error(transparent)]
    Other(#[from] Box<dyn std::error::Error + Send + Sync>),
}

impl QDeveloperError {
    pub fn api<E: std::fmt::Display>(error: E) -> Self {
        Self::ApiError(error.to_string())
    }

    pub fn auth<E: std::fmt::Display>(error: E) -> Self {
        Self::AuthError(error.to_string())
    }

    pub fn stream<E: std::fmt::Display>(error: E) -> Self {
        Self::StreamError(error.to_string())
    }

    pub fn init<E: std::fmt::Display>(error: E) -> Self {
        Self::InitializationError(error.to_string())
    }

    pub fn db<E: std::fmt::Display>(error: E) -> Self {
        Self::DatabaseError(error.to_string())
    }

    pub fn config<E: std::fmt::Display>(error: E) -> Self {
        Self::ConfigError(error.to_string())
    }
}
