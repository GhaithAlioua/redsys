/**
 * Docker Service for RedSys Desktop Agent
 * 
 * This service provides a clean interface for Docker operations
 * with real-time event streaming and professional error handling.
 * 
 * Following official Tauri documentation and best practices.
 */

// ============================================================================
// TYPE DEFINITIONS
// ============================================================================

export type DockerDaemonStatus = 'Running' | 'Stopped' | 'Unknown' | 'Error';

export interface DockerServiceInfo {
  /** Whether Docker is available */
  is_available: boolean;
  
  /** Docker version string */
  version?: string;
  
  /** Docker daemon status */
  daemon_status: DockerDaemonStatus;
  
  /** Last health check timestamp */
  last_health_check: string;
  
  /** Connection error message (if any) */
  connection_error?: string;
}

/**
 * Docker event for real-time updates
 */
export interface DockerEvent {
  /** Type of event (container, image, volume, network, etc.) */
  event_type: string;
  
  /** Action performed (create, start, stop, delete, etc.) */
  action: string;
  
  /** ID of the affected resource */
  actor_id?: string;
  
  /** Additional attributes of the event */
  actor_attributes: Record<string, string>;
  
  /** Timestamp when the event occurred */
  timestamp: string;
}

/**
 * Docker service configuration
 */
export interface DockerServiceConfig {
  /** Enable real-time event streaming */
  enableEventStreaming?: boolean;
  
  /** Event handler for real-time updates */
  onEvent?: (event: DockerEvent) => void;
  
  /** Error handler for events */
  onEventError?: (error: Error) => void;
  
  /** Connection state handler */
  onConnectionChange?: (connected: boolean) => void;
}

// ============================================================================
// ERROR HANDLING
// ============================================================================

/**
 * Docker service error class
 */
export class DockerServiceError extends Error {
  public readonly code: string;
  public readonly retryable: boolean;
  public readonly timestamp: number;
  public readonly cause?: Error;

  constructor(
    message: string,
    code: string = 'UNKNOWN_ERROR',
    retryable: boolean = false,
    cause?: Error
  ) {
    super(message);
    this.name = 'DockerServiceError';
    this.code = code;
    this.retryable = retryable;
    this.timestamp = Date.now();
    this.cause = cause;
  }

  /**
   * Create error from Tauri error
   */
  static fromTauriError(error: unknown): DockerServiceError {
    if (error instanceof DockerServiceError) {
      return error;
    }

    const message = error instanceof Error ? error.message : String(error);
    const code = error instanceof Error && 'code' in error ? String(error.code) : 'TAURI_ERROR';
    
    return new DockerServiceError(message, code, false, error instanceof Error ? error : undefined);
  }

  /**
   * Create daemon not running error
   */
  static daemonNotRunning(): DockerServiceError {
    return new DockerServiceError(
      'Docker daemon is not running',
      'DAEMON_NOT_RUNNING',
      true
    );
  }

  /**
   * Create connection failed error
   */
  static connectionFailed(cause?: Error): DockerServiceError {
    return new DockerServiceError(
      'Failed to connect to Docker daemon',
      'CONNECTION_FAILED',
      true,
      cause
    );
  }

  /**
   * Create timeout error
   */
  static timeout(operation: string): DockerServiceError {
    return new DockerServiceError(
      `Docker operation timed out: ${operation}`,
      'TIMEOUT',
      true
    );
  }
}

// ============================================================================
// DOCKER SERVICE CLASS
// ============================================================================

/**
 * Docker service implementation with real-time event streaming
 * 
 * This class provides a clean interface for Docker operations
 * with real-time event streaming and professional error handling.
 */
export class DockerService {
  private config: Required<DockerServiceConfig>;
  private abortController: AbortController | null = null;
  private eventStreamActive: boolean = false;
  private eventListeners: (() => void)[] = [];

  constructor(config: DockerServiceConfig = {}) {
    this.config = {
      enableEventStreaming: config.enableEventStreaming ?? true,
      onEvent: config.onEvent ?? (() => {}),
      onEventError: config.onEventError ?? (() => {}),
      onConnectionChange: config.onConnectionChange ?? (() => {}),
    };
  }

  /**
   * Check Docker health with real-time event streaming
   * 
   * @returns Promise resolving to Docker service information
   */
  async checkHealth(): Promise<DockerServiceInfo> {
    try {
      console.log('🐳 DockerService: checkHealth called');
      
      // Import Tauri invoke dynamically to avoid build issues
      const { invoke } = await import('@tauri-apps/api/core');
      
      // Add timeout to prevent infinite loading
      const timeoutPromise = new Promise<never>((_, reject) => {
        setTimeout(() => reject(new Error('Docker health check timed out after 20 seconds')), 20000);
      });
      
      const invokePromise = invoke<DockerServiceInfo>('check_docker_health');
      
      const result = await Promise.race([invokePromise, timeoutPromise]);
      
      console.log('🐳 DockerService: health check completed', result);
      
      // Start real-time event streaming if enabled and not already active
      if (this.config.enableEventStreaming && !this.eventStreamActive) {
        this.startRealEventStream();
      }
      
      return result;
      
    } catch (error) {
      console.error('🐳 DockerService: health check failed', error);
      throw DockerServiceError.fromTauriError(error);
    }
  }

