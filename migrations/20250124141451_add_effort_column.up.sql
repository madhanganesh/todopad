-- Add up migration script here
ALTER TABLE todos ADD COLUMN effort FLOAT NOT NULL DEFAULT 1.0;
UPDATE todos SET effort = 1.0;
