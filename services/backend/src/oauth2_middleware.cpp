#include "oauth2_middleware.h"
#include <drogon/drogon.h>
#include <iostream>

namespace redsys {

OAuth2Middleware::OAuth2Middleware() {
    // Initialize OAuth2 middleware
}

void OAuth2Middleware::doFilter(const drogon::HttpRequestPtr& req,
                               drogon::FilterCallback&& fcb,
                               drogon::FilterChainCallback&& fccb) {
    // For development, allow all requests without authentication
    // In production, this would validate JWT tokens with Hydra
    
    // Extract token from Authorization header
    auto authHeader = req->getHeader("Authorization");
    
    if (authHeader.empty()) {
        // No token provided - allow for development
        // In production, this would return 401 Unauthorized
        std::cout << "OAuth2: No token provided, allowing request (dev mode)" << std::endl;
        fccb();
        return;
    }
    
    // Check if it's a Bearer token
    if (authHeader.substr(0, 7) == "Bearer ") {
        std::string token = authHeader.substr(7);
        
        // For development, accept any non-empty token
        // In production, validate with Hydra introspection endpoint
        if (!token.empty()) {
            std::cout << "OAuth2: Token provided, allowing request (dev mode)" << std::endl;
            
            // Add user info headers for downstream services
            req->addHeader("X-User-ID", "dev-user-123");
            req->addHeader("X-User-Scope", "redsys.api");
            req->addHeader("X-Client-ID", "redsys-backend");
            
            fccb();
            return;
        }
    }
    
    // Invalid token format
    std::cout << "OAuth2: Invalid token format, allowing request (dev mode)" << std::endl;
    fccb();
}

} // namespace redsys 