-- Add down migration script here
-- DROP FUNCTION find_user_by_username(VARCHAR);
DROP FUNCTION insert_user(
    p_email TEXT,
    p_phone_number TEXT,
    p_password TEXT,
    p_first_name TEXT,
    p_last_name TEXT,
    p_birthday TIMESTAMPTZ,
    p_nationality TEXT,
    p_languages TEXT[]
);
DROP FUNCTION update_user(
    p_id INT,
    p_email TEXT,
    p_phone_number TEXT,
    p_password TEXT,
    p_first_name TEXT,
    p_last_name TEXT,
    p_birthday TIMESTAMPTZ,
    p_nationality TEXT,
    p_languages TEXT[]
);
DROP TABLE users_languages;
DROP TABLE languages;
DROP TABLE users;