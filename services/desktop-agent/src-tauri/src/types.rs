//! Type definitions for RedSys Desktop Agent
//!
//! This module contains all the type definitions used throughout the application,
//! including application state.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Application state
///
/// This struct holds the global state of the application, including
/// runtime information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppState {
    /// Application metadata
    pub app_metadata: AppMetadata,

    /// Last update timestamp
    pub last_updated: DateTime<Utc>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            app_metadata: AppMetadata::default(),
            last_updated: Utc::now(),
        }
    }
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
        assert_eq!(state.app_metadata.name, "RedSys Desktop Agent");
    }
}
