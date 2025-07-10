#pragma once

#include <drogon/drogon.h>
#include <functional>

namespace redsys {

class MarketplaceService {
public:
    static MarketplaceService& getInstance();
    
    // Job management
    drogon::Task<void> processJobSubmission(const drogon::HttpRequestPtr& req, std::function<void(const drogon::HttpResponsePtr&)>&& callback);
    
    // Provider management
    drogon::Task<void> getProviderStatus(const drogon::HttpRequestPtr& req, std::function<void(const drogon::HttpResponsePtr&)>&& callback);

private:
    MarketplaceService();
    ~MarketplaceService() = default;
    MarketplaceService(const MarketplaceService&) = delete;
    MarketplaceService& operator=(const MarketplaceService&) = delete;
};

} // namespace redsys 