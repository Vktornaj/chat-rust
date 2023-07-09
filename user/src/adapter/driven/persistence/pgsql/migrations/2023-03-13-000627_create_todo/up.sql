-- Your SQL goes here
CREATE TABLE _user (
    id SERIAL PRIMARY KEY,
    username VARCHAR NOT NULL UNIQUE,
    first_name VARCHAR,
    last_name VARCHAR,
    password VARCHAR NOT NULL
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

-- Add a primary key column to the tag_entry table
-- ALTER IF EXISTS TABLE tag_entry ADD COLUMN tag_entry_id SERIAL PRIMARY KEY;