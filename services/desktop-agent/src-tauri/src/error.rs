//! Error handling for RedSys Desktop Agent
//!
//! This module provides comprehensive error handling for the application,
//! including Docker-specific errors and general application errors.

use thiserror::Error;

/// Application error types
#[derive(Error, Debug)]
pub enum AppError {
    /// Docker-related errors
    #[error("Docker error: {0}")]
    Docker(#[from] DockerError),

    /// Application not initialized error
    #[error("Application not initialized")]
    ApplicationNotInitialized,

    /// General error with message
    #[error("Application error: {message}")]
    General { message: String },

    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Serialization error
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// System error
    #[error("System error: {0}")]
    System(String),
}

/// Docker-specific error types
#[derive(Error, Debug)]
pub enum DockerError {
    /// Docker daemon is not running
    #[error("Docker daemon is not running")]
    DaemonNotRunning,

    /// Docker command failed
    #[error("Docker command failed: {command}")]
    CommandFailed { command: String },

    /// Docker connection failed
    #[error("Docker connection failed: {0}")]
    ConnectionFailed(String),

    /// Docker container not found
    #[error("Docker container not found: {container_id}")]
    ContainerNotFound { container_id: String },

    /// Docker image not found
    #[error("Docker image not found: {image_name}")]
    ImageNotFound { image_name: String },

    /// Docker permission denied
    #[error("Docker permission denied: {0}")]
    PermissionDenied(String),

    /// Docker timeout
    #[error("Docker operation timed out: {operation}")]
    Timeout { operation: String },

    /// Docker resource limit exceeded
    #[error("Docker resource limit exceeded: {resource}")]
    ResourceLimitExceeded { resource: String },

    /// Docker network error
    #[error("Docker network error: {0}")]
    NetworkError(String),

    /// Docker configuration error
    #[error("Docker configuration error: {0}")]
    ConfigurationError(String),
}

/// Result type for application operations
pub type AppResult<T> = Result<T, AppError>;

impl From<String> for AppError {
    fn from(message: String) -> Self {
        AppError::General { message }
    }
}

impl From<&str> for AppError {
    fn from(message: &str) -> Self {
        AppError::General {
            message: message.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_error_from_string() {
        let error: AppError = "Test error".into();
        assert!(matches!(error, AppError::General { message } if message == "Test error"));
    }

    #[test]
    fn test_docker_error_display() {
        let error = DockerError::DaemonNotRunning;
        assert_eq!(error.to_string(), "Docker daemon is not running");
    }
}
