-- Seed users
INSERT OR IGNORE INTO users (id, hashed_password) VALUES
('admin', '$argon2id$v=19$m=19456,t=2,p=1$BLqBY8W7hpGFWlFG4en52g$B69DLn0xlikLL0XDbwTDg8v3mQiJqbItLi3uWAvAqfo'),
('madhan', '$argon2id$v=19$m=19456,t=2,p=1$BLqBY8W7hpGFWlFG4en52g$B69DLn0xlikLL0XDbwTDg8v3mQiJqbItLi3uWAvAqfo');

-- Seed tags
INSERT OR IGNORE INTO tags (id, name) VALUES
(1, 'work'),
(2, 'personal'),
(3, 'urgent');

-- Seed todos
INSERT OR IGNORE INTO todos (id, user_id, title, due, completed, notes) VALUES
(1, 'admin', 'Complete project', '2024-12-29', 0, 'Finish before New Year'),
(1, 'admin', 'Complete project', '2024-12-30', 0, 'for tomorrow'),
(2, 'madhan', 'Buy groceries', '2024-12-30', 0, 'Get milk, eggs, and bread');

-- Seed relationships
INSERT OR IGNORE INTO todo_tags (todo_id, tag_id) VALUES
(1, 1),
(1, 3),
(2, 2);

INSERT OR IGNORE INTO user_tags (user_id, tag_id) VALUES
('admin', 1),
('admin', 2),
('madhan', 3);

