-- +goose Up
-- +goose StatementBegin
create table users (id integer primary key AUTOINCREMENT, name text not null, email char(50) UNIQUE, hpassword char(200));
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
drop table users;
-- +goose StatementEnd
