-- Add migration script here
ALTER TABLE questions ADD COLUMN difficulty INTEGER NOT NULL DEFAULT 0;
ALTER TABLE questions DROP COLUMN IF EXISTS "order";
