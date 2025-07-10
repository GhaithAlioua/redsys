# Redsys Security Configuration

## 🔒 **SECURITY OVERVIEW**

This document outlines the comprehensive security measures implemented in the Redsys GPU compute marketplace.

## 🛡️ **AUTHENTICATION & AUTHORIZATION**

### Ory Hydra (OAuth2/OpenID Connect)
- **Version**: v2.2.0 (latest stable)
- **Security Features**:
  - Token introspection enabled
  - Internal error exposure disabled
  - Secure cookie configuration
  - Access and refresh token hooks
  - SCRAM-SHA-256 database authentication

### Ory Oathkeeper (API Gateway)
- **Version**: v0.40.6 (latest stable)
- **Security Features**:
  - OAuth2 token introspection
  - Scope-based authorization
  - Rate limiting
  - CORS protection
  - Admin API CORS disabled

## 🔐 **SECURITY HEADERS**

### Nginx Security Headers
- `X-Frame-Options: SAMEORIGIN`
- `X-Content-Type-Options: nosniff`
- `X-XSS-Protection: 1; mode=block`
- `Referrer-Policy: strict-origin-when-cross-origin`
- `Strict-Transport-Security: max-age=31536000; includeSubDomains`
- `X-Permitted-Cross-Domain-Policies: none`
- `X-Download-Options: noopen`
- `X-Robots-Tag: noindex, nofollow`

## 🚫 **RATE LIMITING**

### API Gateway Level
- **General API**: 10 requests/second with burst of 20
- **Login endpoints**: 1 request/second
- **Backend middleware**: 100 requests/minute per IP

### OAuth2 Middleware
- **API endpoints**: Stricter rate limiting
- **Public endpoints**: Relaxed rate limiting
- **IP-based tracking**: Enhanced monitoring

## 🔍 **ACCESS CONTROL**

### Oathkeeper Access Rules
- **Public endpoints**: `/health`, `/api/v1/hello`
- **Protected endpoints**: `/api/v1/users`, `/api/v1/providers`, `/api/v1/jobs`
- **Required scope**: `redsys.api`
- **Authentication**: OAuth2 introspection

### Backend Authorization
- **Token validation**: Expiration, scope, client validation
- **Header injection**: User ID, scope, client ID
- **Security logging**: All authentication events

## 🛡️ **NETWORK SECURITY**

### Docker Security
- **Non-root containers**: All services run as non-root users
- **Security options**: `no-new-privileges:true`
- **Resource limits**: Memory and CPU limits configured
- **Network isolation**: Custom bridge network

### Database Security
- **Authentication**: SCRAM-SHA-256
- **Network access**: Internal Docker network only
- **Encryption**: TLS/SSL ready (configure for production)

## 📊 **MONITORING & LOGGING**

### Security Events Logged
- Authentication success/failure
- Token introspection results
- Rate limit violations
- Invalid token attempts
- Scope validation failures

### Log Format
- **Structured logging**: JSON format
- **Sensitive data**: Never logged
- **Timestamp**: UTC timestamps
- **IP tracking**: Client IP addresses

## 🔧 **CONFIGURATION SECURITY**

### Environment Variables
- **Secrets**: Use Docker secrets in production
- **Database passwords**: Strong passwords required
- **OAuth2 secrets**: 32-character minimum
- **System secrets**: Unique per environment

### Production Recommendations
1. **HTTPS/TLS**: Enable SSL termination
2. **Secrets management**: Use HashiCorp Vault or AWS Secrets Manager
3. **Network security**: Restrict admin APIs to internal network
4. **Monitoring**: Implement SIEM integration
5. **Backup encryption**: Encrypt database backups
6. **Access logging**: Comprehensive audit trails

## 🚨 **SECURITY CHECKLIST**

### Development Environment
- [x] OAuth2 authentication implemented
- [x] Rate limiting configured
- [x] Security headers set
- [x] Non-root containers
- [x] Resource limits
- [x] Health checks
- [x] Error handling

### Production Readiness
- [ ] SSL/TLS certificates
- [ ] Secrets management
- [ ] Network segmentation
- [ ] Monitoring integration
- [ ] Backup encryption
- [ ] Access logging
- [ ] Security scanning
- [ ] Penetration testing

## 📁 **PRODUCTION SECURITY CONFIGURATIONS**

### Security Infrastructure Directory
The `infrastructure/security/` directory contains production-ready security configurations:

```
infrastructure/security/
├── ssl/                      # SSL/TLS configurations
│   └── ssl.conf              # Production SSL configuration for Nginx
├── firewall/                 # Network security configurations
│   └── iptables-rules.conf   # Firewall rules for production
└── monitoring/               # Security monitoring configurations
    └── security-alerts.yml   # Prometheus alerting rules for security events
```

### Production Deployment

#### SSL/TLS Setup
```bash
# Copy SSL configuration
cp infrastructure/security/ssl/ssl.conf /etc/nginx/conf.d/

# Add your SSL certificates
cp your-cert.crt /etc/ssl/certs/redsys.crt
cp your-key.key /etc/ssl/private/redsys.key
```

#### Firewall Setup
```bash
# Apply firewall rules
sudo bash infrastructure/security/firewall/iptables-rules.conf
```

#### Monitoring Setup
```bash
# Add to Prometheus configuration
cp infrastructure/security/monitoring/security-alerts.yml /etc/prometheus/rules/
```

### Customization
- **SSL Configuration**: Update `server_name` and certificate paths in `ssl.conf`
- **Firewall Rules**: Modify IP ranges and rate limiting in `iptables-rules.conf`
- **Monitoring Alerts**: Customize thresholds in `security-alerts.yml`

## 📋 **SECURITY CONTACTS**

For security issues, please contact:
- **Security Team**: security@redsys.com
- **Bug Bounty**: https://redsys.com/security
- **Responsible Disclosure**: security@redsys.com

---

**Last Updated**: 2025-01-08
**Version**: 1.0.0
**Security Level**: Enterprise Grade 