#!/bin/bash
# Development watch script for Redsys Backend
# Enterprise-grade auto-rebuild with comprehensive error handling

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

log "👀 Starting development watch mode..."
log "📁 Watching for changes in /app/services/backend/"

# Global variables
BACKEND_PID=""
BUILD_IN_PROGRESS=false

# Function to kill existing process
kill_backend() {
    if [ -n "$BACKEND_PID" ] && kill -0 "$BACKEND_PID" 2>/dev/null; then
        log "🛑 Stopping existing backend process (PID: $BACKEND_PID)..."
        kill "$BACKEND_PID" 2>/dev/null || true
        wait "$BACKEND_PID" 2>/dev/null || true
        BACKEND_PID=""
    fi
}

# Function to build and run
build_and_run() {
    if [ "$BUILD_IN_PROGRESS" = true ]; then
        warning "Build already in progress, skipping..."
        return
    fi
    
    BUILD_IN_PROGRESS=true
    
    log "🔄 Changes detected, rebuilding..."
    
    # Kill existing process
    kill_backend
    
    # Build the application
    cd /app/build
    
    # Clean previous build artifacts
    log "🧹 Cleaning previous build..."
    rm -rf CMakeCache.txt CMakeFiles/ 2>/dev/null || true
    
    log "📋 Configuring with CMake..."
    if ! cmake /app/services/backend \
        -DCMAKE_BUILD_TYPE=Debug \
        -DCMAKE_CXX_FLAGS="-g -O0 -DDEBUG -fstack-protector-strong" \
        -DCMAKE_EXPORT_COMPILE_COMMANDS=ON; then
        error "CMake configuration failed!"
        BUILD_IN_PROGRESS=false
        return
    fi
    
    log "🏗️ Building application..."
    if make -j$(nproc); then
        success "Build successful!"
        
        # Check if executable exists
        if [ -f "/app/build/redsys-backend" ]; then
            # Start the application in background
            log "🚀 Starting Redsys Backend..."
            /app/build/redsys-backend &
            BACKEND_PID=$!
            log "📊 Backend running with PID: $BACKEND_PID"
            log "🔧 Health check available at http://localhost:8080/health"
        else
            error "Build succeeded but executable not found!"
        fi
    else
        error "Build failed!"
    fi
    
    BUILD_IN_PROGRESS=false
}

# Set up signal handling
cleanup() {
    log "🛑 Shutting down development watch mode..."
    kill_backend
    exit 0
}

trap cleanup SIGTERM SIGINT

# Initial build
build_and_run

# Watch for file changes
log "👀 Watching for file changes..."
log "📝 Supported file extensions: .cpp, .h, .hpp, .cc, .cxx"
log "🛑 Press Ctrl+C to stop"

inotifywait -m -r -e modify,create,delete /app/services/backend/ | while read -r path action file; do
    log "📝 $action: $path$file"
    
    # Only rebuild for source files
    if [[ "$file" =~ \.(cpp|h|hpp|cc|cxx)$ ]]; then
        build_and_run
    fi
done 