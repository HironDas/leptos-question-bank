-- Add migration script here

-- Create Subject table
CREATE TABLE IF NOT EXISTS subjects (
    id SERIAL PRIMARY KEY,
    title VARCHAR(250) NOT NULL,
    class_id INTEGER NOT NULL REFERENCES classes(id),
    "order" INTEGER NOT NULL
);