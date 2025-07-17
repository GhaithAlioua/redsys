/**
 * Docker Status Bar Item Component
 *
 * Displays real-time Docker daemon status with professional tooltip.
 * Uses React Bits patterns for clean separation of concerns.
 *
 * Features:
 * - Real-time status monitoring
 * - Professional tooltip design
 * - Type-safe communication
 * - Clean, simple implementation
 * - React Bits patterns for reusability
 *
 * References:
 * - [React useEffect](https://react.dev/reference/react/useEffect)
 * - [Tauri invoke](https://tauri.app/v2/api/js/core/#invoke)
 * - [Tauri event.listen](https://tauri.app/v2/api/js/event/#listen)
 * - [Tippy.js](https://atomiks.github.io/tippyjs/)
 * - [React Bits](https://reactbits.dev/)
 */

import React, { useCallback } from "react";
import { useDockerStatus, useTippy } from "../../../hooks";
import { GlowingCircle, StatusBarItemWrapper } from "../../common";

/**
 * Docker Status Bar Item Component
 */
export const DockerStatusBarItem: React.FC = () => {
  // React Bit: Custom hook for Docker status management
  const { docker, isLoading } = useDockerStatus();

  // React Bit: Memoized tooltip content generator
  const getTooltipContent = useCallback(() => {
    if (isLoading) {
      return `
        <div style="
          background: #1e1e1e;
          border: 1px solid rgba(255, 255, 255, 0.08);
          padding: 0;
          min-width: 300px;
          font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
          color: #cccccc;
          position: relative;
          overflow: hidden;
        ">
          <!-- Top highlight line -->
          <div style="
            position: absolute;
            top: 0;
            left: 0;
            right: 0;
            height: 1px;
            background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.1), transparent);
            pointer-events: none;
          "></div>
          
          <!-- Header Section -->
          <div style="
            padding: 20px 24px 16px 24px;
            border-bottom: 1px solid rgba(255, 255, 255, 0.06);
          ">
            <div style="
              font-weight: 600;
              font-size: 14px;
              color: #ffffff;
              letter-spacing: 0.2px;
              text-transform: uppercase;
              font-size: 11px;
              opacity: 0.8;
            ">
              Docker Information
            </div>
          </div>
          
          <!-- Content Section -->
          <div style="padding: 16px 24px 20px 24px;">
            <div style="
              display: flex;
              align-items: center;
              justify-content: center;
              color: #969696;
              font-size: 13px;
              font-weight: 400;
            ">
              Checking Docker status...
            </div>
          </div>
        </div>
      `;
    }

    const isRunning = docker.status === "Running" && docker.version;
    return `
      <div style="
        background: #1e1e1e;
        border: 1px solid rgba(255, 255, 255, 0.08);
        padding: 0;
        min-width: 300px;
        font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
        color: #cccccc;
        position: relative;
        overflow: hidden;
      ">
        <!-- Top highlight line -->
        <div style="
          position: absolute;
          top: 0;
          left: 0;
          right: 0;
          height: 1px;
          background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.1), transparent);
          pointer-events: none;
        "></div>
        
        <!-- Header Section -->
        <div style="
          padding: 20px 24px 16px 24px;
          border-bottom: 1px solid rgba(255, 255, 255, 0.06);
        ">
          <div style="
            font-weight: 600;
            font-size: 14px;
            color: #ffffff;
            letter-spacing: 0.2px;
            text-transform: uppercase;
            font-size: 11px;
            opacity: 0.8;
          ">
            Docker Information
          </div>
        </div>
        
        <!-- Content Section -->
        <div style="padding: 16px 24px 20px 24px;">
          <!-- Status Row -->
          <div style="
            display: flex;
            align-items: center;
            justify-content: space-between;
            margin-bottom: 12px;
          ">
            <span style="
              color: #969696;
              font-size: 13px;
              font-weight: 400;
            ">
              Daemon Status
            </span>
            <div style="
              display: flex;
              align-items: center;
              gap: 8px;
            ">
              <div style="
                width: 8px;
                height: 8px;
                border-radius: 50%;
                background-color: ${docker.color};
                box-shadow: 0 0 0 2px ${docker.color}20;
                ${
                  isRunning
                    ? "animation: pulse 2s infinite;"
                    : "animation: pulse 3s infinite;"
                }
              "></div>
              <span style="
                color: #ffffff;
                font-size: 13px;
                font-weight: 500;
              ">
                ${isRunning ? "Running" : "Not Running"}
              </span>
            </div>
          </div>
          
          ${
            isRunning
              ? `
            <!-- Version Row -->
            <div style="
              display: flex;
              align-items: center;
              justify-content: space-between;
              padding-top: 12px;
              border-top: 1px solid rgba(255, 255, 255, 0.04);
            ">
              <span style="
                color: #969696;
                font-size: 13px;
                font-weight: 400;
              ">
                Engine Version
              </span>
              <span style="
                color: #ffffff;
                font-size: 13px;
                font-weight: 500;
                font-family: 'SF Mono', Monaco, 'Cascadia Code', 'Roboto Mono', Consolas, 'Courier New', monospace;
                background: rgba(255, 255, 255, 0.06);
                padding: 4px 8px;
                border-radius: 4px;
              ">
                ${docker.version}
              </span>
            </div>
          `
              : ""
          }
        </div>
      </div>
    `;
  }, [docker, isLoading]);

  // React Bit: Custom hook for tooltip management
  const tooltipRef = useTippy({
    content: getTooltipContent,
    dependencies: [docker.status, docker.version, docker.color, isLoading],
  });

  return (
    <StatusBarItemWrapper tooltipRef={tooltipRef}>
      <GlowingCircle
        color={docker.color}
        size="md"
        glowIntensity="medium"
        isPulsing={docker.status === "Running" || docker.status === "Stopped"}
        animationType={docker.status === "Running" ? "pulse" : "slow-pulse"}
        className="mr-2.5"
      />
      <span className="text-xs font-medium text-[#cccccc] transition-colors duration-200">
        Docker
      </span>
    </StatusBarItemWrapper>
  );
};
