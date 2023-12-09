-- Add up migration script here

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE auths (
    user_id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    hashed_password TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE tokens_metadata (
    token_id UUID NOT NULL PRIMARY KEY,
    user_id UUID NOT NULL UNIQUE,
    creation_timestamp INTEGER NOT NULL,
    last_use_timestamp INTEGER NOT NULL,
    is_active BOOLEAN NOT NULL,
    browser TEXT NOT NULL,
    os TEXT NOT NULL,
    CONSTRAINT fk_auth FOREIGN KEY(user_id) REFERENCES auths(user_id)
);