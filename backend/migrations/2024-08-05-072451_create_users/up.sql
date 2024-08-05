-- Your SQL goes here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    user_type VARCHAR(255) NOT NULL CHECK (user_type IN ('customer', 'admin', 'superadmin')),
    username VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);