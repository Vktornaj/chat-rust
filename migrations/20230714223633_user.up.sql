-- Add up migration script here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    email VARCHAR UNIQUE,
    phone_number VARCHAR UNIQUE,
    password VARCHAR NOT NULL,
    first_name VARCHAR,
    last_name VARCHAR,
    birthday TIMESTAMPTZ NOT NULL,
    nationality VARCHAR NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE languages (
    id VARCHAR NOT NULL UNIQUE PRIMARY KEY,
    name VARCHAR NOT NULL UNIQUE
);

CREATE TABLE users_languages (
    user_id INTEGER NOT NULL REFERENCES users(id),
    language_id VARCHAR NOT NULL REFERENCES languages(id),
    CONSTRAINT users_languages_pkey PRIMARY KEY (user_id, language_id)
);

--=================== Functions ===================--
CREATE FUNCTION insert_user(
    p_email VARCHAR,
    p_phone_number VARCHAR,
    p_password VARCHAR,
    p_first_name VARCHAR,
    p_last_name VARCHAR,
    p_birthday TIMESTAMPTZ,
    p_nationality VARCHAR,
    p_languages VARCHAR[]
)
RETURNS TABLE (
    id INTEGER,
    email VARCHAR,
    phone_number VARCHAR,
    password VARCHAR,
    first_name VARCHAR,
    last_name VARCHAR,
    birthday TIMESTAMPTZ,
    nationality VARCHAR,
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
-- CREATE FUNCTION find_user_by_email(_email VARCHAR)
-- RETURNS TABLE (
--     id integer,
--     first_name varchar,
--     last_name varchar
-- ) AS $$
--     BEGIN
--         RETURN QUERY
--         SELECT u.id, u.first_name, u.last_name
--         FROM users AS u
--         WHERE u.username = _username;
--     END;
-- $$ LANGUAGE plpgsql;

-- CREATE FUNCTION find_user_by_phone_number(_email VARCHAR)
-- RETURNS TABLE (
--     id integer,
--     first_name varchar,
--     last_name varchar
-- ) AS $$
--     BEGIN
--         RETURN QUERY
--         SELECT u.id, u.first_name, u.last_name
--         FROM users AS u
--         WHERE u.username = _username;
--     END;
-- $$ LANGUAGE plpgsql;