CREATE TABLE contacts(
    id UUID NOT NULL,
    user_id UUID NOT NULL,
    alias TEXT NOT NULL,
    block BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT fk_user FOREIGN KEY(user_id) REFERENCES users(id)
    PRIMARY KEY (user_id, id)
);
