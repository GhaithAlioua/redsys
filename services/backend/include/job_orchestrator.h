#pragma once

#include <drogon/drogon.h>
#include <string>

namespace redsys {

class JobOrchestrator {
public:
    static JobOrchestrator& getInstance();
    
    // Job assignment and monitoring
    drogon::Task<void> assignJobToProvider(const std::string& jobId, const std::string& providerId);
    drogon::Task<void> monitorJobProgress(const std::string& jobId);

private:
    JobOrchestrator();
    ~JobOrchestrator() = default;
    JobOrchestrator(const JobOrchestrator&) = delete;
    JobOrchestrator& operator=(const JobOrchestrator&) = delete;
};

} // namespace redsys 