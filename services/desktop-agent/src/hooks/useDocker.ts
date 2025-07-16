/**
 * Docker Hook for RedSys Desktop Agent
 * 
 * This hook provides a clean, type-safe interface for Docker state management
 * with real-time event streaming and React best practices.
 * 
 * Following official React hooks documentation and best practices.
 */

import { useState, useEffect, useCallback, useRef, useMemo } from 'react';
import { DockerService, DockerServiceInfo, DockerServiceError, DockerEvent, DockerServiceConfig } from '../services/docker';

// ============================================================================
// TYPE DEFINITIONS
// ============================================================================

/**
 * Hook configuration interface
 */
export interface UseDockerConfig extends DockerServiceConfig {
  /** Enable real-time event streaming */
  enableEventStreaming?: boolean;
  
  /** Custom error handler */
  onError?: (error: DockerServiceError) => void;
  
  /** Custom success handler */
  onSuccess?: (data: DockerServiceInfo) => void;
  
  /** Custom loading state handler */
  onLoadingChange?: (loading: boolean) => void;
  
  /** Custom event handler */
  onEvent?: (event: DockerEvent) => void;
}

/**
 * Hook return state interface
 */
export interface UseDockerState {
  /** Docker service information */
  data: DockerServiceInfo | null;
  
  /** Loading state */
  isLoading: boolean;
  
  /** Error state */
  error: DockerServiceError | null;
  
  /** Last update timestamp */
  lastUpdated: Date | null;
  
  /** Whether Docker is available */
  isAvailable: boolean;
  
  /** Docker daemon status */
  status: string;
  
  /** Event streaming status */
  eventStreamActive: boolean;
  
  /** Connection status */
  isConnected: boolean;
}

/**
 * Hook return actions interface
 */
export interface UseDockerActions {
  /** Refresh Docker status */
  refresh: () => Promise<void>;
  
  /** Clear error state */
  clearError: () => void;
  
  /** Start event streaming */
  startEventStream: () => Promise<void>;
  
  /** Stop event streaming */
  stopEventStream: () => void;
  
  /** Update configuration */
  updateConfig: (config: Partial<UseDockerConfig>) => void;
}

/**
 * Hook return type
 */
export type UseDockerReturn = UseDockerState & UseDockerActions;

// ============================================================================
// DOCKER HOOK
// ============================================================================

/**
 * Docker hook implementation with real-time event streaming
 * 
 * This hook provides comprehensive Docker state management with
 * real-time event streaming and professional error handling.
 * 
 * @param config Hook configuration
 * @returns Hook state and actions
 */
