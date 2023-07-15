-- Add down migration script here
-- DROP FUNCTION find_user_by_username(VARCHAR);
DROP FUNCTION insert_user(
    p_email VARCHAR,
    p_phone_number VARCHAR,
    p_password VARCHAR,
    p_first_name VARCHAR,
    p_last_name VARCHAR,
    p_birthday TIMESTAMPTZ,
    p_nationality VARCHAR,
    p_languages VARCHAR[]
);
DROP TABLE users_languages;
DROP TABLE languages;
DROP TABLE users;