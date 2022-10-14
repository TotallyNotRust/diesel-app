-- This file should undo anything in `up.sql`
ALTER TABLE todo
  DROP COLUMN is_completed;