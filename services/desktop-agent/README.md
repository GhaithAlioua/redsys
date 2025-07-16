# RedSys Desktop Agent - Enterprise Docker Integration

A professional, enterprise-grade desktop agent with Docker integration built using Tauri 2.0, React, and TypeScript. This application provides robust Docker monitoring, system resource tracking, and a modern cross-platform interface.

## 🏗️ Architecture Overview

### Backend (Rust/Tauri)
- **Docker Service**: Bollard-based Docker daemon communication with retry logic and error handling
- **System Monitor**: Comprehensive system resource monitoring using sysinfo
- **Error Handling**: Professional error types with proper categorization and recovery
- **Logging**: Structured logging with tracing and proper log levels
- **Type Safety**: Strong typing throughout with serde serialization

### Frontend (React/TypeScript)
- **Enterprise Service Layer**: Type-safe Docker service with caching and retry logic
- **Custom Hooks**: Professional React hooks with proper state management
- **Component Architecture**: Modular, reusable components with proper separation of concerns
- **Error Boundaries**: Comprehensive error handling and user feedback
- **Performance**: Optimized rendering with memoization and proper cleanup

## 🚀 Features

### Docker Integration
- ✅ Real-time Docker daemon status monitoring
- ✅ Docker version detection and display
- ✅ Cross-platform Docker socket support (Windows, macOS, Linux)
- ✅ Professional error handling with retry logic
- ✅ Request caching with TTL for performance
- ✅ Auto-refresh capabilities with configurable intervals

### System Monitoring
- ✅ CPU usage tracking
- ✅ Memory utilization monitoring
- ✅ Disk space and usage statistics
- ✅ Network interface monitoring
- ✅ System load average tracking
- ✅ Process count monitoring
- ✅ System uptime tracking

### User Interface
- ✅ Modern, responsive design with Tailwind CSS
- ✅ Tippy.js tooltips for detailed information
- ✅ Lucide React icons for consistent visual design
- ✅ Status indicators with color-coded states
- ✅ Loading states and error feedback
- ✅ Click-to-refresh functionality

## 📦 Dependencies

### Backend Dependencies
```toml
# Core Tauri
tauri = "2"
tauri-plugin-opener = "2"
tauri-plugin-shell = "2"

# Docker Integration
bollard = "0.19.1"  # Docker API client
sysinfo = "0.36"    # System monitoring

# Async Runtime
tokio = "1.46"      # Async runtime with full features

# Serialization & Error Handling
serde = "1"         # Serialization framework
thiserror = "1"     # Error handling
anyhow = "1"        # Error propagation

# Logging & Observability
tracing = "0.1"     # Structured logging
tracing-subscriber = "0.3"  # Logging subscriber

# Utilities
chrono = "0.4"      # Date/time handling
uuid = "1"          # Unique identifiers
once_cell = "1"     # Lazy static initialization
```

### Frontend Dependencies
```json
{
  "dependencies": {
    "@tauri-apps/api": "^2.0.0",
    "@tippyjs/react": "^5.0.0",
    "lucide-react": "^0.400.0",
    "react": "^18.0.0",
    "react-dom": "^18.0.0"
  },
  "devDependencies": {
    "@types/react": "^18.0.0",
    "@types/react-dom": "^18.0.0",
    "typescript": "^5.0.0",
    "tailwindcss": "^3.0.0",
    "vite": "^5.0.0"
  }
}
```

## 🏛️ Code Architecture

### Backend Architecture

```
src-tauri/src/
├── main.rs          # Tauri application entry point
├── lib.rs           # Library exports and global state
├── docker.rs        # Docker service implementation
├── system.rs        # System monitoring service
├── error.rs         # Error types and handling
└── types.rs         # Type definitions and serialization
```

#### Docker Service (`docker.rs`)
- **Bollard Integration**: Professional Docker API client usage
- **Cross-Platform Support**: Windows named pipes, Unix sockets, HTTP fallback
- **Error Handling**: Comprehensive error categorization and recovery
- **Timeout Management**: Configurable timeouts with proper error reporting
- **Logging**: Structured logging with proper log levels

#### System Monitor (`system.rs`)
- **Resource Monitoring**: CPU, memory, disk, network, processes
- **Performance Optimization**: Efficient data collection and caching
- **Configuration**: Flexible monitoring configuration
- **Statistics**: Success/error tracking and performance metrics

### Frontend Architecture

```
src/
├── services/
│   └── docker.ts    # Enterprise Docker service layer
├── hooks/
│   └── useDocker.ts # Professional React hook
├── components/
│   └── Layout/
│       └── StatusBar/
│           └── DockerStatusItem.tsx  # Docker status component
└── types/
    └── layout.ts    # Type definitions
```

#### Docker Service (`services/docker.ts`)
- **Type Safety**: Comprehensive TypeScript interfaces
- **Error Handling**: Custom error types with proper categorization
- **Caching**: TTL-based request caching for performance
- **Retry Logic**: Exponential backoff with configurable attempts
- **Request Management**: Abort controller support for cancellation

#### Docker Hook (`hooks/useDocker.ts`)
- **State Management**: Professional React state handling
- **Memory Management**: Proper cleanup and memory leak prevention
- **Performance**: Memoization and optimized re-renders
- **Real-time Updates**: Configurable auto-refresh capabilities
- **Error Recovery**: Automatic error recovery and retry logic

## 🔧 Configuration

