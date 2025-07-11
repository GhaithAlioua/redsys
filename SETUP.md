# 🚀 Redsys Setup Guide

## Quick Start

Get the Redsys decentralized GPU compute marketplace running in minutes with our **enterprise-grade foundation**!

## 🎉 What's New

**✅ Enterprise-Grade Foundation Complete**
- **OpenAPI 3.1.0** specification with Swagger UI
- **Industry-standard** service communication patterns
- **No shell scripts** - pure Docker Compose enterprise patterns
- **Consistent naming** conventions (`redsys-*`)
- **Proper health checks** and monitoring
- **Database migrations** with Flyway

## 📋 Prerequisites

Before you begin, ensure you have the following installed:

### Required Software
- **Docker Desktop** (v20.10+) - [Download here](https://www.docker.com/products/docker-desktop)
- **Docker Compose** (v2.0+) - Usually included with Docker Desktop
- **Git** (v2.30+) - [Download here](https://git-scm.com/downloads)

### System Requirements
- **OS**: Windows 10/11, macOS 10.15+, or Ubuntu 18.04+
- **RAM**: Minimum 8GB (16GB recommended)
- **Storage**: At least 10GB free space
- **Network**: Internet connection for Docker image downloads

### Verify Installation
```bash
# Check Docker
docker --version
docker-compose --version

# Check Git
git --version
```

## 🚀 Installation Steps

### Step 1: Clone the Repository
```bash
git clone https://github.com/GhaithAlioua/redsys.git
cd redsys
```

### Step 2: Start the Services (No Configuration Required!)

The system is pre-configured with sensible defaults for development. Simply run:

```bash
# Start all services (enterprise-grade setup)
docker-compose up -d --build
```

**That's it!** The system will:
- ✅ Build all services with proper dependencies
- ✅ Run database migrations automatically
- ✅ Start all services in the correct order
- ✅ Configure OAuth2 authentication
- ✅ Set up API Gateway with security headers
- ✅ Enable OpenAPI documentation

### Step 3: Verify Installation
Wait for all services to start (this may take 2-3 minutes on first run). You can monitor the progress with:

```bash
# Check service status
docker-compose ps

# View logs
docker-compose logs -f
```

## 🌐 Accessing the Services

Once all services are running, you can access:

| Service | URL | Description |
|---------|-----|-------------|
| **API Gateway** | http://localhost | Main entry point with security |
| **Backend API** | http://localhost:8080 | Core marketplace API |
| **API Documentation** | http://localhost/docs | **Swagger UI** - Interactive API docs |
| **OpenAPI Spec** | http://localhost/openapi.yaml | Raw OpenAPI specification |
| **Hydra Admin** | http://localhost:4444 | OAuth2 administration |
| **Hydra Public** | http://localhost:4445 | OAuth2 public endpoints |
| **Oathkeeper** | http://localhost:4456 | API Gateway authentication |

### Health Check Endpoints
- **API Gateway Health**: http://localhost/health
- **Backend Health**: http://localhost:8080/health
- **Hydra Health**: http://localhost:4444/health/ready
- **Oathkeeper Health**: http://localhost:4456/health/alive

## 🧪 Testing the Setup

### Test API Gateway Health
```bash
curl http://localhost/health
```
Expected response: `healthy`

### Test Backend Health
```bash
curl http://localhost:8080/health
```
Expected response:
```json
{
  "status": "healthy",
  "service": "redsys-backend",
  "timestamp": "1234567890",
  "version": "1.0.0",
  "environment": "development"
}
```

### Test API Endpoints
```bash
# Test hello endpoint
curl http://localhost/api/v1/hello

# Test users endpoint
curl http://localhost/api/v1/users

# Test jobs endpoint
curl http://localhost/api/v1/jobs

# Test providers endpoint
curl http://localhost/api/v1/providers
```

### Test API Documentation
```bash
# Access Swagger UI
curl http://localhost/docs

# Access OpenAPI spec
curl http://localhost/openapi.yaml
```

## 📚 API Documentation

### Swagger UI
Visit **http://localhost/docs** to access the interactive API documentation. This provides:
- **All available endpoints** with descriptions
- **Interactive testing** - test endpoints directly from the browser
- **Request/response schemas** with examples
- **Authentication information** and requirements

### OpenAPI Specification
The OpenAPI 3.1.0 specification is available at:
- **Raw spec**: http://localhost/openapi.yaml
- **Infrastructure location**: `infrastructure/api/openapi.yaml`

This follows **industry standards** used by Google, Netflix, and Uber APIs.

## 🏗️ Architecture Overview

### Service Communication
```
Internet → API Gateway → Oathkeeper → Backend
                    ↓
                Hydra (OAuth2)
                    ↓
                Hydra DB (isolated)
```

### Enterprise Features
- **Security-first**: All requests go through authentication proxy
- **Database isolation**: Separate DBs for app vs OAuth2
- **Health monitoring**: All services have health endpoints
- **Resource management**: Memory and CPU limits configured
- **No shell scripts**: Pure Docker Compose enterprise patterns

## 🛠️ Development Workflow

### Starting Development
```bash
# Start all services
docker-compose up -d

# View logs in real-time
docker-compose logs -f backend
```

### Stopping Services
```bash
# Stop all services
docker-compose down

# Stop and remove volumes (WARNING: This deletes all data)
docker-compose down -v
```

### Rebuilding Services
```bash
# Rebuild all services
docker-compose up -d --build

# Rebuild specific service
docker-compose build backend
docker-compose up -d backend
```

### Viewing Logs
```bash
# All services
docker-compose logs -f

# Specific service
docker-compose logs -f backend
docker-compose logs -f api-gateway
docker-compose logs -f oathkeeper
```

## 📁 Project Structure

```
redsys/
├── docker-compose.yml              # Enterprise Compose file
├── services/
│   ├── backend/                    # C++20 + Drogon backend
│   │   ├── src/                    # Source code
│   │   ├── include/                # Headers
│   │   └── config.json             # Configuration
│   └── api-gateway/                # Nginx API gateway
│       ├── nginx.conf              # Main config
│       └── conf.d/                 # Route configs
├── infrastructure/
│   ├── api/                        # OpenAPI specifications
│   │   └── openapi.yaml            # API documentation
│   ├── docker/                     # Dockerfiles
│   │   ├── Dockerfile.backend.dev      # Development backend
│   │   ├── Dockerfile.backend.prod     # Production backend
│   │   ├── Dockerfile.gateway.dev      # Development gateway
│   │   └── Dockerfile.gateway.prod     # Production gateway
│   ├── oathkeeper/                 # Authentication proxy
│   │   ├── config.yaml             # Oathkeeper config
│   │   └── access-rules.yaml       # Authorization rules
│   └── security/                   # Security configurations
├── shared/
│   └── database/                   # Database schemas and migrations
│       ├── migrations/             # Flyway migrations
│       ├── schema.sql              # Database schema
│       └── flyway.conf             # Migration config
└── logs/                           # Application logs
```

## 🔧 Configuration

### Environment Variables
The system uses sensible defaults for development. For production, you can set:

```bash
# Database passwords
HYDRA_DB_PASSWORD=your_secure_password
HYDRA_SECRETS_SYSTEM=your_secure_secret

# OAuth2 configuration
OAUTH2_CLIENT_SECRET=your_client_secret
```

### Production Configuration
For production deployment:
1. **Enable HTTPS** by uncommenting HTTPS config in API Gateway
2. **Enable OAuth2 authentication** in Oathkeeper access rules
3. **Set secure passwords** for all databases
4. **Configure SSL certificates** for HTTPS

## 🚨 Troubleshooting

### Common Issues

**Services not starting:**
```bash
# Check Docker resources
docker system df
docker system prune

# Rebuild from scratch
docker-compose down -v
docker-compose up -d --build
```

**API Gateway not responding:**
```bash
# Check if all services are healthy
docker-compose ps

# Check API Gateway logs
docker-compose logs api-gateway
```

**Database connection issues:**
```bash
# Check database logs
docker-compose logs redsys-postgres-db
docker-compose logs redsys-hydra-db

# Check migration status
docker-compose logs flyway
```

### Getting Help
- **Check logs**: `docker-compose logs -f [service-name]`
- **Verify health**: All services have `/health` endpoints
- **Test endpoints**: Use the provided curl commands above
- **API documentation**: Visit http://localhost/docs for interactive testing

## 🎯 Next Steps

### For Development
1. **Explore the API**: Visit http://localhost/docs
2. **Test endpoints**: Use the provided curl commands
3. **Check health**: Verify all services are running
4. **Start coding**: Begin implementing your marketplace features

### For Production
1. **Enable HTTPS**: Configure SSL certificates
2. **Enable OAuth2**: Switch from anonymous to OAuth2 authentication
3. **Set secure passwords**: Update all default passwords
4. **Configure monitoring**: Set up proper logging and alerting
5. **Scale services**: Configure resource limits and scaling

---

**🎉 Congratulations!** Your Redsys enterprise-grade foundation is now running and ready for development! 