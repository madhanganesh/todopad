-- +goose Up
-- +goose StatementBegin
create table users (id serial primary key, name text not null, email varchar(75) UNIQUE NOT NULL, hpassword text UNIQUE NOT NULL);
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
drop table users;
-- +goose StatementEnd
