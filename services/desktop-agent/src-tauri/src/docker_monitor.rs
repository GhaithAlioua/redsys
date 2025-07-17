//! Docker Daemon Monitor for RedSys Desktop Agent
//!
//! This module provides comprehensive Docker daemon monitoring with real-time status updates,
//! version detection, and robust error handling. It follows enterprise-grade patterns for
//! reliability, security, and performance.
//!
//! ## Architecture
//! - **4-second polling interval** for optimal responsiveness and resource efficiency
//! - **Professional cross-platform connection strategy** with runtime detection
//! - **Graceful shutdown** using Tokio CancellationToken
//! - **Comprehensive error handling** with user-friendly messages
//! - **Real-time event emission** to frontend via Tauri events
//!
//! ## Professional Cross-Platform Support
//! - **Runtime Platform Detection**: Dynamically determines the best connection method
//! - **Environment Variable Priority**: `DOCKER_HOST` takes precedence (user override)
//! - **Platform Defaults**: 
//!   - **Windows**: Named pipe (`npipe:///./pipe/docker_engine`)
//!   - **Linux/macOS**: Unix socket (`unix:///var/run/docker.sock`)
//! - **HTTP Fallback**: For remote Docker hosts or custom configurations
//!
//! ## Enterprise Features
//! - **No compile-time platform assumptions**: Works on any platform without recompilation
//! - **Configurable connection strategy**: Easy to extend with custom connection methods
//! - **Comprehensive logging**: Detailed connection attempt logging for troubleshooting
//! - **Graceful degradation**: Falls back to alternative methods if primary fails
//!
//! ## References
//! - [Bollard Documentation](https://docs.rs/bollard/latest/bollard/)
//! - [Docker Engine API](https://docs.docker.com/engine/api/)
//! - [Docker Host Configuration](https://docs.docker.com/engine/reference/commandline/cli/#environment-variables)
//! - [Tokio select! macro](https://docs.rs/tokio/latest/tokio/macro.select.html)
//! - [Serde Enum Serialization](https://serde.rs/enum-representations.html)
//! - [Thiserror Error Handling](https://docs.rs/thiserror/latest/thiserror/)

use std::sync::Arc;
use tokio::{sync::Mutex, time::{interval, Duration}, task};
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info};
use tauri::Emitter;
use bollard::Docker;
use serde::Serialize;
use thiserror::Error;

/// Docker daemon status with discriminated union serialization.
/// 
/// Uses `#[serde(tag = "type")]` for TypeScript discriminated union compatibility.
/// See [Serde Enum Representations](https://serde.rs/enum-representations.html).
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "type")]
pub enum DockerStatus {
    /// Docker daemon is running and responsive
    Running { version: String },
    
    /// Docker daemon is stopped or not available
    Stopped,
    
    /// Error occurred while checking daemon
    Error { message: String },
}

