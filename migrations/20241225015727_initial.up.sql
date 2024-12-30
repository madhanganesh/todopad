-- Users table
CREATE TABLE users (
    id TEXT NOT NULL UNIQUE,
    hashed_password TEXT NOT NULL
);

-- Todos table
CREATE TABLE todos (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id TEXT NOT NULL,
    title TEXT NOT NULL,
    due DATE DEFAULT (DATE('now')),
    completed BOOLEAN NOT NULL DEFAULT 0,
    notes TEXT,
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
);

-- Tags table
CREATE TABLE tags (
    user_id TEXT NOT NULL,
    todo_id INTEGER NOT NULL,
    tag TEXT NOT NULL,
    PRIMARY KEY (user_id, todo_id, tag),
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
    FOREIGN KEY (todo_id) REFERENCES todos (id) ON DELETE CASCADE
);
