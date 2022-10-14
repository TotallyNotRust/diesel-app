-- Your SQL goes here
ALTER TABLE todo
  ADD is_completed INTEGER NOT NULL default(0);