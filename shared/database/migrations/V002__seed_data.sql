-- Migration: V002__seed_data.sql
-- Description: Seed data for Redsys development and testing
-- Created: 2024-07-09
-- Author: Redsys Development Team
-- Version: 1.0.0

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
('770e8400-e29b-41d4-a716-446655440002', '550e8400-e29b-41d4-a716-446655440001', 'Data Processing Pipeline', 'Process large dataset for analysis', 'python:3.9-slim', 'python process_data.py', 0, 4, 1.0, 0.1000, 'pending'); 