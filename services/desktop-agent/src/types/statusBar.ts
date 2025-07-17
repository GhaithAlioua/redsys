/**
 * StatusBar Types
 * 
 * Shared types and interfaces for StatusBar components.
 */

import React from "react";

/**
 * Base interface for all status bar items
 */
export interface StatusBarItemProps {
  children?: React.ReactNode;
  className?: string;
  tooltipRef?: React.RefObject<HTMLDivElement>;
}

/**
 * Status bar item configuration for dynamic rendering
 */
export interface StatusBarItemConfig {
  id: string;
  component: React.ComponentType<any>;
  props?: Record<string, any>;
  order?: number;
}

/**
 * Status bar layout configuration
 */
export interface StatusBarConfig {
  items: StatusBarItemConfig[];
  className?: string;
} 