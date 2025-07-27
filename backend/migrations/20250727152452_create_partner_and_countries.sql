-- Add migration script here
-- Create countries table
CREATE TABLE countries (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL UNIQUE
);

-- Create partners table
CREATE TABLE partners (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    country TEXT NOT NULL,
    city TEXT,
    email TEXT,
    website_url TEXT,
    created_at TIMESTAMP DEFAULT NOW()
);

-- Adding some countries straight away for testing purposes
INSERT INTO countries (name) VALUES 
('Germany'),
('France'),
('Georgia'),
('USA');
