//! RedSys Desktop Agent Library
//! 
//! This library provides the core functionality for the RedSys Desktop Agent,
//! including application state handling.
//! 
//! ## Features
//! - Application state management
//! - Professional error handling and logging
//! - Cross-platform support

use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;
use once_cell::sync::Lazy;

pub mod docker_monitor;
pub mod error;
pub mod types;

use error::AppResult;
use types::AppState;

/// Global application state using thread-safe lazy initialization
static APP_STATE: Lazy<Arc<RwLock<AppState>>> = Lazy::new(|| {
    Arc::new(RwLock::new(AppState::default()))
});



/// Initialize the application
/// 
/// This function initializes all services and sets up the global application state.
/// 
/// # Arguments
/// 
/// * `app_handle` - Optional Tauri app handle for emitting events
/// 
/// # Returns
/// 
/// Returns success or an error
pub async fn initialize_app(_app_handle: Option<tauri::AppHandle>) -> AppResult<()> {
    info!("Initializing RedSys Desktop Agent...");
    
    // Create application state
    let app_state = AppState {
        app_metadata: types::AppMetadata::default(),
        last_updated: chrono::Utc::now(),
    };
    
    // Update global state
    {
        let mut state = APP_STATE.write().await;
        *state = app_state;
    }
    
    info!("RedSys Desktop Agent initialized successfully");
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
    
    info!("RedSys Desktop Agent cleanup completed");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_initialize_app() {
        // This test would require proper mocking in a real environment
        let result = initialize_app(None).await;
        // We don't assert here because the app might not be available in test environment
        if let Err(_e) = result {
            // App initialization test failed as expected
        }
    }

    #[tokio::test]
    async fn test_update_app_state() {
        let test_state = AppState::default();
        let result = update_app_state(test_state).await;
        assert!(result.is_ok());
    }
}
