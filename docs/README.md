# Redsys Documentation

This directory contains all documentation for the Redsys decentralized GPU compute marketplace.

## Structure

```
docs/
└── README.md              # This file

infrastructure/
└── api/
    └── openapi.yaml       # OpenAPI 3.1.0 specification for the backend API
```

## API Documentation

### OpenAPI Specification

The API documentation is written in OpenAPI 3.1.0 format and is located at `infrastructure/api/openapi.yaml`.

**Why in `infrastructure/api/`?**
- **Infrastructure-as-Code**: OpenAPI specs are treated as infrastructure configuration
- **Version Control**: Specs are versioned alongside API changes
- **CI/CD Integration**: Can be used in automated testing and validation
- **Enterprise Pattern**: Follows the same pattern as other infrastructure configs (Docker, Oathkeeper, etc.)

### Viewing the Documentation

#### Option 1: Swagger UI (Recommended)
The backend serves a Swagger UI interface at:
- **Development**: http://localhost:8080/docs
- **Production**: https://api.redsys.com/docs

#### Option 2: Direct OpenAPI Spec
The raw OpenAPI specification is available at:
- **Development**: http://localhost:8080/openapi.yaml
- **Production**: https://api.redsys.com/openapi.yaml

#### Option 3: External Tools
You can also view the OpenAPI spec using:
- **Swagger Editor**: https://editor.swagger.io/ (paste the content of `infrastructure/api/openapi.yaml`)
- **Redoc**: https://redocly.github.io/redoc/ (paste the content of `infrastructure/api/openapi.yaml`)

### API Endpoints

The API includes the following endpoint categories:

- **Health**: `/health` - Service health checks
- **API**: `/api/v1/*` - Core API endpoints
- **Users**: `/api/v1/users` - User management
- **Providers**: `/api/v1/providers` - GPU provider management
- **Jobs**: `/api/v1/jobs` - Job management
- **Authentication**: `/login`, `/consent` - OAuth2 flows

### Authentication

Most API endpoints require OAuth2 authentication via Hydra. Public endpoints include:
- `/health` - Health check
- `/login` - OAuth2 login initiation
- `/consent` - OAuth2 consent handling
- `/docs` - API documentation
- `/openapi.yaml` - OpenAPI specification

### Rate Limiting

API endpoints are rate limited to 10 requests per second per IP address.

## Contributing

When adding new endpoints to the backend:

1. Update the OpenAPI specification in `infrastructure/api/openapi.yaml`
2. Add the endpoint to the backend code in `services/backend/src/main.cpp`
3. Update the Oathkeeper access rules in `infrastructure/oathkeeper/access-rules.yaml` if needed
4. Test the endpoint and documentation

## Industry Standards

This documentation follows industry best practices:

- **OpenAPI 3.1.0**: Latest stable version of the OpenAPI specification
- **Swagger UI 5.9.0**: Latest stable version of Swagger UI
- **Infrastructure placement**: API specs in `infrastructure/api/` for enterprise-grade projects
- **Embedded serving**: Backend serves docs directly for easy access
- **Comprehensive examples**: All endpoints include example responses
- **Clear descriptions**: Each endpoint has detailed descriptions and usage notes 