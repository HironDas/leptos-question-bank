-- Add migration script here
ALTER TABLE questions ADD COLUMN updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW();
