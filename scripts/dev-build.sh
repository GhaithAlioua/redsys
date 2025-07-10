#!/bin/bash
# Development build script for Redsys Backend
# Enterprise-grade build process with error handling and validation

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Logging function
log() {
    echo -e "${BLUE}[$(date +'%Y-%m-%d %H:%M:%S')]${NC} $1"
}

error() {
    echo -e "${RED}[ERROR]${NC} $1" >&2
}

success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

# Check if source directory exists
if [ ! -d "/app/services/backend" ]; then
    error "Source directory /app/services/backend not found!"
    error "Make sure the volume mount is correct in docker-compose.yml"
    exit 1
fi

log "🔨 Building Redsys Backend (Development Mode)..."

# Create build directory if it doesn't exist
mkdir -p /app/build

# Navigate to build directory
cd /app/build

# Clean previous build artifacts
log "🧹 Cleaning previous build..."
rm -rf CMakeCache.txt CMakeFiles/ 2>/dev/null || true

# Configure with CMake (Debug mode)
log "📋 Configuring with CMake..."
if ! cmake /app/services/backend \
    -DCMAKE_BUILD_TYPE=Debug \
    -DCMAKE_CXX_FLAGS="-g -O0 -DDEBUG -fstack-protector-strong" \
    -DCMAKE_EXPORT_COMPILE_COMMANDS=ON; then
    error "CMake configuration failed!"
    exit 1
fi

# Build the application
log "🏗️ Building application..."
if ! make -j$(nproc); then
    error "Build failed!"
    exit 1
fi

success "Build completed successfully!"

# Check if executable exists
if [ -f "/app/build/redsys-backend" ]; then
    log "🚀 Starting Redsys Backend..."
    log "📊 Debug mode enabled"
    log "🔍 Logs will be written to /app/logs/"
    log "🔧 Health check available at http://localhost:8080/health"
    
    # Set up signal handling
    trap 'log "Received signal, shutting down..."; kill $BACKEND_PID 2>/dev/null; exit 0' SIGTERM SIGINT
    
    # Run the application
    exec /app/build/redsys-backend &
    BACKEND_PID=$!
    
    # Wait for the process
    wait $BACKEND_PID
else
    error "Build failed: executable not found"
    error "Expected: /app/build/redsys-backend"
    exit 1
fi 