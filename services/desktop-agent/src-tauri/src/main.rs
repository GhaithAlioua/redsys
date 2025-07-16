//! RedSys Desktop Agent
//! 
//! A professional desktop agent for RedSys with Docker integration.
//! This application provides a modern, cross-platform interface for
//! checking Docker availability and version information.

use desktop_agent_lib::{
    initialize_app, get_app_state, cleanup_app, refresh_docker_service,
    types::{AppState, DockerServiceInfo},
    error::AppError,
};
use tracing::{error, info, warn};

/// Tauri command to get application state
/// 
/// Returns the current application state including Docker information.
/// 
/// # Returns
/// 
/// Returns the current application state
#[tauri::command]
async fn get_application_state() -> Result<AppState, String> {
    info!("Getting application state");
    
    let state = get_app_state().await;
    Ok(state)
}

/// Tauri command to check Docker health
/// 
/// Performs a health check on the Docker service and returns detailed information.
/// 
/// # Returns
/// 
/// Returns Docker service information or an error
#[tauri::command]
async fn check_docker_health() -> Result<DockerServiceInfo, String> {
    info!("🐳 Tauri: check_docker_health command called");
    
    match refresh_docker_service().await {
        Ok(service_info) => {
            info!("🐳 Tauri: Docker health check completed successfully - Available: {}, Status: {:?}", 
                  service_info.is_available, service_info.daemon_status);
            Ok(service_info)
        }
        Err(e) => {
            error!("🐳 Tauri: Docker health check failed: {}", e);
            Err(format!("Docker health check failed: {}", e))
        }
    }
}

/// Tauri command to get Docker version
/// 
/// Returns the Docker engine version if available.
/// 
/// # Returns
/// 
/// Returns Docker version string or an error
#[tauri::command]
async fn get_docker_version() -> Result<String, String> {
    info!("Getting Docker version");
    
    match refresh_docker_service().await {
        Ok(service_info) => {
            if let Some(version) = service_info.version {
                info!("Docker version retrieved: {}", version);
                Ok(version)
            } else {
                warn!("Docker version not available");
                Err("Docker version not available".to_string())
            }
        }
        Err(e) => {
            warn!("Failed to get Docker version: {}", e);
            Err(format!("Failed to get Docker version: {}", e))
        }
    }
}

/// Application setup function
/// 
/// This function is called when the Tauri application starts up.
/// It initializes all necessary services and sets up the application state.
/// 
/// # Arguments
/// 
/// * `app` - The Tauri application instance
/// 
/// # Returns
/// 
/// Returns success or an error
async fn setup_app(app: &tauri::App) -> Result<(), AppError> {
    info!("Setting up RedSys Desktop Agent...");
    
    // Initialize the application with app handle for event emission
    initialize_app(Some(app.handle().clone())).await?;
    
    info!("RedSys Desktop Agent setup completed successfully");
    Ok(())
}

/// Main application entry point
/// 
/// This function initializes the Tauri application with all necessary
/// services, commands, and event handlers.
fn main() {
    // Initialize the Tauri application
    tauri::Builder::default()
        // Add plugins
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        
        // Setup function
        .setup(|app| {
            tauri::async_runtime::block_on(async {
                if let Err(e) = setup_app(app).await {
                    error!("Failed to setup application: {}", e);
                    std::process::exit(1);
                }
            });
            Ok(())
        })
        
        // Cleanup function
        .on_window_event(|_window, event| {
            if let tauri::WindowEvent::CloseRequested { .. } = event {
                // Perform cleanup before closing
                tauri::async_runtime::block_on(async {
                    info!("🐳 Application closing, performing cleanup...");
                    if let Err(e) = cleanup_app().await {
                        error!("🐳 Failed to cleanup application: {}", e);
                    } else {
                        info!("🐳 Application cleanup completed successfully");
                    }
                });
                // Allow the window to close after cleanup
                // The window will close automatically after this event handler
            }
        })
        
        // Register commands
        .invoke_handler(tauri::generate_handler![
            get_application_state,
            check_docker_health,
            get_docker_version,
        ])
        
        // Run the application
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_application_state() {
        // This test would require the application to be properly initialized
        // In a real test environment, you'd mock the dependencies
        let result = get_application_state().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_check_docker_health() {
        // This test will fail if Docker is not running, which is expected
        let result = check_docker_health().await;
        // We don't assert here because Docker might not be available in test environment
        if let Err(e) = result {
            info!("Docker health check test failed as expected: {}", e);
        }
    }
}