### Docker Service Configuration
```typescript
interface DockerServiceConfig {
  autoRefresh?: boolean;        // Enable auto-refresh
  refreshInterval?: number;     // Refresh interval (ms)
  retryOnError?: boolean;       // Enable retry on error
  maxRetries?: number;          // Maximum retry attempts
  retryDelay?: number;          // Base retry delay (ms)
  cacheTTL?: number;           // Cache TTL (ms)
  enableCache?: boolean;        // Enable request caching
}
```

### System Monitor Configuration
```rust
pub struct SystemMonitorConfig {
    pub refresh_interval: Duration,    // Refresh interval
    pub enable_cpu: bool,              // Enable CPU monitoring
    pub enable_memory: bool,           // Enable memory monitoring
    pub enable_disk: bool,             // Enable disk monitoring
    pub enable_network: bool,          // Enable network monitoring
    pub enable_processes: bool,        // Enable process monitoring
    pub max_processes: usize,          // Max processes to track
}
```

## 🚀 Usage Examples

### Using the Docker Hook
```typescript
import { useDocker } from '../hooks/useDocker';

function MyComponent() {
  const {
    data: dockerInfo,
    isLoading,
    error,
    isAvailable,
    refresh,
  } = useDocker({
    autoRefresh: true,
    refreshInterval: 30000,
    retryOnError: true,
    maxRetries: 3,
  });

  if (isLoading) return <div>Checking Docker...</div>;
  if (error) return <div>Error: {error.message}</div>;
  
  return (
    <div>
      <p>Docker Status: {dockerInfo?.daemon_status}</p>
      <p>Version: {dockerInfo?.version}</p>
      <button onClick={refresh}>Refresh</button>
    </div>
  );
}
```

### Using the Docker Service Directly
```typescript
import { DockerService } from '../services/docker';

const dockerService = new DockerService({
  enableCache: true,
  cacheTTL: 10000,
  retryOnError: true,
  maxRetries: 3,
});

try {
  const response = await dockerService.checkHealth();
  console.log('Docker is available:', response.data.is_available);
  console.log('Version:', response.data.version);
} catch (error) {
  console.error('Docker check failed:', error.message);
}
```

### Tauri Commands
```rust
// Check Docker health
#[tauri::command]
async fn check_docker_health() -> Result<DockerServiceInfo, String> {
    let mut docker_service = DockerService::new().await?;
    docker_service.check_health().await?;
    Ok(docker_service.get_service_info().await)
}

// Get system information
#[tauri::command]
async fn get_system_info() -> Result<SystemInfo, String> {
    let mut system_monitor = SystemMonitor::new(None);
    system_monitor.refresh().await
}
```

## 🧪 Testing

### Backend Tests
```bash
# Run all tests
cargo test

# Run specific test module
cargo test docker

# Run tests with output
cargo test -- --nocapture
```

### Frontend Tests
```bash
# Run tests
npm test

# Run tests with coverage
npm run test:coverage
```

## 🔍 Error Handling

### Error Types
- **DockerError**: Docker-specific errors with proper categorization
- **SystemError**: System monitoring errors
- **AppError**: General application errors
- **NetworkError**: Network-related errors
- **TimeoutError**: Operation timeout errors

### Error Recovery
- **Automatic Retry**: Configurable retry logic with exponential backoff
- **Graceful Degradation**: Fallback behavior when services are unavailable
- **User Feedback**: Clear error messages and status indicators
- **Logging**: Comprehensive error logging for debugging

## 📊 Performance

### Optimization Strategies
- **Request Caching**: TTL-based caching to reduce API calls
- **Memoization**: React component and hook optimization
- **Lazy Loading**: On-demand service initialization
- **Memory Management**: Proper cleanup and resource disposal
- **Async Operations**: Non-blocking operations with proper error handling

### Monitoring
- **Performance Metrics**: Request duration tracking
- **Cache Statistics**: Hit/miss ratio monitoring
- **Error Rates**: Success/failure tracking
- **Resource Usage**: Memory and CPU monitoring

## 🔒 Security

### Security Features
- **Input Validation**: Comprehensive input sanitization
- **Error Sanitization**: Safe error message handling
- **Resource Limits**: Configurable resource usage limits
- **Permission Handling**: Proper permission checking
- **Secure Communication**: Safe IPC and API communication

## 🚀 Deployment

### Build Commands
```bash
# Development
npm run tauri dev

# Production build
npm run tauri build

# Build for specific platform
npm run tauri build -- --target x86_64-unknown-linux-gnu
```

### Distribution
- **Cross-Platform**: Windows, macOS, Linux support
- **Auto-Updates**: Tauri auto-update integration
- **Code Signing**: Platform-specific code signing
- **Installation**: Native installer packages

## 🤝 Contributing

### Development Guidelines
1. **Code Style**: Follow Rust and TypeScript best practices
2. **Documentation**: Comprehensive code documentation
3. **Testing**: Unit and integration tests for all features
4. **Error Handling**: Proper error handling and logging
5. **Performance**: Optimize for performance and memory usage

### Code Review Checklist
- [ ] Type safety and proper error handling
- [ ] Performance optimization and memory management
- [ ] Comprehensive testing coverage
- [ ] Proper documentation and comments
- [ ] Security considerations
- [ ] Cross-platform compatibility

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- **Tauri Team**: For the excellent cross-platform framework
- **Bollard Maintainers**: For the robust Docker API client
- **Sysinfo Maintainers**: For the comprehensive system monitoring library
- **React Team**: For the powerful frontend framework
- **TypeScript Team**: For the type-safe JavaScript experience