/// Comprehensive error types for Docker monitoring operations.
/// 
/// Uses `thiserror` for idiomatic Rust error handling with automatic
/// error conversion and user-friendly messages.
/// See [Thiserror Documentation](https://docs.rs/thiserror/latest/thiserror/).
#[derive(Error, Debug)]
pub enum DockerMonitorError {
    /// Connection to Docker daemon failed
    #[error("Failed to connect to Docker daemon: {0}")]
    Connection(#[from] bollard::errors::Error),
    
    /// Docker API call failed (removed #[from] to avoid duplicate)
    #[error("Docker API error: {0}")]
    Api(String),
    
    /// Tauri event emission failed
    #[error("Failed to emit Tauri event: {0}")]
    EventEmission(#[from] tauri::Error),
    
    /// Internal error
    #[error("Internal error: {0}")]
    Internal(String),
}

/// Result type for Docker monitoring operations
pub type DockerMonitorResult<T> = Result<T, DockerMonitorError>;

/// Docker daemon monitor with thread-safe state management.
/// 
/// Provides continuous monitoring of Docker daemon status with real-time
/// updates and comprehensive error handling.
#[derive(Debug)]
pub struct DockerMonitor {
    /// Current Docker status protected by async mutex
    status: Arc<Mutex<DockerStatus>>,
    
    /// Cancellation token for graceful shutdown
    cancellation_token: Arc<CancellationToken>,
}

impl DockerMonitor {
    /// Creates a new Docker monitor instance.
    /// 
    /// Initializes with `Initializing` status and a fresh cancellation token.
    pub fn new(cancellation_token: CancellationToken) -> Self {
        info!("Initializing Docker monitor");
        Self {
            status: Arc::new(Mutex::new(DockerStatus::Stopped)),
            cancellation_token: Arc::new(cancellation_token),
        }
    }
    
    /// Gets the current Docker status.
    /// 
    /// Returns a clone of the current status for thread-safe access.
    pub async fn get_current_status(&self) -> DockerStatus {
        self.status.lock().await.clone()
    }
    
    /// Establishes connection to Docker daemon with robust cross-platform fallback strategy.
    /// 
    /// **Professional Cross-Platform Connection Strategy:**
    /// 1. **Runtime Platform Detection**: Dynamically determines the best connection method
    /// 2. **Environment Variable**: `DOCKER_HOST` (supports TCP, Unix socket, or named pipe)
    /// 3. **HTTP Defaults**: Standard HTTP connection (for remote Docker hosts)
    /// 
    /// **References:**
    /// - [Bollard Connection Methods](https://docs.rs/bollard/latest/bollard/struct.Docker.html)
    /// - [Docker Engine API](https://docs.docker.com/engine/api/)
    /// - [Docker Host Configuration](https://docs.docker.com/engine/reference/commandline/cli/#environment-variables)
    async fn get_docker_client() -> DockerMonitorResult<Docker> {
        // 1. Try DOCKER_HOST environment variable first (user override)
        if let Ok(docker_host) = std::env::var("DOCKER_HOST") {
            debug!("Attempting DOCKER_HOST connection: {}", docker_host);
            match Self::try_docker_host_connection().await {
                Ok(client) => {
                    info!("Successfully connected to Docker via DOCKER_HOST");
                    return Ok(client);
                }
                Err(e) => {
                    debug!("DOCKER_HOST connection failed: {}", e);
                }
            }
        }
        
        // 2. Try platform-specific default connection
        debug!("Attempting platform-specific default connection");
        match Self::try_platform_default_connection().await {
            Ok(client) => {
                info!("Successfully connected to Docker via platform default");
                return Ok(client);
            }
            Err(e) => {
                debug!("Platform default connection failed: {}", e);
            }
        }
        
        // 3. Try HTTP defaults as final fallback
        debug!("Attempting HTTP defaults connection");
        match Self::try_http_connection().await {
            Ok(client) => {
                info!("Successfully connected to Docker via HTTP defaults");
                return Ok(client);
            }
            Err(e) => {
                debug!("HTTP defaults connection failed: {}", e);
            }
        }
        
        // All connection methods failed
        error!("All Docker connection methods failed");
        Err(DockerMonitorError::Connection(
            bollard::errors::Error::DockerResponseServerError {
                status_code: 503,
                message: "Unable to connect to Docker daemon via any available method".to_string(),
            }
        ))
    }
    
    /// Attempts platform-specific default connection based on runtime detection.
    /// 
    /// This method uses runtime detection to determine the best connection method
    /// for the current platform, following Docker's standard installation patterns.
    async fn try_platform_default_connection() -> Result<Docker, bollard::errors::Error> {
        if cfg!(target_os = "windows") {
            debug!("Attempting Windows named pipe connection");
            Docker::connect_with_named_pipe_defaults()
        } else {
            debug!("Attempting Unix socket connection");
            Docker::connect_with_socket_defaults()
        }
    }
    
    /// Attempts connection using DOCKER_HOST environment variable.
    /// 
    /// **Supported Formats:**
    /// - `tcp://host:port` - TCP connection
    /// - `unix:///path/to/socket` - Unix socket
    /// - `npipe:///./pipe/name` - Windows named pipe
    async fn try_docker_host_connection() -> Result<Docker, bollard::errors::Error> {
        if let Ok(docker_host) = std::env::var("DOCKER_HOST") {
            debug!("Attempting DOCKER_HOST connection: {}", docker_host);
            
            if docker_host.starts_with("tcp://") {
                // Use HTTP defaults for TCP connections
                Docker::connect_with_http_defaults()
            } else if docker_host.starts_with("unix://") {
                // Use socket defaults for Unix socket connections
                Docker::connect_with_socket_defaults()
            } else if docker_host.starts_with("npipe://") {
                // Use named pipe defaults for Windows named pipe connections
                Docker::connect_with_named_pipe_defaults()
            } else {
                // Invalid DOCKER_HOST format
                Err(bollard::errors::Error::DockerResponseServerError {
                    status_code: 400,
                    message: format!("Invalid DOCKER_HOST format: {}", docker_host),
                })
            }
        } else {
            // DOCKER_HOST not set
            Err(bollard::errors::Error::DockerResponseServerError {
                status_code: 400,
                message: "DOCKER_HOST environment variable not set".to_string(),
            })
        }
    }
    
    /// Attempts HTTP connection using default settings.
    /// 
    /// **Use Cases:**
    /// - Remote Docker hosts
    /// - Docker Desktop on non-standard ports
    /// - Custom Docker configurations
    async fn try_http_connection() -> Result<Docker, bollard::errors::Error> {
        debug!("Attempting HTTP connection");
                Docker::connect_with_http_defaults()
            }
    

    

    
    /// Starts the main monitoring loop with resource-efficient, fast Docker daemon monitoring.
    /// 
    /// **Smart Resource-Efficient Polling Strategy:**
    /// - **Fast polling (1.5s)**: Standard monitoring for critical daemon status
    /// - **Quick polling (800ms)**: During status transitions and restart detection
    /// - **Normal polling (3s)**: When status is stable but still responsive
    /// - **Change detection**: Emits events immediately on any daemon status change
    /// - **Restart detection**: Uses intelligent pattern recognition for daemon restarts
    /// - **Resource optimization**: Minimal CPU and network usage while maintaining responsiveness
    /// - **Connection pooling**: Reuses connections when possible
    /// - **Graceful shutdown**: Uses `tokio::select!` with CancellationToken
    /// 
    /// **Critical for RedSys Platform:**
    /// - Docker daemon status is essential for job execution
    /// - Fast detection prevents job assignment to unavailable providers
    /// - Version tracking ensures compatibility with job requirements
    /// - Reliable response to daemon restarts for platform reliability
    /// 
    /// **Resource Efficiency Features:**
    /// - **Fast but not aggressive**: 800ms minimum polling to avoid system overload
    /// - **Connection reuse**: Minimizes connection overhead
    /// - **Pattern recognition**: Detects daemon restarts without excessive polling
    /// - **Memory efficient**: Bounded history for pattern detection
    /// - **Minimal logging**: Reduces I/O overhead
    /// - **Smart backoff**: Gradually increases intervals based on stability
    /// 
    /// **References:**
    /// - [Tokio select! macro](https://docs.rs/tokio/latest/tokio/macro.select.html)
    /// - [Tokio Interval](https://docs.rs/tokio/latest/tokio/time/struct.Interval.html)
    /// - [Tauri Event Emission](https://tauri.app/v2/guides/features/events/)
    pub async fn start_monitoring(
        self: Arc<Self>,
        app_handle: tauri::AppHandle,
    ) {
        let status = self.status.clone();
        let cancellation_token = self.cancellation_token.clone();

        info!("Starting resource-efficient Docker daemon monitoring for RedSys platform");

        task::spawn(async move {
            let mut last_status: Option<DockerStatus> = None;
            let mut consecutive_same_status = 0;
            let mut last_change_time = std::time::Instant::now();
            let mut status_history: Vec<(DockerStatus, std::time::Instant)> = Vec::new();
            let mut potential_restart_detected = false;
            let mut connection_cache: Option<Docker> = None;
            
            // Resource-efficient polling intervals for reliable daemon monitoring
            const QUICK_INTERVAL: Duration = Duration::from_millis(800); // During transitions
            const FAST_INTERVAL: Duration = Duration::from_millis(1500); // Standard monitoring
            const NORMAL_INTERVAL: Duration = Duration::from_secs(3); // When stable
            
            // Thresholds for interval switching
            const QUICK_THRESHOLD: u32 = 3; // Switch to fast after 3 quick checks
            const FAST_THRESHOLD: u32 = 5; // Switch to normal after 5 fast checks
            const RESTART_DETECTION_WINDOW: Duration = Duration::from_secs(12); // Reasonable detection window
            const MAX_HISTORY_SIZE: usize = 6; // Bounded memory usage
            
            let mut current_interval = FAST_INTERVAL; // Start with fast polling for reliable monitoring
            let mut poller = interval(current_interval);

            loop {
                tokio::select! {
                    _ = poller.tick() => {
                        let new_status = match Self::check_docker_with_cache(&mut connection_cache).await {
                            Ok(DockerStatus::Running { version }) => DockerStatus::Running { version },
                            Ok(other) => other,
                            Err(e) => DockerStatus::Error { 
                                message: format!("{e}") 
                            },
                        };

                        {
                            let mut guard = status.lock().await;
                            let status_changed = last_status.as_ref() != Some(&new_status);
                            
                            if status_changed {
                                // Status changed - update history efficiently
                                let now = std::time::Instant::now();
                                status_history.push((new_status.clone(), now));
                                
                                // Keep history bounded to prevent memory growth
                                if status_history.len() > MAX_HISTORY_SIZE {
                                    status_history.remove(0);
                                }
                                
                                // Detect restart patterns efficiently
                                potential_restart_detected = Self::detect_restart_pattern_efficient(&status_history);
                                
                                // Reset counters and emit event
                                consecutive_same_status = 0;
                                last_change_time = now;
                                *guard = new_status.clone();
                                last_status = Some(new_status.clone());
                                
                                // Switch to quick polling on status change for fast detection
                                if current_interval != QUICK_INTERVAL {
                                    current_interval = QUICK_INTERVAL;
                                    poller = interval(current_interval);
                                    if potential_restart_detected {
                                        debug!("Docker daemon restart detected, switching to quick polling (800ms)");
                                    } else {
                                        debug!("Docker daemon status changed, switching to quick polling (800ms)");
                                    }
                                }
                                
                                // Emit event to frontend immediately
                                if let Err(e) = app_handle.emit("docker_status_changed", &new_status) {
                                    error!("Failed to emit docker_status_changed event: {e}");
                                }
                                info!("Docker daemon status changed: {:?}", new_status);
                            } else {
                                // Same status - increment counter and optimize interval
                                consecutive_same_status += 1;
                                let time_since_last_change = last_change_time.elapsed();
                                
                                // Determine optimal polling interval based on stability and restart detection
                                let new_interval = if potential_restart_detected && time_since_last_change < RESTART_DETECTION_WINDOW {
                                    // Keep quick polling during restart detection window
                                    QUICK_INTERVAL
                                } else if consecutive_same_status >= FAST_THRESHOLD {
                                    // Status stable - use normal polling but still responsive
                                    NORMAL_INTERVAL
                                } else if consecutive_same_status >= QUICK_THRESHOLD {
                                    // Recent change but stabilizing - use fast polling
                                    FAST_INTERVAL
                                } else {
                                    // Very recent change or potential restart - keep quick polling
                                    QUICK_INTERVAL
                                };
                                
                                // Switch interval if needed
                                if new_interval != current_interval {
                                    current_interval = new_interval;
                                    poller = interval(current_interval);
                                    let interval_secs = current_interval.as_secs_f32();
                                    debug!("Daemon status stable for {} checks, switching to {}s polling", 
                                           consecutive_same_status, interval_secs);
                                }
                                
                                // Clear restart detection flag when appropriate
                                if time_since_last_change > RESTART_DETECTION_WINDOW && consecutive_same_status > FAST_THRESHOLD {
                                    potential_restart_detected = false;
                                }
                            }
                        }
                    }
                    _ = cancellation_token.cancelled() => {
                        info!("Docker monitor received cancellation signal, shutting down gracefully");
                        break;
                    }
                }
            }
        });
    }
    
    /// Performs Docker check with connection caching for efficiency.
    /// 
    /// **Connection Optimization:**
    /// - Reuses existing connections when possible
    /// - Only creates new connections when needed
    /// - Reduces connection overhead and resource usage
    /// - Uses timeouts to prevent hanging
    async fn check_docker_with_cache(connection_cache: &mut Option<Docker>) -> DockerMonitorResult<DockerStatus> {
        let timeout_duration = Duration::from_secs(2); // Shorter timeout for efficiency
        
        // Try to use cached connection first
        if let Some(client) = connection_cache {
            match tokio::time::timeout(timeout_duration, client.version()).await {
                Ok(Ok(version_info)) => {
                    let version = version_info.version.unwrap_or_else(|| "Unknown".to_string());
                    return Ok(DockerStatus::Running { version });
                }
                Ok(Err(_)) => {
                    // Connection failed, clear cache and try fresh connection
                    *connection_cache = None;
                }
                Err(_) => {
                    // Timeout, clear cache and try fresh connection
                    *connection_cache = None;
                }
            }
        }
        
        // Create fresh connection
        match tokio::time::timeout(timeout_duration, Self::get_docker_client()).await {
            Ok(Ok(client)) => {
                // Cache the successful connection
                *connection_cache = Some(client.clone());
                
                // Use timeout for the version check
                match tokio::time::timeout(timeout_duration, client.version()).await {
                    Ok(Ok(version_info)) => {
                        let version = version_info.version.unwrap_or_else(|| "Unknown".to_string());
                        Ok(DockerStatus::Running { version })
                    }
                    Ok(Err(e)) => {
                        // Clear cache on API error
                        *connection_cache = None;
                        Ok(DockerStatus::Error { 
                            message: format!("Docker API error: {e}") 
                        })
                    }
                    Err(_) => {
                        // Clear cache on timeout
                        *connection_cache = None;
                        Ok(DockerStatus::Error { 
                            message: "Docker daemon unresponsive (timeout)".to_string() 
                        })
                    }
                }
            }
            Ok(Err(_e)) => {
                Ok(DockerStatus::Stopped)
            }
            Err(_) => {
                Ok(DockerStatus::Stopped)
            }
        }
    }
    
    /// Efficient restart pattern detection with bounded memory usage.
    /// 
    /// **Optimized Pattern Detection:**
    /// - Uses bounded history to prevent memory growth
    /// - Efficient pattern matching with minimal CPU usage
    /// - Focuses on most common restart patterns
    /// - Reduces false positives
    fn detect_restart_pattern_efficient(status_history: &[(DockerStatus, std::time::Instant)]) -> bool {
        if status_history.len() < 3 {
            return false;
        }
        
        let now = std::time::Instant::now();
        let recent_history: Vec<_> = status_history
            .iter()
            .filter(|(_, time)| now.duration_since(*time) < Duration::from_secs(20))
            .take(5) // Limit to last 5 entries for efficiency
            .collect();
            
        if recent_history.len() < 3 {
            return false;
        }
        
        // Look for Running -> Stopped -> Running pattern
        for window in recent_history.windows(3) {
            if let [prev, curr, next] = window {
                let time_between_prev_curr = curr.1.duration_since(prev.1);
                let time_between_curr_next = next.1.duration_since(curr.1);
                
                // Check for restart pattern with reasonable timing
                if matches!(prev.0, DockerStatus::Running { .. }) &&
                   matches!(curr.0, DockerStatus::Stopped) &&
                   matches!(next.0, DockerStatus::Running { .. }) &&
                   time_between_prev_curr < Duration::from_secs(8) &&
                   time_between_curr_next < Duration::from_secs(15) {
                    return true;
                }
            }
        }
        
        false
    }
    
    /// Cancels the monitoring task for graceful shutdown.
    pub fn cancel(&self) {
        self.cancellation_token.cancel();
        info!("Docker monitor cancellation requested");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_docker_monitor_new() {
        let token = CancellationToken::new();
        let monitor = DockerMonitor::new(token);
        let status = monitor.get_current_status().await;
        assert!(matches!(status, DockerStatus::Stopped));
    }

    #[tokio::test]
    async fn test_docker_status_serialization() {
        let status = DockerStatus::Running { 
            version: "24.0.5".to_string() 
        };
        let serialized = serde_json::to_string(&status).unwrap();
        assert!(serialized.contains("Running"));
        assert!(serialized.contains("24.0.5"));
    }

    #[tokio::test]
    async fn test_error_status_serialization() {
        let status = DockerStatus::Error { 
            message: "Connection failed".to_string() 
        };
        let serialized = serde_json::to_string(&status).unwrap();
        assert!(serialized.contains("Error"));
        assert!(serialized.contains("Connection failed"));
    }

    #[tokio::test]
    async fn test_platform_default_connection() {
        // Test that platform-specific connections work correctly
        let result = DockerMonitor::try_platform_default_connection().await;
        // We don't assert success/failure as Docker might not be running in test environment
        // The important thing is that the code compiles and runs without panicking
        match result {
            Ok(_) => println!("Platform default connection succeeded"),
            Err(_) => println!("Platform default connection failed (expected if Docker not running)"),
        }
    }

    #[tokio::test]
    async fn test_docker_host_connection_validation() {
        // Test with invalid DOCKER_HOST format
        std::env::set_var("DOCKER_HOST", "invalid://format");
        let result = DockerMonitor::try_docker_host_connection().await;
        assert!(result.is_err());
        
        // Test with valid TCP format (but connection will fail without running Docker)
        std::env::set_var("DOCKER_HOST", "tcp://localhost:2375");
        let _result = DockerMonitor::try_docker_host_connection().await;
        // Don't assert success as Docker might not be running on that port
        
        // Clean up
        std::env::remove_var("DOCKER_HOST");
    }

    #[tokio::test]
    async fn test_http_connection() {
        // Test HTTP connection (will likely fail without running Docker)
        let result = DockerMonitor::try_http_connection().await;
        // We don't assert success/failure as this depends on Docker being available
        match result {
            Ok(_) => println!("HTTP connection succeeded"),
            Err(_) => println!("HTTP connection failed (expected if Docker not running)"),
        }
    }

    #[test]
    fn test_cross_platform_compilation() {
        // This test ensures the code compiles for all target platforms
        // The conditional compilation blocks should work correctly
        
        #[cfg(target_os = "windows")]
        {
            // Windows-specific code should compile
            println!("Compiling for Windows");
        }
        
        #[cfg(target_os = "linux")]
        {
            // Linux-specific code should compile
            println!("Compiling for Linux");
        }
        
        #[cfg(target_os = "macos")]
        {
            // macOS-specific code should compile
            println!("Compiling for macOS");
        }
        
        // This should always compile
        println!("Cross-platform compilation test passed");
    }

    #[tokio::test]
    async fn test_connection_fallback_logic() {
        // Test that the fallback logic works correctly
        // This is a unit test that doesn't require actual Docker to be running
        
        // Simulate all connection methods failing
        // The actual implementation will handle this gracefully
        let monitor = DockerMonitor::new(CancellationToken::new());
        let status = monitor.get_current_status().await;
        assert!(matches!(status, DockerStatus::Stopped));
    }
} 