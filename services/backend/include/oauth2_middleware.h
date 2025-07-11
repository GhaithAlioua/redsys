#pragma once

#include <drogon/HttpFilter.h>
#include <string>

namespace redsys {

class OAuth2Middleware : public drogon::HttpFilter<OAuth2Middleware> {
public:
    OAuth2Middleware();
    static constexpr bool isAutoCreation = false;
    
    void doFilter(const drogon::HttpRequestPtr& req,
                  drogon::FilterCallback&& fcb,
                  drogon::FilterChainCallback&& fccb) override;
};

} // namespace redsys 