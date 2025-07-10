# 🚀 Redsys Setup Guide

## Quick Start

Get the Redsys decentralized GPU compute marketplace running in minutes!

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

### Step 2: Create Environment Configuration
Create a `.env` file in the project root with the following content:

```bash
# Redsys Environment Configuration
# Database Configuration
DB_PASSWORD=admin
HYDRA_DB_PASSWORD=hydrapassword

# OAuth2 Configuration
OAUTH2_CLIENT_SECRET=redsys_backend_client_secret_2024
HYDRA_SECRETS_SYSTEM=redsys_hydra_secrets_system_2024

# Database Connection Details
DB_HOST=db
DB_PORT=5432
DB_USER=redsys
DB_NAME=redsys_db

# Hydra Database Connection Details
HYDRA_DB_HOST=hydra-db
HYDRA_DB_PORT=5432
HYDRA_DB_USER=hydra
HYDRA_DB_NAME=hydra

# OAuth2 Client Configuration
OAUTH2_CLIENT_ID=redsys-backend
OAUTH2_INTROSPECTION_URL=http://hydra:4445/oauth2/introspect

# Security Configuration
HYDRA_CLIENT_SECRET=backend_secret_2024
```

**Note**: You can copy this content directly into a new `.env` file in the project root.

### Step 3: Start the Services

#### For Development (Recommended for local development)
```bash
# Start all services for development (uses .dev Dockerfiles)
docker-compose up -d

# Or start with logs visible (recommended for first run)
docker-compose up
```

#### For Production (Optimized for deployment)
```bash
# Start all services for production (uses .prod Dockerfiles, set build args or context as needed)
docker-compose up -d --build

# Or start with logs visible
docker-compose up --build
```

### Dockerfile Usage
- `.dev` Dockerfiles are used for development (fast, debug, hot reload)
- `.prod` Dockerfiles are used for production (optimized, secure)

### Step 4: Verify Installation
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
| **API Gateway** | http://localhost | Main entry point |
| **Backend API** | http://localhost:8080 | Core marketplace API |
| **Hydra Admin** | http://localhost:4444 | OAuth2 administration |
| **Hydra Public** | http://localhost:4445 | OAuth2 public endpoints |
| **Oathkeeper** | http://localhost:4456 | API Gateway authentication |

### Health Check Endpoints
- Backend Health: http://localhost:8080/health
- API Gateway Health: http://localhost/health
- Hydra Health: http://localhost:4444/health/ready
- Oathkeeper Health: http://localhost:4456/health/alive

## 🧪 Testing the Setup

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

### Test API Gateway
```bash
curl http://localhost/health
```
Expected response: `healthy`

## 🛠️ Development Workflow

### Starting Development
```bash
# Start services for development (uses .dev Dockerfiles by default)
docker-compose up -d

# View logs in real-time
docker-compose logs -f backend
```

### Starting Production
```bash
# Start services for production (uses .prod Dockerfiles)
docker-compose up -d --build

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
# Rebuild backend service after code changes
docker-compose build backend
docker-compose up -d backend
```

## 📁 Project Structure

```
redsys/
├── docker-compose.yml              # Single Compose file
├── .env                            # Environment variables (create this)
├── services/
│   ├── backend/                    # C++20 + Drogon backend
│   └── api-gateway/                # Nginx API gateway
├── infrastructure/
│   ├── docker/                     # Dockerfiles
│   │   ├── Dockerfile.backend.dev      # Development backend
│   │   ├── Dockerfile.backend.prod     # Production backend
│   │   ├── Dockerfile.gateway.dev      # Development gateway
│   │   └── Dockerfile.gateway.prod     # Production gateway
│   └── oathkeeper/                 # OAuth2 configuration
└── shared/
    └── database/                   # Database schemas
```

## 🔧 Configuration

### Environment Variables
The `.env` file contains all necessary configuration. Key variables:

- `DB_PASSWORD`: Main database password
- `HYDRA_DB_PASSWORD`: OAuth2 database password
- `OAUTH2_CLIENT_SECRET`: OAuth2 client secret
- `HYDRA_SECRETS_SYSTEM`: Hydra encryption key

### Docker Compose Services
- **api-gateway**: Nginx reverse proxy with OAuth2 authentication
- **backend**: C++20 backend service with Drogon framework
- **hydra**: OAuth2/OpenID Connect provider
- **hydra-db**: PostgreSQL database for OAuth2
- **oathkeeper**: API Gateway authentication middleware
- **db**: PostgreSQL database for application data

## 🐛 Troubleshooting

### Common Issues

#### 1. Port Already in Use
```bash
# Check what's using the port
netstat -ano | findstr :8080

# Stop conflicting services or change ports in docker-compose.yml
```

#### 2. Docker Not Running
```bash
# Start Docker Desktop
# On Windows/macOS: Launch Docker Desktop application
# On Linux: sudo systemctl start docker
```

#### 3. Insufficient Memory
```bash
# Check Docker memory allocation
# In Docker Desktop: Settings > Resources > Memory (increase to 8GB+)
```

#### 4. Services Not Starting
```bash
# Check logs for specific service
docker-compose logs backend
docker-compose logs hydra

# Restart specific service
docker-compose restart backend
```

#### 5. Database Connection Issues
```bash
# Wait for database to be ready
docker-compose logs db

# Check database health
docker-compose exec db pg_isready -U postgres
```

### Reset Everything
```bash
# Complete reset (WARNING: Deletes all data)
docker-compose down -v
docker system prune -a
docker-compose up -d
```

## 🔐 Security Notes

### Development vs Production
- **Current setup**: Development configuration with default passwords
- **Production**: Change all passwords and secrets
- **Secrets**: Use Docker secrets or external vaults in production

### Default Credentials (Development Only)
- Database: `postgres/admin`
- Hydra Database: `postgres/admin`
- OAuth2 Client: `redsys-backend/backend_secret_2024`

## 📚 Next Steps

After successful setup:

1. **Explore the API**: Check the backend endpoints at http://localhost:8080
2. **Review Documentation**: Read the main README.md for project details
3. **Start Development**: Begin working on features or customizations
4. **Join the Community**: Contribute to the project or report issues

## 🆘 Getting Help

If you encounter issues:

1. **Check the logs**: `docker-compose logs [service-name]`
2. **Verify prerequisites**: Ensure Docker and Git are properly installed
3. **Check network**: Ensure ports 80, 8080, 4444, 4445, 4455, 4456, 4457 are available
4. **Create an issue**: Report bugs on the GitHub repository

## 🎯 Success Criteria

You know the setup is successful when:

- ✅ All services show "healthy" status
- ✅ `curl http://localhost:8080/health` returns JSON response
- ✅ `curl http://localhost/health` returns "healthy"
- ✅ No error messages in `docker-compose logs`

---

**Happy coding! 🚀**

For more information, see the main [README.md](README.md) file. 