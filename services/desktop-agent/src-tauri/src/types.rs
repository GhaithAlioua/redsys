//! Type definitions for RedSys Desktop Agent
//!
//! This module contains all the type definitions used throughout the application,
//! including Docker service information and application state.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Application state
///
/// This struct holds the global state of the application, including
/// service instances and runtime information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppState {
    /// Docker service instance
    pub docker_service: Option<DockerServiceInfo>,

    /// Application metadata
    pub app_metadata: AppMetadata,

    /// Last update timestamp
    pub last_updated: DateTime<Utc>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            docker_service: None,
            app_metadata: AppMetadata::default(),
            last_updated: Utc::now(),
        }
    }
}

/// Docker service information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DockerServiceInfo {
    /// Whether Docker is available
    pub is_available: bool,

    /// Docker version
    pub version: Option<String>,

    /// Docker daemon status
    pub daemon_status: DockerDaemonStatus,

    /// Last health check
    pub last_health_check: DateTime<Utc>,

    /// Connection error message (if any)
    pub connection_error: Option<String>,
}

/// Docker daemon status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DockerDaemonStatus {
    /// Daemon is running
    Running,

    /// Daemon is stopped
    Stopped,

    /// Daemon status is unknown
    Unknown,

    /// Error occurred while checking status
    Error,
}

/// Docker event for real-time updates
///
/// This struct represents a Docker daemon event that can be streamed
/// to the frontend for real-time updates.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DockerEvent {
    /// Type of event (container, image, volume, network, etc.)
    pub event_type: String,

    /// Action performed (create, start, stop, delete, etc.)
    pub action: String,

    /// ID of the affected resource
    pub actor_id: Option<String>,

    /// Additional attributes of the event
    pub actor_attributes: HashMap<String, String>,

    /// Timestamp when the event occurred
    pub timestamp: DateTime<Utc>,
}

/// Application metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppMetadata {
    /// Application name
    pub name: String,

    /// Application version
    pub version: String,

    /// Application description
    pub description: String,

    /// Build timestamp
    pub build_timestamp: DateTime<Utc>,
}

impl Default for AppMetadata {
    fn default() -> Self {
        Self {
            name: "RedSys Desktop Agent".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            description: "Professional desktop agent for RedSys".to_string(),
            build_timestamp: Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_state_default() {
        let state = AppState::default();
        assert!(state.docker_service.is_none());
        assert_eq!(state.app_metadata.name, "RedSys Desktop Agent");
    }

    #[test]
    fn test_docker_daemon_status_serialization() {
        let status = DockerDaemonStatus::Running;
        let serialized = serde_json::to_string(&status).unwrap();
        let deserialized: DockerDaemonStatus = serde_json::from_str(&serialized).unwrap();
        assert!(matches!(deserialized, DockerDaemonStatus::Running));
    }

    #[test]
    fn test_docker_event_creation() {
        let mut attributes = HashMap::new();
        attributes.insert("name".to_string(), "test-container".to_string());

        let event = DockerEvent {
            event_type: "container".to_string(),
            action: "start".to_string(),
            actor_id: Some("abc123".to_string()),
            actor_attributes: attributes,
            timestamp: Utc::now(),
        };

        assert_eq!(event.event_type, "container");
        assert_eq!(event.action, "start");
        assert_eq!(event.actor_id, Some("abc123".to_string()));
    }
}
