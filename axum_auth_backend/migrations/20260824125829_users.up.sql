-- -- Add up migration script here
-- CREATE TYPE user_role AS ENUM ('admin', 'user');
-- CREATE EXTENSION IF NOT EXISTS "uuid-oosp";

-- CREATE TABLE "users" (
--     id UUID NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
--     name VARCHAR(100) NOT NULL,
--     email VARCHAR(255) NOT NULL UNIQUE,
--     verified BOOLEAN NOT NULL DEFAULT FALSE,
--     password VARCHAR(100) NOT NULL,
--     verification_token VARCHAR(255),
--     token_expires_at TIMESTAMP WITH TIME ZONE,
--     role user_role NOT NULL DEFAULT 'user',
--     created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
--     updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
-- );

-- CREATE INDEX users_email_idx ON users (email);


-- Create enum type safely if it doesn't exist
DO $$ BEGIN
    CREATE TYPE user_role AS ENUM ('admin', 'user');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

-- Create table using native Postgres UUID generator (no extensions required)
CREATE TABLE IF NOT EXISTS "users" (
    id UUID NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    verified BOOLEAN NOT NULL DEFAULT FALSE,
    password VARCHAR(100) NOT NULL,
    verification_token VARCHAR(255),
    token_expires_at TIMESTAMP WITH TIME ZONE,
    role user_role NOT NULL DEFAULT 'user',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS users_email_idx ON users (email);