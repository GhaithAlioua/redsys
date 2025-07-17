//! Error handling for RedSys Desktop Agent
//!
//! This module provides comprehensive error handling for the application,
//! including general application errors.

use thiserror::Error;

/// Application result type
pub type AppResult<T> = Result<T, AppError>;

/// Main application error type
#[derive(Error, Debug)]
pub enum AppError {
    /// General application error
    #[error("Application error: {0}")]
    Application(String),

    /// Configuration error
    #[error("Configuration error: {0}")]
    Configuration(String),

    /// Network error
    #[error("Network error: {0}")]
    Network(String),

    /// Timeout error
    #[error("Operation timed out: {operation}")]
    Timeout { operation: String },

    /// Permission error
    #[error("Permission denied: {0}")]
    Permission(String),

    /// Resource not found
    #[error("Resource not found: {resource}")]
    NotFound { resource: String },

    /// Invalid state error
    #[error("Invalid state: {0}")]
    InvalidState(String),

    /// Serialization error
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

impl From<String> for AppError {
    fn from(err: String) -> Self {
        AppError::Application(err)
    }
}

impl From<&str> for AppError {
    fn from(err: &str) -> Self {
        AppError::Application(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_error_display() {
        let error = AppError::Application("test error".to_string());
        assert_eq!(error.to_string(), "Application error: test error");
    }

    #[test]
    fn test_timeout_error() {
        let error = AppError::Timeout {
            operation: "test".to_string(),
        };
        assert_eq!(error.to_string(), "Operation timed out: test");
    }
}
