#pragma once

#include <drogon/drogon.h>
#include <string>
#include <vector>

namespace redsys {

class Database {
public:
    static Database& getInstance();
    
    // Simplified query methods with individual parameters
    drogon::Task<std::vector<drogon::orm::Row>> executeQuery(const std::string& sql);
    drogon::Task<std::vector<drogon::orm::Row>> executeQuery(const std::string& sql, const std::string& param1);
    drogon::Task<std::vector<drogon::orm::Row>> executeQuery(const std::string& sql, const std::string& param1, const std::string& param2);
    drogon::Task<std::vector<drogon::orm::Row>> executeQuery(const std::string& sql, const std::string& param1, const std::string& param2, const std::string& param3);
    drogon::Task<std::vector<drogon::orm::Row>> executeQuery(const std::string& sql, const std::string& param1, const std::string& param2, const std::string& param3, const std::string& param4);
    drogon::Task<std::vector<drogon::orm::Row>> executeQuery(const std::string& sql, const std::string& param1, const std::string& param2, const std::string& param3, const std::string& param4, const std::string& param5);
    drogon::Task<std::vector<drogon::orm::Row>> executeQuery(const std::string& sql, const std::string& param1, const std::string& param2, const std::string& param3, const std::string& param4, const std::string& param5, const std::string& param6);
    drogon::Task<std::vector<drogon::orm::Row>> executeQuery(const std::string& sql, const std::string& param1, const std::string& param2, const std::string& param3, const std::string& param4, const std::string& param5, const std::string& param6, const std::string& param7);
    drogon::Task<std::vector<drogon::orm::Row>> executeQuery(const std::string& sql, const std::string& param1, const std::string& param2, const std::string& param3, const std::string& param4, const std::string& param5, const std::string& param6, const std::string& param7, const std::string& param8);
    drogon::Task<std::vector<drogon::orm::Row>> executeQuery(const std::string& sql, const std::string& param1, const std::string& param2, const std::string& param3, const std::string& param4, const std::string& param5, const std::string& param6, const std::string& param7, const std::string& param8, const std::string& param9);

private:
    Database();
    Database(const Database&) = delete;
    Database& operator=(const Database&) = delete;
    
    std::shared_ptr<drogon::orm::DbClient> dbClient_;
};

// Database operations namespace
namespace db {
    // User operations
    drogon::Task<std::vector<drogon::orm::Row>> getUserById(const std::string& userId);
    drogon::Task<std::vector<drogon::orm::Row>> getUserByEmail(const std::string& email);
    drogon::Task<std::vector<drogon::orm::Row>> getUserByUsername(const std::string& username);
    drogon::Task<bool> createUser(const std::string& email, const std::string& username, 
                                 const std::string& passwordHash, const std::string& role);
    
    // Provider operations
    drogon::Task<std::vector<drogon::orm::Row>> getProviderById(const std::string& providerId);
    drogon::Task<std::vector<drogon::orm::Row>> getProvidersByUserId(const std::string& userId);
    drogon::Task<bool> createProvider(const std::string& userId, const std::string& name, 
                                     const std::string& description, int gpuCount, int gpuMemoryGb,
                                     const std::string& gpuModel, const std::string& cudaVersion, 
                                     double hourlyRate);
    
    // Job operations
    drogon::Task<std::vector<drogon::orm::Row>> getJobsByUserId(const std::string& userId);
    drogon::Task<std::vector<drogon::orm::Row>> getJobById(const std::string& jobId);
    drogon::Task<bool> createJob(const std::string& userId, const std::string& title, 
                                const std::string& description, const std::string& dockerImage,
                                const std::string& dockerCommand, int gpuRequirements, 
                                int memoryRequirementsGb, double estimatedDurationHours, 
                                double budget);
    drogon::Task<bool> updateJobStatus(const std::string& jobId, const std::string& status);
    drogon::Task<bool> assignJobToProvider(const std::string& jobId, const std::string& providerId);
    
    // System metrics operations
    drogon::Task<bool> insertSystemMetrics(const std::string& providerId, double cpuUsage, 
                                          double memoryUsage, double gpuUsage, double gpuMemoryUsage,
                                          double networkUsage, double diskUsage, double temperature);
    
    // Payment operations
    drogon::Task<std::vector<drogon::orm::Row>> getPaymentsByJobId(const std::string& jobId);
    drogon::Task<bool> createPayment(const std::string& jobId, const std::string& fromUserId, 
                                    const std::string& toUserId, double amount, double platformFee);
}

} // namespace redsys 