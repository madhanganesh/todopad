-- Add up migration script here
CREATE TABLE insights (
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
