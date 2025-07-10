#include "database.h"
#include <drogon/drogon.h>
#include <iostream>
#include <memory>

namespace redsys {

Database& Database::getInstance() {
    static Database instance;
    return instance;
}

Database::Database() {
    // Initialize database connection
    auto dbClient = drogon::app().getDbClient("default");
    if (!dbClient) {
        std::cerr << "Failed to get database client" << std::endl;
        return;
    }
    
    // Store the client
    dbClient_ = dbClient;
}

drogon::Task<std::vector<drogon::orm::Row>> Database::executeQuery(const std::string& sql) {
    if (!dbClient_) {
        co_return std::vector<drogon::orm::Row>();
    }
    
    try {
        auto result = co_await dbClient_->execSqlCoro(sql);
        std::vector<drogon::orm::Row> rows;
        for (auto it = result.begin(); it != result.end(); ++it) {
            rows.push_back(*it);
        }
        co_return rows;
    } catch (const std::exception& e) {
        std::cerr << "Database query error: " << e.what() << std::endl;
        co_return std::vector<drogon::orm::Row>();
    }
}

drogon::Task<std::vector<drogon::orm::Row>> Database::executeQuery(const std::string& sql, const std::string& param1) {
    if (!dbClient_) {
        co_return std::vector<drogon::orm::Row>();
    }
    
    try {
        auto result = co_await dbClient_->execSqlCoro(sql, param1);
        std::vector<drogon::orm::Row> rows;
        for (auto it = result.begin(); it != result.end(); ++it) {
            rows.push_back(*it);
        }
        co_return rows;
    } catch (const std::exception& e) {
        std::cerr << "Database query error: " << e.what() << std::endl;
        co_return std::vector<drogon::orm::Row>();
    }
}

drogon::Task<std::vector<drogon::orm::Row>> Database::executeQuery(const std::string& sql, const std::string& param1, const std::string& param2) {
    if (!dbClient_) {
        co_return std::vector<drogon::orm::Row>();
    }
    
    try {
        auto result = co_await dbClient_->execSqlCoro(sql, param1, param2);
        std::vector<drogon::orm::Row> rows;
        for (auto it = result.begin(); it != result.end(); ++it) {
            rows.push_back(*it);
        }
        co_return rows;
    } catch (const std::exception& e) {
        std::cerr << "Database query error: " << e.what() << std::endl;
        co_return std::vector<drogon::orm::Row>();
    }
}

drogon::Task<std::vector<drogon::orm::Row>> Database::executeQuery(const std::string& sql, const std::string& param1, const std::string& param2, const std::string& param3) {
    if (!dbClient_) {
        co_return std::vector<drogon::orm::Row>();
    }
    
    try {
        auto result = co_await dbClient_->execSqlCoro(sql, param1, param2, param3);
        std::vector<drogon::orm::Row> rows;
        for (auto it = result.begin(); it != result.end(); ++it) {
            rows.push_back(*it);
        }
        co_return rows;
    } catch (const std::exception& e) {
        std::cerr << "Database query error: " << e.what() << std::endl;
        co_return std::vector<drogon::orm::Row>();
    }
}

drogon::Task<std::vector<drogon::orm::Row>> Database::executeQuery(const std::string& sql, const std::string& param1, const std::string& param2, const std::string& param3, const std::string& param4) {
    if (!dbClient_) {
        co_return std::vector<drogon::orm::Row>();
    }
    
    try {
        auto result = co_await dbClient_->execSqlCoro(sql, param1, param2, param3, param4);
        std::vector<drogon::orm::Row> rows;
        for (auto it = result.begin(); it != result.end(); ++it) {
            rows.push_back(*it);
        }
        co_return rows;
    } catch (const std::exception& e) {
        std::cerr << "Database query error: " << e.what() << std::endl;
        co_return std::vector<drogon::orm::Row>();
    }
}

drogon::Task<std::vector<drogon::orm::Row>> Database::executeQuery(const std::string& sql, const std::string& param1, const std::string& param2, const std::string& param3, const std::string& param4, const std::string& param5) {
    if (!dbClient_) {
        co_return std::vector<drogon::orm::Row>();
    }
    
    try {
        auto result = co_await dbClient_->execSqlCoro(sql, param1, param2, param3, param4, param5);
        std::vector<drogon::orm::Row> rows;
        for (auto it = result.begin(); it != result.end(); ++it) {
            rows.push_back(*it);
        }
        co_return rows;
    } catch (const std::exception& e) {
        std::cerr << "Database query error: " << e.what() << std::endl;
        co_return std::vector<drogon::orm::Row>();
    }
}

drogon::Task<std::vector<drogon::orm::Row>> Database::executeQuery(const std::string& sql, const std::string& param1, const std::string& param2, const std::string& param3, const std::string& param4, const std::string& param5, const std::string& param6) {
    if (!dbClient_) {
        co_return std::vector<drogon::orm::Row>();
    }
    
    try {
        auto result = co_await dbClient_->execSqlCoro(sql, param1, param2, param3, param4, param5, param6);
        std::vector<drogon::orm::Row> rows;
        for (auto it = result.begin(); it != result.end(); ++it) {
            rows.push_back(*it);
        }
        co_return rows;
    } catch (const std::exception& e) {
        std::cerr << "Database query error: " << e.what() << std::endl;
        co_return std::vector<drogon::orm::Row>();
    }
}

drogon::Task<std::vector<drogon::orm::Row>> Database::executeQuery(const std::string& sql, const std::string& param1, const std::string& param2, const std::string& param3, const std::string& param4, const std::string& param5, const std::string& param6, const std::string& param7) {
    if (!dbClient_) {
        co_return std::vector<drogon::orm::Row>();
    }
    
    try {
        auto result = co_await dbClient_->execSqlCoro(sql, param1, param2, param3, param4, param5, param6, param7);
        std::vector<drogon::orm::Row> rows;
        for (auto it = result.begin(); it != result.end(); ++it) {
            rows.push_back(*it);
        }
        co_return rows;
    } catch (const std::exception& e) {
        std::cerr << "Database query error: " << e.what() << std::endl;
        co_return std::vector<drogon::orm::Row>();
    }
}

drogon::Task<std::vector<drogon::orm::Row>> Database::executeQuery(const std::string& sql, const std::string& param1, const std::string& param2, const std::string& param3, const std::string& param4, const std::string& param5, const std::string& param6, const std::string& param7, const std::string& param8) {
    if (!dbClient_) {
        co_return std::vector<drogon::orm::Row>();
    }
    
    try {
        auto result = co_await dbClient_->execSqlCoro(sql, param1, param2, param3, param4, param5, param6, param7, param8);
        std::vector<drogon::orm::Row> rows;
        for (auto it = result.begin(); it != result.end(); ++it) {
            rows.push_back(*it);
        }
        co_return rows;
    } catch (const std::exception& e) {
        std::cerr << "Database query error: " << e.what() << std::endl;
        co_return std::vector<drogon::orm::Row>();
    }
}

drogon::Task<std::vector<drogon::orm::Row>> Database::executeQuery(const std::string& sql, const std::string& param1, const std::string& param2, const std::string& param3, const std::string& param4, const std::string& param5, const std::string& param6, const std::string& param7, const std::string& param8, const std::string& param9) {
    if (!dbClient_) {
        co_return std::vector<drogon::orm::Row>();
    }
    
    try {
        auto result = co_await dbClient_->execSqlCoro(sql, param1, param2, param3, param4, param5, param6, param7, param8, param9);
        std::vector<drogon::orm::Row> rows;
        for (auto it = result.begin(); it != result.end(); ++it) {
            rows.push_back(*it);
        }
        co_return rows;
    } catch (const std::exception& e) {
        std::cerr << "Database query error: " << e.what() << std::endl;
        co_return std::vector<drogon::orm::Row>();
    }
}

} // namespace redsys

