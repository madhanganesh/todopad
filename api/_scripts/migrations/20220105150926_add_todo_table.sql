-- +goose Up
-- +goose StatementBegin
CREATE TABLE todos (
	id integer primary key AUTOINCREMENT,
	userid integer not null,
	title TEXT NOT NULL,
	due DATETIME DEFAULT(datetime('now')),
	done NUMERIC,
	effort FLOAT default 0,
	tags TEXT NOT NULL default "",
	notes TEXT NOT NULL default "",
	FOREIGN KEY (userid) REFERENCES users(id) ON UPDATE RESTRICT
);
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
SELECT 'drop down todos';
-- +goose StatementEnd
