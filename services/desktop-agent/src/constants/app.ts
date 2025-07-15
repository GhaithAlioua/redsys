// Application constants
export const APP_CONSTANTS = {
  VERSION: '0.1.0',
  NAME: 'RedSys Desktop Agent',
  DESCRIPTION: 'GPU Provider Client',
} as const;

// Status constants for status bar items
export const STATUS_CONSTANTS = {
  DOCKER: {
    RUNNING: 'Running',
    STOPPED: 'Stopped',
    ERROR: 'Error',
  },
} as const; 