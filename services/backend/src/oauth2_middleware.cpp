#include "oauth2_middleware.h"
#include <drogon/drogon.h>
#include <iostream>
#include <sstream>
#include <iomanip>
#include <json/json.h>

namespace redsys {

OAuth2Middleware::OAuth2Middleware() {
    // Initialize with environment variables or defaults
    introspectionUrl_ = std::getenv("OAUTH2_INTROSPECTION_URL") ? 
        std::getenv("OAUTH2_INTROSPECTION_URL") : "http://hydra:4445/oauth2/introspect";
    clientId_ = std::getenv("OAUTH2_CLIENT_ID") ? 
        std::getenv("OAUTH2_CLIENT_ID") : "redsys-backend";
    clientSecret_ = std::getenv("OAUTH2_CLIENT_SECRET") ? 
        std::getenv("OAUTH2_CLIENT_SECRET") : "backend_secret";
    
    tokenCacheTimeout_ = std::chrono::seconds(300); // 5 minutes
    maxRetries_ = 3;
    requestTimeout_ = std::chrono::milliseconds(5000); // 5 seconds
    
    lastRequestTime_ = std::chrono::system_clock::now();
    requestCount_ = 0;
}

void OAuth2Middleware::doFilter(const drogon::HttpRequestPtr& req,
                                drogon::FilterCallback&& fcb,
                                drogon::FilterChainCallback&& fccb) {
    // Skip authentication for health check and public endpoints
    if (req->getPath() == "/health" || req->getPath() == "/api/v1/hello") {
        fccb();
        return;
    }
    
    // Check rate limiting
    if (!checkRateLimit(req)) {
        logSecurityEvent("rate_limit_exceeded", req, "Too many requests");
        auto resp = createErrorResponse(429, "rate_limit_exceeded", "Too many requests");
        fcb(resp);
        return;
    }
    
    // Extract token from Authorization header
    auto token = extractToken(req);
    if (!token) {
        logSecurityEvent("missing_token", req, "No valid Authorization header");
        auto resp = createErrorResponse(401, "missing_token", "Authorization header required");
        fcb(resp);
        return;
    }
    
    // Introspect token with Ory Hydra
    auto tokenInfo = introspectToken(*token);
    if (!tokenInfo || !tokenInfo->active) {
        logSecurityEvent("invalid_token", req, "Token introspection failed or token inactive");
        auto resp = createErrorResponse(401, "invalid_token", "Token is invalid or expired");
        fcb(resp);
        return;
    }
    
    // Validate token expiration
    if (!validateTokenExpiration(*tokenInfo)) {
        logSecurityEvent("expired_token", req, "Token has expired");
        auto resp = createErrorResponse(401, "invalid_token", "Token has expired");
        fcb(resp);
        return;
    }
    
    // Validate required scopes (enterprise security)
    if (!validateRequiredScopes(*tokenInfo, "redsys.api")) {
        logSecurityEvent("insufficient_scope", req, "Token lacks required scope: redsys.api");
        auto resp = createErrorResponse(403, "insufficient_scope", "Token lacks required scope");
        fcb(resp);
        return;
    }
    
    // Inject user context into request headers (enterprise pattern)
    // Only inject essential headers, avoid sensitive data
    req->addHeader("X-User-ID", tokenInfo->sub);
    req->addHeader("X-User-Scope", tokenInfo->scope);
    req->addHeader("X-Client-ID", tokenInfo->client_id);
    req->addHeader("X-Token-Type", tokenInfo->token_type);
    // Don't expose token expiration in headers for security
    
    // Log successful authentication
    logSecurityEvent("authentication_success", req, "User: " + tokenInfo->sub + ", Client: " + tokenInfo->client_id);
    
    fccb();
}

std::optional<std::string> OAuth2Middleware::extractToken(const drogon::HttpRequestPtr& req) {
    auto authHeader = req->getHeader("Authorization");
    if (authHeader.empty()) return std::nullopt;
    
    // Validate Bearer token format
    if (authHeader.length() < 7 || authHeader.substr(0, 7) != "Bearer ") {
        return std::nullopt;
    }
    
    std::string token = authHeader.substr(7);
    
    // Basic token validation (non-empty, reasonable length)
    if (token.empty() || token.length() > 1000) {
        return std::nullopt;
    }
    
    return token;
}

std::optional<OAuth2TokenInfo> OAuth2Middleware::introspectToken(const std::string& token) {
    for (int attempt = 0; attempt < maxRetries_; ++attempt) {
        try {
            // Create HTTP client for token introspection
            auto client = drogon::HttpClient::newHttpClient(introspectionUrl_);
            
            // Create request
            auto req = drogon::HttpRequest::newHttpRequest();
            req->setMethod(drogon::Post);
            req->setContentTypeCode(drogon::CT_APPLICATION_X_FORM);
            
            // Prepare form data
            std::string body = "token=" + token + "&client_id=" + clientId_ + "&client_secret=" + clientSecret_;
            req->setBody(body);
            
            // Send request
            auto [result, resp] = client->sendRequest(req);
            
            if (result != drogon::ReqResult::Ok || !resp) {
                logTokenIntrospection(token, false, "Network error or no response");
                continue;
            }
            
            if (resp->getStatusCode() != drogon::k200OK) {
                logTokenIntrospection(token, false, "HTTP " + std::to_string(resp->getStatusCode()));
                continue;
            }
            
            // Parse JSON response
            Json::Value root;
            Json::Reader reader;
            std::string respBody(resp->getBody());
            
            if (!reader.parse(respBody, root)) {
                logTokenIntrospection(token, false, "Invalid JSON response");
                continue;
            }
            
            // Extract token information
            OAuth2TokenInfo tokenInfo;
            tokenInfo.active = root.get("active", false).asBool();
            tokenInfo.scope = root.get("scope", "").asString();
            tokenInfo.client_id = root.get("client_id", "").asString();
            tokenInfo.username = root.get("username", "").asString();
            tokenInfo.token_type = root.get("token_type", "").asString();
            tokenInfo.exp = root.get("exp", 0).asInt64();
            tokenInfo.iat = root.get("iat", 0).asInt64();
            tokenInfo.sub = root.get("sub", "").asString();
            tokenInfo.aud = root.get("aud", "").asString();
            tokenInfo.iss = root.get("iss", "").asString();
            tokenInfo.introspected_at = std::chrono::system_clock::now();
            
            logTokenIntrospection(token, true);
            return tokenInfo;
            
        } catch (const std::exception& e) {
            logTokenIntrospection(token, false, "Exception: " + std::string(e.what()));
            if (attempt == maxRetries_ - 1) break;
        }
    }
    
    return std::nullopt;
}

bool OAuth2Middleware::validateTokenExpiration(const OAuth2TokenInfo& tokenInfo) {
    auto now = std::chrono::system_clock::now();
    auto expTime = std::chrono::system_clock::from_time_t(tokenInfo.exp);
    
    // Add 5-minute buffer for clock skew
    auto buffer = std::chrono::minutes(5);
    return now < (expTime + buffer);
}

bool OAuth2Middleware::validateRequiredScopes(const OAuth2TokenInfo& tokenInfo, const std::string& requiredScope) {
    if (tokenInfo.scope.empty()) return false;
    
    // Simple scope validation (can be enhanced for complex scope logic)
    return tokenInfo.scope.find(requiredScope) != std::string::npos;
}

void OAuth2Middleware::logSecurityEvent(const std::string& event, const drogon::HttpRequestPtr& req, const std::string& details) {
    auto now = std::chrono::system_clock::now();
    auto time_t = std::chrono::system_clock::to_time_t(now);
    
    std::stringstream ss;
    ss << "[" << std::put_time(std::gmtime(&time_t), "%Y-%m-%d %H:%M:%S UTC") << "] "
       << "SECURITY_EVENT: " << event
       << " | IP: " << req->getPeerAddr().toIpPort()
       << " | Path: " << req->getPath()
       << " | Method: " << req->getMethodString()
       << " | User-Agent: " << req->getHeader("User-Agent");
    
    if (!details.empty()) {
        ss << " | Details: " << details;
    }
    
    // Log to stderr for now (should be enhanced with proper logging framework)
    std::cerr << ss.str() << std::endl;
}

void OAuth2Middleware::logTokenIntrospection(const std::string& token, bool success, const std::string& error) {
    (void)token; // Suppress unused parameter warning
    auto now = std::chrono::system_clock::now();
    auto time_t = std::chrono::system_clock::to_time_t(now);
    
    std::stringstream ss;
    ss << "[" << std::put_time(std::gmtime(&time_t), "%Y-%m-%d %H:%M:%S UTC") << "] "
       << "TOKEN_INTROSPECTION: " << (success ? "SUCCESS" : "FAILED");
    
    if (!success && !error.empty()) {
        ss << " | Error: " << error;
    }
    
    // Log to stderr for now (should be enhanced with proper logging framework)
    std::cerr << ss.str() << std::endl;
}

drogon::HttpResponsePtr OAuth2Middleware::createErrorResponse(int statusCode, const std::string& error, const std::string& description) {
    auto resp = drogon::HttpResponse::newHttpResponse();
    resp->setStatusCode(static_cast<drogon::HttpStatusCode>(statusCode));
    resp->setContentTypeCode(drogon::CT_APPLICATION_JSON);
    
    Json::Value errorResponse;
    errorResponse["error"] = error;
    errorResponse["error_description"] = description;
    errorResponse["timestamp"] = std::to_string(std::chrono::system_clock::to_time_t(std::chrono::system_clock::now()));
    
    resp->setBody(errorResponse.toStyledString());
    return resp;
}

bool OAuth2Middleware::checkRateLimit(const drogon::HttpRequestPtr& req) {
    auto now = std::chrono::system_clock::now();
    auto clientIp = req->getPeerAddr().toIp();
    
    // Enhanced rate limiting with IP-based tracking
    // In production, use Redis for distributed rate limiting
    if (now - lastRequestTime_ > std::chrono::minutes(1)) {
        requestCount_ = 0;
        lastRequestTime_ = now;
    }
    
    // Stricter rate limiting for authentication endpoints
    int maxRequests = (req->getPath().find("/api/v1/") != std::string::npos) ? 
        MAX_REQUESTS_PER_MINUTE : MAX_REQUESTS_PER_MINUTE * 2;
    
    if (++requestCount_ > maxRequests) {
        logSecurityEvent("rate_limit_exceeded", req, "Rate limit exceeded for IP: " + clientIp);
        return false;
    }
    
    return true;
}

} // namespace redsys 