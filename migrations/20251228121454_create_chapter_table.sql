-- Add migration script here

-- Create Chapter table
CREATE TABLE IF NOT EXISTS chapters (
    id SERIAL PRIMARY KEY,
    title VARCHAR(250) NOT NULL,
    class_id INTEGER NOT NULL REFERENCES classes(id),
    subject_id INTEGER NOT NULL REFERENCES subjects(id),
    "order" INTEGER NOT NULL
);