export function useDocker(config: UseDockerConfig = {}): UseDockerReturn {
  // ============================================================================
  // STATE MANAGEMENT
  // ============================================================================
  
  const [state, setState] = useState<UseDockerState>({
    data: null,
    isLoading: false,
    error: null,
    lastUpdated: null,
    isAvailable: false,
    status: 'idle',
    eventStreamActive: false,
    isConnected: false,
  });

  // ============================================================================
  // STABLE REFERENCES
  // ============================================================================
  
  const isMountedRef = useRef(true);
  const hasInitializedRef = useRef(false);
  const dockerServiceRef = useRef<DockerService | null>(null);
  const eventListenersRef = useRef<(() => void)[]>([]);
  
  // Stable config reference to prevent re-renders
  const configRef = useRef({
    enableEventStreaming: config.enableEventStreaming ?? true,
    onError: config.onError ?? (() => {}),
    onSuccess: config.onSuccess ?? (() => {}),
    onLoadingChange: config.onLoadingChange ?? (() => {}),
    onEvent: config.onEvent ?? (() => {}),
    onEventError: config.onEventError ?? (() => {}),
    onConnectionChange: config.onConnectionChange ?? (() => {}),
  });

  // ============================================================================
  // SERVICE INITIALIZATION
  // ============================================================================
  
  // Initialize Docker service once
  if (!dockerServiceRef.current) {
    dockerServiceRef.current = new DockerService({
      enableEventStreaming: configRef.current.enableEventStreaming,
      onEvent: (event: DockerEvent) => {
        console.log('🔍 useDocker: Received Docker event', event);
        configRef.current.onEvent(event);
        
        // Only update timestamp, don't refresh data
        setState(prev => ({
          ...prev,
          lastUpdated: new Date(),
        }));
      },
      onEventError: (error: Error) => {
        console.error('🔍 useDocker: Event error', error);
        configRef.current.onEventError(error);
      },
      onConnectionChange: (connected: boolean) => {
        console.log('🔍 useDocker: Connection changed', connected);
        configRef.current.onConnectionChange(connected);
        
        // Update connection state
        setState(prev => ({
          ...prev,
          isConnected: connected,
          lastUpdated: new Date(),
        }));

        // Automatically refresh Docker status on connection change
        if (isMountedRef.current && dockerServiceRef.current) {
          refresh();
        }
      },
    });
  }

  // ============================================================================
  // CORE FUNCTIONS
  // ============================================================================
  
  /**
   * Perform initial Docker status check
   */
  const performInitialCheck = useCallback(async (): Promise<void> => {
    console.log('🔍 useDocker: Performing initial Docker status check');
    
    if (!isMountedRef.current || !dockerServiceRef.current) {
      console.log('🔍 useDocker: Skipping initial check - not mounted or service not available');
      return;
    }

    try {
      setState(prev => ({ ...prev, isLoading: true, status: 'loading' }));
      configRef.current.onLoadingChange(true);

      const data = await dockerServiceRef.current.checkHealth();
      
      if (!isMountedRef.current) return;

      console.log('🔍 useDocker: Initial check completed', data);
      
      setState(prev => ({
        ...prev,
        data,
        isLoading: false,
        error: null,
        lastUpdated: new Date(),
        isAvailable: data.is_available,
        status: data.is_available ? 'success' : 'error',
        eventStreamActive: dockerServiceRef.current?.isEventStreamActive() ?? false,
      }));

      configRef.current.onSuccess(data);
      hasInitializedRef.current = true;

    } catch (error) {
      if (!isMountedRef.current) return;

      console.error('🔍 useDocker: Initial check failed', error);
      
      const dockerError = error instanceof DockerServiceError 
        ? error 
        : DockerServiceError.fromTauriError(error);

      setState(prev => ({
        ...prev,
        isLoading: false,
        error: dockerError,
        status: 'error',
        eventStreamActive: false,
      }));

      configRef.current.onError(dockerError);
      hasInitializedRef.current = true;
    } finally {
      configRef.current.onLoadingChange(false);
    }
  }, []); // Empty dependency array - stable reference

  /**
   * Manual refresh function
   */
  const refresh = useCallback(async (): Promise<void> => {
    console.log('🔍 useDocker: Manual refresh requested');
    
    if (!dockerServiceRef.current) return;
    
    try {
      setState(prev => ({ ...prev, isLoading: true, status: 'loading' }));
      
      const data = await dockerServiceRef.current.checkHealth();
      
      if (!isMountedRef.current) return;
      
      setState(prev => ({
        ...prev,
        data,
        isLoading: false,
        error: null,
        lastUpdated: new Date(),
        isAvailable: data.is_available,
        status: data.is_available ? 'success' : 'error',
        eventStreamActive: dockerServiceRef.current?.isEventStreamActive() ?? false,
      }));
      
      configRef.current.onSuccess(data);
      
    } catch (error) {
      if (!isMountedRef.current) return;
      
      const dockerError = error instanceof DockerServiceError 
        ? error 
        : DockerServiceError.fromTauriError(error);
      
      setState(prev => ({
        ...prev,
        isLoading: false,
        error: dockerError,
        status: 'error',
      }));
      
      configRef.current.onError(dockerError);
    }
  }, []); // Empty dependency array - stable reference

  /**
   * Clear error state
   */
  const clearError = useCallback(() => {
    setState(prev => ({ ...prev, error: null }));
  }, []);

  /**
   * Start event streaming
   */
  const startEventStream = useCallback(async (): Promise<void> => {
    if (!dockerServiceRef.current) return;
    
    try {
      await dockerServiceRef.current.checkHealth();
      setState(prev => ({ 
        ...prev, 
        eventStreamActive: dockerServiceRef.current?.isEventStreamActive() ?? false 
      }));
    } catch (error) {
      console.error('Failed to start event stream:', error);
    }
  }, []);

  /**
   * Stop event streaming
   */
  const stopEventStream = useCallback(() => {
    if (!dockerServiceRef.current) return;
    
    dockerServiceRef.current.stopEventStream();
    setState(prev => ({ ...prev, eventStreamActive: false }));
  }, []);

  /**
   * Update configuration
   */
  const updateConfig = useCallback((newConfig: Partial<UseDockerConfig>) => {
    if (!dockerServiceRef.current) return;
    
    dockerServiceRef.current.updateConfig(newConfig);
  }, []);

  // ============================================================================
  // MEMOIZED VALUES
  // ============================================================================
  
  // Memoize the return value to prevent unnecessary re-renders
  const returnValue = useMemo((): UseDockerReturn => ({
    // State
    ...state,
    
    // Actions
    refresh,
    clearError,
    startEventStream,
    stopEventStream,
    updateConfig,
  }), [
    state,
    refresh,
    clearError,
    startEventStream,
    stopEventStream,
    updateConfig,
  ]);

  // ============================================================================
  // EFFECTS
  // ============================================================================
  
  /**
   * Initialize hook on mount - only run once
   */
  useEffect(() => {
    console.log('🔍 useDocker: Hook mounted');
    isMountedRef.current = true;
    hasInitializedRef.current = false;
    
    // Show checking state immediately
    setState(prev => ({
      ...prev,
      isLoading: true,
      status: 'loading',
    }));
    
    // Wait for backend grace period to complete, then check status
    const timer = setTimeout(() => {
      if (isMountedRef.current) {
        performInitialCheck();
      }
    }, 3500); // Wait 3.5 seconds to allow backend grace period (3s) + buffer
    
    // Cleanup on unmount
    return () => {
      console.log('🔍 useDocker: Hook unmounting');
      isMountedRef.current = false;
      clearTimeout(timer);
      
      // Stop event streaming
      if (dockerServiceRef.current) {
        dockerServiceRef.current.stopEventStream();
      }
      
      // Cleanup event listeners
      eventListenersRef.current.forEach(unlisten => unlisten());
      eventListenersRef.current = [];
    };
  }, []); // Empty dependency array - only run on mount

  // ============================================================================
  // RETURN VALUE
  // ============================================================================
  
  return returnValue;
}

// ============================================================================
// EXPORTS
// ============================================================================

export default useDocker; 