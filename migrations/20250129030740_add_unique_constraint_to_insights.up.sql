-- Add up migration script here
PRAGMA foreign_keys=OFF;

-- Step 1: Create a new table with the correct UNIQUE constraint
CREATE TABLE insights_new (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    user_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    metric TEXT NOT NULL,
    chart_type TEXT NOT NULL,
    periods TEXT,
    tags TEXT,
    UNIQUE(user_id, name),  
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
);

-- Step 2: Copy data from old table into new table
INSERT INTO insights_new (id, user_id, name, description, metric, chart_type, periods, tags)
SELECT id, user_id, name, description, metric, chart_type, periods, tags FROM insights;

-- Step 3: Drop old table and rename new table
DROP TABLE insights;
ALTER TABLE insights_new RENAME TO insights;

PRAGMA foreign_keys=ON;