// Database operations implementation
namespace redsys::db {

drogon::Task<std::vector<drogon::orm::Row>> getUserById(const std::string& userId) {
    auto& db = redsys::Database::getInstance();
    co_return co_await db.executeQuery("SELECT * FROM users WHERE id = $1", userId);
}

drogon::Task<std::vector<drogon::orm::Row>> getUserByEmail(const std::string& email) {
    auto& db = redsys::Database::getInstance();
    co_return co_await db.executeQuery("SELECT * FROM users WHERE email = $1", email);
}

drogon::Task<std::vector<drogon::orm::Row>> getUserByUsername(const std::string& username) {
    auto& db = redsys::Database::getInstance();
    co_return co_await db.executeQuery("SELECT * FROM users WHERE username = $1", username);
}

drogon::Task<bool> createUser(const std::string& email, const std::string& username, 
                             const std::string& passwordHash, const std::string& role) {
    try {
        auto& db = redsys::Database::getInstance();
        co_await db.executeQuery(
            "INSERT INTO users (email, username, password_hash, role) VALUES ($1, $2, $3, $4)",
            email, username, passwordHash, role
        );
        co_return true;
    } catch (const std::exception& e) {
        std::cerr << "Failed to create user: " << e.what() << std::endl;
        co_return false;
    }
}

drogon::Task<std::vector<drogon::orm::Row>> getProviderById(const std::string& providerId) {
    auto& db = redsys::Database::getInstance();
    co_return co_await db.executeQuery("SELECT * FROM providers WHERE id = $1", providerId);
}

drogon::Task<std::vector<drogon::orm::Row>> getProvidersByUserId(const std::string& userId) {
    auto& db = redsys::Database::getInstance();
    co_return co_await db.executeQuery("SELECT * FROM providers WHERE user_id = $1", userId);
}

drogon::Task<bool> createProvider(const std::string& userId, const std::string& name, 
                                 const std::string& description, int gpuCount, int gpuMemoryGb,
                                 const std::string& gpuModel, const std::string& cudaVersion, 
                                 double hourlyRate) {
    try {
        auto& db = redsys::Database::getInstance();
        co_await db.executeQuery(
            "INSERT INTO providers (user_id, name, description, gpu_count, gpu_memory_gb, gpu_model, cuda_version, hourly_rate) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
            userId, name, description, std::to_string(gpuCount), std::to_string(gpuMemoryGb), gpuModel, cudaVersion, std::to_string(hourlyRate)
        );
        co_return true;
    } catch (const std::exception& e) {
        std::cerr << "Failed to create provider: " << e.what() << std::endl;
        co_return false;
    }
}

drogon::Task<std::vector<drogon::orm::Row>> getJobsByUserId(const std::string& userId) {
    auto& db = redsys::Database::getInstance();
    co_return co_await db.executeQuery("SELECT * FROM jobs WHERE user_id = $1 ORDER BY created_at DESC", userId);
}

drogon::Task<std::vector<drogon::orm::Row>> getJobById(const std::string& jobId) {
    auto& db = redsys::Database::getInstance();
    co_return co_await db.executeQuery("SELECT * FROM jobs WHERE id = $1", jobId);
}

drogon::Task<bool> createJob(const std::string& userId, const std::string& title, 
                            const std::string& description, const std::string& dockerImage,
                            const std::string& dockerCommand, int gpuRequirements, 
                            int memoryRequirementsGb, double estimatedDurationHours, 
                            double budget) {
    try {
        auto& db = redsys::Database::getInstance();
        co_await db.executeQuery(
            "INSERT INTO jobs (user_id, title, description, docker_image, docker_command, gpu_requirements, memory_requirements_gb, estimated_duration_hours, budget) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
            userId, title, description, dockerImage, dockerCommand, std::to_string(gpuRequirements), std::to_string(memoryRequirementsGb), std::to_string(estimatedDurationHours), std::to_string(budget)
        );
        co_return true;
    } catch (const std::exception& e) {
        std::cerr << "Failed to create job: " << e.what() << std::endl;
        co_return false;
    }
}

drogon::Task<bool> updateJobStatus(const std::string& jobId, const std::string& status) {
    try {
        auto& db = redsys::Database::getInstance();
        co_await db.executeQuery("UPDATE jobs SET status = $1 WHERE id = $2", status, jobId);
        co_return true;
    } catch (const std::exception& e) {
        std::cerr << "Failed to update job status: " << e.what() << std::endl;
        co_return false;
    }
}

drogon::Task<bool> assignJobToProvider(const std::string& jobId, const std::string& providerId) {
    try {
        auto& db = redsys::Database::getInstance();
        co_await db.executeQuery(
            "INSERT INTO job_assignments (job_id, provider_id) VALUES ($1, $2)",
            jobId, providerId
        );
        co_await db.executeQuery(
            "UPDATE jobs SET provider_id = $1, status = 'assigned' WHERE id = $2",
            providerId, jobId
        );
        co_return true;
    } catch (const std::exception& e) {
        std::cerr << "Failed to assign job to provider: " << e.what() << std::endl;
        co_return false;
    }
}

drogon::Task<bool> insertSystemMetrics(const std::string& providerId, double cpuUsage, 
                                      double memoryUsage, double gpuUsage, double gpuMemoryUsage,
                                      double networkUsage, double diskUsage, double temperature) {
    try {
        auto& db = redsys::Database::getInstance();
        co_await db.executeQuery(
            "INSERT INTO system_metrics (provider_id, cpu_usage_percent, memory_usage_percent, gpu_usage_percent, gpu_memory_usage_percent, network_usage_mbps, disk_usage_percent, temperature_celsius) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
            providerId, std::to_string(cpuUsage), std::to_string(memoryUsage), std::to_string(gpuUsage), std::to_string(gpuMemoryUsage), std::to_string(networkUsage), std::to_string(diskUsage), std::to_string(temperature)
        );
        co_return true;
    } catch (const std::exception& e) {
        std::cerr << "Failed to insert system metrics: " << e.what() << std::endl;
        co_return false;
    }
}

drogon::Task<std::vector<drogon::orm::Row>> getPaymentsByJobId(const std::string& jobId) {
    auto& db = redsys::Database::getInstance();
    co_return co_await db.executeQuery("SELECT * FROM payments WHERE job_id = $1", jobId);
}

drogon::Task<bool> createPayment(const std::string& jobId, const std::string& fromUserId, 
                                const std::string& toUserId, double amount, double platformFee) {
    try {
        auto& db = redsys::Database::getInstance();
        co_await db.executeQuery(
            "INSERT INTO payments (job_id, from_user_id, to_user_id, amount, platform_fee) VALUES ($1, $2, $3, $4, $5)",
            jobId, fromUserId, toUserId, std::to_string(amount), std::to_string(platformFee)
        );
        co_return true;
    } catch (const std::exception& e) {
        std::cerr << "Failed to create payment: " << e.what() << std::endl;
        co_return false;
    }
}

} // namespace redsys::db 