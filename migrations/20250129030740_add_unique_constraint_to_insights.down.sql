-- Add down migration script here
PRAGMA foreign_keys=OFF;

-- Revert back to original schema (without UNIQUE constraint)
CREATE TABLE insights_old (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    user_id INTEGER NOT NULL,
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    metric TEXT NOT NULL,
    chart_type TEXT NOT NULL,
    periods TEXT,
    tags TEXT,
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
);

-- Copy data back to old table
INSERT INTO insights_old (id, user_id, name, description, metric, chart_type, periods, tags)
SELECT id, user_id, name, description, metric, chart_type, periods, tags FROM insights;

-- Drop modified table and rename back to old schema
DROP TABLE insights;
ALTER TABLE insights_old RENAME TO insights;

PRAGMA foreign_keys=ON;
