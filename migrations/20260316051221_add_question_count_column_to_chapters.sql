-- Add migration script here
ALTER TABLE chapters ADD COLUMN question_count INTEGER NOT NULL DEFAULT 0;