-- This file should undo anything in `up.sql`
ALTER TABLE todo
RENAME COLUMN task_id TO todo_id;