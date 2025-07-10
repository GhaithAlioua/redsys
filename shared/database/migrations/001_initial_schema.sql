-- Migration: 001_initial_schema.sql
-- Description: Initial database schema for Redsys GPU compute marketplace
-- Created: 2024-07-09
-- Author: Redsys Development Team
-- Version: 1.0.0

-- Enable required extensions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- Database versioning table (enterprise standard)
CREATE TABLE schema_migrations (
    version VARCHAR(50) PRIMARY KEY,
    applied_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    checksum VARCHAR(64) NOT NULL,
    description TEXT
);

-- Users table (authentication and profile data)
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email VARCHAR(255) UNIQUE NOT NULL,
    username VARCHAR(100) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    role VARCHAR(50) NOT NULL DEFAULT 'user' CHECK (role IN ('user', 'provider', 'admin')),
    is_active BOOLEAN DEFAULT true,
    is_deleted BOOLEAN DEFAULT false, -- Soft delete for audit compliance
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Providers table (GPU owners)
CREATE TABLE providers (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    gpu_count INTEGER NOT NULL DEFAULT 1,
    gpu_memory_gb INTEGER NOT NULL,
    gpu_model VARCHAR(100) NOT NULL,
    cuda_version VARCHAR(50),
    hourly_rate DECIMAL(10,4) NOT NULL DEFAULT 0.1000,
    is_available BOOLEAN DEFAULT true,
    is_deleted BOOLEAN DEFAULT false, -- Soft delete
    last_heartbeat TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Jobs table (compute tasks)
CREATE TABLE jobs (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    provider_id UUID REFERENCES providers(id) ON DELETE SET NULL,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    docker_image VARCHAR(255) NOT NULL,
    docker_command TEXT,
    gpu_requirements INTEGER NOT NULL DEFAULT 1,
    memory_requirements_gb INTEGER NOT NULL DEFAULT 4,
    estimated_duration_hours DECIMAL(5,2),
    budget DECIMAL(10,4) NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'assigned', 'running', 'completed', 'failed', 'cancelled')),
    priority INTEGER DEFAULT 5 CHECK (priority BETWEEN 1 AND 10),
    is_deleted BOOLEAN DEFAULT false, -- Soft delete
    started_at TIMESTAMP WITH TIME ZONE,
    completed_at TIMESTAMP WITH TIME ZONE,
    result_url TEXT,
    error_message TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Payments table (transaction tracking)
