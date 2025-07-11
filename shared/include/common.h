#ifndef REDSYS_COMMON_H
#define REDSYS_COMMON_H

#include <string>
#include <chrono>
#include <memory>
#include <vector>
#include <map>

namespace redsys {

// Common types used across the application
using UserId = std::string;
using JobId = std::string;
using ProviderId = std::string;
using Timestamp = std::chrono::system_clock::time_point;

// Status enums
enum class JobStatus {
    PENDING,
    RUNNING,
    COMPLETED,
    FAILED,
    CANCELLED
};

enum class ProviderStatus {
    ONLINE,
    OFFLINE,
    BUSY,
    MAINTENANCE
};

enum class UserRole {
    USER,
    PROVIDER,
    ADMIN
};

// Common response structure
struct ApiResponse {
    bool success;
    std::string message;
    std::string timestamp;
    std::string error_code;  // Empty string if no error
    std::string request_id;  // Empty string if no request ID
};

// Utility functions
inline std::string getCurrentTimestamp() {
    auto now = std::chrono::system_clock::now();
    auto time_t = std::chrono::system_clock::to_time_t(now);
    return std::to_string(time_t);
}

inline std::string generateId() {
    // Simple ID generation - in production, use UUID library
    auto now = std::chrono::high_resolution_clock::now();
    auto duration = now.time_since_epoch();
    auto millis = std::chrono::duration_cast<std::chrono::milliseconds>(duration).count();
    return "id_" + std::to_string(millis);
}

} // namespace redsys

#endif // REDSYS_COMMON_H 