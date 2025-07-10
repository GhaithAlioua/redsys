-- Seed Data: 001_initial_data.sql
-- Description: Initial seed data for Redsys development and testing
-- Created: 2024-07-09
-- Author: Redsys Development Team

-- Insert admin user (password: admin123)
INSERT INTO users (id, email, username, password_hash, role) VALUES 
('550e8400-e29b-41d4-a716-446655440000', 'admin@redsys.com', 'admin', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewdBPj/RK.s5uO.G', 'admin');

-- Insert sample users
INSERT INTO users (id, email, username, password_hash, role) VALUES 
('550e8400-e29b-41d4-a716-446655440001', 'user1@example.com', 'user1', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewdBPj/RK.s5uO.G', 'user'),
('550e8400-e29b-41d4-a716-446655440002', 'provider1@example.com', 'provider1', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewdBPj/RK.s5uO.G', 'provider'),
('550e8400-e29b-41d4-a716-446655440003', 'provider2@example.com', 'provider2', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewdBPj/RK.s5uO.G', 'provider');

-- Insert sample providers
INSERT INTO providers (id, user_id, name, description, gpu_count, gpu_memory_gb, gpu_model, cuda_version, hourly_rate) VALUES 
('660e8400-e29b-41d4-a716-446655440001', '550e8400-e29b-41d4-a716-446655440002', 'High-Performance GPU Cluster', 'RTX 4090 with 24GB VRAM', 1, 24, 'RTX 4090', '12.0', 0.1500),
('660e8400-e29b-41d4-a716-446655440002', '550e8400-e29b-41d4-a716-446655440003', 'Multi-GPU Workstation', 'Dual RTX 3080 setup', 2, 20, 'RTX 3080', '11.8', 0.2000);

-- Insert sample jobs
INSERT INTO jobs (id, user_id, title, description, docker_image, docker_command, gpu_requirements, memory_requirements_gb, estimated_duration_hours, budget, status) VALUES 
('770e8400-e29b-41d4-a716-446655440001', '550e8400-e29b-41d4-a716-446655440001', 'ML Training Job', 'Train a deep learning model on image dataset', 'pytorch/pytorch:latest', 'python train.py --epochs 100', 1, 8, 2.5, 0.3750, 'pending'),
('770e8400-e29b-41d4-a716-446655440002', '550e8400-e29b-41d4-a716-446655440001', 'Data Processing Pipeline', 'Process large dataset with GPU acceleration', 'nvidia/cuda:11.8-base', 'python process_data.py', 1, 16, 1.0, 0.1500, 'pending'),
('770e8400-e29b-41d4-a716-446655440003', '550e8400-e29b-41d4-a716-446655440001', 'Multi-GPU Inference', 'Run inference on multiple GPUs', 'tensorflow/tensorflow:latest-gpu', 'python inference.py --gpus 2', 2, 12, 0.5, 0.2000, 'pending');

-- Insert sample system metrics
INSERT INTO system_metrics (provider_id, cpu_usage_percent, memory_usage_percent, gpu_usage_percent, gpu_memory_usage_percent, network_usage_mbps, disk_usage_percent, temperature_celsius) VALUES 
('660e8400-e29b-41d4-a716-446655440001', 45.2, 67.8, 23.4, 34.5, 125.6, 42.1, 68.5),
('660e8400-e29b-41d4-a716-446655440002', 38.7, 54.2, 12.8, 28.9, 89.3, 38.7, 65.2);

-- Insert sample job assignments
INSERT INTO job_assignments (job_id, provider_id, status) VALUES 
('770e8400-e29b-41d4-a716-446655440001', '660e8400-e29b-41d4-a716-446655440001', 'assigned'),
('770e8400-e29b-41d4-a716-446655440002', '660e8400-e29b-41d4-a716-446655440001', 'assigned'),
('770e8400-e29b-41d4-a716-446655440003', '660e8400-e29b-41d4-a716-446655440002', 'assigned'); 