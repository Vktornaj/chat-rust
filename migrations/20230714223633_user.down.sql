-- Add down migration script here

-- Todo
DROP FUNCTION find_tags_sql(INT);
DROP FUNCTION find_todo_sql(UUID, TEXT, TEXT, INT, TEXT[]);
DROP FUNCTION create_tag(TEXT, UUID);
DROP FUNCTION update_todo(INT, TEXT, TEXT, INT, TIMESTAMPTZ, TIMESTAMPTZ);
DROP TABLE status;
DROP TABLE todo_tag;
DROP TABLE tags;
DROP TABLE todos;

-- User
-- DROP FUNCTION find_user_by_username(TEXT);
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
DROP TABLE users_languages;
DROP TABLE languages;
DROP TABLE users;