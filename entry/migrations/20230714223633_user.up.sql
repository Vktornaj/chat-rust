-- Add up migration script here

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE users (
    id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    email TEXT UNIQUE,
    phone_number TEXT UNIQUE,
    hashed_password TEXT NOT NULL,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    birthday TIMESTAMPTZ NOT NULL,
    nationality TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE languages (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    code TEXT NOT NULL UNIQUE
);

CREATE TABLE users_languages (
    user_id UUID NOT NULL REFERENCES users(id),
    language_id INTEGER NOT NULL REFERENCES languages(id),
    CONSTRAINT users_languages_pkey PRIMARY KEY (user_id, language_id)
);

--=================== Functions ===================--
-- User 
CREATE FUNCTION insert_user(
    p_email TEXT,
    p_phone_number TEXT,
    p_hashed_password TEXT,
    p_first_name TEXT,
    p_last_name TEXT,
    p_birthday TIMESTAMPTZ,
    p_nationality TEXT,
    p_languages TEXT[]
)
RETURNS TABLE (
    id UUID,
    email TEXT,
    phone_number TEXT,
    hashed_password TEXT,
    first_name TEXT,
    last_name TEXT,
    birthday TIMESTAMPTZ,
    nationality TEXT,
    created_at TIMESTAMPTZ,
    updated_at TIMESTAMPTZ
) AS $$
    DECLARE
        _language_id INTEGER;
        _language_code TEXT;
        id UUID;
    BEGIN
        INSERT INTO users (email, phone_number, hashed_password, first_name, last_name, birthday, nationality)
        VALUES (p_email, p_phone_number, p_hashed_password, p_first_name, p_last_name, p_birthday, p_nationality) 
        RETURNING users.id, users.email, users.phone_number, users.hashed_password, users.first_name, users.last_name, users.birthday, users.nationality, users.created_at, users.updated_at
        INTO id, email, phone_number, hashed_password, first_name, last_name, birthday, nationality, created_at, updated_at;

        FOR _language_code IN SELECT unnest(p_languages) LOOP
            SELECT l.id into _language_id
            FROM languages AS l
            WHERE l.code = _language_code;

            INSERT INTO users_languages (user_id, language_id)
            VALUES (id, _language_id);
        END LOOP;

        RETURN QUERY SELECT id, email, phone_number, hashed_password, first_name, last_name, birthday, nationality, created_at, updated_at;
    END;
$$ LANGUAGE plpgsql;

INSERT INTO languages (code, name)
VALUES
    ('AR', 'Arabic'),
    ('BE', 'Belarusian'),
    ('BG', 'Bulgarian'),
    ('CS', 'Czech'),
    ('CY', 'Welsh'),
    ('DA', 'Danish'),
    ('DE', 'German'),
    ('EL', 'Greek'),
    ('EN', 'English'),
    ('EO', 'Esperanto'),
    ('ES', 'Spanish'),
    ('ET', 'Estonian'),
    ('FI', 'Finnish'),
    ('FR', 'French'),
    ('GA', 'Irish'),
    ('GD', 'Scottish Gaelic'),
    ('HU', 'Hungarian'),
    ('HY', 'Armenian'),
    ('ID', 'Indonesian'),
    ('IS', 'Icelandic'),
    ('IT', 'Italian'),
    ('JA', 'Japanese'),
    ('KO', 'Korean'),
    ('LT', 'Lithuanian'),
    ('LV', 'Latvian'),
    ('MK/SL', 'Macedonian/Slovenian'),
    ('MN', 'Mongolian'),
    ('MO', 'Moldavian'),
    ('NE', 'Nepali'),
    ('NL', 'Dutch'),
    ('NN', 'Norwegian'),
    ('PL', 'Polish'),
    ('PT', 'Portuguese'),
    ('RO', 'Romanian'),
    ('RU', 'Russian'),
    ('SK', 'Slovak'),
    ('SL', 'Slovenian'),
    ('SQ', 'Albanian'),
    ('SR', 'Serbian'),
    ('SV', 'Swedish'),
    ('TH', 'Thai'),
    ('TR', 'Turkish'),
    ('UK', 'Ukrainian'),
    ('VI', 'Vietnamese'),
    ('YI', 'Yiddish'),
    ('ZH', 'Chinese');