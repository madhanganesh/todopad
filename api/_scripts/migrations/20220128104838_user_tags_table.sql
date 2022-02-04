-- +goose Up
-- +goose StatementBegin
CREATE TABLE usertags (
    userid integer NOT NULL,
    tag text NOT NULL,
    CONSTRAINT PK_USERTAG PRIMARY KEY (userid, tag) 
);
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
drop table userstags;
-- +goose StatementEnd
