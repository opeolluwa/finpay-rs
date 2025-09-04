-- Add migration script here

DO $$ BEGIN
CREATE TYPE account_type_enum AS ENUM('freelancer', 'company');
EXCEPTION 
    WHEN duplicate_object THEN null;
END $$;

CREATE TABLE
    IF NOT EXISTS users (
        identifier UUID NOT NULL PRIMARY KEY,
        first_name VARCHAR,
        last_name VARCHAR,
        email VARCHAR NOT NULL UNIQUE,
        password VARCHAR,
        account_type account_type_enum NOT NULL,
        country VARCHAR,
        address VARCHAR,
        phone_number VARCHAR,
        country_code VARCHAR,
        occupation VARCHAR,
        created_date TIMESTAMPTZ NOT NULL DEFAULT NOW (),
        updated_at TIMESTAMPTZ,
        is_verified BOOLEAN DEFAULT FALSE
    )