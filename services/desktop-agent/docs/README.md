# RedSys Desktop Agent

## Architecture Overview

The RedSys Desktop Agent is built with Tauri 2.0, React, TypeScript, and follows enterprise-grade patterns for scalability and maintainability.

## Directory Structure

```
src/
├── components/          # React components
│   ├── Layout/         # Layout components (Sidebar, StatusBar, etc.)
│   ├── features/       # Feature-specific components
│   └── common/         # Reusable UI components
├── hooks/              # React hooks
├── services/           # Business logic and API services
├── types/              # TypeScript type definitions
├── constants/          # Application constants
├── utils/              # Utility functions
└── styles/             # Global styles

src-tauri/
├── src/
│   ├── models/         # Data models
│   ├── services/       # Backend services
│   └── lib.rs          # Main application entry
```

## Key Features

- **System Monitoring**: Real-time system resource monitoring
- **Event-Driven Architecture**: Uses Tauri events for frontend-backend communication
- **Type Safety**: Full TypeScript coverage with proper type definitions
- **Modular Design**: Clean separation of concerns with barrel exports
- **Enterprise Patterns**: Follows industry best practices for scalability

## Development Guidelines

1. **Naming Conventions**: Use PascalCase for components, camelCase for functions/variables
2. **File Organization**: Group related functionality in dedicated directories
3. **Type Safety**: Always define proper TypeScript interfaces
4. **Barrel Exports**: Use index.ts files for clean imports
5. **Error Handling**: Implement proper error boundaries and error states 