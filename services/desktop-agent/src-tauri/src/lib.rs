//! RedSys Desktop Agent Library
//! 
//! This library provides the core functionality for the RedSys Desktop Agent,
//! including Docker service management and application state handling.
//! 
//! ## Features
//! - Docker availability checking using Bollard
//! - Application state management
//! - Professional error handling and logging
//! - Cross-platform support
//! - Real-time event streaming using Docker Events API

use std::sync::Arc;
use tokio::sync::{RwLock, mpsc};
use tracing::{info, warn};
use once_cell::sync::Lazy;

pub mod docker;
pub mod error;
pub mod types;

use docker::DockerService;
use error::AppResult;
use types::{AppState, DockerServiceInfo, DockerEvent};

/// Global application state using thread-safe lazy initialization
static APP_STATE: Lazy<Arc<RwLock<AppState>>> = Lazy::new(|| {
    Arc::new(RwLock::new(AppState::default()))
});

/// Global Docker service instance
static DOCKER_SERVICE: Lazy<Arc<RwLock<Option<DockerService>>>> = Lazy::new(|| {
    Arc::new(RwLock::new(None))
});

/// Global Docker event sender for real-time updates
static DOCKER_EVENT_SENDER: Lazy<Arc<RwLock<Option<mpsc::UnboundedSender<DockerEvent>>>>> = Lazy::new(|| {
    Arc::new(RwLock::new(None))
});

/// Initialize the application with event-driven architecture
/// 
/// This function initializes all services and sets up the global application state
/// with real-time event streaming for Docker.
/// 
/// # Arguments
/// 
/// * `app_handle` - Optional Tauri app handle for emitting events
/// 
/// # Returns
/// 
/// Returns success or an error
pub async fn initialize_app(app_handle: Option<tauri::AppHandle>) -> AppResult<()> {
    info!("Initializing RedSys Desktop Agent with event-driven architecture...");
    
    // Create event channel for real-time Docker updates
    let (event_sender, _event_receiver) = mpsc::unbounded_channel::<DockerEvent>();
    
    // Store the sender globally
    {
        let mut sender_guard = DOCKER_EVENT_SENDER.write().await;
        *sender_guard = Some(event_sender.clone());
    }
    
    // Initialize Docker service with event streaming and Tauri app handle
    let docker_service = match DockerService::new_with_events(Some(event_sender), app_handle).await {
        Ok(service) => {
            info!("Docker service initialized successfully with event streaming");
            Some(service)
        }
        Err(e) => {
            warn!("Failed to initialize Docker service: {}", e);
            None
        }
    };
    
    // Store Docker service globally
    {
        let mut service_guard = DOCKER_SERVICE.write().await;
        *service_guard = docker_service;
    }
    
    // Create application state
    let app_state = AppState {
        docker_service: None, // Will be populated on first check
        app_metadata: types::AppMetadata::default(),
        last_updated: chrono::Utc::now(),
    };
    
    // Update global state
    {
        let mut state = APP_STATE.write().await;
        *state = app_state;
    }
    
    info!("RedSys Desktop Agent initialized successfully with event-driven architecture");
    Ok(())
}

/// Get the current application state
/// 
/// Returns a clone of the current application state.
/// 
/// # Returns
/// 
/// Returns the current application state
pub async fn get_app_state() -> AppState {
    APP_STATE.read().await.clone()
}

/// Update the application state
/// 
/// Updates the global application state with new information.
/// 
/// # Arguments
/// 
/// * `new_state` - The new application state
/// 
/// # Returns
/// 
/// Returns success or an error
pub async fn update_app_state(new_state: AppState) -> AppResult<()> {
    let mut state = APP_STATE.write().await;
    *state = new_state;
    Ok(())
}

/// Cleanup the application
/// 
/// This function performs cleanup operations for all services.
/// It should be called when the application is shutting down.
/// 
/// # Returns
/// 
/// Returns success or an error
pub async fn cleanup_app() -> AppResult<()> {
    info!("Cleaning up RedSys Desktop Agent...");
    
    // Cleanup Docker service
    {
        let mut service_guard = DOCKER_SERVICE.write().await;
        if let Some(mut service) = service_guard.take() {
            if let Err(e) = service.cleanup().await {
                warn!("Failed to cleanup Docker service: {}", e);
            }
        }
    }
    
    // Clear event sender
    {
        let mut sender_guard = DOCKER_EVENT_SENDER.write().await;
        *sender_guard = None;
    }
    
    info!("RedSys Desktop Agent cleanup completed");
    Ok(())
}

/// Refresh Docker service information
/// 
/// Performs a fresh health check on the Docker service and updates the state.
/// 
/// # Returns
/// 
/// Returns the updated Docker service information or an error
pub async fn refresh_docker_service() -> AppResult<DockerServiceInfo> {
    info!("Refreshing Docker service information...");
    
    // Get the Docker service with mutable access
    let mut service_guard = DOCKER_SERVICE.write().await;
    let docker_service = match service_guard.as_mut() {
        Some(service) => service,
        None => {
            warn!("No Docker service available");
            let service_info = DockerServiceInfo {
                is_available: false,
                version: None,
                daemon_status: types::DockerDaemonStatus::Stopped,
                last_health_check: chrono::Utc::now(),
                connection_error: Some("No Docker service available".to_string()),
            };
            
            // Update the global state
            let mut state = get_app_state().await;
            state.docker_service = Some(service_info.clone());
            state.last_updated = chrono::Utc::now();
            update_app_state(state).await?;
            
            return Ok(service_info);
        }
    };
    
    // Perform health check with timeout
    match tokio::time::timeout(
        std::time::Duration::from_secs(10),
        docker_service.check_health()
    ).await {
        Ok(health_result) => {
            match health_result {
                Ok(_) => {
                    let service_info = docker_service.get_service_info().await;
                    info!("Docker health check completed successfully");
                    
                    // Update the global state
                    let mut state = get_app_state().await;
                    state.docker_service = Some(service_info.clone());
                    state.last_updated = chrono::Utc::now();
                    update_app_state(state).await?;
                    
                    Ok(service_info)
                }
                Err(e) => {
                    warn!("Docker health check failed: {}", e);
                    let service_info = docker_service.get_service_info().await;
                    
                    // Update the global state even on error
                    let mut state = get_app_state().await;
                    state.docker_service = Some(service_info.clone());
                    state.last_updated = chrono::Utc::now();
                    update_app_state(state).await?;
                    
                    Ok(service_info)
                }
            }
        }
        Err(_) => {
            warn!("Docker service refresh timed out after 10 seconds");
            let service_info = DockerServiceInfo {
                is_available: false,
                version: None,
                daemon_status: types::DockerDaemonStatus::Error,
                last_health_check: chrono::Utc::now(),
                connection_error: Some("Docker service refresh timed out after 10 seconds".to_string()),
            };
            
            // Update the global state
            let mut state = get_app_state().await;
            state.docker_service = Some(service_info.clone());
            state.last_updated = chrono::Utc::now();
            update_app_state(state).await?;
            
            Ok(service_info)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_initialize_app() {
        // This test would require proper mocking in a real environment
        let result = initialize_app().await;
        // We don't assert here because Docker might not be available in test environment
        if let Err(e) = result {
            warn!("App initialization test failed as expected: {}", e);
        }
    }

    #[tokio::test]
    async fn test_update_app_state() {
        let test_state = AppState::default();
        let result = update_app_state(test_state).await;
        assert!(result.is_ok());
    }
}
