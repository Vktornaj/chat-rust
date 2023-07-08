-- Your SQL goes here
CREATE TABLE _user (
    id SERIAL PRIMARY KEY,
    username VARCHAR NOT NULL UNIQUE,
    first_name VARCHAR,
    last_name VARCHAR,
    password VARCHAR NOT NULL
);

CREATE TABLE _status (
    id SERIAL PRIMARY KEY,
    status_value VARCHAR NOT NULL
);

CREATE TABLE _tag (
    id SERIAL PRIMARY KEY,
    username VARCHAR NOT NULL,
    tag_value VARCHAR NOT NULL,
    CONSTRAINT fk_user FOREIGN KEY(username) REFERENCES _user(username)
);

CREATE TABLE _todo (
    id SERIAL PRIMARY KEY,
    username VARCHAR NOT NULL,
    title VARCHAR NOT NULL,
    description VARCHAR,
    status INT NOT NULL,
    create_date TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    done_date TIMESTAMP WITH TIME ZONE,
    deadline TIMESTAMP WITH TIME ZONE,
    CONSTRAINT fk_user FOREIGN KEY(username) REFERENCES _user(username)
);

CREATE TABLE _todo_tag (
    todo_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    PRIMARY KEY (todo_id, tag_id),
    CONSTRAINT fk_todo FOREIGN KEY(todo_id) REFERENCES _todo(id),
    CONSTRAINT fk_tag FOREIGN KEY(tag_id) REFERENCES _tag(id)
);

-- Functions
CREATE FUNCTION find_user_by_username(_username VARCHAR)
RETURNS TABLE (
    id integer,
    first_name varchar,
    last_name varchar
) AS $$
    BEGIN
        RETURN QUERY
        SELECT u.id, u.first_name, u.last_name
        FROM _user AS u
        WHERE u.username = _username;
    END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION find_todo_sql(
    p_username varchar,
    p_title varchar,
    p_description varchar,
    p_status INT,
    p_tags varchar[]
)
RETURNS TABLE (
    id INT,
    username varchar,
    title varchar,
    description varchar,
    status INT,
    create_date TIMESTAMPTZ,
    done_date TIMESTAMPTZ,
    deadline TIMESTAMPTZ
)
AS $$
BEGIN
    RETURN QUERY
    SELECT t.id, t.username, t.title, t.description, t.status, t.create_date, t.done_date, t.deadline
    FROM _todo AS t
    WHERE
        (p_title IS NULL OR p_title = t.title) AND
        (p_description IS NULL OR p_description = t.description) AND
        (p_status IS NULL OR p_status = t.status) AND
--         (p_tags IS NULL OR p_tags @> t.tags) AND
        (p_username = t.username)
    LIMIT 1;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION find_tags_sql(p_todo_id INT)
RETURNS VARCHAR[]
AS $$
DECLARE
    tags VARCHAR[];
BEGIN
    SELECT array_agg(t.tag_value)
    INTO tags
    FROM _tag AS t
    JOIN _todo_tag AS tt ON tt.tag_id = t.id
    WHERE tt.todo_id = p_todo_id;

    RETURN tags;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION create_tag(p_tag_value VARCHAR, p_username VARCHAR)
RETURNS TABLE (id INT, tag_value VARCHAR)
AS $$
DECLARE
    tag_entry _tag;
BEGIN
    SELECT *
    INTO tag_entry
    FROM _tag AS t
    JOIN _user AS u ON u.username = t.username
    WHERE t.tag_value = p_tag_value;
    IF tag_entry IS NULL THEN
        INSERT INTO _tag (tag_value, username)
        VALUES (p_tag_value, p_username)
        RETURNING _tag.id, _tag.username, _tag.tag_value INTO tag_entry;
    END IF;
    RETURN QUERY SELECT tag_entry.id, tag_entry.tag_value;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION update_todo(
    p_id INT,
    p_title VARCHAR,
    p_description VARCHAR,
    p_status INT,
    p_done_date TIMESTAMPTZ DEFAULT '9999-12-31 23:59:59.999999+00',
    p_deadline TIMESTAMPTZ DEFAULT '9999-12-31 23:59:59.999999+00'
)
RETURNS TABLE (
    id INT,
    username VARCHAR,
    title VARCHAR,
    description VARCHAR,
    status INT,
    create_date TIMESTAMPTZ,
    done_date TIMESTAMPTZ,
    deadline TIMESTAMPTZ
)
AS $$
BEGIN
    UPDATE _todo AS t
    SET
        username = t.username,
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


-- Add a primary key column to the tag_entry table
-- ALTER IF EXISTS TABLE tag_entry ADD COLUMN tag_entry_id SERIAL PRIMARY KEY;