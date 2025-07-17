//! RedSys Desktop Agent
//! 
//! A professional desktop agent for RedSys with Docker daemon monitoring.
//! This application provides a modern, cross-platform interface for
//! monitoring Docker daemon status and system resources.

use desktop_agent_lib::{
    initialize_app, get_app_state, cleanup_app,
    types::AppState,
    error::AppError,
};
use desktop_agent_lib::docker_monitor::{DockerMonitor, DockerStatus};
use std::sync::Arc;
use tokio_util::sync::CancellationToken;
use tokio::time::{sleep, Duration};
use tracing::{error, info};
use tauri::Manager;
use tauri::Listener;

/// Tauri command to get application state
/// 
/// Returns the current application state.
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

/// Tauri command to get Docker daemon status
/// 
/// Returns the current Docker daemon status without performing a new check.
/// 
/// # Returns
/// 
/// Returns Docker status information or an error
#[tauri::command]
async fn get_docker_status(state: tauri::State<'_, Arc<DockerMonitor>>) -> Result<DockerStatus, String> {
    info!("Getting Docker daemon status");
    
    match state.get_current_status().await {
        status => {
            info!("Docker status retrieved successfully: {:?}", status);
            Ok(status)
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
/// * `app_handle` - The Tauri application handle
/// 
/// # Returns
/// 
/// Returns success or an error
async fn setup_app(app_handle: &tauri::AppHandle) -> Result<(), AppError> {
    info!("Setting up RedSys Desktop Agent with Docker monitoring...");
    
    // Initialize the application with app handle for event emission
    initialize_app(Some(app_handle.clone())).await?;
    
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
            // Show the window immediately when app is ready
            let window = app.get_webview_window("main").unwrap();
            window.show().unwrap();
            
            // Initialize Docker monitor
            let cancellation_token = CancellationToken::new();
            let docker_monitor = Arc::new(DockerMonitor::new(cancellation_token.clone()));
            
            // Start Docker monitoring in background
            let docker_monitor_clone = docker_monitor.clone();
            let app_handle = app.handle().clone();
            // Use Tauri's async runtime for background tasks (official best practice)
            tauri::async_runtime::spawn(async move {
                docker_monitor_clone.start_monitoring(app_handle).await;
            });
            
            // Store Docker monitor in app state
            app.manage(docker_monitor);
            
            // Initialize app in background with minimal delay
            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                // Small delay to ensure UI is fully loaded
                sleep(Duration::from_millis(100)).await;
                
                if let Err(e) = setup_app(&app_handle).await {
                    error!("Failed to setup application: {}", e);
                    // Don't exit the process, just log the error
                }
            });
            
            // Setup graceful shutdown
            let cancellation_token_clone = cancellation_token.clone();
            app.listen("tauri://close-requested", move |_| {
                info!("Application closing, cancelling Docker monitor");
                cancellation_token_clone.cancel();
            });
            
            Ok(())
        })
        
        // Cleanup function
        .on_window_event(|_window, event| {
            if let tauri::WindowEvent::CloseRequested { .. } = event {
                // Perform cleanup before closing
                tauri::async_runtime::block_on(async {
                    info!("Application closing, performing cleanup...");
                    if let Err(e) = cleanup_app().await {
                        error!("Failed to cleanup application: {}", e);
                    } else {
                        info!("Application cleanup completed successfully");
                    }
                });
                // Allow the window to close after cleanup
                // The window will close automatically after this event handler
            }
        })
        
        // Register commands
        .invoke_handler(tauri::generate_handler![
            get_application_state,
            get_docker_status,
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
}
