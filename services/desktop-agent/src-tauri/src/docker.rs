//! Docker service module for RedSys Desktop Agent
//! 
//! This module provides Docker availability checking using Bollard crate,
//! with cross-platform support and real-time event streaming.
//! 
//! Following official Bollard 0.19.1 documentation and Docker Events API.

use std::sync::Arc;
use tokio::sync::{RwLock, mpsc};
use tokio::task::JoinHandle;
use tracing::{debug, info, warn, error};
use chrono::{TimeZone, Utc};
use bollard::Docker;
use bollard::query_parameters::EventsOptions;
use tokio::time::{timeout, Duration, interval};
use futures::StreamExt;

use std::collections::HashMap;

use crate::error::{AppResult, DockerError};
use crate::types::{DockerServiceInfo, DockerDaemonStatus, DockerEvent};
use tauri::Emitter;

/// Docker service for checking availability and version
/// 
/// This struct provides a reliable interface for Docker operations
/// using Bollard, focusing on availability checking and real-time events.
#[derive(Debug)]
pub struct DockerService {
    /// Bollard Docker client
    docker: Option<Docker>,
    
    /// Whether Docker is available
    is_available: bool,
    
    /// Docker version
    version: Option<String>,
    
    /// Last health check
    last_health_check: chrono::DateTime<Utc>,
    
    /// Connection error message
    connection_error: Option<String>,
    
    /// Event stream handle
    event_stream_handle: Option<JoinHandle<()>>,
    
    /// Daemon monitoring handle
    daemon_monitor_handle: Option<JoinHandle<()>>,
    
    /// Event sender for real-time updates
    event_sender: Option<mpsc::UnboundedSender<DockerEvent>>,
    
    /// Service state
    state: Arc<RwLock<DockerServiceState>>,
    
    /// Tauri app handle for emitting events
    app_handle: Option<tauri::AppHandle>,
}

/// Docker service state for real-time updates
#[derive(Debug, Clone)]
pub struct DockerServiceState {
    /// Current Docker status
    pub status: DockerDaemonStatus,
    
    /// Docker version
    pub version: Option<String>,
    
    /// Last update timestamp
    pub last_updated: chrono::DateTime<Utc>,
    
    /// Connection error
    pub connection_error: Option<String>,
}

impl Default for DockerServiceState {
    fn default() -> Self {
        Self {
            status: DockerDaemonStatus::Unknown,
            version: None,
            last_updated: Utc::now(),
            connection_error: None,
        }
    }
}

impl DockerService {
    /// Create a new Docker service instance with event streaming
    /// 
    /// This function initializes the Docker service and performs an initial
    /// health check to verify Docker availability using Bollard.
    /// 
    /// # Arguments
    /// 
    /// * `event_sender` - Optional sender for real-time Docker events
    /// * `app_handle` - Optional Tauri app handle for emitting events
    /// 
    /// # Returns
    /// 
    /// Returns a new `DockerService` instance or an error if initialization fails
    pub async fn new_with_events(
        event_sender: Option<mpsc::UnboundedSender<DockerEvent>>,
        app_handle: Option<tauri::AppHandle>,
    ) -> AppResult<Self> {
        info!("🐳 Initializing Docker service with event streaming...");
        
        let state = Arc::new(RwLock::new(DockerServiceState::default()));
        
        let mut service = Self {
            docker: None,
            is_available: false,
            version: None,
            last_health_check: Utc::now(),
            connection_error: None,
            event_stream_handle: None,
            daemon_monitor_handle: None,
            event_sender,
            state: state.clone(),
            app_handle,
        };
        
        // STEP 1: Initial check - connect and verify Docker daemon
        info!("🐳 STEP 1: Performing initial Docker daemon check...");
        service.perform_initial_check().await?;
        
        // STEP 2: Start event-driven system
        info!("🐳 STEP 2: Starting event-driven system...");
        service.start_event_driven_system().await?;
        
        info!("🐳 Docker service initialized successfully with event streaming");
        Ok(service)
    }
    
    /// Create a new Docker service instance (backward compatibility)
    pub async fn new() -> AppResult<Self> {
        Self::new_with_events(None, None).await
    }
    
