#include <drogon/drogon.h>
#include <iostream>
#include <ctime>
#include <cstdlib>
#include <string>
#include <memory>
#include <fstream>
#include <sstream>

int main() {
    try {
        // Configure Drogon application with config file (recommended approach)
        drogon::app()
            .loadConfigFile("/etc/redsys/config.json")  // Load configuration from file
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
            .registerHandler("/login", [](const drogon::HttpRequestPtr& req, std::function<void(const drogon::HttpResponsePtr&)>&& callback) {
                (void)req; // Suppress unused parameter warning
                auto resp = drogon::HttpResponse::newHttpResponse();
                resp->setStatusCode(drogon::k200OK);
                resp->setContentTypeCode(drogon::CT_APPLICATION_JSON);
                
                Json::Value response;
                response["message"] = "OAuth2 login endpoint - Redirect to Hydra";
                response["status"] = "success";
                response["timestamp"] = std::to_string(std::time(nullptr));
                response["oauth2_url"] = "http://hydra:4444/oauth2/auth";
                
                resp->setBody(response.toStyledString());
                callback(resp);
            })
            .registerHandler("/consent", [](const drogon::HttpRequestPtr& req, std::function<void(const drogon::HttpResponsePtr&)>&& callback) {
                (void)req; // Suppress unused parameter warning
                auto resp = drogon::HttpResponse::newHttpResponse();
                resp->setStatusCode(drogon::k200OK);
                resp->setContentTypeCode(drogon::CT_APPLICATION_JSON);
                
                Json::Value response;
                response["message"] = "OAuth2 consent endpoint - Handle user consent";
                response["status"] = "success";
                response["timestamp"] = std::to_string(std::time(nullptr));
                
                resp->setBody(response.toStyledString());
                callback(resp);
            })
            .registerHandler("/docs", [](const drogon::HttpRequestPtr& req, std::function<void(const drogon::HttpResponsePtr&)>&& callback) {
                (void)req; // Suppress unused parameter warning
                auto resp = drogon::HttpResponse::newHttpResponse();
                resp->setStatusCode(drogon::k200OK);
                resp->setContentTypeCode(drogon::CT_TEXT_HTML);
                
                // Swagger UI HTML with embedded OpenAPI spec
                std::string html = R"(
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <meta name="description" content="Redsys Backend API Documentation" />
    <title>Redsys Backend API - Swagger UI</title>
    <link rel="stylesheet" type="text/css" href="https://unpkg.com/swagger-ui-dist@5.9.0/swagger-ui.css" />
    <style>
        html { box-sizing: border-box; overflow: -moz-scrollbars-vertical; overflow-y: scroll; }
        *, *:before, *:after { box-sizing: inherit; }
        body { margin:0; background: #fafafa; }
    </style>
</head>
<body>
    <div id="swagger-ui"></div>
    <script src="https://unpkg.com/swagger-ui-dist@5.9.0/swagger-ui-bundle.js" crossorigin></script>
    <script src="https://unpkg.com/swagger-ui-dist@5.9.0/swagger-ui-standalone-preset.js" crossorigin></script>
    <script>
        window.onload = () => {
            window.ui = SwaggerUIBundle({
                url: '/openapi.yaml',
                dom_id: '#swagger-ui',
                deepLinking: true,
                presets: [SwaggerUIBundle.presets.apis, SwaggerUIStandalonePreset],
                plugins: [SwaggerUIBundle.plugins.DownloadUrl],
                layout: "StandaloneLayout"
            });
        };
    </script>
</body>
</html>
                )";
                
                resp->setBody(html);
                callback(resp);
            })
            .registerHandler("/openapi.yaml", [](const drogon::HttpRequestPtr& req, std::function<void(const drogon::HttpResponsePtr&)>&& callback) {
                (void)req; // Suppress unused parameter warning
                auto resp = drogon::HttpResponse::newHttpResponse();
                resp->setStatusCode(drogon::k200OK);
                resp->setContentTypeCode(drogon::CT_TEXT_PLAIN);
                
                // Serve the external OpenAPI spec file (industry standard)
                std::ifstream file("/etc/redsys/openapi.yaml");
                if (file.is_open()) {
                    std::stringstream buffer;
                    buffer << file.rdbuf();
                    resp->setBody(buffer.str());
                } else {
                    // Fallback to embedded spec if file not found
                    resp->setBody("openapi: 3.1.0\ninfo:\n  title: Redsys Backend API\n  version: 1.0.0\n  description: API specification\n");
                }
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