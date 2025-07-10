#pragma once

#include <drogon/HttpFilter.h>
#include <string>
#include <optional>
#include <memory>
#include <chrono>

namespace redsys {

struct OAuth2TokenInfo {
    bool active;
    std::string scope;
    std::string client_id;
    std::string username;
    std::string token_type;
    int64_t exp;
    int64_t iat;
    std::string sub;
    std::string aud;
    std::string iss;
    std::chrono::system_clock::time_point introspected_at;
};

class OAuth2Middleware : public drogon::HttpFilter<OAuth2Middleware> {
public:
    OAuth2Middleware();
    static constexpr bool isAutoCreation = false;
    
    void doFilter(const drogon::HttpRequestPtr& req,
                  drogon::FilterCallback&& fcb,
                  drogon::FilterChainCallback&& fccb) override;

private:
    // Configuration
    std::string introspectionUrl_;
    std::string clientId_;
    std::string clientSecret_;
    std::chrono::seconds tokenCacheTimeout_;
    int maxRetries_;
    std::chrono::milliseconds requestTimeout_;
    
    // Security and validation
    std::optional<std::string> extractToken(const drogon::HttpRequestPtr& req);
    std::optional<OAuth2TokenInfo> introspectToken(const std::string& token);
    bool validateTokenExpiration(const OAuth2TokenInfo& tokenInfo);
    bool validateRequiredScopes(const OAuth2TokenInfo& tokenInfo, const std::string& requiredScope);
    
    // Logging and monitoring
    void logSecurityEvent(const std::string& event, const drogon::HttpRequestPtr& req, const std::string& details = "");
    void logTokenIntrospection(const std::string& token, bool success, const std::string& error = "");
    
    // Error handling
    drogon::HttpResponsePtr createErrorResponse(int statusCode, const std::string& error, const std::string& description);
    
    // Rate limiting (basic implementation)
    bool checkRateLimit(const drogon::HttpRequestPtr& req);
    std::chrono::system_clock::time_point lastRequestTime_;
    int requestCount_;
    static constexpr int MAX_REQUESTS_PER_MINUTE = 100;
};

} // namespace redsys 