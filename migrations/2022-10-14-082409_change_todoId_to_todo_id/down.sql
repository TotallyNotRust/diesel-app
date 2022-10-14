-- This file should undo anything in `up.sql`
ALTER TABLE todo
RENAME COLUMN todo_id TO todoId;