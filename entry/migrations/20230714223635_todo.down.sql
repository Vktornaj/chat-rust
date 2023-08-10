-- Add down migration script here

DROP FUNCTION find_tags_sql(INT);
DROP FUNCTION find_todo_sql(UUID, TEXT, TEXT, INT, TEXT[]);
DROP FUNCTION create_tag(TEXT, UUID);
DROP FUNCTION update_todo(INT, TEXT, TEXT, INT, TIMESTAMPTZ, TIMESTAMPTZ);
DROP TABLE status;
DROP TABLE todo_tag;
DROP TABLE tags;
DROP TABLE todos;