/**
 * StatusBar Utils
 * 
 * Helper functions for StatusBar components.
 */

import { StatusBarItemConfig } from '../types/statusBar';

/**
 * Sort status bar items by their order property
 */
export const sortStatusBarItems = (items: StatusBarItemConfig[]): StatusBarItemConfig[] => {
  return [...items].sort((a, b) => (a.order || 0) - (b.order || 0));
};

/**
 * Validate status bar item configuration
 */
export const validateStatusBarItem = (item: StatusBarItemConfig): boolean => {
  return !!(item.id && item.component);
};

/**
 * Generate unique ID for status bar items
 */
export const generateStatusBarItemId = (prefix: string): string => {
  return `${prefix}-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;
}; 