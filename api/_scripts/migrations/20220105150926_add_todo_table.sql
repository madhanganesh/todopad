-- +goose Up
-- +goose StatementBegin
CREATE TABLE todos (
	id SERIAL PRIMARY KEY,
	userid integer NOT NULL,
	title VARCHAR(500) NOT NULL,
	due timestamp with time zone default (now() at time zone 'utc'),
	done BOOLEAN NOT NULL default(false),
	effort FLOAT default (0),
	tags VARCHAR(500) NOT NULL default(''),
	notes TEXT NOT NULL default(''),
    CONSTRAINT fk_user FOREIGN KEY(userid) REFERENCES users(id)
);
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
SELECT 'drop down todos';
-- +goose StatementEnd
