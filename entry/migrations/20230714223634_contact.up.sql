CREATE TABLE contacts(
    id UUID NOT NULL,
    user_id UUID NOT NULL,
    alias TEXT,
    is_blocked BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT fk_user FOREIGN KEY(user_id) REFERENCES auths(user_id),
    PRIMARY KEY (user_id, id)
);