  /**
   * Start real-time Docker event streaming using Tauri events
   * 
   * This method establishes a real-time connection to Docker events
   * using Tauri's event system for true event-driven updates.
   */
  private async startRealEventStream(): Promise<void> {
    try {
      console.log('🐳 DockerService: Starting real-time Docker event stream');
      
      // Abort any existing stream
      if (this.abortController) {
        this.abortController.abort();
      }

      this.abortController = new AbortController();

      // Import Tauri event system
      const { listen } = await import('@tauri-apps/api/event');
      
      // Listen for Docker events from the backend
      const unlistenDockerEvent = await listen('docker-event', (event) => {
        console.log('🐳 DockerService: Received Docker event from backend', event);
        
        try {
          const payload = event.payload as any;
          const dockerEvent: DockerEvent = {
            event_type: payload?.event_type || 'unknown',
            action: payload?.action || 'unknown',
            actor_id: payload?.actor_id,
            actor_attributes: payload?.actor_attributes || {},
            timestamp: payload?.timestamp || new Date().toISOString(),
          };
          
          this.config.onEvent(dockerEvent);
        } catch (error) {
          console.error('🐳 DockerService: Error processing Docker event', error);
          this.config.onEventError(error as Error);
        }
      });

      // Listen for Docker connection state changes
      const unlistenConnection = await listen('docker-connection-change', (event) => {
        console.log('🐳 DockerService: Docker connection state changed', event);
        
        try {
          const payload = event.payload as any;
          const connected = payload?.connected || false;
          
          console.log('🐳 DockerService: Connection state changed to:', connected);
          this.config.onConnectionChange(connected);
        } catch (error) {
          console.error('🐳 DockerService: Error processing connection change event', error);
          this.config.onEventError(error as Error);
        }
      });

      // Store cleanup functions
      this.eventListeners = [unlistenDockerEvent, unlistenConnection];
      
      this.abortController.signal.addEventListener('abort', () => {
        this.eventListeners.forEach(unlisten => unlisten());
        this.eventListeners = [];
      });

      this.eventStreamActive = true;
      console.log('🐳 DockerService: Real-time Docker event stream started successfully');
      
    } catch (error) {
      console.error('🐳 DockerService: Failed to start real-time event stream', error);
      this.config.onEventError(error as Error);
    }
  }

  /**
   * Stop event streaming
   */
  stopEventStream(): void {
    console.log('🐳 DockerService: Stopping event stream');
    
    if (this.abortController) {
      this.abortController.abort();
      this.abortController = null;
    }
    
    // Cleanup event listeners
    this.eventListeners.forEach(unlisten => unlisten());
    this.eventListeners = [];
    
    this.eventStreamActive = false;
  }

  /**
   * Get Docker version
   * 
   * @returns Promise resolving to Docker version string
   */
  async getVersion(): Promise<string> {
    try {
      console.log('🐳 DockerService: Getting Docker version');
      
      const { invoke } = await import('@tauri-apps/api/core');
      
      const version = await invoke<string>('get_docker_version');
      console.log('🐳 DockerService: Docker version retrieved:', version);
      
      return version;
      
    } catch (error) {
      console.error('🐳 DockerService: Failed to get Docker version', error);
      throw DockerServiceError.fromTauriError(error);
    }
  }

  /**
   * Get application state
   * 
   * @returns Promise resolving to application state
   */
  async getApplicationState(): Promise<any> {
    try {
      console.log('🐳 DockerService: Getting application state');
      
      const { invoke } = await import('@tauri-apps/api/core');
      
      const state = await invoke('get_application_state');
      console.log('🐳 DockerService: Application state retrieved:', state);
      
      return state;
      
    } catch (error) {
      console.error('🐳 DockerService: Failed to get application state', error);
      throw DockerServiceError.fromTauriError(error);
    }
  }

  /**
   * Check if event stream is active
   * 
   * @returns Whether event streaming is currently active
   */
  isEventStreamActive(): boolean {
    return this.eventStreamActive;
  }

  /**
   * Update configuration
   * 
   * @param newConfig New configuration
   */
  updateConfig(newConfig: Partial<DockerServiceConfig>): void {
    this.config = {
      ...this.config,
      ...newConfig,
    };
  }

  /**
   * Get current configuration
   * 
   * @returns Current configuration
   */
  getConfig(): Required<DockerServiceConfig> {
    return { ...this.config };
  }

  /**
   * Cleanup resources
   */
  cleanup(): void {
    console.log('🐳 DockerService: Cleaning up resources');
    
    this.stopEventStream();
  }
}

// ============================================================================
// UTILITY FUNCTIONS
// ============================================================================

/**
 * Get a singleton Docker service instance
 * 
 * @param config Optional configuration
 * @returns Docker service instance
 */
export function getDockerService(config?: DockerServiceConfig): DockerService {
  if (!(getDockerService as any).instance) {
    (getDockerService as any).instance = new DockerService(config);
  }
  return (getDockerService as any).instance;
}

/**
 * Cleanup the singleton Docker service
 */
export function cleanupDockerService(): void {
  if ((getDockerService as any).instance) {
    (getDockerService as any).instance.cleanup();
    (getDockerService as any).instance = null;
  }
}

// ============================================================================
// EXPORTS
// ============================================================================ 