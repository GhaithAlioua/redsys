/**
 * useDockerStatus Hook - React Bit Pattern
 *
 * A custom hook that manages Docker status state and events following React Bits patterns.
 * This separates business logic from UI rendering logic.
 *
 * React Bits Pattern: Custom Hook for State Management
 * Reference: https://reactbits.dev/
 */

import { useEffect, useState, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import type {
  DockerStatusPayload,
  ProcessedDockerStatus,
} from "../types/docker";
import { processDockerPayload } from "../types/docker";

export const useDockerStatus = () => {
  const [docker, setDocker] = useState<ProcessedDockerStatus>({
    status: "Initializing",
    color: "#f59e42",
    version: null,
    message: null,
  });

  // React Bit: Memoized status fetcher
  const fetchDockerStatus = useCallback(async () => {
    try {
      const payload = await invoke<DockerStatusPayload>("get_docker_status");
      console.log("Initial Docker payload:", payload);
      const processed = processDockerPayload(payload);
      console.log("Processed Docker status:", processed);
      setDocker(processed);
    } catch (error) {
      console.error("Failed to fetch initial Docker status:", error);
      setDocker({
        status: "Error",
        color: "#ef4444",
        version: null,
        message: "Failed to connect to backend",
      });
    }
  }, []);

  // React Bit: Memoized event handler
  const handleDockerStatusChange = useCallback((event: { payload: DockerStatusPayload }) => {
    console.log("Docker status changed event:", event.payload);
    const processed = processDockerPayload(event.payload);
    console.log("Processed status from event:", processed);
    setDocker(processed);
  }, []);

  // React Bit: Side effect for initial status
  useEffect(() => {
    fetchDockerStatus();
  }, [fetchDockerStatus]);

  // React Bit: Side effect for event listeners
  useEffect(() => {
    let unlisten: UnlistenFn | undefined;
    
    const setupEventListeners = async () => {
      try {
        unlisten = await listen<DockerStatusPayload>(
          "docker_status_changed",
          handleDockerStatusChange
        );
      } catch (error) {
        console.error("Failed to setup Docker status event listener:", error);
      }
    };

    setupEventListeners();

    return () => {
      if (unlisten) unlisten();
    };
  }, [handleDockerStatusChange]);

  return docker;
}; 