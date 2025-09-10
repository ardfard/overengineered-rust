-- migrate:up
-- Add username and password_hash columns to users table
ALTER TABLE users 
ADD COLUMN username VARCHAR UNIQUE,
ADD COLUMN password_hash VARCHAR;

-- Update existing test user
UPDATE users 
SET username = 'testuser', password_hash = '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewdBPj4J/HS.s8i2' -- 'password123'
WHERE email = 'test@mail.com';

-- migrate:down
-- Remove the added columns
ALTER TABLE users 
DROP COLUMN IF EXISTS username,
DROP COLUMN IF EXISTS password_hash;

