-- Add migration script here

-- create class table
CREATE TABLE IF NOT EXISTS classes (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL,
    name_bn VARCHAR(120) NOT NULL,
    "order" INTEGER NOT NULL
);