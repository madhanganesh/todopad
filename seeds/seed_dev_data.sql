-- Seed users
INSERT OR IGNORE INTO users (id, hashed_password) VALUES
('admin', '$argon2id$v=19$m=19456,t=2,p=1$BLqBY8W7hpGFWlFG4en52g$B69DLn0xlikLL0XDbwTDg8v3mQiJqbItLi3uWAvAqfo'),
('madhan', '$argon2id$v=19$m=19456,t=2,p=1$BLqBY8W7hpGFWlFG4en52g$B69DLn0xlikLL0XDbwTDg8v3mQiJqbItLi3uWAvAqfo');

-- Seed todos
INSERT OR IGNORE INTO todos (id, user_id, title, due, completed, notes) VALUES
(1, 'admin', 'Complete project', '2024-12-29', 0, 'Finish before New Year'),
(1, 'admin', 'Complete project', '2024-12-30', 0, 'for tomorrow'),
(2, 'madhan', 'Buy groceries', '2024-12-30', 0, 'Get milk, eggs, and bread');

INSERT OR IGNORE INTO tags (user_id, todo_id, tag) VALUES
('admin', 1, "AMX"),
('admin', 1, "Coordination"),
('madhan',2, "Design");
