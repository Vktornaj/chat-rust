-- This file should undo anything in `up.sql`
-- ALTER TABLE IF EXISTS tag_entry DROP COLUMN tag_entry_id;
DROP FUNCTION find_tags_sql(INT);
DROP FUNCTION find_user_by_username(VARCHAR);
DROP FUNCTION find_todo_sql(VARCHAR, VARCHAR, VARCHAR, INT, VARCHAR[]);
DROP FUNCTION create_tag(CHARACTER varying, CHARACTER varying);
DROP FUNCTION update_todo(INT, VARCHAR, VARCHAR, INT, TIMESTAMPTZ, TIMESTAMPTZ);
DROP TABLE _status;
DROP TABLE _todo_tag;
DROP TABLE _tag;
DROP TABLE _todo;
DROP TABLE _user;