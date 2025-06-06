-- Enable necessary extensions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pgcrypto";
CREATE EXTENSION IF NOT EXISTS "citext";

-- Schema for authentication related tables
CREATE SCHEMA auth;

-- =============================================
-- Core Authentication Tables
-- =============================================

-- Users table
CREATE TABLE auth.users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    external_id VARCHAR(255) NULL UNIQUE,  -- For linking with User Management service
    username CITEXT UNIQUE NULL,  -- NULL for SSO-only users
    email CITEXT UNIQUE NOT NULL,
    password_hash VARCHAR(255) NULL,  -- NULL for SSO-only users
    password_updated_at TIMESTAMPTZ NULL,
    password_reset_required BOOLEAN DEFAULT FALSE,
    failed_login_attempts INTEGER DEFAULT 0,
    last_failed_attempt TIMESTAMPTZ NULL,
    account_locked_until TIMESTAMPTZ NULL,
    email_verified BOOLEAN DEFAULT FALSE,
    email_verification_token VARCHAR(255) NULL,
    email_verification_sent_at TIMESTAMPTZ NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    requires_mfa BOOLEAN DEFAULT FALSE,
    auth_provider VARCHAR(50) DEFAULT 'local',  -- 'local', 'google', 'microsoft', 'apple', 'lti', etc.
    user_state VARCHAR(50) DEFAULT 'registered', 
    access_range VARCHAR(50) NOT NULL,
    deletion_scheduled_at TIMESTAMPTZ NULL,
    CONSTRAINT valid_auth_provider CHECK (auth_provider IN 
        ('local', 'google', 'microsoft', 'apple', 'facebook', 'lti', 'saml', 'ldap', 'custom')),
    CONSTRAINT valid_user_state CHECK (user_state IN 
        ('registered', 'verified','active','incomplete','disabled','locked','deleted'))
);

-- Create index on fields commonly used in auth queries
CREATE INDEX idx_users_email ON auth.users(email);
CREATE INDEX idx_users_username ON auth.users(username);
CREATE INDEX idx_users_external_id ON auth.users(external_id);
CREATE INDEX idx_users_auth_provider ON auth.users(auth_provider);


-- Sessions for user auth
CREATE TABLE auth.sessions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
    refresh_token_hash TEXT NOT NULL,
    device_identifier VARCHAR(255) NULL,
    device_name VARCHAR(255) NULL,
    device_type VARCHAR(50) NULL,  -- 'mobile', 'desktop', 'tablet', etc.
    ip_address INET NULL,
    user_agent TEXT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_active_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    is_revoked BOOLEAN DEFAULT FALSE,
    revoked_reason VARCHAR(50) NULL,
    revoked_at TIMESTAMPTZ NULL
);