    /// Perform initial Docker daemon check
    /// 
    /// This is the first step: connect to Docker daemon and verify it's running.
    /// This happens once on startup with a grace period to handle simultaneous startup.
    async fn perform_initial_check(&mut self) -> AppResult<()> {
        info!("🐳 [InitialCheck] Starting with 3-second grace period for Docker daemon startup...");
        
        // Emit checking state during grace period
        if let Some(handle) = &self.app_handle {
            let payload = serde_json::json!({
                "status": "checking",
                "timestamp": Utc::now().to_rfc3339(),
            });
            
            if let Err(e) = handle.emit("docker-status-change", payload) {
                warn!("🐳 [InitialCheck] Failed to emit checking status: {}", e);
            } else {
                info!("🐳 [InitialCheck] Emitted checking status during grace period");
            }
        }
        
        // Grace period: wait 3 seconds for Docker daemon to fully start
        tokio::time::sleep(Duration::from_secs(3)).await;
        info!("🐳 [InitialCheck] Grace period completed, now checking Docker daemon...");
        
        // Try different connection methods based on platform
        let docker_result = match std::env::consts::OS {
            "windows" => {
                debug!("Windows detected, trying named pipe connection...");
                Docker::connect_with_named_pipe_defaults()
                    .or_else(|e| {
                        debug!("Named pipe failed: {}, trying HTTP...", e);
                        Docker::connect_with_http_defaults()
                    })
            }
            "macos" => {
                debug!("macOS detected, trying Unix socket connection...");
                Docker::connect_with_socket_defaults()
                    .or_else(|e| {
                        debug!("Unix socket failed: {}, trying HTTP...", e);
                        Docker::connect_with_http_defaults()
                    })
            }
            "linux" => {
                debug!("Linux detected, trying Unix socket connection...");
                Docker::connect_with_socket_defaults()
                    .or_else(|e| {
                        debug!("Unix socket failed: {}, trying HTTP...", e);
                        Docker::connect_with_http_defaults()
                    })
            }
            _ => {
                debug!("Unknown platform, trying HTTP connection...");
                Docker::connect_with_http_defaults()
            }
        };
        
        match docker_result {
            Ok(docker) => {
                self.docker = Some(docker);
                info!("🐳 [InitialCheck] Successfully connected to Docker daemon");
                
                // Verify daemon is responsive
                info!("🐳 [InitialCheck] Verifying daemon is responsive...");
                match timeout(Duration::from_secs(10), self.docker.as_ref().unwrap().version()).await {
                    Ok(Ok(version_info)) => {
                        self.is_available = true;
                        self.version = version_info.version.clone();
                        self.last_health_check = Utc::now();
                        self.connection_error = None;
                        
                        // Update state
                        let mut state_guard = self.state.write().await;
                        state_guard.status = DockerDaemonStatus::Running;
                        state_guard.version = self.version.clone();
                        state_guard.last_updated = Utc::now();
                        state_guard.connection_error = None;
                        
                        info!("🐳 [InitialCheck] Docker daemon is running (Version: {})", 
                              self.version.as_ref().unwrap_or(&"Unknown".to_string()));
                        
                        // Emit running status
                        if let Some(handle) = &self.app_handle {
                            let payload = serde_json::json!({
                                "status": "running",
                                "connected": true,
                                "timestamp": Utc::now().to_rfc3339(),
                            });
                            
                            if let Err(e) = handle.emit("docker-status-change", payload) {
                                warn!("🐳 [InitialCheck] Failed to emit running status: {}", e);
                            } else {
                                info!("🐳 [InitialCheck] Emitted running status");
                            }
                        }
                        
                        Ok(())
                    }
                    Ok(Err(e)) => {
                        self.is_available = false;
                        let error_msg = format!("Docker version check failed: {}", e);
                        self.connection_error = Some(error_msg.clone());
                        warn!("🐳 [InitialCheck] Docker version check failed: {}", e);
                        
                        let mut state_guard = self.state.write().await;
                        state_guard.status = DockerDaemonStatus::Stopped;
                        state_guard.connection_error = Some(error_msg);
                        state_guard.last_updated = Utc::now();
                        
                        // Emit stopped status
                        if let Some(handle) = &self.app_handle {
                            let payload = serde_json::json!({
                                "status": "stopped",
                                "connected": false,
                                "timestamp": Utc::now().to_rfc3339(),
                            });
                            
                            if let Err(e) = handle.emit("docker-status-change", payload) {
                                warn!("🐳 [InitialCheck] Failed to emit stopped status: {}", e);
                            } else {
                                info!("🐳 [InitialCheck] Emitted stopped status");
                            }
                        }
                        
                        Err(DockerError::DaemonNotRunning.into())
                    }
                    Err(_) => {
                        self.is_available = false;
                        let error_msg = "Docker health check timed out".to_string();
                        self.connection_error = Some(error_msg.clone());
                        error!("🐳 [InitialCheck] Docker health check timed out after 10 seconds");
                        
                        let mut state_guard = self.state.write().await;
                        state_guard.status = DockerDaemonStatus::Error;
                        state_guard.connection_error = Some(error_msg);
                        state_guard.last_updated = Utc::now();
                        
                        // Emit error status
                        if let Some(handle) = &self.app_handle {
                            let payload = serde_json::json!({
                                "status": "error",
                                "connected": false,
                                "timestamp": Utc::now().to_rfc3339(),
                            });
                            
                            if let Err(e) = handle.emit("docker-status-change", payload) {
                                warn!("🐳 [InitialCheck] Failed to emit error status: {}", e);
                            } else {
                                info!("🐳 [InitialCheck] Emitted error status");
                            }
                        }
                        
                        Err(DockerError::Timeout { operation: "health_check".to_string() }.into())
                    }
                }
            }
            Err(e) => {
                self.is_available = false;
                let error_msg = format!("Connection failed: {}", e);
                self.connection_error = Some(error_msg.clone());
                error!("🐳 [InitialCheck] Docker connection failed: {}", e);
                
                let mut state_guard = self.state.write().await;
                state_guard.status = DockerDaemonStatus::Stopped;
                state_guard.connection_error = Some(error_msg);
                state_guard.last_updated = Utc::now();
                
                // Emit stopped status
                if let Some(handle) = &self.app_handle {
                    let payload = serde_json::json!({
                        "status": "stopped",
                        "connected": false,
                        "timestamp": Utc::now().to_rfc3339(),
                    });
                    
                    if let Err(e) = handle.emit("docker-status-change", payload) {
                        warn!("🐳 [InitialCheck] Failed to emit stopped status: {}", e);
                    } else {
                        info!("🐳 [InitialCheck] Emitted stopped status");
                    }
                }
                
                Err(DockerError::DaemonNotRunning.into())
            }
        }
    }
    
