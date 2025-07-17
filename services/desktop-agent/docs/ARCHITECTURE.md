# RedSys Desktop Agent Architecture

## Overview

The RedSys Desktop Agent follows a modular architecture with clear separation of concerns, consistent naming conventions, and a professional design system. This document outlines the structure and best practices.

## Directory Structure

```
src/
├── components/           # React components
│   ├── Layout/          # Layout components (Sidebar, StatusBar, etc.)
│   │   └── StatusBar/   # Status bar specific components
│   ├── common/          # Reusable UI components
│   └── features/        # Feature-specific components
├── constants/           # Application constants
│   ├── app.ts          # App-specific constants
│   ├── tooltip.ts      # Tooltip styling constants
│   └── index.ts        # Barrel exports
├── hooks/              # Custom React hooks
├── services/           # External service integrations
├── types/              # TypeScript type definitions
├── utils/              # Utility functions
│   ├── tooltipUtils.ts # Tooltip content generators
│   └── index.ts        # Barrel exports
└── App.css             # Global styles and CSS variables
```

## Design System

### Color Palette

The application uses a consistent dark theme color palette:

```css
:root {
  --dark-bg: #0f0f23;           /* Background */
  --dark-editor: #1e1e2e;       /* Sidebar, StatusBar, Tooltip background */
  --dark-border: #313244;       /* Borders and separators */
  --dark-text: #cdd6f4;         /* Primary text */
  --dark-textMuted: #6c7086;    /* Secondary/muted text */
  --dark-highlight: #313244;    /* Hover states */
}
```

### Tooltip Design

All tooltips follow a consistent design pattern:

- **Background**: `#1e1e2e` (matches sidebar/statusbar)
- **Border**: `#313244` with 1px solid
- **Header**: Bold text with bottom separator
- **Content**: Organized in rows with labels and values
- **Status indicators**: Colored dots for status visualization

## Modular Architecture

### 1. Constants (`src/constants/`)

Centralized configuration and styling constants:

```typescript
// constants/tooltip.ts
export const TOOLTIP_STYLES = {
  container: "padding: 16px; min-width: 240px; ...",
  header: "font-weight: 600; font-size: 14px; ...",
  // ... more styles
} as const;

export const TIPPY_CONFIG = {
  placement: "top",
  delay: [300, 0],
  // ... more config
} as const;
```

### 2. Utilities (`src/utils/`)

Reusable utility functions:

```typescript
// utils/tooltipUtils.ts
export const generateDockerTooltipContent = (docker: ProcessedDockerStatus): string => {
  // Generates consistent tooltip HTML
};

export const generateVersionTooltipContent = (version: string): string => {
  // Generates version tooltip HTML
};
```

### 3. Types (`src/types/`)

TypeScript type definitions with discriminated unions:

```typescript
// types/docker.ts
export type DockerStatusPayload =
  | { type: "Running"; version: string }
  | { type: "Stopped" }
  | { type: "Error"; message: string }
  | { type: "Initializing" };

export interface ProcessedDockerStatus {
  status: "Running" | "Stopped" | "Error" | "Initializing" | "Unknown";
  color: string;
  version: string | null;
  message: string | null;
}
```

### 4. Components (`src/components/`)

React components organized by feature and responsibility:

#### Layout Components
- `Layout/AppLayout.tsx` - Main application layout
- `Layout/Sidebar.tsx` - Sidebar component
- `Layout/StatusBar.tsx` - Status bar container
- `Layout/StatusBar/DockerStatusBarItem.tsx` - Docker status display
- `Layout/StatusBar/VersionStatusBarItem.tsx` - Version information display

#### Component Structure
Each component follows this pattern:
```typescript
/**
 * Component Name
 *
 * Brief description of the component's purpose and features.
 *
 * References:
 * - [External links to documentation]
 */

import React, { useEffect, useState, useRef, useCallback } from "react";
// ... other imports

/**
 * Component description
 */
export const ComponentName: React.FC<ComponentProps> = ({ props }) => {
  // State and refs
  const [state, setState] = useState<StateType>(initialState);
  const ref = useRef<RefType>(null);

  // Memoized functions
  const memoizedFunction = useCallback(() => {
    // Implementation
  }, [dependencies]);

  // Effects with clear purposes
  useEffect(() => {
    // Effect description
  }, [dependencies]);

  return (
    // JSX with consistent styling
  );
};
```

## Naming Conventions

### Files and Directories
- **Components**: PascalCase (`DockerStatusBarItem.tsx`)
- **Utilities**: camelCase (`tooltipUtils.ts`)
- **Constants**: camelCase (`tooltip.ts`)
- **Types**: camelCase (`docker.ts`)
- **Directories**: camelCase (`StatusBar/`)

### Functions and Variables
- **Components**: PascalCase (`DockerStatusBarItem`)
- **Functions**: camelCase (`generateDockerTooltipContent`)
- **Constants**: UPPER_SNAKE_CASE (`TOOLTIP_STYLES`)
- **Types**: PascalCase (`ProcessedDockerStatus`)
- **Interfaces**: PascalCase with `Props` suffix (`VersionStatusBarItemProps`)

### CSS Classes
- **Tailwind**: kebab-case (`bg-dark-editor`)
- **Custom**: kebab-case (`tippy-custom`)

## Best Practices

### 1. Separation of Concerns
- **Constants**: Centralized configuration
- **Utilities**: Reusable business logic
- **Types**: Type safety and contracts
- **Components**: UI presentation and interaction

### 2. Modularity
- Each file has a single responsibility
- Barrel exports for clean imports
- Consistent interfaces across components

### 3. Type Safety
- Discriminated unions for complex state
- Strict TypeScript configuration
- Proper type annotations

### 4. Performance
- Memoized callbacks with proper dependencies
- Efficient re-renders
- Proper cleanup in useEffect

### 5. Maintainability
- Comprehensive documentation
- Consistent code style
- Clear naming conventions

## Integration with Backend

The frontend integrates with the Rust backend through:

1. **Tauri Commands**: `invoke("get_docker_status")`
2. **Tauri Events**: `listen("docker_status_changed")`
3. **Type Safety**: Shared TypeScript/Rust types
4. **Error Handling**: Graceful fallbacks and user feedback

## Testing and Development

### Development Commands
```bash
npm run dev          # Development server
npm run build        # Production build
npm run tauri dev    # Tauri development
npm run tauri build  # Tauri production build
```

### Code Quality
- TypeScript strict mode
- ESLint configuration
- Prettier formatting
- Comprehensive documentation

## Future Enhancements

1. **Component Library**: Extract reusable components
2. **Theme System**: Support for multiple themes
3. **Internationalization**: Multi-language support
4. **Accessibility**: ARIA labels and keyboard navigation
5. **Testing**: Unit and integration tests 