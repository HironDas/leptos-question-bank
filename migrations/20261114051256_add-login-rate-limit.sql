-- Add migration script here
CREATE TABLE IF NOT EXISTS login_rate_limits{
    key BYTEA PRIMARY KEY,
    attempts INTEGER NOT NULL DEFAULT 0,
    window_start TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
}

CREATE INDEX idx_login_rate_limits_updated_at
ON login_rate_limits (updated_at);
