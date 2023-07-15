-- This file should undo anything in `up.sql`
-- ALTER TABLE IF EXISTS tag_entry DROP COLUMN tag_entry_id;
DROP FUNCTION find_user_by_username(VARCHAR);
DROP TABLE _user;