    /// Start event-driven system
    /// 
    /// This is the second step: start real-time event streaming and daemon monitoring.
    /// This runs continuously after the initial check.
    async fn start_event_driven_system(&mut self) -> AppResult<()> {
        info!("🐳 [EventSystem] Starting event-driven system...");
        
        // Start Docker Events API stream (for container/image events)
        self.start_docker_events_stream().await?;
        
        // Start daemon monitoring (for daemon up/down detection)
        self.start_daemon_monitoring().await?;
        
        info!("🐳 [EventSystem] Event-driven system started successfully");
        Ok(())
    }
    
    /// Start Docker Events API stream
    /// 
    /// This listens to real Docker events (containers, images, etc.)
    /// using the official Docker Events API.
    async fn start_docker_events_stream(&mut self) -> AppResult<()> {
        let event_sender = self.event_sender.clone();
        let state = self.state.clone();
        let app_handle = self.app_handle.clone();
        
        info!("🐳 [EventSystem] Starting Docker Events API stream...");
        
        let handle = tokio::spawn(async move {
            // Create a new Docker client for the event stream
            let docker = match std::env::consts::OS {
                "windows" => {
                    Docker::connect_with_named_pipe_defaults()
                        .or_else(|_| Docker::connect_with_http_defaults())
                }
                "macos" | "linux" => {
                    Docker::connect_with_socket_defaults()
                        .or_else(|_| Docker::connect_with_http_defaults())
                }
                _ => Docker::connect_with_http_defaults(),
            };
            
            let docker = match docker {
                Ok(d) => d,
                Err(e) => {
                    error!("🐳 [EventSystem] Failed to connect to Docker for event stream: {}", e);
                    return;
                }
            };
            
            // Configure Events API options following official documentation
            let mut filters = HashMap::new();
            filters.insert("type".to_string(), vec!["container".to_string(), "image".to_string(), "volume".to_string(), "network".to_string()]);
            
            let events_options = EventsOptions {
                since: None,      // Start from now
                until: None,      // Continue indefinitely
                filters: Some(filters),    // Filter for relevant events
            };
            
            // Start listening to Docker events using official Events API
            let mut events_stream = docker.events(Some(events_options));
            info!("🐳 [EventSystem] Docker Events API stream started successfully");
            
            while let Some(event_result) = events_stream.next().await {
                match event_result {
                    Ok(event) => {
                        debug!("🐳 [EventSystem] Received Docker event: {:?}", event);
                        
                        // Update state
                        let mut state_guard = state.write().await;
                        state_guard.last_updated = Utc::now();
                        
                        // Create Docker event for frontend
                        let docker_event = DockerEvent {
                            event_type: event.typ.map(|t| t.to_string()).unwrap_or_default(),
                            action: event.action.clone().unwrap_or_default(),
                            actor_id: event.actor.as_ref().and_then(|a| a.id.clone()),
                            actor_attributes: event.actor.as_ref().and_then(|a| a.attributes.clone()).unwrap_or_default(),
                            timestamp: chrono::Utc.timestamp_opt(event.time.unwrap_or(0) as i64, 0).single().unwrap_or_else(Utc::now),
                        };
                        
                        // Send event to frontend via Tauri
                        if let Some(handle) = &app_handle {
                            if let Err(e) = handle.emit("docker-event", &docker_event) {
                                warn!("🐳 [EventSystem] Failed to emit Docker event to frontend: {}", e);
                            }
                        }
                        
                        // Send event to internal sender if available
                        if let Some(sender) = &event_sender {
                            if let Err(e) = sender.send(docker_event) {
                                warn!("🐳 [EventSystem] Failed to send Docker event to internal handler: {}", e);
                            }
                        }
                        
                        // Log significant events
                        if let Some(actor) = &event.actor {
                            if let Some(container_id) = &actor.id {
                                info!("🐳 [EventSystem] Docker event - Type: {:?}, Action: {:?}, Container: {:?}", 
                                      event.typ, event.action, container_id);
                            }
                        }
                    }
                    Err(e) => {
                        error!("🐳 [EventSystem] Error receiving Docker event: {}", e);
                        break;
                    }
                }
            }
            
            info!("🐳 [EventSystem] Docker Events API stream ended");
        });
        
        self.event_stream_handle = Some(handle);
        Ok(())
    }
    