CREATE TABLE payments (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    job_id UUID NOT NULL REFERENCES jobs(id) ON DELETE CASCADE,
    from_user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    to_user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    amount DECIMAL(10,4) NOT NULL,
    platform_fee DECIMAL(10,4) NOT NULL DEFAULT 0.0500,
    status VARCHAR(50) NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'completed', 'failed', 'refunded')),
    transaction_hash VARCHAR(255), -- For blockchain integration
    is_deleted BOOLEAN DEFAULT false, -- Soft delete
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- System monitoring table (health metrics)
CREATE TABLE system_metrics (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    provider_id UUID NOT NULL REFERENCES providers(id) ON DELETE CASCADE,
    cpu_usage_percent DECIMAL(5,2),
    memory_usage_percent DECIMAL(5,2),
    gpu_usage_percent DECIMAL(5,2),
    gpu_memory_usage_percent DECIMAL(5,2),
    network_usage_mbps DECIMAL(10,2),
    disk_usage_percent DECIMAL(5,2),
    temperature_celsius DECIMAL(5,2),
    recorded_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Job assignments table (for job scheduling)
CREATE TABLE job_assignments (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    job_id UUID NOT NULL REFERENCES jobs(id) ON DELETE CASCADE,
    provider_id UUID NOT NULL REFERENCES providers(id) ON DELETE CASCADE,
    assigned_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    started_at TIMESTAMP WITH TIME ZONE,
    completed_at TIMESTAMP WITH TIME ZONE,
    status VARCHAR(50) NOT NULL DEFAULT 'assigned' CHECK (status IN ('assigned', 'running', 'completed', 'failed')),
    UNIQUE(job_id, provider_id)
);

-- Create indexes for performance (enterprise optimization)
CREATE INDEX idx_users_email ON users(email) WHERE is_deleted = false;
CREATE INDEX idx_users_username ON users(username) WHERE is_deleted = false;
CREATE INDEX idx_users_role ON users(role) WHERE is_deleted = false;
CREATE INDEX idx_providers_user_id ON providers(user_id) WHERE is_deleted = false;
CREATE INDEX idx_providers_available ON providers(is_available) WHERE is_available = true AND is_deleted = false;
CREATE INDEX idx_providers_gpu_model ON providers(gpu_model) WHERE is_deleted = false;
CREATE INDEX idx_jobs_user_id ON jobs(user_id) WHERE is_deleted = false;
CREATE INDEX idx_jobs_provider_id ON jobs(provider_id) WHERE is_deleted = false;
CREATE INDEX idx_jobs_status ON jobs(status) WHERE is_deleted = false;
CREATE INDEX idx_jobs_created_at ON jobs(created_at) WHERE is_deleted = false;
CREATE INDEX idx_jobs_priority_status ON jobs(priority, status) WHERE is_deleted = false;
CREATE INDEX idx_payments_job_id ON payments(job_id) WHERE is_deleted = false;
CREATE INDEX idx_payments_status ON payments(status) WHERE is_deleted = false;
CREATE INDEX idx_payments_from_user ON payments(from_user_id) WHERE is_deleted = false;
CREATE INDEX idx_payments_to_user ON payments(to_user_id) WHERE is_deleted = false;
CREATE INDEX idx_system_metrics_provider_id ON system_metrics(provider_id);
CREATE INDEX idx_system_metrics_recorded_at ON system_metrics(recorded_at);
CREATE INDEX idx_job_assignments_job_id ON job_assignments(job_id);
CREATE INDEX idx_job_assignments_provider_id ON job_assignments(provider_id);
CREATE INDEX idx_job_assignments_status ON job_assignments(status);

-- Create updated_at trigger function
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Apply updated_at triggers
CREATE TRIGGER update_users_updated_at BEFORE UPDATE ON users FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_providers_updated_at BEFORE UPDATE ON providers FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_jobs_updated_at BEFORE UPDATE ON jobs FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_payments_updated_at BEFORE UPDATE ON payments FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Row Level Security (RLS) policies for enterprise security
ALTER TABLE users ENABLE ROW LEVEL SECURITY;
ALTER TABLE providers ENABLE ROW LEVEL SECURITY;
ALTER TABLE jobs ENABLE ROW LEVEL SECURITY;
ALTER TABLE payments ENABLE ROW LEVEL SECURITY;

-- RLS Policies (basic implementation - can be enhanced)
CREATE POLICY users_own_data ON users FOR ALL USING (id = current_setting('app.current_user_id')::uuid OR current_setting('app.current_user_role') = 'admin');
CREATE POLICY providers_own_data ON providers FOR ALL USING (user_id = current_setting('app.current_user_id')::uuid OR current_setting('app.current_user_role') = 'admin');
CREATE POLICY jobs_own_data ON jobs FOR ALL USING (user_id = current_setting('app.current_user_id')::uuid OR current_setting('app.current_user_role') = 'admin');
CREATE POLICY payments_own_data ON payments FOR ALL USING (from_user_id = current_setting('app.current_user_id')::uuid OR to_user_id = current_setting('app.current_user_id')::uuid OR current_setting('app.current_user_role') = 'admin');

-- Insert initial migration record
INSERT INTO schema_migrations (version, checksum, description) VALUES 
('001_initial_schema', encode(digest('Initial schema creation', 'sha256'), 'hex'), 'Initial database schema for Redsys GPU compute marketplace'); 