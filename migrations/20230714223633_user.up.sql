-- Add up migration script here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    email TEXT UNIQUE,
    phone_number TEXT UNIQUE,
    password TEXT NOT NULL,
    first_name TEXT,
    last_name TEXT,
    birthday TIMESTAMPTZ NOT NULL,
    nationality TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE languages (
    id TEXT NOT NULL UNIQUE PRIMARY KEY,
    name TEXT NOT NULL UNIQUE
);

CREATE TABLE users_languages (
    user_id INTEGER NOT NULL REFERENCES users(id),
    language_id TEXT NOT NULL REFERENCES languages(id),
    CONSTRAINT users_languages_pkey PRIMARY KEY (user_id, language_id)
);

--=================== Functions ===================--
CREATE FUNCTION insert_user(
    p_email TEXT,
    p_phone_number TEXT,
    p_password TEXT,
    p_first_name TEXT,
    p_last_name TEXT,
    p_birthday TIMESTAMPTZ,
    p_nationality TEXT,
    p_languages TEXT[]
)
RETURNS TABLE (
    id INTEGER,
    email TEXT,
    phone_number TEXT,
    password TEXT,
    first_name TEXT,
    last_name TEXT,
    birthday TIMESTAMPTZ,
    nationality TEXT,
    created_at TIMESTAMPTZ,
    updated_at TIMESTAMPTZ
) AS $$
    DECLARE
        _language_id INTEGER;
        _user_id INTEGER;
    BEGIN
        INSERT INTO users (email, phone_number, password, first_name, last_name, birthday, nationality)
        VALUES (p_email, p_phone_number, p_password, p_first_name, p_last_name, p_birthday, nationality) 
        RETURNING id, email, phone_number, password, first_name, last_name, birthday, nationality, created_at, updated_at
        INTO id, email, phone_number, password, first_name, last_name, birthday, nationality, created_at, updated_at;

        FOR _language_id IN SELECT unnest(p_languages) LOOP
            INSERT INTO user_languages (user_id, language_id)
            VALUES (_user_id, _language_id);
        END LOOP;

        RETURN NEXT;
    END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION update_user(
    p_id INTEGER,
    p_email TEXT,
    p_phone_number TEXT,
    p_password TEXT,
    p_first_name TEXT,
    p_last_name TEXT,
    p_birthday TIMESTAMPTZ,
    p_nationality TEXT,
    p_languages TEXT[]
)
RETURNS TABLE (
    id INTEGER,
    email TEXT,
    phone_number TEXT,
    password TEXT,
    first_name TEXT,
    last_name TEXT,
    birthday TIMESTAMPTZ,
    nationality TEXT,
    created_at TIMESTAMPTZ,
    updated_at TIMESTAMPTZ
)
AS $$
BEGIN
    UPDATE _todo AS t
    SET
        username = t.username,
        title = COALESCE(p_title, t.title),
        description = COALESCE(p_description, t.description),
        status = COALESCE(p_status, t.status),
        done_date = COALESCE(p_done_date, t.done_date),
    WHERE t.id = p_id
    RETURNING
        t.id,
        t.username,
        t.title,
        t.description,
        t.status,
        t.create_date,
        t.done_date,
        t.deadline
    INTO
        id,
        username,
        title,
        description,
        status,
        create_date,
        done_date,
        deadline;

    RETURN NEXT;

    RETURN;
END;
$$ LANGUAGE plpgsql;
