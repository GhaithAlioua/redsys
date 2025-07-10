#include <drogon/drogon.h>
#include "database.h"
#include "oauth2_middleware.h"
#include <iostream>
#include <ctime>
#include <cstdlib>
#include <string>
#include <memory>

int main() {
    try {
        // Configure Drogon application with config file (recommended approach)
        drogon::app()
            .loadConfigFile("./config.json")  // Load configuration from file
            .registerHandler("/health", [](const drogon::HttpRequestPtr& req, std::function<void(const drogon::HttpResponsePtr&)>&& callback) {
                (void)req; // Suppress unused parameter warning
                auto resp = drogon::HttpResponse::newHttpResponse();
                resp->setStatusCode(drogon::k200OK);
                resp->setContentTypeCode(drogon::CT_APPLICATION_JSON);
                
                // Enterprise health check response
                Json::Value healthStatus;
                healthStatus["status"] = "healthy";
                healthStatus["service"] = "redsys-backend";
                healthStatus["timestamp"] = std::to_string(std::time(nullptr));
                healthStatus["version"] = "1.0.0";
                healthStatus["environment"] = "development";
                
                resp->setBody(healthStatus.toStyledString());
                callback(resp);
            })
            .registerHandler("/api/v1/hello", [](const drogon::HttpRequestPtr& req, std::function<void(const drogon::HttpResponsePtr&)>&& callback) {
                (void)req; // Suppress unused parameter warning
                auto resp = drogon::HttpResponse::newHttpResponse();
                resp->setStatusCode(drogon::k200OK);
                resp->setContentTypeCode(drogon::CT_APPLICATION_JSON);
                
                // Get user information from OAuth2 middleware headers
                auto userId = req->getHeader("X-User-ID");
                auto userScope = req->getHeader("X-User-Scope");
                
                Json::Value response;
                response["message"] = "Hello, Redsys Backend API!";
                response["status"] = "success";
                response["timestamp"] = std::to_string(std::time(nullptr));
                response["user_id"] = userId;
                response["user_scope"] = userScope;
                response["service"] = "redsys-backend";
                
                resp->setBody(response.toStyledString());
                callback(resp);
            })
            .registerHandler("/api/v1/users", [](const drogon::HttpRequestPtr& req, std::function<void(const drogon::HttpResponsePtr&)>&& callback) {
                (void)req; // Suppress unused parameter warning
                auto resp = drogon::HttpResponse::newHttpResponse();
                resp->setStatusCode(drogon::k200OK);
                resp->setContentTypeCode(drogon::CT_APPLICATION_JSON);
                
                Json::Value response;
                response["message"] = "Users endpoint - Coming soon";
                response["status"] = "success";
                response["timestamp"] = std::to_string(std::time(nullptr));
                
                resp->setBody(response.toStyledString());
                callback(resp);
            })
            .registerHandler("/api/v1/providers", [](const drogon::HttpRequestPtr& req, std::function<void(const drogon::HttpResponsePtr&)>&& callback) {
                (void)req; // Suppress unused parameter warning
                auto resp = drogon::HttpResponse::newHttpResponse();
                resp->setStatusCode(drogon::k200OK);
                resp->setContentTypeCode(drogon::CT_APPLICATION_JSON);
                
                Json::Value response;
                response["message"] = "Providers endpoint - Coming soon";
                response["status"] = "success";
                response["timestamp"] = std::to_string(std::time(nullptr));
                
                resp->setBody(response.toStyledString());
                callback(resp);
            })
            .registerHandler("/api/v1/jobs", [](const drogon::HttpRequestPtr& req, std::function<void(const drogon::HttpResponsePtr&)>&& callback) {
                (void)req; // Suppress unused parameter warning
                auto resp = drogon::HttpResponse::newHttpResponse();
                resp->setStatusCode(drogon::k200OK);
                resp->setContentTypeCode(drogon::CT_APPLICATION_JSON);
                
                Json::Value response;
                response["message"] = "Jobs endpoint - Coming soon";
                response["status"] = "success";
                response["timestamp"] = std::to_string(std::time(nullptr));
                
                resp->setBody(response.toStyledString());
                callback(resp);
            })
            .run();

        return 0;
    } catch (const std::exception& e) {
        std::cerr << "Fatal error: " << e.what() << std::endl;
        return 1;
    } catch (...) {
        std::cerr << "Unknown fatal error occurred" << std::endl;
        return 1;
    }
} 