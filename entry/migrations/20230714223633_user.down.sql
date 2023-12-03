-- Add down migration script here

DROP FUNCTION insert_user(
    p_id UUID,
    p_first_name TEXT,
    p_last_name TEXT,
    p_birthday TIMESTAMPTZ,
    p_nationality TEXT,
    p_languages TEXT[]
);
DROP TABLE users_languages;
DROP TABLE languages;
DROP TABLE users;