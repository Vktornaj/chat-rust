-- Add up migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- User 
CREATE TABLE users (
    id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
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
    user_id UUID NOT NULL REFERENCES users(id),
    language_id TEXT NOT NULL REFERENCES languages(id),
    CONSTRAINT users_languages_pkey PRIMARY KEY (user_id, language_id)
);
-- Todo 
CREATE TABLE status (
    id SERIAL PRIMARY KEY,
    status_value TEXT NOT NULL
);

CREATE TABLE tags (
    id SERIAL PRIMARY KEY,
    user_id UUID NOT NULL,
    tag_value TEXT NOT NULL,
    CONSTRAINT fk_user FOREIGN KEY(user_id) REFERENCES users(id)
);

CREATE TABLE todos (
    id SERIAL PRIMARY KEY,
    user_id UUID NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    status INT NOT NULL,
    create_date TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    done_date TIMESTAMP WITH TIME ZONE,
    deadline TIMESTAMP WITH TIME ZONE,
    CONSTRAINT fk_user FOREIGN KEY(user_id) REFERENCES users(id)
);

CREATE TABLE todo_tag (
    todo_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    PRIMARY KEY (todo_id, tag_id),
    CONSTRAINT fk_todo FOREIGN KEY(todo_id) REFERENCES todos(id),
    CONSTRAINT fk_tag FOREIGN KEY(tag_id) REFERENCES tags(id)
);

--=================== Functions ===================--
-- User 
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
    id UUID,
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
        _user_id UUID;
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

-- Todo
CREATE OR REPLACE FUNCTION find_todo_sql(
    p_user_id UUID,
    p_title TEXT,
    p_description TEXT,
    p_status INT,
    p_tags TEXT[]
)
RETURNS TABLE (
    id INT,
    user_id UUID,
    title TEXT,
    description TEXT,
    status INT,
    create_date TIMESTAMPTZ,
    done_date TIMESTAMPTZ,
    deadline TIMESTAMPTZ
)
AS $$
BEGIN
    RETURN QUERY
    SELECT t.id, t.user_id, t.title, t.description, t.status, t.create_date, t.done_date, t.deadline
    FROM todos AS t
    WHERE
        (p_title IS NULL OR p_title = t.title) AND
        (p_description IS NULL OR p_description = t.description) AND
        (p_status IS NULL OR p_status = t.status) AND
        (p_user_id = t.user_id)
    LIMIT 1;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION find_tags_sql(p_todo_id INT)
RETURNS TEXT[]
AS $$
DECLARE
    tags TEXT[];
BEGIN
    SELECT array_agg(t.tag_value)
    INTO tags
    FROM tags AS t
    JOIN todo_tag AS tt ON tt.tag_id = t.id
    WHERE tt.todo_id = p_todo_id;

    RETURN tags;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION create_tag(p_tag_value TEXT, p_user_id UUID)
RETURNS TABLE (id INT, tag_value TEXT)
AS $$
DECLARE
    tag_entry tags;
BEGIN
    SELECT *
    INTO tag_entry
    FROM tags AS t
    JOIN users AS u ON u.user_id = t.user_id
    WHERE t.tag_value = p_tag_value;
    IF tag_entry IS NULL THEN
        INSERT INTO tags (tag_value, user_id)
        VALUES (p_tag_value, p_user_id)
        RETURNING tags.id, tags.user_id, tags.tag_value INTO tag_entry;
    END IF;
    RETURN QUERY SELECT tag_entry.id, tag_entry.tag_value;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION update_todo(
    p_id INT,
    p_title TEXT,
    p_description TEXT,
    p_status INT,
    p_done_date TIMESTAMPTZ DEFAULT '9999-12-31 23:59:59.999999+00',
    p_deadline TIMESTAMPTZ DEFAULT '9999-12-31 23:59:59.999999+00'
)
RETURNS TABLE (
    id INT,
    user_id UUID,
    title TEXT,
    description TEXT,
    status INT,
    create_date TIMESTAMPTZ,
    done_date TIMESTAMPTZ,
    deadline TIMESTAMPTZ
)
AS $$
BEGIN
    UPDATE todos AS t
    SET
        user_id = t.user_id,
        title = COALESCE(p_title, t.title),
        description = COALESCE(p_description, t.description),
        status = COALESCE(p_status, t.status),
        done_date = CASE
            WHEN p_done_date = '9999-12-31 23:59:59.999999+00' THEN t.done_date
            ELSE p_done_date
        END,
        deadline = CASE
            WHEN p_deadline = '9999-12-31 23:59:59.999999+00' THEN t.deadline
            ELSE p_deadline
        END
    WHERE t.id = p_id
    RETURNING
        t.id,
        t.user_id,
        t.title,
        t.description,
        t.status,
        t.create_date,
        t.done_date,
        t.deadline
    INTO
        id,
        user_id,
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