    /// Start daemon monitoring for connection state changes
    /// 
    /// This monitors the Docker daemon connection state by periodically
    /// checking if the daemon is responsive. This is separate from the Events API
    /// which only reports container/image events, not daemon state changes.
    async fn start_daemon_monitoring(&mut self) -> AppResult<()> {
        let state = self.state.clone();
        let app_handle = self.app_handle.clone();
        
        info!("🐳 [EventSystem] Starting Docker daemon monitoring...");
        
        let handle = tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(5)); // Check every 5 seconds
            
            loop {
                interval.tick().await;
                debug!("🐳 [DaemonMonitor] Checking Docker daemon state...");
                
                // Create a new Docker client for monitoring
                let docker = match std::env::consts::OS {
                    "windows" => {
                        Docker::connect_with_named_pipe_defaults()
                            .or_else(|_| Docker::connect_with_http_defaults())
                    }
                    "macos" | "linux" => {
                        Docker::connect_with_socket_defaults()
                            .or_else(|_| Docker::connect_with_http_defaults())
                    }
                    _ => Docker::connect_with_http_defaults(),
                };
                
                let docker = match docker {
                    Ok(d) => {
                        debug!("🐳 [DaemonMonitor] Docker client created, checking version...");
                        d
                    },
                    Err(e) => {
                        debug!("🐳 [DaemonMonitor] Could not create Docker client: {}", e);
                        // Daemon is not available
                        let mut state_guard = state.write().await;
                        if state_guard.status != DockerDaemonStatus::Stopped {
                            state_guard.status = DockerDaemonStatus::Stopped;
                            state_guard.last_updated = Utc::now();
                            state_guard.connection_error = Some("Docker daemon not available".to_string());
                            
                            info!("🐳 [DaemonMonitor] Docker daemon is now STOPPED");
                            
                            // Emit connection change event
                            if let Some(handle) = &app_handle {
                                let payload = serde_json::json!({
                                    "connected": false,
                                    "timestamp": Utc::now().to_rfc3339(),
                                });
                                
                                if let Err(e) = handle.emit("docker-connection-change", payload) {
                                    warn!("🐳 [DaemonMonitor] Failed to emit Docker connection change event: {}", e);
                                } else {
                                    info!("🐳 [DaemonMonitor] Emitted Docker connection change event: connected = false");
                                }
                            }
                        }
                        continue;
                    }
                };
                
                // Check if daemon is responsive
                match timeout(Duration::from_secs(3), docker.version()).await {
                    Ok(Ok(version_info)) => {
                        // Daemon is available
                        let mut state_guard = state.write().await;
                        if state_guard.status != DockerDaemonStatus::Running {
                            state_guard.status = DockerDaemonStatus::Running;
                            state_guard.version = version_info.version.clone();
                            state_guard.last_updated = Utc::now();
                            state_guard.connection_error = None;
                            
                            info!("🐳 [DaemonMonitor] Docker daemon is now RUNNING (Version: {:?})", version_info.version);
                            
                            // Emit connection change event
                            if let Some(handle) = &app_handle {
                                let payload = serde_json::json!({
                                    "connected": true,
                                    "timestamp": Utc::now().to_rfc3339(),
                                });
                                
                                if let Err(e) = handle.emit("docker-connection-change", payload) {
                                    warn!("🐳 [DaemonMonitor] Failed to emit Docker connection change event: {}", e);
                                } else {
                                    info!("🐳 [DaemonMonitor] Emitted Docker connection change event: connected = true");
                                }
                            }
                        }
                    }
                    Ok(Err(_)) | Err(_) => {
                        // Daemon is not responsive
                        let mut state_guard = state.write().await;
                        if state_guard.status != DockerDaemonStatus::Stopped {
                            state_guard.status = DockerDaemonStatus::Stopped;
                            state_guard.last_updated = Utc::now();
                            state_guard.connection_error = Some("Docker daemon not responsive".to_string());
                            
                            info!("🐳 [DaemonMonitor] Docker daemon is now STOPPED (not responsive)");
                            
                            // Emit connection change event
                            if let Some(handle) = &app_handle {
                                let payload = serde_json::json!({
                                    "connected": false,
                                    "timestamp": Utc::now().to_rfc3339(),
                                });
                                
                                if let Err(e) = handle.emit("docker-connection-change", payload) {
                                    warn!("🐳 [DaemonMonitor] Failed to emit Docker connection change event: {}", e);
                                } else {
                                    info!("🐳 [DaemonMonitor] Emitted Docker connection change event: connected = false");
                                }
                            }
                        }
                    }
                }
            }
        });
        
        self.daemon_monitor_handle = Some(handle);
        Ok(())
    }
    
    /// Check Docker health and availability
    /// 
    /// This function verifies that Docker is available and running by
    /// using Bollard to ping the Docker daemon and get version info.
    /// 
    /// # Returns
    /// 
    /// Returns `Ok(())` if Docker is healthy, or an error if not
    pub async fn check_health(&mut self) -> AppResult<()> {
        debug!("🐳 Starting Docker health check");
        
        let was_available = self.is_available;
        
        if self.docker.is_none() {
            debug!("🐳 No Docker connection, attempting to establish one");
            if let Err(e) = self.connect_to_docker().await {
                self.is_available = false;
                self.connection_error = Some(format!("Connection failed: {}", e));
                error!("🐳 Docker connection failed: {}", e);
                
                // Emit connection change event if status changed
                if was_available != self.is_available {
                    self.emit_connection_change_event().await;
                }
                
                return Err(e);
            }
        }
        
        if let Some(ref docker) = self.docker {
            info!("🐳 Calling docker.version() with 10-second timeout");
            
            match timeout(Duration::from_secs(10), docker.version()).await {
                Ok(Ok(version_info)) => {
                    self.is_available = true;
                    self.version = version_info.version.clone();
                    self.last_health_check = Utc::now();
                    self.connection_error = None;
                    
                    // Update state
                    let mut state_guard = self.state.write().await;
                    state_guard.status = DockerDaemonStatus::Running;
                    state_guard.version = self.version.clone();
                    state_guard.last_updated = Utc::now();
                    state_guard.connection_error = None;
                    
                    info!("🐳 Docker health check passed (Version: {})", 
                          self.version.as_ref().unwrap_or(&"Unknown".to_string()));
                    
                    // Emit connection change event if status changed
                    if was_available != self.is_available {
                        self.emit_connection_change_event().await;
                    }
                    
                    Ok(())
                }
                Ok(Err(e)) => {
                    self.is_available = false;
                    let error_msg = format!("Docker version check failed: {}", e);
                    self.connection_error = Some(error_msg.clone());
                    warn!("🐳 Docker version check failed: {}", e);
                    
                    let mut state_guard = self.state.write().await;
                    state_guard.status = DockerDaemonStatus::Stopped;
                    state_guard.connection_error = Some(error_msg);
                    state_guard.last_updated = Utc::now();
                    
                    self.docker = None;
                    
                    // Emit connection change event if status changed
                    if was_available != self.is_available {
                        self.emit_connection_change_event().await;
                    }
                    
                    Err(DockerError::DaemonNotRunning.into())
                }
                Err(_) => {
                    self.is_available = false;
                    let error_msg = "Docker health check timed out".to_string();
                    self.connection_error = Some(error_msg.clone());
                    error!("🐳 Docker health check timed out after 10 seconds");
                    
                    let mut state_guard = self.state.write().await;
                    state_guard.status = DockerDaemonStatus::Error;
                    state_guard.connection_error = Some(error_msg);
                    state_guard.last_updated = Utc::now();
                    
                    // Emit connection change event if status changed
                    if was_available != self.is_available {
                        self.emit_connection_change_event().await;
                    }
                    
                    Err(DockerError::Timeout { operation: "health_check".to_string() }.into())
                }
            }
        } else {
            self.is_available = false;
            let error_msg = "No Docker connection available".to_string();
            self.connection_error = Some(error_msg.clone());
            error!("🐳 No Docker connection available for health check");
            
            let mut state_guard = self.state.write().await;
            state_guard.status = DockerDaemonStatus::Stopped;
            state_guard.connection_error = Some(error_msg);
            state_guard.last_updated = Utc::now();
            
            // Emit connection change event if status changed
            if was_available != self.is_available {
                self.emit_connection_change_event().await;
            }
            
            Err(DockerError::DaemonNotRunning.into())
        }
    }
    
    /// Connect to Docker daemon using Bollard
    /// 
    /// Attempts to establish a connection to the Docker daemon using
    /// platform-appropriate connection methods with proper error handling.
    /// 
    /// # Returns
    /// 
    /// Returns `Ok(())` if connection is successful
    async fn connect_to_docker(&mut self) -> AppResult<()> {
        debug!("🐳 Attempting to connect to Docker daemon");
        
        // Try different connection methods based on platform
        let docker_result = match std::env::consts::OS {
            "windows" => {
                debug!("Windows detected, trying named pipe connection...");
                Docker::connect_with_named_pipe_defaults()
                    .or_else(|e| {
                        debug!("Named pipe failed: {}, trying HTTP...", e);
                        Docker::connect_with_http_defaults()
                    })
            }
            "macos" => {
                debug!("macOS detected, trying Unix socket connection...");
                Docker::connect_with_socket_defaults()
                    .or_else(|e| {
                        debug!("Unix socket failed: {}, trying HTTP...", e);
                        Docker::connect_with_http_defaults()
                    })
            }
            "linux" => {
                debug!("Linux detected, trying Unix socket connection...");
                Docker::connect_with_socket_defaults()
                    .or_else(|e| {
                        debug!("Unix socket failed: {}, trying HTTP...", e);
                        Docker::connect_with_http_defaults()
                    })
            }
            _ => {
                debug!("Unknown platform, trying HTTP connection...");
                Docker::connect_with_http_defaults()
            }
        };
        
        match docker_result {
            Ok(docker) => {
                self.docker = Some(docker);
                debug!("🐳 Successfully connected to Docker daemon");
                Ok(())
            }
            Err(e) => {
                let error_msg = format!("Failed to connect to Docker daemon: {}", e);
                self.connection_error = Some(error_msg.clone());
                warn!("🐳 {}", error_msg);
                Err(DockerError::DaemonNotRunning.into())
            }
        }
    }
    
    /// Emit connection change event to frontend
    async fn emit_connection_change_event(&self) {
        if let Some(handle) = &self.app_handle {
            let payload = serde_json::json!({
                "connected": self.is_available,
                "timestamp": Utc::now().to_rfc3339(),
            });
            
            if let Err(e) = handle.emit("docker-connection-change", payload) {
                warn!("Failed to emit Docker connection change event: {}", e);
            } else {
                info!("🐳 Emitted Docker connection change event: connected = {}", self.is_available);
            }
        }
    }
    
    /// Get Docker service information
    /// 
    /// Returns current information about the Docker service status.
    /// 
    /// # Returns
    /// 
    /// Returns `DockerServiceInfo` with current status
    pub async fn get_service_info(&self) -> DockerServiceInfo {
        // Get the current state from the daemon monitor
        let state = self.state.read().await;
        
        DockerServiceInfo {
            is_available: state.status == DockerDaemonStatus::Running,
            version: state.version.clone(),
            daemon_status: state.status.clone(),
            last_health_check: state.last_updated,
            connection_error: state.connection_error.clone(),
        }
    }
    
    /// Get real-time service state
    /// 
    /// Returns the current real-time state of the Docker service.
    /// 
    /// # Returns
    /// 
    /// Returns the current service state
    pub async fn get_service_state(&self) -> DockerServiceState {
        self.state.read().await.clone()
    }
    
    /// Cleanup Docker service resources
    /// 
    /// # Returns
    /// 
    /// Returns success or an error
    pub async fn cleanup(&mut self) -> AppResult<()> {
        info!("🐳 Cleaning up Docker service");
        
        // Stop event streaming
        if let Some(handle) = self.event_stream_handle.take() {
            handle.abort();
            info!("🐳 Docker event stream stopped");
        }
        
        // Stop daemon monitoring
        if let Some(handle) = self.daemon_monitor_handle.take() {
            handle.abort();
            info!("🐳 Docker daemon monitoring stopped");
        }
        
        // Clear Docker connection
        self.docker = None;
        self.is_available = false;
        
        info!("🐳 Docker service cleanup completed");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_docker_service_new() {
        // This test will fail if Docker is not running, which is expected
        let result = DockerService::new().await;
        // We don't assert here because Docker might not be available in test environment
        if let Err(e) = result {
            debug!("Docker service test failed as expected: {}", e);
        }
    }

    #[tokio::test]
    async fn test_docker_service_info() {
        let service = DockerService {
            docker: None,
            is_available: false,
            version: None,
            last_health_check: Utc::now(),
            connection_error: Some("Test error".to_string()),
            event_stream_handle: None,
            daemon_monitor_handle: None,
            event_sender: None,
            state: Arc::new(RwLock::new(DockerServiceState::default())),
            app_handle: None,
        };
        
        let info = service.get_service_info().await;
        assert!(!info.is_available);
        assert_eq!(info.daemon_status, DockerDaemonStatus::Stopped);
        assert!(info.connection_error.is_some());
    }
} 
